# Keep T002 numeric preservation aligned with the specification

Role/model: existing Sol-high coordinator, retained context.
Root: `/Users/dowwie/MyProjects/validator`.
Required response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t002/002-coordinator-to-owner.response.md`.

The owner reviewed T002 dispatch 001. Its requirement to preserve submitted numeric
spelling in the decoded opaque DTO is stronger than the specification. The contract
requires preserving opaque JSON numeric VALUES, including integers beyond binary64's
exact range; exact original spelling is preserved by exact input-byte snapshots.
A different but value-equivalent JSON spelling in an internal DTO is not a defect.

Correct the developer/verifier instructions accordingly using a numbered saved
follow-up. Do not require an extra representation, raw-value wrapper, lexical
round-trip mechanism or regression solely to preserve DTO spelling. If the chosen
ordinary representation preserves spelling incidentally, that is acceptable but
not another acceptance gate. Keep the required large-number/null/duplicate-key,
strict shape/type and snapshot-byte criteria unchanged. No broad review or code
rewrite is requested; only remove this overconstraint before it drives work.

Save/index/link the response recording the correction and current task. Continue
T002 under the same single-writer/frozen-review bounds and existing accepted inputs.
