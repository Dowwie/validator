# T020 check and inspection schema handoff

Status: complete local schema and authorized inspection-replay repair handoff.
This is not T020 acceptance or a combined-review request.

## Input identity

All seven prompt014 fixed hashes matched before schema edits. Prompt017 later
reconciled the prompt016 conformance boundary to the focused prompt014 test
addition made in this worker context; no other writer was active. The completed
report schema remains byte-identical to the accepted prompt013 artifact:
`7215342a2958fd4d9e9735c2837130a02837d205db93b342b9f64d62a882fcf1`.

## Schema contract mapping

| Schema definition | Actual serialized contract and focused discrimination |
|---|---|
| Check root, `task`, `singleIntegrity` | The real single-label `check` serializer emits a strict single-label task, counts/UUID arrays, signal availability `none`, `categorical`, `confidence`, or `both`, plus required `normalized_count` and `maximum_sum_error`. |
| Check `multiIntegrity` | The real multi-label serializer emits only `none` or `marginals`; `normalized_count` and `maximum_sum_error` are forbidden. Root task selection prevents the two integrity families from mixing. |
| Inspection identity and targets | Required closed identity, episode UUID, expected class/label-set target, original prediction identity/source/outcome, final outcome, observations, probability, reported confidence, and configuration map to `InspectionResult` exactly. `input` and full configuration remain unconstrained raw JSON. |
| Inspection single-label branch | Class or abstention prediction/final outcomes, categorical original probabilities, optional confidence, and categorical report probability evidence are required in their actual closed forms. |
| Inspection multi-label branch | Label-set or abstention prediction, label-marginal original probabilities, exact matched/missed/extra/correct answered outcome, marginal report evidence, and `reported_confidence: null` are required. Single-label confidence and categorical evidence are rejected. |
| Observations and opaque values | Named observation records have closed scalar, bounded Bernoulli/reported-confidence, or vector forms. The real relocated single-label command confirms the raw input literal `9007199254740993`, `1e400`, explicit null, and literal serde-number-key object pass unchanged. |

`check_inspection_schema_contract` validates legal single/multi documents and
rejects missing task, integrity, identity, target, prediction outcome, and
probability fields; foreign nested prediction fields; single/multi family mixing;
answered/abstention mixing; multi-label confidence; and single-label diagnostic
fields on multi-label integrity.

## Stored-evidence repair

The required relocated inspection initially failed before schema validation with
`E_IO`: rebuilding a stored report tried to resolve original source evidence
paths. Owner-authorized prompt016 changes `src/app.rs` to reuse
`StoredRun::verify_evidence` in both concrete rebuild branches, and passes the
verified contained bindings into the shared report rebuild. The real multi-label
CLI test now removes originals after relocation and proves a mutation of
contained `evidence/0.bin` fails with `E_PROVENANCE` at replay.

## Verification

| Command | Exit | Actual result |
|---|---:|---|
| `python3 -m json.tool schemas/v2/check.schema.json` | 0 | valid JSON document. |
| `python3 -m json.tool schemas/v2/inspection.schema.json` | 0 | valid JSON document. |
| `cargo test --locked --test conformance check_inspection_schema_contract -- --nocapture` | 0 | 1 selected, 1 passed. |
| `cargo test --locked --test cli cli_check_evaluate -- --nocapture` | 0 | 1 selected, 1 passed. |
| `cargo test --locked --test cli relocated_run_inspection -- --nocapture` | 0 | 1 selected, 1 passed after the stored-binding repair. |
| `cargo test --locked --test cli shared_commands_multi_label -- --nocapture` | 0 | 1 selected, 1 passed, including relocated evidence success and tamper rejection. |
| `cargo test --locked --test conformance replay_binding_and_result_tampering -- --nocapture` | 0 | 1 selected, 1 passed. |
| `cargo fmt --all -- --check` | 0 | passed. |
| `git diff --check` | 0 | passed. |

No scoring/oracle or full release gates ran. Cargo's pre-existing staged
dead-code warnings remained; no suppression or new warning class was added.

## Final identity and scope

| Artifact | SHA-256 |
|---|---|
| `schemas/v2/check.schema.json` | `1bfd2fb3a644df1203b043244b38a9ed80d404cae5068901b7f162ea6dd662b9` |
| `schemas/v2/inspection.schema.json` | `3c32ad02a1376b790294bcd655c8c4ffc711a4d90274546303e60577425fc176` |
| `tests/conformance.rs` | `15507426e673eea44aa04616b3ce8601785f270308cba113ec72d7ad9ba20ff6` |
| `tests/cli.rs` | `c23e38a343fb8eaccb1b784bbdf24cd15c0c4f50f081e461cf70af7add399999` |
| `src/app.rs` | `b8f1b5b32bdff042eb5bca3b667da6fccdeb793bec04bb3b35c9bd7d4ce47b1b` |
| `schemas/v2/report.schema.json` | `7215342a2958fd4d9e9735c2837130a02837d205db93b342b9f64d62a882fcf1` |

Only the two owned schemas, focused conformance/CLI assertions, and the
owner-authorized minimal stored-evidence inspection repair changed. Report
schema, artifacts, fixtures, dependencies, governance, plans, artifact index,
session notes, acceptance records, Fizzy, and T021/T022 behavior remain unchanged.
