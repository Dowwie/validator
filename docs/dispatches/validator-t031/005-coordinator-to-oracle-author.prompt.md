# Correct the four T031 label-B transition cells

Resume the same sole oracle author. Do not delegate. Required response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t031/005-oracle-author-to-coordinator.response.md`.

Read the complete focused Revise verdict at
`docs/dispatches/validator-t031/004-verifier-to-coordinator.response.md`, SHA-256
`2a9002f9ff1c65be3f5030b6e95de9c9ff2d638568dcc7d7eba73858ca1d8788`.
All input identities, schemas, run metrics, probability values, comparison
categories and non-B transitions are settled. Correct only these four cells in
`tests/fixtures/acceptance-multi/expected.json`:

- full comparison label B: `absent_to_present` 0 to 1 and
  `present_to_present` 1 to 0;
- targeted intersection label B: `absent_to_present` 0 to 1 and
  `present_to_present` 1 to 0.

Update only the resulting `expected.json` hash entry in `manifest.json`. Preserve
all six frozen input/config/prediction bytes and hashes, every other expected
value, the eight-file set and prior response artifacts. Do not run Validator,
production scoring or add a script/case/checking framework.

Confirm the two corrected B tables against the frozen outcomes, their population
sums, exact before/after fixture diff, JSON parse, seven manifest hashes and
`git diff --check`. Save the compact response005 with exact new expected/manifest
hashes and commands/results, then stop for the same verifier's cell-only recheck.
