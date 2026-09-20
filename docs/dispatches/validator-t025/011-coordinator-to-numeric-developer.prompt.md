# Repair four bounded T025 test-evidence gaps

Role/model: retained sole oracle/test developer, `gpt-5.6-terra`, high reasoning,
original fresh `fork_turns: none` context. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #187 — Verify Validator numerical conformance](http://localhost:3006/1/cards/187).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t025/011-numeric-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat return only the
path, SHA-256 and terse status. Do not edit governance, plans, artifact-index,
session notes, acceptance records or Fizzy. Do not delegate.

This is the one finding-driven test/fixture repair cycle authorized by owner
prompt001 and confirmed by the owner after the frozen review. Read the complete
verifier verdict010 before editing. Write only `tests/conformance.rs`; no fixture
change is expected. Production, schemas, fixtures and tolerances are read-only.
Preserve every other assertion and the complete 74-test inventory. Do not start
T026 or add optional coverage.

Starting identities:

| Artifact | SHA-256 |
|---|---|
| `009-coordinator-candidate-manifest.md` | `8071c565c7c4f544cf3072c993df11ea70d3f80a71cbc85c28fd93a99aea6076` |
| `010-coordinator-to-verifier.prompt.md` | `6ce3c2ff6397d67743db9518434104a3ffd8291c4674e6c3bd04f8db5658eb67` |
| `010-verifier-to-coordinator.response.md` | `9b952709641bcbbe89f2be5cc032be318fad1a37483e8b4a58875a67a15f6faa` |
| `tests/conformance.rs` | `5dd2fb2b0b0c6adcdc6150466f475fe4395a4b60b107ae3ab03a4ed0397bee6d` |
| `tests/fixtures/single-label/expected.json` | `ab93c379b9c0df1d0c0fb7aac7a6df5e7748f3f671d55dd69d173e640090b268` |
| `tests/fixtures/multi-label/expected.json` | `213183a46c0f6c011296ef5087b5c44d3852081bdf20eaa80cbb494d3d2c84f2` |
| `src/model/multi_label.rs` | `3f515a0a9122db963b2e01ca14d5474b4836ba3543a48a4e0766a4459eea0b7c` |
| `schemas/v2/report.schema.json` | `805de93d2a50339370e803b7ff1a9a1b8d34c3028b5b21a1a6f428f591733f90` |
| `schemas/v2/inspection.schema.json` | `2c7e064d0adb915b77c558a7ebbfade96f7bcf0eee890b881ced1383f0eedaef` |

## Exact corrections

1. **Direct-ratio population metadata.** Extend the shared test-only direct-ratio
   assertion boundary so each caller supplies and verifies independently expected
   `population_count`, `population_unit` and `population_scope` in addition to
   value/status/numerator/denominator. Update every current call site, including
   both exhaustive oracles and exact S/E/M cases, from the ratified population
   semantics. In S03 specifically, selected accuracy/coverage/class metrics must
   remain distinguishable from `G=3` answered selective accuracy. Do not infer the
   expectation from the actual report or loosen the assertion for convenience.

2. **Exact S12 bin placement.** Preserve the threshold half. For the combined bin
   inputs `0`, `next_down/equal/next_up(i/10)` for `i=1..9`, and `1`, assert the
   independently derived exact per-bin count vector `[2,3,3,3,3,3,3,3,3,3]` and
   an independently calculated exact/tolerance-checked mean signal for every bin,
   in addition to declared boundary metadata and inclusion of signal `1` in bin 9.
   The evidence must fail if a below/equal/above sample moves across a boundary;
   family-wide included IDs plus nonzero bins are insufficient.

3. **M02 per-label undefined F1.** For the `[A,B]` answered empty-set case, iterate
   over both per-label entries and assert label identity, `TP=FP=FN=0,TN=1`,
   support/answered support/predicted support zero, F1 null with
   `undefined_zero_denominator`, direct operands `0/0`, and the ratified population
   count/unit/scope. Preserve the existing exact-match, coverage, Hamming,
   undefined micro-F1 and macro undefined-class assertions.

4. **Actual M14 multi-label class target.** Replace or add the required second
   subcase so the multi-label golden episode itself contains
   `"expected":{"type":"class","label":"A"}` with an otherwise legal
   multi-label prediction. Assert public `E_CONFIG` and absent output through the
   existing public error helper. A separate invalid class prediction may remain,
   but cannot substitute for this target-side input. Preserve the other three
   required M14 subcases.

These are proof corrections, not product or expectation changes. If the stricter
assertions reproduce a product mismatch, freeze exact input/expected/actual
evidence and stop; do not edit production/schema/fixtures or change tolerance.

## Focused boundary

Run serially and record exact exits/counts:

```text
cargo test --locked --test conformance exhaustive_single_label -- --nocapture
cargo test --locked --test conformance exhaustive_multi_label -- --nocapture
cargo test --locked --test conformance case_s03 -- --nocapture
cargo test --locked --test conformance case_s12 -- --nocapture
cargo test --locked --test conformance case_m02 -- --nocapture
cargo test --locked --test conformance case_m14 -- --nocapture
cargo test --locked --test conformance full_numeric_conformance -- --nocapture
cargo test --locked --test conformance -- --list
cargo test --all-features --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo fmt --all -- --check
git diff --check
```

The list must remain 74 tests. Full tests must pass. Clippy may exit 101 only for
the exact unchanged accepted 17 production plus three duplicate lib-test staged
diagnostics; no repair-owned warning is allowed. Do not rerun release by habit.

Response011 must map each finding to the concrete correction and discriminating
assertion, record commands/exits/counts, exact changed/unchanged hashes and any
real mismatch. Return only when the complete focused repair is ready for the same
verifier's bounded recheck.
