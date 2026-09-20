# Frozen T026 combined candidate manifest

Status: frozen for the single independent T026 review authorized by owner
prompt001. Every listed candidate file remains read-only until a saved verdict or
the one authorized finding-driven correction. T027 and later implementation remain
paused. T028 may use only the separate protected-source developer window while
this review is active and cannot modify this candidate.

## Governing and evidence chain

| Artifact | SHA-256 |
|---|---|
| `001-owner-to-coordinator.prompt.md` | `e9d4036002883fef4d94973d5f4519e5729b67afc26b3854bfbb6fbbae880115` |
| `001-coordinator-to-owner.response.md` | `eaa064ddc9322cf11079534207dd319301e3aae2ad9c1c4b5c1a1d45c29da888` |
| `004-owner-to-coordinator.prompt.md` | `b2d46e766a4ed65bc9d9f0a5cfe3f356b3baf1a1b6e3e3c6e3584e38a6afa24f` |
| `005-safety-developer-to-coordinator.response.md` | `0f8079ac895875992cc37d30a6da54ed94dee883d1504d4488eb8064bc93bfec` |
| `006-owner-to-coordinator.prompt.md` | `11e31fa021363f0c88f40611ea853645867548203ce9bfb9b6b06328c72a216f` |
| `008-evidence-developer-to-coordinator.response.md` | `c58f8638fb6c6f44d0a1bad3c2e868f14878dd8069c4e54ac2e3f38910f3c334` |
| `../validator-t025/020-coordinator-repaired-manifest.md` | `deeed4c908fa385de1cd8494a31fdcc7fb8d46646e03b32766691fe007f24fc9` |
| `../validator-t025/021-verifier-to-coordinator.response.md` | `a6ca4df6161aea825f5721097a1782b7fc520d5eb3f0febff9f1edb6db9ebc43` |

## Contracts and mappings

| Artifact | SHA-256 |
|---|---|
| `../../specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `../../specs/validator-data-model.md` | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| `../../plans/validator/tasks/T026.json` | `bff0e493602b69d73f05ed0268922a549a804610eab27f0a94c9a4b8669f4975` |
| `../../plans/validator/tasks/T027.json` | `53c6f7f0d1b1e7eabbe2053e7034126a0d8a6773fd1e5dc4be7a454a312f1f57` |
| `../../plans/validator/physical-map.json` | `96cfc6381c66c798081daba75587342f3cec0f903acf9da8e738b60efa4a2f3a` |
| `../../plans/validator/coverage.json` | `efe2f425fc975f953c5765d1ea40ed47ffea8f672249d85f4e3eb07c9843e896` |

## Frozen candidate files

| Artifact | SHA-256 |
|---|---|
| `../../../src/app.rs` | `007c00ac10a4db21f19f7f21cdbd3861c9581fed03c864bc59debd514d54d800` |
| `../../../src/artifacts.rs` | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |
| `../../../src/comparison.rs` | `4ed45a737ed4ea49fc3c88728b83156acc9938f696078c193bc53979b530f845` |
| `../../../src/cli.rs` | `ed6424fa545b1ccb8418cfc008abe466c7953b9ecad17b0c2f494fc2d4c06969` |
| `../../../src/evaluation/multi_label.rs` | `c4158facd454e66e66b3f1ddc0bb9976ca07fca6961bf200597ba9b31107b461` |
| `../../../tests/conformance.rs` | `87230d024ab18e0b3c1b61a82dec7414593e782f8c769d43ef6839fee55f0812` |
| `../../../tests/cli.rs` | `e0bf26cd96c2c06a221572c748e20818e66638e7227cc789e2f62705acca9197` |
| `../../../schemas/v2/report.schema.json` | `805de93d2a50339370e803b7ff1a9a1b8d34c3028b5b21a1a6f428f591733f90` |
| `../../../schemas/v2/comparison.schema.json` | `4f2dd6886056d335c3d4209c3a47dbfab2685a8424b4b8153d8d02fd77f9a104` |
| `../../../schemas/v2/inspection.schema.json` | `2c7e064d0adb915b77c558a7ebbfade96f7bcf0eee890b881ced1383f0eedaef` |
| `../../../schemas/v2/receipt.schema.json` | `6f20777251015af381ad321030490c9c28996a165a4528e8b739d40227cc9637` |
| `../../../schemas/v2/error.schema.json` | `a1be234949560a90dff9391a0fa9c42de4f0431c5c4e2f20d745730f40eb80d5` |
| `../../../Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `../../../Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |
| `../../../rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |

## Combined developer evidence

- Real single-label and multi-label saved-run controls, precise verified inspect
  and compare errors, and absent comparison publication cover snapshot, binding,
  evidence, containment, source-array, result, count/status/configuration and
  tolerance mutations. The accepted replay filter and the T026 artifact matrix
  execute one shared complete scenario.
- The demonstrated multi-label within-tolerance failure is corrected by a closed
  complete-path classifier covering hard, per-label, macro, binary-loss/Brier and
  marginal-bin computed values. Tolerance/constants and single-label behavior are
  unchanged; counts, operands, populations, statuses, configuration, observations
  and structure remain exact.
- `publication_and_privacy_matrix` uses the real binary for both task kinds.
  Relocation, copied evidence, deterministic no-replace, late failure, receipts,
  privacy and exact selected-input/integer inspection are covered by S23-S28 and
  owning seams.
- Exact S15/S16/S18-S20/S22, M17-M19 and E08 cover mixed/changed sources,
  incompatible gold, intersection/exclusions/empty availability, recovery and
  regression IDs, multi-label transitions/answered populations and externally
  bound preparation evidence.
- All 16 exact T026 filters, both matrices and the accepted replay filter execute
  nonzero. Full locked tests pass 44 library, 11 CLI, 91 conformance and zero doc
  tests. Format, locked release and diff checks pass. Warning-denied Clippy retains
  only the accepted 17 production plus three duplicate lib-test staged diagnostics.

The verifier may inspect necessary surrounding accepted code and run bounded
current checks using the existing target. It must not edit the candidate, widen
the review into T027/private-source work, or demand duplicate workflows where
collective API/CLI evidence proves the exact requirement.
