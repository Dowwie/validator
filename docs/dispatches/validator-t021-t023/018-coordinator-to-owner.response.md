# T021-T023 repaired checkpoint handoff

Status: **Ready for owner acceptance**. T024 and later work remain paused.

## Accepted review boundary

The original frozen candidate at manifest008 received Revise verdict009 for one
T023 defect: explicit intersection on equal selected populations was silently
emitted as identical scope, and the schema forbade a legal zero-exclusion
intersection document. T021, T022 and every other reviewed T023 criterion passed.

The one owner-authorized repair cycle changed only `src/app.rs`, the comparison
schema and the two owning test files. Repaired manifest013 binds the exact candidate:

| Artifact | SHA-256 |
|---|---|
| `013-coordinator-repaired-manifest.md` | `2cd39b7c27d9b0408ff0903441f3afa975616edf3644f3e502f560d6d26f0bc7` |
| `012-repair-developer-to-coordinator.response.md` | `eca345d053562b72d0e826b1f1ce10a8bb260bd2a9b2636bd85f4f5d8bf1a685` |
| `src/app.rs` | `f23a95f6c6f4d0f6c33cd872567ef14526243f053673a973ee0021a94d79012a` |
| `schemas/v2/comparison.schema.json` | `8bc3b8b3c481da31725fd41cadf5a61c8a13839c77e882d3483dce9ec06d3288` |
| `tests/conformance.rs` | `486c14e2f63d2f8a05eacbdd65586e6f07ca0d2de6d8718eb788293332366cd1` |
| `tests/cli.rs` | `26b4a8aa28ebedcf9121679337700965e783e7aaddb90f50c9f56d871a5dc673` |

Every other manifest008 hash remains unchanged.

## Final independent disposition

The same independent Sol-high verifier reconciled manifest013, inspected the
repair, ran the affected checks and executed an independent actual-binary probe
for single-label/multi-label × equal nonempty/equal empty. The production/schema
repair passes:

- explicit intersection always runs compatibility, checked restriction, concrete
  recomputation and emits requested intersection scope;
- all four equal-selection cases have exact compared populations, zero exclusions,
  schema-valid output, matching receipt digest and correct typed values/statuses;
- the schema admits zero-exclusion intersection while keeping identical scope,
  task alternatives, required fields and root property closure strict;
- unequal intersection, default mismatch, empty availability, immutable output and
  flag-error behavior remain passing.

Verdict014 was saved before owner prompt015 arrived and requested duplicate typed
assertions in the CLI test. It remains preserved at SHA-256
`7c2d6598b135eda14cc8b2e30bdf39ed310c41fd4d02c35a79e404aadf5439a2`.
Owner prompt015 clarified that conformance owns typed recomputation assertions,
CLI owns routing/scope/population/schema/receipt assertions, and the independent
binary probe establishes the combined real-process facts; duplication is not a
contract requirement.

The verifier applied that ruling without changing or rerunning the candidate and
saved the superseding **Ready** disposition:

| Artifact | SHA-256 |
|---|---|
| `017-verifier-to-coordinator.response.md` | `0f44bdf19bc2834e563b402b60f2f9669110e452801b7f0da55620d0491b4dcc` |

No current-contract defect remains in the T021-T023 checkpoint.

## Gate evidence and operational state

- All eight original focused filters pass; the three repair-affected filters pass.
- Full locked suite passes: 44 library, 9 CLI, 21 conformance and 0 doc tests.
- Formatting, locked release build and `git diff --check` pass.
- Warning-denied Clippy exits 101 only for the owner-authorized staged inventory:
  17 production diagnostics and 3 matching lib-test duplicates. No T021-T023 or
  repair-owned warning remains. The later clean T027/T035 gates remain unchanged.
- `docs/plans/validator/verify-plan.rb` passes after dispatch/index reconciliation.
- Card185 remains Working On with T021, T022 and T023 implementation steps complete;
  its first incomplete step is owner acceptance of this frozen Ready checkpoint.
- The task-owned caffeinate hold remains active. No commit, staging, reset, cleanup
  or unrelated file change occurred.

Owner acceptance may now record this exact repaired T021-T023 candidate and
authorize the next bounded sequence. Until then, T024 remains paused.
