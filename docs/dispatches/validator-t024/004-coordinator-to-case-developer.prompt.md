# Implement all fourteen T024 assigned S/M/E case filters

Role/model: fresh sole implementation developer, `gpt-5.6-terra`, high reasoning,
`fork_turns: none`. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #186 — Audit Validator's nine machine schemas](http://localhost:3006/1/cards/186).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t024/004-case-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat return only the
path, SHA-256 and terse status. Do not edit governance, plans, artifact-index,
session notes, acceptance records or Fizzy. Do not delegate.

This is the second and final local construction boundary inside T024. The first
boundary completed all nine schema contracts plus passing nonzero
`all_schema_contracts` and `cli_stdout_schema_matrix`. Implement every one of the
fourteen exact case filters assigned to T024; do not begin T025 or claim
independent acceptance.

Read global/repository AGENTS, complete T024, owner prompt001, schema prompts002/
003 and complete response002, every assigned coverage row and its exact normative
specification row. Read current schemas and test helpers before editing. Use public
synthetic inputs and real application paths; never private Chord data or inference.

Starting identities:

| Artifact | SHA-256 |
|---|---|
| `docs/plans/validator/tasks/T024.json` | `73edfe573170afca4c842b3dfed02c23c2d09d0d7fe8ad533c9f70035aebf78d` |
| `docs/plans/validator/coverage.json` | `efe2f425fc975f953c5765d1ea40ed47ffea8f672249d85f4e3eb07c9843e896` |
| `docs/specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `docs/specs/validator-data-model.md` | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| `docs/dispatches/validator-t024/002-schema-developer-to-coordinator.response.md` | `3f00d279951624aca41b3b26a78a686faae90d8938d5fe73249a21ddca91207b` |
| `schemas/v2/check.schema.json` | `1bfd2fb3a644df1203b043244b38a9ed80d404cae5068901b7f162ea6dd662b9` |
| `schemas/v2/comparison.schema.json` | `d0dd16bef3902ac1597bb6538204ffe7c13442c655b4779f9953f6fd5200c8e9` |
| `schemas/v2/config.schema.json` | `c740c7cd2c1381398b3121ee161b1d995a23ceb9dd4325bd73cd0073f66d65c3` |
| `schemas/v2/error.schema.json` | `a1be234949560a90dff9391a0fa9c42de4f0431c5c4e2f20d745730f40eb80d5` |
| `schemas/v2/golden.schema.json` | `97d811a192274f1ee0703e5dff475723301fdcc68c20b031d3b22431968b777b` |
| `schemas/v2/inspection.schema.json` | `039e6060b88042c908c27a34374592d9f64434aa1aac4725da13146b01c0ac82` |
| `schemas/v2/predictions.schema.json` | `654908068da5d6b3cf9c666cddf4ebc888f94203a04fb6addd5164fbbb893352` |
| `schemas/v2/receipt.schema.json` | `d820751593f82ac68cbca2ed381b305383d96d388dee417abcefec03ed3d1c00` |
| `schemas/v2/report.schema.json` | `5500e80464cd87b196f291634c00c073113220747f03898c66406f5106e94ea9` |
| `tests/conformance.rs` | `d085c1fc94ac15748b5e56fa0dc5c94528cad165c804ba9fef0dc8cc3e7737e4` |
| `tests/cli.rs` | `8c7b9c00d777f4a290911dd77199d8e1b9e085cc14fb0dd78a801108c3a10fcf` |

Stop and save the exact mismatch before editing. Write primarily
`tests/conformance.rs`. Minimal changes to the nine T024 schemas or `tests/cli.rs`
are allowed only when an actual assigned case proves a current contract gap. Do
not edit production Rust. Save any production-output/spec mismatch with the exact
reproduction for coordinator/owner routing rather than changing the test oracle or
schema to certify invalid output.

## Fourteen exact filters

Implement each named `#[test]` as an actual discriminating compound row, reusing
shared helpers without turning the suite into a generic audit framework:

| Filter | Required compound evidence |
|---|---|
| `case_s13` | Invalid UUID, duplicate ID/key, unknown class/field, missing input and partial scoring signals each produce the specific typed rejection; no silent subset/repair. |
| `case_s14` | Missing prediction, extra prediction and failed dataset digest are invalid runs; none becomes abstention or successful publication. |
| `case_s17` | Reordered episodes and probability object keys preserve calculations and sorted evidence while byte digests may differ. |
| `case_s21` | Declared `UNCERTAIN` is a legal class; missing, null and undeclared reference shapes reject without claiming reference settlement. |
| `case_s29` | Published schemas validate every real result; agent-facing stdout is one stable typed JSON document with statuses, receipts and errors. Reuse the umbrella evidence rather than duplicating CLI business logic. |
| `case_m16` | Missing multi-label row is alignment error; empty set is answered; whole abstention is explicit; missing marginal key is probability error. |
| `case_m21` | Count maps, mixed task kinds, explicit partial-reference masks, per-label abstention and wire version 1 all reject without fallback; legally shaped unreviewed reference sets remain a preparation concern, not a new validator rule. |
| `case_e05` | Retain scalar `1.33`, the categorical distribution and prepared `UNCERTAIN`; do not override to `MATCH`. Score distribution only when separately supplied as canonical scoring evidence. |
| `case_e06` | A retained categorical observation summing `0.99` plus scoring-family omission succeeds as hard-label evaluation; observation values remain unchanged, probability is `not_applicable`, preparation discloses omission. |
| `case_e09` | Displayed-vector mean differing from returned scalar neither replaces the scalar nor fails automatically; no consistency equation is invented. |
| `case_e10` | Auxiliary Bernoulli observation without gold is inspection-only; the same numeric value supplied as a scoring marginal under a separately labeled task receives specified binary metrics, with no task conversion. |
| `case_e11` | Unknown observation name/kind, nonfinite scalar, out-of-range Bernoulli and broken preparation evidence index each produce typed input/provenance errors with no silent omission. |
| `case_e12` | Observation absence on one row remains unavailable without imputation; a top-level scoring family missing on one row rejects as incomplete. |
| `case_e13` | Probabilities or scalar without an outcome is a schema error; no synthesized choice or abstention. |

Each exact command must list and execute nonzero:

```text
cargo test --locked --test conformance case_s13 -- --nocapture
cargo test --locked --test conformance case_s14 -- --nocapture
cargo test --locked --test conformance case_s17 -- --nocapture
cargo test --locked --test conformance case_s21 -- --nocapture
cargo test --locked --test conformance case_s29 -- --nocapture
cargo test --locked --test conformance case_m16 -- --nocapture
cargo test --locked --test conformance case_m21 -- --nocapture
cargo test --locked --test conformance case_e05 -- --nocapture
cargo test --locked --test conformance case_e06 -- --nocapture
cargo test --locked --test conformance case_e09 -- --nocapture
cargo test --locked --test conformance case_e10 -- --nocapture
cargo test --locked --test conformance case_e11 -- --nocapture
cargo test --locked --test conformance case_e12 -- --nocapture
cargo test --locked --test conformance case_e13 -- --nocapture
```

Rerun the two umbrella filters, all affected existing regressions, format, full
locked tests, warning-denied Clippy, locked release build and diff check. Preserve
opaque-object member edge values, strict task/status alternatives, zero-exclusion
intersection and the accepted 17+3 staged diagnostic limit. No suppression, fake
consumer, export widening, new framework, weakened tolerance or future-task work.

Response004 must map every filter to every subcase/assertion, public path,
diagnostic/status/value/population expectation and actual result; include nonzero
list confirmation, commands/exits/counts, exact schema/test hashes and residual
lint inventory. This completes only local T024 construction; the coordinator will
freeze the exact candidate for a fresh independent verifier.
