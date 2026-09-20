# T025 finding-driven numerical evidence repair

## Scope and reconciliation

All prompt011 starting identities matched before editing. I read verifier
verdict010, owner prompt012, acknowledgment012, and superseding prompt013. Prompt013
replaced only the copied S12 aggregate-vector instruction. I changed only
`tests/conformance.rs`; fixtures, production, schemas, tolerances, governance,
plans, index, session notes, acceptance records, and Fizzy remain unchanged.

The expected values below are derived from the ratified formulas and raw inputs
before the public API is invoked. I did not read `src/evaluation*`, production
metric/count helpers, or production scoring code to derive them.

## Corrections

| Finding | Concrete repair and discriminating assertion |
|---|---|
| Direct-ratio metadata | `assert_direct_fraction` now requires expected population count, unit, and scope and verifies them for every caller: exhaustive single/multi paths, S/E/M cases, and named exact cases. S03 explicitly requires selected accuracy/coverage/class metrics to use `N=4`, `episode`, `selected`, while selective accuracy requires `G=3`, `episode`, `answered`. |
| S12 binary64 bins | Threshold evidence remains unchanged. For the existing 29 values, a fixed independent bin table drives 29 isolated public evaluations. Each report must have count one in its expected bin and zero in all others, include its sole ID, and have matching mean signal; boundary metadata and signal-one/bin9 remain asserted. For `i=1..8`, below/equal/above map `[i-1,i,i]`. `next_down(.9) * 10` floors to exactly 9, so all three .9 samples, plus endpoint one, map to bin9. This is the owner-corrected binary64 rule, not prompt011's superseded aggregate vector. |
| M02 undefined labels | Both A/B entries now assert identity; support, answered support, and predicted support zero; `TP=FP=FN=0,TN=1`; F1 null `undefined_zero_denominator`, operands `0/0`, and population `1`, `episode`, `answered`. Existing exact/coverage/Hamming/micro/macro assertions remain. |
| M14 class target | The second M14 subcase now puts `expected:{type:class,label:A}` in a multi-label golden record while keeping an otherwise legal labels/marginals prediction. It asserts public `E_CONFIG` and absent output with the existing error helper. The remaining wrong target, probability, and policy subcases remain. |

No literal public-contract discrepancy appeared.

## Focused command evidence

| Command | Exit | Result |
|---|---:|---|
| `cargo test --locked --test conformance exhaustive_single_label -- --nocapture` | 0 | 1 passed, 73 filtered; 301 independent populations. |
| `cargo test --locked --test conformance exhaustive_multi_label -- --nocapture` | 0 | 1 passed, 73 filtered; 64 answered pairs plus 8 abstentions. |
| `cargo test --locked --test conformance case_s03 -- --nocapture` | 0 | 1 passed, 73 filtered. |
| `cargo test --locked --test conformance case_s12 -- --nocapture` | 0 | 1 passed, 73 filtered; all 29 isolated binary64 evaluations passed. |
| `cargo test --locked --test conformance case_m02 -- --nocapture` | 0 | 1 passed, 73 filtered. |
| `cargo test --locked --test conformance case_m14 -- --nocapture` | 0 | 1 passed, 73 filtered. |
| `cargo test --locked --test conformance full_numeric_conformance -- --nocapture` | 0 | 1 passed, 73 filtered. |
| `cargo test --locked --test conformance -- --list` | 0 | 74 tests, unchanged inventory. |
| `cargo test --all-features --locked` | 0 | 44 library, 10 CLI, 74 conformance, and 0 doc tests passed. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Only accepted unchanged 17 production plus 3 duplicate lib-test diagnostics. |
| `cargo fmt --all -- --check` | 0 | Formatting valid. |
| `git diff --check` | 0 | No whitespace errors. |

Clippy's unchanged staged inventory is the prior 17 production dead-code
diagnostics in `src/model.rs`, `src/model/common.rs`,
`src/model/single_label.rs`, and `src/validation/wire.rs`, plus duplicate lib-test
diagnostics for `GoldenDataset.schema_version`, `GoldenEpisode.input`, and
`PredictionArtifact.schema_version`. The repair adds no warning. Release build
was intentionally not rerun under prompt011.

## Final identities

| Artifact | SHA-256 |
|---|---|
| `tests/conformance.rs` changed | `40b9d231e3c77416d9c0f6bc93784bd309d45fa1cd6fe8a3eea6327f27ed3cd1` |
| `tests/fixtures/single-label/expected.json` unchanged | `ab93c379b9c0df1d0c0fb7aac7a6df5e7748f3f671d55dd69d173e640090b268` |
| `tests/fixtures/multi-label/expected.json` unchanged | `213183a46c0f6c011296ef5087b5c44d3852081bdf20eaa80cbb494d3d2c84f2` |
| `src/model/multi_label.rs` unchanged | `3f515a0a9122db963b2e01ca14d5474b4836ba3543a48a4e0766a4459eea0b7c` |
| `schemas/v2/report.schema.json` unchanged | `805de93d2a50339370e803b7ff1a9a1b8d34c3028b5b21a1a6f428f591733f90` |
| `schemas/v2/inspection.schema.json` unchanged | `2c7e064d0adb915b77c558a7ebbfade96f7bcf0eee890b881ced1383f0eedaef` |

Verdict: **The authorized one-cycle repair is complete and ready for the same
verifier's bounded recheck. T026 and later work remain unstarted.**
