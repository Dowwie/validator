# Published Jev evidence: a broader example portfolio

**Authority: Reference.** Three additional source projects supply twelve diagnostic
cohorts. These examples replay **saved predictions**, not fake model responses or
new API calls. They do not approve labels, run moderation bots, trade, or change
the measurement core. The task-specific [rubric](rubric.json) is **Proposed**.

| Project | Tasks | What the example adds |
|---|---|---|
| [jev-acento](https://github.com/marcosmartinez/jev-acento/tree/7e007b4c2bd552471b56eff035f9a3df7593f9fe) | XNLI entailment, PAWS-X paraphrase, MASSIVE intent, Belebele answer selection | Paired language and instruction changes; two recorded passes; Noul and Choice-derived decisions |
| [jevmod](https://github.com/ohernandezdev/jevmod/tree/01063f3e927ddbc3c7de923b73f56dcc33034ca9) | One four-label moderation cohort, four individually labeled cohorts | Complete versus partial reference labels, real marginal probabilities, cache/no-decision accounting, threshold tradeoffs |
| [fedjev-bench](https://github.com/maybern-tripp-smith/fedjev-bench/tree/7911b5da1dee9947c75784a6962ee5d4fd3d315f) | Extreme pairs, labeled sentence pairs, adjacent statements | Choice orientation, duplicate-log handling, position sensitivity, and an explicit agreement-only abstention policy |

Read the [execution report](../../../docs/acceptance/evidence-portfolio.md) and
[compact outcomes](outcomes.json) before interpreting a number. This extension is
separate from the concurrent `portfolio/` work, and does not supersede it.

## Run the examples

Acquisition needs Python 3.12+, git, a network connection, and the pinned parquet
reader. It does not execute source-project code. Keep downloaded corpora outside
the repository; they contain material subject to their own terms and may include
sensitive moderation examples.

```sh
python3 -m pip install 'pyarrow==21.0.0'
python3 examples/jev-poc/benchmarks/fetch.py --out /tmp/jev-benchmark-sources
cargo build --locked
VALIDATOR_BIN="$PWD/target/debug/validator" \
  python3 -m unittest discover -s examples/jev-poc/benchmarks -p 'test_*.py' -v

# No network or inference occurs below.
python3 examples/jev-poc/benchmarks/run.py \
  --sources /tmp/jev-benchmark-sources \
  --validator target/debug/validator \
  --example acento --out /tmp/jev-bilingual-proof
python3 examples/jev-poc/benchmarks/run.py \
  --sources /tmp/jev-benchmark-sources \
  --validator target/debug/validator \
  --example pairwise --out /tmp/jev-pairwise-proof
```

All output destinations must be new. The read-only workflow also retains a
`benchmark-sources` archive for use in a sandbox without direct network access.
Acquisition verifies pinned revisions and [file digests](sources.lock.json).
A small deterministic JSON export links the acento parquet reference metadata
and its original digest; the export itself is also pinned.

### The moderation exception is explicit

The published moderation predictions contain positional IDs but **no original
input hashes**. The independent reference file comes from
[OpenAI's frozen dataset](https://github.com/openai/moderation-api-release/tree/f4ab51b5edd3bfbcb349a56324274235b674e0e4).
The source preparation script specifies the index-based join. It is not possible
to authenticate that the author's original file order and content were identical.
Therefore moderation replay refuses to run unless this limitation is accepted:

```sh
python3 examples/jev-poc/benchmarks/run.py \
  --sources /tmp/jev-benchmark-sources \
  --validator target/debug/validator \
  --example moderation --allow-unverified-moderation-join \
  --out /tmp/jev-moderation-qualified-proof
```

To run all twelve cohorts, use `--example all` and that same explicit exception.
This is qualified **agreement with reconstructed references**, not authenticated
reproduction of the original moderation workload. The core's strict admission
and evidence checks are not disabled.

## Reference and prediction rules

### Bilingual classification

For each of the four tasks, all six cells have identical logical case IDs:
A = English state/English instructions; B = Spanish state/English instructions;
C = Spanish state/Spanish instructions. Each cell has passes 0 and 1.

We cross-check every recorded reference and effective state hash against the
independent committed item table and verify the upstream prompt/preregistration
hash files. Reference inputs store both language hashes, not invented text.
The same golden bytes are used across arms. Original text must be recovered from
the source datasets before making semantic root-cause claims.

The source parser derives Choice predictions by lexical argmax of the published
probabilities; Noul uses **strictly greater than 0.5**, so equality means false.
We preserve those rules, rather than substitute our normal inclusive threshold.
Native Choice may differ from that derived answer; the source retains a flag but
not the original selected label. We cannot reconstruct information it discarded.

Some rounded categorical maps do not sum to one within Validator's tolerance.
They remain observations. Where a scoring family becomes partial, all rows remain
in the hard-outcome evaluation and the whole family is disclosed as unscored.
No silent normalization, complete-case substitution, or forced ECE reproduction.
Two repeats describe this recorded run's variability, not a general noise bound.

### Multi-label moderation

OpenAI's dataset explicitly defines an omitted label as **unknown**. The published
jevmod preparation converts absence into no positive label; that is not adequate
for a complete-reference multi-label task. This adapter instead uses three-valued
logic for the author's mapping:

- A combined label is positive if any reviewed component is 1.
- It is negative only if every required component is explicitly 0.
- Otherwise it is unknown and is not scored for that label.

The four mapped labels are `harassment` (H/HR/V), `nsfw` (S/V2), `selfharm` (SH),
and `minors` (S3). This taxonomy is inherited from the source and is not asserted
to be semantically equivalent to every application's moderation rule.

Only **765 of 1,680** rows have known values for all four mapped labels. The four
binary cohorts preserve additional individually labeled cases: 774 harassment,
860 nsfw, 1,447 selfharm, and 994 minors. These cohorts overlap: do not sum them
as independent messages. Unknown gold is not an abstention by the model.

Recorded cached scores remain attributed as cached observations. The one
`too short` bypass in the selfharm cohort is an explicit no-model-decision, not
a clean prediction. No resolved model version was saved, so none is invented.
The example compares shipped thresholds with a **retrospective uniform 0.5**
sensitivity test. It does not recommend that policy for a community.

### Pairwise selection

Reference labels describe the winner among fixed document IDs `a` and `b`.
Native options `A` and `B` instead describe displayed positions. Reversing the
input requires remapping both the selected label and the probability vector.
Otherwise the same chosen document can be falsely reported as a regression.

The source log includes 80 duplicate entries for extreme-pair calls. Selection
uses the last log entry only when repeated decision/probability/confidence/model
fields agree, never the most favorable score. Effective input metadata can differ
and remains visible. Conflicting repeated results require separate arms.

The agreement-only candidate answers only when both displayed orders select the
same underlying document. It does not manufacture a new confidence value, nor
claim that model agreement proves correctness. Scores are against the external
reference pair, not against the model's other answer.

All frozen references put the designated winner at `a`. Therefore report
reference agreement and coverage, not a balanced classifier claim. Keep extreme,
Shah sentence, and adjacent-statement cohorts separate: the adjacent/extreme
labels use policy-rate proxies, while Shah uses source sentence labels. This
example does not validate rankings, economic predictions, or financial actions.

## Evidence and verification

Each arm publishes canonical inputs, provenance, the native report, and receipts.
Each comparison preserves complete transition IDs. A separate arithmetic oracle
checks every raw/final count and transition ID, plus probability loss/Brier or
explicit non-applicability. Representative changed cases are inspected. Each task
also proves relocation and corruption rejection.

`outcomes.json` is compact: counts, metrics, hashes, and bounded case previews.
The destination also includes `outcomes.full.json`, complete native run bundles,
and untruncated transition sets. Routine documentation contains no source texts.
The downloaded source corpora and complete run bundles are intentionally not
vendored into this repository. Consult each upstream `LICENSE`, `THIRD_PARTY.md`,
or `DATA.md`; Shah data is marked non-commercial research with attribution.

The 25 checks in [test_benchmarks.py](test_benchmarks.py) cover unknown references,
ID ordering, duplicate IDs, invalid signals, rounded maps, orientation, repeated
captures, and native multi-label accounting. The native test does not silently
skip when the executable is missing. Original predictions and human labels are
never changed to make a result look better.
