# Ready

The final narrow correction closes the omitted-selection construction bypass.
`Population::new` now encodes omitted-means-select-all while preserving the legal
all-selected, empty-dataset, explicit-empty, and explicit-subset cases. No blocking
defect remains in the scope of recheck 025. I recommend owner acceptance of the
frozen T004–T006 candidate, subject to the already documented later clean-Clippy
gate.

## Frozen identity

I recomputed every manifest-listed hash before inspection and after all commands.
All values remained stable:

| Artifact | SHA-256 |
|---|---|
| `docs/dispatches/validator-t004-t006/024-coordinator-final-manifest.md` | `9e9b953930d1d8d334375f3a0ca4f0a22aed5f91eb8cb88cabb32cddc6c1107d` |
| `Cargo.toml` | `981783137668727726799ca99ee963f5333978cd6090ad9953b6d237a8e2c621` |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |
| `src/lib.rs` | `b60b0bf5d4fed5fe6045e7c1039f3c21e315dc46d9bc08159d56d2aa4ab35162` |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` |
| `src/main.rs` | `536e506bb90914c243a12b397b9a998f85ae2cbd9ba02dfd03a9e155ca5ca0f4` |
| `src/model.rs` | `ceffb61ab5e523fa4bf3161d5db5a6fcd1276137d951982da4382edb79c58c66` |
| `src/model/common.rs` | `edfb4bdfe763d43318007d21c9afeb2ef60ad30da561ab1bc3e169f1227eabd5` |
| `src/model/single_label.rs` | `8195f4555b14618b00605d879c8d09cc9564f387421fc1849a3c21c1f977a266` |
| `src/validation.rs` | `4d257a78924bb134c04ad620a90cfc92b107709ed0a2db424476ff01014dfcbe` |
| `src/validation/wire.rs` | `46a42903c2edbb556b1ae1ee7b3c3d9642e562e6b2de2a6d63a852c584f6a3b3` |
| `docs/dispatches/validator-t004-t006/023-developer-to-coordinator.response.md` | `82b98c72f39906a8d14fdd109f163960fd8c9c98ab2b807a556758fef3511de6` |

Only `src/model/common.rs` changed from manifest 019, from
`7892d11b1d16ef6a548ae20eac60177d2d82a80f43d2426ed8b83196c98b5859`
to the hash above. Cargo, parser/numeric code, T004/T005 code, validation entry,
single-label model, public crate wiring, and all other frozen files are unchanged.

## Focused criterion evidence

1. `Population::new` at `src/model/common.rs:882-914` retains its existing
   duplicate, overlap, explicit-list consistency, sorting, digest, and derived
   count checks. The added condition at line 900 rejects an omitted requested-ID
   configuration when `unselected` is nonempty, returning the existing
   `DiagnosticCode::Invariant` path.
2. The owning `population_alignment` test at
   `src/model/common.rs:129-163` proves omitted selection remains legal when all
   IDs are selected and when the dataset is empty. It also directly proves the
   formerly accepted omitted/subset partition now fails.
3. Explicit selection logic is unchanged. The prior actual-entry evidence from
   response 020 remains applicable: explicit `[]` selects none, explicit subset
   selection preserves its sorted selected/unselected partition, and omitted
   actual-entry selection chooses all. The full 24-test suite passes against the
   new guard.
4. `population_alignment` runs all three owning/model/actual-entry tests and
   passes. `validate_before_selection` and `dataset_digest_binding` each run one
   test and pass. The unchanged source hashes plus those regressions preserve UUID
   sorting, digest, exact counts, description, role, parent, duplicate/overlap,
   explicit-selection consistency, semantic-validation ordering, and digest
   binding.
5. The change adds one boolean invariant clause and focused assertions in the
   existing owning test. It adds no visibility, API, type, abstraction, parser or
   numeric behavior, suppression, fallback, placeholder, fake caller, or unrelated
   mutation.

All unchanged Ready evidence from response 020 is reused, including the complete
large-number/marker/null repair and the accepted T004, T005, and remaining T006
criteria.

## Independent command results

| Command | Exit | Result |
|---|---:|---|
| `cargo test --locked --lib population_alignment -- --nocapture` | 0 | 3 passed, 0 failed, 21 filtered. |
| `cargo test --locked --lib validate_before_selection -- --nocapture` | 0 | 1 passed, 0 failed, 23 filtered. |
| `cargo test --locked --lib dataset_digest_binding -- --nocapture` | 0 | 1 passed, 0 failed, 23 filtered. |
| `cargo fmt --all -- --check` | 0 | No formatting differences or output. |
| `cargo test --all-features --locked` | 0 | 24 library tests passed; binary and doc-test targets each ran 0 tests and passed. |
| `cargo build --locked --bin validator` | 0 | Locked binary built; 107 known production dead-code warnings. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Failed solely on the unchanged incomplete-consumer `dead_code` limit classified below. |
| `git diff --check` | 0 | No tracked whitespace error; rustfmt is the substantive formatting check in this unborn/untracked repository. |

## Clippy classification and remaining limit

Clippy did **not** pass and is not waived. Its sole diagnostic class remains
`dead_code`:

- Production library: exactly 107 groups — `src/model.rs` 6,
  `src/model/common.rs` 39, `src/model/single_label.rs` 16,
  `src/validation.rs` 15, and `src/validation/wire.rs` 31.
- Library-test target: exactly 19 unread-field/method groups —
  `src/model/common.rs` 11, `src/model/single_label.rs` 3, and
  `src/validation/wire.rs` 5.

No unused import, other lint class, suppression, fake caller, placeholder, or
fallback appeared. Clean full Clippy remains mandatory at T014 and T017.

## Read-only and execution state

I did not edit implementation, tests, Cargo files, specifications, plans,
tracking, or the artifact index. This response is my only repository write. No
verifier-owned Cargo, Rust compiler, Clippy, or test process remains active. The
task-owned `/usr/bin/caffeinate -i -m` process remains active as PID 84732 and was
not modified. `target/` is approximately 523 MiB and the filesystem has
approximately 16 GiB available; there is no process or resource blocker.
