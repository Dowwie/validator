# T019 multi-label hard accounting handoff

Status: complete local implementation candidate. This implements only T019's
private concrete hard-accounting milestone. No T020 probability metrics,
application/CLI/schema integration, public conformance facade, policy execution,
comparison, governance, artifact-index, session-note, or Fizzy work was performed.

## Implementation and criterion map

`src/evaluation/multi_label.rs` defines `evaluate(&MultiLabelEvaluation)` and
accepts no wire or generic task input. It borrows the validated rows and computes
each raw and final family independently under the present `AsRecorded` policy.
`N` is `total`, `U` is `abstained`, `G` is `answered`, `D` is `exact_matches`,
and `E` is `wrong_sets`. `score`, `verify_accounting`, `checked_add`,
`checked_sub`, and `checked_mul` reject overflow or accounting mismatches.

`src/model/multi_label.rs` adds the concrete `MultiLabelHardResults`,
`MultiLabelLabelMetrics`, and `MultiLabelMacroF1` result contracts. Every hard
family binds exact-set metrics to selected episodes and selective metrics to
answered episodes. Per-label precision/recall/F1 are answered-episode metrics;
micro F1 and Hamming use answered `label_decision` populations. Undefined
per-label and micro denominators remain null. `N=0` uses `no_data`, and
`N>0,G=0` uses `no_answered_predictions`. Macro F1 zero-fills only its average
and records the affected label names with `contains_undefined_classes`.

For every answered row, label accounting assigns exactly one TP/FP/FN/TN state
per vocabulary label. `support` counts references over `N`; `answered_support`
and `predicted_support` use `TP+FN` and `TP+FP` over `G`. The evaluator verifies
every four-count sum equals `G` and the aggregate sum equals `G*K`.

`MultiLabelEpisodeEvidence` contains separate `raw` and `final_outcome`
`MultiLabelOutcomeEvidence` structures. Answered evidence stores expected,
predicted, matched, missed, and extra vectors in vocabulary order plus
correctness. Abstentions use `MultiLabelEpisodeStatus::Abstained` with null
predicted/matched/missed/extra/correct fields, preserving the source,
observations, and abstention reason without inventing an empty answer or a label
substitution.

`tests/fixtures/multi-label/expected.json` is hand-authored and records rational
or status expectations for the two-row aggregate, answered-empty, abstention,
all-abstained, empty-population, and equal-binary-counts cases. The local test
deserializes it as `serde_json::Value`; no production scoring code reads its
shape. The two-row derivation is `TP=1, FP=1, FN=1, TN=3`, hence micro F1
`2/(2+1+1)=1/2`, macro F1 `(2/3+0+1/3)/3=1/3`, and Hamming `2/(2*3)=1/3`.
The equal-counts case keeps each label at one TP and one FP in both arrangements,
while exact-set accuracy is `1/2` versus `0/2`.

`evaluation::multi_label::tests::multi_label_hard_accounting` asserts all
required oracle/status/evidence cases against both raw and final hard families,
including selected versus answered scope/unit fields, all four-count totals,
empty answered sets, abstention null evidence, all-abstained, and `N=0`.
`evaluation::multi_label::tests::equal_binary_counts_distinct_exact_sets` asserts
the discriminator that aggregate binary counts cannot substitute for exact-set
accuracy.

## Verification

| Command | Exit | Actual result |
|---|---:|---|
| `cargo test --locked --lib multi_label_hard_accounting -- --nocapture` | 0 | 1 selected, 1 passed |
| `cargo test --locked --lib equal_binary_counts_distinct_exact_sets -- --nocapture` | 0 | 1 selected, 1 passed |
| `cargo test --locked --lib -- --list` | 0 | Both required names present; each has one test |
| `cargo fmt --all -- --check` | 0 | passed |
| `cargo test --all-features --locked` | 0 | 44 library, 6 CLI, 11 conformance passed |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | only `dead_code`; 69 library and 11 library-test diagnostics |
| `git diff --check` | 0 | passed |

The Clippy result has no ordinary or new warning class. Its 80 emitted
`dead_code` diagnostics comprise the prior T018/T021 staged private contracts,
plus these T019 symbols whose only real consumer is T020's closed evaluation and
typed report integration: `checked_mul`; `evaluation::multi_label::{evaluate,
score, Counts, label_metrics, macro_f1, AggregateCounts, aggregate_counts,
verify_accounting, selected_metric, answered_episode_metric,
answered_label_metric, evidence, outcome_evidence, labels, same_set,
intersection, difference, count}`; and the hard-result/evidence fields of
`MultiLabelLabelMetrics`, `MultiLabelHardResults`, and
`MultiLabelEpisodeEvidence`. T020 will call the evaluator from the real closed
application path and serialize these concrete results. No suppression, fake use,
widened visibility, or placeholder path was added.

## Hashes

Starting fixed inputs matched exactly:

| Artifact | SHA-256 |
|---|---|
| `docs/plans/validator/tasks/T019.json` | `0d6be8fa0d8b48cf4372e2a353c860420b75a54107c30446e21c4efd0cdc11f1` |
| `docs/specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `docs/specs/validator-data-model.md` | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| `docs/plans/validator/execution-contract.md` | `cdd917974df4fd505f91d165039988ca01d0936ad2e7703da47f30c7b3041e8f` |
| `docs/dispatches/validator-t018-t020/003-t018-developer-to-coordinator.response.md` | `034a17855aed0330cce60ebfbc73ebf8125e059ae388e742d360850b278bf079` |
| `src/model.rs` | `846677319091911cd5f1a9a869aa45700b8ce46caf0923e779f74100c71dd4f6` |
| `src/model/common.rs` | `dfe120adb33f7a773edd5a8c44c8331fb918e417aec52dfc3d1c0a8077138791` |
| `src/model/multi_label.rs` | `26fd8a8798dcf14947e7ea9a8b1548979ca204eb84b2abff2cff7422b0b938c5` |
| `src/evaluation.rs` | `dbee2b6c74f60d5375545b746dbfea3ecc311ad55f3183ab02bdc260157bf891` |
| `src/evaluation/multi_label.rs` | absent (created by T019) |
| `tests/fixtures/multi-label/expected.json` | absent (created by T019) |

Final relevant artifacts:

| Artifact | SHA-256 |
|---|---|
| `docs/plans/validator/tasks/T019.json` | `0d6be8fa0d8b48cf4372e2a353c860420b75a54107c30446e21c4efd0cdc11f1` |
| `docs/specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `docs/specs/validator-data-model.md` | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| `docs/plans/validator/execution-contract.md` | `cdd917974df4fd505f91d165039988ca01d0936ad2e7703da47f30c7b3041e8f` |
| `docs/dispatches/validator-t018-t020/003-t018-developer-to-coordinator.response.md` | `034a17855aed0330cce60ebfbc73ebf8125e059ae388e742d360850b278bf079` |
| `src/model.rs` | `846677319091911cd5f1a9a869aa45700b8ce46caf0923e779f74100c71dd4f6` |
| `src/model/common.rs` | `dfe120adb33f7a773edd5a8c44c8331fb918e417aec52dfc3d1c0a8077138791` |
| `src/model/multi_label.rs` | `df2b98c78f4ebed9945be256b0371b06705729e3053ef4319b5a878cdc392081` |
| `src/evaluation.rs` | `172ab7db055ca9483e3b64593411bb9e64c1983eecdbd06013fe0f6ce1a483ff` |
| `src/evaluation/multi_label.rs` | `2d6ec1dd615a8aac4882a6330b5e08319bc1385aa9f1e70b462c116bd4ea8e4c` |
| `tests/fixtures/multi-label/expected.json` | `a65de157415f75567cb907f2dd45ae3d524dcb552dce7c10ef9ca87efbfc63ec` |

No blocker remains for T020 to consume this local hard-accounting contract.
