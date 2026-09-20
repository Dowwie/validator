# Implement T025 output 3: M01-M15/M20, full numerical umbrella and final gates

Role/model: fresh sole oracle/test developer, `gpt-5.6-terra`, high reasoning,
`fork_turns: none`. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #187 — Verify Validator numerical conformance](http://localhost:3006/1/cards/187).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t025/008-numeric-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat return only the
path, SHA-256 and terse status. Do not edit governance, plans, artifact-index,
session notes, acceptance records or Fizzy. Do not delegate.

This is T025 local output 3 of 3. Read global/repository AGENTS, T025, owner
prompt001, complete output-1 response006, complete output-2 response007, the
ratified numerical/verification sections and assigned coverage rows M01-M15/M20.
Preserve all accepted tests, fixtures, repairs and assertions. Do not begin the
independent review, T026 or later work.

## Independence and write boundary

Write only `tests/conformance.rs` and, only if a fixed expected value cannot
reasonably stay clear inline, the existing task-owned expected fixtures under
`tests/fixtures/single-label/` or `tests/fixtures/multi-label/`. A new fixture is
allowed only when necessary and must be named in the response for coordinator
indexing. Production and schemas are read-only.

Derive every expectation directly from the ratified formulas and raw synthetic
cases. Do **not** read/copy `src/evaluation*`, production metric/count helpers or
production scoring implementation to obtain expected values. Do not invoke
production to generate expectations or tune expectations/tolerance to observed
output. You may read public API/schema contracts and existing conformance setup
and helpers, then call the real public application after fixing the independent
expectation. Use exact integer/rational checks where representable and only
`validator::FIXTURE_ABSOLUTE_TOLERANCE` for required floating comparisons.

If a literal public result violates an independently derived expectation, save
the exact input, derivation, expected result, actual result and owning location,
freeze the mismatch, and stop. Do not edit production/schema or weaken evidence.

Starting identities:

| Artifact | SHA-256 |
|---|---|
| `docs/dispatches/validator-t025/001-owner-to-coordinator.prompt.md` | `4c6409c55242078377ff37b87343a41b9b997267d3fbf47dac3d9c889d2f0ab0` |
| `docs/dispatches/validator-t025/006-oracle-developer-to-coordinator.response.md` | `a328aeab9ad0f7a529fb77ca0ed494b43982601696ea3d7327dc80026ddfb741` |
| `docs/dispatches/validator-t025/007-case-developer-to-coordinator.response.md` | `7bf593b4e3d7ffe783e0ac8cb77d908c461a2b776e740e084834a7564d8b6bed` |
| `docs/specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `docs/specs/validator-data-model.md` | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| `docs/plans/validator/tasks/T025.json` | `c29765ace2e7859665e290a5a7afad8b71d312ae3ff2ef9c4bb01d6aebdb4de6` |
| `tests/conformance.rs` | `71f81666afd7792958f57a3cc9b75d86f936b45d073b411976884ce3676e8f2b` |
| `tests/fixtures/single-label/expected.json` | `ab93c379b9c0df1d0c0fb7aac7a6df5e7748f3f671d55dd69d173e640090b268` |
| `tests/fixtures/multi-label/expected.json` | `213183a46c0f6c011296ef5087b5c44d3852081bdf20eaa80cbb494d3d2c84f2` |
| `src/model/multi_label.rs` | `3f515a0a9122db963b2e01ca14d5474b4836ba3543a48a4e0766a4459eea0b7c` |
| `schemas/v2/report.schema.json` | `805de93d2a50339370e803b7ff1a9a1b8d34c3028b5b21a1a6f428f591733f90` |
| `schemas/v2/inspection.schema.json` | `2c7e064d0adb915b77c558a7ebbfade96f7bcf0eee890b881ced1383f0eedaef` |

## Exact named multi-label evidence

Add one listed and executing nonzero test per exact filter. Each test must call the
real public application/error path and assert the whole row, including statuses,
counts, scopes, units, ratio operands and ordered episode evidence that the row
makes material. Reuse small test-only setup helpers; do not duplicate setup or
build a generic oracle framework.

- `case_m01`: vocabulary `[A,B,C]`, expected `[{A,B},{}]`, predicted
  `[{A,C},{}]`. Assert `N=G=2,D=1,E=1,U=0`, exact-match `1/2`, aggregate
  `TP=1,FP=1,FN=1,TN=3`, answered micro-F1 wire operands `2/4` (value `1/2`),
  macro-F1 `1/3`, Hamming operands `2/6` (value `1/3`), every per-label four-count
  sum `G`, total binary counts `G*K`, and vocabulary-ordered episode differences.
- `case_m02`: expected and predicted empty sets with nonempty vocabulary. Assert
  answered exact match, coverage one, Hamming zero, per-label and micro-F1 null at
  zero denominator, and macro-F1 zero with the exact undefined-class status/list.
- `case_m03`: expected `[{A},{}]`, predicted `[abstention,{}]`. Assert
  `N=2,G=1,U=1,D=1,E=0`, exact match `1/2`, selective exact match one, coverage
  `1/2`; label A total support one/answered support zero/TN one, and no binary
  decision or set differences fabricated for the abstained row.
- `case_m04`: every selected multi-label row explicitly abstains. Assert coverage
  zero, exact-match accuracy zero for the nonempty selection, answered metrics
  null `no_answered_predictions`, `status: "abstained"` with null set differences,
  and no fabricated empty sets or binary counts.
- `case_m05`: reference `{A}`, complete marginals A=`0.8`, B=`0.7`. Assert values
  remain unchanged although they sum to `1.5`; independently derive finite mean
  binary log loss `(-ln(0.8)-ln(0.3))/2`, mean binary Brier `0.265`, per-label
  results and selected label-decision populations.
- `case_m06`: same row with raw set `{A,B}` and thresholds A=`0.8`, B=`0.75`.
  Assert equality accepts A, final set is `{A}`, raw/final separation, and raw
  metrics plus probability losses/populations remain unchanged by policy.
- `case_m07`: all marginals below their label thresholds. Assert final outcome is
  the answered empty set, not abstention, with final hard metrics based on that set.
- `case_m08`: reference empty set and all marginals zero. Assert exact zero
  per-label/mean binary losses, defined finite status, and no `0*ln(0)`/invalid JSON.
- `case_m09`: cover both observed-branch endpoints: reference contains A with
  marginal A=`0`, and reference excludes A with marginal A=`1`. Assert per-label
  and mean binary log loss null `positive_infinity`/`+infinity` without clipping.
- `case_m10`: construct exact boundary cases at `0`, `0.1` and `1` for per-label
  probability bins. Assert the declared bin assignment including one in the last
  bin and that positive rates follow reference presence rather than prediction
  correctness.
- `case_m11`: independently compare categorical `[0.8,0.2]` with true A against a
  one-label marginal A=`0.8` with true-present. Assert categorical Brier `0.08`
  and binary marginal Brier `0.04`, retaining their distinct metric names/scaling.
- `case_m12`: two reference rows both `{A,B}`. Compare predictions
  `[{}, {A,B}]` against `[{A},{B}]`. Assert identical per-label confusion counts,
  exact-match accuracy `1/2` versus zero, and distinct ordered episode difference
  evidence so aggregate equality cannot hide set-level behavior.
- `case_m13`: separately reject duplicate label in target, duplicate label in
  prediction, unknown label and partial marginals before set conversion/filtering,
  including when thresholding would otherwise remove the invalid member. Assert
  typed no-report errors and no partial scoring.
- `case_m14`: separately reject label-set target in a single-label task, class
  target in a multi-label task, wrong probability kind and wrong policy kind.
  Assert typed schema/configuration errors with no inference or coercion.
- `case_m15`: separately reject a `scored_choice` source and top-level reported
  confidence in a multi-label artifact. Assert the supported-task/source boundary
  and no fallback interpretation.
- `case_m20`: permute multi-label target/prediction set order, episode order and
  JSON marginal-map key order. Assert identical scores/counts and vocabulary-ordered
  evidence while allowing exact input-byte digests to differ; never assert digest
  equality across changed bytes.

## Full numerical umbrella

Add listed, executing `full_numeric_conformance`. It must exercise the independent
exhaustive oracles, F04, every S01-S12, E01-E04/E07 and M01-M15/M20 scenario, and
therefore cover all required ratios/populations/statuses, infinities, binary64
boundaries, ties, raw/final policy separation and permutation invariance. Reuse the
same test-only case functions/derivations rather than copying decision logic. The
umbrella does not replace any named filter; all exact filters must still list and
execute independently.

## Required final T025 boundary

Run Cargo serially. First run list confirmation and each new exact M filter plus
the umbrella independently. Then run the complete exact T025 inventory, including
the two exhaustive filters and every S/E/M/umbrella filter. A short shell loop is
acceptable only to invoke each literal filter separately and preserve its exit;
no broad substring may stand in for exact filters.

Finish with:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
git diff --check
```

Clippy is expected to exit nonzero only for the accepted exact staged inventory of
17 production diagnostics plus 3 duplicate lib-test diagnostics. Record every
diagnostic/symbol and confirm no new warning; do not suppress warnings, invent fake
uses, expand visibility or perform cleanup-only edits. Formatting, full locked
tests, release build and diff check must pass. If any other diagnostic or gate
failure appears, correct only your owned test/fixture work or freeze a demonstrated
out-of-scope discrepancy.

Response008 must map every M row/subcase and the umbrella to concrete independent
assertions; record exact commands/exits/counts, full filter inventory, staged
Clippy comparison, exact changed hashes and unchanged accepted fixture/repair
hashes. Report missing evidence honestly. Return only when output 3 and the final
T025 developer boundary are complete, or a frozen literal discrepancy is saved.
