# Validator T009 atomic scorer-developer dispatch 009

Role/model: retained sole developer `/root/coordinator/scorer_developer`,
`gpt-5.6-terra`, reasoning `high`, retained task context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/009-scorer-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator's verified single-label backbone](http://localhost:3006/1/cards/183).

T008 is reconciled as a complete local implementation milestone. Complete
**T009 only** next. This is still inside the combined T007-T014 candidate and is
not an independent review or acceptance boundary. You remain the sole writer of
implementation/tests. Do not delegate or spawn agents.

## Required context and input identity

Read in full:

- `/Users/dowwie/.codex/AGENTS.md`, repository `AGENTS.md`, and the already-read
  manage-dev-team and Rust best-practices skills;
- `docs/plans/validator/tasks/T009.json`;
- `docs/specs/validator-v1.md` section **Single-label probability metrics and
  confidence diagnostics**, plus its numerical/ordering rules and relevant
  verification cases;
- `docs/specs/validator-data-model.md` section **Reports and metric reuse**;
- owner clarification
  `docs/dispatches/validator-t007-t014/002-owner-to-coordinator.prompt.md`, only
  for its T009 rule: threshold-rejected bin preservation belongs to T021 and is
  outside this unit.

The complete T008 handoff is
`docs/dispatches/validator-t007-t014/008-scorer-developer-to-coordinator.response.md`,
SHA-256 `cae181e54f349f9dd4df3f3ce353059dd7155c6c7b956445adfe8a4610b54296`.
Confirm these exact inputs before editing:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `981783137668727726799ca99ee963f5333978cd6090ad9953b6d237a8e2c621` |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` |
| `src/lib.rs` | `bd89b6b7148e8ef25e64d28b465218a1f4afd90b8f95e6f92ed3a510de69aade` |
| `src/model/common.rs` | `2e76390eab471cdf3341d46cf7ab5f9c71651d3a62f6f5aec341bb20df76a8d8` |
| `src/model/single_label.rs` | `7022724b5f4136d40f9b71cdbcd16757036c5ca704b4dc500c566f87437ded9d` |
| `src/evaluation.rs` | `3b6e74d440fb76d539b3fbd338c774e81062d3d4c86e1717b3c02b6696b2ddaa` |
| `src/evaluation/single_label.rs` | `71cf7040a7c2821b9392ec5d269efdd861685aca128444e349d9cddcef7cdddf` |
| `tests/fixtures/single-label/expected.json` | `99cb21c88d4d497b8d805537d9e2648304b9aab34ea748ea6752f350641728c5` |

Preserve the 30-test passing library baseline. The repository is unborn/untracked;
do not stage, commit, clean, reset, modify `.zvec-grep`, edit governance/index/
Fizzy records, or start/stop task-owned caffeinate PID 84732.

## Exact behavior

Extend the typed T008 result/evidence shape with categorical probability and
reported-confidence diagnostics while preserving the original hard decisions.

- Probability metrics consume every selected row, including raw abstentions.
  Compute natural log loss from the true-class working probability and full
  multiclass Brier as the sum across all classes without division by K or 2.
  `[0.7,0.2,0.1]` for true A yields `-ln(0.7)` and `0.14`; `[1,0,0]` for true B
  yields typed `positive_infinity` log loss and Brier `2`. Never serialize NaN or
  Infinity numeric tokens.
- Derive argmax from the working vector, breaking exact ties by earliest declared
  vocabulary position. Argmax accuracy uses all selected rows. Raw-choice/argmax
  disagreement uses raw answered rows only and reports that population count.
- Episode evidence separately retains submitted and working vectors, diagnostic
  argmax, maximum probability, chosen probability, choice/argmax disagreement,
  and reported confidence. On raw abstention, chosen probability and disagreement
  are null; probability loss, Brier, argmax, maximum probability, and confidence
  remain available.
- Produce ten fixed bins per available signal with index, exact lower/upper bounds,
  last-bin inclusive semantics, count, correct count, mean signal, empirical
  accuracy, included IDs, and excluded IDs. Use
  `min(floor(10*h),9)` on the exact binary64 value. Test immediately adjacent
  binary64 values below/equal/above boundaries, including `0`, all internal
  boundaries, and `1`.
- Maximum-probability bins use all selected IDs and argmax correctness, declare
  `selected` scope/count, and alone produce top-label ECE. Confidence bins use raw
  answered IDs and recorded-choice correctness, declare `raw_answered` scope/count,
  and disclose raw abstention IDs as excluded. Do not compute or label confidence
  ECE/calibration.
- Status precedence is availability first, then empty selected population, then
  `no_answered_predictions` for confidence bins when confidence exists, N>0, and
  no raw answered row. Keep categorical and confidence availability independent.
- This unit is strictly as-recorded. Do not add rejection-policy behavior or tests;
  threshold-rejected confidence-bin retention is staged to T021.

Use checked arithmetic for all counts/bin totals and finite checks for every
derived numeric. Add the minimal exact T007 `MetricResult` support needed for
`positive_infinity` plus `special_value: "+infinity"` in
`src/model/common.rs`; preserve existing statuses, serialization, and T007/T008
tests. Do not add a generic calibration framework or public SDK surface.

## Allowed writes

- `src/model/single_label.rs` for categorical/signal result and episode-evidence
  types plus crate-private checked accessors;
- `src/evaluation/single_label.rs` for losses, deterministic argmax, bins, ECE,
  and owning tests;
- `src/model/common.rs` only for the exact positive-infinity metric representation;
- `src/evaluation.rs` only if a small shared checked arithmetic helper is actually
  required;
- `tests/conformance.rs` only if it can use the real T014 application API, which
  does not exist yet;
- the required response file.

Treat `tests/fixtures/single-label/expected.json` as the unchanged T008 oracle;
do not rewrite it or derive new expected values from production scoring. Do not
edit manifests, dependencies, validation, schemas, app/CLI/artifacts, other
fixtures, specs/plans/session notes/index/Fizzy, multi-label code, or T010+ work.

## Local evidence and pending integration filters

Add owning-module tests with independently stated analytical values for:

- finite/infinite log-loss and full Brier oracles;
- ties, abstentions, submitted-versus-working values, chosen/max probability, and
  exact raw-answered disagreement population;
- distinct maximum-probability and confidence included/excluded ID populations,
  all bin fields, empty bins, ECE only for max probability, and the confidence
  all-abstain status;
- exact adjacent binary64 bin-boundary placement through score 1;
- absent-signal `not_applicable` and empty-present-signal status behavior where
  constructible through checked inputs.

Use distinct local filter names such as `single_label_categorical_loss_local`,
`single_label_signal_bins_local`, and `single_label_bin_boundaries_local`. The
exact task conformance filters remain pending for T014's real application API:

```text
categorical_loss_oracles
signal_population_bins
bin_boundary_binary64
```

Do not create placeholders, private exports, a test facade, source includes, or a
duplicate scorer to run them early. Run:

```sh
cargo fmt --all -- --check
cargo check --locked
cargo test --locked --lib <each-new-T009-filter> -- --nocapture
cargo test --locked --lib
cargo test --locked --lib single_label_ -- --nocapture
cargo test --locked --lib metric_status_precedence -- --nocapture
cargo test --locked --lib count_overflow_is_error -- --nocapture
cargo test --locked --lib macro_undefined_classes -- --nocapture
git diff --check
```

Do not run the full Clippy/all-target/release gate by habit; T014 still owns it.
Disclose exact current unwired warnings without suppression or fake uses.

## Required response

Continue until T009 production behavior, owning tests, format, compile, and local
regressions are complete. Save the response before returning. Include exact
input/output hashes, criterion-to-code/test locations, independent analytic
derivations, every command/exit/nonzero test count, pending T014 filter status,
warning count, and any unresolved failure.

Return early only for a concrete reproduced blocker with the exact command and
smallest decision. An incomplete return without a blocker is escalated without
reissuing the same instruction.
