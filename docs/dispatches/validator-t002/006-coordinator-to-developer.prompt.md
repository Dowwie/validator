# Validator T002 bounded repair dispatch 006

Role/model: existing developer, `gpt-5.6-terra`, reasoning `high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required full response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t002/006-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator’s verified single-label backbone](http://localhost:3006/1/cards/183).

This is the one authorized repair cycle for frozen T002. Read the complete verifier
Revise response at
`docs/dispatches/validator-t002/005-verifier-to-coordinator.response.md`. Fix only
its demonstrated legal-shape omission. Do not delegate or spawn agents.

## Required correction

In the private wire DTO layer:

- Add optional `observation_definitions` to `Source` as an exact string-keyed map.
  Each definition has exactly `kind`, `description`, and optional `question_id`.
  The closed kinds are `scalar`, `bernoulli`, `reported_confidence`, `categorical`,
  and `label_marginals`.
- Add optional `observations` to each `Prediction` as an exact string-keyed map.
  Add strict tagged private DTO variants for all five legal values:
  `scalar`/`bernoulli`/`reported_confidence` with a numeric `value`, and
  `categorical`/`label_marginals` with string-keyed numeric `values`.
- Use the existing deferred `JsonNumber` representation. Do not check nonblank
  names/descriptions/question IDs, numeric finiteness/ranges, vector nonemptiness,
  source-definition matching, scoring-vocabulary binding, sum rules, completeness,
  or observation/scoring promotion. T004 owns those semantic checks.
- Preserve `deny_unknown_fields`, explicit tags, recursive duplicate checking,
  privacy, original bytes, exact numeric values, and every existing strict wire
  behavior. Unknown observation kind/extra field/wrong primitive must fail safely.

Extend `wire_tags_and_fields` or add focused owning-module assertions so one legal
single-label prediction artifact containing definitions and row values for all five
observation kinds decodes, while an unknown kind and an extra field fail. Keep the
existing three T002 named tests and their prior coverage. Do not create fixtures,
public APIs, semantic model types, or alternate decode routes.

## Write scope and checks

Modify only `src/validation/wire.rs`, the owning tests in `src/validation.rs`, and
the required response. Do not change Cargo/lock/lib/model/error/main or docs/tracking.

Run and record:

```sh
cargo test --locked --lib wire_tags_and_fields -- --nocapture
cargo test --locked --lib strict_json_keys -- --nocapture
cargo test --locked --lib opaque_number_and_null -- --nocapture
cargo fmt --all -- --check
cargo test --all-features --locked
cargo build --locked --bin validator
cargo clippy --all-targets --all-features --locked -- -D warnings
```

Clippy remains expected to exit 101 only on incomplete-consumer `dead_code`; new
observation DTOs can add such groups. Record exact classes/count/locations and
confirm no unused import or other warning class. No suppression or fake caller.

## Handoff

Save the response with changed/current hashes, the complete five-kind DTO shape,
positive/negative regression evidence, commands/exits/counts, remaining Clippy
classification, reused unchanged evidence, limits/processes/resources. Do not
claim independent Ready. Return only the saved response path.
