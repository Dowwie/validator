# T010 final discriminating evidence

Completed the owner-bounded final T010 cases. `tests/conformance.rs` adds isolated
named source-key red/green, complete `reject_below`, rejected `label_set`, legal
all-observation fixture (including scalar 1.33 and categorical 0.99), and isolated
empty/wrong observation-map cases. `schemas/v2/predictions.schema.json` now adds
`sources.propertyNames` referencing the existing nonblank label definition; the
whitespace source-key case was the red condition and now rejects.

`cargo fmt --all -- --check`, focused conformance (1 passed), library regression,
and `git diff --check` exited 0. The 161 known unwired warnings remain. No T011
work was started. The coordinator must index this required response under the
repository dispatch policy.
