# Stored-evidence inspection repair handoff

Status: complete narrow repair. This is not T020 acceptance or a combined-review request.

## Boundary and root cause

Prompt017 reconciled the focused prompt014 conformance-test hash. The corrected
prompt016 boundary matched before the Rust edit: `src/app.rs`
`d08e50c7ab32bffa0e9791bdcb32ffa27f8c60e0ed8fc586b23d643e18588ca0`,
the check schema `1bfd2fb3a644df1203b043244b38a9ed80d404cae5068901b7f162ea6dd662b9`,
the inspection schema `3c32ad02a1376b790294bcd655c8c4ffc711a4d90274546303e60577425fc176`,
the report schema `7215342a2958fd4d9e9735c2837130a02837d205db93b342b9f64d62a882fcf1`,
and `tests/cli.rs` `c20061b8d01ff67df9bb11d99883caa300147df7fdefd6d907bf425ba1811400`.

Before the repair, `cargo test --locked --test cli relocated_run_inspection -- --nocapture`
failed with exit 101 because the inspect subprocess returned a structured
`E_IO` filesystem error. The run's contained `evidence/0.bin` existed, but both
`rebuild_report` branches called `load_evidence_bindings` against the stored
`predictions.json`; that resolved the source's original relative evidence path
after relocation had removed it.

## Repair

`inspect` now calls `StoredRun::verify_evidence` for either checked task branch
before rebuilding. `rebuild_report` receives those verified contained bindings,
so source paths remain provenance only. The change reuses the established
comparison-replay verifier; it adds no fallback, duplicate resolution, artifact
change, public API, policy, tolerance, or report-equality change.

`shared_commands_multi_label` now binds one real evidence file, relocates the
run, removes every original input and evidence file before inspection, validates
the real inspection result, then mutates `relocated/evidence/0.bin`. The second
inspection returns the existing `E_PROVENANCE` replay error with exit 2.

## Verification

| Command | Exit | Result |
|---|---:|---|
| `cargo test --locked --test cli relocated_run_inspection -- --nocapture` | 0 | 1 selected real single-label relocated inspection passed with the original input and evidence unavailable. |
| `cargo test --locked --test cli shared_commands_multi_label -- --nocapture` | 0 | 1 selected real multi-label check/evaluate/relocated-inspection path passed; contained evidence tampering rejected as `E_PROVENANCE` at `replay`. |
| `cargo test --locked --test conformance replay_binding_and_result_tampering -- --nocapture` | 0 | 1 selected existing replay binding/result tamper test passed. |
| `cargo fmt --all -- --check` | 0 | passed. |
| `git diff --check` | 0 | passed. |

Cargo emitted the existing staged dead-code warnings; this repair added no new
warning class or suppression.

## Final identity

| Artifact | SHA-256 |
|---|---|
| `src/app.rs` | `b8f1b5b32bdff042eb5bca3b667da6fccdeb793bec04bb3b35c9bd7d4ce47b1b` |
| `tests/cli.rs` | `c23e38a343fb8eaccb1b784bbdf24cd15c0c4f50f081e461cf70af7add399999` |
| `schemas/v2/report.schema.json` | `7215342a2958fd4d9e9735c2837130a02837d205db93b342b9f64d62a882fcf1` |
| `src/artifacts.rs` | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |

The report schema and every production Rust file other than the authorized
`src/app.rs` repair are unchanged.
