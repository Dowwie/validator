# Complete only the T020 report schema

Role/model: existing replacement010 sole developer, `gpt-5.6-terra`, high
reasoning, retained context. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #184 — Build Validator’s concrete multi-label core](http://localhost:3006/1/cards/184).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t018-t020/013-t020-replacement-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat, return only
the response path, SHA-256 and terse status. Do not edit governance, plans,
dispatches, artifact-index, session notes, acceptance records or Fizzy.

Owner prompt012 supersedes the bundled completion instruction. Do not reconstruct
or repeat the whole T020 assignment. Your only output is the complete concrete
report schema and its focused conformance assertions. Preserve all production and
other schema files.

Read owner prompt012, task T020's report contract, `CLI and run artifacts` and
`Reports and metric reuse`, current typed single/multi serialized report builders,
current `schemas/v2/report.schema.json`, and the report-schema test helpers/
`single_report_schema` before editing. The current report schema SHA-256 is
`ec1e649325b7e621f16a471989b2a956fdfc1d6fab8a83263e2351b3ae904ecf`;
`tests/conformance.rs` is
`a18391bdfbf5a1f9aee9d6ae4a4bf1ee841bd154a76644a33192bd3f3914a07d`.
Stop and save the exact mismatch if either differs.

## Write scope

Own only:

- `schemas/v2/report.schema.json`;
- focused report-schema assertions in `tests/conformance.rs`.

Do not touch check/inspection schemas, CLI tests, production Rust, fixtures,
dependencies or other schemas. Do not run numerical-oracle, CLI or full release
gates. Do not depend on an unavailable earlier file; work from actual typed
serialization and the ratified contract.

## Required report contract

1. Define the complete shared report root and shared identity, artifacts/evidence
   bindings, sources/preparation, composition/source counts, population, policy and
   integrity structures with exact required fields and closed owned objects.
   Opaque configuration/observation payload values remain opaque only at their
   specified value positions.
2. Bind `task`, `raw`, `final`, `probability`, `signals` and every episode to one
   coherent concrete task family. The schema must reject mixed single-label and
   multi-label branches even when each fragment is independently well shaped.
3. Restore all accepted single-label hard structures: totals, class matrix with
   typed abstention column, per-class counts/support/coverage/metrics, aggregate
   exact/selective metrics and macro status. Require every metric's value/status,
   population count/unit/scope and ratio numerator/denominator as applicable.
4. Define complete multi-label hard totals, per-label TP/FP/FN/TN/support fields,
   per-label metrics, micro/macro/Hamming/set/selective metrics and raw/final
   episode evidence. Answered sets require concrete expected/predicted/matched/
   missed/extra arrays; abstentions require the specified nulls and status.
5. Define concrete categorical and marginal probability alternatives, legal
   unavailable/no-data/infinite/finite metric forms, and concrete single-label
   versus per-label multi-label signal/bin families. Require boundaries/counts,
   legal null means/rates and task-specific fields; reject arbitrary metric keys.
6. Close every owned nested object with `additionalProperties: false`; require
   exact fields, UUID/digest/path/count constraints and legal enums/null/status/
   special-value branches. Do not weaken accepted single-label validation to admit
   the new alternative.

## Focused evidence

Extend existing conformance schema tests with real valid single-label and
multi-label report documents plus focused invalid mutations that prove at least:

- deleting `raw.accuracy.population_unit` fails;
- other required nested metric fields fail when deleted;
- unexpected nested hard/probability/signal/episode fields fail;
- single/multi task-family fields cannot be mixed;
- invalid finite/null/infinite/status combinations fail;
- an answered multi-label episode cannot use abstention nulls and an abstained
  episode cannot use fabricated empty sets.

Run only the justified local evidence:

```text
python3 -m json.tool schemas/v2/report.schema.json
cargo test --locked --test conformance single_report_schema -- --nocapture
cargo test --locked --test conformance multi_label_hard_oracles -- --nocapture
cargo test --locked --test conformance marginal_loss_and_bins -- --nocapture
cargo fmt --all -- --check
git diff --check
```

Every filtered test must select one real test. Response013 must map each schema
definition to the actual serialized contract and named mutation, record exact
commands/exits/counts, and give starting/final hashes. It must confirm no other
file or behavior changed. Do not return a progress-only sketch; if incomplete,
record the actual stop condition and smallest unfinished schema item for owner
reassessment.
