# Complete the demonstrated multi-label replay-path correction

Existing Sol-high coordinator; repository `/Users/dowwie/MyProjects/validator`.
Save acknowledgment to `docs/dispatches/validator-t026/006-coordinator-to-owner.response.md`.

Owner acknowledges the public failure reported under prompt005: an unmodified
multi-label run replays, but changing `raw.exact_match_accuracy.value` from1 to
1.00000000005, inside the declared tolerance, yields `E_INVARIANT` at accounting.
The failing owned run is preserved at
`/var/folders/6c/shrfzrvj5fx6gc0by_g7gg1r0000gn/T/validator-multi-replay-94006-54`.
This is a demonstrated original T026 contract violation; owner001 already permits
the necessary smallest correction in `src/app.rs` after ownership reconciliation.

Owner read `computed_result_path`, `metric_result_path`,
`signal_bin_result_path` in `src/app.rs` and the actual multi-label report
serialization at `src/model/multi_label.rs:567-622`. The current complete-path
allowlist covers single-label metrics/bins and only incidentally recognizes a few
shared multi-label names. The defect is the missing closed multi-label result
paths, not just one isolated exact-match key.

## Smallest complete correction

Extend the existing complete-path classification for the actual multi-label
hard-metric scalar values under raw/final, per-label precision/recall/F1,
answered macro-F1's metric value, per-label and mean binary losses/Brier, and
per-label marginal-bin mean probability/observed-positive-rate. Use the exact
existing report shapes and closed key alternatives. Preserve the single-label
rules. Do not add dynamic metric discovery, suffix matching, a generic registry,
new framework or a different tolerance. Necessary small private helpers to keep
the classifier clear and within the project complexity limit are ordinary local
implementation choices, not an extension API.

All counts, ratio operands, populations, statuses, array structure, bin boundaries,
recorded source/preparation configuration and observations remain exact. Do not
broaden the rule to every number or to metric-shaped recorded configuration.

Use the real public multi-label fixture with defined hard/probability/bin results
to prove within-tolerance success through inspect and compare for a representative
of each distinct new complete-path shape. Include raw/final paths and an
outside-tolerance value rejection. Keep exact count/config/status/operand negative
controls. One small fixed test-only path table is enough; no cross-product of all
metrics is required. The existing single-label replay controls must remain green.
These checks establish the complete owning-path correction rather than a patch
that handles only the first failing key.

Record the unchanged tolerance/constants and precise added path families in the
new complete handoff. Continue the original output1 completion sequence and its
remaining publication/privacy obligations; do not reset the incomplete-handoff
history or start output2 early. Update task/physical/downstream ownership before
the production edit, index/link this decision and acknowledgment in Card188, and
route the writer's consequential follow-up in a new numbered prompt. No other
production or schema scope is added.
