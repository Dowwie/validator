# Ready

The single authorized T003 repair closes the task-minimum construction bypass and
removes the unused reexports. The repaired candidate is behaviorally ready for
T003. Clippy still exits 101 solely on the explicitly authorized incomplete-
consumer `dead_code` integration limit; this verdict does not call Clippy passed
or complete, and clean full Clippy remains required at T014 and T017.

## Frozen candidate identity

I recomputed the manifest, repair-response, and all implementation hashes before
and after the focused checks. Every value matches manifest 008 and remained stable:

| Artifact | Recomputed SHA-256 |
|---|---|
| `docs/dispatches/validator-t003/008-coordinator-repair-manifest.md` | `ceb00209087532787858fbb60045d5dbb2f9ae10c91ff60c5e5be4eee3c802c3` |
| `docs/dispatches/validator-t003/007-developer-to-coordinator.response.md` | `15dfc00c6439af28102278ea0df092dd342db9ddef92bcb3a6d4c6fad2430e3f` |
| `Cargo.toml` | `fab94b0c36151466e787b9c8e49d655961b95eea947b0e27fc82df4c905b3da0` |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |
| `src/lib.rs` | `4ff48cc3bfa59a05482c3e939251a88bce41dfea9f2c62e45a2cc0df95fad509` |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` |
| `src/main.rs` | `536e506bb90914c243a12b397b9a998f85ae2cbd9ba02dfd03a9e155ca5ca0f4` |
| `src/model.rs` | `3275697520566394a2a20d4011ad28b3912d9d4773a44203980127d3f2d4d4d1` |
| `src/model/common.rs` | `ee0b0169d88707663c7b1540d1406868ba61cb0db2fd4d36dd2728d3baf9cecf` |

Only `src/model.rs` and the owning test section of `src/model/common.rs` changed
from manifest 004. All other implementation hashes equal manifest 008, including
the owner-retained `src/lib.rs` bytes.

## Focused repair assessment

### Task-definition construction is sealed

- `src/model.rs:7-12` retains exactly the closed `SingleLabel` and `MultiLabel`
  variants, but their payloads are now `SingleLabelTask` and `MultiLabelTask`
  rather than a bare interchangeable vocabulary.
- `src/model.rs:39-55` gives `SingleLabelTask` private vocabulary storage. Its only
  exposed constructor calls `LabelVocabulary::for_single_label`, enforcing at
  least two labels.
- `src/model.rs:57-73` gives `MultiLabelTask` the same private-storage pattern and
  delegates only to the one-label-minimum constructor.
- `TaskDefinition::single_label` and `multi_label` delegate to the corresponding
  checked wrapper. `TaskDefinition::vocabulary` borrows the wrapped vocabulary,
  and `is_single_label` preserves closed variant inspection.
- A sibling crate module can name the wrapper types but cannot initialize their
  private fields, move a `MultiLabelTask` into the `SingleLabel` variant, or place
  a bare one-label `LabelVocabulary` in either variant. No `From`, `Default`, wire
  `Deserialize`, setter, mutable accessor, or alternate unchecked constructor was
  added.

The focused test at `src/model/common.rs:383-397` exercises both exposed
single-label routes: `SingleLabelTask::new(["only"])` and
`TaskDefinition::single_label(["only"])` both fail. It also proves
`MultiLabelTask::new(["only"])` succeeds. Static inspection confirms
`TaskDefinition::multi_label` directly delegates to that same checked constructor,
so one-label multi-label construction remains legal through the enum factory.

### Unused-reexport cleanup and scope

`src/model.rs:3` now retains only `LabelVocabulary`, which the two wrappers use.
The former unused group for ArtifactDigest, Episode, EpisodeId, LabelIndex,
LabelSet, Outcome, RunId, and SourceId is gone. Neither normal compilation nor
Clippy reports `unused_imports`.

Search and hash review found no fake caller, lint suppression, public SDK export,
generic framework, Prediction/Observation/T004 behavior, dependency change, or
unrelated mutation. `src/lib.rs` is unchanged as directed. No new production
unwrap/expect/panic, unsafe code, async runtime, or placeholder was introduced.

Unchanged identity, digest, vocabulary/index/set, Episode, Outcome, UUID-feature,
and corrected T004-ownership evidence from verifier response 005 remains applicable.
The two unchanged named tests were rerun as the repair dispatch required.

## Focused command evidence

All commands used the repository-pinned Rust/Cargo 1.98.1 toolchain on
`x86_64-apple-darwin`.

| Command | Exit | Actual result |
|---|---:|---|
| `cargo test --locked --lib model_boundary_visibility -- --nocapture` | 0 | Exactly 1 passed, 0 failed, 5 filtered; no warning. |
| `cargo test --locked --lib canonical_identifiers -- --nocapture` | 0 | Exactly 1 passed, 0 failed, 5 filtered; no warning. |
| `cargo test --locked --lib vocabulary_exact_identity -- --nocapture` | 0 | Exactly 1 passed, 0 failed, 5 filtered; no warning. |
| `cargo fmt --all -- --check` | 0 | No differences or output. |
| `cargo test --all-features --locked` | 0 | 6 library tests passed; binary and doc-test targets ran 0 and passed. Production compilation emitted only 24 dead-code warnings. |
| `cargo build --locked --bin validator` | 0 | Locked binary built; emitted the same 24 dead-code warnings only. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Failed on exactly 24 `dead_code` diagnostics; no `unused_imports` or other warning class. |
| `git diff --check` | 0 | No tracked whitespace error; rustfmt is the substantive format check in this unborn untracked repository. |

## Exact remaining Clippy limit

Clippy did **not** pass. Its 24 diagnostics are all `dead_code` caused by the
private model having no permitted production consumer yet:

1. `src/model.rs:7` — TaskDefinition.
2. `src/model.rs:16` — TaskDefinition method group.
3. `src/model.rs:41` — SingleLabelTask.
4. `src/model.rs:47` — SingleLabelTask method group.
5. `src/model.rs:59` — MultiLabelTask.
6. `src/model.rs:65` — MultiLabelTask method group.
7. `src/model/common.rs:30` — EpisodeId.
8. `src/model/common.rs:34` — RunId.
9. `src/model/common.rs:38` — parse_uuid.
10. `src/model/common.rs:53` — invalid_identifier.
11. `src/model/common.rs:59` — SourceId.
12. `src/model/common.rs:75` — SourceId::as_str.
13. `src/model/common.rs:82` — ArtifactDigest.
14. `src/model/common.rs:117` — hex_digit.
15. `src/model/common.rs:127` — LabelVocabulary.
16. `src/model/common.rs:132` — LabelVocabulary method group.
17. `src/model/common.rs:207` — LabelIndex.
18. `src/model/common.rs:214` — LabelSet.
19. `src/model/common.rs:221` — LabelSet method group.
20. `src/model/common.rs:234` — Episode.
21. `src/model/common.rs:242` — Episode method group.
22. `src/model/common.rs:264` — Outcome.
23. `src/model/common.rs:269` — OutcomeState.
24. `src/model/common.rs:276` — Outcome method group.

This exactly matches manifest 008 and the owner's incomplete-consumer disposition.
There is no unused-import diagnostic and no additional warning or failure. Real
production wiring must remove this limit; clean full Clippy remains mandatory at
T014 and T017.

## Execution state

No implementation, test, fixture, schema, specification, plan, acceptance artifact,
artifact index, or Fizzy record was edited. This response is my only write. No
verifier-owned command or background process remains active. Disk had about 15 GiB
available and `target/` was approximately 146 MiB, with no resource blocker.
