# Portfolio expansion — 2026-09-21

Authority: **Reference**. Recorded execution, not owner acceptance, new human
label approval, or production model-performance evidence.

Entry points: [portfolio index](../../examples/jev-poc/PORTFOLIO.md),
[operator instructions](../../examples/jev-poc/portfolio/README.md), and
[machine outcomes](../../examples/jev-poc/portfolio/outcomes.json).

## Added and executed

Three source projects add six task scopes: citation support, local sarcasm,
contextual sarcasm, watched-feed triage, sponsor skipping, and moderation actions.
Four tasks use original published Jev responses. Two execute original application
logic with synthetic responses. They share the existing canonical preparation,
measurement, inspection, comparison and artifact-verification path.

The exact source revisions and selected byte identities are in
[sources.json](../../examples/jev-poc/portfolio/sources.json). Source acquisition
used read-only GitHub Actions run
[35617609759](https://github.com/Dowwie/validator/actions/runs/35617609759).
The downloaded artifact SHA-256 was
`27a03332b29892ca0d3d7475f07160eb4e8114b4698f859e752c9ed9da6a1735`.
The unchanged native Validator executable used locally has SHA-256
`bf33ab4a1ca16ee7cf985ff6d83bacf58344b452580639ddd0ce4c7ad0898dab`.

Executed locally:

- Six tasks, twelve native evaluations, six paired comparisons.
- All raw/final matrix cells, per-class rates, applicable categorical log-loss and
  Brier values independently recounted; non-applicable action probabilities checked.
- Exact recovered/regressed/changed IDs, affected-case inspections, receipt hashes,
  relocation, and evidence-tamper rejection checked.
- 29 new Python tests passed. Required source/results inputs do not silently skip.
- 18 native-check groups passed, including all 44 expected action assertions
  across the 12 sponsor and 10 moderation cases at two thresholds each.
- All 55 prior top-level example tests passed without skips; all 25 prior gallery
  tests passed; five original sponsor live-controller tests passed.

An additional attempt to load the sponsor's full pipeline test file stopped at a
missing `youtubei.js` dependency in this deliberately minimal environment. No full
pipeline test was passed or claimed. The original live controller is the tested
scope; browser/audio, transcript boundary localization and full agent operation
remain outside this extension.

## Published-response findings

There are 44 recorded Atlas responses: 42 have declared reference labels; two
contextual-sarcasm references are explicitly unresolved and stay unscored.
These are small upstream-author diagnostics, not a representative or held-out
population. The source records `jev-latest`, not a resolved model version or
original request hash. Required input states are reconstructed from pinned source
and data; that does not prove identity with the original producing request.

| Task | Baseline | Candidate | Finding |
|---|---|---|---|
| Citation support, 5 references | All 5 agree with author labels | Confidence >=0.8: 4 answered, all agree | One correct answer is deferred; no error is fixed. |
| Local sarcasm, 12 references | All 12 agree | Same 0.8 gate leaves all 12 answered | No decision changes; tiny-set agreement is not general accuracy. |
| Contextual sarcasm, 10 settled references | All 10 agree | Same 0.8 gate leaves all 10 answered | Two null-reference cases remain outside scoring, at confidences 0.19 and 1.0. |
| Feed triage, 15 historical dispositions | Threshold 0.75: 3 commitments, 1 agrees, 12 deferred | Threshold 0.90: 2 commitments, 1 agrees, 13 deferred | One historical disagreement is deferred, not corrected. |

The citation case `subtle_thin_support` becomes an abstention. The author already
notes its reference is contestable. Review it independently before interpreting
that confidence as an error signal. Do not change it silently in this replay.

The two sarcasm populations are not paired. Their different inputs do not support
an inference that adding context improves accuracy. Nor does the confidence-one
answer for unresolved `AMB2` establish that the answer is right or wrong.

Feed triage has historical labels `active`, `frozen`, `rejected`. The runtime's
`batch_review` is not equated with `frozen`; it is an abstention against historical
final dispositions. Moving `case-47` from active to deferred review reduces wrong
commitments from two to one, but leaves correct commitments at one. The historical
policy and labels need review before a real optimization objective is justified.

## Native fixture findings

The sponsor controller consumes ordered utterance/seek events. Two policies use
the same synthetic replies at thresholds 0.7 and 0.9. The higher threshold recovers
`s11` and regresses `s02` and `s12`. A positive Noul at the skip limit still produces
no skip. Silence and absent replies stay distinct from a negative probability.
None of these outcomes establishes real sponsor recognition or time saved.

The moderation processor asks a Noul blocking question and a separate Choice for
category logging. The category never becomes the gate. At thresholds 0.7 and 0.9,
`m10` recovers and `m02` regresses, with the same aggregate correct count. An HTTP
failure produces an actual pass-through action and no model probability; the
example preserves both facts rather than hiding the action as abstention or
inventing p=0. This does not approve a fail-open policy or certify moderation safety.

The native code is unmodified. Node strips type-only Mastra imports; actual locked
Zod validates the response. No real Mastra session or TypeSafe endpoint is used.
Provider credentials are removed from the child environment and fetch is replaced
with bounded in-memory test responses.

## Architectural result

No Rust source, wire schema, numerical convention, or persistence contract needed
changes. The added flexibility is explicit preparation: unresolved-reference
exclusion with a durable manifest, historical-disposition comparison, and action
semantics that differ from the producing model's judgment. These are outside the
measurement core, so they do not silently weaken its admission rules.

The illustrative task rubrics remain separate from counts. The report exposes
correct/incorrect counts, coverage, per-case changes, and conditional next steps;
it never automatically chooses a winner or changes gold. The portfolio index
separates working replays from native fixture tests and unfinished capture paths.

## Limits and next use

No new inference or human approval occurred. The 22 new native cases are software
fixtures, not an expanded approved semantic dataset. The earlier 846 draft cases
are unchanged. Live captures, independent reference review and larger held-out
experiments remain separate work. This expansion does not claim exhaustive
application path coverage, despite its explicit negative and transition tests.

The published-response replays are usable now without a model key. They demonstrate
how the same utility exposes reference ambiguity, unnecessary rejection, misleading
aggregate improvements, and differences between model decisions and application actions.
