# Frozen T027 combined structural/offline candidate manifest

Status: frozen for the one independent combined T027 review authorized by owner
prompt001. Every listed candidate file is read-only until a saved verdict or the
one authorized finding-driven correction. T029 and later implementation remain
undispatched. Conditional T030 source-isolated oracle work may use the sole writer
only after this review is active and cannot modify this candidate.

## Governing and developer evidence

| Artifact | SHA-256 |
|---|---|
| `001-owner-to-coordinator.prompt.md` | `5dfe8339ecdfdeef5d4ff15e7ddd72d4fe91afba1b3f147a51f063ad755d39d6` |
| `001-coordinator-to-owner.response.md` | `0f8d6364dca4f96774abde947c4d13f97f9665f3ca0f5630dc747cb3ef7c1fb9` |
| `004-owner-to-coordinator.prompt.md` | `ded05f01e82a322e5f8782f07c6ed2a97c5e84856841343e9e9a13813537c01e` |
| `004-coordinator-to-owner.response.md` | `bf53f22ecbdb9068fe2b141240fde1b40520ba10d4c407ebdb10c6ea4bfdf3bb` |
| `002-structure-developer-to-coordinator.response.md` | `3eb3225a406491626343972cec004d0b9ea585f596f176a1cfcb2753f97de7cb` |
| `007-proof-developer-to-coordinator.response.md` | `26337bf5c39ba05d9b913098872f3a94f63a276ab537c53e1fb53e2dbd556f33` |
| `../validator-t026/017-coordinator-source-field-manifest.md` | `bb0ea141de203596fcd9ee3c7db8b7bf37f8a2033e7858d05074b9bf03e3cf44` |
| `../validator-t026/018-verifier-to-coordinator.response.md` | `b6d8ea67f0eccaaa0f3813e5e45dd557bda1bc8f553632145f2ba78c8eb94c98` |

## Contracts and mappings

| Artifact | SHA-256 |
|---|---|
| `../../specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `../../specs/validator-data-model.md` | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| `../../plans/validator/tasks/T027.json` | `82df45c1655fca9abad8d72ea33bc4cb2be64c5a8ffee187d277e6883b5ad01f` |
| `../../plans/validator/physical-map.json` | `08c68ddd91e5f3ac3a9f231d369bf78383832556f94e8174a6ee16a21f5f9d6d` |
| `../../plans/validator/tasks/T032.json` | `7e4c6cbfc7325adafd372290eb3397cceea9865addae0146879b2a73c7de886b` |
| `../../plans/validator/tasks/T033.json` | `1fca1a28670a906457bc4c2052f1b578863d2cbee99d136496d5f0f3b9f326ca` |
| `../../plans/validator/tasks/T034.json` | `83495835b9905cc3a2aaec233aa3c52dccaff858fe4f986c0a0fec11bacbe64e` |
| `../../plans/validator/coverage.json` | `efe2f425fc975f953c5765d1ea40ed47ffea8f672249d85f4e3eb07c9843e896` |

## Frozen source and tests

| Artifact | SHA-256 |
|---|---|
| `../../../src/lib.rs` | `8690583df0fffe301262a476fc39d021148cca08c8860f7dfe0ce03b9698c9c7` |
| `../../../src/cli.rs` | `ed6424fa545b1ccb8418cfc008abe466c7953b9ecad17b0c2f494fc2d4c06969` |
| `../../../src/model.rs` | `992adf37259839f71dbffa931ac1a341894e90f29283bf34f0f393f1a762ce56` |
| `../../../src/model/common.rs` | `f5f626654b33c63255de26c0e4a07ba79696b980042fa240e2741d73e54bfc8c` |
| `../../../src/model/single_label.rs` | `1acd5af54ef01b2cc49d6e8c9c7a973ee04e529e4000e4cf0b2edf8fc1a10222` |
| `../../../src/model/multi_label.rs` | `43e80100dd9a5c59232ac48fe1925c6bd1d4f2192b86b492027052f8776bd873` |
| `../../../src/validation.rs` | `1ae4707c48c2310e45340ccbc24109eee616d74eaf03a664c247905ba840ebbb` |
| `../../../src/validation/wire.rs` | `1a6d6887dc3ed2fd1a02c337592cf43dd97b59c2dcf6f361459e396634f27ee4` |
| `../../../src/app.rs` | `f741c03b17788303c7ae79ff43a11f9605cb63a6bc565a98d40f5f55b131d64b` |
| `../../../src/evaluation.rs` | `172ab7db055ca9483e3b64593411bb9e64c1983eecdbd06013fe0f6ce1a483ff` |
| `../../../src/evaluation/single_label.rs` | `f0cad620c80d8a1b7cef1847d49179c1b028f33546d40c5fc0b38cf5f002938c` |
| `../../../src/evaluation/multi_label.rs` | `c4158facd454e66e66b3f1ddc0bb9976ca07fca6961bf200597ba9b31107b461` |
| `../../../src/comparison.rs` | `4ed45a737ed4ea49fc3c88728b83156acc9938f696078c193bc53979b530f845` |
| `../../../src/artifacts.rs` | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |
| `../../../tests/conformance.rs` | `e3e092d6bd6d0a7fff53751e22974c1e3e11904a1c9b4c11e64a5aa3c2f5bc53` |
| `../../../tests/cli.rs` | `ecd69a603a69c786256dba05d9057a6641d9ed412aae7330bc32400ed80ed99b` |

## Frozen schemas and build inputs

| Artifact | SHA-256 |
|---|---|
| `../../../schemas/v2/golden.schema.json` | `97d811a192274f1ee0703e5dff475723301fdcc68c20b031d3b22431968b777b` |
| `../../../schemas/v2/predictions.schema.json` | `654908068da5d6b3cf9c666cddf4ebc888f94203a04fb6addd5164fbbb893352` |
| `../../../schemas/v2/config.schema.json` | `c740c7cd2c1381398b3121ee161b1d995a23ceb9dd4325bd73cd0073f66d65c3` |
| `../../../schemas/v2/report.schema.json` | `805de93d2a50339370e803b7ff1a9a1b8d34c3028b5b21a1a6f428f591733f90` |
| `../../../schemas/v2/comparison.schema.json` | `4f2dd6886056d335c3d4209c3a47dbfab2685a8424b4b8153d8d02fd77f9a104` |
| `../../../schemas/v2/inspection.schema.json` | `6677b1689d581744636cf480cea690a55c566415e794f995e34c34f1b9da3883` |
| `../../../schemas/v2/check.schema.json` | `1bfd2fb3a644df1203b043244b38a9ed80d404cae5068901b7f162ea6dd662b9` |
| `../../../schemas/v2/receipt.schema.json` | `6f20777251015af381ad321030490c9c28996a165a4528e8b739d40227cc9637` |
| `../../../schemas/v2/error.schema.json` | `a1be234949560a90dff9391a0fa9c42de4f0431c5c4e2f20d745730f40eb80d5` |
| `../../../Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `../../../Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |
| `../../../rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |

## Combined evidence

- Private wire decode constructs the closed concrete task definition and enforces
  version 2; each golden row owns one checked Episode with opaque input, ID and
  task-specific target. Evaluators borrow target/ID; public inspection discloses a
  clone only after verified replay. Metrics/comparison never own opaque input.
- Required wrappers/types are real consumers. Only source-confirmed convenience
  methods and the obsolete signal wrapper were removed or test-localized. No fake
  use, suppression, registry, public DTO export or dependency change exists.
- Exact nonzero `structure_dm01`-`structure_dm12` and `offline_cli_contract`
  filters execute discriminating public behavior. Direct signature/caller/shared-
  path review supplies the static half of DM01/DM02 and related rows without
  compiler-test or source-text-test machinery.
- Inventory is 44 library, 12 CLI, 103 conformance and zero doc tests. Format,
  warning-denied Clippy, full locked suite, release binary, exact sandboxed
  deny-network locked-offline full suite and diff checks all pass. Both full suites
  report 44/12/103/0. Later T028-T035 evidence remains explicitly pending.

The verifier may run the exact current commands using the existing target and
inspect necessary accepted surrounding code. It must not edit the candidate,
extend review into private acceptance data/later tasks, or demand a new compiler-
test/source-text framework where behavioral plus direct static review proves the
property.
