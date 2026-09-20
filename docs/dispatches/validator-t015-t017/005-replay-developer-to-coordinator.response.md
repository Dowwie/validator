# T015 computed-path collision correction handoff

Applied correction005 only. Response002 replay/inspection behavior remains in
place; no T016 work, dependency, schema, governance, Fizzy, or index change was
made.

## Input and output identity

| Artifact | SHA-256 |
|---|---|
| Response002 input | `9e1b78221884ecbcab9dfa60e2677bf0b0b279978261f40172d1f511c5fc231c` |
| Pre-correction `src/app.rs` | `eb80bef69e0ef88600eeb4634779c85ba84697cb445fb7ce4a316dde77c2ced3` |
| Pre-correction `tests/conformance.rs` | `37656035d455a75a38a5fcbd8e245da40bb132d809966020ac66875e124f53cf` |
| Corrected `src/app.rs` | `698aeee2afa06013755b8e5a621b65b0eaea6b06d813cf14955a70509b35ee7f` |
| Corrected `tests/conformance.rs` | `3159a6af2fe6839e59fac18df8c0a6b549ff7ab9e8916bfcf5a3bdb52a8cd3b6` |

## Correction

`src/app.rs:349-404` now permits numerical tolerance only for complete structural
report paths:

- `integrity.maximum_sum_error`;
- `signals.(maximum_probability|confidence).bins.<index>.(mean_signal|empirical_accuracy)`;
- actual metric `value` locations under `raw`, `final`, `probability`, and
  `signals`, including class and macro-F1 nesting.

All other numbers compare by their exact JSON spelling. In particular, any path
under `sources`, source/preparation configuration, artifacts, identity,
population, task, policy, episodes, counts, statuses, or bindings remains exact.
This prevents a retained configuration key such as `accuracy.value` from being
treated as a computed metric.

`tests/conformance.rs:58,217-228` adds a real source configuration containing
`{"metric_like":{"accuracy":{"value":0.5}}}`. Changing only that recorded
value to `0.50000000005` fails replay even though it is within the fixed metric
tolerance. The existing test still proves unchanged replay succeeds, a nearby
fractional count and ordinary configuration value fail, and a nearby real
`raw.accuracy.value` succeeds.

## Verification

| Command | Exit | Result |
|---|---:|---|
| `cargo test --locked --test conformance replay_binding_and_result_tampering -- --nocapture` | 0 | 1 passed; collision regression and all earlier replay tamper/tolerance cases pass. |
| `cargo fmt --all -- --check` | 0 | Passed. |
| `cargo test --all-features --locked` | 0 | 39 library, 4 CLI, 9 conformance, 0 binary-unit, and 0 doc tests passed. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Only the unchanged owner-approved 26 production dead-code diagnostics remain. |
| `git diff --check` | 0 | Passed. |

No new warnings, lint classes, suppressions, or unresolved behavior failures were
introduced. The sole nonzero command remains the staged Clippy inventory recorded
in response002.
