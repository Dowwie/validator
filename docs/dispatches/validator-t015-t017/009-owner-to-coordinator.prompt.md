# Enforce T016's concrete comparison contract before the local handoff

Role/model: existing Sol-high coordinator, retained context.
Required full acknowledgment:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t015-t017/009-coordinator-to-owner.response.md`.

A focused owner read of current `src/comparison.rs` and the new comparison model
found that the declared typed boundary is still incomplete. `compare` accepts
generic Value reports, `collect_metrics` discovers arbitrary metrics recursively,
and `ComparisonMetricFamily` stores a Vec of string-named metric records. Required
fields therefore still exist only by naming convention. Adding typed outer structs
has not corrected the generic metric-map approach from the partial implementation.

This directly conflicts with `docs/specs/validator-data-model.md:64-68` (no dynamic
metric map whose required fields exist only by naming convention), its typed
report/comparison requirement at lines 215-218, and T016's concrete paired-metric
contract. This is required architecture, not optional elegance or a new feature.

Apply this bounded correction within active T016:

- Feed the pure comparison boundary the verified, recomputed concrete single-label
  data/results and typed run metadata already produced by replay. Minimal private
  app/model wiring is authorized. A private input record with the necessary typed
  references is appropriate; merely wrapping a Value and retaining recursive
  discovery is not sufficient.
- Replace the generic metric collector/family with concrete single-label hard and
  categorical paired-result structures whose required metric fields are explicit
  Rust fields and required schema properties. Per-label entries may remain a
  vocabulary-ordered collection of a concrete per-label type. Raw and final use the
  same concrete hard-pair type, with distinct values. Reuse a small metric-pair
  helper and existing checked MetricResult/status/population contracts; do not
  reconstruct them through string lookups or string-based status inference.
- Keep source/configuration differences as recorded data and keep the already
  required categories, transitions, answered populations and receipts. Opaque data
  may remain opaque at its proper boundary. No universal registry, generic metric
  framework, broader SDK or new supported task kind.
- Use the existing checked arithmetic for transition/category accounting while
  touching that path; current `+= 1` and unchecked count summation should not
  establish a second counting convention.
- Preserve all useful current behavior and named test assertions. Update the
  strict comparison schema and existing T016 conformance expectations to the
  concrete required fields. Continue the same three named tests and applicable
  gates. Demonstrate the mapping from each required family to actual fields;
  passing a dynamic list of metric names is not that evidence.

Do not start T017 until this existing contract is fulfilled. No additional review
gate or test framework is added, and the combined T015-T017 review/repair allowance
is unchanged. Route the correction as a numbered full file-based developer brief,
preserve the sole writer and all staged-lint rules, and index/link the artifacts.
If another concrete obstacle prevents the typed boundary, report its exact cause
and smallest resolution rather than returning a generic map as completion.
