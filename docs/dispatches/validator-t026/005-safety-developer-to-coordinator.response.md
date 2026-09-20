# T026 output 1 completion handoff

This completes prompt002 output 1. Responses002 and003 remain historical partial handoffs. Output2 and later work are not started.

## Reproduced defect and correction

The real multi-label run selects `01995c20-7d00-7000-8000-000000000011`, has labels `A`/`B`, source `a`, and two copied evidence inputs. The unmodified run passed public inspect and compare. Before correction, storing `raw.exact_match_accuracy.value` as `1.00000000005` while recomputation yielded `1.0` caused both public operations to reject with `E_INVARIANT` at `accounting`; comparison published no output. The failing run remains at `/var/folders/6c/shrfzrvj5fx6gc0by_g7gg1r0000gn/T/validator-multi-replay-94006-54`.

`src/app.rs` now extends the existing explicit complete-path classifier without changing either tolerance constant (`1e-12` absolute; `1e-10` relative) or any single-label rule. New closed multi-label computed paths are:

- `raw`/`final` hard metric `.value`: exact-match, wrong-set, coverage, abstention, selective exact-match/risk, answered micro precision/recall/F1, and answered Hamming loss.
- `raw`/`final` per-label precision/recall/F1 `.value` and nested answered macro-F1 metric `.value`.
- Per-label and mean binary log-loss/Brier `.value` paths.
- Per-label marginal-bin `mean_probability` and `observed_positive_rate` paths.

Counts, ratio operands, status, populations, arrays, bin boundaries, sources, preparation, observations, and metric-shaped recorded configuration remain exact.

## Matrix assertions

Both accepted `replay_binding_and_result_tampering` and required `artifact_adversarial_matrix` call the same complete real-API scenario. It has single- and multi-label valid inspect/compare controls. Every invalid case calls inspect and compare, asserts the typed code/stage, and proves its unique output directory remains absent.

- Golden/predictions/config snapshots, evidence bytes, symlink escape, missing/extra/duplicate/swapped bindings, wrong original/ordinal/index all reject `E_PROVENANCE`/`replay`.
- Source-array, count, recorded configuration, and metric-shaped recorded configuration reject `E_INVARIANT`/`accounting`; status rejects `E_SCHEMA`/`schema`.
- Single `raw.accuracy.value` and multi representative raw/final hard, per-label, macro-F1, probability, and bin fields all accept an inside-tolerance stored value through inspect and compare.
- Multi `raw.exact_match_accuracy.value = 1.01` and its `numerator = 2` each reject `E_INVARIANT`/`accounting` with no comparison publication.

Representative multi stored values accepted by both public operations include `raw.exact_match_accuracy.value = 1.00000000005`, `final.wrong_set_rate.value = 0.0000000000005`, raw precision/final F1 at `1.00000000005`, raw/final macro-F1 at `0.50000000005`, per-label log loss `0.2231435513242097`, mean Brier `0.040000000002`, and bin mean/observed values `0.80000000005`/`1.00000000005`.

## Remaining output-1 evidence

`case_s23` covers public no-overwrite file/directory/symlink, late failure, and no publication. `case_s24` plus the CLI matrix and owning binding test cover relocation, copied same-basename/parent-relative evidence, original removal, UTF-8 ordering, repeated paths, and ordinals. `case_s25` invokes the matrix. `case_s26` independently hashes the exact comparison receipt file. `case_s27` and the real CLI matrix keep input/evidence sentinels out of routine surfaces; explicit inspection returns the input. `case_s28` preserves integer `9007199254740993`. The CLI matrix exercises single- and multi-label real process paths, including exit 3 / `E_OUTPUT_EXISTS`, receipts, privacy, and replay.

## Completion boundary

All commands exited 0; Cargo emitted only the established 17 dead-code warnings.

```text
cargo test --locked --test conformance replay_binding_and_result_tampering -- --nocapture  # 1 passed
cargo test --locked --test conformance artifact_adversarial_matrix -- --nocapture          # 1 passed
cargo test --locked --test cli publication_and_privacy_matrix -- --nocapture                # 1 passed
cargo test --locked --test conformance case_s23 -- --nocapture                              # 1 passed
cargo test --locked --test conformance case_s24 -- --nocapture                              # 1 passed
cargo test --locked --test conformance case_s25 -- --nocapture                              # 1 passed
cargo test --locked --test conformance case_s26 -- --nocapture                              # 1 passed
cargo test --locked --test conformance case_s27 -- --nocapture                              # 1 passed
cargo test --locked --test conformance case_s28 -- --nocapture                              # 1 passed
cargo test --locked --test conformance -- --list                                            # 81 listed
cargo test --locked --test cli -- --list                                                    # 11 listed
cargo test --locked --test conformance                                                      # 81 passed
cargo test --locked --test cli                                                              # 11 passed
cargo fmt --all -- --check                                                                  # passed
git diff --check                                                                             # passed
```

## Final hashes

| Artifact | SHA-256 |
|---|---|
| `src/app.rs` | `007c00ac10a4db21f19f7f21cdbd3861c9581fed03c864bc59debd514d54d800` |
| `src/artifacts.rs` | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |
| `tests/conformance.rs` | `b0f8f3817245d6bb45e4721fb44dd728fa0231e9c21306199ae350b1fcc9555f` |
| `tests/cli.rs` | `e0bf26cd96c2c06a221572c748e20818e66638e7227cc789e2f62705acca9197` |
| `src/evaluation/multi_label.rs` | `c4158facd454e66e66b3f1ddc0bb9976ca07fca6961bf200597ba9b31107b461` |
| `Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |
| `002-safety-developer-to-coordinator.response.md` | `431d340af7de41bda8b533a75b0080d7bb5babf0ff75c677808146f1d531c016` |
| `003-safety-developer-to-coordinator.response.md` | `5bf4abeb34fa483a540ff24f74a4e8c9f91e86e4e4c20363e99ca711bd9bd60c` |

The coordinator owns Fizzy, artifact-index, and acceptance-evidence updates.

