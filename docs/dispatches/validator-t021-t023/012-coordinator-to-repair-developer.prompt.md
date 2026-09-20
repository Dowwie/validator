# Repair explicit zero-exclusion intersection and its strict schema

Role/model/context: return the existing T023 implementation developer as the sole
repair writer, `gpt-5.6-terra`, high reasoning. Do not delegate. This is the one
finding-driven repair cycle authorized by owner prompt001.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #185 — Build Validator policies and comparison core](http://localhost:3006/1/cards/185).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t021-t023/012-repair-developer-to-coordinator.response.md`.

Save the full substantive repair handoff there before returning. In chat return
only the path, SHA-256 and terse status. Do not edit governance, plans,
artifact-index, session notes, acceptance records or Fizzy. Do not delegate.

Read frozen manifest008, full Revise verdict009 SHA-256
`23918686aebea8181d7e49196ca72ae35cfdb9e5793821e52614ecfd140ea75b`,
owner prompt010, verifier addendum011 and current affected code/schema/tests.
Reconcile every manifest008 hash before editing. Only this reproduced T023 defect
is in scope; T021/T022 and all other reviewed T023 behavior are accepted for this
repair boundary. T024 remains prohibited.

## Required smallest correction

The actual CLI probe proved that explicit `--intersection` with compatible equal
selections succeeds but silently emits `scope: "identical"` in four legal cases:
single-label/multi-label × equal nonempty/equal empty. A correctly requested
zero-exclusion intersection document is then rejected by the schema's current
nonzero-exclusion rule.

Correct only the affected app, schema and owning tests:

1. In both concrete branches of `app::compare`, select intersection behavior on
   `options.intersection` alone. Run intersection compatibility,
   `intersection_scope`, checked validated restriction and concrete recomputation
   even when both selected-ID vectors are equal. Do not silently change the
   requested scope and do not create artificial exclusions.
2. Permit legal `intersection` population documents with both excluded lists empty
   and both counts zero. Resolve the resulting structural overlap with the
   `identical` population shape using the smallest strict schema change (`anyOf`
   or the existing scope conditionals), while preserving the `identical` scope's
   exact empty-list/zero-count constraints, task-family discrimination, required
   fields and `additionalProperties: false` behavior.
3. Add real regression assertions for all four cases: single-label and multi-label,
   each with equal nonempty and equal empty selections. Assert emitted
   `scope: "intersection"`, exact compared IDs/counts, zero exclusions, concrete
   recomputed typed results, schema validity and receipt digest. Preserve default
   identical comparison without the flag and unequal-intersection behavior.

Use the existing `intersection_recomputation` and `cli_intersection` paths rather
than a new framework. Do not broaden schema auditing, change policy/comparison
semantics, clean staged warnings or touch future work.

## Required checks and handoff

Run at minimum:

```text
cargo test --locked --test conformance intersection_recomputation -- --nocapture
cargo test --locked --test conformance empty_intersection_availability -- --nocapture
cargo test --locked --test cli cli_intersection -- --nocapture
cargo fmt --all -- --check
cargo test --all-features --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo build --release --locked
git diff --check
```

The exact authorized 17 production plus 3 duplicate lib-test staged diagnostics
may remain; no repair-owned diagnostic is allowed. Response012 must map the four
cases to real assertions, record commands/exits/counts, reconcile unchanged
accepted files, give exact hashes for every changed file and exact residual lint
inventory. Stop and report any second material failure before further correction.
On complete evidence, the coordinator will freeze a repaired manifest for the
same verifier's focused recheck of this finding only.
