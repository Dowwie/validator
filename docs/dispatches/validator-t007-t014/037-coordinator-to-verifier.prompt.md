# T007-T014 focused repaired-candidate recheck 037

Role/model: retained independent verifier
`/root/coordinator/t007_t014_verifier`, `gpt-5.6-sol`, reasoning `high`, retained
context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/037-verifier-to-coordinator.response.md`.

Remain read-only except for the response file. Do not delegate, fix artifacts,
reopen settled criteria, or review T015+. This is the one focused recheck after
your response034 `Revise` verdict.

Read repaired manifest036 and repair response035. Hash manifest033, verdict034,
response035, and all five repaired paths. Confirm every other manifest033 input is
unchanged. A mismatch is `Blocked`.

## Finding 1 recheck

Inspect the shared admitted-row normalization helper and both callers. Confirm:

- it derives count and maximum absolute submitted-sum error from checked admitted
  categorical rows without scoring, report assembly, time/ID, or I/O;
- check and report use the same helper, with checked count conversion;
- the built-binary regression independently asserts count 1 and the actual nonzero
  error for `[0.7,0.2,0.1000000005]`, validates stdout schema, and preserves
  check's no-write behavior.

Run:

```sh
cargo test --locked --test cli cli_check_evaluate -- --nocapture
cargo test --locked --lib report_sources_and_privacy -- --nocapture
```

## Finding 2 recheck

Inspect the exact report schema branch and handcrafted fixture. Confirm `minItems: 2`,
the positive report is coherent across vocabulary/matrix/classes/probabilities, and
the one-label-only mutation is rejected.

Run:

```sh
cargo test --locked --test conformance single_report_schema -- --nocapture
```

## Justified regressions and verdict

Run:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
git diff --check
ruby docs/plans/validator/verify-plan.rb
```

All except the declared Clippy limitation must exit 0. Clippy must remain exactly
the 26 unique owner-staged production `dead_code` diagnostics plus test duplicates,
with no new/ordinary lint. Reuse your unchanged T007-T012 and unaffected T013/T014
evidence from response034; do not rerun the other 15 named filters or expand review.

Save a complete `Ready`, `Revise`, or `Blocked` verdict. Include repaired identity,
finding-by-finding evidence, focused/full command results, staged Clippy statement,
findings or `none`, and residual risks. A second material defect must cite the
current criterion, reproduction, consequence, and smallest correction.
