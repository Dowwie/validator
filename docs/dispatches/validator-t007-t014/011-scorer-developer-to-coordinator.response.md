# T009 scorer-developer completion handoff

Status: complete local implementation milestone; not independent verification or
acceptance. This supersedes the response path in dispatch 009.

## Candidate identity

The T008 inputs matched dispatch 009 exactly, including the saved T008 response
SHA-256 `cae181e54f349f9dd4df3f3ce353059dd7155c6c7b956445adfe8a4610b54296`.
The unchanged inputs are `Cargo.toml`
`981783137668727726799ca99ee963f5333978cd6090ad9953b6d237a8e2c621`,
`Cargo.lock` `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3`,
`src/lib.rs` `bd89b6b7148e8ef25e64d28b465218a1f4afd90b8f95e6f92ed3a510de69aade`,
`src/evaluation.rs` `3b6e74d440fb76d539b3fbd338c774e81062d3d4c86e1717b3c02b6696b2ddaa`,
and the unchanged T008 fixture
`99cb21c88d4d497b8d805537d9e2648304b9aab34ea748ea6752f350641728c5`.

Final SHA-256 values:

| Path | SHA-256 |
|---|---|
| `src/model/common.rs` | `a9f823207e1fd1aaa6181b5594e5d2a8ac4df150216b7299901b52c98a3ae63b` |
| `src/model/single_label.rs` | `93a9ba28d911841a8d89249ce19846cb001d8dda217961ec95faa94043b22343` |
| `src/evaluation/single_label.rs` | `d631caff112d4cea8ec74be14da0cefe067226f23f7f600f0ce5d3b172613b33` |
| `tests/fixtures/single-label/expected.json` | `99cb21c88d4d497b8d805537d9e2648304b9aab34ea748ea6752f350641728c5` |

I did not edit the unchanged T008 fixture, dependencies, validation, schemas,
application/CLI/artifacts, plans, index, Fizzy, or the task-owned caffeinate
process. The repository remains unborn/untracked.

## Delivered behavior

- `MetricResult::positive_infinity` serializes a null value, status
  `positive_infinity`, and `special_value: "+infinity"` at
  `src/model/common.rs:1132`.
- Typed probability results, bins, signal diagnostics, and per-episode evidence
  are in `src/model/single_label.rs:272-333`. Episode evidence owns submitted and
  working categorical vectors, diagnostic argmax/max/chosen probability,
  disagreement, and reported confidence without copying opaque input.
- `src/evaluation/single_label.rs:97-218` scores every selected categorical row,
  including abstentions; computes log loss/Brier/argmax and raw-answered
  disagreement; and preserves raw hard decisions.
- `src/evaluation/single_label.rs:288-434` constructs ten exact bins for each
  signal family and computes maximum-probability ECE only. Confidence bins retain
  raw-abstention IDs as exclusions and report `no_answered_predictions` when
  applicable.

Independent local evidence is at:

- `single_label_categorical_loss_local` (`src/evaluation/single_label.rs:1160`):
  `[0.7,0.2,0.1]`, true A gives `-ln(0.7)` and `0.14`; `[1,0,0]`, true B gives
  positive-infinity loss and Brier `2`.
- `single_label_signal_bins_local` (`:1208`): tied A/B argmax uses A while the
  recorded B choice disagrees; one submitted abstention is excluded from
  confidence but retained in max-probability bins; concrete ECE is
  `(1/2)*abs(1-0.5) + (1/2)*abs(0-0.8) = 0.65`.
- `single_label_bin_boundaries_local` (`:1267`): independent explicit expected
  bin triples for adjacent binary64 values at every fixed boundary, including 0
  and 1; absent categorical/confidence signals are `not_applicable`.

## Exact ECE correction

During implementation, the first ECE fold used its running accumulator as the
denominator, so the first populated bin divided by zero. Dispatch 011 identified
the same defect. The final code at `src/evaluation/single_label.rs:338-360` keeps
the selected-population `total` parameter distinct from `accumulated`; each term
is exactly `bin.count as f64 / total as f64 * abs(accuracy - mean)`. The concrete
`0.65` regression and explicit boundary table pass.

## Checks

Every final command exited 0:

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | formatted |
| `cargo check --locked` | compiled; 161 warnings |
| `cargo test --locked --lib single_label_categorical_loss_local -- --nocapture` | 1 passed, 32 filtered |
| `cargo test --locked --lib single_label_signal_bins_local -- --nocapture` | 1 passed, 32 filtered |
| `cargo test --locked --lib single_label_bin_boundaries_local -- --nocapture` | 1 passed, 32 filtered |
| `cargo test --locked --lib` | 33 passed, 0 failed |
| `cargo test --locked --lib single_label_ -- --nocapture` | 6 passed, 27 filtered |
| `cargo test --locked --lib metric_status_precedence -- --nocapture` | 1 passed, 32 filtered |
| `cargo test --locked --lib count_overflow_is_error -- --nocapture` | 1 passed, 32 filtered |
| `cargo test --locked --lib macro_undefined_classes -- --nocapture` | 1 passed, 32 filtered |
| `git diff --check` | no whitespace errors |

Transient nonzero evidence: the first categorical-loss test exited 101 because
the ECE denominator shadowing produced a numeric diagnostic; after the targeted
fix it passed. An initial boundary assertion assumed every `next_up` crossed a
decimal bin; probing exact binary64 positions showed the declared values have
the explicit triples now asserted by the final test. No production formula is
used as the sole test oracle.

The 161 `cargo check` warnings are unresolved dead-code diagnostics from the
unwired pure scorer/application boundary, including the new probability helpers
and typed result fields. They were not suppressed. The library test build reports
18 warning groups. The hash command also emitted only the host `shasum` locale
fallback warning and exited 0.

The T014-only filters `categorical_loss_oracles`, `signal_population_bins`, and
`bin_boundary_binary64` remain pending because there is no real T014 application
API or `tests/conformance.rs`. No placeholder, private export, facade, source
include, or duplicate scorer was added. Full Clippy/all-target/release gates also
remain at T014.
