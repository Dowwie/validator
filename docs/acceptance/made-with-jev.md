# Made with Jev: measured feedback examples

**Authority: Reference.** Retrospective replay and synthetic software tests, not a production release approval.

All measurements below were produced by the actual Validator executable from case-level data. There were no new inference calls and no fabricated human reference approvals.

## Published-response replay

| Task | Arm | Cases | Correct | Answered | Accuracy | Accuracy among answered |
|---|---|---:|---:|---:|---:|---:|
| banking77 | confidence-0.8 | 3080 | 2141 | 2379 | 69.51% | 90.00% |
| banking77 | recorded | 3080 | 2457 | 3080 | 79.77% | 79.77% |
| email-fresh | enriched | 3300 | 3148 | 3300 | 95.39% | 95.39% |
| email-fresh | framing | 3300 | 3160 | 3300 | 95.76% | 95.76% |
| email-fresh | text | 3300 | 3058 | 3300 | 92.67% | 92.67% |
| email-main | enriched | 5733 | 5617 | 5733 | 97.98% | 97.98% |
| email-main | framing | 5733 | 5655 | 5733 | 98.64% | 98.64% |
| email-main | text | 5733 | 5367 | 5733 | 93.62% | 93.62% |
| email-recent | enriched | 853 | 813 | 853 | 95.31% | 95.31% |
| email-recent | framing | 853 | 806 | 853 | 94.49% | 94.49% |
| email-recent | text | 853 | 782 | 853 | 91.68% | 91.68% |
| spam-binary | criteria | 19528 | 19194 | 19528 | 98.29% | 98.29% |
| spam-binary | plain | 19528 | 18740 | 19528 | 95.96% | 95.96% |

The recent email cohort contains phishing only. Its displayed accuracy is numerically recall; it cannot establish useful precision or a false-positive rate. Binary email uses all 19,528 saved records, not the upstream deduplicated 18,514-message headline sample.

## Paired changes

| Task | Comparison | Recovered | Regressed | Changed decisions |
|---|---|---:|---:|---:|
| banking77 | recorded-vs-confidence-0.8 | 0 | 316 | 701 |
| email-fresh | enriched-vs-framing | 19 | 7 | 26 |
| email-fresh | text-vs-enriched | 93 | 3 | 97 |
| email-main | enriched-vs-framing | 46 | 8 | 55 |
| email-main | text-vs-enriched | 256 | 6 | 263 |
| email-recent | enriched-vs-framing | 0 | 7 | 7 |
| email-recent | text-vs-enriched | 31 | 0 | 31 |
| spam-binary | plain-vs-criteria | 511 | 57 | 568 |

The reference data and vocabulary are byte-identical within each comparison. Existing source labels are retained; `fresh` is a source cohort name, not a new held-out claim.

## Task-rubric findings

### spam-binary

```json
{
  "candidate": "criteria",
  "false_positive_change": -508,
  "false_negative_change": 54,
  "rubric_status": "tradeoff_or_regression",
  "recommendation": "Choose an explicit allowed missed-spam cost before accepting a reduction in false alarms. Do not call this the deduplicated headline population."
}
```

### email-main

```json
[
  {
    "baseline": "text",
    "candidate": "enriched",
    "phishing_missed_change": -243,
    "legitimate_to_phishing_change": 0,
    "rubric_status": "meets_declared_cohort_guards",
    "recommendation": "Preserve the metadata-rich input and test wording changes on a new locked mixed-label corpus; do not trade away recent phishing recall unnoticed."
  },
  {
    "baseline": "enriched",
    "candidate": "framing",
    "phishing_missed_change": 1,
    "legitimate_to_phishing_change": 0,
    "rubric_status": "tradeoff_or_regression",
    "recommendation": "Preserve the metadata-rich input and test wording changes on a new locked mixed-label corpus; do not trade away recent phishing recall unnoticed."
  }
]
```

### email-fresh

```json
[
  {
    "baseline": "text",
    "candidate": "enriched",
    "phishing_missed_change": -78,
    "legitimate_to_phishing_change": 0,
    "rubric_status": "meets_declared_cohort_guards",
    "recommendation": "Preserve the metadata-rich input and test wording changes on a new locked mixed-label corpus; do not trade away recent phishing recall unnoticed."
  },
  {
    "baseline": "enriched",
    "candidate": "framing",
    "phishing_missed_change": -2,
    "legitimate_to_phishing_change": 0,
    "rubric_status": "meets_declared_cohort_guards",
    "recommendation": "Preserve the metadata-rich input and test wording changes on a new locked mixed-label corpus; do not trade away recent phishing recall unnoticed."
  }
]
```

### email-recent

```json
[
  {
    "baseline": "text",
    "candidate": "enriched",
    "phishing_missed_change": -31,
    "legitimate_to_phishing_change": null,
    "rubric_status": "meets_declared_cohort_guards",
    "recommendation": "Preserve the metadata-rich input and test wording changes on a new locked mixed-label corpus; do not trade away recent phishing recall unnoticed."
  },
  {
    "baseline": "enriched",
    "candidate": "framing",
    "phishing_missed_change": 7,
    "legitimate_to_phishing_change": null,
    "rubric_status": "tradeoff_or_regression",
    "recommendation": "Preserve the metadata-rich input and test wording changes on a new locked mixed-label corpus; do not trade away recent phishing recall unnoticed."
  }
]
```

### banking77

```json
{
  "candidate": "confidence-0.8",
  "conditional_accuracy": 0.899957965531736,
  "coverage": 0.7724025974025974,
  "accuracy_at_least_0_95": false,
  "coverage_at_least_0_75": true,
  "rubric_status": "does_not_meet_both_guards",
  "recommendation": "Review high-confidence confusion pairs and refine intent criteria on separate development data; do not equate abstaining with fixing predictions."
}
```

### clean-code

```json
{
  "rubric_status": "semantic_performance_unmeasured",
  "recommendation": "Human-review the eight proposed code cases, capture real per-question answers, and preserve applicability. Do not score final worst-window signals as calibrated marginals."
}
```

## Interpretation limits

- The published email capture manifest names a runner hash different from the sole published runner revision. This replay explicitly acknowledges that gap; it verifies recorded-output accounting, not exact reproduction of the original producing code.
- Email source bodies are not committed upstream. Case IDs, original labels, model outputs and available state digests are preserved, but this replay cannot review the missing message contents or reconstruct every request.
- Banking77 requests and labels were joined to the independently frozen corpus by source ID. 145 recorded vectors sum to 0.99; they remain observations. They are not normalized to force probability-scoring admission. Native confidence remains available for its own gate.
- Where complete scored vectors are admissible, their probability metrics remain separate from hard-label accuracy. Saved zero probabilities for reviewed classes yield an explicit positive_infinity log-loss status; they are not silently clipped. This does not recover hidden pre-rounding model probabilities.
- Upstream ASSAY uses different calibration conventions, including a relaxed sum tolerance and right-closed bins. This replay does not claim to reproduce its ECE by changing Validator's semantics.
- Clean Code Review testing executes the original pure question builder and our output adapter with synthetic replies. It does not execute the complete AI SDK, windowing/comment-stripping runtime or live inference.
- Clean Code Review combines per-window answers using maximum Noul/minimum Score values. Aggregated signals are kept as observations. The four-label task is deliberately smaller than the full 31-finding schema.
- Eight new code-reference proposals require human review. Reported fixture transitions establish software behavior, not Jev accuracy.
- None of these small/retrospective evaluations supplies a deployment guarantee or proves representative production performance.

## Execution and verification

Executed locally on 2026-09-21 with the CI-built Validator from the existing PR.
The binary hash is retained in outcomes.json. This extension changed no Rust
source or canonical schema. Fifteen native evaluations and nine comparisons
completed: thirteen evaluations/eight comparisons use published model responses;
two evaluations/one comparison use explicit code-review fixtures.

The 25 new contract tests and all 55 prior example tests passed locally. The
actual Clean Code Review question builder passed all four applicability profiles.
Independent hard-count/rate assertions cover every raw/final report and complete
transition-ID set. A separate canonical-input arithmetic check reconciles
Brier/log-loss or explicit non-applicability across all 13 single-label reports.
These are scoped proofs, not exhaustive coverage of every application path.

Each completed run retains input snapshots, source observations, preparation
evidence, report hashes and native inspection support. All 16 new artifact-index
entries resolve, and git diff --check passes. Upstream raw corpora are acquired
separately and are not copied into the Git repository.
