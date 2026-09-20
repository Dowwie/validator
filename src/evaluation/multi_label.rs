//! Pure multi-label hard-decision accounting.

use crate::{
    Diagnostic, DiagnosticCode, Result,
    evaluation::{checked_add, checked_mul, checked_sub},
    model::{
        common::{MetricResult, MetricScope, MetricStatus, MetricUnit},
        multi_label::{
            MultiLabelEpisodeEvidence, MultiLabelEpisodeStatus, MultiLabelEvaluation,
            MultiLabelHardResults, MultiLabelLabelBins, MultiLabelLabelMetrics, MultiLabelMacroF1,
            MultiLabelOutcomeEvidence, MultiLabelProbabilityLabel, MultiLabelProbabilityResults,
            MultiLabelResults, MultiLabelSignalBin, MultiLabelSignalDiagnostics,
        },
    },
};

/// Scores recorded multi-label sets without accepting wire data.
pub(crate) fn evaluate(evaluation: &MultiLabelEvaluation) -> Result<MultiLabelResults> {
    let raw = score(evaluation, None)?;
    let final_sets = threshold_sets(evaluation)?;
    let final_results = score(evaluation, final_sets.as_deref())?;
    let (probability, signals) = probability(evaluation)?;
    let episodes = evidence(evaluation, final_sets.as_deref())?;
    Ok(MultiLabelResults {
        raw,
        final_results,
        probability,
        signals,
        episodes,
    })
}

fn threshold_sets(
    evaluation: &MultiLabelEvaluation,
) -> Result<Option<Vec<crate::model::common::LabelSet>>> {
    let crate::model::multi_label::MultiLabelPolicy::LabelThresholds { thresholds } =
        evaluation.population().config().policy()
    else {
        return Ok(None);
    };
    evaluation
        .rows()
        .iter()
        .map(|row| {
            let marginals = row
                .prediction()
                .output()
                .marginals()
                .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))?;
            let mut selected = Vec::new();
            for (label, probability) in evaluation.vocabulary().labels().zip(marginals.values()) {
                let threshold = thresholds
                    .get(label)
                    .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))?;
                if probability >= threshold {
                    selected.push(label);
                }
            }
            evaluation.vocabulary().label_set(selected)
        })
        .collect::<Result<Vec<_>>>()
        .map(Some)
}

fn probability(
    evaluation: &MultiLabelEvaluation,
) -> Result<(MultiLabelProbabilityResults, MultiLabelSignalDiagnostics)> {
    let total = count(evaluation.rows().len())?;
    let label_count = count(evaluation.vocabulary().len())?;
    let population_count = checked_mul(total, label_count)?;
    if !evaluation.marginals_available() {
        let unavailable = || {
            MetricResult::status(
                MetricStatus::NotApplicable,
                0,
                MetricScope::Selected,
                MetricUnit::LabelDecision,
            )
        };
        return Ok((
            MultiLabelProbabilityResults {
                labels: evaluation
                    .vocabulary()
                    .labels()
                    .map(|label| MultiLabelProbabilityLabel {
                        label: label.to_owned(),
                        binary_log_loss: unavailable(),
                        binary_brier: unavailable(),
                    })
                    .collect(),
                mean_binary_log_loss: unavailable(),
                mean_binary_brier: unavailable(),
            },
            MultiLabelSignalDiagnostics {
                status: MetricStatus::NotApplicable,
                labels: Vec::new(),
            },
        ));
    }
    if total == 0 {
        let no_data = || {
            MetricResult::status(
                MetricStatus::NoData,
                0,
                MetricScope::Selected,
                MetricUnit::LabelDecision,
            )
        };
        return Ok((
            MultiLabelProbabilityResults {
                labels: evaluation
                    .vocabulary()
                    .labels()
                    .map(|label| MultiLabelProbabilityLabel {
                        label: label.to_owned(),
                        binary_log_loss: no_data(),
                        binary_brier: no_data(),
                    })
                    .collect(),
                mean_binary_log_loss: no_data(),
                mean_binary_brier: no_data(),
            },
            MultiLabelSignalDiagnostics {
                status: MetricStatus::NoData,
                labels: evaluation
                    .vocabulary()
                    .labels()
                    .map(|label| empty_label_bins(label, total))
                    .collect::<Result<_>>()?,
            },
        ));
    }

    let mut losses = vec![0.0; evaluation.vocabulary().len()];
    let mut briers = vec![0.0; evaluation.vocabulary().len()];
    let mut infinite = vec![false; evaluation.vocabulary().len()];
    let mut bins =
        vec![vec![(0_u64, 0_u64, 0.0); crate::TEN_BIN_COUNT]; evaluation.vocabulary().len()];
    for row in evaluation.rows() {
        let marginals = row
            .prediction()
            .output()
            .marginals()
            .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))?;
        for (index, (label, probability)) in evaluation
            .vocabulary()
            .labels()
            .zip(marginals.values())
            .enumerate()
        {
            let positive = row.expected().labels().any(|expected| expected == label);
            let observed_zero =
                (positive && *probability == 0.0) || (!positive && *probability == 1.0);
            if observed_zero {
                infinite[index] = true;
            } else {
                let loss = if positive {
                    -probability.ln()
                } else {
                    -(-probability).ln_1p()
                };
                losses[index] += loss;
            }
            let target = if positive { 1.0 } else { 0.0 };
            briers[index] += (probability - target).powi(2);
            let bin_index = (probability * crate::TEN_BIN_COUNT as f64)
                .floor()
                .min((crate::TEN_BIN_COUNT - 1) as f64) as usize;
            let bin = &mut bins[index][bin_index];
            bin.0 = checked_add(bin.0, 1)?;
            if positive {
                bin.1 = checked_add(bin.1, 1)?;
            }
            bin.2 += probability;
        }
    }
    let labels = evaluation
        .vocabulary()
        .labels()
        .enumerate()
        .map(|(index, label)| {
            Ok(MultiLabelProbabilityLabel {
                label: label.to_owned(),
                binary_log_loss: if infinite[index] {
                    MetricResult::positive_infinity(
                        total,
                        MetricScope::Selected,
                        MetricUnit::LabelDecision,
                    )
                } else {
                    MetricResult::finite_value(
                        losses[index] / total as f64,
                        MetricStatus::Defined,
                        total,
                        MetricScope::Selected,
                        MetricUnit::LabelDecision,
                    )?
                },
                binary_brier: MetricResult::finite_value(
                    briers[index] / total as f64,
                    MetricStatus::Defined,
                    total,
                    MetricScope::Selected,
                    MetricUnit::LabelDecision,
                )?,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let mean_log_loss = if infinite.iter().any(|value| *value) {
        MetricResult::positive_infinity(
            population_count,
            MetricScope::Selected,
            MetricUnit::LabelDecision,
        )
    } else {
        MetricResult::finite_value(
            losses.iter().sum::<f64>() / population_count as f64,
            MetricStatus::Defined,
            population_count,
            MetricScope::Selected,
            MetricUnit::LabelDecision,
        )?
    };
    let mean_brier = MetricResult::finite_value(
        briers.iter().sum::<f64>() / population_count as f64,
        MetricStatus::Defined,
        population_count,
        MetricScope::Selected,
        MetricUnit::LabelDecision,
    )?;
    let label_bins = evaluation
        .vocabulary()
        .labels()
        .enumerate()
        .map(|(index, label)| materialize_label_bins(label, total, &bins[index]))
        .collect::<Result<Vec<_>>>()?;
    Ok((
        MultiLabelProbabilityResults {
            labels,
            mean_binary_log_loss: mean_log_loss,
            mean_binary_brier: mean_brier,
        },
        MultiLabelSignalDiagnostics {
            status: MetricStatus::Defined,
            labels: label_bins,
        },
    ))
}

fn empty_label_bins(label: &str, total: u64) -> Result<MultiLabelLabelBins> {
    materialize_label_bins(label, total, &vec![(0, 0, 0.0); crate::TEN_BIN_COUNT])
}

fn materialize_label_bins(
    label: &str,
    population_count: u64,
    values: &[(u64, u64, f64)],
) -> Result<MultiLabelLabelBins> {
    let bins = values
        .iter()
        .enumerate()
        .map(|(index, (count, positive_count, sum))| {
            Ok(MultiLabelSignalBin {
                index: u8::try_from(index)
                    .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?,
                lower: index as f64 / crate::TEN_BIN_COUNT as f64,
                upper: (index + 1) as f64 / crate::TEN_BIN_COUNT as f64,
                upper_inclusive: index + 1 == crate::TEN_BIN_COUNT,
                count: *count,
                positive_count: *positive_count,
                mean_probability: (*count > 0).then_some(*sum / *count as f64),
                observed_positive_rate: (*count > 0)
                    .then_some(*positive_count as f64 / *count as f64),
            })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(MultiLabelLabelBins {
        label: label.to_owned(),
        population_count,
        population_scope: MetricScope::Selected,
        bins,
    })
}

fn score(
    evaluation: &MultiLabelEvaluation,
    final_sets: Option<&[crate::model::common::LabelSet]>,
) -> Result<MultiLabelHardResults> {
    let total = count(evaluation.rows().len())?;
    let label_count = count(evaluation.vocabulary().len())?;
    let mut labels = evaluation
        .vocabulary()
        .labels()
        .map(|label| Counts::new(label.to_owned()))
        .collect::<Vec<_>>();
    let mut abstained = 0;
    let mut answered = 0;
    let mut exact_matches = 0;

    for (index, row) in evaluation.rows().iter().enumerate() {
        let expected = row.expected();
        for counts in &mut labels {
            if expected.labels().any(|label| label == counts.label) {
                counts.support = checked_add(counts.support, 1)?;
            }
        }
        let predicted = match final_sets {
            Some(sets) => Some(
                sets.get(index)
                    .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))?,
            ),
            None => row.prediction().output().outcome().answered_target(),
        };
        let Some(predicted) = predicted else {
            abstained = checked_add(abstained, 1)?;
            continue;
        };
        answered = checked_add(answered, 1)?;
        if same_set(expected.labels(), predicted.labels()) {
            exact_matches = checked_add(exact_matches, 1)?;
        }
        for counts in &mut labels {
            let expected_present = expected.labels().any(|label| label == counts.label);
            let predicted_present = predicted.labels().any(|label| label == counts.label);
            counts.record(expected_present, predicted_present)?;
        }
    }

    let wrong_sets = checked_sub(answered, exact_matches)?;
    if total != count(evaluation.population().selected_count())?
        || total != checked_add(answered, abstained)?
    {
        return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
    }
    let labels = label_metrics(labels, total, answered)?;
    let totals = aggregate_counts(&labels)?;
    let label_decisions = checked_mul(answered, label_count)?;
    verify_accounting(
        total,
        abstained,
        answered,
        exact_matches,
        wrong_sets,
        &labels,
        label_decisions,
    )?;

    Ok(MultiLabelHardResults {
        total,
        abstained,
        answered,
        exact_matches,
        wrong_sets,
        exact_match_accuracy: selected_metric(exact_matches, total)?,
        wrong_set_rate: selected_metric(wrong_sets, total)?,
        coverage: selected_metric(answered, total)?,
        abstention_rate: selected_metric(abstained, total)?,
        selective_exact_match_accuracy: answered_episode_metric(
            exact_matches,
            answered,
            answered,
            total,
        )?,
        selective_risk: answered_episode_metric(wrong_sets, answered, answered, total)?,
        answered_micro_precision: answered_label_metric(
            totals.true_positive,
            checked_add(totals.true_positive, totals.false_positive)?,
            answered,
            total,
            label_decisions,
        )?,
        answered_micro_recall: answered_label_metric(
            totals.true_positive,
            checked_add(totals.true_positive, totals.false_negative)?,
            answered,
            total,
            label_decisions,
        )?,
        answered_micro_f1: answered_label_metric(
            checked_mul(2, totals.true_positive)?,
            checked_add(
                checked_mul(2, totals.true_positive)?,
                checked_add(totals.false_positive, totals.false_negative)?,
            )?,
            answered,
            total,
            label_decisions,
        )?,
        answered_macro_f1: macro_f1(&labels, total, answered, label_decisions)?,
        answered_hamming_loss: answered_label_metric(
            checked_add(totals.false_positive, totals.false_negative)?,
            label_decisions,
            answered,
            total,
            label_decisions,
        )?,
        labels,
    })
}

#[derive(Debug)]
struct Counts {
    label: String,
    support: u64,
    true_positive: u64,
    false_positive: u64,
    false_negative: u64,
    true_negative: u64,
}

impl Counts {
    fn new(label: String) -> Self {
        Self {
            label,
            support: 0,
            true_positive: 0,
            false_positive: 0,
            false_negative: 0,
            true_negative: 0,
        }
    }

    fn record(&mut self, expected: bool, predicted: bool) -> Result<()> {
        match (expected, predicted) {
            (true, true) => self.true_positive = checked_add(self.true_positive, 1)?,
            (false, true) => self.false_positive = checked_add(self.false_positive, 1)?,
            (true, false) => self.false_negative = checked_add(self.false_negative, 1)?,
            (false, false) => self.true_negative = checked_add(self.true_negative, 1)?,
        }
        Ok(())
    }
}

fn label_metrics(
    counts: Vec<Counts>,
    total: u64,
    answered: u64,
) -> Result<Vec<MultiLabelLabelMetrics>> {
    counts
        .into_iter()
        .map(|counts| {
            let answered_support = checked_add(counts.true_positive, counts.false_negative)?;
            let predicted_support = checked_add(counts.true_positive, counts.false_positive)?;
            Ok(MultiLabelLabelMetrics {
                label: counts.label,
                support: counts.support,
                answered_support,
                predicted_support,
                true_positive: counts.true_positive,
                false_positive: counts.false_positive,
                false_negative: counts.false_negative,
                true_negative: counts.true_negative,
                precision: answered_label_metric(
                    counts.true_positive,
                    predicted_support,
                    answered,
                    total,
                    answered,
                )?,
                recall: answered_label_metric(
                    counts.true_positive,
                    answered_support,
                    answered,
                    total,
                    answered,
                )?,
                f1: answered_label_metric(
                    checked_mul(2, counts.true_positive)?,
                    checked_add(
                        checked_mul(2, counts.true_positive)?,
                        checked_add(counts.false_positive, counts.false_negative)?,
                    )?,
                    answered,
                    total,
                    answered,
                )?,
            })
        })
        .collect()
}

fn macro_f1(
    labels: &[MultiLabelLabelMetrics],
    total: u64,
    answered: u64,
    label_decisions: u64,
) -> Result<MultiLabelMacroF1> {
    if total == 0 {
        return Ok(MultiLabelMacroF1 {
            metric: MetricResult::status(
                MetricStatus::NoData,
                0,
                MetricScope::Answered,
                MetricUnit::LabelDecision,
            ),
            undefined_classes: Vec::new(),
        });
    }
    if answered == 0 {
        return Ok(MultiLabelMacroF1 {
            metric: MetricResult::status(
                MetricStatus::NoAnsweredPredictions,
                0,
                MetricScope::Answered,
                MetricUnit::LabelDecision,
            ),
            undefined_classes: Vec::new(),
        });
    }
    let undefined_classes = labels
        .iter()
        .filter(|label| label.f1.value().is_none())
        .map(|label| label.label.clone())
        .collect::<Vec<_>>();
    let value = labels
        .iter()
        .filter_map(|label| label.f1.value())
        .sum::<f64>()
        / labels.len() as f64;
    let status = if undefined_classes.is_empty() {
        MetricStatus::Defined
    } else {
        MetricStatus::ContainsUndefinedClasses
    };
    Ok(MultiLabelMacroF1 {
        metric: MetricResult::finite_value(
            value,
            status,
            label_decisions,
            MetricScope::Answered,
            MetricUnit::LabelDecision,
        )?,
        undefined_classes,
    })
}

#[derive(Default)]
struct AggregateCounts {
    true_positive: u64,
    false_positive: u64,
    false_negative: u64,
    true_negative: u64,
}

fn aggregate_counts(labels: &[MultiLabelLabelMetrics]) -> Result<AggregateCounts> {
    labels
        .iter()
        .try_fold(AggregateCounts::default(), |totals, label| {
            Ok(AggregateCounts {
                true_positive: checked_add(totals.true_positive, label.true_positive)?,
                false_positive: checked_add(totals.false_positive, label.false_positive)?,
                false_negative: checked_add(totals.false_negative, label.false_negative)?,
                true_negative: checked_add(totals.true_negative, label.true_negative)?,
            })
        })
}

fn verify_accounting(
    total: u64,
    abstained: u64,
    answered: u64,
    exact_matches: u64,
    wrong_sets: u64,
    labels: &[MultiLabelLabelMetrics],
    label_decisions: u64,
) -> Result<()> {
    let aggregate = aggregate_counts(labels)?;
    let per_label_valid = labels.iter().try_fold(true, |valid, label| {
        let four_counts = checked_add(
            checked_add(label.true_positive, label.false_positive)?,
            checked_add(label.false_negative, label.true_negative)?,
        )?;
        let answered_support = checked_add(label.true_positive, label.false_negative)?;
        let predicted_support = checked_add(label.true_positive, label.false_positive)?;
        Ok(valid
            && four_counts == answered
            && label.answered_support == answered_support
            && label.predicted_support == predicted_support)
    })?;
    let aggregate_total = checked_add(
        checked_add(aggregate.true_positive, aggregate.false_positive)?,
        checked_add(aggregate.false_negative, aggregate.true_negative)?,
    )?;
    if total != checked_add(answered, abstained)?
        || answered != checked_add(exact_matches, wrong_sets)?
        || !per_label_valid
        || label_decisions != aggregate_total
    {
        return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
    }
    Ok(())
}

fn selected_metric(numerator: u64, total: u64) -> Result<MetricResult> {
    if total == 0 {
        return Ok(MetricResult::status(
            MetricStatus::NoData,
            0,
            MetricScope::Selected,
            MetricUnit::Episode,
        ));
    }
    MetricResult::ratio_with_population(
        numerator,
        total,
        total,
        MetricScope::Selected,
        MetricUnit::Episode,
    )
}

fn answered_episode_metric(
    numerator: u64,
    denominator: u64,
    answered: u64,
    total: u64,
) -> Result<MetricResult> {
    if total == 0 {
        return Ok(MetricResult::status_with_ratio(
            MetricStatus::NoData,
            numerator,
            denominator,
            0,
            MetricScope::Answered,
            MetricUnit::Episode,
        ));
    }
    if answered == 0 {
        return Ok(MetricResult::status_with_ratio(
            MetricStatus::NoAnsweredPredictions,
            numerator,
            denominator,
            0,
            MetricScope::Answered,
            MetricUnit::Episode,
        ));
    }
    MetricResult::ratio_with_population(
        numerator,
        denominator,
        answered,
        MetricScope::Answered,
        MetricUnit::Episode,
    )
}

fn answered_label_metric(
    numerator: u64,
    denominator: u64,
    answered: u64,
    total: u64,
    label_decisions: u64,
) -> Result<MetricResult> {
    if total == 0 {
        return Ok(MetricResult::status_with_ratio(
            MetricStatus::NoData,
            numerator,
            denominator,
            0,
            MetricScope::Answered,
            MetricUnit::LabelDecision,
        ));
    }
    if answered == 0 {
        return Ok(MetricResult::status_with_ratio(
            MetricStatus::NoAnsweredPredictions,
            numerator,
            denominator,
            0,
            MetricScope::Answered,
            MetricUnit::LabelDecision,
        ));
    }
    MetricResult::ratio_with_population(
        numerator,
        denominator,
        label_decisions,
        MetricScope::Answered,
        MetricUnit::LabelDecision,
    )
}

fn evidence(
    evaluation: &MultiLabelEvaluation,
    final_sets: Option<&[crate::model::common::LabelSet]>,
) -> Result<Vec<MultiLabelEpisodeEvidence>> {
    evaluation
        .rows()
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let expected = labels(row.expected().labels());
            let raw = outcome_evidence(&expected, row.prediction().output().outcome())?;
            let final_outcome = match final_sets {
                Some(sets) => outcome_evidence_for_set(
                    &expected,
                    sets.get(index)
                        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))?,
                ),
                None => outcome_evidence(&expected, row.prediction().output().outcome()),
            }?;
            Ok(MultiLabelEpisodeEvidence {
                episode_id: row.episode_id(),
                expected,
                raw,
                final_outcome,
                raw_abstention_reason: row
                    .prediction()
                    .output()
                    .outcome()
                    .abstention_reason()
                    .map(str::to_owned),
                source_id: row.prediction().source_id().clone(),
                observations: row.prediction().observations().clone(),
                marginals: row
                    .prediction()
                    .output()
                    .marginals()
                    .map(|marginals| marginals.values().to_vec()),
            })
        })
        .collect()
}

fn outcome_evidence_for_set(
    expected: &[String],
    predicted_set: &crate::model::common::LabelSet,
) -> Result<MultiLabelOutcomeEvidence> {
    let predicted = labels(predicted_set.labels());
    let matched = intersection(expected, &predicted);
    let missed = difference(expected, &predicted);
    let extra = difference(&predicted, expected);
    let correct = missed.is_empty() && extra.is_empty();
    Ok(MultiLabelOutcomeEvidence {
        status: MultiLabelEpisodeStatus::Answered,
        predicted: Some(predicted),
        matched: Some(matched),
        missed: Some(missed),
        extra: Some(extra),
        correct: Some(correct),
    })
}

fn outcome_evidence(
    expected: &[String],
    outcome: &crate::model::common::Outcome<crate::model::common::LabelSet>,
) -> Result<MultiLabelOutcomeEvidence> {
    let Some(predicted_set) = outcome.answered_target() else {
        return Ok(MultiLabelOutcomeEvidence {
            status: MultiLabelEpisodeStatus::Abstained,
            predicted: None,
            matched: None,
            missed: None,
            extra: None,
            correct: None,
        });
    };
    let predicted = labels(predicted_set.labels());
    let matched = intersection(expected, &predicted);
    let missed = difference(expected, &predicted);
    let extra = difference(&predicted, expected);
    let correct = missed.is_empty() && extra.is_empty();
    Ok(MultiLabelOutcomeEvidence {
        status: MultiLabelEpisodeStatus::Answered,
        predicted: Some(predicted),
        matched: Some(matched),
        missed: Some(missed),
        extra: Some(extra),
        correct: Some(correct),
    })
}

fn labels<'a>(values: impl Iterator<Item = &'a str>) -> Vec<String> {
    values.map(str::to_owned).collect()
}

fn same_set<'a>(left: impl Iterator<Item = &'a str>, right: impl Iterator<Item = &'a str>) -> bool {
    left.eq(right)
}

fn intersection(left: &[String], right: &[String]) -> Vec<String> {
    left.iter()
        .filter(|label| right.contains(*label))
        .cloned()
        .collect()
}

fn difference(left: &[String], right: &[String]) -> Vec<String> {
    left.iter()
        .filter(|label| !right.contains(*label))
        .cloned()
        .collect()
}

fn count(value: usize) -> Result<u64> {
    u64::try_from(value).map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::*;
    use crate::{
        model::common::{ArtifactDigest, MetricScope, MetricStatus, MetricUnit},
        validation::validate_multi_label,
    };

    const DIGEST: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    fn oracle() -> Value {
        serde_json::from_str(include_str!(
            "../../tests/fixtures/multi-label/expected.json"
        ))
        .unwrap()
    }

    fn evaluated(
        labels: &[&str],
        expected: &[&[&str]],
        predicted: &[Option<&[&str]>],
    ) -> MultiLabelResults {
        let ids = (1..=expected.len())
            .map(|number| format!("01995c20-7d00-7000-8000-{number:012}"))
            .collect::<Vec<_>>();
        let golden = serde_json::to_vec(&json!({
            "schema_version": 2,
            "task": {"kind": "multi_label", "labels": labels},
            "episodes": expected.iter().zip(&ids).map(|(labels, id)| json!({
                "id": id,
                "expected": {"type": "labels", "labels": labels},
                "input": {"opaque": "not copied into scorer evidence"},
            })).collect::<Vec<_>>(),
        }))
        .unwrap();
        let predictions = serde_json::to_vec(&json!({
            "schema_version": 2,
            "dataset_sha256": DIGEST,
            "sources": {"source": {"kind": "classifier", "model": "example", "configuration": {},
                "observation_definitions": {"reported": {"kind": "reported_confidence", "description": "retained"}}
            }},
            "predictions": predicted.iter().zip(&ids).map(|(labels, id)| {
                let outcome = labels.map_or_else(
                    || json!({"type": "abstention", "reason": "review"}),
                    |labels| json!({"type": "labels", "labels": labels}),
                );
                json!({"id": id, "source_id": "source", "outcome": outcome,
                    "observations": {"reported": {"kind": "reported_confidence", "value": 0.6}}})
            }).collect::<Vec<_>>(),
        }))
        .unwrap();
        let config = serde_json::to_vec(&json!({
            "schema_version": 2,
            "population": "all",
            "role": "development",
            "decision": {"type": "as_recorded"},
        }))
        .unwrap();
        let evaluation = validate_multi_label(
            &golden,
            &predictions,
            &config,
            ArtifactDigest::try_from(DIGEST).unwrap(),
        )
        .unwrap();
        evaluate(&evaluation).unwrap()
    }

    fn assert_ratio(metric: &MetricResult, numerator: u64, denominator: u64) {
        assert_eq!(metric.status_value(), MetricStatus::Defined);
        assert_eq!(metric.numerator(), Some(numerator));
        assert_eq!(metric.denominator(), Some(denominator));
    }

    fn assert_oracle_ratio(metric: &MetricResult, expected: &Value) {
        assert_ratio(
            metric,
            expected["numerator"].as_u64().unwrap(),
            expected["denominator"].as_u64().unwrap(),
        );
    }

    #[test]
    fn multi_label_hard_accounting() {
        let oracle = oracle();
        let results = evaluated(
            &["A", "B", "C"],
            &[&["A", "B"], &[]],
            &[Some(&["A", "C"]), Some(&[])],
        );
        for hard in [&results.raw, &results.final_results] {
            assert_eq!(hard.total, oracle["two_episode"]["N"].as_u64().unwrap());
            assert_eq!(hard.answered, oracle["two_episode"]["G"].as_u64().unwrap());
            assert_eq!(hard.exact_matches, 1);
            assert_eq!(hard.wrong_sets, 1);
            assert_oracle_ratio(
                &hard.exact_match_accuracy,
                &oracle["two_episode"]["exact_match_accuracy"],
            );
            assert_oracle_ratio(
                &hard.answered_micro_f1,
                &oracle["two_episode"]["answered_micro_f1"],
            );
            assert_oracle_ratio(
                &hard.answered_hamming_loss,
                &oracle["two_episode"]["answered_hamming_loss"],
            );
            assert_eq!(hard.exact_match_accuracy.scope(), MetricScope::Selected);
            assert_eq!(hard.exact_match_accuracy.unit(), MetricUnit::Episode);
            assert_eq!(hard.answered_micro_f1.scope(), MetricScope::Answered);
            assert_eq!(hard.answered_micro_f1.unit(), MetricUnit::LabelDecision);
            assert_eq!(hard.answered_hamming_loss.population_count(), 6);
            assert!(hard.labels.iter().all(|label| {
                label.true_positive
                    + label.false_positive
                    + label.false_negative
                    + label.true_negative
                    == hard.answered
            }));
            assert_eq!(
                hard.labels
                    .iter()
                    .map(|label| label.true_positive)
                    .sum::<u64>(),
                oracle["two_episode"]["aggregate"]["tp"].as_u64().unwrap()
            );
            assert_eq!(
                hard.labels
                    .iter()
                    .map(|label| label.false_positive)
                    .sum::<u64>(),
                oracle["two_episode"]["aggregate"]["fp"].as_u64().unwrap()
            );
            assert_eq!(
                hard.labels
                    .iter()
                    .map(|label| label.false_negative)
                    .sum::<u64>(),
                oracle["two_episode"]["aggregate"]["fn"].as_u64().unwrap()
            );
            assert_eq!(
                hard.labels
                    .iter()
                    .map(|label| label.true_negative)
                    .sum::<u64>(),
                oracle["two_episode"]["aggregate"]["tn"].as_u64().unwrap()
            );
            assert_eq!(
                hard.answered_macro_f1.metric.value(),
                Some(
                    oracle["two_episode"]["answered_macro_f1"]["numerator"]
                        .as_f64()
                        .unwrap()
                        / oracle["two_episode"]["answered_macro_f1"]["denominator"]
                            .as_f64()
                            .unwrap(),
                )
            );
        }
        let first = &results.episodes[0];
        assert_eq!(first.expected, vec!["A", "B"]);
        assert_eq!(first.raw.matched.as_ref().unwrap(), &vec!["A"]);
        assert_eq!(first.raw.missed.as_ref().unwrap(), &vec!["B"]);
        assert_eq!(first.raw.extra.as_ref().unwrap(), &vec!["C"]);
        assert_eq!(first.raw.correct, Some(false));
        assert_eq!(first.raw, first.final_outcome);

        let empty = evaluated(&["A", "B"], &[&[]], &[Some(&[])]);
        assert_oracle_ratio(
            &empty.raw.exact_match_accuracy,
            &oracle["empty_answered"]["exact_match_accuracy"],
        );
        assert_oracle_ratio(&empty.raw.coverage, &oracle["empty_answered"]["coverage"]);
        assert_oracle_ratio(
            &empty.raw.answered_hamming_loss,
            &oracle["empty_answered"]["answered_hamming_loss"],
        );
        assert_eq!(empty.raw.answered_micro_f1.value(), None);
        assert_eq!(empty.raw.answered_macro_f1.metric.value(), Some(0.0));
        assert_eq!(
            empty.raw.answered_macro_f1.undefined_classes,
            vec!["A", "B"]
        );

        let abstained = evaluated(&["A"], &[&["A"], &[]], &[None, Some(&[])]);
        assert_eq!(
            (
                abstained.raw.total,
                abstained.raw.answered,
                abstained.raw.abstained
            ),
            (2, 1, 1)
        );
        assert_oracle_ratio(
            &abstained.raw.exact_match_accuracy,
            &oracle["abstention"]["exact_match_accuracy"],
        );
        assert_oracle_ratio(
            &abstained.raw.selective_exact_match_accuracy,
            &oracle["abstention"]["selective_exact_match_accuracy"],
        );
        assert_oracle_ratio(&abstained.raw.coverage, &oracle["abstention"]["coverage"]);
        assert_eq!(abstained.raw.labels[0].support, 1);
        assert_eq!(abstained.raw.labels[0].answered_support, 0);
        assert_eq!(abstained.raw.labels[0].true_negative, 1);
        assert_eq!(
            abstained.episodes[0].raw.status,
            MultiLabelEpisodeStatus::Abstained
        );
        assert!(abstained.episodes[0].raw.predicted.is_none());
        assert!(abstained.episodes[0].raw.matched.is_none());

        let all_abstained = evaluated(&["A"], &[&["A"]], &[None]);
        assert_oracle_ratio(
            &all_abstained.raw.coverage,
            &oracle["all_abstained"]["coverage"],
        );
        assert_oracle_ratio(
            &all_abstained.raw.exact_match_accuracy,
            &oracle["all_abstained"]["exact_match_accuracy"],
        );
        assert_eq!(
            all_abstained.raw.answered_micro_f1.status_value(),
            MetricStatus::NoAnsweredPredictions
        );
        assert_eq!(
            all_abstained.raw.answered_hamming_loss.status_value(),
            MetricStatus::NoAnsweredPredictions
        );

        let no_data = evaluated(&["A"], &[], &[]);
        assert_eq!(
            no_data.raw.exact_match_accuracy.status_value(),
            MetricStatus::NoData
        );
        assert_eq!(
            no_data.raw.answered_micro_f1.status_value(),
            MetricStatus::NoData
        );
    }

    #[test]
    fn equal_binary_counts_distinct_exact_sets() {
        let exact = evaluated(
            &["A", "B"],
            &[&["A", "B"], &[]],
            &[Some(&["A", "B"]), Some(&["A", "B"])],
        );
        let split = evaluated(
            &["A", "B"],
            &[&["A"], &["B"]],
            &[Some(&["A", "B"]), Some(&["A", "B"])],
        );
        let counts = |results: &MultiLabelResults| {
            results
                .raw
                .labels
                .iter()
                .map(|label| {
                    (
                        label.true_positive,
                        label.false_positive,
                        label.false_negative,
                        label.true_negative,
                    )
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(counts(&exact), counts(&split));
        let oracle = oracle();
        assert_oracle_ratio(
            &exact.raw.exact_match_accuracy,
            &oracle["equal_binary_counts"]["exact_set_accuracy"],
        );
        assert_oracle_ratio(
            &split.raw.exact_match_accuracy,
            &oracle["equal_binary_counts"]["split_set_accuracy"],
        );
    }
}
