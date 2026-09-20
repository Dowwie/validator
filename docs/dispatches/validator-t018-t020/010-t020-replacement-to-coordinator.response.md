# T020 replacement handoff

Status: incomplete; do not freeze or send for independent review.

## Frozen inputs

All governing hashes named in prompt010 matched before edits, including the v1 specification, data model, execution contract, T020 task, response008, T019 oracle, current production files, schemas, and the two integration-test files.

## Work completed

- Rebuilt the check schema with closed top-level, task, and integrity objects, concrete task alternatives, UUID/count constraints, and the legal signal variants.
- Rebuilt the inspection schema with closed root, identity, expected target, prediction, and outcome alternatives while retaining opaque input/configuration values.
- Replaced the report schema after the prior simplified document. The current report schema parses and validates real multi-label reports, but it is still incomplete: its single-label hard branch does not yet bind all required metric and matrix fields, so it does not preserve the accepted single-label negative contract.

No production code, tests, task contracts, plans, governance, Fizzy, artifact index, session notes, acceptance records, dependencies, or T021/T022 behavior was changed.

## Focused evidence

| Command | Exit | Actual result |
|---|---:|---|
| `python3 -m json.tool schemas/v2/report.schema.json` | 0 | report schema parses |
| `cargo test --locked --test conformance multi_label_hard_oracles -- --nocapture` | 0 | 1 selected, 1 passed |
| `cargo test --locked --test conformance marginal_loss_and_bins -- --nocapture` | 0 | 1 selected, 1 passed |
| `cargo test --locked --test cli shared_commands_multi_label -- --nocapture` | 0 | 1 selected, 1 passed |
| `cargo test --locked --test conformance single_report_schema -- --nocapture` | 101 | negative mutation `raw.accuracy` without `population_unit` remains accepted by the incomplete report hard branch |
| `git diff --check` | 0 | passed |

The attempted grouped `cargo test` invocation was rejected by Cargo because it accepts one filter at a time; no behavior was inferred from it.

## Exact remaining work

1. Complete the strict report schema, particularly concrete single/multi hard alternatives, all metric/null/special-value branches, closed nested objects, and task-family discrimination; restore all `single_report_schema` negative assertions.
2. Expand the existing public conformance bodies for the full two-row, empty-answer, abstention, all-abstained, no-data, equal-binary-count, marginal endpoint/bin, and raw/final evidence oracles specified in prompt010.
3. Add the remaining CLI opaque-payload/privacy/one-document/output-refusal assertions.
4. Run the six required nonzero filters and all mandatory fmt, Clippy, full-test, release-build, and diff gates after the contract is complete.

## Current hashes

| Artifact | SHA-256 |
|---|---|
| `schemas/v2/check.schema.json` | `42f2a7d390e53af59159d85254273fc929af25d871b28fa10d56cc415b877362` |
| `schemas/v2/report.schema.json` | `ec1e649325b7e621f16a471989b2a956fdfc1d6fab8a83263e2351b3ae904ecf` |
| `schemas/v2/inspection.schema.json` | `7cf676b2393907a4ce6079260d44935be9123a809efd8ad074e659d53c2c72e0` |
| `tests/conformance.rs` | `a18391bdfbf5a1f9aee9d6ae4a4bf1ee841bd154a76644a33192bd3f3914a07d` |
| `tests/cli.rs` | `c20061b8d01ff67df9bb11d99883caa300147df7fdefd6d907bf425ba1811400` |

The residual lint inventory was not re-run because the mandatory schema contract and its existing single-label regression are failing. No new Rust code or diagnostic class was introduced.
