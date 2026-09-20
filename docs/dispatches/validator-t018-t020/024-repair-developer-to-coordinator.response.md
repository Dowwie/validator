# T020 source-kind repair handoff

Status: the single authorized repair cycle is complete. This freezes a candidate
for the same verifier's focused recheck; it is not owner acceptance.

## Findings repaired

Before this repair, verdict023 reproduced an empty multi-label artifact with an
unused, well-formed `scored_choice` source accepted by real `check` (exit 0), and
the report schema accepted the same foreign source shape.

`validate_multi_label` now examines the complete admitted source map before row
alignment and returns `E_CONFIG` when any declared source is not a classifier.
The existing referenced scored-choice negative remains, and the added owning T018
regression uses an empty valid multi-label dataset, no predictions, and an unused
well-formed scored-choice source. It now returns `E_CONFIG` at checked admission.
Single-label source admission is unchanged.

The multi-label conditional in `report.schema.json` now constrains every source
definition through `classifierSource`; the base `source` definition remains shared
and retains both legal single-label kinds. `single_report_schema` proves a
single-label scored-choice source with its required `question_id` remains valid.
`multi_label_hard_oracles` mutates a valid published multi-label report to that
otherwise well-formed scored-choice source and proves schema rejection. No report
serialization or artifact format changed.

## Changed and fixed identity

| Artifact | Starting SHA-256 | Final SHA-256 |
|---|---|---|
| `src/validation.rs` | `c6e40115a863bba54fab8f84b32beba53b56df153de008bb3fbc63e6ea2886fa` | `891e6ee4e95b2be1931dd2006d02940308766b8a7b7d486b85f15faafbb69892` |
| `schemas/v2/report.schema.json` | `7215342a2958fd4d9e9735c2837130a02837d205db93b342b9f64d62a882fcf1` | `8f7c4fba2c12381e535888bbde5627a0723471c2018f3ab516a7ce661cb590d6` |
| `tests/conformance.rs` | `f184b28026250ec38da1d228d8bf563272142d21c5844ac2ba4e30e9ab8de291` | `aab6e45f86a34d01421e26195b9f06bbca195aefd92e3d565ce84d2fcb1c3f00` |
| `src/model/common.rs` | `dfe120adb33f7a773edd5a8c44c8331fb918e417aec52dfc3d1c0a8077138791` | unchanged |
| `tests/cli.rs` | `edaf69f7744fbc2df630e280234041e8f9bd0608ed1c2bdd719187090ca85275` | unchanged |
| `src/app.rs` | `3324dad26b9571529bdfb1a75b2bb17a9900bbb8f1a65ad822e7b389b76c6671` | unchanged |

Check and inspection schemas remain unchanged. No app/CLI, hard/marginal,
comparison/policy, tolerance, dependency, export, or staged-diagnostic change
occurred.

## Verification

| Command | Exit | Actual result |
|---|---:|---|
| `cargo test --locked --lib multi_label_checked_admission -- --nocapture` | 0 | 1 passed, 43 filtered |
| `cargo test --locked --lib cross_task_boundaries -- --nocapture` | 0 | 1 passed, 43 filtered |
| `cargo test --locked --test conformance multi_label_hard_oracles -- --nocapture` | 0 | 1 passed, 14 filtered |
| `cargo test --locked --test conformance single_report_schema -- --nocapture` | 0 | 1 passed, 14 filtered |
| `cargo test --locked --test cli shared_commands_multi_label -- --nocapture` | 0 | 1 passed, 6 filtered |
| `cargo fmt --all -- --check` | 0 | passed |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | exactly 20 staged `dead_code` diagnostics; no ordinary/new class |
| `cargo test --all-features --locked` | 0 | 44 library, 7 CLI, 15 conformance, 0 doc tests passed |
| `cargo build --release --locked --bin validator` | 0 | release binary built |
| `git diff --check` | 0 | passed |

The exact residual `dead_code` inventory is unchanged: task-definition wrappers
in `src/model.rs`; LabelSet/Episode/ObservationSet/EvaluationConfig/MetricResult
accessors in `src/model/common.rs`; `signal_availability` in
`src/model/single_label.rs`; and strict version/input/later-policy DTO fields in
`src/validation/wire.rs`. They remain the owner-authorized staged private-symbol
inventory for T018/T021/T022 and strict T010 DTO consumers. No suppression, fake
use, visibility expansion, or unrelated cleanup was added.
