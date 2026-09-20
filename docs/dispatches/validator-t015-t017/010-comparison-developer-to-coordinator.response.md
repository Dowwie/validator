# T016 concrete comparison correction handoff

This correction replaces the rejected dynamic comparison boundary with concrete
single-label result pairs. No T017, intersection, multi-label, winner or
significance behavior, governance, index, session-note, or Fizzy work was done.

## Candidate identity

All fixed response008 starting hashes matched before edits. Final changed hashes:

| Artifact | Starting SHA-256 | Final SHA-256 |
| --- | --- | --- |
| `src/comparison.rs` | `b439555cb5f0f0a81d3b170ca348689d4e9220339d83539bdc3d69773bba830a` | `a3e918d662aeb2ab40b5b518d01592964b145084f9f0065e7d2fb15d9053fa38` |
| `src/model/single_label.rs` | `ccb45043880960ffb131ee1aa35f9f78cd4a9f2f83f958ef909d97ba8f68dd2d` | `5cff2210c215bc84e6a95777649a6e0700a3547056fbb8a93365f9d1e6b1a019` |
| `src/app.rs` | `ce33a915af602128ed13ca61bd957d946cf8ba092e7210e14a593d969e457136` | `28418611219e65eb32ffcdf6ae470f48bca944f68112e9997ee87c1f134b7ad7` |
| `schemas/v2/comparison.schema.json` | `c5639b223643f11ab4d487810e23ed92d9de8268efab2eefe370b1dfc162d6e3` | `24002dce44ffe08c9900c71a2db3a5c81e7b5b274bd05fd647c46468bf2c75d6` |
| `tests/conformance.rs` | `3b3349b14ef0f5130ca859bddbb9fde419fba6aa28c5f74c2b8a94251b2edc31` | `92edf9bfdc0c2da990052a966b3fd4dabc033fb28d741fc4590080b74a59a593` |

`src/artifacts.rs`, `src/cli.rs`, `src/lib.rs`, and `tests/cli.rs` retain their
response008 hashes.

## Correction

- `app::replay_report` now returns a private verified typed run containing the
  recomputed `SingleLabelEvaluation`, `SingleLabelResults`, and narrow recorded
  comparison metadata. The pure comparison function no longer accepts a report
  `Value` or discovers metrics recursively.
- `ComparisonHardResults` exposes explicit raw/final fields: accuracy,
  wrong-class rate, abstention rate, coverage, selective accuracy, selective
  risk, ordered concrete per-label precision/recall/F1/coverage, and macro-F1.
  `ComparisonProbabilityResults` exposes log loss, Brier score, and argmax
  accuracy. Every field holds a typed `ComparisonMetricPair` with cloned checked
  `MetricResult`s, direction, defined-only delta, typed status reason, and
  answered population where applicable.
- The comparison schema requires those explicit fields and strict nested pair,
  class, macro, hard, and probability objects.
- Category and transition increments/totals use `checked_add`; conversion from
  collection lengths returns the typed numeric diagnostic rather than truncating.
- The existing named conformance case now asserts concrete directions, exact
  undefined macro-F1 null-reason, and positive-infinity log-loss null-reason.

## Verification

| Command | Result |
| --- | --- |
| `cargo test --locked --test conformance single_comparison_transitions -- --nocapture` | Exit 0, 1 passed |
| `cargo test --locked --test conformance comparison_compatibility_and_deltas -- --nocapture` | Exit 0, 1 passed |
| `cargo test --locked --test cli cli_compare_receipt -- --nocapture` | Exit 0, 1 passed |
| `cargo fmt --all -- --check` | Exit 0 |
| `cargo test --all-features --locked` | Exit 0: 39 unit, 5 CLI, 11 conformance passed |
| `cargo build --release --locked` | Exit 0 |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | Exit 101: 24 inherited dead-code diagnostics only |
| `git diff --check` | Exit 0 |

The residual warnings are the staged dead-code inventory, now 24 because this
correction consumes two `MetricResult` accessors. They include `checked_mul`, the
unused shared/multi-label staged model types and methods, `LabelSet`, `Episode`,
`ObservationSet::values`, `EvaluationConfig::policy`, `MetricUnit::LabelDecision`,
remaining `MetricResult` accessors, `signal_availability`, and strict wire DTO
fields. No new lint class or suppression was added.

## Remaining limitation

The existing conformance incompatibility assertion still modifies a stored report,
so replay rejects it before the typed compatibility boundary. The implementation
does enforce golden digest, ordered vocabulary, role, selected IDs, and numerical
semantics through typed inputs, but this exact assertion should be replaced with
an owning-module typed compatibility test before acceptance. This handoff is
therefore a concrete implementation correction with that evidence limitation
disclosed, not an assertion that the requested correction is accepted.
