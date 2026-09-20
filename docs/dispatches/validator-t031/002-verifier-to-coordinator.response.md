# T031 independent multi-label oracle review

## Verdict

**Revise.** The frozen inputs and all run-level arithmetic reconcile, but
`expected.json` has four incorrect label-`B` transition cells. The four-episode
bundle also is not the absolute smallest bundle that can satisfy the features
named in the dispatch.

I did not run Validator, read production scoring, change any fixture, or mutate
Fizzy. I used only read-only JSON Schema checks, SHA-256 checks, and temporary
set/rational arithmetic over the frozen public inputs.

## Required oracle correction

The comparison transition state is the label's state in each final outcome,
independent of the reference. In the full comparison:

- `001`: baseline `B` absent, candidate `B` present.
- `002`: absent to absent.
- `003`: abstained to absent.
- `004`: absent to present.

Therefore
`comparisons.full_default.transitions.per_label.B.absent_to_present` must be `2`
and `present_to_present` must be `0`, rather than `1` and `1` at
`tests/fixtures/acceptance-multi/expected.json:158`.

The targeted intersection contains `001` and `002`. Therefore
`comparisons.targeted_intersection.transitions.per_label.B.absent_to_present`
must be `1` and `present_to_present` must be `0`, rather than `0` and `1` at
`tests/fixtures/acceptance-multi/expected.json:180`.

The smallest arithmetic correction is those four count changes followed by a
new SHA-256 for `expected.json` in
`tests/fixtures/acceptance-multi/manifest.json:22`. The six frozen input hashes
do not change. Every label transition table must still sum to its comparison
population after correction.

## Reconciliation that passed

The six frozen input hashes exactly match the handoff and manifest:

- `golden.json`: `5633686e670e90401d526ca2254ec419f3d3ebaa9a1e489f97e28fb3f66db122`
- `baseline.json`: `e10d90a218d646a5045ff87cf389f9535a01dfb45567f98b0fec8561e73a81e3`
- `candidate.json`: `7be74f697b326a1316e9f0eaa83dd22f5d2d0172ac4f44df30483838f11f80af`
- `config.json`: `0576bf41732e305ae8b216b2b2dc258db5b96e35deaa7d96ec1f526964bebcc4`
- `targeted-config.json`: `86e42ef9259e950fea3212f0e2f9a7e00d87bb395c7a142add9171afa4b76a37`
- `targeted-predictions.json`: `99544d40730e78850e1ebe17eeadd8eb42edafcd17e2c80c1fe455194debc246`

The current `expected.json` hash is
`36561e09a3551c5020e8ab2a80d2457c9ce0bb6443322b1118d6a7e1e15198f8`,
which also matches the handoff and manifest before the required correction.
`manifest.json` has exactly the six input entries plus `expected.json`, excludes
itself, and records the correct freeze order. The fixture directory contains only
the eight intended JSON files; it contains no generated run, report, comparison,
receipt, artifact, or evidence output.

The published schemas accept the golden file, all three prediction files, and
both configuration files. The following independently derived values match
`expected.json` exactly for both raw and final families under `as_recorded`:

- Baseline full: `N=4`, `G=3`, `U=1`, `D=2`, `E=1`; selected IDs are all four
  IDs and answered IDs are `001`, `002`, and `004`.
- Candidate full: `N=4`, `G=4`, `U=0`, `D=3`, `E=1`; all four IDs are answered.
- Targeted candidate: `N=G=D=2`, `U=E=0`, over `001` and `002`.
- Baseline recomputed on that intersection: `N=G=2`, `D=1`, `E=1`, `U=0`,
  over `001` and `002`.
- Every per-label `TP/FP/FN/TN`, support, answered support, predicted support,
  precision, recall, F1, undefined status, exact-set metric, micro metric,
  macro-F1, and Hamming value in all four run entries reconciles.
- Every selected marginal assigns `4/5` to the observed binary outcome. Thus each
  per-label and mean Brier is exactly `1/25`, each finite per-label and mean log
  loss is `-ln(4/5)`, and the probability populations are respectively `12`,
  `12`, `6`, and `6` label decisions.
- Full comparison correctness is: both correct `002`; recovered `001`,`003`;
  regressed `004`; neither correct empty; changed outcomes `001`,`003`,`004`.
  Its answered populations, overlap, all metric deltas, and every `A` and `C`
  transition cell reconcile.
- Targeted comparison correctness is: both correct `002`; recovered `001`;
  regressed and neither correct empty; changed outcome `001`. Its exclusions,
  answered populations, overlap, all metric deltas, and every `A` and `C`
  transition cell reconcile.
- The known failure `001` is correctly located: expected `{A,B}`, baseline
  recorded `{A,C}`, matched `{A}`, missed `{B}`, and extra `{C}`.

## Minimality finding

The four episodes are cleanly separated and collectively cover the requested
features, but four is not the literal minimum under the dispatch's stated
constraints. Two episodes suffice:

1. Expected `{A,B}`; baseline `{A,C}`; candidate `{A,B}` gives the overlapping
   known failure and a recovery.
2. Expected `{}`; baseline `{}`; candidate whole abstention with complete
   marginals gives an answered empty set and a regression.

The full selections remain compatible, and a targeted candidate selection of
episode 1 is a nonempty strict subset whose intersection passes. Thus the current
four-case design can be retained as a deliberately separated minimal-readable
case, but it cannot be described as the absolute smallest case without adding an
explicit constraint that each named behavior occupy its own episode. If literal
minimum size is mandatory, the fixtures must be refrozen and the oracle rederived;
that is a larger revision than the four transition-cell correction.

## Re-review boundary

After correction, recheck the four label-`B` cells, all per-label transition sums,
the new `expected.json` hash, and the manifest entry. No broader re-audit is needed
unless the fixture population is redesigned for literal minimality.
