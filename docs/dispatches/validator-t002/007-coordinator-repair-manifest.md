# T002 repaired candidate manifest 007

Frozen after the single bounded repair. Implementation writes are paused. Reuse
all unchanged evidence from verifier response 005; focused recheck covers only the
added observation wire shapes, their tests, preserved hashes, and updated dead-code
classification.

## Repaired candidate hashes

| Path | SHA-256 | Change from manifest 004 |
|---|---|---|
| `Cargo.toml` | `8c758ad6246244317ac02156077b417eab3cd49994daf2597c65ae86bb24629c` | Unchanged |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` | Unchanged |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` | Unchanged |
| `src/lib.rs` | `b60b0bf5d4fed5fe6045e7c1039f3c21e315dc46d9bc08159d56d2aa4ab35162` | Unchanged |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` | Unchanged |
| `src/model.rs` | `3275697520566394a2a20d4011ad28b3912d9d4773a44203980127d3f2d4d4d1` | Unchanged |
| `src/model/common.rs` | `ee0b0169d88707663c7b1540d1406868ba61cb0db2fd4d36dd2728d3baf9cecf` | Unchanged |
| `src/validation.rs` | `6c6c71080372b1d5b0a1bf707b88e1db694d88a142a3f2d94b8a467ff0020f72` | Positive/all-five and negative observation-shape tests |
| `src/validation/wire.rs` | `e81c528e0659e1fb4b2364be82639bbb27175947b042145a0f314ec00e827d17` | Optional definition/row maps and five strict tagged variants |
| `docs/dispatches/validator-t002/006-developer-to-coordinator.response.md` | `88cea4d46bc9cf14ed19766d0077d55ae03ac7f58b9a13ea5445b138404255ee` | Repair evidence |

`Source.observation_definitions` and `Prediction.observations` are optional exact
string-keyed maps. Private DTOs cover definition `kind`/`description`/optional
`question_id` and scalar, Bernoulli, reported-confidence, categorical, and label-
marginal row values. No semantic range/name/binding/sum/completeness/promotion
logic was added.

Developer evidence: all three T002 named tests pass one filtered case each; format,
full locked tests, and locked build pass. Unknown observation kind and extra field
fail. Clippy exits 101 only on `dead_code`: 24 accepted T003 production groups,
31 T002 production groups, and 21 lib-test unread-field groups. No unused import,
other lint class, suppression, or fake consumer appears. Clean Clippy remains a
T014/T017 gate.
