# T022 implementation handoff

Status: complete local implementation and verification.

## Delivered behavior

- `src/comparison.rs` adds a closed typed multi-label comparison path. It compares
  replayed `MultiLabelEvaluation` and `MultiLabelResults`, checks the established
  common compatibility axes, records paired raw/final metrics separately, and
  does not inspect serialized report metrics.
- `src/model/multi_label.rs` defines the concrete paired result, multi-label
  episode evidence, 3×3 per-label transition tables, and task-specific hard and
  marginal comparison fields. Whole abstention becomes `abstained` for every
  label; no set-power-class representation exists.
- `src/app.rs` replays either concrete run variant through the existing stored
  evidence verification path, rejects mixed task kinds, and sends the typed
  document to the existing atomic publisher. The single-label compare branch is
  retained unchanged.
- `schemas/v2/comparison.schema.json` discriminates by task kind. It accepts the
  concrete multi-label artifact and rejects task-foreign/missing hard fields.
- `tests/conformance.rs` adds the two required filters. The transition case uses
  `{A}` → `{B}` against `{A,B}`: it is `neither_correct`, changed, and produces
  independent A/B table entries. The population case proves equal selected
  populations can have baseline IDs `{1,3}`, candidate IDs `{1,2}`, and overlap
  `{1}`. It also proves both valid schemas and each task family's foreign/missing
  field rejection.
- `tests/cli.rs` adds real binary multi-label compare publication, receipt digest,
  and schema evidence.

## Schema discrimination evidence

`multi_label_comparison_transitions` validates a real multi-label result; rejects
the single-label-only `baseline_correct` field and missing
`baseline_exact_matches`. The same test validates a real single-label result;
rejects multi-label-only `baseline_exact_matches` and missing `accuracy`.

## Commands and results

All focused tests, fmt, full tests, release build, and diff check exited 0.

```text
cargo test --locked --test conformance multi_label_comparison_transitions -- --nocapture
1 passed

cargo test --locked --test conformance answered_population_overlap -- --nocapture
1 passed

cargo test --locked --test cli cli_multi_label_compare_receipt -- --nocapture
1 passed

cargo test --locked --test conformance single_comparison_transitions -- --nocapture
1 passed

cargo fmt --check
cargo test --locked
44 unit + 8 CLI + 19 conformance passed
cargo build --locked --release
git diff --check
```

The mandatory warning-denied Clippy gate found the pre-existing accepted staged
dead-code inventory and a `large_enum_variant` diagnostic for the closed
verified-run dispatcher. Both task variants are boxed before the final rerun so
the new ordinary diagnostic is removed.

## Candidate hashes

| Artifact | SHA-256 |
|---|---|
| `src/comparison.rs` | `a2cdaa46d4e008261e3b9e75edf55c6e4f54a51737fe6287250e77e14fa8f4b0` |
| `src/model/multi_label.rs` | `19f6716d1bcf9e33ce47951e676de4973001a2cafe450bdf24cafa79367a1592` |
| `src/app.rs` | `1f789f1850a97ae20c91e31777e978fbed7bcc5b591d75ef103eec5327477421` |
| `schemas/v2/comparison.schema.json` | `4bf70998e7ff0117a8a09853213d118cf2e35e6abac11c3ba22e439e8771ab2d` |
| `tests/conformance.rs` | `290dc063b95da1a9f65f5ba79ff6cffeaa84d0d539617d12a056e28523034d62` |
| `tests/cli.rs` | `4df3d3bf0c1e1fa6e75d4710e1f163c5d5cf48a2259554fc3c35db1a04131228` |

No T023 or governance, plan, artifact-index, session-note, acceptance, or Fizzy
artifact was changed.
