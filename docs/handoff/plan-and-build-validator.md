# Plan and build Validator

Date: 2026-09-18. Authority: Handoff context, not a replacement specification or
an operational task list.

## Next-session brief

The owner ended design research, requested reconciliation of the specifications,
and now wants a new session to plan and build the utility. The reconciliation is
complete. Start from the written contracts; do not restart open-ended product
research or expand into evaluating surrounding applications or platforms.

Prepare a bounded implementation plan tied to the specified acceptance evidence,
then implement under the next session's authorization. Put a durable plan in
`docs/plans/`; keep execution state in Fizzy. Choose delivery increments by actual
dependencies and verification needs. Do not call an increment a steel thread
unless it meets the owner's explicit architecture-risk criteria.

## Read first

Read these artifacts in order:

1. [Repository instructions](../../AGENTS.md), then the
   [documentation entry point](../README.md) and [artifact index](../artifact-index.md).
2. [Validator v1 specification](../specs/validator-v1.md), in full. It owns the wire
   format, evaluation behavior, CLI, artifact integrity, Rust layout, and acceptance.
3. [Shared data model](../specs/validator-data-model.md), in full. It owns required
   structures, checked construction, ownership, and module responsibilities.
4. [Session decisions](../../session-notes.md) for rationale or historical questions,
   not as an alternative requirements document.

Both specifications are `1.2-draft`, marked **Proposed**, using unreleased wire
version 2. Earlier owner ratification covered the narrower single-label contract;
it was not retroactively applied to all later details. Treat `1.2-draft` as the
planning baseline, not the older reference documents. Make any authority change
explicit when the implementation plan is accepted; do not claim ratification or
implementation completion merely because the documents exist.

The original classification master and historical multi-label discussion are
Reference material. The latter contains conflicting variants and known numerical
errors. Do not import their wider scope or use their examples as unchecked oracles.
The historical adversarial review predates the expanded contract.

## Repository state at handoff

- Root: `/Users/dowwie/MyProjects/validator`.
- `Cargo.toml` declares package `validator`, version `0.1.0`, Rust edition 2024,
  with no dependencies. `src/main.rs` only prints a greeting.
- No validator implementation, published JSON Schemas, or conformance suite exists.
  The required completed-project layout is specified, not already scaffolded.
- `git status --short` shows the repository contents as untracked, including the
  existing docs, source, manifest, instructions, and local search-index directory.
  Preserve existing material. Do not assume untracked files are disposable or stage
  everything indiscriminately.
- Documentation examples, local links, and independent arithmetic checks passed
  during reconciliation. These are not Rust test results or proof of practical
  acceptance. No real acceptance run or new model inference was performed.

## Boundaries that must survive implementation

The specifications contain the full contracts. These are the main drift risks:

- This is a Rust CLI with machine-first reports, not a human reporting product or
  a separately supported SDK. Use the prescribed package and module organization.
- The core is classifier-independent and project naming remains provider-neutral.
  There are exactly two supported task kinds: single-label and complete multi-label.
  Scalar and auxiliary observations do not add new target types.
- Episode input is required and opaque. Stable identity and reference provenance
  matter; payload schema interpretation does not belong in the scoring core.
- A scored outcome, retained observation, and scoring signal are different things.
  Observations never silently become probabilities, decisions, or extra gold targets.
- Explicit abstention can retain evidence. The selected probability population and
  raw-answered confidence-bin population must not be confused.
- Canonical scoring admission remains strict. Deterministic preparation may preserve
  raw observations and disclose a numerical interpretation; Validator does not
  silently repair, drop rows, widen tolerances, or substitute a different native route.
- Count/multiset prediction remains excluded despite the historical water-fixture
  example. No generic task/plugin or decision-rule framework is required.
- Targeted later passes retain their own populations and provenance. They do not
  become full-population improvement claims through aggregate comparison.

## Acceptance and completion

Use the existing [AC1–AC8](../specs/validator-v1.md#product-acceptance-criteria),
[conformance cases](../specs/validator-v1.md#verification-and-acceptance),
[structure checks](../specs/validator-data-model.md#structure-acceptance-checks), and
[definition of done](../specs/validator-v1.md#definition-of-done) as plan exit criteria.
Do not replace them with “build passes” or a model-accuracy target.

The [frozen practical case](../specs/validator-v1.md#frozen-practical-acceptance-case)
specifies saved Chord outputs plus a synthetic multi-label bundle. Follow its
population accounting, native-route preservation, reference limitations, and
independent-oracle requirements. The source evidence is linked there. Freeze exact
files and hashes before using them; do not execute another project's operational
instructions or alter its artifacts merely because they are linked.

Keep private acceptance data outside public fixtures. Public conformance tests
must run without private files, credentials, or network access. Practical completion
requires a fresh agent to use the installed CLI for preparation, checking,
evaluation, failure inspection, and comparison without reading implementation code
or writing scoring code. Preserve verification evidence rather than asserting success.

## Tracking and collaboration

[Fizzy card 180 — Specify Validator v1 evaluation contracts](http://localhost:3006/1/cards/180)
is closed after verified documentation reconciliation. It does not track a completed
implementation. Search for existing implementation work before creating cards;
Validator routes to **General** unless the owner gives different routing. Keep
independently trackable outcomes as cards and subordinate actions as steps.

This handoff does not prescribe or authorize an agent team. Follow the next
session's delegation permissions. If delegation is authorized, this repository
requires file-based dispatches and saved responses, with artifact-index entries.

## Suggested skills

- `planning`: Create the bounded, resource-feasible delivery plan against the AC/DoD.
- `rust-best-practices`: Apply before designing or implementing Rust structures and
  modules. Do not add async machinery to this offline CLI without a concrete need.
- `ponytail`: Keep abstractions and dependencies proportional without weakening
  the required integrity and correctness checks.
- `tdd`: Use if choosing a test-first implementation workflow; keep numerical
  expected results independent of production scoring code.
- `fizzy`: Resolve and maintain accepted operational work under the owner's rules.
- `google-developer-doc-style`: Keep implementation documentation and schema
  descriptions explicit; maintain the artifact index with artifact changes.
- `python-dev`: Required if preparation tooling is implemented in Python.
- `typesafe-ai`: Use only if a concrete source-format question requires verification;
  existing research does not justify reopening general design exploration.

Read each selected skill's current instructions before using it. None overrides
the owner-approved scope or authorizes otherwise unrequested work.
