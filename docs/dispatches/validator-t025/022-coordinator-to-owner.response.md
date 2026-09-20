# T025 final coordinator handoff for owner acceptance

T025 is complete on the exact final repaired candidate and is ready for owner
acceptance. T026 and later work remain undispatched.

## Acceptance identity

| Artifact | SHA-256 |
|---|---|
| Final repaired manifest020 | `deeed4c908fa385de1cd8494a31fdcc7fb8d46646e03b32766691fe007f24fc9` |
| Final independent Ready verdict021 | `a6ca4df6161aea825f5721097a1782b7fc520d5eb3f0febff9f1edb6db9ebc43` |
| Final developer correction response017 | `7114d8862feb20091412b45d91ce1b0ea400441eddd3022a21c51fb9a89fe08f` |
| `src/evaluation/multi_label.rs` | `c4158facd454e66e66b3f1ddc0bb9976ca07fca6961bf200597ba9b31107b461` |
| `tests/conformance.rs` | `13409645624e53b8a3f2b4e5719dd7ae651527e0c4f886dcb511bfa10bf38bef` |
| `src/model/multi_label.rs` | `3f515a0a9122db963b2e01ca14d5474b4836ba3543a48a4e0766a4459eea0b7c` |
| `schemas/v2/report.schema.json` | `805de93d2a50339370e803b7ff1a9a1b8d34c3028b5b21a1a6f428f591733f90` |
| `schemas/v2/inspection.schema.json` | `2c7e064d0adb915b77c558a7ebbfade96f7bcf0eee890b881ced1383f0eedaef` |
| Single-label expected fixture | `ab93c379b9c0df1d0c0fb7aac7a6df5e7748f3f671d55dd69d173e640090b268` |
| Multi-label expected fixture | `213183a46c0f6c011296ef5087b5c44d3852081bdf20eaa80cbb494d3d2c84f2` |

## Completed evidence

- Production-independent direct oracles cover 301 single-label populations and
  64 answered multi-label subset pairs plus eight whole abstentions. F04 is
  reconstructed from raw rows with accuracy `5/8`, direct class-F1 operands
  `4/7`, `4/5`, `2/4`, and macro-F1 `131/210`.
- Every exact S01-S12, E01-E04/E07 and M01-M15/M20 filter lists and executes
  nonzero through the public path. `full_numeric_conformance` reuses all required
  cases without substituting for their exact filters. The inventory is 74 tests.
- The owner-authorized whole-abstention repair emits literal
  `status: "abstained"` and present null matched/missed/extra fields in report and
  verified inspection output while preserving a legal ordinary class named
  `ABSTAIN` and closed report/inspection schemas.
- S12 now runs 29 isolated binary64 boundary evaluations. In particular,
  `next_down(0.9) * 10` rounds to exactly 9 and maps to bin 9; every evaluation
  proves one expected bin and zero counts in the other nine.
- Direct-ratio assertions verify population count/unit/scope. M02 explicitly
  proves present zero numerator and denominator fields. M14 executes the actual
  multi-label class-target rejection.
- Multi-label per-label precision/recall/F1 now use population `G`, unit
  `label_decision`, scope `answered` via the existing `answered_label_metric`.
  Normal, undefined, all-abstained and empty raw/final cases, aggregate `G*K`,
  set-level episode metrics and a persisted report plus real verified-inspection
  path are discriminated. No formula, value, ratio, status, fixture, schema,
  tolerance, policy or probability behavior changed.

## Gates and governance

- Full locked tests pass: 44 library, 10 CLI, 74 conformance, zero documentation.
- Formatting, locked release build and diff check pass.
- Warning-denied Clippy exits 101 only for the accepted unchanged 17 production
  plus three duplicate lib-test staged diagnostics. The clean T027/T035 gates
  remain unfulfilled and unchanged; no T025 diagnostic exists.
- T025 task mapping now owns the necessary evaluator correction; physical map and
  T027 input routing are synchronized. Final mapping hashes are:
  - T025 `7bc49b83a565c7abcbfcebd8e5336903820aec00e185783ad8e449aedceead71`;
  - T027 `a53eed042464e0e0ecb263747f53c2933f508701e10b679a9cc4f685c0a308b7`;
  - physical map `a15159a581d008a962df99501cc64a7bf708d2c3c6b66b7c92c944410a961563`.
- `verify-plan.rb` and `git diff --check` pass after all artifact-index and mapping
  updates. Card187 remains Working On with only owner acceptance incomplete.
- Task-owned caffeinate PID84732 remains active. No cleanup, staging, commit,
  deployment or data-source mutation occurred.

I recommend owner acceptance of exact manifest020/Ready021, completion of
Card187, and only then dispatch of T026 under a new owner instruction.
