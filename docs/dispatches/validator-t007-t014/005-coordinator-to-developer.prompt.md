# Validator T008 atomic developer dispatch 005

Role/model: existing replacement sole developer, `gpt-5.6-terra`, reasoning
`high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/005-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator's verified single-label backbone](http://localhost:3006/1/cards/183).

Complete **T008 only** as the next atomic implementation unit. Preserve the current
passing T007 slice and accepted T001-T006 baseline. This is a local implementation
milestone inside the still-combined T007-T014 candidate; it is not an independent
review or acceptance boundary. After a complete saved handoff, the coordinator
will dispatch T009 without an owner or verifier turn.

You remain the sole implementation/test writer. Do not delegate or spawn agents.

## Governing context and exact contract

Read in full before editing:

- `/Users/dowwie/.codex/AGENTS.md`, repository `AGENTS.md`,
  `/Users/dowwie/.agents/skills/manage-dev-team/SKILL.md`, and
  `/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md`.
- `docs/plans/validator/tasks/T008.json`, including every requirement ID, input
  artifact, interface, named check, and common completion check.
- `docs/plans/validator/execution-contract.md`, the physical/capability maps, and
  `docs/dev-team/validator-build/charter.md`, including the atomic T007-T014
  handoff rule and the single combined review boundary.
- `docs/specs/validator-v1.md` sections **Single-label hard decisions** and
  **Verification and acceptance**, plus all definitions those sections cite.
- `docs/specs/validator-data-model.md` section **Reports and metric reuse** and
  its relevant single-label checked-model definitions.
- Owner reassessment
  `docs/dispatches/validator-t007-t014/004-owner-to-coordinator.prompt.md` and
  coordinator acknowledgment
  `docs/dispatches/validator-t007-t014/004-coordinator-to-owner.response.md`.

T004-T006 remains owner-accepted at
`docs/dispatches/validator-t004-t006/024-coordinator-final-manifest.md`, SHA-256
`9e9b953930d1d8d334375f3a0ca4f0a22aed5f91eb8cb88cabb32cddc6c1107d`.
The retained T007 local inputs are:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `981783137668727726799ca99ee963f5333978cd6090ad9953b6d237a8e2c621` |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` |
| `src/lib.rs` | `bd89b6b7148e8ef25e64d28b465218a1f4afd90b8f95e6f92ed3a510de69aade` |
| `src/model/common.rs` | `5be10755552728c249ac2c809769c89585c80e6ce5dc35784aef95ddb913dd46` |
| `src/evaluation.rs` | `6fbab6a97da6d53c19d03897a09b3901d7ae6bf66a1a50c7df9ceebbc25f0fb8` |

Confirm these identities before editing and report any mismatch. The repository is
unborn/untracked. Preserve unrelated and accepted files. Do not stage, commit,
clean, reset, modify `.zvec-grep`, edit project governance/dispatch/index files,
or start/stop the task-owned caffeinate process.

## T008 outcome and behavior

Implement pure hard-decision accounting over a borrowed checked
`SingleLabelEvaluation`; the evaluator must never accept raw DTOs or JSON.

- Add typed `SingleLabelResults` and typed per-episode evidence.
- Produce both raw and final as-recorded K by K+1 matrices. The final matrix is
  equal in value to raw under the only admitted `as_recorded` policy, while the
  typed result retains both families for later policy work.
- Keep the abstention column structurally distinct from every vocabulary label,
  including a literal label named `ABSTAIN`.
- Report totals, class supports including abstentions, correct/wrong/abstained
  counts, accuracy, wrong rate, abstention rate, coverage, selective accuracy,
  selective risk, per-class precision/recall/F1, and macro-F1 with the exact T007
  status/population/unit/ratio semantics.
- Enforce every matrix/accounting identity with checked count arithmetic. A wrong
  A-to-B prediction contributes once, to row A and column B.
- Episode evidence contains episode ID, expected label, raw outcome, final
  outcome, correctness, source binding, and checked observations. It must omit
  opaque episode input and must be deterministically ordered as specified.
- The fixed F04 oracle is accuracy `5/8` and macro-F1 `131/210`. The fixed thread
  oracle is accuracy `1/2`, coverage `3/4`, and macro-F1 `1/2`.

Expected numbers must be authored independently from production scoring. Do not
generate or rewrite `expected.json` with the evaluator under test. Assert ratios,
statuses, units, populations, matrix cells, identities, ordering, and episode
evidence rather than checking only headline floats. Do not weaken tolerances or
delete a valid oracle to make tests pass.

## Allowed writes

Write only T008-owned artifacts and the smallest necessary parent/accessor wiring:

- `src/model/single_label.rs`;
- create `src/evaluation/single_label.rs`;
- `src/evaluation.rs` for the closed single-label dispatch/module wiring and reuse
  of checked T007 helpers;
- `src/model/common.rs` only for minimal read-only accessors needed to retain
  checked source/observation data in T008 evidence; do not alter admission
  invariants or public SDK surface;
- `tests/fixtures/single-label/expected.json` and other minimal T008-only input
  fixtures needed for independent local tests;
- `tests/conformance.rs` only if useful code can compile against the current real
  public application API. Because T014 does not exist, do not add placeholders,
  fake tests, private exports, source includes, or duplicate evaluators merely to
  create the two future conformance filters;
- the required response file.

Necessary module declarations within these listed parent files are allowed. Do not
edit dependencies, manifests, schemas, CLI/app/artifact modules, specifications,
plans, session notes, artifact index, Fizzy, T009+ behavior, multi-label behavior,
or private acceptance data. Do not add speculative abstractions or unrelated
cleanup.

## Local verification and pending final filters

Add real owning-module unit tests for every T008 criterion, including matrix
identities, literal-`ABSTAIN` separation, wrong-count-once behavior, empty and
abstention denominators, F04, thread values, and complete privacy-safe episode
evidence. Use clear T008-local test names; the two exact task conformance filters
remain pending until T014 exposes the real application API:

```text
single_matrix_identities
f04_asymmetric_oracle
```

Do not claim either exact filter now unless it genuinely executes through that real
API. Run the smallest relevant checks that establish this atomic unit, including:

```sh
cargo fmt --all -- --check
cargo check --locked
cargo test --locked --lib <each-new-T008-filter> -- --nocapture
cargo test --locked --lib
cargo test --locked --lib metric_status_precedence -- --nocapture
cargo test --locked --lib count_overflow_is_error -- --nocapture
cargo test --locked --lib macro_undefined_classes -- --nocapture
```

Do not rerun the full release/Clippy/integration gates by habit at this unit. T014
still owns the clean format, `cargo clippy --locked --all-targets --all-features -- -D warnings`,
all locked tests, and locked release build boundary. If current
partial integration still emits only expected unwired dead-code diagnostics,
record them exactly; do not suppress them, add fake uses, or expand visibility.

## Complete handoff

Continue until T008 production behavior, its independent fixture, owning tests,
compile, format, and regression checks are complete. Save the response before
returning. The response must include:

- exact input and changed-output SHA-256 hashes;
- a criterion-by-criterion implementation map with source/test locations;
- every command, exit code, nonzero test count, and expected/actual result;
- how the F04/thread fixture values were independently obtained;
- exact status of the two API-dependent conformance filters and why they remain
  pending for T014;
- any disclosed interim warnings or unresolved failures.

Return early only for a concrete reproduced blocker, with the exact command,
contract conflict, evidence, and smallest requested decision, after completing
independent work where possible. An incomplete atomic T008 handoff without such a
blocker returns to owner reassessment.
