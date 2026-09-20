# T025 independent numerical-conformance verdict

Verdict: **Revise**.

The frozen candidate is internally consistent and its production behavior passed
the bounded review, but three exact T025 conformance rows and the shared direct
ratio assertion boundary do not yet prove their ratified obligations. These are
test-only coverage defects in `tests/conformance.rs`; I found no required
production, schema, fixture, or tolerance correction.

## Candidate reconciliation and independence

Manifest009 has SHA-256
`8071c565c7c4f544cf3072c993df11ea70d3f80a71cbc85c28fd93a99aea6076`.
Every governed/current hash in that manifest reconciled exactly before review,
including the full governing/evidence chain, both ratified specifications, T025,
coverage.json, production, schemas, tests, fixtures, Cargo files, and toolchain.
After the executed checks, the frozen candidate hashes still match:

| Artifact | SHA-256 |
|---|---|
| `tests/conformance.rs` | `5dd2fb2b0b0c6adcdc6150466f475fe4395a4b60b107ae3ab03a4ed0397bee6d` |
| `src/model/multi_label.rs` | `3f515a0a9122db963b2e01ca14d5474b4836ba3543a48a4e0766a4459eea0b7c` |
| `schemas/v2/report.schema.json` | `805de93d2a50339370e803b7ff1a9a1b8d34c3028b5b21a1a6f428f591733f90` |
| `schemas/v2/inspection.schema.json` | `2c7e064d0adb915b77c558a7ebbfade96f7bcf0eee890b881ced1383f0eedaef` |
| `tests/fixtures/single-label/expected.json` | `ab93c379b9c0df1d0c0fb7aac7a6df5e7748f3f671d55dd69d173e640090b268` |
| `tests/fixtures/multi-label/expected.json` | `213183a46c0f6c011296ef5087b5c44d3852081bdf20eaa80cbb494d3d2c84f2` |

I derived expectations from the ratified formulas and raw cases before using
product results. The single-label enumeration is `1 + 3*4 +
2*(3^2)*(4^2) = 301` populations, including both record orders at length two.
The multi-label enumeration is `8*8 = 64` answered subset pairs plus eight whole
abstentions. Its oracle derives each TP/FP/FN/TN directly from expected/predicted
bit membership and constructs matched/missed/extra lists before calling the
public evaluator. The single-label oracle likewise constructs its matrix and
counts directly from raw `(actual, decision)` rows. Neither oracle imports or
reuses production scoring/count logic.

For F04, the raw rows independently yield:

```text
[[2,1,1,0],
 [0,2,0,0],
 [1,0,1,0]]
```

Thus `N=8,D=5,E=3,U=0,G=8`, accuracy is `5/8`, class F1 wire
operands are `4/7`, `4/5`, and `2/4`, and macro-F1 is `131/210`.
The S03, S07-S10, E01/E04/E07, and M01/M05/M11/M12 mathematical
expectations also agree with the specification: in particular S03 is
`N=4,D=2,E=1,U=1,G=3`; M05/E04 have mean binary Brier `.265` and
mean log loss `(-ln(.8)-ln(.3))/2`; M11 distinguishes categorical `.08`
from marginal `.04`; and M12 preserves different exact-set evidence despite
equal per-label counts.

## Required corrections

### 1. Direct-ratio checks omit required population metadata

Requirement: T025 requires all ratio operands and exact population count, unit,
and scope fields, and verifier prompt010 lines 47-48 repeats that boundary. Each
named row must prove its material fields through the public path.

Location: `tests/conformance.rs:1974-1986`, used throughout the exhaustive
oracles and exact cases; a concrete manifestation is S03 at
`tests/conformance.rs:3823-3870`.

Evidence: `assert_direct_fraction` checks only `value`, `status`, `numerator`, and
`denominator`. S03 invokes it for accuracy, coverage, selective accuracy, and all
three class coverages but never checks those metrics' `population_count`,
`population_unit`, or `population_scope`. The exact filter passes even if, for
example, selected accuracy is serialized with answered scope or the wrong
population count, provided the ratio remains `2/4`. Schema validation cannot
detect a semantically wrong but well-typed count/scope.

Consequence: the candidate does not establish the ratified accounting fields,
despite correct headline arithmetic and passing exhaustive runs.

Smallest correction: extend the test-only ratio assertion boundary to accept and
check the independently expected population count/unit/scope, then supply those
expectations for the exhaustive and exact hard-metric assertions. At minimum,
S03 must discriminate selected episode populations from its `G=3` answered
population and verify all direct ratios material to that row. Do not change
production results or tolerances.

### 2. S12 does not discriminate adjacent bin assignments

Requirement: validator-v1.md line 1170 and T025 S12 require values immediately
below, equal to, and above every threshold or bin boundary to prove the exact
declared policy, including `1` in the last bin.

Location: `tests/conformance.rs:4159-4193`, especially lines 4177-4188.

Exact input and expected result: S12 submits `0`, each
`next_down(i/10), i/10, next_up(i/10)` for `i=1..9`, and `1`. Under
`min(floor(10*h),9)`, the independently expected bin-count vector is
`[2,3,3,3,3,3,3,3,3,3]`; each exact boundary and `next_up` belongs to the
new bin, while `next_down` belongs to the preceding bin.

Actual evidence: for each value the test computes the expected index, but only
asserts that the episode ID occurs somewhere in the family-wide `included_ids`
and that `bins[index].count > 0`. Every bin already has other samples, so moving
an adjacent-binary64 sample across a boundary can leave both assertions true.
The separately executed `case_s12` and the umbrella therefore pass without
proving the bin side of S12.

Consequence: an off-by-one boundary regression can pass the required exact
filter and umbrella.

Smallest correction: assert independently calculated exact per-bin counts and
means (or run isolated boundary records so each result identifies one bin), plus
the existing boundary metadata and last-bin inclusion. Keep the threshold half
of S12 unchanged.

### 3. M02 never checks the required undefined per-label F1 values

Requirement: validator-v1.md line 1194 requires undefined label F1 and
micro-F1 to remain null for answered empty sets, with macro-F1 zero and the
undefined-class status.

Location: `tests/conformance.rs:4441-4466`.

Exact input and expected result: vocabulary `[A,B]`, expected `{}`, predicted
`{}`. Each label has `TP=FP=FN=0,TN=1`; each direct per-label F1 must have null
value, `undefined_zero_denominator`, and direct operands `0/0`. Micro-F1 is
likewise undefined, while macro-F1 is zero with undefined classes `[A,B]`.

Actual evidence: `case_m02` asserts exact match, coverage, Hamming loss,
micro-F1, and macro-F1, but contains no assertion on either element of
`hard["labels"]`. The filter and umbrella pass if production incorrectly emits a
defined zero per-label F1 while retaining the asserted aggregate fields.

Consequence: the exact M02 filter does not prove one of its explicitly named
undefined-value requirements.

Smallest correction: iterate over both per-label results and assert their exact
counts plus F1 null/status/operands and required population metadata. Preserve
the existing aggregate assertions.

### 4. M14 substitutes a prediction-outcome error for the required class target

Requirement: validator-v1.md line 1206 requires four distinct subcases:
label-set target in a single-label task, class target in a multi-label task,
wrong probability kind, and wrong policy kind. Each must return the precise
typed public error and publish no report.

Location: `tests/conformance.rs:4793-4833`, specifically lines 4806-4814.

Exact required input and expected result: the multi-label golden episode must
contain `"expected":{"type":"class","label":"A"}` and the public evaluation
must return `E_CONFIG` with no output directory.

Actual evidence: the second M14 subcase instead supplies the legal multi-label
golden target `{"type":"labels","labels":["A"]}` and puts the class variant in
the submitted prediction outcome. The exact filter passes with `E_CONFIG` and no
report, but it never executes the required class-target input.

Consequence: the compound M14 row is incomplete; the target-side cross-kind
admission path could regress while `case_m14` and the umbrella remain green.

Smallest correction: add or replace that subcase with the exact class target in
the multi-label golden record and assert `E_CONFIG` plus absent output through
the existing public error helper. Keeping a separate wrong-outcome regression is
fine but does not satisfy the target subcase.

## Repaired abstention boundary

The owner-authorized production/schema correction satisfies its scoped contract.
`src/model/multi_label.rs:624-637` emits whole multi-label abstentions with
literal `status: "abstained"` and present null `matched`, `missed`, and `extra`
in raw and final report evidence. Both report and inspection schemas use a closed
multi-label abstention alternative requiring those fields. The focused real
report/verified-inspection test checks field presence, values, reason present/null,
retained marginal/observation evidence, deletion/status/array negatives, and the
old bare form. The real single-label `ABSTAIN` vocabulary test validates an
ordinary label column separate from the typed abstention column and rejects a
matrix with no typed column. The full suite executed both focused tests.

## Executed evidence

- `cargo test --locked --test conformance -- --list`: exit 0; exactly 74 tests,
  every exact filter, both exhaustive filters, F04, and umbrella listed.
- Both exhaustive filters, F04, `full_numeric_conformance`, and every
  representative exact filter required by prompt010 were run separately: each
  exited 0 with one passed and 73 filtered.
- `cargo test --all-features --locked`: exit 0; 44 library, 10 CLI, 74
  conformance, and zero documentation tests passed.
- `cargo clippy --all-targets --all-features --locked -- -D warnings`: exit 101
  with exactly the accepted unchanged 17 production diagnostics and three
  duplicate library-test diagnostics. No new warning appeared.
- The developer's unchanged formatting, locked release-build, and diff-check
  evidence remains applicable; I did not rerun the release build or create a
  fresh target.

The passing commands establish current product behavior but cannot substitute
for the missing discriminating assertions and exact M14 input. A focused
test-only correction in `tests/conformance.rs`, followed by affected exact-filter,
umbrella, and justified regression checks, is sufficient for re-review. T026 and
later work remain outside this verdict.
