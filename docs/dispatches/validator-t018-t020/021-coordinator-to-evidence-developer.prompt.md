# Remove the three T020 ordinary Clippy diagnostics and run the final boundary

Role/model: retained sole evidence developer, `gpt-5.6-terra`, high reasoning,
retained context. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t018-t020/021-evidence-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat, return only
the response path, SHA-256 and terse status. Do not edit governance, plans,
dispatches, artifact-index, session notes, acceptance records or Fizzy.

Read owner prompt020, response019, `/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md`
and the complete owning functions before editing.

Starting hashes:

| Artifact | SHA-256 |
|---|---|
| `src/app.rs` | `b8f1b5b32bdff042eb5bca3b667da6fccdeb793bec04bb3b35c9bd7d4ce47b1b` |
| `src/model/multi_label.rs` | `a4f2f7fa678c525f1c3aed34fb1c3931c887fe3359aa13cd7396cbeee228be46` |
| `tests/conformance.rs` | `f184b28026250ec38da1d228d8bf563272142d21c5844ac2ba4e30e9ab8de291` |
| `tests/cli.rs` | `edaf69f7744fbc2df630e280234041e8f9bd0608ed1c2bdd719187090ca85275` |
| check schema | `1bfd2fb3a644df1203b043244b38a9ed80d404cae5068901b7f162ea6dd662b9` |
| report schema | `7215342a2958fd4d9e9735c2837130a02837d205db93b342b9f64d62a882fcf1` |
| inspection schema | `3c32ad02a1376b790294bcd655c8c4ffc711a4d90274546303e60577425fc176` |

Stop and save the exact mismatch before editing.

Make only these three Clippy-specified mechanical removals:

1. Pass `evidence` rather than `&evidence` to the single-label report builder in
   `src/app.rs`.
2. Pass `evidence` rather than `&evidence` to the multi-label report builder in
   `src/app.rs`.
3. Remove the identity `map` after zipping vocabulary labels and marginal values
   in `src/model/multi_label.rs`; collect the zipped iterator directly.

Do not change any other line except formatting forced by these removals. Do not
touch tests/schemas, clean staged dead code, suppress warnings, add fake uses,
widen visibility or change behavior/API/tolerances/oracles/dependencies.

Confirm the six exact names exist on the correct list targets, then run each
separately and nonzero:

```text
cargo test --locked --lib multi_label_checked_admission -- --nocapture
cargo test --locked --lib cross_task_boundaries -- --nocapture
cargo test --locked --test conformance multi_label_hard_oracles -- --nocapture
cargo test --locked --test conformance equal_counts_distinct_exact_sets -- --nocapture
cargo test --locked --test conformance marginal_loss_and_bins -- --nocapture
cargo test --locked --test cli shared_commands_multi_label -- --nocapture
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
git diff --check
```

Clippy may still exit 101 only for the 20 already authorized staged private-symbol
`dead_code` diagnostics. The three ordinary diagnostics must be gone and no new
ordinary class may appear. Record exact counts/symbols and owner task mappings.

Response021 must state exact changed lines, starting/final hashes, list/selected
counts, every command/exit/actual result and the residual staged inventory. Confirm
tests/schemas and all other production files are unchanged. This is the final
local candidate for immediate combined review, not owner acceptance.
