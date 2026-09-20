# T025 multi-label per-label hard-metric population-unit correction

## Scope and fixed expectation

I reconciled all dispatch017 starting hashes, read verdict015 and owner prompt016,
and applied prompt019's owner018 clarification. The contract was fixed before
production inspection: every multi-label per-label precision, recall, and F1 has
population `G`, unit `label_decision`, and scope `answered`, including status-only
results. Aggregate binary metrics retain `G*K` label decisions; set metrics retain
episode units.

Only `src/evaluation/multi_label.rs` and `tests/conformance.rs` changed. Fixtures,
schemas, tolerances, formulas, ratio operands, values, statuses, count accounting,
probability metrics, policies, and comparison semantics remain unchanged.

## Owning-layer correction

`label_metrics` now constructs per-label precision, recall, and F1 with the
existing `answered_label_metric`, passing `answered` as the per-label
label-decision population. This replaces only its three prior
`answered_episode_metric` calls. The existing helper preserves status precedence,
numerator/denominator, value, answered scope, and total context while serializing
`MetricUnit::LabelDecision`.

## Public-path assertions

| Scenario | Assertions |
|---|---|
| Normal answered M01 | Raw/final per-label precision/recall/F1 retain derived values, statuses, and operands with `G=2`, `label_decision`, `answered`. Aggregate micro/Hamming/macro retain `G*K=6` label decisions; exact-set metrics retain episodes. |
| Answered empty-set M02 | Raw/final A/B precision, recall, F1 retain null `undefined_zero_denominator`, `TP=FP=FN=0,TN=1`, and `G=1`, `label_decision`, `answered`. Each F1 explicitly proves present `numerator: 0` and `denominator: 0`. |
| Every-row-abstained M04 | Raw/final per-label precision/recall/F1 retain null `no_answered_predictions` with `G=0`, `label_decision`, `answered`; set coverage/exact metrics remain episodes. |
| Empty selection M04 | Raw/final per-label precision/recall/F1 retain null `no_data` with `N=G=0`, `label_decision`, `answered`; aggregate micro and exact-set unit checks prevent blanket replacement. |
| Verified persisted report/inspection | `multi_label_abstentions_preserve_wire_evidence_in_report_and_inspection` checks persisted raw/final report per-label metadata before real verified inspections. |

No expected numeric value, ratio, status, or unrelated public output changed.

## Command evidence

| Command | Exit | Result |
|---|---:|---|
| Serial `case_m01`, `case_m02`, `case_m04` | 0 | Each: 1 passed, 73 filtered. |
| `exhaustive_single_label` | 0 | 1 passed, 73 filtered; 301 populations. |
| `exhaustive_multi_label` | 0 | 1 passed, 73 filtered; 64 answered pairs plus 8 abstentions. |
| `full_numeric_conformance` | 0 | 1 passed, 73 filtered. |
| `multi_label_abstentions_preserve_wire_evidence_in_report_and_inspection` | 0 | 1 passed, 73 filtered. |
| `cargo test --locked --test conformance -- --list` | 0 | Unchanged 74-test inventory. |
| `cargo fmt --all -- --check` | 0 | Formatting valid. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Only accepted unchanged 17 production plus 3 duplicate lib-test diagnostics. |
| `cargo test --all-features --locked` | 0 | 44 library, 10 CLI, 74 conformance, and 0 doc tests passed. |
| `cargo build --release --locked --bin validator` | 0 | Release binary built. |
| `git diff --check` | 0 | No whitespace errors. |

Clippy retains only the staged dead-code inventory in `src/model.rs`,
`src/model/common.rs`, `src/model/single_label.rs`, and `src/validation/wire.rs`,
plus duplicate lib-test diagnostics for `GoldenDataset.schema_version`,
`GoldenEpisode.input`, and `PredictionArtifact.schema_version`. The correction
introduces no diagnostic.

## Final identities

| Artifact | SHA-256 |
|---|---|
| `src/evaluation/multi_label.rs` changed | `c4158facd454e66e66b3f1ddc0bb9976ca07fca6961bf200597ba9b31107b461` |
| `tests/conformance.rs` changed | `13409645624e53b8a3f2b4e5719dd7ae651527e0c4f886dcb511bfa10bf38bef` |
| `tests/fixtures/single-label/expected.json` unchanged | `ab93c379b9c0df1d0c0fb7aac7a6df5e7748f3f671d55dd69d173e640090b268` |
| `tests/fixtures/multi-label/expected.json` unchanged | `213183a46c0f6c011296ef5087b5c44d3852081bdf20eaa80cbb494d3d2c84f2` |
| `schemas/v2/report.schema.json` unchanged | `805de93d2a50339370e803b7ff1a9a1b8d34c3028b5b21a1a6f428f591733f90` |
| `schemas/v2/inspection.schema.json` unchanged | `2c7e064d0adb915b77c558a7ebbfade96f7bcf0eee890b881ced1383f0eedaef` |

Verdict: **The bounded owner-authorized correction is complete and ready for
freezing and the same verifier's focused recheck. T026 and later work remain
unstarted.**
