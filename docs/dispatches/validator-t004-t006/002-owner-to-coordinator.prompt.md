# Correct explicit-null admission before freezing the combined candidate

Role/model: existing Sol-high coordinator, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t004-t006/002-coordinator-to-owner.response.md`.

## Concrete owner concern and reassessment

While checking the selection boundary, the owner found a specific previously
uncovered T002 defect: optional typed wire fields use ordinary `Option<T>` without
a presence-sensitive deserializer. Serde maps explicit JSON null to None, so null
is indistinguishable from omission before semantic validation. In the accepted
wire source this affects fields on Source, ObservationDefinition, Prediction,
the abstention reason, and EvaluationConfig.

The normative rules distinguish omission from a provided typed value. In
particular, main-spec Canonical prediction artifact says confidence, when present,
is a finite number, probabilities contain exactly kind/values; Retained classifier
observations requires an object; Evaluation configuration permits an optional
array of episode IDs. Explicit `confidence: null`, `observations: null`, or
`episode_ids: null` cannot satisfy those shapes. Treating the last case as omitted
selection can select all episodes instead of rejecting malformed input. Only
opaque `input` explicitly permits null; null values nested in opaque configuration
or input must remain legal.

This is a concrete contract concern, not a request to repeat the earlier general
review. The cause is default optional deserialization erasing presence, and the
changed next approach is a direct omission-versus-null regression through the
actual decode/admission entry before repairing that boundary. It is worth the
bounded correction now because later schema/runtime parity and selection safety
depend on it. Earlier T002 Ready evidence did not exercise this distinction.

## Authorized bounded correction

Before freezing T004–T006, dispatch the sole developer to establish the described
failure with focused owning-module tests. Where confirmed, permit the smallest
wire-deserialization correction in `src/validation/wire.rs` and associated tests
in `src/validation.rs`, in addition to the active assignment's existing scope.
Optional means omission permitted; a present value must deserialize as its
specified non-null type. Check all existing optional wire fields governed by that
same contract, preserving legitimately opaque nulls. Do not add a permissive mode,
alternate raw decode path, public API, or blanket null ban over opaque JSON.

Required evidence includes: omitted fields still accepted; valid present values
still accepted; explicit null rejected as E_SCHEMA for optional typed scalar,
object and array fields, including confidence and episode_ids; explicit null
opaque input and nested opaque nulls still accepted. Preserve all previous T002
named tests and T004–T006 local results; rerun affected tests and the required
combined checks after the correction. No T007+ work is added.

If the production path already rejects these present nulls, provide its concrete
regression evidence instead of making an unnecessary edit. Any contradictory
normative allowance returns to the owner rather than inventing a stricter rule.

Include the result and changed T002 hashes in the combined candidate and its
independent review. The verifier checks this specific repaired distinction along
with T004–T006, reusing unchanged T002 evidence. The normal combined checkpoint's
single repair/recheck allowance remains, with further failures escalated.

Record this consequential correction in the existing Fizzy card and index the
prompt/response and worker dispatch, preserving earlier accepted history rather
than rewriting it. Save the required response and continue active coordination.
