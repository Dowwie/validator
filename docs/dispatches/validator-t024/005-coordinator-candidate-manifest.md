# Frozen T024 candidate manifest

Status: frozen for the single independent T024 review authorized by owner
prompt001. The nine schemas and two test files remain read-only until a saved
verdict or one owner-authorized finding-bound repair prompt.

## Governing chain

| Artifact | SHA-256 |
|---|---|
| `001-owner-to-coordinator.prompt.md` | `bfe635ffb06f5ebdeda58610dec181896cb6fd4432726d0355fd583039d87269` |
| `001-coordinator-to-owner.response.md` | `740ca222c8b432b3df70edffd9efd13041ad5b5e683e5a209ad6e6bc5287aae8` |
| `002-schema-developer-to-coordinator.response.md` | `3f00d279951624aca41b3b26a78a686faae90d8938d5fe73249a21ddca91207b` |
| `004-case-developer-to-coordinator.response.md` | `ea7cd976a1a9f798f4a0500d73ac9393cc3a06058c04fb233ed1699f0d8bf145` |
| `../validator-t021-t023/013-coordinator-repaired-manifest.md` | `2cd39b7c27d9b0408ff0903441f3afa975616edf3644f3e502f560d6d26f0bc7` |

## Frozen T024 files

| Artifact | SHA-256 |
|---|---|
| `schemas/v2/check.schema.json` | `1bfd2fb3a644df1203b043244b38a9ed80d404cae5068901b7f162ea6dd662b9` |
| `schemas/v2/comparison.schema.json` | `d0dd16bef3902ac1597bb6538204ffe7c13442c655b4779f9953f6fd5200c8e9` |
| `schemas/v2/config.schema.json` | `c740c7cd2c1381398b3121ee161b1d995a23ceb9dd4325bd73cd0073f66d65c3` |
| `schemas/v2/error.schema.json` | `a1be234949560a90dff9391a0fa9c42de4f0431c5c4e2f20d745730f40eb80d5` |
| `schemas/v2/golden.schema.json` | `97d811a192274f1ee0703e5dff475723301fdcc68c20b031d3b22431968b777b` |
| `schemas/v2/inspection.schema.json` | `039e6060b88042c908c27a34374592d9f64434aa1aac4725da13146b01c0ac82` |
| `schemas/v2/predictions.schema.json` | `654908068da5d6b3cf9c666cddf4ebc888f94203a04fb6addd5164fbbb893352` |
| `schemas/v2/receipt.schema.json` | `d820751593f82ac68cbca2ed381b305383d96d388dee417abcefec03ed3d1c00` |
| `schemas/v2/report.schema.json` | `5500e80464cd87b196f291634c00c073113220747f03898c66406f5106e94ea9` |
| `tests/conformance.rs` | `4b44ea67d2f902e139790a9be339539abd7ee3a90f64052df7092c37b62cc9b7` |
| `tests/cli.rs` | `8c7b9c00d777f4a290911dd77199d8e1b9e085cc14fb0dd78a801108c3a10fcf` |
| `Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |

## Local construction evidence

- All nine self-contained Draft 2020-12 schemas load with pinned offline tooling.
- `all_schema_contracts` and real-binary `cli_stdout_schema_matrix` each list and
  pass one nonzero test with positive and discriminating mutation evidence.
- Every T024-owned exact filter exists and passes one nonzero test: S13, S14, S17,
  S21, S29, M16, M21, E05, E06 and E09-E13.
- Full locked suite passes: 44 unit, 36 conformance, 10 CLI, 0 doc tests.
- Formatting, locked release build and diff check pass.
- Warning-denied Clippy exits 101 only for the accepted 17 production plus three
  duplicate lib-test staged diagnostics; no T024-owned warning exists.

No production Rust, fixture, dependency or future-task file changed. T025 and
later work remain paused. The verifier may read surrounding accepted interfaces and
execute bounded checks but must not modify this candidate.
