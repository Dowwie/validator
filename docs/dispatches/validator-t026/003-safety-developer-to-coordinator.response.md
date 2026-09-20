# T026 output 1: superseding artifact and publication safety handoff

This supersedes [response002](002-safety-developer-to-coordinator.response.md).
It retains the complete output-1 evidence and corrects its test inventory: the
accepted `replay_binding_and_result_tampering` filter is restored, while
`artifact_adversarial_matrix` remains a distinct T026 filter. No production
source changed. This is a local developer handoff; it does not claim T026 or
owner acceptance is complete.

## Inventory reconciliation

The initial output renamed the accepted replay test, leaving 80 conformance
tests. `tests/conformance.rs` now contains one shared test-only function,
`replay_binding_and_result_tampering_scenario`, with two separately executable
wrappers:

- `replay_binding_and_result_tampering` preserves the accepted filter and
  evidence identity.
- `artifact_adversarial_matrix` provides the T026 named matrix.

Both wrappers execute the same complete corruption workflow, so no corruption
logic was copied. The baseline 74 tests plus the matrix and S23--S28 filters now
list and pass exactly 81 tests.

## Output-1 requirement mapping

| Requirement | Actual assertion |
|---|---|
| Replay corruption and tolerance | The shared replay scenario independently mutates stored golden/predictions/config bytes; evidence bytes; escaping symlink; missing/extra/duplicate/swapped bindings; original string, ordinal path, and evidence index; report source-array agreement; stored count/status; and recorded configuration. It rejects every invalid case through typed inspection except the declared within-tolerance `raw.accuracy.value` mutation, and rejects metric-like configuration fields. `case_s25` executes the named matrix. |
| Relocation and stored evidence | `case_s24` relocates a public-API run, removes every original input/evidence file, and verifies inspection from stored snapshots. `publication_and_privacy_matrix` runs the existing real CLI multi-label and end-to-end single-label flows: the latter covers same-basename evidence, parent-relative evidence, relocation, original-tree removal, inspect, and compare. The owning `evidence_ordinal_binding` test checks UTF-8 byte source ordering, per-source indexes, repeated paths, and ordinal storage. |
| Publication safety and process evidence | `case_s23` verifies public evaluation cannot replace existing file, directory, or symlink outputs; it rejects late invalid input without a result; damaged replay rejects comparison without publication. Existing owning tests retain the deterministic no-replace creation seam and late-failure temporary-sibling cleanup. The CLI matrix verifies real exit 3 / `E_OUTPUT_EXISTS` for evaluation and comparison destinations. |
| Receipt integrity | `case_s26` uses public comparison, asserts the exact absolute `comparison.json` path, and computes SHA-256 over that file's bytes. The CLI matrix independently checks evaluation `report.json` and comparison receipts. |
| Privacy and exact inspection | `case_s27` proves routine report bytes omit unique opaque-input and evidence sentinels while selected public inspection returns exact opaque input. The CLI matrix checks stdout, errors, reports, and receipts. `case_s28` stores and inspects `9007199254740993_u64` as an exact JSON number. |

## Verification record

All commands exited 0. Cargo emitted the established 17 dead-code warnings; no
warning suppression or production change was made.

```text
cargo test --locked --test conformance replay_binding_and_result_tampering -- --nocapture  # 1 passed
cargo test --locked --test conformance artifact_adversarial_matrix -- --nocapture          # 1 passed
cargo test --locked --test conformance -- --list                                            # 81 listed
cargo test --locked --test conformance                                                      # 81 passed
cargo test --locked --test cli                                                              # 11 passed
cargo fmt --all -- --check                                                                  # passed
git diff --check                                                                             # passed
```

Response002 additionally records the independently executed output-1 filters:
the CLI matrix and each of S23--S28 passed, followed by the affected-target
tests, format, and diff checks. The restored replay wrapper changes no
production behavior and only adds the accepted test name back to the inventory.

## Production-defect disposition

No ratified artifact-integrity, replay, or publication defect was reproduced.
No production correction was made. The only implementation failure was a
test-only selected-episode helper with a fixed UUID; S27/S28 now pass their own
selected UUID to the public inspection API.

## Final identities

| Artifact | SHA-256 | State |
|---|---|---|
| `src/artifacts.rs` | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` | unchanged |
| `src/app.rs` | `f23a95f6c6f4d0f6c33cd872567ef14526243f053673a973ee0021a94d79012a` | unchanged |
| `tests/conformance.rs` | `9ea97165fdd895e11e682f8f89389b70c69f8df3ee5dcc08432016d4ebd12c4f` | changed |
| `tests/cli.rs` | `e0bf26cd96c2c06a221572c748e20818e66638e7227cc789e2f62705acca9197` | changed |
| `src/evaluation/multi_label.rs` | `c4158facd454e66e66b3f1ddc0bb9976ca07fca6961bf200597ba9b31107b461` | unchanged |
| `Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` | unchanged |
| `Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` | unchanged |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` | unchanged |
| `002-safety-developer-to-coordinator.response.md` | `431d340af7de41bda8b533a75b0080d7bb5babf0ff75c677808146f1d531c016` | superseded |

The coordinator owns Fizzy, artifact-index, and acceptance-evidence updates.
Output 2 remains outside this dispatch's scope.
