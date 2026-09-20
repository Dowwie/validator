# Validator T011 atomic artifact-loader dispatch 023

Role/model: retained sole developer `/root/coordinator/schema_test_developer`,
`gpt-5.6-terra`, reasoning `high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/023-schema-test-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator's verified single-label backbone](http://localhost:3006/1/cards/183).

T010 is reconciled as a complete local milestone. Complete **T011 only** next.
This remains inside the combined T007-T014 candidate; there is no independent
review or acceptance at this unit. You are the sole implementation/test writer.
Do not delegate or begin T012.

## Required context and input identity

Read in full:

- `/Users/dowwie/.codex/AGENTS.md`, repository `AGENTS.md`, and the already-read
  manage-dev-team and Rust best-practices skills;
- `docs/plans/validator/tasks/T011.json`;
- `docs/specs/validator-v1.md` sections **CLI and run artifacts** and **Evidence
  bindings and replay**;
- `docs/specs/validator-data-model.md` sections **Reports and metric reuse** and
  **Source organization and ownership** only as needed for typed placement.

The final T010 response is
`docs/dispatches/validator-t007-t014/022-schema-test-developer-to-coordinator.response.md`,
SHA-256 `265514cda64a788de7a56f6570eda506ad1ce8081f659a49202f8ad43b1cd03c`.
Confirm these exact relevant inputs before editing:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `e257b5d70c78f0dd7465093aad8d092db99c35ffa28227298271512faf49bec6` |
| `Cargo.lock` | `86caa4cd0470cc4d1f8d4d8dedbdfccb1c43e42b9f0a5a119343e35b256dd74e` |
| `src/lib.rs` | `bd89b6b7148e8ef25e64d28b465218a1f4afd90b8f95e6f92ed3a510de69aade` |
| `src/model/common.rs` | `a9f823207e1fd1aaa6181b5594e5d2a8ac4df150216b7299901b52c98a3ae63b` |
| `src/model.rs` | use the current file unchanged unless module wiring proves necessary |
| `src/validation.rs` | accepted T006/T002 state; read-only for this unit |
| `tests/conformance.rs` | `99b0a0eda61a8294a8f3489c019eb3827f919e36abb296ba83c77644176e18e7` |

Preserve the 33-test library baseline and the passing T010 conformance filter. The
repository is unborn/untracked. Do not stage, commit, clean, reset, modify
`.zvec-grep`, edit governance/index/Fizzy/session records, or start/stop task-owned
caffeinate PID 84732.

## Exact outcome

Implement filesystem ingestion only in `src/artifacts.rs` with shared typed
records in `src/model/common.rs`.

- Read the golden, predictions, and config input paths exactly once each. Retain
  their exact bytes, exact original path identity needed by the caller, fixed
  stored destinations `golden.json`, `predictions.json`, and `config.json`, and an
  exact lowercase SHA-256 digest. Return bytes and digests for later validation;
  do not parse, rewrite, normalize, or add filesystem access to validation.
- Load evidence only after checked source definitions are available. Resolve each
  new-submission evidence string relative to the prediction artifact's parent;
  parent-relative paths are allowed. Missing or unreadable files return `E_IO`.
- Enumerate every declared source, including unused sources, by source ID UTF-8
  bytes; preserve each source's evidence array order. Assign consecutive global
  ordinals from zero and stored paths `evidence/n.bin` using unpadded decimal `n`.
- Read and retain every evidence entry separately, even when paths repeat or
  different directories share a basename. Each binding carries source ID,
  zero-based source-local index, the exact original path string, deterministic
  stored relative path, exact bytes, and SHA-256 digest.
- Original paths are used only for new-submission ingestion and remain provenance
  strings. Do not add replay dereferencing, containment/symlink replay checks,
  directory publication, overwrite handling, or metrics; those belong to
  T012/T015.

Use ordinary concrete records and typed `Result<_, Diagnostic>`. Production code
must not use `unwrap`, `expect`, panic recovery, fallback modes, or warning
suppression. Keep implementation functions under cyclomatic complexity 10 and
factor repeated exact-file loading/hash logic once.

## Allowed writes

- create `src/artifacts.rs`;
- `src/model/common.rs` for snapshot manifest/evidence binding and loaded-byte
  records, plus only the accessors needed to borrow exact source evidence strings;
- `src/lib.rs` solely for the private parent module declaration required to compile;
- `Cargo.toml` and `Cargo.lock` solely for one established minimal SHA-256 runtime
  crate with only needed features, and a test-only temporary-directory dependency
  only if the standard library cannot keep the owning tests hermetic;
- the required response file.

Do not modify validation/evaluation/scoring, schemas, conformance tests, existing
fixtures, app/CLI/report/publication code, specifications/plans/session/index/Fizzy,
multi-label code, or T012+ behavior. Do not expose a public SDK surface merely to
support integration tests; T011's named tests are owning library tests.

## Required owning tests

Implement and run the exact nonzero filters:

```sh
cargo test --locked --lib evidence_ordinal_binding -- --nocapture
cargo test --locked --lib artifact_exact_bytes -- --nocapture
```

The tests must independently hash known bytes and prove:

- canonical snapshot bytes are byte-for-byte unchanged and digests match an
  independent expected SHA-256 value;
- UTF-8 source ordering, source-local indices, global ordinals, stored paths, and
  exact original strings match explicit expected values;
- repeated path entries and same-basename paths from different directories yield
  separate bindings/bytes/digests;
- declared but otherwise unused sources/evidence are retained;
- a parent-relative evidence path resolves correctly;
- a missing evidence or input file returns `DiagnosticCode::Io`.

Also run:

```sh
cargo fmt --all -- --check
cargo check --locked
cargo test --locked --lib
cargo test --locked --test conformance input_schema_contract -- --nocapture
git diff --check
```

Do not run full Clippy/all-target/release gates by habit; T014 owns them. Disclose
current incomplete-consumer warnings without suppressing or manufacturing uses.

## Required response

Continue until T011 production behavior, exact named tests, format, compile, and
regressions are complete. Save the response before returning. Include exact
input/output hashes, criterion-to-code/test locations, independent digest values,
every command/exit/nonzero count, dependency changes, warning status, and unresolved
failures. Return early only for a concrete reproduced blocker with the exact
command and smallest decision.
