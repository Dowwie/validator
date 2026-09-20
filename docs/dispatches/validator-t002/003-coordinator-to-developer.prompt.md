# Validator T002 corrected developer dispatch 003

Role/model: existing developer, `gpt-5.6-terra`, reasoning `high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Coordinator: `/root/coordinator`, Sol high. Required full response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t002/003-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator’s verified single-label backbone](http://localhost:3006/1/cards/183).

This complete prompt supersedes T002 dispatch 001 before implementation began.
The owner correction at
`docs/dispatches/validator-t002/002-owner-to-coordinator.prompt.md` removes only
the extra decoded-number lexical-spelling requirement. All other T002 scope and
criteria remain.

## Outcome and authority

Implement T002 only: decode exact untrusted UTF-8 JSON bytes into private strict
wire DTOs before semantic interpretation, rejecting duplicate keys recursively and
preserving opaque JSON numeric values. Exact lexical spelling/formatting is
preserved by retaining the original input bytes for later snapshots; internal DTOs
may use an ordinary value representation and may render value-equivalent numbers
differently. Do not add a raw-value/lexical round-trip mechanism solely for DTO
spelling.

You remain the sole implementation and test writer. Do not delegate or spawn
agents. T001 and corrected T003 are owner-accepted; T003's accepted repair manifest
is `ceb00209087532787858fbb60045d5dbb2f9ae10c91ff60c5e5be4eee3c802c3`
with its incomplete-consumer dead-code integration limit still open.

Read:

- Corrected `docs/plans/validator/tasks/T002.json` and
  `docs/plans/validator/execution-contract.md`.
- `docs/dispatches/validator-build/003-owner-to-coordinator.prompt.md` for narrow
  module/dependency wiring, and the numeric correction named above.
- `docs/specs/validator-v1.md`: Format revision, Canonical golden dataset,
  Canonical prediction artifact, Evaluation configuration, Validation/numerical
  rules, exact snapshot obligations, and Rust organization.
- `docs/specs/validator-data-model.md`: Decode/validate/align/score and source
  organization. Wire DTOs remain distinct from checked models.
- Build charter, repository `AGENTS.md`, Rust guidance already read, and T002's
  coverage IDs. Later tasks own semantics, multi-label concrete paths, scoring,
  alignment, artifacts, schemas, and CLI.

## Baseline and write scope

No T002 implementation bytes existed when dispatch 001 was interrupted. Accepted
baseline hashes:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `fab94b0c36151466e787b9c8e49d655961b95eea947b0e27fc82df4c905b3da0` |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |
| `src/lib.rs` | `4ff48cc3bfa59a05482c3e939251a88bce41dfea9f2c62e45a2cc0df95fad509` |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` |
| `src/model.rs` | `3275697520566394a2a20d4011ad28b3912d9d4773a44203980127d3f2d4d4d1` |
| `src/model/common.rs` | `ee0b0169d88707663c7b1540d1406868ba61cb0db2fd4d36dd2728d3baf9cecf` |

You may create/modify only `src/validation.rs`, `src/validation/wire.rs`, the narrow
private validation-module declaration in `src/lib.rs`, Cargo.toml/Cargo.lock only
for serialization features/dependencies actually necessary now, and the required
response file. Do not widen the public SDK or edit other code/docs/tracking files.
Preserve unrelated/untracked material; no staging, commit, clean, or reset.

## Required boundary

- Input is exact UTF-8 bytes. Reject duplicate keys before overwrite at every
  nesting depth, including opaque input/configuration/preparation/observation maps
  and escaped-equivalent names such as `"a"` and `"\u0061"`.
- Decode strict private/super-private wire DTOs for the three canonical envelopes
  needed by the single-label path: golden dataset, predictions, and config. T018
  owns concrete multi-label paths; reject unsupported variants without inference.
- Enforce tagged shapes, required fields, primitive types, `schema_version` integer
  2, and unknown-field rejection outside opaque input/configuration objects. No
  untagged fallback, aliases, coercion, or cardinality/sum inference.
- Required episode `input` distinguishes missing from explicit null. Preserve
  opaque JSON numeric **values**, including integer `9007199254740993`, without
  binary64 rounding. A value-equivalent decoded representation is sufficient;
  original numeric spelling/formatting belongs to the exact retained bytes.
- Return or retain the exact submitted byte slice/owned bytes with decoded content
  so later snapshot publication can write byte-for-byte input. Never reconstruct
  a snapshot by serializing DTOs.
- Strict decoding establishes shape only. Do not implement UUID/digest/nonblank/
  vocabulary/source-reference/range/sum/completeness/alignment/policy semantics.
  Use accepted typed diagnostics without raw-payload/credential interpolation.
- No filesystem, environment, clock, RNG, scoring, artifact, schema-engine, or CLI
  behavior. Prefer serde/serde_json plus the smallest recursive duplicate visitor;
  do not build a general parser.

## Tests and checks

Implement exact local tests:

1. `strict_json_keys`: root/nested/opaque/configuration and escaped-equivalent
   duplicates fail before overwrite.
2. `opaque_number_and_null`: integer `9007199254740993` remains the exact numeric
   value without binary64 conversion; explicit null input passes; missing input
   fails; retained input bytes remain byte-for-byte unchanged. Do not require DTO
   lexical spelling beyond value preservation.
3. `wire_tags_and_fields`: valid single-label envelopes decode; unknown fields,
   wrong primitives/tags, unsupported versions, incompatible/future multi-label
   shapes, and untagged inference attempts fail safely.

Run and record:

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

Named filters must run nonzero counts. Full Clippy may exit 101 only for documented
incomplete-consumer dead code in private pre-CLI modules; list exact classes/count/
locations and never call it passed. `unused_imports`, other warning classes,
suppression, fake callers, or semantic failures require correction. Clean full
Clippy remains mandatory at T014/T017.

## Handoff

Save the complete response with behavior, changed hashes, criterion/source mapping,
commands/exits/counts, dependency necessity/features, exact numeric-value and
snapshot-byte evidence, complete Clippy classification, accepted-input hashes,
limits/processes/resources. Do not claim independent acceptance. Return only the
response path after saving it.
