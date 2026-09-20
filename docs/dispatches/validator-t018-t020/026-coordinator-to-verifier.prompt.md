# Recheck the frozen T018-T020 source-kind repair

Role/model: same independent verifier, `gpt-5.6-sol`, high reasoning, retained
review context. Read-only; do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Required complete verdict:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t018-t020/026-verifier-to-coordinator.response.md`.

Save the full substantive verdict there before returning. In chat, return only
the response path, SHA-256 and verdict. Your response file is your only permitted
repository write. Do not modify candidate files, governance, plans, index, notes,
acceptance records or Fizzy.

Read repair prompt024, response024 and
`docs/dispatches/validator-t018-t020/025-coordinator-repaired-manifest.md`.
Manifest025 SHA-256 is
`e26438bd371f261e5d91a05d55312bfa1d329db75dc4e24e761d75d6bd646f11`.
Reconcile it, the original manifest022/verdict023 and every changed/unchanged hash
before relying on the repair. A mismatch is `Blocked`.

This is the focused recheck within the single authorized repair cycle. Recheck
both original findings and justified regressions only:

1. Inspect `validate_multi_label` and independently reproduce that an empty or
   nonempty multi-label artifact with an unused, otherwise well-formed
   `scored_choice` source now rejects with `E_CONFIG` before alignment, while an
   unused valid classifier source remains legal and counted.
2. Inspect the report schema's task-family source constraint. Confirm a valid
   multi-label report mutated to a well-formed scored-choice source rejects, while
   a legal single-label scored-choice report with required `question_id` remains
   valid. Ensure configuration/observations/evidence/preparation contracts were not
   weakened.
3. Confirm only `src/validation.rs`, `report.schema.json`, and
   `tests/conformance.rs` changed; no report serialization, artifact format,
   check/inspection schema, app/CLI, marginal/hard, comparison/policy, tolerance,
   dependency, export or staged-diagnostic behavior changed.

Run at least these serialized checks:

```text
cargo test --locked --lib multi_label_checked_admission -- --nocapture
cargo test --locked --lib cross_task_boundaries -- --nocapture
cargo test --locked --test conformance multi_label_hard_oracles -- --nocapture
cargo test --locked --test conformance single_report_schema -- --nocapture
cargo test --locked --test cli shared_commands_multi_label -- --nocapture
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
git diff --check
```

Clippy may exit 101 only for the same exact 20 authorized staged `dead_code`
diagnostics. Use the original verdict's already-settled evidence; do not reopen
other criteria, add new audit scope or inspect T021/T022/future cleanup.

Return exactly `Ready`, `Revise` or `Blocked`. A `Revise` finding must be a
reproducible remaining violation of the original two findings or a regression
caused by this repair, with exact location, consequence and smallest correction.
Record hash reconciliation, reproductions, commands/exits/counts, residual lint
status and final verdict.
