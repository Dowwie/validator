# T020 implementation handoff

Implemented the concrete marginal scorer and began the closed multi-label application path.

## Completed work

- `src/evaluation/multi_label.rs` scores every selected `N*K` marginal: observed-positive `-ln(p)`, observed-negative `-ln_1p(-p)`, explicit positive-infinity at observed zero probability, binary Brier, and ten fixed bins paired with reference positivity.
- `src/model/multi_label.rs` carries typed marginal probability and bin results and serializes a task-tagged multi-label report without exposing opaque payloads.
- `src/app.rs` now admits through `ValidatedTask`, checks and evaluates both concrete tasks, publishes multi-label runs through the existing immutable artifact path, and rebuilds either report during inspection. Comparison remains single-label-only.
- `schemas/v2/check.schema.json` and `schemas/v2/report.schema.json` admit concrete task-tagged alternatives.
- `tests/conformance.rs` adds `marginal_loss_and_bins`, using the public evaluate/published-report path. Its finite discriminator is `[.8,.7]` against `{A}`: binary Brier `(0.2^2 + 0.7^2)/2 = .265`; log loss `(-ln(.8)-ln(.3))/2`.

## Verification

- `cargo check --locked`: exit 0, inherited staged dead-code diagnostics only.
- `cargo test --locked --lib multi_label_hard_accounting -- --nocapture`: exit 0, one selected test.
- `cargo test --locked --test cli cli_check_evaluate -- --nocapture`: exit 0, one selected test.
- `cargo test --locked --test conformance marginal_loss_and_bins -- --nocapture`: exit 0, one selected test.

## Incomplete / not ready for acceptance

Required `multi_label_hard_oracles`, `equal_counts_distinct_exact_sets`, `shared_commands_multi_label`, full schemas, and combined gates remain incomplete. `inspection.schema.json` and the required CLI integration evidence have not been updated. This is a partial implementation handoff, not a frozen candidate.

No T021/T022 behavior or multi-label comparison was implemented. The only changed dispatch artifact is the required correction acknowledgment; no governance, Fizzy, index, plans, or session notes were edited.
