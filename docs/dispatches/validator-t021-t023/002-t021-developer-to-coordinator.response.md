# T021 implementation handoff

Status: complete local implementation and verification. This is the atomic T021
policy milestone only; no T022 comparison, T023 intersection, or T024 work was
implemented.

## Delivered behavior

- `src/model/single_label.rs` now has concrete checked `SingleLabelPolicy`
  variants for `AsRecorded` and `RejectBelow`, with a checked signal enum.
  `src/validation.rs` admits only `confidence` or working
  `max_probability`, validates a finite `[0,1]` minimum, and rejects the wrong
  task, abstentions, or missing required scoring signals with `E_CONFIG` before
  scoring.
- `src/evaluation/single_label.rs` retains the recorded raw decisions and
  derives a separate final decision family. It uses strict `<` rejection,
  preserves probability metrics and raw confidence-bin participation, and writes
  the stable `below_minimum` rejection reason for a threshold-induced abstention.
- `src/model/multi_label.rs` now has concrete `MultiLabelPolicy` variants for
  `AsRecorded` and `LabelThresholds`. Admission verifies an exact complete
  vocabulary map, finite `[0,1]` values, answered outcomes, and complete
  marginals before scoring. The evaluator selects `p >= threshold` in vocabulary
  order; an all-below result is an answered empty set.
- The report serializes each actual applied policy and parameters. Report and
  inspection schemas bind policy alternatives to their task family; inspection
  validates the preserved configuration evidence, including the task-specific
  policy. Existing input schemas were unchanged.
- `tests/conformance.rs` adds public `evaluate`/published-report/verified-
  `inspect` coverage. It proves immediate binary64 below/equal/above cases,
  scored-choice confidence versus probability, classifier recorded-choice versus
  argmax, unchanged raw/probability results, retained confidence-bin IDs, and
  all-below answered empty-set behavior. It also proves `E_CONFIG` for wrong
  task, missing scoring family, incomplete threshold map, and submitted
  abstentions before publication.

## Criterion mapping

| T021 criterion | Evidence |
|---|---|
| Single-label exact rejection | `SingleLabelPolicy`, `admit_single_label_evaluation_config`, and `final_decisions`; `decision_policy_boundaries` checks adjacent binary64 values and both signals. |
| Multi-label exact thresholds | `MultiLabelPolicy`, `admit_multi_label_evaluation_config`, and `threshold_sets`; the same filter checks adjacent values, ordered output, and answered empty output. |
| Preserve raw/probability populations | Concrete evaluator split keeps raw scoring unchanged; the filter compares policy reports against as-recorded reports and checks raw confidence-bin IDs. |
| Report/inspection policy evidence | Both report assemblers serialize actual variants; `report.schema.json` and `inspection.schema.json` reject cross-task alternatives; public inspection assertions validate the preserved policy and final outcome. |
| Pre-score failures | `policy_preconditions` uses public evaluation calls and asserts `DiagnosticCode::Config` with no published run directory. |

## Verification

All commands ran in `/Users/dowwie/MyProjects/validator`.

| Command | Exit | Result |
|---|---:|---|
| `cargo test --locked --test conformance decision_policy_boundaries -- --nocapture` | 0 | 1 passed, 16 filtered. |
| `cargo test --locked --test conformance policy_preconditions -- --nocapture` | 0 | 1 passed, 16 filtered. |
| `cargo test --locked --test conformance -- --list` filtered for both names | 0 | Both filters listed as tests; nonzero count confirmed. |
| `cargo test --locked --test conformance` | 0 | 17 passed. |
| `cargo fmt --all -- --check` | 0 | Passed. |
| `cargo test --all-features --locked` | 0 | 44 library, 7 CLI, 17 conformance, and 0 doc tests passed. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Only the owner-authorized staged `dead_code` inventory: 17 production diagnostics and 3 matching lib-test duplicates, 20 total. No T021 policy/DTO/consumer diagnostic appeared. |
| `git diff --check` | 0 | Passed. |

The independent boundary expectations were: below rejects and equal/above pass
for both policies; scored-choice confidence can reject despite a higher maximum
probability; a classifier's non-argmax recorded class passes a max-probability
policy; and thresholding can return `[]` as an answered label set. All matched.

## Hash reconciliation

Starting hashes matched prompt002 exactly for T021, the v1/data-model/execution
contracts, owner prompt001, all listed source/schema/test inputs, and the
accepted T018-T020 handoff boundary.

| Artifact | Final SHA-256 |
|---|---|
| `src/model/single_label.rs` | `cc6a448ef4ca162aa3353e72084cb93a1f2c91d47e12b086463c8046b279976f` |
| `src/model/multi_label.rs` | `c019632dfef36ae441bc00cc3be1c7a58b539e9081107066d893b2993807f94f` |
| `src/validation.rs` | `fb6148069b4ade27eab57917e0111211274f3aa31c45b4458d3fa1b76a5482a0` |
| `src/evaluation/single_label.rs` | `f0cad620c80d8a1b7cef1847d49179c1b028f33546d40c5fc0b38cf5f002938c` |
| `src/evaluation/multi_label.rs` | `ff11dc9c2afbb5d1387051e245fd9532a90171a8a2c0086e1ea6b0fa1e8de8f3` |
| `schemas/v2/report.schema.json` | `4078be102abd69034932d38bda2e0c9c68e98b9bf3f59aa64cef116867a543d3` |
| `schemas/v2/inspection.schema.json` | `039e6060b88042c908c27a34374592d9f64434aa1aac4725da13146b01c0ac82` |
| `tests/conformance.rs` | `f80c6318e61882bc3b577756c9ed618451650282bb751206fbd0ccb77ba7b2e2` |

Unchanged listed artifacts retain their prompt002 hashes: `src/validation/wire.rs`
`3c98ed9c193cb8dff0f889956f58a4198b4d8a956a3da34592b45e5e381d29ad`
and `src/app.rs`
`3324dad26b9571529bdfb1a75b2bb17a9900bbb8f1a65ad822e7b389b76c6671`.

The required response artifact is intentionally not added to
`docs/artifact-index.md`: prompt002 explicitly prohibits editing the artifact
index, governance, plans, session notes, acceptance records, and Fizzy. The
coordinator owns that tracking follow-up.
