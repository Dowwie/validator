# T007-T014 frozen combined candidate manifest

Frozen by the Sol-high coordinator at 2026-09-18T22:13-0400 for the one combined
independent review. No implementation writer may modify these paths until a saved
verifier verdict is reconciled. The repository is unborn/untracked, so exact file
hashes, rather than a Git commit, identify the candidate.

## Product source, schemas, tests, and oracle

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |
| `rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |
| `src/app.rs` | `1be35a7228f0a4d7bb4d5449e088f3914a0b3c9f28a0c3866a4466dbbb256fcf` |
| `src/artifacts.rs` | `14a7383a204c88a7922dedc6159d2b7ad4819b395bab3a8c2185e83ad39aef61` |
| `src/cli.rs` | `608ab90dcda6cd36f8b4d03575f47efc8f2a5977480a0c4bb670aa37933a0fbe` |
| `src/error.rs` | `5e47f1afde84a682e6d33744133820a43c14a761383717fc91012c63b330abcb` |
| `src/evaluation.rs` | `dbee2b6c74f60d5375545b746dbfea3ecc311ad55f3183ab02bdc260157bf891` |
| `src/evaluation/single_label.rs` | `fd271a77be2c1c9aa244749cea9a026cc9efb3e0c00f5a263df428c82caaa19d` |
| `src/lib.rs` | `b0c5d00e000747368fc9e18e6459c61417dbbf24f06ac58077e9c4cf382ceac7` |
| `src/main.rs` | `ca701aa73cb1d687d9ff086e654bd9b08e6c285a91c9577c3b2d1eb7fc54a3c0` |
| `src/model.rs` | `ceffb61ab5e523fa4bf3161d5db5a6fcd1276137d951982da4382edb79c58c66` |
| `src/model/common.rs` | `5dd0c2652439434ba9496a503c6317d8775cdbc90d8ded7b7dac5d351005d4f3` |
| `src/model/single_label.rs` | `c97ae399fe81896e0e552e0152e99db53addb14fa72043f7c68048f4bfb61e1b` |
| `src/validation.rs` | `5a76b0ccc8bd838c13e8671cfb063b27888f0f9b20ecb6e5fa6c26a03c8b49bf` |
| `src/validation/wire.rs` | `46a42903c2edbb556b1ae1ee7b3c3d9642e562e6b2de2a6d63a852c584f6a3b3` |
| `schemas/v2/check.schema.json` | `675f6b1faa3fd67b5b42bc42122f7ccf916925920dd59d14ddee886fa90804ad` |
| `schemas/v2/config.schema.json` | `c740c7cd2c1381398b3121ee161b1d995a23ceb9dd4325bd73cd0073f66d65c3` |
| `schemas/v2/error.schema.json` | `a1be234949560a90dff9391a0fa9c42de4f0431c5c4e2f20d745730f40eb80d5` |
| `schemas/v2/golden.schema.json` | `97d811a192274f1ee0703e5dff475723301fdcc68c20b031d3b22431968b777b` |
| `schemas/v2/predictions.schema.json` | `654908068da5d6b3cf9c666cddf4ebc888f94203a04fb6addd5164fbbb893352` |
| `schemas/v2/receipt.schema.json` | `d820751593f82ac68cbca2ed381b305383d96d388dee417abcefec03ed3d1c00` |
| `schemas/v2/report.schema.json` | `a99d4ab2bfc85180b9ce7619e2399ed721b7e5a60d98459efc6935578e6fdb0b` |
| `tests/cli.rs` | `fc03a9d04dbe0c58faab9574ef9eb68d8f4347d6d43d6d5424ae55e7e6cdd111` |
| `tests/conformance.rs` | `16bc7d8eb4644491ae86f30a85a5f3a69f8b471e8106351184b67e30809f9637` |
| `tests/fixtures/single-label/expected.json` | `99cb21c88d4d497b8d805537d9e2648304b9aab34ea748ea6752f350641728c5` |

## Governing authority

| Path | SHA-256 |
|---|---|
| `docs/specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `docs/specs/validator-data-model.md` | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| `docs/dev-team/validator-build/charter.md` | `4cb39ab1b23a3a0bcf3a73dcc33f3e4269c84a5beb9fc8c072c5fd1a5a75879e` |
| `docs/plans/validator/execution-contract.md` | `84ba3895e2562f1f41d8a4697c0fdd84d57f0c14c276d8cce4ef80fcb1875893` |
| `docs/plans/validator/tasks/T007.json` | `eccd9d7b21507cbd85b1a690dd26ccc4f823fd615295fb659e78ecea2aac87ee` |
| `docs/plans/validator/tasks/T008.json` | `bfe1560e90db6c810bd9a9becd239bfc7338e5994d335e687c5fafd85085b275` |
| `docs/plans/validator/tasks/T009.json` | `ba06af7caa066e16953174de4b7e463f79729966da99aea088021022511cd1aa` |
| `docs/plans/validator/tasks/T010.json` | `fdff3572185e1d0bd63e12d6eb55c945280a2a2ace61668c9725309f77f75c08` |
| `docs/plans/validator/tasks/T011.json` | `1ca3437677ac732c403c459047208f89bd2199d3c3d784564438272e03b9da4f` |
| `docs/plans/validator/tasks/T012.json` | `4f5bc1865cd8e129ae152b93dcede4671806357b7a4229607a06ddd18fbd8d45` |
| `docs/plans/validator/tasks/T013.json` | `14a5af3c6f9e5ff5900c14dfbc34aa7374379dd34d2193fe55856838f71cf858` |
| `docs/plans/validator/tasks/T014.json` | `258e57effacafad873e7c2a5fcd5773e98922e28d4b5e38a50ba089e97a10bd0` |
| `docs/dispatches/validator-t007-t014/028-owner-to-coordinator.prompt.md` | `c389855ef2640208c7b28214ecbdb941df54b59cd071ddca458c19a4b4e209ac` |

## Implementation evidence inputs

| Task | Evidence path | SHA-256 |
|---|---|---|
| T007 | `003-developer-to-coordinator.response.md` plus the final named-filter reruns in response031 | `98a697b9535978f363fdd2824135f3736229e9668b954bad92e5f4ecf05f8f89` |
| T008 | `008-scorer-developer-to-coordinator.response.md` | `cae181e54f349f9dd4df3f3ce353059dd7155c6c7b956445adfe8a4610b54296` |
| T009 | `011-scorer-developer-to-coordinator.response.md` | `70adec8312203e5193204ca4450e5d0bcf3ce7b110d6723912dd205096763627` |
| T010 | `022-schema-test-developer-to-coordinator.response.md` | `265514cda64a788de7a56f6570eda506ad1ce8081f659a49202f8ad43b1cd03c` |
| T011 | `023-schema-test-developer-to-coordinator.response.md` | `9bf24c9206cd6764d06ccec45828142759bfa8486ce372da6c4bcc529be49398` |
| T012 | `024-schema-test-developer-to-coordinator.response.md` | `d264550555d10186fcce1e966f902a2385b5468158aff4f2220d0d0328b29982` |
| T013 | `025-schema-test-developer-to-coordinator.response.md` | `c34a306011c87da662ad3eb1d2098f2524fed250d702dc145eda18a320f39a9c` |
| T014 | `031-t014-completion-developer-to-coordinator.response.md` | `e9e1478791aa0e258946fe0e7f2a364c183dd0145d8204f3b19a56f0ee7cf755` |
| T014 correction | `032-t014-completion-developer-to-coordinator.response.md` | `d400ed0401f054af18724798d0d963061b11d18f28c9a2efc9dd7e44cf18e911` |

## Coordinator reconciliation

After response032, the coordinator independently confirmed:

- `cargo fmt --all -- --check`: exit 0;
- `cargo test --all-features --locked`: exit 0, 39 library + 3 CLI + 8
  conformance + 0 doc tests;
- `cargo build --release --locked --bin validator`: exit 0;
- `git diff --check`: exit 0;
- `ruby docs/plans/validator/verify-plan.rb`: exit 0;
- exact `cargo clippy --all-targets --all-features --locked -- -D warnings`:
  exit 101 with the same 26 unique owner-staged production `dead_code` diagnostics
  plus their test-target duplicate subset, and no ordinary/new lint class.

The checkpoint is not warning-free. Owner dispatch028 authorizes precisely this
intermediate limitation; the fully warning-free gate remains mandatory at T027 and
T035. A `Ready` verdict may accept this declared limitation only if the exact
inventory matches and no behavior, schema, build, or additional-lint defect exists.
