# Validator T004-T006 fresh-developer repair completion 014

Role: replacement sole implementation/test writer.
Model: `gpt-5.6-terra`.
Reasoning: `high`.
Context inheritance: `fork_turns=none` (fresh context).
Repository: `/Users/dowwie/MyProjects/validator`.
Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t004-t006/014-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator’s verified single-label backbone](http://localhost:3006/1/cards/183).

You replace a retired developer and are the sole code/test writer. Do not delegate
or spawn agents. Complete the two original bounded repairs described below; do not
reconstruct the full dispatch history or begin T007-or-later work. Do not return a
final handoff after orientation, a partial file change, or only one repair unit.

## Required orientation

Read these files in full before editing:

- `/Users/dowwie/MyProjects/validator/AGENTS.md`.
- `/Users/dowwie/.agents/skills/manage-dev-team/SKILL.md` for role/write and
  evidence boundaries.
- `/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md` for bounded idiomatic
  Rust implementation and tests.
- `docs/specs/validator-v1.md`: Canonical golden dataset, Canonical prediction
  artifact, Retained classifier observations, Scored-choice profile, Evaluation
  configuration, and Validation and numerical rules.
- `docs/specs/validator-data-model.md`: Shared records and typed task data,
  Task-specific structures, Absence and uncertainty, Decode/validate/align/score,
  and Source organization and ownership.
- `docs/plans/validator/tasks/T002.json`, `T004.json`, `T005.json`, and `T006.json`.
- `docs/dispatches/validator-t004-t006/007-verifier-to-coordinator.response.md` for
  the two exact Revise findings and passing unchanged evidence.
- `docs/dispatches/validator-t004-t006/012-coordinator-to-owner.escalation.md` for
  the current partial state. Do not rely on the retired worker's completeness.

Specifications override task summaries. Existing accepted source is retained and
must not be rebuilt.

## Reconciled current partial baseline

The repository is unborn/untracked. Confirm these exact hashes before editing and
report any drift before dependent work:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `981783137668727726799ca99ee963f5333978cd6090ad9953b6d237a8e2c621` |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |
| `src/lib.rs` | `b60b0bf5d4fed5fe6045e7c1039f3c21e315dc46d9bc08159d56d2aa4ab35162` |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` |
| `src/main.rs` | `536e506bb90914c243a12b397b9a998f85ae2cbd9ba02dfd03a9e155ca5ca0f4` |
| `src/model.rs` | `ceffb61ab5e523fa4bf3161d5db5a6fcd1276137d951982da4382edb79c58c66` |
| `src/model/common.rs` | `24b1541ca7ec1cf2e49f06e8586c323e642b889711a5fa11c35d7dc22678b18c` |
| `src/model/single_label.rs` | `35fe11ada58053273708751631c079984f86e6e6fad3f7d24dcae502c4b92fe5` |
| `src/validation.rs` | `cea3789e2addbeedaaeee216dfa192b989d1508aac10c9cb25b3c3d1c81f23ba` |
| `src/validation/wire.rs` | `550d9f56a780d690d8606347be6cf8d6a5b9d750a22a4667e5b44e1c6a9011bc` |

Only Cargo.toml, validation.rs, and validation/wire.rs differ from frozen manifest
006. Their partial numeric correction is unreviewed and may be adjusted.

## Bounded write scope

You may modify only:

- `Cargo.toml` and `Cargo.lock` for the existing serde_json feature boundary;
- `src/validation/wire.rs` and `src/validation.rs` for numeric-token decoding,
  duplicate scanning, ordered checked admission, and owning tests;
- `src/model/common.rs` for checked `EvaluationConfig<Policy>` and complete
  `Population` invariants;
- `src/model/single_label.rs` for checked aligned-row/evaluation construction and
  owning tests;
- the required response file.

Preserve every other hash. Do not edit specs, plans, documentation/index, Fizzy,
schemas, fixtures, lib/error/main/model.rs, `.zvec-grep`, or later scoring/CLI code.
Do not stage, commit, clean, or reset. Do not start/stop task-owned caffeinate.

## Completion unit 1: numeric boundary and actual proof

The previous candidate failed because recursive duplicate scanning rejected legal
`1e400` as `E_PARSE`, so opaque RawValue retention and typed semantic diagnostics
were never reached. The partial code now enables serde_json `arbitrary_precision`
with `raw_value`, captures `JsonNumber` as RawValue, and parses it to binary64 in
semantic validation. Inspect and correct it as needed; do not assume it is sound.

Required final behavior:

- recursive duplicate-key scanning accepts syntactically legal JSON numbers beyond
  binary64 range while still rejecting duplicate keys at every depth;
- golden `input`, source configuration, and nested preparation configuration retain
  `1e400` as opaque numeric JSON through the actual production decode path;
- scalar observation `value: 1e400` reaches finite binary64 validation and returns
  `E_OBSERVATION`;
- the literal marker object `{"$serde_json::private::Number":"0.5"}` rejects as
  `E_SCHEMA` in typed numeric positions and remains the same object in opaque
  positions;
- typed-null rejection, legal omitted/present fields, opaque nulls, exact large
  integers, legal tagged numeric shapes, original submitted bytes, and binary64
  scoring arithmetic remain unchanged.

Add a nonzero `large_number_boundary` test through actual decode/admission that
covers all three opaque locations, nested scanning where needed, and the typed
scalar diagnostic. Run and retain:

```sh
cargo test --locked --lib large_number_boundary -- --nocapture
cargo test --locked --lib json_number_marker_collision -- --nocapture
cargo test --locked --lib typed_optional_nulls -- --nocapture
cargo test --locked --lib wire_tags_and_fields -- --nocapture
cargo test --locked --lib strict_json_keys -- --nocapture
cargo test --locked --lib opaque_number_and_null -- --nocapture
cargo test --locked --lib observation_contracts -- --nocapture
```

Each filter must execute a nonzero count. When this unit passes, send the
coordinator one concise runtime milestone with the concrete results and continue;
do not return the final response or request acceptance.

Use established serde_json mechanisms. Do not add a reserved-key or large-number
blacklist, relax duplicate detection, implement a parser/framework, add a fallback
or permissive entry, or create a lexical-number-spelling contract.

## Completion unit 2: checked T006 boundary

Implement the exact missing model invariants from verifier finding 2:

1. Add generic checked `EvaluationConfig<Policy>` in `src/model/common.rs` with
   private fields for exact nonblank population description, `EvaluationRole`,
   optional checked parent `RunId`, optional checked episode IDs retaining omitted
   versus explicit-empty selection, and a task-legal policy. Its checked
   constructor rejects duplicate requested IDs and validates all configuration
   semantics available before selection. Only `SingleLabelPolicy::AsRecorded` is
   admitted here; do not implement reject-below behavior.
2. In `validate_single_label`, convert the complete wire configuration into that
   checked model before population selection. Selection borrows/consumes checked
   IDs; it may not precede description, role, parent, policy, or duplicate-ID
   validation.
3. Make `Population` privately own the supplied checked dataset `ArtifactDigest`,
   exact dataset count, exact selected count, UUID-byte-sorted selected IDs, and
   UUID-byte-sorted unselected IDs, plus exact description/role/parent semantics
   either directly or via checked configuration. Its checked constructor enforces
   a complete unique disjoint partition and consistent counts; derive stored counts
   from checked collections when simpler.
4. Make `AlignedRow` and `SingleLabelEvaluation` construction checked/fallible or
   equivalently sealed from arbitrary later crate callers. Verify row identity,
   order, and count exactly equal population selection; each prediction ID equals
   its row ID; checked targets belong to the evaluation vocabulary; each prediction
   source exists; and the evaluation owns vocabulary, sources, population, legal
   policy/config, signal availability, and private aligned rows. Add no setter,
   `Deserialize`, raw DTO constructor, public SDK, builder framework, or trait.
5. Preserve the caller-supplied exact dataset digest after it matches the declared
   digest. Do not calculate hashes or access the filesystem.

Extend the one filtered `population_alignment` test to prove through the actual
entry and checked model accessors: stored digest, dataset/selected counts,
description, role, parent, selected/unselected UUID ordering, omitted/all,
explicit-empty, and subset behavior. Add an owning-module negative construction
case under the same filter proving mismatched/arbitrary aligned rows reject.

Run and retain:

```sh
cargo test --locked --lib population_alignment -- --nocapture
cargo test --locked --lib validate_before_selection -- --nocapture
cargo test --locked --lib dataset_digest_binding -- --nocapture
cargo test --locked --lib source_preparation_bindings -- --nocapture
cargo test --locked --lib categorical_admission -- --nocapture
cargo test --locked --lib scored_choice_ties -- --nocapture
cargo test --locked --lib artifact_signal_completeness -- --nocapture
```

## Final verification and handoff

After both units, rerun every named filter above so each executes nonzero, then:

```sh
cargo test --locked --lib -- --list
cargo fmt --all -- --check
cargo test --all-features --locked
cargo build --locked --bin validator
cargo clippy --all-targets --all-features --locked -- -D warnings
git diff --check
```

Clippy may exit 101 only on updated incomplete-consumer `dead_code`; enumerate exact
counts/locations and treat any other warning class as a blocker. Do not add
suppression, fake callers, placeholder uses, or unnecessary exports. Clean Clippy
remains mandatory at T014/T017.

Save one complete response only after both repairs and all commands. Include exact
input/output hashes, changed-file set, interfaces/invariants, before/after numeric
evidence, every named filter count/exit, combined command results, warning
classification, preserved unchanged evidence, and process/resource state. Do not
claim acceptance. If blocked, report the exact command, contract/source conflict,
and smallest owner decision while completing independent remaining work. Do not
end on an orientation summary or partial repair.

