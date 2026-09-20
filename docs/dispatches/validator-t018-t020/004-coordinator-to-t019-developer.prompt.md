# Implement T019 multi-label hard accounting

Role/model: fresh sole implementation developer, `gpt-5.6-terra`, high
reasoning, `fork_turns: none`. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #184 — Build Validator’s concrete multi-label core](http://localhost:3006/1/cards/184).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t018-t020/004-t019-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat, return only
the response path, SHA-256 and terse status. Do not edit governance, plans,
dispatches, artifact-index, session notes, acceptance records or Fizzy.

## Read first and fixed input

Read completely before editing:

- `/Users/dowwie/.codex/AGENTS.md`, repository `AGENTS.md`, and
  `/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md`;
- `docs/plans/validator/tasks/T019.json`;
- `Multi-label hard decisions` and its required evidence cases in
  `docs/specs/validator-v1.md`;
- `Reports and metric reuse` in `docs/specs/validator-data-model.md`;
- `docs/plans/validator/execution-contract.md` and owner prompt001;
- T018 response003, the current multi-label model, common metric/status model,
  checked arithmetic, and single-label evaluator patterns before editing.

T018 is reconciled as a complete local milestone. T019 is the next atomic local
milestone, not an acceptance gate. Do not begin T020 probability metrics,
application/CLI/schema integration, T021 policies, T022 comparison, or later work.

Starting hashes:

| Artifact | SHA-256 |
|---|---|
| T019 contract | `0d6be8fa0d8b48cf4372e2a353c860420b75a54107c30446e21c4efd0cdc11f1` |
| v1 specification | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| data model | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| execution contract | `cdd917974df4fd505f91d165039988ca01d0936ad2e7703da47f30c7b3041e8f` |
| T018 handoff | `034a17855aed0330cce60ebfbc73ebf8125e059ae388e742d360850b278bf079` |
| `src/model.rs` | `846677319091911cd5f1a9a869aa45700b8ce46caf0923e779f74100c71dd4f6` |
| `src/model/common.rs` | `dfe120adb33f7a773edd5a8c44c8331fb918e417aec52dfc3d1c0a8077138791` |
| `src/model/multi_label.rs` | `26fd8a8798dcf14947e7ea9a8b1548979ca204eb84b2abff2cff7422b0b938c5` |
| `src/evaluation.rs` | `dbee2b6c74f60d5375545b746dbfea3ecc311ad55f3183ab02bdc260157bf891` |

Stop and report an exact mismatch before editing.

## Allowed write scope

Own only:

- `src/model/multi_label.rs` for concrete hard-result and per-episode evidence
  structures plus their owning tests;
- `src/evaluation/multi_label.rs` for borrowed concrete evaluation and exact hard
  accounting;
- `src/evaluation.rs` for the module declaration/private dispatch needed by the
  concrete evaluator;
- `tests/fixtures/multi-label/expected.json` for a hand-authored independent hard
  oracle.

Do not edit validation, application, CLI, schemas, public integration tests,
Cargo dependencies, or single-label behavior. In particular, do not edit
`tests/conformance.rs`: the task's exact public filters
`multi_label_hard_oracles` and `equal_counts_distinct_exact_sets` depend on the
real T020 application path and remain explicitly staged to T020. T019 must prove
the same accounting locally through concrete owning-module tests with nonzero
selection; it must not add a facade, source inclusion, public test API or widened
export to simulate that later boundary.

The evaluator accepts only `&MultiLabelEvaluation`. Keep task-specific types and
denominators concrete. Reuse checked arithmetic and existing checked metric,
scope, unit and status structures. Do not add a dynamic metric map, universal
accuracy abstraction, arbitrary metric selection, normalization, probability
scoring, policy execution or placeholder success fields.

## Required hard accounting

Implement raw and final hard families with the same structures now; under the
current `as_recorded` policy they are equal. Do not invent threshold behavior.

1. Define `N` as selected episodes, `U` as whole-episode abstentions, `G=N-U`,
   `D` as answered exact-set matches and `E=G-D`. An answered empty set counts in
   `G`; abstention does not. Use checked arithmetic throughout.
2. Produce exact-match accuracy `D/N`, wrong-set rate `E/N`, coverage `G/N`,
   abstention rate `U/N`, selective exact-match accuracy `D/G`, and selective risk
   `E/G`, with the specification's scopes, episode unit, denominators and statuses.
   For `G>0`, preserve the identity `exact_match_accuracy = coverage *
   selective_exact_match_accuracy`.
3. For each vocabulary label, count TP/FP/FN/TN only over answered episodes.
   Each four-count sum must equal `G`; the aggregate count sum must equal `G*K`.
   Also report reference `support` over all `N`, `answered_support=TP+FN`, and
   `predicted_support=TP+FP`.
4. Compute per-label precision, recall and F1 using answered counts. Compute
   answered micro precision/recall/F1, macro F1 with undefined label F1 zero-filled
   and `contains_undefined_classes` naming affected labels, and answered Hamming
   loss `(sum(FP)+sum(FN))/(G*K)`. Use `no_answered_predictions` when `N>0,G=0`
   and `no_data` when `N=0`; do not report unavailable values as numeric zero.
5. Preserve exact episode evidence in selected UUID order. For every answered row,
   emit expected, predicted, matched, missed and extra sets in vocabulary order
   plus correctness. For abstention, matched/missed/extra and predicted set are
   null with explicit `abstained` status; do not fabricate negatives or empty sets.
   Do not infer missed/extra substitutions.
6. Keep raw and final evidence separate even while equal under `as_recorded`.
   Bind every result to its evaluated population and unit. Preserve retained
   observations and admitted marginal values without scoring or copying them into
   hard evidence.

## Required independent evidence

Add a hand-authored `tests/fixtures/multi-label/expected.json`. It must not be
generated by production scoring. At minimum it records the independently
calculated rational expectations for these cases:

- vocabulary `[A,B,C]`, expected `[{A,B},{}]`, predicted `[{A,C},{}]`:
  `N=G=2`, exact match `1/2`, aggregate TP=1/FP=1/FN=1/TN=3,
  answered micro-F1 `1/2`, macro-F1 `1/3`, Hamming loss `1/3`;
- answered expected/predicted empty sets with nonempty vocabulary: exact match and
  coverage 1, Hamming loss 0, undefined label/micro F1 null, macro F1 zero with
  affected labels;
- expected `[{A},{}]`, predicted `[abstention,{}]`: `N=2,G=1,U=1,D=1`, exact
  match `1/2`, selective exact match 1, coverage `1/2`, and label A total support
  1, answered support 0, TN 1;
- every selected row abstains: coverage and nonempty-selection exact match zero,
  answered-only metrics `no_answered_predictions`, no fabricated empty sets;
- `N=0`: applicable metrics use `no_data`;
- two distinct episode arrangements with equal aggregate binary counts but
  different exact-set accuracy.

Use exact integer numerator/denominator assertions where the model supports them;
do not weaken the oracle to floating tolerances. Test the artifact by deserializing
its hand-authored values in test code only, without giving production code a
fixture-shaped interface.

Implement and run these exact nonzero owning-library filters:

```text
cargo test --locked --lib multi_label_hard_accounting -- --nocapture
cargo test --locked --lib equal_binary_counts_distinct_exact_sets -- --nocapture
```

The first must cover all listed oracle/status/evidence cases and both equal raw
and final families. The second must demonstrate the discriminating equal-counts
case. Confirm both names exist with `cargo test --locked --lib -- --list` and
record the selected count; a zero-test result is failure.

Also run:

```text
cargo fmt --all -- --check
cargo test --all-features --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
git diff --check
```

Clippy started with exactly 43 owner-staged `dead_code` diagnostics: 24 inherited
from the accepted single-label backbone and 19 T018 contracts mapped to T019-T021.
Real T019 consumers should reduce that inventory. No ordinary warning or new
unmapped dead code is allowed. A residual hard-result/report field is stageable
only when its exact real consumer is T020 integration; name each symbol and
consumer. Do not suppress warnings, add fake uses, expand visibility or delete a
required contract to make the intermediate lint output look clean.

## Handoff

The response must map every T019 criterion and normative clause to concrete types
and named assertions; explain independent oracle derivations; record exact command,
exit, selected test count and actual result; list starting and final hashes for
every changed/relied-on artifact; give the exact residual Clippy count/symbol and
owning-task map; disclose any blocker; and confirm no T020+ implementation,
app/CLI/schema/conformance/governance/index/Fizzy work. Completion is a real local
hard-accounting milestone, not independent acceptance.
