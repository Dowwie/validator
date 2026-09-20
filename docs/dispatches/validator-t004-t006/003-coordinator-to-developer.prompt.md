# Validator T004-T006 explicit-null correction dispatch 003

Role/model: existing developer, `gpt-5.6-terra`, reasoning `high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t004-t006/003-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator’s verified single-label backbone](http://localhost:3006/1/cards/183).

This is an owner-authorized pre-freeze correction to the coherent T004-T006
candidate. It does not consume the later verifier-driven repair allowance. You
remain the sole implementation/test writer; do not delegate or spawn agents.

Read in full before editing:

- `002-owner-to-coordinator.prompt.md` and `002-coordinator-to-owner.response.md`.
- Your `001-developer-to-coordinator.response.md` and the exact current source.
- Main-spec Canonical golden dataset, Canonical prediction artifact, Retained
  classifier observations, and Evaluation configuration sections.
- The accepted T002 task contract and current `src/validation/wire.rs` decoder.

## Required reproduction and correction

Establish through focused owning-module tests and the actual decode/admission
entry whether ordinary `Option<T>` fields accept explicit JSON null as omission.
The expected contract is:

- omission of an optional typed field remains legal;
- a valid present typed value remains legal;
- a present JSON null for an optional typed scalar, object, array, or string fails
  strict decoding with `E_SCHEMA`;
- at minimum, explicit null `confidence`, `observations`, and `episode_ids` are
  rejected, so null selection cannot take the omitted/select-all path;
- check every current optional typed wire field governed by the same rule:
  Source `question_id`, `evidence`, `observation_definitions`, and `preparation`;
  ObservationDefinition `question_id`; Prediction `probabilities`, `confidence`,
  and `observations`; abstention `reason`; EvaluationConfig `episode_ids` and
  `parent_run_id`;
- the required opaque golden `input` explicitly remains allowed to be null;
  opaque source/preparation configuration may contain top-level or nested nulls.

If the production path already rejects all present typed nulls, add the concrete
regressions and do not edit production code unnecessarily. Otherwise make the
smallest presence-sensitive deserialization correction in
`src/validation/wire.rs`, with associated owning tests in `src/validation.rs`.
Optional means omittable, not nullable. Do not add a raw alternate decoder,
permissive mode, blanket JSON-null rejection, public API, dependency, schema work,
or T007-or-later behavior. Do not change legitimate opaque-number or opaque-null
retention. Do not suppress lints or invent callers.

## Frozen baseline before correction

The pre-correction implementation hashes are recorded in
`002-coordinator-candidate-manifest.md`, SHA-256
`4fdb0782c7b59d9ec5af3f025f61093adcc30cd51edcc5adf8d406f26bb6b3d2`.
Only `src/validation/wire.rs`, `src/validation.rs`, and your required response are
write-authorized. Preserve all other candidate hashes. The repository remains
unborn/untracked; do not stage, commit, clean, reset, or modify `.zvec-grep`.

## Required evidence

Add a focused nonzero test filter named `typed_optional_nulls` (or report the exact
owning filter if the existing style requires a clearer name). Its cases must show
omitted and valid present values succeed, every listed typed field rejects explicit
null as `E_SCHEMA`, and opaque input/configuration nulls survive. Demonstrate the
`episode_ids: null` case through the real validation entry or otherwise prove it
cannot reach the select-all branch.

Rerun and record:

```sh
cargo test --locked --lib typed_optional_nulls -- --nocapture
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

Every named filter must run a nonzero count. Clippy may still exit 101 only on the
known incomplete-consumer `dead_code`; enumerate updated counts and locations and
report any other warning class as a blocker. Clean Clippy remains mandatory at
T014/T017.

Save a complete response with the reproduction, exact implementation and response
hashes, focused cases, all command exits/counts, preservation of prior evidence,
Clippy classification, processes/resources, and any blocker. Do not claim the
combined candidate accepted. Return only after saving the response, then message
the coordinator with its path.

