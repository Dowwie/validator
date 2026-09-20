# T007-T014 combined independent verification

## Verdict: Revise

The frozen candidate satisfies T007-T012 and most of T013-T014, but two
demonstrated machine-contract defects remain. `check` reports false normalization
integrity values for valid near-unit categorical input, and the published report
schema accepts a one-label `single_label` report even though that task requires at
least two labels. Neither defect is covered by the staged Clippy exception.

## Candidate identity

I hashed every path listed in
`docs/dispatches/validator-t007-t014/033-coordinator-final-manifest.md` before
reviewing the candidate. All 48 product, governing, and evidence inputs matched
their declared SHA-256 values. I repeated the same reconciliation after all review
commands; all 48 still matched. The reviewed candidate therefore remained the
exact frozen candidate throughout this review.

I read the required global/repository instructions, team skill, task contracts,
linked specification sections, data-model section, execution contract, charter,
owner decision 028, manifest 033, and all nine implementation evidence responses.
I read final T014 responses 031 and 032 before relying on the earlier task
handoffs.

## Criterion-level results

| Task | Result | Independent evidence |
|---|---|---|
| T007 | Ready | Checked add/subtract/multiply return `E_NUMERIC` on overflow/underflow; ratio/status constructors retain explicit population/unit/scope and ratio fields; evaluator ordering applies `not_applicable` before empty population, then `no_data`, answered-only `no_answered_predictions`, and zero-denominator status. Only the named macro-F1 zero-fills undefined classes. Positive infinity is serialized as null plus `special_value: "+infinity"`. All three named filters passed with one test each, and the wider evaluator tests exercised the nontrivial branches. |
| T008 | Ready | The production path builds separate raw/final K by K+1 matrices with a typed abstention column, checked count updates, matrix/accounting invariants, per-class metrics, macro-F1, and sorted privacy-safe episode evidence. The F04 matrix is `[[2,1,1,0],[0,2,0,0],[1,0,1,0]]`, accuracy `5/8`, and independently derived macro-F1 `131/210`; the thread case yields accuracy `1/2`, coverage `3/4`, selective accuracy `2/3`, class coverages `[1/2,1,1]`, and macro-F1 `1/2`. Both named filters passed with one test each. The expected fixture is static and production scoring does not generate it. |
| T009 | Ready | Categorical log loss/Brier use all selected probability rows, preserve submitted/working vectors, encode true-class zero probability as positive infinity, use deterministic vocabulary-order argmax, and keep confidence on the raw-answered population. Maximum-probability and confidence bins retain distinct included/excluded IDs and exact fixed boundaries; only maximum-probability bins feed ECE. The corrected ECE denominator uses the selected total. All three named filters passed with one test each; owning unit evidence covers `-ln(0.7)`, Brier `0.14`, infinite loss/Brier `2`, exact disagreement/abstention behavior, ECE `0.65`, and adjacent binary64 boundaries. |
| T010 | Ready | The three input schemas declare Draft 2020-12, close all non-opaque shapes, preserve opaque input/configuration, cover both task alternatives, do not require scoring signals merely because observations exist, and leave cross-record/vocabulary/digest semantics to runtime. The discriminating valid/invalid matrix covers task/target pairing, tags, closed fields, nonblank strings/keys, bounded scalars, probability/observation branches, nonempty vector maps, scored-choice requirements, and all policy shapes. `input_schema_contract` passed with one test. I did not treat schema-only source-binding acceptance as runtime proof. |
| T011 | Ready | Inputs are read once into exact bytes/digests; source IDs sort by UTF-8 bytes and evidence retains source-local array order while receiving consecutive `evidence/n.bin` ordinals. Repeated paths, same basenames, parent-relative paths, unused definitions, exact bytes/digests, and `E_IO` on missing files are covered. Both named filters passed with one test each. |
| T012 | Ready | Publication uses a private temporary sibling and host `renameat_with(..., RenameFlags::NOREPLACE)`; there is no exists-check rename, overwrite fallback, or retry mode. File/directory/symlink competitors remain intact, late failure removes only the owned temporary sibling, final/evidence directories are mode 0700, files are mode 0600, layout is exact, report path is absolute, and receipt digest hashes exact report bytes. Both named filters passed with one test each. |
| T013 | Revise | Production report assembly is typed and complete for the implemented single-label variant: source paths are rewritten to bound evidence paths; unused sources remain with zero counts; metric, signal, and episode fields are present; snapshots/raw evidence/opaque episode input do not enter routine reports; and actual reports validate. The check, receipt, and error schemas also validate actual CLI documents. However, `report.schema.json` permits an impossible one-label `single_label` report; finding 2 gives the reproduction and correction. Both named filters otherwise passed with one test each. |
| T014 | Revise | `check` and `evaluate` share `admit`, which loads exact bytes, performs strict validation/alignment, and binds evidence. `check` does not score or publish; `evaluate` supplies UUIDv7/time, scores, publishes atomically, and returns a typed receipt. Built-binary tests establish one schema-valid JSON stdout document, empty stderr, exits 0/2/3, typed category-4 unit mapping, help/version, malformed settings, no-write check, immutable existing destination, exact snapshot/evidence bytes, receipt hash, and a post-admission secret-bearing failure with no leak. However, `check` hardcodes false normalization diagnostics; finding 1 gives an actual CLI reproduction and correction. All three named filters passed with one test each. |

## Required named filters

Every exact task command exited 0 and executed a nonzero count: one passing test
per filter.

| Task | Filter | Exit | Count |
|---|---|---:|---:|
| T007 | `metric_status_precedence` | 0 | 1 |
| T007 | `count_overflow_is_error` | 0 | 1 |
| T007 | `macro_undefined_classes` | 0 | 1 |
| T008 | `single_matrix_identities` | 0 | 1 |
| T008 | `f04_asymmetric_oracle` | 0 | 1 |
| T009 | `categorical_loss_oracles` | 0 | 1 |
| T009 | `signal_population_bins` | 0 | 1 |
| T009 | `bin_boundary_binary64` | 0 | 1 |
| T010 | `input_schema_contract` | 0 | 1 |
| T011 | `evidence_ordinal_binding` | 0 | 1 |
| T011 | `artifact_exact_bytes` | 0 | 1 |
| T012 | `publish_no_replace_race` | 0 | 1 |
| T012 | `publish_late_failure` | 0 | 1 |
| T013 | `single_report_schema` | 0 | 1 |
| T013 | `report_sources_and_privacy` | 0 | 1 |
| T014 | `cli_check_evaluate` | 0 | 1 |
| T014 | `cli_help_version_errors` | 0 | 1 |
| T014 | `receipt_hash_matches_report` | 0 | 1 |

## Full command results

| Command | Exit | Result |
|---|---:|---|
| `cargo fmt --all -- --check` | 0 | Passed. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Exact staged inventory only; reconciliation below. |
| `cargo test --all-features --locked` | 0 | 39 library, 0 binary-unit, 3 CLI, 8 conformance, and 0 doc tests passed. |
| `cargo build --release --locked --bin validator` | 0 | Passed, emitting only the staged warnings. |
| `git diff --check` | 0 | Passed. |
| `ruby docs/plans/validator/verify-plan.rb` | 0 | Passed: 35 tasks, acyclic dependencies, 433 source blocks, 63 conformance cases, 12 structure checks, 8 ACs, 5 DoD clauses, physical owners, local links, and artifact index. |

## Actual CLI, schema, privacy, and publication conclusions

- The conformance tests exercise the real public `validator::evaluate` application
  API, and the process tests invoke `CARGO_BIN_EXE_validator`; no included source,
  test facade, or static report stands in for application evidence.
- Actual check, evaluation-receipt, and error stdout documents validate against
  their published schemas. Every operational path inspected emits one JSON
  document; help/version are the specified text exemptions. The CLI preserves
  typed exit categories 0/2/3, and unit evidence maps numeric/invariant failures
  to 4.
- Routine reports and errors omit opaque episode input and raw evidence bytes.
  The built-binary secret-bearing existing-destination failure occurs after
  admission/evidence loading and leaks the sentinel to neither stdout nor stderr.
- Evaluate publishes exact submitted golden, prediction, configuration, and bound
  evidence bytes. The destination is created through a real atomic no-replace
  operation; competitors are preserved and owned temporary state is removed on
  failure. The receipt names an absolute `report.json`, and an independent SHA-256
  of its exact bytes matches `result_sha256`.
- The four T013 output schemas are closed at their object boundaries and validate
  actual emitted documents, subject to finding 2's too-permissive single-label
  vocabulary cardinality.
- No scoring repair, tolerance change, overwrite fallback, raw-payload routine
  output, lint suppression, fake read, artificial consumer, or broadened public
  SDK surface was found. The three `cfg(test)` conveniences authorized in response
  032 are confined to tests and are not findings.

## Staged Clippy reconciliation

The exact warning-denied Clippy command exited 101. The production library emitted
exactly these 26 unique `dead_code` diagnostics, matching response 032 and manifest
033, with no ordinary or new lint class:

1. `evaluation.rs`: `checked_mul`.
2. `model.rs`: `TaskDefinition`.
3. `model.rs`: `TaskDefinition::{single_label,multi_label,vocabulary,is_single_label}`.
4. `model.rs`: `SingleLabelTask`.
5. `model.rs`: `SingleLabelTask::{new,vocabulary}`.
6. `model.rs`: `MultiLabelTask`.
7. `model.rs`: `MultiLabelTask::{new,vocabulary}`.
8. `model/common.rs`: `LabelVocabulary::{for_multi_label,label_set}`.
9. `model/common.rs`: `LabelSet`.
10. `model/common.rs`: `LabelSet::{is_empty,contains}`.
11. `model/common.rs`: `Episode`.
12. `model/common.rs`: `Episode::{new,id,input,target}`.
13. `model/common.rs`: `ObservationSet::values`.
14. `model/common.rs`: `EvaluationConfig::policy` field.
15. `model/common.rs`: `EvaluationConfig::policy()`.
16. `model/common.rs`: `Population::dataset_digest` field.
17. `model/common.rs`: `Population::dataset_digest()`.
18. `model/common.rs`: `MetricUnit::LabelDecision`.
19. `model/common.rs`: `MetricResult::{ratio,status_value,population_count,unit,scope,numerator,denominator,special_value}`.
20. `model/single_label.rs`: `signal_availability`.
21. `validation/wire.rs`: `GoldenDataset::schema_version`.
22. `validation/wire.rs`: `SingleLabelTask::kind`.
23. `validation/wire.rs`: `GoldenEpisode::input`.
24. `validation/wire.rs`: `PredictionArtifact::schema_version`.
25. `validation/wire.rs`: `EvaluationConfig::schema_version`.
26. `validation/wire.rs`: `SingleLabelDecision::{signal,minimum}`.

The library test target then reported the corresponding duplicated six-member
subset (the label-decision unit and DTO/tag fields used differently under tests),
with no additional symbol or lint class. The Value constructors and
`ObservationSet::is_empty` are absent from production diagnostics exactly as
response 032 requires. No lint flags, suppression attributes, fake consumers, or
exports were added.

This checkpoint is **not warning-free**. The owner-authorized intermediate
exception covers only this exact staged inventory; T027 and T035 retain the
mandatory fully warning-free gate.

## Findings, ordered by severity

### 1. Moderate: `check` reports false normalization integrity

Current criterion: the machine interface says `check` returns integrity counts
(`docs/specs/validator-v1.md:798-800`), the check schema requires
`normalized_count` and `maximum_sum_error`
(`schemas/v2/check.schema.json:13-22`), and T014 requires the application to run
full signal admission through the shared path without scoring or publication
(`docs/plans/validator/tasks/T014.json:65-73`).

Location: `src/app.rs:90-99` hardcodes `normalized_count: 0` and
`maximum_sum_error: 0.0` for every successful check. The same admitted categorical
data is normalized in `src/model/single_label.rs:36-41`, while report assembly
computes the real diagnostics at `src/model/single_label.rs:729-796`.

Reproduction executed against `target/release/validator check`: a valid one-row,
three-label input with categorical values `0.7`, `0.2`, and `0.1000000005` has
sum `1.0000000005`, within the fixed `1e-9` admission tolerance, so its working
vector is normalized. The command exited 0 but emitted:

```json
{"integrity":{"signal_availability":"categorical","normalized_count":0,"maximum_sum_error":0.0}}
```

Observable consequence: an agent using `check` is told no submitted row was
normalized and no sum error occurred, while `evaluate` over the same admitted
input reports one normalized row and a nonzero maximum error. The two commands
share admission but disagree on integrity evidence.

Smallest correction: derive normalization count and maximum absolute categorical
sum error from the admitted evaluation rows in a shared non-scoring helper used
by both check and report assembly. Add a built-binary `check` regression for a
within-tolerance non-unit vector and assert the schema-valid stdout reports count
1 and the actual error.

### 2. Moderate: the report schema accepts an impossible one-label single-label report

Current criterion: JSON Schemas are the serialization contract
(`docs/specs/validator-v1.md:783-787`), report `task` must contain the exact declared
task and ordered vocabulary (`docs/specs/validator-v1.md:815`), and a canonical
single-label task requires at least two labels
(`docs/specs/validator-v1.md:102-110`). T013 owns the report schema and direct
schema conformance.

Location: `schemas/v2/report.schema.json:50` sets the single-label task's
`labels.minItems` to 1. The named schema test itself constructs
`"task":{"kind":"single_label","labels":["A"]}` at
`tests/conformance.rs:149` and confirms that document is valid at lines 157-165.

Reproduction: `cargo test --locked --test conformance single_report_schema --
--nocapture` exits 0 with one test even though its accepted complete report has
only one declared single-label class.

Observable consequence: a producer or agent can present a structurally impossible
single-label report as schema-valid, so the published schema is broader than the
wire-v2 report contract.

Smallest correction: change `report.schema.json` task labels to `minItems: 2`,
make the positive handcrafted report fixture use a valid two-label shape, and add
a discriminating mutation that proves a one-label report is rejected.

## Residual risks at this checkpoint

- The exact 26-production-diagnostic Clippy inventory remains intentionally staged;
  this review does not claim a warning-free checkpoint. T027/T035 still own the
  clean gate.
- This review covers only T007-T014. Replay, inspect, comparison, multi-label, and
  T015+ behavior were not reviewed or inferred from current private contracts.
- JSON Schema establishes the published structural contract; runtime-only
  cross-record and semantic constraints remain runtime evidence and were not
  credited to T010 merely because schema cases passed.

No other T007-T014 finding was demonstrated within the bounded review.
