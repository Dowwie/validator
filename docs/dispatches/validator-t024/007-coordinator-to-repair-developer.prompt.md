# Repair T024 comparison/report/receipt schema completeness

Role/model/context: return the original T024 schema developer as the sole repair
writer, `gpt-5.6-terra`, high reasoning. Do not delegate. This is the one
finding-driven repair cycle authorized by owner prompt001.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #186 — Audit Validator's nine machine schemas](http://localhost:3006/1/cards/186).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t024/007-repair-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat return only the
path, SHA-256 and terse status. Do not edit governance, plans, artifact-index,
session notes, acceptance records or Fizzy. Do not delegate.

Read frozen manifest005 and the complete independent Revise verdict006 SHA-256
`cf10fef91f3c58af5b9b8e924a53d099143faf3a060cc820e30a6c874d77db52`.
Reconcile every frozen hash before editing. Only the single demonstrated T024
schema-completeness finding is in scope. All fourteen assigned rows, both umbrella
paths and other schemas are accepted for this repair boundary. Do not begin T025.

Write only:

- `schemas/v2/comparison.schema.json`;
- `schemas/v2/report.schema.json`;
- `schemas/v2/receipt.schema.json`;
- `tests/conformance.rs` for focused mutation evidence.

Production Rust, fixtures, CLI tests, other schemas and dependencies remain
read-only. Stop and report any production/spec mismatch or second material defect
before further correction.

## Required smallest correction

Derive exact shapes from the normative specification and existing serialized Rust
types. Do not use current valid output as the only authority.

1. In the comparison schema, replace non-opaque `true`, untyped objects and untyped
   arrays with closed definitions for side source records, nonnegative source
   counts, concrete single/multi hard counts, class/macro records, transition rows,
   probability counts and multi-label episode label sets. Preserve local source
   IDs while typing their definition values. Reject blank source/count keys where
   contract strings are nonblank.
2. Restrict scalar `metric.population_scope` to `selected` or `answered` in report
   and comparison. Keep `raw_answered` only in the explicit bin-population
   definitions that normatively use it.
3. In the report schema, keep source/preparation `configuration` as an opaque
   **object**: the root must be an object while its member values remain arbitrary
   JSON. Preserve arbitrary golden `input`. Constrain stored manifest `path` to the
   required relative run-artifact forms; reject absolute/traversal paths without
   rejecting legal evidence ordinals or named snapshots.
4. Discriminate receipt `result_path` by operation while retaining its absolute
   path contract: evaluation must end in `report.json`, comparison must end in
   `comparison.json`. Preserve exact digest and identity constraints.
5. Add focused positive and one-at-a-time mutation assertions for every repaired
   position to `all_schema_contracts` or an existing focused schema helper. At
   minimum prove rejection of every mutant listed in verdict006: arbitrary/blank
   sources, string source count/hard count, null class/transition, invented macro,
   boolean probability count, untyped multi episode set, scalar
   `raw_answered`, null source/preparation configuration, absolute/traversal stored
   artifact path, and wrong evaluation/comparison receipt filename. Include
   discriminating legal neighbors so blanket rejection cannot pass.

Do not add `$id`, external resolver/registry machinery, generic schema generation,
duplicate CLI assertions, cosmetic rewrite or unrelated strictness work. Preserve
zero-exclusion intersection, task-family discrimination, opaque member edge values
and all existing legal status alternatives.

## Checks and handoff

Run:

```text
cargo test --locked --test conformance all_schema_contracts -- --nocapture
cargo test --locked --test cli cli_stdout_schema_matrix -- --nocapture
cargo test --locked --test conformance -- --nocapture
cargo test --locked --test cli -- --nocapture
cargo fmt --all -- --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --release --locked --bin validator
git diff --check
```

The accepted 17 production plus 3 duplicate lib-test staged diagnostics may remain;
no repair-owned warning is allowed. Response007 must map each verdict mutant to its
new definition/assertion, list legal neighbors, commands/exits/counts, exact changed
hashes and residual lint inventory. On complete evidence the coordinator will
freeze a repaired delta for the same verifier's focused recheck; no new broad review.
