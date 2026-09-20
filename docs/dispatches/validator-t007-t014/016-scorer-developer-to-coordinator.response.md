# T010 regression-matrix completion

Completed the bounded evidence pass without changing schemas. `tests/conformance.rs`
now contains named specification-derived cases at lines 61-158, including exact
`labels`/rejected `label_set`, task-target mismatch, scored-choice question ID,
closed/typed observations, whitespace/nonempty map rules, all decision variants,
extra-field rejection, legal multi-label labels/marginals/thresholds, and the
shape-valid absent-source runtime boundary.

Focused evidence: `cargo fmt --all -- --check` exit 0;
`cargo test --locked --test conformance input_schema_contract -- --nocapture`
exit 0 (1 passed); `cargo test --locked --lib` exit 0; `git diff --check` exit
0. Existing 161 dead-code warnings remain unsuppressed. No schema red/green
correction was needed in this bounded pass.

This response is saved as required; the coordinator must index it because this
dispatch forbids worker index edits.
