# T004-T006 focused recheck escalation 021

The completed repair recheck returned `Revise` with one material invariant gap.
Dispatch 020 authorized no further implementation loop, so replacement-developer
writes remain paused and this exact result returns to owner reassessment.

## Passing evidence

- The numeric-boundary finding is fully repaired. Opaque `1e400`, typed scalar
  `E_OBSERVATION`, marker-object separation, typed-null distinction, duplicates,
  legal numeric DTOs, exact large integers, and binary64 semantics all pass.
- The actual `validate_single_label` path correctly builds checked configuration
  before selection and preserves digest, population counts/metadata, omitted/all,
  explicit empty, subset, ordering, validation-before-selection, and alignment.
- Fallible row/evaluation constructors reject the tested explicit-population
  mismatch and validate row identity/order/count, target vocabulary, and sources.
- All 14 filters pass with nonzero counts; 24 library tests pass. Format, build,
  and diff checks pass. Clippy exits 101 only on the disclosed 107 production and
  19 library-test `dead_code` groups.
- Repair manifest 019 and every source hash remained stable through recheck.

## Sole remaining finding

`Population::new` enforces selected IDs against `requested_episode_ids` only when
the checked configuration contains `Some(ids)`. When `requested_episode_ids` is
`None`, the constructor accepts a nonempty `unselected` vector. Crate-visible
checked constructors can therefore create a complete two-ID partition selecting
only one ID while the owned configuration says omitted episode IDs, which
normatively means select all; a matching one-row `SingleLabelEvaluation` is then
accepted.

The smallest correction identified by the verifier is:

1. reject `EvaluationConfig` with omitted requested IDs when `Population::new`
   receives any unselected ID; and
2. extend the owning construction filter with that exact negative case.

No numeric, parser, actual-entry, T004, T005, or other T006 defect remains. No
implementation or verifier process is active. Task-owned caffeinate PID 84732
remains active and unchanged.

Required owner decision: authorize or reject this final narrow invariant/test
correction and define its recheck boundary. The existing repair allowance is not
silently reset.

