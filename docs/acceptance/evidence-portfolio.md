# Expanded Jev evidence portfolio

Date: 2026-09-21. **Authority: Decision record.** This records example execution,
not owner acceptance, new gold approval, or a production-performance guarantee.
It is stacked on the existing examples PR to preserve concurrent portfolio work.

## Delivered

Three new projects provide twelve explicit task/cohort integrations. The actual
Validator executable completed **43 evaluations and 31 paired comparisons**:
24/20 for bilingual tasks, 10/5 for moderation, and 9/6 for pairwise choices.
All use published response evidence; **zero fresh inference calls and zero new
human approvals**. Moderation requires an explicit unverified-reference-join
exception, described below. The exception does not relax the core schema.

The extension adds readers, acquisition, task rubrics, an executable replay,
25 tests, and [machine outcomes](../../examples/jev-poc/benchmarks/outcomes.json).
The [operator README](../../examples/jev-poc/benchmarks/README.md) explains inputs,
commands, source pins, and interpretation. No numerical-core, schema, persistence,
third-party application, or task-management changes were required.

## 1. Language and instruction sensitivity

Source: `marcosmartinez/jev-acento@7e007b4c2bd552471b56eff035f9a3df7593f9fe`.
There are 3,200 paired items and 19,200 saved observations across six cells.
Every saved label/state hash was reconciled with the committed metadata table.
The actual frozen prompt and preregistration digests were checked; a stale README
sentence claiming the freeze is pending was not used as execution authority.

| Task | Cases | EN state / EN instructions | ES state / EN instructions | ES state / ES instructions |
|---|---:|---:|---:|---:|
| XNLI entailment | 1,000 | 85.0% | 78.6% | 78.4% |
| PAWS-X paraphrase | 1,000 | 83.4% | 77.2% | 78.8% |
| MASSIVE intents | 600 | 84.5% | 80.83% | 80.17% |
| Belebele answer selection | 600 | 98.17% | 95.17% | 95.67% |

These are pass-0 agreement rates. The second saved pass was also evaluated and
compared for every cell. Maximum absolute pass-to-pass accuracy changes are
0.2, 0.5, 0.5, and 0.33 percentage points, respectively. That is evidence about
these repetitions, not statistical significance or a future stability guarantee.

Under the illustrative maximum-three-percentage-point language-loss guard,
XNLI, PAWS-X, and MASSIVE fail; Belebele is exactly at the allowed boundary.
Changing the instruction language does not uniformly recover the lost agreement.
The next investigation is task-specific language regressions, not a blanket rule
that translated instructions are better or worse.

The source predictions are derived from rounded probability maps, not always
native Choice selections. The Noul tie rule is strict `> 0.5`. Invalid unit sums
remain observations; no loss family is silently normalized or scored on a hidden
subset. Confidence is retained separately when it was actually reported.

## 2. Moderation with incomplete reference labels

Source scores: `ohernandezdev/jevmod@01063f3e927ddbc3c7de923b73f56dcc33034ca9`.
Reference source: `openai/moderation-api-release@f4ab51b5edd3bfbcb349a56324274235b674e0e4`.

The original dataset says that an omitted annotation means **unknown**. Only
765 of its 1,680 rows establish all four inherited combined labels. The remaining
915 rows are not fabricated complete negative sets. Four separate binary tasks
also use their known-label populations (774/860/1,447/994 cases).

The source prediction log provides positional IDs but no original input hashes.
The join follows the pinned preparation script and a separately pinned reference
file; it cannot authenticate the author's original file. Execution therefore
requires `--allow-unverified-moderation-join`. These numbers are **qualified
reference agreement**, not fully authenticated original-workload accuracy. The
combined category meanings also need semantic review before operational use.

On the complete-reference cohort, recorded shipped thresholds give 592/765 exact
sets (77.39%); uniform 0.5 gives 532/765 (69.54%). The per-label changes explain why:

| Label | Change in false negatives | Change in false positives |
|---|---:|---:|
| harassment | −20 | +73 |
| nsfw | −11 | +29 |
| selfharm | −7 | +4 |
| minors | −8 | +7 |

All four recover some missed positives and introduce additional false alarms.
The no-more-misses-and-no-more-false-alarms rubric does not declare a winner.
The probability measurements are unchanged by the threshold policy: both arms
have mean binary Brier 0.0647368 and mean binary log loss 0.214382 over the same
3,060 known label decisions. The separate selfharm task retains one no-model-
decision for a short-input bypass; it does not turn it into a clean prediction.

## 3. Pairwise choices and displayed order

Source: `maybern-tripp-smith/fedjev-bench@7911b5da1dee9947c75784a6962ee5d4fd3d315f`.
262 declared pairs are separated into three reference strata. Both display orders
are mapped back to fixed underlying document IDs before comparison.

| Reference stratum | Cases | Original display order | Reversed display order | Agreement-only accuracy / coverage |
|---|---:|---:|---:|---:|
| Extreme pairs | 40 | 100% | 100% | 100% / 100% |
| Shah sentence pairs | 200 | 74.5% | 85.0% | 83.24% / 89.5% |
| Adjacent statements | 22 | 90.91% | 100% | 100% / 90.91% |

For the 200 sentence pairs, 21 decisions change with display order. Thirty
incorrect decisions remain wrong in both orders. Agreement therefore does not
prove correctness: the agreement-only policy still fails the example's 95%
reference-agreement and 90% coverage requirements.

All source references designate underlying document `a` as the winner. These
numbers do not establish performance on a balanced label population. Extreme
and adjacent references use policy-rate proxies; Shah uses a different source
labeling scheme. No cohort pooling, ranking validation, economic causality, or
financial-action conclusion is made.

The source contains 80 duplicate extreme-pair log entries. Last-entry selection
is explicit and allowed only because repeated decision evidence agrees; changed
input metadata is retained, not called an identical repeat. The displayed-order
comparison was also checked for mirrored document input hashes.

## Verification and limitations

All 25 new Python tests passed locally with a real native executable. The full
replay checks every raw/final confusion count and complete recovery/regression ID
set with separate arithmetic, and checks probability Brier/log loss or explicit
non-applicability/infinity. There are no silent skipped native tests. Every task
also passes selected inspection, relocation, and tamper rejection.

The executable SHA-256 is
`bf33ab4a1ca16ee7cf985ff6d83bacf58344b452580639ddd0ce4c7ad0898dab`.
It was downloaded from the already verified core artifact, not substituted with a
Python implementation. The acquisition archive has SHA-256
`cfcc7da01e5804ddb6e9fd590e735b557f8548495130e5eb3e6d7fddc25d9a9b`.
The permanent workflow independently acquires the pins, builds the Rust executable,
and repeats the new tests and all examples; its result is in PR checks rather than
predeclared here.

Complete runs and untruncated transition sets are generated outside the repository.
The compact committed summary retains their hashes and bounded case previews.
Raw source corpora are not vendored; source data terms remain applicable. No source
texts are copied into routine reports. These results add task-appropriate feedback
examples, not evidence that every path or every Jev primitive is fully validated.
