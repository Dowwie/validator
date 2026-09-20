pub mod common;
pub mod multi_label;
pub mod single_label;

pub use common::LabelVocabulary;
use multi_label::MultiLabelEvaluation;
use single_label::SingleLabelEvaluation;

/// A closed validated task boundary used only inside the library.
#[derive(Debug)]
pub(crate) enum ValidatedTask {
    SingleLabel(SingleLabelEvaluation),
    MultiLabel(MultiLabelEvaluation),
}

/// A closed task definition with an owned, checked label vocabulary.
#[derive(Debug)]
pub enum TaskDefinition {
    /// A categorical task with at least two labels.
    SingleLabel(SingleLabelTask),
    /// A complete label-set task with at least one label.
    MultiLabel(MultiLabelTask),
}

impl TaskDefinition {
    /// Creates a single-label task from at least two exact labels.
    pub fn single_label(labels: impl IntoIterator<Item = String>) -> crate::Result<Self> {
        SingleLabelTask::new(labels).map(Self::SingleLabel)
    }

    /// Creates a multi-label task from at least one exact label.
    pub fn multi_label(labels: impl IntoIterator<Item = String>) -> crate::Result<Self> {
        MultiLabelTask::new(labels).map(Self::MultiLabel)
    }

    /// Returns the checked vocabulary for this task.
    pub const fn vocabulary(&self) -> &LabelVocabulary {
        match self {
            Self::SingleLabel(task) => task.vocabulary(),
            Self::MultiLabel(task) => task.vocabulary(),
        }
    }
}

/// A checked single-label vocabulary that cannot contain fewer than two labels.
#[derive(Debug)]
pub struct SingleLabelTask {
    vocabulary: LabelVocabulary,
}

impl SingleLabelTask {
    /// Creates a single-label vocabulary from at least two exact labels.
    pub fn new(labels: impl IntoIterator<Item = String>) -> crate::Result<Self> {
        LabelVocabulary::for_single_label(labels).map(|vocabulary| Self { vocabulary })
    }

    /// Returns the checked vocabulary.
    pub const fn vocabulary(&self) -> &LabelVocabulary {
        &self.vocabulary
    }
}

/// A checked multi-label vocabulary that contains at least one label.
#[derive(Debug)]
pub struct MultiLabelTask {
    vocabulary: LabelVocabulary,
}

impl MultiLabelTask {
    /// Creates a multi-label vocabulary from at least one exact label.
    pub fn new(labels: impl IntoIterator<Item = String>) -> crate::Result<Self> {
        LabelVocabulary::for_multi_label(labels).map(|vocabulary| Self { vocabulary })
    }

    /// Returns the checked vocabulary.
    pub const fn vocabulary(&self) -> &LabelVocabulary {
        &self.vocabulary
    }
}

/// The canonical wire-format version accepted and emitted by Validator.
pub const WIRE_VERSION: u32 = 2;

/// The ratified Validator specification version.
pub const SPECIFICATION_VERSION: &str = "1.2-draft";

/// The permitted absolute error when validating a categorical probability sum.
pub const CATEGORICAL_SUM_TOLERANCE: f64 = 1e-9;

/// The absolute component of the fixture-comparison tolerance.
pub const FIXTURE_ABSOLUTE_TOLERANCE: f64 = 1e-12;

/// The relative component of the fixture-comparison tolerance.
pub const FIXTURE_RELATIVE_TOLERANCE: f64 = 1e-10;

/// The fixed number of equal-width bins used by signal diagnostics.
pub const TEN_BIN_COUNT: usize = 10;

#[cfg(test)]
mod tests {
    use super::{
        CATEGORICAL_SUM_TOLERANCE, FIXTURE_ABSOLUTE_TOLERANCE, FIXTURE_RELATIVE_TOLERANCE,
        SPECIFICATION_VERSION, TEN_BIN_COUNT, WIRE_VERSION,
    };

    #[test]
    fn shared_contract_constants_match_the_specification() {
        assert_eq!(WIRE_VERSION, 2);
        assert_eq!(SPECIFICATION_VERSION, "1.2-draft");
        assert_eq!(CATEGORICAL_SUM_TOLERANCE, 1e-9);
        assert_eq!(FIXTURE_ABSOLUTE_TOLERANCE, 1e-12);
        assert_eq!(FIXTURE_RELATIVE_TOLERANCE, 1e-10);
        assert_eq!(TEN_BIN_COUNT, 10);
    }
}
