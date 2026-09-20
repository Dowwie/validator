# T026 exact-filter repair completion

This is the sole authorized finding-driven repair cycle from prompt011. Only
`tests/conformance.rs` changed. No public-contract mismatch was reproduced, so
no production, schema, fixture, tolerance, or settled T025/T026 behavior changed.

## Verifier findings resolved

| Finding | Discriminating repair assertion |
|---|---|
| R1 / S15 origins | Both fixed Q1 and Q2 rows are inspected through the public API. Each asserts its fixed episode ID, expected class, and `prediction.source_id`. The self-comparison asserts source counts plus Q1/Q2 model and complete configuration on both baseline and candidate sides. |
| R2 / S24 relocation | After relocation and deletion of every original golden, prediction, config, and evidence input, the test retains verified inspection and uses public self-comparison on the relocated run. It asserts receipt `result_path` and actual `comparison.json` publication. |
| R3 / S26 evaluation receipt | A real second evaluation of the same fixed input retains its receipt. The test asserts its absolute `report.json` path and compares `result_sha256` with an independently computed SHA-256 of the exact report bytes, in addition to the existing comparison-receipt proof. |
| R4 / S27 error privacy | Public inspection of a fixed unknown selected ID yields `E_ID`. Its real `to_machine_json()` result is checked for the absence of both opaque-input and evidence sentinels, while the existing report privacy and successful explicit inspection assertions remain. |
| R5 / M17 vocabulary order | The exact `transitions.per_label` label sequence is asserted as `[A,B]` before asserting A `present→absent` and B `absent→present` cells. |
| R6 / M18 conditional metrics | The fixed inputs independently imply two answered rows per side, one shared row, and selective exact-match accuracy `1/2` for each side. The test asserts `defined`, baseline/candidate `0.5`, delta `0.0`, answered populations and scope, and absence of root/final `winner` and `improvement` representations. |

## Executed checks

Every command below exited as shown:

```text
cargo test --locked --test conformance case_s15 -- --nocapture                         # 0
cargo test --locked --test conformance case_s24 -- --nocapture                         # 0
cargo test --locked --test conformance case_s26 -- --nocapture                         # 0
cargo test --locked --test conformance case_s27 -- --nocapture                         # 0
cargo test --locked --test conformance case_m17 -- --nocapture                         # 0
cargo test --locked --test conformance case_m18 -- --nocapture                         # 0
cargo test --locked --test conformance artifact_adversarial_matrix -- --nocapture       # 0
cargo test --locked --test conformance replay_binding_and_result_tampering -- --nocapture # 0
cargo test --locked --test cli publication_and_privacy_matrix -- --nocapture            # 0
cargo test --locked --test conformance -- --list                                        # 0; 91
cargo test --all-features --locked                                                       # 0; 44 library, 11 CLI, 91 conformance, 0 doc
cargo clippy --all-targets --all-features --locked -- -D warnings                       # 101; accepted 17+3 staged diagnostics only
cargo fmt --all -- --check                                                               # 0
git diff --check                                                                         # 0
```

The release build was deliberately not repeated: prompt011 directs reuse of the
unchanged release evidence, and this test-only correction introduced no release
surface change.

## Exact identities

| Artifact | SHA-256 |
|---|---|
| `009-coordinator-candidate-manifest.md` | `fcb905dcee73958340458b88433e5bd43cc2cc78cfadd37470a19ed50a1efb9f` |
| `010-verifier-to-coordinator.response.md` | `06ded06b8faacd28c19ef80d32288d312cc15563ef83a36140763b5ad71e0b45` |
| `tests/conformance.rs` (changed) | `a685073e0e136f0f16167594833c58ef18dc024d6733a9f1d5014d665b59950d` |
| `src/app.rs` (unchanged) | `007c00ac10a4db21f19f7f21cdbd3861c9581fed03c864bc59debd514d54d800` |
| `src/artifacts.rs` (unchanged) | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |
| `tests/cli.rs` (unchanged) | `e0bf26cd96c2c06a221572c748e20818e66638e7227cc789e2f62705acca9197` |

All six exact-filter proofs are complete for the same verifier's focused recheck.
