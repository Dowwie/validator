# T020 report-schema completion handoff

Status: complete local report-schema handoff. This is not a T020 acceptance or combined-review request.

## Input identity

The prompt013 starting hashes matched:

| Artifact | SHA-256 |
|---|---|
| `schemas/v2/report.schema.json` | `ec1e649325b7e621f16a471989b2a956fdfc1d6fab8a83263e2351b3ae904ecf` |
| `tests/conformance.rs` | `a18391bdfbf5a1f9aee9d6ae4a4bf1ee841bd154a76644a33192bd3f3914a07d` |

## Contract mapping

The report root closes the shared serialization produced by both report builders: identity, immutable artifact/evidence records, source/preparation descriptions, population, policy, integrity, raw/final results, probability, signals, and episode evidence.

| Schema definition | Serialized contract |
|---|---|
| `identity`, `artifact`, `source`, `observation`, `preparation`, `population`, `policy`, `integrity` | Shared single/multi report fields. Opaque configuration and observation values remain unconstrained only at their documented value positions. |
| `metric` | Every metric carries value, status, population count/unit/scope, numerator, and denominator. Infinity requires `value: null` plus `special_value: "+infinity"`; non-infinite statuses cannot carry that marker. |
| `singleHard`, `matrix`, `singleClass`, `singleProbability`, `singleSignals`, `singleEpisode` | Typed single-label totals, class matrix/abstention column, class metrics, categorical diagnostics, ten signal bins, and class/abstention episode evidence. |
| `multiHard`, `multiClass`, `multiProbability`, `multiSignals`, `multiBins`, `multiEpisode`, `multiOutcome` | Typed multi-label set/answered totals, TP/FP/FN/TN fields, label-decision metrics, marginal losses/bins, and exact set-difference evidence. |
| Root task `allOf` selection | Requires raw/final/probability/signals/episode alternatives to agree with `task.kind`, rejecting cross-family shapes. |

## Focused mutations

`single_report_schema` retains valid single-label documents and rejects missing top-level fields, a one-label single task, missing `raw.accuracy.population_unit`, an `ABSTAIN` label masquerading as the typed matrix abstention column, malformed evidence binding, and raw input leakage. Its check sample now includes the already-required concrete task field.

`multi_label_hard_oracles` validates a real published multi-label report and rejects a missing `answered_micro_f1.population_unit`, a foreign `matrix` hard field, a single-label probability family on a multi-label task, a finite value marked `positive_infinity`, and null answered-set evidence.

## Verification

| Command | Exit | Actual result |
|---|---:|---|
| `python3 -m json.tool schemas/v2/report.schema.json` | 0 | valid JSON Schema document |
| `cargo test --locked --test conformance single_report_schema -- --nocapture` | 0 | 1 selected, 1 passed |
| `cargo test --locked --test conformance multi_label_hard_oracles -- --nocapture` | 0 | 1 selected, 1 passed |
| `cargo test --locked --test conformance marginal_loss_and_bins -- --nocapture` | 0 | 1 selected, 1 passed |
| `cargo fmt --all -- --check` | 0 | passed |
| `git diff --check` | 0 | passed |

Cargo emitted the existing staged dead-code warnings during test compilation; no new Rust code or warning class was introduced. The required report-only gate did not include Clippy or release build.

## Final identity and scope

| Artifact | SHA-256 |
|---|---|
| `schemas/v2/report.schema.json` | `7215342a2958fd4d9e9735c2837130a02837d205db93b342b9f64d62a882fcf1` |
| `tests/conformance.rs` | `a08fe9ba75afefe1ff248128952aa37c6e05191b2991240566c79bbab4d79cbd` |

Only the report schema and focused conformance assertions changed. No production behavior, check/inspection schema, CLI test, fixture, dependency, governance, plan, artifact index, session note, acceptance record, Fizzy card, or T021/T022 behavior changed.
