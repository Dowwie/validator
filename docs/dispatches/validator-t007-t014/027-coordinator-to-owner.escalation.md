# T014 clean-gate sequencing escalation 027

Role/model: Sol-high coordinator to Astra-high owner.
Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card 183](http://localhost:3006/1/cards/183).

## Preserved working candidate

The sole developer stopped after saving
`026-schema-test-developer-to-coordinator.response.md`, SHA-256
`615f6334b650df5a7eecbcabf30bb05a1ffd6311826a97b6f4a0dd680eb3db3f`.
No T015, replay, comparison, inspection, or multi-label work was begun. The task-owned
caffeinate PID 84732 remains active.

All nine T014 deferred/application/process filters pass nonzero through the real
production API or built binary:

- `single_matrix_identities`, `f04_asymmetric_oracle`,
  `categorical_loss_oracles`, `signal_population_bins`,
  `bin_boundary_binary64`, and `report_sources_and_privacy`: 1 passed each;
- `cli_check_evaluate`, `cli_help_version_errors`, and
  `receipt_hash_matches_report`: 1 passed each.

The current candidate hashes match the developer handoff, including:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |
| `src/app.rs` | `18d400afe2f5c343512d4ed535b53905561f9770eb7b53731ed8344c7b610248` |
| `src/cli.rs` | `608ab90dcda6cd36f8b4d03575f47efc8f2a5977480a0c4bb670aa37933a0fbe` |
| `src/lib.rs` | `b0c5d00e000747368fc9e18e6459c61417dbbf24f06ac58077e9c4cf382ceac7` |
| `src/main.rs` | `ca701aa73cb1d687d9ff086e654bd9b08e6c285a91c9577c3b2d1eb7fc54a3c0` |
| `src/error.rs` | `5e47f1afde84a682e6d33744133820a43c14a761383717fc91012c63b330abcb` |
| `tests/conformance.rs` | `16bc7d8eb4644491ae86f30a85a5f3a69f8b471e8106351184b67e30809f9637` |
| `tests/cli.rs` | `d0e255789545846fb57c97c3338cee6344d5cf2f9f915fc3a3df88eaf6d77e4b` |

## Reproduced blocker

The coordinator reran the exact mandatory command against those hashes:

```sh
cargo clippy --all-targets --all-features --locked -- -D warnings
```

It exited 101 with 34 primary errors: 30 `dead_code` diagnostics plus four ordinary
Clippy style diagnostics. The final `could not compile` line is a summary, not a
35th primary defect. No suppression, fake read/consumer, public export, or code
change was made.

The four style diagnostics are independently repairable in one bounded pass:

| Path/symbol | Diagnostic | Smallest correction |
|---|---|---|
| `src/app.rs:127`, `RunId` | `clone_on_copy` | Remove `.clone()`. |
| `src/evaluation/single_label.rs:234`, `probability_results` | `too_many_arguments` | Group the already-related probability accumulator inputs without changing formulas. |
| `src/evaluation/single_label.rs:746`, `sum` | `redundant_closure` | Pass `checked_add` directly. |
| `src/model/common.rs:1227`, `MetricResult::ratio` | `needless_question_mark` | Return `ratio_with_population` directly. |

The complete dead-code mapping is:

| Exact diagnostic symbol/group | Current or later criterion status |
|---|---|
| `src/evaluation.rs:14 checked_mul` | Required by T018 multi-label `N*K` accounting. |
| `src/model.rs:8 TaskDefinition` and `single_label`, `multi_label`, `vocabulary`, `is_single_label` | Accepted closed task boundary; T018 is the first complete multi-label consumer. |
| `src/model.rs:42 SingleLabelTask` plus `new`, `vocabulary` | Accepted T003 task model; no additional honest T014 path consumer. T018 consumes the closed task model. |
| `src/model.rs:60 MultiLabelTask` plus `new`, `vocabulary` | Required directly by T018. |
| `src/model/common.rs:400 LabelVocabulary::for_multi_label`, `label_set` | Required by T018 multi-label admission. |
| `src/model/common.rs:488 LabelSet` plus `is_empty`, `contains` | Required by T018 checked set targets/outputs. |
| `src/model/common.rs:508 Episode` plus `new`, `id`, `input`, `target` | Accepted opaque-episode contract; T015 explicit inspection is the first complete payload consumer. |
| `src/model/common.rs:781 ObservationSet::is_empty`, `values` | `values` is needed by T015 inspection; `is_empty` is genuinely obsolete unless that presentation requires it. |
| `src/model/common.rs:808 PreparationDescriptor::new` | Checked construction boundary expected by T029; current wire admission legitimately uses `new_raw`. |
| `src/model/common.rs:885 SourceDefinition::new` | Checked construction boundary expected by T029; current wire admission legitimately uses `new_raw`. |
| `src/model/common.rs:1016 EvaluationConfig.policy` and `policy()` | T016 policy compatibility/difference comparison. |
| `src/model/common.rs:1076 Population.dataset_digest` and `dataset_digest()` | T016 exact-dataset compatibility. |
| `src/model/common.rs:1167 MetricUnit::LabelDecision` | T018 multi-label metrics. |
| `src/model/common.rs:1220 MetricResult::ratio` and `status_value`, `population_count`, `unit`, `scope`, `numerator`, `denominator`, `special_value` | T016 typed metric comparison and T018 label-decision metrics. |
| `src/model/single_label.rs:115 signal_availability` | T018 artifact-wide signal admission; T014 single-label validation uses the lower checked flag path. |
| `src/validation.rs:24 Decoded.bytes` plus `bytes()`, `value()` | Genuinely obsolete after exact bytes moved to `InputArtifacts` and admission consumes DTOs through `into_value`. |
| `src/validation/wire.rs:15 GoldenDataset.schema_version` | Current T010 strict DTO field; Serde invokes the validating deserializer, so there is no honest later value consumer. |
| `src/validation/wire.rs:23 SingleLabelTask.kind` | Current tagged strict DTO discriminator consumed by Serde shape selection; no honest later value consumer. |
| `src/validation/wire.rs:38 GoldenEpisode.input` | Current required opaque-input presence/retention field; T015 reads the stored exact snapshot for inspection, rather than this consumed admission DTO. |
| `src/validation/wire.rs:51 PredictionArtifact.schema_version` | Current T010 strict DTO field; Serde performs the validation. |
| `src/validation/wire.rs:218 EvaluationConfig.schema_version` | Current T010 strict DTO field; Serde performs the validation. |
| `src/validation/wire.rs:239 SingleLabelDecision.signal`, `minimum` | Current strict rejected-policy variant payload. Its presence/shape is validated before T014 rejects the unsupported policy; there is no legal T014 value consumer. |

This makes the conflict narrower than the first report suggested. Several items are
honestly obsolete and can be removed; several style issues can be fixed; some DTO
fields are semantically consumed only by Serde; and the rest have ratified later
consumers. The current T014 prompt forbids early multi-label/replay work, public-SDK
broadening, fake reads, and lint suppression, while the ratified clean gate denies
all warnings now.

## Remaining T014 process-evidence gaps

Owner review of the actual `tests/cli.rs` found that the nine green filter names do
not yet establish the complete T014 process contract. These are existing task/
dispatch requirements and remain mandatory after the lint sequencing decision:

| Required evidence | Current test gap |
|---|---|
| Externally emitted success/error documents validate against `check.schema.json`, `receipt.schema.json`, and `error.schema.json`. | The process helper only parses stdout as generic JSON and checks `kind`; it never invokes the locked schema engine on CLI output. |
| Missing/unreadable input exits 3 with a safe structured error. | No missing-input process case exists. |
| Existing destination exits 3, remains unchanged, and leaves no partial result. | No process-level existing-output case exists. |
| Structurally/semantically invalid input exits 2 with safe structured JSON and no output. | No invalid-input process case exists. |
| Unknown flags, missing flag values, duplicate flags/settings, and unsupported settings fail as structured input errors. | The only malformed-CLI case is the unknown command `compare`; no unknown-flag, missing-value, duplicate-setting, or unsupported-setting case exists. |
| Published golden/predictions/config and evidence bytes exactly equal the submitted sources. | The success test checks only that four files exist. It compares no bytes and supplies no bound evidence file. |
| A failure that actually handles secret-bearing input/evidence does not leak the sentinel. | The sentinel assertion runs only the `compare` unknown-command path, which never opens or processes the sentinel-bearing files. It is therefore not privacy evidence. |

The next developer pass must add these assertions through the real built binary;
static schema examples remain insufficient. The owner can resolve lint sequencing
and authorize this bounded evidence completion in one correction without widening
T014 behavior.

## Required owner decision

Please select the smallest criterion-preserving sequencing rule. The coordinator
will not infer one. Plausible bounded decisions are:

1. authorize one T014 cleanup pass for the four style issues and genuinely obsolete
   members, explicitly disposition the Serde-only DTO dead-code diagnostics, and
   defer only the enumerated ratified-later-consumer dead-code gate until their
   owning tasks; or
2. authorize removal/deferment of the enumerated later-consumer structures now,
   with their owning tasks restoring them when implemented, then require the full
   warning-denied T014 gate; or
3. move the full warning-denied gate to the first sequence containing all named
   consumers, while still requiring T014 tests/build and a bounded no-new-warning
   comparison for the files T014 changed.

The candidate is not frozen or ready for independent review until this decision is
implemented and the resulting required gate is rerun.

## Exact Clippy output

The following is the complete stderr from the reproduced command.

```text
    Checking ahash v0.8.12
    Checking validator v0.1.0 (/Users/dowwie/MyProjects/validator)
    Checking referencing v0.37.4
    Checking jsonschema v0.37.4
error: function `checked_mul` is never used
  --> src/evaluation.rs:14:15
   |
14 | pub(crate) fn checked_mul(left: u64, right: u64) -> Result<u64> {
   |               ^^^^^^^^^^^
   |
   = note: `-D dead-code` implied by `-D warnings`
   = help: to override `-D warnings` add `#[expect(dead_code)]` or `#[allow(dead_code)]`

error: enum `TaskDefinition` is never used
 --> src/model.rs:8:10
  |
8 | pub enum TaskDefinition {
  |          ^^^^^^^^^^^^^^

error: associated items `single_label`, `multi_label`, `vocabulary`, and `is_single_label` are never used
  --> src/model.rs:17:12
   |
15 | impl TaskDefinition {
   | ------------------- associated items in this implementation
16 |     /// Creates a single-label task from at least two exact labels.
17 |     pub fn single_label(labels: impl IntoIterator<Item = String>) -> crate::Result<Self> {
   |            ^^^^^^^^^^^^
...
22 |     pub fn multi_label(labels: impl IntoIterator<Item = String>) -> crate::Result<Self> {
   |            ^^^^^^^^^^^
...
27 |     pub const fn vocabulary(&self) -> &LabelVocabulary {
   |                  ^^^^^^^^^^
...
35 |     pub const fn is_single_label(&self) -> bool {
   |                  ^^^^^^^^^^^^^^^

error: struct `SingleLabelTask` is never constructed
  --> src/model.rs:42:12
   |
42 | pub struct SingleLabelTask {
   |            ^^^^^^^^^^^^^^^

error: associated items `new` and `vocabulary` are never used
  --> src/model.rs:48:12
   |
46 | impl SingleLabelTask {
   | -------------------- associated items in this implementation
47 |     /// Creates a single-label vocabulary from at least two exact labels.
48 |     pub fn new(labels: impl IntoIterator<Item = String>) -> crate::Result<Self> {
   |            ^^^
...
53 |     pub const fn vocabulary(&self) -> &LabelVocabulary {
   |                  ^^^^^^^^^^

error: struct `MultiLabelTask` is never constructed
  --> src/model.rs:60:12
   |
60 | pub struct MultiLabelTask {
   |            ^^^^^^^^^^^^^^

error: associated items `new` and `vocabulary` are never used
  --> src/model.rs:66:12
   |
64 | impl MultiLabelTask {
   | ------------------- associated items in this implementation
65 |     /// Creates a multi-label vocabulary from at least one exact label.
66 |     pub fn new(labels: impl IntoIterator<Item = String>) -> crate::Result<Self> {
   |            ^^^
...
71 |     pub const fn vocabulary(&self) -> &LabelVocabulary {
   |                  ^^^^^^^^^^

error: associated items `for_multi_label` and `label_set` are never used
   --> src/model/common.rs:400:12
    |
395 | impl LabelVocabulary {
    | -------------------- associated items in this implementation
...
400 |     pub fn for_multi_label(labels: impl IntoIterator<Item = String>) -> Result<Self> {
    |            ^^^^^^^^^^^^^^^
...
458 |     pub fn label_set<'a>(&self, labels: impl IntoIterator<Item = &'a str>) -> Result<LabelSet> {
    |            ^^^^^^^^^

error: struct `LabelSet` is never constructed
   --> src/model/common.rs:488:12
    |
488 | pub struct LabelSet {
    |            ^^^^^^^^

error: methods `is_empty` and `contains` are never used
   --> src/model/common.rs:495:12
    |
493 | impl LabelSet {
    | ------------- methods in this implementation
494 |     /// Returns whether the set has no labels.
495 |     pub fn is_empty(&self) -> bool {
    |            ^^^^^^^^
...
500 |     pub fn contains(&self, index: &LabelIndex) -> bool {
    |            ^^^^^^^^

error: struct `Episode` is never constructed
   --> src/model/common.rs:508:12
    |
508 | pub struct Episode<Target> {
    |            ^^^^^^^

error: associated items `new`, `id`, `input`, and `target` are never used
   --> src/model/common.rs:516:12
    |
514 | impl<Target> Episode<Target> {
    | ---------------------------- associated items in this implementation
515 |     /// Constructs an episode from already checked identity and target values.
516 |     pub fn new(id: EpisodeId, input: serde_json::Value, target: Target) -> Self {
    |            ^^^
...
521 |     pub fn id(&self) -> EpisodeId {
    |            ^^
...
526 |     pub fn input(&self) -> &serde_json::Value {
    |            ^^^^^
...
531 |     pub fn target(&self) -> &Target {
    |            ^^^^^^

error: methods `is_empty` and `values` are never used
   --> src/model/common.rs:781:12
    |
763 | impl ObservationSet {
    | ------------------- methods in this implementation
...
781 |     pub fn is_empty(&self) -> bool {
    |            ^^^^^^^^
...
785 |     pub(crate) fn values(&self) -> &HashMap<String, Observation> {
    |                   ^^^^^^

error: associated function `new` is never used
   --> src/model/common.rs:808:12
    |
806 | impl PreparationDescriptor {
    | -------------------------- associated function in this implementation
807 |     /// Validates nonblank metadata and unique in-range evidence indices.
808 |     pub fn new(
    |            ^^^

error: associated function `new` is never used
   --> src/model/common.rs:885:12
    |
883 | impl SourceDefinition {
    | --------------------- associated function in this implementation
884 |     /// Creates a checked source definition.
885 |     pub fn new(
    |            ^^^

error: field `policy` is never read
    --> src/model/common.rs:1016:5
     |
1011 | pub struct EvaluationConfig<Policy> {
     |            ---------------- field in this struct
...
1016 |     policy: Policy,
     |     ^^^^^^
     |
     = note: `EvaluationConfig` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

error: method `policy` is never used
    --> src/model/common.rs:1041:18
     |
1019 | impl<Policy> EvaluationConfig<Policy> {
     | ------------------------------------- method in this implementation
...
1041 |     pub const fn policy(&self) -> &Policy {
     |                  ^^^^^^

error: field `dataset_digest` is never read
    --> src/model/common.rs:1076:5
     |
1075 | pub struct Population {
     |            ---------- field in this struct
1076 |     dataset_digest: ArtifactDigest,
     |     ^^^^^^^^^^^^^^
     |
     = note: `Population` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

error: method `dataset_digest` is never used
    --> src/model/common.rs:1128:18
     |
1093 | impl Population {
     | --------------- method in this implementation
...
1128 |     pub const fn dataset_digest(&self) -> ArtifactDigest {
     |                  ^^^^^^^^^^^^^^

error: variant `LabelDecision` is never constructed
    --> src/model/common.rs:1167:5
     |
1163 | pub enum MetricUnit {
     |          ---------- variant in this enum
...
1167 |     LabelDecision,
     |     ^^^^^^^^^^^^^
     |
     = note: `MetricUnit` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

error: multiple associated items are never used
    --> src/model/common.rs:1220:12
     |
1218 | impl MetricResult {
     | ----------------- associated items in this implementation
1219 |     /// Constructs a finite ratio metric.
1220 |     pub fn ratio(
     |            ^^^^^
...
1355 |     pub(crate) const fn status_value(&self) -> MetricStatus {
     |                         ^^^^^^^^^^^^
...
1359 |     pub(crate) const fn population_count(&self) -> u64 {
     |                         ^^^^^^^^^^^^^^^^
...
1363 |     pub(crate) const fn unit(&self) -> MetricUnit {
     |                         ^^^^
...
1367 |     pub(crate) const fn scope(&self) -> MetricScope {
     |                         ^^^^^
...
1371 |     pub(crate) const fn numerator(&self) -> Option<u64> {
     |                         ^^^^^^^^^
...
1375 |     pub(crate) const fn denominator(&self) -> Option<u64> {
     |                         ^^^^^^^^^^^
...
1379 |     pub(crate) const fn special_value(&self) -> Option<&'static str> {
     |                         ^^^^^^^^^^^^^

error: function `signal_availability` is never used
   --> src/model/single_label.rs:115:8
    |
115 | pub fn signal_availability(outputs: &[SingleLabelOutput]) -> Result<SignalAvailability> {
    |        ^^^^^^^^^^^^^^^^^^^

error: field `bytes` is never read
  --> src/validation.rs:24:5
   |
23 | pub(crate) struct Decoded<T> {
   |                   ------- field in this struct
24 |     bytes: Vec<u8>,
   |     ^^^^^
   |
   = note: `Decoded` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

error: methods `bytes` and `value` are never used
  --> src/validation.rs:30:19
   |
28 | impl<T> Decoded<T> {
   | ------------------ methods in this implementation
29 |     /// Returns the original submitted bytes without reconstructing JSON.
30 |     pub(crate) fn bytes(&self) -> &[u8] {
   |                   ^^^^^
...
35 |     pub(crate) fn value(&self) -> &T {
   |                   ^^^^^

error: field `schema_version` is never read
  --> src/validation/wire.rs:15:16
   |
13 | pub(crate) struct GoldenDataset {
   |                   ------------- field in this struct
14 |     #[serde(deserialize_with = "schema_version_two")]
15 |     pub(crate) schema_version: (),
   |                ^^^^^^^^^^^^^^
   |
   = note: `GoldenDataset` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

error: field `kind` is never read
  --> src/validation/wire.rs:23:16
   |
22 | pub(crate) struct SingleLabelTask {
   |                   --------------- field in this struct
23 |     pub(crate) kind: SingleLabelTag,
   |                ^^^^
   |
   = note: `SingleLabelTask` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

error: field `input` is never read
  --> src/validation/wire.rs:38:16
   |
35 | pub(crate) struct GoldenEpisode {
   |                   ------------- field in this struct
...
38 |     pub(crate) input: Box<RawValue>,
   |                ^^^^^
   |
   = note: `GoldenEpisode` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

error: field `schema_version` is never read
  --> src/validation/wire.rs:51:16
   |
49 | pub(crate) struct PredictionArtifact {
   |                   ------------------ field in this struct
50 |     #[serde(deserialize_with = "schema_version_two")]
51 |     pub(crate) schema_version: (),
   |                ^^^^^^^^^^^^^^
   |
   = note: `PredictionArtifact` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

error: field `schema_version` is never read
   --> src/validation/wire.rs:218:16
    |
216 | pub(crate) struct EvaluationConfig {
    |                   ---------------- field in this struct
217 |     #[serde(deserialize_with = "schema_version_two")]
218 |     pub(crate) schema_version: (),
    |                ^^^^^^^^^^^^^^
    |
    = note: `EvaluationConfig` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

error: fields `signal` and `minimum` are never read
   --> src/validation/wire.rs:239:9
    |
238 |     RejectBelow {
    |     ----------- fields in this variant
239 |         signal: RejectionSignal,
    |         ^^^^^^
240 |         minimum: JsonNumber,
    |         ^^^^^^^
    |
    = note: `SingleLabelDecision` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

error: using `clone` on type `RunId` which implements the `Copy` trait
   --> src/app.rs:127:17
    |
127 |                 run_id.clone(),
    |                 ^^^^^^^^^^^^^^ help: try removing the `clone` call: `run_id`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#clone_on_copy
    = note: `-D clippy::clone-on-copy` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::clone_on_copy)]`

error: this function has too many arguments (8/7)
   --> src/evaluation/single_label.rs:234:1
    |
234 | / fn probability_results(
235 | |     available: bool,
236 | |     total: u64,
237 | |     loss: f64,
...   |
242 | |     infinite_loss: bool,
243 | | ) -> Result<SingleLabelProbabilityResults> {
    | |__________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`

error: redundant closure
   --> src/evaluation/single_label.rs:746:22
    |
746 |         .try_fold(0, |total, value| checked_add(total, value))
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace the closure with the function itself: `checked_add`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#redundant_closure
    = note: `-D clippy::redundant-closure` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::redundant_closure)]`

error: enclosing `Ok` and `?` operator are unneeded
    --> src/model/common.rs:1227:20
     |
1227 |               return Ok(Self::ratio_with_population(
     |  ____________________^
1228 | |                 numerator,
1229 | |                 denominator,
1230 | |                 0,
1231 | |                 scope,
1232 | |                 unit,
1233 | |             )?);
     | |_______________^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#needless_question_mark
     = note: `-D clippy::needless-question-mark` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::needless_question_mark)]`
help: remove the enclosing `Ok` and `?` operator
     |
1227 ~             return Self::ratio_with_population(
1228 |                 numerator,
 ...
1232 |                 unit,
1233 ~             );
     |

error: could not compile `validator` (lib) due to 34 previous errors
warning: build failed, waiting for other jobs to finish...
```
