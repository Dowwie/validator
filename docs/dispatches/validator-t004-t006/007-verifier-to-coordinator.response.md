# Revise

The frozen candidate implements most of the T004 and T005 checked-admission
rules and the central T006 alignment flow, and both owner-authorized null/marker
corrections work for their focused cases. It is not Ready because the bounded
owner follow-up demonstrates a numeric-range regression in the strict JSON
boundary, and the T006 checked result omits required population/configuration
invariants while exposing unchecked crate-wide construction.

## Required findings

### 1. The duplicate-key pass rejects legal opaque `1e400` and prevents typed `1e400` from reaching `E_OBSERVATION`

- **Violated contract:** main-spec Canonical golden dataset permits arbitrary JSON
  in required opaque `input`; Canonical prediction artifact permits opaque source
  and preparation configuration objects; Validation and numerical rules requires
  opaque numeric values to retain their JSON value while typed observation
  numbers are converted to finite binary64 and invalid values report
  `E_OBSERVATION`. Follow-up 009 requires this exact distinction.
- **Location:** `Cargo.toml:9` enables serde_json `raw_value` and `std` but removed
  `arbitrary_precision`. `src/validation/wire.rs:245-257` runs every JSON value
  through `serde_json::Deserializer::deserialize_any` before DTO decoding;
  `src/validation/wire.rs:266-375` recursively visits nested objects/arrays and
  ordinary numeric visitor methods. Any error in that pass becomes `E_PARSE`.
  RawValue admission at `src/validation/wire.rs:35-39,59-70,101-106,203-218`
  and typed conversion at `src/validation.rs:294-345` occur only afterward.
- **Independent reproduction:** I compiled a disposable harness that imports the
  frozen production `src/validation/wire.rs`, calls its actual
  `reject_duplicate_keys`, and then applies the same strict DTO decode used by
  `validation::decode`. The inputs were a valid golden record with
  `"input":{"huge":1e400}`, a valid prediction source whose configuration was
  `{"huge":1e400}` and whose preparation configuration nested the same number,
  and a valid scalar observation with `"value":1e400`. The observed output was:

  ```text
  golden-scan=err Parse
  opaque-scan=err Parse
  typed-scan=err Parse
  golden-decode=err Parse
  opaque-decode=err Parse
  typed-decode=err Parse
  ```

  Thus no opaque case reaches RawValue retention, and the typed scalar never
  reaches `number_as_f64` and its `E_OBSERVATION` mapping. This is a range failure,
  distinct from the passing exact opaque integer `9007199254740993` regression.
- **Consequence:** syntactically legal opaque dataset/configuration JSON can be
  rejected and a typed observation range violation receives the wrong stable
  diagnostic. The marker fix therefore did not preserve the prior opaque-value
  contract.
- **Smallest correction:** make the duplicate scanner consume syntactically legal
  JSON numbers without imposing binary64 range. One bounded route is to enable
  serde_json `arbitrary_precision` alongside `raw_value`, while changing
  `JsonNumber` to admit only an actual raw numeric token before constructing its
  numeric representation. That retains large opaque numbers, lets typed `1e400`
  reach finite-number validation, and keeps the literal private-marker object an
  object that fails numeric DTO decoding. Add actual-entry regressions for all
  three opaque locations and the scalar `E_OBSERVATION` result. Do not add a
  reserved-key or large-number blacklist, relax duplicate detection, or change
  binary64 semantic arithmetic.

### 2. The T006 checked boundary lacks the required checked evaluation configuration and population identity

- **Violated contract:** T006 and data-model requirements D022, D023, D050,
  D052, and D053 require `Population` to own dataset identity and exact counts,
  require a checked `EvaluationConfig<Policy>`, require complete configuration
  validation before selection, and require validated structures to use checked
  constructors rather than an arbitrary-vector construction path.
- **Location and reproduction by inspection:** `src/model/common.rs:787-835`
  defines `Population` with description, role, parent ID, selected IDs, and
  unselected IDs only. It contains no `ArtifactDigest`, dataset count, or selected
  count. There is no checked model `EvaluationConfig<Policy>` anywhere in
  `src/model`; the only `EvaluationConfig` is the private wire DTO at
  `src/validation/wire.rs:165-177`. In `validate_single_label`, selection occurs
  at `src/validation.rs:146` before policy, role, parent ID, and nonblank population
  checks at `src/validation.rs:153-176`. Finally,
  `SingleLabelEvaluation::new` at `src/model/single_label.rs:204-221` is a
  `pub(crate)` infallible constructor accepting an arbitrary `Vec<AlignedRow>`;
  `AlignedRow::new` at lines 183-190 is likewise infallible. These are not checked
  constructors returning typed errors and are callable by later crate modules.
- **Consequence:** the validated evaluation drops the dataset identity it just
  verified, cannot carry the specified checked evaluation configuration as a
  model invariant, and permits later in-crate code to manufacture an evaluation
  from arbitrary rows. Configuration semantics are also evaluated after the
  selection step contrary to the required boundary order.
- **Smallest correction:** add the specified checked
  `EvaluationConfig<SingleLabelPolicy>` model record; make `Population` own the
  checked dataset digest and exact dataset/selected counts with its selected and
  unselected identities; validate population description, role, parent, and legal
  policy before selection; and make aligned-evaluation construction checked (or
  otherwise inaccessible to later scoring except through the validated boundary).
  Extend `population_alignment` to assert the preserved digest, counts,
  description, role, parent ID, selected/unselected IDs, and rejected arbitrary
  construction.

## Frozen candidate identity

I recomputed the complete manifest set before review, after the suspension
liveness check, and after every required command. All frozen values remained
stable:

| Artifact | SHA-256 |
|---|---|
| `docs/dispatches/validator-t004-t006/006-coordinator-candidate-manifest.md` | `0aab3ef37c8635485c4e5c40ebbaa3dedcb03be374fcf9d44e02187f6495ae3d` |
| `Cargo.toml` | `70fdbc505a467643c079f2b1a514514370cb3d5524e81bb8ca43c3eb5566686d` |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |
| `src/lib.rs` | `b60b0bf5d4fed5fe6045e7c1039f3c21e315dc46d9bc08159d56d2aa4ab35162` |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` |
| `src/main.rs` | `536e506bb90914c243a12b397b9a998f85ae2cbd9ba02dfd03a9e155ca5ca0f4` |
| `src/model.rs` | `ceffb61ab5e523fa4bf3161d5db5a6fcd1276137d951982da4382edb79c58c66` |
| `src/model/common.rs` | `24b1541ca7ec1cf2e49f06e8586c323e642b889711a5fa11c35d7dc22678b18c` |
| `src/model/single_label.rs` | `35fe11ada58053273708751631c079984f86e6e6fad3f7d24dcae502c4b92fe5` |
| `src/validation.rs` | `2acf5459b809dc60409a0f3cd4e0ee0a70af981ee668fcbc8f42f1b6bdd82f22` |
| `src/validation/wire.rs` | `cfe043a59326edd5fc611e5b7feba11df2ae96d30a25b1e8d2501ed0088a8047` |
| `docs/dispatches/validator-t004-t006/001-developer-to-coordinator.response.md` | `ecd49c30a119c44ac09b46873f35f8000cc9ee69e2bc93c28868563205d32fe0` |
| `docs/dispatches/validator-t004-t006/003-developer-to-coordinator.response.md` | `38211a393ef73a711a8cf9858826546ac83a79d4a090692a603bda0439acf12a` |

The bounded follow-up artifacts read during review were also stable:

| Artifact | SHA-256 |
|---|---|
| `008-owner-to-coordinator.prompt.md` | `7a79179dc5222752c01f0d8b4736fbe860b2f59bf76cb7e64c79862a62afc3f0` |
| `008-coordinator-to-owner.response.md` | `a1b9ba90c43bba8243dee8385bd86774aaa4f562cd5f0c07c049a1c420536dad` |
| `009-coordinator-to-verifier.prompt.md` | `0c6d7b37b22a68a857dee79fbaed18b0441952a5447c38cf37d73600b23573a3` |

## T002 correction evidence

The omission/null correction is sound for the enumerated typed optional fields.
`src/validation/wire.rs:63-70,85-86,114-119,128-130,173-176` combines serde
`default` with `omitted_or_present`, so omission yields `None` while a present
JSON null must deserialize as the concrete `T` and fails `E_SCHEMA`.
`typed_optional_nulls` passes and includes the actual `validate_single_label`
path for `episode_ids: null`. Required opaque golden input still accepts null,
and opaque configuration objects retain top-level and nested null members.

The private-number marker correction also satisfies its focused contract.
`json_number_marker_collision` proves the literal marker object fails as numeric
confidence with `E_SCHEMA` while golden input and source/preparation configuration
retain its object kind, exact key/string value, and nesting. `raw_value` also
preserves the tested large integer and explicit null cases, original submitted
bytes remain in `Decoded`, strict duplicate and legal wire-shape tests pass, and
no alternate decoder or public construction path was added. Finding 1 is the
separate large-exponent consequence that remains unsatisfied.

## T004 criterion evidence

Apart from the shared parser defect in finding 1, the T004 implementation meets
the checked source and observation rules:

- `SourceId`, source model, scored-choice question ID, observation names,
  definition descriptions/question IDs, and preparation method/version receive
  exact nonblank checks. Declared unused sources and definitions remain owned.
- All five observation variants are closed and distinct. Scalar is finite and
  unbounded; Bernoulli/reported confidence are finite in `[0,1]`; categorical and
  label-marginal observations are nonempty maps with nonblank exact keys and
  finite `[0,1]` values. Observation vectors are neither normalized nor bound to
  vocabulary, so the admitted categorical sum `0.99` is retained.
- Row omission produces an empty checked `ObservationSet`. Unknown names and kind
  mismatches return `E_PROVENANCE`; invalid numeric/range/vector values return
  `E_OBSERVATION`. Values remain distinct from scoring vectors, so no observation
  promotion, scalar/vector consistency rule, or implicit class choice exists.
- Preparation indices are converted to `usize`, required nonempty, unique, and in
  range of retained evidence. Source and preparation opaque configurations remain
  owned RawValues. `Prediction<Output>` owns an actual checked `ObservationSet`.

## T005 criterion evidence

The T005 admission logic is otherwise sound:

- `CategoricalDistribution::new` requires exact vocabulary coverage, finite
  `[0,1]` components, positive vocabulary-order mass, and absolute sum error at
  most `1e-9`. It keeps separate submitted and normalized working vectors and
  performs no clipping, insertion, inference, or observation promotion.
- `ReportedConfidence` enforces finite `[0,1]`. Both signals can remain attached
  to answered or abstained classifier outputs, and classifier outcomes are not
  forced to argmax.
- Scored-choice sources require a nonblank question ID at source admission and a
  class outcome, categorical distribution, confidence, and exact maximum choice
  per row. Exact tied maxima are accepted.
- The all-or-none flags are computed across every admitted submitted prediction
  before selection. Probability and confidence presence are independently
  constant because the complete boolean pair must match on every row; the empty
  artifact yields neither family.

## T006 criterion evidence

The central T006 flow correctly decodes all three inputs, checks the supplied
digest against the declared digest, validates every dataset episode and every
prediction/source before population selection, rejects duplicate dataset and
prediction IDs, and checks signals before selection. Omitted selection chooses
all, explicit empty selection chooses none, explicit IDs are checked unique and
existing, prediction IDs must equal selection exactly, and selected/unselected
IDs are sorted through `EpisodeId` UUID byte ordering. Missing/extra rows are
`E_ALIGNMENT`; digest mismatch is `E_PROVENANCE`; only as-recorded policy is
admitted. The closed result owns vocabulary, sources, population, policy, signal
availability, and rows.

Those behaviors do not cure finding 2: the model-level population/configuration
identity and construction boundary remain incomplete.

## Independent commands

Every required filter executed a nonzero count. Each named filter below exited 0
with one passing test and 20 filtered tests; the test target emitted the same 24
incomplete-consumer dead-field warnings:

| Filter |
|---|
| `typed_optional_nulls` |
| `json_number_marker_collision` |
| `wire_tags_and_fields` |
| `strict_json_keys` |
| `opaque_number_and_null` |
| `observation_contracts` |
| `source_preparation_bindings` |
| `categorical_admission` |
| `scored_choice_ties` |
| `artifact_signal_completeness` |
| `population_alignment` |
| `validate_before_selection` |
| `dataset_digest_binding` |

| Command | Exit | Result |
|---|---:|---|
| `cargo test --locked --lib -- --list` | 0 | Listed 21 tests and 0 benchmarks; all required exact names appeared once. |
| `cargo fmt --all -- --check` | 0 | No formatting differences or output. |
| `cargo test --all-features --locked` | 0 | 21 library tests passed; binary and doc-test targets each ran 0 tests and passed. |
| `cargo build --locked --bin validator` | 0 | Locked binary built; 98 production dead-code warnings. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Failed solely on the incomplete-consumer `dead_code` groups classified below. |
| `git diff --check` | 0 | No tracked whitespace error; rustfmt remains the substantive check in this unborn/untracked repository. |
| Frozen production-path `1e400` probe | 0 | Probe process completed; its six application results were the `E_PARSE` failures shown in finding 1. |

## Exact Clippy classification and remaining limit

Clippy did **not** pass and is not waived. Its only diagnostic class is
`dead_code`:

- Production library: exactly 98 groups — `src/model.rs` 6,
  `src/model/common.rs` 36, `src/model/single_label.rs` 16,
  `src/validation.rs` 14, and `src/validation/wire.rs` 26.
- Library-test target: exactly 24 unread-field/method groups —
  `src/model/common.rs` 14, `src/model/single_label.rs` 5, and
  `src/validation/wire.rs` 5.

There is no `unused_imports`, Clippy style/performance class, suppression,
placeholder, fake production caller, or other warning class. Searches found no
`allow`/`expect`, `todo!`, or `unimplemented!`. `validate_single_label` has no
production caller yet; all current calls are owning-module tests. Clean full
Clippy remains mandatory at T014 and T017.

## Execution and read-only state

I did not edit implementation, tests, Cargo files, specifications, plans,
tracking, or the artifact index. This response is my only repository write. The
disposable `/tmp` probe executable was removed after it completed. No verifier-
owned Cargo, Rust compiler, Clippy, or probe process remains active. `target/` is
approximately 445 MiB and the filesystem has approximately 16 GiB available;
there is no process or resource blocker.
