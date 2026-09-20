# Numeric-marker collision coordination response

I accept the owner's concrete serde_json marker-collision concern and have added it
to the active pre-freeze correction through numbered developer follow-up 005. The
developer must reproduce the literal
`{"$serde_json::private::Number":"0.5"}` through the actual strict T002 decode
boundary in both a numeric position and opaque positions before choosing a fix.

The required outcome is exact: a numeric scoring or observation position rejects
the object with `E_SCHEMA`, while opaque input and configuration retain it as an
object with the exact key and string value, including when nested. Existing
big-integer, null, duplicate-key, strict-shape, and exact-byte evidence must remain
valid.

If the locked parser path is immune, the developer will retain the regression
evidence without a production change. If reproduced, the developer may make the
smallest reliable correction in `src/validation/wire.rs`, `src/validation.rs`, and
affected checked opaque fields/tests in `src/model/common.rs`, with a required
serde_json feature/lock update if needed. Raw-value use is authorized only to keep
actual JSON kinds and opaque payloads correct. The follow-up forbids a reserved-key
blacklist, custom parser, fallback, alternate permissive entry, broad dependency
audit, public API, or T007-or-later behavior.

This remains part of the same active correction and future combined T004-T006
review. I will freeze one superseding candidate after the developer returns both
the optional-null and marker-collision evidence. The independent verifier will
check those two bounded parser invariants alongside the unchanged admission
criteria; no standalone committee or unrelated audit is added.

