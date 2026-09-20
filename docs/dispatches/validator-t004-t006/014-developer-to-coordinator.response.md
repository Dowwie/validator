# T004–T006 replacement developer repair handoff

The two required repairs are complete. This is implementation and test evidence
only; it does not claim verifier or owner acceptance.

## Candidate identity

The reconciled input hashes matched prompt 014 before editing. The final hashes
are:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `981783137668727726799ca99ee963f5333978cd6090ad9953b6d237a8e2c621` |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |
| `src/lib.rs` | `b60b0bf5d4fed5fe604e7c1039f3c21e315dc46d9bc08159d56d2aa4ab35162` |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` |
| `src/main.rs` | `536e506bb90914c243a12b397b9a998f85ae2cbd9ba02dfd03a9e155ca5ca0f4` |
| `src/model.rs` | `ceffb61ab5e523fa4bf3161d5db5a6fcd1276137d951982da4382edb79c58c66` |
| `src/model/common.rs` | `7892d11b1d16ef6a548ae20eac60177d2d82a80f43d2426ed8b83196c98b5859` |
| `src/model/single_label.rs` | `8195f4555b14618b00605d879c8d09cc9564f387421fc1849a3c21c1f977a266` |
| `src/validation.rs` | `4d257a78924bb134c04ad620a90cfc92b107709ed0a2db424476ff01014dfcbe` |
| `src/validation/wire.rs` | `46a42903c2edbb556b1ae1ee7b3c3d9642e562e6b2de2a6d63a852c584f6a3b3` |

Changed implementation files are `src/model/common.rs`,
`src/model/single_label.rs`, `src/validation.rs`, and
`src/validation/wire.rs`. `Cargo.toml` and `Cargo.lock` retain the reconciled
partial baseline's `arbitrary_precision` and `raw_value` feature boundary and
their input hashes. All other required manifest paths retain their input hashes.

| Changed path | Input SHA-256 | Output SHA-256 |
|---|---|---|
| `src/model/common.rs` | `24b1541ca7ec1cf2e49f06e8586c323e642b889711a5fa11c35d7dc22678b18c` | `7892d11b1d16ef6a548ae20eac60177d2d82a80f43d2426ed8b83196c98b5859` |
| `src/model/single_label.rs` | `35fe11ada58053273708751631c079984f86e6e6fad3f7d24dcae502c4b92fe5` | `8195f4555b14618b00605d879c8d09cc9564f387421fc1849a3c21c1f977a266` |
| `src/validation.rs` | `cea3789e2addbeedaaeee216dfa192b989d1508aac10c9cb25b3c3d1c81f23ba` | `4d257a78924bb134c04ad620a90cfc92b107709ed0a2db424476ff01014dfcbe` |
| `src/validation/wire.rs` | `550d9f56a780d690d8606347be6cf8d6a5b9d750a22a4667e5b44e1c6a9011bc` | `46a42903c2edbb556b1ae1ee7b3c3d9642e562e6b2de2a6d63a852c584f6a3b3` |

## Repair 1: numeric boundary

The duplicate-key scan continues to parse legal arbitrary-precision JSON
numbers while rejecting duplicate keys recursively. Fixed strict wire structs
now decode numeric observation, categorical-probability, and reject-below fields
directly, before internally tagged serde content buffering can replace an
arbitrary-precision token with its private number representation. They preserve
closed tags and `deny_unknown_fields`; typed numeric positions still use
`JsonNumber` backed by `RawValue` and semantic `f64` conversion.

Before this repair, `large_number_boundary` reproduced the typed scalar result
`E_SCHEMA` (`invalid type: newtype struct, expected any valid JSON value`) while
the opaque cases decoded. It now proves through `validate_single_label` that:

- `1e400` remains opaque in golden `input`, source configuration, and nested
  preparation configuration;
- a scalar observation `1e400` reaches finite binary64 validation and returns
  `E_OBSERVATION`;
- a reject-below `minimum: 1e400` decodes as a legal DTO and then returns the
  intended unsupported-policy `E_CONFIG`;
- the private-number marker object remains `E_SCHEMA` in typed numeric fields
  and remains opaque where allowed.

## Repair 2: checked evaluation boundary

`EvaluationConfig<SingleLabelPolicy>` now owns nonblank description, role,
optional checked parent ID, omitted-versus-explicit selection, and only the
admitted `AsRecorded` policy. Requested IDs are parsed and deduplicated before
selection. `validate_single_label` constructs this checked configuration before
selection or population creation.

`Population` owns the supplied checked dataset digest, checked configuration,
derived exact dataset and selected counts, and byte-sorted selected/unselected
ID vectors. Its constructor rejects duplicate, overlapping, or configuration
inconsistent partitions. `AlignedRow::new` is fallible and verifies row and
prediction identity plus target vocabulary membership. The fallible
`SingleLabelEvaluation::new` verifies row count/order against the population,
row/prediction identity, source presence, and target vocabulary membership.

`population_alignment` exercises actual admission and checks stored digest,
counts, description, role, parent ID, UUID order, omitted/all selection,
explicit-empty selection, and subset selection. The owning single-label module
also proves an otherwise checked but population-mismatched arbitrary row rejects.

## Verification

Every named focused command exited 0 and executed a nonzero test count in the
final rerun:

| Filter | Executed | Result |
|---|---:|---|
| `large_number_boundary` | 1 | pass |
| `json_number_marker_collision` | 1 | pass |
| `typed_optional_nulls` | 1 | pass |
| `wire_tags_and_fields` | 1 | pass |
| `strict_json_keys` | 1 | pass |
| `opaque_number_and_null` | 1 | pass |
| `observation_contracts` | 1 | pass |
| `population_alignment` | 3 | pass |
| `validate_before_selection` | 1 | pass |
| `dataset_digest_binding` | 1 | pass |
| `source_preparation_bindings` | 1 | pass |
| `categorical_admission` | 1 | pass |
| `scored_choice_ties` | 1 | pass |
| `artifact_signal_completeness` | 1 | pass |

Combined checks:

| Command | Exit | Result |
|---|---:|---|
| `cargo test --locked --lib -- --list` | 0 | 24 tests, 0 benchmarks; every named filter present |
| `cargo fmt --all -- --check` | 0 | clean |
| `cargo test --all-features --locked` | 0 | 24 library tests passed; binary/doc targets ran 0 and passed |
| `cargo build --locked --bin validator` | 0 | built |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | incomplete-consumer `dead_code` only |
| `git diff --check` | 0 | clean |

Clippy reported only `dead_code`, with 107 production groups (`src/model.rs` 6,
`src/model/common.rs` 39, `src/model/single_label.rs` 16,
`src/validation.rs` 15, and `src/validation/wire.rs` 31) and 19 library-test
groups (`src/model/common.rs` 11, `src/model/single_label.rs` 3, and
`src/validation/wire.rs` 5). No other warning class appeared; no suppression,
placeholder, fake caller, or fallback was added. Clean Clippy remains the known
later-consumer gate.

The task-owned `caffeinate` process remains active as PID 84732
(`/usr/bin/caffeinate -i -m`). No Cargo, Rust compiler, Clippy, or test process
remains active. The workspace has about 16 GiB free. The repository remains
unborn/untracked, so `git diff --check` is whitespace verification only.
