//! Shared Validator contracts.
//!
//! This library exposes the stable diagnostic and numerical-policy definitions
//! used by the Validator executable. Command behavior is added through the
//! application API as that behavior is implemented.

#![deny(missing_docs)]

mod app;
mod artifacts;
mod comparison;
mod error;
mod evaluation;
pub(crate) mod model;
mod validation;

pub use app::{ComparisonOptions, EvaluationOptions, EvaluationReceipt, compare, evaluate};
pub use error::{Diagnostic, DiagnosticCode, DiagnosticStage, ExitCategory, Result};
pub use model::{
    CATEGORICAL_SUM_TOLERANCE, FIXTURE_ABSOLUTE_TOLERANCE, FIXTURE_RELATIVE_TOLERANCE,
    SPECIFICATION_VERSION, TEN_BIN_COUNT, WIRE_VERSION,
};
