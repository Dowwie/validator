# Addendum: bind R1 to the missing resolved inspection source

Role/model: same independent verifier from verdicts010/013, `gpt-5.6-sol`, high
reasoning, original `fork_turns: none` context. Do not delegate. Read owner
prompt014 in full before evaluating this addendum.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #188 — Verify Validator artifact integrity](http://localhost:3006/1/cards/188).

Preserve Ready response013 unchanged. Save this focused addendum at:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t026/015-verifier-to-coordinator.response.md`.

Return only its path, SHA-256 and verdict. Do not edit repository files or Fizzy.
This is one read-only finding clarification, not a new general review.

Inspect the frozen manifest012 versions of `src/app.rs`,
`schemas/v2/inspection.schema.json` and the repaired S15 assertions. Bind them to
the original R1 requirement that both full origins be available through public
inspection and comparison. Determine whether current inspection exposes only the
prediction's local `source_id` plus evaluation configuration, while the selected
source's resolved `SourceDefinition` (model, source configuration, observation
definitions, preparation and verified evidence bindings where present) remains
absent from the inspection result and strict schema.

Preserve the other five repaired conclusions and the comparison-side source
definition evidence. Do not rerun their filters, the full suite or broad schema
review. Return **Revise** if the current public inspection result/schema cannot
prove the required full selected origin, identifying exact locations,
expected/actual surface and the smallest required source-field/schema correction.
Return **Ready** only if the current response already exposes that resolved
definition through the inspected episode itself. Stop when this exact question is
settled.
