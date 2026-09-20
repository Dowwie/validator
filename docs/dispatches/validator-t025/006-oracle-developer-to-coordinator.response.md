# T025 output 1 completion after abstention-wire repair

## Reconciliation and scope

I reconciled every pinned identity in dispatch006 before editing. The prior
oracle response, owner/repair dispatches and response, ratified specification,
T025 contract, repaired test candidate, fixtures, and repaired production/schema
files matched the supplied SHA-256 values. The repair-owned hashes remain
unchanged below.

I did not read `src/evaluation*`, production metric/count helpers, or any
production scoring implementation to derive or revise an expected value. The
only changed oracle artifact is the existing single-label expected fixture. The
exhaustive oracle bodies, multi-label fixture, production, and schemas are
unchanged.

## Independent F04 correction

The raw F04 rows remain:

```text
A→A, A→A, A→B, A→C, B→B, B→B, C→A, C→C
```

The independently constructed matrix is:

```text
[[2, 1, 1, 0],
 [0, 2, 0, 0],
 [1, 0, 1, 0]]
```

For class C, row/column accounting gives support `2`, predicted support `2`,
`TP=1`, `FP=1`, and `FN=1`. The ratified direct F1 formula therefore requires
the serialized operands:

```text
2*TP / (2*TP + FP + FN) = 2 / (2 + 1 + 1) = 2/4
```

`2/4` has mathematical value `1/2`, but the fixture assertion deliberately
checks the literal numerator and denominator reported by the public wire
contract. The old fixture pair `[1,2]` was a reduced-value spelling and was the
sole source of the F04 failure; it did not indicate a production discrepancy.

The smallest correction changes F04 `class_f1` from:

```json
[[4,7],[4,5],[1,2]]
```

to:

```json
[[4,7],[4,5],[2,4]]
```

The independently derived F04 values remain accuracy `5/8`, class F1 values
`4/7`, `4/5`, `1/2`, and macro-F1 `131/210`.

## Required command evidence

| Command | Exit | Result |
|---|---:|---|
| `cargo test --locked --test conformance -- --list` | 0 | 40 tests, 0 benchmarks; both exhaustive filters, F04, and `single_matrix_identities` listed. |
| `cargo test --locked --test conformance exhaustive_single_label -- --nocapture` | 0 | 1 passed, 39 filtered; direct enumeration covers 301 populations. |
| `cargo test --locked --test conformance exhaustive_multi_label -- --nocapture` | 0 | 1 passed, 39 filtered; 64 answered subset pairs plus 8 whole abstentions pass after the wire repair. |
| `cargo test --locked --test conformance f04_asymmetric_oracle -- --nocapture` | 0 | 1 passed, 39 filtered; literal F04 operand assertion now matches `2/4`. |
| `cargo test --locked --test conformance single_matrix_identities -- --nocapture` | 0 | 1 passed, 39 filtered. |
| `cargo test --locked --test conformance -- --nocapture` | 0 | 40 passed, 0 failed. |
| `cargo fmt --all -- --check` | 0 | Formatting valid. |
| `git diff --check` | 0 | No whitespace errors. |

The focused repaired abstention/schema/inspection/CLI/release evidence in
response005 is reused as instructed; I did not rerun it. The expected staged
Clippy inventory was not rerun.

## Final identities

| Artifact | SHA-256 |
|---|---|
| `tests/conformance.rs` | `6295d99c7124944608556b2ce9f1197af37acab5f8c9c9590cc4a111882ef825` |
| `tests/fixtures/single-label/expected.json` | `ab93c379b9c0df1d0c0fb7aac7a6df5e7748f3f671d55dd69d173e640090b268` |
| `tests/fixtures/multi-label/expected.json` | `213183a46c0f6c011296ef5087b5c44d3852081bdf20eaa80cbb494d3d2c84f2` |
| `src/model/multi_label.rs` unchanged | `3f515a0a9122db963b2e01ca14d5474b4836ba3543a48a4e0766a4459eea0b7c` |
| `schemas/v2/report.schema.json` unchanged | `805de93d2a50339370e803b7ff1a9a1b8d34c3028b5b21a1a6f428f591733f90` |
| `schemas/v2/inspection.schema.json` unchanged | `2c7e064d0adb915b77c558a7ebbfade96f7bcf0eee890b881ced1383f0eedaef` |

Verdict: **T025 local output 1 is complete and ready for independent review.
Later S/E, M, T026, and later work remain unstarted.**
