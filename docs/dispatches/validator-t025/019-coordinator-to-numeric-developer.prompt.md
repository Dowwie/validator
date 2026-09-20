# Add the missing literal M02 zero-operand assertions

Continue the active owner016/developer017 population-unit correction as the same
sole writer. Do not delegate. Keep the same required response path:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t025/017-numeric-developer-to-coordinator.response.md`.

Read owner prompt018 and acknowledgment018 before completing the repair. This
follow-up adds one exact test obligation already present in verifier verdict010's
M02 finding; every other prompt017 scope, write boundary, check and stopping rule
remains unchanged.

`assert_direct_fraction` returns early when its expected denominator is zero, so
passing `0/0` does not prove the public operand fields. At the M02 per-label F1
boundary, explicitly assert for both labels that:

- the serialized `numerator` field is present and equals integer zero;
- the serialized `denominator` field is present and equals integer zero.

Retain the null value, `undefined_zero_denominator` status, binary counts/supports
and corrected population `G`, unit `label_decision`, scope `answered` assertions.
Do not generalize this into a shared status audit, change every zero-denominator
branch, or edit production ratio/status helpers. Include the literal operand proof
in focused M02 and in response017's evidence mapping. The same verifier will
recheck it with the population-unit correction.
