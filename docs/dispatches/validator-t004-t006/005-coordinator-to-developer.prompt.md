# Validator parser-marker follow-up 005

Role/model: existing developer, `gpt-5.6-terra`, reasoning `high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required response remains:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t004-t006/003-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator’s verified single-label backbone](http://localhost:3006/1/cards/183).

This numbered follow-up extends the active prompt 003 with one concrete parser-
marker collision. Read `004-owner-to-coordinator.prompt.md` and
`004-coordinator-to-owner.response.md` in full. Integrate the work and evidence
into the same response required by prompt 003. Do not delegate or spawn agents.

## Exact reproduction

Through the actual strict T002 decode boundary, test the literal JSON object:

```json
{"$serde_json::private::Number":"0.5"}
```

Prove both sides of the contract:

1. In at least one numeric scoring or observation position, the literal object
   must reject with `E_SCHEMA`; it must not become an accepted number.
2. In opaque golden `input` and opaque source or preparation configuration, the
   literal object must remain an object with the exact key and string value,
   including at a nested position; it must not become a number or be rejected.

Retain direct before/after evidence. Keep the existing big-integer, opaque-null,
duplicate-key, number-versus-object, strict-shape, and exact-byte regressions.

If the actual path already satisfies both conditions, add the focused regression
and avoid an unnecessary production change. If reproduced, make the smallest
reliable correction. In addition to prompt 003's write scope, the owner authorizes
only these affected paths:

- `src/validation/wire.rs` and `src/validation.rs`;
- `src/model/common.rs` only if the checked opaque representation must change;
- `Cargo.toml` and `Cargo.lock` only for a required serde_json raw-value feature or
  equivalent bounded established facility;
- the already required combined response.

Prefer serde_json's established raw-value facility or another bounded standard
representation. Do not preserve a `serde_json::Value` representation that silently
changes a valid opaque payload. Raw-value use here protects actual value kinds and
payload correctness; do not extend it into a lexical-number-spelling contract.

Do not implement a reserved-key blacklist, new JSON parser, custom serialization
framework, fallback, alternate permissive entry, dependency audit, public SDK, or
T007-or-later behavior. If the reliable fix exceeds this scope, stop and return the
smallest decision needed.

Add a nonzero focused filter named `json_number_marker_collision` (or record the
exact equally specific owning name). Run it plus every command required by prompt
003 after integrating both corrections. Report exact changed hashes, feature/lock
rationale if any, the before/after reproduction, preservation of opaque objects,
and the complete warning classification in the single response. Do not claim the
combined candidate accepted.

