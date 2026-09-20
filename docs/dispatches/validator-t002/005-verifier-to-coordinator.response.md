# Revise

The frozen T002 candidate correctly implements recursive duplicate-key rejection,
strict single-label envelope decoding, exact opaque-number values, and original-
byte retention. It nevertheless rejects two legal optional parts of the canonical
single-label prediction shape: source `observation_definitions` and per-row
`observations`. That prevents T004 from receiving the wire evidence it owns.

## Candidate identity

I recomputed the manifest, developer evidence, and every candidate hash before
review and again after all commands. Each value remained unchanged:

| Artifact | Recomputed SHA-256 |
|---|---|
| `docs/dispatches/validator-t002/004-coordinator-candidate-manifest.md` | `bb98ca897818623a95c82ba026cc07a243661e6835f14af7ebf26454f8efe820` |
| `docs/dispatches/validator-t002/003-developer-to-coordinator.response.md` | `04da6eea5529336e2ac927cbf2a2aee3a2ca1ade210709ae1fb5580caf79456f` |
| `Cargo.toml` | `8c758ad6246244317ac02156077b417eab3cd49994daf2597c65ae86bb24629c` |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |
| `src/lib.rs` | `b60b0bf5d4fed5fe6045e7c1039f3c21e315dc46d9bc08159d56d2aa4ab35162` |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` |
| `src/model.rs` | `3275697520566394a2a20d4011ad28b3912d9d4773a44203980127d3f2d4d4d1` |
| `src/model/common.rs` | `ee0b0169d88707663c7b1540d1406868ba61cb0db2fd4d36dd2728d3baf9cecf` |
| `src/validation.rs` | `0c2e784d933ee074610963d36488331102c4cda99f7da17b470f5105d2d26c8e` |
| `src/validation/wire.rs` | `718c871da7764ff26502043f111d8ecc1224a97b0cce27cc82081460e479f895` |

## Required finding

### Legal retained-observation fields are rejected by the strict DTOs

- **Requirement:** corrected T002 dispatch Required boundary and verifier Required
  review; `validator-v1.md` Canonical prediction artifact and Retained classifier
  observations. T002 must admit all legal shared/single-label wire fields needed
  by T004 while deferring their semantic validation.
- **Location:** `src/validation/wire.rs:58-65` defines `Source` with
  `deny_unknown_fields` but no optional `observation_definitions` field.
  `src/validation/wire.rs:85-91` defines `Prediction` with
  `deny_unknown_fields` but no optional `observations` field. There are no wire
  types for observation definitions or the five tagged observation value kinds.
- **Reproduction:** a legal single-label prediction source and row containing:

  ```json
  {
    "observation_definitions": {
      "score": {"kind": "scalar", "description": "native score"}
    }
  }
  ```

  and:

  ```json
  {
    "observations": {
      "score": {"kind": "scalar", "value": 1.33}
    }
  }
  ```

  reaches the duplicate pass successfully, then `decode_prediction_artifact`
  returns `E_SCHEMA` because serde treats both names as unknown. The positive
  `PREDICTIONS` fixture at `src/validation.rs:69-86` omits both legal fields, so
  `wire_tags_and_fields` does not detect the missing shape.
- **Consequence:** a conforming artifact that retains classifier observations
  cannot cross T002's wire boundary. T004 therefore cannot validate or preserve
  those observations without changing the already accepted T002 contract or
  inventing an alternate decode path.
- **Smallest correction:** add optional `observation_definitions` to `Source` and
  optional `observations` to `Prediction`, plus strict private DTOs for definition
  fields and all five tagged values: `scalar`, `bernoulli`,
  `reported_confidence`, `categorical`, and `label_marginals`. Keep numeric values
  as `JsonNumber`, vector maps as exact string-keyed maps, and defer nonblank,
  range, sum, vocabulary, definition-reference, and completeness semantics to
  T004/T005. Extend the positive wire-shape test with legal definitions and row
  observations, and retain a discriminating unknown-kind/extra-field rejection.

## Satisfied T002 criteria

### Duplicate rejection and diagnostics

`src/validation/wire.rs:181-311` runs a streaming visitor over the entire input
before DTO deserialization. Every object receives a fresh decoded-string key set;
values recurse through objects and arrays. It therefore rejects root, nested
opaque input/configuration/preparation, escaped-equivalent, and any future legal
observation-map duplicate before serde can overwrite it. The duplicate path
returns typed `E_SCHEMA`; syntactically malformed/trailing input without a
duplicate returns typed `E_PARSE`. No raw value is interpolated into diagnostics.
The named test passes root, opaque nested, escaped-equivalent, source-configuration,
and preparation-containing cases. Its combined source/preparation sample stops at
the first duplicate, but the generic recursive visitor independently establishes
the deeper behavior.

### Existing wire shapes and strictness

- Golden DTOs require integer schema version 2, an explicit `single_label` task,
  labels, episodes, typed class targets, IDs, and a present arbitrary JSON input.
  Explicit null input is accepted; absence is rejected.
- Prediction DTOs correctly cover dataset digest text, source kind/model/object
  configuration/question/evidence/preparation, class or abstention outcome,
  optional categorical probability map, and optional numeric confidence. The
  finding above is the only missing legal single-label shape found.
- Configuration DTOs cover required population/role/decision, both legal
  single-label policies, optional selection IDs, and optional parent run ID.
- Tagged enums and `deny_unknown_fields` reject unknown fields, wrong primitives,
  unsupported wire versions, untagged targets, multi-label concrete tags,
  label-marginal scoring probability, and label-threshold policy without inference
  or coercion. The DTO layer does not perform UUID, digest, nonblank, vocabulary,
  source-reference, range, sum, signal-completeness, alignment, or policy semantics.

### Exact numeric value and original bytes

`src/validation.rs:7-24,43-51` owns an unchanged `Vec<u8>` beside the decoded DTO
and exposes the original slice rather than reconstructing JSON. The numeric test
proves `9007199254740993` remains exactly `Some(9_007_199_254_740_993)` through
`serde_json::Value`, explicit null succeeds, missing input fails, and retained
bytes equal the submitted bytes. DTO lexical number spelling is correctly not an
acceptance gate; the authoritative bytes preserve it for later snapshots.

### Privacy, purity, dependencies, and regressions

`src/lib.rs:11` adds only private `mod validation`. `wire` is private beneath it;
DTOs and decode functions have no external SDK exposure and are distinct from
validated model types. Searches found no filesystem, environment, clock, RNG,
scoring, artifact publication, schema engine, CLI, unsafe, async, suppression,
fake caller, placeholder, or production unwrap/expect/panic. The only unwrap is
inside the T002 test module.

Cargo adds no package and changes only serde_json from `std` to
`arbitrary_precision` plus `std`. The feature tree contains no `raw_value`; UUID
still has no optional feature. `Cargo.lock` and all accepted T001/T003 source files
retain their manifest hashes. The full suite passes all six prior T001/T003 tests.

## Independent commands

All commands used pinned Rust/Cargo 1.98.1 on `x86_64-apple-darwin`.

| Command | Exit | Actual result |
|---|---:|---|
| `cargo test --locked --lib -- --list` | 0 | Listed 9 tests and 0 benchmarks; each exact T002 test appears once. Test compilation emitted 15 DTO-field dead-code warnings. |
| `cargo test --locked --lib strict_json_keys -- --nocapture` | 0 | 1 passed, 0 failed, 8 filtered; same 15 test-target warnings. |
| `cargo test --locked --lib opaque_number_and_null -- --nocapture` | 0 | 1 passed, 0 failed, 8 filtered; same 15 warnings. |
| `cargo test --locked --lib wire_tags_and_fields -- --nocapture` | 0 | 1 passed, 0 failed, 8 filtered; same 15 warnings. |
| `cargo fmt --all -- --check` | 0 | No differences or output. |
| `cargo test --all-features --locked` | 0 | 9 library tests passed; binary/doc targets ran 0 and passed. Production library emitted 52 dead-code warnings; lib-test emitted 15. |
| `cargo build --locked --bin validator` | 0 | Locked binary built; emitted the same 52 production dead-code warnings. |
| `cargo tree --locked -e features` | 0 | Confirmed serde_json `std` + `arbitrary_precision`, no `raw_value`, and UUID with no optional feature. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Failed only on the documented incomplete-consumer `dead_code` groups described below. |
| `cargo metadata --locked --no-deps --format-version 1` | 0 | One package, one private library module declaration, one binary, and the expected feature set. |
| `git diff --check` | 0 | No tracked whitespace error; rustfmt is the substantive format check in this unborn untracked repository. |

## Exact Clippy classification

Clippy did **not** pass. The non-test library reports 52 `dead_code` groups:

- The 24 accepted T003 groups previously enumerated in
  `docs/dispatches/validator-t003/009-verifier-to-coordinator.response.md`.
- 28 T002 groups: `Decoded`; its accessor group; the three public-crate decode
  entry points; generic `decode`; GoldenDataset; SingleLabelTask; SingleLabelTag;
  GoldenEpisode; ClassTarget; PredictionArtifact; Source; SourceKind; Preparation;
  Prediction; PredictionOutcome; CategoricalProbabilities; EvaluationConfig;
  EvaluationRole; SingleLabelDecision; RejectionSignal; OpaqueObject; JsonNumber;
  `reject_duplicate_keys`; DuplicateKeyVisitor; DuplicateKeySeed; and
  `schema_version_two` at their definitions in `src/validation.rs:9-43` and
  `src/validation/wire.rs:12-313`.

The lib-test target additionally reports 15 unread-field `dead_code` groups:
GoldenDataset fields; SingleLabelTask fields; GoldenEpisode ID/expected;
ClassTarget label; PredictionArtifact fields; Source fields; Preparation fields;
Prediction fields; class label; abstention reason; categorical payload;
EvaluationConfig fields; reject-below fields; OpaqueObject payload; and JsonNumber
payload.

No `unused_imports`, Clippy style/performance lint, suppression, or other warning
class appeared. These warnings are the owner-authorized incomplete-consumer limit;
they are unresolved and clean full Clippy remains mandatory at T014/T017. They do
not conceal or supersede the wire-shape defect above.

## Execution state

No implementation, test, fixture, schema, specification, plan, acceptance artifact,
artifact index, or Fizzy record was edited. This response is my only write. No
verifier-owned command or background process remains active. Disk had about 15 GiB
available and `target/` was approximately 197 MiB, with no resource blocker.
