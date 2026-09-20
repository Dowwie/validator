# Validator T001 verifier dispatch 003

Role/model: verifier, `gpt-5.6-sol`, reasoning `high`, context inheritance remains
the previously provisioned verifier session.
Repository: `/Users/dowwie/MyProjects/validator`.
Coordinator: `/root/coordinator`, Sol high. Required full response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t001/003-verifier-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator’s verified single-label backbone](http://localhost:3006/1/cards/183).

## Outcome and review boundary

Independently verify only frozen T001: package/toolchain foundation, typed errors,
and shared constants. Return exactly one verdict for this candidate: **Ready**,
**Revise**, or **Blocked**. Do not review T002+ behavior, demand future wiring,
expand into a repository-wide audit, edit implementation/tests/fixtures, or design
optional improvements. Interim lack of later modules is intentional. No worker
delegation or subagents.

Read the exact T001 contract and controlling sources before the implementation
summary:

- `docs/plans/validator/tasks/T001.json` and
  `docs/plans/validator/execution-contract.md`.
- `docs/specs/validator-v1.md`, especially Evidence bindings and replay, Package
  and source layout, and Rust and dependency conventions.
- `docs/specs/validator-data-model.md`, especially Source organization and ownership.
- `docs/dev-team/validator-build/charter.md` and repository `AGENTS.md`.
- The original assignment:
  `docs/dispatches/validator-t001/001-coordinator-to-developer.prompt.md`.
- The frozen manifest:
  `docs/dispatches/validator-t001/002-coordinator-candidate-manifest.md`, SHA-256
  `a12e570a1299b86d6793b177301bd287979ae824e84e8b5e739796ee03e64174`.
- Then read the developer's full handoff:
  `docs/dispatches/validator-t001/001-developer-to-coordinator.response.md`,
  SHA-256
  `ec7eee2fb7903bf8d228b03e2c661849b0ec1485c8e958f9a314701b45eb8da4`.

The implementation writer is idle. The following exact implementation hashes are
frozen and must remain unchanged during review:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `ef2bb8fa026d665fe09f8670e9b15dd27518dffd1f553124857c293cff05b5c2` |
| `Cargo.lock` | `157fcfe04a5fdff11f1e1770f0e3228a822ff32c008215c987d89c63a2678fa9` |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |
| `src/main.rs` | `536e506bb90914c243a12b397b9a998f85ae2cbd9ba02dfd03a9e155ca5ca0f4` |
| `src/lib.rs` | `cad8e175086607a8f194176a738e6a10e1b5fc7372f38679b6952246e23e22df` |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` |
| `src/model.rs` | `a16faa58a7d727ea3894509529564b3906dffa76f32f534d6966413e8dae3cb5` |

If any implementation hash changes, stop and return Blocked with the changed path.
Coordinator dispatch/index files may change and are outside this candidate.

## Required independent review

Inspect the complete scoped candidate and the baseline preimages recorded in the
manifest. Verify each T001 criterion and relevant source requirement, including:

- One Rust 2024 package named `validator`, one library and one binary, no workspace,
  async runtime, extra package, future empty module, placeholder, or pre-T014 CLI.
- Exact stable toolchain with rustfmt/Clippy, lockfile, minimal necessary dependency
  features, and `package.rust-version` actually tested on that version.
- `#![deny(missing_docs)]`, documented public boundary, ordinary typed structures,
  and no production panic/unwrap/expect or bypass attribute.
- Every stable error code and its typed 2/3/4 classification without message
  parsing; safe structured finite JSON that cannot leak supplied sentinel payload
  or credential text.
- Central, non-configurable wire 2, specification `1.2-draft`, categorical `1e-9`,
  fixture absolute `1e-12`, fixture relative `1e-10`, and ten-bin constants.
- T001's portions of V197–V209, V231–V254, D061–D064, and DOD3. Do not report
  behavior explicitly owned by later tasks as a T001 defect.

Run at minimum and record exact exits/test counts:

```sh
cargo test --locked --lib -- --list
cargo test --locked --lib typed_error_categories -- --nocapture
cargo test --locked --lib safe_error_serialization -- --nocapture
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --locked --bin validator
```

The release build already passed developer-side and is mandatory at T017; rerun it
only if needed to resolve a concrete T001 uncertainty. Reuse valid unchanged
evidence otherwise. Confirm each named filter executes a nonzero expected count.

## Response contract

Save the complete response at the required path before returning. Include:

- The single candidate verdict at the top.
- Recomputed manifest, implementation, and developer-response hashes.
- Each T001 acceptance criterion/source group with inspected implementation and
  independent command/review evidence.
- Exact commands, exits, test counts, expected/actual results, warnings, and any
  checks consciously reused.
- If Revise: each finding must name the violated requirement, exact source location,
  reproduction, consequence, and smallest required correction. Optional elegance
  or later-task wiring is not a finding.
- Remaining limits, active processes, and confirmation that no implementation,
  test, fixture, schema, or acceptance artifact was edited.

Do not alter the frozen files. After saving the response, return only its absolute
path to the coordinator.
