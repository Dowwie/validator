# Validator T007-T014 completion follow-up 003

Continue the exact authorized sequence from dispatch 001. Required complete
response is now:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/003-developer-to-coordinator.response.md`.

Your response 001 ended after an initial T007 slice, before either required
milestone, and reported no blocker. It is not a candidate. Preserve the current
T007 code and passing named filters, then complete T008 through T014 and the full
handoff. This is continuation of the same assignment, not a new scope or repair.
A second incomplete return goes directly to owner reassessment; do not return an
orientation or partial implementation summary.

Read owner clarification 002 and coordinator response 002 in full before affected
work. Apply these exact boundaries:

1. **T010 schema-only tests may use the schema validator directly.** Legal
   multi-label schema alternatives do not go through the single-label runtime
   before T018. T008/T009/T013 scoring/report conformance and runtime parity cases
   still use the real T014 application API; do not expose private core APIs.
2. **T009 remains as-recorded.** `signal_population_bins` proves the probability
   and confidence populations for recorded answers and abstentions, exact
   included/excluded IDs, bins, and counts. Do not implement or fake threshold-
   rejected behavior. The updated T009 task expectation stages unchanged raw
   confidence membership for threshold-rejected answered rows in T021's
   `decision_policy_boundaries` case.
3. **Run layout is exact.** A successful run directory contains only
   `golden.json`, `predictions.json`, `config.json`, optional `evidence/`, and
   `report.json`. Put artifact/manifest metadata inside `report.json`; do not add a
   separate manifest or per-run schema copies. Published schemas remain in
   `schemas/v2/`.

Current partial hashes to reconcile before continuing:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `981783137668727726799ca99ee963f5333978cd6090ad9953b6d237a8e2c621` |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` |
| `src/lib.rs` | `bd89b6b7148e8ef25e64d28b465218a1f4afd90b8f95e6f92ed3a510de69aade` |
| `src/model/common.rs` | `5be10755552728c249ac2c809769c89585c80e6ce5dc35784aef95ddb913dd46` |
| `src/evaluation.rs` | `6fbab6a97da6d53c19d03897a09b3901d7ae6bf66a1a50c7df9ceebbc25f0fb8` |
| `docs/dispatches/validator-t007-t014/001-developer-to-coordinator.response.md` | `ba42f29dfe00c7e2831e2dfa42685a7d004319297c133d5880ca600ef39776dd` |

All other implementation hashes remain at accepted manifest 024. The coordinator
independently reran `metric_status_precedence`, `count_overflow_is_error`, and
`macro_undefined_classes`; each executes one passing test. Treat T007 as a local
milestone, not accepted work, and correct it later if integration reveals a
criterion defect.

Every artifact, criterion, command, clean T014 gate, write scope, real-API rule,
milestone, preservation obligation, and prohibition from dispatch 001 remains in
force except where the owner clarification above narrows ambiguity. Continue to
the pure-scoring milestone after T008/T009, then artifact-publication milestone,
then complete T014 and save the full response. Return early only for a concrete
reproduced blocker with the exact command/contract and smallest decision while
finishing independent work where possible. Do not delegate or spawn agents.

