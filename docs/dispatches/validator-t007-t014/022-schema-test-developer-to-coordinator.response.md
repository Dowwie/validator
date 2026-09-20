# T010 matrix implementation handoff

## Scope and frozen inputs

I completed the five remaining schema-matrix families only in
`tests/conformance.rs`. The required frozen inputs matched before editing:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `e257b5d70c78f0dd7465093aad8d092db99c35ffa28227298271512faf49bec6` |
| `Cargo.lock` | `86caa4cd0470cc4d1f8d4d8dedbdfccb1c43e42b9f0a5a119343e35b256dd74e` |
| `schemas/v2/golden.schema.json` | `97d811a192274f1ee0703e5dff475723301fdcc68c20b031d3b22431968b777b` |
| `schemas/v2/predictions.schema.json` | `654908068da5d6b3cf9c666cddf4ebc888f94203a04fb6addd5164fbbb893352` |
| `schemas/v2/config.schema.json` | `c740c7cd2c1381398b3121ee161b1d995a23ceb9dd4325bd73cd0073f66d65c3` |
| `tests/conformance.rs` before this change | `916a7d485f4819b436eb6ef2983e9ceb86a3875cf79561d4c10cb7bd80fe2d17` |

`prediction_base` and `config_base` provide valid reusable documents. Every new
negative case mutates only the field or shape named by its assertion.

## Case evidence

1. Bounded values and closed fields:
   - `prediction_rejects_out_of_range_bernoulli_observation` at line 154 expects
     rejection of only `bernoulli.value: 2`.
   - `prediction_rejects_extra_envelope_field` at line 162,
     `prediction_rejects_extra_prediction_row_field` at line 170, and
     `prediction_rejects_extra_outcome_field` at line 178 each expect rejection
     of exactly one extra field.
2. Whitespace model and source key:
   - `source_rejects_whitespace_only_model` at line 186 expects rejection of an
     otherwise valid source whose only mutation is `model: " "`.
   - The retained `source_key_whitespace_red_green` at line 226 expects rejection
     of the whitespace-only source key. `sources.propertyNames` and
     `source.model` both use the shared nonblank `$defs.label` reference.
3. Configuration rules:
   - `config_rejects_empty_label_thresholds` at line 200 expects rejection of an
     otherwise valid `label_thresholds` decision with only an empty map.
   - `config_rejects_extra_decision_field` at line 208 expects rejection of an
     otherwise valid `as_recorded` decision with only `extra: true` added.
4. Probability branches:
   - `prediction_accepts_categorical_probabilities` at line 118 expects
     acceptance of nonempty categorical values.
   - `prediction_rejects_empty_label_marginals_probability_map` at line 136
     expects rejection of empty marginal values.
   - Retained branch evidence is `prediction_accepts_label_marginals` at line
     109 and `prediction_rejects_empty_categorical_probability_map` at line 127.
5. Observation shapes:
   - `observation_rejects_scalar_values_field` at line 255 expects rejection of
     scalar `values`.
   - `observation_rejects_vector_value_field` at line 264 expects rejection of
     categorical `value`.
   - `observation_rejects_whitespace_vector_key` at line 273 expects rejection
     of the whitespace-only categorical map key.
   - `observation_rejects_empty_label_marginals_values` at line 291 expects
     rejection of empty marginal values.
   - Retained complementary evidence is `all_observation_variants_legal` at line
     246, including legal scalar `1.33`, all five kinds, and categorical values
     summing to `0.99`; `observation_rejects_empty_categorical_values` at line
     282 retains the empty categorical rejection.

The existing `runtime_source_binding_boundary` remains at line 220. No second
runtime-only source boundary was added. Existing legal policy, multi-label,
label-marginal, empty-categorical, and `reject_below_complete` assertions remain.

## Results

| Command | Exit | Result |
|---|---:|---|
| `cargo fmt --all -- --check` | 0 | Passed; 0 formatting errors. |
| `cargo test --locked --test conformance input_schema_contract -- --nocapture` | 0 | 1 passed, 0 failed. |
| `cargo test --locked --lib` | 0 | 33 passed, 0 failed. |
| `git diff --check` | 0 | 0 whitespace errors. |

The Cargo runs emitted existing dead-code warnings from unfinished library areas;
they produced no test failures or nonzero command exits.

## Candidate identity and limits

| Path | SHA-256 after change |
|---|---|
| `tests/conformance.rs` | `99b0a0eda61a8294a8f3489c019eb3827f919e36abb296ba83c77644176e18e7` |
| `schemas/v2/golden.schema.json` | `97d811a192274f1ee0703e5dff475723301fdcc68c20b031d3b22431968b777b` |
| `schemas/v2/predictions.schema.json` | `654908068da5d6b3cf9c666cddf4ebc888f94203a04fb6addd5164fbbb893352` |
| `schemas/v2/config.schema.json` | `c740c7cd2c1381398b3121ee161b1d995a23ceb9dd4325bd73cd0073f66d65c3` |

No isolated case exposed a schema defect, so all three schemas remain unchanged.
There are no unresolved failures. Per the dispatch scope, I made no dependency,
framework, production, fixture, Fizzy, artifact-index, or schema changes. The
required response artifact is the sole documentation write; the coordinator owns
its indexing and Card 183 update.
