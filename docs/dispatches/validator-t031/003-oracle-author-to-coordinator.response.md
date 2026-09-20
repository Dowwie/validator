# T031 literal-minimum two-episode oracle repair handoff

## Status

Ready for a fresh independent re-review. This repair refroze the public bundle to
the verifier's literal-minimum two-episode design and stops before any Validator
evaluation.

## Scope and independence

I read verdict002 with SHA-256
`80dc03b2ea78b223ee26be861358b5abc88f7848c857f0803636f98b3a894b92`, the
T031 contract, the normative multi-label data/metric/practical-case sections,
and the published input schemas. I changed only the six input/config/prediction
fixtures, the hand-derived expected oracle, its manifest, and the required
artifact-index entry for this handoff. I did not read
production scoring, Validator results, private acceptance artifacts, or earlier
production-derived expected values; I did not invoke Validator or add a script,
case, or generated run output.

The frozen case is exactly:

1. `001`: expected `{A,B}`, baseline `{A,C}`, candidate `{A,B}`. It is the
   inspectable overlapping known failure and candidate recovery.
2. `002`: expected `{}`, baseline `{}`, candidate whole abstention with complete
   marginals. It supplies the answered empty set and candidate regression.

Both full runs select both episodes and are compatible. The targeted candidate
selects only `001`, a strict nonempty subset. Its intersection recomputes both
runs on `001` and excludes `002`.

## Freeze sequence and hashes

The six input/config/prediction bytes were written and frozen before the oracle
calculation:

| File | SHA-256 |
|---|---|
| `golden.json` | `36a22398a40ed88cc81112bfcf009c6cf3c0cbfa0d041278e048ea634975d885` |
| `baseline.json` | `495b8aadd0d520a1b602c2692ab2cb92635ed2b6c2e017bf24c04c0c55069507` |
| `candidate.json` | `07ea4b27c21b0c2c99971cd0e4789d7f64ebf2f20da43fe125d91b4c427abfe9` |
| `config.json` | `97ddda3dda9c64fb81df21c2d7edb754ed8c37561bad127036933b34a210b80a` |
| `targeted-config.json` | `87d1cd1b18e2044b286d93fa3baed8be84c26211c666e4c1cbfa9bfd05f334b3` |
| `targeted-predictions.json` | `13270771769a6c523b59a789980d044107ae6c1dc085ff0b8c8b207ee2d3e8e3` |

Only after that freeze I derived and wrote `expected.json`:
`0737c59d45b371c46d38cda71696bc16d8df49125ca72f96238548c4eb47ff98`.
`manifest.json` was written last and records exactly those six input hashes plus
the expected-oracle hash, with no self-hash.

## Independent derivation

`as_recorded` makes raw and final results identical in every run. In the full
baseline, `N=2`, `G=2`, `U=0`, `D=1`, and `E=1`; aggregate answered binary
counts are `TP=1`, `FP=1`, `FN=1`, and `TN=3`. Its exact-match accuracy,
micro-F1, macro-F1, and Hamming loss are respectively `1/2`, `1/2`, `1/3`, and
`1/3`. Its per-label counts are A `(1,0,0,1)`, B `(0,0,1,1)`, and C
`(0,1,0,1)` in TP/FP/FN/TN order.

In the full candidate, `N=2`, `G=1`, `U=1`, `D=1`, and `E=0`; the aggregate
counts are `TP=2`, `FP=0`, `FN=0`, and `TN=1`. Exact-match accuracy is `1/2`,
coverage is `1/2`, selective exact-match accuracy and answered micro-F1 are `1`,
macro-F1 is `2/3` with `contains_undefined_classes` for C, and Hamming loss is
`0`. A and B each have `(1,0,0,0)` and C has `(0,0,0,1)`.

The targeted candidate on `001` has `N=G=D=1`, `U=E=0`, counts
`TP=2,FP=FN=0,TN=1`, exact-match accuracy/micro-F1 `1`, macro-F1 `2/3` with C
undefined, and Hamming `0`. The baseline recomputed on that same intersection
has `N=G=E=1`, `U=D=0`, counts `TP=FP=FN=1,TN=0`, exact-match accuracy `0`,
micro-F1 `1/2`, macro-F1 `1/3`, and Hamming `2/3`.

Every complete marginal gives `4/5` to the observed binary outcome, including
the submitted abstention. Thus every per-label and mean Brier is exactly `1/25`
and every finite per-label and mean binary log loss is the analytic
`-ln(4/5)`. The full runs have six probability label decisions each; the
targeted and recomputed-intersection runs have three each.

For the full comparison, `001` is recovered, `002` is regressed, neither is
both-correct or neither-correct, and both final outcomes changed. For labels A
and B, the final-state transitions are one `present_to_present` (`001`) and one
`absent_to_abstained` (`002`). For C, they are one `present_to_absent` and one
`absent_to_abstained`. Each table sums to two. The targeted intersection has
only recovered `001`; A and B are `present_to_present`, C is
`present_to_absent`, and each table sums to one.

The retained known failure is baseline `001`: expected `{A,B}`, recorded
`{A,C}`, matched `{A}`, missed `{B}`, and extra `{C}`.

## Checks

| Check | Exit | Result |
|---|---:|---|
| Ruby JSON parse of all eight fixture files | 0 | Valid UTF-8 JSON. |
| Temporary Ruby schema-shape, SHA-256, hard-count, probability-population, and transition reconciliation | 0 | All assertions passed. |
| `git diff --check` | 0 | No whitespace errors. |

The temporary arithmetic check rederived selected/answered populations,
per-label TP/FP/FN/TN and support identities for raw/final families, all seven
manifest hashes, complete marginal populations, full and targeted correctness
categories, and all per-label transition-table sums. It did not call Validator
or production scoring.

## Next action

A fresh reviewer should reconcile the six frozen inputs, the seven manifest
hashes, every raw/final metric and status, probability identities, and the two
comparison transition tables before any Validator evaluation.
