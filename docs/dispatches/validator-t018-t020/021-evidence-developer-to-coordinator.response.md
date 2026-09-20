# T020 mechanical Clippy correction handoff

Status: complete local candidate for the combined T018–T020 review; not owner
acceptance.

## Scope and identity

All prompt021 starting hashes matched before editing. I changed only three
mechanical expressions:

- `src/app.rs:369`: passed `evidence` directly to the single-label report builder.
- `src/app.rs:390`: passed `evidence` directly to the multi-label report builder.
- `src/model/multi_label.rs:463`: collected the vocabulary/marginal zip directly,
  removing its identity map.

No test, schema, API, behavior, tolerance, oracle, dependency, visibility, or
suppression changed. Final hashes are:

| Artifact | SHA-256 |
|---|---|
| `src/app.rs` | `3324dad26b9571529bdfb1a75b2bb17a9900bbb8f1a65ad822e7b389b76c6671` |
| `src/model/multi_label.rs` | `d0f8a9c7a6c164ae3d34f6111a31c2b52aeead7021c9cf9b25de7d079d7eb46a` |
| `tests/conformance.rs` | `f184b28026250ec38da1d228d8bf563272142d21c5844ac2ba4e30e9ab8de291` unchanged |
| `tests/cli.rs` | `edaf69f7744fbc2df630e280234041e8f9bd0608ed1c2bdd719187090ca85275` unchanged |
| `schemas/v2/check.schema.json` | `1bfd2fb3a644df1203b043244b38a9ed80d404cae5068901b7f162ea6dd662b9` unchanged |
| `schemas/v2/report.schema.json` | `7215342a2958fd4d9e9735c2837130a02837d205db93b342b9f64d62a882fcf1` unchanged |
| `schemas/v2/inspection.schema.json` | `3c32ad02a1376b790294bcd655c8c4ffc711a4d90274546303e60577425fc176` unchanged |

## Verification

Cargo list output contained all six required filter names. Each selected one test
and exited 0: `multi_label_checked_admission` and `cross_task_boundaries` had 43
filtered library tests; each conformance filter had 14 filtered tests; and
`shared_commands_multi_label` had 6 filtered CLI tests.

| Command | Exit | Actual result |
|---|---:|---|
| `cargo test --locked --lib multi_label_checked_admission -- --nocapture` | 0 | 1 passed |
| `cargo test --locked --lib cross_task_boundaries -- --nocapture` | 0 | 1 passed |
| `cargo test --locked --test conformance multi_label_hard_oracles -- --nocapture` | 0 | 1 passed |
| `cargo test --locked --test conformance equal_counts_distinct_exact_sets -- --nocapture` | 0 | 1 passed |
| `cargo test --locked --test conformance marginal_loss_and_bins -- --nocapture` | 0 | 1 passed |
| `cargo test --locked --test cli shared_commands_multi_label -- --nocapture` | 0 | 1 passed |
| `cargo fmt --all -- --check` | 0 | passed |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | only 20 staged `dead_code` diagnostics; all three ordinary diagnostics are gone |
| `cargo test --all-features --locked` | 0 | 44 library, 7 CLI, 15 conformance, and 0 doc tests passed |
| `cargo build --release --locked --bin validator` | 0 | release binary built |
| `git diff --check` | 0 | passed |

## Residual staged lint reconciliation

The remaining 20 diagnostics are all `dead_code`, with no ordinary Clippy class.
They are the owner-authorized incomplete-consumer inventory:

- `src/model.rs`: `TaskDefinition` and methods, `SingleLabelTask` and methods,
  and `MultiLabelTask` and methods. These preserve the closed checked-task
  boundary for T018 and its later task consumers.
- `src/model/common.rs`: `LabelSet::is_empty`/`contains`, `Episode` and its
  constructors/accessors, `ObservationSet::values`, `EvaluationConfig` policy
  field/accessor, and listed `MetricResult` accessors. These are staged for the
  T018 set path, T021 policy application, and T022 task-specific comparison/
  inspection consumers.
- `src/model/single_label.rs`: `signal_availability`, retained for T018's
  artifact-wide signal admission.
- `src/validation/wire.rs`: strict DTO schema-version, raw-input, and later
  policy fields. These are required deserialization contracts owned by T010;
  deserialization consumes them without a fake read.

No T021 policy behavior, T022 multi-label comparison, governance, planning,
artifact-index, session-note, acceptance, or Fizzy work occurred.
