# T024 schema-contract construction handoff

This handoff covers only the nine schema contracts and the two umbrella filters
assigned by dispatch002/003. It does not claim T024 completion or independent
acceptance. The assigned S/M/E case filters remain for the next fresh local
construction context: `case_s13`, `case_s14`, `case_s17`, `case_s21`, `case_s29`,
`case_m16`, `case_m21`, and `case_e05`, `case_e06`, `case_e09`, `case_e10`,
`case_e11`, `case_e12`, `case_e13`.

## Scope and contract decisions

I read the ratified T024 task, `Machine interface`, `Reports and metric reuse`,
the owner/coordinator dispatches, coverage rows, T021-T023 handoff018, current
schemas, and schema/CLI tests. All starting identities in dispatch002 matched.

The schemas remain self-contained Draft 2020-12 documents with local `#/$defs`
references. Pinned `jsonschema::validator_for` compiles every one without a
registry, network access, or external resolver. Per amendment003, no `$id` or
resolver layer was added because no actual reference-resolution failure exists.

Source and preparation `configuration` remain opaque **objects**. Their members
remain arbitrary JSON; positive input evidence includes null, exact integer
`9007199254740993`, `1e400`, and the literal
`$serde_json::private::Number`-shaped key. No scalar, array, or null root was
admitted for either configuration object.

The demonstrated repairs are deliberately narrow:

- `comparison.schema.json` now closes metric and paired-metric shapes, restricts
  metric statuses and positive-infinity representation, distinguishes task
  cardinalities, and closes/required-types single-label comparison episode rows.
- `report.schema.json` now types and closes per-episode observation values for
  scalar, unit-interval, categorical, and marginal variants rather than accepting
  an arbitrary object.
- `all_schema_contracts` and `cli_stdout_schema_matrix` provide the required
  online-independent evidence without adding a generic audit engine.

## Schema-to-clause and evidence matrix

| Schema | Normative clause | Positive alternatives | Discriminating negative coverage |
|---|---|---|---|
| `golden` | v1 Machine interface; data-model structure acceptance | Single-label class and multi-label set task/target documents; opaque input with null, large integer, `1e400`, literal tag-shaped key | Root required-field removal; missing episode input; cross-task target mutation |
| `predictions` | v1 Machine interface; data-model source/observation rules | Classifier/scored-choice sources; all five observation definitions; class, labels, abstention; categorical/marginal observations | Root/configuration removal; scalar `values` shape mutation; closed source/row alternatives retain rejection |
| `config` | v1 Machine interface policy contract | `as_recorded`, single-label `reject_below`, multi-label `label_thresholds`; both roles | Every required root field deleted; decision variants remain closed through existing policy regressions |
| `report` | v1 Machine interface report field table; data-model Reports and metric reuse | Generated single and multi reports; finite/status metric families, task hard/probability/signal variants | Required identity/result-family/episode-field deletions; incompatible observation value; unknown metric status |
| `comparison` | v1 Comparison and successive passes; data-model typed comparison variants | Generated completed single-label comparison and receipt; existing focused suite preserves multi-label report/comparison variants and zero-intersection statuses | Missing single comparison outcome; unknown nested metric status; closed task/cardinality/pair fields |
| `inspection` | v1 Machine interface inspection requirement | Generated replayed single inspection; existing focused contract covers both single and multi alternatives | Required prediction deletion; existing focused test mutates type, outcome, confidence, policy, and task-family shapes |
| `check` | v1 Machine interface check integrity/selected-ID contract | Completed single check; existing focused test covers completed multi check | Required nested `selected_count` deletion; existing focused tests reject missing task/integrity and single-only fields on multi |
| `receipt` | v1 Machine interface receipt/path/digest contract | Generated comparison receipt plus evaluation receipt variant | Required operation identity deletion; existing contract test rejects relative path/bad digest and wrong kind identity |
| `error` | v1 Machine interface stable code/stage contract | Serialized stable `E_SCHEMA` machine diagnostic; CLI matrix exercises real command and output-destination errors | Required code deletion; existing contract tests reject success status and constrain stable code/stage alternatives |

`all_schema_contracts` explicitly compiles all nine schemas first, uses positive
documents for every input decision/type alternative, generated reports/comparison/
inspection/receipt/error documents, and the root/nested deletions and mutations
listed above. Existing focused schema regressions continue to cover remaining
concrete inspection/check/report/comparison alternatives; the umbrella test calls
the same published schemas, not production-only type assertions.

`cli_stdout_schema_matrix` invokes the real binary for successful `check`,
`evaluate`, `inspect`, and `compare`; checks each stdout payload is exactly one
JSON document; validates it against check/receipt/inspection schemas; validates
the completed `report.json` and `comparison.json`; and validates real parse/
command and output-exists error payloads against the error schema.

## Commands and results

| Command | Exit | Evidence |
|---|---:|---|
| `cargo test --locked --test conformance -- --list` | 0 | 22 listed conformance tests, including nonzero `all_schema_contracts` |
| `cargo test --locked --test cli -- --list` | 0 | 10 listed CLI tests, including nonzero `cli_stdout_schema_matrix` |
| `cargo test --locked --test conformance all_schema_contracts -- --nocapture` | 0 | 1 passed, 21 filtered |
| `cargo test --locked --test cli cli_stdout_schema_matrix -- --nocapture` | 0 | 1 passed, 9 filtered |
| `cargo fmt --all -- --check` | 0 | Formatted |
| `cargo test --locked` | 0 | 44 unit + 10 CLI + 22 conformance = 76 passed; doc tests 0 |
| `cargo clippy --locked --all-targets -- -D warnings` | 101 | Only the accepted 17 production plus 3 duplicate lib-test staged dead-code diagnostics; no new warning class or suppression |
| `cargo build --release --locked --bin validator` | 0 | Release binary built; the same accepted 17 non-denied warnings are emitted |
| `git diff --check` | 0 | No whitespace errors |

The residual warning-denied inventory is unchanged: `TaskDefinition`,
`SingleLabelTask`, `MultiLabelTask`, `LabelSet::{is_empty,contains}`, generic
`Episode`, `ObservationSet::values`, the single-label config constructor,
selected `MetricResult` helpers/accessors, `signal_availability`, and checked-wire
snapshot fields. The same three duplicate lib-test diagnostics are the unused
`schema_version` fields in `GoldenDataset`, `PredictionArtifact`, and
`EvaluationConfig`. No lint suppression, fake consumer, or export widening was
added.

## Candidate identities

| Artifact | SHA-256 |
|---|---|
| `schemas/v2/check.schema.json` | `1bfd2fb3a644df1203b043244b38a9ed80d404cae5068901b7f162ea6dd662b9` |
| `schemas/v2/comparison.schema.json` | `d0dd16bef3902ac1597bb6538204ffe7c13442c655b4779f9953f6fd5200c8e9` |
| `schemas/v2/config.schema.json` | `c740c7cd2c1381398b3121ee161b1d995a23ceb9dd4325bd73cd0073f66d65c3` |
| `schemas/v2/error.schema.json` | `a1be234949560a90dff9391a0fa9c42de4f0431c5c4e2f20d745730f40eb80d5` |
| `schemas/v2/golden.schema.json` | `97d811a192274f1ee0703e5dff475723301fdcc68c20b031d3b22431968b777b` |
| `schemas/v2/inspection.schema.json` | `039e6060b88042c908c27a34374592d9f64434aa1aac4725da13146b01c0ac82` |
| `schemas/v2/predictions.schema.json` | `654908068da5d6b3cf9c666cddf4ebc888f94203a04fb6addd5164fbbb893352` |
| `schemas/v2/receipt.schema.json` | `d820751593f82ac68cbca2ed381b305383d96d388dee417abcefec03ed3d1c00` |
| `schemas/v2/report.schema.json` | `5500e80464cd87b196f291634c00c073113220747f03898c66406f5106e94ea9` |
| `tests/conformance.rs` | `d085c1fc94ac15748b5e56fa0dc5c94528cad165c804ba9fef0dc8cc3e7737e4` |
| `tests/cli.rs` | `8c7b9c00d777f4a290911dd77199d8e1b9e085cc14fb0dd78a801108c3a10fcf` |

Only `comparison.schema.json`, `report.schema.json`, `tests/conformance.rs`, and
`tests/cli.rs` changed in this construction boundary. No production Rust, fixture,
policy, configuration-object semantics, dependency, governance, plan,
artifact-index, session-note, acceptance, or Fizzy artifact was edited.
