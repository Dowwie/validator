# Validator T012 atomic no-replace publisher dispatch 024

Role/model: retained sole developer `/root/coordinator/schema_test_developer`,
`gpt-5.6-terra`, reasoning `high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/024-schema-test-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator's verified single-label backbone](http://localhost:3006/1/cards/183).

T011 is reconciled as a complete local milestone. Complete **T012 only** next.
This remains inside the combined T007-T014 candidate; there is no independent
review or acceptance at this unit. You remain the sole implementation/test writer.
Do not delegate or begin T013.

## Required context and input identity

Read in full:

- `/Users/dowwie/.codex/AGENTS.md`, repository `AGENTS.md`, and the already-read
  manage-dev-team and Rust best-practices skills;
- `docs/plans/validator/tasks/T012.json`;
- `docs/specs/validator-v1.md` sections **CLI and run artifacts**, **Machine
  interface**, and **Evidence bindings and replay** only for publication rules;
- the accepted owner clarification in
  `docs/dispatches/validator-t007-t014/002-owner-to-coordinator.prompt.md`: a run
  directory contains only `golden.json`, `predictions.json`, `config.json`,
  optional `evidence/`, and `report.json`; manifest metadata belongs inside
  `report.json`, and schemas stay under `schemas/v2/`.

The complete T011 handoff is
`docs/dispatches/validator-t007-t014/023-schema-test-developer-to-coordinator.response.md`,
SHA-256 `9bf24c9206cd6764d06ccec45828142759bfa8486ce372da6c4bcc529be49398`.
Confirm these exact inputs before editing:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `6ce0bfe1de0d76da9067f69cea8ea8b8d33f147fc0777b324a31eab2f5824d6f` |
| `Cargo.lock` | `347f173eb4734db7e6b5c7d21c0218ade12aeb547a8870387001f06b2ac6fff2` |
| `src/lib.rs` | `a61426bfb002b8b5634692e7e6f321f626e3d4d73ec1fabb90e0564c657a9a9c` |
| `src/model/common.rs` | `b227e49cff07d09923534bbee5fd59ecdd23541c908cbae86fd5f120a37722d1` |
| `src/artifacts.rs` | `f9df9fd81758f244b81542db1791c8696a5d59e0a90b363b5b3c07f23e86a8b8` |

Preserve the 36-test library and one-test conformance baselines. The repository
is unborn/untracked. Do not stage, commit, clean, reset, modify `.zvec-grep`, edit
governance/index/Fizzy/session records, or start/stop caffeinate PID 84732.

## Exact publication behavior

Extend `src/artifacts.rs` with one private publisher whose input is a complete
serialized `report.json`, the T011 exact snapshots, and evidence bytes/bindings.

- Create a uniquely owned temporary **sibling** of the requested final directory.
  Write only exact `golden.json`, `predictions.json`, `config.json`, optional
  `evidence/n.bin` entries, and exact `report.json`. Do not add a manifest file,
  per-run schemas, marker files, or any other entry.
- Protect the temporary/final run directory on the tested macOS host with owner-
  only permissions suitable for sensitive snapshots. Verify the final permissions
  in the owning test.
- After every required byte is written and closed, publish with a host-supported
  atomic **no-replace** rename. An exists-check followed by ordinary rename is
  forbidden. On this host, an established direct dependency such as `rustix`
  `renameat_with(..., RenameFlags::NOREPLACE)` is appropriate; enable only the
  needed filesystem feature and record the exact primitive/result mapping.
- If the final path already exists or a competing file/directory/symlink appears,
  it wins unchanged and publication returns `E_OUTPUT_EXISTS`. Do not retry or
  fall back to a replacing operation.
- Any validation/preparation/write/rename failure leaves no final success
  directory and cleans only the temporary sibling owned by this call. Never
  remove or modify the caller's destination, source inputs, or unrelated siblings.
- Return the absolute path to the final `report.json` plus SHA-256 of its exact
  bytes. The digest is of the report file only.

Use a private deterministic test seam around the pre-publish point or writer only
as needed to reproduce the race and late failure. It must not create a public/test
SDK, bypass the production no-replace path, or add a fallback mode. Production
code uses typed diagnostics and no `unwrap`, `expect`, panic-based operational
handling, or suppression.

## Allowed writes

- `src/artifacts.rs` only for publication and its owning tests;
- `Cargo.toml`/`Cargo.lock` only for one established minimal no-replace filesystem
  dependency/feature if needed on the host (reuse the locked graph where possible);
- `src/model/common.rs` only if a minimal typed published-result record is required;
- the required response file.

Do not modify validation/evaluation/scoring, schemas, conformance tests, existing
fixtures, app/CLI/report assembly, specifications/plans/session/index/Fizzy,
multi-label code, or T013+ behavior.

## Required owning tests

Implement and run the exact nonzero filters:

```sh
cargo test --locked --lib publish_no_replace_race -- --nocapture
cargo test --locked --lib publish_late_failure -- --nocapture
```

The first must exercise the real no-replace primitive after the temporary sibling
is complete and prove competing **file, directory, and symlink** destinations all
remain unchanged while publication returns `E_OUTPUT_EXISTS` and removes only its
own temporary sibling. Also prove one successful publish has the exact five-entry
layout (with evidence), owner-only directory permissions, absolute report path,
and independent report-byte digest.

The second must inject a late failure after at least one temporary output write,
then prove no final path exists, the owned temporary sibling is removed, source
snapshot/evidence bytes are unchanged, and unrelated sibling content is untouched.

Also run:

```sh
cargo fmt --all -- --check
cargo check --locked
cargo test --locked --lib
cargo test --locked --test conformance input_schema_contract -- --nocapture
git diff --check
```

Do not run the full Clippy/all-target/release gates by habit; T014 owns them.
Disclose incomplete-consumer warnings exactly without suppressing them.

## Required response

Continue until T012 publication, real no-replace races, late-failure cleanup,
permissions/layout/path/digest evidence, format, compile, and regressions are
complete. Save the response before returning. Include exact input/output hashes,
primitive/dependency choice, criterion-to-code/test locations, every command/exit/
nonzero count, warning status, and unresolved failures. Return early only for a
concrete reproduced blocker with its exact command and smallest decision.
