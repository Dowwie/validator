# T008 scorer-developer completion handoff

Status: implementation handoff complete. This is not an independent verifier
verdict or owner acceptance.

## Candidate identity

The repository remains unborn/untracked. I did not stage, commit, reset, clean,
change governance, dispatches other than this required response, the artifact
index, Fizzy, or the task-owned caffeinate process.

The required response is a new `docs/` artifact. The repository instructions
normally require an artifact-index entry, but the T008 dispatch explicitly
prohibits editing the index; the coordinator must reconcile that index update.

Confirmed input SHA-256 values before editing:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `981783137668727726799ca99ee963f5333978cd6090ad9953b6d237a8e2c621` |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` |
| `src/lib.rs` | `bd89b6b7148e8ef25e64d28b465218a1f4afd90b8f95e6f92ed3a510de69aade` |
| `src/model/common.rs` | `5be10755552728c249ac2c809769c89585c80e6ce5dc35784aef95ddb913dd46` |
| `src/model/single_label.rs` | `63b808e20181c18b6e0b6ae1a035213c72020ee471fe13e10601543d685ad71d` |
| `src/evaluation.rs` | `9911006f932bb132bb4d93a54eceba8df606bd551489b998c3cab052fd09c2dd` |
| `src/evaluation/single_label.rs` | `297eb09bba7eb786bb0b1bf67eecc10f693a78fe3711244dd2c630c0fe8a3fb2` |

Final SHA-256 values:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `981783137668727726799ca99ee963f5333978cd6090ad9953b6d237a8e2c621` |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` |
| `src/lib.rs` | `bd89b6b7148e8ef25e64d28b465218a1f4afd90b8f95e6f92ed3a510de69aade` |
| `src/model/common.rs` | `2e76390eab471cdf3341d46cf7ab5f9c71651d3a62f6f5aec341bb20df76a8d8` |
| `src/model/single_label.rs` | `7022724b5f4136d40f9b71cdbcd16757036c5ca704b4dc500c566f87437ded9d` |
| `src/evaluation.rs` | `3b6e74d440fb76d539b3fbd338c774e81062d3d4c86e1717b3c02b6696b2ddaa` |
| `src/evaluation/single_label.rs` | `71cf7040a7c2821b9392ec5d269efdd861685aca128444e349d9cddcef7cdddf` |
| `tests/fixtures/single-label/expected.json` | `99cb21c88d4d497b8d805537d9e2648304b9aab34ea748ea6752f350641728c5` |

Toolchain: `cargo 1.98.1 (797e8a9bc 2026-08-05)` and `rustc 1.98.1
(48a229cea 2026-09-01)`.

## Delivered behavior and criterion mapping

| Criterion | Production implementation | Executing local evidence |
|---|---|---|
| Evaluator borrows only checked `SingleLabelEvaluation`; raw/final are separately owned | `src/evaluation/single_label.rs:17-27`; `src/model/single_label.rs:224-260` | All three T008 owning tests validate admission followed by `evaluate`; F04 also checks distinct final matrix equality under `as_recorded`. |
| K by K+1 matrix has a typed abstention column, checked increments, and matrix-derived totals | `src/model/single_label.rs:179-198`; `src/evaluation/single_label.rs:75-143` | `single_label_f04_asymmetric_hard_metrics`; `single_label_literal_abstain_and_undefined_cases`. |
| `N=D+E+U=sum(matrix)=sum(support)`, `G=sum(predicted_support)`, TP/FP/FN identities are enforced | `src/evaluation/single_label.rs:290-317` | F04 test asserts all required sums and an A-to-B row once as A FN/B FP. |
| Per-class support/predicted support/TP/FP/FN, precision/recall/F1/coverage and aggregate hard metrics | `src/model/single_label.rs:200-239`; `src/evaluation/single_label.rs:146-287` | F04 checks counts, precision `2/3`, recall `1/2`, F1 `4/7`, accuracy/rates; thread checks coverage and selective metrics. |
| Explicit metric statuses, population scope/unit, ratio fields, no-data/no-answered handling, and macro undefined-class zero fill | `src/model/common.rs:980-1151`; `src/evaluation/single_label.rs:180-287` | Owning helpers assert exact values/status/ratio/population/unit/scope and serialized names. Literal/undefined test covers `contains_undefined_classes`, observed-always-missed F1 zero with undefined precision, all-abstain `no_answered_predictions`, and empty `no_data`. |
| Sorted typed episode evidence includes IDs, expected/raw/final outcomes, correctness, source, and observations without opaque input | `src/model/single_label.rs:241-260`; `src/evaluation/single_label.rs:48-72` | Thread test proves UUID ordering, source binding, retained checked observation, raw/final correctness, and that result debug evidence does not contain the opaque input sentinel. |
| Independent fixture constants | `tests/fixtures/single-label/expected.json:1-20` | Fixture was authored from the direct counts below, not by calling the evaluator. |

The minimal shared-model additions are `Clone` for retained observations and
observation sets, crate-private observation access, checked subtraction, and
metric constructors/accessors that retain the exact population independent of a
ratio denominator. No public SDK surface was added.

## Independent oracle derivation

F04 direct rows are A→A, A→A, A→B, A→C, B→B, B→B, C→A, C→C. Therefore its
matrix is `[[2,1,1,0],[0,2,0,0],[1,0,1,0]]`, so `N=8`, `D=5`, `E=3`, `U=0`,
and accuracy is `5/8`. Per-class F1 is `4/7`, `4/5`, and `1/2`; their average
is `(4/7 + 4/5 + 1/2) / 3 = 131/210`.

The thread rows are A→A, A→abstention, B→A, C→C. Thus `D=2`, `E=1`, `U=1`,
`N=4`, and `G=3`; accuracy is `2/4=1/2`, coverage is `3/4`, and selective
accuracy is `2/3`. Class coverages are `1/2`, `1`, and `1`. F1 is `2/3`, `0`,
and `2/3`, whose average is `1/2`.

## Commands and evidence

All final required commands exited zero:

| Command | Exit | Result |
|---|---:|---|
| `cargo fmt --all -- --check` | 0 | Formatted. |
| `cargo check --locked` | 0 | Compiled. |
| `cargo test --locked --lib single_label_f04_asymmetric_hard_metrics -- --nocapture` | 0 | 1 passed, 29 filtered. |
| `cargo test --locked --lib single_label_thread_and_abstention_metrics -- --nocapture` | 0 | 1 passed, 29 filtered. |
| `cargo test --locked --lib single_label_literal_abstain_and_undefined_cases -- --nocapture` | 0 | 1 passed, 29 filtered. |
| `cargo test --locked --lib` | 0 | 30 passed, 0 failed. |
| `cargo test --locked --lib metric_status_precedence -- --nocapture` | 0 | 1 passed, 29 filtered. |
| `cargo test --locked --lib count_overflow_is_error -- --nocapture` | 0 | 1 passed, 29 filtered. |
| `cargo test --locked --lib macro_undefined_classes -- --nocapture` | 0 | 1 passed, 29 filtered. |
| `git diff --check` | 0 | No whitespace errors. |

Two transient development failures were corrected before final evidence: an
initial scorer compile had one `&u64`/`u64` iterator mismatch (exit 101), and a
test assertion temporarily assumed `selected` scope for selective metrics (exit
101). Neither remains in the candidate. A format-only precheck also returned exit
1 before `cargo fmt --all`; the final format check above is green.

`cargo check --locked` emits 143 dead-code warnings because T014 has not yet
wired the evaluator into the real application API. The new unwired scorer
diagnostics are `checked_add`, `checked_mul`, `checked_sub`, and every pure
function in `evaluation/single_label.rs` from `evaluate` through `sum`; the typed
result structures are likewise presently consumerless. The library test builds
emit 21 remaining pre-existing/incomplete-consumer dead-code warning groups. No
warning was suppressed and no visibility was widened to hide it.

The named integration filters `single_matrix_identities` and
`f04_asymmetric_oracle` remain pending as required: there is no
`tests/conformance.rs` or T014 real application API yet, and I did not add a
placeholder, facade, fake filter, or duplicate scorer. Full Clippy, all-target,
and release gates likewise remain the T014 boundary.

The final hash command emitted only the host `shasum` locale fallback warning
(`LC_ALL=C.UTF-8`, `LANG=en_US.UTF-8` unsupported); it did not affect the printed
SHA-256 values or its exit status 0.
