# Validator T008 fresh scorer-developer dispatch 008

Role/model: fresh sole developer, `gpt-5.6-terra`, reasoning `high`,
`fork_turns=none`.
Repository: `/Users/dowwie/MyProjects/validator`.
Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/008-scorer-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator's verified single-label backbone](http://localhost:3006/1/cards/183).

Complete **T008 only** from the reconciled partial source. You are the sole writer
of implementation, tests, and fixtures. Do not delegate or spawn agents. Keep the
existing accessors and matrix counter only if they help; do not rebuild accepted
admission or repeat passing T007 work.

## Required context

Read these files in full before editing:

- `/Users/dowwie/.codex/AGENTS.md` and repository `AGENTS.md`;
- `/Users/dowwie/.agents/skills/manage-dev-team/SKILL.md` and
  `/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md`;
- `docs/plans/validator/tasks/T008.json`;
- `docs/plans/validator/execution-contract.md` sections governing implementation,
  evidence, file dispatch, and the atomic T007-T014 sequence;
- `docs/specs/validator-v1.md` sections **Counts and metrics**, **Single-label hard
  decisions**, **Undefined values**, and the relevant single-label cases in
  **Verification and acceptance**;
- `docs/specs/validator-data-model.md` section **Reports and metric reuse**.

Do not read the full dispatch history or T009-T014 task contracts. The controlling
state summary is escalation
`docs/dispatches/validator-t007-t014/006-coordinator-to-owner.escalation.md`.
T004-T006 is accepted at manifest
`docs/dispatches/validator-t004-t006/024-coordinator-final-manifest.md`, SHA-256
`9e9b953930d1d8d334375f3a0ca4f0a22aed5f91eb8cb88cabb32cddc6c1107d`.

Confirm these exact inputs before editing:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `981783137668727726799ca99ee963f5333978cd6090ad9953b6d237a8e2c621` |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` |
| `src/lib.rs` | `bd89b6b7148e8ef25e64d28b465218a1f4afd90b8f95e6f92ed3a510de69aade` |
| `src/model/common.rs` | `5be10755552728c249ac2c809769c89585c80e6ce5dc35784aef95ddb913dd46` |
| `src/model/single_label.rs` | `63b808e20181c18b6e0b6ae1a035213c72020ee471fe13e10601543d685ad71d` |
| `src/evaluation.rs` | `9911006f932bb132bb4d93a54eceba8df606bd551489b998c3cab052fd09c2dd` |
| `src/evaluation/single_label.rs` | `297eb09bba7eb786bb0b1bf67eecc10f693a78fe3711244dd2c630c0fe8a3fb2` |

The repository is unborn/untracked. Preserve unrelated and accepted files. Do not
stage, commit, clean, reset, modify `.zvec-grep`, edit governance/dispatch/index/
Fizzy records, or start/stop task-owned caffeinate PID 84732.

## Exact outcome

Implement a complete pure scorer that borrows `SingleLabelEvaluation` and never
accepts raw DTOs/JSON.

- Typed results own **raw and final** K by K+1 matrices. Under `as_recorded` their
  values match, but both families remain explicit. Abstention is the typed last
  column and cannot collide with a literal vocabulary label `ABSTAIN`.
- For each family expose `N,D,E,U,G`, matrix, support, predicted support, TP, FP,
  FN, per-class precision/recall/F1/class coverage, accuracy, wrong-class rate,
  abstention rate, coverage, selective accuracy/risk, and macro-F1.
- Use checked integer arithmetic and enforce:
  `N=D+E+U=sum(matrix)=sum(support)`, `G=sum(predicted_support)`,
  `sum(TP)=D`, `sum(FP)=E`, and `sum(FN)=E+U`. A wrong A-to-B row increments one
  matrix cell and represents one wrong episode.
- Use T007's typed status/population/unit/ratio representation. Empty applicable
  populations are `no_data`; nonempty zero denominators are explicit; answered-
  only metrics with `G=0` are `no_answered_predictions`. Per-class precision may
  be undefined while direct F1 is defined zero for an observed always-missed
  class. Macro-F1 zero-fills only undefined class F1 terms on a nonempty population
  and reports the affected classes. Complete the minimal T007 helper/model behavior
  in `src/model/common.rs`/`src/evaluation.rs` if T008 requires it, preserving all
  existing T007 tests and avoiding unused public SDK surface.
- Typed episode evidence is canonically ordered and contains episode ID, expected
  label, raw outcome, final outcome, correctness, source binding, and checked
  observations. It must never contain opaque episode input or raw response data.
- F04 must yield accuracy `5/8` and macro-F1 `131/210`. The thread case actual
  `[A,A,B,C]`, final `[A,abstain,A,C]` must yield `D=2,E=1,U=1`, accuracy `1/2`,
  coverage `3/4`, selective accuracy `2/3`, class coverages `[1/2,1,1]`, and
  macro-F1 `1/2`.

Author `tests/fixtures/single-label/expected.json` independently from production
scoring. Record its direct count/rational derivation in the response. Never use the
evaluator under test to generate expected values. Use the fixed tolerance only for
finite comparisons: `abs(actual-expected) <= 1e-12 + 1e-10*abs(expected)`.

## Allowed writes

- `src/model/single_label.rs`;
- `src/evaluation/single_label.rs`;
- `src/evaluation.rs` for module dispatch and necessary T007 checked helpers;
- `src/model/common.rs` only for necessary T007 metric semantics or crate-private
  checked observation/source accessors;
- `tests/fixtures/single-label/expected.json` and minimal T008-only input fixtures;
- `tests/conformance.rs` only if it can use an existing real public app API. T014
  does not exist, so do not add placeholders, fake filters, private exports,
  `include!` of production source, or a duplicate scorer;
- the required response file.

Do not edit manifests/dependencies, validation invariants, schemas, app/CLI/
artifact modules, specs, plans, session notes, index, Fizzy, T009+ behavior,
multi-label behavior, or private acceptance data. Keep cyclomatic complexity at
or below 10 and consolidate repeated arithmetic rather than duplicating it.

## Local evidence and milestone

Create real owning-module T008 tests covering at least:

- matrix identities and wrong-count-once;
- literal-`ABSTAIN` label separation;
- identity, all-abstain, empty, observed-always-missed, thread, and F04 cases;
- exact metric value/status/scope/unit/ratio fields;
- sorted complete episode evidence and absence of opaque input.

The next progress milestone is a compiling complete hard-metric path plus at least
one executing T008 owning test. Continue through the whole unit after reaching it;
do not return a sketch or milestone as the final response.

Run the smallest sufficient checks, including:

```sh
cargo fmt --all -- --check
cargo check --locked
cargo test --locked --lib <each-new-T008-filter> -- --nocapture
cargo test --locked --lib
cargo test --locked --lib metric_status_precedence -- --nocapture
cargo test --locked --lib count_overflow_is_error -- --nocapture
cargo test --locked --lib macro_undefined_classes -- --nocapture
```

The exact integration filters `single_matrix_identities` and
`f04_asymmetric_oracle` remain pending until the real T014 application API. Do not
claim or fake them now. The full Clippy/all-tests/release boundary also remains at
T014. Disclose interim unwired dead-code diagnostics exactly; never suppress them,
add fake uses, or expand visibility to hide them.

## Required response

Continue until all T008 production behavior, independent fixtures, owning tests,
format, compile, and local regression checks are complete. Save the full response
before returning. Include exact input/output hashes, criterion-to-code/test
locations, every command/exit code/nonzero test count, direct F04/thread oracle
derivation, pending T014 filter status, and any warnings/failures.

Return early only for a concrete reproduced blocker with its exact command,
contract conflict, evidence, and smallest decision. An incomplete return without
such a blocker ends this worker's assignment and is escalated without reissue.
