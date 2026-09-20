# Validator T004-T006 corrected candidate review 007

Role/model: existing verifier, `gpt-5.6-sol`, reasoning `high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t004-t006/007-verifier-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator’s verified single-label backbone](http://localhost:3006/1/cards/183).

Independently review the exact corrected coherent T004 -> T005 -> T006 candidate
and the two owner-authorized T002 wire invariants incorporated before freeze. You
are read-only except for the required response file. Do not delegate or spawn
agents. Developer writes are paused.

Read in full, in this order:

1. `docs/plans/validator/tasks/T004.json`, `T005.json`, and `T006.json`, plus the
   relevant requirement mappings in `coverage.json`.
2. Main-spec Canonical golden dataset, Canonical prediction artifact, Retained
   classifier observations, Scored-choice profile, Evaluation configuration, and
   Validation and numerical rules. Read the data-model Shared records/typed task
   data, Task-specific structures, Absence/uncertainty, Decode/validate/align/score,
   and Source ownership sections. Specifications override summaries.
3. Developer dispatch/response 001, owner corrections 002 and 004, developer
   correction prompts 003 and 005, and developer correction response 003.
4. Superseding manifest 006, SHA-256
   `0aab3ef37c8635485c4e5c40ebbaa3dedcb03be374fcf9d44e02187f6495ae3d`.
5. The exact frozen Cargo and source files named in that manifest.

Recompute every manifest hash before and after commands. Return Blocked on drift.
Reuse unchanged owner-accepted T001/T003 and unaffected T002 evidence; do not
restart a repository-wide review or audit unrelated dependencies/design/style.

## T002 correction criteria

Confirm the actual strict decode boundary now preserves omission versus presence:

- every optional typed Source, ObservationDefinition, Prediction, abstention, and
  EvaluationConfig field accepts omission and a valid present value but rejects
  explicit JSON null with `E_SCHEMA`;
- `episode_ids: null` cannot reach omitted/select-all behavior through
  `validate_single_label`;
- required opaque golden `input` still accepts null, and opaque source/preparation
  configuration still accepts valid top-level/nested nulls;
- the literal object `{"$serde_json::private::Number":"0.5"}` rejects as
  `E_SCHEMA` in an actual numeric position and remains an object with the exact
  key/string value in opaque golden input and top-level/nested configuration;
- strict duplicate rejection, number-versus-object separation, legal wire shapes,
  original artifact-byte retention, and opaque big-integer/null behavior remain;
- changing serde_json from `arbitrary_precision` to `raw_value` does not weaken the
  specified numeric-value contract or create lexical-number requirements, a
  reserved-key exception, alternate decoder, or public construction bypass.

## T004 criteria

Check all closed source/observation variants and error boundaries: exact nonblank
IDs/model/definitions, scored-choice question IDs, retained unused sources,
finite/unbounded scalar, bounded scalar variants, nonempty/nonblank bounded vector
maps, legal categorical sum 0.99 without normalization or vocabulary binding,
observation/definition disagreement retention, omitted row observations, correct
`E_OBSERVATION` versus `E_PROVENANCE`, and nonempty unique in-range preparation
bindings. Confirm `Prediction<Output>` owns a real checked `ObservationSet` and
source/preparation opaque configuration remains correct through checked admission.

## T005 criteria

Check exact vocabulary coverage; finite `[0,1]` values; positive near-unit sum;
submitted values separate from vocabulary-order normalized working values; no
clip/infer/promotion; confidence validation; classifier non-argmax decisions;
signals on answered/abstained outputs; scored-choice class/question/distribution/
confidence requirements and exact tied maximum; and independent all-or-none
probability/confidence families across the complete submitted artifact before
selection, with empty artifacts having neither family.

## T006 criteria

Check the pure exact-byte/digest entry, complete semantic validation before
selection, duplicate IDs, omitted/all versus explicit-empty versus subset
selection, unique/existing requested IDs, exact selected prediction IDs,
missing/extra rejection, UUID-byte sorting, selected/unselected identities and
counts, description/role/checked parent preservation, malformed unselected record
rejection, digest mismatch, legal as-recorded policy, and the closed private
evaluation owning vocabulary/sources/population/policy/signals/aligned rows. No raw
DTO or arbitrary-vector construction bypass may enter scoring later.

## Commands and verdict

Run every filter and confirm it executes a nonzero count:

```sh
cargo test --locked --lib typed_optional_nulls -- --nocapture
cargo test --locked --lib json_number_marker_collision -- --nocapture
cargo test --locked --lib wire_tags_and_fields -- --nocapture
cargo test --locked --lib strict_json_keys -- --nocapture
cargo test --locked --lib opaque_number_and_null -- --nocapture
cargo test --locked --lib observation_contracts -- --nocapture
cargo test --locked --lib source_preparation_bindings -- --nocapture
cargo test --locked --lib categorical_admission -- --nocapture
cargo test --locked --lib scored_choice_ties -- --nocapture
cargo test --locked --lib artifact_signal_completeness -- --nocapture
cargo test --locked --lib population_alignment -- --nocapture
cargo test --locked --lib validate_before_selection -- --nocapture
cargo test --locked --lib dataset_digest_binding -- --nocapture
cargo test --locked --lib -- --list
cargo fmt --all -- --check
cargo test --all-features --locked
cargo build --locked --bin validator
cargo clippy --all-targets --all-features --locked -- -D warnings
git diff --check
```

Clippy is expected to exit 101 only on the disclosed incomplete-consumer
`dead_code` integration limit. Independently enumerate/classify it; do not call it
passed or waived. Any unused import, other lint class, suppression, fake caller,
placeholder, or unnecessary public expansion is a finding. Clean full Clippy
remains mandatory at T014 and T017.

Return one overall `Ready`, `Revise`, or `Blocked` verdict with separate T002-
correction, T004, T005, and T006 criterion evidence. A `Revise` finding must cite
the exact violated contract and smallest correction; the combined checkpoint has
one verifier-driven repair allowance. Stop once these criteria support a verdict.
Save exact pre/post hashes, command exits/counts, warning classification, remaining
limits, resource/process state, and read-only confirmation before returning only
the response path.

