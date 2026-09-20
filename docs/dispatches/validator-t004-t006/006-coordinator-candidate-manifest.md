# T004-T006 corrected checked-admission candidate manifest 006

This manifest supersedes pre-correction manifest 002. The sole developer completed
the owner-authorized optional-null and serde_json marker corrections before
independent review. Implementation writes are paused while the verifier reviews
this exact coherent T004 -> T005 -> T006 candidate and the two bounded T002 wire
invariants. None of the constituent tasks is independently accepted here.

## Frozen candidate hashes

| Path | SHA-256 | Candidate role |
|---|---|---|
| `Cargo.toml` | `70fdbc505a467643c079f2b1a514514370cb3d5524e81bb8ca43c3eb5566686d` | serde_json uses `raw_value` and `std`; marker-vulnerable `arbitrary_precision` removed |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` | Resolved package set unchanged |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` | Accepted toolchain baseline, unchanged |
| `src/lib.rs` | `b60b0bf5d4fed5fe6045e7c1039f3c21e315dc46d9bc08159d56d2aa4ab35162` | Accepted crate wiring, unchanged |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` | Accepted diagnostics, unchanged |
| `src/main.rs` | `536e506bb90914c243a12b397b9a998f85ae2cbd9ba02dfd03a9e155ca5ca0f4` | Accepted binary scaffold, unchanged |
| `src/model.rs` | `ceffb61ab5e523fa4bf3161d5db5a6fcd1276137d951982da4382edb79c58c66` | Single-label module wiring |
| `src/model/common.rs` | `24b1541ca7ec1cf2e49f06e8586c323e642b889711a5fa11c35d7dc22678b18c` | T004/T006 checked records; source/preparation opaque configuration owns raw JSON |
| `src/model/single_label.rs` | `35fe11ada58053273708751631c079984f86e6e6fad3f7d24dcae502c4b92fe5` | T005 checked signals/output and T006 closed evaluation |
| `src/validation.rs` | `2acf5459b809dc60409a0f3cd4e0ee0a70af981ee668fcbc8f42f1b6bdd82f22` | Ordered admission plus wire-correction and task tests |
| `src/validation/wire.rs` | `cfe043a59326edd5fc611e5b7feba11df2ae96d30a25b1e8d2501ed0088a8047` | Strict DTOs with presence-sensitive options, raw opaque JSON, and numeric/object separation |
| `docs/dispatches/validator-t004-t006/001-developer-to-coordinator.response.md` | `ecd49c30a119c44ac09b46873f35f8000cc9ee69e2bc93c28868563205d32fe0` | Original T004-T006 developer evidence |
| `docs/dispatches/validator-t004-t006/003-developer-to-coordinator.response.md` | `38211a393ef73a711a8cf9858826546ac83a79d4a090692a603bda0439acf12a` | Pre-freeze correction evidence |

The accepted T002 repair manifest remains historical input at SHA-256
`3e0c155c1a64e6ec75d8bf518400fcc3c12d283ec3774de705833eb912843e91`;
this candidate deliberately changes its Cargo feature and wire/validation hashes
under owner prompts 002 and 004.

## Correction evidence

Both owner concerns reproduced. Ordinary `Option<T>` accepted present JSON null as
omission. Every optional typed wire field now uses a defaulted presence-sensitive
deserializer, so omission remains legal while present null fails `E_SCHEMA`.
`typed_optional_nulls` covers all Source, ObservationDefinition, Prediction,
abstention, and EvaluationConfig optional fields, including the real
`validate_single_label` path for `episode_ids: null`.

With serde_json `arbitrary_precision`, the literal object
`{"$serde_json::private::Number":"0.5"}` collided with the parser's private number
marker. The candidate replaces that feature with `raw_value`: typed numeric
positions use normal number decoding and reject the object as `E_SCHEMA`, while
golden input and source/preparation opaque JSON preserve the object kind, exact
key, string value, nesting, nulls, and big integers. Original artifact bytes remain
retained separately. `json_number_marker_collision` exercises both sides.

## Task evidence and limit

All 13 required filters pass exactly one test: the two correction filters, the
three accepted T002 filters, and the eight T004-T006 filters. The complete library
inventory contains 21 tests. `cargo fmt --all -- --check`, full all-feature locked
tests, the locked binary build, and `git diff --check` pass. Coordinator reruns of
both correction filters, the 21-test inventory, format check, planning verifier,
and diff check also pass with stable hashes.

Clippy does not pass. The developer records exit 101 only on incomplete-consumer
`dead_code`: 98 library diagnostics and 24 library-test diagnostics across private
model, validation, wire, and accepted task-wrapper code. No unused import or other
warning class, suppression, fake caller, placeholder, or public SDK expansion is
reported. The verifier must independently confirm this classification. Clean full
Clippy remains mandatory at T014 and T017.

No T007-or-later metric, schema, evidence-loading, publication, report, or CLI
behavior is part of this candidate. The normal combined checkpoint's single
verifier-driven repair allowance remains unused.

