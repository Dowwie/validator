# T012 no-replace publication handoff

## Scope and frozen inputs

Completed only T012 in `src/artifacts.rs`. The required inputs matched before
editing:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `6ce0bfe1de0d76da9067f69cea8ea8b8d33f147fc0777b324a31eab2f5824d6f` |
| `Cargo.lock` | `347f173eb4734db7e6b5c7d21c0218ade12aeb547a8870387001f06b2ac6fff2` |
| `src/lib.rs` | `a61426bfb002b8b5634692e7e6f321f626e3d4d73ec1fabb90e0564c657a9a9c` |
| `src/model/common.rs` | `b227e49cff07d09923534bbee5fd59ecdd23541c908cbae86fd5f120a37722d1` |
| `src/artifacts.rs` | `f9df9fd81758f244b81542db1791c8696a5d59e0a90b363b5b3c07f23e86a8b8` |
| `023-schema-test-developer-to-coordinator.response.md` | `9bf24c9206cd6764d06ccec45828142759bfa8486ce372da6c4bcc529be49398` |

## Publication implementation

- [src/artifacts.rs](../../../src/artifacts.rs) lines 112-178 adds the private
  publisher and receipt. It writes only the three T011 snapshots, an optional
  `evidence/` directory containing bound `n.bin` files, and `report.json`; it
  returns the absolute report path plus the SHA-256 digest of the exact report
  bytes.
- Lines 122-147 create and clean a uniquely owned temporary sibling for every
  unsuccessful publication. The private pre-publish hook is a deterministic
  module-local test seam; production calls it only with success at line 119.
- Lines 191-246 create the sensitive temporary directory at mode `0700`, write
  files at mode `0600`, and protect an optional evidence directory at mode `0700`.
  The immediate post-create permission-failure branch removes its own sibling
  before returning.
- Lines 249-262 invoke the actual host primitive
  `rustix::fs::renameat_with(CWD, temporary, CWD, final, RenameFlags::NOREPLACE)`.
  On the tested Darwin host, Rustix maps this to `renameatx_np` with
  `RENAME_EXCL`; `EXIST` and `NOTEMPTY` map to `E_OUTPUT_EXISTS`, and every other
  filesystem error maps to `E_IO`. No exists-check/ordinary-rename path, retry,
  replacement, manifest file, per-run schema, marker file, replay, or publication
  fallback exists.

The direct dependency is `rustix` with only the required `fs` and `std` features;
the lock resolves `rustix 1.1.5` and adds `errno` plus `linux-raw-sys`.
`std` is required for the direct `Path` arguments to `renameat_with` after
default features were disabled.

## Owning test evidence

- `artifacts::tests::publish_no_replace_race` at
  [src/artifacts.rs](../../../src/artifacts.rs) line 575 first publishes a
  complete run and proves the exact five-entry root layout:
  `golden.json`, `predictions.json`, `config.json`, `evidence/`, and `report.json`.
  It verifies every copied byte, `0700` final/evidence permissions, `0600` report
  permissions, the absolute report path, and the independent report digest
  `a1e73038b20b14c8814f26b22e8107c1b410db346c6736eea60480137a09109b`.
  Its pre-publish seam confirms the completed sibling has report/evidence bytes,
  then separately creates competing file, directory, and symlink destinations.
  The real `NOREPLACE` call returns `E_OUTPUT_EXISTS`; each competitor remains
  byte-identical or link-identical and the captured owned sibling no longer exists.
- `artifacts::tests::publish_late_failure` at
  [src/artifacts.rs](../../../src/artifacts.rs) line 703 injects `E_IO` after
  temporary `golden.json` and `report.json` writes. It proves no final directory
  exists, the owned temporary sibling is removed, all source snapshot/evidence
  bytes are unchanged, and an unrelated sibling remains unchanged.

## Verification

| Command | Exit | Nonzero test/failure count |
|---|---:|---|
| `cargo fmt --all -- --check` | 0 | 0 formatting errors |
| `cargo test --locked --lib publish_no_replace_race -- --nocapture` | 0 | 1 passed, 0 failed, 37 filtered |
| `cargo test --locked --lib publish_late_failure -- --nocapture` | 0 | 1 passed, 0 failed, 37 filtered |
| `cargo check --locked` | 0 | 0 compile errors |
| `cargo test --locked --lib` | 0 | 38 passed, 0 failed |
| `cargo test --locked --test conformance input_schema_contract -- --nocapture` | 0 | 1 passed, 0 failed |
| `git diff --check` | 0 | 0 whitespace errors |

The initial dependency-resolution `cargo check` exited 0. A subsequent
`cargo check --locked` with Rustix `fs` alone exited 101 with `E0277` because
`&Path` is supplied by Rustix's `std` feature; adding that required feature made
the final locked check pass. No test or validation gate was skipped.

The final locked check and conformance build report 186 dead-code warnings from
the staged incomplete consumers, including the private T011/T012 loader and
publisher awaiting later application wiring. Library-test compilation reports 18
existing dead-code warnings. No warning was suppressed.

## Candidate identity and limits

| Path | SHA-256 after change |
|---|---|
| `Cargo.toml` | `366bef64d9ac1e898ca6fcf1559c7ac744b671068fc3383d8c374530c5242bd8` |
| `Cargo.lock` | `33165fd692aaa3355ad66c391d31236da8f4b9a302704c3228b699ee7958c4ca` |
| `src/lib.rs` | `a61426bfb002b8b5634692e7e6f321f626e3d4d73ec1fabb90e0564c657a9a9c` |
| `src/model/common.rs` | `b227e49cff07d09923534bbee5fd59ecdd23541c908cbae86fd5f120a37722d1` |
| `src/artifacts.rs` | `14a7383a204c88a7922dedc6159d2b7ad4819b395bab3a8c2185e83ad39aef61` |
| `tests/conformance.rs` | `99b0a0eda61a8294a8f3489c019eb3827f919e36abb296ba83c77644176e18e7` |

There are no unresolved T012 failures. T013 and later work—including typed
report assembly, application wiring, replay containment/symlink verification,
and any further publisher behavior—remains outside this unit. This required
response is the only documentation write; the coordinator owns artifact-index
and Fizzy updates.
