# Sift: from one captured run to a reviewable threshold comparison

Authority: **Reference**. Status: the capture/review path is exercised with explicit
software-test replies. No live Jev accuracy result or human approval is claimed.

## The decision being validated

Keep **affirmative evidence of a current paid offering of the described company's
own software to hospitals**. A subscription available to hospital operators counts;
reselling another vendor's product, buying software, free-only offerings, consulting
without a product, future/canceled plans, and mere hospital keywords do not.
“Irrelevant” means that the supplied text does not meet this inclusion rule. It is
not proof of a fact about the company beyond the text.

This explicit objective fixes an ambiguity in the earlier proposal. Sift asks
whether content helps accomplish a task **or answer a query**. For a yes/no query,
a denial may help answer it even though the intended gold labels it irrelevant.
The original generator's terse question is therefore not assumed to express this
retrieval objective correctly. This is a task-definition correction identified
before new measurements, **not a measured accuracy improvement**. See the pinned
[classifier](https://github.com/kbhuw/jev-sift/blob/966de12e2bb5f94d47886ee51f30a07ec8ef1607/src/classify.js).

## The small reference packet

`development.proposed.json` contains 16 original cases in eight matched families.
`held_out.proposed.json` contains eight further cases in four disjoint families.
The human-readable development packet is [review-development.md](review-development.md).
These are a new focused diagnostic, not a silently modified version of the earlier
80-case generator. The original 846 proposals remain unchanged.

Every reference is still **pending human review**. All inputs are fictional.
The same assistant authored these proposals and their rationales; this is not an
independent review or human approval. The rubric makes labels inspectable rather
than magically authoritative. Verify each text, target, and rationale; correct or
withhold disputed cases before capture. Review all four label facts implicitly
used here: who is described, what is sold, to whom, and whether it is current.

Keep the separate held-out file out of tuning contexts. It is author-visible and
is not cryptographically sealed. Its small constructed sample does not estimate
production accuracy. Review it independently only after fixing the candidate.

## 1. Preflight, without a provider call

From the repository root:

```sh
python3 examples/jev-poc/sift/run.py preflight \
  --book examples/jev-poc/sift/development.proposed.json \
  --validator target/debug/validator --upstream /path/to/pinned/jev-sift
```

A blocked preflight exits 2, lists pending case IDs and missing prerequisites,
and makes no model call. It checks key **presence**, never reveals its value or
claims that connectivity/key validity has been tested. The capture runner checks
the source revision and clean source tree.

Copy the proposed development casebook outside the repository for actual review.
For each case that was genuinely reviewed, use the declaration described in the
review packet. Do not stamp approval merely to pass a gate. No command in this
example creates or approves labels.

## 2. Capture the actual application

Use a clean checkout of `kbhuw/jev-sift` at
`966de12e2bb5f94d47886ee51f30a07ec8ef1607` with its locked dependencies installed.
Make a TypeSafe key available securely as `TYPESAFE_API_KEY` or `JEV_API_KEY` in
the execution environment; do not put it in source, command arguments, or chat.
Start with a five-case smoke capture:

```sh
node examples/jev-poc/capture_sift.mjs \
  --book /private/approved-development.json \
  --upstream /path/to/pinned/jev-sift \
  --limit 5 --out /private/sift-smoke.json --execute
```

Without `--execute` it only prints the selected application inputs. The CLI has no
fixture bypass. A live call cannot use pending references. File destinations must
not exist. The capture records the exact selected casebook's SHA-256, actual
request/response evidence, attempts, no-decisions, and unattempted cases. An aborted
in-flight case is not incorrectly reported as never attempted. HTTP 401/403 stops
further requests. Invalid JSON reaches the original provider's parsing-error path.

After inspecting smoke capture integrity, capture all 16 reviewed development
cases to a new path with `--limit 16`. There are no retries; each call is bounded
to 15 seconds and the whole capture to 120 seconds. These are time/request limits,
not a guarantee of billing. The output remains incomplete on interrupted execution
and cannot be scored. Completed per-item operational errors remain no-decisions;
they are not silently discarded or treated as irrelevant.

## 3. Measure and review, using the same saved predictions

```sh
python3 examples/jev-poc/sift/run.py compare \
  --book /private/sift-development.json.book.json \
  --capture /private/sift-development.json \
  --validator target/debug/validator --out /private/sift-comparison-001
```

This runs **the existing Validator**, not a second implementation of its metrics.
It compares inclusive thresholds 0.5 and 0.7 on identical golden bytes, verifies
receipts, inspects up to ten affected cases in both runs, and saves:

* `summary.json` and `review.md`: metric statuses and denominators, every case's
  expected/actual labels, probabilities, operational errors, recoveries,
  regressions, and one bounded recommendation.
* `baseline/`, `candidate/`, `comparison/`, receipts and canonical prepared inputs:
  complete native measurement artifacts, not merely headline scores.
* `reference-snapshot.json`, `capture.json`, selected inspections and
  `evidence-manifest.json`: traceable source and result bytes.

No new inference occurs in comparison. The tool checks that the effective model
request matches the reference task and text, and that native Noul values and model
identity survived translation. Editing the captured reference file invalidates its
binding. A reconstructed hash is not proof of human review or authentic origin.

The recommendation is conditional on measured tradeoffs; it does not choose a
winner, identify a root cause, edit gold, implement a change, or create tasks. A
higher threshold can eliminate false positives **and lose relevant evidence**.
Assess that tradeoff before choosing a candidate. Then run a separately reviewed
held-out capture (`--partition held_out --limit 8`) using the fixed objective and
policies. Reading the final held-out results consumes that holdout for tuning.

## Offline regression of this exact path

With the same pinned upstream checkout and no provider key:

```sh
node examples/jev-poc/sift/test-capture.mjs \
  /path/to/pinned/jev-sift /tmp/sift-capture-proof-001
python3 examples/jev-poc/sift/run.py compare \
  --book /tmp/sift-capture-proof-001/capture.json.book.json \
  --capture /tmp/sift-capture-proof-001/capture.json \
  --validator target/debug/validator --out /tmp/sift-review-proof-001 --fixture
```

The test injects explicitly synthetic HTTP replies into the **actual capture
implementation and original Sift implementation**. Injection forces origin
`synthetic_fixture` and never fabricates a review declaration. The comparison
allows that origin only with explicit `--fixture`; it suppresses model-quality
claims and summary quality metrics. Native reports still contain ordinary test
arithmetic. This verifies software behavior, not Jev prediction quality.

Local executed evidence is summarized in [outcomes.json](outcomes.json). The
remaining external inputs are actual reference approval and authenticated provider
execution on a network-enabled host. This continuation does not claim those
inputs were supplied.
