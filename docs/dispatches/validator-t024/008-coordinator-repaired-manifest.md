# Repaired T024 candidate manifest

Status: frozen for the same independent verifier's single focused recheck. This
manifest inherits every unchanged artifact/hash from manifest005 and binds the
sole repair cycle to the four changed files below.

## Repair chain

| Artifact | SHA-256 |
|---|---|
| `005-coordinator-candidate-manifest.md` | `502f5f7e4458b11ebdb46ec0a271b3ffe8eb76dd329e2ba7c123a5ccfbbce82c` |
| `006-verifier-to-coordinator.response.md` | `cf10fef91f3c58af5b9b8e924a53d099143faf3a060cc820e30a6c874d77db52` |
| `007-coordinator-to-repair-developer.prompt.md` | `e9b64a8b14fa98f757d9e0fe4fbba126e322718cb2d825b0deede3caffa15f73` |
| `007-repair-developer-to-coordinator.response.md` | `cfdf90c88835fbea0e038aa53d51fa21fe808686dc06ecb2fc173c4e26d2983e` |

## Repaired files

| Artifact | Manifest005 SHA-256 | Repaired SHA-256 |
|---|---|---|
| `schemas/v2/comparison.schema.json` | `d0dd16bef3902ac1597bb6538204ffe7c13442c655b4779f9953f6fd5200c8e9` | `4f2dd6886056d335c3d4209c3a47dbfab2685a8424b4b8153d8d02fd77f9a104` |
| `schemas/v2/report.schema.json` | `5500e80464cd87b196f291634c00c073113220747f03898c66406f5106e94ea9` | `aa3589ea93b9a6a0e7c519898988087acb87f6854dc72db11bd974d99cbb0da3` |
| `schemas/v2/receipt.schema.json` | `d820751593f82ac68cbca2ed381b305383d96d388dee417abcefec03ed3d1c00` | `6f20777251015af381ad321030490c9c28996a165a4528e8b739d40227cc9637` |
| `tests/conformance.rs` | `4b44ea67d2f902e139790a9be339539abd7ee3a90f64052df7092c37b62cc9b7` | `6ad3652d62a447e6a0e32ab3ab69633c2b66041b6e0a3d15a52c76376e9692b6` |

Every other manifest005 hash remains exact, including unchanged `tests/cli.rs`
`8c7b9c00d777f4a290911dd77199d8e1b9e085cc14fb0dd78a801108c3a10fcf`.

## Repaired behavior and evidence

- Comparison non-opaque source/count/hard/class/macro/transition/probability/
  multi-label set fields are now closed and typed.
- Scalar metric scope is selected/answered only; explicit bin definitions retain
  their normative raw-answered scope.
- Report source/preparation configuration remains an opaque object with arbitrary
  member values; stored artifact paths admit only legal relative snapshot/evidence
  forms, and golden input remains arbitrary.
- Evaluation/comparison receipt paths remain absolute and end in the required
  report/comparison filename respectively.
- Every verdict006 mutant rejects one at a time beside generated legal single/multi
  reports, comparisons and receipts. Both umbrellas, all fourteen assigned rows,
  full 44+36+10 suite, format, release and diff pass; Clippy remains only the exact
  accepted 17 production plus 3 duplicate lib-test diagnostics.

The repaired candidate is read-only during focused recheck. T025 remains paused.
