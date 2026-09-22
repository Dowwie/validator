use std::{cell::Cell, collections::BTreeMap, fmt};

use serde::{
    Deserialize,
    de::{self, DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor},
};
use serde_json::value::RawValue;

use crate::{Diagnostic, DiagnosticCode, Result};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct GoldenDataset {
    pub(crate) schema_version: u8,
    pub(crate) task: Task,
    pub(crate) episodes: Vec<GoldenEpisode>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum Task {
    SingleLabel { labels: Vec<String> },
    MultiLabel { labels: Vec<String> },
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct GoldenEpisode {
    pub(crate) id: String,
    pub(crate) expected: Target,
    #[serde(rename = "input")]
    pub(crate) _input: Box<RawValue>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum Target {
    Class { label: String },
    Labels { labels: Vec<String> },
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PredictionArtifact {
    pub(crate) schema_version: u8,
    pub(crate) dataset_sha256: String,
    pub(crate) sources: BTreeMap<String, Source>,
    pub(crate) predictions: Vec<Prediction>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Source {
    pub(crate) kind: SourceKind,
    pub(crate) model: String,
    pub(crate) configuration: OpaqueObject,
    #[serde(default, deserialize_with = "omitted_or_present")]
    pub(crate) question_id: Option<String>,
    #[serde(default, deserialize_with = "omitted_or_present")]
    pub(crate) evidence: Option<Vec<String>>,
    #[serde(default, deserialize_with = "omitted_or_present")]
    pub(crate) observation_definitions: Option<BTreeMap<String, ObservationDefinition>>,
    #[serde(default, deserialize_with = "omitted_or_present")]
    pub(crate) preparation: Option<Preparation>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SourceKind {
    ScoredChoice,
    Classifier,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ObservationDefinition {
    pub(crate) kind: ObservationKind,
    pub(crate) description: String,
    #[serde(default, deserialize_with = "omitted_or_present")]
    pub(crate) question_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ObservationKind {
    Scalar,
    Bernoulli,
    ReportedConfidence,
    Categorical,
    LabelMarginals,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Preparation {
    pub(crate) method: String,
    pub(crate) version: String,
    pub(crate) configuration: OpaqueObject,
    pub(crate) evidence_indices: Vec<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Prediction {
    pub(crate) id: String,
    pub(crate) source_id: String,
    pub(crate) outcome: PredictionOutcome,
    #[serde(default, deserialize_with = "omitted_or_present")]
    pub(crate) probabilities: Option<Probabilities>,
    #[serde(default, deserialize_with = "omitted_or_present")]
    pub(crate) confidence: Option<JsonNumber>,
    #[serde(default, deserialize_with = "omitted_or_present")]
    pub(crate) observations: Option<BTreeMap<String, Observation>>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum PredictionOutcome {
    Class {
        label: String,
    },
    Labels {
        labels: Vec<String>,
    },
    Abstention {
        #[serde(default, deserialize_with = "omitted_or_present")]
        reason: Option<String>,
    },
}

#[derive(Debug)]
pub(crate) enum Observation {
    Scalar {
        value: JsonNumber,
    },
    Bernoulli {
        value: JsonNumber,
    },
    ReportedConfidence {
        value: JsonNumber,
    },
    Categorical {
        values: BTreeMap<String, JsonNumber>,
    },
    LabelMarginals {
        values: BTreeMap<String, JsonNumber>,
    },
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ObservationFields {
    kind: ObservationKind,
    #[serde(default, deserialize_with = "omitted_or_present")]
    value: Option<JsonNumber>,
    #[serde(default, deserialize_with = "omitted_or_present")]
    values: Option<BTreeMap<String, JsonNumber>>,
}

impl<'de> Deserialize<'de> for Observation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let fields = ObservationFields::deserialize(deserializer)?;
        match (fields.kind, fields.value, fields.values) {
            (ObservationKind::Scalar, Some(value), None) => Ok(Self::Scalar { value }),
            (ObservationKind::Bernoulli, Some(value), None) => Ok(Self::Bernoulli { value }),
            (ObservationKind::ReportedConfidence, Some(value), None) => {
                Ok(Self::ReportedConfidence { value })
            }
            (ObservationKind::Categorical, None, Some(values)) => Ok(Self::Categorical { values }),
            (ObservationKind::LabelMarginals, None, Some(values)) => {
                Ok(Self::LabelMarginals { values })
            }
            _ => Err(de::Error::custom("observation fields do not match kind")),
        }
    }
}

#[derive(Debug)]
pub(crate) enum Probabilities {
    Categorical {
        values: BTreeMap<String, JsonNumber>,
    },
    LabelMarginals {
        values: BTreeMap<String, JsonNumber>,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum ProbabilityKind {
    Categorical,
    LabelMarginals,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProbabilityFields {
    kind: ProbabilityKind,
    values: BTreeMap<String, JsonNumber>,
}

impl<'de> Deserialize<'de> for Probabilities {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match ProbabilityFields::deserialize(deserializer)? {
            ProbabilityFields {
                kind: ProbabilityKind::Categorical,
                values,
            } => Ok(Self::Categorical { values }),
            ProbabilityFields {
                kind: ProbabilityKind::LabelMarginals,
                values,
            } => Ok(Self::LabelMarginals { values }),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct EvaluationConfig {
    pub(crate) schema_version: u8,
    pub(crate) population: String,
    pub(crate) role: EvaluationRole,
    pub(crate) decision: Decision,
    #[serde(default, deserialize_with = "omitted_or_present")]
    pub(crate) episode_ids: Option<Vec<String>>,
    #[serde(default, deserialize_with = "omitted_or_present")]
    pub(crate) parent_run_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum EvaluationRole {
    Development,
    HeldOut,
}

#[derive(Debug)]
pub(crate) enum Decision {
    AsRecorded,
    RejectBelow {
        signal: RejectionSignal,
        minimum: JsonNumber,
    },
    LabelThresholds {
        thresholds: BTreeMap<String, JsonNumber>,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum DecisionTag {
    AsRecorded,
    RejectBelow,
    LabelThresholds,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DecisionFields {
    #[serde(rename = "type")]
    kind: DecisionTag,
    #[serde(default, deserialize_with = "omitted_or_present")]
    signal: Option<RejectionSignal>,
    #[serde(default, deserialize_with = "omitted_or_present")]
    minimum: Option<JsonNumber>,
    #[serde(default, deserialize_with = "omitted_or_present")]
    thresholds: Option<BTreeMap<String, JsonNumber>>,
}

impl<'de> Deserialize<'de> for Decision {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let fields = DecisionFields::deserialize(deserializer)?;
        match (
            fields.kind,
            fields.signal,
            fields.minimum,
            fields.thresholds,
        ) {
            (DecisionTag::AsRecorded, None, None, None) => Ok(Self::AsRecorded),
            (DecisionTag::RejectBelow, Some(signal), Some(minimum), None) => {
                Ok(Self::RejectBelow { signal, minimum })
            }
            (DecisionTag::LabelThresholds, None, None, Some(thresholds)) => {
                Ok(Self::LabelThresholds { thresholds })
            }
            _ => Err(de::Error::custom("decision fields do not match type")),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RejectionSignal {
    Confidence,
    MaxProbability,
}

/// An opaque configuration object whose semantic interpretation is deferred.
#[derive(Debug)]
pub(crate) struct OpaqueObject(pub(crate) Box<RawValue>);

impl<'de> Deserialize<'de> for OpaqueObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Box::<RawValue>::deserialize(deserializer)?;
        if !value.get().trim_start().starts_with('{') {
            return Err(de::Error::custom("configuration must be an object"));
        }
        Ok(Self(value))
    }
}

/// A raw JSON numeric token whose range and semantic interpretation are deferred.
#[derive(Debug)]
pub(crate) struct JsonNumber(pub(crate) Box<RawValue>);

fn omitted_or_present<'de, D, T>(deserializer: D) -> std::result::Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}

impl<'de> Deserialize<'de> for JsonNumber {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Box::<RawValue>::deserialize(deserializer)?;
        if !matches!(value.get().as_bytes().first(), Some(b'-' | b'0'..=b'9')) {
            return Err(de::Error::custom("value must be a JSON number"));
        }
        Ok(Self(value))
    }
}

pub(super) fn reject_duplicate_keys(bytes: &[u8]) -> Result<()> {
    let duplicate_found = Cell::new(false);
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let result = (&mut deserializer).deserialize_any(DuplicateKeyVisitor {
        duplicate_found: &duplicate_found,
    });

    if duplicate_found.get() {
        return Err(Diagnostic::for_code(DiagnosticCode::Schema));
    }
    if result.is_err() || deserializer.end().is_err() {
        return Err(Diagnostic::for_code(DiagnosticCode::Parse));
    }

    Ok(())
}

struct DuplicateKeyVisitor<'a> {
    duplicate_found: &'a Cell<bool>,
}

impl<'de> Visitor<'de> for DuplicateKeyVisitor<'_> {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON value")
    }

    fn visit_bool<E>(self, _value: bool) -> std::result::Result<(), E>
    where
        E: de::Error,
    {
        Ok(())
    }

    fn visit_i64<E>(self, _value: i64) -> std::result::Result<(), E>
    where
        E: de::Error,
    {
        Ok(())
    }

    fn visit_u64<E>(self, _value: u64) -> std::result::Result<(), E>
    where
        E: de::Error,
    {
        Ok(())
    }

    fn visit_f64<E>(self, _value: f64) -> std::result::Result<(), E>
    where
        E: de::Error,
    {
        Ok(())
    }

    fn visit_str<E>(self, _value: &str) -> std::result::Result<(), E>
    where
        E: de::Error,
    {
        Ok(())
    }

    fn visit_string<E>(self, _value: String) -> std::result::Result<(), E>
    where
        E: de::Error,
    {
        Ok(())
    }

    fn visit_none<E>(self) -> std::result::Result<(), E>
    where
        E: de::Error,
    {
        Ok(())
    }

    fn visit_unit<E>(self) -> std::result::Result<(), E>
    where
        E: de::Error,
    {
        Ok(())
    }

    fn visit_seq<A>(self, mut sequence: A) -> std::result::Result<(), A::Error>
    where
        A: SeqAccess<'de>,
    {
        while sequence
            .next_element_seed(DuplicateKeySeed {
                duplicate_found: self.duplicate_found,
            })?
            .is_some()
        {}
        Ok(())
    }

    fn visit_map<A>(self, mut map: A) -> std::result::Result<(), A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut keys = std::collections::HashSet::new();
        while let Some(key) = map.next_key::<String>()? {
            if !keys.insert(key) {
                self.duplicate_found.set(true);
                return Err(de::Error::custom("duplicate JSON key"));
            }
            map.next_value_seed(DuplicateKeySeed {
                duplicate_found: self.duplicate_found,
            })?;
        }
        Ok(())
    }
}

struct DuplicateKeySeed<'a> {
    duplicate_found: &'a Cell<bool>,
}

impl<'de> DeserializeSeed<'de> for DuplicateKeySeed<'_> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> std::result::Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DuplicateKeyVisitor {
            duplicate_found: self.duplicate_found,
        })
    }
}

pub(crate) fn require_schema_version_two(version: u8) -> Result<()> {
    if version == crate::WIRE_VERSION as u8 {
        Ok(())
    } else {
        Err(Diagnostic::for_code(DiagnosticCode::Schema))
    }
}
