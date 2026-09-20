# T026 S15 resolved-inspection-origin addendum

Verdict: **Revise**.

Ready response013 remains unchanged. Its conclusions for S24, S26, S27, M17 and
M18 remain valid, as does S15's comparison-side source-definition evidence. This
addendum changes only the S15 public-inspection conclusion: the frozen candidate
does not expose the selected episode's resolved source definition.

## Frozen identity

The inspected artifacts retain manifest012 identities:

- `012-coordinator-repaired-manifest.md`:
  `4fc0552b87d895a5597977a12884fb09b6c1e5c2145152ede8551b54abb85895`
- `013-verifier-to-coordinator.response.md`:
  `285cc24cc012121d3b8d969c95be6e6cd6bcb70b23201e9350e3dad635b5aa37`
- `src/app.rs`:
  `007c00ac10a4db21f19f7f21cdbd3861c9581fed03c864bc59debd514d54d800`
- `schemas/v2/inspection.schema.json`:
  `2c7e064d0adb915b77c558a7ebbfade96f7bcf0eee890b881ced1383f0eedaef`
- `tests/conformance.rs`:
  `a685073e0e136f0f16167594833c58ef18dc024d6733a9f1d5014d665b59950d`

I read owner prompt014 and inspected only the manifest012 S15 assertions, public
inspection construction, and strict inspection schema. I did not rerun settled
filters or inspect T027/T028.

## Reproduced contract gap

Requirement: the ratified S15 row and original R1 require both full origins to be
available through public inspection and comparison. Owner prompt014 resolves the
inspection meaning: the selected episode must expose the complete resolved
`SourceDefinition`, including kind, model, source configuration and applicable
question, observation definitions, preparation and verified stored evidence
bindings. The evaluation configuration remains a separate field.

Expected surface: inspecting the fixed Q1 or Q2 episode returns its raw prediction
and a resolved `source` value taken from the already verified report source map.
For evidence-bearing sources, that value uses stored evidence paths and does not
dereference original source paths.

Actual production surface:

- `src/app.rs:76-90` defines `InspectionResult` with `prediction` and
  `configuration`, but no `source` field.
- `src/app.rs:315-322` selects only the raw prediction record. It does not resolve
  that record's `source_id` to a source definition.
- `src/app.rs:333-337` builds `configuration` from stored `config.json`; this is
  the evaluation configuration, not the prediction source configuration.
- `src/app.rs:338-355` constructs the public result without any source definition.

Actual schema surface:

- `schemas/v2/inspection.schema.json:5` does not require `source`.
- `schemas/v2/inspection.schema.json:6-20` declares no `source` property and uses
  `additionalProperties: false`, so a resolved source cannot be published by a
  conforming inspection result.
- The task-specific prediction definitions at lines 145-168 contain only the
  artifact-local `source_id`; they do not embed or resolve the referenced source.

Actual S15 proof: `tests/conformance.rs:1677-1689` inspects both fixed rows but
asserts only episode ID, expected label and prediction `source_id`. The complete
Q1/Q2 definitions are asserted later only in the comparison at lines 1712-1723.
Therefore the repaired test proves local references and comparison origins, but
not full origins through inspection.

Consequence: an inspection consumer cannot determine the selected prediction's
model or source configuration from the inspection result itself, and cannot see
its observation-definition, preparation or verified stored-evidence provenance
when those fields exist. It must open and correlate a separate report by a local
ID, contrary to the resolved S15 inspection contract.

## Smallest required correction

Add one required `source` field to `InspectionResult` and the strict inspection
schema. Resolve exactly the selected prediction's `source_id` against the already
verified stored report source map, preserving the verified serialized
`SourceDefinition` with stored evidence paths. Keep the raw prediction, opaque
input and evaluation `configuration` unchanged and separate. Do not include
unrelated sources or evidence-file contents.

Update S15 to assert the complete fixed Q1/Q2 inspected source definitions.
Extend the existing preparation/evidence inspection coverage to prove optional
observation definitions, preparation and stored evidence paths survive, and keep
both task kinds/schema/relocation/privacy regressions bounded to this new field.

No repository source, schema, test, fixture, governance, plan, index, session
note, acceptance record or Fizzy state was edited during this addendum. The other
five repaired conclusions and the comparison-side S15 proof remain accepted.
