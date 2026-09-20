# T025 focused repair recheck

Verdict: **Revise**.

Three of the four findings from verdict010 are corrected on the exact repaired
candidate. The M02 repair instead codifies a pre-existing public population-unit
defect: multi-label per-label hard F1 is emitted and asserted as `episode`, while
the ratified contract requires `label_decision`.

## Candidate reconciliation

Repaired manifest014 has SHA-256
`2e908298bef177b9af9ffdd30f2562ad0b4a4f612dd43ec9a24a1e18a15a8e8e`.
Every listed governing, finding, repair, contract, source, schema, test, fixture,
Cargo, and toolchain hash reconciled exactly before review. In particular:

| Artifact | SHA-256 |
|---|---|
| `tests/conformance.rs` | `40b9d231e3c77416d9c0f6bc93784bd309d45fa1cd6fe8a3eea6327f27ed3cd1` |
| `tests/fixtures/single-label/expected.json` | `ab93c379b9c0df1d0c0fb7aac7a6df5e7748f3f671d55dd69d173e640090b268` |
| `tests/fixtures/multi-label/expected.json` | `213183a46c0f6c011296ef5087b5c44d3852081bdf20eaa80cbb494d3d2c84f2` |
| `src/model/multi_label.rs` | `3f515a0a9122db963b2e01ca14d5474b4836ba3543a48a4e0766a4459eea0b7c` |
| `schemas/v2/report.schema.json` | `805de93d2a50339370e803b7ff1a9a1b8d34c3028b5b21a1a6f428f591733f90` |
| `schemas/v2/inspection.schema.json` | `2c7e064d0adb915b77c558a7ebbfade96f7bcf0eee890b881ced1383f0eedaef` |

I read owner correction012, acknowledgment012, superseding developer
instruction013, and full repair response011. All unchanged evidence from
verdict010 remains settled.

Before finalizing this verdict, I also read owner decision016, SHA-256
`0a129c1fc80602aebd2a133b6cfb945e708a9fcfb33511d43b27d19f9f35ce07`.
It independently identifies the same literal per-label unit contradiction,
preserves this candidate until the bounded finding is saved, and authorizes the
smallest owning-layer correction only after this verdict. That later authorization
does not change the current frozen candidate or this Revise result.

## Focused correction results

### Direct-ratio boundary and S03

The shared `assert_direct_fraction` boundary at
`tests/conformance.rs:1974-1998` now requires and checks independently supplied
`population_count`, `population_unit`, and `population_scope` alongside the
value, status, numerator, and denominator. Every caller supplies those fields.

The exhaustive single-label oracle correctly uses selected episode populations
for accuracy, rates, coverage, and class metrics, and answered episode
populations for selective metrics. The exhaustive multi-label oracle correctly
uses selected episodes for exact-set/coverage metrics and answered
`label_decision` populations for micro-F1 and Hamming loss. S03 at lines
3914-3970 distinguishes accuracy/coverage/class metrics over selected `N=4`
episodes from selective accuracy over answered `G=3` episodes. These focused
filters pass.

The one remaining semantic error is the M02 per-label caller described below;
the helper itself is correctly discriminating and exposes the mismatch once the
ratified expected unit is supplied.

### S12 binary64 boundaries

Verdict010's illustrative uniform count vector was incorrect. Under binary64
`min(floor(10*h),9)`, `next_down(0.9) * 10` rounds to exactly `9`, so the
combined expected vector is `[2,3,3,3,3,3,3,3,2,4]`.

The repaired S12 test fixes the expected bin for all 29 values in a literal
test-only table before any public result is inspected. The table maps boundaries
1 through 8 as `[i-1,i,i]`, maps all three boundary-9 values to bin 9, maps zero
to bin 0, and maps one to bin 9. It then runs 29 isolated public evaluations.
Each requires one count in the fixed expected bin, zero in all other bins,
tolerance-checked mean signal in the occupied bin, null mean signal elsewhere,
and the sole episode ID in `included_ids`. Existing boundary metadata remains,
and the isolated endpoint-one run proves signal one is in inclusive bin 9.
Nothing in the expected table is derived from product output. The threshold half
remains discriminating. `case_s12` passes.

### M02 per-label identities and remaining defect

M02 now asserts both label identities; support, answered support, and predicted
support zero; `TP=FP=FN=0,TN=1`; null `undefined_zero_denominator` F1 with
operands `0/0`; and a count-one answered population. Existing exact-match,
coverage, Hamming, micro-F1, macro-F1, and undefined-class evidence remains.

However, `tests/conformance.rs:4635-4642` expects that per-label F1 population as:

```text
population_count = 1
population_unit  = episode
population_scope = answered
```

The repaired exact filter passes, proving that this is also the current public
output. The ratified report contract at `docs/specs/validator-v1.md:830-837`
states that multi-label per-label metrics use `label_decision`; only multi-label
set metrics use episodes. The data-model contract also requires each metric to
name its evaluated population and unit accurately.

The production cause is bounded: `src/evaluation/multi_label.rs:453-473`
constructs every per-label precision, recall, and F1 through
`answered_episode_metric`, which emits `MetricUnit::Episode` at lines 609-641.
Aggregate answered label metrics already use the separate
`answered_label_metric` and correctly emit `MetricUnit::LabelDecision`.

Requirement: M02 must assert ratified answered population metadata, and the
public report must comply with the ratified per-label unit contract.

Exact input: vocabulary `[A,B]`, expected `{}`, predicted `{}`. For each label,
the answered population contains one label decision with
`TP=FP=FN=0,TN=1`.

Expected: per-label F1 is null `undefined_zero_denominator`, operands `0/0`,
`population_count: 1`, `population_unit: "label_decision"`, and
`population_scope: "answered"`.

Actual: the same value, status, operands, count, and scope, but
`population_unit: "episode"`; `case_m02` passes because the repaired assertion
expects that wrong unit.

Consequence: machine consumers receive a semantically incorrect public unit for
all multi-label per-label hard precision/recall/F1 metrics, and T025's repaired
proof certifies the defect instead of detecting it.

Smallest correction: make the multi-label per-label hard metric constructor emit
`MetricUnit::LabelDecision` while retaining count `G` and answered scope; update
M02's independent F1 expectation from `episode` to `label_decision`; then rerun
M02, the exhaustive multi-label filter, the umbrella, and justified affected
regressions. No schema, fixture, tolerance, probability metric, or aggregate
metric change is needed.

This production correction was outside the completed test-only developer write
boundary, so returning Revise is required rather than silently changing the
expected test to current output. Owner decision016 now authorizes the bounded
follow-up after this saved finding: reuse `answered_label_metric` with per-label
population `G`, correct the M02 expectation, and verify normal, undefined,
all-abstained, empty-selection, and one replay/comparison path without altering
formulas, statuses, schemas, fixtures, or other behavior.

### M14 class target

M14 now supplies a multi-label golden episode whose target is exactly
`{"type":"class","label":"A"}` while the prediction is otherwise a legal
multi-label labels/marginals record. The public path returns `E_CONFIG`, and the
shared error helper verifies that no output directory exists. The label-set
target in a single-label task, wrong probability kind, and wrong policy kind
subcases remain. `case_m14` passes.

## Executed affected evidence

Using the existing target directory, each required focused filter passed one
test with 73 filtered:

- `exhaustive_single_label`
- `exhaustive_multi_label`
- `case_s03`
- `case_s12`
- `case_m02`
- `case_m14`
- `full_numeric_conformance`

`cargo test --locked --test conformance -- --list` exited 0 and still lists 74
tests. The successful M02 run is reproduction evidence for the incorrect public
unit because the exact repaired assertion requires `episode`.

Per prompt015, I reused response011's full-test, Clippy, format, and diff evidence
and response008's release evidence. No new failure justified rerunning those
broader gates. T026 and later work remain outside this verdict.
