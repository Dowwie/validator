# Focused zero-exclusion repair recheck

Verdict: **Revise**.

The repaired production and schema behavior resolves the original
zero-exclusion finding. Both concrete application branches now honor explicit
intersection for equal selections, all four application cases assert recomputed
typed results, and real binary probing produces correct schema-valid artifacts.
One repair-owned verification requirement remains incomplete: the checked-in CLI
regression exercises all four cases but never asserts a concrete recomputed
typed result or status for any of them.

## Repaired candidate reconciliation

`docs/dispatches/validator-t021-t023/013-coordinator-repaired-manifest.md`
matches its required SHA-256
`2cd39b7c27d9b0408ff0903441f3afa975616edf3644f3e502f560d6d26f0bc7`.
The complete repair chain reconciles:

| Artifact | SHA-256 |
|---|---|
| `008-coordinator-candidate-manifest.md` | `344c542a392e5387da2bff7a681dfe7f61b1f81c8975cd3e48dc78efe11a68f4` |
| `009-verifier-to-coordinator.response.md` | `23918686aebea8181d7e49196ca72ae35cfdb9e5793821e52614ecfd140ea75b` |
| `012-coordinator-to-repair-developer.prompt.md` | `6b0c0f93ef78b3e996f397d267f2a6afee1e5b4fc94559f22b735c67d7ba942f` |
| `012-repair-developer-to-coordinator.response.md` | `eca345d053562b72d0e826b1f1ce10a8bb260bd2a9b2636bd85f4f5d8bf1a685` |

The four repaired files match manifest013:

| Artifact | SHA-256 |
|---|---|
| `src/app.rs` | `f23a95f6c6f4d0f6c33cd872567ef14526243f053673a973ee0021a94d79012a` |
| `schemas/v2/comparison.schema.json` | `8bc3b8b3c481da31725fd41cadf5a61c8a13839c77e882d3483dce9ec06d3288` |
| `tests/conformance.rs` | `486c14e2f63d2f8a05eacbdd65586e6f07ca0d2de6d8718eb788293332366cd1` |
| `tests/cli.rs` | `26b4a8aa28ebedcf9121679337700965e783e7aaddb90f50c9f56d871a5dc673` |

Every inherited manifest008 artifact also reconciles, including unchanged
`src/comparison.rs`
`4ed45a737ed4ea49fc3c88728b83156acc9938f696078c193bc53979b530f845`.
No candidate mismatch or review blocker exists.

## Original finding recheck

### Production dispatch and recomputation — pass

- `src/app.rs:420-456` dispatches the single-label branch on
  `options.intersection` alone. It performs intersection compatibility,
  `intersection_scope`, checked restriction of both validated evaluations,
  concrete single-label recomputation, and typed comparison assembly.
- `src/app.rs:479-515` does the same for multi-label evaluations.
- The no-option branches at `src/app.rs:457-476` and `src/app.rs:516-535` retain
  identical-scope comparison. Their typed comparison calls still enforce exact
  selection equality, so default unequal selections continue to return
  `E_COMPARISON`.
- Unequal intersection, empty availability, immutable-output refusal and flag
  errors remain covered by the passing affected filters. T021/T022 candidate
  behavior remains accepted by unchanged hashes.

### Four equal-selection cases — production behavior passes

The checked-in application test covers single-label and multi-label, each with
equal one-ID and equal empty selections. `tests/conformance.rs:392-514` asserts
intersection scope, exact compared IDs/counts, zero exclusions, schema validity,
receipt SHA-256, and concrete recomputed values/statuses:

| Task | Population | Recomputed evidence |
|---|---|---|
| single-label | one ID | raw baseline accuracy and final candidate accuracy are `1.0` |
| single-label | empty | raw baseline accuracy is `no_data`; candidate log loss is `not_applicable` |
| multi-label | one ID | raw baseline and final candidate exact-match accuracy are `1.0` |
| multi-label | empty | raw baseline exact-match accuracy is `no_data`; candidate mean binary log loss is `not_applicable` |

An independent external probe ran the actual repaired binary for the same four
task/population shapes. Every compare command exited 0 and emitted
`scope: "intersection"`; nonempty outputs had the exact one compared ID/count 1,
empty outputs had count 0, both sides had empty excluded lists/counts 0, every
document validated, and every receipt digest matched the exact bytes. Independent
inspection confirmed defined hard metrics on nonempty populations, hard
`no_data` on empty populations, and absent signal families
`not_applicable` on empty populations.

### Schema change — pass

- `schemas/v2/comparison.schema.json:5` uses `anyOf` for the structurally
  overlapping population alternatives.
- The required scope conditionals at lines 7-8 still bind `identical` to
  `identical_population` and `intersection` to `intersection_population`.
- `intersection_population` at line 18 permits zero exclusions; the four real
  outputs validate.
- `identical_population` at line 17 still requires empty excluded arrays and
  zero counts. An independent mutation with `scope: "identical"` and one
  exclusion fails validation.
- Required root fields, root `additionalProperties: false`, and both task-family
  conditionals are unchanged. Independent mutations adding a multi-label hard
  field to a single-label document or an unknown root field fail validation.

## Required revision

### Real CLI regression omits every required typed result/status assertion

Requirement: repair prompt012 requires the four real regressions in the existing
`intersection_recomputation` **and** `cli_intersection` paths to assert concrete
recomputed typed results, along with scope, population, schema and receipt.
Recheck prompt014 specifically requires inspection and execution of the real
application and binary assertions for all four cases, including typed
result/status.

Actual: `tests/cli.rs:297-335` defines `assert_equal_intersection`. It invokes the
real binary and asserts success, receipt digest, requested scope, compared IDs and
count, zero exclusions, and schema validity. The helper returns `()` and never
reads `comparison["raw"]`, `comparison["final"]`, or
`comparison["probability"]`. Its four calls at `tests/cli.rs:336-430` add no
typed value or status assertions.

Consequently, `cli_intersection` would remain green if the binary emitted a
schema-valid but semantically wrong recomputed value/status for all four repaired
cases. The conformance test proves the public application today, and my external
probe proves the current binary today, but neither substitutes for the explicitly
required persistent binary regression assertion.

Smallest correction: change the CLI helper to return the parsed comparison (or
accept a case-specific assertion callback), then assert the same concrete values
and statuses already used by `intersection_recomputation`:

- single-label nonempty: raw/final accuracy `1.0` for the generated equal runs;
- single-label empty: hard accuracy `no_data`, absent log loss
  `not_applicable`;
- multi-label nonempty: raw/final exact-match accuracy `1.0`;
- multi-label empty: hard exact-match accuracy `no_data`, absent mean binary log
  loss `not_applicable`.

This is a test-only correction in `tests/cli.rs`; production and schema files do
not need another change.

## Recheck commands

| Command | Exit | Result |
|---|---:|---|
| `cargo test --locked --test conformance intersection_recomputation -- --nocapture` | 0 | 1 passed, 20 filtered |
| `cargo test --locked --test conformance empty_intersection_availability -- --nocapture` | 0 | 1 passed, 20 filtered |
| `cargo test --locked --test cli cli_intersection -- --nocapture` | 0 | 1 passed, 8 filtered |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | exactly 17 accepted production dead-code diagnostics plus 3 duplicate lib-test diagnostics; no repair-owned warning |
| `git diff --check` | 0 | passed |
| external actual-binary four-case probe | 0 in all cases | repaired behavior, typed values/statuses, schema and receipts independently confirmed |

The Clippy locations remain the exact accepted inventory from verdict009 and
manifest013. I reused the repair developer's passing full 44-library + 9-CLI +
21-conformance suite, formatting and locked release-build evidence as authorized;
the focused recheck produced no reason to rerun those gates.

No source, test, schema, governance, plan, index, session note, acceptance record
or Fizzy state was changed by this recheck. This required response is the only
repository artifact written; its indexing and operational follow-up remain with
the coordinator.
