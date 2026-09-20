# T016 compatibility-evidence completion handoff

Completed the response010 evidence limitation without changing comparison output,
models, schema, publisher, CLI, or task scope. Response010 remains the inherited
implementation evidence with SHA-256
`888e4646fe2455acaf690231642c43c152cfe0011546bb34d42cbee0dfb10c3e`.

## Changes and identity

| Artifact | Starting SHA-256 | Final SHA-256 |
| --- | --- | --- |
| `src/comparison.rs` | `a3e918d662aeb2ab40b5b518d01592964b145084f9f0065e7d2fb15d9053fa38` | `a0288c9d79f116b03925802a52771dff0e4894506a23701b9b15e6cb36b02961` |
| `tests/conformance.rs` | `92edf9bfdc0c2da990052a966b3fd4dabc033fb28d741fc4590080b74a59a593` | `bdf526a6a60fde605c6c65e5691fd7b4e397642b0166cf216b1a169f05d3e353` |

`CompatibilityFacts` is a private extraction of the exact facts already consumed
by `compatible`: golden digest, ordered labels, evaluation role, selected IDs,
categorical tolerance, and bin count. `compatible` still receives only concrete
single-label comparison inputs. The owning-module test proves each runtime axis
returns `E_COMPARISON`; task kind is fixed by `ComparisonInput`'s
`SingleLabelEvaluation`, so no generic task-kind branch exists.

`comparison_compatibility_and_deltas` now creates a separate, valid held-out
evaluation from the same golden and candidate predictions, confirms evaluation
succeeds, then compares it with the verified development run. It asserts the
actual comparison call returns `DiagnosticCode::Comparison` (`E_COMPARISON`),
proving replay did not short-circuit the compatibility check. It retains the
concrete direction, source-difference, undefined-macro, and positive-infinity
assertions from response010.

## Verification

| Command | Result |
| --- | --- |
| `cargo test --locked comparison::tests::typed_compatibility_facts_reject_each_runtime_axis -- --nocapture` | Exit 0, 1 passed |
| `cargo test --locked --test conformance comparison_compatibility_and_deltas -- --nocapture` | Exit 0, 1 passed; real held-out pair returns `E_COMPARISON` |
| `cargo test --locked --test conformance single_comparison_transitions -- --nocapture` | Exit 0, 1 passed |
| `cargo test --locked --test cli cli_compare_receipt -- --nocapture` | Exit 0, 1 passed |
| `cargo fmt --all -- --check` | Exit 0 |
| `cargo test --all-features --locked` | Exit 0; 40 unit, 5 CLI, 11 conformance tests passed |
| `git diff --check` | Exit 0 |

No production behavior changed beyond the private fact extraction used by the
existing compatibility check. Response010's release-build and warning-denied
Clippy evidence remains applicable; the residual owner-staged dead-code inventory
is unchanged at 24 diagnostics. No T017, intersection, multi-label,
winner/significance, governance, index, or Fizzy work was performed.
