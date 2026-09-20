# T031 independent multi-label oracle handoff

## Status

Ready for a fresh independent oracle review. This handoff stops before any Validator evaluation.

## Scope and independence

Created only the six public input/config/prediction fixtures, `expected.json`, and `manifest.json` in `tests/fixtures/acceptance-multi/`. I read the governing multi-label hard-decision, probability, practical-case, selection, comparison, and published input-schema contracts. I did not read production scoring code, Validator outputs, T029/T030 private artifacts or results, earlier production-derived expected fixtures, or internal numerical handoffs. I did not invoke Validator.

The four frozen episodes cover an overlapping set error (`001`), an answered empty set (`002`), a whole abstention with complete label marginals (`003`), and a candidate regression (`004`). Baseline and candidate are full compatible selections. The targeted candidate selects `001` and `002`; intersection with the full baseline recomputes on exactly those two IDs and excludes `003` and `004`.

## Freeze sequence and hashes

The inputs were written and frozen before any expected-value calculation. Their SHA-256 values are:

- `golden.json`: `5633686e670e90401d526ca2254ec419f3d3ebaa9a1e489f97e28fb3f66db122`
- `baseline.json`: `e10d90a218d646a5045ff87cf389f9535a01dfb45567f98b0fec8561e73a81e3`
- `candidate.json`: `7be74f697b326a1316e9f0eaa83dd22f5d2d0172ac4f44df30483838f11f80af`
- `config.json`: `0576bf41732e305ae8b216b2b2dc258db5b96e35deaa7d96ec1f526964bebcc4`
- `targeted-config.json`: `86e42ef9259e950fea3212f0e2f9a7e00d87bb395c7a142add9171afa4b76a37`
- `targeted-predictions.json`: `99544d40730e78850e1ebe17eeadd8eb42edafcd17e2c80c1fe455194debc246`

Only after that freeze I derived and wrote `expected.json` (`36561e09a3551c5020e8ab2a80d2457c9ce0bb6443322b1118d6a7e1e15198f8`). `manifest.json` was written last and records all seven preceding hashes without a self-hash.

## Independent derivation

`as_recorded` makes raw and final hard results equal. The full baseline has `N=4`, `G=3`, `U=1`, `D=2`, `E=1`; the candidate has `N=4`, `G=4`, `U=0`, `D=3`, `E=1`. The expected file records all per-label TP/FP/FN/TN, supports, precision/recall/F1 statuses, aggregate exact-set/micro/macro/Hamming metrics, selected and answered IDs, and raw/final per-episode set evidence.

Every marginal assigns `4/5` to its observed binary outcome, including the submitted abstention. Thus each per-label and mean Brier value is exactly `1/25`, and each finite per-label and mean log loss is the analytic expression `-ln(4/5)`. The full comparison records the recovered IDs `001` and `003`, regressed ID `004`, changed-outcome IDs `001`, `003`, `004`, and every nine-cell per-label present/absent/abstained transition count. The targeted intersection records its own recomputed baseline/candidate metrics and transitions; it is explicitly not a full-population claim.

## Commands and results

| Command | Exit | Result |
|---|---:|---|
| `ruby -rjson -e 'ARGV.each { |path| JSON.parse(File.read(path)) }'` over the six frozen inputs | 0 | All frozen inputs parse as UTF-8 JSON. |
| `ruby -rjson -e 'ARGV.each { |path| JSON.parse(File.read(path)) }'` over the six inputs plus `expected.json` | 0 | Input/oracle JSON parse succeeds. |
| `openssl dgst -sha256 -r` over the six inputs and `expected.json` | 0 | Hashes above were emitted. |

The published schemas were used for input-shape review. No installed JSON Schema CLI was available, so no schema-validator command was run. No command calculated classification scores or called Validator.

## Limitations and next action

This task intentionally contains no generated run directory, report, comparison output, or Validator confirmation. A different fresh reviewer must reconcile the frozen hashes and hand arithmetic before the acceptance agent evaluates this bundle.
