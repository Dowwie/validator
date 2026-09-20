# T023 zero-exclusion intersection repair handoff

Status: complete. This is the sole finding-bound repair from prompt012. No T024
work or unrelated cleanup was performed, and no second material failure occurred.

## Correction

- Both concrete `app::compare` branches now take the intersection path on
  `options.intersection` alone. Equal selections therefore run original
  intersection compatibility, `intersection_scope`, consuming validated
  restriction, concrete evaluator recomputation, and typed comparison assembly.
  They publish `scope: "intersection"` with no artificial exclusions.
- The comparison schema accepts zero-exclusion intersection populations. Its
  root population alternatives use `anyOf` because an all-zero population has the
  same structural fields as an identical population; existing scope conditionals
  still force `identical` scope to the exact empty-list/zero-count shape and
  `intersection` scope to the intersection shape. Required fields,
  `additionalProperties: false`, and task-family conditionals are unchanged.
- `intersection_recomputation` now contains all four real application cases:
  single-label and multi-label, each with equal nonempty and equal empty
  selection. Each checks requested scope, IDs/count, zero exclusions, a concrete
  typed metric/status, schema validity, and receipt SHA-256.
- `cli_intersection` runs the same four cases through the real binary. It checks
  scope, IDs/count, zero exclusions, schema validity, and receipt SHA-256, while
  preserving the prior unequal-intersection, default mismatch, immutable-output,
  duplicate, unknown, and missing-value checks.

## Four regression cases

| Task | Equal selection | Assertions |
|---|---|---|
| Single-label | one selected ID | `scope: intersection`; that ID/count 1; zero exclusions; raw/final accuracy 1; valid schema and matching receipt digest. |
| Single-label | `episode_ids: []` | `scope: intersection`; empty IDs/count 0; zero exclusions; hard `no_data` and absent probability `not_applicable`; valid schema and matching receipt digest. |
| Multi-label | one selected ID | `scope: intersection`; that ID/count 1; zero exclusions; raw/final exact-match accuracy 1; valid schema and matching receipt digest. |
| Multi-label | `episode_ids: []` | `scope: intersection`; empty IDs/count 0; zero exclusions; hard `no_data` and absent marginals `not_applicable`; valid schema and matching receipt digest. |

The CLI filter executes all four rows through the actual binary. The conformance
filter executes the same four rows through the public application API, in addition
to preserving the accepted unequal-selection recomputation coverage.

## Verification

All commands ran in `/Users/dowwie/MyProjects/validator`.

| Command | Exit | Result |
|---|---:|---|
| `cargo fmt --all -- --check` | 0 | Passed. |
| `cargo test --locked --test conformance intersection_recomputation -- --nocapture` | 0 | 1 passed, 20 filtered. |
| `cargo test --locked --test conformance empty_intersection_availability -- --nocapture` | 0 | 1 passed, 20 filtered. |
| `cargo test --locked --test cli cli_intersection -- --nocapture` | 0 | 1 passed, 8 filtered. |
| `cargo test --all-features --locked` | 0 | 44 library, 9 CLI, 21 conformance and 0 doc tests passed. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Only the accepted 17 staged production dead-code diagnostics and 3 matching lib-test duplicates. |
| `cargo build --release --locked` | 0 | Passed. |
| `git diff --check` | 0 | Passed. |

The exact residual 17 production diagnostics remain at `src/model.rs` 18, 27,
52, 58, 70, 76; `src/model/common.rs` 495, 515, 523, 793, 1058, 1295;
`src/model/single_label.rs` 115; and `src/validation/wire.rs` 15, 32, 46, 228.
The three lib-test duplicates are the existing wire DTO diagnostics. No
repair-owned diagnostic appears.

## Hash reconciliation

Every manifest008 artifact reconciled before repair. The four changed artifacts
are:

| Artifact | Final SHA-256 |
|---|---|
| `src/app.rs` | `f23a95f6c6f4d0f6c33cd872567ef14526243f053673a973ee0021a94d79012a` |
| `schemas/v2/comparison.schema.json` | `8bc3b8b3c481da31725fd41cadf5a61c8a13839c77e882d3483dce9ec06d3288` |
| `tests/conformance.rs` | `486c14e2f63d2f8a05eacbdd65586e6f07ca0d2de6d8718eb788293332366cd1` |
| `tests/cli.rs` | `26b4a8aa28ebedcf9121679337700965e783e7aaddb90f50c9f56d871a5dc673` |

All other manifest008 implementation, schema, evidence, dependency and governing
handoff hashes remain unchanged, including `src/comparison.rs`
`4ed45a737ed4ea49fc3c88728b83156acc9938f696078c193bc53979b530f845`.
This response is not indexed because prompt012 reserves artifact-index,
governance, plans, session notes, acceptance records and Fizzy for the
coordinator.
