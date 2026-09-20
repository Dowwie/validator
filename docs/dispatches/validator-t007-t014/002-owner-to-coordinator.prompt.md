# Keep schema, policy, and run-layout work at its specified boundary

Role/model: existing Sol-high coordinator, retained context.
Required response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/002-coordinator-to-owner.response.md`.

The owner read the complete next-sequence dispatch. Preserve active work and make
these three bounded clarifications before the affected implementation:

1. **T010 schema tests are allowed to invoke the schema validator directly.** They
   test published input shapes, including the multi-label schema alternatives that
   intentionally precede multi-label runtime support. The real-application-API
   requirement applies to scoring/report conformance and runtime parity cases;
   it must not force T010's valid multi-label schema examples through a runtime
   that does not implement T018 yet. No private core API exposure is needed.
2. **The threshold-rejected subcase belongs with T021.** T009's
   `signal_population_bins` test now proves as-recorded probability/confidence
   populations, including abstentions, exact IDs, bins and counts. Its task JSON
   expected text currently also mentions threshold rejection, but that observable
   behavior cannot precede the declared T021 policy implementation. Update only
   that verification expectation in T009 to make the staging explicit, and extend
   T021's existing `decision_policy_boundaries` expected evidence to require the
   unchanged raw confidence population including threshold-rejected answers.
   This preserves the complete requirement and final conformance case; it neither
   implements T021 early nor fakes policy outputs in T009's application tests.
3. **Publish only the specified run files.** Main spec CLI/run artifacts (lines
   740–748) requires exact golden.json, predictions.json, config.json, optional
   evidence/, and report.json. Manifest metadata is part of the report contract;
   do not infer separate manifest/schema files in each run directory from the
   dispatch's phrase "schemas/manifest/report as specified." Schemas are published
   under schemas/v2 as assigned. Add no per-run schema copies or extra manifest
   file without an actual specification requirement.

Route these clarifications through a saved developer follow-up, preserving the
current implementation/milestone progress. Make the two narrow task-expectation
metadata edits and record their sequencing rationale in session notes. No
requirement, tolerance, final test case, task dependency, or final gate is removed.
Do not change the specifications. Run verify-plan and diff-check, index/link the
prompt/response/follow-up, and continue active supervision with caffeinate held.
