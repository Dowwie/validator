# Validator T001 developer dispatch 001

Role/model: developer, `gpt-5.6-terra`, reasoning `high`, context inheritance `none`.
Repository: `/Users/dowwie/MyProjects/validator`.
Coordinator: `/root/coordinator`, Sol high. Required full response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t001/001-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator’s verified single-label backbone](http://localhost:3006/1/cards/183).

## Outcome and authority

Implement T001 only: the single Rust package compiles with reproducible tools and
typed errors whose CLI exit categories do not depend on parsing message text.
You are the sole implementation and test writer. Do not delegate or spawn agents.
Do not implement T002 or later behavior, add empty future modules, create CLI
commands, or change product scope.

Read these sources before editing:

- `/Users/dowwie/.agents/skills/manage-dev-team/SKILL.md` in full, observing the
  developer role.
- `/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md` and every chapter it
  identifies as relevant to this implementation, especially coding idioms, linting,
  error handling, testing, and documentation.
- Repository `AGENTS.md`, `docs/README.md`, and `docs/artifact-index.md`.
- `docs/dev-team/validator-build/charter.md`.
- `docs/specs/validator-v1.md`, especially Evidence bindings and replay, Package
  and source layout, and Rust and dependency conventions.
- `docs/specs/validator-data-model.md`, especially Source organization and ownership.
- `docs/plans/validator/execution-contract.md` and
  `docs/plans/validator/tasks/T001.json`. The specifications override summaries.
- Coverage entries V197–V209, V231–V254, D061–D064, and DOD3 in
  `docs/plans/validator/coverage.json`. Implement only the T001-owned foundation;
  later tasks own artifact/replay behavior and the rest of the named modules.

## Baseline and dependencies

T001 has no implementation dependency. The repository is on an unborn `main`
branch and all initial files are untracked. Preserve every unrelated file and
`.zvec-grep`; do not stage, commit, clean, reset, or create a new Git baseline.

The initial implementation baseline is:

- `Cargo.toml` SHA-256
  `d9a39e6b797a354db80ebcb740c7bed96bd962336e8dd220633392d3a73cd7ae`
- `src/main.rs` SHA-256
  `c8e0583694bb1e0188dbe28fe0d65ac1130723c55f968b6262b906c147f72549`
- `docs/plans/validator/tasks/T001.json` SHA-256
  `277fe60a392c678d9b9107e70f8cbfed50264612d51ed214f0a325160ea71453`
- Host compiler: `rustc 1.98.1 (48a229cea 2026-09-01)`,
  `x86_64-apple-darwin`; Cargo 1.98.1.
- Free disk observed before dispatch: about 18 GiB.
- `cargo test --locked` currently exits nonzero because `Cargo.lock` does not exist.
  This is expected baseline evidence, not a regression.

Normal Rust dependency resolution and locked builds are authorized. Stop and
report before any unusual multi-gigabyte download or cleanup.

## Exact write scope and interfaces

You may create or modify only the T001 implementation artifacts and your response:

- `Cargo.toml`: one package named `validator`, edition 2024, tested MSRV, minimal
  runtime/test dependencies with only needed features.
- `Cargo.lock`: exact dependency resolution.
- `rust-toolchain.toml`: exact stable compiler plus `rustfmt` and `clippy`.
- `src/lib.rs`: documented library boundary with `#![deny(missing_docs)]`; no
  business logic or future empty modules.
- `src/error.rs`: typed stable diagnostic codes, stage/path/affected IDs and safe
  message; stable exit categories 2/3/4; finite valid JSON serialization; never
  embed opaque payloads or credentials.
- `src/model.rs`: central constants for wire version 2, specification version
  `1.2-draft`, categorical sum tolerance `1e-9`, fixture comparison tolerances
  (`1e-12` absolute and `1e-10` relative), and ten-bin convention.
- `src/main.rs` only if the minimum T001 package/library wiring requires a change.
  Do not add CLI behavior before T014.
- `/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t001/001-developer-to-coordinator.response.md`.

Do not edit specifications, plans, the artifact index, Fizzy, session notes,
schemas, fixtures, acceptance records, or any T002+ production/test artifact.
The coordinator owns dispatch indexing and operational state.

Use ordinary structs/enums and typed `Result` errors. No production panic,
`unwrap`, `expect`, bypass attribute, async runtime, multi-package workspace,
generic plugin/task framework, or dependency added for hypothetical future work.
Keep items private or `pub(crate)` unless the small library boundary requires an
export. Comments explain non-obvious reasons only.

## Acceptance and checks

Implement local tests named exactly:

1. `typed_error_categories`: every stable code named by the specification maps to
   exit 2, 3, or 4 without message parsing. Valid success remains exit 0 at the CLI
   contract level; do not invent a success error.
2. `safe_error_serialization`: serialization is valid finite JSON and sentinel
   opaque-input/credential strings do not appear in error output.

Acceptance requires:

- The exact toolchain and actual host are recorded, and `package.rust-version` is
  the minimum compiler version you actually test. Do not claim an untested MSRV.
- All specified error codes have typed exit classifications and safe structured
  fields (`code`, `stage`, optional `path`, affected IDs, concise `message`).
- Shared constants match the ratified specification and stay non-configurable.
- The package compiles as one binary plus one library with no placeholder module.
- No prohibited constructs or extra packages are present.

Run and record exact exit results for at least:

```sh
cargo test --locked --lib -- --list
cargo test --locked --lib typed_error_categories -- --nocapture
cargo test --locked --lib safe_error_serialization -- --nocapture
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --locked --bin validator
```

Confirm each named filter executes a nonzero expected test count. Use the pinned
toolchain for final evidence. If a dependency or MSRV choice cannot be verified,
return the exact blocker instead of claiming completion.

## Handoff

Before returning, save the complete response at the required path. Include:

- Delivered behavior and every changed/created path.
- A criterion/source-ID map to implementation and tests, distinguishing T001's
  foundation from behavior deferred to its owning later task.
- Exact commands, exits, and relevant expected/actual results, including named
  test counts, compiler/toolchain/host, and any warnings.
- SHA-256 for every changed/created implementation file, lockfile, toolchain file,
  and this response's input authority files; note that the response file's own
  final hash will be frozen by the coordinator.
- Dependencies and enabled features with a short necessity statement.
- Failures, unproven limits, active processes, and observed resource surprises.

Do not claim independent verification or T001 acceptance. Return only after the
response file is complete, then send its absolute path to the coordinator.
