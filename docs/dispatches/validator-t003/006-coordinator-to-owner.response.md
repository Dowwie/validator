# T003 bounded repair active

The coordinator read the verifier's complete Revise response at
`docs/dispatches/validator-t003/005-verifier-to-coordinator.response.md`. Review
found one substantive T003 defect: a crate-internal caller could wrap a one-label
multi-label vocabulary directly in the `SingleLabel` enum variant, bypassing the
task-specific minimum. The verifier also confirmed that the sole non-dead-code
Clippy warning is the unused T003 reexport group.

The single authorized repair cycle is dispatched at:

`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t003/007-coordinator-to-developer.prompt.md`

Active worker: `/root/coordinator/developer`, the retained Terra-high sole writer.
The repair is limited to:

- task-specific checked wrapper payloads that prevent direct construction of an
  invalid `TaskDefinition`, with focused boundary regression evidence; and
- removal of only the currently unused reexports introduced in `src/model.rs`.

The developer may not add fake consumers, suppression, later behavior, a public
SDK, or generic state/task abstractions. Per the owner decision, it must not change
or redesign `src/lib.rs` module visibility. The verifier's unchanged UUID,
identity, vocabulary mechanics, Episode, Outcome, dependency-feature, and
ownership evidence will be reused.

After repair the coordinator will freeze new hashes and return the same verifier
to the changed `src/model.rs` surface, relevant named tests/format/build, and exact
Clippy classification. Full Clippy is still expected to fail only on authorized
incomplete-consumer dead code; the unused-import group must be gone. This is one
repair/recheck, not a restarted review.
