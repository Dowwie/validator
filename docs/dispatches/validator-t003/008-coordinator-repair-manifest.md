# T003 repaired candidate manifest 008

Frozen by the coordinator after reading repair response 007. Implementation writes
are paused. This is the sole-repair candidate for focused independent recheck; all
unchanged evidence from verifier response 005 remains applicable.

## Candidate identity

| Path | SHA-256 | Change from manifest 004 |
|---|---|---|
| `Cargo.toml` | `fab94b0c36151466e787b9c8e49d655961b95eea947b0e27fc82df4c905b3da0` | Unchanged |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` | Unchanged |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` | Unchanged |
| `src/lib.rs` | `4ff48cc3bfa59a05482c3e939251a88bce41dfea9f2c62e45a2cc0df95fad509` | Unchanged per owner decision |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` | Unchanged |
| `src/model.rs` | `3275697520566394a2a20d4011ad28b3912d9d4773a44203980127d3f2d4d4d1` | Sealed task wrappers; unused reexports removed |
| `src/model/common.rs` | `ee0b0169d88707663c7b1540d1406868ba61cb0db2fd4d36dd2728d3baf9cecf` | Focused wrapper-boundary regression assertions |
| `docs/dispatches/validator-t003/007-developer-to-coordinator.response.md` | `15dfc00c6439af28102278ea0df092dd342db9ddef92bcb3a6d4c6fad2430e3f` | Repair evidence |

## Repair and evidence

`TaskDefinition::SingleLabel` now carries `SingleLabelTask` and
`TaskDefinition::MultiLabel` carries `MultiLabelTask`. Each wrapper's vocabulary
field is private and its constructor applies the correct label-count minimum.
Checked TaskDefinition constructors delegate to those wrappers. The current
production import/reexport list retains only `LabelVocabulary`, which the wrappers
use; the old unused-import group is absent.

Developer checks pass for the three named tests, format, full locked tests, locked
binary build, and unchanged UUID feature tree. The coordinator independently
reproduced full Clippy exit 101 with exactly twenty-four `dead_code` diagnostics:
TaskDefinition and methods; both checked wrapper types and methods; the UUID/source/
digest helpers and identities; vocabulary/index/set types and methods; Episode;
Outcome/OutcomeState; and their methods. There is no `unused_imports` or other
warning class. These remain the owner-authorized incomplete-consumer integration
limit; clean full Clippy is still required at T014 and T017.
