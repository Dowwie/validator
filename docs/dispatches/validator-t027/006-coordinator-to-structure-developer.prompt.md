# Correct the surfaced conformance-test Clippy diagnostics

Continue the same output1 and response002. `tests/conformance.rs` was already an
owner001 initial test target, a ratified T027 artifact and a physical T027 writer;
the clean-gate scope therefore includes the six diagnostics that were masked by
the prior library failures.

Write only `tests/conformance.rs` for this correction. Replace the three repeated
complex test tuple signatures with the smallest descriptive test-local type alias
or equivalent simple structure, and change the two helper arguments from `&Vec<_>`
to slices where Clippy reports `ptr_arg`. Update only their mechanical call sites.
Do not add `allow`, change assertions/fixtures/oracles, alter filter names/counts,
move production logic into tests or perform unrelated test refactoring.

Rerun warning-denied Clippy immediately and preserve its exact exit/output. Then
complete the full output1 gate sequence from prompt002. If a further diagnostic
class appears, return its exact location/cause before another scope change. Record
these previously masked test-only dispositions and unchanged behavior/counts in
response002.
