# Correct the concrete per-label hard-metric population unit

Existing Sol-high coordinator; repository `/Users/dowwie/MyProjects/validator`.
Save acknowledgment to `docs/dispatches/validator-t025/016-coordinator-to-owner.response.md`.

Owner read full repair response011 and focused verifier prompt015. There is a
concrete remaining contradiction inside the repaired population-metadata finding:
M02 now asserts `episode` for per-label F1, and the passing handoff confirms the
product emits that unit. This is not the ratified unit. Notify the active verifier
before accepting its recheck; preserve the current frozen candidate until it saves
its bounded finding on this exact concern.

## Governing expectation and observed cause

`docs/specs/validator-v1.md` under Machine interface explicitly says that
multi-label set metrics use episodes and per-label metrics use label decisions.
Its Multi-label hard decisions section says per-label precision/recall/F1 use
the G answered records. Thus each per-label hard metric requires population
`G`, unit `label_decision`, scope `answered`, including the status-only cases.
Aggregate metrics retain G*K; set-level metrics retain episode units.

Owner inspected only the concrete implicated paths after reading the handoff:
`tests/conformance.rs:case_m02` hardcodes `episode` for per-label F1.
`src/evaluation/multi_label.rs:label_metrics` calls `answered_episode_metric` for
precision, recall and F1. The existing `answered_label_metric` already implements
the required status precedence and label-decision unit with an explicit population
count. This is a production metadata defect plus an incorrect new test expectation;
the ratios, counts, F1 values and independently frozen fixtures are unchanged.

Have verifier015 reconcile the literal requirement, current test and real public
M02 result, then save its focused verdict with this concrete remaining defect.
It may inspect the owning helper to explain the cause. Do not expand into a new
general audit. The valid S12/M14/undefined-value corrections remain accepted as
local evidence, subject to its recheck.

## Owner-authorized bounded correction after that saved finding

The original test-only repair exposed a real metadata bug that its author
incorrectly copied into an expected unit. The additional correction is worthwhile
because the machine contract expressly distinguishes these units, and it is a
small, known owning-layer change. This owner decision authorizes it without a new
user approval or resetting the prior repair history.

Resume the retained numeric developer as the sole writer, with a new file-based
dispatch and complete response path. Its expected population metadata is fixed
above before any production read. Read the Rust skill and owning helper/callers
before editing. Scope: `src/evaluation/multi_label.rs` per-label hard metric
construction and focused assertions in `tests/conformance.rs` / owning module
tests if needed. Reuse `answered_label_metric` with per-label population G; do not
introduce another helper or alter formulas, statuses, ratios, count accounting,
probability metrics, policy logic, schemas, fixtures, or other task behavior.

Correct the M02 unit expectation and verify all three per-label hard metrics in
both raw/final results for a normal answered case, an undefined empty-set case,
all-abstained input and empty selection. Assert count G, unit label_decision,
scope answered and the appropriate unchanged status/value. Retain aggregate G*K
and set-level episode assertions to prevent blanket unit replacement. Where the
existing tests suffice, extend those rather than adding duplicate workflows.
Check one real verified inspect or comparison replay path so recomputation uses
the same corrected metadata. Do not write expected numeric values from output.

Run affected exact filters, both exhaustive filters, the numerical umbrella,
relevant replay/comparison regression, and the four literal all-feature/pinned
format, Clippy, test and release commands from the specification plus diff check.
Only the same staged 17+3 Clippy diagnostics may remain. Save precise changed
hashes and unchanged fixture/schema hashes and the preserved original failure.

Freeze once and return to the same Sol verifier for a focused recheck of the unit
correction and any unresolved original findings, reusing all unaffected evidence.
One further concrete discrepancy returns to owner; do not generalize scope or
weaken expectations. Update T025's task/physical/T027 input mappings for the
necessary evaluator ownership and index/link every prompt, result and decision
in Card187. T026 remains undispatched pending owner acceptance.
