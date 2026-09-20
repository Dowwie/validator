# Validator v1 specification

Version: 1.2-draft. Date: 2026-09-18. Authority: Ratified.

This document defines a first-version Rust command-line utility for evaluating
single-label and multi-label classifiers against golden datasets. This revision
incorporates the owner's final classifier-evidence scope and acceptance direction. On 2026-09-18,
the owner authorized the development team to build against this reconciled contract
and its delivery plan. **Must** identifies an approved implementation requirement;
ratification does not assert existing implementation or completed acceptance.

The [shared data model](validator-data-model.md) defines required Rust structures,
ownership, and validated-data boundaries. This document defines their wire contract
and behavior. Neither earlier master document supplies missing implementation rules.

## Format revision

The first release remains the v1 product, but this candidate uses wire
`schema_version: 2` for all inputs, reports, and command results. Explicit task
definitions, typed targets, and tagged probability evidence replace the earlier
single-label-only shapes. Reject other wire versions; do not infer a legacy format
or provide aliases. No implementation or persisted production runs require migration.
Documentation version, wire version, and executable package version are distinct.
The `1.2-draft` amendment completes the unreleased wire version 2 design; it does
not introduce a supported migration from an earlier draft.

## Purpose and product boundary

Validator gives agents repeatable, machine-readable evidence about classification
performance, failure cases, confidence signals, and differences between runs.
It evaluates saved outputs against declared references. It supports development,
regression testing, and evidence review for release decisions.

Reports and CLI results are machine-first. The primary consumer is an agent;
human presentation is not part of the v1 report contract.

The scoring core is classifier-independent. A provider-neutral scored-choice
profile supports outputs with a selected label, a complete probability distribution,
and reported confidence. Domain payloads remain opaque.

V1 includes:

- Strict canonical input validation and ID-based alignment.
- Scalar-class and label-set predictions, categorical distributions or complete
  label marginals, single-label reported confidence, and whole-episode abstention.
- Named scalar and auxiliary observations retained separately from scoring signals,
  including observations associated with an abstention.
- Task-specific confusion accounting, hard-label and probability metrics, and
  explicit rejection or label-selection policies.
- Immutable evaluation artifacts, explicit episode selection, comparisons, and
  full episode inspection on request.

V1 does not include count/multiset targets, object matching, partially labeled
references, per-label abstention, hierarchical partial credit, free-text
similarity scoring, domain-payload validation, annotation adjudication, sample
weighting, top-k metrics, statistical intervals or significance tests, a general
acceptance-gate language, automatic perturbation generation, or a plugin system.
It provides evidence for decisions; it does not certify deployment suitability.
The evaluation subject is the classifier, not a surrounding application or platform.
Deterministic conversion of saved classifier output into a canonical prediction is
input preparation. Scalar observations do not introduce regression or ordinal
evaluation tasks. No general decision-rule interpreter is part of this contract.

## Terms and evidence boundaries

| Term | Meaning |
|---|---|
| Episode | One reference case identified by a stable UUID. |
| Golden dataset | A fixed artifact containing one task definition, label vocabulary, and reference episodes. |
| Prediction | A recorded class, label set, or explicit whole-episode abstention. |
| Observation | A named, typed value returned by the classifier and retained for inspection; not automatically an input to a metric. |
| Scoring signal | A probability vector or reported-confidence field explicitly submitted under this specification's scoring contract. |
| Run | One evaluation of a declared prediction artifact, configuration, and episode population. |
| Comparison | Paired analysis of two runs on an explicitly identified common population. |
| Iteration or pass | A description of surrounding review work; not a separate v1 entity. |

An expected label is a declared reference judgment. Structural validity, model
agreement, or a large dataset does not prove that judgment correct. Human or
independently reviewed reference creation remains the dataset owner's responsibility.

Reference corrections create a new dataset artifact. Repeated predictions do not
create additional independent reference episodes. A targeted conflict set does
not estimate performance on the full workload merely because it came from that
workload. A `held_out` declaration records the owner's claim; Validator cannot
verify absence of information leakage from these files.

## Canonical golden dataset

The transport is UTF-8 JSON. Each dataset declares exactly one task. This complete
single-label dataset illustrates the shared envelope:

```json
{
  "schema_version": 2,
  "task": {
    "kind": "single_label",
    "labels": ["billing", "technical", "sales"]
  },
  "episodes": [
    {
      "id": "01995c20-7d00-7000-8000-000000000001",
      "expected": {"type": "class", "label": "billing"},
      "input": {
        "subject": "Charged twice",
        "message": "I was billed twice for my subscription."
      }
    }
  ]
}
```

| Field | Contract |
|---|---|
| `schema_version` | Required integer `2`; other versions are rejected. |
| `task` | Required object with exactly `kind` and `labels`. |
| `task.kind` | `single_label` or `multi_label`; never inferred from records. |
| `task.labels` | Ordered unique nonblank strings: at least two for single-label; at least one for multi-label. Defines vocabulary order throughout reports. |
| `episodes` | Required array. Empty is valid but supplies no performance evidence. |
| `id` | Required non-nil UUID in lowercase hyphenated form; unique within the dataset. |
| `expected` | Required typed target matching the task, as defined below. |
| `input` | Required JSON value. Its content and shape have no scoring semantics. Explicit JSON null is permitted; absence is not. |

Single-label targets are exactly `{"type":"class","label":"billing"}` with one
declared label. Multi-label targets are exactly
`{"type":"labels","labels":["needs_human","worker_stuck"]}` with unique declared
labels. Input order within a label set has no meaning; emitted sets use vocabulary
order. Duplicates are errors, not a request for deduplication or count evaluation.

A complete multi-label dataset is:

```json
{
  "schema_version": 2,
  "task": {"kind": "multi_label", "labels": ["needs_human", "worker_stuck"]},
  "episodes": [
    {
      "id": "01995c20-7d00-7000-8000-000000000001",
      "expected": {"type": "labels", "labels": ["needs_human"]},
      "input": {"observation": "The worker needs permission to continue."}
    }
  ]
}
```

Multi-label references are complete: every label not listed was judged absent.
An empty set means all labels were judged absent. Unknown or unreviewed labels
must not be encoded as absence. Partial-label annotation is outside this version;
preparation withholds unresolved episodes and records the reasons.
Validator cannot detect an unreviewed negative encoded as a legal omission from
the set; successful validation does not certify reference completeness.

UUIDv7 is the default for newly assigned episode IDs. Existing standard UUID
versions are accepted; the nil and max sentinel UUIDs are rejected. Validator
does not generate, replace, or repair episode IDs. Dataset preparation assigns
IDs once and persists them in source records or a reusable source-ID mapping.
Regenerating IDs during every import is prohibited.

Class matching preserves case, whitespace, and Unicode code points. No trimming,
normalization, synonym mapping, or schema inference occurs. Unknown fields outside
`input` are errors. JSON syntax and duplicate-key checks apply inside `input`, but
no domain-required-field, type, meaning, or relevance checks occur there.

All episode-specific data supplied to the classifier can remain in its opaque
payload. The payload must not be copied into routine logs or summaries.
Changing it changes dataset identity even when the episode ID remains stable.

`UNCERTAIN`, `Other`, and `ABSTAIN` are ordinary classes when explicitly declared
and assigned as settled reference judgments. An unresolved, disputed, or withheld
reference is different: it is not a valid scored episode in this format. Preserve
such records in the source/reference-review artifact. A transformation must list
withheld IDs and reasons in its preparation evidence; it must not silently drop
them or fabricate a class. Validator rejects missing/null targets, wrong target
variants, duplicates, and undeclared labels. It cannot detect an unresolved judgment encoded as
a legal class string. Successful validation does not attest that references are
settled; that remains the preparation author's responsibility. Neither the spelling
of a valid class nor the contents of opaque input imply review status.

## Canonical prediction artifact

The envelope has these fields. Every row corresponds to one episode.

| Field | Contract |
|---|---|
| `schema_version` | Required integer `2`. |
| `dataset_sha256` | Required lowercase SHA-256 hex digest of the exact golden file bytes. |
| `sources` | Required object mapping nonblank source IDs to source definitions. |
| `predictions` | Required array of prediction records. |

Each source definition contains required `kind` (`scored_choice` or `classifier`), `model`
(nonblank recorded model identifier), and `configuration` (an opaque JSON object
containing the exact available prediction configuration). It can also contain
`question_id` (required for `scored_choice`) and `evidence` (an array of local file
paths containing raw outputs, transformation provenance, or reference-review
evidence). For a newly submitted artifact, paths resolve relative to the prediction
file. Evidence files must exist and are copied and hashed in the run. Replay uses
the stored evidence bindings defined below, never the original source paths.
No remote content is fetched.

Two additional optional source fields are defined:

- `observation_definitions`: an object keyed by nonblank observation names. Each
  definition contains exactly `kind`, nonblank `description`, and optional nonblank
  `question_id`. Kinds are defined below. Definitions identify meanings, not gold
  targets. Record full criteria, scalar scales, and option meanings in the source's
  opaque `configuration`; names alone do not define them.
- `preparation`: an object with exactly nonblank `method`, nonblank `version`,
  opaque object `configuration`, and nonempty `evidence_indices`. The indices are
  unique nonnegative integers into this source's `evidence` array. Together the
  referenced files must preserve the deterministic transformation, raw source
  artifacts, and preparation receipt. Require this field when preparation derives
  an outcome from a scalar or changes numerical values beyond Validator's defined
  near-unit normalization. Configuration records the mapping and numerical rule;
  the receipt records input/output identities, counts, and affected episode IDs.

The validator checks these shapes, index bindings, files, and hashes. It does not
execute preparation code or certify the truth of its provenance claims. Pure field
renaming and explicit label/ID mapping still require the preparation evidence
described in the dataset preparation contract, but not this additional descriptor.

Source IDs are artifact-local names, compared exactly. Every prediction must
reference a definition through `source_id`. One-source artifacts use the same
ID on every row. Assembled artifacts declare each originating model/question
configuration separately and retain the corresponding source ID on each row.
Do not repeat source definitions inside individual prediction records or assign
retained outputs to a revised configuration that did not produce them.
An empty prediction artifact can have an empty `sources` object. Unused source
definitions are allowed but contribute zero episodes to source counts.

The recorded model identifier must be what the producer can establish. An alias
must not be represented as a resolved model version. Missing details in supplied
configuration or evidence must not be invented; the report does not attest to
provenance completeness. Credentials must not appear in any artifact.

A prediction record has required `id`, `source_id`, and `outcome`. It can additionally
contain `probabilities`, `confidence`, and `observations`. Outcome and evidence are
separate fields: an outcome is required even when observations are present.
Missing outcomes are invalid, not implicit abstentions. For example:

```json
{
  "id": "01995c20-7d00-7000-8000-000000000001",
  "source_id": "initial",
  "outcome": {"type": "class", "label": "billing"},
  "probabilities": {
    "kind": "categorical",
    "values": {"billing": 0.6, "technical": 0.38, "sales": 0.02}
  },
  "confidence": 0.39
}
```

An explicit abstention uses a typed outcome, with optional nonblank reason.
It is legal for either task kind. This independent example references a source
of kind `classifier`:

```json
{
  "id": "01995c20-7d00-7000-8000-000000000001",
  "source_id": "review_policy",
  "outcome": {"type": "abstention", "reason": "manual_review_required"}
}
```

Rules:

- `id` follows the golden ID contract. Duplicate IDs are errors even for identical rows.
- `source_id` is required and must exactly match a key in `sources`. Missing or
  unknown source references are `E_PROVENANCE`; source-kind rules apply per row.
- A class outcome contains exactly `type` and a valid `label`. An abstention
  contains `type` and optionally `reason`. A multi-label answered outcome contains
  exactly `type: "labels"` and `labels`, following the target-set rules.
  Reject outcomes incompatible with the task. Unknown fields are rejected.
- `probabilities` contains exactly `kind` and `values`. `kind: "categorical"`
  is legal only for single-label; `kind: "label_marginals"` only for multi-label.
  The values map has exactly the task label keys with finite numbers in `[0,1]`.
  Booleans and numeric strings are invalid. Missing marginal keys never mean zero.
- `confidence`, when present, is a finite number in `[0,1]`. Its source is the
  prediction producer. It is never used as a sample weight. It is supported only
  for single-label outputs; reject it for multi-label rather than invent set-level
  or per-label confidence semantics.
- Probability and confidence fields can accompany answered or explicitly abstained
  outcomes of kind `classifier`. Abstention does not erase evidence. Confidence
  attached to an abstention has no recorded-choice correctness target. The stricter
  `scored_choice` profile still requires a class outcome.
- Within one prediction artifact, probabilities are either present on every row
  or absent on every row. Confidence follows the same independent all-or-none
  rule, including across different sources. Empty submitted artifacts contain
  neither signal family. This input rule does not erase signal applicability when
  restricting an existing run for comparison. Partly scored artifacts are rejected
  rather than evaluated on a silently reduced population.

### Retained classifier observations

`observations` is an optional object keyed by observation name. Every entry must
match a definition in its referenced source, including its kind. A definition may
be unused; an observation may be absent on individual rows because it is not a
scoring family. Omission means unavailable, never zero, false, or abstention.
Reject unknown names, unknown kinds, and extra fields. The closed value shapes are:

| Kind | Exact value fields and checks |
|---|---|
| `scalar` | `kind`, `value`: finite binary64 number; no implicit range, order, or class interpretation. |
| `bernoulli` | `kind`, `value`: finite number in `[0,1]`, representing reported probability of the named condition. |
| `reported_confidence` | `kind`, `value`: finite number in `[0,1]`; does not acquire correctness or calibration semantics. |
| `categorical` | `kind`, `values`: nonempty map of nonblank option strings to finite numbers in `[0,1]`. Preserve values without normalization or a sum-to-one admission check. |
| `label_marginals` | `kind`, `values`: nonempty map of nonblank label strings to finite numbers in `[0,1]`; no sum constraint. |

Observation option keys describe the source question and need not equal the task
vocabulary. They are not validated scoring vectors. In particular, a reported
categorical vector with sum `0.99` can be retained here but cannot be submitted
unchanged as top-level scoring `probabilities`. Malformed or nonfinite raw values
belong in attached original response files, not typed observations.

For a source defining these four observations, a score-based classifier row is:

```json
{
  "id": "01995c20-7d00-7000-8000-000000000001",
  "source_id": "identity_score",
  "outcome": {"type": "class", "label": "UNCERTAIN"},
  "observations": {
    "identity_score": {"kind": "scalar", "value": 1.33},
    "identity_distribution": {
      "kind": "categorical",
      "values": {"NO_MATCH": 0.24, "UNCERTAIN": 0.19, "MATCH": 0.57}
    },
    "same_capability": {"kind": "bernoulli", "value": 0.91},
    "material_difference": {"kind": "bernoulli", "value": 0.12}
  }
}
```

This row requires a separate single-label dataset with the three illustrated
classes, source definitions, and a preparation descriptor for the scalar-to-class
mapping. The recorded outcome is not replaced with the distribution's maximum.
The example omits scoring signals intentionally; observation presence alone does
not activate probability metrics. To score a class distribution, preparation must
explicitly supply a conforming top-level `probabilities` vector.

Preserve returned scalar values even when they disagree with a mean calculated
from displayed probabilities. Validator does not know the source's scalar formula
and does not enforce a scalar/vector consistency equation. Auxiliary observations
have no reference targets in this run and receive no accuracy, loss, or calibration
metrics. A separately labeled auxiliary classification task requires its own run.
Arbitrary decision scores and normalized class weights must not be relabeled as
probabilities; retaining them does not establish probabilistic meaning.

### Recorded outcomes and scoring signals

The general classifier format permits a recorded class that differs from argmax,
or a label set that differs from a later configured threshold selection. The
evaluator preserves recorded outcomes in the raw family; only the explicit
decision policy can produce a different final outcome.

For the multi-label dataset above, a prediction row can be:

```json
{
  "id": "01995c20-7d00-7000-8000-000000000001",
  "source_id": "initial",
  "outcome": {"type": "labels", "labels": ["needs_human", "worker_stuck"]},
  "probabilities": {
    "kind": "label_marginals",
    "values": {"needs_human": 0.9, "worker_stuck": 0.8}
  }
}
```

Each marginal is the probability that its label applies. Their sum need not be
one; they are not a joint distribution over sets. No statistical independence
assumption is needed for per-label evaluation. The example's sum of 1.7 is valid.

### Scored-choice profile

The `scored_choice` source kind is legal only for single-label tasks. It declares
an output contract, not a model or provider.
Any producer that satisfies this profile can use it. Other producers use
`classifier`, including those whose recorded choice can differ from argmax.

For the selected question, map the producer's selected label to `outcome.label`,
copy the complete probability map to categorical `probabilities.values`, and copy reported confidence
to `confidence` unchanged. Set the referenced source's
`kind` to `scored_choice`, record `question_id`, and preserve the available model and
question configuration. Saved raw responses can be attached through that source's
`evidence` array.

For this source kind, every referencing record must contain all three values
and a class outcome. The choice must be among the maximum-probability options;
any exact tied maximum is valid. A contradiction is an input error. Response-map
iteration order does not determine probability-to-class correspondence.

Reported confidence is distinct from `max_probability`. Validator does not
reimplement the producer's confidence computation. A low-confidence class is still
a prediction; rejection occurs only through an explicitly selected policy.

One run evaluates one classification task. Each scored-choice source identifies one selected
question; revisions of that question can be represented by separate sources.
Independent questions with different class meanings require separate runs.

## Evaluation configuration

An example complete minimal configuration is:

```json
{
  "schema_version": 2,
  "population": "All episodes in the billing development reference set",
  "role": "development",
  "decision": {"type": "as_recorded"}
}
```

Required fields are `schema_version: 2`, a nonblank `population` description,
`role` (`development` or `held_out`), and `decision`. Optional fields are
`episode_ids` (an array of unique UUIDs) and `parent_run_id` (a UUID identifying a
previous run). Unknown fields are errors. A parent ID is descriptive lineage,
not an instruction to inherit predictions or combine populations.

Omitting `episode_ids` selects all golden episodes. An explicit empty array
selects none. Every selected ID must exist in gold. Validate the entire supplied
dataset and prediction artifact before scoring. Prediction IDs must equal the
selected golden IDs exactly: missing, extra, or duplicate records invalidate the
run. To score a subset of a larger prediction artifact, prepare an explicit
subset artifact bound to the same golden file. Selection never silently removes
malformed records. Unselected golden IDs and selected counts remain visible.

The decision policy is a closed task-specific choice:

- `{"type":"as_recorded"}` preserves the recorded outcome in either task kind.
- `{"type":"reject_below","signal":"confidence","minimum":0.8}` is single-label
  only. It rejects class
  predictions with a signal below `minimum`. `signal` can also be
  `max_probability`; `minimum` must be finite and in `[0,1]`. Equality is accepted.
  Every record must have a class outcome and the required signal. Missing signals
  and explicit-abstention inputs with this policy are configuration/input errors.
- `{"type":"label_thresholds","thresholds":{"needs_human":0.8,"worker_stuck":0.85}}`
  is multi-label only. The map must contain exactly every declared label, each
  with a finite threshold in `[0,1]`. Every row must have an answered label set and
  complete marginals. The final set contains exactly labels with probability
  greater than or equal to their threshold; it can be empty. Below-threshold
  labels are negative decisions, not abstentions. Retain original sets and
  probabilities. No default thresholds, partial maps, or policy chaining exist.

Maximum probability is computed from the working probability vector. For a
general classifier whose choice is not argmax, it measures distribution
concentration, not the probability assigned to that recorded choice. The report
exposes the disagreement and the selected policy explicitly.

Representative, challenge, and conflict-focused populations must be named
accurately. V1 does not infer representativeness, combine runs into one population,
or convert a targeted review into a full-dataset result. Revised references must
be saved to a new file. Existing run artifacts remain unchanged.

## Validation and numerical rules

Validation proceeds through JSON parsing, schema/configuration checks, record
checks, digest verification, ID alignment, decision derivation, then scoring.
No successful report may result from a fatal input or arithmetic error. Multiple
diagnostics can be collected; invalid rows must not be discarded and scored around.

There is no permissive mode, automatic fallback, or row-level salvage. Top-level
scoring probabilities and confidence either satisfy their complete canonical
contracts or invalidate the submitted artifact. Retained observations are checked
only against their explicitly weaker observation contracts; they cannot flow into
metrics or threshold policies. This is a type boundary, not an error-recovery mode.

For limited-precision source output, preparation can submit a hard-label artifact
with all original numerical observations retained and probability metrics absent.
Alternatively, it can explicitly produce canonical scoring probabilities using a
documented, deterministic source-precision interpretation, with the required
`preparation` descriptor and evidence. Every row must follow the same declared rule
for its source; numerical changes, affected IDs, and probability-family omissions
must appear in the receipt. Validator applies its unchanged strict checks to the
prepared vector. It never infers a rounding tolerance, widens one to obtain better
results, or treats a parse/numerical failure as an `UNCERTAIN` label or abstention.
Failure to obtain a native classification outcome remains an explicit preparation
failure; do not drop the reference episode to manufacture a successful full run.

- Reject duplicate serialized JSON keys before a parser overwrites them, including
  keys in opaque payloads and configuration.
- Use exact nonnegative integer counts. Parse scoring probabilities, confidence,
  thresholds, and typed observation numbers to IEEE 754 binary64 with round-to-nearest, ties-to-even, and use
  binary64 working arithmetic for normalization, decisions, and metrics. Do not
  make decisions with a different hidden precision. Detect count overflow and
  nonfinite arithmetic failures. Opaque payload/configuration numbers must retain
  their JSON numeric values during inspection, including integers beyond binary64's
  exact-integer range; they are not scoring values.
- For categorical distributions only, sum probabilities in vocabulary order. Require positive sum `s` and
  `abs(s - 1) <= 1e-9`; use `q[c] = p[c] / s` for accepted vectors. Preserve source
  values. Record the number of normalized rows and maximum absolute sum error.
  This tolerance is a v1 input rule, not a claim about TypeSafe serialization.
- Never sum-normalize label marginals. Their working values equal their parsed
  original values; report categorical normalization as not applicable for them.
- Do not clip probabilities, insert missing classes, convert logits, or manufacture
  distributions from labels or arbitrary scores.
- Process episodes in canonical UUID byte order. Arrays that refer to classes use
  declared class order. This makes record reordering independent of calculations.
- Finite fixture comparisons use `abs(actual - expected) <= 1e-12 + 1e-10 * abs(expected)`.
  This tolerance does not alter ties, thresholds, or label comparisons.

## Counts and metrics

Keep raw and final outcomes separate. Raw metrics score the recorded outcomes;
final metrics score outcomes after the configured decision policy. With
`as_recorded`, the two are equal. Probability metrics evaluate every selected
probability record, including submitted abstentions and predictions subsequently
rejected by a threshold. Retained observations are never scored implicitly.

The following class-matrix formulas apply only to `single_label`. Multi-label
accounting and probability formulas are defined separately below. Every metric
uses the population and unit specified in the shared data model.

### Single-label hard decisions

For each hard-decision family, use a `K × (K + 1)` count matrix: actual classes
are rows, declared classes are the first `K` columns, and typed abstention is the
last column. Keep the abstention column even when zero. Each episode increments
one cell exactly once. A wrong A-to-B decision is one error, not two episodes.

Define `N` as selected episodes, `D` as correct classes, `E` as wrong classes,
`U` as abstentions, and `G = D + E`. For each class, support includes abstentions:

```text
support[c]           = sum(matrix[c, all columns])
predicted_support[c] = sum(matrix[all rows, c])
TP[c]                = matrix[c, c]
FN[c]                = support[c] - TP[c]
FP[c]                = predicted_support[c] - TP[c]
precision[c]         = TP[c] / (TP[c] + FP[c])
recall[c]            = TP[c] / (TP[c] + FN[c])
F1[c]                = 2*TP[c] / (2*TP[c] + FP[c] + FN[c])
class_coverage[c]    = (support[c] - matrix[c, abstention]) / support[c]
accuracy            = D / N
wrong_class_rate    = E / N
abstention_rate     = U / N
coverage            = G / N
selective_accuracy  = D / G
selective_risk      = E / G
macro_f1            = sum(F1[c] for all declared classes) / K
```

Report all quantities above, the matrix, and supports. Always present selective
accuracy with coverage. Abstention is not a correct class and is not a wrong-class
event; both rates are separately visible. Specific off-diagonal cells support
domain interpretations such as false merges without hardcoding class names.

Require these accounting identities:

```text
N = D + E + U = sum(matrix) = sum(support)
G = sum(predicted_support)
sum(TP) = D
sum(FP) = E
sum(FN) = E + U
accuracy = coverage * selective_accuracy   when N > 0 and G > 0
```

### Undefined values

Each scalar metric is an object with `value` and `status`. A missing denominator
must not appear as measured success or failure.

| Status | Value and behavior |
|---|---|
| `defined` | Finite number. |
| `undefined_zero_denominator` | `null`; include numerator and denominator for ratios. |
| `contains_undefined_classes` | Macro-F1 numeric value using zero for undefined class F1 terms; include affected classes. |
| `no_data` | `null`; no episodes in this applicable population. |
| `no_answered_predictions` | `null`; answered-only metric with `G = 0` and `N > 0`. |
| `not_applicable` | `null`; required signal family is absent. |
| `positive_infinity` | `null` plus `special_value: "+infinity"`; mathematically infinite log loss. |

Determine applicability before empty-population status. For nonempty populations,
zero-denominator class metrics are null. Only the explicitly named fixed-schema
macro-F1 average zero-fills missing class F1 values; its status preserves that fact.
Use the direct F1 formula: an observed, always-missed class has defined F1 zero
even if its precision is undefined. Empty populations do not zero-fill aggregates.

### Single-label probability metrics and confidence diagnostics

For a complete distribution, calculate natural-log loss and full multiclass Brier:

```text
log_loss = -sum(ln(q[i, expected[i]])) / N
brier_score = sum(sum((q[i,c] - indicator(expected[i] = c))^2 for c)) / N
```

A zero probability assigned to a true class yields infinite log loss. Brier uses
the sum across classes with no division by `K` or `2`, including binary tasks;
its range is `[0,2]`. No output may contain JSON NaN or Infinity tokens.

When probabilities exist, also derive a diagnostic argmax, breaking exact ties
by earlier schema position. Report `argmax_accuracy` and the count of recorded
choices differing from that diagnostic argmax. A different tied selection remains
valid and is not replaced. Report `max_probability` and `chosen_probability`
separately in episode evidence.
Argmax accuracy uses all `N` probability-bearing episodes, including abstentions.
The disagreement count uses only raw answered episodes and reports their count;
an abstention has null `chosen_probability` and null choice/argmax disagreement.

Produce ten fixed equal-width bins for each available signal:

- For `max_probability`, pair the signal with diagnostic argmax correctness.
- For `confidence`, pair the reported signal with recorded-choice correctness on
  raw answered episodes only. Submitted abstentions have no chosen class and are
  excluded from these bins, with their IDs/count disclosed. Threshold-rejected
  classes remain in these bins because they had a recorded class.

Assign score `h` to bin `min(floor(10*h), 9)`. Bins are left-closed/right-open,
except the last includes `1`. Report boundaries, count, correct count, mean signal,
and empirical accuracy. Empty-bin means and accuracy are null. Maximum-probability
bins use all selected records; confidence bins use raw answered records. Each
family exposes `population_scope` (`selected` or `raw_answered`), `population_count`,
and included/excluded IDs. With confidence present and `N>0` but no raw answered
rows, confidence-bin availability is `no_answered_predictions`; with `N=0` it is
`no_data`. The two signals must not be mixed.
For mixed-source runs, bins describe the supplied mixture; they do not establish
a common confidence definition or calibration across its contributing models.

Top-label ECE is `sum(count[b]/N * abs(accuracy[b] - mean_signal[b]))` over the
maximum-probability bins. It is a bin-dependent diagnostic, not classwise calibration
or a release verdict. Do not calculate or label an ECE of vendor confidence as
probability calibration. Reported confidence, empirical accuracy, and uncertainty
in an estimate remain distinct. V1 makes no statistical-confidence claims.

### Multi-label hard decisions

Compute raw and final families separately. Define `N` as selected episodes, `U`
as whole-episode abstentions, `G=N-U` as answered episodes, `D` as answered exact
set matches, and `E=G-D` as answered nonmatches. An empty predicted set is answered;
an abstention is not. Report these totals and the following metrics:

```text
exact_match_accuracy           = D / N
wrong_set_rate                 = E / N
coverage                       = G / N
abstention_rate                = U / N
selective_exact_match_accuracy = D / G
selective_risk                 = E / G
```

Per-label binary counts use only the `G` answered episodes. For expected set `Y`
and predicted set `P`, label `c` contributes to exactly one of TP (`c` in both),
FN (`c` only in `Y`), FP (`c` only in `P`), or TN (`c` in neither). Whole-episode
abstentions contribute to none of those four counts; they are never imputed as
negative predictions. Report both `support` over all `N` references and
`answered_support=TP+FN`, plus `predicted_support=TP+FP`.

Require `TP[c]+FP[c]+FN[c]+TN[c]=G` for every label and total binary counts `G*K`.
Report each label's precision, recall, and F1 using the single-label ratio formulas
on these answered-only counts. Report aggregate metrics with these explicit names:

```text
answered_micro_precision = sum(TP) / (sum(TP) + sum(FP))
answered_micro_recall    = sum(TP) / (sum(TP) + sum(FN))
answered_micro_f1        = 2*sum(TP) / (2*sum(TP) + sum(FP) + sum(FN))
answered_macro_f1        = sum(per-label F1, zero-fill undefined) / K
answered_hamming_loss    = (sum(FP) + sum(FN)) / (G*K)
```

For `G>0`, undefined per-label ratios are null. The named macro average zero-fills
undefined label F1 and uses `contains_undefined_classes`, identifying affected
labels; this status name is shared across task kinds. When `N>0,G=0`, answered-only
metrics use `no_answered_predictions`, not zero-filled values. When `N=0`, applicable
metrics use `no_data`. Exact-set accuracy counts abstentions as not correct and
must accompany answered-only metrics and coverage. It satisfies
`exact_match_accuracy=coverage*selective_exact_match_accuracy` when `G>0`.

Use per-label count objects, not a class-to-class matrix with a null category.
For each answered episode, emit matched, missed, and extra sets in vocabulary
order for both raw and final families. Their cardinalities explain the per-label
counts. For abstentions those sets are null with `status: "abstained"`.
Do not infer a substitution between a missed label and an extra label.
Instance-averaged F1, Jaccard, ranking metrics, and arbitrary metric selection are
outside this version; exact-set and Hamming metrics supply the initial set-level
and label-decision views.

### Multi-label probability metrics and diagnostics

Complete marginals are scored on all `N` selected episodes, independently of the
label-selection thresholds, including submitted abstentions. The artifact-wide
completeness rule still applies. Do not silently select probability-bearing rows.

For each label, let `y[i,c]` be 1 when present in the complete reference set and
0 otherwise. Use the following definitions:

```text
binary_log_loss[c] = sum(loss[i,c]) / N
loss[i,c]         = -ln(p[i,c])       when y[i,c] = 1
                  = -ln(1-p[i,c])    when y[i,c] = 0
binary_brier[c]   = sum((p[i,c] - y[i,c])^2) / N
mean_binary_log_loss = sum(binary_log_loss[c]) / K
mean_binary_brier    = sum(binary_brier[c]) / K
```

Evaluate log loss using the observed-label branch, never `0*ln(0)`. Implement the
negative branch with the numerically stable `ln_1p(-p)` operation. A zero assigned
probability for the observed binary outcome yields positive infinity, encoded by
the shared status rule. An infinite per-label loss makes the mean infinite.
No clipping is allowed. `mean_binary_brier` lies in `[0,1]`; it is deliberately
not the single-label summed Brier, whose binary-task range is `[0,2]`.

Per-label metrics have population `N` label decisions; the overall means have
population `N*K` label decisions. They are means of marginal scores, not a claimed
joint set likelihood or a probability that the whole set is correct.

For each label, produce ten fixed probability bins using the same boundary rule
as single-label bins. Each contains boundaries, count, `positive_count`,
`mean_probability`, and `observed_positive_rate`; empty means/rates are null.
Pair each marginal with reference presence, not thresholded-prediction correctness.
Do not aggregate different labels into one calibration curve. No multi-label
argmax, maximum-probability confidence, reported-confidence diagnostics, or ECE
summary is defined in this version. Source-mixture interpretation limits still apply.

## CLI and run artifacts

The binary is named `validator`. The following are the proposed command contracts,
not commands available in the existing scaffold.

```sh
validator check --dataset golden.json --predictions predictions.json --config evaluation.json
validator evaluate --dataset golden.json --predictions predictions.json --config evaluation.json --out runs/run-001
validator compare --baseline runs/run-001 --candidate runs/run-002 --out comparisons/001-002
validator inspect --run runs/run-001 --episode 01995c20-7d00-7000-8000-000000000001
```

- `check` performs the full structural, signal, policy, hash, and alignment checks
  without publishing a run or calculating performance metrics.
- `evaluate` validates, scores, and creates a new immutable run directory.
- `compare` creates a new immutable comparison directory under the rules below.
- `inspect` verifies the run and shows the requested selected episode's full input,
  expected target, original prediction, final decision, and recorded configuration.
  It is the explicit operation that reveals payload contents.
- Every operational command emits exactly one versioned JSON document to stdout.
  Progress, if any, uses stderr. Errors emit one structured error document, not
  a partial successful result. No text-output mode is required.
- `--help` and `--version` are required. Unknown flags and unsupported settings fail.

Successful runs contain the following files:

| File | Contents |
|---|---|
| `golden.json` | Exact submitted dataset bytes, including opaque episode inputs. |
| `predictions.json` | Exact submitted prediction artifact bytes. |
| `config.json` | Exact submitted evaluation configuration bytes. |
| `evidence/` | Optional exact local evidence-file copies, assigned unique filenames. |
| `report.json` | Authoritative structured result. |

The report uses integer `schema_version: 2` and includes:

- A newly generated UUIDv7 `run_id`, UTC RFC 3339 creation timestamp, Validator
  version, and this specification version.
- Exact-byte SHA-256 digests and stored relative paths for dataset, predictions,
  supplied configuration, and each evidence file; also the report's effective
  decision, selection, numerical, and binning settings.
- Recorded prediction sources/configurations, declared population and role, optional
  parent ID, task kind and full vocabulary order, selected IDs, and unselected IDs.
- Integrity counts, missing-ID counts (zero on success), signal availability,
  normalization diagnostics, task-specific raw/final accounting and metrics,
  probability metrics, and applicable signal diagnostics.
- A sorted `episodes` array containing each selected ID, `source_id`, expected target, recorded
  outcome, final outcome, raw/final correctness, rejection reason, and available
  original/working probability evidence and task-specific diagnostics. Single-label
  evidence includes argmax, maximum/chosen probability, and reported confidence
  when available. Multi-label evidence includes the raw/final set differences.
  Both variants include an `observations` object (empty when absent), preserving
  the named typed values independently of the outcome. It does not contain raw
  provider payloads. Source definitions and preparation bindings supply meaning.
  Correctness means class equality or exact set equality; it is false for abstention.
- `status: "complete"`. Completion denotes successful evaluation, not acceptable
  classification performance. V1 emits no model-acceptance verdict.

Metric keys use the names defined in this document. `raw` and `final` distinguish
the two hard-decision families. Inapplicable required metric families remain
present with status rather than disappearing. Counts, metric populations, statuses,
denominators, and episode references must be explicit fields; an agent must not
parse prose to determine them. Preserve full numerical precision in serialization.
Reports omit raw `input` and arbitrary evidence content; the protected snapshots
remain available for explicit inspection.

### Machine interface

Implementation must publish versioned JSON Schemas for the golden dataset,
predictions, configuration, run report, comparison, inspection, check result,
command receipt, and error result. The schemas are the serialization contract;
examples are not substitutes for them. Schema conformance is tested alongside
numerical correctness. Breaking changes require a new `schema_version`.

Every stdout document includes `schema_version: 2`, `kind`, and `status`.
`kind` is `check`, `evaluation`, `comparison`, `inspection`, or `error`;
`status` is `complete` or `error`. Successful `evaluate` and `compare` commands
return a receipt with `run_id` or `comparison_id`, respectively, plus `result_path`
and `result_sha256`. `result_path` is the absolute path of the completed result
file: `report.json` for evaluation or `comparison.json` for comparison.
`result_sha256` hashes that file's exact bytes, never the directory or a collection
of files. The receipt's `kind` matches the operation. A consumer can open and hash
`result_path` directly; no filename inference is needed.
The report file contains the complete result. `check` returns integrity
counts and selected IDs; `inspect` returns the selected episode and associated
prediction/configuration evidence directly. Help and version output are exempt.

Run reports use these required top-level fields:

| Field | Contents |
|---|---|
| `schema_version` | Report format integer. |
| `kind` | `evaluation`. |
| `status` | `complete`. |
| `identity` | `run_id`, `created_at`, `validator_version`, `specification_version`, optional `parent_run_id`. |
| `artifacts` | Snapshot manifest: file entries with `kind`, stored relative `path`, and `sha256`; evidence entries additionally contain the bindings defined below. |
| `sources` | Source definitions from the prediction envelope, keyed by source ID; evidence paths replaced with stored relative artifact paths. |
| `composition` | `empty`, `single_source`, or `mixed_source`, based on the distinct source IDs contributing selected episodes. |
| `source_counts` | Object mapping each declared source ID to its selected episode count, including zero for unused definitions. |
| `population` | `description`, `role`, `dataset_count`, `selected_count`, `selected_ids`, `unselected_ids`. |
| `task` | Exact declared task kind and ordered label vocabulary. |
| `policy` | Effective decision, numerical conventions, and binning settings. |
| `integrity` | Source/selected/prediction counts, missing/extra ID arrays, signal availability, and normalization diagnostics. |
| `raw`, `final` | Task-tagged hard results: single-label matrix or multi-label binary counts, totals, supports, and the required metrics. |
| `probability` | Task-tagged categorical or marginal probability metrics with availability and exact populations. |
| `signals` | Task-tagged single-label signal bins or multi-label per-label marginal bins, with availability status. |
| `episodes` | Sorted per-episode evidence. |

The `raw`, `final`, `probability`, and `signals` objects each carry `kind` equal
to `task.kind`; their schemas select the corresponding required fields. No
single-label-only keys are required on multi-label variants or vice versa.
Within a variant, absent optional signal families retain their required keys with
`not_applicable` statuses. For example, a label-only multi-label report still has
per-label binary losses, their means, and marginal-bin availability objects.

Each per-label result names its label. Single-label matrix columns are typed objects, so a
literal `ABSTAIN` class cannot be confused with an abstention column. Every metric
includes `population_count`, `population_unit` (`episode` or `label_decision`), and
`population_scope` (`selected` or `answered`). Ratios also include numerator and
denominator. Single-label metrics use selected episodes except selective metrics,
which use answered episodes. Multi-label set metrics use episodes; per-label
metrics use label decisions; aggregate label metrics use `G*K` answered or `N*K`
selected label decisions as appropriate. The metric key defines the aggregation.
Diagnostic messages supplement stable codes, stages, paths, and affected IDs;
consumers must not parse message wording to branch on error or metric state.
Consumers select properties by name, not object-key order. All output
documents use UTF-8 JSON with no nonstandard numeric tokens or terminal decoration.

The output directory must not already exist. Validate before publishing. Write
artifacts to a temporary sibling and publish only a complete directory using an
operation that cannot overwrite an existing destination. Failures leave no final
success directory. A run must never overwrite another run or any source input.
Treat run directories as sensitive because they contain full dataset snapshots.

### Evidence bindings and replay

The report's `artifacts` array is the snapshot manifest. Each evidence entry has
`kind: "evidence"`, `source_id`, zero-based `evidence_index`, `original_path`,
stored relative `path`, and `sha256`. The pair `(source_id, evidence_index)` binds
exactly one original source-array entry to exactly one copied file. The original
path must equal the string at that position in the prediction snapshot.

Enumerate sources in UTF-8 byte order of source ID, then their evidence arrays
in index order. Assign consecutive global ordinals starting at zero. Store entry
`n` at `evidence/n.bin`, where `n` is its unpadded decimal ordinal. Copy every
entry separately, including repeated source paths. Hash the copied bytes. This
rule avoids basename collisions and gives replay a deterministic path binding.

Keep `predictions.json` byte-for-byte unchanged. During replay, resolve evidence
only through these bindings. Verify complete one-to-one coverage, source ID,
array index, original string, expected ordinal path, and file digest. Missing,
extra, duplicate, or inconsistent bindings produce `E_PROVENANCE`. Original
source paths are provenance strings during replay and must never be dereferenced.
Stored paths must remain within the run directory, including after resolving
symlinks. Report source evidence arrays must match the bound stored paths.

Moving a complete run directory must preserve inspection and comparison behavior
without access to the original source files. These checks detect inconsistent
artifacts; they do not authenticate an entirely rewritten manifest and snapshot set.

Hashes identify bytes, not semantic equivalence. Reformatting a source file changes
its hash even if scoring is unchanged. This deliberate v1 rule avoids introducing
a separate canonical-JSON scheme. A manifest detects accidental artifact changes;
it is not cryptographic proof against someone rewriting an entire run.

Replay is deterministic in decisions, counts, ordering, statuses, and numerical
results within tolerance. Run IDs and timestamps differ. Verify stored artifact
digests and recompute results before trusting a run for comparison or inspection;
reject inconsistent stored results. Never follow snapshot paths outside the run
directory, including through symlinks. Replaying saved predictions requires no
external service or credentials.

Exit status distinguishes execution from model quality:

| Exit code | Meaning |
|---|---|
| `0` | Valid command completed; performance can still be poor or contain no data. |
| `2` | CLI, schema, input, policy, alignment, or comparison-contract error. |
| `3` | Filesystem error, missing artifact, or existing output destination. |
| `4` | Numeric failure or internal accounting/replay inconsistency. |

Error objects contain `code`, `stage`, `path` when available, and a concise
`message`. Stable codes are `E_PARSE`, `E_SCHEMA`, `E_ID`, `E_DUPLICATE_ID`,
`E_LABEL`, `E_PROBABILITY`, `E_CONFIDENCE`, `E_CONFIG`, `E_ALIGNMENT`,
`E_OBSERVATION`, `E_PROVENANCE`, `E_COMPARISON`, `E_IO`, `E_OUTPUT_EXISTS`, `E_NUMERIC`, and
`E_INVARIANT`. A digest mismatch is `E_PROVENANCE`; malformed record shapes and
unknown fields are `E_SCHEMA`. Invalid observation numeric values are
`E_OBSERVATION`; missing definitions or kind mismatches are `E_PROVENANCE`.
These input errors use exit code 2, not arithmetic-failure exit code 4.
Diagnostics must not dump payloads or credentials.

## Comparison and successive passes

Default comparison requires equal golden-file digests, task kinds, ordered vocabularies, selected
ID sets, evaluation roles, and metric/numerical semantics. Model, source question
configuration, predictions, and decision policies can differ; report those
differences explicitly. Source IDs are local to each run: compare referenced
definitions as well as IDs, never infer identical origins from matching ID strings.
Differences include observation definitions and preparation descriptors. Expose
probability availability and preparation differences alongside probability deltas;
do not describe transformed vectors as untouched native outputs. Observation values
remain inspectable in the source runs, not additional comparison metric families.
Different reference revisions are not comparable in v1.

`compare --intersection` permits different selections from the same golden file.
It explicitly restricts both runs to their intersection and recomputes metrics;
it does not compare aggregates calculated on different populations. Report the
shared IDs and each side's excluded IDs/counts. Preserve each source run's signal
applicability when restricting the population. An empty intersection is valid:
hard-decision metrics and applicable signal metrics have `no_data`; absent signal
families retain `not_applicable`. Apply this rule independently to each side.
Neither case produces an improvement claim or a numeric delta.

A comparison stores `comparison.json`, with a new UUIDv7
comparison ID, timestamp, the two source run IDs, SHA-256 digests of their report
files, scope (`identical` or `intersection`), exact compared/excluded IDs,
configuration differences, recomputed metrics, and candidate-minus-baseline
deltas. Each comparable metric includes both values and statuses. Emit a numeric
delta only when both statuses are `defined`; otherwise emit a null delta with
the reason. Positive deltas do not universally mean improvement: name the metric
and preserve its direction. V1 emits no overall winner or significance judgment.

The comparison's required top-level fields are `schema_version`,
`kind: "comparison"`, `status: "complete"`, `identity`, `baseline`, `candidate`,
`scope`, `task`, `population`, `configuration_differences`, `raw`, `final`, `probability`,
`transitions`, and `episodes`. `identity` contains comparison ID, timestamp, and
implementation/specification versions. Each of `baseline` and `candidate` contains
`run_id`, `report_sha256`, `sources`, `source_counts` for the compared population,
and `composition`. `population` contains compared and excluded ID lists and counts.
Each configuration difference identifies its field path and both values. `raw`,
`final`, and `probability` contain paired metrics and deltas. `transitions` contains
the correctness-category counts/IDs and typed final-outcome transition table.

Classify every compared episode into exactly one category using final correctness
(class equality or exact-set equality, with abstention not correct):

- `both_correct`.
- `recovered`: baseline not correct, candidate correct.
- `regressed`: baseline correct, candidate not correct.
- `neither_correct`: both not correct.

These counts sum to the comparison population. Also report changed final outcomes.
For single-label, include the transition table across all classes plus abstention.
For multi-label, include a separate 3-by-3 transition table for each label with
states `absent`, `present`, and `abstained`. Whole-episode abstention maps every
label to `abstained`; each label's table sums to the comparison population.
Never enumerate the power set as classes. Retain IDs for correctness categories
and changed outcomes, including changes between two nonmatching sets.
Raw-family deltas remain separate from final-family deltas. For every episode,
record both outcomes, both correctness values, and both source IDs in the comparison
artifact. Source IDs resolve against their respective side's source definitions.

Multi-label per-episode comparisons also preserve each side's matched/missed/extra
sets and their abstention status. In either task kind, answered-only deltas must expose both sides'
answered IDs/counts and their overlap; equal selected populations do not imply
equal answered populations. Such deltas are descriptive conditional differences,
not paired improvement claims. Probability deltas use the common selected
population and unchanged signal applicability.

Two runs with the same source configuration can be compared to inspect repeat
stability. They remain paired observations of the same episodes; they must not
be pooled as additional independent gold. Pair-order transformations require
task-authored expected outcomes. Validator never assumes a relation is symmetric.

If a later pass processes only conflicting episodes, record only that selection.
A full-population result requires a complete, explicitly assembled prediction
artifact. Its `sources` map identifies all originating configurations, and each
row's `source_id` identifies the configuration that produced it. For example,
retained A-from-Q1 and revised B-from-Q2 produce a `mixed_source` result, with one
episode attributed to each source. The report must not describe that mixture as
Q2 performance over both episodes. Structural checks verify references and counts;
the preparation author remains responsible for truthful source attribution.
`parent_run_id` does not automatically supply earlier predictions. Preserving
previous outputs does not establish that a revised classifier was evaluated on
untouched episodes.

## Dataset preparation contract

People can keep their source formats. An agent-authored deterministic script
converts them into the canonical format. The script must:

- Select fields and map labels explicitly, preserve UUID assignments, and retain
  complete intended episode inputs without interpreting them inside Validator.
- Keep reference judgments and review notes separate from classifier-visible
  payloads; identify any intentional domain-specific preparation.
- Record source identities, transformation version, input/output counts, and any
  withheld episode IDs/reasons in preparation evidence. Repeated transformation
  with unchanged inputs and persisted ID mapping must reproduce the same output.
- Preserve source data; never infer missing references from the predictions under
  evaluation or silently remove conflicting cases.
- Preserve classifier-returned scalars and auxiliary observations separately from
  any canonical scoring signals. When a native output needs a scalar-to-class
  mapping, record exact boundaries, tie behavior, and the source of that mapping.
  Do not substitute argmax for the classifier's recorded route. Missing native
  outcomes are not abstentions, and unrelated application actions are not targets.
- Declare numerical interpretation before measuring performance. Preserve raw
  values and attach the required preparation descriptor for changed numeric values
  or derived outcomes. The receipt lists every affected episode and any omitted
  scoring family with its reason; no gold labels may determine a numeric repair.

No transformation-language framework or automatic schema inference is required.
Preparation evidence can be attached through a source definition's `evidence` array.

## Rust project organization

These are implementation requirements, not optional layout suggestions. Validator
must ship as the `validator` command-line executable. Its library target exists to
separate and test application logic; it is not a separately supported SDK product.

### Package and source layout

Use one Cargo package named `validator`, with one binary target and one library
target. Do not introduce a multi-package workspace. The completed v1 implementation
must use this layout; create files as their behavior is implemented, not as empty
scaffolding:

```text
Cargo.toml
Cargo.lock
rust-toolchain.toml
src/
  main.rs
  cli.rs
  lib.rs
  app.rs
  model.rs
  model/
    common.rs
    single_label.rs
    multi_label.rs
  validation.rs
  validation/
    wire.rs
  evaluation.rs
  evaluation/
    single_label.rs
    multi_label.rs
  comparison.rs
  artifacts.rs
  error.rs
tests/
  cli.rs
  conformance.rs
  fixtures/
schemas/
  v2/
```

Existing documentation stays in its documented locations. `schemas/v2/` contains
the published JSON Schemas required by this specification; `tests/fixtures/`
contains synthetic canonical inputs, invalid cases, and independent expected
results. Generated runs and build output must not be committed as test fixtures.

| Module | Required responsibility |
|---|---|
| `main.rs` | Declare the binary-only `cli` module, invoke it, and return its process exit code. No validation, scoring, or artifact logic. |
| `cli.rs` | Parse arguments, dispatch the four commands through the library API, and enforce stdout, stderr, help, version, and exit-code contracts. |
| `lib.rs` | Declare library modules and re-export the small application API and its required types. No business-logic implementation. |
| `app.rs` | Coordinate `check`, `evaluate`, `compare`, and `inspect`: load, validate, calculate, verify replay, and publish results. |
| `model.rs` | Typed canonical records, configuration, outcomes, reports, and shared versioned contract constants. Opaque JSON only where this specification permits it. |
| `validation.rs` | Strict JSON decoding, duplicate-key rejection, structural and semantic checks, alignment, and construction of validated evaluation inputs. No filesystem access. |
| `evaluation.rs` | Pure decision-policy application, confusion accounting, metrics, bins, and episode diagnostics over validated inputs. |
| `comparison.rs` | Pure compatibility checks, paired-population selection, transitions, and deltas. Reuse `evaluation` for restricted-population calculations. |
| `artifacts.rs` | File reads, exact-byte hashing, evidence copying and containment checks, snapshot integrity checks, and non-overwriting publication. No metric calculations. |
| `error.rs` | Typed failures, stable diagnostic codes, and safe diagnostic fields. Do not embed opaque payloads or raw sensitive values in errors. |

The [shared data model](validator-data-model.md#source-organization-and-ownership)
assigns responsibilities to the required task-specific submodules. They remain
modules in this package, not separate crates or plugins.

The binary must import application behavior from the library target; it must not
redeclare library modules or include their source files. Library modules must not
depend on the CLI. `app` coordinates the other modules; none may depend back on
`app`. `model` and `error` must not depend on application, scoring, or I/O modules.
`validation`, `evaluation`, and `comparison` must not access files, environment
variables, clocks, random generators, or process streams. Supply required values
through arguments. Artifact loading and integrity checks precede pure comparison;
`app` owns replay recomputation through the same evaluation path used for new runs.

Keep modules private and expose items as `pub(crate)` unless the binary or
integration tests require them through the library API. Tests of internal behavior
belong in the owning module under `#[cfg(test)]`, not behind extra public APIs.
Any later module split must preserve these responsibility and dependency boundaries;
do not introduce generic `utils`, service containers, plugin traits, or parallel
implementations of the same rules.

### Rust and dependency conventions

- Use Rust edition 2024. Commit `rust-toolchain.toml` with an exact stable toolchain
  version and the `rustfmt` and `clippy` components. Set `package.rust-version` to
  the minimum compiler version actually verified; do not claim an untested MSRV.
- Commit `Cargo.lock`. Normal verification and release builds must use `--locked`;
  dependency updates are explicit changes, not a side effect of verification.
- Put runtime dependencies in `[dependencies]` and test-only dependencies in
  `[dev-dependencies]`. Enable only needed features. Use established crates for
  standard primitives; do not add an async runtime or an abstraction framework.
- Use ordinary structs and enums for contracts and typed `Result` errors for
  fallible operations. Preserve error categories to the CLI boundary instead of
  recovering them from strings. Do not use production `unwrap`, `expect`, or
  panic-based handling for invalid input or operational failures.
- Keep shared contract constants, including numerical-policy constants, in one
  place in `model.rs`; they are not additional user-configurable options. Borrow
  records for scoring rather than cloning opaque episode inputs.
- Document the exported library API and its error behavior with rustdoc. Enable
  `deny(missing_docs)` on the library. Comments explain non-obvious reasons, not
  restatements of the code. Do not suppress checks to pass verification.

### Test placement and build checks

Unit tests belong beside the behavior they exercise. `tests/conformance.rs` must
exercise the library against the required numerical and artifact cases.
`tests/cli.rs` must invoke Cargo's built `validator` executable and check actual
process exit codes, JSON stdout, stderr separation, and filesystem effects.
Tests must use isolated temporary output directories and must not depend on a
globally installed executable, external service, or private dataset. Share fixture
setup when needed; never derive numerical expected results from production scoring
code. JSON Schema checks must cover the externally emitted artifacts and responses.

Implementation acceptance requires these commands to pass on the pinned toolchain:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
```

The test command includes library, binary, integration, and documentation tests.
Review must also verify the module boundaries above; Cargo does not enforce
dependency direction between modules within a crate.

## Verification and acceptance

Implement only the defined capability set. Use ordinary Rust structs/enums for
canonical records and outcomes, an opaque JSON value for `input`, and explicit
errors for invalid data. Keep metric calculations independent of CLI formatting
and filesystem writes. No class hierarchy or extension framework is required.

Both task kinds must pass shared input, artifact, CLI, and comparison checks.
The following existing numerical cases apply to the single-label variant, using
the revised typed targets and categorical probability wrapper:

| Case | Required evidence |
|---|---|
| Three classes, one correct prediction each | Identity class matrix, accuracy and macro-F1 1, no abstentions. |
| Actual A, predicted B | One off-diagonal cell; A FN 1, B FP 1, exactly one wrong episode. |
| Actual `[A,A,B,C]`, final `[A,abstain,A,C]` | `D=2,E=1,U=1`, accuracy 1/2, coverage 3/4, selective accuracy 2/3, class coverages `[1/2,1,1]`, macro-F1 1/2. |
| Only A observed and correctly predicted in schema `[A,B,C]` | Accuracy 1; B/C F1 null; macro-F1 1/3 with `contains_undefined_classes`. |
| All episodes abstain | Coverage 0; selective metrics null; accuracy 0; no missing-prediction count. |
| Empty aligned selection | Zero counts; applicable metrics `no_data`; no success-performance claim. |
| Probabilities `[0.7,0.2,0.1]`, actual A | Log loss `-ln(0.7)`, Brier 0.14, argmax accuracy 1. |
| Probabilities `[1,0,0]`, actual B | Infinite log loss status, Brier 2, valid JSON. |
| `scored_choice`: choice A, probabilities `[0.6,0.38,0.02]`, confidence 0.39 | Threshold 0.5 rejects on confidence and accepts on maximum probability; probability losses unchanged. |
| `scored_choice`: probabilities `[0.5,0.5,0]`, choice B | Valid tied choice retained; diagnostic argmax A; raw accuracy follows B. |
| `scored_choice` with non-argmax selection; missing/extra probability key; invalid sum | Input errors, no successful run. |
| Values immediately below/equal/above a threshold or bin boundary | Exact declared policy, including signal 1 in the last bin. |
| Invalid UUID, duplicate ID/key, unknown class/field, missing input, partial signals | Specific diagnostics; no silent repair or scoring subset. |
| Missing/extra prediction, failed dataset hash | Invalid run; absence never becomes abstention. |
| Retained A-from-Q1 plus revised B-from-Q2 | Two source definitions and correct row references; `mixed_source`, source counts of one each, and both origins available in inspection/comparison. |
| Missing/unknown source ID; same source ID reused across runs with changed configuration | Invalid row reference rejected; comparison exposes definition changes despite equal source-ID strings. |
| Reordered episodes and probability object keys | Same calculations and sorted evidence; byte digests can differ. |
| Whole-file dataset/input/reference change | Different digest; existing runs unchanged; incompatible comparison rejected. |
| Two selections compared normally and with intersection | Default rejection; explicit paired recomputation with exclusions disclosed. |
| Empty intersections: probability/probability, label/label, probability/label | Hard metrics `no_data` on both sides. Probability statuses respectively `no_data`/`no_data`, `not_applicable`/`not_applicable`, and `no_data`/`not_applicable`; null deltas. |
| Declared `UNCERTAIN` versus missing/null/undeclared reference | Valid class accepted; structural defects rejected; no claim of verified reference settlement. |
| Candidate recovers one case and regresses another | Both ID lists preserved even if headline accuracy is unchanged. |
| Existing output path, late validation failure, damaged snapshot | No overwrite or finalized partial result; correct exit category. |
| Move a run containing same-basename evidence from different directories, including a parent-relative source path | Inspection/comparison succeeds from copied evidence alone after original sources become unavailable. |
| Missing/duplicate/swapped evidence binding, changed evidence byte, or escaping stored symlink | Integrity failure with no lookup of the original evidence path. |
| Evaluation and comparison receipts | `result_path` names the actual result file; independent SHA-256 of its bytes equals `result_sha256`. |
| Routine output versus explicit inspection | No payload in reports/errors; inspection returns the stored selected input as JSON. |
| Opaque input contains integer `9007199254740993` | Inspection preserves that numeric value; no conversion to rounded binary64. |
| Agent-facing output | Published schemas validate every result; one JSON document on stdout; stable typed fields, statuses, receipts, and errors. |

The multi-label and shared-structure cases are also required:

| Case | Required evidence |
|---|---|
| Schema `[A,B,C]`; expected `[{A,B},{}]`; predicted `[{A,C},{}]` | `N=G=2`, exact-match accuracy `1/2`; TP=1, FP=1, FN=1, TN=3 in aggregate; answered micro-F1 `1/2`, macro-F1 `1/3`, Hamming loss `1/3`. |
| Expected and predicted empty sets, nonempty vocabulary | Answered exact match; coverage 1, Hamming loss 0; undefined label/micro-F1 remain null, macro-F1 zero with undefined-class status. |
| Expected `[{A},{}]`; predicted `[abstention,{}]` | `N=2,G=1,U=1,D=1`; exact-match accuracy `1/2`, selective exact-match accuracy 1, coverage `1/2`; label A has total support 1, answered support 0, TN 1. Abstention contributes no binary decision. |
| Every multi-label row abstains | Coverage 0, exact-match accuracy 0 for nonempty selection, answered-only metrics `no_answered_predictions`; no fabricated empty sets. |
| Complete marginals `[0.8,0.7]`, expected `{A}` | Values stay unchanged despite sum 1.5; mean binary log loss `(-ln(0.8)-ln(0.3))/2`, mean binary Brier `0.265`. |
| Same row, raw set `{A,B}`, thresholds A=0.8/B=0.75 | Final set `{A}`; equality accepted; raw metrics and probability losses unchanged. |
| Every marginal below its label threshold | Final set empty and answered, not abstained. |
| Expected empty set, marginals all zero | Binary losses zero; no `0*ln(0)` or invalid JSON. |
| Expected `{A}`, marginal A=0; or A absent and marginal A=1 | Per-label and mean binary log loss positive-infinity statuses; no clipping. |
| Per-label probability bins at 0, 0.1, and 1 | Declared boundary assignment; positive rates follow reference presence, not prediction correctness. |
| Single-label probabilities `[0.8,0.2]`, true A; one-label marginal A=0.8, true present | Categorical Brier `0.08`; binary marginal Brier `0.04`. Distinct metric names/scaling retained. |
| Two references `{A,B}`; predictions `[{}, {A,B}]` versus `[{A},{B}]` | Same per-label confusion counts, but exact-match accuracy `1/2` versus 0. Episode evidence must retain the difference. |
| Duplicate label in target or prediction, unknown label, partial marginals | Rejection before set conversion or threshold filtering, even when the invalid prediction would be filtered out. |
| Label-set target in single-label task, class target in multi-label task, wrong probability kind, or wrong policy kind | Typed schema/configuration error; no inference or coercion. |
| `scored_choice` source or reported confidence in multi-label artifact | Unsupported task/source or field combination rejected. |
| Missing prediction row, empty set, abstention, missing marginal key | Respectively alignment error, answered set, explicit unanswered outcome, and probability error. |
| Same multi-label episode changes `{A}` to `{B}` while reference is `{A,B}` | `neither_correct` plus changed-outcome ID and per-label transition evidence; partial changes are not hidden by exact-match categories. |
| Multi-label run comparison with different abstentions | Same selected population, explicit answered populations/overlap; conditional metric changes are not described as paired model improvement. |
| Multi-label intersection becomes empty | Signal applicability retained; present marginals yield `no_data`, absent marginals `not_applicable`. |
| Permuted label sets and JSON map keys | Same scores and vocabulary-ordered evidence; exact-byte digests can change. |
| Count maps, mixed task kinds, explicit partial-reference masks, per-label abstention, or wire version 1 | Rejection with no fallback interpretation. Legally shaped but unreviewed reference sets remain a preparation responsibility. |

The final evidence contract also requires these conformance cases. In the previous
table, the multi-label confidence rejection refers to the top-level scoring field,
not a retained `reported_confidence` observation.

| Case | Required evidence |
|---|---|
| Single-label reference A; submitted abstention with categorical `[0.7,0.2,0.1]` and confidence `0.8` | Raw accuracy 0 and coverage 0; log loss `-ln(0.7)`, Brier `0.14`, argmax accuracy 1; chosen probability null; confidence bins `no_answered_predictions` with the excluded episode ID. |
| Two rows: one raw answered and one raw abstained, both with confidence, policy `as_recorded` | Confidence bins count exactly one row; excluded ID disclosed. |
| Submitted abstention carrying signals with `reject_below` or `label_thresholds` | Configuration/input error under the existing answered-input policy preconditions; signals do not synthesize an answer. |
| Multi-label reference `{A}`, submitted abstention, marginals `[0.8,0.7]` | Coverage 0 and no binary hard decisions; mean binary Brier `0.265` and finite binary log loss over the full selected population. |
| Scalar `1.33`, distribution `[0.24,0.19,0.57]` over `[NO_MATCH,UNCERTAIN,MATCH]`, prepared native outcome `UNCERTAIN` | Scalar and outcome retained; no override to `MATCH`. Distribution is scored only if also submitted as canonical scoring evidence. |
| Retained categorical observation sums to `0.99`; scoring family omitted on all rows | Successful hard-label evaluation with unchanged observation values; probability metrics `not_applicable`. Preparation receipt discloses the omission. |
| The same `0.99` vector submitted as scoring probabilities | `E_PROBABILITY`, exit 2, no successful run. No automatic promotion, normalization beyond the fixed tolerance, or fallback. |
| Explicitly prepared normalized vector with original `0.99` observation, descriptor, and bound receipt/script/raw evidence | Strict canonical validation passes; original, prepared, and working values remain distinct and reproducible. No claim that normalization establishes calibration. |
| Returned scalar differs from displayed-vector mean | Neither scalar replacement nor automatic failure; no scalar consistency equation is part of Validator's scoring contract. |
| Auxiliary Bernoulli observation lacks gold; same number submitted as a scoring marginal under a separately labeled task | First case is inspection-only; second receives the specified binary metrics. No implicit task conversion. |
| Unknown observation name/kind, nonfinite scalar, out-of-range Bernoulli value, or broken preparation evidence index | Typed input/provenance error; no silent omission. |
| Observation absent on one row; top-level probability missing on one row | Observation is unavailable without imputation; incomplete scoring family is rejected. |
| Probabilities or scalar present without an outcome | Schema error; no synthesized choice or abstention. |

Also satisfy the [structure acceptance checks](validator-data-model.md#structure-acceptance-checks).
Exhaustively enumerate all expected/predicted subsets of a three-label vocabulary
to verify binary accounting and set identities. Preserve existing single-label
fixtures rather than replacing them with multi-label tests.

Use the earlier master specification's asymmetric fixture F04 as an additional
numerical oracle: accuracy `5/8` and macro-F1 `131/210`. Its wider feature set and
zero-filled per-class output convention are not v1 requirements. Independently
enumerate small hard-label datasets to check matrix identities and reorder
invariance. Pin and record any external reference-library version used for
differential tests; account for zero-probability clipping and averaging differences.

Before claiming implementation completion, format and lint Rust, run the defined
tests, exercise the CLI on a complete golden/prediction pair for each task kind,
inspect one failure in each, and compare two runs of each kind. Verify that an agent can locate failures and compare metrics
from structured fields without parsing explanatory prose.
This specification's publication does not satisfy those implementation checks.

### Product acceptance criteria

The following criteria define observable product behavior. The conformance tables
above and the shared-model checks supply the required detailed cases.

| ID | Criterion | Completion evidence |
|---|---|---|
| AC1 | Canonical inputs faithfully represent both supported task kinds, stable episode identities, opaque input, references, outcomes, and provenance. | Valid and invalid fixtures prove the typed contracts and distinguish empty sets, abstentions, missing records, and semantic uncertainty classes. |
| AC2 | Classifier evidence retains its meaning independently of the outcome. | Round-trip cases retain categorical and marginal values, confidence, scalar scores, and auxiliary judgments; abstention does not erase signals; unreferenced auxiliary judgments receive no invented metrics. |
| AC3 | Numerical interpretation is explicit and reproducible. | Boundary tests cover strict scoring admission, observation-only retention, disclosed deterministic preparation, and no silent repair, exclusion, or altered native route. |
| AC4 | Metrics and populations are correct. | Independent expected values and exhaustive small-case accounting checks pass; undefined/infinite results, ties, zero probabilities, abstentions, and empty populations are represented honestly. |
| AC5 | An agent can investigate failures through structured output. | Published JSON Schemas validate outputs; an agent identifies and inspects failed episodes without parsing explanatory prose; routine output does not expose opaque input. |
| AC6 | Successive runs preserve history and population meaning. | Comparison tests expose recovered/regressed IDs, source/preparation changes, conditional populations, explicit intersections, and incompatible-run rejection. |
| AC7 | Artifacts are immutable and results reproducible. | Replay, relocation, tampering, no-overwrite, and late-failure tests pass; digests identify the exact saved evidence and result files. |
| AC8 | The CLI is a usable machine interface. | Process-level tests verify all four commands, JSON stdout, safe errors, exit categories, receipts, help/version, and operation on saved inputs without network access or credentials. |

### Frozen practical acceptance case

Use the Chord saved classification outputs described in the
[restored-score evaluation record](/Users/dowwie/MyProjects/chord/docs/experiments/typesafe-restored-score630-evaluation.md).
That external document is evidence about an existing case, not a source of new
Validator requirements. Freeze the exact source files and record their paths and
hashes in the acceptance bundle before deriving expected results. Never read a
moving working directory as an implicit acceptance oracle.

The acceptance workflow must:

1. Account for the full 630-episode source population: 603 labeled episodes and
   27 withheld/unscored episodes, with source IDs, persisted UUID mapping, and
   exclusion reasons in preparation evidence. The scored gold contains only the
   603 labeled episodes. Preserve the accepted reference snapshot and its review
   limitations; do not relabel to improve acceptance results.
2. Prepare native selected-choice and score-based categorical outputs as separate
   prediction artifacts bound to the same gold. Preserve original input context,
   scalar values, distributions, reported confidence where present, and the three
   auxiliary Bernoulli judgments where available. Record actual source/prompt
   differences rather than claiming the two calls held instructions constant.
   Use source kind `classifier` when retaining observation-only numerical evidence;
   use `scored_choice` only when every row satisfies that stricter profile.
3. Preserve native routes, including score-derived `UNCERTAIN` outcomes that differ
   from distribution argmax. Freeze the documented scalar mapping in preparation;
   do not evaluate a broader application policy or introduce a different route as
   an acceptance target.
4. Account for the documented numerical discrepancies using the observation/scoring
   boundary. Exercise a successful hard-label run with original observations and
   no probability scoring, plus strict rejection of nonconforming scoring vectors.
   Conformance fixtures cover explicitly prepared probability vectors. Do not
   assume raw outputs satisfy canonical precision or discard rows with a failed
   historical parsed projection when the native output is recoverable from saved
   response evidence. Missing native outcomes must fail the claimed complete run.
5. Independently check the expected hard-label counts against the frozen native
   outputs and references, without using Validator's implementation to generate
   the oracle. Explain any differences from historical reports through population,
   parsing, or route definitions; do not require copying their headline scores.
6. Evaluate the full labeled development population and keep representative and
   challenge cohorts distinguishable through explicit selections. Inspect a known
   failure, compare compatible runs, and demonstrate a targeted-pass intersection
   without describing it as full-population improvement.

Use a frozen synthetic multi-label bundle for the second task kind, with
independently calculated results, partial overlap, empty sets, abstention, complete
marginals, two comparable runs, and an inspectable failed episode. Public automated
tests must not depend on private Chord files; preserve synthetic equivalents of
its important numeric and evidence cases in the conformance suite. Protect the
private acceptance bundle and do not publish episode payloads as test fixtures.

### Definition of done

The development project is done only when all of the following are verified:

1. The reconciled specification and shared data model are the approved implementation
   baseline. Wire shapes, observation/scoring boundaries, numerical behavior, and
   error semantics contain no unresolved implementation choices. Publication alone
   does not imply owner ratification or implementation completion.
2. AC1 through AC8, all required conformance cases, and shared structure checks pass.
   Preserve an acceptance record mapping each criterion to commands, fixtures or
   frozen input hashes, expected and actual results, and verification evidence.
   Expected numerical results must be independent of production scoring code.
3. The pinned-toolchain format, Clippy, test, and locked release-build commands pass
   without bypasses. Review verifies the prescribed single-package binary/library
   organization, module dependencies, checked type boundaries, and provider-neutral
   naming. No implementation placeholder counts as a completed capability.
4. A fresh agent uses the installed executable, published schemas, documentation,
   and frozen acceptance bundles to complete preparation, checking, evaluation,
   failure inspection, and comparison for both task kinds. It must not read the
   implementation or write scoring code. Deterministic domain-to-canonical
   transformation code is allowed. Record commands, outputs, exit codes, and hashes.
5. Deliver verified installation instructions for the tested host/toolchain, CLI
   examples, version identification, all published schemas, documented limits,
   and the protected acceptance evidence. The user can repeat the workflow with
   another dataset without repository-specific scoring changes.

No classifier accuracy threshold is a project completion criterion. Correctly
exposing poor performance, a regression, or invalid evidence is successful tool
behavior. Acceptance does not certify reference truth, unseen-data performance,
or deployment suitability. Specification reconciliation is documentation work,
not evidence that any of these implementation checks have passed.

## Design sources and relation to the master specification

The [earlier classification master specification](../multi_class_classification_validation_specs.md)
is reference material. This document defines v1 scope and overrides it where
contracts differ: episodes and UUIDs, opaque required input, coexisting choices
and signals, probability maps, null per-class undefined ratios, reduced metrics,
and explicit run comparisons. V1 must not claim full master-spec conformance.

The [historical multi-label discussion](../generalized_multi-label_specs.md) is
also Reference material, not a second contract. Exact overlap is valid for sets;
minimum per-label counts are valid for multiset counting. Neither is a universal
object-matching algorithm. This revision rejects confidence-weighted confusion
counts, inferred vocabularies, silent deduplication, and the sparse null-pair matrix
as core structures. Its threshold example at line 162 has F1 1 after filtering,
and its example at line 188 has micro-F1 `1/2`; those historical values are not
test oracles. The original file remains unchanged as design history.

The following sources informed the design; their example thresholds are not defaults:

- [TypeSafe Choice](https://docs.typesafe.ai/primitives/choice), consulted 2026-09-18:
  selected option, complete keyed probability distribution, and a separate confidence signal.
- [TypeSafe confidence](https://docs.typesafe.ai/confidence), consulted 2026-09-18:
  reported concentration statistics and task-dependent threshold interpretation.
- [RFC 9562](https://www.rfc-editor.org/rfc/rfc9562.html): UUID representation and UUIDv7 generation.
- [Multi-label confusion accounting](https://scikit-learn.org/stable/modules/generated/sklearn.metrics.multilabel_confusion_matrix.html)
  and [Hamming loss](https://scikit-learn.org/stable/modules/generated/sklearn.metrics.hamming_loss.html):
  per-label binary counts and the label-decision denominator.
- [Cargo package layout](https://doc.rust-lang.org/cargo/guide/project-layout.html),
  [binary/library separation](https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html),
  and [Cargo.lock](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html):
  conventional Rust package organization, testable logic, and dependency reproducibility.
- *LLM Evaluation and Alignment*, v4 MEAP, sections 1.3.1, 4.3.1, 4.4.3–4.4.4,
  5.2.5, and 8.5.4: iterative evaluation, perturbation checks, versioned criteria,
  targeted versus representative review, consistency limitations, and regression evidence.

The book supports preserving evidence and criteria across runs. Its broad sample-size
heuristics and score-spreading advice do not define this utility's numerical rules.
