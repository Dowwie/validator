# T001 developer handoff

## Delivered foundation

T001 now provides one Rust 2024 package named `validator`, with one library and
one inert binary target. The library is documented and denies missing public
documentation. Its typed diagnostics preserve stable exit categories without
inspecting messages, serialize as finite valid JSON, and cannot include opaque
payload or credential text through the implemented constructor. Shared wire,
specification, tolerance, and bin-count constants are centralized in `model`.

No CLI command, artifact handling, replay, validation, scoring, generated schema,
future empty module, workspace, async runtime, fallback, or placeholder was added.
The `main` target has no behavior until T014 owns CLI wiring.

Changed or created implementation artifacts:

| Path | Purpose | SHA-256 |
|---|---|---|
| `Cargo.toml` | Rust 2024 package metadata, tested MSRV, minimal runtime dependencies | `ef2bb8fa026d665fe09f8670e9b15dd27518dffd1f553124857c293cff05b5c2` |
| `Cargo.lock` | Exact resolution of the 11 locked packages | `157fcfe04a5fdff11f1e1770f0e3228a822ff32c008215c987d89c63a2678fa9` |
| `rust-toolchain.toml` | Exact `1.98.1` toolchain with `rustfmt` and `clippy` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |
| `src/main.rs` | Minimal binary target with no pre-T014 behavior | `536e506bb90914c243a12b397b9a998f85ae2cbd9ba02dfd03a9e155ca5ca0f4` |
| `src/lib.rs` | Documented private-module library boundary and narrow re-exports | `cad8e175086607a8f194176a738e6a10e1b5fc7372f38679b6952246e23e22df` |
| `src/error.rs` | Typed diagnostic codes, stages, exit categories, safe JSON serialization, and local tests | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` |
| `src/model.rs` | Central version, tolerance, and bin-count constants with a local contract test | `a16faa58a7d727ea3894509529564b3906dffa76f32f534d6966413e8dae3cb5` |

The response file is intentionally not self-hashed; the coordinator freezes its
final hash with the candidate manifest.

## Requirement mapping

| Criteria/source IDs | T001 evidence | Deferred ownership |
|---|---|---|
| V203–V209 | `DiagnosticCode` contains all sixteen specified stable codes. Its typed mapping returns exit category 2, 3, or 4; `Diagnostic` stores code, stage, optional path, affected IDs, and a fixed concise message. `typed_error_categories` checks the explicit mapping without reading a message. `safe_error_serialization` parses each serialized result as JSON and checks both opaque-input and credential sentinels are absent. | CLI process exit handling and actual diagnostic producers are owned by later application, validation, artifact, replay, comparison, and CLI tasks. |
| V231–V235, V249–V254, D061–D063 | `Cargo.toml`, lockfile, and pinned toolchain establish the single binary/library package; `lib.rs` enforces `deny(missing_docs)`; `model.rs` owns the required non-configurable constants. | The listed completed-v1 module layout is created only as each module gains behavior. No module is pre-created here. |
| V197–V202 | The `E_PROVENANCE` typed code and replay stage make the later artifact/replay diagnostic category available. | Evidence copying, exact-byte hashing, deterministic ordering, containment, manifest checking, and replay behavior remain with T011, T012, T015, and T026. |
| V236–V248, D064 | The binary remains separate from private library modules and has no validation, scoring, or artifact logic. | CLI/application wiring; validation, evaluation, comparison, artifacts, and task-specific model modules are owned by their named later tasks. Snapshot ownership and borrowing requirements begin when validated records/evaluators exist. |
| DOD3 | `fmt`, Clippy, locked tests, locked debug build, and locked release build passed on the pinned toolchain. | Integrated source-layout and full public-suite audit remain T027/T035 work. |

## Checks and results

Pinned toolchain and host actually used:

```text
rustc 1.98.1 (48a229cea 2026-09-01)
cargo 1.98.1 (797e8a9bc 2026-08-05)
active toolchain: 1.98.1-x86_64-apple-darwin
host: x86_64-apple-darwin
package.rust-version: 1.98.1
```

`rustup` installed the missing exact host toolchain and its five normal components.
Cargo resolved and locked 11 packages. No unusual multi-gigabyte download,
cleanup, active Cargo/Rust compiler process, warning, or failed check remains.
The task build output is `target/` at approximately 118 MiB.

| Command | Exit | Expected and actual result |
|---|---:|---|
| `cargo test --locked --lib -- --list` | 0 | Listed three library tests, including exactly `error::tests::typed_error_categories` and `error::tests::safe_error_serialization`. |
| `cargo test --locked --lib typed_error_categories -- --nocapture` | 0 | Ran one matching test; passed, two filtered. |
| `cargo test --locked --lib safe_error_serialization -- --nocapture` | 0 | Ran one matching test; passed, two filtered. |
| `cargo fmt --all -- --check` | 0 | No formatting differences. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 0 | Completed without warnings. |
| `cargo test --all-features --locked` | 0 | Three library tests passed; binary and doc-test targets each had zero tests and passed. |
| `cargo build --locked --bin validator` | 0 | Debug binary built. |
| `cargo build --release --locked --bin validator` | 0 | Required release binary built for DOD3. |
| `git diff --check` | 0 | No whitespace errors in tracked diff; this unborn repository's candidate files are untracked. |

The focused JSON test uses `serde_json::Value` to prove each serialized diagnostic
is parseable JSON. It has no numeric error fields, so its emitted values are finite.
The only `expect` calls are test assertions; production code has no `unwrap`,
`expect`, `panic`, `todo`, `unimplemented`, or lint-bypass attribute.

## Dependencies

Runtime dependencies only; there are no development dependencies:

| Dependency | Enabled features | Necessity |
|---|---|---|
| `serde` 1.0.229 | `derive`, `std`; defaults disabled | Derives the fixed machine-readable diagnostic fields. |
| `serde_json` 1.0.151 | `std`; defaults disabled | Produces the required valid JSON diagnostic serialization and verifies it in the local unit test. |

## Input authority hashes

| Path | SHA-256 |
|---|---|
| `AGENTS.md` | `16a210983a47737e371717bbec81f0517cc0a33047a150eb5c2e39012716e070` |
| `docs/README.md` | `2df02b63a619872bc49120943c2b3bb6e3aa6a8b39e87e1acdfd9ba23ba8453e` |
| `docs/artifact-index.md` | `8c88f0575127b5af61ebab09a322431098ffa8c4610a35702c01272792cfa8db` |
| `docs/dev-team/validator-build/charter.md` | `71b5e1498ecd3d6e2cab5221b7503d528e6129e98c1e35256607fde51687413c` |
| `docs/specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `docs/specs/validator-data-model.md` | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| `docs/plans/validator/execution-contract.md` | `373e15389bea05544a18d4006e7b6d958449754f9f37e38172c011e4f46e3009` |
| `docs/plans/validator/tasks/T001.json` | `277fe60a392c678d9b9107e70f8cbfed50264612d51ed214f0a325160ea71453` |
| `docs/plans/validator/coverage.json` | `8753c1f1e84341f2eaa37cfe866d0b621a6d6ba43c4efaf2ee4c2f9060d1d3c1` |
| `docs/dispatches/validator-t001/001-coordinator-to-developer.prompt.md` | `0864ee65a6640b9be02182c702d904eee4ca2e221986d55d016cab445baef892` |
| `/Users/dowwie/.agents/skills/manage-dev-team/SKILL.md` | `e393d952f12a9a17b8a060b9967040902f4ff07a2757ffc6aec753003dc1b850` |
| `/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md` | `e2e93a95c899079e5c3391319f309a0961f4f7cdd882cf7221e2e56da9813a18` |
| `/Users/dowwie/.agents/skills/rust-best-practices/references/chapter_01.md` | `05aed21a95cce7a97badc44f628eb7e23466f7be178a8c8ce8e3c7c1f82fd306` |
| `/Users/dowwie/.agents/skills/rust-best-practices/references/chapter_02.md` | `05c6495150ea1b3a038555231e1965b75dc4c2499e4c7a784a0a35640eaee579` |
| `/Users/dowwie/.agents/skills/rust-best-practices/references/chapter_04.md` | `3257b7dfa06698bd59fe2d315b33b22181926d30cda3776bc16c792b51eeb239` |
| `/Users/dowwie/.agents/skills/rust-best-practices/references/chapter_05.md` | `7b44b4a6e245e692feb07df573162ce8b2c7ffc20716f8a5518d3d4ee2793c7c` |
| `/Users/dowwie/.agents/skills/rust-best-practices/references/chapter_08.md` | `452edcd46db8bf105c9b208ffb9ee41f876ae1a26ea60c699660047861e077c6` |
| `/Users/dowwie/.codex/skills/python-dev/SKILL.md` | `ee31d729219ccb0e73a85356d4058111f6d425084f9e7228297eac2cddac5cf4` |
| `/Users/dowwie/.codex/skills/python-dev/references/workflow.md` | `f4b6d28af717375ae9fe191d1a7934c1ab3d52a82d6d35eaf9e460411a90cf82` |

No independent verification or T001 acceptance is claimed. The coordinator must
freeze this candidate, update the required operational/index records, and obtain
the independent verifier's verdict.
