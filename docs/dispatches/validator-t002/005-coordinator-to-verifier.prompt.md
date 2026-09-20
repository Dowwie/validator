# Validator T002 verifier dispatch 005

Role/model: existing verifier, `gpt-5.6-sol`, reasoning `high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required full response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t002/005-verifier-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator’s verified single-label backbone](http://localhost:3006/1/cards/183).

## Outcome and candidate

Independently verify the exact frozen T002 candidate for strict private single-label
wire decoding, recursive duplicate rejection, exact opaque numeric values, and
original-byte retention. Return **Ready**, **Revise**, or **Blocked**. Do not audit
semantic validation/scoring/artifacts/CLI, require lexical number spelling inside
DTOs, or broaden into T004/T018 behavior.

Read before implementation evidence:

- Corrected T002 contract and execution contract.
- Main specification sections for format, golden/prediction/config wire shapes,
  validation/numerical rules, exact snapshot bytes, and Rust ownership.
- Data-model decode boundary, build charter, and AGENTS.md.
- Numeric correction:
  `docs/dispatches/validator-t002/002-owner-to-coordinator.prompt.md`.
- Corrected developer prompt 003.
- Frozen manifest:
  `docs/dispatches/validator-t002/004-coordinator-candidate-manifest.md`, SHA-256
  `bb98ca897818623a95c82ba026cc07a243661e6835f14af7ebf26454f8efe820`.
- Then developer response 003, SHA-256
  `04da6eea5529336e2ac927cbf2a2aee3a2ca1ade210709ae1fb5580caf79456f`.

Implementation writes are paused. Recompute every candidate hash in manifest 004;
stop Blocked if any implementation/evidence/manifest hash changes.

## Required review

Inspect the complete scoped change and surrounding contracts. Verify:

- Duplicate keys are rejected before overwrite at every recursive object depth,
  including opaque input/configuration/preparation/observation maps and escaped-
  equivalent names. Malformed syntax and duplicates use typed safe diagnostics.
- Golden, prediction, and configuration DTOs accept all legal shared/single-label
  wire fields needed by T004–T006 and reject unknown fields, wrong primitive types,
  unsupported versions/tags, multi-label concrete variants, and untagged inference.
  Check legal optional source/prediction structures from the spec as shapes even
  though T004 owns their semantic validation, including observation definitions,
  per-row observations, source evidence/preparation, abstentions, scoring signals,
  selections, and parent IDs. T002 must not reject a legal single-label artifact
  merely because later semantic checks are not implemented; it also must not
  perform those semantic checks early.
- Every envelope requires integer schema version 2. Required episode input
  distinguishes explicit null from absence. Opaque configuration remains an object;
  opaque input remains arbitrary JSON.
- Integer `9007199254740993` is retained as an exact numeric value without binary64
  conversion. DTO lexical spelling is not an acceptance criterion. Original input
  bytes are retained independently for byte-for-byte future snapshots and are not
  reconstructed from serialization.
- DTOs remain private/super-private and are not validated model types; validation
  has no filesystem/environment/clock/RNG/scoring/schema-engine/CLI behavior.
- `src/lib.rs` change is only the private module declaration. serde_json feature
  change is necessary/minimal; no raw-value or framework dependency appears.
- T001/T003 accepted behavior remains unchanged. No suppressions, fake consumers,
  placeholders, public SDK expansion, or production panic/unwrap/expect appear.

Run and record exact exits/counts:

```sh
cargo test --locked --lib -- --list
cargo test --locked --lib strict_json_keys -- --nocapture
cargo test --locked --lib opaque_number_and_null -- --nocapture
cargo test --locked --lib wire_tags_and_fields -- --nocapture
cargo fmt --all -- --check
cargo test --all-features --locked
cargo build --locked --bin validator
cargo tree --locked -e features
cargo clippy --all-targets --all-features --locked -- -D warnings
```

Confirm named tests execute nonzero counts. Clippy is expected to exit 101 only on
documented incomplete-consumer `dead_code`; classify production and lib-test field
groups and report any other class normally. Do not call it passed or waived; clean
full Clippy remains mandatory at T014/T017.

## Response

Save the complete verdict before returning. Include recomputed hashes, criterion-
level source/evidence, exact commands/exits/test counts, legal positive and
discriminating negative wire-shape assessment, numeric/byte conclusion, dependency
feature conclusion, exact Clippy classification, remaining limit, processes, and
read-only confirmation.

For Revise, include requirement, exact location, reproduction, consequence, and
smallest correction. Do not fix code or acceptance fixtures. Return only the saved
response path.
