# Revise

The numeric-boundary repair is complete, and the actual `validate_single_label`
path now preserves the required T006 state and rejects mismatched aligned rows.
One material construction bypass remains: the checked `Population` constructor
does not enforce omitted-selection semantics, so crate code can manufacture a
validated evaluation whose configuration says “select all” while its population
selects only a subset. This recheck therefore returns to owner reassessment; no
further repair is authorized by dispatch 020.

## Required finding: omitted selection is not sealed in checked population construction

- **Violated contract:** Evaluation configuration defines omitted `episode_ids`
  as selection of all golden episodes, while explicit `[]` selects none. Finding
  2 and dispatch 020 require the checked configuration to retain this distinction,
  `Population` to enforce a complete partition consistent with the selection,
  and the final evaluation to have no arbitrary-vector construction bypass.
- **Location:** `src/model/common.rs:812-815` correctly exposes omitted selection
  as `None`. `Population::new` at `src/model/common.rs:870-901` checks duplicate
  and overlapping vectors, but its configuration consistency condition at lines
  885-887 runs only when requested IDs are `Some`. When selection is omitted,
  `None.is_some_and(...)` is false regardless of `selected`/`unselected` content.
  `SingleLabelEvaluation::new` at `src/model/single_label.rs:216-242` then checks
  rows against the already accepted population, not against the configuration's
  omitted-means-all rule.
- **Concrete construction path:** all of the following checked values are accepted
  by the frozen source: create `EvaluationConfig::new(..., None)`; create
  `Population::new(digest, config, vec![first], vec![second])`; create one valid
  row for `first`; then create a `SingleLabelEvaluation` with that one row. The
  population has a complete two-ID disjoint partition and the evaluation row
  matches `population.selected`, so every implemented check passes, even though
  omitted selection requires both `first` and `second` to be selected. These
  constructors are crate-visible to later scoring modules.
- **Why existing evidence does not close it:** the actual-entry
  `population_alignment` case correctly proves omitted selection chooses all, and
  the owning negative construction case rejects a row whose ID disagrees with an
  explicitly selected population. Neither case exercises omitted configuration
  with a nonempty unselected partition.
- **Consequence:** the normal validation entry is sound, but the model boundary is
  not sealed against the exact arbitrary construction that finding 2 required it
  to reject. Later crate code can produce a value typed as
  `SingleLabelEvaluation` with selection semantics inconsistent with its owned
  checked configuration.
- **Narrow correction for owner reassessment:** `Population::new` must reject a
  `None` requested selection when `unselected` is nonempty, and the owning
  construction filter must cover that case. This identifies the smallest missing
  invariant only; I did not start or authorize another implementation loop.

## Candidate identity

I recomputed every manifest hash before inspection, after all commands, and before
saving this response. Each value remained equal to repair manifest 019:

| Artifact | SHA-256 |
|---|---|
| `docs/dispatches/validator-t004-t006/019-coordinator-repair-manifest.md` | `2ef9dc6c9c7c8d6c0863c956c291ab52d5e3366f6d566f895b54a1044daad7d2` |
| `Cargo.toml` | `981783137668727726799ca99ee963f5333978cd6090ad9953b6d237a8e2c621` |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |
| `src/lib.rs` | `b60b0bf5d4fed5fe6045e7c1039f3c21e315dc46d9bc08159d56d2aa4ab35162` |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` |
| `src/main.rs` | `536e506bb90914c243a12b397b9a998f85ae2cbd9ba02dfd03a9e155ca5ca0f4` |
| `src/model.rs` | `ceffb61ab5e523fa4bf3161d5db5a6fcd1276137d951982da4382edb79c58c66` |
| `src/model/common.rs` | `7892d11b1d16ef6a548ae20eac60177d2d82a80f43d2426ed8b83196c98b5859` |
| `src/model/single_label.rs` | `8195f4555b14618b00605d879c8d09cc9564f387421fc1849a3c21c1f977a266` |
| `src/validation.rs` | `4d257a78924bb134c04ad620a90cfc92b107709ed0a2db424476ff01014dfcbe` |
| `src/validation/wire.rs` | `46a42903c2edbb556b1ae1ee7b3c3d9642e562e6b2de2a6d63a852c584f6a3b3` |
| `docs/dispatches/validator-t004-t006/014-developer-to-coordinator.response.md` | `ed424418660c82b15331e492f8da8e51eeccdfb290512fb88ac89014275dc0af` |

There is no candidate drift.

## Finding 1 recheck: numeric boundary repaired

The first Revise finding is resolved.

- Cargo enables serde_json `arbitrary_precision`, `raw_value`, and `std` without a
  dependency-set change. The recursive duplicate visitor therefore consumes legal
  arbitrary-range numeric tokens, while `strict_json_keys` still rejects root,
  nested, escaped-equivalent, opaque-configuration, and preparation duplicates.
- `JsonNumber` at `src/validation/wire.rs:302-325` owns a RawValue and accepts only
  a syntactically parsed token whose first byte is a numeric start. The marker
  object begins with `{` and fails as a typed number; legal numeric tokens retain
  their original range until semantic conversion.
- Strict field DTOs at `src/validation/wire.rs:158-212,244-275` avoid internally
  tagged content buffering for observation, categorical-probability, and
  reject-below numeric fields. `deny_unknown_fields`, closed tag enums, and exact
  `value`/`values`/decision-field combinations remain enforced.
- `large_number_boundary` passes through production decode/admission: opaque
  `1e400` is retained in golden input, source configuration, and nested
  preparation configuration; scalar observation `1e400` returns
  `E_OBSERVATION`; reject-below `minimum: 1e400` decodes and the unsupported policy
  returns `E_CONFIG`.
- `json_number_marker_collision`, `typed_optional_nulls`,
  `opaque_number_and_null`, `wire_tags_and_fields`, and `strict_json_keys` all pass.
  Together they preserve marker object identity, omission-versus-null, exact large
  integers, original bytes, legal tags, unknown-field rejection, and number/object
  separation. Semantic conversion remains `parse::<f64>()` plus a finite check at
  `src/validation.rs:350-357`, so scoring arithmetic remains binary64.
- Inspection found one generic `decode` entry after the recursive duplicate pass,
  no reserved-key or range blacklist, alternate decoder, fallback, parser
  framework, permissive route, public API, or lexical-number requirement.

## Finding 2 recheck: repaired except for the required finding

The implementation now constructs `EvaluationConfig<SingleLabelPolicy>` before
selection. It owns a nonblank exact description, role, checked optional parent,
omitted-versus-explicit selection, sorted unique checked requested IDs, and the
only admitted policy, `AsRecorded`.

`Population` now owns the supplied checked digest, complete checked configuration,
derived dataset/selected counts, and sorted selected/unselected vectors. It
rejects duplicates, overlap, and disagreement with an explicit requested list.
The actual validation path supplies the complete expected-ID partition, so its
omitted/all, explicit-empty, and subset results are correct and preserve digest,
counts, description, role, parent, and UUID ordering.

`AlignedRow::new` is fallible and checks prediction identity and target vocabulary
ownership. `SingleLabelEvaluation::new` is fallible and checks row count/order,
row and prediction identity, target vocabulary, and source presence. Its fields
remain private, it owns vocabulary, sources, population/configuration, signal
availability, and rows, and there is no setter, `Deserialize`, builder, trait,
wire-DTO constructor, or SDK export. The sole remaining deficiency is the omitted
configuration construction path documented above.

## Independent command results

All 14 required filters exited 0 and executed nonzero counts. Thirteen filters ran
one passing test with 23 filtered; `population_alignment` ran three passing tests
with 21 filtered:

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

| Command | Exit | Result |
|---|---:|---|
| `cargo test --locked --lib -- --list` | 0 | Listed 24 tests and 0 benchmarks; every required filter appeared. |
| `cargo fmt --all -- --check` | 0 | No formatting differences or output. |
| `cargo test --all-features --locked` | 0 | 24 library tests passed; binary and doc-test targets each ran 0 tests and passed. |
| `cargo build --locked --bin validator` | 0 | Locked binary built; 107 production dead-code warnings. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Failed solely on the disclosed incomplete-consumer `dead_code` limit classified below. |
| `git diff --check` | 0 | No tracked whitespace error; rustfmt is the substantive formatting check in this unborn/untracked repository. |

## Exact Clippy classification and remaining limit

Clippy did **not** pass and is not waived. Its sole diagnostic class is
`dead_code`:

- Production library: exactly 107 groups — `src/model.rs` 6,
  `src/model/common.rs` 39, `src/model/single_label.rs` 16,
  `src/validation.rs` 15, and `src/validation/wire.rs` 31.
- Library-test target: exactly 19 unread-field/method groups —
  `src/model/common.rs` 11, `src/model/single_label.rs` 3, and
  `src/validation/wire.rs` 5.

No `unused_imports`, Clippy style/performance diagnostic, suppression, fake
caller, placeholder, fallback, or other warning class appeared. Searches found no
`allow`/`expect`, `todo!`, `unimplemented!`, builder framework, or new trait.
Clean full Clippy remains mandatory at T014 and T017.

## Execution and read-only state

I did not edit implementation, tests, Cargo files, specifications, plans,
tracking, or the artifact index. This response is my only repository write. No
verifier-owned Cargo, Rust compiler, Clippy, or test process remains active. The
task-owned `/usr/bin/caffeinate -i -m` process remains active as PID 84732 and was
not modified. `target/` is approximately 521 MiB and the filesystem has
approximately 16 GiB available; there is no resource blocker.
