# T026 output 1: artifact and publication safety handoff

Scope completed: the named output-1 matrices and `case_s23` through `case_s28`
are present and execute through the real public API or the compiled CLI. No
production source changed. This is a local developer handoff only; it does not
claim T026, its final gates, or owner acceptance is complete.

## Changed files

- `tests/conformance.rs`
  - Renamed the existing replay corruption scenario to
    `artifact_adversarial_matrix` and extended it with independent golden,
    predictions, and config snapshot corruption.
  - Added the exact S23--S28 filters. They use isolated temporary runs and the
    public `evaluate`, `inspect`, and `compare` API paths.
- `tests/cli.rs`
  - Added `publication_and_privacy_matrix`, which executes the existing real
    multi-label CLI scenario and the end-to-end CLI scenario as one required
    nonzero matrix filter.

## Requirement mapping

| Requirement | Public assertion and execution owner |
|---|---|
| Copied-evidence relocation, original removal, same-basename and parent-relative paths | `publication_and_privacy_matrix` runs `steel_thread_end_to_end`, which creates two same-basename `evidence.bin` inputs in distinct directories, retains a parent-relative binding, relocates both runs, removes the input/source trees, then runs CLI inspect and compare from stored files. `src/artifacts.rs::evidence_ordinal_binding` independently checks UTF-8 byte source ordering, per-source indexes, repeated paths, and ordinal storage. `case_s24` separately relocates a public-API run, deletes original golden/prediction/config/evidence files, and verifies inspection. |
| Snapshot, result, binding, path, evidence, symlink, source-array corruption | `artifact_adversarial_matrix` independently mutates stored golden/predictions/config bytes, evidence bytes, escaping evidence symlink, missing/extra/duplicate binding entries, swapped ordinal path, wrong original string, wrong index, report source evidence array, stored count/status, and recorded configuration. Every mutation except the explicit computed metric inside tolerance rejects `inspect` through its typed path. `case_s25` invokes that matrix as its exact filter. `case_s23` additionally verifies that damaged replay rejects `compare` and leaves its comparison destination absent. |
| Replay tolerance only on fixed computed paths | `artifact_adversarial_matrix` accepts a changed `raw.accuracy.value` within declared tolerance and rejects changed count, status, recorded configuration, and metric-like configuration fields. |
| No replacement and late failure | `case_s23` uses public `evaluate` against existing file, directory, and symlink outputs, asserts `E_OUTPUT_EXISTS`, and checks each target unchanged. It also asserts an invalid late evaluation creates no output, and a damaged replay publishes no comparison. `src/artifacts.rs::publish_no_replace_race` retains the deterministic pre-publish creation seam; `publish_late_failure` proves removal of the temporary sibling. The CLI matrix exercises actual process exit 3 / `E_OUTPUT_EXISTS` for evaluation and comparison destinations. |
| Receipt path and exact hash | `case_s26` uses public `compare`, asserts the absolute `comparison.json` path, and recomputes SHA-256 from its exact bytes. The CLI matrix's existing evaluation and comparison receipts also independently hash `report.json` and `comparison.json`. |
| Privacy and explicit inspection | `case_s27` asserts report bytes omit unique opaque-input and evidence-byte sentinels, then asserts selected inspection returns the exact opaque input. The CLI matrix checks routine stdout, errors, reports, and receipts on the real binary without input/evidence sentinels and checks explicit inspection. |
| Exact integer preservation | `case_s28` stores `9007199254740993_u64`, uses public inspection, and asserts the JSON number exactly. The CLI relocation scenario also asserts the literal appears in inspected JSON. |
| Both task kinds and CLI categories | The CLI matrix executes the multi-label CLI scenario and the complete single-label CLI end-to-end scenario. They verify stdout schemas, safe errors, real exit categories, receipts, relocation, replay corruption, and no successful comparison publication. |

## Exact filter execution

All commands below exited 0. Cargo reported the pre-existing 17 dead-code
warnings; no warning suppression or source change was made.

```text
cargo test --locked --test conformance artifact_adversarial_matrix -- --nocapture  # 1 passed
cargo test --locked --test cli publication_and_privacy_matrix -- --nocapture       # 1 passed
cargo test --locked --test conformance case_s23 -- --nocapture                     # 1 passed
cargo test --locked --test conformance case_s24 -- --nocapture                     # 1 passed
cargo test --locked --test conformance case_s25 -- --nocapture                     # 1 passed
cargo test --locked --test conformance case_s26 -- --nocapture                     # 1 passed
cargo test --locked --test conformance case_s27 -- --nocapture                     # 1 passed
cargo test --locked --test conformance case_s28 -- --nocapture                     # 1 passed
cargo test --locked --test conformance -- --list                                   # 80 tests listed
cargo test --locked --test cli -- --list                                           # 11 tests listed
cargo test --locked --test conformance                                             # 80 passed
cargo test --locked --test cli                                                     # 11 passed
cargo fmt --all -- --check                                                         # passed
git diff --check                                                                    # passed
```

After the final matrix-only extension for predictions/config snapshots, I reran
the matrix, `case_s25`, both complete affected targets, format, and diff checks;
all passed.

## Production-defect disposition

No ratified T026 integrity, replay, or publication violation was reproduced.
No production correction was made. The only failure during implementation was a
test-only selected-episode helper that used a fixed fixture UUID; the S27/S28
tests now pass their own selected UUID directly to the public inspection API.

## Final identities

| Artifact | SHA-256 | State |
|---|---|---|
| `src/artifacts.rs` | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` | unchanged |
| `src/app.rs` | `f23a95f6c6f4d0f6c33cd872567ef14526243f053673a973ee0021a94d79012a` | unchanged |
| `tests/conformance.rs` | `c3f488ad0b23c39979d8150680f79e8819f828098a71b971fd43118f62a5d560` | changed |
| `tests/cli.rs` | `e0bf26cd96c2c06a221572c748e20818e66638e7227cc789e2f62705acca9197` | changed |
| `src/evaluation/multi_label.rs` | `c4158facd454e66e66b3f1ddc0bb9976ca07fca6961bf200597ba9b31107b461` | unchanged |
| `Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` | unchanged |
| `Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` | unchanged |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` | unchanged |

The coordinator owns required Fizzy, artifact-index, and accepted-evidence
updates. Output 2 remains outside this dispatch's scope.
