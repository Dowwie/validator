use std::{collections::HashMap, path::PathBuf};

use serde::Serialize;
use serde_json::Value;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

use crate::{
    Diagnostic, DiagnosticCode, Result,
    artifacts::{load_evidence_bindings, load_input_artifacts, publish_run},
    comparison,
    evaluation::{
        multi_label::evaluate as evaluate_multi_label,
        single_label::evaluate as evaluate_single_label,
    },
    model::{
        ValidatedTask, common::RunId, multi_label::assemble_report as assemble_multi_label_report,
        single_label::assemble_report,
    },
    validation::validate,
};

/// Explicit filesystem inputs for an immutable evaluation publication.
#[derive(Clone, Debug)]
pub struct EvaluationOptions {
    /// Path to the canonical golden dataset.
    pub dataset: PathBuf,
    /// Path to the submitted predictions artifact.
    pub predictions: PathBuf,
    /// Path to the evaluation configuration.
    pub config: PathBuf,
    /// New output directory that must not already exist.
    pub output: PathBuf,
}

/// Receipt for a successfully published evaluation report.
#[derive(Debug, Serialize)]
pub struct EvaluationReceipt {
    schema_version: u32,
    kind: &'static str,
    status: &'static str,
    run_id: String,
    result_path: String,
    result_sha256: String,
}

/// Explicit saved evaluation report inputs in attempt order.
#[derive(Clone, Debug)]
pub struct ComparisonOptions {
    /// Evaluation report files to aggregate; at least two are required.
    pub reports: Vec<PathBuf>,
}

/// Validates, evaluates, and publishes a new immutable evaluation run.
///
/// # Errors
///
/// Returns a typed diagnostic for invalid input, failed evaluation, or failed
/// immutable publication. Existing output paths are never replaced.
pub fn evaluate(options: EvaluationOptions) -> Result<EvaluationReceipt> {
    let (artifacts, evaluation, evidence) =
        admit(&options.dataset, &options.predictions, &options.config)?;
    match evaluation {
        ValidatedTask::SingleLabel(evaluation) => {
            let results = evaluate_single_label(&evaluation)?;
            let run_id = RunId::try_from(uuid::Uuid::now_v7().hyphenated().to_string().as_str())?;
            let created_at = OffsetDateTime::now_utc()
                .format(&Rfc3339)
                .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?;
            let report = assemble_report(
                run_id,
                created_at,
                env!("CARGO_PKG_VERSION").to_owned(),
                &evaluation,
                &results,
                (
                    artifacts.golden(),
                    artifacts.predictions(),
                    artifacts.config(),
                ),
                &evidence,
            )?;
            let report = serde_json::to_vec(&report)
                .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?;
            let published = publish_run(&options.output, &report, &artifacts, &evidence)?;
            Ok(EvaluationReceipt {
                schema_version: crate::WIRE_VERSION,
                kind: "evaluation",
                status: "complete",
                run_id: run_id.to_string(),
                result_path: published.report_path().display().to_string(),
                result_sha256: published.report_digest().to_string(),
            })
        }
        ValidatedTask::MultiLabel(evaluation) => {
            let results = evaluate_multi_label(&evaluation)?;
            let run_id = RunId::try_from(uuid::Uuid::now_v7().hyphenated().to_string().as_str())?;
            let created_at = OffsetDateTime::now_utc()
                .format(&Rfc3339)
                .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?;
            let report = assemble_multi_label_report(
                run_id,
                created_at,
                env!("CARGO_PKG_VERSION").to_owned(),
                &evaluation,
                &results,
                (
                    artifacts.golden(),
                    artifacts.predictions(),
                    artifacts.config(),
                ),
                &evidence,
            )?;
            let report = serde_json::to_vec(&report)
                .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?;
            let published = publish_run(&options.output, &report, &artifacts, &evidence)?;
            Ok(EvaluationReceipt {
                schema_version: crate::WIRE_VERSION,
                kind: "evaluation",
                status: "complete",
                run_id: run_id.to_string(),
                result_path: published.report_path().display().to_string(),
                result_sha256: published.report_digest().to_string(),
            })
        }
    }
}

/// Reads and aggregates saved reports without validation, replay, or publication.
pub fn compare(options: ComparisonOptions) -> Result<Value> {
    let mut reports = Vec::with_capacity(options.reports.len());
    let mut decoded: HashMap<PathBuf, comparison::ReportInput> = HashMap::new();
    for path in options.reports {
        if let Some(report) = decoded.get(&path) {
            reports.push(report.clone());
            continue;
        }
        let bytes =
            std::fs::read_to_string(&path).map_err(|_| Diagnostic::for_code(DiagnosticCode::Io))?;
        let report = comparison::decode_report(path.display().to_string(), &bytes)?;
        decoded.insert(path, report.clone());
        reports.push(report);
    }
    comparison::aggregate(&reports)
}

fn admit(
    dataset: &std::path::Path,
    predictions: &std::path::Path,
    config: &std::path::Path,
) -> Result<(
    crate::artifacts::InputArtifacts,
    ValidatedTask,
    Vec<crate::model::common::EvidenceBinding>,
)> {
    let artifacts = load_input_artifacts(dataset, predictions, config)?;
    let evaluation = validate(
        artifacts.golden().bytes(),
        artifacts.predictions().bytes(),
        artifacts.config().bytes(),
        artifacts.golden().digest(),
    )?;
    let evidence = match &evaluation {
        ValidatedTask::SingleLabel(evaluation) => {
            load_evidence_bindings(artifacts.predictions(), evaluation.sources())?
        }
        ValidatedTask::MultiLabel(evaluation) => {
            load_evidence_bindings(artifacts.predictions(), evaluation.sources())?
        }
    };
    Ok((artifacts, evaluation, evidence))
}
