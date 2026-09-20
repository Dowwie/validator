# Jev application validation: concrete proof-of-concept plans

Date: 2026-09-20. **Authority: Proposed.**

These are implementation-ready experiment designs, not completed experiments or a new Validator specification. The owner requested plans and repository storage; this document does not authorize live provider calls, source changes in third-party projects, publication of their data, or task-management activity. No golden labels have been human-approved, no adapters have been implemented, and no model results have been generated as part of writing these plans.

The eight primary applications and two optional integrations come from the GitHub candidate review. Their source shapes were checked against the references in each section. Pins are reviewed source references, not promises that future default branches have identical APIs. Reconfirm the pin and installed dependency versions when implementing a capture adapter.

Validator baseline: `91554b7d1f03ac8bc8d5ee549a5b714d5d81af0b`, executable 0.1.0, wire schema 2. The [ratified specification](../specs/validator-v1.md), [data model](../specs/validator-data-model.md), and [architecture](../../architecture.md) govern the core. This plan adds no core commands, provider integrations, ordinal task, metric families, database, workflow engine, or plugin system.

## Catalogue and suggested order

| Example | First measurement | Proposed initial data | First runnable mode |
|---|---|---|---|
| [1. jgrep](#1-jgrep-code-change-screening) | Case-level binary change detection | Existing 20-case diagnostic; optional 48 new cases | Saved-output replay, no model calls |
| [2. Jev Sift](#2-jev-sift-relevance-screening) | Query/content relevance | 80 fictional descriptions, 60 development / 20 held out | Capture one Noul per case |
| [3. Foreman](#3-foreman-multilabel-job-state-assessment) | Ten simultaneous assessment labels | 60 snapshots from 20 fictional jobs; 45 / 15 | Assessment only; no coding workers |
| [4. Upwork triage](#4-upwork-applyreviewskip-triage) | Final apply/review/skip classification | 90 fictional jobs; 60 / 30 | Saved or captured Verdict objects |
| [5. Tax documents](#5-tax-documents-form-and-page-kind-classification) | Form identity, then page kind | 60 form-text cases; separate optional 42 page-kind cases | Text-line API, not PDF ingestion |
| [6. classifier.dev](#6-classifierdev-fixed-taxonomy-classification) | Fixed-taxonomy single-label classification | 120 support messages; 80 / 40 | Local Jev integration, not public-service load |
| [7. Document filing](#7-local-document-filing) | Suggested category versus filing disposition | 80 fictional text documents; 60 / 20 | Classification function; no file moves |
| [8. Compaction](#8-fast-jev-compaction-retention-decisions) | Keep / truncate-result / drop-call agreement | 120 eligible calls across 20 transcripts; 90 / 30 | Library-only capture |
| [9. Router, optional](#9-jev-router-optional) | Declared routing-rubric agreement | 60 prompts plus 24 separate policy fixtures | Offline policy fixtures, then captured routing |
| [10. jev-align, optional](#10-jev-align-export-integration-optional) | External replay of a fixed binary task | 80 stories; 60 training / 20 sealed external test | Export labels and before/after predictions |

Start with jgrep, then Sift, then Foreman. Each is independently useful. The later plans are alternatives for demonstrating breadth, not prerequisites for shipping the first proof of concept. Dataset counts are construction targets, not permission to force ambiguous examples into settled labels.

## Shared experiment contract

### What is being proved

Separate three conclusions:

1. **Adapter correctness:** upstream inputs and predictions are represented faithfully, without changed meanings, missing-case concealment, or fabricated signals.
2. **Diagnostic performance:** this pinned application configuration agrees or disagrees with independently reviewed references on this declared population.
3. **Practical usefulness:** run evidence supports a specific review and a next experiment, even when the candidate does not improve.

Successful integration does not require a high Jev score. A correctly measured weak result is a successful proof of the validation process. Synthetic diagnostic accuracy does not estimate a production workload's accuracy or establish deployment safety.

### Constructing gold without circular validation

Infer **structure** from the application: what it sees, the questions and options it uses, the typed outputs it returns, and the deterministic transformations applied afterward. Do not infer the correct labels from Jev's answers, its confidence, or agreement between repeated calls.

For each example, first write a labeling rubric with observable inclusion/exclusion rules and conflict resolution. Astra may propose fictional inputs, labels, and rationales. A separate reviewer should challenge the proposals; a human must approve the scored references before the first target-model run. Neither the generator nor a second Jev call can certify gold. Do not set `human_approved` merely because an agent created a review file.

Keep a proposed authoring sidecar, separate from Validator's wire format:

```json
{
  "case_id": "sift-001",
  "family_id": "hospital-vendor-01",
  "model_input": {"text": "Fictional company description"},
  "proposed_expected": {"type": "class", "label": "relevant"},
  "rationale": "Specific evidence under the frozen rubric",
  "review": {"state": "pending", "reviewer": null},
  "partition": "development"
}
```

`pending`, `approved`, and `withheld` are authoring states, not classification labels. Preserve withheld cases and reasons outside the scored dataset. Multi-label approval must explicitly cover every label, including negatives. Unreviewed does not mean absent.

Group paraphrases, matched pairs, snapshots of the same job, and calls from the same transcript into one partition. Freeze development and held-out IDs before tuning. Do not let a proposer, optimizer, or reviewer choosing a candidate inspect held-out labels or their per-case results. A final held-out review consumes that holdout; subsequent tuning requires a new one. A frozen diagnostic set can still be reused for regression, but must not continue to be described as untouched holdout.

Use a separate request projection that allowlists model-visible fields. Assert that `expected`, `proposed_expected`, labeling rationales, review metadata, split metadata, and seed labels do not reach the provider. Labels naturally printed in documents or legitimate user instructions are task evidence; artificial fixture names such as `positive-case.txt` are leakage.

### Canonical mapping into the existing utility

Every task has one frozen `golden.json`, one ordered label vocabulary, and one complete expected target per episode. Keep the logical input stable across arms; put arm-specific question wording, batching, truncation, rendered state, and configuration into source configuration and capture evidence, not into a rewritten golden file.

Assign UUIDs once and persist `source-id-map.json`. UUIDv5 from a fixed, persisted namespace and an unambiguous composite source key is a viable adapter choice; Validator also accepts standard non-sentinel UUIDs. Never reassign IDs on each import. Hash the exact final golden bytes and bind every prediction artifact to that digest.

Example canonical episode, with deliberately illustrative content:

```json
{
  "schema_version": 2,
  "task": {"kind": "single_label", "labels": ["irrelevant", "relevant"]},
  "episodes": [{
    "id": "01995c20-7d00-7000-8000-000000000001",
    "input": {"query": "Evidence of selling software to hospitals", "text": "We license bed-management software to hospitals."},
    "expected": {"type": "class", "label": "relevant"}
  }]
}
```

Use source kind `classifier` by default. `scored_choice` is permitted only for a single-label source with a recorded `question_id`, selected class among the exact probability maxima, complete categorical probabilities, and reported confidence. Do not repair a nonconforming choice to make it fit that profile.

| Upstream evidence | Permitted mapping |
|---|---|
| Noul probability `p` for one independently labeled proposition | Binary class from an explicit threshold; optional categorical probabilities `{negative: 1-p, positive: p}`. No invented confidence. Record the complement/threshold transformation. |
| Choice over a fixed vocabulary | Selected label as class; full probability map only when it meets the core's scoring contract; Jev-reported confidence remains distinct from selected-option probability. |
| Several independently labeled Nouls | Complete `label_marginals`; an explicitly thresholded label set. No requirement that marginals sum to one; no set-level confidence. |
| Score, weighted fit, maximum across record probabilities, or multi-stage decision | Final class/set under an explicit preparation rule; scalar/distribution observations and raw evidence. Do not invent a probability distribution over final actions. |
| Missing or malformed provider result | Account for a no-decision outcome under the rule below; never an ordinary negative or empty set. |

Categorical scoring requires exactly the task labels, finite probabilities in range, and the core's `1e-9` near-unit sum tolerance. Rounded maps may fail it. Do not silently normalize them or relax Validator. Retain nonconforming but finite maps as categorical observations; an explicitly justified numerical transformation is a separate experiment with preparation evidence. Marginals need all label keys, not a unit sum.

Scored probabilities and scored confidence each obey an all-or-none rule across the selected prediction artifact. Per-row observations can be absent. Prefer a full-population hard-outcome artifact with partial signals retained as observations. Optionally create a separately named complete-signal cohort, using a disclosed fixed selection for both arms; do not silently drop missing-signal cases or claim its score for the full population. JSON null is not a substitute for omitting optional typed fields.

A preparation descriptor is required when deriving outcomes from scalars or changing numeric values beyond the core normalization. Record method, version, configuration, and evidence indexes binding the adapter source, raw capture, and preparation receipt. Record actual provider-returned model versions when available; otherwise retain the requested alias and explicitly disclose the missing resolved version. Never label a gateway alias as a known concrete model version.

### Capture, errors, and provenance

Each adapter has an example-specific upstream schema and a proposed common capture envelope, **not** a new Validator input schema:

```text
case_id, family_id, arm, attempt, status
request_body: actual model/state/questions, with no authentication headers
response_body: exact available provider JSON or explicit absence
application_output: actual returned object
producer/configuration identities, timestamps, truncation/cache/retry metadata
```

Keep the approved reference sidecar separate from model capture. Save all attempts, not only favorable ones. Mocks belong in `adapter-fixtures/` and must never be mixed with observed predictions in `captures/` or reported as Jev accuracy.

The default full-population accounting is: a settled gold case with an attempted but failed model decision becomes explicit whole-episode abstention with a reason such as `upstream_timeout`, provided the preparation contract declares this representation. It is an adapter-recorded no-decision, not a claim that Jev intentionally abstained. Record transport and parsing failures separately in the review. An intentionally unattempted case or a truncated batch is not silently filled in; either resume the declared attempt protocol or stop and disclose incomplete capture.

Adapter schema failures, duplicate/unknown IDs, unauthorized label mappings, missing gold approval, and corrupt provenance stop preparation. Repeated partial multi-label heads do not become an answered empty set. Retry policies and any complete-signal cohort are fixed before capture, not chosen after seeing correctness.

Synthetic cases should contain no real credentials, private work histories, customer documents, or hidden production data. Do not enable public services, file-moving apps, coding workers, trading actions, or remote telemetry as a side effect of capture. Verify upstream code/data licensing and provider publication conditions before redistributing artifacts. A public repository alone does not establish a data license.

### Proposed deliverables per example

Future implementation location: `examples/jev-poc/<slug>/`. These paths are proposed deliverables, not existing scripts:

```text
README.md                      exact setup and offline/live commands
rubric.md                      task definition and human review rules
authoring/cases.jsonl           fictional source cases and approval trail
source-id-map.json              stable identities
split.json                     family-based frozen populations
capture.<py|ts|js>              thin, bounded adapter around the actual application
prepare.py                     deterministic canonical conversion
adapter-fixtures/              clearly mocked positive/negative schema tests
prepared/<task>/golden.json
prepared/<task>/<arm>.predictions.json
prepared/<task>/<partition>.config.json
captures/<arm>/                actual outputs and request evidence, normally ignored
reviews/                       run -> review -> recommend only
```

Do not introduce a general capture framework before two adapters demonstrate shared needs. Reuse one small preparation helper only for mechanically identical ID/hash/source serialization. Domain decisions remain explicit per example. Index new durable docs and evidence when implementation is authorized.

### Running and checking each experiment

After implementing preparation, create an existing parent output directory and unique nonexisting run destinations. The following are the **existing** CLI forms; `TASK`, `OUT`, and `EPISODE_UUID` must be resolved by the operator:

```sh
validator check --dataset "$TASK/golden.json" --predictions "$TASK/baseline.predictions.json" --config "$TASK/development.config.json"
validator evaluate --dataset "$TASK/golden.json" --predictions "$TASK/baseline.predictions.json" --config "$TASK/development.config.json" --out "$OUT/baseline" > "$OUT/baseline.receipt.json"
validator evaluate --dataset "$TASK/golden.json" --predictions "$TASK/candidate.predictions.json" --config "$TASK/development.config.json" --out "$OUT/candidate" > "$OUT/candidate.receipt.json"
validator inspect --run "$OUT/baseline" --episode "$EPISODE_UUID"
validator compare --baseline "$OUT/baseline" --candidate "$OUT/candidate" --out "$OUT/comparison" > "$OUT/comparison.receipt.json"
```

For a subset, the prediction IDs must exactly equal selected gold IDs while remaining bound to the whole golden file. Held-out runs use their own `role: held_out` configuration and are compared only to compatible held-out runs. Default paired comparison is preferable; `--intersection` is not a repair for differing golden files, tasks, roles, or concealed missing cases.

Each implementation must prove a small independent accounting oracle before live capture. For binary gold `[positive, positive, negative, negative]` and predictions `[positive, negative, positive, negative]`, require TP=TN=FP=FN=1 and accuracy/precision/recall=0.5. For multi-label gold `[{A},{A,B},{}]` and predictions `[{A},{B},{}]`, require exact-set accuracy 2/3 and Hamming loss 1/6. These are adapter fixtures, not model results. Add abstention, empty selection, threshold equality, missing-key, duplicate-ID, and golden-digest-change tests.

Completion requires exact row accounting, schema-conforming artifacts, independently checked counts, receipt/hash checks, explicit inspection of representative disagreements, and relocation/replay verification. Preserve the complete run bundles and do not overwrite evidence. Exit 0 means measurement succeeded, not that the model passed a quality requirement. Read metric status, population, and denominator before summarizing.

The skill's report should identify the measured task and population; raw/final metrics and coverage; recovered/regressed cases; operational failures; hypotheses with supporting case IDs; and one next experiment. It must not edit gold, implement fixes, create issues, assign work, or manage a project. No overall deployment verdict follows from these small diagnostics.

Live capture is a separate operator decision. Begin with at most five smoke cases, inspect capture fidelity, then run the declared population. Set explicit request, retry, time, token, and spend limits using the chosen provider; estimated costs are not guarantees. Threshold-only comparisons should reuse saved predictions without additional model calls.

## 1. jgrep: code-change screening

**Source pin:** `keltokhy/jgrep@fdceb6bdf79165a133667b8e57f3b7244545f0a2`.

Sources: [measurement code](https://github.com/keltokhy/jgrep/blob/fdceb6bdf79165a133667b8e57f3b7244545f0a2/bench/code_review.py), [case fixture](https://github.com/keltokhy/jgrep/blob/fdceb6bdf79165a133667b8e57f3b7244545f0a2/bench/fixtures/code_review.json), [saved results](https://github.com/keltokhy/jgrep/blob/fdceb6bdf79165a133667b8e57f3b7244545f0a2/docs/benchmarks/code-review.json), [experiment limitations](https://github.com/keltokhy/jgrep/blob/fdceb6bdf79165a133667b8e57f3b7244545f0a2/docs/CODE_REVIEW.md).

### Decision and inferred schemas

Primary task: did a code change introduce silent ignoring of a persistent-write or rollback failure that was previously handled? One episode is one before/after case, not one output line.

```text
Source case: {id, before: string, after: string,
              diff_relevant: boolean, function_relevant: boolean, reason: string}
Saved result: {fixture_sha256, requested_model, runs: [{mode, task, threshold,
               meter, rows: [{case, file, line?, text, p, ...}], ...}]}
Canonical task: single_label; labels = ["not_introduced", "introduced"]
```

The measurement script emits all scored records using `-p 0`, then flags a case when any associated record has `p >= 0.5`. Join by `case`, never by array order. Verify the fixture digest before interpreting a result file. Treat existing summary counts as a cross-check, not the source of per-case predictions.

### Gold and dataset construction

Stage A replays the 20 existing author-labeled cases unchanged, labeled explicitly as an author-fixture diagnostic. Review their assumptions before treating them as owner-approved gold. Do not silently correct labels while claiming to reproduce the original experiment.

Optional Stage B creates 48 new cases: 12 scenario families with four variants each; nine families/36 cases for development and three/12 held out. Families cover removed return/log checks, transaction commits, rollback, unchanged preexisting bugs, best-effort cleanup, rendering-only errors, deleted functions, and preserved error propagation. Give reviewers the explicit assumption that named helper functions return errors. New cases must be coherent Go excerpts; they need not pretend to be whole compilable applications.

Examples: replacing `return SaveConfig()` with `SaveConfig(); return nil` is positive; replacing an explicit error-return branch with `return SaveConfig()` is negative. Merely changing a comment around an existing ignored error is negative for this change task, although it can be positive for the separate current-function task.

### Adapter and experiment

Prepare one golden file from the before/after source cases. Convert each saved diff arm (`diff_lines`, `added_lines`, `diff_hunks`) into a prediction artifact using the original any-record rule. Preserve individual probabilities and rendering parameters as evidence; do not submit their maximum as a case-level probability. Source configuration records mode, question wording, aggregation threshold, record counts, and the actual recorded model.

A case with zero records may be a legitimate no-match only when successful input enumeration proves it has no eligible records under that mode. Missing output caused by failed scanning is not a negative. If the saved artifact cannot establish that distinction, disclose the replay limitation and obtain a controlled capture before stronger claims.

Compare diff lines to complete hunks on the same 20-case gold. This is a measured configuration contrast, not an isolated context-size ablation: framing also differs. For Stage B, vary only one of threshold or framing. The current-function task, if added, uses its own golden dataset and `function_relevant` labels.

### Acceptance and useful result

Every case appears once in each prepared arm; source rows all map to known cases; aggregation reproduces independently computed TP/FP/FN/TN and known mismatches. Inspect best-effort cleanup and rendering-error negatives as well as introduced-error positives. The saved-data replay needs no Jev key. A new capture uses the actual scanner, not a fabricated reimplementation of its answers.

Review outcome: identify which false positives are caused by question ambiguity or inadequate input units, recommend one controlled change, and preserve any regressions. No claim of general bug detection or production recall.

## 2. Jev Sift: relevance screening

**Source pin:** `kbhuw/jev-sift@966de12e2bb5f94d47886ee51f30a07ec8ef1607`.

Sources: [core classify contract](https://github.com/kbhuw/jev-sift/blob/966de12e2bb5f94d47886ee51f30a07ec8ef1607/src/classify.js), [native provider mapping](https://github.com/kbhuw/jev-sift/blob/966de12e2bb5f94d47886ee51f30a07ec8ef1607/src/provider.js), [usage and limits](https://github.com/kbhuw/jev-sift/blob/966de12e2bb5f94d47886ee51f30a07ec8ef1607/README.md).

### Decision and inferred schemas

Fix one relevance question: does the text provide evidence that the described company currently sells software to hospitals?

```text
Application input: {query: string, items: [{id: string, text: string}]}
Provider request: {model, state: text, questions: {relevant: {type: "noul", instructions}}}
Application output: {results: [{id, answers: {relevant: {type: "boolean", probability}},
                     model?, truncated?} | {id, error}], usage, ...}
Canonical task: single_label; ["irrelevant", "relevant"]
```

Use inline frozen text first; no URL fetch, file traversal, MCP installation, or relevance-judging of unseen tool results is needed. One episode is a query/content pair. `formatOutput`'s 0.5 yes/no display is a baseline threshold, not a proved production policy.

### Gold and dataset construction

Propose 80 descriptions from 20 fictional company families, four matched variants per family. Split 15 families/60 development and five/20 held out. Balance intended positives and negatives by authoring facts, not by filtering on model responses.

Positive evidence explicitly describes an existing software offering sold or licensed to hospitals. Negatives include a hospital using third-party software, a generic IT consultant without such an offering, a vendor explicitly excluding hospitals, historical plans rather than current offerings, and keyword-only mentions. Withhold descriptions whose business relationship is genuinely indeterminate under the rubric.

Example pair: “We license ward scheduling software to hospitals” versus “Our hospital licenses ward scheduling software from another company.” Include negation, irrelevant appended instructions, and terse paraphrases without leaking the label through names or metadata.

### Adapter and experiment

Call the existing `classify` core with the actual provider and a capture wrapper. Persist the response for every item and the exact generated relevance instruction. Map `p >= 0.5` to `relevant`; preserve p and optionally construct the complementary two-class distribution. Record the preparation descriptor. A timeout is a no-decision, not `irrelevant`.

Baseline: original query shorthand at 0.5. First candidate: the same captures at 0.7, purely to expose precision/recall tradeoffs; do not assume improvement. If a question change is more appropriate, make it a separately named later arm with new captures and unchanged gold. Short primary inputs avoid the 60,000-character cap; truncation/fetch failures belong to separate adapter challenge fixtures, not silently mixed full-context gold.

### Acceptance and useful result

Match every result by ID despite concurrency; prove equality at thresholds and the complement mapping; retain all provider errors. Review false negatives as missed evidence and false positives as unnecessary reads. A real model run must exercise the native `noul` to public `boolean` mapping. Report task accuracy, positive precision/recall, and coverage, not downstream agent success.

## 3. Foreman: multilabel job-state assessment

**Source pin:** `thruwire/foreman@a7d21d18d306a0cb9f3e15acefbdb5663521405c`.

Sources: [Jev questions and parser](https://github.com/thruwire/foreman/blob/a7d21d18d306a0cb9f3e15acefbdb5663521405c/src/foreman/foreman/jev.py), [observation model](https://github.com/thruwire/foreman/blob/a7d21d18d306a0cb9f3e15acefbdb5663521405c/src/foreman/observation.py), [application boundary](https://github.com/thruwire/foreman/blob/a7d21d18d306a0cb9f3e15acefbdb5663521405c/README.md).

### Decision and inferred schemas

One episode is one frozen observation. Use all ten assessment names as the ordered multi-label vocabulary:

```text
implementation_complete, tests_sufficient, requirements_satisfied,
needs_verification, meaningful_progress, worker_stuck, work_off_track,
agents_md_drift, ready_to_finish, needs_human
```

`FactoryObservation` requires `original_job`, `run_id`, `factory_status`, `iteration`, `active_workers`, `worker_history`, `latest_worker_output`, `worker_exit_status`, `worker_elapsed_seconds`, `git_status`, `git_diff`, `changed_files`, `test_results`, `verification_results`, `recent_events`, `previous_assessment`, `previous_intervention`, `attempts`, `failures`, and `elapsed_factory_seconds`; repository instruction fields are also available. Construct and validate this upstream type, rather than passing a guessed minimal dictionary.

The provider receives `observation.model_dump(mode="json")` and ten Noul questions. The application returns `FactoryAssessment` with one number per name. Capture the raw response too: the parser clamps numeric values into range, so the final assessment alone cannot prove what the provider emitted.

### Gold and dataset construction

Create 20 fictional jobs with three snapshots each: 60 episodes. Split by job, 15/45 development and five/15 held out. Include a completed implementation with inadequate tests, a genuinely finished job, repeated identical errors without progress, useful long-running work, unrelated edits, explicit denied permission, and an instruction violation. Author enough positive and negative cases for every dimension; disclose any dimension with no support rather than quoting a meaningful-looking macro score.

A proposed rubric distinguishes code completion, requirement completion, and sufficient independent verification. A worker saying “done” is not proof of any of them. `needs_human` requires evidence of missing authority, credentials, unresolved requirements, or another human-only dependency. Do not force insufficient evidence to be a negative unless the named question explicitly defines that behavior, as the current instruction-drift question does. Withhold an entire primary multi-label case if any required reference remains unresolved.

Use initial independent observations with null previous assessments, or record genuine earlier model observations; never populate previous assessments from gold. Review at decision time, not using later worker success.

### Adapter and experiment

Invoke `JevForemanModel.assess` on prepared observations only. Do not launch the runtime, workers, steering, verification agents, or subprocess commands. Capture all ten probabilities and map a complete assessment to marginals. The first baseline label set uses `>= 0.5` for all ten semantic propositions. This is an explicit diagnostic projection, not Foreman's production action policy.

Candidate: change only `ready_to_finish` to threshold 0.8 using Validator's `label_thresholds`, keeping all other thresholds at 0.5. Reuse captures. This asks whether the stricter completion signal reduces false completion at the cost of recall; it does not demonstrate improved worker behavior. A later intervention-outcome task would require separate references and stateful policy capture.

### Acceptance and useful result

Require ten reviewed truth values and ten valid marginals per answered case. Preserve raw versus clamped values, with a numerical-preparation descriptor if clamping occurred; invalid raw evidence is never silently certified. Test missing heads, out-of-range captures, exact threshold equality, and whole-episode failure. Report per-label binary counts, exact-set agreement, micro/macro metrics with statuses, and dangerous completion false positives. No set confidence and no claim that the software factory is safe.

## 4. Upwork: apply/review/skip triage

**Source pin:** `ekkyarmandi/jev-upwork-job-classification@bb072c82b390e74f66587e698f50006cb761c614`.

Sources: [state construction, questions, Verdict and composition](https://github.com/ekkyarmandi/jev-upwork-job-classification/blob/bb072c82b390e74f66587e698f50006cb761c614/main.py), [inputs and intended use](https://github.com/ekkyarmandi/jev-upwork-job-classification/blob/bb072c82b390e74f66587e698f50006cb761c614/README.md).

### Decision and inferred schemas

```text
Entry: {job: {description, title?, budget?, skills?, competition?, ...}, client: {...}, ...}
Model state: {job, client, freelancer}, produced by build_state/prune
Answers: four Nouls, seven Scores, primary_lane Choice, work_type Choice
Verdict: {case_id, decision, fit, lane, lane_confidence, work_type,
          gates, normalized, hard, review, reason, model, error, answers, ...}
Canonical task: single_label; ["apply", "review", "skip"]
```

Use `build_state` and the existing questions/composition rather than manually replicating their transformations. Strip seed labels before provider projection. The output's `fit` is a weighted decision score; `lane_confidence` describes an intermediate lane, not confidence in apply/review/skip.

### Gold and dataset construction

Create a fictional freelancer profile matching the fields referenced by the pinned criteria (`core_stack`, `secondary_stack`, `weak_areas`, `verified_evidence`, and explicit rate/availability facts). Do not copy a real person's contract history. Freeze the profile and preference rubric.

Author 90 jobs in 30 families, with three distinct variants per family: 60 development and 30 held out. Aim for meaningful coverage of apply/review/skip, hard exclusions, uncertain requirements, senior scoping, conflicting lane/gate wording, and poor budget fit. Final labels are human preference judgments under the declared rubric, not objective employment truth or expected financial return.

Example: a clearly scoped, paid Python API integration within the profile's demonstrated skills and constraints may be `apply`; an explicitly unpaid custom implementation sample is `skip`; an otherwise plausible engagement with unresolved onsite/team requirements is `review`. Keep “review” as a real intended class, not an abstention. Do not set expected labels by running `compose` on Jev's own scores.

### Adapter and experiment

Capture the actual state, all answers, and the returned Verdict before presentation. Prepare final decisions as hard-only classifier outcomes. Retain Nouls as Bernoulli observations, Scores/fit as scalar observations, and Choice maps as categorical observations. No probability vector or confidence is invented for the final three classes. Record weights, gates, strict versus inclusive comparisons, rounding, and preparation code as evidence.

Baseline uses the pinned policy. Candidate changes only `FIT_APPLY` from 0.62 to 0.70, reusing the same saved answers through the actual composition code. Freeze that candidate using development evidence before testing holdout. This is a policy experiment; it must not be described as a model improvement. A question rewrite requires a different capture arm.

### Acceptance and useful result

Confirm that `false` and zero survive state pruning; test missing descriptive fields and the description truncation path. Cover exact boundaries for high-is-bad gates (`>`), low-is-bad gates (`<`), and final fit (`>=`). Treat operational `error` results as separately recorded no-decisions. Inspect which apply decisions became review and which genuine opportunities were lost. If primitive accuracy is desired later, create separately reviewed primitive tasks rather than reusing final-decision labels.

## 5. Tax documents: form and page-kind classification

**Source pin:** `kyotofin/tax-doc-classifier@3e95a77f763c6becb78472f8b2ce2f54237f9214`.

Sources: [classification types and hierarchy](https://github.com/kyotofin/tax-doc-classifier/blob/3e95a77f763c6becb78472f8b2ce2f54237f9214/src/classify.ts), [criteria registry](https://github.com/kyotofin/tax-doc-classifier/blob/3e95a77f763c6becb78472f8b2ce2f54237f9214/data/criteria.json), [evaluation scope](https://github.com/kyotofin/tax-doc-classifier/blob/3e95a77f763c6becb78472f8b2ce2f54237f9214/README.md).

### Decision and inferred schemas

```text
Input: classifyPage(lines: string[], {backend, criteria, gate?, firstList?})
PageResult: {form, kind, formConfidence, kindConfidence, stepConfidences,
             gated, calls, inputTokens, probabilities: {form, kind, sub?}}
Form task: single_label, frozen registry IDs plus explicit not_in_this_list and blank
Kind task: single_label, seven KINDS values from source
```

The source has a critical distinction: `formConfidence` is the minimum of selected per-stage probabilities; it is not Jev's reported Choice confidence. Form/sub-form probability maps use different option spaces. `best()` excludes `not_in_this_list` when choosing a form. These are observed code behaviors to test, not assumptions to correct silently in the adapter.

### Gold and dataset construction

Avoid PDF/OCR setup for the first PoC. Author fictional extracted text lines shaped like documents, with no real taxpayer data. Verify six selected form IDs against the pinned registry before freezing: proposed IDs are `form-1040`, `form-1040-schedule-a`, `form-w-2`, `form-1099-int`, `form-5471`, and `form-5471-schedule-j`.

Create 48 known-form cases, eight per form, plus 12 explicit out-of-registry cases: 60 total, split by templates/families into 40 development and 20 held out. Printed identity in header/footer is legitimate evidence, not leakage. Include parent/schedule distinctions, continuation pages, misleading references to other forms, and instructions. Genuinely unidentified continuation pages are withheld, not assigned by guessing. The unknown cases deliberately test whether a reject/unknown option is honored.

An optional separate 42-case page-kind set contains six examples per source kind: form_page, instructions, blank, cover_sheet, state_tax_form, broker_or_bank_statement, letter_or_other. Split templates into 28/14. A reviewer resolves overlapping cover/letter definitions explicitly. These are synthetic document-identification cases, not claims about legal forms or tax-return correctness.

### Adapter and experiment

Invoke `classifyPage` with text lines and a recording wrapper around `Backend.ask`. Preserve both hierarchy requests. Map final `form` to a class and preserve stage vectors and `formConfidence` as observations; do not synthesize one flat final-form distribution.

Baseline is the application's selected form. Candidate is a separately declared abstaining projection: retain the final form only when `formConfidence >= 0.95`, otherwise explicit abstention. Perform this projection in preparation with provenance; do not mislabel the minimum stage probability as native Jev confidence. This separates incorrect answers from withheld answers without a core change.

A later, separately named code experiment can honor `not_in_this_list` when it dominates. Do not merge it with the gate experiment. Page-kind scoring can use the complete native kind distribution when valid and available on every selected row; the blank shortcut's `{blank: 1}` map is not automatically a complete seven-class scoring vector. Keep full-population hard results and a disclosed model-called cohort if needed.

### Acceptance and useful result

Cover hierarchy transitions, missing sub-results, blank shortcuts, out-of-registry inputs, and equality at the gate. Record model bypasses (`calls == 0`) as deterministic application decisions, not Jev predictions. Inspect known-form errors, unknowns forced into known forms, and low-probability correct answers. Report conditional gate coverage; do not equate all gate rejections with wrong answers. Public corpus replay can follow after licensing and extraction provenance are established.

## 6. classifier.dev: fixed-taxonomy classification

**Source pin:** `mrmps/classifier-dev@33ca63816f2bc7e93c3f2d0715f7896500370739`.

Sources: [Jev request/result implementation](https://github.com/mrmps/classifier-dev/blob/33ca63816f2bc7e93c3f2d0715f7896500370739/src/jev.ts), [existing evaluation cautions](https://github.com/mrmps/classifier-dev/blob/33ca63816f2bc7e93c3f2d0715f7896500370739/eval/README.md).

### Decision and inferred schemas

```text
jevClassify(keys, inputs: string[], labels: string[], instructions?, multi: boolean, meter?)
JevResult: {label: string, confidence: number, scores: Record<string, number>, model: string}
Native single-label: Choice per state item
Native multi-label: Noul per item/label; scores rounded by application
```

For multi-label, `JevResult.label` is only the highest-scoring label. It is not the predicted set. The application output's multi-label `confidence` is that label's score, not native set confidence. A faithful adapter must use the complete score map and the declared threshold, and must not copy this confidence into Validator's multi-label format.

### Gold and dataset construction

Primary: 120 original support messages for one fixed taxonomy, `billing`, `technical`, `account_access`, `feature_request`. Construct 30 scenario families of four matched variants; keep 20 families/80 cases in development and ten/40 held out. Aim for 30 intended cases per class through predeclared stratification; a family may contain fact-changing variants with different labels. Preserve family grouping even if the final class balance is imperfect.

Use a rule for the primary requested action: a login failure is account access even if it mentions billing; a request for an unimplemented capability is a feature request rather than a broken promised feature. Withhold truly equal mixed requests from this single-label task.

Optional multi-label extension: 96 messages, 24 families of four, over a single fixed label universe `refund_requested`, `login_problem`, `bug_report`, `feature_request`; 16/64 development and eight/32 held out. Fully review all four conditions. Include no-label, one-label, and overlapping-label cases. Do not pool the upstream seven unrelated taxonomies into a fake shared classification task.

### Adapter and experiment

Run the local exported Jev integration or record equivalent calls from the pinned application. Do not benchmark an unauthenticated public endpoint at scale. Save request groups, order, gateway/direct route, returned model, and full native responses before application rounding where available.

Single-label baseline: unchanged general question. Candidate: only a more explicit primary-action instruction; keep gold, label names/order, batching, and provider fixed. Native complete valid vectors can be scored; rounded non-unit maps remain observations under the shared rule.

Multi-label baseline: sets derived from every `scores[label] >= 0.7`, matching the pinned threshold. Candidate: 0.5, using saved marginals, to expose recall/precision tradeoffs. Do not treat `label` as the set or the native top confidence as set confidence.

Optional fallback experiment is distinct: reuse an identical frozen Jev pass and invoke the fallback only for its declared low-confidence IDs, retaining the actual producing model per row. Compare to independent gold, not to agreement with Jev. A second provider budget is required and is not part of the first PoC.

### Acceptance and useful result

Check request-order reconstruction, complete score maps, all-none signal rules, application rounding provenance, and model-route attribution. Compare native observations with post-rounding decisions at exact boundaries. Report per-class disagreements for single-label and exact-set/per-label/micro/macro results for multi-label. Upstream per-case averaged F1 is not interchangeable with Validator's label-macro or micro F1; explain rather than force numeric equality.

## 7. Local document filing

**Source pin:** `Charlyhno-eng/jev-document-classification@17cdc20acb407e7cfb623ef47d32e1b8568a5c86`.

Sources: [classification interface and request construction](https://github.com/Charlyhno-eng/jev-document-classification/blob/17cdc20acb407e7cfb623ef47d32e1b8568a5c86/server/classification.ts), [document profile](https://github.com/Charlyhno-eng/jev-document-classification/blob/17cdc20acb407e7cfb623ef47d32e1b8568a5c86/server/document-profile.ts), [filing policy](https://github.com/Charlyhno-eng/jev-document-classification/blob/17cdc20acb407e7cfb623ef47d32e1b8568a5c86/shared/document-policy.ts).

### Decision and inferred schemas

```text
ClassificationInput: {apiKey, fileName, text, categories: string[]}
ClassificationResult: {category, categoryConfidence: number|null, destinationCategory,
                       needsReview, confidentiality, promptInjectionScore,
                       promptInjectionRisk, subject, usage, cost, ...}
Canonical category task: single_label, exact configured folder labels
```

The source uses Choice for all four questions, including `promptInjectionScore`, which selects a string from 0 to 100 in steps of ten. It is not a Score primitive. `categoryConfidence` is the selected category's probability, not Choice's separate confidence. `subject` chooses from per-document candidates and is not a fixed global taxonomy.

### Gold and dataset construction

Fix four categories: `Invoices`, `Contracts`, `Reports`, `Correspondence`. Create 80 fictional text documents from 20 families/four variants; 15 families/60 development and five/20 held out. Include short documents, long documents with decisive evidence in the middle/end, attachments described in prose, templates quoted inside other documents, and category keyword distractors. Define category by the document's main purpose, not any word it contains. Ambiguous equal-purpose composites are withheld.

Suggested reference examples: an itemized request for payment is an invoice; negotiated obligations between parties are a contract; a status summary is a report; a cover email discussing an attached invoice remains correspondence. Use neutral filenames unless filename semantics are deliberately part of the documented input.

Keep a separate small operational challenge pack for unsupported/unreadable/empty input. A further binary injection task needs its own reviewed rubric and gold; numeric risk scores or folder names alone are not ground truth.

### Adapter and experiment

Call `classifyDocument` directly with in-memory text and the actual default evaluation path. Do not run the file-moving UI/server workflow on real files. Exclude `apiKey` from every stored object. Capture the exact generated profile, category criteria, output, and provider response when instrumentation permits; the returned application object alone does not preserve the full distribution.

Baseline is raw `category` under `as_recorded`. A second artifact represents the application's actual disposition on that same category task: `Need review` and prompt-injection quarantine mean explicit abstention with different reasons, not new semantic categories. This is coverage accounting relative to intended filing category, not a claim that quarantine itself was wrong.

First candidate: change only the category review threshold from 0.75 to 0.90 in an external disclosed policy replay, retaining the application's security-priority behavior. Use saved selected-option probabilities; do not substitute maximum probability or fabricate missing probabilities. The application's current missing-probability behavior is to keep the category; preserve that baseline and flag it for review rather than silently changing it. A different missing-signal policy is a separate candidate.

### Acceptance and useful result

Distinguish semantic category, disposition, and operational failure. Test threshold equality, missing probability, and precedence of security quarantine over ordinary review. Full-population artifacts omit scored confidence when values are missing; available selected probabilities can remain per-row scalar observations. Compare raw mistakes with mistakes among automatically filed documents and quantify review coverage. Future profile or batch experiments keep logical source documents frozen and record changed effective requests; they do not rewrite gold.

## 8. fast-jev-compaction: retention decisions

**Source pin:** `tamaratran/fast-jev-compaction@e3f262a7f4d42bd8dd32ced30d26176f7cb545b0`.

Sources: [native types](https://github.com/tamaratran/fast-jev-compaction/blob/e3f262a7f4d42bd8dd32ced30d26176f7cb545b0/src/types.ts), [compaction implementation](https://github.com/tamaratran/fast-jev-compaction/blob/e3f262a7f4d42bd8dd32ced30d26176f7cb545b0/src/compact.ts), [state construction](https://github.com/tamaratran/fast-jev-compaction/blob/e3f262a7f4d42bd8dd32ced30d26176f7cb545b0/src/state.ts), [documented policy](https://github.com/tamaratran/fast-jev-compaction/blob/e3f262a7f4d42bd8dd32ced30d26176f7cb545b0/README.md).

### Decision and inferred schemas

```text
Message: {role: "user"|"assistant", text, toolUses: [{tool_use_id, tool, input}],
          toolResults?: [{tool_use_id, text, isError?}]}
CallDecision: {id, tool, keepCall, keepResult,
               action: "keep"|"drop_result"|"drop_call", reason}
CompactResult: {messages, decisions, stats}
Canonical primary task: single_label; ["keep", "drop_result", "drop_call"]
```

Use a stable `(transcript_id, tool_use_id)` source key, preserving the mapping to transient `t1`-style IDs. Pinned calls are deterministic policy decisions and do not count as independent Jev-classified cases.

### Gold and dataset construction

Create 20 fictional transcripts with six eligible calls each: 120 cases, split by transcript into 90 development and 30 held out. Place additional recent/first-message calls outside the eligible count to exercise pinning. Each transcript has an explicit current goal, current repository state, and known availability of repeating tools. No private conversation data is needed.

Reviewers assign the least destructive action consistent with the declared current goal: `keep` when exact result content is still needed and cannot safely be recovered; `drop_result` when call history and the retained head are sufficient but the complete result is not; `drop_call` when neither call nor result contributes unresolved evidence. Withhold genuinely incomparable acceptable actions rather than forcing a single winner.

Include a unique unrecoverable error code, a superseded file read, a resolved old failure, an irrelevant directory listing, repeated equivalent observations, and a required fact beyond the 300-character retained head. Reviewers see the full application transcript and separately record what Jev sees: full result bodies are omitted from its constructed state. Do not use later task outcomes as though they were known at compaction time.

### Adapter and experiment

Use `compact(messages, recordingAsker, options)` without installing session hooks. Save fitted state, questions, raw answers, resolved options, decisions, and resulting messages. Match outputs to eligible tool calls exactly. Preserve `keepCall` and `keepResult` as Bernoulli observations; do not manufacture probabilities over the three actions.

Baseline threshold is 0.5. Candidate lowers it to 0.35 and replays the actual decision function on saved answers, conservatively retaining more. Higher keep thresholds retain less, not more. Hold pinning and `truncateHeadChars` constant. No new Jev call is needed for this threshold arm.

An optional separate two-label semantic task can use independently reviewed `call_needed` and `result_needed` with complete marginals. Its truth is not inferred from the primary action alone when the two question meanings differ.

### Acceptance and useful result

Test both threshold equalities, precedence of keepResult over keepCall, missing paired results, pinned calls, and no-orphan-call/result invariants. Inspect every gold-keep case predicted drop_result/drop_call and disclose transcript correlation. Report action confusion and retained-character counts separately; compression is not correctness. Retention-label agreement does not prove subsequent agent-task performance is preserved.

## 9. Jev router (optional)

**Source pin:** `gargpratyush/jev-router@38da6b84ea01241bfc41fbddc0928d0f40a703f0`.

Sources: [questions, exact-model options and thresholds](https://github.com/gargpratyush/jev-router/blob/38da6b84ea01241bfc41fbddc0928d0f40a703f0/src/config.mjs), [pure policy function](https://github.com/gargpratyush/jev-router/blob/38da6b84ea01241bfc41fbddc0928d0f40a703f0/src/policy.mjs), [capture/explanation behavior](https://github.com/gargpratyush/jev-router/blob/38da6b84ea01241bfc41fbddc0928d0f40a703f0/README.md).

### Decision and inferred schemas

`questionForModels(models)` asks a Choice over exact model IDs. The policy receives a tier-shaped view, not necessarily the original choice ID:

```text
decide({prompt, jev: {choice: tier, confidence}|null,
        current, available: tier[], contextTokens})
  -> {tier, reason, changed}
Current tier vocabulary: ["haiku", "sonnet", "opus", "fable"]
```

Capture the exact model catalogue and explicit model-ID-to-tier mapping. Do not treat exact model IDs as tier labels or copy a model-level probability vector into a tier task without an explicit transformation. Score-derived complexity diagnostics remain observations.

### Gold and dataset construction

First build 24 deterministic policy fixtures with hand-derived expected tier/reason/changed values. Cover overrides, null Jev, unavailable tier, low-confidence downgrade, capped upgrades, and long-context no-downgrade. These are software tests, not Jev accuracy measurements.

For semantic routing, author 60 prompts under a frozen three-tier catalogue with long/Fable disabled: 20 mechanical, 20 bounded engineering, and 20 ambiguous/cross-module tasks. Split families into 40 development and 20 held out. Human labels represent agreement with a declared routing rubric, not proof of the cheapest model that will succeed. Actual cheapest-successful-model claims require running candidate models against independent completion tests, which is outside this PoC.

### Adapter and experiment

Exercise `decide` directly for the offline fixtures without launching CLI proxies or reading account credentials. For live semantic capture, wrap the pinned router's actual request-building path after resolving its entry point and catalogue shape; do not substitute a newly invented prompt and call it the existing router. Capture raw exact-model choices, mapped tiers, and final policy outputs separately.

Baseline is the current `minConfidence: 0.3` policy. Candidate raises only that cutoff to 0.5 on saved calls. Keep contextTokens, current tier, available catalogue, and gold fixed. Explicit user overrides are a separate deterministic cohort; exclude them from claims about Jev classification skill.

### Acceptance and useful result

Prove exact boundary behavior at 0.3 and 20,000 context tokens and native-ID mapping. Missing/invalid Jev can still produce a valid application fallback tier: do not replace that real fallback with an abstention. Report semantic-rubric agreement and policy conformance separately. Do not claim cost savings or task success from tier-label accuracy.

## 10. jev-align export integration (optional)

**Source pin:** `sutro-sh/jev-align@49753df924d30c0d3642b58e0b9b1e89921dc102`.

Sources: [Story, TaskSpec, LabelRecord and Prediction](https://github.com/sutro-sh/jev-align/blob/49753df924d30c0d3642b58e0b9b1e89921dc102/src/jev_align/models.py), [prediction integration](https://github.com/sutro-sh/jev-align/blob/49753df924d30c0d3642b58e0b9b1e89921dc102/src/jev_align/jev.py), [workflow and human labeling](https://github.com/sutro-sh/jev-align/blob/49753df924d30c0d3642b58e0b9b1e89921dc102/README.md).

### Decision and inferred schemas

```text
Story: {id, row_number, fields: Record<string,string>}; state() returns fields
BinaryTaskSpec: {instructions, true_criteria, false_criteria}
LabelRecord: {story_id, label, rationale?, round_number, acquired_by,
              evaluation_split, acquisition_probability, resolved_model?, created_at}
Prediction: {story_id, probability?, probabilities?, choice?, confidence?,
             label_probabilities?, score?, score_probabilities?, resolved_model?}
```

Choose a fixed binary task first. Join on story_id; do not infer task type from whichever optional Prediction field happens to be non-null. Do not hand-edit the application's saved run state to manufacture labels or a successful optimization.

### Gold and dataset construction

Create 80 original aviation/not-aviation stories with fields `title` and `text`. Reserve 20 from distinct topic/template families as an external sealed test set; use 60 in the application's human-labeling workflow. Include airport operations, aircraft maintenance, metaphorical flight, unrelated transport, and explicit negation. Freeze a relevance rubric so aerospace mentions with no aviation content have an explicit treatment.

Only approved labels become gold. Acquisition through uncertainty sampling is not evidence of a representative workload. Preserve acquisition and round metadata outside requests. In case of multiple label revisions, require an explicit approved current-label export; do not choose whichever label best matches predictions or blindly use file order.

### Adapter and experiment

Export read-only snapshots of the task definition, stories, approved labels, and observed predictions for a baseline and one candidate. Candidate creation may use jev-align's existing human-reviewed optimization workflow, but Validator integration does not need to implement or manage GEPA. The external 20-case test set must remain unavailable to that optimizer, including its label rationales.

Map binary boolean gold to fixed negative/positive class labels and actual probabilities through the declared threshold. If no explicit hard prediction is exported, record the adapter's threshold as preparation. Preserve model versions separately per originating source. Future multiclass exports map Choice and matching vocabulary; future multilabel exports require complete label_probabilities and independently reviewed complete gold. Score exports remain out of scope unless converted into a separately specified classification task.

### Acceptance and useful result

Require task-version identity, one authoritative approved label per story, stable story/UUID mapping, compatible before/after populations, and no export of training-only gains as held-out performance. Test mixed optional fields, missing prediction rows, stale cached predictions bound to a different task definition, and changed class names. The useful deliverable is an external inspectable comparison that the existing workflow can consume, not another optimization framework or task manager.

## Evidence and handoff boundaries

The preceding source links establish implementation shapes, not performance measurements conducted here. The synthetic datasets, review trail, adapters, capture commands, and candidate results are future deliverables. Where a wrapper currently discards raw probabilities or resolved model IDs, instrumentation is required before claiming that provenance; missing metadata must remain missing.

For each selected implementation, finish with a compact evidence bundle and a run/review/recommend demonstration. Keep disputed reference semantics, unsupported task shapes, and operational failures explicit. A finding that the available input cannot support a reliable decision is useful; do not hide it by changing the gold, sampling only favorable cases, or weakening the core admission rules.

Official primitive semantics were checked against [TypeSafe primitives](https://docs.typesafe.ai/primitives) and [model/version guidance](https://docs.typesafe.ai/models) on 2026-09-20. The key distinction is structural: Noul returns a probability of yes, Choice returns a selected option with probabilities and separate confidence, and Score returns a position over ordered levels. Those shapes inform adapters; they do not supply reference truth.
