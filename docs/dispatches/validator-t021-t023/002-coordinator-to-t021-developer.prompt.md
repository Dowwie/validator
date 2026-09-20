# Implement T021 exact task-specific decision policies

Role/model: fresh sole implementation developer, `gpt-5.6-terra`, high
reasoning, `fork_turns: none`. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #185 — Build Validator policies and comparison core](http://localhost:3006/1/cards/185).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t021-t023/002-t021-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat, return only
the response path, SHA-256 and terse status. Do not edit governance, plans,
dispatches, artifact-index, session notes, acceptance records or Fizzy.

## Read first and fixed boundary

Read completely before editing:

- `/Users/dowwie/.codex/AGENTS.md`, repository `AGENTS.md`, and Rust best-practices;
- `docs/plans/validator/tasks/T021.json`;
- `Evaluation configuration`, `Recorded outcomes and scoring signals`, relevant
  reporting/inspection clauses and policy evidence rows in `validator-v1.md`;
- data-model checked construction/report reuse sections and execution contract;
- owner sequence prompt001 and the accepted T018-T020 handoff027;
- current policy wire DTOs, checked model/evaluation/application serialization,
  report/inspection schemas and existing conformance helpers before editing.

T018-T020 is owner-accepted. T021 is an atomic local milestone, not acceptance.
Do not begin T022 comparison, T023 intersection or T024 work.

Starting hashes:

| Artifact | SHA-256 |
|---|---|
| T021 task | `fc39187960d1f8b172a4f6efb8173f1a78cea08ee187b64f424acb30f816d80c` |
| v1 specification | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| data model | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| execution contract | `cdd917974df4fd505f91d165039988ca01d0936ad2e7703da47f30c7b3041e8f` |
| owner prompt001 | `73f5ebe3a192a0baedf9787b55a356d49b03211fa093edfcabf47034cecc15b9` |
| `src/model/single_label.rs` | `27d3abef765549fc3f145ed9588f70993cbaa3b152a22754dc45bd277dd2c573` |
| `src/model/multi_label.rs` | `d0f8a9c7a6c164ae3d34f6111a31c2b52aeead7021c9cf9b25de7d079d7eb46a` |
| `src/validation.rs` | `891e6ee4e95b2be1931dd2006d02940308766b8a7b7d486b85f15faafbb69892` |
| `src/validation/wire.rs` | `3c98ed9c193cb8dff0f889956f58a4198b4d8a956a3da34592b45e5e381d29ad` |
| `src/evaluation/single_label.rs` | `fd271a77be2c1c9aa244749cea9a026cc9efb3e0c00f5a263df428c82caaa19d` |
| `src/evaluation/multi_label.rs` | `013f014ed1300a4e2173bc0a4a187af53717079c7d80a37575932e242287257b` |
| `src/app.rs` | `3324dad26b9571529bdfb1a75b2bb17a9900bbb8f1a65ad822e7b389b76c6671` |
| report schema | `8f7c4fba2c12381e535888bbde5627a0723471c2018f3ab516a7ce661cb590d6` |
| inspection schema | `3c32ad02a1376b790294bcd655c8c4ffc711a4d90274546303e60577425fc176` |
| `tests/conformance.rs` | `aab6e45f86a34d01421e26195b9f06bbca195aefd92e3d565ce84d2fcb1c3f00` |

Stop and save the exact mismatch before editing.

## Write scope and architecture

Own only the T021 artifacts: single/multi model and evaluator files,
`src/validation.rs`, `tests/conformance.rs`, and the task-owned report/inspection
schema branches. A minimal existing wire/app accessor or serialization change is
allowed only when the actual real policy path requires it; report and justify it.
Do not add dependencies, public SDK expansion, policy chaining, dynamic policy
registry, generic evaluator framework, comparison or intersection behavior.

Use concrete checked `SingleLabelPolicy` and `MultiLabelPolicy` variants. Admission
must validate policy task kind, thresholds, artifact-wide required signals and
answered-outcome preconditions before scoring. Application must derive final
outcomes while preserving raw outcomes/results, original scoring probabilities,
observations and raw signal populations.

## Exact policy semantics

### Single-label `reject_below`

- Accept only `confidence` or `max_probability` and a finite minimum in `[0,1]`.
- Require every supplied/selected row to be an answered class outcome and to carry
  the requested scoring signal. Observations never substitute. Submitted
  abstention or missing family is `E_CONFIG` before scoring.
- `confidence` uses checked reported scoring confidence. `max_probability` uses
  the working categorical vector maximum, including scored-choice confidence versus
  probability and classifier recorded-choice versus argmax distinctions.
- Reject only when signal `< minimum`; equality passes. Prove immediately adjacent
  binary64 below/equal/above values exactly. Final rejection uses the specified
  stable reason. Raw outcomes/metrics/probability and confidence bins retain later
  rejected answered rows and their IDs.

### Multi-label `label_thresholds`

- Require an exact complete vocabulary threshold map, each finite `[0,1]`, every
  row answered and every row carrying complete marginals. Reject wrong kind,
  partial/extra/default thresholds, abstention or missing marginals before scoring.
- Select each label exactly when `p >= threshold`, in vocabulary order. All below
  produces an answered empty set, never abstention. Equality passes; adjacent
  binary64 values discriminate.
- Preserve raw sets/results, original marginals/probability populations and
  observations. No normalization, defaults, chaining or synthesized answers.

Policy configuration and effective report/inspection evidence must serialize the
actual applied variant and parameters. Extend strict report/inspection schema
alternatives without weakening accepted as-recorded or task-family constraints.
The existing input schemas change only if an actual reproduced mismatch requires it.

## Required real-path evidence

Implement and run the exact nonzero filters through public evaluate/published
report and verified inspect paths:

```text
cargo test --locked --test conformance decision_policy_boundaries -- --nocapture
cargo test --locked --test conformance policy_preconditions -- --nocapture
```

The first must independently assert below/equal/above for both policies, the two
single-label signal distinctions, all-below answered empty set, unchanged raw/
probability metrics, final evidence/reason and raw confidence-bin retained IDs.
The second must assert wrong-task, missing family, incomplete threshold map and
submitted-abstention failures before scoring with stable diagnostics.

Also run listing/nonzero confirmation, focused accepted regressions, then:

```text
cargo fmt --all -- --check
cargo test --all-features --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
git diff --check
```

Clippy may retain only the exact owner-authorized staged private inventory; real
T021 consumers should naturally remove policy-field diagnostics. No ordinary/new
warning, suppression, fake use, export widening or unrelated cleanup.

Response002 must map every criterion to concrete types/paths/schema assertions,
record independent boundary expectations, exact list/command/exit/count/results,
starting/final hashes and residual lint inventory. Confirm no T022+ work. A complete
local handoff advances automatically to a fresh T022 writer.
