# T031 two-episode oracle recheck

## Verdict

**Revise.** The refrozen two-episode bundle satisfies the literal-minimum case
shape and every run-level oracle value reconciles, but the repaired oracle still
misclassifies label `B` for episode `001` in both comparison transition tables.
The required repair is four count-cell changes plus the resulting
`expected.json` hash update in `manifest.json`.

I remained read-only except for this required response. I did not run Validator,
production scoring, or any maintained oracle; I did not read private T029/T030
artifacts or mutate Fizzy.

## Candidate identities

The repaired author handoff SHA-256 is
`e2798adb1d86860a2943ba539e9385b7a4e6470466ba487902355108ee295e52`,
exactly as dispatched.

The eight fixture files and their current SHA-256 identities are:

| File | SHA-256 |
|---|---|
| `golden.json` | `36a22398a40ed88cc81112bfcf009c6cf3c0cbfa0d041278e048ea634975d885` |
| `baseline.json` | `495b8aadd0d520a1b602c2692ab2cb92635ed2b6c2e017bf24c04c0c55069507` |
| `candidate.json` | `07ea4b27c21b0c2c99971cd0e4789d7f64ebf2f20da43fe125d91b4c427abfe9` |
| `config.json` | `97ddda3dda9c64fb81df21c2d7edb754ed8c37561bad127036933b34a210b80a` |
| `targeted-config.json` | `87d1cd1b18e2044b286d93fa3baed8be84c26211c666e4c1cbfa9bfd05f334b3` |
| `targeted-predictions.json` | `13270771769a6c523b59a789980d044107ae6c1dc085ff0b8c8b207ee2d3e8e3` |
| `expected.json` | `0737c59d45b371c46d38cda71696bc16d8df49125ca72f96238548c4eb47ff98` |
| `manifest.json` | `a94ca089fe713809a98b94cc8823fe52b8d34ea650e7eb4c7c82cfc32f62b7c7` |

The manifest's seven non-self entries exactly match the first seven identities.
It excludes itself, records six inputs before the oracle, and contains no
generated run-output entry. The directory contains exactly these eight JSON
files and no run, report, comparison, receipt, evidence, or artifact output.

## Exact finding

Episode `001` has:

- reference `{A,B}`;
- baseline final outcome `{A,C}`; and
- candidate final outcome `{A,B}`.

Transition states come from the two final outcomes, not from the reference.
Therefore label `B` is **absent in the baseline and present in the candidate**.

At `tests/fixtures/acceptance-multi/expected.json:39`, change the full comparison
label-`B` cells:

- `absent_to_present`: `0` to `1`;
- `present_to_present`: `1` to `0`.

The full table then represents `001` as `absent_to_present` and `002` as
`absent_to_abstained`, and still sums to the two-episode comparison population.

At `tests/fixtures/acceptance-multi/expected.json:40`, change the targeted
intersection label-`B` cells:

- `absent_to_present`: `0` to `1`;
- `present_to_present`: `1` to `0`.

That table then represents its only compared episode and still sums to one.
After these four changes, recompute the `expected.json` SHA-256 and replace the
entry at `tests/fixtures/acceptance-multi/manifest.json:22`. The six frozen input
hashes do not change. The frozen response003 should remain historical evidence;
its transition narrative is superseded by this finding rather than edited.

## Reconciliation that passed

The bundle has exactly two episodes and the required independent roles:

- `001` is the overlapping known failure and recovery: expected `{A,B}`,
  baseline `{A,C}`, candidate `{A,B}`, with matched `{A}`, missed `{B}`, and
  extra `{C}` on the baseline.
- `002` is the answered-empty-set and whole-abstention regression: expected `{}`,
  baseline answered `{}`, candidate abstention with complete `A/B/C` marginals.

Both full runs select the same two episodes from the same golden file and ordered
vocabulary. The targeted candidate selects only `001`; the intersection compares
only `001`, excludes `002` from the baseline, and excludes nothing from the
targeted candidate. This is a strict nonempty partial overlap and is the literal
minimum because a proper nonempty subset requires at least two full-population
episodes.

Fresh set and rational arithmetic reconciled every raw and final run entry under
`as_recorded`:

- Baseline full: `N=2`, `G=2`, `U=0`, `D=1`, `E=1`; aggregate
  `TP=1`, `FP=1`, `FN=1`, `TN=3`; exact match `1/2`, micro-F1 `1/2`,
  macro-F1 `1/3`, Hamming `1/3`.
- Candidate full: `N=2`, `G=1`, `U=1`, `D=1`, `E=0`; aggregate
  `TP=2`, `FP=0`, `FN=0`, `TN=1`; exact match `1/2`, coverage `1/2`,
  selective exact match and micro-F1 `1`, macro-F1 `2/3` with `C` undefined,
  Hamming `0`.
- Targeted candidate: `N=G=D=1`, `U=E=0`; aggregate `TP=2`, `TN=1`;
  exact match and micro-F1 `1`, macro-F1 `2/3` with `C` undefined, Hamming `0`.
- Baseline on the targeted intersection: `N=G=E=1`, `U=D=0`; aggregate
  `TP=FP=FN=1`, `TN=0`; exact match `0`, micro-F1 `1/2`, macro-F1 `1/3`,
  Hamming `2/3`.

For all four run entries, every selected/answered ID, total, per-label support,
`TP/FP/FN/TN`, precision, recall, F1, undefined-class status, exact-set metric,
micro metric, macro metric, Hamming value, and full-run episode record matches
the independent derivation.

Every selected marginal assigns `4/5` to the observed binary outcome, including
the submitted abstention. Each per-label and mean Brier is exactly `1/25`; each
finite per-label and mean log loss is analytically `-ln(4/5)`. The full runs have
six label decisions each, and the targeted and recomputed-intersection runs have
three each.

The full comparison populations, metric deltas, correctness categories, and
changed IDs reconcile: recovered `001`, regressed `002`, both-correct and
neither-correct empty, and both outcomes changed. The targeted comparison has
only recovered/changed `001`. All `A` and `C` transition cells reconcile. Every
current transition table sums to its population; that invariant alone does not
detect the label-`B` state substitution described above.

No stale four-case content remains in the fixture bundle: IDs `003` and `004`,
the superseded golden and expected hashes, selected population `4`, and
12-label-decision probability populations are absent.

## Executed checks

| Check | Exit | Result |
|---|---:|---|
| SHA-256 over response003 and all eight fixture files | 0 | Frozen handoff and all candidate identities matched. |
| Published JSON Schema validation over the golden, three prediction, and two configuration inputs | 0 | All six inputs accepted. |
| Temporary Ruby exact set/rational recomputation | 1 | All run/probability/oracle checks passed; both comparisons differed only at the four label-`B` cells cited above. |
| Exact stale-value search over the fixture directory | 0 | No superseded IDs, hashes, four-episode population, or 12-decision population found. |
| `git diff --check` | 0 | No whitespace errors before saving this response. |

## Re-review boundary

After the four cell corrections, recheck those cells, the two label-`B`
transition sums, the new `expected.json` digest, and the manifest entry. No
broader re-audit is needed unless an input fixture changes.
