# Complete T025 output 1 after the bounded abstention-wire repair

Role/model: resumed sole oracle/test developer, `gpt-5.6-terra`, high reasoning,
original fresh `fork_turns: none` context. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #187 — Verify Validator numerical conformance](http://localhost:3006/1/cards/187).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t025/006-oracle-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat return only the
path, SHA-256 and terse status. Do not edit governance, plans, artifact-index,
session notes, acceptance records or Fizzy. Do not delegate.

This resumes T025 local output 1 after the authorized production/schema repair in
response005. Read global/repository AGENTS, original dispatch002 and response002,
owner prompt004, repair dispatch005 and the full repair response005. Reconcile the
pinned candidate below before editing. Preserve the completed abstention repair and
all production/schema files read-only. Do not begin the assigned S/E rows, M rows,
T026 or later work.

The original independence boundary remains absolute: **do not read or copy
`src/evaluation*`, production metric/count helpers, or any production scoring
implementation to derive or revise expected values.** Use the ratified formula,
the independently derived matrix/counts, and the public report only after the
expectation is fixed. Production must not generate the expected result.

Starting identities:

| Artifact | SHA-256 |
|---|---|
| `docs/dispatches/validator-t025/002-oracle-developer-to-coordinator.response.md` | `43c0dfa849fce1ff3a4625135e1ea0f8b9306628860c75b255203ee4942fb925` |
| `docs/dispatches/validator-t025/004-owner-to-coordinator.prompt.md` | `f1d8599e210e6d5f1ef3b9a02e53076ebc6f92b4a846df59843f0417e6fe8b3a` |
| `docs/dispatches/validator-t025/005-coordinator-to-repair-developer.prompt.md` | `064ca8b1015e23eff781c3be11b579b29e7e028584a98993770a9e8b62125326` |
| `docs/dispatches/validator-t025/005-repair-developer-to-coordinator.response.md` | `90122378998d184c8ec882a9e5e2431809c693e12417f5ad52bf87612d65e5ac` |
| `docs/specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `docs/plans/validator/tasks/T025.json` | `c29765ace2e7859665e290a5a7afad8b71d312ae3ff2ef9c4bb01d6aebdb4de6` |
| `tests/conformance.rs` | `6295d99c7124944608556b2ce9f1197af37acab5f8c9c9590cc4a111882ef825` |
| `tests/fixtures/single-label/expected.json` | `1fc52d5a24d7900fbef992b0f19227089d83e38d0656b9809fc08f9ed0dcbfe0` |
| `tests/fixtures/multi-label/expected.json` | `213183a46c0f6c011296ef5087b5c44d3852081bdf20eaa80cbb494d3d2c84f2` |
| `src/model/multi_label.rs` | `3f515a0a9122db963b2e01ca14d5474b4836ba3543a48a4e0766a4459eea0b7c` |
| `schemas/v2/report.schema.json` | `805de93d2a50339370e803b7ff1a9a1b8d34c3028b5b21a1a6f428f591733f90` |
| `schemas/v2/inspection.schema.json` | `2c7e064d0adb915b77c558a7ebbfade96f7bcf0eee890b881ced1383f0eedaef` |

## Required completion

The repair author reran both exhaustive filters successfully. Its full-suite run
then exposed the preserved F04 assertion at `tests/conformance.rs:1913`:

```text
assertion `left == right` failed
  left: Number(2)
 right: Number(1)
```

Reconcile this using only the independent F04 rows/matrix and the normative
single-label formula at `docs/specs/validator-v1.md:529-531`. The formula defines
the serialized F1 ratio operands as `2*TP` over `2*TP+FP+FN`; the rational value
may reduce mathematically, but the expected wire numerator/denominator must match
those literal operands. For class C, record the counts and determine whether the
fixture's `[1,2]` is merely a reduced-value spelling where the normative wire pair
is `[2,4]`. If confirmed, make the smallest oracle-owned correction in
`tests/fixtures/single-label/expected.json` and update the handoff derivation to
distinguish the value `1/2` from its required wire operands `2/4`. Do not change
production or weaken the exact numerator/denominator assertion. If the formula or
public output instead demonstrates a real contract discrepancy, freeze exact
input/expected/actual evidence and stop without tuning the fixture.

Complete output 1 by running and recording:

```text
cargo test --locked --test conformance -- --list
cargo test --locked --test conformance exhaustive_single_label -- --nocapture
cargo test --locked --test conformance exhaustive_multi_label -- --nocapture
cargo test --locked --test conformance f04_asymmetric_oracle -- --nocapture
cargo test --locked --test conformance single_matrix_identities -- --nocapture
cargo test --locked --test conformance -- --nocapture
cargo fmt --all -- --check
git diff --check
```

Reuse response005's passing focused abstention/schema/CLI/release evidence; do not
rerun those checks or Clippy by habit. Confirm the repaired production/schema
hashes remain unchanged and report exact final hashes for the two expected
fixtures and `tests/conformance.rs`. Response006 must state the independent counts
and F04 operands/value explicitly, list every requested command exit and nonzero
filter/list evidence, and return either a complete output-1 handoff or a frozen
new discrepancy. Do not claim all of T025 complete.
