# Repair the multi-label source-kind boundary and matching report schema

Role/model: retained sole T020 evidence developer, `gpt-5.6-terra`, high
reasoning, retained context. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #184 — Build Validator’s concrete multi-label core](http://localhost:3006/1/cards/184).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t018-t020/024-repair-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat, return only
the response path, SHA-256 and terse status. Do not edit governance, plans,
dispatches, artifact-index, session notes, acceptance records or Fizzy. Do not
delegate.

Read the complete independent Revise verdict023, T018/T020 source-kind criteria,
the `Scored-choice profile` specification, current `validate_multi_label` source
admission, source model accessors, report schema task-family selection and the
existing admission/schema tests before editing.

This is owner001's single finding-driven repair cycle. The two findings are two
manifestations of one current contract: a multi-label artifact/report may declare
only classifier sources, including sources unused by predictions. Do not broaden
the repair.

Starting hashes:

| Artifact | SHA-256 |
|---|---|
| verifier verdict023 | `41ef819eeb12042ebebd2e51f86a52589150b95354fb1d990a575bd12fbf34e8` |
| `src/validation.rs` | `c6e40115a863bba54fab8f84b32beba53b56df153de008bb3fbc63e6ea2886fa` |
| `src/model/common.rs` | `dfe120adb33f7a773edd5a8c44c8331fb918e417aec52dfc3d1c0a8077138791` |
| `schemas/v2/report.schema.json` | `7215342a2958fd4d9e9735c2837130a02837d205db93b342b9f64d62a882fcf1` |
| `tests/conformance.rs` | `f184b28026250ec38da1d228d8bf563272142d21c5844ac2ba4e30e9ab8de291` |
| `tests/cli.rs` | `edaf69f7744fbc2df630e280234041e8f9bd0608ed1c2bdd719187090ca85275` |
| `src/app.rs` | `3324dad26b9571529bdfb1a75b2bb17a9900bbb8f1a65ad822e7b389b76c6671` |

Stop and save the exact mismatch before editing.

## Exact repair

1. In `validate_multi_label`, immediately after shared source admission and before
   row alignment/evaluation construction, reject every declared source whose kind
   is not `Classifier`. This must include fully well-formed unused sources in an
   empty or nonempty artifact. Reuse the established configuration diagnostic
   (`E_CONFIG`); do not change shared source decoding or single-label behavior.
2. Add a focused regression to existing T018 owning tests using a valid empty
   multi-label dataset, empty predictions and one unused well-formed
   `scored_choice` source. Assert rejection at checked admission with the stable
   configuration diagnostic. Preserve the existing referenced-source negative.
3. Constrain the report schema so the multi-label task branch accepts only
   classifier source definitions while the single-label branch retains its legal
   classifier and scored-choice alternatives. Keep source configuration,
   observations, preparation and evidence bindings at their existing exact/opaque
   contracts. Do not change report serialization or artifact format.
4. Add a focused schema mutation of a valid multi-label published report: replace
   a source with an otherwise well-formed scored-choice definition including its
   required `question_id`, and assert report-schema rejection. Preserve valid
   single-label scored-choice schema behavior.

Do not touch check/inspection schemas, marginal/hard accounting, app/CLI behavior,
comparison/policy, dependencies, tolerances, public exports or the 20 staged
diagnostics. Do not add a generic source-policy framework.

## Evidence

Run the affected filters and justified full boundary once:

```text
cargo test --locked --lib multi_label_checked_admission -- --nocapture
cargo test --locked --lib cross_task_boundaries -- --nocapture
cargo test --locked --test conformance multi_label_hard_oracles -- --nocapture
cargo test --locked --test conformance single_report_schema -- --nocapture
cargo test --locked --test cli shared_commands_multi_label -- --nocapture
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
git diff --check
```

Every filtered command must select one test. Clippy may exit 101 only for the same
20 owner-authorized `dead_code` diagnostics and no ordinary/new class.

Response024 must map both verdict findings to code/schema/assertions, record the
before reproduction and after results, exact command exits/counts, final hashes,
residual lint inventory and confirmation that no unrelated file/behavior changed.
This repair freezes a candidate for the same verifier's focused recheck; it is not
owner acceptance. Any second material defect returns to the owner rather than
starting another repair cycle.
