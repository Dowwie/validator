# Ready

The repaired candidate closes the sole T002 defect from verdict 005. The private
strict prediction boundary now accepts the complete legal retained-observation
wire shape while leaving all observation semantics to T004/T005. The candidate is
Ready within the explicitly accepted incomplete-consumer `dead_code` limit; this
is not a clean-Clippy claim or waiver.

## Candidate identity

I recomputed the repaired manifest, repair response, and every frozen candidate
hash before and after the checks. All values match manifest 007:

| Artifact | Recomputed SHA-256 |
|---|---|
| `docs/dispatches/validator-t002/007-coordinator-repair-manifest.md` | `3e0c155c1a64e6ec75d8bf518400fcc3c12d283ec3774de705833eb912843e91` |
| `docs/dispatches/validator-t002/006-developer-to-coordinator.response.md` | `88cea4d46bc9cf14ed19766d0077d55ae03ac7f58b9a13ea5445b138404255ee` |
| `Cargo.toml` | `8c758ad6246244317ac02156077b417eab3cd49994daf2597c65ae86bb24629c` |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |
| `src/lib.rs` | `b60b0bf5d4fed5fe6045e7c1039f3c21e315dc46d9bc08159d56d2aa4ab35162` |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` |
| `src/model.rs` | `3275697520566394a2a20d4011ad28b3912d9d4773a44203980127d3f2d4d4d1` |
| `src/model/common.rs` | `ee0b0169d88707663c7b1540d1406868ba61cb0db2fd4d36dd2728d3baf9cecf` |
| `src/validation.rs` | `6c6c71080372b1d5b0a1bf707b88e1db694d88a142a3f2d94b8a467ff0020f72` |
| `src/validation/wire.rs` | `e81c528e0659e1fb4b2364be82639bbb27175947b042145a0f314ec00e827d17` |

The two repaired source hashes are exactly the values bound by dispatch 008. All
other source, dependency, lock, and toolchain hashes remain the manifest values;
there is no drift.

## Focused repair evidence

### Complete legal observation shape

- `src/validation/wire.rs:58-65` adds only optional
  `Source.observation_definitions`, represented by an exact string-keyed
  `BTreeMap<String, ObservationDefinition>`.
- `src/validation/wire.rs:75-91` makes each definition a strict
  `deny_unknown_fields` object with required `kind`, required `description`, and
  optional `question_id`. `ObservationKind` is a closed snake-case enum containing
  exactly `scalar`, `bernoulli`, `reported_confidence`, `categorical`, and
  `label_marginals`.
- `src/validation/wire.rs:102-138` adds only optional
  `Prediction.observations`, also an exact string-keyed map. Its explicitly tagged,
  `deny_unknown_fields` enum has the same five kinds. Scalar, Bernoulli, and
  reported-confidence variants require a `JsonNumber` `value`; categorical and
  label-marginal variants require `BTreeMap<String, JsonNumber>` `values`.
- The existing `decode_prediction_artifact` entry still performs recursive
  duplicate detection and then deserializes the strict DTO. No second decode path
  was introduced. The positive `PREDICTIONS` fixture at
  `src/validation.rs:69-100` supplies definitions and row values for all five
  kinds, and the focused test decodes it successfully.

### Rejection behavior and semantic boundary

- The focused test executes rejection of an unknown observation-definition kind
  and an extra row-observation field. The closed `ObservationKind` and
  `Observation` enums reject unknown definition and row tags, respectively, and
  `deny_unknown_fields` rejects surplus definition or row fields.
- Every scalar-like `value` and every categorical/label-marginal map value uses
  the existing `JsonNumber` deserializer at `src/validation/wire.rs:206-218`. It
  accepts only a JSON number, so strings, booleans, null, arrays, and objects fail
  as wrong primitives. Map containers are also statically required for both
  vector-shaped variants.
- No nonblank, finiteness, range, nonempty-vector, source-definition matching,
  scoring-vocabulary binding, sum, completeness, or observation-promotion logic
  appears in the repaired DTOs. Names and descriptions remain plain strings;
  numeric values remain deferred `JsonNumber`s; maps are not semantically
  inspected. T004/T005 therefore retain their assigned checks.

### Scope and regressions

The repair did not change `src/lib.rs`, so `validation` remains private and no
public API was added. Cargo manifests and lockfile are unchanged. Searches found
no `allow`/`expect` suppression, fake production caller, alternate prediction
deserializer, fixture file, or unrelated source mutation. The only production
JSON entry remains the generic strict `serde_json::from_slice` path after the
recursive duplicate visitor. The prior verdict's duplicate, exact-number,
original-byte, privacy, and dependency evidence remains applicable, and the two
unchanged focused regression tests pass.

## Independent commands

Pinned Rust/Cargo 1.98.1 on `x86_64-apple-darwin` produced:

| Command | Exit | Result |
|---|---:|---|
| `cargo test --locked --lib wire_tags_and_fields -- --nocapture` | 0 | 1 passed, 0 failed, 8 filtered; 21 expected lib-test dead-field warnings. |
| `cargo test --locked --lib strict_json_keys -- --nocapture` | 0 | 1 passed, 0 failed, 8 filtered; 21 expected lib-test dead-field warnings. |
| `cargo test --locked --lib opaque_number_and_null -- --nocapture` | 0 | 1 passed, 0 failed, 8 filtered; 21 expected lib-test dead-field warnings. |
| `cargo fmt --all -- --check` | 0 | No formatting differences or output. |
| `cargo test --all-features --locked` | 0 | 9 library tests passed; binary and doc-test targets each ran 0 tests and passed. |
| `cargo build --locked --bin validator` | 0 | Locked binary built successfully. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Failed only on the accepted incomplete-consumer `dead_code` diagnostics classified below. |
| `git diff --check` | 0 | No tracked whitespace error; the repository remains unborn/untracked, so rustfmt is the substantive format check. |

I also ran the broader `cargo test --locked --all-targets` and
`cargo build --locked --all-targets`; both exited 0. They add no qualification to
the required-command results.

## Exact Clippy classification and remaining limit

Clippy did **not** pass. Its sole diagnostic class is `dead_code`:

- The production library reports 55 groups: the 24 accepted T003 foundation
  groups and 31 T002 groups. The T002 set is the 28 groups recorded in verdict
  005 plus the new private `ObservationDefinition`, `ObservationKind`, and
  `Observation` types.
- The library-test target reports 21 unread-field groups: the 15 groups recorded
  in verdict 005 plus the `ObservationDefinition` field group and the five
  observation-variant payload groups.
- No `unused_imports`, Clippy style/performance diagnostic, semantic diagnostic,
  suppression, or fake-consumer diagnostic appeared.

These diagnostics are the owner-authorized incomplete-consumer limit. They remain
unresolved, and clean full Clippy is still required at T014/T017.

## Execution and read-only state

I did not edit implementation, tests, fixtures, manifests, lockfiles,
specifications, plans, tracking records, or the artifact index. This response is
my only source-tree write; Cargo generated only replaceable `target/` artifacts.
No verifier-owned Cargo, Rust compiler, or Clippy process remains active. `target/`
is approximately 227 MiB, and the filesystem has approximately 17 GiB available;
there is no process or resource blocker.
