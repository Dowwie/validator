# T013/T014 focused repair handoff

Status: the two verifier findings from response034 are repaired. No T015+ work,
scoring formula/tolerance change, lint suppression, or unrelated cleanup was
made. This checkpoint remains **not warning-free**: the required warning-denied
Clippy command exits 101 only for the unchanged staged inventory, and T027/T035
retain the fully clean gate.

## Manifest and change scope

Before editing, the dispatch035 manifest hashes for `src/app.rs`,
`src/model/single_label.rs`, `schemas/v2/report.schema.json`, `tests/cli.rs`,
and `tests/conformance.rs` matched. The verifier response hash also matched
`bb1fc45987ded8236ef9a7e244dfc2f8ef0782d826a32bab7832cea0c732236d`.

Changed artifacts and final SHA-256 values:

| Artifact | SHA-256 |
|---|---|
| `src/app.rs` | `819627406e44e184df13d8baf4c1feb115ea0444cf9b7518f83afd2b9e02b875` |
| `src/model/single_label.rs` | `899963586c722a37af0ce0dd8f28d03b86423c8c9bbce431b14280ae8a6cee98` |
| `schemas/v2/report.schema.json` | `4ff9c9f1ad59cd2ed73965c9cf154209e6eaf1b6871691c8d5128ca4cbd7ff46` |
| `tests/cli.rs` | `3b0cc62faedcb00e46e76a407b3bdf384da90b4596d593a4bf53da46273a567e` |
| `tests/conformance.rs` | `982f128ba6a1b66e3071113b3c05a7fd37ef7269d20a7fa8f34644bb596075a2` |

## Finding 1: truthful non-scoring check diagnostics

`src/model/single_label.rs:141-170` adds the shared
`normalization_diagnostics` helper. It walks admitted `SingleLabelEvaluation`
rows, uses each checked categorical distribution's submitted and working vectors,
counts only submitted/working differences with checked count arithmetic and
conversion, and reports the maximum absolute submitted-sum error. It does not
evaluate rows, assemble results, generate identity/time, or publish.

`src/app.rs` calls that helper while constructing `CheckResult`; report assembly
at `src/model/single_label.rs:542-552` calls the same helper and uses its fields
in report integrity. The previous result-derived duplicate calculation was
removed.

`tests/cli.rs:29-35` supplies the valid within-tolerance categorical vector
`[0.7, 0.2, 0.1000000005]`; `116-125` validates actual check stdout against the
locked schema, asserts `normalized_count == 1`, asserts a nonzero error, and
independently checks it against `(0.7 + 0.2 + 0.1000000005 - 1.0).abs()` within
`validator::FIXTURE_ABSOLUTE_TOLERANCE`. The check still asserts no output path.
The existing invalid-input mutation changes from `C` to `D` because the expanded
valid fixture now correctly includes C.

An independent release-binary run emitted this schema-valid check document:

```json
{"schema_version":2,"kind":"check","status":"complete","integrity":{"source_count":1,"selected_count":1,"prediction_count":1,"missing_ids":[],"extra_ids":[],"signal_availability":"categorical","normalized_count":1,"maximum_sum_error":4.999998193255806e-10},"selected_ids":["01995c20-7d00-7000-8000-000000000001"]}
```

The same run published an evaluation receipt with report SHA-256
`169058cc1d80ba5866368b25b607fa8e3f19143b09f6ad0499368d2c21fe2911`.
Exact submitted/published digest pairs were golden
`f87243e1ff71fc4e0371049495b765b3ef665f00510a122f0f84046b58a3b486`,
predictions `471a3b3c2f3d1c800bdb3dc4a231a3370a0c49654652738654f3dd7330694eca`,
config `8583c9444461b1202f04040c8af6166f816796f22a769ff2a4c5e146bb10a2c1`,
and bound evidence `91d8523c5b68f4f0f98c1509bf9b30aca5f2e0b55fbd1eb0c3cf4f03ab999c5e`.
The respective published files had the exact same four hashes.

## Finding 2: minimum-two report vocabulary

`schemas/v2/report.schema.json:50` now requires `task.labels.minItems: 2` for
the single-label report branch only. `tests/conformance.rs:126-153` makes the
handcrafted positive report coherent for two labels: task vocabulary, matrix
columns and rows, per-class entries, and categorical submitted/working vectors
all use A/B. `173-175` changes only the completed report's task labels to `["A"]`
and proves that mutation is schema-invalid.

## Required checks

| Command | Exit | Count/result |
|---|---:|---|
| `cargo test --locked --test cli cli_check_evaluate -- --nocapture` | 0 | 1 passed. |
| `cargo test --locked --test conformance single_report_schema -- --nocapture` | 0 | 1 passed. |
| `cargo test --locked --lib report_sources_and_privacy -- --nocapture` | 0 | 1 passed. |
| `cargo fmt --all -- --check` | 0 | Passed. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Only the 26 staged `dead_code` diagnostics below. |
| `cargo test --all-features --locked` | 0 | 39 library, 3 CLI, 8 conformance, and 0 doc tests passed. |
| `cargo build --release --locked --bin validator` | 0 | Passed with only staged warnings. |
| `git diff --check` | 0 | Passed. |

## Exact staged Clippy reconciliation

No new helper, test, or schema warning appeared. The exact command still reports
only these 26 `dead_code` diagnostics: `checked_mul`; `TaskDefinition` and its
four methods; `SingleLabelTask` and its two methods; `MultiLabelTask` and its two
methods; `LabelVocabulary::{for_multi_label,label_set}`; `LabelSet` and
`{is_empty,contains}`; `Episode` and `{new,id,input,target}`;
`ObservationSet::values`; `EvaluationConfig::policy` field and method;
`Population::dataset_digest` field and method; `MetricUnit::LabelDecision`;
`MetricResult::{ratio,status_value,population_count,unit,scope,numerator,denominator,special_value}`;
`signal_availability`; and the six strict Serde DTO groups
`GoldenDataset::schema_version`, `SingleLabelTask::kind`, `GoldenEpisode::input`,
`PredictionArtifact::schema_version`, `EvaluationConfig::schema_version`, and
`SingleLabelDecision::{signal,minimum}`.

These are the owner-staged T015-T021/Serde inventory from response032/manifest033;
the command ended with `could not compile validator (lib) due to 26 previous
errors`. There is no ordinary or new lint class, no suppression, fake consumer,
or exported workaround. The checkpoint is not warning-free; T027/T035 retain the
clean-gate obligation. No review or T015 work began.
