use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json, value::RawValue};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

use crate::{
    Diagnostic, DiagnosticCode, Result,
    artifacts::{
        load_evidence_bindings, load_input_artifacts, load_stored_run, publish_comparison,
        publish_run,
    },
    comparison,
    evaluation::{
        multi_label::evaluate as evaluate_multi_label,
        single_label::evaluate as evaluate_single_label,
    },
    model::{
        ValidatedTask,
        common::RunId,
        multi_label::assemble_report as assemble_multi_label_report,
        single_label::{assemble_report, normalization_diagnostics},
    },
    validation::validate,
};

/// Explicit filesystem inputs for a validation-only check.
#[derive(Clone, Debug)]
pub struct CheckOptions {
    /// Path to the canonical golden dataset.
    pub dataset: PathBuf,
    /// Path to the submitted predictions artifact.
    pub predictions: PathBuf,
    /// Path to the evaluation configuration.
    pub config: PathBuf,
}

/// The complete successful task-tagged machine check result.
pub type CheckResult = Value;

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

/// Explicit stored-run and selected-episode inputs for payload inspection.
#[derive(Clone, Debug)]
pub struct InspectionOptions {
    /// Completed run directory to verify before disclosure.
    pub run: PathBuf,
    /// Selected episode identifier to disclose.
    pub episode: String,
}

/// The sole payload-disclosing successful command result.
#[derive(Serialize)]
pub struct InspectionResult {
    schema_version: u32,
    kind: &'static str,
    status: &'static str,
    identity: Value,
    episode_id: String,
    input: Box<RawValue>,
    expected: Box<RawValue>,
    prediction: Box<RawValue>,
    source: Value,
    final_outcome: Value,
    observations: Value,
    probability: Value,
    reported_confidence: Value,
    configuration: Box<RawValue>,
}

/// Verified stored-run inputs and a new immutable comparison directory.
#[derive(Clone, Debug)]
pub struct ComparisonOptions {
    /// Verified baseline run directory.
    pub baseline: PathBuf,
    /// Verified candidate run directory.
    pub candidate: PathBuf,
    /// Restrict distinct selected populations to their common episode IDs.
    pub intersection: bool,
    /// New comparison output directory.
    pub output: PathBuf,
}

/// Receipt for an immutable comparison document.
#[derive(Debug, Serialize)]
pub struct ComparisonReceipt {
    schema_version: u32,
    kind: &'static str,
    status: &'static str,
    comparison_id: String,
    result_path: String,
    result_sha256: String,
}

#[derive(Deserialize)]
struct RawEpisodeEnvelope {
    episodes: Vec<Box<RawValue>>,
}

#[derive(Deserialize)]
struct RawPredictionEnvelope {
    predictions: Vec<Box<RawValue>>,
}

#[derive(Deserialize)]
struct RawGoldenEpisode {
    id: String,
    expected: Box<RawValue>,
}

#[derive(Deserialize)]
struct RawPrediction {
    id: String,
    source_id: String,
}

struct VerifiedSingleLabelRun {
    evaluation: crate::model::single_label::SingleLabelEvaluation,
    results: crate::model::single_label::SingleLabelResults,
    metadata: comparison::ComparisonMetadata,
}

struct VerifiedMultiLabelRun {
    evaluation: crate::model::multi_label::MultiLabelEvaluation,
    results: crate::model::multi_label::MultiLabelResults,
    metadata: comparison::ComparisonMetadata,
}

enum VerifiedRun {
    SingleLabel(Box<VerifiedSingleLabelRun>),
    MultiLabel(Box<VerifiedMultiLabelRun>),
}

/// Performs full checked admission without scoring or filesystem publication.
///
/// # Errors
///
/// Returns a typed diagnostic for invalid input, filesystem failures, or failed
/// checked admission. This function does not create output files.
pub fn check(options: CheckOptions) -> Result<CheckResult> {
    let (artifacts, evaluation, evidence) =
        admit(&options.dataset, &options.predictions, &options.config)?;
    let _ = artifacts;
    let _ = evidence;
    match evaluation {
        ValidatedTask::SingleLabel(evaluation) => {
            let normalization = normalization_diagnostics(&evaluation)?;
            Ok(json!({
                "schema_version": crate::WIRE_VERSION, "kind": "check", "status": "complete",
                "task": {"kind": "single_label", "labels": evaluation.vocabulary().labels().collect::<Vec<_>>()},
                "integrity": {"source_count": evaluation.sources().len(), "selected_count": evaluation.population().selected_count(), "prediction_count": evaluation.rows().len(), "missing_ids": [], "extra_ids": [], "signal_availability": evaluation.signals(), "normalized_count": normalization.normalized_count, "maximum_sum_error": normalization.maximum_sum_error},
                "selected_ids": evaluation.population().selected().iter().map(ToString::to_string).collect::<Vec<_>>(),
            }))
        }
        ValidatedTask::MultiLabel(evaluation) => Ok(json!({
            "schema_version": crate::WIRE_VERSION, "kind": "check", "status": "complete",
            "task": {"kind": "multi_label", "labels": evaluation.vocabulary().labels().collect::<Vec<_>>()},
            "integrity": {"source_count": evaluation.sources().len(), "selected_count": evaluation.population().selected_count(), "prediction_count": evaluation.rows().len(), "missing_ids": [], "extra_ids": [], "signal_availability": if evaluation.marginals_available() { "marginals" } else { "none" }},
            "selected_ids": evaluation.population().selected().iter().map(ToString::to_string).collect::<Vec<_>>(),
        })),
    }
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

/// Verifies a stored run and reveals one selected episode's protected payload.
///
/// # Errors
///
/// Returns a typed diagnostic when stored snapshots, bindings, replayed results,
/// or the requested selected episode are invalid.
pub fn inspect(options: InspectionOptions) -> Result<InspectionResult> {
    let stored = load_stored_run(&options.run)?;
    let evaluation = validate(
        stored.artifacts().golden().bytes(),
        stored.artifacts().predictions().bytes(),
        stored.artifacts().config().bytes(),
        stored.artifacts().golden().digest(),
    )?;
    let identity = stored
        .report()
        .get("identity")
        .and_then(Value::as_object)
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Schema))?;
    let run_id = identity
        .get("run_id")
        .and_then(Value::as_str)
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Schema))?;
    let created_at = identity
        .get("created_at")
        .and_then(Value::as_str)
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Schema))?;
    let validator_version = identity
        .get("validator_version")
        .and_then(Value::as_str)
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Schema))?;
    let evidence = match &evaluation {
        ValidatedTask::SingleLabel(evaluation) => stored.verify_evidence(evaluation.sources())?,
        ValidatedTask::MultiLabel(evaluation) => stored.verify_evidence(evaluation.sources())?,
    };
    let input = inspection_input(&evaluation, &options.episode)?;
    let (rebuilt, selected) = rebuild_report(
        evaluation,
        RunId::try_from(run_id)?,
        created_at.to_owned(),
        validator_version.to_owned(),
        stored.artifacts(),
        &evidence,
    )?;
    if !same_report(stored.report(), &rebuilt, &mut Vec::new()) {
        return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
    }

    let golden: RawEpisodeEnvelope = serde_json::from_slice(stored.artifacts().golden().bytes())
        .map_err(|_| Diagnostic::for_code(DiagnosticCode::Schema))?;
    let predictions: RawPredictionEnvelope =
        serde_json::from_slice(stored.artifacts().predictions().bytes())
            .map_err(|_| Diagnostic::for_code(DiagnosticCode::Schema))?;
    if !selected.iter().any(|id| id == &options.episode) {
        return Err(Diagnostic::for_code(DiagnosticCode::Id));
    }
    let golden = golden
        .episodes
        .into_iter()
        .find_map(|raw| {
            let row: RawGoldenEpisode = serde_json::from_str(raw.get()).ok()?;
            (row.id == options.episode).then_some(row)
        })
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))?;
    let prediction = predictions
        .predictions
        .into_iter()
        .find(|raw| {
            serde_json::from_str::<RawPrediction>(raw.get())
                .is_ok_and(|row| row.id == options.episode)
        })
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))?;
    let source_id = serde_json::from_str::<RawPrediction>(prediction.get())
        .map_err(|_| Diagnostic::for_code(DiagnosticCode::Invariant))?
        .source_id;
    let source = stored
        .report()
        .get("sources")
        .and_then(Value::as_object)
        .and_then(|sources| sources.get(&source_id))
        .cloned()
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))?;
    let report_episode = stored
        .report()
        .get("episodes")
        .and_then(Value::as_array)
        .and_then(|episodes| {
            episodes.iter().find(|episode| {
                episode.get("id").and_then(Value::as_str) == Some(options.episode.as_str())
            })
        })
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))?;
    let configuration = RawValue::from_string(
        String::from_utf8(stored.artifacts().config().bytes().to_vec())
            .map_err(|_| Diagnostic::for_code(DiagnosticCode::Schema))?,
    )
    .map_err(|_| Diagnostic::for_code(DiagnosticCode::Schema))?;
    Ok(InspectionResult {
        schema_version: crate::WIRE_VERSION,
        kind: "inspection",
        status: "complete",
        identity: stored.report()["identity"].clone(),
        episode_id: options.episode,
        input,
        expected: golden.expected,
        prediction,
        source,
        final_outcome: report_episode["final_outcome"].clone(),
        observations: report_episode["observations"].clone(),
        probability: report_episode["probability"].clone(),
        reported_confidence: report_episode
            .get("reported_confidence")
            .cloned()
            .unwrap_or(Value::Null),
        configuration,
    })
}

fn inspection_input(evaluation: &ValidatedTask, episode_id: &str) -> Result<Box<RawValue>> {
    let input = match evaluation {
        ValidatedTask::SingleLabel(evaluation) => evaluation
            .rows()
            .iter()
            .find(|row| row.episode_id().to_string() == episode_id)
            .map(|row| row.input()),
        ValidatedTask::MultiLabel(evaluation) => evaluation
            .rows()
            .iter()
            .find(|row| row.episode_id().to_string() == episode_id)
            .map(|row| row.input()),
    };
    input
        .map(ToOwned::to_owned)
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Id))
}

fn rebuild_report(
    evaluation: ValidatedTask,
    run_id: RunId,
    created_at: String,
    validator_version: String,
    artifacts: &crate::artifacts::InputArtifacts,
    evidence: &[crate::model::common::EvidenceBinding],
) -> Result<(Value, Vec<String>)> {
    match evaluation {
        ValidatedTask::SingleLabel(evaluation) => {
            let selected = evaluation
                .population()
                .selected()
                .iter()
                .map(ToString::to_string)
                .collect();
            let results = evaluate_single_label(&evaluation)?;
            let report = assemble_report(
                run_id,
                created_at,
                validator_version,
                &evaluation,
                &results,
                (
                    artifacts.golden(),
                    artifacts.predictions(),
                    artifacts.config(),
                ),
                evidence,
            )?;
            let report = serde_json::to_value(report)
                .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?;
            Ok((report, selected))
        }
        ValidatedTask::MultiLabel(evaluation) => {
            let selected = evaluation
                .population()
                .selected()
                .iter()
                .map(ToString::to_string)
                .collect();
            let results = evaluate_multi_label(&evaluation)?;
            let report = assemble_multi_label_report(
                run_id,
                created_at,
                validator_version,
                &evaluation,
                &results,
                (
                    artifacts.golden(),
                    artifacts.predictions(),
                    artifacts.config(),
                ),
                evidence,
            )?;
            Ok((report, selected))
        }
    }
}

/// Verifies two stored runs and publishes their paired comparison.
pub fn compare(options: ComparisonOptions) -> Result<ComparisonReceipt> {
    let baseline = replay_report(&options.baseline)?;
    let candidate = replay_report(&options.candidate)?;
    let comparison_id = uuid::Uuid::now_v7().hyphenated().to_string();
    let created_at = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?;
    let document = match (baseline, candidate) {
        (VerifiedRun::SingleLabel(baseline), VerifiedRun::SingleLabel(candidate)) => {
            if options.intersection {
                comparison::compatible_intersection(
                    &comparison::ComparisonInput {
                        evaluation: &baseline.evaluation,
                        results: &baseline.results,
                        metadata: &baseline.metadata,
                    },
                    &comparison::ComparisonInput {
                        evaluation: &candidate.evaluation,
                        results: &candidate.results,
                        metadata: &candidate.metadata,
                    },
                )?;
                let (ids, scope) = comparison::intersection_scope(
                    baseline.evaluation.population().selected(),
                    candidate.evaluation.population().selected(),
                )?;
                let baseline_evaluation = baseline.evaluation.restrict(&ids)?;
                let candidate_evaluation = candidate.evaluation.restrict(&ids)?;
                let baseline_results = evaluate_single_label(&baseline_evaluation)?;
                let candidate_results = evaluate_single_label(&candidate_evaluation)?;
                serde_json::to_value(comparison::compare(
                    comparison::ComparisonInput {
                        evaluation: &baseline_evaluation,
                        results: &baseline_results,
                        metadata: &baseline.metadata,
                    },
                    comparison::ComparisonInput {
                        evaluation: &candidate_evaluation,
                        results: &candidate_results,
                        metadata: &candidate.metadata,
                    },
                    scope,
                    comparison_id.clone(),
                    created_at,
                )?)
                .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?
            } else {
                let scope =
                    comparison::identical_scope(baseline.evaluation.population().selected())?;
                serde_json::to_value(comparison::compare(
                    comparison::ComparisonInput {
                        evaluation: &baseline.evaluation,
                        results: &baseline.results,
                        metadata: &baseline.metadata,
                    },
                    comparison::ComparisonInput {
                        evaluation: &candidate.evaluation,
                        results: &candidate.results,
                        metadata: &candidate.metadata,
                    },
                    scope,
                    comparison_id.clone(),
                    created_at,
                )?)
                .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?
            }
        }
        (VerifiedRun::MultiLabel(baseline), VerifiedRun::MultiLabel(candidate)) => {
            if options.intersection {
                comparison::compatible_multi_label_intersection(
                    &comparison::MultiLabelComparisonInput {
                        evaluation: &baseline.evaluation,
                        results: &baseline.results,
                        metadata: &baseline.metadata,
                    },
                    &comparison::MultiLabelComparisonInput {
                        evaluation: &candidate.evaluation,
                        results: &candidate.results,
                        metadata: &candidate.metadata,
                    },
                )?;
                let (ids, scope) = comparison::intersection_scope(
                    baseline.evaluation.population().selected(),
                    candidate.evaluation.population().selected(),
                )?;
                let baseline_evaluation = baseline.evaluation.restrict(&ids)?;
                let candidate_evaluation = candidate.evaluation.restrict(&ids)?;
                let baseline_results = evaluate_multi_label(&baseline_evaluation)?;
                let candidate_results = evaluate_multi_label(&candidate_evaluation)?;
                serde_json::to_value(comparison::compare_multi_label(
                    comparison::MultiLabelComparisonInput {
                        evaluation: &baseline_evaluation,
                        results: &baseline_results,
                        metadata: &baseline.metadata,
                    },
                    comparison::MultiLabelComparisonInput {
                        evaluation: &candidate_evaluation,
                        results: &candidate_results,
                        metadata: &candidate.metadata,
                    },
                    scope,
                    comparison_id.clone(),
                    created_at,
                )?)
                .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?
            } else {
                let scope =
                    comparison::identical_scope(baseline.evaluation.population().selected())?;
                serde_json::to_value(comparison::compare_multi_label(
                    comparison::MultiLabelComparisonInput {
                        evaluation: &baseline.evaluation,
                        results: &baseline.results,
                        metadata: &baseline.metadata,
                    },
                    comparison::MultiLabelComparisonInput {
                        evaluation: &candidate.evaluation,
                        results: &candidate.results,
                        metadata: &candidate.metadata,
                    },
                    scope,
                    comparison_id.clone(),
                    created_at,
                )?)
                .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?
            }
        }
        _ => return Err(Diagnostic::for_code(DiagnosticCode::Comparison)),
    };
    let bytes =
        serde_json::to_vec(&document).map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?;
    let published = publish_comparison(&options.output, &bytes)?;
    Ok(ComparisonReceipt {
        schema_version: crate::WIRE_VERSION,
        kind: "comparison",
        status: "complete",
        comparison_id,
        result_path: published.report_path().display().to_string(),
        result_sha256: published.report_digest().to_string(),
    })
}

fn same_report(left: &Value, right: &Value, path: &mut Vec<String>) -> bool {
    match (left, right) {
        (Value::Object(left), Value::Object(right)) => {
            left.len() == right.len()
                && left.iter().all(|(key, value)| {
                    path.push(key.clone());
                    let matches = right
                        .get(key)
                        .is_some_and(|other| same_report(value, other, path));
                    path.pop();
                    matches
                })
        }
        (Value::Array(left), Value::Array(right)) => {
            left.len() == right.len()
                && left.iter().zip(right).enumerate().all(|(index, (a, b))| {
                    path.push(index.to_string());
                    let matches = same_report(a, b, path);
                    path.pop();
                    matches
                })
        }
        (Value::Number(left), Value::Number(right)) => match (left.as_f64(), right.as_f64()) {
            (Some(left), Some(right))
                if computed_result_path(path) && left.is_finite() && right.is_finite() =>
            {
                (left - right).abs()
                    <= crate::FIXTURE_ABSOLUTE_TOLERANCE
                        + crate::FIXTURE_RELATIVE_TOLERANCE * left.abs().max(right.abs())
            }
            _ => left.to_string() == right.to_string(),
        },
        _ => left == right,
    }
}

fn computed_result_path(path: &[String]) -> bool {
    path == ["integrity", "maximum_sum_error"]
        || signal_bin_result_path(path)
        || metric_result_path(path)
}

fn replay_report(run: &std::path::Path) -> Result<VerifiedRun> {
    let stored = load_stored_run(run)?;
    let kind = stored
        .report()
        .get("task")
        .and_then(|task| task.get("kind"))
        .and_then(Value::as_str)
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Schema))?;
    match kind {
        "single_label" => {
            replay_single_label_report(run).map(|run| VerifiedRun::SingleLabel(Box::new(run)))
        }
        "multi_label" => {
            replay_multi_label_report(run).map(|run| VerifiedRun::MultiLabel(Box::new(run)))
        }
        _ => Err(Diagnostic::for_code(DiagnosticCode::Schema)),
    }
}

fn replay_single_label_report(run: &std::path::Path) -> Result<VerifiedSingleLabelRun> {
    let stored = load_stored_run(run)?;
    let evaluation = validate(
        stored.artifacts().golden().bytes(),
        stored.artifacts().predictions().bytes(),
        stored.artifacts().config().bytes(),
        stored.artifacts().golden().digest(),
    )?;
    let ValidatedTask::SingleLabel(evaluation) = evaluation else {
        return Err(Diagnostic::for_code(DiagnosticCode::Config));
    };
    let evidence = stored.verify_evidence(evaluation.sources())?;
    let results = evaluate_single_label(&evaluation)?;
    let identity = stored
        .report()
        .get("identity")
        .and_then(Value::as_object)
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Schema))?;
    let run_id = identity
        .get("run_id")
        .and_then(Value::as_str)
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Schema))?;
    let created_at = identity
        .get("created_at")
        .and_then(Value::as_str)
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Schema))?;
    let validator_version = identity
        .get("validator_version")
        .and_then(Value::as_str)
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Schema))?;
    let rebuilt = assemble_report(
        RunId::try_from(run_id)?,
        created_at.to_owned(),
        validator_version.to_owned(),
        &evaluation,
        &results,
        (
            stored.artifacts().golden(),
            stored.artifacts().predictions(),
            stored.artifacts().config(),
        ),
        &evidence,
    )?;
    let rebuilt =
        serde_json::to_value(rebuilt).map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?;
    if !same_report(stored.report(), &rebuilt, &mut Vec::new()) {
        return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
    }
    let metadata = comparison::ComparisonMetadata {
        run_id: RunId::try_from(run_id)?,
        report_digest: stored.report_digest(),
        golden_digest: evaluation.population().dataset_digest(),
        sources: required_report_value(stored.report(), "sources")?.clone(),
        decision: stored
            .report()
            .get("policy")
            .and_then(|policy| policy.get("decision"))
            .cloned()
            .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Schema))?,
        signal_availability: stored
            .report()
            .get("integrity")
            .and_then(|integrity| integrity.get("signal_availability"))
            .cloned()
            .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Schema))?,
        categorical_sum_tolerance: crate::CATEGORICAL_SUM_TOLERANCE,
        bin_count: crate::TEN_BIN_COUNT,
    };
    Ok(VerifiedSingleLabelRun {
        evaluation,
        results,
        metadata,
    })
}

fn replay_multi_label_report(run: &std::path::Path) -> Result<VerifiedMultiLabelRun> {
    let stored = load_stored_run(run)?;
    let evaluation = validate(
        stored.artifacts().golden().bytes(),
        stored.artifacts().predictions().bytes(),
        stored.artifacts().config().bytes(),
        stored.artifacts().golden().digest(),
    )?;
    let ValidatedTask::MultiLabel(evaluation) = evaluation else {
        return Err(Diagnostic::for_code(DiagnosticCode::Comparison));
    };
    let evidence = stored.verify_evidence(evaluation.sources())?;
    let results = evaluate_multi_label(&evaluation)?;
    let identity = stored
        .report()
        .get("identity")
        .and_then(Value::as_object)
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Schema))?;
    let run_id = identity
        .get("run_id")
        .and_then(Value::as_str)
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Schema))?;
    let created_at = identity
        .get("created_at")
        .and_then(Value::as_str)
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Schema))?;
    let validator_version = identity
        .get("validator_version")
        .and_then(Value::as_str)
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Schema))?;
    let rebuilt = assemble_multi_label_report(
        RunId::try_from(run_id)?,
        created_at.to_owned(),
        validator_version.to_owned(),
        &evaluation,
        &results,
        (
            stored.artifacts().golden(),
            stored.artifacts().predictions(),
            stored.artifacts().config(),
        ),
        &evidence,
    )?;
    if !same_report(stored.report(), &rebuilt, &mut Vec::new()) {
        return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
    }
    let metadata = comparison::ComparisonMetadata {
        run_id: RunId::try_from(run_id)?,
        report_digest: stored.report_digest(),
        golden_digest: evaluation.population().dataset_digest(),
        sources: required_report_value(stored.report(), "sources")?.clone(),
        decision: stored
            .report()
            .get("policy")
            .and_then(|policy| policy.get("decision"))
            .cloned()
            .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Schema))?,
        signal_availability: stored
            .report()
            .get("integrity")
            .and_then(|integrity| integrity.get("signal_availability"))
            .cloned()
            .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Schema))?,
        categorical_sum_tolerance: crate::CATEGORICAL_SUM_TOLERANCE,
        bin_count: crate::TEN_BIN_COUNT,
    };
    Ok(VerifiedMultiLabelRun {
        evaluation,
        results,
        metadata,
    })
}

fn required_report_value<'a>(report: &'a Value, field: &str) -> Result<&'a Value> {
    report
        .get(field)
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Schema))
}

fn signal_bin_result_path(path: &[String]) -> bool {
    matches!(
        path,
        [
            signals,
            signal,
            bins,
            index,
            value
        ] if signals == "signals"
            && matches!(signal.as_str(), "maximum_probability" | "confidence")
            && bins == "bins"
            && index.parse::<usize>().is_ok()
            && matches!(value.as_str(), "mean_signal" | "empirical_accuracy")
    )
}

fn metric_result_path(path: &[String]) -> bool {
    matches!(
        path,
        [root, metric, value]
            if matches!(root.as_str(), "raw" | "final")
                && matches!(metric.as_str(), "accuracy" | "wrong_class_rate" | "abstention_rate" | "coverage" | "selective_accuracy" | "selective_risk" | "exact_match_accuracy")
                && value == "value"
    ) || matches!(
        path,
        [root, classes, index, metric, value]
            if matches!(root.as_str(), "raw" | "final")
                && classes == "classes"
                && index.parse::<usize>().is_ok()
                && matches!(metric.as_str(), "precision" | "recall" | "f1" | "coverage")
                && value == "value"
    ) || matches!(
        path,
        [root, macro_f1, metric, value]
            if matches!(root.as_str(), "raw" | "final")
                && macro_f1 == "macro_f1"
                && metric == "metric"
                && value == "value"
    ) || matches!(
        path,
        [probability, metric, value]
            if probability == "probability"
                && matches!(metric.as_str(), "log_loss" | "brier_score" | "argmax_accuracy")
                && value == "value"
    ) || matches!(
        path,
        [signals, metric, value]
            if signals == "signals" && metric == "top_label_ece" && value == "value"
    ) || multi_label_hard_metric_path(path)
        || multi_label_probability_metric_path(path)
        || multi_label_signal_bin_path(path)
}

fn multi_label_hard_metric_path(path: &[String]) -> bool {
    matches!(
        path,
        [root, metric, value]
            if matches!(root.as_str(), "raw" | "final")
                && matches!(metric.as_str(), "exact_match_accuracy" | "wrong_set_rate" | "abstention_rate" | "coverage" | "selective_exact_match_accuracy" | "selective_risk" | "answered_micro_precision" | "answered_micro_recall" | "answered_micro_f1" | "answered_hamming_loss")
                && value == "value"
    ) || matches!(
        path,
        [root, labels, index, metric, value]
            if matches!(root.as_str(), "raw" | "final")
                && labels == "labels"
                && index.parse::<usize>().is_ok()
                && matches!(metric.as_str(), "precision" | "recall" | "f1")
                && value == "value"
    ) || matches!(
        path,
        [root, macro_f1, metric, value]
            if matches!(root.as_str(), "raw" | "final")
                && macro_f1 == "answered_macro_f1"
                && metric == "metric"
                && value == "value"
    )
}

fn multi_label_probability_metric_path(path: &[String]) -> bool {
    matches!(
        path,
        [probability, metric, value]
            if probability == "probability"
                && matches!(metric.as_str(), "mean_binary_log_loss" | "mean_binary_brier")
                && value == "value"
    ) || matches!(
        path,
        [probability, labels, index, metric, value]
            if probability == "probability"
                && labels == "labels"
                && index.parse::<usize>().is_ok()
                && matches!(metric.as_str(), "binary_log_loss" | "binary_brier")
                && value == "value"
    )
}

fn multi_label_signal_bin_path(path: &[String]) -> bool {
    matches!(
        path,
        [signals, labels, label_index, bins, bin_index, value]
            if signals == "signals"
                && labels == "labels"
                && label_index.parse::<usize>().is_ok()
                && bins == "bins"
                && bin_index.parse::<usize>().is_ok()
                && matches!(value.as_str(), "mean_probability" | "observed_positive_rate")
    )
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
