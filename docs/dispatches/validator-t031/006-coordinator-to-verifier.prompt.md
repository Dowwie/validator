# Recheck the four corrected T031 label-B cells

Resume the same independent verifier. Remain read-only and do not delegate.
Required response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t031/006-verifier-to-coordinator.response.md`.

Read response005, SHA-256
`cde94b723699236d87a4b212093d585a2b0fc7bfbc4746c5f0246afa15a4d4e2`.
The new expected and manifest identities reported are respectively
`c97e3a5b0debed6ae4dbf11ec2a67339c9009bb0953763bf56bde77ef297ca81`
and `a61aa36e748e5cc0063d33594e07ef35d0c7bc00c09bd087d9fac570cc8f91b0`.

Recheck only verdict004's finding: the full and targeted label-B
`absent_to_present`/`present_to_present` cells, their population sums, the exact
cell-only expected diff, the refreshed expected hash and manifest entry. Confirm
the six input identities and all other seven-cell values in both B tables remain
unchanged. Reuse every other settled conclusion from verdict004. Do not rerun
schemas or all arithmetic, run Validator/scoring, read private artifacts or
broaden review.

Save a compact Ready/Revise/Blocked verdict with exact identities, check results
and `git diff --check`, then return path, SHA-256 and verdict.
