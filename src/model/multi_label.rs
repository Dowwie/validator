use std::collections::{BTreeMap, HashMap};

use crate::model::common::{
    ArtifactSnapshot, Episode, EpisodeId, LabelSet, LabelVocabulary, MetricResult, MetricStatus,
    ObservationSet, Outcome, Population, Prediction, SourceDefinition, SourceId,
};
use crate::{Diagnostic, DiagnosticCode, Result};
use serde::Serialize;

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
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
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
}

/// Exact per-label counts and answered-only metrics.
#[derive(Debug, Serialize)]
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
#[derive(Debug, Serialize)]
pub(crate) struct MultiLabelMacroF1 {
    pub(crate) metric: MetricResult,
    pub(crate) undefined_classes: Vec<String>,
}

/// Complete hard-decision accounting for one outcome family.
#[derive(Debug, Serialize)]
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

/// Marginal probability scores over the complete selected population.
#[derive(Debug, Serialize)]
pub(crate) struct MultiLabelProbabilityResults {
    pub(crate) labels: Vec<MultiLabelProbabilityLabel>,
    pub(crate) mean_binary_log_loss: MetricResult,
    pub(crate) mean_binary_brier: MetricResult,
}

/// One label's independent binary probability scores.
#[derive(Debug, Serialize)]
pub(crate) struct MultiLabelProbabilityLabel {
    pub(crate) label: String,
    pub(crate) binary_log_loss: MetricResult,
    pub(crate) binary_brier: MetricResult,
}

/// Ten fixed marginal-probability bins for each vocabulary label.
#[derive(Debug, Serialize)]
pub(crate) struct MultiLabelSignalDiagnostics {
    pub(crate) status: MetricStatus,
    pub(crate) labels: Vec<MultiLabelLabelBins>,
}

/// Marginal bins for one label.
#[derive(Debug, Serialize)]
pub(crate) struct MultiLabelLabelBins {
    pub(crate) label: String,
    pub(crate) population_count: u64,
    pub(crate) population_scope: crate::model::common::MetricScope,
    pub(crate) bins: Vec<MultiLabelSignalBin>,
}

/// A fixed-width marginal probability bin.
#[derive(Debug, Serialize)]
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

/// A concrete multi-label report with raw opaque metadata and typed metric fields.
#[derive(Serialize)]
pub(crate) struct MultiLabelRunReport<'a> {
    #[serde(flatten)]
    context: super::report::ReportContext,
    policy: ReportPolicy<'a>,
    integrity: ReportIntegrity,
    raw: ReportPart<'a, MultiLabelHardResults>,
    #[serde(rename = "final")]
    final_results: ReportPart<'a, MultiLabelHardResults>,
    probability: ReportPart<'a, MultiLabelProbabilityResults>,
    signals: ReportPart<'a, MultiLabelSignalDiagnostics>,
    episodes: Vec<ReportEpisode<'a>>,
}

#[derive(Serialize)]
struct ReportPart<'a, T> {
    kind: &'static str,
    #[serde(flatten)]
    data: &'a T,
}

impl<'a, T> ReportPart<'a, T> {
    fn new(data: &'a T) -> Self {
        Self {
            kind: "multi_label",
            data,
        }
    }
}

#[derive(Serialize)]
struct ReportPolicy<'a> {
    decision: &'a MultiLabelPolicy,
    bin_count: usize,
}

#[derive(Serialize)]
struct ReportIntegrity {
    source_count: usize,
    selected_count: usize,
    prediction_count: usize,
    missing_ids: Vec<String>,
    extra_ids: Vec<String>,
    signal_availability: &'static str,
    categorical_normalization: MetricStatus,
}

#[derive(Serialize)]
struct ReportEpisode<'a> {
    id: String,
    source_id: &'a str,
    expected: ReportLabelSet<'a>,
    raw_outcome: ReportOutcome<'a>,
    final_outcome: ReportOutcome<'a>,
    raw_correct: bool,
    final_correct: bool,
    rejection_reason: Option<&'a str>,
    observations: BTreeMap<String, super::common::ReportObservation>,
    probability: Option<ReportMarginals<'a>>,
}

#[derive(Serialize)]
struct ReportLabelSet<'a> {
    #[serde(rename = "type")]
    kind: &'static str,
    labels: &'a [String],
}

#[derive(Serialize)]
struct ReportMarginals<'a> {
    marginals: BTreeMap<&'a str, &'a f64>,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ReportOutcome<'a> {
    Labels {
        labels: &'a [String],
        matched: &'a [String],
        missed: &'a [String],
        extra: &'a [String],
        correct: bool,
    },
    Abstention {
        reason: Option<&'a str>,
        status: &'static str,
        matched: Option<&'a [String]>,
        missed: Option<&'a [String]>,
        extra: Option<&'a [String]>,
    },
}

impl<'a> ReportOutcome<'a> {
    fn new(outcome: &'a MultiLabelOutcomeEvidence, reason: Option<&'a str>) -> Result<Self> {
        let invalid = || Diagnostic::for_code(DiagnosticCode::Invariant);
        Ok(match outcome.status {
            MultiLabelEpisodeStatus::Answered => Self::Labels {
                labels: outcome.predicted.as_deref().ok_or_else(invalid)?,
                matched: outcome.matched.as_deref().ok_or_else(invalid)?,
                missed: outcome.missed.as_deref().ok_or_else(invalid)?,
                extra: outcome.extra.as_deref().ok_or_else(invalid)?,
                correct: outcome.correct.ok_or_else(invalid)?,
            },
            MultiLabelEpisodeStatus::Abstained => Self::Abstention {
                reason,
                status: "abstained",
                matched: None,
                missed: None,
                extra: None,
            },
        })
    }
}

pub(crate) fn assemble_report<'a>(
    run_id: crate::model::common::RunId,
    created_at: String,
    validator_version: String,
    evaluation: &'a MultiLabelEvaluation,
    results: &'a MultiLabelResults,
    snapshots: (&ArtifactSnapshot, &ArtifactSnapshot, &ArtifactSnapshot),
    evidence: &[crate::model::common::EvidenceBinding],
) -> Result<MultiLabelRunReport<'a>> {
    let population = evaluation.population();
    let episodes = results
        .episodes
        .iter()
        .map(|episode| {
            let reason = episode.raw_abstention_reason.as_deref();
            Ok(ReportEpisode {
                id: episode.episode_id.to_string(),
                source_id: episode.source_id.as_str(),
                expected: ReportLabelSet {
                    kind: "labels",
                    labels: &episode.expected,
                },
                raw_outcome: ReportOutcome::new(&episode.raw, reason)?,
                final_outcome: ReportOutcome::new(&episode.final_outcome, reason)?,
                raw_correct: episode.raw.correct.unwrap_or(false),
                final_correct: episode.final_outcome.correct.unwrap_or(false),
                rejection_reason: None,
                observations: episode.observations.report_values(),
                probability: episode.marginals.as_ref().map(|values| ReportMarginals {
                    marginals: evaluation.vocabulary().labels().zip(values).collect(),
                }),
            })
        })
        .collect::<Result<_>>()?;
    Ok(MultiLabelRunReport {
        context: super::report::ReportContext::new(
            (run_id, created_at, validator_version),
            ("multi_label", evaluation.vocabulary()),
            population,
            evaluation.sources(),
            results.episodes.iter().map(|episode| &episode.source_id),
            snapshots,
            evidence,
        )?,
        policy: ReportPolicy {
            decision: population.config().policy(),
            bin_count: crate::TEN_BIN_COUNT,
        },
        integrity: ReportIntegrity {
            source_count: evaluation.sources().len(),
            selected_count: population.selected_count(),
            prediction_count: results.episodes.len(),
            missing_ids: Vec::new(),
            extra_ids: Vec::new(),
            signal_availability: if evaluation.marginals_available() {
                "marginals"
            } else {
                "none"
            },
            categorical_normalization: MetricStatus::NotApplicable,
        },
        raw: ReportPart::new(&results.raw),
        final_results: ReportPart::new(&results.final_results),
        probability: ReportPart::new(&results.probability),
        signals: ReportPart::new(&results.signals),
        episodes,
    })
}
