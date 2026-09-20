# T014 application and CLI handoff

Status: blocked at the required clean gate by the confirmed contract conflict below. The implemented T014 application/CLI behavior and all nine named deferred/process filters pass. No T015, multi-label behavior, replay, comparison, or inspection work was started.

## Implemented bounded behavior

- Added documented public library API:
  - `pub fn check(options: CheckOptions) -> Result<CheckResult>`
  - `pub fn evaluate(options: EvaluationOptions) -> Result<EvaluationReceipt>`
- `CheckOptions` contains explicit `dataset`, `predictions`, and `config` paths. `EvaluationOptions` adds an explicit `output` path. Both return typed `Diagnostic` failures through the existing `Result`/exit-category path.
- `src/app.rs` performs exact snapshot loading, strict admission, and evidence binding once in a shared private path. `check` returns the versioned check document without scoring or writes. `evaluate` generates UUIDv7 and UTC RFC3339 identity, scores, serializes the typed T013 report, publishes with the T012 no-replace publisher, and returns a receipt bound to the actual absolute `report.json` path and its exact SHA-256.
- Added `Diagnostic::to_machine_json`, which emits the complete safe versioned error envelope. It exposes only the schema’s stable diagnostic fields.
- Added binary-only `src/cli.rs` and `src/main.rs`. The CLI supports only `check` and `evaluate`; successful operations write one JSON document to stdout and return zero. Bad command/argument input emits schema-valid safe error JSON and returns typed categories. Help/version are text exemptions.
- Added the six real integration conformance cases using isolated real files and the exported production API. Added `tests/cli.rs`, which uses Cargo’s built `validator` executable and checks actual stdout/stderr, exits, files, and receipt hashing.
- Added `time` with `formatting`/`std` and enabled UUID `std`/`v7`; no framework, runtime, logging stack, or additional application abstraction was introduced.

## Passing named evidence

Every command below exited 0 with a nonzero test count.

| Command | Result |
| --- | --- |
| `cargo test --locked --test conformance single_matrix_identities -- --nocapture` | 1 passed |
| `cargo test --locked --test conformance f04_asymmetric_oracle -- --nocapture` | 1 passed |
| `cargo test --locked --test conformance categorical_loss_oracles -- --nocapture` | 1 passed |
| `cargo test --locked --test conformance signal_population_bins -- --nocapture` | 1 passed |
| `cargo test --locked --test conformance bin_boundary_binary64 -- --nocapture` | 1 passed |
| `cargo test --locked --test conformance report_sources_and_privacy -- --nocapture` | 1 passed |
| `cargo test --locked --test cli cli_check_evaluate -- --nocapture` | 1 passed |
| `cargo test --locked --test cli cli_help_version_errors -- --nocapture` | 1 passed |
| `cargo test --locked --test cli receipt_hash_matches_report -- --nocapture` | 1 passed |
| `cargo check` | compiled; warnings described below |
| `git diff --check` | pass |

The process tests prove check has no output-directory side effect, evaluate publishes `golden.json`, `predictions.json`, `config.json`, and `report.json`, stdout is a single parseable JSON document, stderr is empty, errors use structured JSON/exit 2, and the receipt hash equals an independently calculated SHA-256 of `report.json`. The API report tests prove emitted report schema validity, numerical/matrix values, signal populations, source counts, and privacy-sentinel omission.

## Required clean-gate blocker

The exact command below was run and exited 101:

```sh
cargo clippy --all-targets --all-features --locked -- -D warnings
```

It reports 34 errors. Four are direct style fixes (`clone_on_copy` in `src/app.rs`, `too_many_arguments` and `redundant_closure` in `src/evaluation/single_label.rs`, and `needless_question_mark` in `src/model/common.rs`). The remaining 30 are `dead_code` errors in accepted pre-T014 private contracts, including:

- `src/model.rs`: `TaskDefinition`, `SingleLabelTask`, `MultiLabelTask`, and their constructors/accessors;
- `src/model/common.rs`: multi-label vocabulary/set APIs, generic `Episode`, validation/test constructors, retained policy/dataset fields and accessors, `MetricUnit::LabelDecision`, and metric inspection accessors;
- `src/validation.rs`/`src/validation/wire.rs`: retained exact decoded bytes and strict DTO discriminator/input fields;
- `src/evaluation.rs`: `checked_mul`; and `src/model/single_label.rs`: `signal_availability`.

The coordinator confirmed this is a genuine conflict: T014 explicitly forbids implementing multi-label behavior, broadening the public SDK, fake runtime consumers, or lint suppression. The dead code includes the still-required multi-label/task scaffolding and strict-wire retention contract, which have no honest single-label T014 consumer. I therefore stopped rather than changing an accepted contract or gaming the clean gate.

Smallest criterion-preserving options for the coordinator/owner:

1. Authorize the later task that consumes the multi-label/retained-contract code before requiring the all-target clean gate; or
2. Authorize a narrowly scoped contract revision that removes/deferred-gates those accepted unused structures and their unit coverage, then rerun T014’s clean gate; or
3. Revise the T014 clean gate to exclude unconsumed accepted later-task structures, without lint suppression.

No `allow`, `expect`, underscore/dead read, fake usage, or public export was added.

### Exact dead-code diagnostic mapping

| Clippy symbol(s) | Status and smallest honest consumer/decision |
| --- | --- |
| `checked_mul` | Required by T018 multi-label `N*K` accounting. |
| `TaskDefinition`, `SingleLabelTask`, `MultiLabelTask` and their constructors/accessors | T018’s closed multi-label model consumes the task boundary. |
| `LabelVocabulary::for_multi_label`, `LabelVocabulary::label_set`, `LabelSet::{is_empty,contains}` | T018’s checked multi-label target/output admission. |
| `Episode::{new,id,input,target}` | Required retained opaque-episode contract for T015 inspection; `input` must not be removed before explicit inspection exists. |
| `ObservationSet::{is_empty,values}` | `values` is a likely T015 inspection consumer; `is_empty` is genuinely obsolete if no inspection presentation needs it. |
| `PreparationDescriptor::new`, `SourceDefinition::new` | T029 preparation/provenance construction boundary; current wire admission uses the corresponding `new_raw` paths. |
| `EvaluationConfig::policy` and its `policy` field | T016 comparison needs effective policy compatibility/differences. |
| `Population::dataset_digest` and its field | T016 comparison needs exact-dataset compatibility. |
| `MetricUnit::LabelDecision`, `MetricResult::{ratio,status_value,population_count,unit,scope,numerator,denominator,special_value}` | T018 multi-label metrics and T016 typed metric-pair comparison. |
| `signal_availability` | T018 artifact-wide multi-label signal admission; T014’s single-label app reaches the lower `signal_availability_flags` through validation. |
| `Decoded::{bytes,value}` and its `bytes` field | Genuinely obsolete in the current one-shot validation path: exact bytes are correctly held by `InputArtifacts`, and admission consumes DTOs with `into_value`. Removal can be separately authorized without changing T014 behavior. |
| `wire::{GoldenDataset.schema_version, SingleLabelTask.kind, GoldenEpisode.input, PredictionArtifact.schema_version, EvaluationConfig.schema_version, SingleLabelDecision::{signal,minimum}}` | Required current strict-deserialization shape/tag retention. Their read is performed by Serde’s derived deserializer rather than explicit Rust expressions; removing them would weaken T010/T004 admission. They do not have an honest later runtime consumer. |
| `RunId::clone`, `probability_results` argument count, `sum`’s redundant closure, and `MetricResult::ratio`’s needless question mark | Four ordinary style diagnostics in `src/app.rs`, `src/evaluation/single_label.rs`, and `src/model/common.rs`; independently repairable once the clean-gate sequencing decision permits a final repair pass. They are not the contract conflict. |

## Current hashes

| Path | SHA-256 |
| --- | --- |
| `Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |
| `src/lib.rs` | `b0c5d00e000747368fc9e18e6459c61417dbbf24f06ac58077e9c4cf382ceac7` |
| `src/app.rs` | `18d400afe2f5c343512d4ed535b53905561f9770eb7b53731ed8344c7b610248` |
| `src/cli.rs` | `608ab90dcda6cd36f8b4d03575f47efc8f2a5977480a0c4bb670aa37933a0fbe` |
| `src/main.rs` | `ca701aa73cb1d687d9ff086e654bd9b08e6c285a91c9577c3b2d1eb7fc54a3c0` |
| `src/error.rs` | `5e47f1afde84a682e6d33744133820a43c14a761383717fc91012c63b330abcb` |
| `tests/conformance.rs` | `16bc7d8eb4644491ae86f30a85a5f3a69f8b471e8106351184b67e30809f9637` |
| `tests/cli.rs` | `d0e255789545846fb57c97c3338cee6344d5cf2f9f915fc3a3df88eaf6d77e4b` |

The verified T013 response input remained `c34a306011c87da662ad3eb1d2098f2524fed250d702dc145eda18a320f39a9c` before T014 edits.
