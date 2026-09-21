# Validator example portfolio

This index distinguishes executed model-response replay, native software tests,
and draft integrations. An adapter name is not a claim of live model validation.
All examples use the same strict classification core. Preparation and task rubrics
remain outside that core. No workflow orchestrator or task manager is introduced.

## Published Jev responses: runnable offline feedback

| Example | Task | What can be reproduced now |
|---|---|---|
| [jgrep](README.md#quick-offline-proof) | Binary code-change/current-function judgments | Six saved configurations, four paired comparisons; 20 author-diagnostic cases per task |
| [Spam detection](madewithjev/README.md) | Binary Noul decisions | Plain versus detailed criteria; false alarms versus missed spam |
| [Phishing and email](madewithjev/README.md) | Three-class Choice | Separate cohorts, context/framing contrasts; cohort regressions retained |
| [Banking77](madewithjev/README.md) | 77-class Choice | Native-confidence rejection; answered accuracy versus coverage |
| [Citation support](portfolio/README.md#citation-support-rejection-is-not-correction) | Three-class Choice | Five declared references; a confidence gate withholds a correct thin-support answer |
| [Local sarcasm](portfolio/README.md#sarcasm-unknown-is-not-false) | Binary Choice | Twelve declared references; same-text intent classification |
| [Contextual sarcasm](portfolio/README.md#sarcasm-unknown-is-not-false) | Binary Choice | Ten scored references plus two explicitly unresolved responses retained outside scoring |
| [Watched-feed triage](portfolio/README.md#feed-triage-deferred-is-not-an-expected-class) | Two Nouls composed into action | Fifteen historical dispositions; active/rejected commitments versus deferred review |

These are saved-response replays, not fresh inference. Every entry retains its
own population and provenance limits. Perfect agreement on a small authored set
is not a production estimate. Do not sum datasets or compare unrelated averages.

## Original application code: explicit fixture responses

| Example | What runs | What is NOT established |
|---|---|---|
| [Sponsor skipping](portfolio/README.md#sponsor-skipping-sequence-order-matters-inside-a-case) | Original live controller, ordered hearing/seek state, 12 cases in two arms | Real sponsor recognition, audio capture, exact skip boundaries or time saved |
| [Mastra moderation](portfolio/README.md#moderation-the-action-survives-a-missing-verdict) | Original processor and Zod parsing, 10 cases in two arms | Live moderation quality, complete agent operation or safety acceptance |
| [Sift](sift/README.md) | Pinned classifier/provider and evidence-bound capture/comparison path | Live accuracy; all new semantic references still need independent approval |
| [Foreman](README.md#native-application-boundary-tests-run-locally) | Pinned observation/assessment types and injected-client path | Autonomous worker performance or assessment accuracy |
| [Router](README.md#native-application-boundary-tests-run-locally) | Original routing policy, 24 deterministic fixtures | Cheapest model that succeeds on a real task |
| [Clean Code Review](madewithjev/README.md) | Pure question builder and a four-label protocol fixture | Full windowing/runtime or live code-review accuracy |

The new sponsor and moderation cases are deliberately synthetic. Raw model replies,
application actions and label references are not conflated. These tests supplement,
not replace, the published-response examples.

## Preparation paths and proposals, not complete applications

[The original application plans](../../docs/plans/jev-validation-pocs.md) and
[general instructions](README.md) include Upwork triage, tax form classification,
classifier.dev single-/multi-label tasks, document filing, compaction and jev-align.
Their source-shaped adapters and review-pending proposals are available. Full native
capture and independently reviewed model-performance experiments are not all complete.
The original 846 proposals and focused Sift packet are not approved golden data.

## Choose an example by the question you need answered

For a first no-key replay, use **jgrep or citation support**. For error-cost tradeoffs,
use **spam or Banking77**. For unknown references, use **contextual sarcasm**. For a
stateful policy, use **sponsor skipping**. For the difference between a model answer
and an application action, use **moderation or feed triage**. For multiple simultaneous
labels, start from **Foreman, classifier.dev multi, or Clean Code Review**, retaining
the fixture-versus-observed distinction above.

A useful feedback result identifies a decision to investigate. It need not show
an improved average or declare a winner. The intended review remains
**run → review → recommend**; developers choose and implement changes.
