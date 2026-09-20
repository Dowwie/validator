# T001 frozen candidate manifest 002

Frozen by the coordinator on 2026-09-18 after reading the complete developer
handoff. The sole implementation writer is idle and implementation writes are
paused until the independent verifier returns. This hash-bound snapshot is the
candidate for T001 review; coordinator-only dispatch/index updates are outside it.

## Baseline

The repository has no initial commit. Before T001, the only implementation files
were:

| Path | Baseline SHA-256 | Baseline content summary |
|---|---|---|
| `Cargo.toml` | `d9a39e6b797a354db80ebcb740c7bed96bd962336e8dd220633392d3a73cd7ae` | Bare `validator` 0.1.0 edition-2024 package with no dependencies. |
| `src/main.rs` | `c8e0583694bb1e0188dbe28fe0d65ac1130723c55f968b6262b906c147f72549` | `fn main() { println!("Hello, world!"); }` |

`Cargo.lock`, `rust-toolchain.toml`, `src/lib.rs`, `src/error.rs`, and
`src/model.rs` did not exist. `cargo test --locked` exited 101 because the lockfile
was absent.

## Frozen candidate

| Path | SHA-256 | State |
|---|---|---|
| `Cargo.toml` | `ef2bb8fa026d665fe09f8670e9b15dd27518dffd1f553124857c293cff05b5c2` | Modified |
| `Cargo.lock` | `157fcfe04a5fdff11f1e1770f0e3228a822ff32c008215c987d89c63a2678fa9` | Created |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` | Created |
| `src/main.rs` | `536e506bb90914c243a12b397b9a998f85ae2cbd9ba02dfd03a9e155ca5ca0f4` | Modified |
| `src/lib.rs` | `cad8e175086607a8f194176a738e6a10e1b5fc7372f38679b6952246e23e22df` | Created |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` | Created |
| `src/model.rs` | `a16faa58a7d727ea3894509529564b3906dffa76f32f534d6966413e8dae3cb5` | Created |
| `docs/dispatches/validator-t001/001-developer-to-coordinator.response.md` | `ec7eee2fb7903bf8d228b03e2c661849b0ec1485c8e958f9a314701b45eb8da4` | Developer evidence |

No other implementation, test, schema, fixture, preparation, or acceptance file
belongs to this candidate. `target/` is replaceable build output and is excluded.

## Developer evidence received

The developer reports all exact T001 checks passing on pinned Rust/Cargo 1.98.1
for `x86_64-apple-darwin`: the two named test filters each ran one test, the full
library listed three tests, formatting and Clippy passed without warnings, all
feature tests passed, and locked debug/release binary builds passed. The developer
reported no active process or unresolved failure. The independent verifier must
rerun and assess this evidence; this manifest does not accept the candidate.
