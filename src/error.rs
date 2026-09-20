use std::fmt;

use serde::Serialize;

/// A result whose failure preserves Validator's stable diagnostic category.
pub type Result<T> = std::result::Result<T, Diagnostic>;

/// A stable, machine-readable Validator diagnostic code.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum DiagnosticCode {
    /// The submitted JSON could not be parsed.
    #[serde(rename = "E_PARSE")]
    Parse,
    /// The submitted shape or fields violate the schema.
    #[serde(rename = "E_SCHEMA")]
    Schema,
    /// An identifier is malformed or unknown.
    #[serde(rename = "E_ID")]
    Id,
    /// An identifier appears more than once where uniqueness is required.
    #[serde(rename = "E_DUPLICATE_ID")]
    DuplicateId,
    /// A label is malformed or outside the declared vocabulary.
    #[serde(rename = "E_LABEL")]
    Label,
    /// Probability evidence violates its declared contract.
    #[serde(rename = "E_PROBABILITY")]
    Probability,
    /// Reported confidence violates its declared contract.
    #[serde(rename = "E_CONFIDENCE")]
    Confidence,
    /// Configuration or policy values are invalid.
    #[serde(rename = "E_CONFIG")]
    Config,
    /// Dataset and prediction identifiers do not align.
    #[serde(rename = "E_ALIGNMENT")]
    Alignment,
    /// Retained observation evidence is invalid.
    #[serde(rename = "E_OBSERVATION")]
    Observation,
    /// Provenance or artifact bindings are inconsistent.
    #[serde(rename = "E_PROVENANCE")]
    Provenance,
    /// Compared artifacts do not meet their compatibility contract.
    #[serde(rename = "E_COMPARISON")]
    Comparison,
    /// A filesystem operation failed.
    #[serde(rename = "E_IO")]
    Io,
    /// An output destination already exists.
    #[serde(rename = "E_OUTPUT_EXISTS")]
    OutputExists,
    /// A numeric calculation could not produce a valid result.
    #[serde(rename = "E_NUMERIC")]
    Numeric,
    /// Internal accounting or replay invariants are inconsistent.
    #[serde(rename = "E_INVARIANT")]
    Invariant,
}

impl DiagnosticCode {
    /// Every stable diagnostic code required by the Validator specification.
    pub const ALL: [Self; 16] = [
        Self::Parse,
        Self::Schema,
        Self::Id,
        Self::DuplicateId,
        Self::Label,
        Self::Probability,
        Self::Confidence,
        Self::Config,
        Self::Alignment,
        Self::Observation,
        Self::Provenance,
        Self::Comparison,
        Self::Io,
        Self::OutputExists,
        Self::Numeric,
        Self::Invariant,
    ];

    /// Returns the process exit category prescribed for this diagnostic code.
    #[must_use]
    pub const fn exit_category(self) -> ExitCategory {
        match self {
            Self::Parse
            | Self::Schema
            | Self::Id
            | Self::DuplicateId
            | Self::Label
            | Self::Probability
            | Self::Confidence
            | Self::Config
            | Self::Alignment
            | Self::Observation
            | Self::Provenance
            | Self::Comparison => ExitCategory::Input,
            Self::Io | Self::OutputExists => ExitCategory::Filesystem,
            Self::Numeric | Self::Invariant => ExitCategory::Numeric,
        }
    }

    const fn stage(self) -> DiagnosticStage {
        match self {
            Self::Parse => DiagnosticStage::Parse,
            Self::Schema => DiagnosticStage::Schema,
            Self::Id
            | Self::DuplicateId
            | Self::Label
            | Self::Probability
            | Self::Confidence
            | Self::Observation => DiagnosticStage::Validation,
            Self::Config => DiagnosticStage::Configuration,
            Self::Alignment => DiagnosticStage::Alignment,
            Self::Provenance => DiagnosticStage::Replay,
            Self::Comparison => DiagnosticStage::Comparison,
            Self::Io | Self::OutputExists => DiagnosticStage::Filesystem,
            Self::Numeric => DiagnosticStage::Numeric,
            Self::Invariant => DiagnosticStage::Accounting,
        }
    }

    const fn message(self) -> &'static str {
        match self {
            Self::Parse => "input could not be parsed",
            Self::Schema => "input does not match the required schema",
            Self::Id => "identifier is invalid",
            Self::DuplicateId => "identifier is duplicated",
            Self::Label => "label is invalid",
            Self::Probability => "probability evidence is invalid",
            Self::Confidence => "reported confidence is invalid",
            Self::Config => "configuration is invalid",
            Self::Alignment => "identifiers do not align",
            Self::Observation => "observation evidence is invalid",
            Self::Provenance => "provenance evidence is inconsistent",
            Self::Comparison => "comparison contract is not satisfied",
            Self::Io => "filesystem operation failed",
            Self::OutputExists => "output destination already exists",
            Self::Numeric => "numeric calculation failed",
            Self::Invariant => "internal accounting invariant failed",
        }
    }
}

/// A process exit category that does not depend on diagnostic text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExitCategory {
    /// A CLI, schema, input, policy, alignment, or comparison-contract error.
    Input,
    /// A filesystem error, missing artifact, or existing output destination.
    Filesystem,
    /// A numeric failure or internal accounting/replay inconsistency.
    Numeric,
}

impl ExitCategory {
    /// Returns the stable process exit code for this category.
    #[must_use]
    pub const fn code(self) -> i32 {
        match self {
            Self::Input => 2,
            Self::Filesystem => 3,
            Self::Numeric => 4,
        }
    }
}

/// The processing stage where a diagnostic arose.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticStage {
    /// JSON parsing.
    Parse,
    /// Schema validation.
    Schema,
    /// Semantic record validation.
    Validation,
    /// Evaluation configuration validation.
    Configuration,
    /// Dataset and prediction identifier alignment.
    Alignment,
    /// Stored-artifact replay or provenance verification.
    Replay,
    /// Run comparison compatibility validation.
    Comparison,
    /// Filesystem access or publication.
    Filesystem,
    /// Numeric calculation.
    Numeric,
    /// Internal metric accounting.
    Accounting,
}

/// A safe structured error suitable for machine-readable diagnostics.
#[derive(Debug, Serialize)]
pub struct Diagnostic {
    code: DiagnosticCode,
    stage: DiagnosticStage,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    affected_ids: Vec<String>,
    message: &'static str,
}

impl Diagnostic {
    /// Creates a generic diagnostic with a fixed safe message for `code`.
    ///
    /// The initial foundation intentionally does not accept payload text or
    /// credentials. Later producers add only domain-specific safe context.
    #[must_use]
    pub const fn for_code(code: DiagnosticCode) -> Self {
        Self {
            code,
            stage: code.stage(),
            path: None,
            affected_ids: Vec::new(),
            message: code.message(),
        }
    }

    /// Returns the stable diagnostic code.
    #[must_use]
    pub const fn code(&self) -> DiagnosticCode {
        self.code
    }

    /// Returns the stage associated with the diagnostic code.
    #[must_use]
    pub const fn stage(&self) -> DiagnosticStage {
        self.stage
    }

    /// Returns the prescribed exit category without inspecting the message.
    #[must_use]
    pub const fn exit_category(&self) -> ExitCategory {
        self.code.exit_category()
    }

    /// Serializes this diagnostic as valid JSON with no opaque payload fields.
    ///
    /// # Errors
    ///
    /// Returns a serialization error if the JSON serializer cannot write the
    /// diagnostic. The current fixed diagnostic fields are all finite JSON values.
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    /// Serializes this diagnostic as the complete versioned machine error document.
    ///
    /// # Errors
    ///
    /// Returns a serialization error if the fixed safe diagnostic document cannot
    /// be represented as JSON.
    pub fn to_machine_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(&MachineError::from(self))
    }
}

#[derive(Serialize)]
struct MachineError<'a> {
    schema_version: u32,
    kind: &'static str,
    status: &'static str,
    #[serde(flatten)]
    diagnostic: &'a Diagnostic,
}

impl<'a> From<&'a Diagnostic> for MachineError<'a> {
    fn from(diagnostic: &'a Diagnostic) -> Self {
        Self {
            schema_version: crate::WIRE_VERSION,
            kind: "error",
            status: "error",
            diagnostic,
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.message)
    }
}

impl std::error::Error for Diagnostic {}

#[cfg(test)]
mod tests {
    use super::{Diagnostic, DiagnosticCode, ExitCategory};

    #[test]
    fn typed_error_categories() {
        for code in DiagnosticCode::ALL {
            let diagnostic = Diagnostic::for_code(code);
            let expected = match code {
                DiagnosticCode::Parse
                | DiagnosticCode::Schema
                | DiagnosticCode::Id
                | DiagnosticCode::DuplicateId
                | DiagnosticCode::Label
                | DiagnosticCode::Probability
                | DiagnosticCode::Confidence
                | DiagnosticCode::Config
                | DiagnosticCode::Alignment
                | DiagnosticCode::Observation
                | DiagnosticCode::Provenance
                | DiagnosticCode::Comparison => ExitCategory::Input,
                DiagnosticCode::Io | DiagnosticCode::OutputExists => ExitCategory::Filesystem,
                DiagnosticCode::Numeric | DiagnosticCode::Invariant => ExitCategory::Numeric,
            };

            assert_eq!(diagnostic.exit_category(), expected);
            assert_eq!(diagnostic.exit_category().code(), expected.code());
        }
    }

    #[test]
    fn safe_error_serialization() {
        const OPAQUE_INPUT: &str = "opaque-input-sentinel";
        const CREDENTIAL: &str = "credential-sentinel";

        for code in DiagnosticCode::ALL {
            let serialized = Diagnostic::for_code(code)
                .to_json()
                .expect("fixed diagnostic fields serialize to JSON");
            let value: serde_json::Value =
                serde_json::from_str(&serialized).expect("diagnostic serialization is valid JSON");

            assert!(!serialized.contains(OPAQUE_INPUT));
            assert!(!serialized.contains(CREDENTIAL));
            assert_eq!(
                value["code"],
                serde_json::to_value(code).expect("code serializes")
            );
            assert!(value["message"].is_string());
        }
    }
}
