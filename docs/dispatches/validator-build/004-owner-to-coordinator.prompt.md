# Preserve application test boundaries in the next coherent sequence

Role/model: existing Sol-high coordinator, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-build/004-coordinator-to-owner.response.md`.
Fizzy: [Card 183](http://localhost:3006/1/cards/183).

## Decision and reason

This changes future checkpoint granularity, not the active T002 repair or the
authorized T004–T006 admission sequence. Both still require owner acceptance.

Owner inspection found a construction dependency in the plan: T008/T009/T013
require `tests/conformance.rs` cases, but the small application API arrives in
T014. The shared-model specification explicitly requires integration tests to use
that API, and the main specification requires internal tests to remain beside
private implementation. Do not expose internals, include production source again
in integration tests, create test-only application facades, or invent placeholders
to make an early task look independently complete.

After owner acceptance of T004–T006, the next authorized coherent sequence is
**T007 -> T008 -> T009 -> T010 -> T011 -> T012 -> T013 -> T014**. The outcome is
real single-label `check` and `evaluate` through the specified small application
API and actual CLI, including typed reports, strict schemas, exact snapshots, and
atomic non-replacing publication. This supersedes the earlier instruction to keep
T007/T010/T011 as separate acceptance checkpoints, but changes none of their scope
or criteria. Do not dispatch this sequence before the admission acceptance.

## Execution boundaries

- Dispatch all eight exact task contracts and their normative references, owning
  files, dependencies, test names and expected outcomes. This is not authorization
  for an unspecified phase or any T015+ behavior.
- Implement and locally test prerequisites before consumers. Unit tests exercise
  private scoring and artifact invariants as those functions are built. Add the
  named integration tests through the real application API when T014 wires it.
  All original named filtered commands must execute nonzero passing test counts
  before this combined candidate can be handed off. No skipped or fake tests.
- Local checks permit progression inside this enumerated sequence. No constituent
  task is independently verified or owner-accepted until the combined frozen
  review and owner decision. Do not claim an incomplete integration case passed.
- Report concise intermediate evidence after pure scoring and after artifact
  publication, plus normal escalation for contract/resource issues. Do not create
  new review committees, repetitive attestations, or additional worker roles.
- Required small dependency/module wiring is already authorized. The absence of
  a schema dev dependency in T001 is not a reason to implement a schema engine;
  T010 may add the needed established development dependency explicitly/locked.
- Full clean fmt, Clippy with `-D warnings`, tests, and release build are mandatory
  at this T014 boundary. The interim unused-private-code limitation ends here;
  no suppressions, fake consumers or gratuitous public exports may conceal it.
- One sole implementation writer, frozen candidate, one independent read-only
  verification, one coordinator-managed repair/recheck allowance, and owner
  acceptance remain in force. The verifier covers all eight criteria sets and
  material regressions without unrelated audits or optional redesign.

T015–T017 remain the following architectural gate; neither this dispatch nor a
passing `check`/`evaluate` path establishes the complete steel thread or product.
The value of this grouping is resolving the actual API dependency and eliminating
repeated acceptance of private code before its intended caller exists, while
retaining every required behavioral and structural check.

## Durable handoff

Record this decision in the build charter, execution contract, and session notes
without creating a new task-state ledger. Index this prompt and your response,
link them in Fizzy, verify formatting/first incomplete step after any mutation,
and run the existing plan/index structural check. Preserve current T002 progress.
Save the concise response stating the understood boundary and current active
worker, then continue active coordination. No user reapproval is needed.
