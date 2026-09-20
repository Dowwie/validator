//! Pure single-label hard-decision accounting.

use crate::{
    Diagnostic, DiagnosticCode, Result,
    evaluation::{checked_add, checked_sub},
    model::{
        common::{MetricResult, MetricScope, MetricStatus, MetricUnit},
        single_label::{
            SingleLabelClassMetrics, SingleLabelDecision, SingleLabelEpisodeEvidence,
            SingleLabelEvaluation, SingleLabelHardResults, SingleLabelMacroF1, SingleLabelMatrix,
            SingleLabelMatrixColumn, SingleLabelProbabilityEvidence, SingleLabelProbabilityResults,
            SingleLabelResults, SingleLabelSignalBin, SingleLabelSignalBins,
            SingleLabelSignalDiagnostics,
        },
    },
};

/// Scores recorded decisions without accepting wire data.
pub(crate) fn evaluate(evaluation: &SingleLabelEvaluation) -> Result<SingleLabelResults> {
    let raw = decisions(evaluation)?;
    let final_decisions = final_decisions(evaluation, &raw)?;
    let raw_results = score(evaluation, &raw)?;
    let final_results = score(evaluation, &final_decisions)?;
    let probability = probability(evaluation, &raw)?;
    let episodes = evidence(evaluation, raw, final_decisions, &probability.evidence)?;
    Ok(SingleLabelResults {
        raw: raw_results,
        final_results,
        probability: probability.results,
        signals: probability.signals,
        episodes,
    })
}

fn decisions(evaluation: &SingleLabelEvaluation) -> Result<Vec<SingleLabelDecision>> {
    evaluation
        .rows()
        .iter()
        .map(
            |row| match row.prediction().output().outcome().answered_target() {
                Some(label) => {
                    if evaluation.vocabulary().position(label).is_none() {
                        return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
                    }
                    Ok(SingleLabelDecision::Label(label.clone()))
                }
                None => Ok(SingleLabelDecision::Abstained),
            },
        )
        .collect()
}

fn final_decisions(
    evaluation: &SingleLabelEvaluation,
    raw: &[SingleLabelDecision],
) -> Result<Vec<SingleLabelDecision>> {
    match evaluation.population().config().policy() {
        crate::model::single_label::SingleLabelPolicy::AsRecorded => Ok(raw.to_vec()),
        crate::model::single_label::SingleLabelPolicy::RejectBelow { signal, minimum } => {
            evaluation
                .rows()
                .iter()
                .zip(raw)
                .map(|(row, decision)| {
                    let output = row.prediction().output();
                    let value = match signal {
                        crate::model::single_label::SingleLabelRejectionSignal::Confidence => {
                            output
                                .confidence()
                                .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))?
                                .value()
                        }
                        crate::model::single_label::SingleLabelRejectionSignal::MaxProbability => {
                            output
                                .categorical()
                                .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))?
                                .working()
                                .iter()
                                .copied()
                                .fold(f64::NEG_INFINITY, f64::max)
                        }
                    };
                    if value < *minimum {
                        Ok(SingleLabelDecision::Abstained)
                    } else {
                        Ok(decision.clone())
                    }
                })
                .collect()
        }
    }
}

fn evidence(
    evaluation: &SingleLabelEvaluation,
    raw: Vec<SingleLabelDecision>,
    final_decisions: Vec<SingleLabelDecision>,
    probability: &[Option<SingleLabelProbabilityEvidence>],
) -> Result<Vec<SingleLabelEpisodeEvidence>> {
    if probability.len() != evaluation.rows().len() {
        return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
    }
    evaluation
        .rows()
        .iter()
        .zip(raw)
        .zip(final_decisions)
        .zip(probability.iter().cloned())
        .map(|(((row, raw_outcome), final_outcome), probability)| {
            let raw_correct = is_correct(evaluation, row.expected(), &raw_outcome)?;
            let final_correct = is_correct(evaluation, row.expected(), &final_outcome)?;
            let rejection_reason = matches!(
                (
                    evaluation.population().config().policy(),
                    &raw_outcome,
                    &final_outcome
                ),
                (
                    crate::model::single_label::SingleLabelPolicy::RejectBelow { .. },
                    SingleLabelDecision::Label(_),
                    SingleLabelDecision::Abstained,
                )
            )
            .then_some("below_minimum");
            Ok(SingleLabelEpisodeEvidence {
                episode_id: row.episode_id(),
                expected: row.expected().clone(),
                raw_outcome,
                final_outcome,
                raw_abstention_reason: row
                    .prediction()
                    .output()
                    .outcome()
                    .abstention_reason()
                    .map(str::to_owned),
                raw_correct,
                final_correct,
                rejection_reason,
                source_id: row.prediction().source_id().clone(),
                observations: row.prediction().observations().clone(),
                reported_confidence: row
                    .prediction()
                    .output()
                    .confidence()
                    .map(|value| value.value()),
                probability,
            })
        })
        .collect()
}

struct ProbabilityEvaluation {
    results: SingleLabelProbabilityResults,
    signals: SingleLabelSignalDiagnostics,
    evidence: Vec<Option<SingleLabelProbabilityEvidence>>,
}

fn probability(
    evaluation: &SingleLabelEvaluation,
    raw: &[SingleLabelDecision],
) -> Result<ProbabilityEvaluation> {
    let total = u64::try_from(evaluation.rows().len())
        .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?;
    let has_categorical = evaluation.signals().has_categorical();
    let has_confidence = evaluation.signals().has_confidence();
    let mut evidence = Vec::with_capacity(evaluation.rows().len());
    let mut loss = 0.0;
    let mut brier = 0.0;
    let mut correct = 0_u64;
    let mut disagreements = 0_u64;
    let mut raw_answered = 0_u64;
    let mut infinite_loss = false;
    let mut max_samples = Vec::new();
    let mut confidence_samples = Vec::new();
    let mut excluded_confidence_ids = Vec::new();

    for (row, raw_decision) in evaluation.rows().iter().zip(raw) {
        let output = row.prediction().output();
        let probability_evidence = if has_categorical {
            let distribution = output
                .categorical()
                .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))?;
            let expected = label_position(evaluation, row.expected())?;
            let (argmax_position, max_probability) = argmax(distribution.working())?;
            let argmax_label = label_at(evaluation, argmax_position)?;
            let argmax_correct = argmax_position == expected;
            if argmax_correct {
                correct = checked_add(correct, 1)?;
            }
            let truth = distribution.working()[expected];
            if truth == 0.0 {
                infinite_loss = true;
            } else {
                loss += -truth.ln();
            }
            let row_brier = distribution
                .working()
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    let expected = if index == expected { 1.0 } else { 0.0 };
                    (value - expected).powi(2)
                })
                .sum::<f64>();
            brier += row_brier;
            if !loss.is_finite() || !brier.is_finite() {
                return Err(Diagnostic::for_code(DiagnosticCode::Numeric));
            }
            let (chosen_probability, disagreement) = match raw_decision {
                SingleLabelDecision::Label(label) => {
                    let choice = label_position(evaluation, label)?;
                    raw_answered = checked_add(raw_answered, 1)?;
                    let disagreement = choice != argmax_position;
                    if disagreement {
                        disagreements = checked_add(disagreements, 1)?;
                    }
                    (Some(distribution.working()[choice]), Some(disagreement))
                }
                SingleLabelDecision::Abstained => (None, None),
            };
            max_samples.push((row.episode_id(), max_probability, argmax_correct));
            Some(SingleLabelProbabilityEvidence {
                submitted: distribution.submitted().to_vec(),
                working: distribution.working().to_vec(),
                argmax: argmax_label,
                max_probability,
                chosen_probability,
                choice_argmax_disagreement: disagreement,
                argmax_correct,
            })
        } else {
            None
        };
        if has_confidence {
            let confidence = output
                .confidence()
                .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))?
                .value();
            match raw_decision {
                SingleLabelDecision::Label(_) => confidence_samples.push((
                    row.episode_id(),
                    confidence,
                    is_correct(evaluation, row.expected(), raw_decision)?,
                )),
                SingleLabelDecision::Abstained => excluded_confidence_ids.push(row.episode_id()),
            }
        }
        evidence.push(probability_evidence);
    }

    let results = probability_results(
        has_categorical,
        total,
        ProbabilityTotals {
            loss,
            brier,
            correct,
            raw_answered,
            disagreements,
            infinite_loss,
        },
    )?;
    let maximum_probability = signal_bins(
        has_categorical,
        MetricScope::Selected,
        total,
        max_samples,
        Vec::new(),
        false,
    )?;
    let confidence = signal_bins(
        has_confidence,
        MetricScope::RawAnswered,
        total,
        confidence_samples,
        excluded_confidence_ids,
        true,
    )?;
    let top_label_ece = ece(&maximum_probability, total)?;
    Ok(ProbabilityEvaluation {
        results,
        signals: SingleLabelSignalDiagnostics {
            maximum_probability,
            confidence,
            top_label_ece,
        },
        evidence,
    })
}

struct ProbabilityTotals {
    loss: f64,
    brier: f64,
    correct: u64,
    raw_answered: u64,
    disagreements: u64,
    infinite_loss: bool,
}

fn probability_results(
    available: bool,
    total: u64,
    totals: ProbabilityTotals,
) -> Result<SingleLabelProbabilityResults> {
    if !available {
        return Ok(SingleLabelProbabilityResults {
            log_loss: unavailable_metric(),
            brier_score: unavailable_metric(),
            argmax_accuracy: unavailable_metric(),
            raw_answered_count: 0,
            choice_argmax_disagreement_count: None,
        });
    }
    if total == 0 {
        return Ok(SingleLabelProbabilityResults {
            log_loss: no_data_metric(),
            brier_score: no_data_metric(),
            argmax_accuracy: no_data_metric(),
            raw_answered_count: 0,
            choice_argmax_disagreement_count: Some(0),
        });
    }
    let log_loss = if totals.infinite_loss {
        MetricResult::positive_infinity(total, MetricScope::Selected, MetricUnit::Episode)
    } else {
        MetricResult::finite_value(
            totals.loss / total as f64,
            MetricStatus::Defined,
            total,
            MetricScope::Selected,
            MetricUnit::Episode,
        )?
    };
    Ok(SingleLabelProbabilityResults {
        log_loss,
        brier_score: MetricResult::finite_value(
            totals.brier / total as f64,
            MetricStatus::Defined,
            total,
            MetricScope::Selected,
            MetricUnit::Episode,
        )?,
        argmax_accuracy: MetricResult::ratio_with_population(
            totals.correct,
            total,
            total,
            MetricScope::Selected,
            MetricUnit::Episode,
        )?,
        raw_answered_count: totals.raw_answered,
        choice_argmax_disagreement_count: Some(totals.disagreements),
    })
}

fn signal_bins(
    available: bool,
    scope: MetricScope,
    total: u64,
    samples: Vec<(crate::model::common::EpisodeId, f64, bool)>,
    excluded_ids: Vec<crate::model::common::EpisodeId>,
    confidence: bool,
) -> Result<SingleLabelSignalBins> {
    if !available {
        return Ok(SingleLabelSignalBins {
            status: MetricStatus::NotApplicable,
            scope,
            population_count: 0,
            included_ids: Vec::new(),
            excluded_ids: Vec::new(),
            bins: empty_bins(),
        });
    }
    let population_count =
        u64::try_from(samples.len()).map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?;
    let status = if total == 0 {
        MetricStatus::NoData
    } else if confidence && population_count == 0 {
        MetricStatus::NoAnsweredPredictions
    } else {
        MetricStatus::Defined
    };
    let mut bins = bin_accumulators();
    for (_, signal, correct) in &samples {
        let index = bin_index(*signal)?;
        let bin = &mut bins[index];
        bin.0 = checked_add(bin.0, 1)?;
        if *correct {
            bin.1 = checked_add(bin.1, 1)?;
        }
        bin.2 += signal;
        if !bin.2.is_finite() {
            return Err(Diagnostic::for_code(DiagnosticCode::Numeric));
        }
    }
    Ok(SingleLabelSignalBins {
        status,
        scope,
        population_count,
        included_ids: samples.iter().map(|sample| sample.0).collect(),
        excluded_ids,
        bins: materialize_bins(bins)?,
    })
}

fn ece(bins: &SingleLabelSignalBins, total: u64) -> Result<MetricResult> {
    if bins.status == MetricStatus::NotApplicable {
        return Ok(unavailable_metric());
    }
    if total == 0 {
        return Ok(no_data_metric());
    }
    let value = bins.bins.iter().fold(0.0, |accumulated, bin| {
        match (bin.mean_signal, bin.empirical_accuracy) {
            (Some(mean), Some(accuracy)) => {
                accumulated + bin.count as f64 / total as f64 * (accuracy - mean).abs()
            }
            _ => accumulated,
        }
    });
    MetricResult::finite_value(
        value,
        MetricStatus::Defined,
        total,
        MetricScope::Selected,
        MetricUnit::Episode,
    )
}

fn unavailable_metric() -> MetricResult {
    MetricResult::status(
        MetricStatus::NotApplicable,
        0,
        MetricScope::Selected,
        MetricUnit::Episode,
    )
}

fn no_data_metric() -> MetricResult {
    MetricResult::status(
        MetricStatus::NoData,
        0,
        MetricScope::Selected,
        MetricUnit::Episode,
    )
}

fn argmax(values: &[f64]) -> Result<(usize, f64)> {
    let Some((&first, tail)) = values.split_first() else {
        return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
    };
    let mut best = (0, first);
    for (offset, value) in tail.iter().enumerate() {
        if *value > best.1 {
            best = (offset + 1, *value);
        }
    }
    Ok(best)
}

fn bin_index(signal: f64) -> Result<usize> {
    if !signal.is_finite() || !(0.0..=1.0).contains(&signal) {
        return Err(Diagnostic::for_code(DiagnosticCode::Numeric));
    }
    Ok((10.0 * signal).floor().min(9.0) as usize)
}

fn bin_accumulators() -> Vec<(u64, u64, f64)> {
    vec![(0, 0, 0.0); 10]
}

fn empty_bins() -> Vec<SingleLabelSignalBin> {
    materialize_bins(bin_accumulators()).expect("fixed empty bins are finite")
}

fn materialize_bins(values: Vec<(u64, u64, f64)>) -> Result<Vec<SingleLabelSignalBin>> {
    values
        .into_iter()
        .enumerate()
        .map(|(index, (count, correct_count, signal_sum))| {
            let mean_signal = (count > 0).then_some(signal_sum / count as f64);
            let empirical_accuracy = (count > 0).then_some(correct_count as f64 / count as f64);
            if mean_signal.is_some_and(|value| !value.is_finite())
                || empirical_accuracy.is_some_and(|value| !value.is_finite())
            {
                return Err(Diagnostic::for_code(DiagnosticCode::Numeric));
            }
            Ok(SingleLabelSignalBin {
                index: u8::try_from(index)
                    .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?,
                lower: index as f64 / 10.0,
                upper: (index + 1) as f64 / 10.0,
                upper_inclusive: index == 9,
                count,
                correct_count,
                mean_signal,
                empirical_accuracy,
            })
        })
        .collect()
}

fn score(
    evaluation: &SingleLabelEvaluation,
    decisions: &[SingleLabelDecision],
) -> Result<SingleLabelHardResults> {
    if decisions.len() != evaluation.rows().len() {
        return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
    }
    let class_count = evaluation.vocabulary().len();
    let mut rows = vec![vec![0; class_count + 1]; class_count];
    for (row, decision) in evaluation.rows().iter().zip(decisions) {
        let expected = label_position(evaluation, row.expected())?;
        let predicted = decision_position(evaluation, decision, class_count)?;
        rows[expected][predicted] = checked_add(rows[expected][predicted], 1)?;
    }

    let matrix = SingleLabelMatrix {
        columns: matrix_columns(evaluation),
        rows,
    };
    hard_results(evaluation, matrix)
}

fn matrix_columns(evaluation: &SingleLabelEvaluation) -> Vec<SingleLabelMatrixColumn> {
    let mut columns = evaluation
        .vocabulary()
        .labels()
        .filter_map(|label| evaluation.vocabulary().lookup(label))
        .map(SingleLabelMatrixColumn::Label)
        .collect::<Vec<_>>();
    columns.push(SingleLabelMatrixColumn::Abstention);
    columns
}

fn hard_results(
    evaluation: &SingleLabelEvaluation,
    matrix: SingleLabelMatrix,
) -> Result<SingleLabelHardResults> {
    let total = sum_matrix(&matrix.rows)?;
    let selected = u64::try_from(evaluation.population().selected_count())
        .map_err(|_| Diagnostic::for_code(DiagnosticCode::Numeric))?;
    if total != selected {
        return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
    }
    let correct = diagonal(&matrix.rows)?;
    let abstained = column_sum(&matrix.rows, evaluation.vocabulary().len())?;
    let answered = checked_sub(total, abstained)?;
    let wrong = checked_sub(answered, correct)?;
    let classes = class_metrics(evaluation, &matrix, total)?;
    let macro_f1 = macro_f1(&classes, total)?;
    verify_accounting(
        &matrix, total, correct, wrong, abstained, answered, &classes,
    )?;

    Ok(SingleLabelHardResults {
        total,
        correct,
        wrong,
        abstained,
        answered,
        matrix,
        classes,
        accuracy: selected_metric(correct, total)?,
        wrong_class_rate: selected_metric(wrong, total)?,
        abstention_rate: selected_metric(abstained, total)?,
        coverage: selected_metric(answered, total)?,
        selective_accuracy: answered_metric(correct, answered, total)?,
        selective_risk: answered_metric(wrong, answered, total)?,
        macro_f1,
    })
}

fn class_metrics(
    evaluation: &SingleLabelEvaluation,
    matrix: &SingleLabelMatrix,
    total: u64,
) -> Result<Vec<SingleLabelClassMetrics>> {
    (0..evaluation.vocabulary().len())
        .map(|class| {
            let support = sum(matrix.rows[class].iter().copied())?;
            let predicted_support = column_sum(&matrix.rows, class)?;
            let true_positive = matrix.rows[class][class];
            let false_negative = checked_sub(support, true_positive)?;
            let false_positive = checked_sub(predicted_support, true_positive)?;
            let precision_denominator = checked_add(true_positive, false_positive)?;
            let recall_denominator = checked_add(true_positive, false_negative)?;
            let f1_denominator = checked_add(
                checked_add(true_positive, true_positive)?,
                checked_add(false_positive, false_negative)?,
            )?;
            let covered = checked_sub(support, matrix.rows[class][matrix.columns.len() - 1])?;
            Ok(SingleLabelClassMetrics {
                label: label_at(evaluation, class)?,
                support,
                predicted_support,
                true_positive,
                false_positive,
                false_negative,
                precision: class_metric(true_positive, precision_denominator, total)?,
                recall: class_metric(true_positive, recall_denominator, total)?,
                f1: class_metric(
                    checked_add(true_positive, true_positive)?,
                    f1_denominator,
                    total,
                )?,
                coverage: class_metric(covered, support, total)?,
            })
        })
        .collect()
}

fn macro_f1(classes: &[SingleLabelClassMetrics], total: u64) -> Result<SingleLabelMacroF1> {
    if total == 0 {
        return Ok(SingleLabelMacroF1 {
            metric: MetricResult::status(
                MetricStatus::NoData,
                0,
                MetricScope::Selected,
                MetricUnit::Episode,
            ),
            undefined_classes: Vec::new(),
        });
    }
    let undefined_classes = classes
        .iter()
        .filter(|class| class.f1.value().is_none())
        .map(|class| class.label.clone())
        .collect::<Vec<_>>();
    let value = classes
        .iter()
        .filter_map(|class| class.f1.value())
        .sum::<f64>()
        / classes.len() as f64;
    let status = if undefined_classes.is_empty() {
        MetricStatus::Defined
    } else {
        MetricStatus::ContainsUndefinedClasses
    };
    Ok(SingleLabelMacroF1 {
        metric: MetricResult::finite_value(
            value,
            status,
            total,
            MetricScope::Selected,
            MetricUnit::Episode,
        )?,
        undefined_classes,
    })
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

fn answered_metric(numerator: u64, answered: u64, total: u64) -> Result<MetricResult> {
    if total == 0 {
        return Ok(MetricResult::status(
            MetricStatus::NoData,
            0,
            MetricScope::Answered,
            MetricUnit::Episode,
        ));
    }
    if answered == 0 {
        return Ok(MetricResult::status_with_ratio(
            MetricStatus::NoAnsweredPredictions,
            numerator,
            answered,
            0,
            MetricScope::Answered,
            MetricUnit::Episode,
        ));
    }
    MetricResult::ratio_with_population(
        numerator,
        answered,
        answered,
        MetricScope::Answered,
        MetricUnit::Episode,
    )
}

fn class_metric(numerator: u64, denominator: u64, total: u64) -> Result<MetricResult> {
    if total == 0 {
        return Ok(MetricResult::status_with_ratio(
            MetricStatus::NoData,
            numerator,
            denominator,
            0,
            MetricScope::Selected,
            MetricUnit::Episode,
        ));
    }
    MetricResult::ratio_with_population(
        numerator,
        denominator,
        total,
        MetricScope::Selected,
        MetricUnit::Episode,
    )
}

fn verify_accounting(
    matrix: &SingleLabelMatrix,
    total: u64,
    correct: u64,
    wrong: u64,
    abstained: u64,
    answered: u64,
    classes: &[SingleLabelClassMetrics],
) -> Result<()> {
    let class_total = sum(classes.iter().map(|class| class.support))?;
    let predicted_total = sum(classes.iter().map(|class| class.predicted_support))?;
    let true_positive_total = sum(classes.iter().map(|class| class.true_positive))?;
    let false_positive_total = sum(classes.iter().map(|class| class.false_positive))?;
    let false_negative_total = sum(classes.iter().map(|class| class.false_negative))?;
    let accounted = checked_add(checked_add(correct, wrong)?, abstained)?;
    let misses = checked_add(wrong, abstained)?;
    if matrix.columns.len() != matrix.rows.len() + 1
        || total != class_total
        || answered != predicted_total
        || correct != true_positive_total
        || wrong != false_positive_total
        || misses != false_negative_total
        || total != accounted
    {
        return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
    }
    Ok(())
}

fn is_correct(
    evaluation: &SingleLabelEvaluation,
    expected: &crate::model::common::LabelIndex,
    outcome: &SingleLabelDecision,
) -> Result<bool> {
    Ok(match outcome {
        SingleLabelDecision::Label(label) => {
            label_position(evaluation, expected)? == label_position(evaluation, label)?
        }
        SingleLabelDecision::Abstained => false,
    })
}

fn label_at(
    evaluation: &SingleLabelEvaluation,
    position: usize,
) -> Result<crate::model::common::LabelIndex> {
    evaluation
        .vocabulary()
        .labels()
        .nth(position)
        .and_then(|label| evaluation.vocabulary().lookup(label))
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))
}

fn label_position(
    evaluation: &SingleLabelEvaluation,
    label: &crate::model::common::LabelIndex,
) -> Result<usize> {
    evaluation
        .vocabulary()
        .position(label)
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))
}

fn decision_position(
    evaluation: &SingleLabelEvaluation,
    decision: &SingleLabelDecision,
    abstention_column: usize,
) -> Result<usize> {
    match decision {
        SingleLabelDecision::Label(label) => label_position(evaluation, label),
        SingleLabelDecision::Abstained => Ok(abstention_column),
    }
}

fn sum_matrix(rows: &[Vec<u64>]) -> Result<u64> {
    sum(rows.iter().flat_map(|row| row.iter().copied()))
}

fn diagonal(rows: &[Vec<u64>]) -> Result<u64> {
    sum(rows.iter().enumerate().map(|(index, row)| row[index]))
}

fn column_sum(rows: &[Vec<u64>], column: usize) -> Result<u64> {
    sum(rows.iter().map(|row| row[column]))
}

fn sum(values: impl IntoIterator<Item = u64>) -> Result<u64> {
    values.into_iter().try_fold(0, checked_add)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::{
        model::{
            common::{ArtifactDigest, MetricScope, MetricStatus, MetricUnit},
            single_label::{SingleLabelMatrixColumn, ValidatedEvaluation},
        },
        validation::validate_single_label,
    };

    const DIGEST: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    fn evaluated(
        labels: &[&str],
        actual: &[&str],
        predicted: &[Option<&str>],
    ) -> SingleLabelResults {
        let ids = (1..=actual.len())
            .map(|number| format!("01995c20-7d00-7000-8000-{number:012}"))
            .collect::<Vec<_>>();
        let golden = serde_json::to_vec(&json!({
            "schema_version": 2,
            "task": {"kind": "single_label", "labels": labels},
            "episodes": actual.iter().zip(&ids).map(|(label, id)| json!({
                "id": id,
                "expected": {"type": "class", "label": label},
                "input": {"opaque": "not copied into scorer evidence"},
            })).collect::<Vec<_>>(),
        }))
        .unwrap();
        let predictions = serde_json::to_vec(&json!({
            "schema_version": 2,
            "dataset_sha256": DIGEST,
            "sources": {"source": {
                "kind": "classifier",
                "model": "example",
                "configuration": {},
                "observation_definitions": {
                    "native_score": {"kind": "scalar", "description": "native score"}
                }
            }},
            "predictions": predicted.iter().zip(&ids).map(|(label, id)| {
                let outcome = match label {
                    Some(label) => json!({"type": "class", "label": label}),
                    None => json!({"type": "abstention"}),
                };
                json!({
                    "id": id,
                    "source_id": "source",
                    "outcome": outcome,
                    "observations": {"native_score": {"kind": "scalar", "value": 1.33}},
                })
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
        let digest = ArtifactDigest::try_from(DIGEST).unwrap();
        let ValidatedEvaluation::SingleLabel(evaluation) =
            validate_single_label(&golden, &predictions, &config, digest).unwrap();
        evaluate(&evaluation).unwrap()
    }

    fn evaluated_with_signals(
        labels: &[&str],
        actual: &[&str],
        predicted: &[Option<&str>],
        probabilities: &[Vec<f64>],
        confidence: Option<&[f64]>,
    ) -> SingleLabelResults {
        let ids = (1..=actual.len())
            .map(|number| format!("01995c20-7d00-7000-8000-{number:012}"))
            .collect::<Vec<_>>();
        let golden = serde_json::to_vec(&json!({
            "schema_version": 2,
            "task": {"kind": "single_label", "labels": labels},
            "episodes": actual.iter().zip(&ids).map(|(label, id)| json!({
                "id": id, "expected": {"type": "class", "label": label}, "input": {"opaque": true}
            })).collect::<Vec<_>>(),
        }))
        .unwrap();
        let mut rows = Vec::new();
        for (index, (label, id)) in predicted.iter().zip(&ids).enumerate() {
            let outcome = label.map_or_else(
                || json!({"type": "abstention"}),
                |label| json!({"type": "class", "label": label}),
            );
            let mut row = json!({"id": id, "source_id": "source", "outcome": outcome});
            if !probabilities.is_empty() {
                let values = labels
                    .iter()
                    .zip(&probabilities[index])
                    .map(|(label, value)| ((*label).to_owned(), json!(value)))
                    .collect::<serde_json::Map<_, _>>();
                row.as_object_mut().unwrap().insert(
                    "probabilities".to_owned(),
                    json!({"kind": "categorical", "values": values}),
                );
            }
            if let Some(confidence) = confidence {
                row.as_object_mut()
                    .unwrap()
                    .insert("confidence".to_owned(), json!(confidence[index]));
            }
            rows.push(row);
        }
        let predictions = serde_json::to_vec(&json!({
            "schema_version": 2, "dataset_sha256": DIGEST,
            "sources": {"source": {"kind": "classifier", "model": "example", "configuration": {}}},
            "predictions": rows,
        }))
        .unwrap();
        let config = serde_json::to_vec(&json!({
            "schema_version": 2, "population": "all", "role": "development", "decision": {"type": "as_recorded"}
        })).unwrap();
        let digest = ArtifactDigest::try_from(DIGEST).unwrap();
        let ValidatedEvaluation::SingleLabel(evaluation) =
            validate_single_label(&golden, &predictions, &config, digest).unwrap();
        evaluate(&evaluation).unwrap()
    }

    fn assert_ratio(
        metric: &MetricResult,
        expected: f64,
        numerator: u64,
        denominator: u64,
        scope: MetricScope,
        population: u64,
    ) {
        let actual = metric.value().unwrap();
        assert!((actual - expected).abs() <= 1e-12 + 1e-10 * expected.abs());
        assert_eq!(metric.status_value(), MetricStatus::Defined);
        assert_eq!(metric.numerator(), Some(numerator));
        assert_eq!(metric.denominator(), Some(denominator));
        assert_eq!(metric.scope(), scope);
        assert_eq!(metric.unit(), MetricUnit::Episode);
        assert_eq!(metric.population_count(), population);
        let serialized = serde_json::to_value(metric).unwrap();
        assert_eq!(serialized["status"], "defined");
        assert_eq!(
            serialized["population_scope"],
            serde_json::to_value(scope).unwrap()
        );
        assert_eq!(serialized["population_unit"].as_str(), Some("episode"));
    }

    #[test]
    fn single_label_f04_asymmetric_hard_metrics() {
        let results = evaluated(
            &["A", "B", "C"],
            &["A", "A", "A", "A", "B", "B", "C", "C"],
            &[
                Some("A"),
                Some("A"),
                Some("B"),
                Some("C"),
                Some("B"),
                Some("B"),
                Some("A"),
                Some("C"),
            ],
        );
        let hard = &results.raw;
        assert_eq!(
            hard.matrix.rows,
            vec![vec![2, 1, 1, 0], vec![0, 2, 0, 0], vec![1, 0, 1, 0]]
        );
        assert_eq!(
            (
                hard.total,
                hard.correct,
                hard.wrong,
                hard.abstained,
                hard.answered
            ),
            (8, 5, 3, 0, 8)
        );
        assert_eq!(
            hard.classes.iter().map(|class| class.support).sum::<u64>(),
            hard.total
        );
        assert_eq!(
            hard.classes
                .iter()
                .map(|class| class.predicted_support)
                .sum::<u64>(),
            hard.answered
        );
        assert_eq!(
            hard.classes
                .iter()
                .map(|class| class.true_positive)
                .sum::<u64>(),
            hard.correct
        );
        assert_eq!(
            hard.classes
                .iter()
                .map(|class| class.false_positive)
                .sum::<u64>(),
            hard.wrong
        );
        assert_eq!(
            hard.classes
                .iter()
                .map(|class| class.false_negative)
                .sum::<u64>(),
            hard.wrong + hard.abstained
        );
        assert_ratio(&hard.accuracy, 5.0 / 8.0, 5, 8, MetricScope::Selected, 8);
        assert_ratio(
            &hard.wrong_class_rate,
            3.0 / 8.0,
            3,
            8,
            MetricScope::Selected,
            8,
        );
        assert_ratio(&hard.abstention_rate, 0.0, 0, 8, MetricScope::Selected, 8);
        assert_ratio(&hard.coverage, 1.0, 8, 8, MetricScope::Selected, 8);
        assert_ratio(
            &hard.selective_accuracy,
            5.0 / 8.0,
            5,
            8,
            MetricScope::Answered,
            8,
        );
        assert_ratio(
            &hard.selective_risk,
            3.0 / 8.0,
            3,
            8,
            MetricScope::Answered,
            8,
        );
        assert_ratio(
            &hard.classes[0].precision,
            2.0 / 3.0,
            2,
            3,
            MetricScope::Selected,
            8,
        );
        assert_ratio(
            &hard.classes[0].recall,
            1.0 / 2.0,
            2,
            4,
            MetricScope::Selected,
            8,
        );
        assert_ratio(
            &hard.classes[0].f1,
            4.0 / 7.0,
            4,
            7,
            MetricScope::Selected,
            8,
        );
        assert_eq!(hard.macro_f1.metric.status_value(), MetricStatus::Defined);
        assert!((hard.macro_f1.metric.value().unwrap() - 131.0 / 210.0).abs() <= 1e-12);
        assert_eq!(hard.classes[0].false_negative, 2);
        assert_eq!(hard.classes[1].false_positive, 1);
        assert_eq!(results.final_results.matrix.rows, hard.matrix.rows);
    }

    #[test]
    fn single_label_thread_and_abstention_metrics() {
        let results = evaluated(
            &["A", "B", "C"],
            &["A", "A", "B", "C"],
            &[Some("A"), None, Some("A"), Some("C")],
        );
        let hard = &results.raw;
        assert_eq!((hard.correct, hard.wrong, hard.abstained), (2, 1, 1));
        assert_ratio(&hard.accuracy, 1.0 / 2.0, 2, 4, MetricScope::Selected, 4);
        assert_ratio(&hard.coverage, 3.0 / 4.0, 3, 4, MetricScope::Selected, 4);
        assert_ratio(
            &hard.selective_accuracy,
            2.0 / 3.0,
            2,
            3,
            MetricScope::Answered,
            3,
        );
        assert_ratio(
            &hard.classes[0].coverage,
            1.0 / 2.0,
            1,
            2,
            MetricScope::Selected,
            4,
        );
        assert_ratio(
            &hard.classes[1].coverage,
            1.0,
            1,
            1,
            MetricScope::Selected,
            4,
        );
        assert_ratio(
            &hard.classes[2].coverage,
            1.0,
            1,
            1,
            MetricScope::Selected,
            4,
        );
        assert!((hard.macro_f1.metric.value().unwrap() - 1.0 / 2.0).abs() <= 1e-12);
        assert!(
            results
                .episodes
                .windows(2)
                .all(|pair| pair[0].episode_id < pair[1].episode_id)
        );
        assert!(
            results
                .episodes
                .iter()
                .all(|episode| !episode.observations.is_empty())
        );
        assert!(
            results
                .episodes
                .iter()
                .all(|episode| episode.source_id.as_str() == "source")
        );
        let first = &results.episodes[0];
        assert!(first.raw_correct && first.final_correct);
        assert!(matches!(first.raw_outcome, SingleLabelDecision::Label(_)));
        assert!(matches!(first.final_outcome, SingleLabelDecision::Label(_)));
        assert!(first.observations.values().contains_key("native_score"));
        assert!(!format!("{:?}", results.episodes).contains("opaque"));
    }

    #[test]
    fn single_label_literal_abstain_and_undefined_cases() {
        let identity = evaluated(
            &["A", "B", "C"],
            &["A", "B", "C"],
            &[Some("A"), Some("B"), Some("C")],
        );
        assert_eq!(
            identity.raw.matrix.rows,
            vec![vec![1, 0, 0, 0], vec![0, 1, 0, 0], vec![0, 0, 1, 0]]
        );
        assert_ratio(&identity.raw.accuracy, 1.0, 3, 3, MetricScope::Selected, 3);
        assert!((identity.raw.macro_f1.metric.value().unwrap() - 1.0).abs() <= 1e-12);

        let literal = evaluated(
            &["A", "ABSTAIN"],
            &["A", "ABSTAIN", "A"],
            &[Some("ABSTAIN"), Some("ABSTAIN"), None],
        );
        assert!(matches!(
            literal.raw.matrix.columns[1],
            SingleLabelMatrixColumn::Label(_)
        ));
        assert!(matches!(
            literal.raw.matrix.columns[2],
            SingleLabelMatrixColumn::Abstention
        ));
        assert_eq!(literal.raw.matrix.rows, vec![vec![0, 1, 1], vec![0, 1, 0]]);
        assert_eq!(literal.raw.wrong, 1);

        let observed = evaluated(&["A", "B", "C"], &["A"], &[Some("A")]);
        assert_eq!(
            observed.raw.macro_f1.metric.status_value(),
            MetricStatus::ContainsUndefinedClasses
        );
        assert_eq!(observed.raw.macro_f1.undefined_classes.len(), 2);
        assert!((observed.raw.macro_f1.metric.value().unwrap() - 1.0 / 3.0).abs() <= 1e-12);

        let missed = evaluated(&["A", "B"], &["A"], &[Some("B")]);
        assert_eq!(
            missed.raw.classes[0].precision.status_value(),
            MetricStatus::UndefinedZeroDenominator
        );
        assert_ratio(
            &missed.raw.classes[0].f1,
            0.0,
            0,
            1,
            MetricScope::Selected,
            1,
        );

        let abstained = evaluated(&["A", "B"], &["A", "B"], &[None, None]);
        assert_eq!(
            abstained.raw.selective_accuracy.status_value(),
            MetricStatus::NoAnsweredPredictions
        );
        assert_eq!(
            serde_json::to_value(&abstained.raw.selective_accuracy).unwrap()["status"],
            "no_answered_predictions"
        );
        assert_ratio(&abstained.raw.accuracy, 0.0, 0, 2, MetricScope::Selected, 2);

        let empty = evaluated(&["A", "B"], &[], &[]);
        assert_eq!(empty.raw.accuracy.status_value(), MetricStatus::NoData);
        assert_eq!(
            empty.raw.macro_f1.metric.status_value(),
            MetricStatus::NoData
        );
    }

    #[test]
    fn single_label_categorical_loss_local() {
        let finite = evaluated_with_signals(
            &["A", "B", "C"],
            &["A"],
            &[Some("A")],
            &[vec![0.7, 0.2, 0.1]],
            None,
        );
        assert!((finite.probability.log_loss.value().unwrap() + 0.7_f64.ln()).abs() <= 1e-12);
        assert!((finite.probability.brier_score.value().unwrap() - 0.14).abs() <= 1e-12);
        assert_ratio(
            &finite.probability.argmax_accuracy,
            1.0,
            1,
            1,
            MetricScope::Selected,
            1,
        );
        let evidence = finite.episodes[0].probability.as_ref().unwrap();
        assert_eq!(evidence.submitted, vec![0.7, 0.2, 0.1]);
        assert_ne!(evidence.working, evidence.submitted);
        assert!((evidence.working[0] - 0.7).abs() <= 1e-12);
        assert!((evidence.chosen_probability.unwrap() - 0.7).abs() <= 1e-12);
        assert_eq!(evidence.choice_argmax_disagreement, Some(false));

        let infinite = evaluated_with_signals(
            &["A", "B", "C"],
            &["B"],
            &[Some("A")],
            &[vec![1.0, 0.0, 0.0]],
            None,
        );
        assert_eq!(
            infinite.probability.log_loss.status_value(),
            MetricStatus::PositiveInfinity
        );
        assert_eq!(
            infinite.probability.log_loss.special_value(),
            Some("+infinity")
        );
        assert_eq!(
            serde_json::to_value(&infinite.probability.log_loss).unwrap()["special_value"],
            "+infinity"
        );
        assert!((infinite.probability.brier_score.value().unwrap() - 2.0).abs() <= 1e-12);
    }

    #[test]
    fn single_label_signal_bins_local() {
        let results = evaluated_with_signals(
            &["A", "B"],
            &["A", "B"],
            &[Some("B"), None],
            &[vec![0.5, 0.5], vec![0.8, 0.2]],
            Some(&[0.4, 0.8]),
        );
        let probability = &results.probability;
        assert_eq!(probability.raw_answered_count, 1);
        assert_eq!(probability.choice_argmax_disagreement_count, Some(1));
        let first = results.episodes[0].probability.as_ref().unwrap();
        assert_eq!(first.choice_argmax_disagreement, Some(true));
        assert_eq!(first.max_probability, 0.5);
        assert!(first.argmax_correct);
        let abstained = results.episodes[1].probability.as_ref().unwrap();
        assert_eq!(abstained.chosen_probability, None);
        assert_eq!(abstained.choice_argmax_disagreement, None);
        assert_eq!(results.episodes[1].reported_confidence, Some(0.8));
        let max = &results.signals.maximum_probability;
        assert_eq!(max.status, MetricStatus::Defined);
        assert_eq!(max.scope, MetricScope::Selected);
        assert_eq!(max.population_count, 2);
        assert_eq!(max.included_ids.len(), 2);
        assert!(max.excluded_ids.is_empty());
        assert_eq!(max.bins[5].count, 1);
        assert_eq!(max.bins[5].index, 5);
        assert_eq!(max.bins[5].lower, 0.5);
        assert_eq!(max.bins[5].upper, 0.6);
        assert_eq!(max.bins[5].mean_signal, Some(0.5));
        assert_eq!(max.bins[5].empirical_accuracy, Some(1.0));
        assert_eq!(max.bins[8].count, 1);
        assert_eq!(max.bins[8].mean_signal, Some(0.8));
        assert_eq!(max.bins[8].empirical_accuracy, Some(0.0));
        assert!(max.bins[9].upper_inclusive);
        assert!((results.signals.top_label_ece.value().unwrap() - 0.65).abs() <= 1e-12);
        let confidence = &results.signals.confidence;
        assert_eq!(confidence.scope, MetricScope::RawAnswered);
        assert_eq!(confidence.population_count, 1);
        assert_eq!(confidence.included_ids.len(), 1);
        assert_eq!(confidence.excluded_ids.len(), 1);
        assert_eq!(confidence.bins[4].count, 1);
        assert_eq!(confidence.bins[4].correct_count, 0);

        let all_abstain = evaluated_with_signals(
            &["A", "B"],
            &["A"],
            &[None],
            &[vec![0.7, 0.3]],
            Some(&[0.8]),
        );
        assert_eq!(
            all_abstain.signals.confidence.status,
            MetricStatus::NoAnsweredPredictions
        );
        assert_eq!(all_abstain.signals.confidence.excluded_ids.len(), 1);
    }

    #[test]
    fn single_label_bin_boundaries_local() {
        let expected = [
            (None, 0, Some(0)),
            (Some(0), 1, Some(1)),
            (Some(1), 2, Some(2)),
            (Some(2), 3, Some(3)),
            (Some(3), 4, Some(4)),
            (Some(4), 5, Some(5)),
            (Some(5), 6, Some(6)),
            (Some(6), 7, Some(7)),
            (Some(7), 8, Some(8)),
            (Some(9), 9, Some(9)),
            (Some(9), 9, None),
        ];
        for (boundary, (below, equal, above)) in expected.into_iter().enumerate() {
            let value = boundary as f64 / 10.0;
            assert_eq!(bin_index(value).unwrap(), equal);
            if let Some(expected) = below {
                assert_eq!(bin_index(value.next_down()).unwrap(), expected);
            }
            if let Some(expected) = above {
                assert_eq!(bin_index(value.next_up()).unwrap(), expected);
            }
        }
        assert_eq!(bin_index(0.0).unwrap(), 0);
        assert_eq!(bin_index(1.0).unwrap(), 9);
        let absent = evaluated(&["A", "B"], &["A"], &[Some("A")]);
        assert_eq!(
            absent.probability.log_loss.status_value(),
            MetricStatus::NotApplicable
        );
        assert_eq!(
            absent.signals.maximum_probability.status,
            MetricStatus::NotApplicable
        );
        assert_eq!(
            absent.signals.confidence.status,
            MetricStatus::NotApplicable
        );
    }
}
