# Revise

The frozen corrected-T003 candidate passes its named behavioral tests and most
criteria, but it does not yet enforce the task-definition construction boundary.
It also contains one unnecessary parent-module visibility change that the verifier
dispatch explicitly requires correcting. The known incomplete-consumer Clippy
failure is separately classified below and is not presented as passed or waived.

## Candidate identity

I recomputed the manifest, evidence, and candidate hashes before review and after
all commands. Every value remained unchanged:

| Artifact | Recomputed SHA-256 |
|---|---|
| `docs/dispatches/validator-t003/004-coordinator-candidate-manifest.md` | `6d6113cfc205806781f738e65770b781cde12f9a2fba2c84c013b39333f6e27b` |
| `docs/dispatches/validator-t003/002-developer-to-coordinator.response.md` | `3e1e5a546899d1cbdad7a3cb6cbe376c0a83e9e82d5a5ab72fafaee5f0416e01` |
| `Cargo.toml` | `fab94b0c36151466e787b9c8e49d655961b95eea947b0e27fc82df4c905b3da0` |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |
| `src/lib.rs` | `4ff48cc3bfa59a05482c3e939251a88bce41dfea9f2c62e45a2cc0df95fad509` |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` |
| `src/model.rs` | `05abe8af935ced43e50b14bc07812569a3984aaff9b9c6ba2c1c28896d88ad0f` |
| `src/model/common.rs` | `fb977bb4c1c9c0eb5508ad846fbc01010bb332a2e57451d3096c31c5cf62b88b` |

## Required findings

### 1. `TaskDefinition` can bypass the task-specific vocabulary minimum

- **Requirement:** T003 outcome and interfaces; T003 acceptance criterion that
  task-specific vocabulary minima are enforced with no construction bypass;
  `validator-data-model.md` Shared records and typed task data and Decode/validate/
  align/score checked-construction boundary.
- **Location:** `src/model.rs:10-15` exposes the enum variants with a bare
  `LabelVocabulary` payload. `src/model/common.rs:132-137` exposes both the
  single-label and multi-label vocabulary constructors to crate callers.
- **Reproduction:** any later crate-internal module can execute the equivalent of:

  ```rust
  let vocabulary = LabelVocabulary::for_multi_label(vec!["only".to_owned()])?;
  let task = TaskDefinition::SingleLabel(vocabulary);
  assert_eq!(task.vocabulary().len(), 1);
  ```

  The one-label vocabulary is valid for multi-label construction, and the public
  enum variant then reclassifies it as `SingleLabel` without the required minimum
  of two. The current `model_boundary_visibility` test uses only the checked
  constructor and therefore does not expose this direct-variant path.
- **Consequence:** validation and later internal consumers can manufacture an
  invalid object that appears to be a checked `TaskDefinition`, defeating the
  core T003 boundary before wire decoding exists.
- **Smallest correction:** make each public enum variant carry a task-specific
  checked wrapper whose fields cannot be constructed directly, and have
  `TaskDefinition::single_label`/`multi_label` create those wrappers after applying
  the appropriate minimum. Preserve the required closed enum and borrowed
  vocabulary access. Add focused regression evidence that no one-label vocabulary
  can become `SingleLabel` through the exposed construction surface.

### 2. The `src/lib.rs` visibility change is unnecessary routing

- **Requirement:** corrected execution contract's narrow parent-declaration/
  reexport rule, the corrected developer scope, and `validator-v1.md` Package and
  source layout instruction to keep modules private unless wider visibility is
  required. The verifier dispatch specifically requires Revise when this change
  is unnecessary.
- **Location:** `src/lib.rs:10`, changed from the accepted `mod model;` to
  `pub(crate) mod model;`.
- **Reproduction:** the parent declaration already existed in accepted T001.
  Rust crate-root private items are accessible to crate-root descendants, so
  later library sibling modules can use `crate::model` with `mod model;`. T003
  adds no separate crate or other caller needing `pub(crate)` on the declaration;
  Cargo metadata still reports the same single library and binary targets.
- **Consequence:** the candidate changes a shared parent file and explicitly
  broadens its declared visibility without a required routing effect, contrary to
  the surgical integration allowance. It does not expose an external SDK, but it
  is still an unauthorized/unnecessary scope change under this dispatch.
- **Smallest correction:** restore `src/lib.rs:10` to `mod model;`. No placeholder
  caller, public export, or lint suppression is needed.

## Other criterion evidence

- **Canonical UUID identities:** `src/model/common.rs:7-55` gives `EpisodeId` and
  `RunId` distinct private `uuid::Uuid` fields and one checked parser. It rejects
  nil, max, non-RFC variant, version zero, uppercase, brace, URN, simple, and
  malformed spellings by semantic checks plus exact canonical string comparison.
  The named test accepts canonical versions 1 through 8. No generation API or UUID
  feature is enabled.
- **Source and digest identities:** `src/model/common.rs:57-123` keeps SourceId and
  ArtifactDigest distinct with private storage. Source IDs reject blank/all-
  whitespace input while preserving nonblank text exactly. Digests require exactly
  64 lowercase hexadecimal characters and round-trip canonically.
- **Vocabulary/index/set behavior:** apart from Finding 1's TaskDefinition
  reclassification path, `src/model/common.rs:125-230` preserves ordered exact
  case/whitespace/Unicode labels, rejects blank/duplicates, binds indexes to their
  Arc-owned vocabulary, rejects duplicate/unknown submitted set labels, and stores
  label sets in vocabulary order. LabelIndex has no serialization or freely
  constructible fields.
- **Episode and outcome:** `src/model/common.rs:232-311` privately owns one UUID,
  one opaque `serde_json::Value`, and one target; accessors borrow the input and
  target. Outcome privately distinguishes answered from abstained and rejects an
  all-whitespace reason. No checked model type derives wire `Deserialize`, exposes
  mutable fields/setters, or adds a phantom-state/plugin framework.
- **Corrected ownership:** searches found no T003 `Prediction`, `ObservationSet`,
  source definition, or preparation implementation. The only `Observation` text
  is unchanged T001 diagnostic-code material. Those records remain with T004.
- **T001 and dependency integrity:** `rust-toolchain.toml` and `src/error.rs` retain
  their accepted hashes. The constants in `src/model.rs:41-57` retain their exact
  accepted values, and all three T001 tests pass in the full suite. Cargo metadata
  reports one package, one library, and one binary. `uuid 1.26.1` is locked with
  `default-features = false`; `cargo tree --locked -e features -i uuid` shows no
  UUID optional feature, generation, RNG, time, serde, or macro dependency.
- Static scans found no production unwrap/expect/panic/todo/unimplemented, unsafe,
  async runtime, bypass attribute, fake runtime consumer, generic task/plugin
  registry, placeholder, or test weakening. All unwrap/expect occurrences remain
  inside `#[cfg(test)]` modules.

## Commands and test counts

All commands used pinned Rust/Cargo 1.98.1 on `x86_64-apple-darwin`.

| Command | Exit | Actual result |
|---|---:|---|
| `cargo test --locked --lib -- --list` | 0 | Listed 6 tests and 0 benchmarks, including each exact T003 name once; emitted the one reexport `unused_imports` warning. |
| `cargo test --locked --lib canonical_identifiers -- --nocapture` | 0 | 1 passed, 0 failed, 5 filtered; same one warning. |
| `cargo test --locked --lib vocabulary_exact_identity -- --nocapture` | 0 | 1 passed, 0 failed, 5 filtered; same one warning. |
| `cargo test --locked --lib model_boundary_visibility -- --nocapture` | 0 | 1 passed, 0 failed, 5 filtered; same one warning. |
| `cargo fmt --all -- --check` | 0 | No differences or output. |
| `cargo test --all-features --locked` | 0 | 6 library tests passed; binary and doc tests ran 0 and passed. Non-test compilation emitted 21 warnings. |
| `cargo build --locked --bin validator` | 0 | Binary built; emitted the same 21 warnings. |
| `cargo tree --locked -e features -i uuid` | 0 | `uuid v1.26.1` appears only as Validator's direct dependency; no optional UUID features. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Failed with exactly the one unused-import group and twenty incomplete-consumer dead-code diagnostics described below. |
| `cargo metadata --locked --no-deps --format-version 1` | 0 | Confirmed one package, one library, one binary, and UUID with no enabled feature. |
| `git diff --check` | 0 | No tracked whitespace error; candidate files are untracked in the unborn repository, so rustfmt is the substantive format check. |

## Clippy classification and import/reexport decision

Clippy did **not** pass. It exited 101. Its complete diagnostic set matches the
manifest; there was no additional warning:

- One `unused_imports` group at `src/model.rs:4-5` for `ArtifactDigest`, `Episode`,
  `EpisodeId`, `LabelIndex`, `LabelSet`, `Outcome`, `RunId`, and `SourceId`.
- Twenty `dead_code` diagnostics: `TaskDefinition` and its method group;
  EpisodeId; RunId; `parse_uuid`; `invalid_identifier`; SourceId and `as_str`;
  ArtifactDigest; `hex_digit`; LabelVocabulary and its method group; LabelIndex;
  LabelSet and its method group; Episode and its method group; Outcome;
  OutcomeState; and Outcome's method group.

The twenty dead-code diagnostics are exactly the incomplete-consumer condition
authorized by the owner: the implemented private model has no permitted production
caller before later admission/application wiring. They remain unresolved and must
be removed by real production consumers, with clean full Clippy at T014 and T017.

The one unused-import group is also an unavoidable temporary consequence of the
T003 contract rather than an independent defect. T003's artifact contract assigns
`src/model.rs` the common-type reexports so later internal model consumers use one
parent route. `LabelVocabulary` escapes the warning only because TaskDefinition
already consumes it; the other required reexports await their real consumers.
Removing those reexports solely to silence this partial-build warning would defer
required T003 routing. No suppression, fake caller, or external public export is
appropriate. This classification does not excuse Finding 2: the `src/lib.rs`
`pub(crate)` declaration change supplies no such routing need and should be reverted.

## Limits and execution state

The eventual corrected T003 can still carry the explicitly authorized incomplete-
consumer warning limit; this verdict does not require T004/T014 behavior or demand
a clean intermediate Clippy result. It does require the two T003-owned corrections
above and a newly frozen candidate. Admission, T014, T017, and product completion
remain unverified.

No implementation, test, fixture, schema, specification, plan, acceptance artifact,
artifact index, or Fizzy record was edited. This response is my only write. No
verifier-owned command or background process remains active; the final process
snapshot showed no Cargo, rustc, or repository process in flight. Disk had about
15 GiB available and `target/` was approximately 143 MiB, with no resource blocker.
