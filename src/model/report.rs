use std::collections::{BTreeMap, HashMap};

use serde::Serialize;
use serde_json::value::RawValue;

use super::common::{
    ArtifactSnapshot, EvidenceBinding, LabelVocabulary, ObservationKind, Population, RunId,
    SourceDefinition, SourceId, SourceKind,
};
use crate::{Diagnostic, DiagnosticCode, Result};

/// Shared report metadata, serialized without decoding opaque configurations again.
#[derive(Serialize)]
pub(super) struct ReportContext {
    schema_version: u32,
    kind: &'static str,
    status: &'static str,
    identity: ReportIdentity,
    artifacts: Vec<ReportArtifact>,
    sources: BTreeMap<String, ReportSource>,
    composition: &'static str,
    source_counts: BTreeMap<String, u64>,
    population: ReportPopulation,
    task: ReportTask,
}

#[derive(Serialize)]
struct ReportIdentity {
    run_id: String,
    created_at: String,
    validator_version: String,
    specification_version: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_run_id: Option<String>,
}

#[derive(Serialize)]
struct ReportArtifact {
    kind: &'static str,
    path: String,
    sha256: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evidence_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    original_path: Option<String>,
}

#[derive(Serialize)]
struct ReportSource {
    kind: SourceKind,
    model: String,
    configuration: Box<RawValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    question_id: Option<String>,
    evidence: Vec<String>,
    observations: BTreeMap<String, ReportObservationDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preparation: Option<ReportPreparation>,
}

#[derive(Serialize)]
struct ReportObservationDefinition {
    kind: ObservationKind,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    question_id: Option<String>,
}

#[derive(Serialize)]
struct ReportPreparation {
    method: String,
    version: String,
    configuration: Box<RawValue>,
    evidence_indices: Vec<usize>,
}

#[derive(Serialize)]
struct ReportPopulation {
    description: String,
    role: &'static str,
    dataset_count: usize,
    selected_count: usize,
    selected_ids: Vec<String>,
    unselected_ids: Vec<String>,
}

#[derive(Serialize)]
struct ReportTask {
    kind: &'static str,
    labels: Vec<String>,
}

impl ReportContext {
    pub(super) fn new<'a, Policy>(
        identity: (RunId, String, String),
        task: (&'static str, &LabelVocabulary),
        population: &Population<Policy>,
        sources: &HashMap<SourceId, SourceDefinition>,
        source_ids: impl Iterator<Item = &'a SourceId>,
        snapshots: (&ArtifactSnapshot, &ArtifactSnapshot, &ArtifactSnapshot),
        evidence: &[EvidenceBinding],
    ) -> Result<Self> {
        let (golden, predictions, config) = snapshots;
        if population.dataset_digest() != golden.digest() {
            return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
        }
        let mut artifacts = [
            ("golden", golden),
            ("predictions", predictions),
            ("config", config),
        ]
        .into_iter()
        .map(|(kind, snapshot)| ReportArtifact {
            kind,
            path: snapshot.stored_path().display().to_string(),
            sha256: snapshot.digest().to_string(),
            source_id: None,
            evidence_index: None,
            original_path: None,
        })
        .collect::<Vec<_>>();
        artifacts.extend(evidence.iter().map(|binding| ReportArtifact {
            kind: "evidence",
            path: binding.stored_path().display().to_string(),
            sha256: binding.digest().to_string(),
            source_id: Some(binding.source_id().as_str().to_owned()),
            evidence_index: Some(binding.evidence_index()),
            original_path: Some(binding.original_path().to_owned()),
        }));
        let mut source_counts = sources
            .keys()
            .map(|id| (id.as_str().to_owned(), 0_u64))
            .collect::<BTreeMap<_, _>>();
        for id in source_ids {
            let count = source_counts
                .get_mut(id.as_str())
                .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))?;
            *count = count
                .checked_add(1)
                .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Numeric))?;
        }
        Ok(Self {
            schema_version: crate::WIRE_VERSION,
            kind: "evaluation",
            status: "complete",
            identity: ReportIdentity {
                run_id: identity.0.to_string(),
                created_at: identity.1,
                validator_version: identity.2,
                specification_version: crate::SPECIFICATION_VERSION,
                parent_run_id: population.config().parent_run_id().map(|id| id.to_string()),
            },
            artifacts,
            sources: report_sources(sources, evidence),
            composition: match source_counts.values().filter(|count| **count > 0).count() {
                0 => "empty",
                1 => "single_source",
                _ => "mixed_source",
            },
            source_counts,
            population: ReportPopulation {
                description: population.config().description().to_owned(),
                role: match population.config().role() {
                    super::common::EvaluationRole::Development => "development",
                    super::common::EvaluationRole::HeldOut => "held_out",
                },
                dataset_count: population.dataset_count(),
                selected_count: population.selected_count(),
                selected_ids: population
                    .selected()
                    .iter()
                    .map(ToString::to_string)
                    .collect(),
                unselected_ids: population
                    .unselected()
                    .iter()
                    .map(ToString::to_string)
                    .collect(),
            },
            task: ReportTask {
                kind: task.0,
                labels: task.1.labels().map(str::to_owned).collect(),
            },
        })
    }
}

fn report_sources(
    sources: &HashMap<SourceId, SourceDefinition>,
    evidence: &[EvidenceBinding],
) -> BTreeMap<String, ReportSource> {
    sources
        .iter()
        .map(|(id, source)| {
            (
                id.as_str().to_owned(),
                ReportSource {
                    kind: source.kind(),
                    model: source.model().to_owned(),
                    configuration: source.configuration().to_owned(),
                    question_id: source.question_id().map(str::to_owned),
                    evidence: evidence
                        .iter()
                        .filter(|binding| binding.source_id() == id)
                        .map(|binding| binding.stored_path().display().to_string())
                        .collect(),
                    observations: source
                        .observation_definitions()
                        .iter()
                        .map(|(name, definition)| {
                            (
                                name.clone(),
                                ReportObservationDefinition {
                                    kind: definition.kind(),
                                    description: definition.description().to_owned(),
                                    question_id: definition.question_id().map(str::to_owned),
                                },
                            )
                        })
                        .collect(),
                    preparation: source.preparation().map(|value| ReportPreparation {
                        method: value.method().to_owned(),
                        version: value.version().to_owned(),
                        configuration: value.configuration().to_owned(),
                        evidence_indices: value.evidence_indices().to_vec(),
                    }),
                },
            )
        })
        .collect()
}
