# T007-T014 frozen combined independent review dispatch 034

Role/model: fresh independent verifier `/root/coordinator/t007_t014_verifier`,
`gpt-5.6-sol`, reasoning `high`, context inheritance `none`.
Repository: `/Users/dowwie/MyProjects/validator`.
Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/034-verifier-to-coordinator.response.md`.
Fizzy: [Card 183](http://localhost:3006/1/cards/183).

You are read-only for product source, schemas, tests, fixtures, specifications,
plans, governance, and Fizzy. Your only permitted write is the required response
file. Do not delegate, fix code/tests, start T015, or widen this review.

## Required orientation and identity

Read in full:

- `/Users/dowwie/.codex/AGENTS.md`, repository `AGENTS.md`, and
  `/Users/dowwie/.agents/skills/manage-dev-team/SKILL.md`;
- `docs/specs/validator-v1.md` sections linked by T007-T014 and
  `docs/specs/validator-data-model.md#reports-and-metric-reuse`;
- `docs/plans/validator/tasks/T007.json` through `T014.json`;
- `docs/plans/validator/execution-contract.md` and
  `docs/dev-team/validator-build/charter.md`;
- owner decision `028-owner-to-coordinator.prompt.md`;
- frozen manifest `033-coordinator-final-manifest.md`;
- the nine implementation evidence responses listed in that manifest, reading the
  final T014 responses031/032 before relying on earlier partial handoffs.

First hash every path in manifest033. If any product/governing/evidence input differs,
stop `Blocked` with the exact mismatch; do not review a moving candidate.

## Review scope

Independently inspect and verify every criterion in T007-T014 against current code
and actual behavior:

- T007 checked arithmetic, metric status/population/ratio semantics and overflow;
- T008 single-label raw/final matrix, counts, abstention, per-class and macro
  metrics, including the corrected independent F04 oracle;
- T009 categorical losses, positive-infinity JSON, confidence/max-probability bins,
  included/excluded IDs, exact boundaries and ECE;
- T010 strict Draft 2020-12 input schemas and the discriminating valid/invalid
  matrix, without confusing schema-only evidence with runtime behavior;
- T011 exact input/evidence bytes and digests, deterministic ordinal bindings,
  repeated paths, same basenames, parent-relative sources and unused definitions;
- T012 real atomic no-replace file/directory/symlink races, owner-only permissions,
  exact run layout, late cleanup, absolute report path and exact report digest;
- T013 typed complete/privacy-safe report, source/path rewrites, zero-count sources,
  metric/episode fields and all four output schemas;
- T014 shared check/evaluate application path, UUIDv7/time/report publication,
  typed receipts/errors, exact one-document stdout, exits 0/2/3 and category-4
  unit mapping, help/version, malformed settings, no-write check, immutable existing
  destination, exact snapshot/evidence bytes, receipt hash, schema-valid actual
  stdout and a secret-bearing post-admission failure with no leak.

Inspect for the prohibited shortcuts at these affected boundaries: no source
inclusion/test facade/static report standing in for real API evidence; no overwrite
fallback or exists-check rename; no scoring repair/tolerance change; no raw payload
in routine output; no lint suppression/fake read/artificial consumer/public-SDK
broadening. The three `cfg(test)` conveniences in response032 are explicitly
owner-authorized and are not by themselves a finding.

Do not conduct a repository-wide style/design/security audit, investigate T015+,
request optional coverage, or treat the declared staged Clippy limitation as a
defect. It becomes a finding only if the exact residual inventory differs, includes
an ordinary/new warning, or hides a current T007-T014 consumer defect.

## Commands and evidence

Run all 18 exact named T007-T014 filters from the task JSON files and record a
nonzero count. Then run:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
git diff --check
ruby docs/plans/validator/verify-plan.rb
```

Fmt, all tests, release, diff, plan, and every named behavior filter must exit 0.
Clippy is expected to exit 101 only for the exact 26 unique staged production
`dead_code` diagnostics in response032/manifest033 plus duplicated test-target
members. Capture and reconcile its exact symbols and confirm no ordinary/new lint.

Read the tests and production paths; green commands alone are insufficient. For
any `Revise` finding, cite the exact current criterion, file/line, reproduction,
observable consequence, and smallest correction. Reuse unchanged evidence and stop
once the bounded criteria support a verdict.

## Required response

Save a complete `Ready`, `Revise`, or `Blocked` verdict before returning. Include:

- verified manifest identity;
- one evidence row per T007-T014 with criterion-level result;
- all command exits and nonzero counts;
- explicit actual-CLI/schema/privacy/publication conclusions;
- exact staged Clippy reconciliation and the statement that this checkpoint is not
  warning-free and T027/T035 retain the clean gate;
- findings ordered by severity, or `none`;
- residual risks limited to this checkpoint.

Do not modify any reviewed artifact or return only a chat summary.
