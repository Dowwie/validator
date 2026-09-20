# Validator T004-T006 single bounded repair 010

Role/model: existing developer, `gpt-5.6-terra`, reasoning `high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t004-t006/010-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator’s verified single-label backbone](http://localhost:3006/1/cards/183).

This is the coherent checkpoint's single authorized verifier-driven repair. Read
the complete verifier response 007, especially required findings 1 and 2, then
repair exactly those findings. All other T002-correction, T004, T005, and central
T006 evidence passed and must remain unchanged. Do not delegate or spawn agents.

## Frozen repair baseline

Bind the superseded candidate manifest SHA-256
`0aab3ef37c8635485c4e5c40ebbaa3dedcb03be374fcf9d44e02187f6495ae3d`
and verifier response SHA-256
`7b6cfe73ff1c361130cc8b5cd2bd184fc7d6ffe5bbf5188d950950d7155ecb60`.
Confirm current implementation hashes equal manifest 006 before editing.

You may modify only:

- `Cargo.toml` and `Cargo.lock` for the bounded serde_json feature correction;
- `src/validation/wire.rs` and `src/validation.rs` for actual-token numeric
  decoding, duplicate scanning, ordered checked config/admission, and owning tests;
- `src/model/common.rs` for checked `EvaluationConfig<Policy>` and complete
  `Population` invariants;
- `src/model/single_label.rs` for checked aligned-row/evaluation construction and
  owning tests;
- this required response.

Preserve all other hashes. Do not edit specs, plans, docs/index, Fizzy, schemas,
fixtures, lib/error/main, T007-or-later code, or `.zvec-grep`. The repository is
unborn/untracked; do not stage, commit, clean, or reset.

## Repair 1: legal large numbers and actual numeric tokens

Correct the exact frozen behavior demonstrated by the verifier:

- duplicate-key scanning must accept syntactically legal opaque numbers outside
  binary64 range, including `1e400`, while still rejecting duplicate keys at every
  depth;
- golden `input`, source configuration, and nested preparation configuration must
  retain `1e400` as opaque numeric JSON through the production decode path;
- a typed scalar observation with `value: 1e400` must pass syntax/shape decoding,
  reach finite binary64 semantic validation, and return `E_OBSERVATION`;
- the literal object `{"$serde_json::private::Number":"0.5"}` must still reject
  as `E_SCHEMA` in every typed numeric position and remain the same object in
  opaque positions;
- explicit typed nulls, opaque nulls, exact large integers, legal wire shapes,
  duplicate rejection, original exact input bytes, and binary64 scoring arithmetic
  remain unchanged.

The verifier's bounded route is allowed: enable serde_json `arbitrary_precision`
alongside `raw_value`, and make `JsonNumber` accept only a raw JSON numeric token
before constructing/retaining its numeric value. Use the smallest established
serde_json mechanism. Do not blacklist marker keys or large numbers, relax the
duplicate scan, implement a parser/framework, add fallback/permissive modes, or
add a lexical-number-spelling contract.

Add a focused nonzero filter `large_number_boundary` covering the three opaque
locations and typed scalar diagnostic through the actual entry. Include nested
opaque arrays/objects only as needed to prove the existing recursive scanner no
longer imposes numeric range.

## Repair 2: complete checked T006 model boundary

Implement the already-owned T006 structures and invariants without adding later
policy behavior:

1. Add checked generic `EvaluationConfig<Policy>` in `src/model/common.rs` with
   private fields for exact nonblank population description, role, optional
   checked parent `RunId`, optional checked selected episode IDs preserving the
   omitted-versus-explicit-empty distinction, and a task-legal policy. Its checked
   constructor validates all configuration semantics available before selection,
   including duplicate requested IDs. Only `SingleLabelPolicy::AsRecorded` is
   admitted in this task.
2. Construct that checked configuration before population selection. Selection
   may borrow/consume its checked episode IDs but must not occur before description,
   role, parent, policy, and selection-ID validation.
3. Make `Population` privately own the supplied checked dataset `ArtifactDigest`,
   exact dataset count, exact selected count, UUID-byte-sorted selected IDs, and
   UUID-byte-sorted unselected IDs, plus the declared description/role/parent
   semantics (directly or through the checked configuration without duplication).
   Its checked constructor must enforce a complete unique disjoint partition and
   consistent counts; derive counts from checked collections where simpler.
4. Make aligned-row and `SingleLabelEvaluation` construction checked and
   fallible, or equivalently seal it so later crate modules cannot create an
   arbitrary-vector evaluation. At minimum verify row identity/order/count equals
   the population selection, each prediction ID equals its row ID, checked targets
   belong to the evaluation vocabulary, every prediction source exists, and the
   evaluation owns the checked vocabulary, sources, population, legal policy,
   signal availability, and aligned rows. Keep fields private; add no setter,
   `Deserialize`, raw DTO constructor, public SDK, builder framework, or trait
   abstraction.
5. Preserve the caller-supplied exact dataset digest after matching it to the
   prediction-declared digest. No hash calculation or filesystem access is added.

Extend `population_alignment` so one nonzero filtered test demonstrates through
the actual entry and checked model accessors: digest, dataset/selected counts,
description, role, parent, selected/unselected UUID ordering, omitted/all,
explicit empty, and subset behavior. Include an owning-module negative case that
the fallible evaluation constructor rejects mismatched/arbitrary rows. Keep
`validate_before_selection` and `dataset_digest_binding` evidence intact.

## Required checks and handoff

Run the focused repair filters first:

```sh
cargo test --locked --lib large_number_boundary -- --nocapture
cargo test --locked --lib population_alignment -- --nocapture
cargo test --locked --lib validate_before_selection -- --nocapture
cargo test --locked --lib dataset_digest_binding -- --nocapture
```

Then rerun every unchanged named filter from prompt 007, including
`typed_optional_nulls`, `json_number_marker_collision`, the three T002 filters,
the other T004/T005 filters, and all eight T004-T006 filters. Finish with:

```sh
cargo test --locked --lib -- --list
cargo fmt --all -- --check
cargo test --all-features --locked
cargo build --locked --bin validator
cargo clippy --all-targets --all-features --locked -- -D warnings
git diff --check
```

Every named filter must execute a nonzero count. Clippy may exit 101 only on the
updated incomplete-consumer `dead_code`; record exact counts/locations and any
other warning class as a blocker. Do not add suppression, fake callers,
placeholder uses, or unnecessary exports. Clean Clippy remains mandatory at
T014/T017.

Save one response with before/after reproduction for both findings, exact hashes,
new interfaces/invariants, focused and preserved test evidence, command exits and
counts, warning classification, process/resource state, and no claim of
acceptance. This is the only repair; any unresolved or new material failure must
be reported for owner reassessment rather than another implementation loop.

