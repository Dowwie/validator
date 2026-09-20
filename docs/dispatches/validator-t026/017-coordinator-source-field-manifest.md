# Frozen T026 resolved-inspection-source candidate manifest

Status: frozen for the same verifier's focused R1/source-field recheck authorized
by owner prompt014. This candidate preserves the other five repaired findings and
changes only the bounded application, inspection schema and focused conformance/
CLI proof surfaces. T028's protected checkpoint is separate and untouched. T027
and T029+ remain undispatched.

## Governing and repair chain

| Artifact | SHA-256 |
|---|---|
| `001-owner-to-coordinator.prompt.md` | `e9d4036002883fef4d94973d5f4519e5729b67afc26b3854bfbb6fbbae880115` |
| `012-coordinator-repaired-manifest.md` | `4fc0552b87d895a5597977a12884fb09b6c1e5c2145152ede8551b54abb85895` |
| `013-verifier-to-coordinator.response.md` | `285cc24cc012121d3b8d969c95be6e6cd6bcb70b23201e9350e3dad635b5aa37` |
| `014-owner-to-coordinator.prompt.md` | `a19fed980f87ee4c100029b3b7d636ee84bba0548704727067819c77a6d5a216` |
| `015-verifier-to-coordinator.response.md` | `5721fb83161c839005056236129e4b681169e5c903b081ff69006e403c2e645f` |
| `016-coordinator-to-evidence-developer.prompt.md` | `4ff672222c59d786dca8b4d7e1e45e5559219ef1f7a3b865fd682814362ede21` |
| `016-evidence-developer-to-coordinator.response.md` | `1fe27ed67594525a5db3c3e0592a1b5d101d876d223ac51f87e34b357d573011` |

## Contracts and mappings

| Artifact | SHA-256 |
|---|---|
| `../../specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `../../specs/validator-data-model.md` | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| `../../plans/validator/tasks/T026.json` | `6a5715dfae5a84763523079da19077a8f696383ad0d616abc150a69d9ddf9032` |
| `../../plans/validator/tasks/T027.json` | `4c58adee793a3a5278394aaa76dbb0b985a46f1e3ed72ad9947a7b0cd79a052d` |
| `../../plans/validator/physical-map.json` | `98dab85533b199862bebbb00894eb81f1216d9e8a255b52dc715cdab613168bf` |
| `../../plans/validator/coverage.json` | `efe2f425fc975f953c5765d1ea40ed47ffea8f672249d85f4e3eb07c9843e896` |

## Frozen candidate files

| Artifact | SHA-256 |
|---|---|
| `../../../src/app.rs` | `8326278b0739f7a55f6dcb09d0c97683e17ad4dc2431e4214391cfa5d24407ad` |
| `../../../src/artifacts.rs` | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |
| `../../../src/comparison.rs` | `4ed45a737ed4ea49fc3c88728b83156acc9938f696078c193bc53979b530f845` |
| `../../../src/cli.rs` | `ed6424fa545b1ccb8418cfc008abe466c7953b9ecad17b0c2f494fc2d4c06969` |
| `../../../src/evaluation/multi_label.rs` | `c4158facd454e66e66b3f1ddc0bb9976ca07fca6961bf200597ba9b31107b461` |
| `../../../tests/conformance.rs` | `339971adbc8a1914065b026cf2c372b51ec0ca1f4901a75eccb0c5eba5cfa59c` |
| `../../../tests/cli.rs` | `042d6933411fe35f0a9c0b8bcfb22645fc1bc6c9eb6bca3f8c68df9cb3232f4a` |
| `../../../schemas/v2/report.schema.json` | `805de93d2a50339370e803b7ff1a9a1b8d34c3028b5b21a1a6f428f591733f90` |
| `../../../schemas/v2/comparison.schema.json` | `4f2dd6886056d335c3d4209c3a47dbfab2685a8424b4b8153d8d02fd77f9a104` |
| `../../../schemas/v2/inspection.schema.json` | `6677b1689d581744636cf480cea690a55c566415e794f995e34c34f1b9da3883` |
| `../../../schemas/v2/receipt.schema.json` | `6f20777251015af381ad321030490c9c28996a165a4528e8b739d40227cc9637` |
| `../../../schemas/v2/error.schema.json` | `a1be234949560a90dff9391a0fa9c42de4f0431c5c4e2f20d745730f40eb80d5` |
| `../../../Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `../../../Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |
| `../../../rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |

## Focused completion evidence

- `InspectionResult.source` is resolved exactly from the selected raw
  prediction's `source_id` against the replay-verified stored report source map.
  Raw prediction, opaque input and evaluation configuration remain separate.
- The strict schema requires the existing source-definition layout for each legal
  task kind, including optional observations, preparation and stored evidence
  paths without original-source dereference.
- S15 proves both complete inspected Q1/Q2 origins and retains comparison origins;
  E08 proves observation/preparation/stored bindings; the existing multi-label
  abstention inspection proves the second task kind.
- Relocation and privacy regressions retain the new field after original inputs
  disappear. All focused filters pass. Full locked tests pass 44 library, 11 CLI,
  91 conformance and zero doc tests. Format, release, diff and plan checks pass.
  Clippy reports only the accepted 17+3 staged diagnostics.

The verifier must recheck only R1/source resolution plus affected schema,
relocation, privacy and both-task evidence. The other five repaired findings and
settled T026 behavior remain accepted.
