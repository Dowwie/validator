# Jev portfolio: six additional validation tasks

**Status:** executable examples, not new live inference or production approval.

This extension adds three independently designed source projects. It demonstrates
four tasks using published Jev responses and two tasks using original application
code with explicit synthetic replies. The measurement core is unchanged.

| Task | Reference and response evidence | What the example measures |
|---|---|---|
| Citation support | 5 author-labeled claim/quote pairs and published Choice answers | Support, contradiction, or insufficient support; confidence rejection and review cost |
| Local sarcasm | 12 author-labeled statements and published Choice answers | Two-class agreement within one text; not a universal sentiment benchmark |
| Contextual sarcasm | 10 labeled cases plus 2 explicitly unresolved cases, all with published answers | Context-sensitive classification without turning unknown references into negatives |
| Feed triage | 15 historical dispositions and two published Noul signals per item | Final active/rejected commitments versus deferred review; historical agreement, not adjudicated truth |
| Sponsor skipping | 12 constructed state-machine cases, both policy arms | Ordered audio/seek history, threshold equality, skip limits, missing answers and silence |
| Moderation | 10 constructed message/response cases, both policy arms | Noul-controlled blocking, independent category logging, and explicit fail-open behavior |

There are **44 saved Atlas responses**: 42 have declared reference labels and two
remain withheld. There are also **22 synthetic application cases**. No test result
from the latter is reported as Jev performance. The two sarcasm datasets are
separate populations, not a paired experiment showing that context helps.

## Source contracts

Exact revisions and input bytes are recorded in [sources.json](sources.json):

- [Jev Capability Atlas](https://github.com/Zaious/jev-capability-atlas/tree/4746cfd6bc679fb1c8ffdd8317920fbf77a13542): citation support, sarcasm, and watched-feed triage.
- [YouTube Sponsor Detection](https://github.com/trungdq88/youtube-sponsor-detection/tree/de01f0568d043035889a296a61ce21e0accc8b16): the original pure live controller.
- [Mastra Jev Moderation](https://github.com/CodeAlive-AI/mastra-jev-moderation/tree/8735ea80efad6ec1dab26f74f1a7851007758915): the original processor and HTTP-response parser.

The source check verifies the checkout revision and every required file hash.
It reads Atlas JSON and literal question constants; it never executes an Atlas
model runner. Sponsor and moderation source execute with fixed synthetic replies.
Node removes TypeScript types; actual Zod 3.25.76 validates moderation responses.
The two `moderation.package*` files supply that locked test dependency environment.
Mastra type-only imports are erased, so this is not a full Mastra-agent test.
No third-party source or corpus is vendored here. The pinned sources are acquired
separately. Consult their licenses/notices before further redistribution.

## Reproduce

Requirements: built Validator, Python 3.10+, Node 22.16+ with type stripping, Git,
and npm for one-time acquisition. Provider keys are neither used nor required.
The workflow [portfolio.yml](../../../.github/workflows/portfolio.yml) acquires
these sources, builds Validator, runs every new example and test, and preserves
the full evidence bundle. Repository access in that workflow is read-only.

For local acquisition from the repository root, select a NEW work directory:

```sh
WORK=/tmp/validator-portfolio-001
mkdir "$WORK"
while read -r name repo revision; do
  git init -q "$WORK/$name"
  git -C "$WORK/$name" fetch -q --depth=1 "https://github.com/$repo.git" "$revision"
  git -C "$WORK/$name" checkout -q --detach FETCH_HEAD
done <<'PINS'
atlas Zaious/jev-capability-atlas 4746cfd6bc679fb1c8ffdd8317920fbf77a13542
sponsor trungdq88/youtube-sponsor-detection de01f0568d043035889a296a61ce21e0accc8b16
moderation CodeAlive-AI/mastra-jev-moderation 8735ea80efad6ec1dab26f74f1a7851007758915
PINS
cp examples/jev-poc/portfolio/moderation.package.json "$WORK/moderation/package.json"
cp examples/jev-poc/portfolio/moderation.package-lock.json "$WORK/moderation/package-lock.json"
npm ci --prefix "$WORK/moderation" --ignore-scripts --no-audit --no-fund
```

The following execution is offline. `--out` must not already exist:

```sh
cargo build --locked
python3 examples/jev-poc/portfolio/run.py \
  --sources "$WORK" --validator target/debug/validator \
  --out /tmp/validator-portfolio-results-001

PORTFOLIO_SOURCES="$WORK" \
PORTFOLIO_RESULTS=/tmp/validator-portfolio-results-001 \
python3 -m unittest discover -s examples/jev-poc/portfolio -p 'test_*.py' -v
```

The script runs **12 evaluations and six paired comparisons**, checks raw and
final confusion matrices and per-class rates, audits probability arithmetic,
inspects every changed case, and tests relocated evidence and deliberate tampering.
It writes canonical inputs, reference maps, native captures, reports, inspections,
receipts, and `summary.json`. The committed [outcomes.json](outcomes.json) is one
recorded execution; run IDs, paths and hashes can change on repetition.

## The feedback each example supplies

### Citation support: rejection is not correction

The source defines three outcomes: `supports`, `contradicts`, `says_nothing`.
It has an intentionally thin-support case with native confidence 0.45 but
probability 0.63 on the selected class. Those quantities remain separate.

The example compares the recorded answer with a native-confidence gate at 0.8.
A correct answer sent to review is a loss of coverage, not a new semantic error
and not a correction. The author acknowledges the thin-support reference is
contestable. Keep it unchanged for replay, but review it independently before
using it as authoritative gold for a new experiment.

### Sarcasm: unknown is not false

The contextual source explicitly gives `AMB1` and `AMB2` null references. Their
responses remain in `withheld.json` and provenance. They never enter the scored
population, even when the response confidence is 1.0. That is not evidence that
this prediction is wrong; there is no settled reference with which to decide.

Same-clause and cross-turn datasets have different cases. We compare each to its
own 0.8 gate, not to the other dataset. No claim that context improves accuracy
follows from comparing their averages.

### Feed triage: deferred is not an expected class

The original policy prioritizes an active trigger, then a reject trigger, then
batch review. The two Nouls are independent, not a probability simplex. Their
values stay Bernoulli observations rather than invented final-action probabilities.

Historical targets are `active`, `frozen`, and `rejected`. `batch_review` is an
explicit abstention against those dispositions; it is NOT silently relabeled
`frozen`. The 0.75/0.90 contrast measures historical commitment agreement and review
coverage. A deferred wrong commitment is not a corrected historical classification.
A mismatch can reflect a changed watch policy or weak historical labels; do not
attribute it to the model without reference review.

### Sponsor skipping: sequence order matters inside a case

One case contains ordered `hear` and `skipped` events followed by a requested
answer. Reordering independent Validator records must not change accounting;
reordering the events *inside* a controller case can change its meaning.
The original live controller, its question builders, and its transcript renderer
execute unchanged. No browser, speech recognition, actual video seek, or full
recorded-transcript boundary-localization pipeline is tested here.

At its skip limit the controller can correctly keep playing despite a high
`sponsor_continues` Noul. Missing answers and silence are visible states, not
invented negative probabilities. The 0.7/0.9 fixture contrast deliberately contains
both a recovery and regressions. It measures neither time saved nor real sponsor
accuracy. New live work needs independently reviewed complete sequences.

### Moderation: the action survives a missing verdict

The original application asks a Noul for blocking and a Choice for category.
Only the Noul controls the gate. Category is a log label; missing category must
not turn a block into a pass.

The processor explicitly fails open. An HTTP error therefore produces an actual
`allow` action with no model verdict. The action task records `allow`, while the
absence of a probability remains explicit. Converting that action into abstention
would conceal what the application did; inventing p=0 would misrepresent Jev.
A separate model-judgment task would treat the same missing verdict as no-decision.

The 0.7/0.9 comparison has one recovery and one regression with unchanged aggregate
correct count. The fixtures test this distinction. They do not certify the default
content policy, the safety of fail-open operation, or model accuracy.

## Scope and remaining work

The task rubrics in [rubric.json](rubric.json) state what to inspect and recommend.
They are illustrative, not owner-approved deployment thresholds. Arithmetic and
case transitions come from the executable, not a model's explanation.

Atlas records only the requested `jev-latest` alias, not a resolved model revision
or original request hashes. We retain that missing provenance. Published references
are upstream author judgments or historical dispositions, not newly human-approved
labels. No new reference approval or model inference occurs here.

Live capture, independent semantic review, and larger held-out populations remain
future work. There is no need to finish them to reproduce the bounded evidence in
this extension. The full status of the example collection is in
[the portfolio index](../PORTFOLIO.md).
