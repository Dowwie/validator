# T010 second incomplete evidence handoff escalation 017

The bounded completion response 016 added cases and the filter passes, but it still
does not provide the owner-required case-to-source evidence. I have not accepted
T010 or dispatched T011. The developer is paused and the candidate remains frozen.

## Stable state

The three corrected schema hashes remain unchanged from escalation 014. The only
implementation change is `tests/conformance.rs`, now 185 lines at SHA-256
`e6eb3840af115859899873b8c7b382efd884bf1d173a3353debc8baa7800287e`.
Response 016 is SHA-256
`459dc2ace138d41b86227195947d363ecd5095310bc7d513f3da5f677d4098a6`.

Coordinator reruns pass:

- `cargo fmt --all -- --check`;
- `cargo test --locked --test conformance input_schema_contract -- --nocapture`
  with one executed passing test;
- `cargo test --locked --lib` with 33 passed;
- `git diff --check`.

## Exact unsupported claims

Response 016 says every requested family is present at lines 61-158. The actual
test body does not substantiate that claim:

- There is a golden `label_set` rejection but no prediction-outcome `label_set`
  rejection.
- Observation evidence contains one legal scalar case. It has no legal
  bernoulli, reported-confidence, categorical, or label-marginal observation case;
  no isolated out-of-range reported-confidence case; and no isolated wrong-field
  or empty/vector-option case.
- `prediction_rejects_bounded_observation_and_extra` combines an invalid
  bernoulli value with extra fields at the prediction top level, prediction row,
  and outcome. Any one makes the document invalid, so it proves none of those
  rejection sources individually.
- `prediction_rejects_whitespace_source_model_option` combines a whitespace source
  key and whitespace model. It has no isolated assertions for source ID, model,
  description, question ID, preparation method/version, evidence path,
  observation name/option, or the other required nonblank sites.
- Only an empty categorical probability map is tested; the corresponding
  label-marginal shape is absent.
- `config_accepts_all_policies` validates only `label_thresholds`; there is no
  valid `reject_below` case. The later invalid reject case omits its signal and
  does not exercise a legal complete variant.
- `config_rejects_empty_thresholds_whitespace_extra` combines an empty map and an
  extra field, so it does not isolate either rule.
- Extra fields for golden top/episode/target are bundled into one document, and
  prediction top/row/outcome extras are bundled with an invalid observation.
  These do not provide the requested per-container evidence.
- The absent-source runtime boundary exists, but no second named shape-valid
  semantic boundary such as vocabulary-key coverage or categorical sum exists.

The completion prompt explicitly required named cases for every family, with
spec-derived expected results, and stated that prose without those assertions is
incomplete. A green aggregate filter cannot replace absent or confounded cases.

## Owner reassessment

This is the second incomplete/unsubstantiated T010 evidence handoff, so I am
returning it directly without another developer loop. No dependency or schema
architecture problem exists. If the owner authorizes a final correction, the
smallest reliable shape is an atomic table where every negative case mutates one
field of a valid base document and each positive variant is separately named;
that would make the rejection source observable and finish the still-required
matrix without schema redesign. The combined T007-T014 review allowance remains
unchanged. Card 183 is Waiting; task-owned caffeinate PID 84732/session 29011
remains active.
