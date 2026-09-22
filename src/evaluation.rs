//! Checked arithmetic shared by concrete evaluators.

use crate::{Diagnostic, DiagnosticCode, Result};

pub(crate) mod multi_label;
pub(crate) mod single_label;

/// Adds counts without wrapping.
pub(crate) fn checked_add(left: u64, right: u64) -> Result<u64> {
    left.checked_add(right)
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Numeric))
}

/// Multiplies counts without wrapping.
pub(crate) fn checked_mul(left: u64, right: u64) -> Result<u64> {
    left.checked_mul(right)
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Numeric))
}

/// Subtracts counts without underflow.
pub(crate) fn checked_sub(left: u64, right: u64) -> Result<u64> {
    left.checked_sub(right)
        .ok_or_else(|| Diagnostic::for_code(DiagnosticCode::Numeric))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_overflow_is_error() {
        assert!(checked_add(u64::MAX, 1).is_err());
        assert!(checked_mul(u64::MAX, 2).is_err());
    }
}
