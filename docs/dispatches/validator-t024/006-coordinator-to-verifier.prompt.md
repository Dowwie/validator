# Independently verify frozen T024 nine-schema candidate

Role/model: fresh independent verifier, `gpt-5.6-sol`, high reasoning,
`fork_turns: none`. Do not delegate. You did not implement this candidate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #186 — Audit Validator's nine machine schemas](http://localhost:3006/1/cards/186).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t024/006-verifier-to-coordinator.response.md`.

Save the full substantive Ready/Revise/Blocked verdict there before returning. In
chat return only the path, SHA-256 and verdict. Do not edit source, schemas, tests,
fixtures, governance, plans, index, session notes, acceptance records or Fizzy.
Temporary read-only probes may live outside the repository. Do not delegate.

Reconcile frozen manifest005 SHA-256
`502f5f7e4458b11ebdb46ec0a271b3ffe8eb76dd329e2ba7c123a5ccfbbce82c`
and every inherited/current hash before review. Stop Blocked on any mismatch. Read
global/repository AGENTS, complete T024, owner prompt001, both full developer
handoffs, the full Machine interface and Reports/metric reuse specification
sections, data-model contract, and every T024-owned coverage row before treating
the implementation summary as evidence. T025 and later remain outside scope.

## Independent schema review

Inspect all nine schemas as normative machine contracts, deriving required shapes
from the specification rather than current output:

- compile/load all nine under pinned Draft 2020-12 tooling with local/offline
  references only; no `$id`/registry framework is required for self-contained
  local `$defs`;
- check root/nested required fields, closed task/status/tag/type alternatives,
  UUID/count/digest/path/cardinality forms, ratios and explicit metric population
  count/unit/scope;
- check exact finite/null/undefined/positive-infinity forms, typed abstention
  columns, both task families, applied policies, signal/probability availability,
  zero-exclusion intersection and all legal operation/status variants;
- check sources/bindings, observation definitions/values and preparation
  descriptors. Configuration stays an opaque object whose member values may be
  arbitrary JSON, including null, exact large integer, `1e400` and literal number-
  tag-shaped objects; containing records remain constrained;
- search for any `additionalProperties` escape outside explicit opaque positions;
- validate discriminating legal documents first, then remove required root/nested
  fields and mutate task/status/type/special-value/population/source combinations.
  Blanket rejection or implementation-shaped examples do not prove correctness.

Trace the real binary stdout matrix for check, evaluate receipt and report,
inspection, compare receipt and comparison, stable command error and output-exists
error. Confirm exactly one JSON stdout document and exact schema selection.

## Assigned case review

Independently inspect every compound assertion in S13/S14/S17/S21/S29, M16/M21
and E05/E06/E09-E13. Confirm each filter proves all subcases from its exact
normative row through public synthetic inputs, including no publication on
failure, precise diagnostic/status category, retained values, populations and no
invented inference/conversion. In particular distinguish legal unreviewed
reference sets from forbidden wire shapes and preserve the existing provenance vs
observation diagnostic boundaries.

Rerun every exact filter nonzero:

```text
cargo test --locked --test conformance all_schema_contracts -- --nocapture
cargo test --locked --test cli cli_stdout_schema_matrix -- --nocapture
cargo test --locked --test conformance case_s13 -- --nocapture
cargo test --locked --test conformance case_s14 -- --nocapture
cargo test --locked --test conformance case_s17 -- --nocapture
cargo test --locked --test conformance case_s21 -- --nocapture
cargo test --locked --test conformance case_s29 -- --nocapture
cargo test --locked --test conformance case_m16 -- --nocapture
cargo test --locked --test conformance case_m21 -- --nocapture
cargo test --locked --test conformance case_e05 -- --nocapture
cargo test --locked --test conformance case_e06 -- --nocapture
cargo test --locked --test conformance case_e09 -- --nocapture
cargo test --locked --test conformance case_e10 -- --nocapture
cargo test --locked --test conformance case_e11 -- --nocapture
cargo test --locked --test conformance case_e12 -- --nocapture
cargo test --locked --test conformance case_e13 -- --nocapture
```

Run one locked full suite and current warning-denied Clippy. Confirm 44 unit + 36
conformance + 10 CLI and exactly the accepted 17 production plus 3 duplicate
lib-test staged diagnostics with no T024 warning. Reuse developer fmt/release/diff
evidence unless a concrete finding justifies rerun.

Return **Ready** only if manifest, every schema alternative, both umbrellas, all
fourteen compound rows and scoped gates satisfy T024. Return **Revise** for each
reproducible current-contract defect with requirement, file/line, exact mutation or
input, consequence and smallest correction. Return **Blocked** only when a frozen-
candidate/evidence condition prevents judgment. Do not request optional metadata,
cosmetic rewrite, duplicate assertion across layers, generic audit tooling,
numerical reimplementation, private-data work or future-task checks. Stop once the
bounded evidence supports one verdict.
