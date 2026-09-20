# Validator T014 bounded completion dispatch 029

Role/model: retained sole developer `/root/coordinator/schema_test_developer`,
`gpt-5.6-terra`, reasoning `high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/029-schema-test-developer-to-coordinator.response.md`.
Fizzy: [Card 183](http://localhost:3006/1/cards/183).

Read owner decision
`docs/dispatches/validator-t007-t014/028-owner-to-coordinator.prompt.md`, coordinator
acknowledgment `028-coordinator-to-owner.response.md`, incomplete handoff
`026-schema-test-developer-to-coordinator.response.md`, and escalation
`027-coordinator-to-owner.escalation.md` in full. Resume **T014 only** from the
preserved candidate. You remain the sole writer. Do not delegate or begin T015,
comparison, inspection, replay, multi-label behavior, or later consumers.

## Frozen starting identity

Confirm before editing:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |
| `src/app.rs` | `18d400afe2f5c343512d4ed535b53905561f9770eb7b53731ed8344c7b610248` |
| `src/cli.rs` | `608ab90dcda6cd36f8b4d03575f47efc8f2a5977480a0c4bb670aa37933a0fbe` |
| `src/lib.rs` | `b0c5d00e000747368fc9e18e6459c61417dbbf24f06ac58077e9c4cf382ceac7` |
| `src/main.rs` | `ca701aa73cb1d687d9ff086e654bd9b08e6c285a91c9577c3b2d1eb7fc54a3c0` |
| `src/error.rs` | `5e47f1afde84a682e6d33744133820a43c14a761383717fc91012c63b330abcb` |
| `tests/conformance.rs` | `16bc7d8eb4644491ae86f30a85a5f3a69f8b471e8106351184b67e30809f9637` |
| `tests/cli.rs` | `d0e255789545846fb57c97c3338cee6344d5cf2f9f915fc3a3df88eaf6d77e4b` |
| `026-schema-test-developer-to-coordinator.response.md` | `615f6334b650df5a7eecbcabf30bb05a1ffd6311826a97b6f4a0dd680eb3db3f` |
| `027-coordinator-to-owner.escalation.md` | `ee0d9b6c9786143e8aa3117bd03f5097b5590bcca4f77dbbef182986a97f7bdd` |

Do not edit governance/index/Fizzy/session records, stage/commit/clean/reset, modify
`.zvec-grep`, or start/stop caffeinate PID 84732.

## Ordinary lint and obsolete-boundary correction

Make the smallest behavior-preserving fixes for the four ordinary Clippy findings:

- remove the `RunId` clone-on-copy;
- group the already-related `probability_results` inputs to meet complexity/style
  without changing a formula, status, count, or population;
- pass `checked_add` directly instead of the redundant closure;
- return `ratio_with_population` directly instead of `Ok(...?)`.

Fix any further ordinary non-`dead_code` current-code finding exposed by those
changes. Do not change scoring formulas or test tolerances.

When touching the decoding boundary, remove the obsolete duplicate bytes storage
and unused `Decoded::bytes`/`value` accessors now that `InputArtifacts` owns exact
bytes; preserve `into_value` and every exact-byte/duplicate-key/strict-admission
test. Remove `ObservationSet::is_empty` if it remains genuinely unused. For
`PreparationDescriptor::new` and `SourceDefinition::new`, production admission must
continue using the checked RawValue constructors; keep convenience construction
inside tests only if a current test actually needs it. Do not delete/rename DTO
version, discriminator, opaque-input, or rejected-policy payload fields: Serde uses
them to enforce the current wire contract.

Do not suppress warnings, rename fields only to hide lints, add dead reads/fake
consumers, broaden exports, or implement future consumers. After correction, the
only permitted Clippy diagnostics are the exact remaining `dead_code` symbols from
escalation027 whose accepted consumers are T015-T021 or whose DTO fields enforce
current deserialization. Reconcile by exact symbol, not count.

## Complete the seven existing process-evidence gaps

Strengthen the existing three named `tests/cli.rs` filters through the real built
binary and isolated valid-base mutations. Do not add new filter names merely to
split evidence.

1. Compile the locked `check`, `receipt`, and `error` schemas and validate the
   actual corresponding stdout documents emitted by the binary.
2. Missing/unreadable input must exit 3 with one safe schema-valid error document.
3. An existing output destination must exit 3, remain byte/type-identical, and
   leave no final/temporary partial success owned by the failed call.
4. A structurally or semantically invalid input must exit 2 with one safe
   schema-valid error document and no output directory.
5. Unknown flags, missing values, duplicate flags/settings, unsupported settings,
   and currently unimplemented commands must exit 2 with structured safe errors;
   help/version remain text exemptions.
6. A successful evaluate with bound evidence must prove the published dataset,
   predictions, config, and evidence bytes exactly equal their submitted sources.
7. A failed operation that actually opens/processes secret-bearing input or
   evidence must prove neither stdout nor stderr contains the distinctive sentinel.
   An argument-parse failure that never reads the files does not satisfy this.

Preserve the existing success, no-write check, exact report receipt digest,
help/version, numerical, schema, privacy, and T007-T013 evidence.

## Required verification and response

Run every named T007-T014 filter from the task contracts with a nonzero passing
count. Then run exactly:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
git diff --check
```

Format, complete locked tests, release build, diff check, and every behavior filter
must pass. Clippy must be run unmodified; save its real nonzero status and complete
diagnostics. It may fail only for the exact owner-permitted residual `dead_code`
symbols. Any ordinary warning, new symbol, compile/test/build failure, or behavior
defect is an actual blocker and must be reported immediately.

Save the complete response before returning. Include exact starting/output hashes,
case-to-assertion locations for all seven process gaps, every named command/exit/
nonzero count, schemas/exit/filesystem/byte/privacy results, style/obsolete cleanup,
and an exact residual Clippy symbol inventory mapped to its owner-approved status.
State clearly that the checkpoint is not warning-free and that final warning-free
gates remain T027/T035. Stop after the handoff; do not start review or T015.
