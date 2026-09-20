# Validator T003 developer dispatch 001

Role/model: existing developer, `gpt-5.6-terra`, reasoning `high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Coordinator: `/root/coordinator`, Sol high. Required full response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t003/001-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator’s verified single-label backbone](http://localhost:3006/1/cards/183).

## Outcome and authority

Implement T003 only: canonical identities, closed task definition, exact owned
label vocabulary/indexes, and the shared record foundations owned by this task can
only be constructed through checked invariants. You remain the sole implementation
and test writer. Do not delegate or spawn agents. Do not implement T002, T004, or
later semantic validation, observation/source contracts, alignment, evaluation,
filesystem, schema, CLI, or multi-label evaluator behavior.

T001 is owner-accepted at frozen manifest SHA-256
`a12e570a1299b86d6793b177301bd287979ae824e84e8b5e739796ee03e64174`.
Read the exact T003 contract before editing:

- `docs/plans/validator/tasks/T003.json` and
  `docs/plans/validator/execution-contract.md`.
- `docs/specs/validator-v1.md`, especially Canonical golden dataset and Rust
  project organization.
- `docs/specs/validator-data-model.md`, especially Design boundary, Shared records
  and typed task data, Decode/validate/align/score, and Source organization.
- `docs/dev-team/validator-build/charter.md` and repository `AGENTS.md`.
- Apply the Rust best-practices guidance already read for the prior assignment.
- Requirement IDs named by T003 in `docs/plans/validator/coverage.json`. Implement
  T003's owning portion only; coverage shared with later tasks is not permission to
  implement their behavior early.

## Accepted dependency and baseline

The accepted T001 inputs are unchanged:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `ef2bb8fa026d665fe09f8670e9b15dd27518dffd1f553124857c293cff05b5c2` |
| `Cargo.lock` | `157fcfe04a5fdff11f1e1770f0e3228a822ff32c008215c987d89c63a2678fa9` |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |
| `src/lib.rs` | `cad8e175086607a8f194176a738e6a10e1b5fc7372f38679b6952246e23e22df` |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` |
| `src/model.rs` | `a16faa58a7d727ea3894509529564b3906dffa76f32f534d6966413e8dae3cb5` |

The current worktree remains unborn/untracked. Preserve unrelated files and all
accepted T001 bytes outside the exact T003 write scope below. Do not stage, commit,
clean, reset, or modify `.zvec-grep`.

## Exact write scope and interfaces

You may create or modify only:

- `src/model/common.rs`: checked identity newtypes, vocabulary/index ownership,
  and the shared episode/outcome/prediction record foundation explicitly owned by
  T003.
- `src/model.rs`: closed task declaration, module declaration/re-exports, while
  preserving accepted T001 constants exactly.
- `/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t003/001-developer-to-coordinator.response.md`.

Do not edit `Cargo.toml`, `Cargo.lock`, `src/lib.rs`, error handling, specs, plans,
artifact index, Fizzy, session notes, schemas, fixtures, or other source/test files.
Use the standard library and existing accepted dependencies only.

Required interfaces and boundaries:

- `EpisodeId`, `RunId`, `SourceId`, and `ArtifactDigest` are distinct types, not
  interchangeable strings. UUID text is canonical lowercase hyphenated, accepts
  existing standard versions, and rejects nil/max/noncanonical values. Digests are
  exact lowercase 64-character SHA-256 hex. Source IDs are nonblank and exact.
- `TaskDefinition` is a closed single-label/multi-label enum with a checked
  `LabelVocabulary`; no registry, trait object, plugin abstraction, or mixed task.
- `LabelVocabulary` owns ordered exact strings and exact lookup. Reject blank and
  duplicate labels; require at least two for single-label and one for multi-label.
  Preserve case, whitespace inside nonblank strings, and Unicode without trimming
  or normalization. `LabelIndex` remains vocabulary-bound/internal and is never a
  durable serialized label identity.
- `Episode<Target>` owns one required opaque JSON input and one concrete target.
  `Outcome<Target>` distinguishes answered from explicit abstention with optional
  nonblank reason.
- `Prediction<Output>` must ultimately own independent typed observations under
  the approved model. T004 owns `Observation`, `ObservationSet`, source definitions,
  and their semantic checks. Do not create a raw-value observation bypass,
  placeholder observation type, or implement T004 early. If T003's exact
  `Prediction<Output>` interface cannot be completed inside these boundaries,
  implement the dependency-ready T003 invariants that are unambiguous, stop before
  a workaround, and report the precise contract/write-scope conflict for owner
  resolution.
- All fields stay private. Use checked constructors or `TryFrom`; expose no mutable
  construction bypass. No wire `Deserialize` on validated structures, no setters,
  and no generic task/plugin trait.

## Acceptance and checks

Implement local tests named exactly:

1. `canonical_identifiers`: sentinels, malformed/noncanonical UUID/digest values,
   and blank source IDs fail; supported canonical standard UUID versions and valid
   lowercase digests pass as distinct types.
2. `vocabulary_exact_identity`: task-specific minimums, blank/duplicates, exact
   case/whitespace/Unicode identity, stable order, and lookup behavior.
3. `model_boundary_visibility`: checked constructors and private ownership enforce
   model invariants without a mutable/generic bypass.

Run and record exact exits and nonzero test counts for:

```sh
cargo test --locked --lib -- --list
cargo test --locked --lib canonical_identifiers -- --nocapture
cargo test --locked --lib vocabulary_exact_identity -- --nocapture
cargo test --locked --lib model_boundary_visibility -- --nocapture
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --locked --bin validator
```

Early model types can be intentionally unwired outside owning-module tests. Do not
add exports, placeholder runtime uses, or lint bypasses only to silence a warning.
Report any incomplete-module warning honestly; resolve actual `-D warnings`
failures with the smallest behavior-preserving structure inside the allowed scope.

## Handoff

Before returning, save the complete response at the required path. Include:

- Delivered behavior and every changed/created path with SHA-256.
- Every acceptance criterion and T003-owned requirement group mapped to exact
  implementation and test evidence, with later-task ownership stated concisely.
- Exact commands, exits, named test counts, compiler/toolchain, warnings, failures,
  and active processes.
- Dependency-input hashes and the actual candidate hashes. Reference the prior
  accepted authority/skill evidence instead of repeating an unchanged catalogue.
- Any precise contract/write-scope blocker, unproven limit, or resource surprise.

Do not claim independent verification or acceptance. Return only after the saved
response is complete, then send its absolute path to the coordinator.
