# Validator T003 focused repair recheck 009

Role/model: existing verifier, `gpt-5.6-sol`, reasoning `high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required full response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t003/009-verifier-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator’s verified single-label backbone](http://localhost:3006/1/cards/183).

Recheck the single T003 repair cycle only. Do not restart orientation, revisit
unchanged UUID/identity/Episode/Outcome evidence, propose module-visibility design,
or expand the review. Read repair prompt/response 007, owner decision 006, and the
repaired manifest:

`docs/dispatches/validator-t003/008-coordinator-repair-manifest.md`, SHA-256
`ceb00209087532787858fbb60045d5dbb2f9ae10c91ff60c5e5be4eee3c802c3`.

Implementation writes are paused. Bind these hashes:

- `src/model.rs`:
  `3275697520566394a2a20d4011ad28b3912d9d4773a44203980127d3f2d4d4d1`
- `src/model/common.rs`:
  `ee0b0169d88707663c7b1540d1406868ba61cb0db2fd4d36dd2728d3baf9cecf`
- Repair response:
  `15dfc00c6439af28102278ea0df092dd342db9ddef92bcb3a6d4c6fad2430e3f`

Confirm all other candidate hashes equal manifest 008. Stop Blocked on hash drift.

## Required focused assessment

1. Verify `TaskDefinition` variants now carry task-specific checked wrappers with
   private vocabulary storage, and that sibling crate modules cannot place a
   one-label multi-label vocabulary into the SingleLabel variant. Check both enum
   factories and wrapper construction surfaces; borrowed vocabulary access and the
   closed two-variant enum must remain correct.
2. Verify focused regression evidence fails one-label SingleLabel through every
   exposed construction route and permits one-label MultiLabel.
3. Verify the old unused reexports/import warning is gone; current production code
   retains only imports actually used now.
4. Confirm no fake caller, suppression, public SDK expansion, `src/lib.rs` change,
   later behavior, generic framework, or unrelated mutation entered the repair.

Run only the affected checks and record exact exits/counts:

```sh
cargo test --locked --lib model_boundary_visibility -- --nocapture
cargo test --locked --lib canonical_identifiers -- --nocapture
cargo test --locked --lib vocabulary_exact_identity -- --nocapture
cargo fmt --all -- --check
cargo test --all-features --locked
cargo build --locked --bin validator
cargo clippy --all-targets --all-features --locked -- -D warnings
```

Clippy should exit 101 with only owner-authorized incomplete-consumer `dead_code`.
Confirm the exact classes/locations and that `unused_imports` plus every other
warning class are absent. Do not call Clippy passed or waived. Reuse unchanged
evidence from response 005.

Return one verdict for the repaired candidate: Ready, Revise, or Blocked. A Ready
verdict is behavioral T003 readiness with the explicit dead-code integration limit
and clean Clippy still required at T014/T017. Any repeated/substantive defect must
include requirement, location, reproduction, consequence, and smallest correction;
the coordinator cannot start another repair without owner reassessment.

Save the complete response before returning, including hashes, changed-surface
evidence, commands/results, remaining limit, active processes, and confirmation of
read-only review. Return only the response path.
