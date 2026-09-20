# Correct the review's binary64 bin expectation without widening the repair

Existing Sol-high coordinator; repository `/Users/dowwie/MyProjects/validator`.
Save acknowledgment to `docs/dispatches/validator-t025/012-coordinator-to-owner.response.md`.

Owner read the complete verifier verdict010 and repair prompt011. All four
test-evidence findings are within the authorized repair cycle. However, the
review's expected S12 count vector and prompt011's copied vector are numerically
wrong under the ratified arithmetic. Correct that instruction before the repair
author changes expectations or reports a false product defect. This is the same
repair cycle and does not authorize production, schema, fixture or tolerance edits.

## Independent evidence and decision

`docs/specs/validator-v1.md:483-485` requires binary64 working arithmetic for
decisions. Its line609 bin rule is `min(floor(10*h), 9)`. An independent binary64
calculation, using JavaScript DataView to decrement the bit pattern of `0.9`,
gives `h = 0.8999999999999999`; binary64 `10*h` rounds to exactly `9`.
Consequently this `next_down(0.9)` belongs to bin9 under the stated rule. It is
incorrect to assume every `next_down(i/10)` belongs to bin `i-1`.

For boundaries i=1 through8, the below/equal/above bin triples are
`[i-1,i,i]`. For i=9, they are `[9,9,9]`. Including endpoints0 and1, the combined
count vector is `[2,3,3,3,3,3,3,3,2,4]`, not the review's
`[2,3,3,3,3,3,3,3,3,3]`. This derivation used no Validator output or scoring code.
Preserve the original verdict and dispatch as history; do not overwrite them.

## Bounded S12 correction

Replace prompt011's hardcoded count-vector instruction with independently fixed
binary64 expectations above. Prefer isolated public evaluations for each of the
29 confidence values: assert exactly one count in its expected bin and zero in
every other bin. This directly discriminates each adjacent-boundary sample and
avoids compensating swaps hidden by aggregate counts or floating mean tolerance.
Retain the threshold half, bin boundary/inclusion metadata and signal1 in bin9.
Do not add a generic numerical framework or duplicate whole workflows. Use the
existing setup helper and a small test-only table/loop. The ratified formula and
binary64 semantics govern; no higher-precision reinterpretation is authorized.

Route this correction in a new numbered file to the current sole repair author.
Have the same verifier independently confirm the arithmetic in its focused
recheck, retain the valid missing-discrimination finding, and explicitly correct
its original illustrative count vector. The other three findings, write scope,
existing checks, unchanged evidence reuse and stopping boundary remain unchanged.
Index/link this decision, acknowledgment and worker follow-up in Card187. T026
remains undispatched until owner acceptance of T025.
