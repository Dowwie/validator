# T027 output1 structural-integration handoff

Status: complete local candidate. This is implementation evidence only and is
not independent acceptance.

## Scope and synchronization

I read T027, owner prompt001, the linked specification and data-model sections,
the execution contract, the build-role instructions, and prompts002/003/005/006.
Before editing I sent the required per-diagnostic routing report. Prompt003
synchronized the initial six source files. A first coherent compile then exposed
that the checked `Episode` input had no genuine consumer because inspection
independently re-decoded the golden snapshot. Prompt005 synchronized the smallest
`src/app.rs` disclosure-boundary consumer. Prompt006 synchronized the six
previously masked conformance-test Clippy corrections.

I did not edit governance, plans, schemas, fixtures, Cargo inputs, T028, Chord,
or output2 artifacts. `docs/artifact-index.md` was not changed because the active
dispatch explicitly excluded index/governance writes; this required response is
the prescribed dispatch artifact.

## Diagnostic disposition and real flows

The original warning-denied Clippy command reported 17 production groups and
three duplicate lib-test schema-marker groups.

| Original group | Disposition and actual consumer |
|---|---|
| `TaskDefinition`; `SingleLabelTask`; `MultiLabelTask`; their construction and vocabulary methods | `validation` now constructs the concrete closed task definition from the decoded wire task before target, output, policy, and row admission. It borrows the resulting checked vocabulary for task-specific evaluation construction. The obsolete `is_single_label` helper was removed; exhaustive closed enum matching remains the real dispatch. |
| `Episode` and its constructor/accessors | Each decoded golden row now becomes one `Episode<Target>` containing its checked ID, exact raw opaque input, and task-specific target. The aligned task rows own that episode and evaluators borrow only its ID/target. `app::inspect` finds the selected validated row after replay verification and clones the raw input only into the owned public disclosure result. Metrics and comparisons never receive the payload. |
| `LabelSet::{is_empty,contains}` | Removed as test-only conveniences. Existing unit assertions inspect the established ordered-label iterator directly; production set validation and vocabulary ownership are unchanged. |
| `ObservationSet::values` | Retained only under `cfg(test)` because the only caller is a same-crate local assertion. Production uses `report_values` to serialize retained observations. |
| specialized `EvaluationConfig<SingleLabelPolicy>::new` | Retained only under `cfg(test)` for local default-policy setup. Production continues through the checked generic `new_with_policy` path from validation. |
| `MetricResult::ratio` and five inspection getters | Retained only under `cfg(test)` for existing local numerical assertions. The real evaluator constructors, serializer, and comparison behavior remain production paths. |
| `signal_availability` | Removed as an obsolete test wrapper. Actual admission uses `signal_availability_flags` before predictions move into checked rows; its unit test now exercises that real function. |
| three `wire::schema_version: ()` fields | Replaced with concrete private `u8` fields. Each strict `decode_*` entry reads it and invokes `wire::require_schema_version_two` before exposing its decoded value. Duplicate-key rejection, explicit tags, unknown-field rejection, and opaque raw DTO fields are unchanged. |
| six `tests/conformance.rs` diagnostics revealed after the library test target compiled | Replaced repeated mutation/failure/report-row tuple forms with test-local aliases and accepted `&Path` slices where callers only need path operations. Assertions, fixtures, oracles, and test names were not changed. |

The validation path is now exact bytes in `InputArtifacts` -> strict private DTO
decode -> concrete checked task/wrapper -> exactly one task-specific opaque-input
`Episode` per golden row -> checked aligned evaluation -> closed evaluator -> typed
report. Inspection verifies replay and report identity before it reads the selected
validated episode input. This preserves the existing raw snapshot for immutable
artifact evidence and preserves the exact raw input value returned by inspection.

## Source changes and identities

Starting writer identities from the pre-edit report were:

| File | Starting SHA-256 | Final SHA-256 |
|---|---|---|
| `src/model.rs` | `846677319091911cd5f1a9a869aa45700b8ce46caf0923e779f74100c71dd4f6` | `992adf37259839f71dbffa931ac1a341894e90f29283bf34f0f393f1a762ce56` |
| `src/model/common.rs` | `5b91a1aaad30127f63030d885338e8d54d807d5682cf3c077525102167c0e9cc` | `f5f626654b33c63255de26c0e4a07ba79696b980042fa240e2741d73e54bfc8c` |
| `src/model/single_label.rs` | `4ec2c89ce43f30a79bdb8054c106608257613491415710e56f3139cfbf748ea0` | `1acd5af54ef01b2cc49d6e8c9c7a973ee04e529e4000e4cf0b2edf8fc1a10222` |
| `src/model/multi_label.rs` | `3f515a0a9122db963b2e01ca14d5474b4836ba3543a48a4e0766a4459eea0b7c` | `43e80100dd9a5c59232ac48fe1925c6bd1d4f2192b86b492027052f8776bd873` |
| `src/validation.rs` | `fb6148069b4ade27eab57917e0111211274f3aa31c45b4458d3fa1b76a5482a0` | `1ae4707c48c2310e45340ccbc24109eee616d74eaf03a664c247905ba840ebbb` |
| `src/validation/wire.rs` | `3c98ed9c193cb8dff0f889956f58a4198b4d8a956a3da34592b45e5e381d29ad` | `1a6d6887dc3ed2fd1a02c337592cf43dd97b59c2dcf6f361459e396634f27ee4` |

Prompt005 added the necessary smallest caller file:

| File | Final SHA-256 |
|---|---|
| `src/app.rs` | `f741c03b17788303c7ae79ff43a11f9605cb63a6bc565a98d40f5f55b131d64b` |

Prompt006 added the previously masked, mechanical conformance-test cleanup:

| File | Final SHA-256 |
|---|---|
| `tests/conformance.rs` | `d0be253475cc718ad7ed67d50e9096b7dc2a4a5a1ddccdb096662e06d4f05b00` |

No application API, CLI behavior, numerical result, tolerance, report schema,
publication implementation, comparison implementation, artifact implementation,
fixture, or Cargo source was changed. The app change is private inspection wiring:
it replaces only the independent input re-decode with the existing validated-row
input; raw expected/prediction fields retain their established snapshot path.

## Compile and behavior evidence

The first coherent `cargo check --all-features --locked` after the initial
integration failed with three E0282 errors in the new decode wrappers because
Rust could not infer `Decoded<T>` before the schema-version field read. The
smallest correction was three concrete `Decoded<wire::...>` annotations. The
next check compiled and isolated the real missing `Episode` input consumer.
After prompt005's inspection consumer, `cargo check --all-features --locked`
exited 0 with no diagnostics.

The full test run preserved all existing counts:

- 44 library tests passed.
- 0 binary tests passed.
- 11 CLI tests passed.
- 91 conformance tests passed.
- 0 doc tests passed.

The test inventory still lists the same 44 library, 11 CLI, 91 conformance, and
0 documentation tests. The focused existing behavior exercised by the full suite
includes raw opaque number/null handling, parser-marker rejection, both task
admissions, S27/S28 privacy and inspection behavior, relocation/replay, and both
task CLI routes. No DM01-DM12 or `offline_cli_contract` filter was added; output2
retains those proof obligations.

## Required output1 gates

| Command | Exit | Result |
|---|---:|---|
| `cargo fmt --all -- --check` | 0 | Clean. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 0 | Clean; no diagnostic allowance remains. |
| `cargo test --all-features --locked` | 0 | 44 library, 0 binary, 11 CLI, 91 conformance, and 0 doc tests passed. |
| `cargo build --release --locked --bin validator` | 0 | Release binary built. |
| `git diff --check` | 0 | Clean. |
| `ruby docs/plans/validator/verify-plan.rb` | 0 | 35 task contracts, 433 source blocks, 63 conformance cases, 12 structure checks, 8 ACs, and 5 DoD clauses verified. |

The final Clippy command has no output other than normal compilation completion.
No unresolved limit or material contract decision remains for output1.
