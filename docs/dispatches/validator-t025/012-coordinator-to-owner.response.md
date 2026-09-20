# T025 binary64 correction acknowledgment

I read owner prompt012 in full and accept its correction to the active repair.
The prompt has SHA-256
`85443e02a34f53108b27f67fefd81ea48606284b2b253e6ac5b139fce07261c9`.

The ratified binary64 rule `min(floor(10*h),9)` governs S12. For boundaries
`i=1..8`, below/equal/above map to `[i-1,i,i]`; for `i=9`, all three map to bin
9 because `next_down(0.9) * 10` rounds to exactly 9. With endpoints, the aggregate
count vector would be `[2,3,3,3,3,3,3,3,2,4]`, but the repair will use the more
discriminating owner-selected construction: 29 isolated public evaluations, each
asserting count one in its independently expected bin and zero in all others.

I preserved verdict010 and prompt011 unchanged as historical records. I saved the
superseding consequential instruction in
`013-coordinator-to-numeric-developer.prompt.md` and routed it to the current sole
test author before accepting any S12 expectation or product diagnosis. The
threshold half, bin metadata, signal-one evidence, the other three verifier
findings, the one repair-cycle accounting and the original focused checks remain
unchanged. Production, schemas, fixtures and tolerance remain read-only.

The same verifier will independently confirm this corrected arithmetic during the
focused recheck and explicitly supersede its illustrative count vector. T026
remains undispatched pending T025 owner acceptance.
