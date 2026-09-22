use std::{
    collections::{BTreeMap, HashMap},
    fmt,
    path::{Path, PathBuf},
    sync::Arc,
};

use serde::Serialize;
use uuid::{Uuid, Variant};

use crate::{Diagnostic, DiagnosticCode, Result};

macro_rules! uuid_identity {
    ($name:ident, $description:literal) => {
        #[doc = $description]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(Uuid);

        impl TryFrom<&str> for $name {
            type Error = Diagnostic;

            fn try_from(value: &str) -> Result<Self> {
                parse_uuid(value).map(Self)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

#[cfg(test)]
mod admission_tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn observation_contracts() {
        let definitions = HashMap::from([
            (
                "scalar".to_owned(),
                ObservationDefinition::new(ObservationKind::Scalar, "score".to_owned(), None)
                    .unwrap(),
            ),
            (
                "categorical".to_owned(),
                ObservationDefinition::new(
                    ObservationKind::Categorical,
                    "distribution".to_owned(),
                    None,
                )
                .unwrap(),
            ),
        ]);
        let scalar = Observation::scalar(ObservationKind::Scalar, 1.33).unwrap();
        let categorical = Observation::vector(
            ObservationKind::Categorical,
            HashMap::from([("a".to_owned(), 0.5), ("b".to_owned(), 0.49)]),
        )
        .unwrap();
        assert!(
            ObservationSet::new(
                HashMap::from([
                    ("scalar".to_owned(), scalar),
                    ("categorical".to_owned(), categorical)
                ]),
                &definitions
            )
            .is_ok()
        );
        assert!(Observation::scalar(ObservationKind::Bernoulli, 1.1).is_err());
        assert!(Observation::vector(ObservationKind::Categorical, HashMap::new()).is_err());
        assert!(
            ObservationSet::new(
                HashMap::from([(
                    "missing".to_owned(),
                    Observation::scalar(ObservationKind::Scalar, 1.0).unwrap()
                )]),
                &definitions
            )
            .is_err()
        );
    }

    #[test]
    fn source_preparation_bindings() {
        assert!(
            PreparationDescriptor::new(
                "m".to_owned(),
                "v".to_owned(),
                serde_json::json!({}),
                vec![],
                1
            )
            .is_err()
        );
        assert!(
            PreparationDescriptor::new(
                "m".to_owned(),
                "v".to_owned(),
                serde_json::json!({}),
                vec![0, 0],
                1
            )
            .is_err()
        );
        assert!(
            PreparationDescriptor::new(
                "m".to_owned(),
                "v".to_owned(),
                serde_json::json!({}),
                vec![1],
                1
            )
            .is_err()
        );
        assert!(
            SourceDefinition::new(
                SourceKind::ScoredChoice,
                "model".to_owned(),
                serde_json::json!({}),
                None,
                vec![],
                HashMap::new(),
                None
            )
            .is_err()
        );
    }

    #[test]
    fn population_alignment() {
        let second = EpisodeId::try_from("01995c20-7d00-7000-8000-000000000002").unwrap();
        let first = EpisodeId::try_from("01995c20-7d00-7000-8000-000000000001").unwrap();
        let config = EvaluationConfig::new(
            "selected".to_owned(),
            EvaluationRole::Development,
            None,
            Some(vec![second, first]),
        )
        .unwrap();
        let digest = ArtifactDigest::try_from(
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        )
        .unwrap();
        let population = Population::new(digest, config, vec![second, first], vec![]).unwrap();
        assert_eq!(population.selected(), &[first, second]);
        assert_eq!(population.dataset_count(), 2);
        assert_eq!(population.selected_count(), 2);
        let omitted =
            EvaluationConfig::new("all".to_owned(), EvaluationRole::Development, None, None)
                .unwrap();
        assert!(Population::new(digest, omitted, vec![second, first], vec![]).is_ok());
        let omitted_subset =
            EvaluationConfig::new("all".to_owned(), EvaluationRole::Development, None, None)
                .unwrap();
        assert!(Population::new(digest, omitted_subset, vec![first], vec![second]).is_err());
        let empty_omitted =
            EvaluationConfig::new("empty".to_owned(), EvaluationRole::Development, None, None)
                .unwrap();
        assert!(Population::new(digest, empty_omitted, vec![], vec![]).is_ok());
        assert!(
            EvaluationConfig::new(" ".to_owned(), EvaluationRole::Development, None, None).is_err()
        );
    }
}

uuid_identity!(
    EpisodeId,
    "A checked canonical UUID that identifies one episode."
);
uuid_identity!(
    RunId,
    "A checked canonical UUID that identifies one saved run."
);

fn parse_uuid(value: &str) -> Result<Uuid> {
    let parsed = Uuid::parse_str(value).map_err(|_| invalid_identifier())?;

    if parsed.is_nil()
        || parsed == Uuid::max()
        || parsed.get_variant() != Variant::RFC4122
        || parsed.get_version().is_none()
        || parsed.hyphenated().to_string() != value
    {
        return Err(invalid_identifier());
    }

    Ok(parsed)
}

fn invalid_identifier() -> Diagnostic {
    Diagnostic::for_code(DiagnosticCode::Id)
}

/// A checked, exact artifact-local source identifier.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SourceId(String);

impl TryFrom<&str> for SourceId {
    type Error = Diagnostic;

    fn try_from(value: &str) -> Result<Self> {
        if value.chars().all(char::is_whitespace) {
            return Err(invalid_identifier());
        }

        Ok(Self(value.to_owned()))
    }
}

impl SourceId {
    /// Returns the source identifier exactly as supplied.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A checked lowercase SHA-256 digest of exact artifact bytes.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ArtifactDigest([u8; 32]);

impl TryFrom<&str> for ArtifactDigest {
    type Error = Diagnostic;

    fn try_from(value: &str) -> Result<Self> {
        if value.len() != 64 {
            return Err(invalid_identifier());
        }

        let mut bytes = [0_u8; 32];
        let (pairs, _) = value.as_bytes().as_chunks::<2>();
        for (index, &[high_byte, low_byte]) in pairs.iter().enumerate() {
            let Some(high) = hex_digit(high_byte) else {
                return Err(invalid_identifier());
            };
            let Some(low) = hex_digit(low_byte) else {
                return Err(invalid_identifier());
            };
            bytes[index] = (high << 4) | low;
        }

        Ok(Self(bytes))
    }
}

impl fmt::Display for ArtifactDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(formatter, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl ArtifactDigest {
    /// Constructs a digest from already hashed exact artifact bytes.
    pub(crate) const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

/// Exact bytes and identity of one input artifact retained for later validation.
#[derive(Debug)]
pub(crate) struct ArtifactSnapshot {
    original_path: PathBuf,
    stored_path: PathBuf,
    bytes: Vec<u8>,
    digest: ArtifactDigest,
}

impl ArtifactSnapshot {
    /// Constructs an exact input snapshot from one completed file read.
    pub(crate) fn new(
        original_path: PathBuf,
        stored_path: PathBuf,
        bytes: Vec<u8>,
        digest: ArtifactDigest,
    ) -> Self {
        Self {
            original_path,
            stored_path,
            bytes,
            digest,
        }
    }

    /// Borrows the exact input path supplied by the caller.
    pub(crate) fn original_path(&self) -> &Path {
        &self.original_path
    }

    /// Borrows the deterministic path reserved for the stored snapshot.
    pub(crate) fn stored_path(&self) -> &Path {
        &self.stored_path
    }

    /// Borrows the exact submitted bytes without parsing or rewriting them.
    pub(crate) fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the SHA-256 digest of the exact submitted bytes.
    pub(crate) const fn digest(&self) -> ArtifactDigest {
        self.digest
    }
}

/// One exact source-evidence file bound to its source-array position.
#[derive(Debug)]
pub(crate) struct EvidenceBinding {
    source_id: SourceId,
    evidence_index: usize,
    original_path: String,
    stored_path: PathBuf,
    bytes: Vec<u8>,
    digest: ArtifactDigest,
}

impl EvidenceBinding {
    /// Constructs a binding from one completed evidence-file read.
    pub(crate) fn new(
        source_id: SourceId,
        evidence_index: usize,
        original_path: String,
        stored_path: PathBuf,
        bytes: Vec<u8>,
        digest: ArtifactDigest,
    ) -> Self {
        Self {
            source_id,
            evidence_index,
            original_path,
            stored_path,
            bytes,
            digest,
        }
    }

    /// Borrows the source that declared this evidence position.
    pub(crate) fn source_id(&self) -> &SourceId {
        &self.source_id
    }

    /// Returns the zero-based position in the declaring source's evidence array.
    pub(crate) const fn evidence_index(&self) -> usize {
        self.evidence_index
    }

    /// Borrows the exact original evidence-path string from the prediction artifact.
    pub(crate) fn original_path(&self) -> &str {
        &self.original_path
    }

    /// Borrows the deterministic stored evidence path.
    pub(crate) fn stored_path(&self) -> &Path {
        &self.stored_path
    }

    /// Borrows the exact evidence bytes without rewriting them.
    pub(crate) fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the SHA-256 digest of the exact evidence bytes.
    pub(crate) const fn digest(&self) -> ArtifactDigest {
        self.digest
    }
}

const fn hex_digit(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        _ => None,
    }
}

/// An owned, ordered vocabulary with exact label identity.
#[derive(Clone, Debug)]
pub struct LabelVocabulary {
    labels: Arc<[String]>,
}

impl LabelVocabulary {
    pub fn for_single_label(labels: impl IntoIterator<Item = String>) -> Result<Self> {
        Self::new(labels, 2)
    }

    pub fn for_multi_label(labels: impl IntoIterator<Item = String>) -> Result<Self> {
        Self::new(labels, 1)
    }

    fn new(labels: impl IntoIterator<Item = String>, minimum: usize) -> Result<Self> {
        let labels: Vec<_> = labels.into_iter().collect();
        if labels.len() < minimum
            || labels
                .iter()
                .any(|label| label.chars().all(char::is_whitespace))
            || labels
                .iter()
                .enumerate()
                .any(|(index, label)| labels[..index].contains(label))
        {
            return Err(Diagnostic::for_code(DiagnosticCode::Label));
        }

        Ok(Self {
            labels: Arc::from(labels),
        })
    }

    /// Returns the number of labels in vocabulary order.
    pub fn len(&self) -> usize {
        self.labels.len()
    }

    /// Returns the label at `index` when it belongs to this vocabulary.
    pub fn label(&self, index: &LabelIndex) -> Option<&str> {
        Arc::ptr_eq(&self.labels, &index.vocabulary)
            .then(|| self.labels.get(index.position))
            .flatten()
            .map(String::as_str)
    }

    /// Looks up a label by exact byte and Unicode identity.
    pub fn lookup(&self, label: &str) -> Option<LabelIndex> {
        self.labels
            .iter()
            .position(|candidate| candidate == label)
            .map(|position| LabelIndex {
                vocabulary: Arc::clone(&self.labels),
                position,
            })
    }

    /// Iterates over labels in their declared vocabulary order.
    pub fn labels(&self) -> impl Iterator<Item = &str> {
        self.labels.iter().map(String::as_str)
    }

    /// Returns the position of an index belonging to this vocabulary.
    pub fn position(&self, index: &LabelIndex) -> Option<usize> {
        Arc::ptr_eq(&self.labels, &index.vocabulary).then_some(index.position)
    }

    /// Constructs a checked set of labels belonging to this vocabulary.
    pub fn label_set<'a>(&self, labels: impl IntoIterator<Item = &'a str>) -> Result<LabelSet> {
        let mut indexes: Vec<usize> = Vec::new();

        for label in labels {
            let Some(index) = self.lookup(label) else {
                return Err(Diagnostic::for_code(DiagnosticCode::Label));
            };
            if indexes.contains(&index.position) {
                return Err(Diagnostic::for_code(DiagnosticCode::Label));
            }
            indexes.push(index.position);
        }

        indexes.sort_unstable();
        Ok(LabelSet {
            vocabulary: Arc::clone(&self.labels),
            indexes,
        })
    }
}

/// An internal label position that remains bound to its creating vocabulary.
#[derive(Clone, Debug)]
pub struct LabelIndex {
    vocabulary: Arc<[String]>,
    position: usize,
}

/// A checked set of labels, ordered by its owning vocabulary.
#[derive(Debug)]
pub struct LabelSet {
    vocabulary: Arc<[String]>,
    indexes: Vec<usize>,
}

impl LabelSet {
    /// Iterates selected labels in the owning vocabulary order.
    pub(crate) fn labels(&self) -> impl Iterator<Item = &str> {
        self.indexes
            .iter()
            .map(|&position| self.vocabulary[position].as_str())
    }
}

/// An episode with a checked identity and concrete expected target.
#[derive(Debug)]
pub struct Episode<Target> {
    id: EpisodeId,
    target: Target,
}

impl<Target> Episode<Target> {
    /// Constructs an episode from already checked identity and target values.
    pub fn new(id: EpisodeId, target: Target) -> Self {
        Self { id, target }
    }

    /// Returns the episode identifier.
    pub fn id(&self) -> EpisodeId {
        self.id
    }

    /// Borrows the concrete expected target.
    pub fn target(&self) -> &Target {
        &self.target
    }
}

/// An answered output or an explicit abstention.
#[derive(Debug)]
pub struct Outcome<Target> {
    state: OutcomeState<Target>,
}

#[derive(Debug)]
enum OutcomeState<Target> {
    Answered(Target),
    Abstained { reason: Option<String> },
}

impl<Target> Outcome<Target> {
    /// Creates an answered outcome from its concrete target.
    pub fn answered(target: Target) -> Self {
        Self {
            state: OutcomeState::Answered(target),
        }
    }

    /// Creates an explicit abstention whose optional reason is nonblank.
    pub fn abstained(reason: Option<String>) -> Result<Self> {
        if reason
            .as_deref()
            .is_some_and(|value| value.chars().all(char::is_whitespace))
        {
            return Err(Diagnostic::for_code(DiagnosticCode::Schema));
        }

        Ok(Self {
            state: OutcomeState::Abstained { reason },
        })
    }

    /// Borrows the answered target when this is not an abstention.
    pub fn answered_target(&self) -> Option<&Target> {
        match &self.state {
            OutcomeState::Answered(target) => Some(target),
            OutcomeState::Abstained { .. } => None,
        }
    }

    /// Returns the abstention reason, if this outcome supplies one.
    pub fn abstention_reason(&self) -> Option<&str> {
        match &self.state {
            OutcomeState::Answered(_) => None,
            OutcomeState::Abstained { reason } => reason.as_deref(),
        }
    }
}

/// A source's declared classifier profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    /// A general classifier output.
    Classifier,
    /// A scored-choice output with a required question identifier.
    ScoredChoice,
}

/// A retained observation's declared meaning.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservationKind {
    /// An unrestricted finite scalar.
    Scalar,
    /// A bounded Bernoulli value.
    Bernoulli,
    /// A bounded reported confidence value.
    ReportedConfidence,
    /// An independent categorical observation vector.
    Categorical,
    /// An independent label-marginal observation vector.
    LabelMarginals,
}

/// A checked source-side observation definition.
#[derive(Debug)]
pub struct ObservationDefinition {
    kind: ObservationKind,
    description: String,
    question_id: Option<String>,
}

impl ObservationDefinition {
    /// Creates a definition with nonblank description and optional question ID.
    pub fn new(
        kind: ObservationKind,
        description: String,
        question_id: Option<String>,
    ) -> Result<Self> {
        if blank(&description) || question_id.as_deref().is_some_and(blank) {
            return Err(Diagnostic::for_code(DiagnosticCode::Observation));
        }
        Ok(Self {
            kind,
            description,
            question_id,
        })
    }

    /// Returns the declared observation kind.
    pub const fn kind(&self) -> ObservationKind {
        self.kind
    }

    pub(crate) fn description(&self) -> &str {
        &self.description
    }

    pub(crate) fn question_id(&self) -> Option<&str> {
        self.question_id.as_deref()
    }
}

/// A retained checked observation independent of scoring signals.
#[derive(Clone, Debug)]
pub enum Observation {
    /// A finite unrestricted scalar.
    Scalar(f64),
    /// A finite scalar in the unit interval.
    Bernoulli(f64),
    /// A finite scalar in the unit interval.
    ReportedConfidence(f64),
    /// A nonempty independent option vector.
    Categorical(HashMap<String, f64>),
    /// A nonempty independent label vector.
    LabelMarginals(HashMap<String, f64>),
}

impl Observation {
    /// Creates a checked scalar observation for `kind`.
    pub fn scalar(kind: ObservationKind, value: f64) -> Result<Self> {
        if !value.is_finite()
            || matches!(
                kind,
                ObservationKind::Categorical | ObservationKind::LabelMarginals
            )
            || matches!(
                kind,
                ObservationKind::Bernoulli | ObservationKind::ReportedConfidence
            ) && !(0.0..=1.0).contains(&value)
        {
            return Err(Diagnostic::for_code(DiagnosticCode::Observation));
        }
        let observation = match kind {
            ObservationKind::Scalar => Self::Scalar(value),
            ObservationKind::Bernoulli => Self::Bernoulli(value),
            ObservationKind::ReportedConfidence => Self::ReportedConfidence(value),
            ObservationKind::Categorical | ObservationKind::LabelMarginals => {
                return Err(Diagnostic::for_code(DiagnosticCode::Observation));
            }
        };
        Ok(observation)
    }

    /// Creates a checked independent vector observation for `kind`.
    pub fn vector(kind: ObservationKind, values: HashMap<String, f64>) -> Result<Self> {
        if !matches!(
            kind,
            ObservationKind::Categorical | ObservationKind::LabelMarginals
        ) || values.is_empty()
            || values.iter().any(|(key, value)| {
                blank(key) || !value.is_finite() || !(0.0..=1.0).contains(value)
            })
        {
            return Err(Diagnostic::for_code(DiagnosticCode::Observation));
        }
        let observation = match kind {
            ObservationKind::Categorical => Self::Categorical(values),
            ObservationKind::LabelMarginals => Self::LabelMarginals(values),
            _ => return Err(Diagnostic::for_code(DiagnosticCode::Observation)),
        };
        Ok(observation)
    }

    /// Returns the observation kind.
    pub const fn kind(&self) -> ObservationKind {
        match self {
            Self::Scalar(_) => ObservationKind::Scalar,
            Self::Bernoulli(_) => ObservationKind::Bernoulli,
            Self::ReportedConfidence(_) => ObservationKind::ReportedConfidence,
            Self::Categorical(_) => ObservationKind::Categorical,
            Self::LabelMarginals(_) => ObservationKind::LabelMarginals,
        }
    }

    pub(crate) fn report_value(&self) -> ReportObservation {
        match self {
            Self::Scalar(value) | Self::Bernoulli(value) | Self::ReportedConfidence(value) => {
                ReportObservation::Scalar {
                    kind: self.kind(),
                    value: *value,
                }
            }
            Self::Categorical(values) | Self::LabelMarginals(values) => ReportObservation::Vector {
                kind: self.kind(),
                values: values
                    .iter()
                    .map(|(key, value)| (key.clone(), *value))
                    .collect(),
            },
        }
    }
}

/// A closed observation value shape safe for report serialization.
#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum ReportObservation {
    Scalar {
        kind: ObservationKind,
        value: f64,
    },
    Vector {
        kind: ObservationKind,
        values: BTreeMap<String, f64>,
    },
}

/// A checked map of observations attached to one prediction.
#[derive(Clone, Debug)]
pub struct ObservationSet {
    values: HashMap<String, Observation>,
}

impl ObservationSet {
    /// Validates observation names and definition-kind agreement.
    pub fn new(
        values: HashMap<String, Observation>,
        definitions: &HashMap<String, ObservationDefinition>,
    ) -> Result<Self> {
        for (name, observation) in &values {
            let Some(definition) = definitions.get(name) else {
                return Err(Diagnostic::for_code(DiagnosticCode::Provenance));
            };
            if blank(name) || observation.kind() != definition.kind() {
                return Err(Diagnostic::for_code(DiagnosticCode::Provenance));
            }
        }
        Ok(Self { values })
    }

    /// Returns whether no observation was supplied.
    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    #[cfg(test)]
    pub(crate) fn values(&self) -> &HashMap<String, Observation> {
        &self.values
    }

    pub(crate) fn report_values(&self) -> BTreeMap<String, ReportObservation> {
        self.values
            .iter()
            .map(|(name, value)| (name.clone(), value.report_value()))
            .collect()
    }
}

/// A checked provenance binding for preparation evidence.
#[derive(Debug)]
pub struct PreparationDescriptor {
    method: String,
    version: String,
    configuration: Box<serde_json::value::RawValue>,
    evidence_indices: Vec<usize>,
}

impl PreparationDescriptor {
    /// Validates nonblank metadata and unique in-range evidence indices.
    #[cfg(test)]
    pub fn new(
        method: String,
        version: String,
        configuration: serde_json::Value,
        evidence_indices: Vec<usize>,
        evidence_count: usize,
    ) -> Result<Self> {
        let configuration = serde_json::value::to_raw_value(&configuration)
            .map_err(|_| Diagnostic::for_code(DiagnosticCode::Provenance))?;
        Self::new_raw(
            method,
            version,
            configuration,
            evidence_indices,
            evidence_count,
        )
    }

    pub(crate) fn new_raw(
        method: String,
        version: String,
        configuration: Box<serde_json::value::RawValue>,
        evidence_indices: Vec<usize>,
        evidence_count: usize,
    ) -> Result<Self> {
        if blank(&method)
            || blank(&version)
            || evidence_indices.is_empty()
            || evidence_indices
                .iter()
                .any(|index| *index >= evidence_count)
            || evidence_indices
                .iter()
                .enumerate()
                .any(|(position, index)| evidence_indices[..position].contains(index))
        {
            return Err(Diagnostic::for_code(DiagnosticCode::Provenance));
        }
        Ok(Self {
            method,
            version,
            configuration,
            evidence_indices,
        })
    }

    pub(crate) fn method(&self) -> &str {
        &self.method
    }

    pub(crate) fn version(&self) -> &str {
        &self.version
    }

    pub(crate) fn configuration(&self) -> &serde_json::value::RawValue {
        &self.configuration
    }

    pub(crate) fn evidence_indices(&self) -> &[usize] {
        &self.evidence_indices
    }
}

/// A checked source definition retained even when no row references it.
#[derive(Debug)]
pub struct SourceDefinition {
    kind: SourceKind,
    model: String,
    configuration: Box<serde_json::value::RawValue>,
    question_id: Option<String>,
    evidence: Vec<String>,
    observation_definitions: HashMap<String, ObservationDefinition>,
    preparation: Option<PreparationDescriptor>,
}

impl SourceDefinition {
    /// Creates a checked source definition.
    #[cfg(test)]
    pub fn new(
        kind: SourceKind,
        model: String,
        configuration: serde_json::Value,
        question_id: Option<String>,
        evidence: Vec<String>,
        observation_definitions: HashMap<String, ObservationDefinition>,
        preparation: Option<PreparationDescriptor>,
    ) -> Result<Self> {
        let configuration = serde_json::value::to_raw_value(&configuration)
            .map_err(|_| Diagnostic::for_code(DiagnosticCode::Provenance))?;
        Self::new_raw(
            kind,
            model,
            configuration,
            question_id,
            evidence,
            observation_definitions,
            preparation,
        )
    }

    pub(crate) fn new_raw(
        kind: SourceKind,
        model: String,
        configuration: Box<serde_json::value::RawValue>,
        question_id: Option<String>,
        evidence: Vec<String>,
        observation_definitions: HashMap<String, ObservationDefinition>,
        preparation: Option<PreparationDescriptor>,
    ) -> Result<Self> {
        if blank(&model)
            || question_id.as_deref().is_some_and(blank)
            || kind == SourceKind::ScoredChoice && question_id.is_none()
            || observation_definitions.keys().any(|name| blank(name))
        {
            return Err(Diagnostic::for_code(DiagnosticCode::Provenance));
        }
        Ok(Self {
            kind,
            model,
            configuration,
            question_id,
            evidence,
            observation_definitions,
            preparation,
        })
    }

    /// Returns the source kind.
    pub const fn kind(&self) -> SourceKind {
        self.kind
    }

    /// Borrows the observation definitions.
    pub fn observation_definitions(&self) -> &HashMap<String, ObservationDefinition> {
        &self.observation_definitions
    }

    /// Returns the checked source question identifier when supplied.
    pub fn question_id(&self) -> Option<&str> {
        self.question_id.as_deref()
    }

    pub(crate) fn model(&self) -> &str {
        &self.model
    }

    pub(crate) fn configuration(&self) -> &serde_json::value::RawValue {
        &self.configuration
    }

    pub(crate) fn preparation(&self) -> Option<&PreparationDescriptor> {
        self.preparation.as_ref()
    }

    /// Borrows the source's evidence paths in submitted array order.
    pub(crate) fn evidence(&self) -> &[String] {
        &self.evidence
    }
}

/// A prediction with concrete output and checked retained observations.
#[derive(Debug)]
pub struct Prediction<Output> {
    episode_id: EpisodeId,
    source_id: SourceId,
    output: Output,
    observations: ObservationSet,
}

impl<Output> Prediction<Output> {
    /// Creates a prediction from checked identity, source, output, and observations.
    pub fn new(
        episode_id: EpisodeId,
        source_id: SourceId,
        output: Output,
        observations: ObservationSet,
    ) -> Self {
        Self {
            episode_id,
            source_id,
            output,
            observations,
        }
    }
    /// Returns the episode identity.
    pub fn episode_id(&self) -> EpisodeId {
        self.episode_id
    }
    /// Borrows the source identity.
    pub fn source_id(&self) -> &SourceId {
        &self.source_id
    }
    /// Borrows the concrete output.
    pub fn output(&self) -> &Output {
        &self.output
    }

    pub(crate) fn observations(&self) -> &ObservationSet {
        &self.observations
    }
}

/// A complete checked configuration for a concrete evaluation policy.
#[derive(Debug)]
pub struct EvaluationConfig<Policy> {
    description: String,
    role: EvaluationRole,
    parent_run_id: Option<RunId>,
    requested_episode_ids: Option<Vec<EpisodeId>>,
    policy: Policy,
}

impl<Policy> EvaluationConfig<Policy> {
    /// Returns the exact declared population description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the checked evaluation role.
    pub const fn role(&self) -> EvaluationRole {
        self.role
    }

    /// Returns the optional checked parent run identity.
    pub const fn parent_run_id(&self) -> Option<RunId> {
        self.parent_run_id
    }

    /// Returns omitted selection distinctly from an explicit empty selection.
    pub fn requested_episode_ids(&self) -> Option<&[EpisodeId]> {
        self.requested_episode_ids.as_deref()
    }

    /// Borrows the concrete task policy.
    pub const fn policy(&self) -> &Policy {
        &self.policy
    }
}

#[cfg(test)]
impl EvaluationConfig<crate::model::single_label::SingleLabelPolicy> {
    /// Creates a configuration after parsing every selected episode identity.
    pub fn new(
        description: String,
        role: EvaluationRole,
        parent_run_id: Option<RunId>,
        requested_episode_ids: Option<Vec<EpisodeId>>,
    ) -> Result<Self> {
        Self::new_with_policy(
            description,
            role,
            parent_run_id,
            requested_episode_ids,
            crate::model::single_label::SingleLabelPolicy::AsRecorded,
        )
    }
}

impl<Policy> EvaluationConfig<Policy> {
    pub(crate) fn new_with_policy(
        description: String,
        role: EvaluationRole,
        parent_run_id: Option<RunId>,
        mut requested_episode_ids: Option<Vec<EpisodeId>>,
        policy: Policy,
    ) -> Result<Self> {
        if blank(&description) {
            return Err(Diagnostic::for_code(DiagnosticCode::Config));
        }
        if let Some(ids) = &mut requested_episode_ids {
            ids.sort();
            if let Some(pair) = ids.windows(2).find(|pair| pair[0] == pair[1]) {
                return Err(
                    Diagnostic::for_code(DiagnosticCode::DuplicateId).with_affected_ids([pair[0]])
                );
            }
        }
        Ok(Self {
            description,
            role,
            parent_run_id,
            requested_episode_ids,
            policy,
        })
    }
}

/// A selected population and its excluded episode identities.
#[derive(Debug)]
pub struct Population<Policy> {
    dataset_digest: ArtifactDigest,
    config: EvaluationConfig<Policy>,
    dataset_count: usize,
    selected_count: usize,
    selected: Vec<EpisodeId>,
    unselected: Vec<EpisodeId>,
}

/// A checked evaluation population role.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvaluationRole {
    /// A development population.
    Development,
    /// A held-out population.
    HeldOut,
}

impl<Policy> Population<Policy> {
    /// Creates a complete, disjoint partition of the checked dataset identity.
    pub fn new(
        dataset_digest: ArtifactDigest,
        config: EvaluationConfig<Policy>,
        mut selected: Vec<EpisodeId>,
        mut unselected: Vec<EpisodeId>,
    ) -> Result<Self> {
        selected.sort();
        unselected.sort();
        if selected.windows(2).any(|pair| pair[0] == pair[1])
            || unselected.windows(2).any(|pair| pair[0] == pair[1])
            || selected
                .iter()
                .any(|id| unselected.binary_search(id).is_ok())
            || config
                .requested_episode_ids()
                .is_some_and(|requested| requested != selected)
            || config.requested_episode_ids().is_none() && !unselected.is_empty()
        {
            return Err(Diagnostic::for_code(DiagnosticCode::Invariant));
        }
        let selected_count = selected.len();
        let dataset_count = selected_count + unselected.len();
        Ok(Self {
            dataset_digest,
            config,
            dataset_count,
            selected_count,
            selected,
            unselected,
        })
    }

    /// Returns the caller-supplied digest bound to this dataset.
    pub const fn dataset_digest(&self) -> ArtifactDigest {
        self.dataset_digest
    }

    /// Returns the complete checked evaluation configuration.
    pub fn config(&self) -> &EvaluationConfig<Policy> {
        &self.config
    }

    /// Returns the number of episodes in the complete dataset partition.
    pub const fn dataset_count(&self) -> usize {
        self.dataset_count
    }

    /// Returns the number of selected episodes.
    pub const fn selected_count(&self) -> usize {
        self.selected_count
    }
    /// Returns selected IDs in canonical UUID order.
    pub fn selected(&self) -> &[EpisodeId] {
        &self.selected
    }
    /// Returns unselected IDs in canonical UUID order.
    pub fn unselected(&self) -> &[EpisodeId] {
        &self.unselected
    }
}

fn blank(value: &str) -> bool {
    value.chars().all(char::is_whitespace)
}

/// The unit counted by a metric.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricUnit {
    /// Whole selected episodes.
    Episode,
    /// Individual label decisions.
    LabelDecision,
}

/// The scope from which a metric's population was drawn.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricScope {
    /// Every selected row.
    Selected,
    /// Selected rows with answered outputs.
    Answered,
    /// Raw answered rows used by signal diagnostics.
    RawAnswered,
}

/// Why a metric does not have a finite measured value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricStatus {
    /// The metric has a finite measured value.
    Defined,
    /// Required evidence is unavailable.
    NotApplicable,
    /// The selected population is empty.
    NoData,
    /// No answered row is available for an answered-only metric.
    NoAnsweredPredictions,
    /// A required denominator is zero.
    UndefinedZeroDenominator,
    /// A macro average contains zero-filled undefined class values.
    ContainsUndefinedClasses,
    /// The mathematical value is positive infinity.
    PositiveInfinity,
}

/// A checked metric value together with exact population and ratio accounting.
#[derive(Clone, Debug, Serialize)]
pub struct MetricResult {
    value: Option<f64>,
    status: MetricStatus,
    population_count: u64,
    #[serde(rename = "population_unit")]
    unit: MetricUnit,
    #[serde(rename = "population_scope")]
    scope: MetricScope,
    numerator: Option<u64>,
    denominator: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    special_value: Option<&'static str>,
}

impl MetricResult {
    pub(crate) fn ratio_with_population(
        numerator: u64,
        denominator: u64,
        population_count: u64,
        scope: MetricScope,
        unit: MetricUnit,
    ) -> Result<Self> {
        if denominator == 0 {
            return Ok(Self {
                value: None,
                status: MetricStatus::UndefinedZeroDenominator,
                population_count,
                unit,
                scope,
                numerator: Some(numerator),
                denominator: Some(denominator),
                special_value: None,
            });
        }
        let value = numerator as f64 / denominator as f64;
        if !value.is_finite() {
            return Err(Diagnostic::for_code(DiagnosticCode::Numeric));
        }
        Ok(Self {
            value: Some(value),
            status: MetricStatus::Defined,
            population_count,
            unit,
            scope,
            numerator: Some(numerator),
            denominator: Some(denominator),
            special_value: None,
        })
    }

    /// Constructs an explicit nonnumeric metric result.
    pub const fn status(
        status: MetricStatus,
        population_count: u64,
        scope: MetricScope,
        unit: MetricUnit,
    ) -> Self {
        Self {
            value: None,
            status,
            population_count,
            unit,
            scope,
            numerator: None,
            denominator: None,
            special_value: None,
        }
    }

    pub(crate) fn status_with_ratio(
        status: MetricStatus,
        numerator: u64,
        denominator: u64,
        population_count: u64,
        scope: MetricScope,
        unit: MetricUnit,
    ) -> Self {
        Self {
            value: None,
            status,
            population_count,
            unit,
            scope,
            numerator: Some(numerator),
            denominator: Some(denominator),
            special_value: None,
        }
    }

    pub(crate) fn finite_value(
        value: f64,
        status: MetricStatus,
        population_count: u64,
        scope: MetricScope,
        unit: MetricUnit,
    ) -> Result<Self> {
        if !value.is_finite() {
            return Err(Diagnostic::for_code(DiagnosticCode::Numeric));
        }
        Ok(Self {
            value: Some(value),
            status,
            population_count,
            unit,
            scope,
            numerator: None,
            denominator: None,
            special_value: None,
        })
    }

    pub(crate) const fn positive_infinity(
        population_count: u64,
        scope: MetricScope,
        unit: MetricUnit,
    ) -> Self {
        Self {
            value: None,
            status: MetricStatus::PositiveInfinity,
            population_count,
            unit,
            scope,
            numerator: None,
            denominator: None,
            special_value: Some("+infinity"),
        }
    }

    pub(crate) fn value(&self) -> Option<f64> {
        self.value
    }

    #[cfg(test)]
    pub(crate) const fn status_value(&self) -> MetricStatus {
        self.status
    }

    #[cfg(test)]
    pub(crate) const fn population_count(&self) -> u64 {
        self.population_count
    }

    #[cfg(test)]
    pub(crate) const fn unit(&self) -> MetricUnit {
        self.unit
    }

    #[cfg(test)]
    pub(crate) const fn scope(&self) -> MetricScope {
        self.scope
    }

    #[cfg(test)]
    pub(crate) const fn numerator(&self) -> Option<u64> {
        self.numerator
    }

    #[cfg(test)]
    pub(crate) const fn denominator(&self) -> Option<u64> {
        self.denominator
    }

    #[cfg(test)]
    pub(crate) const fn special_value(&self) -> Option<&'static str> {
        self.special_value
    }
}

#[cfg(test)]
mod tests {
    use super::{ArtifactDigest, Episode, EpisodeId, LabelVocabulary, Outcome, RunId, SourceId};
    use crate::model::{MultiLabelTask, SingleLabelTask, TaskDefinition};

    fn labels(values: &[&str]) -> Vec<String> {
        values.iter().map(ToString::to_string).collect()
    }

    #[test]
    fn canonical_identifiers() {
        for version in 1..=8 {
            let text = format!("01995c20-7d00-{version}000-8000-000000000001");
            assert!(EpisodeId::try_from(text.as_str()).is_ok());
            assert!(RunId::try_from(text.as_str()).is_ok());
        }

        for invalid in [
            "00000000-0000-0000-0000-000000000000",
            "ffffffff-ffff-ffff-ffff-ffffffffffff",
            "01995C20-7D00-7000-8000-000000000001",
            "{01995c20-7d00-7000-8000-000000000001}",
            "urn:uuid:01995c20-7d00-7000-8000-000000000001",
            "01995c207d0070008000000000000001",
            "01995c20-7d00-0000-8000-000000000001",
        ] {
            assert!(EpisodeId::try_from(invalid).is_err());
        }

        assert!(SourceId::try_from("").is_err());
        assert!(SourceId::try_from(" \t\n").is_err());
        assert_eq!(SourceId::try_from(" source ").unwrap().as_str(), " source ");

        let digest = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        assert_eq!(
            ArtifactDigest::try_from(digest).unwrap().to_string(),
            digest
        );
        for invalid in [
            "0123456789ABCDEF0123456789abcdef0123456789abcdef0123456789abcdef",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdeg",
            "0123456789abcdef",
        ] {
            assert!(ArtifactDigest::try_from(invalid).is_err());
        }
    }

    #[test]
    fn vocabulary_exact_identity() {
        let task = TaskDefinition::single_label(labels(&["Billing", " billing", "café", "sales "]))
            .unwrap();
        let vocabulary = task.vocabulary();

        assert_eq!(vocabulary.len(), 4);
        assert!(vocabulary.lookup("billing").is_none());
        assert_eq!(
            vocabulary.label(&vocabulary.lookup(" billing").unwrap()),
            Some(" billing")
        );
        assert_eq!(
            vocabulary.label(&vocabulary.lookup("café").unwrap()),
            Some("café")
        );
        assert!(TaskDefinition::single_label(labels(&["only"])).is_err());
        assert!(TaskDefinition::multi_label(Vec::new()).is_err());
        assert!(TaskDefinition::multi_label(labels(&["x", "x"])).is_err());
        assert!(TaskDefinition::multi_label(labels(&["\t"])).is_err());
    }

    #[test]
    fn model_boundary_visibility() {
        let task = TaskDefinition::single_label(labels(&["left", "right"])).unwrap();
        assert!(matches!(&task, TaskDefinition::SingleLabel(_)));
        let vocabulary = task.vocabulary();
        let left = vocabulary.lookup("left").unwrap();
        let label_set = vocabulary.label_set(["right", "left"]).unwrap();
        assert_eq!(label_set.labels().collect::<Vec<_>>(), ["left", "right"]);
        assert!(vocabulary.label_set(["left", "left"]).is_err());

        let other_task = TaskDefinition::single_label(labels(&["left", "right"])).unwrap();
        assert!(other_task.vocabulary().label(&left).is_none());
        assert!(SingleLabelTask::new(labels(&["only"])).is_err());
        assert!(MultiLabelTask::new(labels(&["only"])).is_ok());
        assert!(TaskDefinition::single_label(labels(&["only"])).is_err());

        let id = EpisodeId::try_from("01995c20-7d00-7000-8000-000000000001").unwrap();
        let episode = Episode::new(id, left.clone());
        assert_eq!(
            episode.id().to_string(),
            "01995c20-7d00-7000-8000-000000000001"
        );
        assert_eq!(vocabulary.label(episode.target()), Some("left"));

        let answered = Outcome::answered(left);
        assert_eq!(
            vocabulary.label(answered.answered_target().unwrap()),
            Some("left")
        );
        assert_eq!(answered.abstention_reason(), None);
        assert_eq!(
            Outcome::<super::LabelIndex>::abstained(None)
                .unwrap()
                .abstention_reason(),
            None
        );
        assert_eq!(
            Outcome::<super::LabelIndex>::abstained(Some("manual_review".to_owned()))
                .unwrap()
                .abstention_reason(),
            Some("manual_review")
        );
        assert!(Outcome::<super::LabelIndex>::abstained(Some(" ".to_owned())).is_err());

        let empty = vocabulary.label_set([]).unwrap();
        assert!(empty.labels().next().is_none());
        let _vocabulary: &LabelVocabulary = vocabulary;
    }
}
