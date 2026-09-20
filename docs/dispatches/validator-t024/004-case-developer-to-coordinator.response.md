# T024 assigned-case construction handoff

This handoff covers the fourteen assigned S/M/E conformance filters only. It is
not an independent acceptance or a T024 completion claim. The preceding schema
and umbrella work remains the authority for the nine schema contracts.

## Starting reconciliation and scope

All starting identities in dispatch004 matched before implementation. The exact
pre-edit mismatch was that `tests/conformance.rs` had no definitions for any of
the fourteen assigned filters: `cargo test --locked --test conformance case_s13
-- --nocapture`, `case_m16`, and `case_e05` each exited 0 with **0 tests** and
22 filtered out. The conformance listing contained 22 tests and no `case_s*`,
`case_m*`, or `case_e*` names from this assignment.

I added only `tests/conformance.rs`: a small filesystem/public-API error helper
and the fourteen named compound tests. No production Rust, schema, CLI test,
fixture, policy, artifact, governance, plan, index, session-note, acceptance, or
Fizzy file changed. Every fixture is synthetic and local.

## Requirement-to-evidence map

| Filter | Public path and compound assertions | Actual result |
|---|---|---|
| `case_s13` | `evaluate` rejects invalid UUID and duplicate episode/prediction IDs as `E_ID`; duplicate vocabulary and unknown class as `E_LABEL`; unknown row field and missing `input` as `E_SCHEMA`; incomplete categorical family as `E_PROBABILITY`. Each failure leaves no published run. | Passed. No repair or subset run exists. |
| `case_s14` | `evaluate` rejects missing and extra rows as `E_ALIGNMENT`, and a wrong dataset digest as `E_PROVENANCE`; no output directory is published. | Passed. No row becomes abstention. |
| `case_s17` | Two real single-label runs reverse episode submission and categorical-object key order. `raw`, `final`, probability, and sorted episode evidence are equal; golden input bytes differ. | Passed. Calculations/evidence are stable while bytes differ. |
| `case_s21` | A declared three-class `UNCERTAIN` reference and outcome complete normally. Missing/null expected shapes fail the golden schema; undeclared reference fails `E_LABEL`. | Passed. The assertion only checks structure and successful evaluation; it makes no settlement claim. |
| `case_s29` | Real generated report validates against the published report schema; typed evaluation receipt and `Diagnostic::to_machine_json()` validate against receipt/error schemas. The existing real-binary `cli_stdout_schema_matrix` remains the single-document stdout evidence. | Passed. |
| `case_m16` | `evaluate` rejects a missing multi-label row as `E_ALIGNMENT`; an empty set is `answered`; whole abstention is explicit; incomplete marginals are `E_PROBABILITY`. | Passed. |
| `case_m21` | A legally shaped unreviewed multi-label reference evaluates successfully. Count-map labels, mixed task outcome, explicit partial-reference mask, and wire version 1 each reject with `E_SCHEMA` or `E_CONFIG`; no fallback interpretation occurs. | Passed. |
| `case_e05` | A prepared native `UNCERTAIN` outcome retains scalar `1.33` and categorical observation `[0.24,0.19,0.57]`; unsubmitted distribution gives probability `not_applicable`. Supplying the same distribution as canonical categorical scoring evidence gives defined log loss. | Passed. No `MATCH` substitution. |
| `case_e06` | Prepared categorical observation `{A:0.5,B:0.49}` is retained unchanged; hard evaluation completes, probability is `not_applicable`, and report source preparation plus the evidence artifact disclose the omission. | Passed. |
| `case_e09` | A scalar `0.2` and displayed categorical vector `{A:0.9,B:0.1}` remain distinct in a successful report. | Passed. No invented consistency equation. |
| `case_e10` | Auxiliary Bernoulli observation survives as inspection evidence with probability `not_applicable`; separately labeled multi-label marginals produce defined binary metrics, including Brier `0.265`. | Passed. No task conversion. |
| `case_e11` | Unknown observation binding is `E_PROVENANCE`; out-of-range Bernoulli and literal `1e400` scalar are `E_OBSERVATION`; unknown observation kind is `E_SCHEMA`; an out-of-range preparation evidence index is `E_PROVENANCE`. No output is published. | Passed. The established `ObservationSet` binding boundary deliberately categorizes unknown names as provenance. |
| `case_e12` | A missing per-row observation remains `{}` while the present value remains `1.0`; one missing top-level categorical scoring family is `E_PROBABILITY`. | Passed. No imputation. |
| `case_e13` | Removing the required outcome from a row with probabilities, and from a row with a scalar observation, each produces `E_SCHEMA` and no run. | Passed. No synthesized choice or abstention. |

## Commands and results

The final conformance listing contains **36** nonzero tests, including every
assigned filter. The CLI listing contains **10** tests, including
`cli_stdout_schema_matrix`.

Each of these exact commands exited 0 with **1 passed, 35 filtered out**:

```text
cargo test --locked --test conformance case_s13 -- --nocapture
cargo test --locked --test conformance case_s14 -- --nocapture
cargo test --locked --test conformance case_s17 -- --nocapture
cargo test --locked --test conformance case_s21 -- --nocapture
cargo test --locked --test conformance case_s29 -- --nocapture
cargo test --locked --test conformance case_m16 -- --nocapture
cargo test --locked --test conformance case_m21 -- --nocapture
cargo test --locked --test conformance case_e05 -- --nocapture
cargo test --locked --test conformance case_e06 -- --nocapture
cargo test --locked --test conformance case_e09 -- --nocapture
cargo test --locked --test conformance case_e10 -- --nocapture
cargo test --locked --test conformance case_e11 -- --nocapture
cargo test --locked --test conformance case_e12 -- --nocapture
cargo test --locked --test conformance case_e13 -- --nocapture
```

Additional final checks:

| Command | Exit | Result |
|---|---:|---|
| `cargo test --locked --test conformance all_schema_contracts -- --nocapture` | 0 | 1 passed, 35 filtered. |
| `cargo test --locked --test cli cli_stdout_schema_matrix -- --nocapture` | 0 | 1 passed, 9 filtered. |
| `cargo test --locked --test conformance -- --nocapture` | 0 | 36 passed. |
| `cargo test --locked --test cli -- --nocapture` | 0 | 10 passed. |
| `cargo fmt --all -- --check` | 0 | Formatted. |
| `cargo test --locked` | 0 | 44 unit + 36 conformance + 10 CLI = 90 passed; 0 doc tests. |
| `cargo build --release --locked --bin validator` | 0 | Release binary built. |
| `git diff --check` | 0 | No whitespace errors. |
| `cargo clippy --locked --all-targets -- -D warnings` | 101 | Only the accepted staged inventory below. |

`clippy -D warnings` remains blocked solely by the accepted staged inventory: 17
production dead-code diagnostics (`TaskDefinition`, `SingleLabelTask`,
`MultiLabelTask`, `LabelSet` helpers, generic `Episode`, observation values,
single-label config constructor, `MetricResult` helpers/accessors, and
`signal_availability`) plus 3 duplicate lib-test diagnostics for unread wire
fields (`GoldenDataset.schema_version`, `PredictionArtifact.schema_version`,
`EvaluationConfig.schema_version`). This change adds no diagnostic, suppression,
fake consumer, or visibility widening.

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
| `tests/conformance.rs` | `4b44ea67d2f902e139790a9be339539abd7ee3a90f64052df7092c37b62cc9b7` |
| `tests/cli.rs` | `8c7b9c00d777f4a290911dd77199d8e1b9e085cc14fb0dd78a801108c3a10fcf` |

No production-output/specification mismatch was found. The coordinator can now
freeze this local construction candidate for the authorized independent review.
