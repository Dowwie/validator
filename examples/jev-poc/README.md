# Jev application validation examples

These examples implement the **preparation and offline proving layers** of
[the application plans](../../docs/plans/jev-validation-pocs.md). They do not turn
Validator into a model runner, optimizer, or task manager.

There are three intentionally separate evidence categories:

* **Published Jev replay:** jgrep's original 20 author-labeled code cases and six
  saved Jev runs. We reconstruct case decisions from recorded row probabilities,
  run the actual Validator executable, and compare the results. No new model call.
* **Synthetic protocol tests:** small clearly labeled fabricated outputs exercise
  all ten adapter profiles through the native utility. Their scores are not model
  performance, and are deliberately omitted from the outcome summary.
* **Reference proposals:** 846 generated cases across ten casebooks. Every review
  is pending. The generator is readable and deterministic; its related templates
  are not a representative production benchmark. No new human approval is claimed.

For the focused end-to-end Sift path, start with [the Sift worked example](sift/README.md).

See [the execution record](../../docs/acceptance/jev-poc-execution.md) and
[outcomes.json](outcomes.json) for the exact executed scope and limitations.

## Quick offline proof

Build the existing pinned Rust package; Python 3.10+ is required for the examples.
Node 20+ is only needed for the optional Sift capture script.

```sh
cargo build --locked
python3 -m unittest discover -s examples/jev-poc -p 'test_*.py' -v
python3 examples/jev-poc/poc.py verify \
  --validator target/debug/validator \
  --upstream examples/jev-poc/upstream \
  --out /tmp/validator-jev-proof-001
```

The destination must not exist. `verify` is offline: it runs native `check`,
`evaluate`, `inspect`, and `compare`, checks receipts, exercises relocation and
intentional evidence tampering on disposable copies, and replays all six jgrep
arms. It retains the canonical inputs and complete native run bundles at the
chosen destination. The executable's SHA-256 is included in `summary.json`.

The upstream directory contains the MIT-licensed frozen jgrep files and their
provenance manifest. To independently retrieve those exact public bytes:

```sh
python3 examples/jev-poc/poc.py fetch-jgrep --out /tmp/jgrep-upstream-001
```

Only `fetch-jgrep` accesses the network in the Python utility. It verifies the
pinned Git blob identities; replay also verifies the saved result's SHA-256
binding to its fixture. It never calls Jev or another inference service.

## Generate proposed gold

```sh
python3 examples/jev-poc/poc.py drafts --out /tmp/jev-reference-proposals-001
```

| Casebook | Cases | Development / held out | Adapter output |
|---|---:|---:|---|
| Sift | 80 | 60 / 20 | Binary relevance; optional complementary distribution |
| Foreman | 60 | 45 / 15 | Ten complete label marginals |
| Upwork | 90 | 60 / 30 | Final apply/review/skip plus observations |
| Tax | 60 | 40 / 20 | Form class, separate stage observations |
| classifier.dev single | 120 | 80 / 40 | Choice, probabilities and native confidence |
| classifier.dev multi | 96 | 64 / 32 | Set from **all** label scores, not only top tag |
| Filing | 80 | 60 / 20 | Category versus review/quarantine abstention |
| Compaction | 120 | 90 / 30 | Keep / truncate-result / drop-call |
| Router (optional) | 60 | 39 / 21 | Final tier, not cheapest-successful-model truth |
| jev-align (optional) | 80 | 60 / 20 | Binary export; optional multiclass/multilabel parsing |

Related examples stay in one partition. The router split deliberately becomes
39/21 to preserve its three-case families; the plan's 40/20 target is not followed
at the expense of leakage. Tax uses a restricted fixture vocabulary: expand and
freeze it against the full upstream registry before unrestricted live capture.
The 42-case tax page-kind extension and new 48-case jgrep set remain unimplemented.
The router's 24 policy fixtures and native Sift/Foreman checks are implemented;
see the native-boundary section below.

Review each case's input, proposed target and rationale independently. Mark an
actually approved case's review as:

```json
{"state":"approved","method":"human","reviewer":"actual reviewer identity"}
```

The software verifies that this declaration is present; it cannot authenticate
that a human really performed it. Do not bulk-fill it from an agent's opinion.
Disputed cases remain unapproved and must be resolved or explicitly withheld in a
new documented dataset revision. Do not relabel failures after seeing Jev answers
while continuing to claim an untouched reference set.

The first author sees the proposed held-out examples. Keep them out of later
optimizer/developer contexts. Preparation produces a **partition-specific golden
file**, so a development run does not snapshot held-out labels. Baseline and
candidate use byte-identical gold within each partition; cross-partition scores
are not paired comparisons.

## Import actual application outputs

The Rust wire format is unchanged. A thin capture envelope links native application
outputs to stable authoring case IDs:

```json
{
  "schema_version": 1,
  "example": "sift",
  "origin": "observed",
  "complete": true,
  "source": {"model": "the-actual-established-model-or-unresolved-alias"},
  "records": [{
    "case_id": "sift-00-00",
    "request": {"state": "the actual provider-visible input", "questions": {}},
    "output": {
      "id": "sift-00-00",
      "answers": {"relevant": {"type": "boolean", "probability": 0.7}}
    }
  }]
}
```

This is an illustrative shape, **not an observed response**. Save exact requests,
responses, producing model and application configuration in real capture records.
`metadata` can retain raw response data and transport status; omit credentials and
all authentication headers. Native `model`/`resolved_model` and optional record
`model` must agree. Distinct recorded models/configurations become distinct source
definitions, so fallback traffic is not falsely attributed to Jev.

```sh
python3 examples/jev-poc/poc.py prepare \
  --book approved-casebook.json --capture actual-capture.json \
  --partition development --out /tmp/jev-prepared-001
validator check --dataset /tmp/jev-prepared-001/golden.json \
  --predictions /tmp/jev-prepared-001/predictions.json \
  --config /tmp/jev-prepared-001/config.json
```

Every selected case must have exactly one capture record. Explicit upstream errors
become whole-episode no-decisions with capture evidence. Unattempted cases,
duplicates, unknown labels and malformed maps fail preparation. An incomplete
capture cannot be scored. `--fixture` only permits explicitly synthetic captures;
it cannot bypass the human-review gate for observed captures.

`--policy policy.json` supports only the operations each adapter can faithfully
apply: Sift/jev-align binary thresholds, Foreman complete per-label thresholds,
classifier.dev multilabel thresholds, tax minimum-stage-probability gating, filing
dispositions/gates, and compaction keep thresholds. Upwork and router policy
composition must run in their original applications, not be approximately
reimplemented from one score here.

### What flexibility was added

`adapters.py` understands the application output shapes without changing the
measurement core. It preserves auxiliary observations, refuses guessed probability
semantics, and keeps every row when scoring signals are only partly available:
partial signal families are explicitly demoted to named observations, with the
choice disclosed in preparation provenance. A complete-signal subgroup is not
silently substituted for the full population.

The utility does not interpret weighted fit, maximum-across-records, selected
category probability, or minimum-across-stages as native Jev confidence. Rounded
categorical vectors outside the core's sum tolerance remain observations instead
of being silently normalized. The core's admission requirements stay strict.

## Optional bounded Sift capture

This invokes the pinned upstream `classify` and `createProvider` implementations,
not a rewritten model prompt. It has been exercised end-to-end with explicitly synthetic transport replies,
including request/reference binding and interruption/error cases;
**a live provider call has not been exercised in this execution**.

```sh
node examples/jev-poc/capture_sift.mjs \
  --book /tmp/jev-reference-proposals-001/sift.json --limit 5
```

Dry-run is the default and needs neither credentials nor an upstream checkout.
After genuine human review, install dependencies in a clean upstream checkout at
`966de12e2bb5f94d47886ee51f30a07ec8ef1607`, set `TYPESAFE_API_KEY` or `JEV_API_KEY`, and:

```sh
node examples/jev-poc/capture_sift.mjs \
  --book approved-sift.json --upstream /path/to/pinned/jev-sift \
  --limit 5 --out /tmp/sift-capture-001.json --execute
python3 examples/jev-poc/poc.py prepare \
  --book /tmp/sift-capture-001.json.book.json \
  --capture /tmp/sift-capture-001.json --out /tmp/sift-prepared-001
```

Five cases is the default; the hard maximum is 80. There are no retries, each call
has a 15-second timeout, and the overall capture is bounded to 120 seconds. These
are request/time limits, **not a billing guarantee**. A partial capture is retained
as incomplete and preparation refuses it. Output files are reserved with no
replacement and owner-only permissions before any model call.

Application-native live capture runners beyond Sift, expanded corpora, and
optimization loops remain unimplemented. Router, Sift and Foreman now have the
bounded native integration tests described below; these are not live model tests.
Other applications' saved output shapes can already be imported. Nothing in these examples
launches coding workers, moves user documents, performs trades, opens issues, or
changes task state. The review boundary remains **run → review → recommend**.

## Native application boundary tests, run locally

The new `native/verify.py` exercises **actual pinned upstream implementations**,
not just objects shaped like their outputs. It runs 24 router policy fixtures,
18 Sift classify/provider checks and 16 Foreman schema/assessment checks, then
feeds the resulting explicitly synthetic captures into the real Validator binary
for six evaluations and three paired comparisons. It also schema-checks all 80
Sift inputs and all 60 Foreman snapshots. Results are in `native/outcomes.json`.

Prepare clean checkouts at the pins in the plan under a common directory named
`router/`, `sift/` and `foreman/`. Run `npm ci --ignore-scripts` in router and Sift;
install `pydantic>=2.7,<3` for Foreman. The CI `pinned-native-runtimes` artifact
contains these exact public checkouts and locked Node dependencies for offline
sandbox use. Its `.git` directories retain their revision identities. It contains
no provider credentials. Source acquisition is separate from the following run:

```sh
python3 examples/jev-poc/native/verify.py \
  --sources /path/to/native-runtimes \
  --validator target/debug/validator \
  --out /tmp/validator-native-proof-001
```

The output directory must not exist. Requests are answered in memory, provider
credentials are removed from the child environment, and network attempts in the
native probes are blocked. Nothing labels these fixtures as live observations or
approves their semantic reference proposals. Foreman uses its own injected-client
path, not an installed TypeSafe SDK or a worker runtime.

`native/router-cases.json` contains the 24 hand-derived policy cases and four
malformed-input probes. `native/router.mjs` runs the exact policy/config with its
installed SDK; `native/sift.mjs` runs the actual classifier and HTTP translation;
`native/foreman.py` runs the actual Pydantic schema and assessment implementation.
These probes are reproducible entry points, not alternate application implementations.

The router tests found that missing, string and out-of-range confidence can still
permit a downgrade at this low-level policy boundary. Foreman clamps a finite
value of 8.0 to 1.0. These are recorded boundary observations, not evidence that a
real provider emitted such values or that the surrounding production validation
fails. No third-party source was modified. Fresh accuracy measurements still need
reviewed references and provider access; the other application-native runners
remain pending.
