# Implement T016 identical-population single-label comparison

Role/model: sole implementation developer, `gpt-5.6-terra`, high reasoning,
retained context from completed T015. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t015-t017/006-comparison-developer-to-coordinator.response.md`.

Save the full substantive handoff at that path before returning. In chat, return
only the response path, its SHA-256, and a terse status. Do not edit governance,
dispatch, plan, artifact-index, session-note, or Fizzy files.

## Governing context and fixed input

Before editing, read:

- `/Users/dowwie/.codex/AGENTS.md`;
- repository `AGENTS.md`;
- `/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md`;
- `docs/plans/validator/tasks/T016.json` in full;
- both complete linked specification sections, `Comparison and successive passes`
  and `Machine interface`, in `docs/specs/validator-v1.md`;
- `docs/plans/validator/execution-contract.md`;
- the fixed steel-thread decision in `docs/plans/build-validator.md`;
- `docs/dispatches/validator-t015-t017/001-owner-to-coordinator.prompt.md`;
- the complete T015 implementation handoff
  `docs/dispatches/validator-t015-t017/002-replay-developer-to-coordinator.response.md`;
- the complete T015 exactness correction handoff
  `docs/dispatches/validator-t015-t017/005-replay-developer-to-coordinator.response.md`;
- the current implementation and tests you will extend.

The T015 local milestone is accepted for automatic advancement. Its replay
comparison exactness cases, contained stored-run reads, opaque-value fidelity,
and relocation/tamper behavior remain required and must not regress.

These are the reconciled starting hashes:

| Artifact | SHA-256 |
|---|---|
| `docs/plans/validator/tasks/T016.json` | `f5395e18f7b8599ef99a2f277372bc90b8f4e21675d9ab7824f6bc91dc5da838` |
| `docs/specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `docs/plans/validator/execution-contract.md` | `e389163c6aa6e26b0800849293f61b5d718bad7edfdd55aad5ab3ee00aed94f2` |
| `docs/plans/build-validator.md` | `0be8489b0cf5873ca40ae22b8b5fd39425631390d3d151ad3484e59b6c29aafb` |
| owner prompt001 | `db1ba26b41b411478550ec18e29e93240ff46ea4196b4cd094d186183a1b2142` |
| T015 response002 | `9e1b78221884ecbcab9dfa60e2677bf0b0b279978261f40172d1f511c5fc231c` |
| T015 response005 | `914cfd24a936595720febcb1dbad30bbb81a27f0200d1410acfc6615300a4238` |
| `src/artifacts.rs` | `50d63c1dddfdb6ef93d6613ff532ceb9c8ea86dce051b3d96baa70d082609e38` |
| `src/app.rs` | `698aeee2afa06013755b8e5a621b65b0eaea6b06d813cf14955a70509b35ee7f` |
| `src/cli.rs` | `34d0d6cc76dbbbf1682fb202834064dc5a5411ddd2c9a42cebb4ba58f63689df` |
| `src/lib.rs` | `0e05e15480f6256f789c4bd6c68d391f0eb81c7557219f964bbb5efa991ba9c7` |
| `src/model/common.rs` | `5dd0c2652439434ba9496a503c6317d8775cdbc90d8ded7b7dac5d351005d4f3` |
| `src/model/single_label.rs` | `899963586c722a37af0ce0dd8f28d03b86423c8c9bbce431b14280ae8a6cee98` |
| `schemas/v2/report.schema.json` | `4ff9c9f1ad59cd2ed73965c9cf154209e6eaf1b6871691c8d5128ca4cbd7ff46` |
| `schemas/v2/check.schema.json` | `675f6b1faa3fd67b5b42bc42122f7ccf916925920dd59d14ddee886fa90804ad` |
| `schemas/v2/receipt.schema.json` | `d820751593f82ac68cbca2ed381b305383d96d388dee417abcefec03ed3d1c00` |
| `schemas/v2/error.schema.json` | `a1be234949560a90dff9391a0fa9c42de4f0431c5c4e2f20d745730f40eb80d5` |
| `schemas/v2/inspection.schema.json` | `5e7990c05d9f87a29768c73d6d39a1579d39a9c842ce790cb780ff616182fcfa` |
| `tests/conformance.rs` | `3159a6af2fe6839e59fac18df8c0a6b549ff7ab9e8916bfcf5a3bdb52a8cd3b6` |
| `tests/cli.rs` | `5fad6fa4f66572f9af4a7ed4d23f9c0474046b0aa6ff0acd7a03fe6cafe4f9e5` |

If any listed input differs before your edit, stop and report the exact mismatch.

## Scope

Implement T016 only: compare two verified single-label runs whose selected
populations are identical. Do not begin T017. Do not implement intersection,
multi-label comparison, policy expansion, winner selection, significance,
replay fallback, or any future T022/T023 behavior.

Use the task-owned files:

- `src/comparison.rs` for pure compatibility, paired metrics/deltas and
  transitions;
- `src/model/single_label.rs` for concrete typed comparison records;
- `src/artifacts.rs` for the smallest reuse of the existing safe publisher;
- `src/app.rs` for verified-run coordination and publication;
- `src/cli.rs` for the compare command;
- `schemas/v2/comparison.schema.json` for the strict Draft 2020-12 output
  contract;
- `tests/conformance.rs` and `tests/cli.rs` for actual application/process
  evidence.

Minimal `src/lib.rs` and module-declaration/export wiring is allowed for the real
application API. Do not create a test facade, broad public SDK, second publisher,
generic artifact framework, new dependency, or unrelated cleanup. Preserve the
evaluation run's exact five-entry layout and all existing publisher safety tests.

## Required behavior

1. The application API and CLI accept baseline and candidate stored-run
   directories. Each side must pass the established T015 contained snapshot,
   binding, digest and recomputation verification before comparison. Never trust
   the stored report without replay verification and never dereference original
   source/evidence paths.
2. Default compatibility requires equal golden digest, task kind, ordered label
   vocabulary, evaluation roles, metric/numerical semantics and exact selected
   episode IDs. Reject incompatible pairs with the existing typed comparison
   diagnostic boundary. Model/source question configuration, predictions and
   decision policies may differ and must be reported rather than rejected where
   the specification permits.
3. Source IDs are local to their own run. Include each side's referenced source
   definitions and compared-population source counts/composition. Report explicit
   definition, configuration, observation-definition and preparation-descriptor
   differences even when source ID strings match. Do not infer shared origin from
   an equal local ID.
4. Build a typed complete comparison document with a caller-supplied UUIDv7,
   timestamp, implementation version and specification version; baseline and
   candidate run IDs plus exact report SHA-256 digests; scope `identical`; exact
   compared IDs/count; empty excluded IDs/counts; configuration differences with
   field path and both values; and the remaining required top-level fields from
   the specification.
5. Keep `raw`, `final` and `probability` metric families separate. Every paired
   metric preserves both typed values/statuses, population metadata, metric name
   and direction. Emit candidate-minus-baseline delta only when both statuses are
   `defined`; otherwise emit null plus a concrete reason. Answered-only metrics
   disclose baseline answered IDs/count, candidate answered IDs/count, and their
   overlap. Probability metrics use the common selected population and preserve
   applicability/preparation differences.
6. Classify every compared episode exactly once as `both_correct`, `recovered`,
   `regressed`, or `neither_correct`; retain exact IDs and counts and prove their
   sum is N. Retain changed-final-outcome IDs. Include a typed final-outcome
   transition table across every ordered class plus abstention and prove it sums
   to N. Every episode record contains both final outcomes, both correctness
   values and both local source IDs.
7. Emit no overall winner, improvement claim or significance judgment.
8. Publish exactly `comparison.json` to a new immutable private output directory
   and return the existing versioned comparison receipt with the absolute result
   path and SHA-256 of the exact published file. Reuse the existing atomic
   no-replace/private-temporary/cleanup implementation through the smallest
   internal refactor. Do not duplicate the publication algorithm. Preserve
   permissions, late-failure cleanup, and no-replace semantics.
9. Add `compare --baseline RUN --candidate RUN --out DIR` through the real binary
   and library API, preserving the established stdout/stderr/error/exit rules.
10. The comparison schema must be strict and validate the actual application
    output. Keep `additionalProperties: false` and explicit required fields at
    each modeled object boundary, consistent with the existing schemas.

## Required evidence

Implement and run the exact task filters with actual named assertions:

- `cargo test --locked --test conformance single_comparison_transitions -- --nocapture`
  must prove a coherent equal-headline-accuracy case with recovered and regressed
  IDs/counts, all four correctness categories, changed outcomes, complete typed
  class-plus-abstention transitions, per-episode source/correctness/outcomes, and
  N-sum invariants.
- `cargo test --locked --test conformance comparison_compatibility_and_deltas -- --nocapture`
  must cover every compatibility axis, same-ID/different-definition source
  differences, raw/final/probability separation, answered populations/overlap,
  defined delta arithmetic/direction, and undefined/not-applicable/non-finite
  null-delta reasons.
- `cargo test --locked --test cli cli_compare_receipt -- --nocapture` must invoke
  the built binary on two real verified runs, validate stdout against the receipt
  schema, validate `comparison.json` against the comparison schema, and prove the
  absolute receipt path and digest bind the exact published bytes.

Also preserve and run justified publisher and T015 replay regressions, then run:

```text
cargo fmt --all -- --check
cargo test --all-features --locked
cargo build --release --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
git diff --check
```

The owner-approved intermediate Clippy exception covers only the exact staged
production dead-code inventory inherited from T014. T016's real consumers should
naturally reduce that inventory. Record the exact residual diagnostic count and
symbols if Clippy still exits nonzero. Any new lint class, suppression, fake use,
public expansion solely to satisfy lint, behavior/test/build failure, or altered
tolerance is a blocker.

## Handoff

Your saved response must:

- state what changed and why, by artifact;
- map each T016 criterion above to concrete code and named assertions;
- list every command, exit code, expected result and actual result, including
  nonzero staged Clippy exactly;
- record all starting and final hashes for every touched or relied-on artifact;
- state the exact residual warnings or `none`;
- disclose any unresolved failure, ambiguity or scope conflict;
- confirm no T017, intersection, multi-label, winner/significance, governance,
  index or Fizzy work was performed.

Completion is a complete local T016 implementation handoff with the specified
evidence. It is not checkpoint acceptance; T016 advances to T017 and both receive
one combined independent review after T017.
