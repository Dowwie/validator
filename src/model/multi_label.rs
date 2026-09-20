use std::collections::{BTreeMap, HashMap};

use crate::model::common::{
    ArtifactSnapshot, Episode, EpisodeId, LabelSet, LabelVocabulary, MetricResult, MetricStatus,
    ObservationSet, Outcome, Population, Prediction, SourceDefinition, SourceId,
};
use crate::{Diagnostic, DiagnosticCode, Result};
use serde::Serialize;
use serde_json::{Value, json};

/// Checked independent per-label probabilities in vocabulary order.
#[derive(Debug)]
pub struct LabelMarginals {
    values: Vec<f64>,
}

impl LabelMarginals {
    /// Validates exact vocabulary coverage without categorical normalization.
    pub(crate) fn new(values: &HashMap<String, f64>, vocabulary: &LabelVocabulary) -> Result<Self> {
        if values.len() != vocabulary.len() {
            return Err(Diagnostic::for_code(DiagnosticCode::Probability));
        }
        let mut ordered = Vec::with_capacity(vocabulary.len());
        for label in vocabulary.labels() {
            let Some(value) = values.get(label) else {
                return Err(Diagnostic::for_code(DiagnosticCode::Probability));
            };
            if !value.is_finite() || !(0.0..=1.0).contains(value) {
                return Err(Diagnostic::for_code(DiagnosticCode::Probability));
            }
            ordered.push(*value);
        }
        Ok(Self { values: ordered })
    }

    /// Borrows marginal values in vocabulary order.
    pub(crate) fn values(&self) -> &[f64] {
        &self.values
    }
}

/// A multi-label output with a whole-episode answer or abstention.
#[derive(Debug)]
pub struct MultiLabelOutput {
    outcome: Outcome<LabelSet>,
    marginals: Option<LabelMarginals>,
}

impl MultiLabelOutput {
    pub(crate) fn new(outcome: Outcome<LabelSet>, marginals: Option<LabelMarginals>) -> Self {
        Self { outcome, marginals }
    }

    pub(crate) fn outcome(&self) -> &Outcome<LabelSet> {
        &self.outcome
    }

    pub(crate) fn marginals(&self) -> Option<&LabelMarginals> {
        self.marginals.as_ref()
    }
}

/// The legal task-specific decision policies for multi-label evaluation.
#[derive(Clone, Debug, PartialEq)]
pub enum MultiLabelPolicy {
    /// Keep each submitted label set or abstention unchanged.
    AsRecorded,
    /// Select every vocabulary label whose marginal meets its threshold.
    LabelThresholds {
        /// Exact checked thresholds keyed by declared vocabulary label.
        thresholds: BTreeMap<String, f64>,
    },
}

/// A checked row with a complete set target and one prediction.
#[derive(Debug)]
pub struct MultiLabelAlignedRow {
    episode: Episode<LabelSet>,
    prediction: Prediction<MultiLabelOutput>,
}

impl MultiLabelAlignedRow {
    pub(crate) fn new(
        episode: Episode<LabelSet>,
        prediction: Prediction<MultiLabelOutput>,
        vocabulary: &LabelVocabulary,
    ) -> Result<Self> {
        if prediction.episode_id() != episode.id()
            || episode
                .target()
                .labels()
                .any(|label| vocabulary.lookup(label).is_none())
        {
            return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
        }
        Ok(Self {
            episode,
            prediction,
        })
    }

    pub(crate) fn episode_id(&self) -> EpisodeId {
        self.episode.id()
    }

    pub(crate) fn expected(&self) -> &LabelSet {
        self.episode.target()
    }

    pub(crate) fn input(&self) -> &serde_json::value::RawValue {
        self.episode.input()
    }

    pub(crate) fn prediction(&self) -> &Prediction<MultiLabelOutput> {
        &self.prediction
    }
}

/// A checked multi-label evaluation ready for later task-specific scoring.
#[derive(Debug)]
pub struct MultiLabelEvaluation {
    vocabulary: LabelVocabulary,
    sources: HashMap<SourceId, SourceDefinition>,
    population: Population<MultiLabelPolicy>,
    marginals_available: bool,
    rows: Vec<MultiLabelAlignedRow>,
}

impl MultiLabelEvaluation {
    pub(crate) fn new(
        vocabulary: LabelVocabulary,
        sources: HashMap<SourceId, SourceDefinition>,
        population: Population<MultiLabelPolicy>,
        marginals_available: bool,
        rows: Vec<MultiLabelAlignedRow>,
    ) -> Result<Self> {
        if rows.len() != population.selected_count()
            || rows.iter().zip(population.selected()).any(|(row, id)| {
                row.episode.id() != *id
                    || row.prediction.episode_id() != *id
                    || row
                        .episode
                        .target()
                        .labels()
                        .any(|label| vocabulary.lookup(label).is_none())
                    || !sources.contains_key(row.prediction.source_id())
                    || row.prediction.output().marginals().is_some() != marginals_available
            })
        {
            return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
        }
        Ok(Self {
            vocabulary,
            sources,
            population,
            marginals_available,
            rows,
        })
    }

    pub(crate) fn vocabulary(&self) -> &LabelVocabulary {
        &self.vocabulary
    }

    pub(crate) fn population(&self) -> &Population<MultiLabelPolicy> {
        &self.population
    }

    pub(crate) fn rows(&self) -> &[MultiLabelAlignedRow] {
        &self.rows
    }

    pub(crate) const fn marginals_available(&self) -> bool {
        self.marginals_available
    }

    pub(crate) fn sources(&self) -> &HashMap<SourceId, SourceDefinition> {
        &self.sources
    }

    pub(crate) fn restrict(self, ids: &[EpisodeId]) -> Result<Self> {
        let mut selected = ids.to_vec();
        selected.sort();
        let Self {
            vocabulary,
            sources,
            population,
            marginals_available,
            rows,
        } = self;
        let population = population.restrict(selected.clone())?;
        let rows = rows
            .into_iter()
            .filter(|row| selected.binary_search(&row.episode_id()).is_ok())
            .collect();
        Self::new(vocabulary, sources, population, marginals_available, rows)
    }
}

/// Exact per-label counts and answered-only metrics.
#[derive(Debug)]
pub(crate) struct MultiLabelLabelMetrics {
    pub(crate) label: String,
    pub(crate) support: u64,
    pub(crate) answered_support: u64,
    pub(crate) predicted_support: u64,
    pub(crate) true_positive: u64,
    pub(crate) false_positive: u64,
    pub(crate) false_negative: u64,
    pub(crate) true_negative: u64,
    pub(crate) precision: MetricResult,
    pub(crate) recall: MetricResult,
    pub(crate) f1: MetricResult,
}

/// The zero-filled macro F1 and labels whose F1 terms were undefined.
#[derive(Debug)]
pub(crate) struct MultiLabelMacroF1 {
    pub(crate) metric: MetricResult,
    pub(crate) undefined_classes: Vec<String>,
}

/// Complete hard-decision accounting for one outcome family.
#[derive(Debug)]
pub(crate) struct MultiLabelHardResults {
    pub(crate) total: u64,
    pub(crate) abstained: u64,
    pub(crate) answered: u64,
    pub(crate) exact_matches: u64,
    pub(crate) wrong_sets: u64,
    pub(crate) labels: Vec<MultiLabelLabelMetrics>,
    pub(crate) exact_match_accuracy: MetricResult,
    pub(crate) wrong_set_rate: MetricResult,
    pub(crate) coverage: MetricResult,
    pub(crate) abstention_rate: MetricResult,
    pub(crate) selective_exact_match_accuracy: MetricResult,
    pub(crate) selective_risk: MetricResult,
    pub(crate) answered_micro_precision: MetricResult,
    pub(crate) answered_micro_recall: MetricResult,
    pub(crate) answered_micro_f1: MetricResult,
    pub(crate) answered_macro_f1: MultiLabelMacroF1,
    pub(crate) answered_hamming_loss: MetricResult,
}

/// Whether a raw or final multi-label episode outcome was answered.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MultiLabelEpisodeStatus {
    Answered,
    Abstained,
}

/// Privacy-safe evidence for one concrete hard-decision outcome family.
#[derive(Debug, PartialEq)]
pub(crate) struct MultiLabelOutcomeEvidence {
    pub(crate) status: MultiLabelEpisodeStatus,
    pub(crate) predicted: Option<Vec<String>>,
    pub(crate) matched: Option<Vec<String>>,
    pub(crate) missed: Option<Vec<String>>,
    pub(crate) extra: Option<Vec<String>>,
    pub(crate) correct: Option<bool>,
}

/// Exact selected-episode evidence for raw and final multi-label outcomes.
#[derive(Debug)]
pub(crate) struct MultiLabelEpisodeEvidence {
    pub(crate) episode_id: EpisodeId,
    pub(crate) expected: Vec<String>,
    pub(crate) raw: MultiLabelOutcomeEvidence,
    pub(crate) final_outcome: MultiLabelOutcomeEvidence,
    pub(crate) raw_abstention_reason: Option<String>,
    pub(crate) source_id: SourceId,
    pub(crate) observations: ObservationSet,
    pub(crate) marginals: Option<Vec<f64>>,
}

/// Complete raw and final hard results plus selected episode evidence.
#[derive(Debug)]
pub(crate) struct MultiLabelResults {
    pub(crate) raw: MultiLabelHardResults,
    pub(crate) final_results: MultiLabelHardResults,
    pub(crate) probability: MultiLabelProbabilityResults,
    pub(crate) signals: MultiLabelSignalDiagnostics,
    pub(crate) episodes: Vec<MultiLabelEpisodeEvidence>,
}

/// Typed paired comparison of two verified multi-label reports.
#[derive(Serialize)]
pub(crate) struct MultiLabelComparison {
    pub(crate) schema_version: u32,
    pub(crate) kind: &'static str,
    pub(crate) status: &'static str,
    pub(crate) identity: crate::model::single_label::ComparisonIdentity,
    pub(crate) baseline: crate::model::single_label::ComparisonSide,
    pub(crate) candidate: crate::model::single_label::ComparisonSide,
    pub(crate) scope: &'static str,
    pub(crate) task: crate::model::single_label::ComparisonTask,
    pub(crate) population: crate::model::single_label::ComparisonPopulation,
    pub(crate) configuration_differences: Vec<crate::model::single_label::ComparisonDifference>,
    pub(crate) raw: MultiLabelComparisonHardResults,
    #[serde(rename = "final")]
    pub(crate) final_results: MultiLabelComparisonHardResults,
    pub(crate) probability: MultiLabelComparisonProbabilityResults,
    pub(crate) transitions: MultiLabelTransitions,
    pub(crate) episodes: Vec<MultiLabelComparisonEpisode>,
}

#[derive(Serialize)]
pub(crate) struct MultiLabelComparisonHardResults {
    pub(crate) baseline_total: u64,
    pub(crate) candidate_total: u64,
    pub(crate) baseline_abstained: u64,
    pub(crate) candidate_abstained: u64,
    pub(crate) baseline_answered: u64,
    pub(crate) candidate_answered: u64,
    pub(crate) baseline_exact_matches: u64,
    pub(crate) candidate_exact_matches: u64,
    pub(crate) baseline_wrong_sets: u64,
    pub(crate) candidate_wrong_sets: u64,
    pub(crate) exact_match_accuracy: crate::model::single_label::ComparisonMetricPair,
    pub(crate) wrong_set_rate: crate::model::single_label::ComparisonMetricPair,
    pub(crate) abstention_rate: crate::model::single_label::ComparisonMetricPair,
    pub(crate) coverage: crate::model::single_label::ComparisonMetricPair,
    pub(crate) selective_exact_match_accuracy: crate::model::single_label::ComparisonMetricPair,
    pub(crate) selective_risk: crate::model::single_label::ComparisonMetricPair,
    pub(crate) answered_micro_precision: crate::model::single_label::ComparisonMetricPair,
    pub(crate) answered_micro_recall: crate::model::single_label::ComparisonMetricPair,
    pub(crate) answered_micro_f1: crate::model::single_label::ComparisonMetricPair,
    pub(crate) answered_macro_f1: MultiLabelComparisonMacroF1,
    pub(crate) answered_hamming_loss: crate::model::single_label::ComparisonMetricPair,
    pub(crate) labels: Vec<MultiLabelComparisonLabelMetrics>,
}

#[derive(Serialize)]
pub(crate) struct MultiLabelComparisonLabelMetrics {
    pub(crate) label: String,
    pub(crate) baseline_support: u64,
    pub(crate) candidate_support: u64,
    pub(crate) baseline_answered_support: u64,
    pub(crate) candidate_answered_support: u64,
    pub(crate) baseline_predicted_support: u64,
    pub(crate) candidate_predicted_support: u64,
    pub(crate) baseline_true_positive: u64,
    pub(crate) candidate_true_positive: u64,
    pub(crate) baseline_false_positive: u64,
    pub(crate) candidate_false_positive: u64,
    pub(crate) baseline_false_negative: u64,
    pub(crate) candidate_false_negative: u64,
    pub(crate) baseline_true_negative: u64,
    pub(crate) candidate_true_negative: u64,
    pub(crate) precision: crate::model::single_label::ComparisonMetricPair,
    pub(crate) recall: crate::model::single_label::ComparisonMetricPair,
    pub(crate) f1: crate::model::single_label::ComparisonMetricPair,
}

#[derive(Serialize)]
pub(crate) struct MultiLabelComparisonMacroF1 {
    pub(crate) metric: crate::model::single_label::ComparisonMetricPair,
    pub(crate) baseline_undefined_classes: Vec<String>,
    pub(crate) candidate_undefined_classes: Vec<String>,
}

#[derive(Serialize)]
pub(crate) struct MultiLabelComparisonProbabilityResults {
    pub(crate) labels: Vec<MultiLabelComparisonProbabilityLabel>,
    pub(crate) mean_binary_log_loss: crate::model::single_label::ComparisonMetricPair,
    pub(crate) mean_binary_brier: crate::model::single_label::ComparisonMetricPair,
}

#[derive(Serialize)]
pub(crate) struct MultiLabelComparisonProbabilityLabel {
    pub(crate) label: String,
    pub(crate) binary_log_loss: crate::model::single_label::ComparisonMetricPair,
    pub(crate) binary_brier: crate::model::single_label::ComparisonMetricPair,
}

#[derive(Serialize)]
pub(crate) struct MultiLabelTransitions {
    pub(crate) both_correct: crate::model::single_label::ComparisonCategory,
    pub(crate) recovered: crate::model::single_label::ComparisonCategory,
    pub(crate) regressed: crate::model::single_label::ComparisonCategory,
    pub(crate) neither_correct: crate::model::single_label::ComparisonCategory,
    pub(crate) changed_final_outcomes: crate::model::single_label::ComparisonCategory,
    pub(crate) per_label: Vec<MultiLabelTransitionTable>,
}

#[derive(Serialize)]
pub(crate) struct MultiLabelTransitionTable {
    pub(crate) label: String,
    pub(crate) transitions: Vec<MultiLabelFinalOutcomeTransition>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum MultiLabelFinalOutcomeState {
    Absent,
    Present,
    Abstained,
}

#[derive(Serialize)]
pub(crate) struct MultiLabelFinalOutcomeTransition {
    pub(crate) baseline: MultiLabelFinalOutcomeState,
    pub(crate) candidate: MultiLabelFinalOutcomeState,
    pub(crate) count: u64,
}

#[derive(Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum MultiLabelComparisonOutcome {
    Labels { labels: Vec<String> },
    Abstention,
}

#[derive(Serialize)]
pub(crate) struct MultiLabelComparisonEpisode {
    pub(crate) id: String,
    pub(crate) baseline_source_id: String,
    pub(crate) candidate_source_id: String,
    pub(crate) baseline_final_outcome: MultiLabelComparisonOutcome,
    pub(crate) candidate_final_outcome: MultiLabelComparisonOutcome,
    pub(crate) baseline_correct: bool,
    pub(crate) candidate_correct: bool,
    pub(crate) baseline_matched: Option<Vec<String>>,
    pub(crate) candidate_matched: Option<Vec<String>>,
    pub(crate) baseline_missed: Option<Vec<String>>,
    pub(crate) candidate_missed: Option<Vec<String>>,
    pub(crate) baseline_extra: Option<Vec<String>>,
    pub(crate) candidate_extra: Option<Vec<String>>,
}

/// Marginal probability scores over the complete selected population.
#[derive(Debug)]
pub(crate) struct MultiLabelProbabilityResults {
    pub(crate) labels: Vec<MultiLabelProbabilityLabel>,
    pub(crate) mean_binary_log_loss: MetricResult,
    pub(crate) mean_binary_brier: MetricResult,
}

/// One label's independent binary probability scores.
#[derive(Debug)]
pub(crate) struct MultiLabelProbabilityLabel {
    pub(crate) label: String,
    pub(crate) binary_log_loss: MetricResult,
    pub(crate) binary_brier: MetricResult,
}

/// Ten fixed marginal-probability bins for each vocabulary label.
#[derive(Debug)]
pub(crate) struct MultiLabelSignalDiagnostics {
    pub(crate) status: MetricStatus,
    pub(crate) labels: Vec<MultiLabelLabelBins>,
}

/// Marginal bins for one label.
#[derive(Debug)]
pub(crate) struct MultiLabelLabelBins {
    pub(crate) label: String,
    pub(crate) population_count: u64,
    pub(crate) population_scope: crate::model::common::MetricScope,
    pub(crate) bins: Vec<MultiLabelSignalBin>,
}

/// A fixed-width marginal probability bin.
#[derive(Debug)]
pub(crate) struct MultiLabelSignalBin {
    pub(crate) index: u8,
    pub(crate) lower: f64,
    pub(crate) upper: f64,
    pub(crate) upper_inclusive: bool,
    pub(crate) count: u64,
    pub(crate) positive_count: u64,
    pub(crate) mean_probability: Option<f64>,
    pub(crate) observed_positive_rate: Option<f64>,
}

pub(crate) fn assemble_report(
    run_id: crate::model::common::RunId,
    created_at: String,
    validator_version: String,
    evaluation: &MultiLabelEvaluation,
    results: &MultiLabelResults,
    snapshots: (&ArtifactSnapshot, &ArtifactSnapshot, &ArtifactSnapshot),
    evidence: &[crate::model::common::EvidenceBinding],
) -> Result<Value> {
    let (golden, predictions, config) = snapshots;
    let artifact = |kind: &str, snapshot: &crate::model::common::ArtifactSnapshot| json!({"kind": kind, "path": snapshot.stored_path(), "sha256": snapshot.digest().to_string()});
    let mut manifest = vec![
        artifact("golden", golden),
        artifact("predictions", predictions),
        artifact("config", config),
    ];
    for binding in evidence {
        manifest.push(json!({
            "kind": "evidence",
            "path": binding.stored_path(),
            "sha256": binding.digest().to_string(),
            "source_id": binding.source_id().as_str(),
            "evidence_index": binding.evidence_index(),
            "original_path": binding.original_path(),
        }));
    }
    let mut source_counts = evaluation
        .sources()
        .keys()
        .map(|id| (id.as_str().to_owned(), 0_u64))
        .collect::<BTreeMap<_, _>>();
    for episode in &results.episodes {
        *source_counts
            .get_mut(episode.source_id.as_str())
            .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))? += 1;
    }
    let mut sources = BTreeMap::new();
    for (id, source) in evaluation.sources() {
        let configuration: Value = serde_json::from_str(source.configuration().get())
            .map_err(|_| Diagnostic::for_code(DiagnosticCode::Invariant))?;
        let observations = source
            .observation_definitions()
            .iter()
            .map(|(name, definition)| {
                (
                    name,
                    json!({
                        "kind": definition.kind(),
                        "description": definition.description(),
                        "question_id": definition.question_id(),
                    }),
                )
            })
            .collect::<BTreeMap<_, _>>();
        let bound = evidence
            .iter()
            .filter(|binding| binding.source_id() == id)
            .map(|binding| binding.stored_path().display().to_string())
            .collect::<Vec<_>>();
        let preparation = source
            .preparation()
            .map(|value| {
                let configuration: Value = serde_json::from_str(value.configuration().get())
                    .map_err(|_| Diagnostic::for_code(DiagnosticCode::Invariant))?;
                Ok(json!({
                    "method": value.method(),
                    "version": value.version(),
                    "configuration": configuration,
                    "evidence_indices": value.evidence_indices(),
                }))
            })
            .transpose()?;
        let mut value = json!({
            "kind": source.kind(),
            "model": source.model(),
            "configuration": configuration,
            "evidence": bound,
            "observations": observations,
        });
        if let Some(question_id) = source.question_id() {
            value["question_id"] = json!(question_id);
        }
        if let Some(preparation) = preparation {
            value["preparation"] = preparation;
        }
        sources.insert(id.as_str().to_owned(), value);
    }
    let hard = |hard: &MultiLabelHardResults| -> Value {
        json!({
            "kind": "multi_label",
            "total": hard.total,
            "abstained": hard.abstained,
            "answered": hard.answered,
            "exact_matches": hard.exact_matches,
            "wrong_sets": hard.wrong_sets,
            "labels": hard.labels.iter().map(|label| json!({
                "label": label.label,
                "support": label.support,
                "answered_support": label.answered_support,
                "predicted_support": label.predicted_support,
                "true_positive": label.true_positive,
                "false_positive": label.false_positive,
                "false_negative": label.false_negative,
                "true_negative": label.true_negative,
                "precision": label.precision,
                "recall": label.recall,
                "f1": label.f1,
            })).collect::<Vec<_>>(),
            "exact_match_accuracy": hard.exact_match_accuracy,
            "wrong_set_rate": hard.wrong_set_rate,
            "coverage": hard.coverage,
            "abstention_rate": hard.abstention_rate,
            "selective_exact_match_accuracy": hard.selective_exact_match_accuracy,
            "selective_risk": hard.selective_risk,
            "answered_micro_precision": hard.answered_micro_precision,
            "answered_micro_recall": hard.answered_micro_recall,
            "answered_micro_f1": hard.answered_micro_f1,
            "answered_macro_f1": {"metric": hard.answered_macro_f1.metric, "undefined_classes": hard.answered_macro_f1.undefined_classes},
            "answered_hamming_loss": hard.answered_hamming_loss,
        })
    };
    let probability = json!({
        "kind": "multi_label",
        "labels": results.probability.labels.iter().map(|label| json!({
            "label": label.label,
            "binary_log_loss": label.binary_log_loss,
            "binary_brier": label.binary_brier,
        })).collect::<Vec<_>>(),
        "mean_binary_log_loss": results.probability.mean_binary_log_loss,
        "mean_binary_brier": results.probability.mean_binary_brier,
    });
    let signals = json!({
        "kind": "multi_label",
        "status": results.signals.status,
        "labels": results.signals.labels.iter().map(|label| json!({
            "label": label.label,
            "population_count": label.population_count,
            "population_scope": label.population_scope,
            "bins": label.bins.iter().map(|bin| json!({
                "index": bin.index,
                "lower": bin.lower,
                "upper": bin.upper,
                "upper_inclusive": bin.upper_inclusive,
                "count": bin.count,
                "positive_count": bin.positive_count,
                "mean_probability": bin.mean_probability,
                "observed_positive_rate": bin.observed_positive_rate,
            })).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
    });
    let episodes = results.episodes.iter().map(|episode| {
        let outcome = |outcome: &MultiLabelOutcomeEvidence, reason: Option<&str>| match outcome.status {
            MultiLabelEpisodeStatus::Answered => json!({
                "type": "labels", "labels": outcome.predicted,
                "matched": outcome.matched, "missed": outcome.missed,
                "extra": outcome.extra, "correct": outcome.correct,
            }),
            MultiLabelEpisodeStatus::Abstained => json!({
                "type": "abstention",
                "reason": reason,
                "status": "abstained",
                "matched": Value::Null,
                "missed": Value::Null,
                "extra": Value::Null,
            }),
        };
        let marginals = episode.marginals.as_ref().map(|values| {
            evaluation.vocabulary().labels().zip(values).collect::<BTreeMap<_, _>>()
        });
        json!({
            "id": episode.episode_id.to_string(),
            "source_id": episode.source_id.as_str(),
            "expected": {"type": "labels", "labels": episode.expected},
            "raw_outcome": outcome(&episode.raw, episode.raw_abstention_reason.as_deref()),
            "final_outcome": outcome(&episode.final_outcome, episode.raw_abstention_reason.as_deref()),
            "raw_correct": episode.raw.correct.unwrap_or(false),
            "final_correct": episode.final_outcome.correct.unwrap_or(false),
            "rejection_reason": Value::Null,
            "observations": episode.observations.report_values(),
            "probability": marginals.map(|values| json!({"marginals": values})),
        })
    }).collect::<Vec<_>>();
    let population = evaluation.population();
    Ok(json!({
        "schema_version": crate::WIRE_VERSION,
        "kind": "evaluation",
        "status": "complete",
        "identity": {"run_id": run_id.to_string(), "created_at": created_at, "validator_version": validator_version, "specification_version": crate::SPECIFICATION_VERSION, "parent_run_id": population.config().parent_run_id().map(|id| id.to_string())},
        "artifacts": manifest,
        "sources": sources,
        "composition": match source_counts.values().filter(|count| **count > 0).count() { 0 => "empty", 1 => "single_source", _ => "mixed_source" },
        "source_counts": source_counts,
        "population": {"description": population.config().description(), "role": match population.config().role() { crate::model::common::EvaluationRole::Development => "development", crate::model::common::EvaluationRole::HeldOut => "held_out" }, "dataset_count": population.dataset_count(), "selected_count": population.selected_count(), "selected_ids": population.selected().iter().map(ToString::to_string).collect::<Vec<_>>(), "unselected_ids": population.unselected().iter().map(ToString::to_string).collect::<Vec<_>>()},
        "task": {"kind": "multi_label", "labels": evaluation.vocabulary().labels().collect::<Vec<_>>()},
        "policy": {"decision": match population.config().policy() {
            MultiLabelPolicy::AsRecorded => json!({"type": "as_recorded"}),
            MultiLabelPolicy::LabelThresholds { thresholds } => json!({"type": "label_thresholds", "thresholds": thresholds}),
        }, "bin_count": crate::TEN_BIN_COUNT},
        "integrity": {"source_count": evaluation.sources().len(), "selected_count": population.selected_count(), "prediction_count": results.episodes.len(), "missing_ids": [], "extra_ids": [], "signal_availability": if evaluation.marginals_available() { "marginals" } else { "none" }},
        "raw": hard(&results.raw),
        "final": hard(&results.final_results),
        "probability": probability,
        "signals": signals,
        "episodes": episodes,
    }))
}
