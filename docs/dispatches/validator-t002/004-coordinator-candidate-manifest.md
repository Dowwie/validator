# T002 frozen candidate manifest 004

Frozen by the coordinator on 2026-09-18 after reading the corrected developer
handoff. The sole implementation writer is idle and implementation writes are
paused for independent T002 review.

## Accepted baseline and candidate hashes

T003 is accepted at repair-manifest SHA-256
`ceb00209087532787858fbb60045d5dbb2f9ae10c91ff60c5e5be4eee3c802c3`.

| Path | Candidate SHA-256 | State |
|---|---|---|
| `Cargo.toml` | `8c758ad6246244317ac02156077b417eab3cd49994daf2597c65ae86bb24629c` | Enables serde_json `arbitrary_precision` |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` | Unchanged package resolution |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` | Unchanged |
| `src/lib.rs` | `b60b0bf5d4fed5fe6045e7c1039f3c21e315dc46d9bc08159d56d2aa4ab35162` | Declares private validation module |
| `src/error.rs` | `9dc5cd3fb698cd3eda261f66a684508cfaa61f968cdf8478b9b794d127809afa` | Unchanged |
| `src/model.rs` | `3275697520566394a2a20d4011ad28b3912d9d4773a44203980127d3f2d4d4d1` | Unchanged |
| `src/model/common.rs` | `ee0b0169d88707663c7b1540d1406868ba61cb0db2fd4d36dd2728d3baf9cecf` | Unchanged |
| `src/validation.rs` | `0c2e784d933ee074610963d36488331102c4cda99f7da17b470f5105d2d26c8e` | Created decode boundary/tests |
| `src/validation/wire.rs` | `718c871da7764ff26502043f111d8ecc1224a97b0cce27cc82081460e479f895` | Created private DTOs/duplicate visitor |
| `docs/dispatches/validator-t002/003-developer-to-coordinator.response.md` | `04da6eea5529336e2ac927cbf2a2aee3a2ca1ade210709ae1fb5580caf79456f` | Developer evidence |

No other implementation, test, schema, fixture, or acceptance file belongs to
this candidate. `target/` is replaceable output and excluded.

## Developer and coordinator evidence

- Nine library tests list; each exact T002 filter runs one test and passes.
- Format, full locked tests, locked binary build, and dependency feature inspection
  pass. `serde_json` adds only `arbitrary_precision`; `raw_value` is not enabled.
- The numeric test reads `9007199254740993` as that exact `u64` value, accepts
  explicit null, rejects missing input, and confirms retained bytes equal input.
- Recursive duplicate tests cover root, nested opaque input, escaped-equivalent
  keys, source configuration, and preparation configuration.
- The DTO retains original bytes independently from its ordinary parsed value; no
  decoded lexical-spelling gate or raw-value wrapper exists.

The coordinator independently reproduced full Clippy exit 101. Its non-test
library diagnostics are 24 accepted T003 `dead_code` groups plus 28 T002 private-
module `dead_code` groups. The library-test target adds 15 unread private DTO-field
groups. No `unused_imports`, Clippy style/performance class, suppression, or fake
consumer appears. This exact incomplete-consumer limit remains unresolved; clean
full Clippy is mandatory at T014/T017 and is not claimed here.
