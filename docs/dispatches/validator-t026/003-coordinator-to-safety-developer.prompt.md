# Preserve the accepted replay filter while adding the T026 matrix

Continue T026 output 1 as the same sole developer. Do not delegate. Save the
superseding complete response to:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t026/003-safety-developer-to-coordinator.response.md`.

The current reported conformance count is 80 because
`replay_binding_and_result_tampering` was renamed to
`artifact_adversarial_matrix`. Preserve accepted evidence: restore the exact
listed/executing `replay_binding_and_result_tampering` filter and add
`artifact_adversarial_matrix` as a separate named test. Extract or reuse one
shared test-only scenario function so the two filters call the same complete
logic; do not duplicate the corruption workflow.

With baseline 74 plus the matrix and S23-S28, the conformance target must list and
pass exactly 81 tests. Run and record:

```text
cargo test --locked --test conformance replay_binding_and_result_tampering -- --nocapture
cargo test --locked --test conformance artifact_adversarial_matrix -- --nocapture
cargo test --locked --test conformance -- --list
cargo test --locked --test conformance
cargo test --locked --test cli
cargo fmt --all -- --check
git diff --check
```

Preserve the passing S23-S28 and CLI matrix evidence, production read-only state
and every prompt002 requirement. Response003 must explain the 80-to-81 inventory
reconciliation, map both names to the shared scenario, retain full output-1
evidence/hashes and supersede any incomplete response002. Do not begin output2.
