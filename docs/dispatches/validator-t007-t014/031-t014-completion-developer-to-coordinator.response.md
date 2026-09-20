# T014 completion handoff

Status: T014 behavior, format, locked test, release-build, and diff checks are
complete. The required warning-denied Clippy command exited 101 solely because of
the owner-approved staged `dead_code` inventory described below. This checkpoint
is **not warning-free**; T027 and T035 retain the mandatory fully warning-free
gate.

## Scope and change

Only `tests/cli.rs` changed. The three existing process filters were retained and
extended; no filter names, production behavior, parser boundary, schemas, or
later-task behavior changed. Its final SHA-256 is
`fc03a9d04dbe0c58faab9574ef9eb68d8f4347d6d43d6d5424ae55e7e6cdd111`.

The initial identity matched every hash in dispatch031 before editing.

## Process evidence and assertion mapping

| Requirement | Assertions |
|---|---|
| 1. Actual check, receipt, and error stdout validate against locked schemas | `tests/cli.rs:50-68` compiles the locked check/receipt/error schemas and validates the parsed one-document stdout. Check/receipt calls are at `104-132`; all failure paths use the error validator at `169-170`, `188-200`, `211-212`, and `230-231`. `45-48` also requires empty stderr for every machine document. |
| 2. Missing input exits 3 safely | `202-212` runs `check` on absent files, asserts exit 3, and validates its sole error document. |
| 3. Existing destination exits 3, is byte/type-identical, and has no call-owned partial | `70-101` recursively records every entry's relative path, type, and bytes. `157-172` snapshots the successful destination parent before a second valid `evaluate`, asserts exit 3/error schema, and proves the entire tree is identical afterwards, including no temporary entry and an unchanged directory destination. |
| 4. Invalid input exits 2, safely, without output | `214-233` makes the otherwise valid prediction label `C`, runs `evaluate`, asserts exit 2/error schema, and requires the output path not exist. |
| 5. Parse errors exit 2; help/version are text exemptions | `181-201` checks help/version success and nonempty text; then unimplemented command, unknown flag, missing value, duplicate setting, unsupported check `--out`, and duplicate evaluate `--out` each exit 2 with a schema-valid error. |
| 6. Successful bound-evidence publication preserves exact bytes | `18-35` creates valid source evidence; `141-156` compares published golden, predictions, config, and `evidence/0.bin` byte-for-byte with submitted files. |
| 7. Post-admission secret-bearing failure leaks no sentinel | The evidence source and distinctive bytes are declared at `28-35`. The initial successful evaluation proves the binding and published bytes; the second evaluation has valid admission/evidence and fails only on existing destination. `173-175` require the sentinel absent from both stdout and stderr. |

`cli_check_evaluate` also retains check-no-write and success assertions
(`115-117`, `130-140`). `receipt_hash_matches_report` continues to independently
hash the actual published report against `result_sha256` at `237-258`.

## Independent release-binary byte record

I also ran `target/release/validator evaluate` against a fresh valid bound-evidence
bundle. The receipt was schema-valid and reported:

```json
{"schema_version":2,"kind":"evaluation","status":"complete","run_id":"01a0b76b-4008-7260-b461-f24b9a624168","result_path":"/tmp/validator-t014-hashes.hFNFpz/run/report.json","result_sha256":"ac18a82716129febe18575d363e9e432357f2a2599c324b15f7977adecdfa709"}
```

Exact SHA-256 results were:

| Submitted file | Published file | SHA-256 |
|---|---|---|
| `golden.json` | `run/golden.json` | `c05db4f0dd82205cbe3236e3029927a86b98392f35eb4cd5753710b0e41a24fa` |
| `predictions.json` | `run/predictions.json` | `26f764ca12aded7e5a51d8f104cfb73a9a6e089cf724905635cc6bd0aa265595` |
| `config.json` | `run/config.json` | `8583c9444461b1202f04040c8af6166f816796f22a769ff2a4c5e146bb10a2c1` |
| `bound-evidence.bin` | `run/evidence/0.bin` | `91d8523c5b68f4f0f98c1509bf9b30aca5f2e0b55fbd1eb0c3cf4f03ab999c5e` |
| n/a | `run/report.json` | `ac18a82716129febe18575d363e9e432357f2a2599c324b15f7977adecdfa709` |
| n/a | `receipt.json` | `45ff828ee733aead7891852bdfc8fdd1b4b822bb89769bb4ac56c40f9dfab570` |

The published report digest exactly equals the receipt's `result_sha256`.

## Required verification matrix

Every named task filter exited 0 with one passing test:

| Task | Commands |
|---|---|
| T007 | `cargo test --locked --lib metric_status_precedence -- --nocapture`; `cargo test --locked --lib count_overflow_is_error -- --nocapture`; `cargo test --locked --lib macro_undefined_classes -- --nocapture` |
| T008 | `cargo test --locked --test conformance single_matrix_identities -- --nocapture`; `cargo test --locked --test conformance f04_asymmetric_oracle -- --nocapture` |
| T009 | `cargo test --locked --test conformance categorical_loss_oracles -- --nocapture`; `cargo test --locked --test conformance signal_population_bins -- --nocapture`; `cargo test --locked --test conformance bin_boundary_binary64 -- --nocapture` |
| T010 | `cargo test --locked --test conformance input_schema_contract -- --nocapture` |
| T011 | `cargo test --locked --lib evidence_ordinal_binding -- --nocapture`; `cargo test --locked --lib artifact_exact_bytes -- --nocapture` |
| T012 | `cargo test --locked --lib publish_no_replace_race -- --nocapture`; `cargo test --locked --lib publish_late_failure -- --nocapture` |
| T013 | `cargo test --locked --test conformance single_report_schema -- --nocapture`; `cargo test --locked --test conformance report_sources_and_privacy -- --nocapture` |
| T014 | `cargo test --locked --test cli cli_check_evaluate -- --nocapture`; `cargo test --locked --test cli cli_help_version_errors -- --nocapture`; `cargo test --locked --test cli receipt_hash_matches_report -- --nocapture` |

Final commands and actual results:

| Command | Exit | Result |
|---|---:|---|
| `cargo fmt --all -- --check` | 0 | Passed. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Only the 28 staged `dead_code` diagnostics below; no ordinary lint class remained. |
| `cargo test --all-features --locked` | 0 | Passed: 39 library, 3 CLI, 8 conformance, and 0 doc tests. |
| `cargo build --release --locked --bin validator` | 0 | Passed (with the staged warnings). |
| `git diff --check` | 0 | Passed. |

## Complete staged Clippy reconciliation

The exact warning-denied command reported 28 `dead_code` errors, all mapped in
the owner-approved inventory; no style, compile, test, or production regression
was added.

| Diagnostic group | Approved rationale |
|---|---|
| `src/evaluation.rs:14 checked_mul` | T018 multi-label `N*K` accounting. |
| `src/model.rs:8 TaskDefinition`; `single_label`, `multi_label`, `vocabulary`, `is_single_label` | Closed task boundary; T018 is its first complete multi-label consumer. |
| `src/model.rs:42 SingleLabelTask`; `new`, `vocabulary` | Accepted task model, consumed with the closed task model in T018. |
| `src/model.rs:60 MultiLabelTask`; `new`, `vocabulary` | T018. |
| `src/model/common.rs:400 LabelVocabulary::for_multi_label`, `label_set`; `LabelSet`, `is_empty`, `contains` | T018 checked multi-label admission and set targets/outputs. |
| `src/model/common.rs:508 Episode`; `new`, `id`, `input`, `target` | T015 explicit inspection. |
| `src/model/common.rs:781 ObservationSet::is_empty`, `values` | `is_empty` has a real current unit-test consumer; `values` is required by T015 inspection presentation. |
| `src/model/common.rs:808 PreparationDescriptor::new`; `src/model/common.rs:885 SourceDefinition::new` | T029 checked preparation construction; T014 wire admission correctly uses checked raw constructors. |
| `src/model/common.rs:1016 EvaluationConfig.policy`, `policy()`; `src/model/common.rs:1076 Population.dataset_digest`, `dataset_digest()` | T016 policy and exact-dataset compatibility. |
| `src/model/common.rs:1167 MetricUnit::LabelDecision`; `src/model/common.rs:1220 MetricResult::ratio`, `status_value`, `population_count`, `unit`, `scope`, `numerator`, `denominator`, `special_value` | T016 typed comparison and T018 label-decision metrics. |
| `src/model/single_label.rs:115 signal_availability` | T018 artifact-wide signal admission. |
| `src/validation/wire.rs:15 GoldenDataset.schema_version`; `:23 SingleLabelTask.kind`; `:38 GoldenEpisode.input`; `:51 PredictionArtifact.schema_version`; `:218 EvaluationConfig.schema_version`; `:239 SingleLabelDecision.signal`, `minimum` | T010 strict Serde DTO version/tag/payload enforcement. These fields are semantically consumed during deserialization; no fake reads or suppression were added. |

The former obsolete `Decoded` bytes/accessor diagnostics are absent, as expected
after the authorized prior cleanup. The full output ended with `could not compile
validator (lib) due to 28 previous errors`; the test target also surfaced the
corresponding staged subset. The command intentionally used no altered lint flags,
suppression, artificial consumer, or public export.

## Unresolved failures

There are no T014 behavior, schema, privacy, filesystem, test, release-build, or
diff failures. The sole unresolved gate is the owner-staged warning-denied Clippy
failure above. It must be resolved at the still-mandatory T027/T035 clean gates,
after its actual T015-T021/Serde consumers exist.
