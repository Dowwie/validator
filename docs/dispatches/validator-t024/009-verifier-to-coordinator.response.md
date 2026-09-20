# T024 focused repaired-candidate verdict

## Verdict: Ready

The exact manifest008 candidate resolves the single schema-completeness finding
from verdict006. Every reproduced admission now rejects under pinned Draft
2020-12 tooling, the legal neighboring single-label and multi-label documents
remain valid, and no repair-caused T024 defect was found. This verdict is limited
to the authorized comparison/report/receipt repair and focused conformance delta.

## Reconciliation

Manifest008 hashes to
`91d756bbb4253a832461c2e3642a585c00286caf471b3776abf5af8325b5e0d3`.
Its repair chain matches exactly:

| Artifact | SHA-256 |
|---|---|
| `005-coordinator-candidate-manifest.md` | `502f5f7e4458b11ebdb46ec0a271b3ffe8eb76dd329e2ba7c123a5ccfbbce82c` |
| `006-verifier-to-coordinator.response.md` | `cf10fef91f3c58af5b9b8e924a53d099143faf3a060cc820e30a6c874d77db52` |
| `007-coordinator-to-repair-developer.prompt.md` | `e9b64a8b14fa98f757d9e0fe4fbba126e322718cb2d825b0deede3caffa15f73` |
| `007-repair-developer-to-coordinator.response.md` | `cfdf90c88835fbea0e038aa53d51fa21fe808686dc06ecb2fc173c4e26d2983e` |

The repaired files also match manifest008:

| Artifact | SHA-256 |
|---|---|
| `schemas/v2/comparison.schema.json` | `4f2dd6886056d335c3d4209c3a47dbfab2685a8424b4b8153d8d02fd77f9a104` |
| `schemas/v2/report.schema.json` | `aa3589ea93b9a6a0e7c519898988087acb87f6854dc72db11bd974d99cbb0da3` |
| `schemas/v2/receipt.schema.json` | `6f20777251015af381ad321030490c9c28996a165a4528e8b739d40227cc9637` |
| `tests/conformance.rs` | `6ad3652d62a447e6a0e32ab3ab69633c2b66041b6e0a3d15a52c76376e9692b6` |
| `tests/cli.rs` (unchanged) | `8c7b9c00d777f4a290911dd77199d8e1b9e085cc14fb0dd78a801108c3a10fcf` |

I also reconciled every unchanged manifest005 file and governing identity. No
candidate, governance, tracking, fixture, production, dependency, or later-task
file changed during this recheck.

## Focused repair judgment

- Comparison side source maps now require nonblank names and closed source
  definitions; source-count values and all named single/multi hard and probability
  counts are nonnegative integers. Single classes, macros and transition rows are
  closed typed records. Multi-label per-episode matched/missed/extra values are
  unique nonblank-label arrays or null. Generated current single-label and
  multi-label comparisons remain valid.
- Scalar report/comparison metrics now admit only `selected` or `answered` scope.
  The explicit signal-bin definition still admits `raw_answered`; a pinned legal
  neighbor using that bin scope validated.
- Report and comparison source/preparation configuration values are objects whose
  members remain arbitrary JSON. A focused pinned probe validated member null,
  exact integer `9007199254740993`, literal `1e400`, and a literal
  `$serde_json::private::Number`-shaped object. The same probe confirmed golden
  `input` remains arbitrary rather than inheriting the object restriction.
- Report artifacts now discriminate exact `golden.json`, `predictions.json`, and
  `config.json` snapshot paths plus `evidence/<ordinal>.bin`; absolute and parent-
  traversal mutations reject while generated legal artifacts validate.
- Receipt branches remain absolute and now discriminate `report.json` from
  `comparison.json`. Both legal branches validate; the `not-report.json` and
  `not-comparison.json` mutants reject.
- Each verdict006 mutant is applied independently in `all_schema_contracts`, so
  rejection is not caused by an unrelated simultaneous defect. Current legal
  reports, both task-specific comparisons, legal opaque positions, receipts and
  the real-binary output matrix are the adjacent positive controls.

The repaired definitions preserve task discrimination and do not alter the
accepted zero-exclusion intersection behavior. Those surrounding tests and all
fourteen assigned S/M/E rows are unchanged except for the focused schema
assertions, and the developer's complete 36-conformance evidence passed them.

## Commands and reused gates

| Command | Exit | Result |
|---|---:|---|
| `cargo test --locked --test conformance all_schema_contracts -- --nocapture` | 0 | 1 passed, 35 filtered; all verdict006 admissions reject beside legal single/multi neighbors. |
| `cargo test --locked --test cli cli_stdout_schema_matrix -- --nocapture` | 0 | 1 passed, 9 filtered; real check/evaluate/inspect/compare, report/comparison files and stable errors remain single-document/schema-valid. |
| Existing external pinned legal-neighbor probe | 0 | 1 passed, 36 filtered; opaque object members, arbitrary golden input, bin-only `raw_answered`, comparison source configuration, and both receipt paths validate. |

Per prompt009, I did not create a fresh target or repeat the broad suite/release.
The repair developer's passing 36 conformance, 10 CLI, 44+36+10 locked full suite,
format, release, diff, and exact accepted 17 production plus 3 duplicate lib-test
Clippy evidence is consistent with the reconciled hashes and is reused. No focused
failure justified reopening those gates.

T024's sole authorized finding is resolved. T025 and later remain outside this
focused verdict.
