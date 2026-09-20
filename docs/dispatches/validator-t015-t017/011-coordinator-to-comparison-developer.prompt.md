# Complete T016 compatibility-boundary evidence

Role/model: current sole implementation developer, `gpt-5.6-terra`, high
reasoning, retained correction context. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t015-t017/011-comparison-developer-to-coordinator.response.md`.

Save the full substantive response before returning. In chat, return only its
path, SHA-256 and terse status. Do not edit governance, dispatch, plan,
artifact-index, session-note or Fizzy files. Do not begin T017.

## Fixed state

Read prompt010 and response010 completely. Response010 correctly discloses one
remaining unmet item and is not yet the complete T016 local handoff. Its SHA-256
is `888e4646fe2455acaf690231642c43c152cfe0011546bb34d42cbee0dfb10c3e`.

The concrete typed implementation and all passing response010 evidence remain
valid. This follow-up is test/evidence completion at one exact seam; do not reopen
the model, schema, publisher, metrics, source differences, CLI or other behavior
unless the required test exposes a real defect.

The fixed starting hashes are:

| Artifact | SHA-256 |
|---|---|
| `src/comparison.rs` | `a3e918d662aeb2ab40b5b518d01592964b145084f9f0065e7d2fb15d9053fa38` |
| `src/model/single_label.rs` | `5cff2210c215bc84e6a95777649a6e0700a3547056fbb8a93365f9d1e6b1a019` |
| `src/app.rs` | `28418611219e65eb32ffcdf6ae470f48bca944f68112e9997ee87c1f134b7ad7` |
| `schemas/v2/comparison.schema.json` | `24002dce44ffe08c9900c71a2db3a5c81e7b5b274bd05fd647c46468bf2c75d6` |
| `tests/conformance.rs` | `92edf9bfdc0c2da990052a966b3fd4dabc033fb28d741fc4590080b74a59a593` |

Stop and report any pre-edit mismatch.

## Required completion

1. In `comparison_compatibility_and_deltas`, replace the stored-report tamper
   assertion with an actual incompatible pair of separately evaluated, verified
   runs. A minimal second configuration with a different evaluation role is an
   appropriate real application case. Assert the returned diagnostic is the
   comparison-contract code, proving both replays succeeded and the T016 typed
   compatibility boundary rejected the pair.
2. Add the smallest owning-module `#[cfg(test)]` coverage for `compatible` so its
   remaining runtime axes are discriminated directly: golden digest, ordered
   vocabulary, role, selected IDs, categorical sum tolerance and bin count. The
   single-label input type itself establishes task kind; record that compile-time
   fact rather than creating a generic task-kind escape hatch.
3. Keep the compatibility implementation private. If direct construction is
   unnecessarily heavy, extract a small private compatibility-facts comparison
   used by the real typed function and test that exact helper. Do not expose test
   APIs, use report `Value` input, add a framework, or duplicate compatibility
   logic.
4. Preserve all concrete-field assertions from response010, including exact
   undefined macro-F1 and positive-infinity log-loss null reasons, directions and
   source differences. No production change is expected unless the new evidence
   finds a defect.

Run the affected owning-module test, the exact named
`comparison_compatibility_and_deltas` filter, the other two unchanged named T016
filters, `cargo fmt --all -- --check`, `cargo test --all-features --locked`, and
`git diff --check`. Reuse response010's unchanged release/Clippy evidence if only
tests/private extraction change; if production behavior changes, rerun the
applicable release and warning-denied Clippy gates and disclose the exact result.

The response must record the exact new assertions and diagnostic code, commands,
results and final hashes; state whether any production behavior changed; inherit
the full response010 evidence by exact hash; and confirm no other scope or T017
work. Completion means the disclosed compatibility-evidence limitation is gone.

