# T022 final candidate reconciliation

Status: complete reconciliation; no source changed in this turn.

Response005 predates the last `VerifiedRun` boxing correction. The current
`src/app.rs` hash is the required
`e102572012bc0204a0312b88607914700486bcc020925cd84ab6deaba0810d5f`.
The final six-file candidate is:

| Artifact | SHA-256 |
|---|---|
| `src/comparison.rs` | `a2cdaa46d4e008261e3b9e75edf55c6e4f54a51737fe6287250e77e14fa8f4b0` |
| `src/model/multi_label.rs` | `19f6716d1bcf9e33ce47951e676de4973001a2cafe450bdf24cafa79367a1592` |
| `src/app.rs` | `e102572012bc0204a0312b88607914700486bcc020925cd84ab6deaba0810d5f` |
| `schemas/v2/comparison.schema.json` | `4bf70998e7ff0117a8a09853213d118cf2e35e6abac11c3ba22e439e8771ab2d` |
| `tests/conformance.rs` | `290dc063b95da1a9f65f5ba79ff6cffeaa84d0d539617d12a056e28523034d62` |
| `tests/cli.rs` | `4df3d3bf0c1e1fa6e75d4710e1f163c5d5cf48a2259554fc3c35db1a04131228` |

The other five response005 hashes reconcile exactly. The only stale identity was
`src/app.rs`, caused by boxing both concrete verified-run variants to remove the
new `large_enum_variant` warning.

Focused post-boxing verification was already run after that correction:

```text
cargo test --locked --test conformance multi_label_comparison_transitions -- --nocapture
exit 0; 1 passed
```

`cargo clippy --all-targets --locked -- -D warnings` now reports exactly 17
pre-existing staged production dead-code diagnostics plus 3 duplicate lib-test
dead-code diagnostics, for 20 total. No T022-owned diagnostic remains and
`large_enum_variant` does not recur. `git diff --check` exits 0.

Response005's unchanged full test suite, release build, real CLI compare,
schema-discrimination, and named-filter evidence remains applicable. No full gate
was repeated for this hash-only reconciliation.
