# Validator T002 developer dispatch 001

Role/model: existing developer, `gpt-5.6-terra`, reasoning `high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Coordinator: `/root/coordinator`, Sol high. Required full response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t002/001-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator’s verified single-label backbone](http://localhost:3006/1/cards/183).

## Outcome and authority

Implement T002 only: decode exact untrusted UTF-8 JSON bytes into private strict
wire DTOs before semantic interpretation, rejecting duplicate keys recursively and
preserving opaque JSON values/numbers. You remain the sole implementation and test
writer. Do not delegate or spawn agents.

T001 and corrected T003 are owner-accepted. T003 acceptance is bound to repair
manifest SHA-256
`ceb00209087532787858fbb60045d5dbb2f9ae10c91ff60c5e5be4eee3c802c3`
with 24 incomplete-consumer `dead_code` diagnostics explicitly unresolved; Clippy
is not passed or waived. Read before editing:

- `docs/plans/validator/tasks/T002.json` and
  `docs/plans/validator/execution-contract.md` in their corrected form.
- `docs/dispatches/validator-build/003-owner-to-coordinator.prompt.md` for the
  narrow parent-module/dependency routing rule.
- `docs/specs/validator-v1.md`: Format revision, Canonical golden dataset,
  Canonical prediction artifact, Evaluation configuration, Validation and numerical
  rules, CLI/run-artifact exact-byte obligations, and Rust organization.
- `docs/specs/validator-data-model.md`: Decode/validate/align/score and source
  organization. Wire DTOs remain private and distinct from checked model types.
- `docs/dev-team/validator-build/charter.md`, repository `AGENTS.md`, and the Rust
  best-practices guidance already read.
- Requirement IDs named by T002 in `docs/plans/validator/coverage.json`, applying
  only T002's strict wire/shape ownership. Later tasks own semantic validation,
  multi-label concrete paths, scoring, policy execution, alignment, artifacts,
  schemas, and CLI behavior.

## Accepted baseline

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `fab94b0c36151466e787b9c8e49d655961b95eea947b0e27fc82df4c905b3da0` |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |
| `src/lib.rs` | `4ff48cc3bfa59a05482c3e939251a88bce41dfea9f2c62e45a2cc0df95fad509` |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` |
| `src/model.rs` | `3275697520566394a2a20d4011ad28b3912d9d4773a44203980127d3f2d4d4d1` |
| `src/model/common.rs` | `ee0b0169d88707663c7b1540d1406868ba61cb0db2fd4d36dd2728d3baf9cecf` |

The repository remains unborn/untracked. Preserve unrelated and accepted bytes
outside the write scope. Do not stage, commit, clean, reset, or edit `.zvec-grep`.

## Exact write scope

You may create or modify only:

- `src/validation.rs`: pure decode entry points and owning tests; no filesystem,
  semantic model construction, scoring, or public application API.
- `src/validation/wire.rs`: private/super-private tagged DTOs and recursive
  duplicate-key rejection.
- `src/lib.rs`: only the narrow declaration of the now-implemented private
  validation module; do not widen the public SDK or change existing exports.
- `Cargo.toml` and `Cargo.lock`: enable/add only serialization support actually
  necessary for recursive strict decoding and exact opaque number/value retention.
  Explain every feature/dependency; no general framework or future dependency.
- `/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t002/001-developer-to-coordinator.response.md`.

Do not edit model/error/main, other source/test files, specs, plans, index, Fizzy,
session notes, schemas, fixtures, or acceptance artifacts.

## Required boundary

- Input is exact UTF-8 bytes. Run a recursive duplicate-key check before any
  parser can overwrite values. Reject duplicate keys at every depth, including
  opaque `input`, source `configuration`, preparation configuration, observation
  vectors, and escaped-equivalent spellings such as `"a"` and `"\u0061"`.
- Decode the three canonical input envelopes needed by the single-label path:
  golden dataset, prediction artifact, and evaluation config. Keep their wire DTOs
  behind `validation/wire.rs`; callers cannot deserialize directly into validated
  model types. T018 owns the later concrete multi-label DTO paths. Reject unsupported
  task/target/probability/policy variants explicitly rather than infer/coerce them.
- Apply exact tagged shapes, required fields, primitive types, and unknown-field
  rejection outside explicitly opaque `input` and configuration objects. Opaque
  regions accept arbitrary JSON shape but still undergo syntax/duplicate checks.
- Require integer `schema_version: 2` in every envelope; reject other versions,
  booleans, strings, floats, and aliases. Preserve explicit tags; no untagged
  fallback or inference from cardinality/probability sums.
- Required episode `input` distinguishes missing from explicit JSON null. Preserve
  arbitrary opaque JSON numbers and their submitted spelling without conversion to
  binary64. At minimum, integer `9007199254740993` must remain exact through decode
  and inspection of the private DTO; include a discriminating lexical-number case
  if the selected serde_json feature claims spelling preservation.
- Strict decoding establishes shape only. Do not implement nonblank/UUID/digest/
  vocabulary/source-reference/range/sum/completeness/alignment/policy semantic
  checks owned by T003–T006. Wrong shapes/tags/fields/versions fail with the
  appropriate accepted typed diagnostic and never expose raw payload text.
- Keep exact submitted bytes available to the caller for later snapshots; do not
  serialize DTOs as a substitute for the original input.

Prefer serde/serde_json facilities plus the smallest custom visitor needed for
recursive duplicate detection. Do not build a general JSON parser or schema engine.

## Acceptance and checks

Implement owning-module tests named exactly:

1. `strict_json_keys`: root, nested, opaque input/configuration, and escaped-
   equivalent duplicate keys fail before overwrite.
2. `opaque_number_and_null`: large integer and representative lexical numeric
   spelling remain exact; explicit null input is legal; missing input is rejected.
3. `wire_tags_and_fields`: valid single-label envelopes decode; unknown fields,
   wrong primitives/tags, unsupported wire versions, incompatible or future
   multi-label shapes, and untagged inference attempts fail safely.

Run and record exact exits and nonzero counts:

```sh
cargo test --locked --lib -- --list
cargo test --locked --lib strict_json_keys -- --nocapture
cargo test --locked --lib opaque_number_and_null -- --nocapture
cargo test --locked --lib wire_tags_and_fields -- --nocapture
cargo fmt --all -- --check
cargo test --all-features --locked
cargo build --locked --bin validator
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo tree --locked -e features
```

Full Clippy may still exit 101 only for documented incomplete-consumer dead code
in private pre-CLI modules. It must not contain `unused_imports`, semantic lint
failures, suppression, fake consumers, or other warning classes. Capture the exact
remaining diagnostic classes/count/locations; do not call it passed. Clean full
Clippy remains mandatory at T014 and T017.

## Handoff

Save the complete response before returning. Include delivered DTO/decode boundary,
all changed hashes, acceptance/source mapping, exact commands/exits/test counts,
dependency features and necessity, exact number-preservation evidence, complete
Clippy classification, unchanged accepted-input hashes, failures/limits/processes/
resources. Reference unchanged authority evidence concisely.

Do not claim independent verification or acceptance. Return only the response path.
