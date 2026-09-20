# Validator T013 typed report and output-schema dispatch 025

Role/model: retained sole developer `/root/coordinator/schema_test_developer`,
`gpt-5.6-terra`, reasoning `high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/025-schema-test-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator's verified single-label backbone](http://localhost:3006/1/cards/183).

T012 is reconciled as a complete local milestone. Complete **T013 only** next.
This remains inside the combined T007-T014 candidate; there is no independent
review or acceptance at this unit. You remain the sole implementation/test writer.
Do not delegate, begin T014 wiring, create a public test facade, or implement any
T015 replay/containment behavior.

## Required context and frozen input

Read in full:

- `/Users/dowwie/.codex/AGENTS.md`, repository `AGENTS.md`, and the already-read
  manage-dev-team and Rust best-practices skills;
- `docs/plans/validator/tasks/T013.json`;
- `docs/specs/validator-v1.md` sections **CLI and run artifacts** and **Machine
  interface**, plus its exact metric definitions referenced by the report fields;
- `docs/specs/validator-data-model.md` section **Reports and metric reuse**;
- `docs/dispatches/validator-build/004-owner-to-coordinator.prompt.md`, especially
  the rule that named application integration evidence waits for T014's real API;
- `docs/dispatches/validator-t007-t014/002-owner-to-coordinator.prompt.md`, which
  fixes the five-entry run layout and keeps manifest metadata inside `report.json`.

The complete T012 handoff is
`docs/dispatches/validator-t007-t014/024-schema-test-developer-to-coordinator.response.md`,
SHA-256 `d264550555d10186fcce1e966f902a2385b5468158aff4f2220d0d0328b29982`.
Confirm these exact inputs before editing:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `366bef64d9ac1e898ca6fcf1559c7ac744b671068fc3383d8c374530c5242bd8` |
| `Cargo.lock` | `33165fd692aaa3355ad66c391d31236da8f4b9a302704c3228b699ee7958c4ca` |
| `src/lib.rs` | `a61426bfb002b8b5634692e7e6f321f626e3d4d73ec1fabb90e0564c657a9a9c` |
| `src/model/common.rs` | `b227e49cff07d09923534bbee5fd59ecdd23541c908cbae86fd5f120a37722d1` |
| `src/model/single_label.rs` | `93a9ba28d911841a8d89249ce19846cb001d8dda217961ec95faa94043b22343` |
| `src/evaluation/single_label.rs` | `d631caff112d4cea8ec74be14da0cefe067226f23f7f600f0ce5d3b172613b33` |
| `src/artifacts.rs` | `14a7383a204c88a7922dedc6159d2b7ad4819b395bab3a8c2185e83ad39aef61` |
| `tests/conformance.rs` | `99b0a0eda61a8294a8f3489c019eb3827f919e36abb296ba83c77644176e18e7` |

Preserve the 38-test library and one-test input-schema conformance baselines. The
repository is unborn/untracked. Do not stage, commit, clean, reset, modify
`.zvec-grep`, edit governance/index/Fizzy/session records, or start/stop caffeinate
PID 84732.

## Typed report contract

Add the smallest typed serialization layer that converts the checked T008/T009
`SingleLabelResults`, checked population/evaluation metadata, and T011
snapshot/evidence bindings into the normative report. The caller supplies the
already-generated UUIDv7 `run_id`, UTC RFC 3339 `created_at`, Validator version,
specification version, and exact artifact digests; report assembly must not fetch
time, generate identity, reread files, score rows, or publish directories.

The serialized report must be a closed `schema_version: 2`, `kind: "evaluation"`,
`status: "complete"` envelope with every required top-level field from the
Machine interface table:

- identity, including optional parent run ID;
- artifact manifest entries for exact dataset/predictions/config snapshots and
  every evidence binding, with kind, stored relative path, digest, and the
  evidence source/index/original-path binding required by the specification;
- source definitions keyed by source ID, including opaque configuration and
  preparation metadata, with evidence references rewritten to bound stored paths;
  never expose or infer source files outside those bindings;
- `empty`, `single_source`, or `mixed_source` composition and a source-count entry
  for every declared source, including zero-count unused sources;
- the exact population description/role/counts/sorted selected and unselected IDs,
  task kind and vocabulary order, effective as-recorded policy/numerical/binning
  settings, and complete integrity/signal/normalization diagnostics;
- task-tagged `raw`, `final`, `probability`, and `signals` objects using the exact
  metric keys and required statuses/accounting already produced by T007-T009;
- sorted single-label episode records with source, expected/raw/final outcomes,
  correctness, rejection reason, observations, and available submitted/working
  probabilities, argmax, maximum/chosen probability, disagreement, and reported
  confidence. Evidence-bearing abstentions retain their probability/observation
  fields. Required unavailable families stay present as `not_applicable`.

Every metric retains `population_count`, `population_unit`, and
`population_scope`; ratios retain numerator and denominator. Matrix columns are
typed label/abstention objects so a literal label named `ABSTAIN` is unambiguous.
Preserve opaque configuration exactly where permitted. Routine report JSON must
contain no golden episode `input`, raw provider payload, arbitrary evidence-file
contents, acceptance verdict, prose-dependent control field, or nonstandard JSON
number. Do not clone raw inputs into report rows.

Use typed Rust structures and `serde` serialization, not hand-built untyped JSON
maps for the report. Add only the narrow internal accessors/conversion helpers
needed to consume already-checked fields. Keep report assembly out of
`artifacts.rs`; it calculates no metrics and continues to accept complete bytes.

## Output schemas

Create strict Draft 2020-12 schemas at these exact paths:

- `schemas/v2/report.schema.json`
- `schemas/v2/check.schema.json`
- `schemas/v2/receipt.schema.json`
- `schemas/v2/error.schema.json`

`report.schema.json` must close the full envelope and select the concrete
single-label report alternative while leaving the planned multi-label alternative
as an actual schema branch only when it can be stated completely; do not add a
placeholder permissive branch. `raw`, `final`, `probability`, and `signals` must
carry `kind: "single_label"`. The report schema must enforce required fields,
typed metric/status/population/ratio records, typed matrix columns, evidence
bindings, source/count/population structures, and episode evidence.

The other three schemas must implement their present normative machine contracts:

- `check`: `schema_version`, `kind: "check"`, `status: "complete"`, integrity
  counts, and selected IDs;
- `receipt`: the closed successful evaluation/comparison alternatives with matching
  `kind`, the appropriate UUID identity, absolute `result_path`, and exact-file
  `result_sha256`;
- `error`: `schema_version`, `kind: "error"`, `status: "error"`, stable diagnostic
  code and safe structured diagnostic fields allowed by the existing error model,
  without opaque payload/raw sensitive values.

Use local `$defs`/relative references consistent with the T010 schemas and exact
Draft 2020-12 meta-schema URI. Do not add a new schema dependency.

## Test boundary and required evidence

Add a nonzero direct-schema conformance filter:

```sh
cargo test --locked --test conformance single_report_schema -- --nocapture
```

It must compile every new schema with the locked schema engine and validate at
least complete, hard-label-only, empty-population, and evidence-bearing-abstention
single-label reports. Include discriminating invalid mutations for missing required
top-level data, wrong kind/status/task tags, malformed metric population/ratio,
ambiguous matrix columns, malformed evidence bindings, and forbidden routine
input/raw-evidence content. Also compile and exercise representative valid/invalid
check, receipt, and error documents rather than merely asserting files exist.

Add a same-module owning unit filter named `report_sources_and_privacy` that
constructs checked data/results and proves report assembly has correct mixed-source
composition, one selected count per contributing source, zero for an unused source,
bound stored evidence-path rewrites, canonical episode/ID order, complete metric
families/statuses, and absence of a unique opaque-input/evidence sentinel from the
serialized report. Include empty and evidence-bearing abstention cases.

Do **not** make the task's eventual integration command
`cargo test --locked --test conformance report_sources_and_privacy -- --nocapture`
appear to pass through a static fixture, private source inclusion, new public
export, or test-only facade. Per the owner decision, T014 must add that named
integration case through the real small application API. Record the passing T013
unit filter and this one explicit deferred integration filter in the response;
this is the expected coherent-sequence boundary, not an unresolved design issue.

Run:

```sh
cargo fmt --all -- --check
cargo test --locked --lib report_sources_and_privacy -- --nocapture
cargo test --locked --test conformance single_report_schema -- --nocapture
cargo check --locked
cargo test --locked --lib
cargo test --locked --test conformance input_schema_contract -- --nocapture
git diff --check
```

Do not run full Clippy/all-target/release gates by habit; T014 owns them. Disclose
incomplete-consumer warnings exactly without suppressing them.

## Allowed writes

- `src/model/common.rs` for shared report/manifest/identity/check/receipt types;
- `src/model/single_label.rs` for the concrete typed report and private assembly;
- `src/evaluation/single_label.rs` only for minimal internal checked-result access
  that the report conversion actually needs;
- `schemas/v2/report.schema.json`, `schemas/v2/check.schema.json`,
  `schemas/v2/receipt.schema.json`, and `schemas/v2/error.schema.json`;
- `tests/conformance.rs` for the direct schema contract only;
- minimal existing module declaration/import changes if compilation requires them;
- the required response file.

Do not modify validation behavior, scoring formulas, T010 input schemas, T011/T012
artifact behavior, fixtures/oracles, application/CLI code, specifications/plans/
session/index/Fizzy, Cargo dependencies, multi-label implementation, comparison,
inspection, replay, or T014+ behavior.

## Required response

Continue until T013 typed assembly, all four strict schemas, the direct schema
matrix, the owning source/privacy unit, format, locked compile, and regressions are
complete. Save the response before returning. Include exact input/output hashes,
criterion-to-type/schema/test locations, every command/exit/nonzero count, the
explicit T014-only integration-filter deferral, warning status, and unresolved
failures. Return early only for a concrete reproduced blocker with its exact
command and smallest decision.
