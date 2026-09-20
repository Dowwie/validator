# T026 output 2 completion handoff

This completes the source, comparison, and preparation evidence assigned by
prompt008. The implementation changes only `tests/conformance.rs`; it adds the
ten required named public-path tests. Output1 and the accepted T025 behavior are
unchanged. No production defect was reproduced and no production correction was
made.

## Case evidence

All assertions below use fixed JSON inputs and fixed IDs before reading a report
or comparison result. Evaluation, inspection, and comparison calls use the
public library API. `publication_and_privacy_matrix` is the required real CLI
process evidence; it was executed separately.

| Case | Actual assertions |
|---|---|
| `case_s15` | A two-row run fixes `Q1`/retained A and `Q2`/revised B. It asserts `mixed_source`, counts `{Q1:1,Q2:1}`, both row source IDs, both full source models/configurations in the verified report, and inspected Q2 row provenance. A real self-comparison asserts both source definitions/counts are published. |
| `case_s16` | Missing `source_id` rejects through the public evaluator as `E_SCHEMA`, unknown `source_id` rejects as `E_PROVENANCE`, and neither publishes a run. Two valid runs deliberately reuse local ID `source` with configuration revisions 1 and 2; real comparison emits `sources.source.configuration.revision`. |
| `case_s18` | Two whole golden byte inputs differ by reference content (`first` versus `revised`), producing unequal SHA-256 digests. The accepted first run snapshot remains byte-identical; default public comparison rejects the different golden populations as `E_COMPARISON` and publishes no directory. |
| `case_s19` | Fixed selected sets `{1,2}` and `{2,3}` reject by default. Explicit intersection recomputes only ID 2, exposes baseline/candidate exclusions and counts of one each, and produces the independently fixed accuracies 0 and 1. |
| `case_s20` | Three separate empty intersections cover categorical/categorical, hard-label/hard-label, and categorical/hard-label. Both hard sides are `no_data` with null values; probability sides are respectively `no_data/no_data`, `not_applicable/not_applicable`, and `no_data/not_applicable`, with null values and deltas. Each consumes a real comparison receipt. |
| `case_s22` | The fixed five-row comparison preserves baseline and candidate accuracy 0.6 while asserting the exact recovered ID 2 and regressed ID 3 lists. |
| `case_m17` | Reference `{A,B}` changes from `{A}` to `{B}`. It asserts `neither_correct`, the exact changed-outcome ID, and vocabulary-ordered A `present→absent` and B `absent→present` transition counts. |
| `case_m18` | The fixed multi-label comparison asserts final answered baseline/candidate populations `{1,3}` and `{1,2}`, counts two each, overlap one, and the conditional selective metric's `answered` scope on both sides. |
| `case_m19` | Empty multi-label intersections separately retain present marginal availability (`no_data`) and absent marginal availability (`not_applicable`). Both exact-match hard sides are `no_data`, values and mean-binary-log-loss deltas are null. |
| `case_e08` | A retained observation stays `{A:0.5,B:0.49}` (sum 0.99), while the explicitly externally prepared canonical scoring vector is `{A:0.5,B:0.5}` and has matching working values. The report and verified inspection assert both fields. The source preparation descriptor, indices `[0,1,2]`, and distinct script/raw/receipt evidence bindings and paths are asserted. This does not assert calibration or an in-process fallback. |

## Executed filters and gates

Each of these literal filters was invoked separately and executed one nonzero
test with exit 0: `case_s15`, `case_s16`, `case_s18`, `case_s19`, `case_s20`,
`case_s22`, `case_s23`, `case_s24`, `case_s25`, `case_s26`, `case_s27`,
`case_s28`, `case_m17`, `case_m18`, `case_m19`, `case_e08`,
`artifact_adversarial_matrix`, `replay_binding_and_result_tampering`, and the
CLI `publication_and_privacy_matrix`.

The full gates ran serially:

```text
cargo fmt --all -- --check                                      # exit 0
cargo clippy --all-targets --all-features --locked -- -D warnings # exit 101
cargo test --all-features --locked                              # exit 0
cargo build --release --locked --bin validator                  # exit 0
git diff --check                                                # exit 0
```

Clippy's exit 101 contains only the accepted 17 production and three duplicate
lib-test staged dead-code diagnostics; it contains no new diagnostic. The full
test run passed 44 library, 11 CLI, 91 conformance, and zero doc tests. Separate
list commands confirmed 91 conformance and 11 CLI tests.

## Hashes

| Artifact | SHA-256 |
|---|---|
| `tests/conformance.rs` (changed) | `87230d024ab18e0b3c1b61a82dec7414593e782f8c769d43ef6839fee55f0812` |
| `src/app.rs` (unchanged) | `007c00ac10a4db21f19f7f21cdbd3861c9581fed03c864bc59debd514d54d800` |
| `src/artifacts.rs` (unchanged) | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |
| `tests/cli.rs` (unchanged) | `e0bf26cd96c2c06a221572c748e20818e66638e7227cc789e2f62705acca9197` |
| `src/evaluation/multi_label.rs` (unchanged) | `c4158facd454e66e66b3f1ddc0bb9976ca07fca6961bf200597ba9b31107b461` |
| `Cargo.toml` (unchanged) | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `Cargo.lock` (unchanged) | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |
| `rust-toolchain.toml` (unchanged) | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |

The governing owner prompt, output1 handoff, task contract, both ratified specs,
and all non-test starting identities retain their supplied SHA-256 values.

The combined T026 developer boundary is complete and ready for the coordinator's
frozen independent review. No review, T027 work, Fizzy mutation, or governance
artifact was started.
