# Validator tagged DTO/raw-value seam follow-up 018

Continue the active replacement repair under prompts 014 and 016. Read
`017-owner-to-coordinator.prompt.md` and
`017-coordinator-to-owner.response.md` in full. Required final response remains
`014-developer-to-coordinator.response.md`.

The current `large_number_boundary` result localizes the typed `1e400` failure to
Serde content buffering before the nested `JsonNumber` sees original bytes.
Repeatedly changing the numeric visitor after that loss is not the route.

Within the existing single private wire boundary, you may add a bounded strict
custom `Deserialize` implementation at an affected tagged-value boundary when
needed: capture the complete value as `RawValue` directly from the JSON parser,
identify the closed tag, and deserialize the corresponding concrete strict payload
from those original bytes. A fixed-kind strict struct with a fixed tag type is also
allowed where simpler.

Tag selection alone is not validation. Preserve `deny_unknown_fields` behavior,
all closed observation and categorical-probability variants, every legal numeric
field, wrong-shape/unknown-tag rejection, reject-below DTO numeric decoding without
implementing its policy semantics, recursive duplicate rejection, and opaque
RawValues. Do not add a parser/framework, alternate path, fallback, permissive
mode, public API, or new product scope.

If you are already applying this seam, preserve progress. Do not restart completed
model work or tests. Continue until the numeric milestone passes, report it, then
finish unit 2 and the full response as originally required.

