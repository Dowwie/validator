# Validator T003 bounded repair dispatch 007

Role/model: existing developer, `gpt-5.6-terra`, reasoning `high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Coordinator: `/root/coordinator`, Sol high. Required full response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t003/007-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator’s verified single-label backbone](http://localhost:3006/1/cards/183).

This is the single authorized repair cycle for the frozen T003 candidate. Do not
delegate or spawn agents. Read:

- `docs/dispatches/validator-t003/005-verifier-to-coordinator.response.md` in full.
- `docs/dispatches/validator-t003/006-owner-to-coordinator.prompt.md`, which narrows
  the repair and rules out module-visibility redesign.
- The corrected T003 contract/owner corrections already used for implementation.

## Required correction

Fix only these two candidate issues:

1. **Seal the task-specific vocabulary minimum.** The current public
   `TaskDefinition::SingleLabel(LabelVocabulary)` variant can accept a vocabulary
   created by `for_multi_label`, allowing a one-label SingleLabel task. Replace the
   bare payloads with task-specific checked wrapper types whose storage cannot be
   constructed directly by sibling crate modules. Keep `TaskDefinition` a closed
   single-label/multi-label enum, keep checked `single_label`/`multi_label`
   constructors and borrowed vocabulary access, and preserve all exact vocabulary
   behavior. Add focused regression evidence to `model_boundary_visibility` (or a
   small additional owning-module test if clearer) that all exposed construction
   routes enforce the single-label minimum. Do not add a generic task trait,
   phantom-state framework, wire Deserialize, mutable setter, or public SDK.
2. **Remove only currently unused reexports introduced by T003.** At
   `src/model.rs:4-5`, retain imports/reexports actually consumed by current T003
   production code and remove the unused group for `ArtifactDigest`, `Episode`,
   `EpisodeId`, `LabelIndex`, `LabelSet`, `Outcome`, `RunId`, and `SourceId`.
   Later owners can add real routing when they add consumers. Do not create fake
   uses or suppress the warning.

Do **not** change `src/lib.rs:10` or redesign module visibility. The owner decided
that the crate-internal `pub(crate) mod model` change is not a blocking public-
boundary issue and does not warrant another style/organization correction. A
demonstrated external public API expansion would be different; none is currently
reported.

## Write scope and preserved evidence

Prefer a `src/model.rs`-only implementation repair plus the required response. You
may change `src/model/common.rs` only if the smallest complete checked-wrapper fix
demonstrably requires an owning invariant/test change; explain why. Do not change
Cargo/lock, lib/error/main, other source/test files, specs/plans, fixtures, schemas,
artifact index, Fizzy, or acceptance artifacts.

The frozen candidate hashes are in
`docs/dispatches/validator-t003/004-coordinator-candidate-manifest.md`. Preserve all
unchanged behavior and the accepted T001 inputs. The verifier already established
that UUID identities, SourceId/digest, vocabulary/index/set mechanics, Episode,
Outcome, corrected T004 ownership, dependency features, and named tests otherwise
satisfy T003.

## Required evidence

Run and record:

```sh
cargo test --locked --lib model_boundary_visibility -- --nocapture
cargo test --locked --lib canonical_identifiers -- --nocapture
cargo test --locked --lib vocabulary_exact_identity -- --nocapture
cargo fmt --all -- --check
cargo test --all-features --locked
cargo build --locked --bin validator
cargo clippy --all-targets --all-features --locked -- -D warnings
```

Clippy is still expected to exit 101 on incomplete-consumer `dead_code`. The repair
must remove the `unused_imports` group and introduce no new warning class. Do not
claim Clippy passed. Capture the exact remaining diagnostic list and show that it
contains only authorized incomplete-consumer dead code. Confirm each named test
filter runs a nonzero count.

## Handoff

Save the response before returning. Include exact changed/current hashes, the
sealed construction design and regression evidence, removed vs retained
reexports, commands/exits/test counts, exact remaining Clippy classification,
unchanged evidence reused, warnings/failures/limits, processes, and resources.
Do not claim independent Ready or owner acceptance. Return only the response path.
