# Validator build charter

Date: 2026-09-18. Authority: Governing execution decisions within the user's authorization.

## Authorized outcome

The user explicitly instructed the owner to use the manage-dev-team skill to build
Validator, supervise the coordinator periodically, and prevent unnecessary work,
particularly verifier drift. This authorizes the reconciled `1.2-draft` main spec,
shared model, and 35-task delivery plan as the implementation baseline. Their
technical requirements and wire version 2 are unchanged. Earlier plan-only limits
are superseded; publication or task execution still does not establish completion.

Follow [the delivery plan](../../plans/build-validator.md),
[task contracts](../../plans/validator/tasks/T001.json),
[execution contract](../../plans/validator/execution-contract.md), and
[manage-dev-team skill](/Users/dowwie/.agents/skills/manage-dev-team/SKILL.md).
The skill controls roles; specifications control product behavior. This charter
resolves role conflicts in task JSON: tasks marked independent-verifier still
assign any implementation, permanent tests, scripts, or fixture changes to the sole
developer. The verifier reads, runs bounded diagnostics, and writes findings/evidence
only. Independent expected values must remain independent of production scoring.

## Roles and supervision

- Owner: Astra high. Own scope, architecture, disputed requirements, acceptance,
  integration authorization and communication with the user. Never second code writer.
- Coordinator: Sol high. Own schedule, task dispatches, evidence freeze, Fizzy,
  handoffs and drift detection. Never author implementation/tests/verification logic.
- Developer: Terra high. Sole writer of implementation, tests and related deliverables.
- Verifier: Sol high. Independently verify exact frozen candidate and return
  Ready/Revise/Blocked. Never edit implementation or acceptance fixtures.

The owner authorizes the coordinator to provision and reuse one developer and one
verifier, retaining four total roles. No workers spawn agents. Do not substitute
models/reasoning silently. Later fresh-agent acceptance may require a replacement
verifier with no inherited implementation context; the owner will authorize that
replacement without exceeding the four-role limit.

The coordinator sends an owner check-in at each coherent milestone, before/after
verification, after approximately 15 minutes without a handoff, and immediately
for scope or contract concerns. Include current task, concrete artifact progress,
active command/worker handles, uncertainty, and smallest next action. The owner
checks active work periodically (target approximately five minutes while running),
reads saved results, and promptly resolves escalations. Routine status inquiries
need no new task instructions; changed scope or instructions require numbered
file-based dispatches under repository policy.

## Bounded delivery

First dispatch boundary: T001–T017, the single-label steel thread. The coordinator
selects dependency-ready tasks and dispatches their full exact contracts, not an
entire feature phase described only by its title. One developer may progress through
a explicitly listed coherent sequence, recording each task's evidence, when the
owner dispatch allows it; do not invent implementation dependencies or extra scope.

First coherent checkpoints are: checked admission/source boundaries; pure single-label
scoring plus check/evaluate publication; replay/inspect/compare and T017. Run compile
checks at the first real path. Local tests accompany code. T014 and T017 run the
exact warning-denied Clippy command and disclose its real result; only the unchanged
private `dead_code` inventory explicitly accepted by the owner may remain staged
for its ratified T015-T021 consumers. Format, complete locked tests, and release
build must pass there. The full warning-free four-command gates are mandatory at
T027 and T035. No warning suppression, artificial consumer/export, stub,
unimplemented success path, tolerance relaxation, test weakening or fallback.

Within the first tranche, owner-approved coherent sequences preserve exact task
contracts while avoiding artificial review boundaries before required callers
exist. After T002 acceptance, T004 -> T005 -> T006 forms one ordered admission
candidate. After that candidate is accepted, T007 -> T008 -> T009 -> T010 -> T011
-> T012 -> T013 -> T014 forms one ordered scoring/publication/application candidate.
Each prerequisite is implemented and locally checked before its consumer; every
named filtered test must exist and run before the combined handoff. One independent
review and owner decision cover each combined candidate with per-task evidence.
No constituent task is separately accepted inside a sequence, and neither sequence
authorizes later behavior.

The T007-T014 grouping resolves a real construction dependency: conformance tests
owned by T008/T009/T013 must exercise the real small application API introduced by
T014, rather than expose private internals, include production source, or use a
test-only facade. Its boundary must finish with clean format, all locked tests, and
locked release build. It must also run warning-denied Clippy and reconcile the exact
result; the owner-enumerated private contracts awaiting T015-T021 consumers may
remain as staged `dead_code` only. No other warning class is accepted. The staged
inventory is rechecked at T017 and must be fully resolved by T027/T035.

Developer handoffs inside T007-T014 use one atomic task at a time in dependency
order. After each complete local handoff, the coordinator dispatches the next task
without a separate verifier turn or owner decision. Each handoff is only a local
implementation milestone; T007-T014 remains one candidate, frozen after T014 for
the single combined independent review and owner acceptance with per-task evidence.

Owner acceptance of the repaired T007-T014 candidate fixes manifest036 as the
starting point for T015 -> T016 -> T017. Those tasks proceed as atomic replay/
inspection, comparison, and fixed steel-thread units with automatic local advance,
then freeze once for one combined independent architecture-gate review and owner
decision. T016 may minimally reuse `artifacts.rs` publication internals for
`comparison.json`; it must not duplicate or generalize the safe publisher. T018
does not begin before owner acceptance of T017.

The coordinator may manage one criterion-bound repair/recheck cycle per frozen
checkpoint. An unresolved repeat returns to the owner with cause and smallest
recommended correction. This is an owner reassessment, not an automatic request
to the user to reauthorize already approved work.

Verifier scope is the current task/checkpoint and material regressions. Every Revise
finding cites a specification/plan criterion, exact location, reproducible evidence
and consequence. Optional elegance, speculative hardening, repeated attestations,
process frameworks, unrelated audits and verifier-of-verifier loops do not block.
Reuse unchanged valid evidence; do not rerun full expensive suites by habit. A lack
of evidence is Blocked, not an invented defect. Preserve mandatory privacy/integrity
floors and all actual acceptance cases. The 433 source blocks are routing context,
not 433 new features or separate essay/attestation gates.

## Workspace and external effects

Necessary parent-module declarations/re-exports and manifest/lock updates are
within a task's integration scope when required to compile its named behavior.
The coordinator may route those ordinary wiring changes, recording their hashes
and smallest purpose. This does not authorize new features, an expanded public SDK,
placeholder uses, or suppression. T002 explicitly includes its library declaration
and serialization features; T003 uses an established UUID primitive. The complete
Prediction record belongs to T004 with ObservationSet, eliminating an accidental
T003-to-T004 forward type dependency without changing the final model contract.

The repository is on an unborn main branch and all initial content is untracked.
Preserve it. Use exact file-hash baseline and frozen candidate manifests; no blanket
staging, cleanup of pre-existing files, or new version-control requirement is needed.
Do not rely on a live changing working tree as the reviewed candidate. Serialize
writes and checks that compete for build output. The coordinator owns index/status
updates and schedules developer documentation writes to avoid conflicts.

At orientation the host had about 17 GiB free. No disk cleanup outside task-owned
outputs is authorized. Reassess before a multi-gigabyte download or unusual build
output; normal locked Rust dependencies/builds and specified verification are in
scope. No model inference, external publication/deployment, service creation, secret
logging or modification/execution of Chord operational artifacts. Read/freeze only
specified acceptance evidence when that branch is dispatched.

Do not add an execution engine, tracker, schema generator framework, or broad audit
service. Use existing plan/tracker/tools. Snapshot metadata and concise requirement
proof are sufficient; mechanical plan/index checks supplement actual behavior.

## Handoff and acceptance

All dispatches/follow-ups use numbered full prompt files and saved response files,
including coordinator-to-workers. Each response binds scope, source hashes,
changed files, commands/exits, expected/actual behavior, failures and limits. Index
and link artifacts in Fizzy. Owner reads the coordinator's complete saved handoff
and the verifier's substantive verdict before acceptance, without redoing routine
verification. Integration/advancement beyond T017 requires owner acceptance.

The owner remains responsible through the full T035 definition of done. After an
accepted checkpoint, dispatch the next necessary bounded tranche under the user's
existing build authority; do not stop at a progress report or declare the entire
utility complete on an intermediate gate. Task state and active next actions stay
in Fizzy; this charter stores execution decisions only.

## Accepted T017 gate and T018-T020 sequence

The owner accepted the T017 architecture gate at manifest013
`46dac241d71c838ce1251ae9ed7914798c84009eeb937a7577f5c16da735c5c3`
and independent Ready verdict014
`12019ccc02925b73697686257587c7ea99abdf847c72cc44068056bd90e40a82`.
This accepts the single-label architecture only; the 24 staged Clippy diagnostics
and every later clean, multi-label and practical-acceptance gate remain open.

T018 -> T019 -> T020 is the next atomic implementation sequence, followed by one
combined independent review and owner decision. Use a fresh Terra-high/fork-none
sole developer for each substantial task and preserve one-writer serialization.
T018 owns checked multi-label admission and owning-library evidence. T019 owns
concrete hard accounting, independent expected results and owning-module tests.
Its two named conformance filters become real only when T020 integrates the actual
application path; T019 must never claim zero-selected or facade evidence. T020
owns marginal metrics/bins, closed app/replay/CLI dispatch, the real T019/T020
conformance/process filters and required check/report/inspection schema variants.
T021 and later work remain prohibited until the owner accepts the T020 checkpoint.
