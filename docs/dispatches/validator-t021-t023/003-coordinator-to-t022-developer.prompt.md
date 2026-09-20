# Implement T022 typed multi-label comparison

Role/model: fresh sole implementation developer, `gpt-5.6-terra`, high
reasoning, `fork_turns: none`. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #185 — Build Validator policies and comparison core](http://localhost:3006/1/cards/185).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t021-t023/003-t022-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat return only the
path, SHA-256 and terse status. Do not edit governance, plans, artifact-index,
session notes, acceptance records or Fizzy. Do not delegate.

Read global/repository AGENTS, Rust best-practices, complete T022 contract, the
full comparison/spec report-reuse sections, owner prompt001, accepted T016/T020
comparison/replay handoffs and T021 response002. Read current typed single-label
comparison, multi-label results/evidence, app/CLI publisher and comparison schema
before editing. T023 intersection and T024 remain prohibited.

Starting hashes:

| Artifact | SHA-256 |
|---|---|
| T022 task | `28ecbba28f7d6c7c6a133a9018862abc0e96733b8da617b39aa8493a40ba9608` |
| owner prompt001 | `73f5ebe3a192a0baedf9787b55a356d49b03211fa093edfcabf47034cecc15b9` |
| T021 response002 | `33be455a22ae14c936121cf28c4e7de1ab0ef1c6fcd7daa20d4a4d75a5d71a6e` |
| `src/comparison.rs` | `a0288c9d79f116b03925802a52771dff0e4894506a23701b9b15e6cb36b02961` |
| `src/model/multi_label.rs` | `c019632dfef36ae441bc00cc3be1c7a58b539e9081107066d893b2993807f94f` |
| `src/app.rs` | `3324dad26b9571529bdfb1a75b2bb17a9900bbb8f1a65ad822e7b389b76c6671` |
| `src/cli.rs` | `ae1d7945252b98eb7b7f5c7312e67661485dddb317fffa4e26f00dd0dfb81695` |
| `src/artifacts.rs` | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |
| comparison schema | `24002dce44ffe08c9900c71a2db3a5c81e7b5b274bd05fd647c46468bf2c75d6` |
| `tests/conformance.rs` | `f80c6318e61882bc3b577756c9ed618451650282bb751206fbd0ccb77ba7b2e2` |
| `tests/cli.rs` | `edaf69f7744fbc2df630e280234041e8f9bd0608ed1c2bdd719187090ca85275` |

Stop and save an exact mismatch before editing.

## Required concrete comparison

Own only the ratified T022 artifacts plus necessary closed app/CLI process wiring.
Reuse verified stored-run replay/evidence, atomic comparison publication, common
identity/compatibility/source/preparation differences and checked arithmetic.
Preserve the accepted single-label compare path exactly.

- Compare typed recomputed `MultiLabelEvaluation` and concrete results, never
  serialized report JSON. Use explicit typed fields; no recursive metric discovery,
  string-key metric map, generic evaluator registry or universal accuracy type.
- Require existing compatibility: golden digest, concrete multi-label task,
  ordered vocabulary, role, selected IDs and numerical semantics. Allow/report
  policy/model/source differences under existing rules. Source IDs are local;
  compare definitions, observation definitions and preparations.
- Each episode has both local source IDs, outcomes, correctness, matched/missed/
  extra or abstention state. Final correctness categories sum to common N; a
  `{A}->{B}` change against `{A,B}` remains `neither_correct` and changed.
- For every label emit a complete absent/present/abstained 3x3 transition table
  summing to N. Whole abstention maps every label to abstained; never enumerate
  power-set classes.
- Keep raw/final paired metrics separate. Answered-only results expose baseline
  and candidate answered ID/count populations and overlap. Numeric deltas require
  both statuses defined; otherwise null with reason and correct direction. No
  winner/significance or paired-improvement claim.
- Wire existing public compare API/CLI through closed verified task dispatch and
  publish the concrete multi-label comparison via the existing safe publisher.
  Extend `comparison.schema.json` with a strict multi-label alternative; validate
  real output and preserve single-label schema/CLI behavior.

Implement and run exact nonzero real-path filters:

```text
cargo test --locked --test conformance multi_label_comparison_transitions -- --nocapture
cargo test --locked --test conformance answered_population_overlap -- --nocapture
```

Also add/use real binary multi-label compare evidence in the existing CLI harness,
validate receipt and exact published schema bytes, then run listing, focused
single-label comparison regressions, fmt, full locked tests, Clippy and diff check.
Only the authorized staged inventory may remain; real T022 consumers should reduce
it. No ordinary warning, suppression, fake use, export widening or T023 work.

Response003 must map criteria to concrete types/real assertions, include independent
expected transition tables/populations, commands/exits/counts, exact hashes and
residual lint inventory. A complete local handoff advances automatically to T023.
