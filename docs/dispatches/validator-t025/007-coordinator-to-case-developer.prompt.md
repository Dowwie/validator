# Implement T025 output 2: exact S01-S12 and E01-E04/E07 cases

Role/model: fresh sole oracle/test developer, `gpt-5.6-terra`, high reasoning,
`fork_turns: none`. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #187 — Verify Validator numerical conformance](http://localhost:3006/1/cards/187).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t025/007-case-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat return only the
path, SHA-256 and terse status. Do not edit governance, plans, artifact-index,
session notes, acceptance records or Fizzy. Do not delegate.

This is T025 local output 2 of 3. Read global/repository AGENTS, T025, owner
prompt001, response006, the complete ratified numerical/verification sections,
and assigned coverage rows S01-S12 and E01-E04/E07. Preserve all accepted tests,
fixtures, repairs and assertions. Do not begin M01-M15/M20,
`full_numeric_conformance`, final gates, independent review, T026 or later work.

## Independence and write boundary

Write only `tests/conformance.rs` and, only if a fixed expected value cannot
reasonably stay clear inline, the existing task-owned expected fixtures under
`tests/fixtures/single-label/` or `tests/fixtures/multi-label/`. A new fixture is
allowed only when necessary and must be named in the response for coordinator
indexing. Production and schemas are read-only.

Derive every expected result from the ratified formulas and raw synthetic cases.
Do **not** read/copy `src/evaluation*`, production metric/count helpers, or
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
| `docs/specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `docs/specs/validator-data-model.md` | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| `docs/plans/validator/tasks/T025.json` | `c29765ace2e7859665e290a5a7afad8b71d312ae3ff2ef9c4bb01d6aebdb4de6` |
| `tests/conformance.rs` | `6295d99c7124944608556b2ce9f1197af37acab5f8c9c9590cc4a111882ef825` |
| `tests/fixtures/single-label/expected.json` | `ab93c379b9c0df1d0c0fb7aac7a6df5e7748f3f671d55dd69d173e640090b268` |
| `tests/fixtures/multi-label/expected.json` | `213183a46c0f6c011296ef5087b5c44d3852081bdf20eaa80cbb494d3d2c84f2` |
| `src/model/multi_label.rs` | `3f515a0a9122db963b2e01ca14d5474b4836ba3543a48a4e0766a4459eea0b7c` |
| `schemas/v2/report.schema.json` | `805de93d2a50339370e803b7ff1a9a1b8d34c3028b5b21a1a6f428f591733f90` |
| `schemas/v2/inspection.schema.json` | `2c7e064d0adb915b77c558a7ebbfade96f7bcf0eee890b881ced1383f0eedaef` |

## Exact named evidence

Add one listed and executing nonzero test for each exact filter below. Each test
must call the real public application/error path and assert the whole stated row,
including statuses, counts, scopes, units, operands and episode evidence that the
row makes material. Reuse small test-only setup helpers; do not duplicate setup or
build a generic oracle framework.

- `case_s01`: three classes, one correct prediction each. Assert the identity
  K×(K+1) matrix, D/N/G/support/predicted support, no abstentions, accuracy and
  fixed macro-F1 one with correct ratio/population metadata.
- `case_s02`: actual A, predicted B. Assert the single off-diagonal cell, A FN=1,
  B FP=1, D=0/E=1/U=0, and exactly one wrong episode/evidence identity.
- `case_s03`: actual `[A,A,B,C]`, final `[A,abstain,A,C]`. Assert
  `D=2,E=1,U=1,N=4,G=3`, accuracy `1/2`, coverage `3/4`, selective accuracy
  `2/3`, class coverages `[1/2,1,1]`, macro-F1 value `1/2`, direct ratio operands
  and statuses/populations.
- `case_s04`: only A observed/correct in schema `[A,B,C]`. Assert accuracy one,
  B/C F1 null with `undefined_zero_denominator`, and macro-F1 `1/3` with
  `contains_undefined_classes` and the exact undefined-class list.
- `case_s05`: every selected episode explicitly abstains. Assert coverage zero,
  accuracy zero for a nonempty population, selective metrics null with
  `no_answered_predictions`, explicit U, and no missing-prediction count.
- `case_s06`: empty aligned selection. Assert zero N/D/E/U/G and applicable hard
  and probability metric `no_data` results without any success-performance claim;
  retain `not_applicable` only where the absent signal family requires it.
- `case_s07`: categorical `[0.7,0.2,0.1]`, actual A. Independently calculate and
  assert log loss `-ln(0.7)`, Brier `0.14`, argmax accuracy one, plus the exact
  population metadata and finite/defined statuses.
- `case_s08`: categorical `[1,0,0]`, actual B. Assert positive-infinity log-loss
  status/value form without clipping, Brier `2`, and valid serialized JSON.
- `case_s09`: scored choice A, categorical `[0.6,0.38,0.02]`, reported confidence
  `0.39`. At threshold `0.5`, independently exercise both configured signal
  choices: confidence rejects and maximum probability accepts. Assert raw/final
  separation, unchanged probability losses/population, and exact working signals.
- `case_s10`: scored choice B with categorical `[0.5,0.5,0]`. Assert the tied
  submitted choice is valid/retained, schema-order diagnostic argmax is A, and raw
  accuracy follows B rather than replacing it with argmax.
- `case_s11`: independently assert rejection/no successful report for every stated
  subcase: untied non-argmax scored choice; missing probability key; extra
  probability key; invalid categorical sum. Check the typed input/probability error
  category and no partial scoring.
- `case_s12`: use exact adjacent binary64 values immediately below/equal/above a
  configured threshold and every relevant bin boundary, including zero and one.
  Assert equality semantics, declared bin assignment and signal `1` in the last
  bin. Avoid decimal approximations that are not the adjacent representable values.
- `case_e01`: reference A with submitted abstention, categorical
  `[0.7,0.2,0.1]`, confidence `0.8`. Assert raw accuracy/coverage zero,
  `-ln(0.7)`, Brier `0.14`, argmax accuracy one, null chosen probability, and
  confidence bins `no_answered_predictions` with the exact excluded episode ID.
- `case_e02`: two raw rows, one answered and one explicitly abstained, both with
  confidence under `as_recorded`. Assert bins count only the answered row and
  disclose exactly the abstained excluded ID.
- `case_e03`: assert both task-specific preconditions: an abstained single-label
  row carrying signals under `reject_below`, and an abstained multi-label row
  carrying marginals under `label_thresholds`, each yields the typed
  configuration/input error and never synthesizes an answer.
- `case_e04`: multi-label reference `{A}`, submitted abstention, marginals
  A=`0.8`, B=`0.7`. Assert coverage zero, no answered binary hard decisions,
  `status: "abstained"` with null set differences already established by output 1, mean
  binary Brier `0.265`, and independently calculated finite mean binary log loss
  over the full selected probability population.
- `case_e07`: submit the retained-observation vector `[0.24,0.19,0.56]` (sum
  `0.99`) as canonical scoring probabilities. Assert `E_PROBABILITY`, CLI exit
  category 2 through the public error contract, and no successful report; no
  normalization, promotion or fallback.

## Required local boundary

Run serially and record exact exits/counts:

```text
cargo test --locked --test conformance -- --list
cargo test --locked --test conformance case_s01 -- --nocapture
...
cargo test --locked --test conformance case_s12 -- --nocapture
cargo test --locked --test conformance case_e01 -- --nocapture
cargo test --locked --test conformance case_e02 -- --nocapture
cargo test --locked --test conformance case_e03 -- --nocapture
cargo test --locked --test conformance case_e04 -- --nocapture
cargo test --locked --test conformance case_e07 -- --nocapture
cargo test --locked --test conformance -- --nocapture
cargo fmt --all -- --check
git diff --check
```

The ellipsis means every exact S02-S11 filter must also be run and recorded; it is
not a combined-filter substitute. Do not rerun Clippy/release/full-project gates
at this local boundary; output 3 owns them. Response007 must map each row and every
subcase to concrete assertions, list all commands/exits/test counts, report exact
changed hashes and unchanged accepted fixture/repair hashes, and state any missing
evidence honestly. Return only when this complete output is ready or a frozen
literal discrepancy is documented.
