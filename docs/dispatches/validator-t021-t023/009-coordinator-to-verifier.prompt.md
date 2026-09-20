# Independently verify frozen T021-T023 candidate

Role/model: fresh independent verifier, `gpt-5.6-sol`, high reasoning,
`fork_turns: none`. Do not delegate. You did not implement this candidate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #185 — Build Validator policies and comparison core](http://localhost:3006/1/cards/185).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t021-t023/009-verifier-to-coordinator.response.md`.

Save the full substantive Ready/Revise/Blocked verdict there before returning. In
chat return only the path, SHA-256 and verdict. Do not edit source, tests, schemas,
governance, plans, index, session notes, acceptance records or Fizzy. Do not
delegate. Temporary read-only probes may live outside the repository and must not
change candidate files.

Start by reconciling the exact frozen manifest at
`docs/dispatches/validator-t021-t023/008-coordinator-candidate-manifest.md`, SHA-256
`344c542a392e5387da2bff7a681dfe7f61b1f81c8975cd3e48dc78efe11a68f4`.
Stop and save Blocked on any candidate mismatch. Read global/repository AGENTS,
owner prompt001, all three ratified tasks and linked normative sections, the three
developer handoffs named in the manifest, then inspect every frozen source/schema/
test file relevant to the criteria. T024 and later remain outside review.

## T021 review

Verify the actual checked single-label `reject_below` and multi-label
`label_thresholds` paths, not only test names:

- exact strict/equality/adjacent-binary64 boundaries and correct signal choice;
- scored-choice confidence versus vector maximum and recorded choice versus argmax;
- complete pre-score requirements, correct typed failures and no publication;
- original raw outcomes/metrics/signal populations retained while final decisions
  change; all-below multi-label remains an answered empty set;
- effective policy serialized in real report/inspection output and strict task
  schemas reject foreign policy alternatives.

Independently derive the boundary results and inspect the concrete assertions.

## T022 review

Verify typed recomputed multi-label comparison rather than serialized JSON metric
discovery or a dynamic metric registry. Derive the episode and 3x3 transition
expectations independently, including `{A}->{B}` against `{A,B}` remaining changed
and neither-correct, whole abstention for every label, and each table/category sum
to N. Verify raw/final pairs, per-side answered IDs/counts and overlap, conditional
null-delta/status/direction semantics, local source-definition/preparation
comparison, real public API/CLI publication, exact receipt, and strict task-specific
comparison schema. Preserve the accepted single-label comparison behavior.

## T023 review

Trace the checked restriction and prove it operates on existing validated
structures, rejects arbitrary/duplicate IDs, preserves task/vocabulary/policy/
source/provenance and original artifact-wide availability, and recomputes both
concrete evaluators on shared rows. Verify default selection mismatch rejection,
intersection-only selection relaxation after all other compatibility checks,
deterministic common/excluded populations, restricted source attribution, original
run/report identities, raw/final conditional populations and no full-population
claim. Independently derive the six empty-intersection availability cells for both
tasks: hard `no_data`; present family `no_data`; absent family `not_applicable`;
mixed sides independent; all undefined deltas null with the correct reason.
Inspect real CLI flag errors, receipt, schema and immutable-output refusal.

## Evidence and verdict

Rerun the exact focused filters and inspect their concrete assertions:

```text
cargo test --locked --test conformance decision_policy_boundaries -- --nocapture
cargo test --locked --test conformance policy_preconditions -- --nocapture
cargo test --locked --test conformance multi_label_comparison_transitions -- --nocapture
cargo test --locked --test conformance answered_population_overlap -- --nocapture
cargo test --locked --test cli cli_multi_label_compare_receipt -- --nocapture
cargo test --locked --test conformance intersection_recomputation -- --nocapture
cargo test --locked --test conformance empty_intersection_availability -- --nocapture
cargo test --locked --test cli cli_intersection -- --nocapture
```

Run the locked full test suite once and current warning-denied Clippy. Confirm the
reported 44 library, 9 CLI and 21 conformance tests, and exactly 17 staged
production plus 3 matching lib-test dead-code diagnostics with no T021-T023 warning.
Reuse the frozen developer's passing fmt/release/diff evidence unless a concrete
finding justifies a rerun. Do not extend into an architecture/style/future-task
audit or request optional cleanup.

Return **Ready** only if exact manifest, behavior, schemas, real paths and scoped
gates satisfy all T021-T023 criteria. Return **Revise** for each reproducible
current-contract defect, with exact file/line, violated requirement, reproduction,
expected/actual result and smallest correction. Return **Blocked** only for an
evidence/candidate condition that prevents a verdict. Record commands/exits/counts,
independent expected results, every scoped criterion disposition and the final
candidate hashes. Stop once this bounded evidence supports the verdict.
