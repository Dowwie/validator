mod wire;

use std::collections::{HashMap, HashSet};

use serde::de::DeserializeOwned;

use crate::model::{
    TaskDefinition, ValidatedTask,
    common::{
        ArtifactDigest, Episode, EpisodeId, EvaluationConfig, EvaluationRole, LabelVocabulary,
        Observation, ObservationDefinition, ObservationKind, ObservationSet, Outcome, Population,
        Prediction, PreparationDescriptor, SourceDefinition, SourceId, SourceKind,
    },
    multi_label::{
        LabelMarginals, MultiLabelAlignedRow, MultiLabelEvaluation, MultiLabelOutput,
        MultiLabelPolicy,
    },
    single_label::{
        AlignedRow, CategoricalDistribution, ReportedConfidence, SingleLabelEvaluation,
        SingleLabelOutput, SingleLabelPolicy, SingleLabelRejectionSignal, ValidatedEvaluation,
        signal_availability_flags, validate_scored_choice,
    },
};
use crate::{Diagnostic, DiagnosticCode, Result};

/// Exact submitted bytes paired with their strict private wire representation.
#[derive(Debug)]
pub(crate) struct Decoded<T> {
    value: T,
}

impl<T> Decoded<T> {
    /// Consumes the decoded wrapper and returns its checked wire representation.
    pub(crate) fn into_value(self) -> T {
        self.value
    }
}

/// Decodes a strict single-label golden dataset from exact input bytes.
pub(crate) fn decode_golden_dataset(bytes: &[u8]) -> Result<Decoded<wire::GoldenDataset>> {
    let decoded: Decoded<wire::GoldenDataset> = decode(bytes)?;
    wire::require_schema_version_two(decoded.value.schema_version)?;
    Ok(decoded)
}

/// Decodes a strict single-label prediction artifact from exact input bytes.
pub(crate) fn decode_prediction_artifact(
    bytes: &[u8],
) -> Result<Decoded<wire::PredictionArtifact>> {
    let decoded: Decoded<wire::PredictionArtifact> = decode(bytes)?;
    wire::require_schema_version_two(decoded.value.schema_version)?;
    Ok(decoded)
}

/// Decodes a strict single-label evaluation configuration from exact input bytes.
pub(crate) fn decode_evaluation_config(bytes: &[u8]) -> Result<Decoded<wire::EvaluationConfig>> {
    let decoded: Decoded<wire::EvaluationConfig> = decode(bytes)?;
    wire::require_schema_version_two(decoded.value.schema_version)?;
    Ok(decoded)
}

fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<Decoded<T>> {
    wire::reject_duplicate_keys(bytes)?;
    let value =
        serde_json::from_slice(bytes).map_err(|_| Diagnostic::for_code(DiagnosticCode::Schema))?;

    Ok(Decoded { value })
}

/// Compares a prediction-declared digest with caller-supplied exact dataset bytes.
pub(crate) fn validate_dataset_digest(declared: &str, actual: ArtifactDigest) -> Result<()> {
    let declared = ArtifactDigest::try_from(declared)
        .map_err(|_| Diagnostic::for_code(DiagnosticCode::Provenance))?;
    if declared != actual {
        return Err(Diagnostic::for_code(DiagnosticCode::Provenance));
    }
    Ok(())
}

/// Decodes and admits a single-label evaluation before any scoring or selection uses it.
pub(crate) fn validate_single_label(
    dataset_bytes: &[u8],
    prediction_bytes: &[u8],
    config_bytes: &[u8],
    dataset_digest: ArtifactDigest,
) -> Result<ValidatedEvaluation> {
    let dataset = decode_golden_dataset(dataset_bytes)?.into_value();
    let artifact = decode_prediction_artifact(prediction_bytes)?.into_value();
    let config = decode_evaluation_config(config_bytes)?.into_value();
    validate_dataset_digest(&artifact.dataset_sha256, dataset_digest)?;

    let task = match dataset.task {
        wire::Task::SingleLabel { labels } => TaskDefinition::single_label(labels)?,
        wire::Task::MultiLabel { .. } => return Err(Diagnostic::for_code(DiagnosticCode::Config)),
    };
    let vocabulary = task.vocabulary().clone();
    let mut expected = HashMap::with_capacity(dataset.episodes.len());
    for episode in dataset.episodes {
        let id = EpisodeId::try_from(episode.id.as_str())?;
        let label = match episode.expected {
            wire::Target::Class { label } => vocabulary.lookup(&label).ok_or_else(|| {
                Diagnostic::for_code(DiagnosticCode::Label).with_affected_ids([id])
            })?,
            wire::Target::Labels { .. } => {
                return Err(Diagnostic::for_code(DiagnosticCode::Config));
            }
        };
        if expected.insert(id, Episode::new(id, label)).is_some() {
            return Err(Diagnostic::for_code(DiagnosticCode::DuplicateId).with_affected_ids([id]));
        }
    }

    let sources = admit_sources(artifact.sources)?;
    let mut predictions = HashMap::with_capacity(artifact.predictions.len());
    for prediction in artifact.predictions {
        let id = EpisodeId::try_from(prediction.id.as_str())?;
        let source_id = SourceId::try_from(prediction.source_id.as_str())?;
        let source = sources.get(&source_id).ok_or_else(|| {
            Diagnostic::for_code(DiagnosticCode::Provenance).with_affected_ids([id])
        })?;
        let output = admit_single_label_output(
            prediction.outcome,
            prediction.probabilities,
            prediction.confidence,
            &vocabulary,
        )
        .map_err(|error| error.with_affected_ids([id]))?;
        if source.kind() == SourceKind::ScoredChoice {
            validate_scored_choice(source, &output, &vocabulary)
                .map_err(|error| error.with_affected_ids([id]))?;
        }
        let observations =
            admit_observations(prediction.observations, source.observation_definitions())
                .map_err(|error| error.with_affected_ids([id]))?;
        if predictions
            .insert(id, Prediction::new(id, source_id, output, observations))
            .is_some()
        {
            return Err(Diagnostic::for_code(DiagnosticCode::DuplicateId).with_affected_ids([id]));
        }
    }
    let config = admit_single_label_evaluation_config(config, &predictions)?;
    let signals = signal_availability_flags(
        &predictions
            .values()
            .map(|prediction| {
                (
                    prediction.output().has_categorical(),
                    prediction.output().has_confidence(),
                )
            })
            .collect::<Vec<_>>(),
    )?;

    let selected = select_population(config.requested_episode_ids(), &expected)?;
    let selected_ids: HashSet<_> = selected.iter().copied().collect();
    validate_alignment(&predictions, &selected_ids)?;
    let unselected = expected
        .keys()
        .filter(|id| !selected_ids.contains(id))
        .copied()
        .collect();
    let population = Population::new(dataset_digest, config, selected.clone(), unselected)?;
    let mut rows = Vec::with_capacity(selected.len());
    for id in selected {
        let episode = expected
            .remove(&id)
            .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))?;
        let prediction = predictions
            .remove(&id)
            .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Alignment))?;
        rows.push(AlignedRow::new(episode, prediction, &vocabulary)?);
    }
    Ok(ValidatedEvaluation::SingleLabel(
        SingleLabelEvaluation::new(vocabulary, sources, population, signals, rows)?,
    ))
}

/// Decodes and admits either declared task kind through the closed validation boundary.
pub(crate) fn validate(
    dataset_bytes: &[u8],
    prediction_bytes: &[u8],
    config_bytes: &[u8],
    dataset_digest: ArtifactDigest,
) -> Result<ValidatedTask> {
    let dataset = decode_golden_dataset(dataset_bytes)?.into_value();
    match dataset.task {
        wire::Task::SingleLabel { .. } => {
            let crate::model::single_label::ValidatedEvaluation::SingleLabel(evaluation) =
                validate_single_label(
                    dataset_bytes,
                    prediction_bytes,
                    config_bytes,
                    dataset_digest,
                )?;
            Ok(ValidatedTask::SingleLabel(evaluation))
        }
        wire::Task::MultiLabel { .. } => validate_multi_label(
            dataset_bytes,
            prediction_bytes,
            config_bytes,
            dataset_digest,
        )
        .map(ValidatedTask::MultiLabel),
    }
}

/// Decodes and admits a multi-label evaluation before any scoring or selection uses it.
pub(crate) fn validate_multi_label(
    dataset_bytes: &[u8],
    prediction_bytes: &[u8],
    config_bytes: &[u8],
    dataset_digest: ArtifactDigest,
) -> Result<MultiLabelEvaluation> {
    let dataset = decode_golden_dataset(dataset_bytes)?.into_value();
    let artifact = decode_prediction_artifact(prediction_bytes)?.into_value();
    let config = decode_evaluation_config(config_bytes)?.into_value();
    validate_dataset_digest(&artifact.dataset_sha256, dataset_digest)?;

    let task = match dataset.task {
        wire::Task::MultiLabel { labels } => TaskDefinition::multi_label(labels)?,
        wire::Task::SingleLabel { .. } => return Err(Diagnostic::for_code(DiagnosticCode::Config)),
    };
    let vocabulary = task.vocabulary().clone();
    let mut expected = HashMap::with_capacity(dataset.episodes.len());
    for episode in dataset.episodes {
        let id = EpisodeId::try_from(episode.id.as_str())?;
        let target = match episode.expected {
            wire::Target::Labels { labels } => vocabulary
                .label_set(labels.iter().map(String::as_str))
                .map_err(|error| error.with_affected_ids([id]))?,
            wire::Target::Class { .. } => return Err(Diagnostic::for_code(DiagnosticCode::Config)),
        };
        if expected.insert(id, Episode::new(id, target)).is_some() {
            return Err(Diagnostic::for_code(DiagnosticCode::DuplicateId).with_affected_ids([id]));
        }
    }

    let sources = admit_sources(artifact.sources)?;
    if sources
        .values()
        .any(|source| source.kind() != SourceKind::Classifier)
    {
        return Err(Diagnostic::for_code(DiagnosticCode::Config));
    }
    let mut predictions = HashMap::with_capacity(artifact.predictions.len());
    for prediction in artifact.predictions {
        let id = EpisodeId::try_from(prediction.id.as_str())?;
        let source_id = SourceId::try_from(prediction.source_id.as_str())?;
        let source = sources.get(&source_id).ok_or_else(|| {
            Diagnostic::for_code(DiagnosticCode::Provenance).with_affected_ids([id])
        })?;
        if source.kind() != SourceKind::Classifier {
            return Err(Diagnostic::for_code(DiagnosticCode::Config));
        }
        let output = admit_multi_label_output(
            prediction.outcome,
            prediction.probabilities,
            prediction.confidence,
            &vocabulary,
        )
        .map_err(|error| error.with_affected_ids([id]))?;
        let observations =
            admit_observations(prediction.observations, source.observation_definitions())
                .map_err(|error| error.with_affected_ids([id]))?;
        if predictions
            .insert(id, Prediction::new(id, source_id, output, observations))
            .is_some()
        {
            return Err(Diagnostic::for_code(DiagnosticCode::DuplicateId).with_affected_ids([id]));
        }
    }
    let config = admit_multi_label_evaluation_config(config, &vocabulary, &predictions)?;
    let marginals_available = predictions
        .values()
        .next()
        .is_some_and(|prediction| prediction.output().marginals().is_some());
    if predictions
        .values()
        .any(|prediction| prediction.output().marginals().is_some() != marginals_available)
    {
        return Err(Diagnostic::for_code(DiagnosticCode::Probability));
    }

    let selected = select_population(config.requested_episode_ids(), &expected)?;
    let selected_ids: HashSet<_> = selected.iter().copied().collect();
    validate_alignment(&predictions, &selected_ids)?;
    let unselected = expected
        .keys()
        .filter(|id| !selected_ids.contains(id))
        .copied()
        .collect();
    let population = Population::new(dataset_digest, config, selected.clone(), unselected)?;
    let mut rows = Vec::with_capacity(selected.len());
    for id in selected {
        let episode = expected
            .remove(&id)
            .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Invariant))?;
        let prediction = predictions
            .remove(&id)
            .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Alignment))?;
        rows.push(MultiLabelAlignedRow::new(episode, prediction, &vocabulary)?);
    }
    MultiLabelEvaluation::new(vocabulary, sources, population, marginals_available, rows)
}

fn admit_single_label_evaluation_config(
    config: wire::EvaluationConfig,
    predictions: &HashMap<EpisodeId, Prediction<SingleLabelOutput>>,
) -> Result<EvaluationConfig<SingleLabelPolicy>> {
    let role = match config.role {
        wire::EvaluationRole::Development => EvaluationRole::Development,
        wire::EvaluationRole::HeldOut => EvaluationRole::HeldOut,
    };
    let policy = match config.decision {
        wire::Decision::AsRecorded => SingleLabelPolicy::AsRecorded,
        wire::Decision::RejectBelow { signal, minimum } => {
            let minimum = number_as_f64(minimum, DiagnosticCode::Config)?;
            if !(0.0..=1.0).contains(&minimum)
                || predictions.values().any(|prediction| {
                    let output = prediction.output();
                    output.outcome().answered_target().is_none()
                        || match signal {
                            wire::RejectionSignal::Confidence => !output.has_confidence(),
                            wire::RejectionSignal::MaxProbability => !output.has_categorical(),
                        }
                })
            {
                return Err(Diagnostic::for_code(DiagnosticCode::Config));
            }
            SingleLabelPolicy::RejectBelow {
                signal: match signal {
                    wire::RejectionSignal::Confidence => SingleLabelRejectionSignal::Confidence,
                    wire::RejectionSignal::MaxProbability => {
                        SingleLabelRejectionSignal::MaxProbability
                    }
                },
                minimum,
            }
        }
        wire::Decision::LabelThresholds { .. } => {
            return Err(Diagnostic::for_code(DiagnosticCode::Config));
        }
    };
    let parent_run_id = config
        .parent_run_id
        .as_deref()
        .map(crate::model::common::RunId::try_from)
        .transpose()?;
    let episode_ids = config
        .episode_ids
        .map(|ids| {
            ids.into_iter()
                .map(|id| EpisodeId::try_from(id.as_str()))
                .collect::<Result<Vec<_>>>()
        })
        .transpose()?;
    EvaluationConfig::new_with_policy(config.population, role, parent_run_id, episode_ids, policy)
}

fn admit_multi_label_evaluation_config(
    config: wire::EvaluationConfig,
    vocabulary: &LabelVocabulary,
    predictions: &HashMap<EpisodeId, Prediction<MultiLabelOutput>>,
) -> Result<EvaluationConfig<MultiLabelPolicy>> {
    let role = match config.role {
        wire::EvaluationRole::Development => EvaluationRole::Development,
        wire::EvaluationRole::HeldOut => EvaluationRole::HeldOut,
    };
    let policy = match config.decision {
        wire::Decision::AsRecorded => MultiLabelPolicy::AsRecorded,
        wire::Decision::LabelThresholds { thresholds } => {
            if thresholds.len() != vocabulary.len()
                || predictions.values().any(|prediction| {
                    let output = prediction.output();
                    output.outcome().answered_target().is_none() || output.marginals().is_none()
                })
            {
                return Err(Diagnostic::for_code(DiagnosticCode::Config));
            }
            let thresholds = vocabulary
                .labels()
                .map(|label| {
                    let value = thresholds
                        .get(label)
                        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Config))?;
                    let value = number_ref_as_f64(value, DiagnosticCode::Config)?;
                    if !(0.0..=1.0).contains(&value) {
                        return Err(Diagnostic::for_code(DiagnosticCode::Config));
                    }
                    Ok((label.to_owned(), value))
                })
                .collect::<Result<std::collections::BTreeMap<_, _>>>()?;
            MultiLabelPolicy::LabelThresholds { thresholds }
        }
        wire::Decision::RejectBelow { .. } => {
            return Err(Diagnostic::for_code(DiagnosticCode::Config));
        }
    };
    let parent_run_id = config
        .parent_run_id
        .as_deref()
        .map(crate::model::common::RunId::try_from)
        .transpose()?;
    let episode_ids = config
        .episode_ids
        .map(|ids| {
            ids.into_iter()
                .map(|id| EpisodeId::try_from(id.as_str()))
                .collect::<Result<Vec<_>>>()
        })
        .transpose()?;
    EvaluationConfig::new_with_policy(config.population, role, parent_run_id, episode_ids, policy)
}

fn admit_sources(
    wire_sources: std::collections::BTreeMap<String, wire::Source>,
) -> Result<HashMap<SourceId, SourceDefinition>> {
    let mut sources = HashMap::with_capacity(wire_sources.len());
    for (id, source) in wire_sources {
        let source_id = SourceId::try_from(id.as_str())?;
        let kind = match source.kind {
            wire::SourceKind::Classifier => SourceKind::Classifier,
            wire::SourceKind::ScoredChoice => SourceKind::ScoredChoice,
        };
        let evidence = source.evidence.unwrap_or_default();
        let preparation = source
            .preparation
            .map(|preparation| {
                let indices = preparation
                    .evidence_indices
                    .into_iter()
                    .map(|index| {
                        usize::try_from(index)
                            .map_err(|_| Diagnostic::for_code(DiagnosticCode::Provenance))
                    })
                    .collect::<Result<Vec<_>>>()?;
                PreparationDescriptor::new_raw(
                    preparation.method,
                    preparation.version,
                    preparation.configuration.0,
                    indices,
                    evidence.len(),
                )
            })
            .transpose()?;
        let definitions = source
            .observation_definitions
            .unwrap_or_default()
            .into_iter()
            .map(|(name, definition)| {
                let kind = match definition.kind {
                    wire::ObservationKind::Scalar => ObservationKind::Scalar,
                    wire::ObservationKind::Bernoulli => ObservationKind::Bernoulli,
                    wire::ObservationKind::ReportedConfidence => {
                        ObservationKind::ReportedConfidence
                    }
                    wire::ObservationKind::Categorical => ObservationKind::Categorical,
                    wire::ObservationKind::LabelMarginals => ObservationKind::LabelMarginals,
                };
                Ok((
                    name,
                    ObservationDefinition::new(
                        kind,
                        definition.description,
                        definition.question_id,
                    )?,
                ))
            })
            .collect::<Result<HashMap<_, _>>>()?;
        let definition = SourceDefinition::new_raw(
            kind,
            source.model,
            source.configuration.0,
            source.question_id,
            evidence,
            definitions,
            preparation,
        )?;
        if sources.insert(source_id, definition).is_some() {
            return Err(Diagnostic::for_code(DiagnosticCode::DuplicateId));
        }
    }
    Ok(sources)
}

fn admit_single_label_output(
    wire_outcome: wire::PredictionOutcome,
    wire_probabilities: Option<wire::Probabilities>,
    wire_confidence: Option<wire::JsonNumber>,
    vocabulary: &LabelVocabulary,
) -> Result<SingleLabelOutput> {
    let outcome = match wire_outcome {
        wire::PredictionOutcome::Class { label } => Outcome::answered(
            vocabulary
                .lookup(&label)
                .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Label))?,
        ),
        wire::PredictionOutcome::Abstention { reason } => Outcome::abstained(reason)?,
        wire::PredictionOutcome::Labels { .. } => {
            return Err(Diagnostic::for_code(DiagnosticCode::Config));
        }
    };
    let categorical = wire_probabilities
        .map(|probabilities| match probabilities {
            wire::Probabilities::Categorical { values } => {
                let values = values
                    .into_iter()
                    .map(|(label, value)| {
                        Ok((label, number_as_f64(value, DiagnosticCode::Probability)?))
                    })
                    .collect::<Result<HashMap<_, _>>>()?;
                CategoricalDistribution::new(&values, vocabulary)
            }
            wire::Probabilities::LabelMarginals { .. } => {
                Err(Diagnostic::for_code(DiagnosticCode::Probability))
            }
        })
        .transpose()?;
    let confidence = wire_confidence
        .map(|value| ReportedConfidence::new(number_as_f64(value, DiagnosticCode::Confidence)?))
        .transpose()?;
    Ok(SingleLabelOutput::new(outcome, categorical, confidence))
}

fn admit_multi_label_output(
    wire_outcome: wire::PredictionOutcome,
    wire_probabilities: Option<wire::Probabilities>,
    wire_confidence: Option<wire::JsonNumber>,
    vocabulary: &LabelVocabulary,
) -> Result<MultiLabelOutput> {
    if wire_confidence.is_some() {
        return Err(Diagnostic::for_code(DiagnosticCode::Confidence));
    }
    let outcome = match wire_outcome {
        wire::PredictionOutcome::Labels { labels } => {
            Outcome::answered(vocabulary.label_set(labels.iter().map(String::as_str))?)
        }
        wire::PredictionOutcome::Abstention { reason } => Outcome::abstained(reason)?,
        wire::PredictionOutcome::Class { .. } => {
            return Err(Diagnostic::for_code(DiagnosticCode::Config));
        }
    };
    let marginals = wire_probabilities
        .map(|probabilities| match probabilities {
            wire::Probabilities::LabelMarginals { values } => LabelMarginals::new(
                &number_map(values, DiagnosticCode::Probability)?,
                vocabulary,
            ),
            wire::Probabilities::Categorical { .. } => {
                Err(Diagnostic::for_code(DiagnosticCode::Probability))
            }
        })
        .transpose()?;
    Ok(MultiLabelOutput::new(outcome, marginals))
}

fn admit_observations(
    wire_observations: Option<std::collections::BTreeMap<String, wire::Observation>>,
    definitions: &HashMap<String, ObservationDefinition>,
) -> Result<ObservationSet> {
    let values = wire_observations
        .unwrap_or_default()
        .into_iter()
        .map(|(name, observation)| {
            let observation = match observation {
                wire::Observation::Scalar { value } => Observation::scalar(
                    ObservationKind::Scalar,
                    number_as_f64(value, DiagnosticCode::Observation)?,
                ),
                wire::Observation::Bernoulli { value } => Observation::scalar(
                    ObservationKind::Bernoulli,
                    number_as_f64(value, DiagnosticCode::Observation)?,
                ),
                wire::Observation::ReportedConfidence { value } => Observation::scalar(
                    ObservationKind::ReportedConfidence,
                    number_as_f64(value, DiagnosticCode::Observation)?,
                ),
                wire::Observation::Categorical { values } => Observation::vector(
                    ObservationKind::Categorical,
                    number_map(values, DiagnosticCode::Observation)?,
                ),
                wire::Observation::LabelMarginals { values } => Observation::vector(
                    ObservationKind::LabelMarginals,
                    number_map(values, DiagnosticCode::Observation)?,
                ),
            }?;
            Ok((name, observation))
        })
        .collect::<Result<HashMap<_, _>>>()?;
    ObservationSet::new(values, definitions)
}

fn number_map(
    values: std::collections::BTreeMap<String, wire::JsonNumber>,
    code: DiagnosticCode,
) -> Result<HashMap<String, f64>> {
    values
        .into_iter()
        .map(|(name, value)| Ok((name, number_as_f64(value, code)?)))
        .collect()
}

fn number_as_f64(value: wire::JsonNumber, code: DiagnosticCode) -> Result<f64> {
    number_ref_as_f64(&value, code)
}

fn number_ref_as_f64(value: &wire::JsonNumber, code: DiagnosticCode) -> Result<f64> {
    value
        .0
        .get()
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite())
        .ok_or_else(|| Diagnostic::for_code(code))
}

fn validate_alignment<T>(
    predictions: &HashMap<EpisodeId, T>,
    selected: &HashSet<EpisodeId>,
) -> Result<()> {
    let predicted = predictions.keys().copied().collect::<HashSet<_>>();
    if &predicted != selected {
        return Err(Diagnostic::for_code(DiagnosticCode::Alignment)
            .with_affected_ids(predicted.symmetric_difference(selected).copied()));
    }
    Ok(())
}

fn select_population<T>(
    requested: Option<&[EpisodeId]>,
    expected: &HashMap<EpisodeId, T>,
) -> Result<Vec<EpisodeId>> {
    let Some(requested) = requested else {
        let mut all: Vec<_> = expected.keys().copied().collect();
        all.sort();
        return Ok(all);
    };
    let mut selected = Vec::with_capacity(requested.len());
    for id in requested {
        if !expected.contains_key(id) {
            return Err(Diagnostic::for_code(DiagnosticCode::Id).with_affected_ids([*id]));
        }
        selected.push(*id);
    }
    selected.sort();
    Ok(selected)
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::{
        decode_evaluation_config, decode_golden_dataset, decode_prediction_artifact, validate,
        validate_multi_label, validate_single_label,
    };
    use crate::DiagnosticCode;

    const GOLDEN: &[u8] = br#"{
        "schema_version": 2,
        "task": {"kind": "single_label", "labels": ["left", "right"]},
        "episodes": [{
            "id": "01995c20-7d00-7000-8000-000000000001",
            "expected": {"type": "class", "label": "left"},
            "input": {"number": 9007199254740993}
        }]
    }"#;

    const PREDICTIONS: &[u8] = br#"{
        "schema_version": 2,
        "dataset_sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        "sources": {
            "source": {
                "kind": "classifier",
                "model": "example",
                "configuration": {"temperature": 0},
                "observation_definitions": {
                    "scalar": {"kind": "scalar", "description": "native score"},
                    "bernoulli": {"kind": "bernoulli", "description": "native decision"},
                    "confidence": {"kind": "reported_confidence", "description": "reported confidence"},
                    "categorical": {"kind": "categorical", "description": "native distribution"},
                    "marginals": {"kind": "label_marginals", "description": "native marginals"}
                }
            }
        },
        "predictions": [{
            "id": "01995c20-7d00-7000-8000-000000000001",
            "source_id": "source",
            "outcome": {"type": "class", "label": "left"},
            "probabilities": {"kind": "categorical", "values": {"left": 1, "right": 0}},
            "confidence": 1,
            "observations": {
                "scalar": {"kind": "scalar", "value": 1.33},
                "bernoulli": {"kind": "bernoulli", "value": 1},
                "confidence": {"kind": "reported_confidence", "value": 0.9},
                "categorical": {"kind": "categorical", "values": {"a": 0.2, "b": 0.8}},
                "marginals": {"kind": "label_marginals", "values": {"left": 0.8, "right": 0.2}}
            }
        }]
    }"#;

    const CONFIG: &[u8] = br#"{
        "schema_version": 2,
        "population": "all episodes",
        "role": "development",
        "decision": {"type": "as_recorded"}
    }"#;

    fn digest() -> crate::model::common::ArtifactDigest {
        crate::model::common::ArtifactDigest::try_from(
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        )
        .unwrap()
    }

    fn multi_dataset() -> Value {
        json!({
            "schema_version": 2,
            "task": {"kind": "multi_label", "labels": ["alpha", "beta"]},
            "episodes": [
                {"id": "01995c20-7d00-7000-8000-000000000001", "expected": {"type": "labels", "labels": ["beta", "alpha"]}, "input": null},
                {"id": "01995c20-7d00-7000-8000-000000000002", "expected": {"type": "labels", "labels": []}, "input": {"opaque": true}},
                {"id": "01995c20-7d00-7000-8000-000000000003", "expected": {"type": "labels", "labels": ["alpha"]}, "input": {"opaque": false}}
            ]
        })
    }

    fn multi_artifact() -> Value {
        let marginals = json!({"kind": "label_marginals", "values": {"beta": 0.8, "alpha": 0.9}});
        json!({
            "schema_version": 2,
            "dataset_sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "sources": {
                "source": {
                    "kind": "classifier",
                    "model": "example",
                    "configuration": {"opaque": "kept"},
                    "observation_definitions": {
                        "reported": {"kind": "reported_confidence", "description": "retained producer signal"}
                    }
                }
            },
            "predictions": [
                {"id": "01995c20-7d00-7000-8000-000000000002", "source_id": "source", "outcome": {"type": "labels", "labels": []}, "probabilities": marginals},
                {"id": "01995c20-7d00-7000-8000-000000000003", "source_id": "source", "outcome": {"type": "abstention", "reason": "review"}, "probabilities": marginals},
                {"id": "01995c20-7d00-7000-8000-000000000001", "source_id": "source", "outcome": {"type": "labels", "labels": ["beta"]}, "probabilities": marginals, "observations": {"reported": {"kind": "reported_confidence", "value": 0.6}}}
            ]
        })
    }

    fn multi_config() -> Value {
        json!({
            "schema_version": 2,
            "population": "all declared episodes",
            "role": "development",
            "episode_ids": [
                "01995c20-7d00-7000-8000-000000000003",
                "01995c20-7d00-7000-8000-000000000001",
                "01995c20-7d00-7000-8000-000000000002"
            ],
            "decision": {"type": "as_recorded"}
        })
    }

    fn admit_multi(
        dataset: &Value,
        artifact: &Value,
        config: &Value,
    ) -> crate::Result<crate::model::multi_label::MultiLabelEvaluation> {
        validate_multi_label(
            &serde_json::to_vec(dataset).unwrap(),
            &serde_json::to_vec(artifact).unwrap(),
            &serde_json::to_vec(config).unwrap(),
            digest(),
        )
    }

    fn assert_multi_code(
        dataset: &Value,
        artifact: &Value,
        config: &Value,
        expected: DiagnosticCode,
    ) {
        assert_eq!(
            admit_multi(dataset, artifact, config).unwrap_err().code(),
            expected
        );
    }

    #[test]
    fn multi_label_checked_admission() {
        let dataset = multi_dataset();
        let artifact = multi_artifact();
        let config = multi_config();
        assert!(decode_golden_dataset(&serde_json::to_vec(&dataset).unwrap()).is_ok());
        assert!(decode_prediction_artifact(&serde_json::to_vec(&artifact).unwrap()).is_ok());
        assert!(decode_evaluation_config(&serde_json::to_vec(&config).unwrap()).is_ok());
        let crate::model::ValidatedTask::MultiLabel(evaluation) = validate(
            &serde_json::to_vec(&dataset).unwrap(),
            &serde_json::to_vec(&artifact).unwrap(),
            &serde_json::to_vec(&config).unwrap(),
            digest(),
        )
        .unwrap() else {
            panic!("declared multi-label task must use the multi-label dispatch");
        };

        assert_eq!(evaluation.population().selected_count(), 3);
        assert_eq!(evaluation.rows().len(), 3);
        assert_eq!(
            evaluation
                .rows()
                .iter()
                .map(|row| row.episode_id().to_string())
                .collect::<Vec<_>>(),
            vec![
                "01995c20-7d00-7000-8000-000000000001",
                "01995c20-7d00-7000-8000-000000000002",
                "01995c20-7d00-7000-8000-000000000003"
            ]
        );
        assert_eq!(
            evaluation.rows()[0].expected().labels().collect::<Vec<_>>(),
            vec!["alpha", "beta"]
        );
        assert!(evaluation.rows()[1].expected().labels().next().is_none());
        assert!(
            evaluation.rows()[2]
                .prediction()
                .output()
                .outcome()
                .answered_target()
                .is_none()
        );
        assert_eq!(
            evaluation.rows()[0]
                .prediction()
                .output()
                .marginals()
                .unwrap()
                .values(),
            &[0.9, 0.8]
        );
        assert!(matches!(
            evaluation.rows()[0]
                .prediction()
                .observations()
                .values()
                .get("reported"),
            Some(crate::model::common::Observation::ReportedConfidence(0.6))
        ));

        let mut duplicate_labels = dataset.clone();
        duplicate_labels["episodes"][0]["expected"]["labels"] = json!(["alpha", "alpha"]);
        assert_multi_code(&duplicate_labels, &artifact, &config, DiagnosticCode::Label);

        let mut missing_row = artifact.clone();
        missing_row["predictions"].as_array_mut().unwrap().pop();
        assert_multi_code(&dataset, &missing_row, &config, DiagnosticCode::Alignment);

        let mut extra_row = artifact.clone();
        extra_row["predictions"]
            .as_array_mut()
            .unwrap()
            .push(json!({
                "id": "01995c20-7d00-7000-8000-000000000004",
                "source_id": "source",
                "outcome": {"type": "labels", "labels": []},
                "probabilities": {"kind": "label_marginals", "values": {"alpha": 0.9, "beta": 0.8}}
            }));
        assert_multi_code(&dataset, &extra_row, &config, DiagnosticCode::Alignment);

        let mut duplicate_row = artifact.clone();
        let first_row = duplicate_row["predictions"][0].clone();
        duplicate_row["predictions"]
            .as_array_mut()
            .unwrap()
            .push(first_row);
        assert_multi_code(
            &dataset,
            &duplicate_row,
            &config,
            DiagnosticCode::DuplicateId,
        );

        let mut missing_marginal = artifact.clone();
        missing_marginal["predictions"][0]["probabilities"]["values"]
            .as_object_mut()
            .unwrap()
            .remove("beta");
        assert_multi_code(
            &dataset,
            &missing_marginal,
            &config,
            DiagnosticCode::Probability,
        );

        let mut extra_marginal = artifact.clone();
        extra_marginal["predictions"][0]["probabilities"]["values"]["extra"] = json!(0.1);
        assert_multi_code(
            &dataset,
            &extra_marginal,
            &config,
            DiagnosticCode::Probability,
        );

        let mut top_level_confidence = artifact.clone();
        top_level_confidence["predictions"][0]["confidence"] = json!(0.6);
        assert_multi_code(
            &dataset,
            &top_level_confidence,
            &config,
            DiagnosticCode::Confidence,
        );

        let mut categorical = artifact.clone();
        categorical["predictions"][0]["probabilities"]["kind"] = json!("categorical");
        assert_multi_code(&dataset, &categorical, &config, DiagnosticCode::Probability);

        let mut scored_choice = artifact.clone();
        scored_choice["sources"]["source"]["kind"] = json!("scored_choice");
        scored_choice["sources"]["source"]["question_id"] = json!("question");
        assert_multi_code(&dataset, &scored_choice, &config, DiagnosticCode::Config);

        let empty_dataset = json!({
            "schema_version": 2,
            "task": {"kind": "multi_label", "labels": ["alpha"]},
            "episodes": [],
        });
        let empty_artifact = json!({
            "schema_version": 2,
            "dataset_sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "sources": {"unused": {"kind": "scored_choice", "model": "example", "configuration": {}, "question_id": "question"}},
            "predictions": [],
        });
        let empty_config = json!({
            "schema_version": 2,
            "population": "empty",
            "role": "development",
            "decision": {"type": "as_recorded"},
        });
        assert_multi_code(
            &empty_dataset,
            &empty_artifact,
            &empty_config,
            DiagnosticCode::Config,
        );

        let partial_labels = br#"{"schema_version":2,"task":{"kind":"multi_label","labels":["alpha"]},"episodes":[{"id":"01995c20-7d00-7000-8000-000000000001","expected":{"type":"labels","labels":{"alpha":true}},"input":null}]}"#;
        assert_eq!(
            validate_multi_label(
                partial_labels,
                &serde_json::to_vec(&artifact).unwrap(),
                &serde_json::to_vec(&config).unwrap(),
                digest()
            )
            .unwrap_err()
            .code(),
            DiagnosticCode::Schema
        );

        let mut per_label_abstention = artifact.clone();
        per_label_abstention["predictions"][0]["outcome"] =
            json!({"type": "labels", "labels": [{"label": "alpha", "abstention": true}]});
        assert_multi_code(
            &dataset,
            &per_label_abstention,
            &config,
            DiagnosticCode::Schema,
        );
    }

    #[test]
    fn cross_task_boundaries() {
        let dataset = multi_dataset();
        let artifact = multi_artifact();
        let config = multi_config();
        assert_eq!(
            validate_single_label(
                &serde_json::to_vec(&dataset).unwrap(),
                &serde_json::to_vec(&artifact).unwrap(),
                &serde_json::to_vec(&config).unwrap(),
                digest(),
            )
            .unwrap_err()
            .code(),
            DiagnosticCode::Config
        );
        assert_eq!(
            validate_multi_label(GOLDEN, PREDICTIONS, CONFIG, digest())
                .unwrap_err()
                .code(),
            DiagnosticCode::Config
        );

        let mut wrong_target = dataset.clone();
        wrong_target["episodes"][0]["expected"] = json!({"type": "class", "label": "alpha"});
        assert_multi_code(&wrong_target, &artifact, &config, DiagnosticCode::Config);

        let mut wrong_outcome = artifact.clone();
        wrong_outcome["predictions"][0]["outcome"] = json!({"type": "class", "label": "alpha"});
        assert_multi_code(&dataset, &wrong_outcome, &config, DiagnosticCode::Config);

        let mut reject_below = config.clone();
        reject_below["decision"] =
            json!({"type": "reject_below", "signal": "confidence", "minimum": 0.5});
        assert_multi_code(&dataset, &artifact, &reject_below, DiagnosticCode::Config);

        let mut thresholds = config.clone();
        thresholds["decision"] =
            json!({"type": "label_thresholds", "thresholds": {"alpha": 0.5, "beta": 0.5}});
        assert_multi_code(&dataset, &artifact, &thresholds, DiagnosticCode::Config);

        let single_dataset = json!({
            "schema_version": 2,
            "task": {"kind": "single_label", "labels": ["alpha", "beta"]},
            "episodes": [{"id": "01995c20-7d00-7000-8000-000000000001", "expected": {"type": "class", "label": "alpha"}, "input": null}]
        });
        let single_artifact = json!({
            "schema_version": 2,
            "dataset_sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "sources": {"source": {"kind": "classifier", "model": "example", "configuration": {}}},
            "predictions": [{"id": "01995c20-7d00-7000-8000-000000000001", "source_id": "source", "outcome": {"type": "class", "label": "alpha"}, "probabilities": {"kind": "categorical", "values": {"alpha": 0.9, "beta": 0.8}}}]
        });
        let single_config = json!({"schema_version": 2, "population": "all", "role": "development", "decision": {"type": "as_recorded"}});
        assert_eq!(
            validate_single_label(
                &serde_json::to_vec(&single_dataset).unwrap(),
                &serde_json::to_vec(&single_artifact).unwrap(),
                &serde_json::to_vec(&single_config).unwrap(),
                digest(),
            )
            .unwrap_err()
            .code(),
            DiagnosticCode::Probability
        );
        assert!(admit_multi(&dataset, &artifact, &config).is_ok());
    }

    #[test]
    fn strict_json_keys() {
        let root_duplicate = br#"{
            "schema_version": 2,
            "schema_version": 2,
            "task": {"kind": "single_label", "labels": ["left", "right"]},
            "episodes": []
        }"#;
        let nested_duplicate = br#"{
            "schema_version": 2,
            "task": {"kind": "single_label", "labels": ["left", "right"]},
            "episodes": [{
                "id": "01995c20-7d00-7000-8000-000000000001",
                "expected": {"type": "class", "label": "left"},
                "input": {"a": 1, "a": 2}
            }]
        }"#;
        let escaped_duplicate = br#"{
            "schema_version": 2,
            "task": {"kind": "single_label", "labels": ["left", "right"]},
            "episodes": [{
                "id": "01995c20-7d00-7000-8000-000000000001",
                "expected": {"type": "class", "label": "left"},
                "input": {"a": 1, "\u0061": 2}
            }]
        }"#;
        let configuration_duplicate = br#"{
            "schema_version": 2,
            "dataset_sha256": "digest",
            "sources": {
                "source": {
                    "kind": "classifier",
                    "model": "example",
                    "configuration": {"a": 1, "a": 2},
                    "preparation": {
                        "method": "m",
                        "version": "v",
                        "configuration": {"a": 1, "a": 2},
                        "evidence_indices": [0]
                    }
                }
            },
            "predictions": []
        }"#;

        for invalid in [
            root_duplicate.as_slice(),
            nested_duplicate.as_slice(),
            escaped_duplicate.as_slice(),
        ] {
            let error = decode_golden_dataset(invalid).unwrap_err();
            assert_eq!(error.code(), DiagnosticCode::Schema);
        }
        let error = decode_prediction_artifact(configuration_duplicate).unwrap_err();
        assert_eq!(error.code(), DiagnosticCode::Schema);
    }

    #[test]
    fn opaque_number_and_null() {
        let decoded = decode_golden_dataset(GOLDEN).unwrap();
        let episode = &decoded.value.episodes[0];
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(episode._input.get()).unwrap()["number"]
                .as_u64(),
            Some(9_007_199_254_740_993)
        );

        let explicit_null = br#"{
            "schema_version": 2,
            "task": {"kind": "single_label", "labels": ["left", "right"]},
            "episodes": [{
                "id": "01995c20-7d00-7000-8000-000000000001",
                "expected": {"type": "class", "label": "left"},
                "input": null
            }]
        }"#;
        assert!(decode_golden_dataset(explicit_null).is_ok());

        let missing_input = br#"{
            "schema_version": 2,
            "task": {"kind": "single_label", "labels": ["left", "right"]},
            "episodes": [{
                "id": "01995c20-7d00-7000-8000-000000000001",
                "expected": {"type": "class", "label": "left"}
            }]
        }"#;
        assert!(decode_golden_dataset(missing_input).is_err());
    }

    #[test]
    fn wire_tags_and_fields() {
        assert!(decode_golden_dataset(GOLDEN).is_ok());
        assert!(decode_prediction_artifact(PREDICTIONS).is_ok());
        assert!(decode_evaluation_config(CONFIG).is_ok());

        let unknown_field = br#"{
            "schema_version": 2,
            "task": {"kind": "single_label", "labels": ["left", "right"]},
            "episodes": [],
            "extra": true
        }"#;
        let wrong_version_primitive = br#"{
            "schema_version": true,
            "task": {"kind": "single_label", "labels": ["left", "right"]},
            "episodes": []
        }"#;
        let unsupported_version = br#"{
            "schema_version": 3,
            "task": {"kind": "single_label", "labels": ["left", "right"]},
            "episodes": []
        }"#;
        let future_multi_label = br#"{
            "schema_version": 2,
            "task": {"kind": "multi_label", "labels": ["left"]},
            "episodes": []
        }"#;
        let untagged_target = br#"{
            "schema_version": 2,
            "task": {"kind": "single_label", "labels": ["left", "right"]},
            "episodes": [{
                "id": "id",
                "expected": {"label": "left"},
                "input": null
            }]
        }"#;
        let unsupported_probability = br#"{
            "schema_version": 2,
            "dataset_sha256": "digest",
            "sources": {},
            "predictions": [{
                "id": "id",
                "source_id": "source",
                "outcome": {"type": "class", "label": "left"},
                "probabilities": {"kind": "label_marginals", "values": {"left": 1}}
            }]
        }"#;
        let unsupported_policy = br#"{
            "schema_version": 2,
            "population": "all",
            "role": "development",
            "decision": {"type": "label_thresholds", "thresholds": {"left": 0.5}}
        }"#;

        for invalid in [
            unknown_field.as_slice(),
            wrong_version_primitive.as_slice(),
            unsupported_version.as_slice(),
            untagged_target.as_slice(),
        ] {
            assert!(decode_golden_dataset(invalid).is_err());
        }
        assert!(decode_golden_dataset(future_multi_label).is_ok());
        assert!(decode_prediction_artifact(unsupported_probability).is_ok());
        assert!(decode_evaluation_config(unsupported_policy).is_ok());

        let unknown_observation_kind = String::from_utf8(PREDICTIONS.to_vec()).unwrap().replacen(
            "\"kind\": \"scalar\"",
            "\"kind\": \"unknown\"",
            1,
        );
        assert!(decode_prediction_artifact(unknown_observation_kind.as_bytes()).is_err());

        let observation_extra_field = String::from_utf8(PREDICTIONS.to_vec()).unwrap().replacen(
            "\"kind\": \"scalar\", \"value\": 1.33",
            "\"kind\": \"scalar\", \"value\": 1.33, \"extra\": true",
            1,
        );
        assert!(decode_prediction_artifact(observation_extra_field.as_bytes()).is_err());
    }

    #[test]
    fn dataset_digest_binding() {
        let actual = crate::model::common::ArtifactDigest::try_from(
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        )
        .unwrap();
        assert!(
            super::validate_dataset_digest(
                "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                actual
            )
            .is_ok()
        );
        assert!(
            super::validate_dataset_digest(
                "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
                actual
            )
            .is_err()
        );
    }

    #[test]
    fn checked_admission_builds_the_closed_evaluation() {
        let digest = crate::model::common::ArtifactDigest::try_from(
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        )
        .unwrap();
        assert!(matches!(
            super::validate_single_label(GOLDEN, PREDICTIONS, CONFIG, digest),
            Ok(crate::model::single_label::ValidatedEvaluation::SingleLabel(_))
        ));
    }

    #[test]
    fn population_alignment() {
        let first = "01995c20-7d00-7000-8000-000000000001";
        let second = "01995c20-7d00-7000-8000-000000000002";
        let parent = "01995c20-7d00-7000-8000-000000000003";
        let dataset = serde_json::to_vec(&json!({
            "schema_version": 2,
            "task": {"kind": "single_label", "labels": ["left", "right"]},
            "episodes": [
                {"id": second, "expected": {"type": "class", "label": "right"}, "input": {}},
                {"id": first, "expected": {"type": "class", "label": "left"}, "input": {}},
            ],
        }))
        .unwrap();
        let artifact = |ids: &[&str]| {
            serde_json::to_vec(&json!({
                "schema_version": 2,
                "dataset_sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "sources": {"source": {"kind": "classifier", "model": "model", "configuration": {}}},
                "predictions": ids.iter().map(|id| json!({
                    "id": id,
                    "source_id": "source",
                    "outcome": {"type": "class", "label": "left"},
                })).collect::<Vec<_>>(),
            }))
            .unwrap()
        };
        let config = |episode_ids: Option<Vec<&str>>, parent_run_id: Option<&str>| {
            let mut config = json!({
                "schema_version": 2,
                "population": "selected episodes",
                "role": "held_out",
                "decision": {"type": "as_recorded"},
            });
            let fields = config.as_object_mut().unwrap();
            if let Some(ids) = episode_ids {
                fields.insert("episode_ids".to_owned(), json!(ids));
            }
            if let Some(id) = parent_run_id {
                fields.insert("parent_run_id".to_owned(), json!(id));
            }
            serde_json::to_vec(&config).unwrap()
        };
        let digest = crate::model::common::ArtifactDigest::try_from(
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        )
        .unwrap();

        let omitted = serde_json::to_vec(&json!({
            "schema_version": 2,
            "population": "selected episodes",
            "role": "held_out",
            "decision": {"type": "as_recorded"},
            "parent_run_id": parent,
        }))
        .unwrap();
        let crate::model::single_label::ValidatedEvaluation::SingleLabel(all) =
            validate_single_label(&dataset, &artifact(&[first, second]), &omitted, digest).unwrap();
        let population = all.population();
        assert_eq!(population.dataset_digest(), digest);
        assert_eq!(population.dataset_count(), 2);
        assert_eq!(population.selected_count(), 2);
        assert_eq!(
            population
                .selected()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            [first, second]
        );
        assert!(population.unselected().is_empty());
        assert_eq!(population.config().description(), "selected episodes");
        assert_eq!(
            population.config().role(),
            crate::model::common::EvaluationRole::HeldOut
        );
        assert_eq!(
            population.config().parent_run_id().unwrap().to_string(),
            parent
        );
        assert!(population.config().requested_episode_ids().is_none());
        assert!(matches!(
            population.config().policy(),
            crate::model::single_label::SingleLabelPolicy::AsRecorded
        ));

        let crate::model::single_label::ValidatedEvaluation::SingleLabel(empty) =
            validate_single_label(
                &dataset,
                &artifact(&[]),
                &config(Some(vec![]), None),
                digest,
            )
            .unwrap();
        assert_eq!(empty.population().selected_count(), 0);
        assert_eq!(
            empty
                .population()
                .unselected()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            [first, second]
        );
        assert_eq!(
            empty.population().config().requested_episode_ids(),
            Some(&[][..])
        );

        let crate::model::single_label::ValidatedEvaluation::SingleLabel(subset) =
            validate_single_label(
                &dataset,
                &artifact(&[second]),
                &config(Some(vec![second]), None),
                digest,
            )
            .unwrap();
        assert_eq!(
            subset
                .population()
                .selected()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            [second]
        );
        assert_eq!(
            subset
                .population()
                .unselected()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            [first]
        );
    }

    #[test]
    fn validate_before_selection() {
        let digest = crate::model::common::ArtifactDigest::try_from(
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        )
        .unwrap();
        let invalid = String::from_utf8(PREDICTIONS.to_vec()).unwrap().replacen(
            "\"kind\": \"bernoulli\", \"value\": 1",
            "\"kind\": \"bernoulli\", \"value\": 2",
            1,
        );
        let config_with_unknown_episode = String::from_utf8(CONFIG.to_vec())
            .unwrap()
            .replacen(
                "\"decision\": {\"type\": \"as_recorded\"}",
                "\"episode_ids\": [\"01995c20-7d00-7000-8000-000000000002\"], \"decision\": {\"type\": \"as_recorded\"}",
                1,
            );
        let error = super::validate_single_label(
            GOLDEN,
            invalid.as_bytes(),
            config_with_unknown_episode.as_bytes(),
            digest,
        )
        .unwrap_err();
        assert_eq!(error.code(), DiagnosticCode::Observation);
    }

    #[test]
    fn typed_optional_nulls() {
        let source = |extra: (&str, serde_json::Value)| {
            json!({
                "kind": "classifier",
                "model": "model",
                "configuration": {"top_level_null": null, "nested": {"null": null}},
                extra.0: extra.1,
            })
        };
        let artifact = |source: serde_json::Value, prediction: Vec<serde_json::Value>| {
            serde_json::to_vec(&json!({
                "schema_version": 2,
                "dataset_sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "sources": {"source": source},
                "predictions": prediction,
            }))
            .unwrap()
        };
        let no_prediction: Vec<serde_json::Value> = vec![];
        let source_nulls = [
            ("question_id", json!(null)),
            ("evidence", json!(null)),
            ("observation_definitions", json!(null)),
            ("preparation", json!(null)),
        ];
        for extra in source_nulls {
            let error = decode_prediction_artifact(&artifact(source(extra), no_prediction.clone()))
                .unwrap_err();
            assert_eq!(error.code(), DiagnosticCode::Schema);
        }

        let definition_null = artifact(
            json!({
                "kind": "classifier",
                "model": "model",
                "configuration": {},
                "observation_definitions": {
                    "score": {"kind": "scalar", "description": "score", "question_id": null}
                },
            }),
            no_prediction.clone(),
        );
        assert_eq!(
            decode_prediction_artifact(&definition_null)
                .unwrap_err()
                .code(),
            DiagnosticCode::Schema
        );

        let base_prediction = |extra: (&str, serde_json::Value)| {
            json!({
                "id": "01995c20-7d00-7000-8000-000000000001",
                "source_id": "source",
                "outcome": {"type": "class", "label": "left"},
                extra.0: extra.1,
            })
        };
        for extra in [
            ("probabilities", json!(null)),
            ("confidence", json!(null)),
            ("observations", json!(null)),
        ] {
            let error = decode_prediction_artifact(&artifact(
                source(("question_id", json!("source-question"))),
                vec![base_prediction(extra)],
            ))
            .unwrap_err();
            assert_eq!(error.code(), DiagnosticCode::Schema);
        }
        let abstention_null = artifact(
            source(("question_id", json!("source-question"))),
            vec![json!({
                "id": "01995c20-7d00-7000-8000-000000000001",
                "source_id": "source",
                "outcome": {"type": "abstention", "reason": null},
            })],
        );
        assert_eq!(
            decode_prediction_artifact(&abstention_null)
                .unwrap_err()
                .code(),
            DiagnosticCode::Schema
        );

        let config = |extra: (&str, serde_json::Value)| {
            serde_json::to_vec(&json!({
                "schema_version": 2,
                "population": "all",
                "role": "development",
                "decision": {"type": "as_recorded"},
                extra.0: extra.1,
            }))
            .unwrap()
        };
        for extra in [("episode_ids", json!(null)), ("parent_run_id", json!(null))] {
            assert_eq!(
                decode_evaluation_config(&config(extra)).unwrap_err().code(),
                DiagnosticCode::Schema
            );
        }

        let all_present = artifact(
            json!({
                "kind": "classifier",
                "model": "model",
                "configuration": {"top_level_null": null, "nested": {"null": null}},
                "question_id": "source-question",
                "evidence": ["evidence.json"],
                "observation_definitions": {
                    "score": {"kind": "scalar", "description": "score", "question_id": "definition-question"}
                },
                "preparation": {
                    "method": "method",
                    "version": "version",
                    "configuration": {"top_level_null": null, "nested": {"null": null}},
                    "evidence_indices": [0]
                },
            }),
            vec![json!({
                "id": "01995c20-7d00-7000-8000-000000000001",
                "source_id": "source",
                "outcome": {"type": "abstention", "reason": "reason"},
                "probabilities": {"kind": "categorical", "values": {"left": 1, "right": 0}},
                "confidence": 1,
                "observations": {"score": {"kind": "scalar", "value": 1.33}},
            })],
        );
        decode_prediction_artifact(&all_present).unwrap();
        assert!(decode_prediction_artifact(PREDICTIONS).is_ok());
        assert!(
            decode_evaluation_config(&config((
                "episode_ids",
                json!(["01995c20-7d00-7000-8000-000000000001"]),
            )))
            .is_ok()
        );
        assert!(
            decode_evaluation_config(&config((
                "parent_run_id",
                json!("01995c20-7d00-7000-8000-000000000002"),
            )))
            .is_ok()
        );

        let golden_null = br#"{
            "schema_version": 2,
            "task": {"kind": "single_label", "labels": ["left", "right"]},
            "episodes": [{
                "id": "01995c20-7d00-7000-8000-000000000001",
                "expected": {"type": "class", "label": "left"},
                "input": null
            }]
        }"#;
        assert!(decode_golden_dataset(golden_null).is_ok());

        let digest = crate::model::common::ArtifactDigest::try_from(
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        )
        .unwrap();
        let episode_ids_null = config(("episode_ids", json!(null)));
        assert_eq!(
            validate_single_label(GOLDEN, PREDICTIONS, &episode_ids_null, digest)
                .unwrap_err()
                .code(),
            DiagnosticCode::Schema
        );
    }

    #[test]
    fn json_number_marker_collision() {
        let marker = json!({"$serde_json::private::Number": "0.5"});
        let numeric_marker = serde_json::to_vec(&json!({
            "schema_version": 2,
            "dataset_sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "sources": {},
            "predictions": [{
                "id": "01995c20-7d00-7000-8000-000000000001",
                "source_id": "source",
                "outcome": {"type": "class", "label": "left"},
                "confidence": marker,
            }],
        }))
        .unwrap();
        assert_eq!(
            decode_prediction_artifact(&numeric_marker)
                .unwrap_err()
                .code(),
            DiagnosticCode::Schema
        );

        let golden_marker = serde_json::to_vec(&json!({
            "schema_version": 2,
            "task": {"kind": "single_label", "labels": ["left", "right"]},
            "episodes": [{
                "id": "01995c20-7d00-7000-8000-000000000001",
                "expected": {"type": "class", "label": "left"},
                "input": {"$serde_json::private::Number": "0.5"},
            }],
        }))
        .unwrap();
        let golden = decode_golden_dataset(&golden_marker).unwrap();
        assert_eq!(
            golden.value.episodes[0]._input.get(),
            "{\"$serde_json::private::Number\":\"0.5\"}"
        );

        let source_marker = serde_json::to_vec(&json!({
            "schema_version": 2,
            "dataset_sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "sources": {
                "source": {
                    "kind": "classifier",
                    "model": "model",
                    "configuration": {"$serde_json::private::Number": "0.5"},
                    "evidence": ["evidence.json"],
                    "preparation": {
                        "method": "method",
                        "version": "version",
                        "configuration": {"nested": {"$serde_json::private::Number": "0.5"}},
                        "evidence_indices": [0]
                    }
                }
            },
            "predictions": [],
        }))
        .unwrap();
        let artifact = decode_prediction_artifact(&source_marker).unwrap();
        let source = &artifact.value.sources["source"];
        assert_eq!(
            source.configuration.0.get(),
            "{\"$serde_json::private::Number\":\"0.5\"}"
        );
        assert_eq!(
            source.preparation.as_ref().unwrap().configuration.0.get(),
            "{\"nested\":{\"$serde_json::private::Number\":\"0.5\"}}"
        );
    }

    #[test]
    fn large_number_boundary() {
        let golden = br#"{
            "schema_version": 2,
            "task": {"kind": "single_label", "labels": ["left", "right"]},
            "episodes": [{
                "id": "01995c20-7d00-7000-8000-000000000001",
                "expected": {"type": "class", "label": "left"},
                "input": {"huge": 1e400}
            }]
        }"#;
        let artifact = br#"{
            "schema_version": 2,
            "dataset_sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "sources": {"source": {
                "kind": "classifier",
                "model": "model",
                "configuration": {"huge": 1e400},
                "evidence": ["evidence.json"],
                "preparation": {
                    "method": "method",
                    "version": "version",
                    "configuration": {"nested": {"huge": 1e400}},
                    "evidence_indices": [0]
                }
            }},
            "predictions": []
        }"#;
        let empty_selection = br#"{
            "schema_version": 2,
            "population": "none",
            "role": "development",
            "episode_ids": [],
            "decision": {"type": "as_recorded"}
        }"#;
        let digest = crate::model::common::ArtifactDigest::try_from(
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        )
        .unwrap();

        let golden_decoded = decode_golden_dataset(golden).unwrap();
        assert!(
            golden_decoded.value.episodes[0]
                ._input
                .get()
                .contains("1e400")
        );
        let artifact_decoded = decode_prediction_artifact(artifact).unwrap();
        let source = &artifact_decoded.value.sources["source"];
        assert!(source.configuration.0.get().contains("1e400"));
        assert!(
            source
                .preparation
                .as_ref()
                .unwrap()
                .configuration
                .0
                .get()
                .contains("1e400")
        );
        assert!(validate_single_label(golden, artifact, empty_selection, digest).is_ok());

        let typed_huge = String::from_utf8(PREDICTIONS.to_vec()).unwrap().replacen(
            "\"value\": 1.33",
            "\"value\": 1e400",
            1,
        );
        assert_eq!(
            validate_single_label(GOLDEN, typed_huge.as_bytes(), CONFIG, digest)
                .unwrap_err()
                .code(),
            DiagnosticCode::Observation
        );

        let rejected_huge = br#"{
            "schema_version": 2,
            "population": "all",
            "role": "development",
            "decision": {"type": "reject_below", "signal": "confidence", "minimum": 1e400}
        }"#;
        assert!(decode_evaluation_config(rejected_huge).is_ok());
        assert_eq!(
            validate_single_label(GOLDEN, PREDICTIONS, rejected_huge, digest)
                .unwrap_err()
                .code(),
            DiagnosticCode::Config
        );
    }
}
