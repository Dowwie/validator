# Validator T010 missing regression-matrix completion 016

Role/model: retained sole developer `/root/coordinator/scorer_developer`,
`gpt-5.6-terra`, reasoning `high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/016-scorer-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator's verified single-label backbone](http://localhost:3006/1/cards/183).

Complete the missing T010 regression matrix only. Response 013 is not accepted
because its claimed correction assertions are absent from the 74-line test file.
Do not redesign the schemas or begin T011. You remain the sole test/schema writer;
do not delegate.

Read owner decision
`docs/dispatches/validator-t007-t014/015-owner-to-coordinator.prompt.md`,
correction `013-coordinator-to-scorer-developer.prompt.md`, and escalation
`014-coordinator-to-owner.escalation.md`. Start from these frozen hashes:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `e257b5d70c78f0dd7465093aad8d092db99c35ffa28227298271512faf49bec6` |
| `Cargo.lock` | `86caa4cd0470cc4d1f8d4d8dedbdfccb1c43e42b9f0a5a119343e35b256dd74e` |
| `schemas/v2/golden.schema.json` | `97d811a192274f1ee0703e5dff475723301fdcc68c20b031d3b22431968b777b` |
| `schemas/v2/predictions.schema.json` | `ef4cf1ccfd80653afc63501fe98900fe897c9eea86e862ae72344e4b2bafab3a` |
| `schemas/v2/config.schema.json` | `c740c7cd2c1381398b3121ee161b1d995a23ceb9dd4325bd73cd0073f66d65c3` |
| `tests/conformance.rs` | `09ef6a87d7c944dca0a94fb566e6837f0b55acffeb0049e68a3b5031d12617ff` |

Primary and expected write: `tests/conformance.rs`. Keep the exact public filter
name `input_schema_contract` and use named table/helper cases so a failure states
which contract case failed. Add specification-derived assertions for all of these:

- accept exact `labels` targets/outcomes; reject `label_set`;
- reject single-label task + labels target and multi-label task + class target;
- accept legal classifier and scored-choice sources, and reject scored-choice
  without `question_id`;
- accept scalar observations outside `[0,1]`; reject out-of-range bernoulli and
  reported-confidence observations; accept legal categorical/label-marginal
  observations; reject empty maps, whitespace option names, wrong fields, and
  extra fields for each closed variant;
- reject whitespace-only task labels, source keys/IDs, model, descriptions,
  question IDs, preparation method/version, evidence paths, observation names/
  options, and threshold keys;
- accept legal nonempty categorical and label-marginal maps; reject empty maps;
- accept legal `as_recorded`, `reject_below`, and nonempty `label_thresholds`
  configurations; reject empty thresholds and malformed/extra policy fields;
- reject extra fields in golden top/episode/target and predictions top/source/
  prediction/outcome containers;
- accept representative full legal multi-label golden, prediction with a `labels`
  outcome and label marginals, and `label_thresholds` config directly through the
  schema engine;
- accept a named shape-valid prediction whose `source_id` is absent from `sources`,
  explicitly proving source binding is a runtime-only rule. Include at least one
  other named shape-valid semantic boundary such as incomplete vocabulary-key
  coverage or categorical sum.

Do not derive expected accept/reject outcomes by asking the schema first. Encode
the expected result from the spec, then assert the compiled schema's result.

Preserve all three schema files. If a required case fails and proves an actual
remaining schema defect, make only the smallest change in those existing files
and record the case's failing and passing evidence. No Cargo/dependency,
production source, fixture, documentation/index/Fizzy, or other test changes.

Run exactly the focused completion evidence:

```sh
cargo fmt --all -- --check
cargo test --locked --test conformance input_schema_contract -- --nocapture
cargo test --locked --lib
git diff --check
```

Save the response before returning. It must include final hashes, actual line
locations and case names for every family above, command exit/nonzero test counts,
any required schema red/green correction, and unresolved failures. A prose claim
without the named assertions is incomplete and returns directly to the owner.
