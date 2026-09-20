# T014 pre-freeze constructor-mapping correction handoff

Status: complete narrow correction. T014 behavior and every completed process
assertion from response031 are preserved. The checkpoint remains **not
warning-free**: the required warning-denied Clippy command exits 101 only for the
reconciled staged inventory, and T027/T035 retain the fully clean gate.

## Change

`src/model/common.rs` now puts exactly three Value-level test conveniences behind
`#[cfg(test)]`:

- `ObservationSet::is_empty` (only current consumer: a unit assertion);
- `PreparationDescriptor::new`;
- `SourceDefinition::new`.

Production admission continues to use `PreparationDescriptor::new_raw` and
`SourceDefinition::new_raw`. No production behavior, formula, schema, parser,
assertion, export, suppression, or replacement abstraction changed. The original
unit callers compile and execute in the test build.

Final SHA-256 values:

| Artifact | SHA-256 |
|---|---|
| `src/model/common.rs` | `5dd0c2652439434ba9496a503c6317d8775cdbc90d8ded7b7dac5d351005d4f3` |
| `tests/cli.rs` (unchanged) | `fc03a9d04dbe0c58faab9574ef9eb68d8f4347d6d43d6d5424ae55e7e6cdd111` |
| response031 (unchanged) | `e9e1478791aa0e258946fe0e7f2a364c183dd0145d8204f3b19a56f0ee7cf755` |

The exact input/output byte record from response031 is unchanged: golden and
published golden `c05db4f0dd82205cbe3236e3029927a86b98392f35eb4cd5753710b0e41a24fa`;
predictions `26f764ca12aded7e5a51d8f104cfb73a9a6e089cf724905635cc6bd0aa265595`;
config `8583c9444461b1202f04040c8af6166f816796f22a769ff2a4c5e146bb10a2c1`;
bound evidence `91d8523c5b68f4f0f98c1509bf9b30aca5f2e0b55fbd1eb0c3cf4f03ab999c5e`;
and report/receipt result SHA-256
`ac18a82716129febe18575d363e9e432357f2a2599c324b15f7977adecdfa709`.

## Verification

| Command | Exit | Count/result |
|---|---:|---|
| `cargo fmt --all -- --check` | 0 | Passed. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Only the 26 staged `dead_code` diagnostics below. |
| `cargo test --all-features --locked` | 0 | 39 library, 3 CLI, 8 conformance, 0 doc tests passed. This covers the existing constructor and `is_empty` unit callers after the cfg-only change. |
| `cargo build --release --locked --bin validator` | 0 | Passed with only staged warnings. |
| `git diff --check` | 0 | Passed. |

The 18 named T007-T014 filters from response031 remain the completed evidence:
each exited 0 with one passing test. This correction changes only conditional
availability in production builds; the complete locked test run above recompiles
and executes those test paths.

## Exact residual Clippy inventory

The exact command reported 26 `dead_code` errors, no ordinary/new warning class,
and no constructor or `ObservationSet::is_empty` diagnostic:

| Diagnostic group | Staged consumer or reason |
|---|---|
| `src/evaluation.rs:14 checked_mul` | T018 multi-label `N*K` accounting. |
| `src/model.rs:8 TaskDefinition`; `single_label`, `multi_label`, `vocabulary`, `is_single_label` | T018 closed task boundary. |
| `src/model.rs:42 SingleLabelTask`; `new`, `vocabulary` | T018 closed task model. |
| `src/model.rs:60 MultiLabelTask`; `new`, `vocabulary` | T018. |
| `src/model/common.rs:400 LabelVocabulary::for_multi_label`, `label_set`; `LabelSet`, `is_empty`, `contains` | T018 multi-label admission and set targets/outputs. |
| `src/model/common.rs:508 Episode`; `new`, `id`, `input`, `target` | T015 inspection. |
| `src/model/common.rs:786 ObservationSet::values` | T015 inspection presentation. |
| `src/model/common.rs:1019 EvaluationConfig.policy`, `policy()`; `src/model/common.rs:1079 Population.dataset_digest`, `dataset_digest()` | T016 compatibility. |
| `src/model/common.rs:1170 MetricUnit::LabelDecision`; `src/model/common.rs:1223 MetricResult::ratio`, `status_value`, `population_count`, `unit`, `scope`, `numerator`, `denominator`, `special_value` | T016 typed metric comparison and T018 label-decision metrics. |
| `src/model/single_label.rs:115 signal_availability` | T018 artifact-wide signal admission. |
| `src/validation/wire.rs:15 GoldenDataset.schema_version`; `:23 SingleLabelTask.kind`; `:38 GoldenEpisode.input`; `:51 PredictionArtifact.schema_version`; `:218 EvaluationConfig.schema_version`; `:239 SingleLabelDecision.signal`, `minimum` | Current T010 strict Serde DTO validation. |

Clippy ended with `could not compile validator (lib) due to 26 previous errors`;
the library test target showed the corresponding staged subset. The removed
constructor and `is_empty` diagnostics prove the owner’s false-T029 mapping no
longer defers a production dead-code convenience. No review or T015 work began.
