# T004–T006 omitted-selection correction handoff

This final narrow correction is complete. It is implementation evidence only and
does not claim verifier or owner acceptance.

## Baseline and changed artifact

The required baselines matched before editing:

| Artifact | SHA-256 |
|---|---|
| `019-coordinator-repair-manifest.md` | `2ef9dc6c9c7c8d6c0863c956c291ab52d5e3366f6d566f895b54a1044daad7d2` |
| `020-verifier-to-coordinator.response.md` | `4a121340e640d12fb0a3c44e7dac6afd82ddbf16af244729c4647ab0d46423c6` |
| `src/model/common.rs` input | `7892d11b1d16ef6a548ae20eac60177d2d82a80f43d2426ed8b83196c98b5859` |
| `src/model/common.rs` output | `edfb4bdfe763d43318007d21c9afeb2ef60ad30da561ab1bc3e169f1227eabd5` |

Only `src/model/common.rs` and this response were written. `Population::new`
now rejects the existing invariant error when the checked configuration omits
`requested_episode_ids` but the partition contains an unselected ID. This seals
the omitted-means-select-all branch while retaining the existing explicit-list,
duplicate, overlap, sort, digest, and count checks.

The extended owning `population_alignment` test proves:

- omitted selection with selected `[first]` and unselected `[second]` rejects;
- omitted selection with all IDs selected remains legal;
- omitted selection over the empty dataset remains legal;
- existing explicit selection, nonblank configuration, and count/order checks
  remain covered.

The following source identities remained unchanged from manifest 019:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `981783137668727726799ca99ee963f5333978cd6090ad9953b6d237a8e2c621` |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |
| `src/lib.rs` | `b60b0bf5d4fed5fe604e7c1039f3c21e315dc46d9bc08159d56d2aa4ab35162` |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` |
| `src/main.rs` | `536e506bb90914c243a12b397b9a998f85ae2cbd9ba02dfd03a9e155ca5ca0f4` |
| `src/model.rs` | `ceffb61ab5e523fa4bf3161d5db5a6fcd1276137d951982da4382edb79c58c66` |
| `src/model/single_label.rs` | `8195f4555b14618b00605d879c8d09cc9564f387421fc1849a3c21c1f977a266` |
| `src/validation.rs` | `4d257a78924bb134c04ad620a90cfc92b107709ed0a2db424476ff01014dfcbe` |
| `src/validation/wire.rs` | `46a42903c2edbb556b1ae1ee7b3c3d9642e562e6b2de2a6d63a852c584f6a3b3` |

Numeric, parser, T004, and T005 evidence is unchanged and was not rerun beyond
the required combined test gate.

## Verification

| Command | Exit | Executed/result |
|---|---:|---|
| `cargo test --locked --lib population_alignment -- --nocapture` | 0 | 3 passed, 21 filtered |
| `cargo test --locked --lib validate_before_selection -- --nocapture` | 0 | 1 passed, 23 filtered |
| `cargo test --locked --lib dataset_digest_binding -- --nocapture` | 0 | 1 passed, 23 filtered |
| `cargo fmt --all -- --check` | 0 | clean |
| `cargo test --all-features --locked` | 0 | 24 library tests passed; binary and doc targets ran 0 and passed |
| `cargo build --locked --bin validator` | 0 | built; 107 known dead-code warnings |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | known incomplete-consumer `dead_code` only |
| `git diff --check` | 0 | clean |

Clippy emitted no class other than `dead_code`: 107 production groups
(`src/model.rs` 6, `src/model/common.rs` 39, `src/model/single_label.rs` 16,
`src/validation.rs` 15, `src/validation/wire.rs` 31) and 19 test-target groups
(`src/model/common.rs` 11, `src/model/single_label.rs` 3,
`src/validation/wire.rs` 5). No suppression, fake caller, placeholder, or
fallback was added.

Task-owned `caffeinate` remains PID 84732 (`/usr/bin/caffeinate -i -m`). No
Cargo, compiler, test, or Clippy process remains. The workspace has about 17 GiB
free. The repository remains unborn/untracked, so `git diff --check` verifies
tracked diff whitespace only.
