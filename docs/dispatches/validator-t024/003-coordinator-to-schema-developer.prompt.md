# Preserve opaque-object and self-contained schema boundaries

Role/model/context remain the active T024 schema developer from prompt002. Keep
the same required response path:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t024/002-schema-developer-to-coordinator.response.md`.

Continue the current nine-schema and umbrella-test boundary without restart. This
amendment resolves two orientation points and changes no T024 product criterion:

- Preserve source/preparation `configuration` as an opaque **object**. The object
  itself remains required/typed/closed by its containing record as specified;
  arbitrary JSON is permitted in its member values. Positive evidence should put
  null, exact large integer, `1e400` and literal number-tag-shaped values inside
  the configuration object. Do not weaken it to accept a scalar/array/null root.
- Missing schema `$id` values are not independently a contract defect. The nine
  schemas are self-contained and use local `#/$defs` references. Prove offline
  loading/reference resolution with the pinned engine as required, but do not add
  named IDs, a metadata gate, registry or resolver framework unless an actual
  required reference-resolution case fails without one. No such failure is
  currently demonstrated.

Proceed only with demonstrated nested cardinality/required-field/closedness,
metric/status/task-alternative gaps and the two exact umbrella filters. Keep every
prompt002 constraint and handoff requirement. Save any actual production/spec
mismatch instead of weakening the schema or editing Rust.
