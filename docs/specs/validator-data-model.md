# Validator shared data model

Version: 1.2-draft. Date: 2026-09-18. Authority: Ratified.

This document defines the data structures and invariant boundaries for the
[Validator v1 specification](validator-v1.md). The two documents form one design
baseline: this document owns Rust structure and ownership rules; the main
specification owns wire fields, task semantics, metrics, and CLI behavior.
**Must** denotes an approved implementation requirement, not existing implementation.

## Design boundary

Share evidence handling, not a universal label-matching algorithm. Support exactly
two task kinds: `single_label` and `multi_label`. A binary classifier is a
single-label task with two classes. Multi-label targets are complete sets of
applicable labels; they are not categorical alternatives and need not be
statistically independent.

Each dataset declares one task and one ordered vocabulary. Do not mix task kinds
within a dataset or introduce a task registry. Separate independent classification
tasks use separate datasets/runs. A fixed family of yes/no conditions on the same
episode can be represented as one multi-label task when every condition has a
reference judgment. Its source configuration must identify all question definitions
and their label mapping, not merely one component question.

Evaluate classifier outputs, not the surrounding application or platform. Scalar
and auxiliary observations do not create additional target types. A canonical
prediction always contains a class, set, or explicit abstention; deterministic
conversion of native classifier output belongs to preparation, not a general
rule-execution subsystem inside Validator.

Count prediction is deliberately excluded. Repeated water-fixture labels represented
counts within a time-window episode: minimum actual/predicted counts give matched
occurrences, and differences give misses/extras. That valid count-specific rule
does not justify silently deduplicating counts into sets or matching identified
objects by their labels. Do not add count variants, object matching, hierarchy,
partial-label masks, score plugins, or generic extension fields in this version.

## Shared records and typed task data

Use the following structure names and responsibilities. Private helper types can
be added for actual implementation needs, but the invariant boundaries must remain.

| Structure | Contents and invariant |
|---|---|
| `EpisodeId`, `RunId`, `SourceId`, `ArtifactDigest` | Distinct newtypes for canonical episode UUIDs, run UUIDs, artifact-local source IDs, and exact-byte SHA-256 digests. Do not interchange them as unvalidated strings. |
| `TaskDefinition` | Closed enum: single-label or multi-label, each with a validated `LabelVocabulary`. |
| `LabelVocabulary` | Ordered unique nonblank label strings and exact string-to-`LabelIndex` lookup. One vocabulary per dataset. |
| `LabelIndex` | An internal index valid only with its owning dataset vocabulary; never serialized as a durable label identity. |
| `LabelSet` | Private sorted unique vector of `LabelIndex` values. Empty is valid. Reject duplicate submitted labels before constructing it; do not silently deduplicate. |
| `Episode<Target>` | Episode ID, required opaque input, and a target of one concrete task type. Own the opaque payload once. |
| `Outcome<Target>` | `Answered(Target)` or `Abstained` with optional reason. Expected targets never use this outcome wrapper. |
| `Prediction<Output>` | Episode ID, source ID, task-specific output, and a named `ObservationSet`. Supporting observations are independent of the scored outcome. |
| `SourceDefinition` | Model, source kind, opaque configuration, optional question ID, evidence references, observation definitions, and optional preparation descriptor. No provider-specific types. |
| `ObservationSet` | Map of nonblank names to a closed `Observation` enum, checked against the referenced source's definitions; may be empty. |
| `Observation` | `Scalar`, `Bernoulli`, `ReportedConfidence`, `Categorical`, or `LabelMarginals` with the finite numeric/range checks in the main specification. Retained evidence, not a scoring input. |
| `ObservationDefinition` | Kind, nonblank description, and optional question ID. Meaning/scale details remain in source configuration. No implicit gold target. |
| `PreparationDescriptor` | Method, version, opaque configuration, and nonempty unique evidence-array indices binding the transformation, source artifacts, and receipt. No executable rule abstraction. |
| `Population` | Declared description/role, selected and unselected IDs, dataset identity, and exact counts. |
| `EvaluationConfig<Policy>` | Population selection, optional parent ID, and a policy legal for the concrete task. |
| `RunManifest` | Existing identity, snapshot digests, source definitions/counts, composition, and evidence bindings. |
| `MetricResult` | Finite value or typed nonnumeric status, population count/unit, and ratio numerator/denominator where defined by the metric. |
| `RunReport<Results, EpisodeEvidence>` | Shared manifest/population/task information plus concrete task results and per-episode evidence. |

Generic records are internal code reuse, not an extensibility API. Instantiate
them only for the two supported task kinds. Use closed enums at dispatch boundaries;
do not introduce a `Task` trait, trait objects, plugin registry, or dynamic metric map
whose required fields exist only by naming convention.

### Task-specific structures

The concrete variants must not be a single struct filled with unrelated optional
fields. Use these types:

| Task | Target | Output | Policy | Results |
|---|---|---|---|---|
| Single-label | `LabelIndex` | `SingleLabelOutput` | `SingleLabelPolicy` | `SingleLabelResults` |
| Multi-label | `LabelSet` | `MultiLabelOutput` | `MultiLabelPolicy` | `MultiLabelResults` |

`SingleLabelOutput` contains `Outcome<LabelIndex>`, optional
`CategoricalDistribution`, and optional `ReportedConfidence`. Both answered and
abstained outcomes can retain these signals. The source-specific `scored_choice`
check still requires a class outcome.
`MultiLabelOutput` contains `Outcome<LabelSet>` and optional `LabelMarginals`, with
no reported-confidence scoring field in this version. Marginals can accompany an
abstention. Both task kinds share retained observations through `Prediction`.
No synthetic confidence is inferred from marginals.

`CategoricalDistribution` and `LabelMarginals` must be distinct types, not aliases
of the same map. Both require exactly the task vocabulary and finite values in
`[0,1]`. Only the categorical type has a sum-to-one check and normalized working
values. Marginals retain their individual values without cross-label normalization.
A common checked finite-probability scalar is reusable; vector validation is not.

Observation vectors must be distinct from scoring vectors even when their names
or numbers match. An observed categorical vector has nonempty string-keyed options
and checked components but no sum-to-one invariant or task-vocabulary binding.
Scoring categorical vectors require both. Observations are not aliases, wrappers
implicitly dereferenced into scoring types, or an alternative input to evaluators.
Only the top-level scoring fields construct `CategoricalDistribution`,
`LabelMarginals`, or scoring `ReportedConfidence`. No automatic promotion exists.
Retain observation maps in exact key/value meaning; serialize map keys in UTF-8
byte order. Scalar observation values use checked finite binary64, not an opaque
JSON number; exact original numeric spelling remains in snapshots.

After validation, probabilities use vectors in vocabulary order, not repeated
string lookups. Retain original parsed scoring values separately from normalized
categorical values. Keep exact submitted JSON bytes in snapshots; normalization and
set ordering never rewrite those snapshots.

`SingleLabelPolicy` has `AsRecorded` and `RejectBelow`; `MultiLabelPolicy` has
`AsRecorded` and `LabelThresholds`. The latter selects labels, not whether to answer.
The type system must not allow a categorical rejection policy to be passed to the
multi-label evaluator or marginal thresholds to the single-label evaluator.

### Absence and uncertainty

These states must remain distinct in both parsing and reports:

| State | Meaning |
|---|---|
| Expected empty label set | Every declared label was assessed and found absent. |
| Predicted empty label set | Answered prediction that no declared label applies. |
| Explicit abstention | No episode-level decision; never converted into an empty set. |
| Missing prediction row | Alignment error, not abstention. |
| Omitted probability family | Hard-label-only artifact; probability metrics not applicable. |
| Observation-only probabilities | Retained for review; never imply that a scoring family is available. |
| Omitted named observation | Unavailable on that row, not zero, false, or a missing prediction. |
| Missing marginal key | Invalid incomplete probability evidence, not probability zero. |
| Missing or unresolved reference | Not scored; preserve upstream review evidence and exclusions. |

Whole-episode abstention is supported. Per-label abstention, partially observed
gold, and partial scoring probability families are not. Reject unsupported shapes rather
than silently scoring the observed portion.
Completeness is a dataset-owner assertion: the validator cannot distinguish an
unreviewed label from a reviewed negative if both are encoded as absence in a
legal reference set. It must not claim otherwise.

## Decode, validate, align, then score

Use one boundary from untrusted wire records to validated evaluation inputs:

```text
exact input bytes
  -> strict wire decoding
  -> task, vocabulary, record, source, and policy validation
  -> digest verification and ID alignment
  -> ValidatedEvaluation::{SingleLabel, MultiLabel}
  -> concrete evaluator
  -> typed report
```

Wire DTOs are private to `validation/wire.rs`. Their enums preserve explicit wire
tags. Reject unknown tags, fields, duplicate JSON keys, conflicting task/target
types, and wrong probability kinds. Do not implement permissive untagged fallbacks
or type inference from target cardinality or probability sums.

The validated task payloads are `SingleLabelEvaluation` and
`MultiLabelEvaluation`. Each owns its checked vocabulary, source definitions,
population, legal policy, and sorted aligned rows. Each row contains exactly one
expected target and exactly one prediction for the same episode. Public callers
cannot construct an evaluation from arbitrary vectors or mutable fields.

Validated structures must have private fields and checked constructors or
`TryFrom` conversions returning typed errors. Constructors belong with their
invariants in `model`; `validation` invokes them. Do not derive `Deserialize`
directly on validated structures or expose setters that bypass validation. Scoring
functions accept only the appropriate validated structure, never raw JSON or DTOs.
Unit tests exercise private constructors; integration tests use the application API.
No phantom-state builder framework is required.

Artifact-wide signal availability is checked before episode selection and carried
as `SignalAvailability` into the validated structure. Restricting a run to an empty
intersection must not change that availability. A validated population-restriction
operation verifies IDs, preserves task/evidence semantics, and rebuilds aligned
rows; comparison must not reconstruct a synthetic unvalidated prediction file.
Observation presence does not contribute to `SignalAvailability`. Observation
shape/range failures and invalid scoring inputs remain fatal; a source's displayed
sum or scalar/mean disagreement is not itself a failure of the weaker observation
contract. Numerical preparation happens outside this boundary and is provenance,
not a configurable tolerance or an in-process repair mechanism.

Indexes, label sets, policies, and probability vectors remain owned by or borrowed
from their evaluation. No API can mix label indexes from two datasets. Across runs,
compare dataset/task identity before using their internal indexes.

## Reports and metric reuse

Share checked count arithmetic, ratio/status construction, UUID sorting, population
alignment, source attribution, and artifact verification. Keep task-specific
denominators and probability semantics in the owning evaluator.

`SingleLabelResults` owns the class-to-class matrix and its metrics.
`MultiLabelResults` owns binary per-label counts, exact-set outcomes, coverage,
and marginal probability metrics. Neither result is reconstructed from the other.
Shared TP/FP/FN arithmetic does not imply shared episode accuracy or Brier scaling.

Every metric must name its evaluated population and unit. The `population_unit`
field is `episode` or `label_decision`; a marginal mean over all labels has `N*K`
label decisions, not `N` independent examples. Micro-F1 denominators are count
expressions, not episode counts. These fields describe accounting, not statistical
independence. Multi-label answered-only counts must be labeled as such.

Task-specific episode evidence retains expected, raw, and final targets and their
correctness. For an answered multi-label outcome, include matched, missed, and
extra label sets. For an abstention those sets are null with an explicit abstained
status, not artificial misses or negatives. Preserve per-label probability evidence
even when threshold selection changes the final set.
Retain each episode's named observations regardless of outcome. Do not copy raw
response bodies into reports; preserve them through bound evidence files. Signal
bins separately declare their population: maximum-probability bins use `selected`,
confidence bins use `raw_answered`, and multi-label marginal bins use `selected`.
`raw_answered` is a bin-population scope, not an additional scalar-metric scope.

Typed report and comparison variants must expose required fields through Rust
types and JSON Schema alternatives. A report for one mode must never masquerade
as the other through reused ambiguous names such as a universal `accuracy`.
The main specification defines exact metric keys and applicability rules.

## Source organization and ownership

Keep the single Cargo package and thin binary from the main specification. Give
the shared structures and evaluators these concrete homes:

```text
src/model.rs
src/model/common.rs
src/model/single_label.rs
src/model/multi_label.rs
src/validation.rs
src/validation/wire.rs
src/evaluation.rs
src/evaluation/single_label.rs
src/evaluation/multi_label.rs
```

`model.rs` declares/re-exports model modules and owns versioned numerical-policy
constants. `common` owns shared records, observation variants/definitions, and
preparation descriptors. Each task model owns its invariants and
result types. `validation.rs` owns the decode/validate/align entry point;
`evaluation.rs` dispatches the closed validated enum and owns shared arithmetic.
Neither task evaluator depends on the other. `comparison` calls the same evaluation
dispatch after population restriction. `artifacts` never calculates metrics.

Retain exact snapshots once and avoid cloning opaque inputs into metric rows or
comparison results. Scoring borrows validated inputs; inspection accesses the
verified dataset payload explicitly. No global mutable schema, hidden defaults,
or mutable run cache is required.

## Structure acceptance checks

- Both task kinds use the same CLI operations, manifest, ID alignment, provenance,
  and artifact publication without duplicate implementations of those rules.
- Checked construction rejects cross-kind targets, probabilities, and policies;
  evaluator signatures prevent cross-kind input at compile time.
- Reordering submitted label sets preserves scores and canonical report ordering;
  duplicate labels fail before any conversion to a set.
- Marginals such as `[0.9,0.8]` remain valid and unchanged; the same numbers are
  invalid as a categorical distribution.
- Empty set, abstention, missing row, and missing marginal key take four distinct
  paths with the required evidence or error.
- Restricting a run to zero episodes retains probability applicability.
- Evidence-bearing abstentions retain distributions and observations. Probability
  metrics use the selected population; confidence/correctness diagnostics use raw
  answered episodes and disclose their included/excluded IDs.
- Observed categorical `[0.5,0.49]` is retained without normalization; the same
  vector in a scoring field fails validation. No implicit observation-to-signal
  conversion or error-recovery path is available.
- Scalar and auxiliary observations round-trip with their definitions and source
  references but do not generate unsupported target types or metrics.
- Broken observation definitions and preparation bindings fail before scoring.
- Per-label counts do not replace episode records: equal aggregate counts can
  coexist with different exact-set accuracy.
- Unsupported counts, mixed task kinds, partial labels, and per-label abstentions
  are rejected without adding extension hooks or inferred interpretations.

## Basis

This design uses Rust's checked conversions and enum-based task dispatch to keep
validation boundaries explicit. It deliberately stops short of a generic task
framework. See the [Rust enum documentation](https://doc.rust-lang.org/book/ch06-00-enums.html)
and [TryFrom contract](https://doc.rust-lang.org/std/convert/trait.TryFrom.html).
Conventional multi-label binary accounting is described in
[scikit-learn's multi-label confusion matrix documentation](https://scikit-learn.org/stable/modules/generated/sklearn.metrics.multilabel_confusion_matrix.html).
