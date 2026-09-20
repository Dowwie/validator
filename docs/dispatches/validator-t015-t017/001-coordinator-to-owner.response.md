# T015-T017 backbone sequence acknowledged

Owner acceptance fixes repaired T007-T014 manifest036 SHA-256
`9e29e12afba887851725cbabacee9ee1ec57c49299688601a3b49c0ad3f6df68`
and independent Ready verdict037 SHA-256
`b7185fd335f3fc472e2cd7bb6ddc693e6a4ecfda3e7082e276442aae81e1240e`
as the starting point. The explicit owner028 Clippy staging remains: this is not a
warning-free or full-product checkpoint, and T027/T035 retain the clean floor.

I recorded the authorized T015 -> T016 -> T017 atomic sequence in the charter,
execution contract, session notes, artifact index and Card183. T016's task and
physical maps now include the smallest `src/artifacts.rs` write seam needed to
reuse, not duplicate or generalize, the existing safe publisher for
`comparison.json`. `verify-plan.rb` and `git diff --check` pass after this mapping.

A fresh Terra-high/fork-none sole writer is assigned T015 only. Complete local T015
evidence advances directly to T016, then complete T016 evidence advances directly
to T017. The exact T017 candidate freezes once for a fresh/read-only combined
verdict and owner acceptance. No T018 work begins before that decision. The
developer owns permanent code/tests/fixtures; the verifier remains read-only.

Card183 is restored to Working On with the first two steps complete and T015-T017
as the first incomplete step. Caffeinate PID 84732 remains active.
