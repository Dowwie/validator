# Supersede only prompt011's S12 bin expectation

Continue the active T025 repair under prompt011 as the same retained sole
oracle/test author. Do not delegate. Keep the same required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t025/011-numeric-developer-to-coordinator.response.md`.

Read owner prompt012 and coordinator acknowledgment012 before completing S12.
Preserve prompt011 and verdict010 as history; this file supersedes only the S12
aggregate count-vector instruction. The other three findings, write boundary,
checks, response contract and single repair-cycle accounting remain unchanged.

The ratified binary64 working rule is `min(floor(10*h),9)`. Independently:

- for boundaries `i=1..8`, `next_down(i/10)`, `i/10`, `next_up(i/10)` map to
  bins `[i-1,i,i]`;
- for `i=9`, `next_down(0.9)`, `0.9`, `next_up(0.9)` all map to bin 9 because
  binary64 `next_down(0.9) * 10` rounds to exactly `9`;
- endpoint `0` maps to bin 0 and endpoint `1` maps to bin 9.

The combined vector would therefore be `[2,3,3,3,3,3,3,3,2,4]`, not the
incorrect vector copied into prompt011. Do not use either aggregate vector as the
primary proof. Use a small table/loop over the existing 29 confidence values and
perform an isolated public evaluation for each value. In each result, assert
exactly one count in the independently expected bin and zero counts in every
other bin. Retain the threshold half of S12, declared bin boundary/inclusion
metadata, episode inclusion and signal `1` in bin 9. Use the existing setup
helper; do not add a generic numerical framework or duplicate workflows.

If current product output disagrees with the corrected per-sample binary64
expectation, freeze exact input/expected/actual evidence and stop. Do not edit
production/schema/fixtures/tolerance. Response011 must explicitly record the
corrected `next_down(0.9)` derivation and the 29 isolated results so the same
verifier can correct its original illustrative vector during recheck.
