# T004-T006 checked-admission candidate manifest 002

Frozen after the ordered T004 -> T005 -> T006 implementation. Implementation
writes are paused while the independent verifier reviews this exact candidate.
The candidate contains one coherent admission boundary; none of the three tasks is
independently accepted by this manifest.

## Frozen candidate hashes

| Path | SHA-256 | Candidate role |
|---|---|---|
| `Cargo.toml` | `8c758ad6246244317ac02156077b417eab3cd49994daf2597c65ae86bb24629c` | Accepted dependency baseline, unchanged |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` | Accepted lock baseline, unchanged |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` | Accepted toolchain baseline, unchanged |
| `src/lib.rs` | `b60b0bf5d4fed5fe6045e7c1039f3c21e315dc46d9bc08159d56d2aa4ab35162` | Accepted crate wiring, unchanged |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` | Accepted diagnostics, unchanged |
| `src/main.rs` | `536e506bb90914c243a12b397b9a998f85ae2cbd9ba02dfd03a9e155ca5ca0f4` | Accepted binary scaffold, unchanged |
| `src/model.rs` | `ceffb61ab5e523fa4bf3161d5db5a6fcd1276137d951982da4382edb79c58c66` | Single-label module wiring |
| `src/model/common.rs` | `6dd18b99bad4be57dbb4c7a1add44b0aa711a37234a0b1c501d207f1dcf79af5` | T004 checked source/observation records and T006 population |
| `src/model/single_label.rs` | `35fe11ada58053273708751631c079984f86e6e6fad3f7d24dcae502c4b92fe5` | T005 checked signals/output and T006 closed evaluation |
| `src/validation.rs` | `1a085d123a35fd270fbd30359bcde449e0ba645f11b59d9eebe63547dcb0f3f5` | Ordered T004-T006 admission and owning tests |
| `src/validation/wire.rs` | `e81c528e0659e1fb4b2364be82639bbb27175947b042145a0f314ec00e827d17` | Owner-accepted T002 wire boundary, unchanged |
| `docs/dispatches/validator-t004-t006/001-developer-to-coordinator.response.md` | `ecd49c30a119c44ac09b46873f35f8000cc9ee69e2bc93c28868563205d32fe0` | Developer evidence |

The accepted T002 repair manifest remains bound by SHA-256
`3e0c155c1a64e6ec75d8bf518400fcc3c12d283ec3774de705833eb912843e91`.

## Candidate evidence and limits

The developer reports one passing, nonzero test for each of the eight required
filters: `observation_contracts`, `source_preparation_bindings`,
`categorical_admission`, `scored_choice_ties`,
`artifact_signal_completeness`, `population_alignment`,
`validate_before_selection`, and `dataset_digest_binding`. The complete library
inventory contains 19 tests. `cargo fmt --all -- --check`, full all-feature locked
tests, the locked binary build, and `git diff --check` pass.

Clippy does not pass. It exits 101 only on incomplete-consumer `dead_code`: 97
library diagnostics and 24 library-test diagnostics across the private model,
validation, wire, and accepted task-wrapper code. The developer reports no unused
import or other warning class, suppression, fake consumer, placeholder, or public
SDK expansion. The verifier must independently confirm this classification. Clean
full Clippy remains mandatory at T014 and T017.

No T007-or-later metric, schema, evidence-loading, publication, report, or CLI
behavior is part of this candidate. The verifier is limited to the exact T004,
T005, and T006 contracts and their interaction at the checked-admission boundary.
