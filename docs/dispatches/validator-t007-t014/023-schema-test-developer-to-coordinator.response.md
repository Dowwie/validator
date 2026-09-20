# T011 artifact-loader implementation handoff

## Scope and frozen inputs

Completed only T011's private filesystem-ingestion unit. The following required
inputs matched before editing:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `e257b5d70c78f0dd7465093aad8d092db99c35ffa28227298271512faf49bec6` |
| `Cargo.lock` | `86caa4cd0470cc4d1f8d4d8dedbdfccb1c43e42b9f0a5a119343e35b256dd74e` |
| `src/lib.rs` | `bd89b6b7148e8ef25e64d28b465218a1f4afd90b8f95e6f92ed3a510de69aade` |
| `src/model/common.rs` | `a9f823207e1fd1aaa6181b5594e5d2a8ac4df150216b7299901b52c98a3ae63b` |
| `tests/conformance.rs` | `99b0a0eda61a8294a8f3489c019eb3827f919e36abb296ba83c77644176e18e7` |
| `022-schema-test-developer-to-coordinator.response.md` | `265514cda64a788de7a56f6570eda506ad1ce8081f659a49202f8ad43b1cd03c` |

## Implemented behavior

- [src/artifacts.rs](../../../src/artifacts.rs) lines 42-51 loads the three
  input paths once each, retaining caller paths, exact bytes, fixed stored names,
  and lowercase SHA-256 digests. The shared reader/hash helper is at lines
  101-105 and maps every read error to `E_IO`.
- [src/artifacts.rs](../../../src/artifacts.rs) lines 55-81 sorts source IDs by
  UTF-8 bytes, preserves each source-array order, assigns `evidence/n.bin` from
  the consecutive binding count, and reads every entry independently. It resolves
  new-submission paths from the prediction snapshot parent at lines 94-99.
- [src/model/common.rs](../../../src/model/common.rs) lines 266-379 adds crate-
  private snapshot and evidence-binding records for exact path, bytes, stored
  path, digest, source ID, and source-local evidence index. `SourceDefinition`
  exposes only its crate-private ordered evidence slice at line 884.
- [src/lib.rs](../../../src/lib.rs) line 9 declares the private module. No
  validation, scoring, schemas, conformance tests, replay, publication, or CLI
  code changed.

The sole dependency addition is `sha2 0.10.9` with default features disabled.
The lockfile adds its six transitive packages: `block-buffer`, `cpufeatures`,
`crypto-common`, `digest`, `generic-array`, and `typenum`.

## Owning test evidence

- `artifacts::tests::artifact_exact_bytes` at
  [src/artifacts.rs](../../../src/artifacts.rs) line 192 verifies exact original
  input paths and bytes, fixed destinations, and independently hard-coded
  SHA-256 values:
  - golden: `b7fd587d6da1189fbf961d5162cf21ec7dfe27ffaf951e133f5048624b0eec4d`
  - predictions: `3a579e1227f987caa851b378c3cb3b861ce60a3771c765744bde461c723732dd`
  - config: `75af282886e284f3f721eff48719a30f63b48239a8aeac90939b0f51506ef29e`
- `artifacts::tests::evidence_ordinal_binding` at
  [src/artifacts.rs](../../../src/artifacts.rs) line 222 explicitly checks the
  byte order `a`, `unused`, `z`, `β`; local indices; global destinations
  `evidence/0.bin` through `evidence/6.bin`; original strings; exact bytes; and
  independent digests. It proves separate repeated entries, two `duplicate.bin`
  files from distinct directories, the unused source's entry, and a
  `../parent-relative.bin` entry.
  The independently expected evidence digests are
  `c0ce1fd25c8c53d9855416855af6003cecd29030c906ba21691e41daf3e1f106`,
  `1823e421813d2b4f001b4a5e200bf67f50612716f4aee619ed7bb212e07e81bc`,
  `5ade6c6055db617fc600b1fecf214b2901640d552152983664ae236f9094c634`, and
  `6d821f2a541fe912714c3285118da83656218f4b12421ed3db79385da8d51696`.
- `artifacts::tests::missing_input_and_evidence_return_io` at
  [src/artifacts.rs](../../../src/artifacts.rs) line 337 verifies both a missing
  input and a missing relative evidence file return `DiagnosticCode::Io`.

## Verification

| Command | Exit | Nonzero test/failure count |
|---|---:|---|
| `cargo fmt --all -- --check` | 0 | 0 formatting errors |
| `cargo test --locked --lib evidence_ordinal_binding -- --nocapture` | 0 | 1 passed, 0 failed, 35 filtered |
| `cargo test --locked --lib artifact_exact_bytes -- --nocapture` | 0 | 1 passed, 0 failed, 35 filtered |
| `cargo check --locked` | 0 | 0 compile errors |
| `cargo test --locked --lib` | 0 | 36 passed, 0 failed |
| `cargo test --locked --test conformance input_schema_contract -- --nocapture` | 0 | 1 passed, 0 failed |
| `git diff --check` | 0 | 0 whitespace errors |

Before the locked checks, `cargo check` exited 0 while resolving the authorized
new dependency and updating `Cargo.lock`. No test command was skipped.

`cargo check --locked` and the conformance build retain the repository's
incomplete-consumer dead-code warnings. This unit adds corresponding warnings for
the private loader and records until later orchestration consumes them; no warning
was suppressed. Library-test compilation reports 18 existing dead-code warnings.

## Candidate hashes and limits

| Path | SHA-256 after change |
|---|---|
| `Cargo.toml` | `6ce0bfe1de0d76da9067f69cea8ea8b8d33f147fc0777b324a31eab2f5824d6f` |
| `Cargo.lock` | `347f173eb4734db7e6b5c7d21c0218ade12aeb547a8870387001f06b2ac6fff2` |
| `src/lib.rs` | `a61426bfb002b8b5634692e7e6f321f626e3d4d73ec1fabb90e0564c657a9a9c` |
| `src/model/common.rs` | `b227e49cff07d09923534bbee5fd59ecdd23541c908cbae86fd5f120a37722d1` |
| `src/artifacts.rs` | `f9df9fd81758f244b81542db1791c8696a5d59e0a90b363b5b3c07f23e86a8b8` |
| `tests/conformance.rs` | `99b0a0eda61a8294a8f3489c019eb3827f919e36abb296ba83c77644176e18e7` |

There are no unresolved T011 failures. T012 replay checks, path containment,
symlink policy, publication, overwrite handling, parsing, validation, and metrics
remain intentionally unimplemented in this unit. Per dispatch scope, this response
is the only documentation write; the coordinator owns artifact-index and Fizzy
updates.
