# T031 label-B cell-only recheck

## Verdict

**Ready.** The four label-`B` transition cells are corrected exactly as required,
both table sums match their comparison populations, the expected and manifest
identities are refreshed correctly, and all six frozen input identities remain
unchanged. Every other conclusion from verdict004 remains settled.

## Candidate identities

- Response005 SHA-256:
  `cde94b723699236d87a4b212093d585a2b0fc7bfbc4746c5f0246afa15a4d4e2`
- `expected.json` SHA-256:
  `c97e3a5b0debed6ae4dbf11ec2a67339c9009bb0953763bf56bde77ef297ca81`
- `manifest.json` SHA-256:
  `a61aa36e748e5cc0063d33594e07ef35d0c7bc00c09bd087d9fac570cc8f91b0`

The unchanged input identities are:

| File | SHA-256 |
|---|---|
| `golden.json` | `36a22398a40ed88cc81112bfcf009c6cf3c0cbfa0d041278e048ea634975d885` |
| `baseline.json` | `495b8aadd0d520a1b602c2692ab2cb92635ed2b6c2e017bf24c04c0c55069507` |
| `candidate.json` | `07ea4b27c21b0c2c99971cd0e4789d7f64ebf2f20da43fe125d91b4c427abfe9` |
| `config.json` | `97ddda3dda9c64fb81df21c2d7edb754ed8c37561bad127036933b34a210b80a` |
| `targeted-config.json` | `87d1cd1b18e2044b286d93fa3baed8be84c26211c666e4c1cbfa9bfd05f334b3` |
| `targeted-predictions.json` | `13270771769a6c523b59a789980d044107ae6c1dc085ff0b8c8b207ee2d3e8e3` |

## Cell and byte checks

At `tests/fixtures/acceptance-multi/expected.json:39`, the full label-`B` table
now has `absent_to_present=1`, `present_to_present=0`, and
`absent_to_abstained=1`; its other six cells are zero and all nine cells sum to
the full comparison population `2`.

At `tests/fixtures/acceptance-multi/expected.json:40`, the targeted label-`B`
table now has `absent_to_present=1` and `present_to_present=0`; its other seven
cells are zero and all nine cells sum to the targeted comparison population `1`.

A temporary byte-preserving probe reversed only those four corrected digits in
memory. The reconstructed file SHA-256 was
`0737c59d45b371c46d38cda71696bc16d8df49125ca72f96238548c4eb47ff98`,
the exact pre-correction identity from verdict004. This proves the expected-file
change is limited to the four authorized cells and that the other seven values
in each label-`B` table remain unchanged.

The manifest entry at
`tests/fixtures/acceptance-multi/manifest.json:22` equals the refreshed
`expected.json` hash. Its other six entries equal the frozen identities above,
and it still excludes a self-hash.

## Executed checks

| Check | Exit | Result |
|---|---:|---|
| SHA-256 of response005, expected, manifest, and six inputs | 0 | All dispatched and frozen identities matched. |
| Temporary B-cell, table-sum, byte-diff, and manifest reconciliation | 0 | Four corrected cells, sums `2`/`1`, exact cell-only prior-hash reconstruction, and all manifest entries passed. |
| `git diff --check` | 0 | No whitespace errors before saving this response. |

No schemas or broader arithmetic were rerun, and no Validator or production
scoring command was invoked, as required by the cell-only dispatch.
