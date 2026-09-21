# Made with Jev: apply the same Validator to new tasks

**Authority: Reference.** These are additional worked examples found through
[Made with Jev](https://madewithjev.com/), checked on 2026-09-21. This directory
extends the existing `examples/jev-poc` preparation layer, not the Rust core.

The goal is **measured task feedback**, not a score for how impressive a demo is.
For each task, identify the reference meaning, preserve actual predictions, run
Validator, and apply an explicit task rubric before recommending a change.

## Examples and evidence strength

| Source | Measurement | Evidence used |
|---|---|---|
| [jev-spam-eval](https://github.com/bitnovus/jev-spam-eval) | Binary ham/spam, then three-way legitimate/spam/phishing | Published per-record Noul/Choice predictions and the author's reference labels. No new model call. |
| [ASSAY-001](https://github.com/jourdanlabs/assay-001) | Banking77 intent classification and native-confidence rejection | Published request/response records joined to the separately frozen 77-class corpus. No new model call. |
| [Clean Code Review](https://github.com/frostney/clean-code-review) | Four explicit code-finding labels; retain three Score observations | Actual pinned question builder plus synthetic output fixtures. Eight semantic reference proposals remain unapproved. |

Each source revision and file digest is frozen in [pins.json](pins.json). The
[task rubric](rubric.json) states proposed error priorities and guardrails. These
priorities are examples, not agreed production deployment requirements.

**Published replay, synthetic tests, and newly human-reviewed evaluation are
separate evidence categories.** This work does not create the third category.

## Reproduce

Requirements: the existing Validator executable, Python 3.10+ for these examples,
and Node 22.6+ with `--experimental-strip-types` for the unchanged pure TypeScript
question module. These programs do not import or execute the other upstream
runners. They need no provider credentials.

Acquire pinned public sources on a network-enabled host:

```sh
python3 examples/jev-poc/madewithjev/fetch.py --out /tmp/made-sources-001
```

That command downloads three source archives, rejects unsafe/link archive members,
and checks all declared file hashes. It does not visit email links or call Jev.
The CI source artifacts are an alternative transport into an offline sandbox.
Downloaded upstream data is **not committed** to this repository. The spam and
code-review repos include MIT licenses. ASSAY's corpus attribution is documented
upstream; a blanket license for all raw logs was not established here. Review
upstream redistribution conditions before publishing captures or corpus copies.

Run the scoped contract tests, then the actual native replay:

```sh
cargo build --locked
python3 -m unittest discover -s examples/jev-poc/madewithjev -p 'test_*.py' -v
python3 examples/jev-poc/madewithjev/run.py \
  --sources /tmp/made-sources-001 \
  --validator target/debug/validator \
  --allow-unverified-email-runner \
  --out /tmp/made-validation-001
python3 examples/jev-poc/madewithjev/record.py \
  --run /tmp/made-validation-001 \
  --summary /tmp/made-outcomes-001.json \
  --report /tmp/made-review-001.md
```

All output destinations must be new. `run.py`, `review.py`, and `record.py` are
offline. Large saved populations require more time than the small fixture tests.
Every native command has the existing example helper's 90-second timeout; a
slower host can fail the run rather than produce an incomplete success claim.
Use `--tasks banking clean` to run only those scopes, without the email exception.

### Why the email acknowledgement flag exists

The capture manifest in `jev-spam-eval` names a producing `evaluate.py` SHA-256
that differs from its published source. The file history at the reviewed revision
contains only that initial published version. No matching original runner bytes
were located. Source bodies are also omitted upstream.

The default is to stop on this mismatch. The explicit flag permits **qualified
accounting of published case decisions**, not a claim that the published code
exactly reproduces their original requests. Both hashes remain in configuration,
capture evidence and outcomes. All downloaded file hashes, reference/prediction
IDs, label mappings and pair completeness are still checked. Changing the flag
cannot bless modified source bytes or silently drop cases.

This is a provenance limitation, not grounds for substituting invented responses
or claiming that the published measurements were fraudulent.

## Task boundaries

### Binary email

Use all **19,528 saved source records**. The upstream headline uses 18,514 unique
messages after content deduplication. Because source emails are not committed,
that deduplication cannot be reconstructed here. The denominator is disclosed,
and results must not be presented as reproduction of that headline cohort.

Compare `spam_plain` with `spam_structured_criteria` at `p >= 0.5`. Map each Noul to
complementary binary probabilities, with no invented native confidence. Keep
missed spam and ham false alarms separate: reducing one can increase the other.

### Three-class email

Replay `text/category`, `enriched/category`, and `enriched/evidence_framing` on
three independent reported cohorts: main (5,733), fresh (3,300), recent (853).
`fresh` means different sampled messages, not a new untouched test for this work.
The recent cohort contains phishing only: it can measure recall, not usable
precision or false-positive performance. These records can overlap binary-email
records; do not add cohort counts and call them independent unique emails.

Golden class meanings come from the original reference records, with explicit
`ham -> legitimate` and `phish -> phishing` mappings. Preserve the selected Choice
rather than overwriting it with an argmax, particularly at rounded ties. Compare
context enrichment and question framing separately. Retain case-level recoveries
and regressions, not only a score difference.

### Banking77

Match every `b77-NNNN` request/response to the corresponding frozen CSV row. Check
text, reference category, requested options, response type and returned model.
Compare the recorded choice with the same predictions under the **native reported
confidence >= 0.8** gate. Equality is accepted. Raw outcomes remain unchanged.

The example rubric asks for at least 95% accuracy among answered cases and at
least 75% coverage. Those are proposed policy thresholds, not release authority.
Do not count an abstained correct prediction as an improvement. Some categorical
vectors sum to 0.99. The all-or-none scoring rule means those vectors remain
non-scoring observations across the population; native confidence is a separate,
complete signal. The core's numeric tolerance is not relaxed. Upstream ASSAY's
ECE convention also differs (right-closed bins and a post-run sum tolerance).

### Clean Code Review

Its actual pure question builder exposes 31 Nouls and three five-level Scores.
Two Noul questions are conditional: `unclear_tests` applies to test files and
`leaves_it_worse` to patches. The probe executes all four scope combinations.
Inapplicable questions must not be fabricated as true negatives.

The scored task deliberately includes only these four all-scope findings:
`too_many_arguments`, `flag_or_output_arguments`, `commented_out_code`, and
`swallowed_errors`. References must review all four, including absent labels.
Other findings are not silently labeled negative. Missing applicable answers or
an explicitly partial/cut file stop preparation.

The full application merges windows with maximum Noul / minimum Score values and
combines comment-stripped and original-code passes. Thus final file-level signals
are observations here, **not calibrated per-label probabilities**. Score positions,
distributions and native confidence stay observations, not invented final-class
probabilities. The verifier does not claim to have executed that complete runtime.

[clean-fixtures.json](clean-fixtures.json) contains fabricated replies for software
checks. [clean-reference-proposals.json](clean-reference-proposals.json) contains
eight separate review-pending code cases. Neither file claims live accuracy or
human approval. Threshold 0.5 versus 0.7 is a controlled fixture demonstration with
both a recovery and a regression.

## What is checked

`oracle.py` does not import preparation or production evaluator code. It computes
hard matrices, denominators, per-class rates, set accounting and exact transition
IDs from source references and decisions. `run.py` compares native raw/final
results against those expectations, checks byte-identical gold between arms,
validates receipts, and performs explicit episode inspection via the existing
native helper. `test_made.py` includes ordering, duplicate/missing identity,
threshold equality, count conservation, incomplete scope and wrong signal
semantics. This is scoped coverage, not a claim to enumerate every core path.

[Outcomes](outcomes.json) and the [execution report](../../../docs/acceptance/made-with-jev.md)
record what actually completed. Full per-case evidence is regenerated in the run
directory. The durable summary keeps transition examples and a hash of complete
transition-ID arrays; it does not need to copy the entire external corpus.

No task management, automatic gold editing, model telemetry, upstream code
patches, or live inference is added. A human or their existing development agent
chooses whether to implement the recommendation.
