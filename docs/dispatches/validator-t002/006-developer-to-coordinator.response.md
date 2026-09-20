# T002 repair 006 developer handoff

## Delivered repair

The private prediction wire boundary now accepts the complete legal retained-
observation shape needed by T004:

- `Source.observation_definitions` is an optional exact string-keyed map. Each
  private `ObservationDefinition` requires `kind` and `description`, and permits
  `question_id`; unknown fields fail.
- `Prediction.observations` is an optional exact string-keyed map of private tagged
  `Observation` variants: `scalar`, `bernoulli`, `reported_confidence`,
  `categorical`, and `label_marginals`.
- Scalar-like variants require numeric `value`; vector variants require a
  string-keyed map of existing deferred `JsonNumber` values. Unknown kinds, extra
  fields, and wrong primitives fail during strict DTO decoding.

The implementation deliberately does not validate names/descriptions/question IDs,
numeric range/finiteness, vector content, source-definition matching, vocabulary
binding, sums, completeness, or scoring promotion. Those semantic invariants remain
with T004 and later tasks. Duplicate checking, original-byte retention, explicit
tags, privacy, and existing T002 shape behavior are unchanged.

`wire_tags_and_fields` now decodes one legal single-label prediction artifact with
definitions and row values for all five kinds. It also verifies an unknown
observation kind and an extra observation field are rejected.

## Changed/current hashes

| Path | SHA-256 | State |
|---|---|---|
| `src/validation/wire.rs` | `e81c528e0659e1fb4b2364be82639bbb27175947b042145a0f314ec00e827d17` | Adds only retained-observation DTO shape. |
| `src/validation.rs` | `6c6c71080372b1d5b0a1bf707b88e1db694d88a142a3f2d94b8a467ff0020f72` | Extends the existing owning wire-shape test. |
| `Cargo.toml` | `8c758ad6246244317ac02156077b417eab3cd49994daf2597c65ae86bb24629c` | Unchanged. |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` | Unchanged. |
| `src/lib.rs` | `b60b0bf5d4fed5fe6045e7c1039f3c21e315dc46d9bc08159d56d2aa4ab35162` | Unchanged. |
| `src/model.rs` | `3275697520566394a2a20d4011ad28b3912d9d4773a44203980127d3f2d4d4d1` | Unchanged. |
| `src/model/common.rs` | `ee0b0169d88707663c7b1540d1406868ba61cb0db2fd4d36dd2728d3baf9cecf` | Unchanged. |

The response file's own final hash is for the coordinator freeze.

## Evidence and checks

| Requirement | Evidence |
|---|---|
| Legal observation-definition and row-observation wire fields are accepted | `Source` and `Prediction` have optional exact maps; `ObservationDefinition`, `ObservationKind`, and `Observation` cover all required fields and five closed tags. The positive `wire_tags_and_fields` case passed. |
| Unknown observation kind, extra field, and wrong primitive fail safely | `serde` tagged enums plus `deny_unknown_fields` reject unsupported kinds and extra fields. `JsonNumber` rejects nonnumeric `value`/map values. The extended named test passed the unknown-kind and extra-field cases. |
| Existing strict T002 boundary is preserved | `strict_json_keys` and `opaque_number_and_null` reran and passed. The repair did not touch duplicate rejection, byte retention, numeric representation, dependencies, module wiring, or public API. |

Pinned Rust/Cargo 1.98.1 on `x86_64-apple-darwin` ran:

| Command | Exit | Actual result |
|---|---:|---|
| `cargo test --locked --lib wire_tags_and_fields -- --nocapture` | 0 | One matching test passed; eight filtered. |
| `cargo test --locked --lib strict_json_keys -- --nocapture` | 0 | One matching test passed; eight filtered. |
| `cargo test --locked --lib opaque_number_and_null -- --nocapture` | 0 | One matching test passed; eight filtered. |
| `cargo fmt --all -- --check` | 0 | No formatting differences. |
| `cargo test --all-features --locked` | 0 | Nine library tests passed; binary and doc-test targets each ran zero tests and passed. |
| `cargo build --locked --bin validator` | 0 | Binary built. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Expected incomplete-consumer dead-code diagnostics only. |
| `git diff --check` | 0 | No tracked whitespace error in the unborn repository. |

## Clippy classification

Clippy is not passed or waived. Its only diagnostic class is `dead_code`:

- 24 accepted T003 private-foundation groups.
- 31 T002 non-test groups: the prior 28 decode/DTO/visitor groups plus
  `ObservationDefinition`, `ObservationKind`, and `Observation`.
- 21 library-test unread-field groups in private wire DTOs, up from 15 because the
  definitions and five observation variants are intentionally not consumed until
  T004.

The non-test library target therefore exits with 55 errors and the lib-test target
with 21 errors under `-D warnings`. There are no `unused_imports`, style,
performance, semantic, suppression, or fake-consumer diagnostics.

## Reused evidence and state

Frozen review inputs:

| Artifact | SHA-256 |
|---|---|
| `004-coordinator-candidate-manifest.md` | `bb98ca897818623a95c82ba026cc07a243661e6835f14af7ebf26454f8efe820` |
| `005-verifier-to-coordinator.response.md` | `d734e9f3d1d0207ced264f2a2e3d671d37c56e885031c0e8fbe8b37df4bed2b9` |
| `006-coordinator-to-developer.prompt.md` | `f74630a83290941e14e36165d0f69ce19e7fd082f6eb92d6ffab8cbdaf959840` |

No background Cargo or Rust compiler process remains. No dependency update or
resource surprise occurred. This response does not claim independent Ready or owner
acceptance.
