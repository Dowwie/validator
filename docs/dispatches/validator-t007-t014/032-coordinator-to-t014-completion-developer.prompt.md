# Validator T014 pre-freeze constructor-mapping correction 032

Role/model: retained fresh sole developer
`/root/coordinator/t014_completion_developer`, `gpt-5.6-terra`, reasoning `high`,
retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/032-t014-completion-developer-to-coordinator.response.md`.

Preserve every passing behavior/process assertion and source hash from response031.
Before freeze, correct one explicit mismatch with owner decision028. Response031
still maps `PreparationDescriptor::new` and `SourceDefinition::new` to T029, but
the owner ruled that out because T029 is a Python preparation script. The owner
directed production admission to use the checked RawValue constructors and any
Value conveniences needed only by tests to be test-only.

Make the smallest change in `src/model/common.rs`:

- compile `PreparationDescriptor::new` and `SourceDefinition::new` only for tests,
  preserving their current test callers and production `new_raw` use;
- likewise make `ObservationSet::is_empty` test-only because its only current
  consumer is a unit assertion, while `ObservationSet::values` remains staged for
  T015 inspection;
- add no suppression, dead read, export, replacement abstraction, formula change,
  or production behavior.

Run:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
git diff --check
```

The first, third, fourth, and fifth commands must exit 0. Record Clippy's exact
real status and full residual symbol inventory; the two constructors and
`ObservationSet::is_empty` must no longer be production dead-code diagnostics,
and no ordinary/new warning may appear. Reuse response031's unchanged 18 named
filter evidence; complete locked tests must cover their code after this cfg-only
change.

Save the response before returning, with input/output hashes, command exits/counts,
and corrected exact staged mapping. State the checkpoint remains not warning-free
and T027/T035 retain the full clean gate. Stop without starting review or T015.
