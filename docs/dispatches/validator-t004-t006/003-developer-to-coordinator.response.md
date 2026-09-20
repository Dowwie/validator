# Explicit-null and parser-marker correction handoff

Both reported defects reproduced through the strict T002 boundary. This correction is not an acceptance decision.

## Changed files and hashes

| Path | SHA-256 |
|---|---|
| Cargo.toml | 70fdbc505a467643c079f2b1a514514370cb3d5524e81bb8ca43c3eb5566686d |
| Cargo.lock | 10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3 |
| src/model/common.rs | 24b1541ca7ec1cf2e49f06e8586c323e642b889711a5fa11c35d7dc22678b18c |
| src/validation/wire.rs | cfe043a59326edd5fc611e5b7feba11df2ae96d30a25b1e8d2501ed0088a8047 |
| src/validation.rs | 2acf5459b809dc60409a0f3cd4e0ee0a70af981ee668fcbc8f42f1b6bdd82f22 |

All other frozen candidate hashes are preserved, including src/model/single_label.rs 35fe11ada58053273708751631c079984f86e6e6fad3f7d24dcae502c4b92fe5 and src/model.rs ceffb61ab5e523fa4bf3161d5db5a6fcd1276137d951982da4382edb79c58c66. Cargo.lock did not change because the serde_json feature choice does not alter resolved packages.

## Explicit optional null correction

Default Option decoding reproduced the omission/null conflation. Every optional typed wire field now uses a defaulted presence-sensitive deserializer: omission yields None, but a present value must deserialize as T and JSON null fails as E_SCHEMA.

typed_optional_nulls passes with one test. It proves null rejection for Source question_id, evidence, observation_definitions, preparation; ObservationDefinition question_id; Prediction probabilities, confidence, observations; abstention reason; EvaluationConfig episode_ids and parent_run_id. It proves omitted and valid present variants remain legal, opaque golden input remains null-capable, and opaque source/preparation configuration retains top-level and nested nulls. It calls validate_single_label with episode_ids null and receives E_SCHEMA before selection.

## Marker collision correction

With serde_json arbitrary_precision enabled, the literal object containing the private Number marker could be classified as a number by Value decoding. The correction enables serde_json raw_value and removes arbitrary_precision. Golden opaque input and source/preparation opaque configuration are retained as RawValue; common source/preparation records own those raw values. This preserves actual opaque JSON kinds and the marker object exactly. Typed scoring values use normal Number decoding, where the marker object is an object and fails E_SCHEMA.

json_number_marker_collision passes with one test: confidence equal to the literal marker object rejects E_SCHEMA; golden input equal to that object, source configuration equal to it, and nested preparation configuration retain the exact object/key/string bytes. Existing opaque big-integer and null regression remains passing.

## Required checks

Every named filter below ran one test and exited 0: typed_optional_nulls, json_number_marker_collision, wire_tags_and_fields, strict_json_keys, opaque_number_and_null, observation_contracts, source_preparation_bindings, categorical_admission, scored_choice_ties, artifact_signal_completeness, population_alignment, validate_before_selection, and dataset_digest_binding.

cargo test --locked --lib -- --list exits 0 and lists 21 tests. cargo fmt --all -- --check, cargo test --all-features --locked, cargo build --locked --bin validator, and git diff --check all exit 0.

cargo clippy --all-targets --all-features --locked -- -D warnings exits 101 solely for dead_code: 98 library diagnostics and 24 library-test diagnostics. Locations remain private incomplete-consumer records, fields, methods, and conversion helpers across src/model/common.rs, src/model/single_label.rs, src/validation.rs, src/validation/wire.rs, and accepted task wrappers in src/model.rs. No unused import or other lint class, suppression, fake caller, or placeholder was introduced.

No network operation, external process, or resource blocker occurred. The repository remains unborn/untracked; no staging, commit, reset, clean, or .zvec-grep modification occurred.

