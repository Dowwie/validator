# Resolve the tagged-DTO/raw-value seam within the existing repair

Role/model: existing Sol-high coordinator, retained context.
Required response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t004-t006/017-coordinator-to-owner.response.md`.

The reported `invalid type: newtype struct, expected any valid JSON value`
failure is a concrete integration issue in the current numeric repair. RawValue
must be captured before a derived internally tagged/untagged enum buffers the
payload through Serde's generic content representation. Capturing only a nested
numeric field after that buffering can no longer recover the original JSON token.
Repeated changes to a numeric visitor after information is lost are not the route.

The owner explicitly permits a bounded custom Deserialize implementation at the
affected existing **wire tagged-value boundary**, if needed: capture that complete
value as RawValue directly from the JSON parser, identify its closed tag, then
deserialize the corresponding concrete strict payload from those original bytes.
A fixed-kind object can instead use a strict struct with a fixed tag type where
that is simpler. The final variant decoding must still reject every unknown or
incompatible field; tag selection alone is not successful validation.

This is an implementation of the same single T002 decode boundary, not a second
permissive route. It requires no public API, new parser, generic framework, schema
engine, fallback or product behavior. Preserve all closed variants and relevant
wire tests, including legal numeric fields on categorical probabilities,
observations and reject-below DTOs, without implementing reject-below semantics.
Keep opaque RawValues and the recursive duplicate pass intact.

Give this architectural clarification to the replacement in a saved follow-up;
leave exact local implementation choices to it. Continue the same bounded repair,
milestone and 15-minute reassessment point. If this is already how the worker is
resolving the issue, do not restart or duplicate its work. Index/link this prompt
and required response in the existing record.
