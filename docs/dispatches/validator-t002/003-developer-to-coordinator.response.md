# T002 developer handoff

## Delivered boundary

T002 now decodes the three single-label JSON envelopes through a private
`validation` module before semantic interpretation:

- `validation::wire` performs a recursive streaming duplicate-key pass before
  `serde_json` deserializes any DTO. It detects duplicate names at every object
  depth after JSON escape decoding, including opaque input and configuration
  objects. Duplicate keys return typed `E_SCHEMA`; malformed JSON returns `E_PARSE`.
- Strict private DTOs accept only `schema_version: 2`, the explicit single-label
  task/target/probability/policy tags, required fields, and declared primitive
  shapes. `deny_unknown_fields` applies outside `input` and opaque configuration
  objects. Unsupported multi-label tags and untagged inference fail during decode.
- `Decoded<T>` owns a byte-for-byte copy of the original submitted slice alongside
  its DTO. Opaque episode input is represented by `serde_json::Value`, so explicit
  `null` is accepted and missing input fails. The `arbitrary_precision` feature
  preserves numeric values without forcing opaque numbers through binary64.

No semantic ID/digest/nonblank/vocabulary/source/range/alignment/policy checks,
filesystem access, schema engine, artifact behavior, scoring, or CLI behavior was
introduced. Wire DTOs remain internal and are never deserialized into checked model
types.

## Changed paths

| Path | Purpose | SHA-256 |
|---|---|---|
| `Cargo.toml` | Adds `serde_json`'s `arbitrary_precision` feature for opaque numeric values | `8c758ad6246244317ac02156077b417eab3cd49994daf2597c65ae86bb24629c` |
| `Cargo.lock` | Unchanged: this feature selection does not alter the already locked package graph | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` |
| `src/lib.rs` | Declares the implemented private `validation` module only | `b60b0bf5d4fed5fe6045e7c1039f3c21e315dc46d9bc08159d56d2aa4ab35162` |
| `src/validation.rs` | Pure decoding entry points, retained bytes, and owning tests | `0c2e784d933ee074610963d36488331102c4cda99f7da17b470f5105d2d26c8e` |
| `src/validation/wire.rs` | Private tagged DTOs, numeric/configuration wrappers, and recursive duplicate-key visitor | `718c871da7764ff26502043f111d8ecc1224a97b0cce27cc82081460e479f895` |

The response file's final hash is reserved for the coordinator freeze.

## Requirement evidence

| Requirement group | Implementation and test evidence | Deferred ownership |
|---|---|---|
| V004, V026–V044, D049–D051 | `GoldenDataset`, `SingleLabelTask`, `GoldenEpisode`, and tagged `ClassTarget` enforce strict single-label envelope shape, schema version, required input, and no unknown fields. `wire_tags_and_fields` passes valid input and rejects unknown fields, primitive/type errors, version 3, multi-label, and untagged targets. | UUID/nonblank/vocabulary/target semantic validation and model construction are T003+ work. |
| V045–V069, V101–V105, E13 | `PredictionArtifact`, `Source`, `Preparation`, `Prediction`, tagged `PredictionOutcome`, categorical probabilities, and numeric wrappers establish shape without source/reference/range/completeness semantics. `EvaluationConfig` and tagged single-label decisions reject unsupported policy variants. | Source/observation semantics, complete signal contracts, probability/policy semantics, alignment, and evaluation are later-task work. |
| V104, D051 | `reject_duplicate_keys` streams every map and sequence before DTO deserialization. It records duplicate detection independently of error text and treats decoded equivalent keys such as `"a"`/`"\\u0061"` as duplicates. `strict_json_keys` passes root, nested opaque input, escaped, source configuration, and preparation-configuration cases. | No duplicate-key fallback exists. |
| V036, V043, S28 | `Decoded<T>` owns original bytes and does not reconstruct them by serialization. `opaque_number_and_null` proves `9007199254740993` remains an exact `u64` value, explicit `input: null` decodes, absent input fails, and retained bytes equal the submitted byte slice. | Snapshot publication and inspection remain artifact/CLI tasks. Exact lexical spelling in DTOs was intentionally not added under owner correction 002; the original retained bytes preserve it for snapshots. |

## Commands and results

Pinned environment: `rustc 1.98.1 (48a229cea 2026-09-01)`, Cargo 1.98.1,
`1.98.1-x86_64-apple-darwin`.

| Command | Exit | Actual result |
|---|---:|---|
| `cargo test --locked --lib -- --list` | 0 | Listed 9 library tests, including each exact T002 test once. |
| `cargo test --locked --lib strict_json_keys -- --nocapture` | 0 | One matching test passed; eight filtered. |
| `cargo test --locked --lib opaque_number_and_null -- --nocapture` | 0 | One matching test passed; eight filtered. |
| `cargo test --locked --lib wire_tags_and_fields -- --nocapture` | 0 | One matching test passed; eight filtered. |
| `cargo fmt --all -- --check` | 0 | No formatting differences. |
| `cargo test --all-features --locked` | 0 | Nine library tests passed; binary and doc-test targets each ran zero tests and passed. |
| `cargo build --locked --bin validator` | 0 | Binary built. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Expected incomplete-consumer dead-code diagnostics only; no unused-import, semantic-lint, or suppression finding. |
| `cargo tree --locked -e features` | 0 | `serde_json` enables only `std` and newly necessary `arbitrary_precision`; UUID remains without optional features. |
| `git diff --check` | 0 | No tracked whitespace error; candidate files are untracked in this unborn repository. |

## Dependency rationale

No package was added. `serde_json` now enables `arbitrary_precision` in addition to
its existing `std` feature so opaque JSON numeric values are retained without a
binary64 conversion. `raw_value` was deliberately not enabled: owner correction
002 states DTO lexical spelling is not a T002 requirement, and `Decoded<T>::bytes`
retains the authoritative exact byte sequence for later snapshots.

## Clippy limit

Clippy is not passed or waived. It exits 101 with only `dead_code` diagnostics:

- The accepted T003 private foundation remains the existing 24 incomplete-consumer
  diagnostics in `src/model.rs` and `src/model/common.rs`.
- The non-test library target adds 28 T002 incomplete-consumer diagnostics for
  `Decoded`, the three decode entry points, `decode`, and the strict wire DTO/
  visitor types in `src/validation.rs` and `src/validation/wire.rs`.
- The library-test target additionally reports 15 unread private DTO field groups
  in `src/validation/wire.rs`, because tests intentionally inspect only the opaque
  input needed for T002's numeric proof.

There are no `unused_imports`, Clippy style/performance diagnostics, semantic errors,
lint attributes, fake consumers, or other warning classes. Real admission and CLI
consumers will remove these documented temporary diagnostics; clean full Clippy
remains required at T014 and T017.

## Accepted inputs and limits

Unchanged accepted inputs: `rust-toolchain.toml`
`7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8`,
`src/error.rs` `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa`,
`src/model.rs` `3275697520566394a2a20d4011ad28b3912d9d4773a44203980127d3f2d4d4d1`,
and `src/model/common.rs`
`ee0b0169d88707663c7b1540d1406868ba61cb0db2fd4d36dd2728d3baf9cecf`.

Relevant correction/dispatch hashes:

| Artifact | SHA-256 |
|---|---|
| `002-owner-to-coordinator.prompt.md` | `d791d348ae8cf9eba9fed04929c240297066dbd271e914034fb35e4a2f9676d8` |
| `003-coordinator-to-developer.prompt.md` | `3f0efa14548c79c9ec98c9d3c2b4e42b65608fe0e5ab98f48ab180ff21932588` |

No Cargo or Rust compiler process remained after verification. The normal feature
resolution introduced no resource surprise. This handoff does not claim independent
verification, a Ready verdict, or owner acceptance.
