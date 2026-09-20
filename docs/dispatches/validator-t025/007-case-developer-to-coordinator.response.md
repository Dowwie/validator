# T025 output 2: S01–S12 and E01–E04/E07 oracle handoff

## Scope and independence

I reconciled every pinned identity in dispatch007 before editing. The owner
prompt001, output1 response006, ratified specification/data model, T025 contract,
accepted fixtures, repaired multi-label serialization, and report/inspection
schemas matched the supplied SHA-256 values.

I read the published public API, schemas, existing conformance setup helpers, and
ratified formulas. I did not read `src/evaluation*`, production metric/count
helpers, or production scoring code to derive an expectation. Each oracle first
constructs its raw synthetic input and independently calculates expected integer,
rational, or analytic values. It then calls `validator::evaluate` through the
public API to compare the actual report or public diagnostic. No production,
schema, fixture, tolerance, governance, plan, index, session-note, acceptance, or
Fizzy artifact was changed.

The only product-facing change is the seventeen exact named test filters added to
`tests/conformance.rs`. They reuse the established public-input helpers; no
generic oracle framework or new fixture was introduced.

## Exact numerical evidence

| Filter | Raw input and independent derivation | Public assertions |
|---|---|---|
| `case_s01` | A→A, B→B, C→C gives the identity `3 × 4` matrix, `N=D=G=3`, `E=U=0`; each direct F1 is `2/2`; macro-F1 is 1. | Matrix, all counts/supports/predicted supports, no abstentions, accuracy `3/3`, all F1 operands, and defined selected-episode macro metadata. |
| `case_s02` | A→B produces matrix cell `[A,B]=1`, `D=0,E=1,U=0`; A has FN 1 and B FP 1. | The complete matrix, A/B count identities, and the one sorted episode’s ID, B raw outcome, and false correctness. |
| `case_s03` | Raw A→A, A→A, B→A, C→C with confidence rejection of the second A gives final A→A, A→abstain, B→A, C→C: `N=4,D=2,E=1,U=1,G=3`; accuracy `2/4`, coverage `3/4`, selective accuracy `2/3`, coverages `1/2,1,1`, macro-F1 `1/2`. | Final counts, all direct ratio operands, class coverages/statuses, defined macro value, and rejection-produced typed abstention. |
| `case_s04` | Schema A/B/C with only A→A yields F1 A=`2/2`, undefined B/C F1, zero-filled macro `(1+0+0)/3`. | Accuracy, B/C null `undefined_zero_denominator`, exact undefined list `[B,C]`, and macro `contains_undefined_classes` at `1/3`. |
| `case_s05` | Two explicit abstentions give `N=U=2,G=D=E=0`. | Coverage and accuracy `0/2`, null selected-only metrics with `no_answered_predictions`, and empty missing-ID evidence. |
| `case_s06` | Empty aligned hard-label artifact gives all hard counts zero. The probability family is absent, so it is inapplicable before population status. | `N=D=E=U=G=0`; hard metrics are null `no_data`; probability and both signal families are null `not_applicable`, with no performance-success claim. |
| `case_s07` | For A and `[.7,.2,.1]`, log loss is `-ln(.7)`, Brier is `(.3²+.2²+.1²)=.14`, argmax accuracy is 1. | All values using only `FIXTURE_ABSOLUTE_TOLERANCE`, plus defined selected/episode population metadata. |
| `case_s08` | For actual B and `[1,0,0]`, true-class log loss is positive infinity without clipping; Brier is `1²+(-1)²=2`. | Null `positive_infinity`/`+infinity` form, Brier 2, and valid serialized report JSON. |
| `case_s09` | Scored choice A, `[.6,.38,.02]`, confidence `.39`: at `.5`, confidence rejects while maximum probability accepts. Both retain losses calculated from `.6`. | Equal raw/probability sections across policies, raw/final separation, raw choice A, confidence `.39`, maximum `.6`, expected log loss, and divergent final outcomes. |
| `case_s10` | Scored choice B with `[.5,.5,0]` is a permitted tied maximum. Schema order makes diagnostic argmax A, but raw correctness is B→B. | Raw accuracy 1, retained B raw outcome, argmax A, disagreement true/count 1, and argmax accuracy 0. |
| `case_s11` | Independently invalid inputs: nonmaximum scored choice; categorical missing key; categorical extra key; sum `1.1`. | Every subcase returns a typed no-report error. The scored-choice profile violation is `E_CONFIG`; all three malformed categorical vectors are `E_PROBABILITY`; every attempted output directory is absent. |
| `case_s12` | Adjacent binary64 values around `.5` are `next_down`, exact, and `next_up`, so rejection applies only below. Confidence bins use `0`, then below/equal/above each `.1`…`.9`, then `1`; bin index is `min(floor(10h),9)`. | Threshold equality acceptance; each input ID remains included; every calculated bin has a count; bin 0 begins at 0, bin 9 ends at 1/inclusive, and signal 1 is in the last bin. |
| `case_e01` | A reference with explicit abstention, categorical `[.7,.2,.1]`, confidence `.8`: hard `D=G=0`, but probability population remains selected. | Raw accuracy/coverage zero, `-ln(.7)`, Brier `.14`, argmax accuracy 1, null chosen probability, and confidence `no_answered_predictions` with the excluded ID. |
| `case_e02` | One answered A and one explicit abstention, each with confidence. Raw answered population has exactly the first row. | Confidence scope `raw_answered`, population 1, first ID included, exactly the abstained second ID excluded, and only the `.8` bin counted. |
| `case_e03` | Explicit abstention cannot satisfy either `reject_below`’s required class outcome or `label_thresholds`’ required answered set. | Both single-label/categorical and multi-label/marginal inputs return typed configuration errors and publish no output directory. |
| `case_e04` | Reference `{A}`, abstention, marginals A=.8/B=.7: no binary hard decisions. Binary Brier mean is `((.8-1)²+(.7-0)²)/2=.265`; mean binary log loss is `(-ln .8-ln .3)/2`. | Coverage 0, zero answered binary counts, literal abstained/null set-difference evidence, finite values/statuses, and selected `2` label-decision population metadata. |
| `case_e07` | Canonical top-level categorical vector `[.24,.19,.56]` sums to `.99`, outside the fixed categorical rule. | Public `E_PROBABILITY`, input exit category 2, absent output directory, and no normalization/promotion/fallback report. |

## Command evidence

The final requested boundary is clean. Each exact named filter is listed and was
run serially, with 1 passed, 0 failed, and 56 filtered out. The final full target
has 57 passed, 0 failed.

| Command | Exit | Result |
|---|---:|---|
| `cargo test --locked --test conformance -- --list` | 0 | 57 tests, 0 benchmarks; every `case_s01`–`case_s12` and `case_e01`–`case_e04`/`case_e07` is listed. |
| `cargo test --locked --test conformance case_s01 -- --nocapture` | 0 | 1 passed, 56 filtered. |
| `cargo test --locked --test conformance case_s02 -- --nocapture` | 0 | 1 passed, 56 filtered. |
| `cargo test --locked --test conformance case_s03 -- --nocapture` | 0 | 1 passed, 56 filtered. |
| `cargo test --locked --test conformance case_s04 -- --nocapture` | 0 | 1 passed, 56 filtered. |
| `cargo test --locked --test conformance case_s05 -- --nocapture` | 0 | 1 passed, 56 filtered. |
| `cargo test --locked --test conformance case_s06 -- --nocapture` | 0 | 1 passed, 56 filtered. |
| `cargo test --locked --test conformance case_s07 -- --nocapture` | 0 | 1 passed, 56 filtered. |
| `cargo test --locked --test conformance case_s08 -- --nocapture` | 0 | 1 passed, 56 filtered. |
| `cargo test --locked --test conformance case_s09 -- --nocapture` | 0 | 1 passed, 56 filtered. |
| `cargo test --locked --test conformance case_s10 -- --nocapture` | 0 | 1 passed, 56 filtered. |
| `cargo test --locked --test conformance case_s11 -- --nocapture` | 0 | 1 passed, 56 filtered. |
| `cargo test --locked --test conformance case_s12 -- --nocapture` | 0 | 1 passed, 56 filtered. |
| `cargo test --locked --test conformance case_e01 -- --nocapture` | 0 | 1 passed, 56 filtered. |
| `cargo test --locked --test conformance case_e02 -- --nocapture` | 0 | 1 passed, 56 filtered. |
| `cargo test --locked --test conformance case_e03 -- --nocapture` | 0 | 1 passed, 56 filtered. |
| `cargo test --locked --test conformance case_e04 -- --nocapture` | 0 | 1 passed, 56 filtered. |
| `cargo test --locked --test conformance case_e07 -- --nocapture` | 0 | 1 passed, 56 filtered. |
| `cargo test --locked --test conformance -- --nocapture` | 0 | 57 passed, 0 failed. |
| `cargo fmt --all -- --check` | 0 | Formatted. |
| `git diff --check` | 0 | No whitespace errors. |

During authoring, two preliminary assertion runs failed and were corrected before
the recorded boundary: S06 had incorrectly treated an absent signal family as
`no_data` instead of required `not_applicable`; S11 had incorrectly called the
scored-choice-profile error `E_PROBABILITY` instead of its public `E_CONFIG`.
Neither was a product discrepancy, tolerance change, or altered expected numeric
value. The final exact reruns above pass.

## Final identities

| Artifact | SHA-256 |
|---|---|
| `tests/conformance.rs` changed | `71f81666afd7792958f57a3cc9b75d86f936b45d073b411976884ce3676e8f2b` |
| `tests/fixtures/single-label/expected.json` unchanged | `ab93c379b9c0df1d0c0fb7aac7a6df5e7748f3f671d55dd69d173e640090b268` |
| `tests/fixtures/multi-label/expected.json` unchanged | `213183a46c0f6c011296ef5087b5c44d3852081bdf20eaa80cbb494d3d2c84f2` |
| `src/model/multi_label.rs` unchanged repair | `3f515a0a9122db963b2e01ca14d5474b4836ba3543a48a4e0766a4459eea0b7c` |
| `schemas/v2/report.schema.json` unchanged repair | `805de93d2a50339370e803b7ff1a9a1b8d34c3028b5b21a1a6f428f591733f90` |
| `schemas/v2/inspection.schema.json` unchanged repair | `2c7e064d0adb915b77c558a7ebbfade96f7bcf0eee890b881ced1383f0eedaef` |

No new fixture was needed. No assigned S/E evidence is missing, and no literal
public-contract discrepancy was observed. M01–M15/M20,
`full_numeric_conformance`, final gates, independent review, T026, and later work
remain unstarted.

Verdict: **T025 local output 2 is complete and ready for coordinator review.**
