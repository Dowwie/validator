# Validator T009 exact ECE correction 011

Continue the active T009 assignment from dispatch 009. Superseding required final
response path:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/011-scorer-developer-to-coordinator.response.md`.

The current `ece` implementation in `src/evaluation/single_label.rs` has one
contract defect: the `fold` closure names its running accumulator `total`, which
shadows the selected-population `total` parameter. Its weight expression therefore
divides each bin count by the accumulating ECE value; the first populated bin
divides by zero. Correct it so every weight is exactly `bin.count / N`, where N is
the selected maximum-probability population, and the separate accumulator only
sums the weighted absolute gaps.

Strengthen the owning evidence within the existing T009 scope:

- assert a concrete independently calculated top-label ECE value, not only
  `is_some()`;
- assert explicit expected bin indexes for adjacent binary64 values around the
  fixed boundaries, rather than recomputing the production formula as the test's
  sole expected result;
- keep the specified `min(floor(10*h),9)` behavior, score 1 in bin 9, exact
  included/excluded populations, and all dispatch-009 checks.

Do not widen into other scoring or refactoring. Continue the full T009 unit and
save the complete response at the superseding path. Record this transient defect
and its corrected regression evidence in that response.
