# Repair the demonstrated abstention evidence boundary, preserve the oracle

Existing Sol-high coordinator; repository `/Users/dowwie/MyProjects/validator`.
Save acknowledgment to
`docs/dispatches/validator-t025/004-coordinator-to-owner.response.md`.

Owner read complete oracle response002 and escalation003. Their exact identities
are `43c0dfa849fce1ff3a4625135e1ea0f8b9306628860c75b255203ee4942fb925`
and `1c945b2cddfae942f329c46833c9fb4eb6052359f94c615dad670217ce997255`.
The expected field/status spelling is literal in validator-v1.md:669-673. This
is a product serialization/schema defect, not an oracle failure or discretionary
wire choice. The evaluator already retains the correct internal evidence.

## Staffing and independence

Preserve the paused oracle writer's context and all current expected fixtures and
assertions. Provision one fresh Terra-high/fork-none production-repair developer
as the sole writer for this bounded correction; it must not delegate. The oracle
author remains inactive and must not read production scoring for this repair.
The repair developer reads the Rust skill and the owning model serialization,
inspection assembly, schema definitions and relevant tests before editing. Do not
inspect or rewrite unrelated scoring/counting logic.

## Required correction

1. At the report serialization boundary in `src/model/multi_label.rs`, preserve
   multi-label abstention evidence in both raw and final outcomes: existing type
   and reason, literal `status: "abstained"`, and explicitly present JSON-null
   `matched`, `missed`, and `extra`. Do not convert it to an answered empty set or
   change any counts, scores, probability family, observations or policy. Preserve
   answered outcome and single-label wire shapes.
2. In `schemas/v2/report.schema.json`, require those exact multi-label abstention
   fields/status/nulls in a closed task-specific branch. Do not widen the shared
   bare single-label/submitted-outcome shape. `inspect.final_outcome` directly
   exposes this evidence, so synchronize its multi-label result branch in
   `schemas/v2/inspection.schema.json`. Submitted prediction outcomes remain the
   original input shape. No comparison schema change unless a direct shared
   serialization consequence is demonstrated; report any such dependency first.
3. Retain the frozen exhaustive assertion. Add focused real report and verified
   inspect evidence proving the fields are present with null values (check field
   presence explicitly; indexing a missing JSON key also returns null). Include
   with/without abstention reason and preserve probability/observation evidence
   where present. Schema negatives remove each required field, change status,
   and replace null differences with arrays. The old bare multi-label report form
   must reject, while valid single-label bare abstention remains supported.

## One additional bounded schema reproduction

Owner inspection found `report.schema.json`'s matrix label-column definition
explicitly excludes the literal string `ABSTAIN`. Validator-v1.md:167 and:831
require a declared `ABSTAIN` class to remain an ordinary class, distinct from the
typed abstention column. Reproduce this exact concern using a real single-label
evaluation whose vocabulary includes `ABSTAIN`, and validate its report. If the
schema rejects it, remove that name-specific prohibition and enforce the presence
of the genuinely typed abstention column structurally. Preserve the existing
negative that replaces the typed column with an ordinary label column: it must
still fail because the typed column is missing, not because a legal class name is
banned. Do not invent a sentinel label, alter vocabulary/order/scoring, or add a
general schema framework. This is a current hard-accounting/schema contract issue.

## Scope, checks and resumption

Write scope is `src/model/multi_label.rs`, the report/inspection schemas, and
focused conformance/CLI assertions. Existing expected fixture values and both
independent oracle bodies remain unchanged. The literal-ABSTAIN reproduction may
add a focused case using current helpers; it does not authorize a wider schema
audit. App/evaluator changes are unnecessary based on the observed boundary; if
an actual required dependency appears, report it before expansion.

Run both exhaustive filters, the affected multi-label report/inspection and schema
tests, the literal-class positive/typed-column negative, then fmt, full locked
tests, unmodified Clippy, release and diff checks. Preserve the exact staged
inventory without suppression, fake use or public expansion. Reuse existing
targets; no fresh full build tree. Save a complete handoff with reproduced
failures, precise corrections, unchanged oracle/fixture evidence, commands/results
and hashes. This correction is not independent acceptance.

After reading the complete repair handoff, resume the original oracle developer
on its saved independent work: rerun and finish output1 without reading production
scoring. Continue owner001's remaining local numerical outputs automatically.
Include this correction and its original failing oracle in the final independent
T025 review. That review and its test/fixture repair allowance have not begun.
Any new material discrepancy or incomplete repair returns with the exact cause
and smallest next step; do not reset the history or weaken expectations.

Update relevant task/physical/input mappings for this necessary correction,
index/link all dispatches/responses, resume Card187 to Working On while the repair
writer is active, verify rendered notes/first incomplete step, and preserve the
existing caffeinate hold. T026+ remain outside the active assignment.
