# Implement T026 output 1: artifact and publication safety

Role/model: fresh sole developer, `gpt-5.6-terra`, high reasoning,
`fork_turns: none`. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #188 — Verify Validator artifact integrity](http://localhost:3006/1/cards/188).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t026/002-safety-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat return only the
path, SHA-256 and terse status. Do not edit governance, plans, artifact-index,
session notes, acceptance records or Fizzy. Do not delegate.

This is T026 local output 1 of 2. Read global/repository AGENTS, the Rust
best-practices skill at
`/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md`, complete T026, owner
prompt001, ratified evidence-binding/replay/machine-interface/verification sections
and coverage rows S23-S28 before editing. Preserve all accepted T025 behavior and
evidence. Do not begin S15/S16/S18-S20/S22, M17-M19, E08, combined final gates,
independent review, T027 or later work.

## Write and correction boundary

Primary write scope is `tests/conformance.rs`, `tests/cli.rs`, and owning test
sections in `src/artifacts.rs`. Read the current owning code before changes.
Production edits in existing `src/artifacts.rs` or `src/app.rs` are permitted only
after first reproducing a concrete ratified T026 integrity/replay/publication
violation and recording exact input, expected behavior and actual behavior. Make
only the smallest owning-layer correction. Any schema, dependency, other source
file, public API or contract change must stop for coordinator/owner review.

Do not build a generic adversarial framework, alternate publisher, timing-only
race, fallback or state engine. Reuse existing real API/CLI fixtures, stored-run
helpers, no-replace publisher and deterministic owning-module seams. Use isolated
temporary paths and mutate only test-owned runs.

Starting identities:

| Artifact | SHA-256 |
|---|---|
| `001-owner-to-coordinator.prompt.md` | `e9d4036002883fef4d94973d5f4519e5729b67afc26b3854bfbb6fbbae880115` |
| `001-coordinator-to-owner.response.md` | `eaa064ddc9322cf11079534207dd319301e3aae2ad9c1c4b5c1a1d45c29da888` |
| `docs/plans/validator/tasks/T026.json` | `d1b1388fabce031a6a84020a72eb069cef42d3aa86c5dc051a6d3d21b8648696` |
| `docs/specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `docs/specs/validator-data-model.md` | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| `src/artifacts.rs` | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |
| `src/app.rs` | `f23a95f6c6f4d0f6c33cd872567ef14526243f053673a973ee0021a94d79012a` |
| `tests/conformance.rs` | `13409645624e53b8a3f2b4e5719dd7ae651527e0c4f886dcb511bfa10bf38bef` |
| `tests/cli.rs` | `8c7b9c00d777f4a290911dd77199d8e1b9e085cc14fb0dd78a801108c3a10fcf` |
| `src/evaluation/multi_label.rs` | `c4158facd454e66e66b3f1ddc0bb9976ca07fca6961bf200597ba9b31107b461` |
| `Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |

## Required matrices

Implement listed, executing nonzero `artifact_adversarial_matrix` in conformance
and `publication_and_privacy_matrix` in the real CLI target. Reuse complete exact
case helpers rather than copying workflows. Across both task kinds, the two
matrices and exact cases collectively must discriminate:

- copied-evidence-only replay after relocation and original-tree removal, with
  same-basename files from different directories, a parent-relative original
  path, repeated paths, UTF-8 byte-ordered source IDs and per-source index order;
  original provenance strings are never dereferenced;
- each individual corruption: golden/predictions/config snapshot digest or bytes,
  stored result, missing/extra/duplicate/swapped `(source_id,evidence_index)`
  binding, wrong original string, wrong ordinal stored path, changed evidence
  byte, escaping stored symlink, and report source-array disagreement. Inspection
  and comparison reject through typed public paths, and no comparison directory
  is published;
- exact structural/count/status/recorded-configuration replay. For each task kind,
  a valid computed result float changed within declared replay tolerance remains
  acceptable, while a count or recorded-configuration mutation rejects. Identify
  computed float fields by complete paths; do not use tail-name/dynamic discovery
  or apply tolerance to recorded data;
- existing file, directory and symlink destinations survive byte-for-byte, plus a
  deterministic concurrent-creation seam that proves no replacement. A genuine
  late failure leaves no final success result or temporary sibling. Use the one
  shared production publisher if a demonstrated defect needs correction;
- routine reports, receipts, stdout, stderr and error JSON omit a unique opaque
  input sentinel and arbitrary evidence-byte sentinel. Explicit selected-episode
  inspection alone returns the exact stored input. Receipts identify the actual
  completed result path and independently hash its exact bytes;
- actual binary exit/code/stage/path categories for filesystem, validation,
  provenance/replay and output-exists failures. Do not infer process exits from
  library error categories where this output requires CLI evidence.

## Exact S23-S28 filters

Each exact filter must call the real public API or built binary, list separately,
execute one nonzero test, and cover its complete normative row:

- `case_s23`: existing file, directory and symlink evaluation/comparison
  destinations remain unchanged with exit 3/`E_OUTPUT_EXISTS`; a late validation
  failure publishes no final result; a damaged stored snapshot is rejected through
  the real replay path with the precise category and no finalized comparison.
- `case_s24`: relocate a complete run with distinct same-basename evidence,
  including parent-relative and repeated original paths; remove/unavailable the
  originals; verified inspect and compare succeed entirely from copied evidence.
  Cover both task kinds collectively with the matrices.
- `case_s25`: execute every binding/digest/path/source-array/symlink corruption
  listed above one at a time. Assert typed integrity failure, containment, no
  original-path lookup and no successful comparison publication.
- `case_s26`: for successful real evaluation and comparison receipts, assert
  absolute `result_path` names actual `report.json`/`comparison.json`, and an
  independent SHA-256 of exact file bytes equals `result_sha256`.
- `case_s27`: assert every routine report/stdout/stderr/error/receipt omits the
  unique opaque-input and evidence-content sentinels; explicit verified inspection
  returns the selected stored input exactly.
- `case_s28`: the inspected opaque input preserves exact integer
  `9007199254740993` as JSON with no binary64 rounding or string substitution.

## Local completion boundary

Run each exact filter and matrix independently, then full affected targets:

```text
cargo test --locked --test conformance artifact_adversarial_matrix -- --nocapture
cargo test --locked --test cli publication_and_privacy_matrix -- --nocapture
cargo test --locked --test conformance case_s23 -- --nocapture
cargo test --locked --test conformance case_s24 -- --nocapture
cargo test --locked --test conformance case_s25 -- --nocapture
cargo test --locked --test conformance case_s26 -- --nocapture
cargo test --locked --test conformance case_s27 -- --nocapture
cargo test --locked --test conformance case_s28 -- --nocapture
cargo test --locked --test conformance -- --list
cargo test --locked --test cli -- --list
cargo test --locked --test conformance
cargo test --locked --test cli
cargo fmt --all -- --check
git diff --check
```

Run any focused owning-module filter if `src/artifacts.rs` tests change. Do not run
Clippy, full all-feature tests or release by habit at this local boundary; output2
owns final gates. The 15-minute milestone is the first coherent matrix/exact-case
block or an exact blocker.

Response002 must map every matrix dimension and each S23-S28 subcase to actual
public assertions, list commands/exits/counts, disclose any reproduced production
defect before its smallest correction, and provide exact changed/unchanged hashes.
Return only when this complete local output is ready, or with a frozen material
contract/scope discrepancy. Do not claim T026 complete.
