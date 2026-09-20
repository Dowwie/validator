# Validator T010 exact schema-shape correction 013

Continue the active T010 assignment from dispatch 012. Superseding required final
response path:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/013-scorer-developer-to-coordinator.response.md`.

The current draft schemas/tests have several exact wire-contract gaps that must be
corrected before T010 is complete:

1. Multi-label expected targets and outcomes use exact `type: "labels"`, not
   `"label_set"` (`validator-v1.md` canonical golden and prediction sections).
2. The golden schema must reject a single-label task paired with a labels target
   and a multi-label task paired with a class target. This same-document pairing
   is expressible with two closed top-level alternatives; it is not a deferred
   cross-document runtime constraint.
3. A `scored_choice` source must require nonblank `question_id`; a `classifier`
   source may omit it. Express the two closed source-kind alternatives without
   inventing extra requirements.
4. Observation value variants must be distinct and closed: `scalar` accepts a JSON
   number without an interval; `bernoulli` and `reported_confidence` require
   `[0,1]`; `categorical` and `label_marginals` require nonempty maps of nonblank
   keys to `[0,1]` numbers.
5. Enforce non-whitespace strings wherever the wire contract says nonblank,
   including labels, source/map keys, model, descriptions, question IDs,
   preparation method/version, evidence paths, observation names/options, and
   threshold keys. `minLength: 1` alone permits whitespace-only strings.
6. Probability value maps must be nonempty with nonblank keys. Exact equality to
   the task vocabulary remains a runtime/cross-document rule.
7. `label_thresholds.thresholds` must be nonempty with nonblank keys and bounded
   values. Exact key equality to the vocabulary remains runtime validation.

Extend `input_schema_contract` with explicit positive and negative named cases for
each item above, plus extra-field rejection in golden, predictions, and config.
Keep the directly authorized schema-engine path and legal multi-label alternatives;
do not route them through the single-label runtime. Preserve the existing exact
scope, dialect, local references, minimal test-only dependency, and all dispatch-
012 checks.

Do not expand into semantic cross-record checks that JSON Schema cannot honestly
express. Continue until the full T010 unit is complete and save the final response
at the superseding path, recording these corrected draft defects and their
regressions.
