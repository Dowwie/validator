# Validator T003 corrected developer dispatch 002

Role/model: existing developer, `gpt-5.6-terra`, reasoning `high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Coordinator: `/root/coordinator`, Sol high. Required full response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t003/002-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator’s verified single-label backbone](http://localhost:3006/1/cards/183).

This prompt fully supersedes
`docs/dispatches/validator-t003/001-coordinator-to-developer.prompt.md`. The owner
resolved its two-file/std-only and Prediction/ObservationSet sequencing conflicts
in `docs/dispatches/validator-build/003-owner-to-coordinator.prompt.md`. Those were
plan gaps, not developer defects. Do not continue under the earlier restrictions.

## Outcome and corrected scope

Implement corrected T003 only: canonical identity newtypes, closed task definition,
exact vocabulary-owned label indexes, `Episode<Target>`, and `Outcome<Target>` can
only be constructed through checked invariants. You remain the sole implementation
and test writer. Do not delegate or spawn agents.

`Prediction<Output>` is now assigned to T004 with its real typed `ObservationSet`.
Do not create `Prediction`, `Observation`, `ObservationSet`, a raw-value observation
bypass, a placeholder observation type, or other T004 source/preparation semantics
in T003. Corrected T003 acceptance does not require the reassigned prediction field.

T001 is accepted at frozen manifest SHA-256
`a12e570a1299b86d6793b177301bd287979ae824e84e8b5e739796ee03e64174`.
Read and apply:

- `docs/dispatches/validator-build/003-owner-to-coordinator.prompt.md` as the
  governing correction.
- `docs/plans/validator/tasks/T003.json` and
  `docs/plans/validator/execution-contract.md`, with the owner correction overriding
  the stale Prediction and two-file/dependency portions until plan metadata lands.
- `docs/specs/validator-v1.md`: Canonical golden dataset and Rust organization.
- `docs/specs/validator-data-model.md`: Design boundary, Shared records and typed
  task data, Decode/validate/align/score, and Source organization. Apply all T003
  identity/task/vocabulary/Episode/Outcome requirements; treat Prediction plus
  observations as T004 under the owner decision.
- `docs/dev-team/validator-build/charter.md`, repository `AGENTS.md`, and the Rust
  best-practices guidance already read.

## Baseline and permitted writes

No T003 implementation bytes were written before the coordinator interrupted the
superseded turn. The accepted dependency hashes remain:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `ef2bb8fa026d665fe09f8670e9b15dd27518dffd1f553124857c293cff05b5c2` |
| `Cargo.lock` | `157fcfe04a5fdff11f1e1770f0e3228a822ff32c008215c987d89c63a2678fa9` |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |
| `src/lib.rs` | `cad8e175086607a8f194176a738e6a10e1b5fc7372f38679b6952246e23e22df` |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` |
| `src/model.rs` | `a16faa58a7d727ea3894509529564b3906dffa76f32f534d6966413e8dae3cb5` |

You may create or modify only:

- `Cargo.toml` and `Cargo.lock`: add the established `uuid` crate with only the
  features actually needed for checked parsing/inspection in this task. Prefer the
  real primitive to a custom UUID parser. Add no generation, serde, RNG, time,
  macro, async, framework, or future-facing feature unless current T003 behavior
  demonstrably requires it.
- `src/model/common.rs`: checked identities, vocabulary/indexes, Episode and Outcome.
- `src/model.rs`: common-module declaration and closed task declaration/re-exports,
  preserving every accepted T001 constant exactly.
- `/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t003/002-developer-to-coordinator.response.md`.

Do not edit `src/lib.rs`, `src/error.rs`, other source/test files, specs, plans,
artifact index, Fizzy, session notes, schemas, or fixtures. Preserve unrelated and
accepted files; do not stage, commit, clean, reset, or modify `.zvec-grep`.

## Required interfaces and invariants

- `EpisodeId` and `RunId` wrap the standard UUID primitive as distinct types.
  Accept canonical lowercase hyphenated standard UUID versions; reject nil, max,
  uppercase, brace/URN/simple forms, and malformed text. Do not generate IDs.
- `SourceId` is a distinct exact nonblank identifier type. Do not trim or normalize.
- `ArtifactDigest` is a distinct type for exact lowercase 64-character SHA-256 hex;
  reject sentinels/noncanonical/malformed text as specified.
- `TaskDefinition` is a closed single-label/multi-label enum, each owning a checked
  `LabelVocabulary`; no registry, generic task trait, trait object, plugin, or mixed
  task representation.
- `LabelVocabulary` owns ordered unique nonblank labels and exact lookup. Require at
  least two labels for single-label and one for multi-label. Preserve exact case,
  surrounding whitespace in a nonblank string, and Unicode; do not trim/normalize.
  `LabelIndex` remains internal/vocabulary-bound and is never a durable serialized
  label identity.
- `Episode<Target>` privately owns an episode ID, the required opaque JSON input
  once (including explicit null and arbitrary JSON numbers), and one concrete
  target. Do not duplicate opaque input into helper rows.
- `Outcome<Target>` is answered or explicit abstention with an optional nonblank
  reason. Expected targets do not use this wrapper.
- All validated fields remain private. Use checked constructors or `TryFrom` and
  immutable accessors only where needed. Do not derive wire `Deserialize` on these
  structures or expose setters/mutable construction bypasses.

Map invalid identity/label/model construction to the accepted typed diagnostic
codes without interpolating raw opaque payloads or credentials. Keep choices
ordinary and minimal; no phantom-state framework is required.

## Acceptance and checks

Implement local tests named exactly:

1. `canonical_identifiers`: nil/max, malformed/noncanonical UUID and digest, and
   blank SourceId fail; supported canonical standard UUID versions and lowercase
   digests pass as distinct types.
2. `vocabulary_exact_identity`: task-specific minimums, blank/duplicates, exact
   case/whitespace/Unicode identity, stable order, and lookup behavior.
3. `model_boundary_visibility`: checked constructors and private ownership enforce
   TaskDefinition, Episode, Outcome, vocabulary/index, and identity invariants with
   no mutable/generic bypass.

Run and record exact exits and nonzero counts:

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

Use the pinned toolchain. Confirm the new lockfile/feature graph is minimal. Do not
add exports, placeholder runtime uses, or lint bypasses to make an early partial
module appear complete. Report any genuine warning or failure.

## Handoff

Save the complete response before returning. Include delivered behavior, all
changed paths and hashes, corrected-scope criterion mapping, exact commands/exits/
test counts, uuid version/features and necessity, accepted dependency hashes,
warnings/failures/limits, active processes, and resource surprises. Reference prior
unchanged authority/skill evidence instead of repeating its hash catalogue.

Do not claim independent verification or acceptance. Return only the required
response path after its contents are complete.
