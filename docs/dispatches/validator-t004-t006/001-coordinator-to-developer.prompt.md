# Validator T004-T006 checked-admission developer dispatch 001

Role/model: existing developer, `gpt-5.6-terra`, reasoning `high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required full response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t004-t006/001-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator’s verified single-label backbone](http://localhost:3006/1/cards/183).

## Outcome and execution order

Implement the exact ordered sequence **T004 -> T005 -> T006** as one coherent
checked-admission candidate. This is not an unspecified phase assignment. Implement
and locally verify each prerequisite before its consumer:

1. T004: checked sources, observation definitions/values, preparation bindings,
   and complete `Prediction<Output>` with a real `ObservationSet`.
2. T005: checked single-label outcomes, categorical distributions, confidence,
   scored-choice rules, and artifact-wide signal availability.
3. T006: decode/validate/digest-bind/select/align into a private complete sorted
   single-label evaluation and population.

You remain the sole implementation/test writer. Do not delegate or spawn agents.
No constituent task is independently accepted inside this turn; keep per-task
evidence for one frozen independent review and owner decision.

Read in full before editing:

- `docs/plans/validator/tasks/T004.json`, `T005.json`, and `T006.json`, in that
  order, plus `docs/plans/validator/execution-contract.md`.
- Main specification sections: Canonical golden dataset, Canonical prediction
  artifact, Retained classifier observations, Scored-choice profile, Evaluation
  configuration, Validation/numerical rules, and the relevant single-label
  evidence boundaries. Specifications override task summaries.
- Data model sections: Shared records/typed task data, Task-specific structures,
  Absence/uncertainty, Decode/validate/align/score, and source ownership.
- Build charter, repository `AGENTS.md`, accepted routing corrections, and Rust
  guidance already read.
- Each task's requirement IDs in coverage.json. Implement its owning portion only;
  later policy/scoring/artifact/CLI/multi-label work remains later.

T001, repaired T003, and repaired T002 are owner-accepted. T002 is bound to repair
manifest SHA-256
`3e0c155c1a64e6ec75d8bf518400fcc3c12d283ec3774de705833eb912843e91`.

## Accepted baseline

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `8c758ad6246244317ac02156077b417eab3cd49994daf2597c65ae86bb24629c` |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |
| `src/lib.rs` | `b60b0bf5d4fed5fe6045e7c1039f3c21e315dc46d9bc08159d56d2aa4ab35162` |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` |
| `src/model.rs` | `3275697520566394a2a20d4011ad28b3912d9d4773a44203980127d3f2d4d4d1` |
| `src/model/common.rs` | `ee0b0169d88707663c7b1540d1406868ba61cb0db2fd4d36dd2728d3baf9cecf` |
| `src/validation.rs` | `6c6c71080372b1d5b0a1bf707b88e1db694d88a142a3f2d94b8a467ff0020f72` |
| `src/validation/wire.rs` | `e81c528e0659e1fb4b2364be82639bbb27175947b042145a0f314ec00e827d17` |

The repository is unborn/untracked. Preserve unrelated/accepted files. Do not
stage, commit, clean, reset, or modify `.zvec-grep`.

## Combined write scope

You may create/modify:

- `src/model/common.rs` for T004 source/observation/preparation/Prediction and T006
  Population/EvaluationConfig records.
- `src/model/single_label.rs` for T005 signal/output/as-recorded types and T006
  private aligned evaluation/row types.
- `src/model.rs` only for necessary single-label submodule declaration/reexports;
  preserve accepted constants and task wrappers.
- `src/validation.rs` for ordered semantic conversion, artifact-wide validation,
  digest binding, selection and alignment, plus owning tests.
- `Cargo.toml`/`Cargo.lock` only if an established dependency/feature is
  demonstrably necessary for these current pure behaviors; ordinary required wiring
  is authorized, but prefer accepted/std facilities and explain any change.
- The required response file.

Do not edit `src/validation/wire.rs` unless a concrete accepted-T002 access defect
blocks semantic conversion; stop and report before changing its wire contract.
Do not edit lib/error/main, schemas, integration tests, fixtures, specs/plans/index,
Fizzy, session notes, or any artifact/scoring/CLI file.

## T004 exact contract: sources, observations, preparation, Prediction

- Convert strict source DTOs into checked `SourceDefinition` records retaining
  source kind, exact nonblank model, opaque configuration, optional/required
  question ID rules, evidence path strings, checked observation definitions, and
  optional preparation descriptor. Source IDs are exact nonblank `SourceId`s.
  Retain declared-but-unused sources; an empty prediction artifact can have none.
- `scored_choice` is single-label only and requires a nonblank question ID;
  `classifier` can carry optional question ID. Unknown/missing source references
  and observation definition mismatches use `E_PROVENANCE`.
- `ObservationDefinition` has closed kind, nonblank description, optional nonblank
  question ID. Definition names are nonblank/exact. No implicit gold target.
- Checked `Observation` variants are Scalar, Bernoulli, ReportedConfidence,
  Categorical, and LabelMarginals. Scalar requires finite binary64 with no implicit
  range. Bounded scalar variants require finite `[0,1]`. Vector maps must be
  nonempty, have nonblank exact keys, finite `[0,1]` values, and retain exact
  key/value meaning. Do not normalize or vocabulary-bind observations; categorical
  sum `0.99` is legal. Omission is unavailable, not zero/false/abstention.
- Invalid observation numeric values use `E_OBSERVATION`; unknown name or kind
  mismatch uses `E_PROVENANCE`; shape/unknown kind remains T002 `E_SCHEMA`.
- `PreparationDescriptor` requires nonblank method/version, opaque configuration,
  and nonempty unique in-range `evidence_indices` into that source's evidence array.
  Store the binding; no filesystem verification/execution (T011).
- Implement complete generic `Prediction<Output>` with private episode/source IDs,
  concrete output, and real checked `ObservationSet`. No raw/empty placeholder,
  mutable bypass, observation promotion, or provider-specific type.

Run before T005 and retain results:

```sh
cargo test --locked --lib observation_contracts -- --nocapture
cargo test --locked --lib source_preparation_bindings -- --nocapture
```

Both filters must run a nonzero count. Cover every observation variant, sum 0.99,
scalar/displayed-vector disagreement retained, missing row observation, unused
definitions/sources, invalid number/range/vector, unknown name/kind mismatch, and
empty/duplicate/out-of-range preparation indices.

## T005 exact contract: single-label output and signal completeness

- Create distinct `CategoricalDistribution` and checked `ReportedConfidence`.
  Categorical input requires exactly every vocabulary key, no extras, finite
  binary64 values in `[0,1]`, positive vocabulary-order sum, and
  `abs(sum-1)<=1e-9`. Preserve original submitted values and separate normalized
  working values in vocabulary order. Do not clip, insert, infer, or promote an
  observation. Confidence is finite `[0,1]`.
- `SingleLabelOutput` contains `Outcome<LabelIndex>` with optional categorical
  scoring distribution and confidence. General classifier decisions can differ
  from argmax; answered or abstained outputs retain valid signals.
- `scored_choice` requires a class outcome, source question ID, categorical
  distribution, confidence, and selected class equal to any exact tied maximum.
  Contradictions fail; vocabulary order defines diagnostic argmax later but must
  not replace a valid tied selected class.
- Calculate independent probability/confidence `SignalAvailability` over the entire
  submitted artifact before selection, regardless of observations. Each family is
  all-present or all-absent across all rows/sources. Partial families fail. Empty
  prediction artifacts have neither family.
- Implement only `AsRecorded` single-label policy needed now. Reject-below behavior
  belongs to T021; do not threshold decisions.

Run before T006 and retain results:

```sh
cargo test --locked --lib categorical_admission -- --nocapture
cargo test --locked --lib scored_choice_ties -- --nocapture
cargo test --locked --lib artifact_signal_completeness -- --nocapture
```

Each must run nonzero. Include missing/extra/wrong numeric keys, zero/bad sum,
near-unit normalization, non-argmax classifier choice, exact ties, contradictory
scored choice, partial families, empty artifact, and signals on abstentions.

## T006 exact contract: full validation, selection and alignment

- Provide a pure entry accepting exact dataset/prediction/config bytes and the
  caller-supplied exact-byte dataset `ArtifactDigest`. Decode through T002, then
  semantically validate all dataset/source/prediction/config records before
  selection. No filesystem/hash calculation; compare the supplied digest to the
  prediction artifact's checked digest. A caller that changes dataset bytes supplies
  their changed exact-byte digest; mismatch must reject with `E_PROVENANCE`.
- Convert dataset task/vocabulary/targets and every ID through checked T003 types.
  Only the current single-label/as-recorded path succeeds. Keep all validated fields
  private; no wire DTO or arbitrary-vector construction bypass.
- Duplicate dataset or prediction IDs are fatal. Validate malformed unselected
  golden records before selection. Omitted `episode_ids` selects all; explicit `[]`
  selects none; listed IDs are unique and must exist. Prediction IDs must equal the
  selected golden IDs exactly; missing/extra never becomes abstention.
- Sort selected and unselected IDs by UUID bytes. Preserve dataset count, selected
  count/IDs, unselected IDs, exact population description, role, optional checked
  parent RunId, sources, signal availability, and legal policy.
- Produce closed `ValidatedEvaluation::SingleLabel` containing a
  `SingleLabelEvaluation` that owns its checked vocabulary/sources/population/policy
  and private aligned rows, each with one expected target and prediction for the
  same ID. Scoring may only borrow this checked type later.

Run:

```sh
cargo test --locked --lib population_alignment -- --nocapture
cargo test --locked --lib validate_before_selection -- --nocapture
cargo test --locked --lib dataset_digest_binding -- --nocapture
```

Each must run nonzero. Cover omitted/all, explicit empty, explicit subset, unknown/
duplicate selection, duplicate/missing/extra predictions, canonical sorting,
counts/unselected/role/description/parent preservation, invalid unselected record,
and digest mismatch after changed dataset bytes with correspondingly changed digest.

## Combined checks and handoff

After T006, run and record:

```sh
cargo test --locked --lib -- --list
cargo fmt --all -- --check
cargo test --all-features --locked
cargo build --locked --bin validator
cargo clippy --all-targets --all-features --locked -- -D warnings
```

Clippy may still exit 101 only on disclosed incomplete-consumer `dead_code` in
private pre-CLI code. List exact classes/count/locations; no unused imports, other
lint class, suppression, fake caller, or placeholder. Clean Clippy is mandatory at
the later T014 boundary.

Save one complete response with separate T004/T005/T006 sections: delivered
interfaces, exact changed/current hashes, per-criterion/requirement evidence, each
named command/count/result, milestone failures and corrections, accepted input
hashes, final warning classification, limits/processes/resources. Briefly message
the coordinator after the T004 and T005 local checkpoints if any blocker or
contract ambiguity appears; routine successful progression needs no new prompt.

Do not claim any constituent independently verified/accepted. Return only after
the full response is saved, then send its path.
