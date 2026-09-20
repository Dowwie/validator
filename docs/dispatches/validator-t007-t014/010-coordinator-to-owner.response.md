# T008 oracle-explanation and verifier-availability acknowledgment 010

I recorded the T008 thread derivation correction without changing the historical
handoff, source, tests, or independent oracle:

- A has TP=1, FP=1, FN=1, so F1=`2/(2+1+1)=1/2`.
- B has TP=0, FP=0, FN=1, so its direct F1 is `0`.
- C has TP=1, FP=0, FN=0, so F1=`1`.
- Macro-F1 is `(1/2 + 0 + 1)/3 = 1/2`.

The original T008 response remains frozen at SHA-256
`cae181e54f349f9dd4df3f3ce353059dd7155c6c7b956445adfe8a4610b54296`,
because T009 binds that identity. `tests/fixtures/single-label/expected.json` and
the actual-entry owning test already use the correct macro value; this correction
therefore requires no implementation, fixture, or test mutation. The correction
is indexed beside the T008 handoff for the eventual verifier.

I also reconciled the current runtime inventory: `list_agents` does not show the
previous `/root/coordinator/verifier` handle. Its saved prior verdicts remain
authoritative historical evidence, but the handle is presently unavailable. At
the T014 frozen boundary, if it remains unavailable, I will provision exactly one
replacement `gpt-5.6-sol`, high-reasoning, `fork_turns=none` verifier with the
complete file-based frozen candidate and criteria. This replaces the missing
reviewer role; it is not an extra or current review, and developer self-review
will not substitute for it.

T009 remains active and unchanged with `/root/coordinator/scorer_developer`.
Card 183 stays Working On. Task-owned caffeinate PID 84732/session 29011 remains
active and untouched.
