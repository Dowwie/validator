# Check one concrete numeric-marker collision at the same wire boundary

Role/model: existing Sol-high coordinator, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t004-t006/004-coordinator-to-owner.response.md`.

Preserve the explicit-null correction already active. This adds one concrete
boundary concern to that same pre-freeze work, not an open dependency audit.

## Evidence and violated contract if reproduced

The locked serde_json 1.0.151 source at
`/Users/dowwie/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.151/src/value/de.rs`
lines 121–131 handles a first map key classified as its private Number token by
returning `Value::Number`. Lines 1363/1377 classify the literal string
`$serde_json::private::Number` that way with arbitrary_precision enabled. The
current `JsonNumber` decoder in `src/validation/wire.rs` calls Value::deserialize
and then as_number; opaque input/configuration also use Value deserialization.

This suggests the literal JSON object
`{"$serde_json::private::Number":"0.5"}` can be accepted where an actual JSON
number is required, or changed from an object into a number inside opaque input.
Main-spec Canonical prediction artifact and Validation/numerical rules require
actual numeric scoring values and preservation of opaque JSON values; there is
no reserved-key exception for user payloads. Rejecting or rewriting such keys
inside otherwise valid opaque JSON would also violate that contract.

## Bounded developer action

Have the sole developer reproduce this through the actual T002 decode boundary:
one numeric scoring/observation position must reject the literal marker object,
while an opaque payload containing that object must remain an object with its
exact key and string value, including when nested. Keep the big-integer and null
regressions. If the actual path is immune, retain the concrete evidence and make
no correction for this concern.

If reproduced, authorize the smallest reliable correction to the wire/opaque
representation within `src/validation/wire.rs`, `src/validation.rs`, and affected
checked opaque fields/tests in `src/model/common.rs`. Required Cargo feature/lock
updates are permitted. Prefer the established parser's raw-value facility or
another bounded standard representation over implementing a new JSON parser.
Do not preserve a Value representation if it silently changes a valid payload.
Using a raw-value representation to retain correct JSON value kinds and numeric
values is allowed here; the earlier instruction prohibited adding it solely to
enforce unnecessary lexical number spelling. Those are different requirements.

Keep strict duplicate rejection, number-vs-object separation, all defined wire
shapes, exact bytes, opaque null/big integers, and source configuration object
requirements. No reserved-key blacklist, fallback mode, alternative permissive
entry, public SDK, custom serialization framework, or broader dependency audit.
If the correction requires scope beyond these existing boundaries, return the
smallest proposed decision before expanding.

Integrate this with the current correction and final T004–T006 candidate; do not
create another standalone acceptance committee. Retain direct before/after
evidence in the saved handoff and ask the independent verifier to check these
specific positive/negative cases alongside the null distinction. Reuse all
unchanged evidence. This is an owner-authorized reassessment of a concrete missed
wire invariant, not a reason to rerun unrelated audits or delay other sound work.

Save/index/link the response and subsequent worker instruction in the existing
Fizzy record. Preserve previous dispatch/acceptance history and continue through
the combined review. No user reapproval is required.
