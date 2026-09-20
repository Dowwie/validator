# Build T024 nine-schema contracts and umbrella evidence

Role/model: fresh sole implementation developer, `gpt-5.6-terra`, high reasoning,
`fork_turns: none`. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #186 — Audit Validator's nine machine schemas](http://localhost:3006/1/cards/186).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t024/002-schema-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat return only the
path, SHA-256 and terse status. Do not edit governance, plans, artifact-index,
session notes, acceptance records or Fizzy. Do not delegate.

This is the first local construction boundary inside T024. It owns all nine schema
contracts plus the two umbrella filters `all_schema_contracts` and
`cli_stdout_schema_matrix`. A later fresh sole-writer context will add the exact
S/M/E case filters after this complete local handoff. Neither boundary is accepted
until the final frozen T024 candidate receives independent review. Do not begin
T025 or any later task.

Read global/repository AGENTS, the complete ratified T024 task, its full linked
Machine interface and Reports/metric reuse sections, owner prompt001, acceptance
acknowledgment001, coverage assignments, accepted T021-T023 handoff018 and current
nine schemas/tests. Derive the contract from the specifications; current output is
validation input, not normative authority.

Starting identities use exact paths:

| Artifact | SHA-256 |
|---|---|
| `docs/dispatches/validator-t024/001-owner-to-coordinator.prompt.md` | `bfe635ffb06f5ebdeda58610dec181896cb6fd4432726d0355fd583039d87269` |
| `docs/dispatches/validator-t024/001-coordinator-to-owner.response.md` | `740ca222c8b432b3df70edffd9efd13041ad5b5e683e5a209ad6e6bc5287aae8` |
| `docs/plans/validator/tasks/T024.json` | `73edfe573170afca4c842b3dfed02c23c2d09d0d7fe8ad533c9f70035aebf78d` |
| `docs/plans/validator/coverage.json` | `efe2f425fc975f953c5765d1ea40ed47ffea8f672249d85f4e3eb07c9843e896` |
| `docs/specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `docs/specs/validator-data-model.md` | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| `docs/plans/validator/execution-contract.md` | `6d46358653125b363253694c1c1bf581226888b3614d178ef401d8f06e70c76c` |
| `schemas/v2/check.schema.json` | `1bfd2fb3a644df1203b043244b38a9ed80d404cae5068901b7f162ea6dd662b9` |
| `schemas/v2/comparison.schema.json` | `8bc3b8b3c481da31725fd41cadf5a61c8a13839c77e882d3483dce9ec06d3288` |
| `schemas/v2/config.schema.json` | `c740c7cd2c1381398b3121ee161b1d995a23ceb9dd4325bd73cd0073f66d65c3` |
| `schemas/v2/error.schema.json` | `a1be234949560a90dff9391a0fa9c42de4f0431c5c4e2f20d745730f40eb80d5` |
| `schemas/v2/golden.schema.json` | `97d811a192274f1ee0703e5dff475723301fdcc68c20b031d3b22431968b777b` |
| `schemas/v2/inspection.schema.json` | `039e6060b88042c908c27a34374592d9f64434aa1aac4725da13146b01c0ac82` |
| `schemas/v2/predictions.schema.json` | `654908068da5d6b3cf9c666cddf4ebc888f94203a04fb6addd5164fbbb893352` |
| `schemas/v2/receipt.schema.json` | `d820751593f82ac68cbca2ed381b305383d96d388dee417abcefec03ed3d1c00` |
| `schemas/v2/report.schema.json` | `4078be102abd69034932d38bda2e0c9c68e98b9bf3f59aa64cef116867a543d3` |
| `tests/conformance.rs` | `486c14e2f63d2f8a05eacbdd65586e6f07ca0d2de6d8718eb788293332366cd1` |
| `tests/cli.rs` | `26b4a8aa28ebedcf9121679337700965e783e7aaddb90f50c9f56d871a5dc673` |
| `Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |

Stop and save the exact mismatch before editing. Write only the nine listed schemas
and existing `tests/conformance.rs` / `tests/cli.rs`. A demonstrated production-
output/specification conflict is not authority to weaken a schema or edit Rust:
save the exact reproduction and proposed owning correction for the coordinator.

## Nine strict contracts

Audit and minimally correct every schema as a complete Draft 2020-12 offline
machine contract:

- all nine load with pinned tooling and every `$ref` resolves locally/offline;
- root and nested required fields, closed tagged/task/status alternatives,
  cardinalities, UUID/count/digest/path forms, ratios and explicit metric
  population count/unit/scope follow the normative contract;
- finite/null/undefined/positive-infinity forms are exact, JSON uses no
  nonstandard numeric token, and selected/answered/label-decision units stay typed;
- input sources/bindings, observation definitions/values and preparation
  descriptors admit every legal task-specific combination and reject incompatible
  combinations;
- opaque input/configuration/value positions preserve arbitrary JSON, including
  null, exact large integers, `1e400` and the literal number-tag-shaped object,
  while containing records remain closed and required object shapes stay strict;
- report, comparison and inspection preserve strict single/multi discrimination,
  applied policy/signal variants, typed abstention columns, zero-exclusion
  intersection and every legal availability/status variant;
- check, receipt and error cover the actual successful/error operations with stable
  kind/status/identity/count/path/digest/code/stage forms;
- no `additionalProperties` escape exists outside explicitly opaque positions.

Reuse the existing definitions. Repair proven gaps, not formatting or a wholesale
rewrite. Preserve every accepted regression, tolerance and legal input.

## Required umbrella evidence

Implement and run nonzero:

```text
cargo test --locked --test conformance all_schema_contracts -- --nocapture
cargo test --locked --test cli cli_stdout_schema_matrix -- --nocapture
```

`all_schema_contracts` must load all nine offline; use discriminating valid
documents for every concrete alternative, then systematically delete required
root/nested fields and mutate task/status/tag/type/population/special-value/source
shapes so blanket rejection cannot pass. Map each schema/definition to its
normative clause and both positive/negative assertions.

`cli_stdout_schema_matrix` must invoke the real binary and validate one and only
one JSON stdout document for every success/error operation assigned to the current
CLI (check, evaluate receipt, compare receipt, inspect, stable errors), while
validating completed result files against report/comparison schemas. Reuse existing
helpers and fixtures; do not duplicate business logic or create an audit engine.

Also run test-list confirmation for both names, all existing schema-focused
regressions affected by edits, formatting, full locked tests, warning-denied
Clippy, locked release build and diff check. Only the accepted 17 production plus
3 duplicate lib-test staged diagnostics may remain; no new warning/suppression/
fake consumer/export widening.

Response002 must provide a nine-row schema-to-clause/evidence matrix, exact
positive and mutation coverage, commands/exits/counts, every changed hash and exact
residual lint inventory. State which assigned S/M/E filters remain for the next
local context; do not claim T024 completion or independent acceptance.
