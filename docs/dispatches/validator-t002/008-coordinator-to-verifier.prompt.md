# Validator T002 focused repair recheck 008

Role/model: existing verifier, `gpt-5.6-sol`, reasoning `high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t002/008-verifier-to-coordinator.response.md`.

Recheck the single T002 repair only. Reuse unchanged evidence from verdict 005;
do not restart broader wire review or reintroduce DTO lexical spelling. Read repair
prompt/response 006 and repaired manifest 007, SHA-256
`3e0c155c1a64e6ec75d8bf518400fcc3c12d283ec3774de705833eb912843e91`.

Writes are paused. Bind changed hashes:

- `src/validation.rs`:
  `6c6c71080372b1d5b0a1bf707b88e1db694d88a142a3f2d94b8a467ff0020f72`
- `src/validation/wire.rs`:
  `e81c528e0659e1fb4b2364be82639bbb27175947b042145a0f314ec00e827d17`
- Repair response:
  `88cea4d46bc9cf14ed19766d0077d55ae03ac7f58b9a13ea5445b138404255ee`

Confirm all other hashes equal manifest 007; return Blocked on drift.

Verify only:

1. legal optional `observation_definitions` and row `observations` decode through
   the existing strict prediction entry;
2. definitions have exactly kind/description/optional question ID and the closed
   five kinds;
3. row values cover scalar, Bernoulli, reported-confidence numeric values and
   categorical/label-marginal numeric maps with explicit tags;
4. unknown kind, extra field, and wrong primitive fail while semantic nonblank/
   range/vector/binding/sum/completeness checks remain deferred to T004/T005;
5. no alternate decode path, public API, dependency change, suppression, fake
   consumer, or unrelated mutation entered the repair.

Run:

```sh
cargo test --locked --lib wire_tags_and_fields -- --nocapture
cargo test --locked --lib strict_json_keys -- --nocapture
cargo test --locked --lib opaque_number_and_null -- --nocapture
cargo fmt --all -- --check
cargo test --all-features --locked
cargo build --locked --bin validator
cargo clippy --all-targets --all-features --locked -- -D warnings
```

Clippy should exit 101 on incomplete-consumer `dead_code` only. Confirm no other
warning class, without claiming pass/waiver. Reuse prior duplicate/numeric/byte/
privacy/dependency evidence.

Return Ready, Revise, or Blocked for the repaired candidate. A repeated material
failure must be fully sourced and returns to owner reassessment; no second repair
is authorized. Save hashes, focused criterion evidence, commands/results, warning
classification, remaining limit, processes, and read-only confirmation before
returning only the response path.
