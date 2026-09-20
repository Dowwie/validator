use std::collections::{BTreeMap, HashMap};

use serde::Serialize;
use serde_json::{Value, value::RawValue};

use crate::model::common::{
    ArtifactSnapshot, Episode, EpisodeId, LabelIndex, LabelVocabulary, MetricResult,
    ObservationSet, Outcome, Population, Prediction, SourceDefinition, SourceId,
};
use crate::{Diagnostic, DiagnosticCode, Result};

/// A checked categorical distribution with submitted and normalized values.
#[derive(Debug)]
pub struct CategoricalDistribution {
    submitted: Vec<f64>,
    working: Vec<f64>,
}

impl CategoricalDistribution {
    /// Validates exact vocabulary coverage and near-unit categorical mass.
    pub fn new(values: &HashMap<String, f64>, vocabulary: &LabelVocabulary) -> Result<Self> {
        if values.len() != vocabulary.len() {
            return Err(Diagnostic::for_code(DiagnosticCode::Probability));
        }
        let mut submitted = Vec::with_capacity(vocabulary.len());
        for label in vocabulary.labels() {
            let Some(value) = values.get(label) else {
                return Err(Diagnostic::for_code(DiagnosticCode::Probability));
            };
            if !value.is_finite() || !(0.0..=1.0).contains(value) {
                return Err(Diagnostic::for_code(DiagnosticCode::Probability));
            }
            submitted.push(*value);
        }
        let sum: f64 = submitted.iter().sum();
        if !sum.is_finite() || sum <= 0.0 || (sum - 1.0).abs() > crate::CATEGORICAL_SUM_TOLERANCE {
            return Err(Diagnostic::for_code(DiagnosticCode::Probability));
        }
        let working = submitted.iter().map(|value| value / sum).collect();
        Ok(Self { submitted, working })
    }
    /// Borrows submitted values in vocabulary order.
    pub fn submitted(&self) -> &[f64] {
        &self.submitted
    }
    /// Borrows normalized working values in vocabulary order.
    pub fn working(&self) -> &[f64] {
        &self.working
    }
}

/// A checked reported confidence scalar.
#[derive(Clone, Copy, Debug)]
pub struct ReportedConfidence(f64);

impl ReportedConfidence {
    /// Validates a finite confidence in the unit interval.
    pub fn new(value: f64) -> Result<Self> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(Diagnostic::for_code(DiagnosticCode::Confidence));
        }
        Ok(Self(value))
    }
    /// Returns the checked scalar.
    pub const fn value(self) -> f64 {
        self.0
    }
}

/// A single-label output retained as recorded with optional scoring signals.
#[derive(Debug)]
pub struct SingleLabelOutput {
    outcome: Outcome<LabelIndex>,
    categorical: Option<CategoricalDistribution>,
    confidence: Option<ReportedConfidence>,
}

impl SingleLabelOutput {
    /// Creates a checked as-recorded output.
    pub fn new(
        outcome: Outcome<LabelIndex>,
        categorical: Option<CategoricalDistribution>,
        confidence: Option<ReportedConfidence>,
    ) -> Self {
        Self {
            outcome,
            categorical,
            confidence,
        }
    }
    /// Borrows the outcome.
    pub fn outcome(&self) -> &Outcome<LabelIndex> {
        &self.outcome
    }
    /// Returns whether categorical evidence exists.
    pub fn has_categorical(&self) -> bool {
        self.categorical.is_some()
    }
    /// Returns whether confidence evidence exists.
    pub fn has_confidence(&self) -> bool {
        self.confidence.is_some()
    }
    /// Borrows categorical evidence.
    pub fn categorical(&self) -> Option<&CategoricalDistribution> {
        self.categorical.as_ref()
    }

    pub(crate) fn confidence(&self) -> Option<ReportedConfidence> {
        self.confidence
    }
}

/// Checks the artifact invariant before checked outputs are moved into rows.
pub(crate) fn signal_availability_flags(signals: &[(bool, bool)]) -> Result<SignalAvailability> {
    let (categorical, confidence) = signals.first().copied().unwrap_or((false, false));
    if signals
        .iter()
        .any(|signal| *signal != (categorical, confidence))
    {
        return Err(Diagnostic::for_code(DiagnosticCode::Probability));
    }
    Ok(match (categorical, confidence) {
        (false, false) => SignalAvailability::None,
        (true, false) => SignalAvailability::Categorical,
        (false, true) => SignalAvailability::Confidence,
        (true, true) => SignalAvailability::Both,
    })
}

/// Admission diagnostics for submitted categorical probability vectors.
pub(crate) struct NormalizationDiagnostics {
    pub(crate) normalized_count: u64,
    pub(crate) maximum_sum_error: f64,
}

/// Derives categorical normalization diagnostics without scoring an evaluation.
pub(crate) fn normalization_diagnostics(
    evaluation: &SingleLabelEvaluation,
) -> Result<NormalizationDiagnostics> {
    let mut normalized_count = 0_usize;
    let mut maximum_sum_error = 0.0_f64;
    for row in evaluation.rows() {
        let Some(distribution) = row.prediction().output().categorical() else {
            continue;
        };
        if distribution.submitted() != distribution.working() {
            normalized_count = normalized_count
                .checked_add(1)
                .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Numeric))?;
        }
        maximum_sum_error =
            maximum_sum_error.max((distribution.submitted().iter().sum::<f64>() - 1.0).abs());
    }
    Ok(NormalizationDiagnostics {
        normalized_count: u64::try_from(normalized_count)
            .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?,
        maximum_sum_error,
    })
}

/// Validates the extra requirements for a scored-choice output.
pub fn validate_scored_choice(
    source: &SourceDefinition,
    output: &SingleLabelOutput,
    vocabulary: &LabelVocabulary,
) -> Result<()> {
    if source.kind() != crate::model::common::SourceKind::ScoredChoice
        || source.question_id().is_none()
    {
        return Err(Diagnostic::for_code(DiagnosticCode::Provenance));
    }
    let Some(choice) = output.outcome().answered_target() else {
        return Err(Diagnostic::for_code(DiagnosticCode::Config));
    };
    let (Some(distribution), true) = (output.categorical(), output.has_confidence()) else {
        return Err(Diagnostic::for_code(DiagnosticCode::Config));
    };
    let Some(position) = vocabulary.position(choice) else {
        return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
    };
    let maximum = distribution
        .working()
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    if distribution.working()[position] != maximum {
        return Err(Diagnostic::for_code(DiagnosticCode::Config));
    }
    Ok(())
}

/// Artifact-wide scoring signal completeness.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalAvailability {
    None,
    Categorical,
    Confidence,
    Both,
}

impl SignalAvailability {
    pub(crate) const fn has_categorical(self) -> bool {
        matches!(self, Self::Categorical | Self::Both)
    }

    pub(crate) const fn has_confidence(self) -> bool {
        matches!(self, Self::Confidence | Self::Both)
    }
}

/// A checked scoring signal used by single-label rejection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SingleLabelRejectionSignal {
    /// Use the recorded reported-confidence value.
    Confidence,
    /// Use the maximum of the normalized categorical distribution.
    MaxProbability,
}

/// The legal task-specific decision policies for single-label evaluation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SingleLabelPolicy {
    /// Keep each submitted class or abstention unchanged.
    AsRecorded,
    /// Replace answered classes below the configured signal with abstention.
    RejectBelow {
        /// The checked signal used for the comparison.
        signal: SingleLabelRejectionSignal,
        /// The inclusive minimum accepted signal.
        minimum: f64,
    },
}

/// A matrix column that cannot confuse a declared class with abstention.
#[derive(Clone, Debug)]
pub(crate) enum SingleLabelMatrixColumn {
    Label(LabelIndex),
    Abstention,
}

/// A K by K+1 hard-decision matrix with typed columns.
#[derive(Debug)]
pub(crate) struct SingleLabelMatrix {
    pub(crate) columns: Vec<SingleLabelMatrixColumn>,
    pub(crate) rows: Vec<Vec<u64>>,
}

/// An answered class or an explicit whole-episode abstention.
#[derive(Clone, Debug)]
pub(crate) enum SingleLabelDecision {
    Label(LabelIndex),
    Abstained,
}

/// Per-class counts and hard-decision metrics.
#[derive(Debug)]
pub(crate) struct SingleLabelClassMetrics {
    pub(crate) label: LabelIndex,
    pub(crate) support: u64,
    pub(crate) predicted_support: u64,
    pub(crate) true_positive: u64,
    pub(crate) false_positive: u64,
    pub(crate) false_negative: u64,
    pub(crate) precision: MetricResult,
    pub(crate) recall: MetricResult,
    pub(crate) f1: MetricResult,
    pub(crate) coverage: MetricResult,
}

/// The macro F1 result and the classes whose terms were undefined.
#[derive(Debug)]
pub(crate) struct SingleLabelMacroF1 {
    pub(crate) metric: MetricResult,
    pub(crate) undefined_classes: Vec<LabelIndex>,
}

/// Complete hard-decision accounting for one outcome family.
#[derive(Debug)]
pub(crate) struct SingleLabelHardResults {
    pub(crate) total: u64,
    pub(crate) correct: u64,
    pub(crate) wrong: u64,
    pub(crate) abstained: u64,
    pub(crate) answered: u64,
    pub(crate) matrix: SingleLabelMatrix,
    pub(crate) classes: Vec<SingleLabelClassMetrics>,
    pub(crate) accuracy: MetricResult,
    pub(crate) wrong_class_rate: MetricResult,
    pub(crate) abstention_rate: MetricResult,
    pub(crate) coverage: MetricResult,
    pub(crate) selective_accuracy: MetricResult,
    pub(crate) selective_risk: MetricResult,
    pub(crate) macro_f1: SingleLabelMacroF1,
}

/// Privacy-safe episode evidence for hard-decision reporting.
#[derive(Debug)]
pub(crate) struct SingleLabelEpisodeEvidence {
    pub(crate) episode_id: EpisodeId,
    pub(crate) expected: LabelIndex,
    pub(crate) raw_outcome: SingleLabelDecision,
    pub(crate) final_outcome: SingleLabelDecision,
    pub(crate) raw_abstention_reason: Option<String>,
    pub(crate) raw_correct: bool,
    pub(crate) final_correct: bool,
    pub(crate) rejection_reason: Option<&'static str>,
    pub(crate) source_id: SourceId,
    pub(crate) observations: ObservationSet,
    pub(crate) probability: Option<SingleLabelProbabilityEvidence>,
    pub(crate) reported_confidence: Option<f64>,
}

/// Complete raw and final hard-decision results plus sorted episode evidence.
#[derive(Debug)]
pub(crate) struct SingleLabelResults {
    pub(crate) raw: SingleLabelHardResults,
    pub(crate) final_results: SingleLabelHardResults,
    pub(crate) probability: SingleLabelProbabilityResults,
    pub(crate) signals: SingleLabelSignalDiagnostics,
    pub(crate) episodes: Vec<SingleLabelEpisodeEvidence>,
}

/// A closed, serializable single-label evaluation report.
#[derive(Serialize)]
pub(crate) struct SingleLabelRunReport {
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
    policy: ReportPolicy,
    integrity: ReportIntegrity,
    raw: ReportHardResults,
    #[serde(rename = "final")]
    final_results: ReportHardResults,
    probability: ReportProbability,
    signals: ReportSignals,
    episodes: Vec<ReportEpisode>,
}

/// Typed, serializable comparison of two verified single-label reports.
#[derive(Serialize)]
pub(crate) struct SingleLabelComparison {
    pub(crate) schema_version: u32,
    pub(crate) kind: &'static str,
    pub(crate) status: &'static str,
    pub(crate) identity: ComparisonIdentity,
    pub(crate) baseline: ComparisonSide,
    pub(crate) candidate: ComparisonSide,
    pub(crate) scope: &'static str,
    pub(crate) task: ComparisonTask,
    pub(crate) population: ComparisonPopulation,
    pub(crate) configuration_differences: Vec<ComparisonDifference>,
    pub(crate) raw: ComparisonHardResults,
    #[serde(rename = "final")]
    pub(crate) final_results: ComparisonHardResults,
    pub(crate) probability: ComparisonProbabilityResults,
    pub(crate) transitions: SingleLabelTransitions,
    pub(crate) episodes: Vec<SingleLabelComparisonEpisode>,
}

#[derive(Serialize)]
pub(crate) struct ComparisonIdentity {
    pub(crate) comparison_id: String,
    pub(crate) created_at: String,
    pub(crate) validator_version: String,
    pub(crate) specification_version: String,
}

#[derive(Serialize)]
pub(crate) struct ComparisonSide {
    pub(crate) run_id: String,
    pub(crate) report_sha256: String,
    pub(crate) sources: Value,
    pub(crate) source_counts: Value,
    pub(crate) composition: String,
}

#[derive(Serialize)]
pub(crate) struct ComparisonTask {
    pub(crate) kind: &'static str,
    pub(crate) labels: Vec<String>,
}

#[derive(Serialize)]
pub(crate) struct ComparisonPopulation {
    pub(crate) compared_ids: Vec<String>,
    pub(crate) compared_count: u64,
    pub(crate) baseline_excluded_ids: Vec<String>,
    pub(crate) baseline_excluded_count: u64,
    pub(crate) candidate_excluded_ids: Vec<String>,
    pub(crate) candidate_excluded_count: u64,
}

#[derive(Serialize)]
pub(crate) struct ComparisonDifference {
    pub(crate) field_path: String,
    pub(crate) baseline: Value,
    pub(crate) candidate: Value,
}

#[derive(Serialize)]
pub(crate) struct ComparisonMetricPair {
    pub(crate) direction: MetricDirection,
    pub(crate) baseline: MetricResult,
    pub(crate) candidate: MetricResult,
    pub(crate) delta: Option<f64>,
    pub(crate) delta_reason: Option<MetricDeltaReason>,
    pub(crate) answered_population: Option<AnsweredPopulation>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum MetricDirection {
    HigherIsBetter,
    LowerIsBetter,
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(crate) enum MetricDeltaReason {
    BaselineStatus {
        status: crate::model::common::MetricStatus,
    },
    CandidateStatus {
        status: crate::model::common::MetricStatus,
    },
    NonFiniteDelta,
}

#[derive(Serialize)]
pub(crate) struct ComparisonHardResults {
    pub(crate) baseline_total: u64,
    pub(crate) candidate_total: u64,
    pub(crate) baseline_correct: u64,
    pub(crate) candidate_correct: u64,
    pub(crate) baseline_wrong: u64,
    pub(crate) candidate_wrong: u64,
    pub(crate) baseline_abstained: u64,
    pub(crate) candidate_abstained: u64,
    pub(crate) baseline_answered: u64,
    pub(crate) candidate_answered: u64,
    pub(crate) accuracy: ComparisonMetricPair,
    pub(crate) wrong_class_rate: ComparisonMetricPair,
    pub(crate) abstention_rate: ComparisonMetricPair,
    pub(crate) coverage: ComparisonMetricPair,
    pub(crate) selective_accuracy: ComparisonMetricPair,
    pub(crate) selective_risk: ComparisonMetricPair,
    pub(crate) classes: Vec<ComparisonClassMetrics>,
    pub(crate) macro_f1: ComparisonMacroF1,
}

#[derive(Serialize)]
pub(crate) struct ComparisonClassMetrics {
    pub(crate) label: String,
    pub(crate) baseline_support: u64,
    pub(crate) candidate_support: u64,
    pub(crate) baseline_predicted_support: u64,
    pub(crate) candidate_predicted_support: u64,
    pub(crate) baseline_true_positive: u64,
    pub(crate) candidate_true_positive: u64,
    pub(crate) baseline_false_positive: u64,
    pub(crate) candidate_false_positive: u64,
    pub(crate) baseline_false_negative: u64,
    pub(crate) candidate_false_negative: u64,
    pub(crate) precision: ComparisonMetricPair,
    pub(crate) recall: ComparisonMetricPair,
    pub(crate) f1: ComparisonMetricPair,
    pub(crate) coverage: ComparisonMetricPair,
}

#[derive(Serialize)]
pub(crate) struct ComparisonMacroF1 {
    pub(crate) metric: ComparisonMetricPair,
    pub(crate) baseline_undefined_classes: Vec<String>,
    pub(crate) candidate_undefined_classes: Vec<String>,
}

#[derive(Serialize)]
pub(crate) struct ComparisonProbabilityResults {
    pub(crate) log_loss: ComparisonMetricPair,
    pub(crate) brier_score: ComparisonMetricPair,
    pub(crate) argmax_accuracy: ComparisonMetricPair,
    pub(crate) baseline_raw_answered_count: u64,
    pub(crate) candidate_raw_answered_count: u64,
    pub(crate) baseline_choice_argmax_disagreement_count: Option<u64>,
    pub(crate) candidate_choice_argmax_disagreement_count: Option<u64>,
}

#[derive(Clone, Serialize)]
pub(crate) struct AnsweredPopulation {
    pub(crate) baseline_ids: Vec<String>,
    pub(crate) baseline_count: u64,
    pub(crate) candidate_ids: Vec<String>,
    pub(crate) candidate_count: u64,
    pub(crate) overlap_ids: Vec<String>,
    pub(crate) overlap_count: u64,
}

#[derive(Serialize)]
pub(crate) struct SingleLabelTransitions {
    pub(crate) both_correct: ComparisonCategory,
    pub(crate) recovered: ComparisonCategory,
    pub(crate) regressed: ComparisonCategory,
    pub(crate) neither_correct: ComparisonCategory,
    pub(crate) changed_final_outcomes: ComparisonCategory,
    pub(crate) final_outcome_transitions: Vec<FinalOutcomeTransition>,
}

#[derive(Serialize)]
pub(crate) struct ComparisonCategory {
    pub(crate) ids: Vec<String>,
    pub(crate) count: u64,
}

#[derive(Serialize)]
pub(crate) struct FinalOutcomeTransition {
    pub(crate) baseline: ComparisonOutcome,
    pub(crate) candidate: ComparisonOutcome,
    pub(crate) count: u64,
}

#[derive(Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum ComparisonOutcome {
    Label { label: String },
    Abstention,
}

#[derive(Serialize)]
pub(crate) struct SingleLabelComparisonEpisode {
    pub(crate) id: String,
    pub(crate) baseline_source_id: String,
    pub(crate) candidate_source_id: String,
    pub(crate) baseline_final_outcome: ComparisonOutcome,
    pub(crate) candidate_final_outcome: ComparisonOutcome,
    pub(crate) baseline_correct: bool,
    pub(crate) candidate_correct: bool,
}

#[derive(Serialize)]
struct ReportIdentity {
    run_id: String,
    created_at: String,
    validator_version: String,
    specification_version: String,
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
    kind: crate::model::common::SourceKind,
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
    kind: crate::model::common::ObservationKind,
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
#[derive(Serialize)]
struct ReportPolicy {
    decision: ReportDecisionPolicy,
    categorical_sum_tolerance: f64,
    bin_count: usize,
}
#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ReportDecisionPolicy {
    AsRecorded,
    RejectBelow {
        signal: ReportRejectionSignal,
        minimum: f64,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum ReportRejectionSignal {
    Confidence,
    MaxProbability,
}
#[derive(Serialize)]
struct ReportIntegrity {
    source_count: usize,
    selected_count: usize,
    prediction_count: usize,
    missing_ids: Vec<String>,
    extra_ids: Vec<String>,
    signal_availability: SignalAvailability,
    normalized_count: u64,
    maximum_sum_error: f64,
}
#[derive(Serialize)]
struct ReportHardResults {
    kind: &'static str,
    total: u64,
    correct: u64,
    wrong: u64,
    abstained: u64,
    answered: u64,
    matrix: ReportMatrix,
    classes: Vec<ReportClass>,
    accuracy: MetricResult,
    wrong_class_rate: MetricResult,
    abstention_rate: MetricResult,
    coverage: MetricResult,
    selective_accuracy: MetricResult,
    selective_risk: MetricResult,
    macro_f1: ReportMacroF1,
}
#[derive(Serialize)]
struct ReportMatrix {
    columns: Vec<ReportMatrixColumn>,
    rows: Vec<Vec<u64>>,
}
#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ReportMatrixColumn {
    Label { label: String },
    Abstention,
}
#[derive(Serialize)]
struct ReportClass {
    label: String,
    support: u64,
    predicted_support: u64,
    true_positive: u64,
    false_positive: u64,
    false_negative: u64,
    precision: MetricResult,
    recall: MetricResult,
    f1: MetricResult,
    coverage: MetricResult,
}
#[derive(Serialize)]
struct ReportMacroF1 {
    metric: MetricResult,
    undefined_classes: Vec<String>,
}
#[derive(Serialize)]
struct ReportProbability {
    kind: &'static str,
    log_loss: MetricResult,
    brier_score: MetricResult,
    argmax_accuracy: MetricResult,
    raw_answered_count: u64,
    choice_argmax_disagreement_count: Option<u64>,
}
#[derive(Serialize)]
struct ReportSignals {
    kind: &'static str,
    maximum_probability: ReportSignalBins,
    confidence: ReportSignalBins,
    top_label_ece: MetricResult,
}
#[derive(Serialize)]
struct ReportSignalBins {
    status: crate::model::common::MetricStatus,
    population_scope: crate::model::common::MetricScope,
    population_count: u64,
    included_ids: Vec<String>,
    excluded_ids: Vec<String>,
    bins: Vec<ReportSignalBin>,
}
#[derive(Serialize)]
struct ReportSignalBin {
    index: u8,
    lower: f64,
    upper: f64,
    upper_inclusive: bool,
    count: u64,
    correct_count: u64,
    mean_signal: Option<f64>,
    empirical_accuracy: Option<f64>,
}
#[derive(Serialize)]
struct ReportEpisode {
    id: String,
    source_id: String,
    expected: ReportDecision,
    raw_outcome: ReportDecision,
    final_outcome: ReportDecision,
    raw_correct: bool,
    final_correct: bool,
    rejection_reason: Option<String>,
    observations: BTreeMap<String, crate::model::common::ReportObservation>,
    probability: Option<ReportProbabilityEvidence>,
    reported_confidence: Option<f64>,
}
#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ReportDecision {
    Class {
        label: String,
    },
    Abstention {
        #[serde(skip_serializing_if = "Option::is_none")]
        reason: Option<String>,
    },
}
#[derive(Serialize)]
struct ReportProbabilityEvidence {
    submitted: Vec<f64>,
    working: Vec<f64>,
    argmax: String,
    max_probability: f64,
    chosen_probability: Option<f64>,
    choice_argmax_disagreement: Option<bool>,
    argmax_correct: bool,
}

pub(crate) fn assemble_report(
    run_id: crate::model::common::RunId,
    created_at: String,
    validator_version: String,
    evaluation: &SingleLabelEvaluation,
    results: &SingleLabelResults,
    snapshots: (&ArtifactSnapshot, &ArtifactSnapshot, &ArtifactSnapshot),
    evidence: &[crate::model::common::EvidenceBinding],
) -> Result<SingleLabelRunReport> {
    let (golden, predictions, config) = snapshots;
    let vocabulary = evaluation.vocabulary();
    let normalization = normalization_diagnostics(evaluation)?;
    let label = |index: &LabelIndex| {
        vocabulary
            .label(index)
            .map(str::to_owned)
            .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))
    };
    let artifact =
        |kind: &'static str, snapshot: &crate::model::common::ArtifactSnapshot| ReportArtifact {
            kind,
            path: snapshot.stored_path().display().to_string(),
            sha256: snapshot.digest().to_string(),
            source_id: None,
            evidence_index: None,
            original_path: None,
        };
    let mut manifest = vec![
        artifact("golden", golden),
        artifact("predictions", predictions),
        artifact("config", config),
    ];
    for binding in evidence {
        manifest.push(ReportArtifact {
            kind: "evidence",
            path: binding.stored_path().display().to_string(),
            sha256: binding.digest().to_string(),
            source_id: Some(binding.source_id().as_str().to_owned()),
            evidence_index: Some(binding.evidence_index()),
            original_path: Some(binding.original_path().to_owned()),
        });
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
        let bound = evidence
            .iter()
            .filter(|binding| binding.source_id() == id)
            .map(|binding| binding.stored_path().display().to_string())
            .collect();
        sources.insert(
            id.as_str().to_owned(),
            ReportSource {
                kind: source.kind(),
                model: source.model().to_owned(),
                configuration: source.configuration().to_owned(),
                question_id: source.question_id().map(str::to_owned),
                evidence: bound,
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
                preparation: source.preparation().map(|preparation| ReportPreparation {
                    method: preparation.method().to_owned(),
                    version: preparation.version().to_owned(),
                    configuration: preparation.configuration().to_owned(),
                    evidence_indices: preparation.evidence_indices().to_vec(),
                }),
            },
        );
    }
    let decision = |value: &SingleLabelDecision, reason: Option<&str>| -> Result<ReportDecision> {
        Ok(match value {
            SingleLabelDecision::Label(index) => ReportDecision::Class {
                label: label(index)?,
            },
            SingleLabelDecision::Abstained => ReportDecision::Abstention {
                reason: reason.map(str::to_owned),
            },
        })
    };
    let hard = |hard: &SingleLabelHardResults| -> Result<ReportHardResults> {
        Ok(ReportHardResults {
            kind: "single_label",
            total: hard.total,
            correct: hard.correct,
            wrong: hard.wrong,
            abstained: hard.abstained,
            answered: hard.answered,
            matrix: ReportMatrix {
                columns: hard
                    .matrix
                    .columns
                    .iter()
                    .map(|column| match column {
                        SingleLabelMatrixColumn::Label(index) => {
                            label(index).map(|label| ReportMatrixColumn::Label { label })
                        }
                        SingleLabelMatrixColumn::Abstention => Ok(ReportMatrixColumn::Abstention),
                    })
                    .collect::<Result<_>>()?,
                rows: hard.matrix.rows.clone(),
            },
            classes: hard
                .classes
                .iter()
                .map(|class| {
                    Ok(ReportClass {
                        label: label(&class.label)?,
                        support: class.support,
                        predicted_support: class.predicted_support,
                        true_positive: class.true_positive,
                        false_positive: class.false_positive,
                        false_negative: class.false_negative,
                        precision: class.precision.clone(),
                        recall: class.recall.clone(),
                        f1: class.f1.clone(),
                        coverage: class.coverage.clone(),
                    })
                })
                .collect::<Result<_>>()?,
            accuracy: hard.accuracy.clone(),
            wrong_class_rate: hard.wrong_class_rate.clone(),
            abstention_rate: hard.abstention_rate.clone(),
            coverage: hard.coverage.clone(),
            selective_accuracy: hard.selective_accuracy.clone(),
            selective_risk: hard.selective_risk.clone(),
            macro_f1: ReportMacroF1 {
                metric: hard.macro_f1.metric.clone(),
                undefined_classes: hard
                    .macro_f1
                    .undefined_classes
                    .iter()
                    .map(&label)
                    .collect::<Result<_>>()?,
            },
        })
    };
    let bins = |bins: &SingleLabelSignalBins| ReportSignalBins {
        status: bins.status,
        population_scope: bins.scope,
        population_count: bins.population_count,
        included_ids: bins.included_ids.iter().map(ToString::to_string).collect(),
        excluded_ids: bins.excluded_ids.iter().map(ToString::to_string).collect(),
        bins: bins
            .bins
            .iter()
            .map(|bin| ReportSignalBin {
                index: bin.index,
                lower: bin.lower,
                upper: bin.upper,
                upper_inclusive: bin.upper_inclusive,
                count: bin.count,
                correct_count: bin.correct_count,
                mean_signal: bin.mean_signal,
                empirical_accuracy: bin.empirical_accuracy,
            })
            .collect(),
    };
    let episodes = results
        .episodes
        .iter()
        .map(|episode| {
            Ok(ReportEpisode {
                id: episode.episode_id.to_string(),
                source_id: episode.source_id.as_str().to_owned(),
                expected: ReportDecision::Class {
                    label: label(&episode.expected)?,
                },
                raw_outcome: decision(
                    &episode.raw_outcome,
                    episode.raw_abstention_reason.as_deref(),
                )?,
                final_outcome: decision(
                    &episode.final_outcome,
                    episode.raw_abstention_reason.as_deref(),
                )?,
                raw_correct: episode.raw_correct,
                final_correct: episode.final_correct,
                rejection_reason: episode.rejection_reason.map(str::to_owned),
                observations: episode.observations.report_values(),
                probability: episode
                    .probability
                    .as_ref()
                    .map(|value| {
                        label(&value.argmax).map(|argmax| ReportProbabilityEvidence {
                            submitted: value.submitted.clone(),
                            working: value.working.clone(),
                            argmax,
                            max_probability: value.max_probability,
                            chosen_probability: value.chosen_probability,
                            choice_argmax_disagreement: value.choice_argmax_disagreement,
                            argmax_correct: value.argmax_correct,
                        })
                    })
                    .transpose()?,
                reported_confidence: episode.reported_confidence,
            })
        })
        .collect::<Result<_>>()?;
    let population = evaluation.population();
    Ok(SingleLabelRunReport {
        schema_version: crate::WIRE_VERSION,
        kind: "evaluation",
        status: "complete",
        identity: ReportIdentity {
            run_id: run_id.to_string(),
            created_at,
            validator_version,
            specification_version: crate::SPECIFICATION_VERSION.to_owned(),
            parent_run_id: population.config().parent_run_id().map(|id| id.to_string()),
        },
        artifacts: manifest,
        sources,
        composition: match source_counts.values().filter(|count| **count > 0).count() {
            0 => "empty",
            1 => "single_source",
            _ => "mixed_source",
        },
        source_counts,
        population: ReportPopulation {
            description: population.config().description().to_owned(),
            role: match population.config().role() {
                crate::model::common::EvaluationRole::Development => "development",
                crate::model::common::EvaluationRole::HeldOut => "held_out",
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
            kind: "single_label",
            labels: vocabulary.labels().map(str::to_owned).collect(),
        },
        policy: ReportPolicy {
            decision: match evaluation.population().config().policy() {
                SingleLabelPolicy::AsRecorded => ReportDecisionPolicy::AsRecorded,
                SingleLabelPolicy::RejectBelow { signal, minimum } => {
                    ReportDecisionPolicy::RejectBelow {
                        signal: match signal {
                            SingleLabelRejectionSignal::Confidence => {
                                ReportRejectionSignal::Confidence
                            }
                            SingleLabelRejectionSignal::MaxProbability => {
                                ReportRejectionSignal::MaxProbability
                            }
                        },
                        minimum: *minimum,
                    }
                }
            },
            categorical_sum_tolerance: crate::CATEGORICAL_SUM_TOLERANCE,
            bin_count: crate::TEN_BIN_COUNT,
        },
        integrity: ReportIntegrity {
            source_count: evaluation.sources().len(),
            selected_count: population.selected_count(),
            prediction_count: results.episodes.len(),
            missing_ids: Vec::new(),
            extra_ids: Vec::new(),
            signal_availability: evaluation.signals(),
            normalized_count: normalization.normalized_count,
            maximum_sum_error: normalization.maximum_sum_error,
        },
        raw: hard(&results.raw)?,
        final_results: hard(&results.final_results)?,
        probability: ReportProbability {
            kind: "single_label",
            log_loss: results.probability.log_loss.clone(),
            brier_score: results.probability.brier_score.clone(),
            argmax_accuracy: results.probability.argmax_accuracy.clone(),
            raw_answered_count: results.probability.raw_answered_count,
            choice_argmax_disagreement_count: results.probability.choice_argmax_disagreement_count,
        },
        signals: ReportSignals {
            kind: "single_label",
            maximum_probability: bins(&results.signals.maximum_probability),
            confidence: bins(&results.signals.confidence),
            top_label_ece: results.signals.top_label_ece.clone(),
        },
        episodes,
    })
}

/// Categorical metrics over every selected episode.
#[derive(Debug)]
pub(crate) struct SingleLabelProbabilityResults {
    pub(crate) log_loss: MetricResult,
    pub(crate) brier_score: MetricResult,
    pub(crate) argmax_accuracy: MetricResult,
    pub(crate) raw_answered_count: u64,
    pub(crate) choice_argmax_disagreement_count: Option<u64>,
}

/// Per-episode categorical and confidence diagnostics.
#[derive(Clone, Debug)]
pub(crate) struct SingleLabelProbabilityEvidence {
    pub(crate) submitted: Vec<f64>,
    pub(crate) working: Vec<f64>,
    pub(crate) argmax: LabelIndex,
    pub(crate) max_probability: f64,
    pub(crate) chosen_probability: Option<f64>,
    pub(crate) choice_argmax_disagreement: Option<bool>,
    pub(crate) argmax_correct: bool,
}

/// One deterministic fixed-width signal bin.
#[derive(Debug)]
pub(crate) struct SingleLabelSignalBin {
    pub(crate) index: u8,
    pub(crate) lower: f64,
    pub(crate) upper: f64,
    pub(crate) upper_inclusive: bool,
    pub(crate) count: u64,
    pub(crate) correct_count: u64,
    pub(crate) mean_signal: Option<f64>,
    pub(crate) empirical_accuracy: Option<f64>,
}

/// Bins and their exact participant population for one signal family.
#[derive(Debug)]
pub(crate) struct SingleLabelSignalBins {
    pub(crate) status: crate::model::common::MetricStatus,
    pub(crate) scope: crate::model::common::MetricScope,
    pub(crate) population_count: u64,
    pub(crate) included_ids: Vec<EpisodeId>,
    pub(crate) excluded_ids: Vec<EpisodeId>,
    pub(crate) bins: Vec<SingleLabelSignalBin>,
}

/// Distinct maximum-probability and reported-confidence diagnostics.
#[derive(Debug)]
pub(crate) struct SingleLabelSignalDiagnostics {
    pub(crate) maximum_probability: SingleLabelSignalBins,
    pub(crate) confidence: SingleLabelSignalBins,
    pub(crate) top_label_ece: MetricResult,
}

/// A checked row with exactly one expected target and prediction.
#[derive(Debug)]
pub struct AlignedRow {
    episode: Episode<LabelIndex>,
    prediction: Prediction<SingleLabelOutput>,
}

impl AlignedRow {
    /// Binds one checked expected target to its checked prediction.
    pub(crate) fn new(
        episode: Episode<LabelIndex>,
        prediction: Prediction<SingleLabelOutput>,
        vocabulary: &LabelVocabulary,
    ) -> Result<Self> {
        if prediction.episode_id() != episode.id() || vocabulary.label(episode.target()).is_none() {
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
    pub(crate) fn expected(&self) -> &LabelIndex {
        self.episode.target()
    }
    pub(crate) fn input(&self) -> &serde_json::value::RawValue {
        self.episode.input()
    }
    pub(crate) fn prediction(&self) -> &Prediction<SingleLabelOutput> {
        &self.prediction
    }
}

/// A checked single-label evaluation ready for later scoring.
#[derive(Debug)]
pub struct SingleLabelEvaluation {
    vocabulary: LabelVocabulary,
    sources: HashMap<SourceId, SourceDefinition>,
    population: Population<SingleLabelPolicy>,
    signals: SignalAvailability,
    rows: Vec<AlignedRow>,
}

impl SingleLabelEvaluation {
    /// Creates the closed evaluation representation after full artifact admission.
    pub(crate) fn new(
        vocabulary: LabelVocabulary,
        sources: HashMap<SourceId, SourceDefinition>,
        population: Population<SingleLabelPolicy>,
        signals: SignalAvailability,
        rows: Vec<AlignedRow>,
    ) -> Result<Self> {
        if rows.len() != population.selected_count()
            || rows.iter().zip(population.selected()).any(|(row, id)| {
                row.episode.id() != *id
                    || row.prediction.episode_id() != *id
                    || vocabulary.label(row.episode.target()).is_none()
                    || !sources.contains_key(row.prediction.source_id())
            })
        {
            return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
        }
        Ok(Self {
            vocabulary,
            sources,
            population,
            signals,
            rows,
        })
    }

    /// Borrows the checked population and its configuration.
    pub fn population(&self) -> &Population<SingleLabelPolicy> {
        &self.population
    }

    pub(crate) fn vocabulary(&self) -> &LabelVocabulary {
        &self.vocabulary
    }
    pub(crate) fn rows(&self) -> &[AlignedRow] {
        &self.rows
    }

    pub(crate) fn sources(&self) -> &HashMap<SourceId, SourceDefinition> {
        &self.sources
    }

    pub(crate) const fn signals(&self) -> SignalAvailability {
        self.signals
    }

    pub(crate) fn restrict(self, ids: &[EpisodeId]) -> Result<Self> {
        let mut selected = ids.to_vec();
        selected.sort();
        let Self {
            vocabulary,
            sources,
            population,
            signals,
            rows,
        } = self;
        let population = population.restrict(selected.clone())?;
        let rows = rows
            .into_iter()
            .filter(|row| selected.binary_search(&row.episode_id()).is_ok())
            .collect();
        Self::new(vocabulary, sources, population, signals, rows)
    }
}

/// A checked single-label evaluation boundary.
#[derive(Debug)]
pub enum ValidatedEvaluation {
    SingleLabel(SingleLabelEvaluation),
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::PathBuf};

    use super::*;
    use crate::evaluation::single_label::evaluate;
    use crate::model::common::{
        ArtifactDigest, ArtifactSnapshot, Episode, EpisodeId, EvaluationConfig, EvaluationRole,
        EvidenceBinding, LabelVocabulary, Observation, ObservationDefinition, ObservationKind,
        ObservationSet, Outcome, Population, Prediction, PreparationDescriptor, RunId,
        SourceDefinition, SourceId, SourceKind,
    };

    #[test]
    fn categorical_admission() {
        let vocabulary =
            LabelVocabulary::for_single_label(["left".to_owned(), "right".to_owned()]).unwrap();
        let distribution = CategoricalDistribution::new(
            &HashMap::from([
                ("left".to_owned(), 0.5),
                ("right".to_owned(), 0.499_999_999_5),
            ]),
            &vocabulary,
        )
        .unwrap();
        assert_ne!(distribution.submitted(), distribution.working());
        assert!(
            CategoricalDistribution::new(&HashMap::from([("left".to_owned(), 1.0)]), &vocabulary)
                .is_err()
        );
        assert!(
            CategoricalDistribution::new(
                &HashMap::from([("left".to_owned(), 0.0), ("right".to_owned(), 0.0)]),
                &vocabulary
            )
            .is_err()
        );
    }

    #[test]
    fn scored_choice_ties() {
        let vocabulary =
            LabelVocabulary::for_single_label(["left".to_owned(), "right".to_owned()]).unwrap();
        let source = SourceDefinition::new(
            SourceKind::ScoredChoice,
            "m".to_owned(),
            serde_json::json!({}),
            Some("q".to_owned()),
            vec![],
            HashMap::new(),
            None,
        )
        .unwrap();
        let output = SingleLabelOutput::new(
            Outcome::answered(vocabulary.lookup("right").unwrap()),
            Some(
                CategoricalDistribution::new(
                    &HashMap::from([("left".to_owned(), 0.5), ("right".to_owned(), 0.5)]),
                    &vocabulary,
                )
                .unwrap(),
            ),
            Some(ReportedConfidence::new(0.5).unwrap()),
        );
        assert!(validate_scored_choice(&source, &output, &vocabulary).is_ok());
        let contradiction = SingleLabelOutput::new(
            Outcome::answered(vocabulary.lookup("left").unwrap()),
            Some(
                CategoricalDistribution::new(
                    &HashMap::from([("left".to_owned(), 0.4), ("right".to_owned(), 0.6)]),
                    &vocabulary,
                )
                .unwrap(),
            ),
            Some(ReportedConfidence::new(0.5).unwrap()),
        );
        assert!(validate_scored_choice(&source, &contradiction, &vocabulary).is_err());
    }

    #[test]
    fn artifact_signal_completeness() {
        assert_eq!(
            signal_availability_flags(&[(false, false), (false, false)]).unwrap(),
            SignalAvailability::None
        );
        assert!(signal_availability_flags(&[(false, true), (false, false)]).is_err());
    }

    #[test]
    fn population_alignment_rejects_arbitrary_rows() {
        let vocabulary =
            LabelVocabulary::for_single_label(["left".to_owned(), "right".to_owned()]).unwrap();
        let first = EpisodeId::try_from("01995c20-7d00-7000-8000-000000000001").unwrap();
        let second = EpisodeId::try_from("01995c20-7d00-7000-8000-000000000002").unwrap();
        let digest = ArtifactDigest::try_from(
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        )
        .unwrap();
        let config = EvaluationConfig::new(
            "one episode".to_owned(),
            EvaluationRole::Development,
            None,
            Some(vec![first]),
        )
        .unwrap();
        let population = Population::new(digest, config, vec![first], vec![second]).unwrap();
        let source_id = SourceId::try_from("source").unwrap();
        let sources = HashMap::from([(
            source_id.clone(),
            SourceDefinition::new(
                SourceKind::Classifier,
                "model".to_owned(),
                serde_json::json!({}),
                None,
                vec![],
                HashMap::new(),
                None,
            )
            .unwrap(),
        )]);
        let output = SingleLabelOutput::new(
            Outcome::answered(vocabulary.lookup("left").unwrap()),
            None,
            None,
        );
        let prediction = Prediction::new(
            second,
            source_id,
            output,
            ObservationSet::new(HashMap::new(), &HashMap::new()).unwrap(),
        );
        let row = AlignedRow::new(
            Episode::new(
                second,
                serde_json::value::RawValue::from_string("null".to_owned()).unwrap(),
                vocabulary.lookup("left").unwrap(),
            ),
            prediction,
            &vocabulary,
        )
        .unwrap();
        assert!(
            SingleLabelEvaluation::new(
                vocabulary,
                sources,
                population,
                SignalAvailability::None,
                vec![row],
            )
            .is_err()
        );
    }

    #[test]
    fn report_sources_and_privacy() {
        let first = EpisodeId::try_from("01995c20-7d00-7000-8000-000000000001").unwrap();
        let second = EpisodeId::try_from("01995c20-7d00-7000-8000-000000000002").unwrap();
        let vocabulary =
            LabelVocabulary::for_single_label(["left".to_owned(), "right".to_owned()]).unwrap();
        let definition = || {
            HashMap::from([(
                "score".to_owned(),
                ObservationDefinition::new(
                    ObservationKind::Scalar,
                    "retained score".to_owned(),
                    None,
                )
                .unwrap(),
            )])
        };
        let source = |evidence: Vec<String>| {
            SourceDefinition::new(
                SourceKind::Classifier,
                "model".to_owned(),
                serde_json::json!({"opaque": true}),
                None,
                evidence,
                definition(),
                Some(
                    PreparationDescriptor::new(
                        "prepare".to_owned(),
                        "1".to_owned(),
                        serde_json::json!({"mode":"fixed"}),
                        vec![0],
                        1,
                    )
                    .unwrap(),
                ),
            )
            .unwrap()
        };
        let a = SourceId::try_from("a").unwrap();
        let z = SourceId::try_from("z").unwrap();
        let unused = SourceId::try_from("unused").unwrap();
        let sources = HashMap::from([
            (a.clone(), source(vec!["a.txt".to_owned()])),
            (z.clone(), source(vec!["z.txt".to_owned()])),
            (unused, source(vec!["a.txt".to_owned()])),
        ]);
        let digest = ArtifactDigest::try_from(
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        )
        .unwrap();
        let population = Population::new(
            digest,
            EvaluationConfig::new(
                "test population".to_owned(),
                EvaluationRole::Development,
                None,
                Some(vec![second, first]),
            )
            .unwrap(),
            vec![first, second],
            vec![],
        )
        .unwrap();
        let categorical = |left: f64, right: f64| {
            CategoricalDistribution::new(
                &HashMap::from([("left".to_owned(), left), ("right".to_owned(), right)]),
                &vocabulary,
            )
            .unwrap()
        };
        let observation = || {
            ObservationSet::new(
                HashMap::from([(
                    "score".to_owned(),
                    Observation::scalar(ObservationKind::Scalar, 0.25).unwrap(),
                )]),
                &definition(),
            )
            .unwrap()
        };
        let rows = vec![
            AlignedRow::new(
                Episode::new(
                    first,
                    serde_json::value::RawValue::from_string("null".to_owned()).unwrap(),
                    vocabulary.lookup("left").unwrap(),
                ),
                Prediction::new(
                    first,
                    a,
                    SingleLabelOutput::new(
                        Outcome::answered(vocabulary.lookup("left").unwrap()),
                        Some(categorical(0.8, 0.2)),
                        Some(ReportedConfidence::new(0.8).unwrap()),
                    ),
                    observation(),
                ),
                &vocabulary,
            )
            .unwrap(),
            AlignedRow::new(
                Episode::new(
                    second,
                    serde_json::value::RawValue::from_string("null".to_owned()).unwrap(),
                    vocabulary.lookup("right").unwrap(),
                ),
                Prediction::new(
                    second,
                    z,
                    SingleLabelOutput::new(
                        Outcome::abstained(Some("insufficient evidence".to_owned())).unwrap(),
                        Some(categorical(0.4, 0.6)),
                        Some(ReportedConfidence::new(0.6).unwrap()),
                    ),
                    observation(),
                ),
                &vocabulary,
            )
            .unwrap(),
        ];
        let evaluation = SingleLabelEvaluation::new(
            vocabulary,
            sources,
            population,
            SignalAvailability::Both,
            rows,
        )
        .unwrap();
        let snapshot = |name: &str, bytes: Vec<u8>, digest: u8| {
            ArtifactSnapshot::new(
                PathBuf::from(name),
                PathBuf::from(name),
                bytes,
                ArtifactDigest::from_bytes([digest; 32]),
            )
        };
        let golden = snapshot("golden.json", br#"{"input":"opaque sentinel"}"#.to_vec(), 1);
        let predictions = snapshot("predictions.json", b"{}".to_vec(), 2);
        let config = snapshot("config.json", b"{}".to_vec(), 3);
        let evidence = vec![
            EvidenceBinding::new(
                SourceId::try_from("a").unwrap(),
                0,
                "a.txt".to_owned(),
                PathBuf::from("evidence/0.bin"),
                b"a evidence".to_vec(),
                ArtifactDigest::from_bytes([4; 32]),
            ),
            EvidenceBinding::new(
                SourceId::try_from("unused").unwrap(),
                0,
                "a.txt".to_owned(),
                PathBuf::from("evidence/1.bin"),
                b"a evidence".to_vec(),
                ArtifactDigest::from_bytes([5; 32]),
            ),
            EvidenceBinding::new(
                SourceId::try_from("z").unwrap(),
                0,
                "z.txt".to_owned(),
                PathBuf::from("evidence/2.bin"),
                b"z evidence".to_vec(),
                ArtifactDigest::from_bytes([6; 32]),
            ),
        ];
        let report = assemble_report(
            RunId::try_from("01995c20-7d00-7000-8000-000000000099").unwrap(),
            "2026-09-18T00:00:00Z".to_owned(),
            "test-version".to_owned(),
            &evaluation,
            &evaluate(&evaluation).unwrap(),
            (&golden, &predictions, &config),
            &evidence,
        )
        .unwrap();
        let value = serde_json::to_value(report).unwrap();

        assert_eq!(value["composition"], "mixed_source");
        assert_eq!(
            value["source_counts"],
            serde_json::json!({"a":1,"unused":0,"z":1})
        );
        assert_eq!(
            value["sources"]["a"]["evidence"],
            serde_json::json!(["evidence/0.bin"])
        );
        assert_eq!(
            value["sources"]["z"]["evidence"],
            serde_json::json!(["evidence/2.bin"])
        );
        assert_eq!(value["episodes"][0]["id"], first.to_string());
        assert_eq!(value["episodes"][1]["id"], second.to_string());
        assert!(value["raw"]["accuracy"].is_object());
        assert!(value["final"]["macro_f1"]["metric"].is_object());
        assert!(value["probability"]["log_loss"].is_object());
        assert!(value["signals"]["maximum_probability"]["bins"].is_array());
        assert_eq!(value["episodes"][1]["final_outcome"]["type"], "abstention");
        assert_eq!(
            value["episodes"][1]["raw_outcome"]["reason"],
            "insufficient evidence"
        );
        assert!(
            !serde_json::to_string(&value)
                .unwrap()
                .contains("opaque sentinel")
        );
        let report_schema =
            serde_json::from_str(include_str!("../../schemas/v2/report.schema.json")).unwrap();
        assert!(
            jsonschema::validator_for(&report_schema)
                .unwrap()
                .is_valid(&value)
        );

        let empty_vocabulary =
            LabelVocabulary::for_single_label(["left".to_owned(), "right".to_owned()]).unwrap();
        let empty_sources = HashMap::from([(
            SourceId::try_from("unused").unwrap(),
            SourceDefinition::new(
                SourceKind::Classifier,
                "model".to_owned(),
                serde_json::json!({}),
                None,
                vec![],
                HashMap::new(),
                None,
            )
            .unwrap(),
        )]);
        let empty_population = Population::new(
            digest,
            EvaluationConfig::new(
                "empty population".to_owned(),
                EvaluationRole::Development,
                None,
                None,
            )
            .unwrap(),
            vec![],
            vec![],
        )
        .unwrap();
        let empty_evaluation = SingleLabelEvaluation::new(
            empty_vocabulary,
            empty_sources,
            empty_population,
            SignalAvailability::None,
            vec![],
        )
        .unwrap();
        let empty_results = evaluate(&empty_evaluation).unwrap();
        let empty_report = serde_json::to_value(
            assemble_report(
                RunId::try_from("01995c20-7d00-7000-8000-000000000098").unwrap(),
                "2026-09-18T00:00:00Z".to_owned(),
                "test-version".to_owned(),
                &empty_evaluation,
                &empty_results,
                (&golden, &predictions, &config),
                &[],
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(empty_report["composition"], "empty");
        assert_eq!(
            empty_report["source_counts"],
            serde_json::json!({"unused":0})
        );
        assert!(empty_report["episodes"].as_array().unwrap().is_empty());
        assert_eq!(empty_report["raw"]["accuracy"]["status"], "no_data");
        assert_eq!(
            empty_report["probability"]["log_loss"]["status"],
            "not_applicable"
        );
        assert!(
            jsonschema::validator_for(&report_schema)
                .unwrap()
                .is_valid(&empty_report)
        );
    }
}
