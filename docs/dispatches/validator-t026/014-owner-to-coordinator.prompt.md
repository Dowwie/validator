# Resolve the remaining S15 inspection-origin gap against the actual public result

Existing Sol-high coordinator; repository `/Users/dowwie/MyProjects/validator`.
Save acknowledgment to `docs/dispatches/validator-t026/014-coordinator-to-owner.response.md`.

Owner read full verdict010, repair response011 and recheck prompt013. R1 originally
required both full origins through inspection and comparison, with an explicit
stop if the public inspection result could not expose the required origin. The
repair still asserts only episode/target/source_id in each inspection. That does
not prove the full origin, and the issue is more than a missing assertion.

Owner inspected `src/app.rs:InspectionResult` and `inspect`, and the published
inspection schema. The response has the prediction's local source_id and the
evaluation `configuration`, but no resolved SourceDefinition. A local source ID
does not itself expose its model, source configuration, observation definitions or
preparation. These exist in the verified report's sources map but are absent from
the selected-episode inspection result.

The ratified S15 row requires both origins available in inspection/comparison;
the CLI inspection contract calls for associated prediction/configuration evidence.
The owner interpretation is that a selected episode's inspection must expose its
resolved source definition, not merely a local reference requiring a different
artifact to supply the origin. This follows the existing requirement and is not
an additional task kind or feature. Preserve evaluation configuration separately.

## Finish the current focused verdict first

Route this exact concern to the active verifier in a numbered file. Have it bind
the current public inspection and schema to the original R1 gap, preserve all five
other corrected findings and save its verdict. Do not restart general review.
Freeze the current candidate until that finding is saved. If a Ready verdict was
already saved, preserve it and save a focused addendum; do not overwrite history.

## Bounded owner-authorized correction after the finding

The previous test-only repair did not investigate the expressly flagged missing
public origin. The necessary next result is a single resolved source in inspected
episode evidence. Its value warrants this small correction to the promised
machine contract. Resume one retained developer as sole writer, after T028 has
yielded as already recorded. No second writer or source-bundle mutation.

Scope: `src/app.rs`, `schemas/v2/inspection.schema.json`, focused
`tests/conformance.rs` and `tests/cli.rs` if needed. Add a required `source` field
to InspectionResult containing exactly the full SourceDefinition resolved by the
selected prediction's source_id from the already verified report. Preserve the
prediction, original opaque input, evaluation configuration and all other fields.
Use verified stored evidence paths in that definition; never dereference original
source paths. Include model/kind/configuration and optional question, observation
definitions, preparation and evidence fields when present. Do not include unrelated
sources or arbitrary evidence-file contents.

Use the existing validated/verified source serialization and current field layout;
do not add a registry, duplicated source validation, new public SDK, replacement
configuration object or fallback. Synchronize the strict inspection schema with
the existing source-definition contract, including the task's legal source kind.
This repairs the under-construction wire2 schema to the ratified requirement;
no new product protocol version or other schema rewrite is authorized.

Make S15 assert both inspected full Q1/Q2 definitions against the fixed inputs,
and preserve complete comparison-side definitions. Extend existing E08 or a
relevant source-observation case to prove inspected definitions/preparation and
stored evidence bindings survive. Verify both task kinds through real inspect and
the published schema, preserving relocation/privacy and existing output fields.
Expected data comes from fixed source inputs or the verified stored binding
contract, not blindly copied from current inspection output.

Run affected filters/inspection/schema/CLI regressions and the literal full
format, all-feature Clippy/test, release and diff gates once. Only the unchanged
staged17+3 Clippy inventory may remain. Freeze exact changed hashes; return to the
same verifier for a focused R1/source-field recheck plus affected schema/privacy/
replay behavior, reusing unaffected evidence. A further concrete discrepancy
returns to owner rather than widening this correction.

Update T026 task/physical/downstream input ownership for the required inspection
schema/CLI changes before dispatch. Index/link all instructions, findings and
responses in Card188. The private source checkpoint remains untouched. T027 and
T029+ remain undispatched; T028 review waits for the verifier role to be free.
