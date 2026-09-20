use std::collections::BTreeSet;

use serde_json::Value;

use crate::{
    Diagnostic, DiagnosticCode, Result,
    evaluation::checked_add,
    model::{
        common::{ArtifactDigest, EpisodeId, EvaluationRole, MetricResult, MetricStatus, RunId},
        multi_label::{
            MultiLabelComparison, MultiLabelComparisonEpisode, MultiLabelComparisonHardResults,
            MultiLabelComparisonLabelMetrics, MultiLabelComparisonMacroF1,
            MultiLabelComparisonOutcome, MultiLabelComparisonProbabilityLabel,
            MultiLabelComparisonProbabilityResults, MultiLabelEpisodeEvidence,
            MultiLabelEpisodeStatus, MultiLabelEvaluation, MultiLabelFinalOutcomeState,
            MultiLabelFinalOutcomeTransition, MultiLabelHardResults, MultiLabelResults,
            MultiLabelTransitionTable, MultiLabelTransitions,
        },
        single_label::{
            AnsweredPopulation, ComparisonCategory, ComparisonClassMetrics, ComparisonDifference,
            ComparisonHardResults, ComparisonIdentity, ComparisonMacroF1, ComparisonMetricPair,
            ComparisonOutcome, ComparisonPopulation, ComparisonProbabilityResults, ComparisonSide,
            ComparisonTask, FinalOutcomeTransition, MetricDeltaReason, MetricDirection,
            SingleLabelComparison, SingleLabelComparisonEpisode, SingleLabelDecision,
            SingleLabelEvaluation, SingleLabelHardResults, SingleLabelResults,
            SingleLabelTransitions,
        },
    },
};

pub(crate) struct ComparisonMetadata {
    pub(crate) run_id: RunId,
    pub(crate) report_digest: ArtifactDigest,
    pub(crate) golden_digest: ArtifactDigest,
    pub(crate) sources: Value,
    pub(crate) decision: Value,
    pub(crate) signal_availability: Value,
    pub(crate) categorical_sum_tolerance: f64,
    pub(crate) bin_count: usize,
}

pub(crate) struct ComparisonScope {
    scope: &'static str,
    population: ComparisonPopulation,
}

pub(crate) fn identical_scope(ids: &[EpisodeId]) -> Result<ComparisonScope> {
    Ok(ComparisonScope {
        scope: "identical",
        population: ComparisonPopulation {
            compared_ids: ids.iter().map(ToString::to_string).collect(),
            compared_count: u64::try_from(ids.len())
                .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?,
            baseline_excluded_ids: Vec::new(),
            baseline_excluded_count: 0,
            candidate_excluded_ids: Vec::new(),
            candidate_excluded_count: 0,
        },
    })
}

pub(crate) fn intersection_scope(
    baseline: &[EpisodeId],
    candidate: &[EpisodeId],
) -> Result<(Vec<EpisodeId>, ComparisonScope)> {
    let common = baseline
        .iter()
        .copied()
        .filter(|id| candidate.binary_search(id).is_ok())
        .collect::<Vec<_>>();
    let baseline_excluded_ids = baseline
        .iter()
        .filter(|id| common.binary_search(id).is_err())
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    let candidate_excluded_ids = candidate
        .iter()
        .filter(|id| common.binary_search(id).is_err())
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    let population = ComparisonPopulation {
        compared_ids: common.iter().map(ToString::to_string).collect(),
        compared_count: u64::try_from(common.len())
            .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?,
        baseline_excluded_count: u64::try_from(baseline_excluded_ids.len())
            .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?,
        baseline_excluded_ids,
        candidate_excluded_count: u64::try_from(candidate_excluded_ids.len())
            .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?,
        candidate_excluded_ids,
    };
    Ok((
        common,
        ComparisonScope {
            scope: "intersection",
            population,
        },
    ))
}

pub(crate) struct ComparisonInput<'a> {
    pub(crate) evaluation: &'a SingleLabelEvaluation,
    pub(crate) results: &'a SingleLabelResults,
    pub(crate) metadata: &'a ComparisonMetadata,
}

pub(crate) fn compare(
    baseline: ComparisonInput<'_>,
    candidate: ComparisonInput<'_>,
    scope: ComparisonScope,
    comparison_id: String,
    created_at: String,
) -> Result<SingleLabelComparison> {
    compatible(&baseline, &candidate)?;
    let labels = baseline
        .evaluation
        .vocabulary()
        .labels()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let mut categories = Categories::default();
    let mut transition_counts = vec![0_u64; state_count(&labels)?];
    let mut episodes = Vec::with_capacity(baseline.results.episodes.len());
    for (baseline_episode, candidate_episode) in baseline
        .results
        .episodes
        .iter()
        .zip(&candidate.results.episodes)
    {
        if baseline_episode.episode_id != candidate_episode.episode_id {
            return Err(Diagnostic::for_code(DiagnosticCode::Comparison));
        }
        let id = baseline_episode.episode_id.to_string();
        categories.add(
            &id,
            baseline_episode.final_correct,
            candidate_episode.final_correct,
        )?;
        let baseline_outcome =
            comparison_outcome(&baseline_episode.final_outcome, baseline.evaluation)?;
        let candidate_outcome =
            comparison_outcome(&candidate_episode.final_outcome, candidate.evaluation)?;
        let baseline_state = outcome_index(&baseline_outcome, &labels)?;
        let candidate_state = outcome_index(&candidate_outcome, &labels)?;
        let index = baseline_state
            .checked_mul(labels.len() + 1)
            .and_then(|value| value.checked_add(candidate_state))
            .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Numeric))?;
        transition_counts[index] = checked_add(transition_counts[index], 1)?;
        if baseline_state != candidate_state {
            categories.changed.push(id.clone());
            categories.changed_count = checked_add(categories.changed_count, 1)?;
        }
        episodes.push(SingleLabelComparisonEpisode {
            id,
            baseline_source_id: baseline_episode.source_id.as_str().to_owned(),
            candidate_source_id: candidate_episode.source_id.as_str().to_owned(),
            baseline_final_outcome: baseline_outcome,
            candidate_final_outcome: candidate_outcome,
            baseline_correct: baseline_episode.final_correct,
            candidate_correct: candidate_episode.final_correct,
        });
    }
    let compared_count =
        u64::try_from(episodes.len()).map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?;
    if categories.total()? != compared_count
        || transition_counts
            .iter()
            .try_fold(0_u64, |total, count| checked_add(total, *count))?
            != compared_count
    {
        return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
    }
    let mut differences = Vec::new();
    collect_differences(
        "policy.decision",
        Some(&baseline.metadata.decision),
        Some(&candidate.metadata.decision),
        &mut differences,
    );
    collect_differences(
        "sources",
        Some(&baseline.metadata.sources),
        Some(&candidate.metadata.sources),
        &mut differences,
    );
    collect_differences(
        "integrity.signal_availability",
        Some(&baseline.metadata.signal_availability),
        Some(&candidate.metadata.signal_availability),
        &mut differences,
    );
    Ok(SingleLabelComparison {
        schema_version: crate::WIRE_VERSION,
        kind: "comparison",
        status: "complete",
        identity: ComparisonIdentity {
            comparison_id,
            created_at,
            validator_version: env!("CARGO_PKG_VERSION").to_owned(),
            specification_version: crate::SPECIFICATION_VERSION.to_owned(),
        },
        baseline: side(
            baseline.metadata,
            baseline
                .results
                .episodes
                .iter()
                .map(|episode| episode.source_id.as_str()),
        )?,
        candidate: side(
            candidate.metadata,
            candidate
                .results
                .episodes
                .iter()
                .map(|episode| episode.source_id.as_str()),
        )?,
        scope: scope.scope,
        task: ComparisonTask {
            kind: "single_label",
            labels: labels.clone(),
        },
        population: scope.population,
        configuration_differences: differences,
        raw: hard_pair(
            &baseline.results.raw,
            &candidate.results.raw,
            &baseline.results.episodes,
            &candidate.results.episodes,
            false,
            baseline.evaluation,
            candidate.evaluation,
        )?,
        final_results: hard_pair(
            &baseline.results.final_results,
            &candidate.results.final_results,
            &baseline.results.episodes,
            &candidate.results.episodes,
            true,
            baseline.evaluation,
            candidate.evaluation,
        )?,
        probability: probability_pair(baseline.results, candidate.results, baseline.evaluation)?,
        transitions: SingleLabelTransitions {
            both_correct: categories.both_correct(),
            recovered: categories.recovered(),
            regressed: categories.regressed(),
            neither_correct: categories.neither_correct(),
            changed_final_outcomes: categories.changed(),
            final_outcome_transitions: transitions(&labels, &transition_counts)?,
        },
        episodes,
    })
}

pub(crate) struct MultiLabelComparisonInput<'a> {
    pub(crate) evaluation: &'a MultiLabelEvaluation,
    pub(crate) results: &'a MultiLabelResults,
    pub(crate) metadata: &'a ComparisonMetadata,
}

pub(crate) fn compare_multi_label(
    baseline: MultiLabelComparisonInput<'_>,
    candidate: MultiLabelComparisonInput<'_>,
    scope: ComparisonScope,
    comparison_id: String,
    created_at: String,
) -> Result<MultiLabelComparison> {
    compatible_multi_label(&baseline, &candidate)?;
    let labels = baseline
        .evaluation
        .vocabulary()
        .labels()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let mut categories = Categories::default();
    let mut per_label_counts = vec![vec![0_u64; 9]; labels.len()];
    let mut episodes = Vec::with_capacity(baseline.results.episodes.len());
    for (baseline_episode, candidate_episode) in baseline
        .results
        .episodes
        .iter()
        .zip(&candidate.results.episodes)
    {
        if baseline_episode.episode_id != candidate_episode.episode_id {
            return Err(Diagnostic::for_code(DiagnosticCode::Comparison));
        }
        let id = baseline_episode.episode_id.to_string();
        let baseline_correct = outcome_correct(&baseline_episode.final_outcome)?;
        let candidate_correct = outcome_correct(&candidate_episode.final_outcome)?;
        categories.add(&id, baseline_correct, candidate_correct)?;
        let baseline_outcome = multi_label_outcome(&baseline_episode.final_outcome)?;
        let candidate_outcome = multi_label_outcome(&candidate_episode.final_outcome)?;
        if !same_multi_label_outcome(&baseline_outcome, &candidate_outcome) {
            categories.changed.push(id.clone());
            categories.changed_count = checked_add(categories.changed_count, 1)?;
        }
        for (index, label) in labels.iter().enumerate() {
            let baseline_state = label_state(&baseline_episode.final_outcome, label);
            let candidate_state = label_state(&candidate_episode.final_outcome, label);
            let state_index = state_index(&baseline_state)
                .checked_mul(3)
                .and_then(|value| value.checked_add(state_index(&candidate_state)))
                .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Numeric))?;
            per_label_counts[index][state_index] =
                checked_add(per_label_counts[index][state_index], 1)?;
        }
        episodes.push(MultiLabelComparisonEpisode {
            id,
            baseline_source_id: baseline_episode.source_id.as_str().to_owned(),
            candidate_source_id: candidate_episode.source_id.as_str().to_owned(),
            baseline_final_outcome: baseline_outcome,
            candidate_final_outcome: candidate_outcome,
            baseline_correct,
            candidate_correct,
            baseline_matched: baseline_episode.final_outcome.matched.clone(),
            candidate_matched: candidate_episode.final_outcome.matched.clone(),
            baseline_missed: baseline_episode.final_outcome.missed.clone(),
            candidate_missed: candidate_episode.final_outcome.missed.clone(),
            baseline_extra: baseline_episode.final_outcome.extra.clone(),
            candidate_extra: candidate_episode.final_outcome.extra.clone(),
        });
    }
    let compared_count =
        u64::try_from(episodes.len()).map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?;
    if categories.total()? != compared_count
        || per_label_counts.iter().any(|counts| {
            counts
                .iter()
                .try_fold(0_u64, |total, count| checked_add(total, *count))
                .map_or(true, |total| total != compared_count)
        })
    {
        return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
    }
    let mut differences = Vec::new();
    collect_differences(
        "policy.decision",
        Some(&baseline.metadata.decision),
        Some(&candidate.metadata.decision),
        &mut differences,
    );
    collect_differences(
        "sources",
        Some(&baseline.metadata.sources),
        Some(&candidate.metadata.sources),
        &mut differences,
    );
    collect_differences(
        "integrity.signal_availability",
        Some(&baseline.metadata.signal_availability),
        Some(&candidate.metadata.signal_availability),
        &mut differences,
    );
    Ok(MultiLabelComparison {
        schema_version: crate::WIRE_VERSION,
        kind: "comparison",
        status: "complete",
        identity: ComparisonIdentity {
            comparison_id,
            created_at,
            validator_version: env!("CARGO_PKG_VERSION").to_owned(),
            specification_version: crate::SPECIFICATION_VERSION.to_owned(),
        },
        baseline: side(
            baseline.metadata,
            baseline
                .results
                .episodes
                .iter()
                .map(|episode| episode.source_id.as_str()),
        )?,
        candidate: side(
            candidate.metadata,
            candidate
                .results
                .episodes
                .iter()
                .map(|episode| episode.source_id.as_str()),
        )?,
        scope: scope.scope,
        task: ComparisonTask {
            kind: "multi_label",
            labels: labels.clone(),
        },
        population: scope.population,
        configuration_differences: differences,
        raw: multi_label_hard_pair(
            &baseline.results.raw,
            &candidate.results.raw,
            &baseline.results.episodes,
            &candidate.results.episodes,
            false,
        )?,
        final_results: multi_label_hard_pair(
            &baseline.results.final_results,
            &candidate.results.final_results,
            &baseline.results.episodes,
            &candidate.results.episodes,
            true,
        )?,
        probability: multi_label_probability_pair(baseline.results, candidate.results)?,
        transitions: MultiLabelTransitions {
            both_correct: categories.both_correct(),
            recovered: categories.recovered(),
            regressed: categories.regressed(),
            neither_correct: categories.neither_correct(),
            changed_final_outcomes: categories.changed(),
            per_label: labels
                .iter()
                .zip(per_label_counts)
                .map(|(label, counts)| {
                    Ok(MultiLabelTransitionTable {
                        label: label.clone(),
                        transitions: multi_label_transitions(&counts)?,
                    })
                })
                .collect::<Result<Vec<_>>>()?,
        },
        episodes,
    })
}

fn compatible_multi_label(
    baseline: &MultiLabelComparisonInput<'_>,
    candidate: &MultiLabelComparisonInput<'_>,
) -> Result<()> {
    if baseline.metadata.golden_digest != candidate.metadata.golden_digest
        || baseline
            .evaluation
            .vocabulary()
            .labels()
            .ne(candidate.evaluation.vocabulary().labels())
        || baseline.evaluation.population().config().role()
            != candidate.evaluation.population().config().role()
        || baseline.evaluation.population().selected()
            != candidate.evaluation.population().selected()
        || baseline.metadata.categorical_sum_tolerance
            != candidate.metadata.categorical_sum_tolerance
        || baseline.metadata.bin_count != candidate.metadata.bin_count
    {
        return Err(Diagnostic::for_code(DiagnosticCode::Comparison));
    }
    Ok(())
}

pub(crate) fn compatible_multi_label_intersection(
    baseline: &MultiLabelComparisonInput<'_>,
    candidate: &MultiLabelComparisonInput<'_>,
) -> Result<()> {
    if baseline.metadata.golden_digest != candidate.metadata.golden_digest
        || baseline
            .evaluation
            .vocabulary()
            .labels()
            .ne(candidate.evaluation.vocabulary().labels())
        || baseline.evaluation.population().config().role()
            != candidate.evaluation.population().config().role()
        || baseline.metadata.categorical_sum_tolerance
            != candidate.metadata.categorical_sum_tolerance
        || baseline.metadata.bin_count != candidate.metadata.bin_count
    {
        return Err(Diagnostic::for_code(DiagnosticCode::Comparison));
    }
    Ok(())
}

fn outcome_correct(outcome: &crate::model::multi_label::MultiLabelOutcomeEvidence) -> Result<bool> {
    match outcome.status {
        MultiLabelEpisodeStatus::Answered => outcome
            .correct
            .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant)),
        MultiLabelEpisodeStatus::Abstained => Ok(false),
    }
}

fn multi_label_outcome(
    outcome: &crate::model::multi_label::MultiLabelOutcomeEvidence,
) -> Result<MultiLabelComparisonOutcome> {
    match outcome.status {
        MultiLabelEpisodeStatus::Answered => Ok(MultiLabelComparisonOutcome::Labels {
            labels: outcome
                .predicted
                .clone()
                .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))?,
        }),
        MultiLabelEpisodeStatus::Abstained => Ok(MultiLabelComparisonOutcome::Abstention),
    }
}

fn same_multi_label_outcome(
    baseline: &MultiLabelComparisonOutcome,
    candidate: &MultiLabelComparisonOutcome,
) -> bool {
    match (baseline, candidate) {
        (
            MultiLabelComparisonOutcome::Labels { labels: baseline },
            MultiLabelComparisonOutcome::Labels { labels: candidate },
        ) => baseline == candidate,
        (MultiLabelComparisonOutcome::Abstention, MultiLabelComparisonOutcome::Abstention) => true,
        _ => false,
    }
}

fn label_state(
    outcome: &crate::model::multi_label::MultiLabelOutcomeEvidence,
    label: &str,
) -> MultiLabelFinalOutcomeState {
    match outcome.status {
        MultiLabelEpisodeStatus::Abstained => MultiLabelFinalOutcomeState::Abstained,
        MultiLabelEpisodeStatus::Answered => {
            if outcome
                .predicted
                .as_ref()
                .is_some_and(|labels| labels.iter().any(|value| value == label))
            {
                MultiLabelFinalOutcomeState::Present
            } else {
                MultiLabelFinalOutcomeState::Absent
            }
        }
    }
}

fn state_index(state: &MultiLabelFinalOutcomeState) -> usize {
    match state {
        MultiLabelFinalOutcomeState::Absent => 0,
        MultiLabelFinalOutcomeState::Present => 1,
        MultiLabelFinalOutcomeState::Abstained => 2,
    }
}

fn multi_label_transitions(counts: &[u64]) -> Result<Vec<MultiLabelFinalOutcomeTransition>> {
    let states = [
        MultiLabelFinalOutcomeState::Absent,
        MultiLabelFinalOutcomeState::Present,
        MultiLabelFinalOutcomeState::Abstained,
    ];
    states
        .iter()
        .enumerate()
        .flat_map(|(baseline_index, baseline)| {
            states
                .iter()
                .enumerate()
                .map(move |(candidate_index, candidate)| {
                    (baseline_index, baseline, candidate_index, candidate)
                })
        })
        .map(|(baseline_index, baseline, candidate_index, candidate)| {
            let index = baseline_index
                .checked_mul(3)
                .and_then(|value| value.checked_add(candidate_index))
                .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Numeric))?;
            Ok(MultiLabelFinalOutcomeTransition {
                baseline: baseline.clone(),
                candidate: candidate.clone(),
                count: *counts
                    .get(index)
                    .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))?,
            })
        })
        .collect()
}

fn multi_label_hard_pair(
    baseline: &MultiLabelHardResults,
    candidate: &MultiLabelHardResults,
    baseline_episodes: &[MultiLabelEpisodeEvidence],
    candidate_episodes: &[MultiLabelEpisodeEvidence],
    final_outcomes: bool,
) -> Result<MultiLabelComparisonHardResults> {
    if baseline.labels.len() != candidate.labels.len() {
        return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
    }
    let answered =
        multi_label_answered_population(baseline_episodes, candidate_episodes, final_outcomes)?;
    let labels = baseline
        .labels
        .iter()
        .zip(&candidate.labels)
        .map(|(baseline_label, candidate_label)| {
            if baseline_label.label != candidate_label.label {
                return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
            }
            Ok(MultiLabelComparisonLabelMetrics {
                label: baseline_label.label.clone(),
                baseline_support: baseline_label.support,
                candidate_support: candidate_label.support,
                baseline_answered_support: baseline_label.answered_support,
                candidate_answered_support: candidate_label.answered_support,
                baseline_predicted_support: baseline_label.predicted_support,
                candidate_predicted_support: candidate_label.predicted_support,
                baseline_true_positive: baseline_label.true_positive,
                candidate_true_positive: candidate_label.true_positive,
                baseline_false_positive: baseline_label.false_positive,
                candidate_false_positive: candidate_label.false_positive,
                baseline_false_negative: baseline_label.false_negative,
                candidate_false_negative: candidate_label.false_negative,
                baseline_true_negative: baseline_label.true_negative,
                candidate_true_negative: candidate_label.true_negative,
                precision: metric_pair(
                    &baseline_label.precision,
                    &candidate_label.precision,
                    MetricDirection::HigherIsBetter,
                    None,
                )?,
                recall: metric_pair(
                    &baseline_label.recall,
                    &candidate_label.recall,
                    MetricDirection::HigherIsBetter,
                    None,
                )?,
                f1: metric_pair(
                    &baseline_label.f1,
                    &candidate_label.f1,
                    MetricDirection::HigherIsBetter,
                    None,
                )?,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(MultiLabelComparisonHardResults {
        baseline_total: baseline.total,
        candidate_total: candidate.total,
        baseline_abstained: baseline.abstained,
        candidate_abstained: candidate.abstained,
        baseline_answered: baseline.answered,
        candidate_answered: candidate.answered,
        baseline_exact_matches: baseline.exact_matches,
        candidate_exact_matches: candidate.exact_matches,
        baseline_wrong_sets: baseline.wrong_sets,
        candidate_wrong_sets: candidate.wrong_sets,
        exact_match_accuracy: metric_pair(
            &baseline.exact_match_accuracy,
            &candidate.exact_match_accuracy,
            MetricDirection::HigherIsBetter,
            None,
        )?,
        wrong_set_rate: metric_pair(
            &baseline.wrong_set_rate,
            &candidate.wrong_set_rate,
            MetricDirection::LowerIsBetter,
            None,
        )?,
        abstention_rate: metric_pair(
            &baseline.abstention_rate,
            &candidate.abstention_rate,
            MetricDirection::LowerIsBetter,
            None,
        )?,
        coverage: metric_pair(
            &baseline.coverage,
            &candidate.coverage,
            MetricDirection::HigherIsBetter,
            None,
        )?,
        selective_exact_match_accuracy: metric_pair(
            &baseline.selective_exact_match_accuracy,
            &candidate.selective_exact_match_accuracy,
            MetricDirection::HigherIsBetter,
            Some(answered.clone()),
        )?,
        selective_risk: metric_pair(
            &baseline.selective_risk,
            &candidate.selective_risk,
            MetricDirection::LowerIsBetter,
            Some(answered.clone()),
        )?,
        answered_micro_precision: metric_pair(
            &baseline.answered_micro_precision,
            &candidate.answered_micro_precision,
            MetricDirection::HigherIsBetter,
            Some(answered.clone()),
        )?,
        answered_micro_recall: metric_pair(
            &baseline.answered_micro_recall,
            &candidate.answered_micro_recall,
            MetricDirection::HigherIsBetter,
            Some(answered.clone()),
        )?,
        answered_micro_f1: metric_pair(
            &baseline.answered_micro_f1,
            &candidate.answered_micro_f1,
            MetricDirection::HigherIsBetter,
            Some(answered.clone()),
        )?,
        answered_macro_f1: MultiLabelComparisonMacroF1 {
            metric: metric_pair(
                &baseline.answered_macro_f1.metric,
                &candidate.answered_macro_f1.metric,
                MetricDirection::HigherIsBetter,
                Some(answered.clone()),
            )?,
            baseline_undefined_classes: baseline.answered_macro_f1.undefined_classes.clone(),
            candidate_undefined_classes: candidate.answered_macro_f1.undefined_classes.clone(),
        },
        answered_hamming_loss: metric_pair(
            &baseline.answered_hamming_loss,
            &candidate.answered_hamming_loss,
            MetricDirection::LowerIsBetter,
            Some(answered),
        )?,
        labels,
    })
}

fn multi_label_probability_pair(
    baseline: &MultiLabelResults,
    candidate: &MultiLabelResults,
) -> Result<MultiLabelComparisonProbabilityResults> {
    if baseline.probability.labels.len() != candidate.probability.labels.len() {
        return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
    }
    let labels = baseline
        .probability
        .labels
        .iter()
        .zip(&candidate.probability.labels)
        .map(|(baseline_label, candidate_label)| {
            if baseline_label.label != candidate_label.label {
                return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
            }
            Ok(MultiLabelComparisonProbabilityLabel {
                label: baseline_label.label.clone(),
                binary_log_loss: metric_pair(
                    &baseline_label.binary_log_loss,
                    &candidate_label.binary_log_loss,
                    MetricDirection::LowerIsBetter,
                    None,
                )?,
                binary_brier: metric_pair(
                    &baseline_label.binary_brier,
                    &candidate_label.binary_brier,
                    MetricDirection::LowerIsBetter,
                    None,
                )?,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(MultiLabelComparisonProbabilityResults {
        labels,
        mean_binary_log_loss: metric_pair(
            &baseline.probability.mean_binary_log_loss,
            &candidate.probability.mean_binary_log_loss,
            MetricDirection::LowerIsBetter,
            None,
        )?,
        mean_binary_brier: metric_pair(
            &baseline.probability.mean_binary_brier,
            &candidate.probability.mean_binary_brier,
            MetricDirection::LowerIsBetter,
            None,
        )?,
    })
}

fn multi_label_answered_population(
    baseline: &[MultiLabelEpisodeEvidence],
    candidate: &[MultiLabelEpisodeEvidence],
    final_outcomes: bool,
) -> Result<AnsweredPopulation> {
    let baseline_ids = multi_label_answered_ids(baseline, final_outcomes);
    let candidate_ids = multi_label_answered_ids(candidate, final_outcomes);
    let overlap_ids = baseline_ids
        .intersection(&candidate_ids)
        .cloned()
        .collect::<Vec<_>>();
    Ok(AnsweredPopulation {
        baseline_count: u64::try_from(baseline_ids.len())
            .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?,
        baseline_ids: baseline_ids.into_iter().collect(),
        candidate_count: u64::try_from(candidate_ids.len())
            .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?,
        candidate_ids: candidate_ids.into_iter().collect(),
        overlap_count: u64::try_from(overlap_ids.len())
            .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?,
        overlap_ids,
    })
}

fn multi_label_answered_ids(
    episodes: &[MultiLabelEpisodeEvidence],
    final_outcomes: bool,
) -> BTreeSet<String> {
    episodes
        .iter()
        .filter(|episode| {
            let outcome = if final_outcomes {
                &episode.final_outcome
            } else {
                &episode.raw
            };
            outcome.status == MultiLabelEpisodeStatus::Answered
        })
        .map(|episode| episode.episode_id.to_string())
        .collect()
}

fn compatible(baseline: &ComparisonInput<'_>, candidate: &ComparisonInput<'_>) -> Result<()> {
    compatible_facts(
        &CompatibilityFacts::from_input(baseline),
        &CompatibilityFacts::from_input(candidate),
        true,
    )
}

pub(crate) fn compatible_intersection(
    baseline: &ComparisonInput<'_>,
    candidate: &ComparisonInput<'_>,
) -> Result<()> {
    compatible_facts(
        &CompatibilityFacts::from_input(baseline),
        &CompatibilityFacts::from_input(candidate),
        false,
    )
}

#[derive(Clone)]
struct CompatibilityFacts {
    golden_digest: ArtifactDigest,
    labels: Vec<String>,
    role: EvaluationRole,
    selected_ids: Vec<EpisodeId>,
    categorical_sum_tolerance: f64,
    bin_count: usize,
}

impl CompatibilityFacts {
    fn from_input(input: &ComparisonInput<'_>) -> Self {
        Self {
            golden_digest: input.metadata.golden_digest,
            labels: input
                .evaluation
                .vocabulary()
                .labels()
                .map(str::to_owned)
                .collect(),
            role: input.evaluation.population().config().role(),
            selected_ids: input.evaluation.population().selected().to_vec(),
            categorical_sum_tolerance: input.metadata.categorical_sum_tolerance,
            bin_count: input.metadata.bin_count,
        }
    }
}

fn compatible_facts(
    baseline: &CompatibilityFacts,
    candidate: &CompatibilityFacts,
    require_same_selection: bool,
) -> Result<()> {
    if baseline.golden_digest != candidate.golden_digest
        || baseline.labels != candidate.labels
        || baseline.role != candidate.role
        || require_same_selection && baseline.selected_ids != candidate.selected_ids
        || baseline.categorical_sum_tolerance != candidate.categorical_sum_tolerance
        || baseline.bin_count != candidate.bin_count
    {
        return Err(Diagnostic::for_code(DiagnosticCode::Comparison));
    }
    Ok(())
}

fn side<'a>(
    metadata: &ComparisonMetadata,
    source_ids: impl IntoIterator<Item = &'a str>,
) -> Result<ComparisonSide> {
    let mut source_counts = std::collections::BTreeMap::new();
    for source_id in source_ids {
        let count = source_counts.entry(source_id.to_owned()).or_insert(0_u64);
        *count = checked_add(*count, 1)?;
    }
    let source_definitions = metadata
        .sources
        .as_object()
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Schema))?;
    let sources = source_counts
        .keys()
        .map(|source_id| {
            let source = source_definitions
                .get(source_id)
                .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))?;
            Ok((source_id.clone(), source.clone()))
        })
        .collect::<Result<serde_json::Map<_, _>>>()?;
    let composition = match source_counts.len() {
        0 => "empty",
        1 => "single_source",
        _ => "mixed_source",
    };
    Ok(ComparisonSide {
        run_id: metadata.run_id.to_string(),
        report_sha256: metadata.report_digest.to_string(),
        sources: Value::Object(sources),
        source_counts: serde_json::to_value(source_counts)
            .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?,
        composition: composition.to_owned(),
    })
}

fn hard_pair(
    baseline: &SingleLabelHardResults,
    candidate: &SingleLabelHardResults,
    baseline_episodes: &[crate::model::single_label::SingleLabelEpisodeEvidence],
    candidate_episodes: &[crate::model::single_label::SingleLabelEpisodeEvidence],
    final_outcomes: bool,
    baseline_evaluation: &SingleLabelEvaluation,
    candidate_evaluation: &SingleLabelEvaluation,
) -> Result<ComparisonHardResults> {
    if baseline.classes.len() != candidate.classes.len() {
        return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
    }
    let answered = answered_population(baseline_episodes, candidate_episodes, final_outcomes)?;
    let classes = baseline
        .classes
        .iter()
        .zip(&candidate.classes)
        .map(|(baseline_class, candidate_class)| {
            let label = baseline_evaluation
                .vocabulary()
                .label(&baseline_class.label)
                .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))?;
            if candidate_evaluation
                .vocabulary()
                .label(&candidate_class.label)
                != Some(label)
            {
                return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
            }
            Ok(ComparisonClassMetrics {
                label: label.to_owned(),
                baseline_support: baseline_class.support,
                candidate_support: candidate_class.support,
                baseline_predicted_support: baseline_class.predicted_support,
                candidate_predicted_support: candidate_class.predicted_support,
                baseline_true_positive: baseline_class.true_positive,
                candidate_true_positive: candidate_class.true_positive,
                baseline_false_positive: baseline_class.false_positive,
                candidate_false_positive: candidate_class.false_positive,
                baseline_false_negative: baseline_class.false_negative,
                candidate_false_negative: candidate_class.false_negative,
                precision: metric_pair(
                    &baseline_class.precision,
                    &candidate_class.precision,
                    MetricDirection::HigherIsBetter,
                    None,
                )?,
                recall: metric_pair(
                    &baseline_class.recall,
                    &candidate_class.recall,
                    MetricDirection::HigherIsBetter,
                    None,
                )?,
                f1: metric_pair(
                    &baseline_class.f1,
                    &candidate_class.f1,
                    MetricDirection::HigherIsBetter,
                    None,
                )?,
                coverage: metric_pair(
                    &baseline_class.coverage,
                    &candidate_class.coverage,
                    MetricDirection::HigherIsBetter,
                    None,
                )?,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(ComparisonHardResults {
        baseline_total: baseline.total,
        candidate_total: candidate.total,
        baseline_correct: baseline.correct,
        candidate_correct: candidate.correct,
        baseline_wrong: baseline.wrong,
        candidate_wrong: candidate.wrong,
        baseline_abstained: baseline.abstained,
        candidate_abstained: candidate.abstained,
        baseline_answered: baseline.answered,
        candidate_answered: candidate.answered,
        accuracy: metric_pair(
            &baseline.accuracy,
            &candidate.accuracy,
            MetricDirection::HigherIsBetter,
            None,
        )?,
        wrong_class_rate: metric_pair(
            &baseline.wrong_class_rate,
            &candidate.wrong_class_rate,
            MetricDirection::LowerIsBetter,
            None,
        )?,
        abstention_rate: metric_pair(
            &baseline.abstention_rate,
            &candidate.abstention_rate,
            MetricDirection::LowerIsBetter,
            None,
        )?,
        coverage: metric_pair(
            &baseline.coverage,
            &candidate.coverage,
            MetricDirection::HigherIsBetter,
            None,
        )?,
        selective_accuracy: metric_pair(
            &baseline.selective_accuracy,
            &candidate.selective_accuracy,
            MetricDirection::HigherIsBetter,
            Some(answered.clone()),
        )?,
        selective_risk: metric_pair(
            &baseline.selective_risk,
            &candidate.selective_risk,
            MetricDirection::LowerIsBetter,
            Some(answered),
        )?,
        classes,
        macro_f1: ComparisonMacroF1 {
            metric: metric_pair(
                &baseline.macro_f1.metric,
                &candidate.macro_f1.metric,
                MetricDirection::HigherIsBetter,
                None,
            )?,
            baseline_undefined_classes: labels_for(
                baseline_evaluation,
                &baseline.macro_f1.undefined_classes,
            )?,
            candidate_undefined_classes: labels_for(
                candidate_evaluation,
                &candidate.macro_f1.undefined_classes,
            )?,
        },
    })
}

fn probability_pair(
    baseline: &SingleLabelResults,
    candidate: &SingleLabelResults,
    _evaluation: &SingleLabelEvaluation,
) -> Result<ComparisonProbabilityResults> {
    let answered = answered_population(&baseline.episodes, &candidate.episodes, false)?;
    Ok(ComparisonProbabilityResults {
        log_loss: metric_pair(
            &baseline.probability.log_loss,
            &candidate.probability.log_loss,
            MetricDirection::LowerIsBetter,
            None,
        )?,
        brier_score: metric_pair(
            &baseline.probability.brier_score,
            &candidate.probability.brier_score,
            MetricDirection::LowerIsBetter,
            None,
        )?,
        argmax_accuracy: metric_pair(
            &baseline.probability.argmax_accuracy,
            &candidate.probability.argmax_accuracy,
            MetricDirection::HigherIsBetter,
            Some(answered),
        )?,
        baseline_raw_answered_count: baseline.probability.raw_answered_count,
        candidate_raw_answered_count: candidate.probability.raw_answered_count,
        baseline_choice_argmax_disagreement_count: baseline
            .probability
            .choice_argmax_disagreement_count,
        candidate_choice_argmax_disagreement_count: candidate
            .probability
            .choice_argmax_disagreement_count,
    })
}

fn metric_pair(
    baseline: &MetricResult,
    candidate: &MetricResult,
    direction: MetricDirection,
    answered_population: Option<AnsweredPopulation>,
) -> Result<ComparisonMetricPair> {
    let (delta, delta_reason) = match (baseline.status_value(), candidate.status_value()) {
        (MetricStatus::Defined, MetricStatus::Defined) => {
            let difference = candidate
                .value()
                .zip(baseline.value())
                .map(|(candidate_value, baseline_value)| candidate_value - baseline_value)
                .filter(|value| value.is_finite());
            match difference {
                Some(value) => (Some(value), None),
                None => (None, Some(MetricDeltaReason::NonFiniteDelta)),
            }
        }
        (status, _) if status != MetricStatus::Defined => {
            (None, Some(MetricDeltaReason::BaselineStatus { status }))
        }
        (_, status) => (None, Some(MetricDeltaReason::CandidateStatus { status })),
    };
    Ok(ComparisonMetricPair {
        direction,
        baseline: baseline.clone(),
        candidate: candidate.clone(),
        delta,
        delta_reason,
        answered_population,
    })
}

fn answered_population(
    baseline: &[crate::model::single_label::SingleLabelEpisodeEvidence],
    candidate: &[crate::model::single_label::SingleLabelEpisodeEvidence],
    final_outcomes: bool,
) -> Result<AnsweredPopulation> {
    let baseline_ids = answered_ids(baseline, final_outcomes);
    let candidate_ids = answered_ids(candidate, final_outcomes);
    let overlap_ids = baseline_ids
        .intersection(&candidate_ids)
        .cloned()
        .collect::<Vec<_>>();
    Ok(AnsweredPopulation {
        baseline_count: u64::try_from(baseline_ids.len())
            .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?,
        baseline_ids: baseline_ids.into_iter().collect(),
        candidate_count: u64::try_from(candidate_ids.len())
            .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?,
        candidate_ids: candidate_ids.into_iter().collect(),
        overlap_count: u64::try_from(overlap_ids.len())
            .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?,
        overlap_ids,
    })
}

fn answered_ids(
    episodes: &[crate::model::single_label::SingleLabelEpisodeEvidence],
    final_outcomes: bool,
) -> BTreeSet<String> {
    episodes
        .iter()
        .filter(|episode| {
            matches!(
                if final_outcomes {
                    &episode.final_outcome
                } else {
                    &episode.raw_outcome
                },
                SingleLabelDecision::Label(_)
            )
        })
        .map(|episode| episode.episode_id.to_string())
        .collect()
}

fn comparison_outcome(
    decision: &SingleLabelDecision,
    evaluation: &SingleLabelEvaluation,
) -> Result<ComparisonOutcome> {
    match decision {
        SingleLabelDecision::Label(label) => Ok(ComparisonOutcome::Label {
            label: evaluation
                .vocabulary()
                .label(label)
                .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))?
                .to_owned(),
        }),
        SingleLabelDecision::Abstained => Ok(ComparisonOutcome::Abstention),
    }
}

fn labels_for(
    evaluation: &SingleLabelEvaluation,
    labels: &[crate::model::common::LabelIndex],
) -> Result<Vec<String>> {
    labels
        .iter()
        .map(|label| {
            evaluation
                .vocabulary()
                .label(label)
                .map(str::to_owned)
                .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))
        })
        .collect()
}

fn outcome_index(outcome: &ComparisonOutcome, labels: &[String]) -> Result<usize> {
    match outcome {
        ComparisonOutcome::Label { label } => labels
            .iter()
            .position(|known| known == label)
            .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant)),
        ComparisonOutcome::Abstention => Ok(labels.len()),
    }
}

fn state_count(labels: &[String]) -> Result<usize> {
    let states = labels
        .len()
        .checked_add(1)
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Numeric))?;
    states
        .checked_mul(states)
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Numeric))
}

fn transitions(labels: &[String], counts: &[u64]) -> Result<Vec<FinalOutcomeTransition>> {
    let states = labels
        .iter()
        .cloned()
        .map(|label| ComparisonOutcome::Label { label })
        .chain(std::iter::once(ComparisonOutcome::Abstention))
        .collect::<Vec<_>>();
    let mut transitions = Vec::with_capacity(counts.len());
    for (baseline_index, baseline) in states.iter().enumerate() {
        for (candidate_index, candidate) in states.iter().enumerate() {
            let index = baseline_index
                .checked_mul(states.len())
                .and_then(|value| value.checked_add(candidate_index))
                .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Numeric))?;
            transitions.push(FinalOutcomeTransition {
                baseline: baseline.clone(),
                candidate: candidate.clone(),
                count: *counts
                    .get(index)
                    .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))?,
            });
        }
    }
    Ok(transitions)
}

#[derive(Default)]
struct Categories {
    both_correct_ids: Vec<String>,
    both_correct_count: u64,
    recovered_ids: Vec<String>,
    recovered_count: u64,
    regressed_ids: Vec<String>,
    regressed_count: u64,
    neither_correct_ids: Vec<String>,
    neither_correct_count: u64,
    changed: Vec<String>,
    changed_count: u64,
}

impl Categories {
    fn add(&mut self, id: &str, baseline: bool, candidate: bool) -> Result<()> {
        let (ids, count) = match (baseline, candidate) {
            (true, true) => (&mut self.both_correct_ids, &mut self.both_correct_count),
            (false, true) => (&mut self.recovered_ids, &mut self.recovered_count),
            (true, false) => (&mut self.regressed_ids, &mut self.regressed_count),
            (false, false) => (
                &mut self.neither_correct_ids,
                &mut self.neither_correct_count,
            ),
        };
        ids.push(id.to_owned());
        *count = checked_add(*count, 1)?;
        Ok(())
    }
    fn total(&self) -> Result<u64> {
        [
            self.both_correct_count,
            self.recovered_count,
            self.regressed_count,
            self.neither_correct_count,
        ]
        .into_iter()
        .try_fold(0_u64, checked_add)
    }
    fn both_correct(&self) -> ComparisonCategory {
        category(&self.both_correct_ids, self.both_correct_count)
    }
    fn recovered(&self) -> ComparisonCategory {
        category(&self.recovered_ids, self.recovered_count)
    }
    fn regressed(&self) -> ComparisonCategory {
        category(&self.regressed_ids, self.regressed_count)
    }
    fn neither_correct(&self) -> ComparisonCategory {
        category(&self.neither_correct_ids, self.neither_correct_count)
    }
    fn changed(&self) -> ComparisonCategory {
        category(&self.changed, self.changed_count)
    }
}

fn category(ids: &[String], count: u64) -> ComparisonCategory {
    ComparisonCategory {
        ids: ids.to_vec(),
        count,
    }
}

fn collect_differences(
    path: &str,
    baseline: Option<&Value>,
    candidate: Option<&Value>,
    differences: &mut Vec<ComparisonDifference>,
) {
    match (baseline, candidate) {
        (Some(Value::Object(baseline)), Some(Value::Object(candidate))) => {
            let keys = baseline
                .keys()
                .chain(candidate.keys())
                .collect::<BTreeSet<_>>();
            for key in keys {
                collect_differences(
                    &format!("{path}.{key}"),
                    baseline.get(key),
                    candidate.get(key),
                    differences,
                );
            }
        }
        (Some(baseline), Some(candidate)) if baseline == candidate => {}
        (baseline, candidate) => differences.push(ComparisonDifference {
            field_path: path.to_owned(),
            baseline: baseline.cloned().unwrap_or(Value::Null),
            candidate: candidate.cloned().unwrap_or(Value::Null),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::{CompatibilityFacts, compatible_facts};
    use crate::{
        DiagnosticCode,
        model::common::{ArtifactDigest, EpisodeId, EvaluationRole},
    };

    fn facts() -> CompatibilityFacts {
        CompatibilityFacts {
            golden_digest: ArtifactDigest::try_from(
                "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            )
            .unwrap(),
            labels: vec!["A".to_owned(), "B".to_owned()],
            role: EvaluationRole::Development,
            selected_ids: vec![
                EpisodeId::try_from("01995c20-7d00-7000-8000-000000000001").unwrap(),
            ],
            categorical_sum_tolerance: 1e-9,
            bin_count: 10,
        }
    }

    #[test]
    fn typed_compatibility_facts_reject_each_runtime_axis() {
        let baseline = facts();
        let assert_rejected = |candidate: CompatibilityFacts| {
            assert_eq!(
                compatible_facts(&baseline, &candidate, true)
                    .unwrap_err()
                    .code(),
                DiagnosticCode::Comparison
            );
        };
        let mut candidate = baseline.clone();
        candidate.golden_digest = ArtifactDigest::try_from(
            "1123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        )
        .unwrap();
        assert_rejected(candidate);
        let mut candidate = baseline.clone();
        candidate.labels.swap(0, 1);
        assert_rejected(candidate);
        let mut candidate = baseline.clone();
        candidate.role = EvaluationRole::HeldOut;
        assert_rejected(candidate);
        let mut candidate = baseline.clone();
        candidate.selected_ids =
            vec![EpisodeId::try_from("01995c20-7d00-7000-8000-000000000002").unwrap()];
        assert_rejected(candidate);
        let mut candidate = baseline.clone();
        candidate.categorical_sum_tolerance = 1e-8;
        assert_rejected(candidate);
        let mut candidate = baseline.clone();
        candidate.bin_count = 9;
        assert_rejected(candidate);
        // The concrete ComparisonInput is single-label by construction; no task-kind
        // branch or generic task escape hatch exists to test.
        assert!(compatible_facts(&baseline, &baseline, true).is_ok());
    }
}
