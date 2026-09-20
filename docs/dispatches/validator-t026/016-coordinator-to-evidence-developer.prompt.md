# Correct the resolved selected source in public inspection

Role/model: retained sole T026 developer, `gpt-5.6-terra`, high reasoning,
original fresh `fork_turns: none` context. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #188 — Verify Validator artifact integrity](http://localhost:3006/1/cards/188).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t026/016-evidence-developer-to-coordinator.response.md`.

Save the full substantive handoff before returning only its path, SHA-256 and
terse status. T028's source writer has yielded and its protected bundle is
read-only; you are the sole code writer. Do not edit governance, plans, index,
acceptance/session records or Fizzy. Do not start T027/T028 review or T029+.

Read `/Users/dowwie/.codex/AGENTS.md`, repository `AGENTS.md`, owner prompt014,
verifier addendum015 and this complete prompt. The owner has authorized this one
required source-field/schema correction after the saved finding. Preserve the
other five accepted repairs and all settled T026/T025 evidence.

Starting identities:

| Artifact | SHA-256 |
|---|---|
| `014-owner-to-coordinator.prompt.md` | `a19fed980f87ee4c100029b3b7d636ee84bba0548704727067819c77a6d5a216` |
| `015-verifier-to-coordinator.response.md` | `5721fb83161c839005056236129e4b681169e5c903b081ff69006e403c2e645f` |
| `src/app.rs` | `007c00ac10a4db21f19f7f21cdbd3861c9581fed03c864bc59debd514d54d800` |
| `schemas/v2/inspection.schema.json` | `2c7e064d0adb915b77c558a7ebbfade96f7bcf0eee890b881ced1383f0eedaef` |
| `tests/conformance.rs` | `a685073e0e136f0f16167594833c58ef18dc024d6733a9f1d5014d665b59950d` |
| `tests/cli.rs` | `e0bf26cd96c2c06a221572c748e20818e66638e7227cc789e2f62705acca9197` |
| `tasks/T026.json` | `6a5715dfae5a84763523079da19077a8f696383ad0d616abc150a69d9ddf9032` |
| `tasks/T027.json` | `4c58adee793a3a5278394aaa76dbb0b985a46f1e3ed72ad9947a7b0cd79a052d` |
| `physical-map.json` | `98dab85533b199862bebbb00894eb81f1216d9e8a255b52dc715cdab613168bf` |

## Exact correction

Write only `src/app.rs`, `schemas/v2/inspection.schema.json`, focused
`tests/conformance.rs` and `tests/cli.rs` as required.

1. Add a required `source` field to `InspectionResult`. Its value is exactly the
   full `SourceDefinition` resolved from the selected raw prediction's
   `source_id` in the already verified stored report's source map.
2. Preserve `prediction`, original opaque input, evaluation `configuration` and
   every other output field unchanged. The new source's own configuration is not
   a replacement for evaluation configuration.
3. Reuse the existing verified/validated source serialization and field layout.
   Include model, legal task-specific kind/configuration and optional question,
   observation definitions, preparation and evidence fields when present. Use
   verified stored evidence paths. Never dereference original source paths or
   include unrelated sources/evidence-file contents.
4. Synchronize the strict inspection schema with the existing source-definition
   contract for both legal task kinds. Do not add a registry, duplicate source
   validation, new public SDK, fallback, protocol version or unrelated schema
   rewrite.
5. Make S15 assert both inspected complete Q1/Q2 source definitions from fixed
   inputs, while preserving the already accepted comparison-side definitions.
6. Extend E08 or the smallest existing source-observation case to prove inspected
   observation definitions, preparation and stored evidence bindings survive.
   Prove both task kinds through real inspect and the published schema. Retain
   relocation, privacy and all current output fields.

Expected values must come from fixed inputs or the verified stored binding
contract. If the existing verified report cannot resolve exactly one selected
source, or any current-contract discrepancy appears beyond this correction,
freeze exact input/expected/actual evidence and stop for owner reassessment.

## Checks

Run each changed owning filter independently, relevant inspection/schema/CLI,
relocation and privacy regressions, then one literal final boundary:

```text
cargo fmt --all -- --check
cargo test --all-features --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo build --release --locked
git diff --check
ruby docs/plans/validator/verify-plan.rb
```

All tests must pass. Clippy may contain only the unchanged accepted 17 production
plus three duplicate lib-test staged diagnostics. Do not suppress warnings,
weaken schemas/oracles, create fake consumers or add cleanup work.

Response016 must map the exact field/schema/test changes to addendum015, list
every command/exit/count, record the staged Clippy inventory and freeze exact
changed/unchanged hashes. Return only when this candidate is ready for the same
verifier's focused R1/source-field recheck or a concrete blocker is preserved.
