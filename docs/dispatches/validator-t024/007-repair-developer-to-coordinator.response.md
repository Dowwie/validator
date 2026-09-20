# T024 bounded schema-completeness repair handoff

This is the single authorized finding-driven repair from verifier verdict006. It
does not claim T024 completion or independent acceptance. I read manifest005 and
the complete verdict006; their SHA-256 identities matched `502f5f7e4458b11ebdb46ec0a271b3ffe8eb76dd329e2ba7c123a5ccfbbce82c` and
`cf10fef91f3c58af5b9b8e924a53d099143faf3a060cc820e30a6c874d77db52`.
Every frozen file in manifest005 matched before editing.

Only the authorized files changed: `comparison.schema.json`,
`report.schema.json`, `receipt.schema.json`, and focused additions to
`tests/conformance.rs`. Production Rust, fixtures, CLI tests, all other schemas,
dependencies, plans, governance, artifact index, session notes, acceptance
records, and Fizzy remain unchanged. No production/specification mismatch or
second material defect was observed.

## Repair mapping and focused evidence

The existing generated documents remain the legal neighbors: both task variants
of reports and comparisons, their receipts, report source/preparation objects,
and the real-binary stdout matrix all validate. `all_schema_contracts` mutates one
position at a time and rejects every verdict006 admission below.

| Verdict006 mutant | Definition repair | Focused rejection assertion |
|---|---|---|
| Blank/null arbitrary `baseline.sources` | `comparison/$defs/side.sources` now has nonblank `propertyNames` and closed `$defs/source` values | `blank_source`, `arbitrary_source` |
| String `source_counts` value | `side.source_counts` is a nonblank keyed map of nonnegative `$defs/count` | `string_source_count` |
| String hard count | single and multi hard count fields use `$defs/count` | `string_hard_count` |
| `classes: [null]` | single hard classes now use closed `$defs/single_class` | `null_class` |
| Invented macro object | single/multi macro fields now use closed `$defs/macro` with metric and both undefined-class arrays | `invented_macro` |
| Boolean probability count | single probability count fields use `$defs/count` or nonnegative integer-or-null disagreement count | `boolean_probability_count` |
| Null transition row | final-outcome transitions use closed `$defs/single_transition`; multi tables retain closed typed rows | `null_transition` beside generated legal rows |
| Untyped multi-label episode set | matched/missed/extra fields now use `$defs/label_set_or_null`, a unique nonblank label array or null | `untyped_multi_set` |
| Scalar metric `population_scope: raw_answered` | report and comparison `$defs/metric` allow only `selected` or `answered`; bin schemas retain their explicit `raw_answered` option | `scalar_raw_answered`, `report_raw_answered` |
| Null source configuration | report/comparison source configuration is an opaque object with arbitrary member values | `null_source_configuration`; valid opaque object remains in the positive document |
| Null preparation configuration | report/comparison preparation configuration has the same opaque-object boundary | `null_preparation_configuration`; `single_report` retains a valid preparation object |
| Absolute/traversal manifest path | report artifact branch now requires exact snapshot names or `evidence/<ordinal>.bin` | `absolute_artifact_path`, `traversal_artifact_path`; legal `golden.json`, `predictions.json`, `config.json`, and evidence ordinal remain valid |
| Wrong evaluation receipt filename | evaluation receipt branch requires an absolute path ending in `report.json` | `wrong_evaluation_filename`; `/run/report.json` stays valid |
| Wrong comparison receipt filename | comparison receipt branch requires an absolute path ending in `comparison.json` | `wrong_comparison_filename`; `/run/comparison.json` stays valid |

The comparison source definition is closed and preserves only intended opaque
positions: configuration member values remain arbitrary JSON; source identity,
model, evidence paths, observations, preparation, counts, and task-specific
result records are typed. Report golden `input` remains genuinely arbitrary JSON.
No `$id`, registry, resolver, generation layer, CLI duplication, or wider schema
redesign was added.

## Commands and results

| Command | Exit | Result |
|---|---:|---|
| `cargo test --locked --test conformance all_schema_contracts -- --nocapture` | 0 | 1 passed, 35 filtered; all verdict006 mutations reject beside valid neighbors |
| `cargo test --locked --test cli cli_stdout_schema_matrix -- --nocapture` | 0 | 1 passed, 9 filtered; real binary preserves one-JSON stdout plus completed report/comparison validation |
| `cargo test --locked --test conformance -- --nocapture` | 0 | 36 passed |
| `cargo test --locked --test cli -- --nocapture` | 0 | 10 passed |
| `cargo fmt --all -- --check` | 0 | Formatting clean |
| `cargo test --locked` | 0 | 44 unit + 36 conformance + 10 CLI = 90 passed; 0 documentation tests |
| `cargo clippy --locked --all-targets -- -D warnings` | 101 | Only the accepted 17 production plus 3 duplicate lib-test dead-code diagnostics |
| `cargo build --release --locked --bin validator` | 0 | Release binary built with the accepted 17 warnings |
| `git diff --check` | 0 | No whitespace errors |

The residual Clippy inventory is unchanged: `TaskDefinition`,
`SingleLabelTask`, `MultiLabelTask`, `LabelSet::{is_empty,contains}`, generic
`Episode`, `ObservationSet::values`, the single-label config constructor,
selected `MetricResult` helpers/accessors, `signal_availability`, and the
unread checked-wire snapshot fields. The lib-test duplicate group consists of the
three unread `schema_version` fields in `GoldenDataset`, `PredictionArtifact`,
and `EvaluationConfig`. This repair added no warning, suppression, fake consumer,
or visibility widening.

## Repaired candidate identities

| Artifact | SHA-256 |
|---|---|
| `schemas/v2/comparison.schema.json` | `4f2dd6886056d335c3d4209c3a47dbfab2685a8424b4b8153d8d02fd77f9a104` |
| `schemas/v2/report.schema.json` | `aa3589ea93b9a6a0e7c519898988087acb87f6854dc72db11bd974d99cbb0da3` |
| `schemas/v2/receipt.schema.json` | `6f20777251015af381ad321030490c9c28996a165a4528e8b739d40227cc9637` |
| `tests/conformance.rs` | `6ad3652d62a447e6a0e32ab3ab69633c2b66041b6e0a3d15a52c76376e9692b6` |
| `tests/cli.rs` (unchanged) | `8c7b9c00d777f4a290911dd77199d8e1b9e085cc14fb0dd78a801108c3a10fcf` |
