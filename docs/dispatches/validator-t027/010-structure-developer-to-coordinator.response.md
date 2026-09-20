# T027 F1 report-boundary repair handoff

Status: complete local repair candidate. This is implementation evidence and is
not independent acceptance.

## Correction

I read verdict009 and prompts001/002/003/010. The sole repair is F1: task model
modules no longer depend on the I/O-owned `artifacts::InputArtifacts` aggregate.

`single_label::assemble_report` and `multi_label::assemble_report` now receive a
single tuple of the three required model-owned `ArtifactSnapshot` references:
golden, predictions, and config. `app` extracts and passes the tuple at both
evaluate branches, both replay-report branches, and both stored-run replay
branches. The model modules use only `ArtifactSnapshot` from `model::common`;
neither contains `use crate::artifacts` or `InputArtifacts`.

The local single-label report fixture constructs model-owned snapshots and
evidence bindings directly, so it has no artifact-module import. Manifest order,
paths, digests, source fields, serialization, replay behavior, and public API are
unchanged. No schema, fixture, oracle, numerical logic, publication, inspection,
artifact implementation, Cargo input, governance, index, Fizzy, T028, or T030
artifact was edited. The required response is the only new documentation file;
the active dispatch excludes index/governance writes.

The first post-split Clippy run found nine report-builder arguments. I replaced
the three snapshot parameters with one tuple of the same model-owned references;
the next warning-denied run was clean.

## Hash reconciliation

| File | Manifest008 SHA-256 | Final SHA-256 |
|---|---|---|
| `src/model/single_label.rs` | `1acd5af54ef01b2cc49d6e8c9c7a973ee04e529e4000e4cf0b2edf8fc1a10222` | `f03500766388d85106a8792bf119ac2df8db63673c90c81d57b2b17fc62d4f0d` |
| `src/model/multi_label.rs` | `43e80100dd9a5c59232ac48fe1925c6bd1d4f2192b86b492027052f8776bd873` | `935b15094c2f8e8817287efad55290ce5f8fd6d78fb25f6d335b7d3eaf30c033` |
| `src/app.rs` | `f741c03b17788303c7ae79ff43a11f9605cb63a6bc565a98d40f5f55b131d64b` | `3dbac840a4ddce89b8c8f04ab82583114f380e969cf65c244af411031e0e9f9d` |

## Focused evidence

All focused commands exited 0:

- `cargo test --all-features --locked --lib model::single_label::tests::report_sources_and_privacy -- --nocapture`: 1 passed, 43 filtered; report manifest, sources, and privacy.
- `cargo test --all-features --locked --test conformance replay_binding_and_result_tampering -- --nocapture`: 1 passed, 102 filtered; rebuilt-report/replay and tampering behavior.
- `cargo test --locked --test conformance structure_dm01 -- --nocapture`: 1 passed, 102 filtered; shared public task path.
- `cargo test --locked --test cli offline_cli_contract -- --nocapture`: 1 passed, 11 filtered; offline saved-input CLI behavior.

The direct F1 source check found no production artifact import or `InputArtifacts`
parameter in either model module. Remaining literal `artifacts` text is the
required serialized report field, not a module dependency.

## Final boundary

Every required command exited 0 after the final source change:

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | Clean. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | Clean. |
| `cargo test --all-features --locked` | 44 library, 12 CLI, 103 conformance, 0 binary, and 0 doc tests passed. |
| `cargo build --release --locked --bin validator` | Release build completed. |
| `sandbox-exec -p '(version 1) (allow default) (deny network*)' cargo test --all-features --locked --offline` | Same 44/12/103/0 suite passed with network denied and Cargo offline. |
| `git diff --check` | Clean. |
| `ruby docs/plans/validator/verify-plan.rb` | Passed: 35 task contracts, 433 source blocks, 63 conformance cases, 12 structure checks, 8 ACs, and 5 DoD clauses. |

There is no remaining F1 defect or unresolved local limit.
