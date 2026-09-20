# T031 label-B transition-cell correction handoff

## Status

Ready for the same verifier's cell-only recheck. I read verdict004 with SHA-256
`2a9002f9ff1c65be3f5030b6e95de9c9ff2d638568dcc7d7eba73858ca1d8788` and
corrected only the four specified label-B transition cells.

`001` has baseline `{A,C}` and candidate `{A,B}`; B is therefore
`absent_to_present` independently of the reference `{A,B}`. The full B table
now has `absent_to_present=1` and `present_to_present=0`, with total `2`; the
targeted B table has those same values, with total `1`. The other seven cells in
each table are unchanged. This is the exact fixture change: those two fields in
each of the full and targeted B tables, plus the resulting manifest digest entry.

The new `expected.json` SHA-256 is
`c97e3a5b0debed6ae4dbf11ec2a67339c9009bb0953763bf56bde77ef297ca81`.
The new `manifest.json` SHA-256 is
`a61aa36e748e5cc0063d33594e07ef35d0c7bc00c09bd087d9fac570cc8f91b0`.
The manifest's `expected.json` entry is that new expected hash; its other six
frozen input/config/prediction hashes are unchanged and it has no self-hash.

| Check | Exit | Result |
|---|---:|---|
| JSON parse of `expected.json` and `manifest.json` | 0 | Both parse. |
| Temporary Ruby B-cell/table-sum, manifest-hash, and eight-file-set check | 0 | Both corrected B tables and all seven manifest entries reconcile. |
| SHA-256 of expected and manifest | 0 | Hashes above. |
| `git diff --check` | 0 | No whitespace errors. |

I did not run Validator or production scoring, alter any input fixture/hash,
add a script/case/framework, or edit historical response003.
