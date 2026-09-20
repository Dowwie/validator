# Validator workflow stabilization plan

Authority: **Proposed** execution plan, revised at the user's request on 2026-09-19.
The user commissioned this planning revision, not execution. When commissioned,
the work includes diagnosing, repairing, and rerunning Validator within its existing
contract. It does not automatically resume the old development team or T032–T035.
[Fizzy card #195](http://localhost:3006/1/cards/195) preserves the deferred acceptance
context; Fizzy owns operational state and subsequent execution commitments.

## Outcome and boundaries

Stabilize the newly built Validator by running complete classification workflows
from the saved evidence available across all six Chord workstreams. Select cases
for materially different Validator paths, not for project count. Existing evaluators
are fallible comparison evidence. Matching their totals is neither necessary nor
sufficient for correctness.

A selected supported workflow must complete preparation, `check`, `evaluate`,
`inspect`, and applicable `compare` operations with independently verified results.
A reproducible Validator defect enters a repair-and-rerun loop; documenting it does
not complete stabilization. Model mistakes are valid evaluation inputs, not Validator
failures. Unsupported upstream semantics remain explicit compatibility limits.

Use one agent initially. Reuse saved predictions, frozen reference versions, and
small existing fixtures. Preserve source runs and install repaired binaries alongside
the original. No provider calls, prompt tuning, new semantic labeling, production
cutover, general adapter framework, or full Chord audit. Semantic adjudication,
translation generation, ETL correctness, retrieval quality, and clustering quality
remain outside Validator unless already expressed as supported labeled decisions.
This is workflow integration and stabilization, not a steel thread.

## Evidence and trust

- [Verification record](../acceptance/validator-v1.md): installed 0.1.0 and accepted
  159-test build evidence; fresh operator workflows and full DoD were deferred.
- [Specification](../specs/validator-v1.md), [data model](../specs/validator-data-model.md),
  [architecture](../../architecture.md), and [schemas](../../schemas/v2/): governing
  behavior, implemented paths, and machine contracts.
- [Accepted input handoff](../dispatches/validator-completion/008-coordinator-to-owner.response.md),
  [Chord preparation](../../scripts/acceptance/prepare_chord.py), and
  [independent Chord oracle](../../scripts/acceptance/oracle_chord.py): accepted
  starting artifacts, not proof of a successful evaluation run.
- [Chord documentation](../../../chord/docs/README.md) and
  [artifact index](../../../chord/docs/artifact-index.md): source discovery and
  authority. Historical summaries do not establish current artifact availability.

Start with `/Users/dowwie/.local/share/validator/install/t027-178fccf38838/bin/validator`,
SHA-256 `4bacbc5947bb70218725916965439837b2afa53d16a04f37ef00a31509a85dde`.
Verify its identity before use. Bind every repaired candidate to source identity,
build commands, toolchain, and a new executable hash; retain the original baseline.

Establish expected accounting from frozen row-level targets and saved outcomes,
using specification formulas, independent integer counts, and small hand-checkable
examples. Reuse accepted independent oracles after checking their input bindings.
Do not import legacy scorer functions into the oracle or copy Validator output into
expected values. Independently recount legacy `correct` flags and aggregate scores.
Check preparation against raw evidence separately from checking arithmetic.

Reference labels define the declared test; they need not establish semantic truth.
Report agreement with historical or provisional labels as such. Preserve disputed,
withheld, and superseded judgments and distinguish each reference version. Never
change labels to resolve a scoring disagreement. Never compare different golden
versions as though their byte identities and populations were the same.

## Available workflow inventory and selection

The paths below were located through Chord's documentation and targeted file checks.
This is a planning inventory, not a completed digest or row-level source audit.
`Chord/` means `/Users/dowwie/MyProjects/chord/`.

| Workstream | Available evidence and fit | Selection and incremental benefit |
|---|---|---|
| Dialect translation | [Frozen dialect evaluations](../../../chord/docs/experiments/taxonomy_induction/competency_dialect/evaluation/), including `expected-rulings-v1.jsonl`, v2/v3 raw responses, comparisons, and manifests. The v3 report describes 34 target dispositions and v2's missing target. Dispositions can be classification outcomes; core equivalence and qualifier meaning still require semantic review. | Reserve candidate for disposition mapping or response-to-target alignment that exposes a path not covered below. Do not automatically replay all prompt versions. Fixture exposure makes these development cases, not held-out evidence. |
| Gold-corpus / ETL | [Phrase Gold v2 manifest](../../../chord/data/taxonomy/production/phrase_gold_bundle_v2/manifest.json) preserves phrase/evidence inputs. [Accepted competency golden v2](../../../chord/data/taxonomy/experiments/competency-golden-dataset-v2/accepted/manifest.json) is a separate phrase-only calibration reference with corrected setting judgments. A corpus or reference alone is not a prediction/reference pair. | Use source/evidence bindings where a selected case needs them. No standalone classification run without saved compatible outcomes. Do not turn ETL invariants or free-text references into invented class labels. |
| L3 taxonomy induction | [Preserved lab bundle](../../../chord/data/taxonomy/experiments/l3-classifier-lab-v2/l3_v2_golden_bundle/) contains reviewed stages and candidate reports; its pass-3 baseline remains pending owner review in the index. [Clustering E4 evidence](../../../chord/docs/experiments/competency-clustering-lab-e4.md) has saved partitions and exposed pair controls, not complete independent group truth. | Conditional pair-decision replay only if a saved partition-to-pair conversion exercises an uncovered path. Do not encode arbitrary cluster IDs as classes or treat candidate semantic reports as ratified labels. |
| V2 adjudicator | Prepared native Choice/Score630 bundle plus the [24-case comparison-first study](../../../chord/docs/experiments/v2-comparison-first-adjudicator-study.md) and its frozen raw arms/key. [Full630 evidence](../../../chord/docs/experiments/typesafe-full630-evaluation.md) retains probability-bearing responses and strict numerical failures. | Select native630 as the first complete workflow. Use the 24-case study only if the failure/provenance path is not covered faithfully by the resolver. Use authenticated probability-bearing records for distinct signal paths when they meet the contract. |
| Resolver validation | [Step12 judge study](../../../chord/docs/experiments/step12_resolver/2026-07-30_judge_precision.md) points to `data/validation/judge_sample_verdicts_deepseek-v4-{flash,pro}.jsonl`. Both exist with 250 rows and 132 distinct historical reference labels; Flash has eight `unparseable` rows. Records retain labels, judge outputs, baseline top-1 and raw evidence. | Select as the next real workflow: many-class sparse support, assignment/residue semantics, and unreadable-response accounting differ from three-class V2. These are historical-label agreement tests. Unlabeled production pilot records cannot supply accuracy targets. |
| Deduplication / model validation | [Semantic deduplication alignment](../../../chord/docs/plans/align-typesafe-semantic-deduplication.md) and [restored Score630 evidence](../../../chord/docs/experiments/typesafe-restored-score630-evaluation.md) overlap the selected V2 family. Other representative/challenge pair studies are routed through Chord's index. | Count overlapping artifacts once. Add a saved pair study only for a demonstrated new path, such as a distinct supported source profile or population condition; changed models or subject matter alone do not justify another run. |

The resolver study and its per-row files are discoverable through the resolution
guide but lack exact entries in Chord's artifact index. Preserve the direct source
links in this plan; report that external index gap without editing Chord's docs.

## Coverage obligations

Before conversion, bind each row below to exact input/configuration artifacts and
named implementation or existing test paths. Record observed coverage in the run
report. These are behavioral obligations, not a claim of measured line coverage.

| Validator path | Cheapest planned evidence | Required distinction |
|---|---|---|
| Strict admission, UUID alignment, selected population, source/evidence preparation (`validation`, `artifacts`, `app`) | Native630; resolver conversion | Full versus subset IDs; withheld references versus unanswered predictions; exact evidence hashes and per-source provenance. |
| Single-label hard accounting (`evaluation/single_label`) | Native630 plus resolver | Three classes versus many sparse classes; class, explicit domain residue, and no decision; exact matrix, support, coverage and metric statuses. |
| Observations versus scored probabilities/confidence | Native630 observations plus a separately declared valid signal case | Auxiliary scalar/Noul/categorical observations must not activate scoring. Categorical losses, confidence/maximum-probability bins and their populations require top-level admitted signals. |
| Source profiles and single-label policy | Saved signals when faithful; otherwise existing [public single-label fixture](../../tests/fixtures/steel-thread/) and conformance cases | `classifier` versus `scored_choice`; `as_recorded` versus `reject_below` with confidence and maximum-probability signals; equality at the threshold, raw/final separation, and policy prerequisite rejection. |
| Multi-label accounting and marginals (`evaluation/multi_label`) | [Accepted two-episode fixture](../../tests/fixtures/acceptance-multi/manifest.json) | Overlapping sets, answered empty set, whole-episode abstention, marginal losses/bins and label transitions. This is synthetic coverage, not a real Chord replacement claim. |
| Multi-label policy | Separate configuration over the fixture's answered baseline, reusing existing conformance expectations | `label_thresholds`, including equality and empty resulting set; raw abstention is invalid for this policy. Never alter the accepted fixture in place. |
| Verified replay and comparison (`app`, `comparison`) | Each selected real workflow and both public task kinds | Independent repeat run, episode inspection, identical-population comparison, explicit intersection, exclusions, and recomputed transitions/deltas. |
| Empty/absent signals, compatibility and integrity failures | Existing conformance/CLI cases plus small isolated fixture derivatives only where process coverage is absent | `no_data` versus `not_applicable`; empty selection/intersection; default rejection of unequal populations; digest/evidence tampering; existing destination preserved; structured errors with no published success artifact. |

Prefer reuse of existing public conformance inputs and independent expectations over
new fixture construction. Materialize only what is needed to exercise an uncovered
path through the executable. Keep synthetic policy/stress variants distinct from
historical replays; do not present changed policies as original model behavior.
Do not run every metric edge case again merely to increase a coverage number.

## Execution sequence

### 1. Freeze the run set and contracts

Verify the binary, selected source manifests, references, IDs, row counts, and local
evidence availability. Resolve each selected workflow's label vocabulary, outcome
mapping, role, source profiles, and scoring signals before examining its aggregate
Validator result. Freeze minimal workflow-specific conversions and independent
expectations; preserve source-to-UUID mappings and transformation receipts.

Start with native630, resolver250, and the existing public single-/multi-label inputs
needed for signal and policy gaps. Review the remaining inventory once against the
coverage table. Promote a reserve case only with an exact new behavior and available
saved evidence; record why other cases are redundant or outside the contract.
If a selected case cannot be represented, disclose that boundary and continue useful
independent coverage. Substituting a case must not erase the original blocker.

For resolver250, verify cross-arm row identity from source records; do not assume
line order or phrase text alone is unique. Freeze the historical label vocabulary
and its treatment of residue. An explicit residue verdict is not a transport failure.
Preserve unreadable responses and their raw evidence. A declared preparation mapping
to whole-episode abstention may represent an absent decision, but must retain the
failure reason and must not masquerade as a model-issued abstention. If that mapping
violates the source or Validator contract, keep the case blocked; do not drop rows,
invent a class, or revive the deprecated label-adjudication worksheet. Resolver
`0.0` placeholder confidence must not become a scoring signal.

### 2. Complete native630 through the installed executable

Use `/Users/dowwie/.local/share/validator/acceptance/chord630/` unchanged: accepted
`golden.json`, Choice/Score predictions, full/subset configurations and predictions,
independent `expected.json`, and bound evidence. Run `check`, both full evaluations,
representative/supplement and targeted50 evaluations, inspections, full Choice/Score
comparison, and explicit full/targeted intersection comparison.

Retain these declared native semantics and verify them against the independent oracle:

- 630 source cases, 603 labeled: 383 representative and 220 supplementary. The
  97-case challenge component is only part of the supplement. Keep withheld cases
  in source accounting, outside labeled accuracy denominators.
- Native expected correct counts: Choice 442/603 and Score 390/603. Historical
  Choice strict projection 440/603 and Score argmax 437/603 describe other routes.
  These numbers are cross-checks, not values to force Validator to reproduce.
- Score boundaries: `0.5` maps to `UNCERTAIN`; `1.5` maps to `MATCH`. Map
  `SAME_CANONICAL`/`DIFFERENT_CANONICAL` explicitly to `MATCH`/`NO_MATCH`.
  `UNCERTAIN` is a class, not an abstention or withheld reference.
- Both native routes are single-label. Their auxiliary observations do not provide
  multi-label gold, categorical scoring signals, or confidence scoring signals.

Inspect at least one recovered and one regressed episode against source evidence.
Successful input checks alone do not satisfy this phase.

### 3. Complete distinct real and synthetic paths

Run resolver250 baseline/Flash/Pro against the same frozen reference bytes and
population, with the full command lifecycle and independent counts/paired changes.
Keep original historical-label agreement separate from any later reference version.
If the 24-case study adds needed coverage, use its accepted `owner-reference-key.jsonl`,
`episodes.jsonl`, and `pass-1/{incumbent,candidate}/results.jsonl` under
`Chord/data/taxonomy/experiments/v2-comparison-first-adjudicator-20260914/`.
Independently recount both arms, including the incumbent no-content failure. Do not
reuse its known-buggy latency scorer or change the rejected candidate's status.

Exercise single-label scoring/profile/policy gaps and the accepted multi-label case
through the executable. Saved TypeSafe distributions may contain sums of 0.99:
never normalize or relax admission to make these scored signals. Preserve them as
observations in the native route and test strict rejection separately. A valid
signal subset must be declared before evaluation, retain its selection receipt,
and make no full-population claim. Use public fixtures when no faithful real signal
case is available. Check applicability, units, denominators, numerical statuses,
raw/final results, and both nonempty and empty intersection behavior.

For every admitted workflow, validate machine outputs against the published schemas,
verify receipt hashes and exact input snapshots, inspect representative decisions,
and compare verified runs. Repeat one unchanged evaluation per task kind to prove
semantic reproducibility while allowing new run IDs/timestamps. Replay must remain
usable from its published snapshots and copied evidence, independent of upstream
working-directory state. Tamper only with disposable copies for rejection checks.

### 4. Repair and rerun until the selected workflows are stable

Classify each discrepancy as a Validator defect, conversion/integration defect,
historical evaluator defect, intentional semantic difference, or reference/input
limitation. Resolve arithmetic disputes from source rows and contract formulas;
no evaluator wins by default.

For an in-contract Validator failure: reduce it to a reproducible input, add a failing
regression, fix the smallest responsible implementation, run focused verification,
and rerun the affected full workflow on a separately installed candidate. Correct
conversion defects at their source and reverify the oracle's independence. Preserve
old outputs and hash-bind new ones. Use public synthetic reproductions when private
content cannot be retained in tests. Update operator documentation when an actual
usage defect prevents completion. This repair loop is part of the proposed execution
scope, not a separate approval request for every ordinary bug fix.

After the final code change, run the specification's required gates:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
```

Preserve the established offline/network-denied verification boundary using the
recorded verification procedure. Then run all selected complete workflows on the
same final installed candidate. Do not mix pre-fix and post-fix results into a passing
verdict. Reopen scope only for changed public semantics, new upstream inference or
labels, a new component/framework, or repairs whose cost materially exceeds the
bounded stabilization outcome. Stop the affected branch at those boundaries and
record the decision needed; do not stop ordinary in-contract repair at diagnosis.

## Evidence, completion, and limits

On execution, write `docs/acceptance/workflow-replay.md` with the six-workstream
selection rationale, coverage-to-case/test mapping, input/binary identities, reference
authority, conversion rules, exact commands and exits, independent expectations,
case results, discrepancy resolutions, and before/after repair evidence. Keep raw
inputs and logs in a new private acceptance directory. Do not copy private episode
text into the repository. Index the report, retained adapters, and durable run
manifests in the same change; keep task state and follow-up in Fizzy.

For long execution, verify a task-owned `caffeinate` hold, retain commands and progress
sufficient to resume interruption, and release only that hold at completion or stop.

Stabilization passes only when:

- Every selected supported workflow completes its full applicable command lifecycle
  on the final binary, with independent exact counts/IDs/transitions and floating
  metrics within specification tolerances.
- Every coverage obligation has passing process or appropriate existing regression
  evidence, with real-workflow and synthetic evidence distinguished.
- Observed in-contract Validator and conversion defects are fixed, regression-tested,
  and verified by complete reruns; required build/test gates pass.
- Historical differences are explained without fitting expected values to output;
  source failures, reference limits, excluded populations, and compatibility gaps
  remain visible and reproducible.

A blocked selected workflow is an incomplete stabilization result for that scope,
not success because its blocker was documented. An expected invalid-input rejection
can pass its negative check but cannot substitute for a valid end-to-end run.
Cases intentionally excluded before execution need a coverage/fit rationale, not
manufactured labels or new provider calls. Broader semantic correctness, classifier
promotion, production cutover, pristine fresh-agent acceptance, and the deferred full
DoD are separate claims; this plan does not imply any of them.

Use `fizzy` during execution, `python-dev` for conversion/oracle changes, `diagnose`
for reproducible failures, and `rust-best-practices` for Validator repairs. Further
workflows are justified only by a concrete remaining path or unresolved defect.
