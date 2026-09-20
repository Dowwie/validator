# Validator T015 computed-path collision correction 005

Continue T015 as the sole writer. Save the complete correction response at
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t015-t017/005-replay-developer-to-coordinator.response.md`.
Do not begin T016.

Response002 and correction004 are otherwise preserved. Coordinator inspection
found one remaining direct violation of owner003: `computed_result_path` decides
tolerance primarily from the final/parent key name. A recorded source or preparation
configuration such as `{"accuracy":{"value":0.5}}` therefore reaches the metric
`value` whitelist even though it is recorded opaque data. Similarly named bin
fields can collide outside the real signals branch.

Make the smallest correction in `src/app.rs`:

- identify finite computed fields from their complete structural path/prefix (or
  an equivalent typed boundary), not only the last one or two property names;
- allow metric `value` tolerance only in the real `raw`, `final`, `probability`,
  and `signals` result locations, including their actual class/macro/bin nesting;
- allow bin means/empirical accuracy only under the actual signals bin arrays and
  maximum sum error only at `integrity.maximum_sum_error`;
- keep every path under `sources`, source/preparation configuration, artifacts,
  identity, population, task, policy, episodes' submitted/recorded data, counts,
  statuses and bindings exact even when keys mimic metric field names.

Add one discriminating case to the existing
`replay_binding_and_result_tampering` filter: a valid source or preparation
configuration contains a nested metric-like key such as
`{"accuracy":{"value":0.5}}`; changing only that recorded value to a nearby
within-tolerance number must fail replay. Preserve the four owner003 cases,
including permitted tolerance at a real computed metric path.

Run:

```sh
cargo test --locked --test conformance replay_binding_and_result_tampering -- --nocapture
cargo fmt --all -- --check
cargo test --all-features --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
git diff --check
```

All except the declared staged Clippy limitation must pass. Clippy may contain only
the unchanged owner-approved inventory. Save the response with input/output hashes,
the exact path rule, collision reproduction, commands/counts and unresolved failures.
