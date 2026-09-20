# Complete only the T020 check and inspection schemas

Role/model: fresh sole implementation developer, `gpt-5.6-terra`, high
reasoning, `fork_turns: none`. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #184 — Build Validator’s concrete multi-label core](http://localhost:3006/1/cards/184).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t018-t020/014-schema-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat, return only
the response path, SHA-256 and terse status. Do not edit governance, plans,
dispatches, artifact-index, session notes, acceptance records or Fizzy. Do not
delegate.

Read `/Users/dowwie/.codex/AGENTS.md`, repository `AGENTS.md`, owner prompt012,
task T020's check/inspection contracts, relevant specification sections, current
typed check and inspection serializers, the two schemas and their current tests.
Read report-schema response013 only to preserve its completed boundary; do not
reopen that schema or its evidence.

Starting hashes:

| Artifact | SHA-256 |
|---|---|
| completed report schema | `7215342a2958fd4d9e9735c2837130a02837d205db93b342b9f64d62a882fcf1` |
| report-schema handoff013 | `4cf336a5dafe12ddb59a42f51f51f8c2211ff0a209e3589c0513a6e6547184db` |
| check schema | `42f2a7d390e53af59159d85254273fc929af25d871b28fa10d56cc415b877362` |
| inspection schema | `7cf676b2393907a4ce6079260d44935be9123a809efd8ad074e659d53c2c72e0` |
| `tests/conformance.rs` | `a08fe9ba75afefe1ff248128952aa37c6e05191b2991240566c79bbab4d79cbd` |
| `tests/cli.rs` | `c20061b8d01ff67df9bb11d99883caa300147df7fdefd6d907bf425ba1811400` |
| `src/app.rs` | `d08e50c7ab32bffa0e9791bdcb32ffa27f8c60e0ed8fc586b23d643e18588ca0` |

Stop and save the exact mismatch if any fixed hash differs before editing.

## Write scope

Own only:

- `schemas/v2/check.schema.json`;
- `schemas/v2/inspection.schema.json`;
- focused check/inspection assertions in `tests/conformance.rs` and `tests/cli.rs`.

Do not touch report schema, production Rust, fixtures, dependencies or other
schemas. Inspect current constraints first and leave correct structures unchanged.
Do not run scoring/oracle or full release gates.

## Required concrete schemas

1. Check: bind one coherent `task` and task-specific `integrity` alternative.
   Single-label requires its signal availability and normalization diagnostics;
   multi-label requires its marginal availability and must reject single-label
   normalization/confidence-only fields. Close root/task/integrity objects, require
   exact count/ID fields, enforce UUID uniqueness and reject mixed task families,
   missing required fields and foreign nested keys.
2. Inspection: bind expected target, original prediction outcome/probability/
   observation/report-confidence fields, final outcome and exposed probability/
   reported-confidence fields to a coherent single-label or multi-label family.
   Define legal answered and abstention branches, marginal versus categorical
   probability shapes, observations, identity and recorded configuration. Close
   all owned objects and require exact fields. Keep `input`, source/configuration
   and observation values opaque only where the specification permits; do not
   constrain or coerce their raw JSON values.
3. Preserve accepted exact payload behavior: large integer `9007199254740993`,
   `1e400`, explicit null and the literal serde-number-key object remain valid
   opaque input/configuration. No report-schema or production change.

## Focused evidence

Use real command output tests for valid single-label and multi-label check and
relocated inspection. Add a focused schema conformance filter named
`check_inspection_schema_contract` if the existing static assertions cannot
isolate these schemas. Prove at least:

- missing task/integrity/identity/target/outcome/probability fields reject;
- extra owned nested fields reject;
- single and multi task/output/probability families cannot mix;
- answered versus abstained outcome shapes cannot mix;
- opaque input/configuration variants above remain valid without conversion.

Run only the local evidence:

```text
python3 -m json.tool schemas/v2/check.schema.json
python3 -m json.tool schemas/v2/inspection.schema.json
cargo test --locked --test conformance check_inspection_schema_contract -- --nocapture
cargo test --locked --test cli cli_check_evaluate -- --nocapture
cargo test --locked --test cli relocated_run_inspection -- --nocapture
cargo test --locked --test cli shared_commands_multi_label -- --nocapture
cargo fmt --all -- --check
git diff --check
```

Each filtered command must select one real test. Response014 must map schema
definitions to actual serialized fields and focused mutations; record commands,
exits/counts and starting/final hashes; confirm the completed report schema and
all production files are unchanged. Do not return a progress-only sketch. If
incomplete, record the actual stop condition and smallest unfinished item for
owner reassessment.
