# Correct the reproduced fixed multi-label replay-tolerance path

Continue prompt005 as the same sole developer. Do not delegate. Keep the required
complete response path:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t026/005-safety-developer-to-coordinator.response.md`.

The public probe now reproduces a concrete owner001-authorized T026 product defect:

- an unmodified multi-label saved run replays successfully;
- at the fixed complete path `raw.exact_match_accuracy.value`, stored value
  `1.00000000005` versus recomputed `1.0` is inside declared replay tolerance;
- verified inspect incorrectly returns `E_INVARIANT`, stage `accounting`;
- the exact run is preserved at
  `/var/folders/6c/shrfzrvj5fx6gc0by_g7gg1r0000gn/T/validator-multi-replay-94006-54`;
- no production edit has occurred.

T026 now owns the necessary `src/app.rs` correction; physical ownership and T027
input routing are reconciled before this instruction. Read the owning
`metric_result_path` code before editing. Make only the smallest explicit fixed-
path correction so multi-label `raw`/`final` exact-match accuracy `.value` fields
receive the existing computed-float replay tolerance. Do not add tail-name/dynamic
discovery, widen tolerance, alter equality for counts/statuses/configuration, or
generalize to unproved paths.

First retain the failing assertion, then make it pass for both verified inspect
and comparison with no unrelated result change. Preserve exact failure for the
multi count/status/recorded-configuration mutations and absent comparison output.
Continue every prompt005 matrix dimension and complete response005. Record the
before/after input, values, code/stage, exact source/test hashes and the updated
mapping hashes. Stop before any additional production/schema/dependency change.
