# Correct multi-label per-label hard-metric population units

Role/model: retained sole numeric developer, `gpt-5.6-terra`, high reasoning,
original fresh `fork_turns: none` context. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #187 — Verify Validator numerical conformance](http://localhost:3006/1/cards/187).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t025/017-numeric-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat return only the
path, SHA-256 and terse status. Do not edit governance, plans, artifact-index,
session notes, acceptance records or Fizzy. Do not delegate.

This is the bounded owner reassessment authorized by prompt016 after the saved
verifier015 finding. It does not reset or erase the prior repair history. Read
global and repository AGENTS, the Rust best-practices skill at
`/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md`, owner prompt016,
complete verdict015 and the owning helper/callers before editing.

Write only `src/evaluation/multi_label.rs`, `tests/conformance.rs`, and an existing
owning module test section if genuinely required. Production expectation is fixed
before inspection: each multi-label per-label precision, recall and F1 metric has
population count `G`, unit `label_decision`, scope `answered`, including status-
only results. Aggregate binary metrics retain population `G*K`; multi-label set
metrics retain unit `episode`. Ratios, values, statuses, count accounting,
probability metrics, policies, schemas, fixtures and tolerances remain unchanged.

Starting identities:

| Artifact | SHA-256 |
|---|---|
| `015-verifier-to-coordinator.response.md` | `ce4293236799e897babf0e1c9edb7f8a432775f58ea00c0c1b533af5d37bfc72` |
| `016-owner-to-coordinator.prompt.md` | `0a129c1fc80602aebd2a133b6cfb945e708a9fcfb33511d43b27d19f9f35ce07` |
| `016-coordinator-to-owner.response.md` | `bd3deebd6a2151754ba5e40ec12369d9146654603c83b379b8a329b47623f1f7` |
| `src/evaluation/multi_label.rs` | `ff11dc9c2afbb5d1387051e245fd9532a90171a8a2c0086e1ea6b0fa1e8de8f3` |
| `tests/conformance.rs` | `40b9d231e3c77416d9c0f6bc93784bd309d45fa1cd6fe8a3eea6327f27ed3cd1` |
| `tests/fixtures/single-label/expected.json` | `ab93c379b9c0df1d0c0fb7aac7a6df5e7748f3f671d55dd69d173e640090b268` |
| `tests/fixtures/multi-label/expected.json` | `213183a46c0f6c011296ef5087b5c44d3852081bdf20eaa80cbb494d3d2c84f2` |
| `schemas/v2/report.schema.json` | `805de93d2a50339370e803b7ff1a9a1b8d34c3028b5b21a1a6f428f591733f90` |
| `schemas/v2/inspection.schema.json` | `2c7e064d0adb915b77c558a7ebbfade96f7bcf0eee890b881ced1383f0eedaef` |
| `docs/plans/validator/tasks/T025.json` | `7bc49b83a565c7abcbfcebd8e5336903820aec00e185783ad8e449aedceead71` |
| `docs/plans/validator/tasks/T027.json` | `a53eed042464e0e0ecb263747f53c2933f508701e10b679a9cc4f685c0a308b7` |
| `docs/plans/validator/physical-map.json` | `a15159a581d008a962df99501cc64a7bf708d2c3c6b66b7c92c944410a961563` |

## Smallest owning-layer correction

In `label_metrics`, replace the per-label precision/recall/F1 construction through
`answered_episode_metric` with the existing `answered_label_metric`. Pass per-label
population `G` as the label-decision population, preserving the existing answered
count and total so `no_data`, `no_answered_predictions`,
`undefined_zero_denominator`, values and ratio operands do not change. Do not add
a helper or change aggregate callers.

Correct the M02 independent expectation to `label_decision`; do not derive the new
unit from output. Extend existing public-path assertions, without duplicating
workflows, so all three per-label metrics (precision, recall, F1) are checked in
both raw and final results for:

- a normal answered case;
- the answered empty-set/undefined M02 case;
- every-row-abstained input (`G=0`, `no_answered_predictions`);
- empty selection (`N=G=0`, `no_data`).

For each, assert population count `G`, unit `label_decision`, scope `answered` and
the appropriate unchanged value/status/operands. Retain assertions that aggregate
binary metrics use `G*K` label decisions and set-level metrics use episodes so a
blanket unit change cannot pass.

Extend one existing real verified-inspection/replay or comparison regression to
show persisted/recomputed multi-label per-label metadata also uses the corrected
unit. Prefer the current `multi_label_abstentions_preserve_wire_evidence_in_report_and_inspection`
or `multi_label_comparison_transitions` path; do not add a parallel workflow.

If any expected numeric value, ratio, status or unrelated output changes, freeze
the exact mismatch and stop. Do not edit schema, fixture, tolerance, probability,
policy or comparison semantics.

## Required checks

Run the affected exact filters, both exhaustive filters, umbrella and chosen real
replay/inspection/comparison filter. Also run the literal specification gates:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
git diff --check
```

Clippy may exit 101 only for the unchanged accepted 17 production plus three
duplicate lib-test staged diagnostics. No suppression, fake use, visibility
expansion or cleanup-only work. Response017 must name every modified caller and
assertion, map each required scenario to its public path, list commands/exits/counts,
and record exact changed plus unchanged fixture/schema hashes. Return only when the
complete bounded correction is ready for freezing and the same verifier's focused
recheck, or a new concrete discrepancy is frozen.
