# Validator verifier orientation dispatch 002

Role/model: verifier, `gpt-5.6-sol`, reasoning `high`, context inheritance `none`.
Repository: `/Users/dowwie/MyProjects/validator`.
Coordinator: `/root/coordinator`, Sol high. Required response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-build/002-verifier-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator’s verified single-label backbone](http://localhost:3006/1/cards/183).

## Outcome

Orient as the independent read-only verifier for the authorized T001–T017 tranche.
Confirm that the verification capability, authoritative contracts, baseline, and
role boundaries are available before a candidate is frozen. Do not review or
approve T001 yet: no candidate exists. Do not edit implementation, tests, schemas,
fixtures, acceptance material, artifact index, Fizzy, specifications, or plans.
Do not delegate or spawn agents.

Read:

- `/Users/dowwie/.agents/skills/manage-dev-team/SKILL.md` in full, observing the
  verifier role.
- `/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md` and the chapters
  relevant to later Rust review: coding idioms, linting, error handling, testing,
  and documentation.
- Repository `AGENTS.md`, `docs/README.md`, `docs/artifact-index.md`.
- `docs/dev-team/validator-build/charter.md`.
- `docs/specs/validator-v1.md` and `docs/specs/validator-data-model.md` in full.
- `docs/plans/build-validator.md`, `docs/plans/validator/execution-contract.md`,
  and `docs/plans/validator/tasks/T001.json` through `T017.json` as the bounded
  review contracts. Do not turn later criteria into current T001 findings.
- The initial developer assignment at
  `docs/dispatches/validator-t001/001-coordinator-to-developer.prompt.md`.

## Baseline and restrictions

The repository is an unborn `main` worktree whose initial material is untracked.
At orientation, `Cargo.toml` is a bare edition-2024 package, `src/main.rs` prints
`Hello, world!`, and no lockfile exists. `cargo test --locked` therefore fails
before building. Compiler is `rustc 1.98.1 (48a229cea 2026-09-01)` for
`x86_64-apple-darwin`; Cargo is 1.98.1; disk free is about 18 GiB.

You are read-only except the required response file and later verifier evidence
files explicitly named in a numbered follow-up. You may run read-only inspection
and bounded diagnostics. Never change implementation or acceptance fixtures,
weaken criteria, invent optional redesign, create a process framework, or add
verification logic to the product. Findings later must cite requirement, source
location, reproduction, consequence, and smallest correction. Verdicts bind to
a frozen hash snapshot and are exactly Ready, Revise, or Blocked.

## Orientation response

Save a concise substantive response at the required path containing:

- Confirmation of role/model/reasoning and that no subagent was spawned.
- Authoritative sources read and review capabilities available.
- Exact baseline facts independently observed, including Git state, compiler,
  Cargo, disk, and the expected locked-test failure.
- Any material contract ambiguity, missing capability, or resource blocker that
  would prevent later T001 or T017 verification. Do not speculate or broaden scope.
- The smallest independent T001 review procedure you will apply once the
  coordinator supplies a frozen candidate: full scoped diff/hash check, named
  test count and execution, acceptance-criterion review, prohibited-shortcut
  scan, and risk-proportionate regression commands.
- Active commands/processes and the required resume condition.

Do not issue a candidate verdict in this orientation response. After saving the
complete response, return only its absolute path and remain available for the
numbered candidate-review follow-up.
