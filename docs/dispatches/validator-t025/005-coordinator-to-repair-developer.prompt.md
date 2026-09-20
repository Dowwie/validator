# Repair literal multi-label abstention evidence and legal ABSTAIN class

Role/model: fresh sole production-repair developer, `gpt-5.6-terra`, high
reasoning, `fork_turns: none`. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #187 — Verify Validator numerical conformance](http://localhost:3006/1/cards/187).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t025/005-repair-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat return only the
path, SHA-256 and terse status. Do not edit governance, plans, artifact-index,
session notes, acceptance records or Fizzy. Do not delegate.

Read global/repository AGENTS, the Rust best-practices skill, complete owner
prompt004 and acknowledgment004, oracle response002 and escalation003, ratified
v1 lines 167, 669-673 and 831, then the owning multi-label serialization,
inspection assembly, report/inspection schemas and focused tests before editing.
Do not inspect or change unrelated scoring/counting logic.

Starting identities:

| Artifact | SHA-256 |
|---|---|
| `docs/dispatches/validator-t025/004-owner-to-coordinator.prompt.md` | `f1d8599e210e6d5f1ef3b9a02e53076ebc6f92b4a846df59843f0417e6fe8b3a` |
| `docs/dispatches/validator-t025/004-coordinator-to-owner.response.md` | `6e2731f5f4cb117f62f2a0cdac23c932feb5c109489387a6a4b0f3910e95fe4f` |
| `docs/dispatches/validator-t025/002-oracle-developer-to-coordinator.response.md` | `43c0dfa849fce1ff3a4625135e1ea0f8b9306628860c75b255203ee4942fb925` |
| `docs/dispatches/validator-t025/003-coordinator-to-owner.escalation.md` | `1c945b2cddfae942f329c46833c9fb4eb6052359f94c615dad670217ce997255` |
| `src/model/multi_label.rs` | `a0d6b55a7930b4acb1c9379d98b77d66eddc3386788eba3fdfb351cb28402c70` |
| `schemas/v2/report.schema.json` | `aa3589ea93b9a6a0e7c519898988087acb87f6854dc72db11bd974d99cbb0da3` |
| `schemas/v2/inspection.schema.json` | `039e6060b88042c908c27a34374592d9f64434aa1aac4725da13146b01c0ac82` |
| `tests/conformance.rs` | `0dff9c6b0fbe51f11cabb9e189b7d86dcf11490a5eae8ad33e0063bd75b8517a` |
| `tests/cli.rs` | `8c7b9c00d777f4a290911dd77199d8e1b9e085cc14fb0dd78a801108c3a10fcf` |
| `tests/fixtures/single-label/expected.json` | `1fc52d5a24d7900fbef992b0f19227089d83e38d0656b9809fc08f9ed0dcbfe0` |
| `tests/fixtures/multi-label/expected.json` | `213183a46c0f6c011296ef5087b5c44d3852081bdf20eaa80cbb494d3d2c84f2` |

Stop and save the exact mismatch before editing. Write only
`src/model/multi_label.rs`, report/inspection schemas, and focused additions to
`tests/conformance.rs` / `tests/cli.rs`. The two exhaustive oracle bodies and both
expected fixture files are read-only. No comparison schema or other production
change unless a direct shared serialization dependency is reproduced and reported
before expansion.

## Required abstention correction

- At public report serialization, both raw and final multi-label abstentions retain
  existing type/reason and add literal `status: "abstained"` plus explicitly
  present JSON-null `matched`, `missed`, and `extra`. Never emit an answered empty
  set and do not change counts, scores, probabilities, observations, policy or
  answered outcome shapes. Preserve single-label wire shapes.
- In the report schema, define a closed task-specific multi-label abstention branch
  requiring every exact field/status/null. Keep the shared/submitted and
  single-label bare abstention shapes unchanged.
- Synchronize only the inspection schema branch that directly exposes the
  multi-label final outcome. Submitted prediction outcome remains its original
  input shape.
- Add real report and verified-inspect evidence with and without an abstention
  reason, including a case with retained probability/observation evidence. Assert
  property presence explicitly (`contains_key`/equivalent), because indexing a
  missing key also yields JSON null.
- Add schema negatives deleting each required field, changing status, replacing
  each null difference with an array, and the old bare multi-label form. Prove a
  valid single-label bare abstention still passes.

## Literal ABSTAIN class reproduction

Run a real single-label evaluation whose ordered vocabulary includes the literal
class name `ABSTAIN`. If its report is rejected by the matrix label-column name
prohibition, remove only that name-specific prohibition. Keep the ordinary class
column and typed abstention column structurally distinct. Preserve/add the negative
where the typed abstention column is replaced by an ordinary label column; it must
fail because the typed column is missing, not because `ABSTAIN` is banned. Do not
invent a sentinel or alter vocabulary/order/scoring.

## Checks and handoff

Run both frozen exhaustive filters, focused report/inspection/schema/ABSTAIN cases,
then fmt, full locked tests, unmodified warning-denied Clippy, locked release and
diff checks using existing targets. Preserve exact 17+3 staged diagnostics; no
suppression/fake use/public expansion.

Response005 must record both reproduced pre-fix failures, exact corrections,
field-presence and schema mutation evidence, unchanged oracle/fixture hashes,
commands/exits/counts and every changed hash. This is a correction handoff, not
independent acceptance. A new material dependency or incomplete repair returns to
the coordinator with exact cause; do not broaden scope.
