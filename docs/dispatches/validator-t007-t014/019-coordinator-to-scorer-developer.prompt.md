# Validator T010 final discriminating matrix completion 019

Role/model: retained sole developer `/root/coordinator/scorer_developer`,
`gpt-5.6-terra`, reasoning `high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/019-scorer-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator's verified single-label backbone](http://localhost:3006/1/cards/183).

Read owner reassessment
`docs/dispatches/validator-t007-t014/018-owner-to-coordinator.prompt.md` and
complete the final bounded T010 matrix. Do not begin T011 or delegate.

Start from escalation 017's stable hashes, including
`tests/conformance.rs` SHA-256
`e6eb3840af115859899873b8c7b382efd884bf1d173a3353debc8baa7800287e`.
Primary write remains `tests/conformance.rs`.

Use reusable valid golden/prediction/config bases. Each negative case must mutate
one relevant field or shape so its rejection proves the named rule. Complete only
these owner-authorized gaps:

- isolate the bundled extra-field, bounded-value, whitespace, and empty-threshold
  cases instead of combining independent defects;
- reject prediction outcome `type: "label_set"`;
- accept one complete legal `reject_below` config;
- accept a legal observation fixture containing scalar, bernoulli,
  reported-confidence, categorical, and label-marginal observations; keep scalar
  `1.33` legal and a retained categorical vector summing to `0.99` legal;
- add separate invalid cases for bounded scalar values, empty vector maps, wrong
  scalar/vector fields, and whitespace vector keys;
- exercise the shared nonblank definition and each distinct field wiring needed
  to establish task/source/model/description/question/preparation/evidence/
  observation/threshold rules, without an exhaustive cross-product of every tag;
- cover both categorical and label-marginal probability branches with separate
  legal and empty-map cases;
- preserve the existing legal multi-label prediction/threshold evidence, golden
  task-target cases, source profiles, all policy variants, explicit-null/version/
  type/unknown-field cases, and the absent-source runtime-boundary case.

The current predictions schema has an actual defect: its top-level `sources`
object does not constrain property names. Add the isolated whitespace-source-key
regression from an otherwise valid document, observe failure against the current
schema, then add the minimal `propertyNames` reference to the existing shared
nonblank definition. Record this red/green result and changed schema hash.

Do not add a second runtime-only boundary; the absent-source case is sufficient.
Do not require exhaustive duplicate coverage for shared references. Do not add a
dependency, framework, production change, new schema architecture, or unrelated
case. Preserve every valid existing assertion while restructuring confounded ones.

Run:

```sh
cargo fmt --all -- --check
cargo test --locked --test conformance input_schema_contract -- --nocapture
cargo test --locked --lib
git diff --check
```

Save the response before returning. Map each authorized case family to exact case
names/line locations/results, include the source-key red/green evidence, final
hashes, command exits/nonzero counts, and unresolved failures. Another incomplete
or unsupported claim returns to the owner without added coverage demands.
