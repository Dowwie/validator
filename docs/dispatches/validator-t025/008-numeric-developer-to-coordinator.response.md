# T025 output 3: multi-label numerical conformance and final gates

## Scope and independence

I verified every starting hash in dispatch008. I read the ratified numerical and
data-model contracts, public schemas, and existing conformance setup. I did not
read `src/evaluation*`, production metric/count helpers, or scoring code to derive
an expected value. All cases construct their raw input and independently derive
integer, rational, or analytic expectations before calling the public API.

Only `tests/conformance.rs` changed. No fixture was added or changed. Production,
schemas, governance, plans, index, session notes, acceptance records, and Fizzy
were not edited.

## M-case and umbrella evidence

| Filter | Independent assertion through the public report/error path |
|---|---|
| M01 | `[A,B,C]`, `[{A,B},{}]` vs `[{A,C},{}]`: N/G/D/E/U, exact `1/2`, aggregate `(1,1,1,3)`, every label total `G`, micro `2/4`, macro `1/3`, Hamming `2/6`, and ordered matched/missed/extra sets. |
| M02 | Empty sets with a nonempty vocabulary: exact/coverage one, Hamming zero, null undefined micro-F1, macro zero and exact undefined-class list. |
| M03 | `[{A},{}]` vs `[abstention,{}]`: `N=2,G=1,U=1,D=1,E=0`, exact `1/2`, selective exact one, coverage `1/2`, A supports/TN, and null abstention differences. |
| M04 | All abstentions: raw/final coverage and exact zero, answered-only null `no_answered_predictions`, and no fabricated set/binary evidence. |
| M05 | `{A}`, marginals `.8/.7`: unchanged sum 1.5, per-label losses/Briers, mean `(-ln .8-ln .3)/2`, Brier `.265`, selected label-decision populations. |
| M06 | Raw `{A,B}` with thresholds A=.8/B=.75: equality admits A, final `{A}`, raw/final hard separation, and unchanged probability evidence. |
| M07 | Below-threshold marginals produce an answered empty set with empty differences, never abstention. |
| M08 | Empty reference and zero marginals produce defined zero binary losses/Briers and schema-valid finite JSON. |
| M09 | Present-at-zero and absent-at-one both serialize null `positive_infinity`/`+infinity` per-label and mean log loss. |
| M10 | Values 0, .1, and 1 select bins 0, 1, and inclusive bin 9; positive rates use reference presence. |
| M11 | True-A categorical `.8/.2` Brier is `.08`; true-present one-label marginal `.8` Brier is `.04`, retaining distinct metric families. |
| M12 | `{A,B}` twice with `[{}, {A,B}]` versus `[{A},{B}]`: equal per-label counts, exact `1/2` versus zero, distinct ordered episode differences. |
| M13 | Duplicate target, duplicate prediction, unknown label, and partial marginals reject before filtering with no report: `E_LABEL`, `E_LABEL`, `E_LABEL`, `E_PROBABILITY`. |
| M14 | Wrong target/outcome/probability/policy task combinations reject with no report: `E_CONFIG`, `E_CONFIG`, `E_PROBABILITY`, `E_CONFIG`. |
| M15 | Multi-label `scored_choice` and reported confidence reject with no report: `E_CONFIG` and `E_CONFIDENCE`. |
| M20 | Permuted set/episode/map-key order preserves raw/final/probability/signal results and vocabulary ordering, while predictions artifact digests differ. |
| `full_numeric_conformance` | Executes both exhaustive filters, F04, S01-S12, E01-E04/E07, and M01-M15/M20 through their existing independent cases. |

## Command evidence

`cargo test --locked --test conformance -- --list` exited 0 with 74 tests. Each
new named M filter and the umbrella ran independently and exited 0: one passed,
zero failed, 73 filtered. The complete serial inventory also exited 0 for both
exhaustive filters, F04, S01-S12, E01-E04/E07, M01-M15/M20, and the umbrella.
The exhaustive single-label oracle covers 301 populations; multi-label covers 64
answered subset pairs and 8 whole abstentions.

| Command | Exit | Result |
|---|---:|---|
| `cargo fmt --all -- --check` | 0 | Formatted. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Only the accepted staged 17 production plus 3 duplicate lib-test diagnostics. |
| `cargo test --all-features --locked` | 0 | 44 library, 10 CLI, 74 conformance, and 0 doc tests passed. |
| `cargo build --release --locked --bin validator` | 0 | Release binary built. |
| `git diff --check` | 0 | No whitespace errors. |

The unchanged staged Clippy inventory is: `TaskDefinition`; its
`single_label`, `multi_label`, `vocabulary`, `is_single_label` methods;
`SingleLabelTask` and `new`/`vocabulary`; `MultiLabelTask` and
`new`/`vocabulary`; `LabelSet::is_empty`/`contains`; `Episode` and
`new`/`id`/`input`/`target`; `ObservationSet::values`; single-label
`EvaluationConfig::new`; unused `MetricResult` items; `signal_availability`; and
three `schema_version` plus one `input` wire fields. They yield the authorized 17
production diagnostics in `src/model.rs`, `src/model/common.rs`,
`src/model/single_label.rs`, and `src/validation/wire.rs`, with 3 duplicate
lib-test diagnostics for `GoldenDataset.schema_version`, `GoldenEpisode.input`,
and `PredictionArtifact.schema_version`. No new warning appeared.

## Final identities

| Artifact | SHA-256 |
|---|---|
| `tests/conformance.rs` changed | `5dd2fb2b0b0c6adcdc6150466f475fe4395a4b60b107ae3ab03a4ed0397bee6d` |
| `tests/fixtures/single-label/expected.json` unchanged | `ab93c379b9c0df1d0c0fb7aac7a6df5e7748f3f671d55dd69d173e640090b268` |
| `tests/fixtures/multi-label/expected.json` unchanged | `213183a46c0f6c011296ef5087b5c44d3852081bdf20eaa80cbb494d3d2c84f2` |
| `src/model/multi_label.rs` unchanged repair | `3f515a0a9122db963b2e01ca14d5474b4836ba3543a48a4e0766a4459eea0b7c` |
| `schemas/v2/report.schema.json` unchanged repair | `805de93d2a50339370e803b7ff1a9a1b8d34c3028b5b21a1a6f428f591733f90` |
| `schemas/v2/inspection.schema.json` unchanged repair | `2c7e064d0adb915b77c558a7ebbfade96f7bcf0eee890b881ced1383f0eedaef` |

No fixture, literal discrepancy, or missing evidence remains. Independent review,
T026, and later work remain unstarted.

Verdict: **T025 output 3 and its developer boundary are complete.**
