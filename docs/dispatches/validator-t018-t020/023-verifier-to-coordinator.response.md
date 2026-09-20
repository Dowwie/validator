# Independent T018–T020 verifier verdict

Verdict: **Revise**.

The frozen candidate is stable and the hard-accounting, marginal-scoring,
application, replay, CLI, and ordinary-lint gates support their current
criteria. Two bounded defects remain in the same cross-task source-kind
contract: multi-label admission accepts an unused `scored_choice` source, and
the report schema certifies the resulting foreign-task source definition.

## Candidate reconciliation

- Manifest022 SHA-256 recomputed as
  `918fd6e13608940232de86f3bfb4b4dfba81c7d5cf618657225cef260e05c177`,
  exactly the dispatch-bound value.
- All 49 file hashes listed by manifest022—governing prompts, task contracts,
  specifications, execution contract, Cargo/toolchain inputs, seven complete
  handoffs, sixteen source files, eight schemas, two test files, and the
  independent multi-label oracle—recomputed exactly. No moving-candidate
  mismatch occurred before or after review.
- The independently rebuilt release binary is
  `d7878036835693417283483dfa7b87fbe204fe39e592463368c588121bd81b41`,
  matching the developer's recorded binary identity.
- The repository remained read-only during review apart from this required
  response. Reproduction files and one reproduced run were created only under
  `/tmp/validator-t020-verifier-unused-source/`.

## Findings

### 1. T018 admits a `scored_choice` source into a multi-label evaluation when no row references it

**Current criterion.** T018 requires scored-choice sources to reject for
multi-label tasks; owner001 and verifier prompt023 state that only classifier
sources are admitted for multi-label. The ratified prediction contract allows
unused sources generally, but its scored-choice profile is explicitly legal
only for single-label tasks.

**Location.** `src/validation.rs:234` admits the complete source map without a
task-specific source-kind check. `src/validation.rs:242-244` checks for
`SourceKind::Classifier` only while visiting a prediction's referenced source.
Consequently, an unused foreign-kind source bypasses the check and is retained
in `MultiLabelEvaluation::sources`.

**Independent reproduction.** The following exact documents were used:

```json
{"schema_version":2,"task":{"kind":"multi_label","labels":["A"]},"episodes":[]}
```

The dataset SHA-256 is
`9ae26f6e48ac56939815a6a492609c132f0f6e1b9f84b498a100c54f00213531`.

```json
{"schema_version":2,"dataset_sha256":"9ae26f6e48ac56939815a6a492609c132f0f6e1b9f84b498a100c54f00213531","sources":{"unused":{"kind":"scored_choice","model":"m","configuration":{},"question_id":"q"}},"predictions":[]}
```

```json
{"schema_version":2,"population":"empty","role":"development","decision":{"type":"as_recorded"}}
```

Running the real binary:

```text
target/debug/validator check --dataset /tmp/validator-t020-verifier-unused-source/dataset.json --predictions /tmp/validator-t020-verifier-unused-source/predictions.json --config /tmp/validator-t020-verifier-unused-source/config.json
```

returned exit `0` and:

```json
{"integrity":{"extra_ids":[],"missing_ids":[],"prediction_count":0,"selected_count":0,"signal_availability":"none","source_count":1},"kind":"check","schema_version":2,"selected_ids":[],"status":"complete","task":{"kind":"multi_label","labels":["A"]}}
```

This is not an empty-artifact ambiguity: the foreign source is fully admitted,
counted, retained in the multi-label evaluation, and subsequently publishable.

**Consequence.** A multi-label run can claim a source kind whose output contract
is defined only for single-label, violating closed task dispatch and the source
profile before scoring. The existing tests mutate a referenced source and do
not discriminate this unused-source path.

**Smallest correction.** In `validate_multi_label`, reject any declared source
whose kind is not `Classifier` immediately after shared source admission, before
row alignment. Add an unused, well-formed `scored_choice` source negative to the
existing `multi_label_checked_admission` or `cross_task_boundaries` test and
assert the stable `E_CONFIG` diagnostic. Shared source decoding need not change.

### 2. The T020 report schema certifies the same foreign-task source definition as a valid multi-label report

**Current criterion.** T020 requires closed, coherent single/multi report
alternatives that reject foreign-task keys and near misses. Owner012 explicitly
requires the shared source constraints and task-family discrimination to be
concrete in `report.schema.json`.

**Location.** `schemas/v2/report.schema.json:12` applies the common `source`
definition to every task. That definition at line 37 permits both `classifier`
and `scored_choice`. The multi-label conditional at line 27 restricts hard,
probability, signal, and episode families, but does not restrict `sources`.

**Independent reproduction.** Evaluating the three documents above with the real
binary returned exit `0` and published a multi-label report whose source was:

```json
{"unused":{"configuration":{},"evidence":[],"kind":"scored_choice","model":"m","observations":{},"question_id":"q"}}
```

Validation of that exact published report against the frozen
`schemas/v2/report.schema.json` using JSON Schema Draft 2020-12 completed
successfully. Thus the published contract independently accepts the forbidden
task/source combination; fixing admission alone would leave the machine schema
advertising it as legal.

**Consequence.** Consumers using the published schema can accept a report that
cannot belong to a valid multi-label run under the ratified source contract.
This is a task-family coherence failure at the application contract boundary.

**Smallest correction.** Add a classifier-only source definition or equivalent
constraint to the multi-label branch of the report schema while preserving the
existing single-label source alternatives. Add a focused mutation of a valid
multi-label report to `scored_choice` (with otherwise legal scored-choice fields)
and assert schema rejection. No check/inspection schema or artifact format
change is required for this defect.

## Independent criterion review

Apart from the two findings above, the inspected frozen paths support the current
T018–T020 criteria:

- Wire task, target, outcome, probability, observation, and decision variants
  are explicitly tagged and closed. Validated task dispatch is concrete. Label
  sets reject unknown and duplicate labels before construction, sort by the
  owning vocabulary, and preserve answered empty sets separately from abstention,
  missing rows, and missing marginal keys. Marginals require exact vocabulary
  coverage, finite `[0,1]` values, preserve `[0.9,0.8]`, and never normalize.
  Top-level multi-label confidence, categorical probabilities, mismatched
  targets/policies, partial objects, and per-label abstention shapes reject.
  Retained reported-confidence observations remain separate evidence. Existing
  exact-byte digest, UUID, source/evidence/preparation, sorted alignment,
  duplicate-key, large-integer, `1e400`, explicit-null, and literal-number-key
  regressions remain exercised.
- `evaluate(&MultiLabelEvaluation)` borrows only the checked concrete type.
  Under current `as_recorded`, raw and final are distinct result values with
  equal contents and no threshold placeholder. `N/U/G/D/E`, set metrics,
  coverage, abstention, and selective metrics use the specified populations.
  Per-label counts use answered rows, each four-count sum is `G`, the aggregate
  is `G*K`, support uses all `N`, and answered/predicted support are `TP+FN` and
  `TP+FP`. Micro, macro, Hamming, zero-denominator, `no_data`, and
  `no_answered_predictions` paths use the named denominators and statuses.
  Ordered episode evidence preserves expected/predicted/matched/missed/extra
  sets and observations; abstentions retain null differences without fabricated
  negative decisions.
- Complete marginals score all `N*K` selected label decisions, including
  abstentions. The positive log branch is `-ln(p)`, the negative branch is
  `-ln_1p(-p)`, impossible observed outcomes become explicit positive infinity,
  and per-label/mean Brier and log populations are correct. Bins use reference
  presence and correct `0`, `.1`, and `1` boundaries. No clipping,
  normalization, argmax, joint-set probability, multi-label confidence, or ECE
  is synthesized.
- Check/evaluate share admission and evidence loading; publication is shared;
  inspect validates stored snapshots and contained evidence for both task kinds,
  rebuilds from verified contained bindings, survives removal of originals, and
  rejects evidence tampering as `E_PROVENANCE`. The real CLI emits one JSON
  document, keeps normal output/errors private, refuses existing destinations
  without tree mutation, and preserves accepted single-label/replay behavior.
  No multi-label comparison or threshold-policy behavior was introduced.
- The frozen check, report, and inspection schemas are closed for the tested
  concrete result families, required nested fields, finite/null/infinite status
  forms, task-specific hard/probability/signal/episode structures, and opaque
  payload positions, subject to finding 2's source-kind relation.

## Independent oracle calculations

- For rows `{A,B}->{A,C}` and `{}` to `{}`, vocabulary `[A,B,C]`, the first row
  contributes one TP (`A`), one FN (`B`), and one FP (`C`); the second contributes
  three TN. Therefore aggregate `TP/FP/FN/TN = 1/1/1/3`, exact accuracy is `1/2`,
  micro F1 is `2/(2+1+1)=1/2`, per-label F1 values are `1,0,0` so zero-filled
  macro F1 is `1/3`, and Hamming loss is `(1+1)/(2*3)=1/3`.
- The equal-count pair is genuinely discriminating. In the first arrangement,
  `{A,B}->{A,B}` and `{}->{A,B}` give each label one TP and one FP and exact
  accuracy `1/2`. In the second, `{A}->{A,B}` and `{B}->{A,B}` retain the same
  per-label one-TP/one-FP counts but neither set is exact, so accuracy is `0/2`.
- For marginals `[.8,.7]` against `{A}`, binary Brier terms are `.04` and `.49`,
  hence mean `.265`. Mean log loss is
  `(-ln(.8)-ln(.3))/2 = 0.7135581778200729`. One-label binary Brier is `.04`,
  distinct from categorical `[.8,.2]` Brier `.04+.04=.08`. Absent-at-zero loss
  is zero; present-at-zero and absent-at-one are positive infinity. Empty answer,
  one/all abstention, and empty-population status results agree with the ratified
  identities and the hand-authored oracle.

## Independent command evidence

| Command | Exit | Independent result |
|---|---:|---|
| `cargo test --locked --lib multi_label_checked_admission -- --nocapture` | 0 | 1 passed, 43 filtered |
| `cargo test --locked --lib cross_task_boundaries -- --nocapture` | 0 | 1 passed, 43 filtered |
| `cargo test --locked --test conformance multi_label_hard_oracles -- --nocapture` | 0 | 1 passed, 14 filtered |
| `cargo test --locked --test conformance equal_counts_distinct_exact_sets -- --nocapture` | 0 | 1 passed, 14 filtered |
| `cargo test --locked --test conformance marginal_loss_and_bins -- --nocapture` | 0 | 1 passed, 14 filtered |
| `cargo test --locked --test conformance single_report_schema -- --nocapture` | 0 | 1 passed, 14 filtered |
| `cargo test --locked --test conformance check_inspection_schema_contract -- --nocapture` | 0 | 1 passed, 14 filtered |
| `cargo test --locked --test conformance replay_binding_and_result_tampering -- --nocapture` | 0 | 1 passed, 14 filtered |
| `cargo test --locked --test cli shared_commands_multi_label -- --nocapture` | 0 | 1 passed, 6 filtered |
| `cargo test --locked --test cli relocated_run_inspection -- --nocapture` | 0 | 1 passed, 6 filtered |
| `cargo fmt --all -- --check` | 0 | passed |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | exactly 20 production `dead_code` diagnostics; no ordinary/new warning class |
| `cargo test --all-features --locked` | 0 | 44 library, 7 CLI, 15 conformance, 0 doc failures |
| `cargo build --release --locked --bin validator` | 0 | passed; hash recorded above |
| `ruby docs/plans/validator/verify-plan.rb` | 0 | 35 tasks, 433 source blocks, 63 conformance cases, 12 structure checks, 8 ACs, 5 DoD clauses; all plan/link/index checks passed |
| `git diff --check` | 0 | passed |

The 20 Clippy diagnostics match response021's owner-authorized staged inventory:
six task-definition/type groups in `src/model.rs`; LabelSet methods, Episode type
and methods, ObservationSet values, EvaluationConfig policy/accessor, and
MetricResult accessors in `src/model/common.rs`; `signal_availability` in
`src/model/single_label.rs`; and schema-version/input/later-policy DTO fields in
`src/validation/wire.rs`. There is no suppression, fake consumer, widened export,
changed tolerance, or ordinary warning in the residual output.

## Review limits and return condition

This review is limited to the frozen T018–T020 candidate. It does not assess T021
threshold execution, T022 multi-label comparison, intersection/installation, or
future staged-warning cleanup. The two findings are reproducible current-contract
violations with bounded corrections. Freeze the corrected admission, report
schema, and focused negatives, rerun the affected filters plus justified schema/
CLI regressions, and return the new hashes for the authorized focused recheck.
