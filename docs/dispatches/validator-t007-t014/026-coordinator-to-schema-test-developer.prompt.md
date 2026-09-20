# Validator T014 check/evaluate application and CLI dispatch 026

Role/model: retained sole developer `/root/coordinator/schema_test_developer`,
`gpt-5.6-terra`, reasoning `high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/026-schema-test-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator's verified single-label backbone](http://localhost:3006/1/cards/183).

T013 is reconciled as a complete local milestone. Complete **T014 only** next.
This is the final implementation unit in the combined T007-T014 candidate. You
remain the sole writer until the complete handoff is saved. Do not delegate or
implement compare, inspect, replay/containment, multi-label behavior, T015+, or any
new capability. After a complete clean handoff, stop; the coordinator will freeze
the exact hashes and assign one independent verifier.

## Required context and frozen input

Read in full:

- `/Users/dowwie/.codex/AGENTS.md`, repository `AGENTS.md`, and the already-read
  manage-dev-team and Rust best-practices skills;
- `docs/plans/validator/tasks/T014.json`;
- `docs/specs/validator-v1.md` sections **CLI and run artifacts**, **Machine
  interface**, **Package and source layout**, and **Test placement and build checks**;
- `docs/dispatches/validator-build/004-owner-to-coordinator.prompt.md` for the real
  application-test and clean-boundary obligations;
- `docs/dispatches/validator-t007-t014/002-owner-to-coordinator.prompt.md` for the
  exact five-entry run layout;
- every T007-T013 task contract's named verification commands, because the final
  application boundary must make each deferred conformance filter real and leave
  every earlier local filter passing.

The complete T013 handoff is
`docs/dispatches/validator-t007-t014/025-schema-test-developer-to-coordinator.response.md`,
SHA-256 `c34a306011c87da662ad3eb1d2098f2524fed250d702dc145eda18a320f39a9c`.
Confirm these exact inputs before editing:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `366bef64d9ac1e898ca6fcf1559c7ac744b671068fc3383d8c374530c5242bd8` |
| `Cargo.lock` | `33165fd692aaa3355ad66c391d31236da8f4b9a302704c3228b699ee7958c4ca` |
| `src/lib.rs` | `a61426bfb002b8b5634692e7e6f321f626e3d4d73ec1fabb90e0564c657a9a9c` |
| `src/model/common.rs` | `d30645baf87c011e919945b65464d3fa890ac7d0b9b906f00496d26a290c874e` |
| `src/model/single_label.rs` | `c97ae399fe81896e0e552e0152e99db53addb14fa72043f7c68048f4bfb61e1b` |
| `src/evaluation/single_label.rs` | `ad9dbeb0dde5b90bf23e6f6b1310270c2f8371d4bce836efca3d7a1a9cbfb944` |
| `src/artifacts.rs` | `14a7383a204c88a7922dedc6159d2b7ad4819b395bab3a8c2185e83ad39aef61` |
| `tests/conformance.rs` | `ca25086989ab2e99b901a607ee2f2ae7c4cbda454817b8bbddae326759b24f47` |
| `schemas/v2/report.schema.json` | `a99d4ab2bfc85180b9ce7619e2399ed721b7e5a60d98459efc6935578e6fdb0b` |
| `schemas/v2/check.schema.json` | `675f6b1faa3fd67b5b42bc42122f7ccf916925920dd59d14ddee886fa90804ad` |
| `schemas/v2/receipt.schema.json` | `d820751593f82ac68cbca2ed381b305383d96d388dee417abcefec03ed3d1c00` |
| `schemas/v2/error.schema.json` | `a1be234949560a90dff9391a0fa9c42de4f0431c5c4e2f20d745730f40eb80d5` |

Preserve the 39-test library and two-test schema conformance baselines. The
repository is unborn/untracked. Do not stage, commit, clean, reset, modify
`.zvec-grep`, edit governance/index/Fizzy/session records, or start/stop caffeinate
PID 84732.

## Small application API

Create `src/app.rs` and expose through `src/lib.rs` one small documented path-based
API for `check` and `evaluate`, with explicit option structs and typed success/error
results. Required behavior:

- `check` accepts explicit dataset, predictions, and config paths. It performs the
  complete T011 load/hash, T004-T006 strict validation/alignment/policy/signal
  admission, and evidence binding validation through the same production path used
  by evaluate. It returns the versioned typed check document with integrity counts
  and canonical selected IDs. It does not score, generate an ID/time, serialize a
  report, create a temporary directory, or write/publish anything.
- `evaluate` accepts those paths plus an output directory. It uses the same load and
  validation path once, then evaluates through T008/T009, supplies a new UUIDv7 and
  UTC RFC 3339 creation timestamp, assembles/serializes the T013 report, and invokes
  the T012 publisher with exact input/evidence/report bytes. It returns the typed
  evaluation receipt whose absolute `result_path` names the published `report.json`
  and whose `result_sha256` hashes that exact file.
- Preserve typed `Diagnostic` values and exit categories; never recover codes from
  strings. App functions do not read process streams or parse CLI arguments. They
  do not print. No network, environment configuration, mutable cache, fallback,
  overwrite, validation subset, or partial-success result.

Add only established minimal dependency features required for UUIDv7 generation,
UTC RFC 3339 formatting, and argument parsing. Lock them. Do not introduce an async
runtime, framework, logging stack, or a second implementation of path handling.

## Thin machine CLI

Create `src/cli.rs`; keep it binary-only via `src/main.rs`. `main.rs` only calls the
CLI and returns its process exit code. The binary imports application behavior from
the library and never redeclares/includes library modules.

Support exactly these current operational contracts:

```text
validator check --dataset PATH --predictions PATH --config PATH
validator evaluate --dataset PATH --predictions PATH --config PATH --out PATH
```

- Every operational success emits exactly one schema-valid JSON document to stdout
  and exits 0. Poor performance, abstention, and `no_data` are successful.
- Every operational application or argument failure emits exactly one schema-valid
  structured error document to stdout, no partial success, and exits 2 (input/CLI),
  3 (filesystem/output), or 4 (numeric/invariant) from the typed category.
- Safe fixed diagnostics may include only the schema's stable code, stage, path/
  affected IDs when safely available, and message. Never echo raw argument values,
  file contents, opaque input/config/evidence, credentials, or provider payloads.
- Progress, if any, goes only to stderr. Do not write ordinary diagnostics there.
  `--help` and `--version` are the specified text-output exemptions and exit 0.
  Unknown commands, flags, missing values, duplicate unsupported settings, and
  currently unimplemented compare/inspect invocations fail as structured input
  errors; do not add placeholder commands or successful stubs.

Use the T013 check/receipt/error schemas as the emitted serialization contracts.
If narrow typed check/receipt/error records or safe Diagnostic accessors are needed,
place them in their owning model/error modules and export only the API option/result
types needed by callers. Do not publish internal validation/evaluation/artifact
modules or broaden a public SDK to silence warnings.

## Real deferred conformance tests

Complete the named `tests/conformance.rs` cases that were deferred until this real
API existed. They must call the exported production API with isolated real files;
they may not include production source, use a test-only facade, static report JSON,
or production-generated expected numerical oracles:

```sh
cargo test --locked --test conformance single_matrix_identities -- --nocapture
cargo test --locked --test conformance f04_asymmetric_oracle -- --nocapture
cargo test --locked --test conformance categorical_loss_oracles -- --nocapture
cargo test --locked --test conformance signal_population_bins -- --nocapture
cargo test --locked --test conformance bin_boundary_binary64 -- --nocapture
cargo test --locked --test conformance report_sources_and_privacy -- --nocapture
```

Use the independent existing T008 oracle fixture where applicable. Assert actual
serialized report fields/values through the result path. Preserve exact tolerances.
The report/privacy case must distinguish real serialized API output from T013's
schema-only examples and prove source counts/path rewriting/sentinel absence on
published bytes.

## Required process tests

Create `tests/cli.rs` using Cargo's built `validator` executable in isolated
temporary directories. Implement and run the exact nonzero filters:

```sh
cargo test --locked --test cli cli_check_evaluate -- --nocapture
cargo test --locked --test cli cli_help_version_errors -- --nocapture
cargo test --locked --test cli receipt_hash_matches_report -- --nocapture
```

Collectively prove:

- check/evaluate each emit one parseable schema-valid JSON document, with success
  stdout separated from stderr and exit 0;
- check creates no run/output side effect and evaluate creates the exact T012 run
  layout with byte-identical snapshots/evidence and a schema-valid report;
- the receipt absolute path opens the actual `report.json`, and an independent
  SHA-256 of those bytes equals `result_sha256`;
- malformed/unknown arguments and an invalid input exit 2 with safe error JSON;
  missing/unreadable input and existing output exit 3 without overwrite/partial
  success; error-category unit evidence maps numeric/invariant to 4 without adding
  a production failure trigger;
- help/version are text exemptions, unknown flags/settings fail, and no error leaks
  a distinctive opaque input/evidence sentinel.

## Clean combined boundary

Run every exact T007-T014 named filter nonzero. Then run the mandatory final gate:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
git diff --check
```

All must exit 0 with no warnings. Resolve warnings through actual T014 consumers,
correct visibility, or removal of code made obsolete by this implementation. Do
not add `allow`, underscore/dead reads, fake runtime uses, gratuitous exports,
weakened lints, skipped tests, placeholders, or changed tolerances. If a warning
belongs to a still-required accepted contract and has no honest T014 consumer,
stop with its exact diagnostic and smallest criterion-preserving resolution rather
than gaming the gate.

## Allowed writes

- create `src/app.rs`, `src/cli.rs`, and `tests/cli.rs`;
- update `src/lib.rs` and `src/main.rs` for the documented API and binary-only CLI;
- update `tests/conformance.rs` only for the six real deferred API cases;
- minimal necessary wiring/serialization/accessors in `src/artifacts.rs`,
  `src/error.rs`, `src/evaluation.rs`, `src/evaluation/single_label.rs`,
  `src/model.rs`, `src/model/common.rs`, `src/model/single_label.rs`, and
  `src/validation.rs`/`wire.rs`;
- `Cargo.toml`/`Cargo.lock` only for the minimal established argument/time/UUID
  dependency features actually used;
- the required response file.

Do not modify schemas unless a real emitted document exposes a concrete mismatch;
any such smallest correction must be named and regression-tested in the response.
Do not modify input criteria, scoring formulas, fixture oracles, T011/T012 safety,
specifications/plans/session/index/Fizzy, comparison/inspection, replay,
multi-label behavior, or T015+.

## Required response

Continue until check/evaluate API and CLI behavior, six deferred conformance cases,
three process filters, every earlier named filter, and the clean combined gate are
complete. Save the response before returning. Include exact input/output hashes,
public API signature, dependency/features, criterion-to-code/test locations, every
named command/exit/nonzero count, stdout/stderr/filesystem assertions, warning
status, and unresolved failures. Return early only for a concrete reproduced
blocker with its exact command and the smallest contract-preserving decision.
