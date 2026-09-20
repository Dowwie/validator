# T003 repair 007 developer handoff

## Delivered repair

The frozen T003 candidate's task-minimum bypass is repaired. `TaskDefinition`
remains the closed `SingleLabel`/`MultiLabel` enum, but each variant now carries a
task-specific wrapper:

- `SingleLabelTask` has a private `LabelVocabulary` field and constructs only
  through `SingleLabelTask::new`, which applies the two-label minimum.
- `MultiLabelTask` has the same private storage pattern and applies the one-label
  minimum.
- `TaskDefinition::single_label` and `multi_label` delegate to those wrappers;
  their borrowed vocabulary access is preserved. A sibling crate module cannot
  construct either wrapper's storage, so a `LabelVocabulary::for_multi_label` value
  cannot be placed in `TaskDefinition::SingleLabel`.

`model_boundary_visibility` now proves the exposed wrapper and enum construction
routes reject a one-label single-label task while allowing a one-label multi-label
task. The test is in `src/model/common.rs`, the owning-module test location already
used by T003; that minimal file change was required to add the regression evidence.

The unused reexports for `ArtifactDigest`, `Episode`, `EpisodeId`, `LabelIndex`,
`LabelSet`, `Outcome`, `RunId`, and `SourceId` were removed from `src/model.rs`.
The only retained reexport is `LabelVocabulary`, which current T003 production code
uses for the task wrappers. No fake consumer, lint suppression, public SDK change,
module-visibility redesign, or change to `src/lib.rs` was made.

## Candidate identity

| Path | SHA-256 | State |
|---|---|---|
| `Cargo.toml` | `fab94b0c36151466e787b9c8e49d655961b95eea947b0e27fc82df4c905b3da0` | Unchanged from frozen candidate. |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` | Unchanged from frozen candidate. |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` | Unchanged accepted input. |
| `src/lib.rs` | `4ff48cc3bfa59a05482c3e939251a88bce41dfea9f2c62e45a2cc0df95fad509` | Unchanged as directed. |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` | Unchanged accepted input. |
| `src/model.rs` | `3275697520566394a2a20d4011ad28b3912d9d4773a44203980127d3f2d4d4d1` | Task-specific wrappers and unused-reexport cleanup. |
| `src/model/common.rs` | `ee0b0169d88707663c7b1540d1406868ba61cb0db2fd4d36dd2728d3baf9cecf` | Focused construction-boundary regression test only. |

The response's own final hash is for the coordinator's freeze.

## Criterion and regression evidence

| Criterion | Implementation and evidence |
|---|---|
| Checked task-specific vocabulary minima with no construction bypass | `src/model.rs` wraps the enum payloads in `SingleLabelTask` and `MultiLabelTask`, each with a private vocabulary field and a checked constructor. The focused `model_boundary_visibility` test passed and checks `SingleLabelTask::new(["only"])` and `TaskDefinition::single_label(["only"])` fail, while `MultiLabelTask::new(["only"])` succeeds. |
| Closed task definition and borrowed vocabulary access | `TaskDefinition` remains the two-variant enum, with unchanged `single_label`, `multi_label`, `vocabulary`, and `is_single_label` behavior. |
| Exact vocabulary/index, identity, episode, outcome, and corrected T004 ownership | Unchanged candidate code and verifier evidence were reused. `canonical_identifiers` and `vocabulary_exact_identity` each reran and passed. No prediction, observation, source, preparation, validation, scoring, filesystem, or CLI behavior was added. |
| Remove current unused imports without a fake consumer | `src/model.rs` retains only `LabelVocabulary`, used directly by both wrappers. The old unused-import diagnostic no longer appears in Clippy. |

## Checks

Pinned toolchain: Rust/Cargo 1.98.1 on `x86_64-apple-darwin` via the repository's
`rust-toolchain.toml`.

| Command | Exit | Result |
|---|---:|---|
| `cargo test --locked --lib model_boundary_visibility -- --nocapture` | 0 | One matching test passed; five filtered. |
| `cargo test --locked --lib canonical_identifiers -- --nocapture` | 0 | One matching test passed; five filtered. |
| `cargo test --locked --lib vocabulary_exact_identity -- --nocapture` | 0 | One matching test passed; five filtered. |
| `cargo fmt --all -- --check` | 0 | No formatting differences. |
| `cargo test --all-features --locked` | 0 | Six library tests passed; binary and doc-test targets ran zero tests and passed. |
| `cargo build --locked --bin validator` | 0 | Binary built. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Expected incomplete-consumer dead-code failure only; no unused-import or new warning class. |
| `cargo tree --locked -e features -i uuid` | 0 | `uuid v1.26.1` remains Validator's direct dependency with no optional UUID features. |
| `git diff --check` | 0 | No tracked whitespace error; the repository remains unborn with candidate files untracked. |

## Remaining Clippy limit

Clippy did not pass and is not claimed as passing. It exits 101 with exactly 24
authorized incomplete-consumer `dead_code` diagnostics:

1. `TaskDefinition` and its method group.
2. `SingleLabelTask` and its method group.
3. `MultiLabelTask` and its method group.
4. `EpisodeId`, `RunId`, `parse_uuid`, and `invalid_identifier`.
5. `SourceId` and `SourceId::as_str`; `ArtifactDigest` and `hex_digit`.
6. `LabelVocabulary` and its constructor/accessor group; `LabelIndex`; `LabelSet`
   and its methods.
7. `Episode` and its constructor/accessor group; `Outcome`, `OutcomeState`, and
   Outcome's constructor/accessor group.

These types have no permitted production consumer until later admission/application
tasks wire them. This is the exact interim warning limit authorized by the owner.
The former `src/model.rs:4-5` `unused_imports` diagnostic is absent. No other
warning or failure appeared.

## Reused authority and execution state

The frozen input manifest and review records remain intact:

| Artifact | SHA-256 |
|---|---|
| `004-coordinator-candidate-manifest.md` | `6d6113cfc205806781f738e65770b781cde12f9a2fba2c84c013b39333f6e27b` |
| `005-verifier-to-coordinator.response.md` | `13d67ea0eb2a105c95988a82815960cb7fc95801ae265d6e227d2f3209154954` |
| `006-owner-to-coordinator.prompt.md` | `d97918fbd5c878dd80e9d849a79b44cd9b1c897084c417341b12815edc5c973f` |
| `007-coordinator-to-developer.prompt.md` | `837612b7e5864e93aff590cb5b8d3785bae978ebb27fb29f8ef068de9ab2dffa` |

No Cargo or Rust compiler process remained after the checks. No resource surprise,
dependency update, or new limitation occurred. This handoff does not claim an
independent Ready verdict or owner acceptance.
