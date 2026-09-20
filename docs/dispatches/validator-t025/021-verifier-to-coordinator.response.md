# T025 final focused unit-metadata verdict

Verdict: **Ready**.

The final frozen candidate corrects the multi-label per-label hard-metric
population unit, preserves the already accepted numerical behavior, and closes
the remaining literal zero-operand proof. No blocking affected regression or
remaining T025 defect was found in this focused recheck.

## Reconciliation and fixed expectation

Final repaired manifest020 has SHA-256
`deeed4c908fa385de1cd8494a31fdcc7fb8d46646e03b32766691fe007f24fc9`.
Every listed governing/correction artifact, ratified contract, mapping, source,
schema, test, fixture, Cargo file, and toolchain hash reconciled exactly before
review. The final changed identities are:

| Artifact | SHA-256 |
|---|---|
| `src/evaluation/multi_label.rs` | `c4158facd454e66e66b3f1ddc0bb9976ca07fca6961bf200597ba9b31107b461` |
| `tests/conformance.rs` | `13409645624e53b8a3f2b4e5719dd7ae651527e0c4f886dcb511bfa10bf38bef` |

The unchanged fixture/schema identities also reconcile:

| Artifact | SHA-256 |
|---|---|
| `tests/fixtures/single-label/expected.json` | `ab93c379b9c0df1d0c0fb7aac7a6df5e7748f3f671d55dd69d173e640090b268` |
| `tests/fixtures/multi-label/expected.json` | `213183a46c0f6c011296ef5087b5c44d3852081bdf20eaa80cbb494d3d2c84f2` |
| `src/model/multi_label.rs` | `3f515a0a9122db963b2e01ca14d5474b4836ba3543a48a4e0766a4459eea0b7c` |
| `schemas/v2/report.schema.json` | `805de93d2a50339370e803b7ff1a9a1b8d34c3028b5b21a1a6f428f591733f90` |
| `schemas/v2/inspection.schema.json` | `2c7e064d0adb915b77c558a7ebbfade96f7bcf0eee890b881ced1383f0eedaef` |

I read owner prompt016, owner clarification018, and the complete developer
response017. Before consulting the repaired public output, I fixed the governing
expectation independently: every multi-label per-label precision, recall, and F1
uses population count `G`, unit `label_decision`, and scope `answered`; aggregate
binary metrics use `G*K` label decisions; set metrics use episodes.

## Production correction

The only scoped production change is at
`src/evaluation/multi_label.rs:453-476`. The three per-label precision, recall,
and F1 constructions now call the existing `answered_label_metric` instead of
`answered_episode_metric`, passing `answered` (`G`) as the per-label
label-decision population. The existing helper retains the prior:

- formulas and numerator/denominator operands;
- `no_data` before empty-population and `no_answered_predictions` precedence;
- defined/null values and statuses;
- answered scope and checked arithmetic.

It changes only the required unit to `MetricUnit::LabelDecision` while preserving
count `G`. No helper, formula, policy, probability metric, count rule, fixture,
schema, or tolerance was added or changed. Set-level selective metrics continue
through `answered_episode_metric`; aggregate binary metrics continue through
`answered_label_metric` with `G*K`.

## Public proof boundary

### Normal answered M01

For both raw and final results, every label's precision, recall, and F1 asserts
its independently derived operands/status with population `G=2`, unit
`label_decision`, and scope `answered`. The aggregate micro-F1 and Hamming
metrics retain population `G*K=6` label decisions. Exact-match accuracy retains
population two episodes, preventing blanket unit replacement. The original
aggregate `(TP,FP,FN,TN)=(1,1,1,3)`, exact-set, macro-F1, and episode-difference
evidence remains unchanged.

### Undefined answered-empty M02

For both raw and final results and both labels A/B, M02 verifies:

- label identity, zero supports, `TP=FP=FN=0`, and `TN=1`;
- precision, recall, and F1 null with
  `undefined_zero_denominator`;
- population `G=1`, unit `label_decision`, scope `answered`;
- F1's serialized `numerator` and `denominator` fields are each present and equal
  integer zero.

The two explicit operand assertions are outside the shared helper's
zero-denominator status branch, so they prove the literal wire fields rather than
merely passing `0/0` as expected arguments. Existing exact-match, coverage,
Hamming, undefined micro-F1, zero-filled macro-F1, and undefined-class evidence
remains.

### All-abstained and empty-selection M04

For both raw and final all-abstained results, every per-label precision, recall,
and F1 remains null `no_answered_predictions` with population zero,
`label_decision`, and `answered`. Coverage and exact-match metrics remain selected
episode metrics.

For both raw and final empty-selection results, every per-label precision, recall,
and F1 remains null `no_data` with population zero, `label_decision`, and
`answered`. Exact-match retains the episode unit, while aggregate micro-F1 retains
the label-decision unit. These checks preserve applicability/status precedence and
prevent a blanket unit rewrite.

### Persisted report and verified inspection

`multi_label_abstentions_preserve_wire_evidence_in_report_and_inspection` first
reads and validates the persisted public report, then verifies raw and final
per-label precision/recall/F1 metadata as population zero, `label_decision`,
`answered`, with `no_answered_predictions`. It retains set-level episode and
aggregate label-decision guards. The test then performs real verified inspections
for both stored episodes, so the stored-run verification/recomputation path
accepts the same corrected report before inspection evidence is returned.

## Executed focused evidence

Using the existing target directory, every required focused command exited 0.
Each exact filter ran one test with 73 filtered:

- `case_m01`
- `case_m02`
- `case_m04`
- `exhaustive_multi_label`
- `full_numeric_conformance`
- `multi_label_abstentions_preserve_wire_evidence_in_report_and_inspection`

`cargo test --locked --test conformance -- --list` exited 0 and lists the
unchanged 74-test inventory.

Per prompt021, I reused response017's full locked test, warning-denied Clippy,
format, release, and diff evidence. That evidence reports 44 library, 10 CLI, 74
conformance, and zero documentation tests passing; format, locked release, and
diff checks passing; and Clippy containing only the accepted unchanged 17
production plus three duplicate library-test staged diagnostics. No focused
failure justified repeating those broader gates.

All S03, corrected S12 binary64, M14 target, exhaustive/oracle, abstention-wire,
schema, and other numerical evidence accepted in verdict010 and the accepted
portions of verdict015 remains unchanged. This Ready verdict applies to the exact
manifest020 candidate and recommends owner acceptance of T025. It does not start
or assess T026 or later work.
