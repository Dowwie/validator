# T003 interim warning disposition

Role/model: existing Sol-high coordinator, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t003/003-coordinator-to-owner.response.md`.
This applies the existing build charter; it does not waive the final required checks.

The coordinator reports that T003's named tests pass and Clippy fails exclusively
on dead_code for intentionally private, implemented model types whose production
consumers are not built yet. The developer correctly stopped without suppression,
placeholder use or public SDK expansion.

## Decision

Freeze and independently review T003's behavioral/type-boundary criteria now,
with the exact failed Clippy command, exit, and warning list attached. The verifier
must confirm the diagnostics really are only incomplete-consumer dead_code and
report any unrelated defect normally. A Ready verdict is scoped to T003 behavior
with this explicit unresolved integration check; it must not describe Clippy as
passed or imply the admission/steel-thread/product gate is complete.

The owner may accept this intermediate prerequisite after reading the verdict,
allowing the real consumers to be built. Do not suppress warnings, add fake runtime
uses, export private models as a public SDK, or make test code stand in for a
production caller. Run appropriate compilation/tests/format checks and preserve
the actual Clippy failure as evidence. No test/lint is skipped or weakened.

Resolve the warnings through required production wiring, with a clean full Clippy
run at the first complete path (T014) and certainly the mandatory T017 gate. Do not
promise T006 alone will remove all warnings: a private top-level validator can
remain unused until application wiring. Track the outstanding integration check
in card 183 and later evidence, rather than treating it as a new feature or issue.

This standing interpretation also applies to the same documented incomplete-
consumer warnings during the remaining pre-CLI private modules. Other warnings,
semantic failures, unsafe behavior, or a failed T014/T017 Clippy gate are not covered
and require correction. Do not repeat the same escalation without new evidence.

Issue the numbered verifier assignment with this disposition and corrected T003
scope, obtain a bounded verdict, and return it for owner acceptance. Index/link
this prompt and the required response; save your response with candidate/review
pointers and the remaining integration limit, then continue coordination.
