# T018 checked multi-label admission handoff

Status: complete local implementation candidate. No T019+ scoring, policy
execution, comparison, application/CLI, report, schema, artifact, governance,
index, session-note, or Fizzy work was performed.

## Implementation and criterion map

`src/model/multi_label.rs` adds checked `LabelMarginals`, `MultiLabelOutput`,
`MultiLabelPolicy::AsRecorded`, `MultiLabelAlignedRow`, and
`MultiLabelEvaluation`. Marginals require every vocabulary key, preserve finite
`[0,1]` inputs in vocabulary order, and perform neither sum validation nor
normalization. Outputs hold only `Outcome<LabelSet>` and optional marginals;
they have no scoring confidence. Empty answered sets and whole-episode
abstention are distinct.

`src/model.rs` declares that module and private closed
`ValidatedTask::{SingleLabel, MultiLabel}` dispatch. The existing single-label
application-facing `ValidatedEvaluation` remains unchanged.

`src/validation/wire.rs` now has strict tagged task, target, outcome,
probability, and decision DTO variants. It never infers a task kind from labels
or probability sums. `src/validation.rs` adds the concrete multi-label admission
and private closed dispatcher. It reuses exact digest, UUID, source,
evidence/preparation, observation, selection, and sorted one-to-one alignment
admission; accepts only classifier sources; rejects top-level confidence and
categorical probabilities; and preserves retained reported-confidence
observations as observations.

The sole shared-model exception is `src/model/common.rs`: `Population` is now
generic over its concrete policy, with `EvaluationConfig::new_with_policy`.
This is necessary to make `Population<MultiLabelPolicy>` impossible to populate
with `SingleLabelPolicy`; single-label construction remains concrete at its
existing call sites.

`validation::tests::multi_label_checked_admission` asserts valid nonempty and
empty answers, abstention, vocabulary-sorted set and `[0.9,0.8]` marginal values,
retained reported confidence, selection/alignment reuse, duplicate labels,
missing/extra/duplicate rows, missing/extra marginal keys, source mismatch,
top-level confidence, categorical probability, and partial/per-label-abstention
shapes with stable diagnostics.

`validation::tests::cross_task_boundaries` asserts declared-task dispatch,
wrong target/outcome/probability/source/policy rejection, no task-kind inference,
and the distinct same-numbers contract: multi-label `[0.9,0.8]` marginals are
legal while a single-label categorical vector with those values is invalid.

Existing wire tests still exercise strict duplicate-key/unknown-field decoding,
opaque large integers, `1e400`, explicit null, and literal serde-number-key
regressions. Their tagged-variant assertions now distinguish wire acceptance
from task-specific admission rejection.

## Commands

| Command | Exit | Actual result |
|---|---:|---|
| `cargo test --locked --lib multi_label_checked_admission -- --nocapture` | 0 | 1 selected, 1 passed |
| `cargo test --locked --lib cross_task_boundaries -- --nocapture` | 0 | 1 selected, 1 passed |
| `cargo fmt --all -- --check` | 0 | passed |
| `cargo test --all-features --locked` | 0 | 42 library, 6 CLI, and 11 conformance tests passed |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | 43 `dead_code` diagnostics; no other warning class |
| `git diff --check` | 0 | passed |

Clippy remains an authorized staged-lint failure. The inherited 24 diagnostics
cover existing private contracts. The additional 19 diagnostics are only private
T018 symbols: `ValidatedTask`; multi-label vocabulary/set constructors,
`LabelMarginals`, output/policy/row/evaluation; generic population-policy
construction; multi-label validation/config/output helpers; and tagged wire
fields. Their real consumers are T019 hard accounting, T020 marginal/application
dispatch, and T021 policy payload execution. No suppression, fake consumer,
widened export, or fallback was added. T027 remains the warning-free gate.

## Hashes

Starting fixed inputs:

| Artifact | SHA-256 |
|---|---|
| `docs/plans/validator/tasks/T018.json` | `c0a07aa8b04bec25733dd20c64dbf9444b6412c90db49c4e8a13736c38100c04` |
| `docs/specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `docs/specs/validator-data-model.md` | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| `src/model.rs` | `ceffb61ab5e523fa4bf3161d5db5a6fcd1276137d951982da4382edb79c58c66` |
| `src/model/common.rs` | `5dd0c2652439434ba9496a503c6317d8775cdbc90d8ded7b7dac5d351005d4f3` |
| `src/model/single_label.rs` | `5cff2210c215bc84e6a95777649a6e0700a3547056fbb8a93365f9d1e6b1a019` |
| `src/validation.rs` | `5a76b0ccc8bd838c13e8671cfb063b27888f0f9b20ecb6e5fa6c26a03c8b49bf` |
| `src/validation/wire.rs` | `46a42903c2edbb556b1ae1ee7b3c3d9642e562e6b2de2a6d63a852c584f6a3b3` |

Final changed/relevant inputs:

| Artifact | SHA-256 |
|---|---|
| `docs/plans/validator/tasks/T018.json` | `c0a07aa8b04bec25733dd20c64dbf9444b6412c90db49c4e8a13736c38100c04` |
| `docs/specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `docs/specs/validator-data-model.md` | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| `src/model.rs` | `846677319091911cd5f1a9a869aa45700b8ce46caf0923e779f74100c71dd4f6` |
| `src/model/common.rs` | `dfe120adb33f7a773edd5a8c44c8331fb918e417aec52dfc3d1c0a8077138791` |
| `src/model/single_label.rs` | `27d3abef765549fc3f145ed9588f70993cbaa3b152a22754dc45bd277dd2c573` |
| `src/model/multi_label.rs` | `26fd8a8798dcf14947e7ea9a8b1548979ca204eb84b2abff2cff7422b0b938c5` |
| `src/validation.rs` | `c6e40115a863bba54fab8f84b32beba53b56df153de008bb3fbc63e6ea2886fa` |
| `src/validation/wire.rs` | `3c98ed9c193cb8dff0f889956f58a4198b4d8a956a3da34592b45e5e381d29ad` |

No blocker remains for the next fresh T019 implementation handoff. This is a
local milestone, not an independent acceptance decision.
