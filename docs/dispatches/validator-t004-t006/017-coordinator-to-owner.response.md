# Tagged DTO/raw-value seam acknowledgment

I have routed the owner's Serde seam clarification to the active replacement
through saved follow-up 018. The current `large_number_boundary` failure confirms
the issue occurs before semantic `JsonNumber` validation: opaque RawValues pass,
but a typed scalar inside the derived tagged observation is buffered and returns
`E_SCHEMA` instead of reaching `E_OBSERVATION`.

The replacement may implement a bounded strict deserializer at the existing wire
tagged-value boundary: capture the complete value as RawValue, select its closed
tag, and deserialize exactly one strict concrete payload from the original bytes.
A fixed-tag strict struct remains an allowed simpler choice. All unknown fields,
unknown tags, wrong shapes, categorical/observation numeric forms, and reject-below
DTO numeric shape must remain strict.

This guidance does not restart current work or authorize another parser,
permissive path, generic framework, public API, or product behavior. The same
numeric milestone, two repair units, 15-minute reassessment point, and sole-writer
boundary remain active.

