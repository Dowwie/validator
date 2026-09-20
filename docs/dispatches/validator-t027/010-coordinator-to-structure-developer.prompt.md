# Remove the model-to-artifacts dependency at the report boundary

Role/model: retained output1 structural developer, `gpt-5.6-terra`, high
reasoning, original fresh `fork_turns: none` context. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #190 — Verify Validator module boundaries and offline suite](http://localhost:3006/1/cards/190).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t027/010-structure-developer-to-coordinator.response.md`.

Save the full handoff before returning only path, SHA-256 and status. T030 has
saved a coherent checkpoint and yielded; you are the sole writer. Do not delegate,
touch T028/T030 protected evidence, or add feature/architecture work.

Read complete verifier verdict009 SHA-256
`d1265d622b8c36eaf0ad6fd6cdb54f79bd4ff855f14bf49a669d37bf8bd7c199`
and completion prompts001/002/003. Correct only F1.

Remove `crate::artifacts::InputArtifacts` from both task model modules. Change the
two report builders to receive only their three required model-owned
`ArtifactSnapshot` values (golden, predictions, config), or the strictly smaller
equivalent model-owned value. Have `src/app.rs` extract and pass those values from
the I/O-owned aggregate at the existing evaluation and replay call sites. Update
only direct local unit-test call sites required by the signature.

Write scope: `src/model/single_label.rs`, `src/model/multi_label.rs`, `src/app.rs`
and their existing same-file tests. No new abstraction/framework, artifact module
rewrite, schema/fixture/test-oracle change or unrelated cleanup. Preserve exact
report JSON/bytes, manifest ordering/paths/digests/evidence, publication, replay,
inspection, numerical outputs and public API. `model` must contain no artifacts/
I/O import after the correction; `app` retains coordination and `artifacts`
retains filesystem/hash/publication ownership.

Run the smallest focused report/replay/DM01/offline checks that prove unchanged
behavior, then the required T027 final boundary once:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
sandbox-exec -p '(version 1) (allow default) (deny network*)' cargo test --all-features --locked --offline
git diff --check
ruby docs/plans/validator/verify-plan.rb
```

Every command must exit 0 and full inventories remain 44/12/103/0. Record exact
before/after hashes, absence of the forbidden import/parameter, report-byte/replay
evidence and commands. Stop after this smallest correction; no extra confidence
or optional improvement work.
