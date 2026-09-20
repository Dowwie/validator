# Implement T018 checked multi-label admission

Role/model: fresh sole implementation developer, `gpt-5.6-terra`, high
reasoning, `fork_turns: none`. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #184 — Build Validator’s concrete multi-label core](http://localhost:3006/1/cards/184).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t018-t020/003-t018-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat, return only
the response path, SHA-256 and terse status. Do not edit governance, plans,
dispatches, artifact-index, session notes, acceptance records or Fizzy.

## Read first and fixed input

Read completely before editing:

- `/Users/dowwie/.codex/AGENTS.md`, repository `AGENTS.md`, and
  `/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md`;
- `docs/plans/validator/tasks/T018.json`;
- `Canonical golden dataset` and `Canonical prediction artifact` in
  `docs/specs/validator-v1.md`;
- `Task-specific structures`, `Absence and uncertainty`, and the shared admission
  boundary in `docs/specs/validator-data-model.md`;
- `docs/plans/validator/execution-contract.md` and the T018-T020 owner prompt001;
- the current shared model, wire decoder, validator and their tests before edits.

The T017 architecture gate is owner-accepted. T018 is an atomic local milestone,
not a separate acceptance gate. Do not begin hard metrics/T019, marginal
metrics/T020, policies/T021, comparison/T022, app/CLI/report integration, or
schema work.

Starting hashes:

| Artifact | SHA-256 |
|---|---|
| T018 contract | `c0a07aa8b04bec25733dd20c64dbf9444b6412c90db49c4e8a13736c38100c04` |
| v1 specification | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| data model | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| `src/model.rs` | `ceffb61ab5e523fa4bf3161d5db5a6fcd1276137d951982da4382edb79c58c66` |
| `src/model/common.rs` | `5dd0c2652439434ba9496a503c6317d8775cdbc90d8ded7b7dac5d351005d4f3` |
| `src/model/single_label.rs` | `5cff2210c215bc84e6a95777649a6e0700a3547056fbb8a93365f9d1e6b1a019` |
| `src/validation.rs` | `5a76b0ccc8bd838c13e8671cfb063b27888f0f9b20ecb6e5fa6c26a03c8b49bf` |
| `src/validation/wire.rs` | `46a42903c2edbb556b1ae1ee7b3c3d9642e562e6b2de2a6d63a852c584f6a3b3` |

Stop and report an exact mismatch before editing.

## Allowed write scope

Own only:

- `src/model/multi_label.rs` (new concrete checked multi-label structures and
  owning unit tests);
- `src/model.rs` (module declaration and private closed validated-task dispatch);
- `src/validation.rs` (actual shared wire-to-domain multi-label admission and
  owning tests while preserving single-label admission);
- `src/validation/wire.rs` (strict concrete tagged multi-label DTO paths).

Use the existing `common` vocabulary/index/set/source/observation/population and
checked configuration contracts as actual consumers. Do not duplicate them. A
small existing shared accessor may be changed only if the new concrete admission
actually requires it; report and justify that exception. Do not touch app, CLI,
artifacts, evaluation, schemas, integration tests or Cargo dependencies.

Application-facing closed dispatch waits for T020. T018 may add a private closed
validated enum and shared validation entry that the owning-library tests exercise,
while preserving the existing single-label application path. Do not add an
unimplemented success variant, placeholder marginal result, facade, public test
API, source inclusion or fake consumer.

## Required checked structures and admission

1. Add separate concrete `MultiLabelEvaluation`, aligned row, output, marginal and
   policy types with private fields and checked construction. Reuse the existing
   `LabelSet` owned by the checked vocabulary. Empty sets are valid answered
   targets/outcomes and serialize/iterate in vocabulary order; reject duplicate
   submitted labels before set construction rather than deduplicating.
2. `MultiLabelOutput` contains `Outcome<LabelSet>`, optional `LabelMarginals`, and
   shared retained observations only. It has no scoring confidence. Marginals are
   distinct from categorical distributions, require exactly every vocabulary key,
   preserve each finite `[0,1]` value unchanged in vocabulary order, and have no
   cross-label sum check or normalization.
3. Extend private wire DTOs with explicit tagged task/target/outcome/probability
   variants. Never infer task kind from target cardinality or probability sums.
   Preserve strict duplicate-key and unknown-field rejection, exact dataset digest,
   UUID/source/evidence/preparation checks, selection and one-to-one sorted
   alignment through the existing shared path.
4. Admit multi-label only for `classifier` sources. Reject `scored_choice`,
   top-level scoring confidence, categorical probabilities, count/multiset shapes,
   partial-label/per-label-abstention shapes, wrong target/outcome variants,
   missing/extra/duplicate rows and missing/extra marginal keys. An explicit
   whole-episode abstention is distinct from an answered empty set. Missing rows
   remain alignment errors.
5. Retained `reported_confidence` observations remain legal when declared and
   checked as observations; they never become scoring confidence or marginals.
   Preserve existing opaque input/source/preparation behavior and single-label
   large-integer, `1e400`, null and literal serde-number-key regressions.
6. Current operational policy is `as_recorded`. Use concrete task-specific policy
   types so cross-kind policies cannot reach the wrong evaluation. Any checked
   threshold structure needed by the ratified model may remain private and staged
   only for its real T021 consumer; never silently execute it as as-recorded.
7. Provide a closed private validation dispatch with concrete SingleLabel and
   MultiLabel variants without changing the working single-label application
   behavior. No traits, dynamic task registry, generic evaluator framework,
   universal probability abstraction or public SDK expansion.

## Required evidence

Implement and run these exact nonzero owning-library filters:

```text
cargo test --locked --lib multi_label_checked_admission -- --nocapture
cargo test --locked --lib cross_task_boundaries -- --nocapture
```

`multi_label_checked_admission` must assert valid nonempty and empty answered
sets, explicit abstention, unchanged `[0.9,0.8]` marginals, vocabulary ordering,
declared retained confidence observation, selection/alignment reuse, and each
listed duplicate/missing/extra/unsupported wire failure with its stable diagnostic.

`cross_task_boundaries` must prove wrong target/outcome/probability/policy/source
combinations are rejected and that no inferred task kind or cross-kind coercion
exists. Include a same-numbers case showing label marginals `[0.9,0.8]` are legal
without normalization while the corresponding single-label categorical contract
is distinct.

Also run:

```text
cargo fmt --all -- --check
cargo test --all-features --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
git diff --check
```

Clippy previously exited 101 for the owner-staged 24 production dead-code
diagnostics. Real T018 consumers should reduce that inventory. No new ordinary
warning is allowed. A new private field/type is stageable only when its exact real
consumer is T019, T020, T021 or T022 and you name that consumer; no suppression,
fake use or widened export.

## Handoff

The response must map every T018 criterion to concrete types and named assertions;
record exact command, exit, selected test count and actual result; list starting
and final hashes for every changed/relied-on artifact; give the exact residual
Clippy count/symbol/owning-task map; disclose any blocker; and confirm no T019+
implementation, app/CLI/schema/governance/index/Fizzy work. Completion is a real
checked admission milestone with both filters nonzero, not checkpoint acceptance.

