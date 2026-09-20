# Frozen repaired T026 candidate manifest

Status: frozen for the focused independent recheck authorized by owner prompt001.
The sole repair changed only `tests/conformance.rs` to close the six exact-filter
proof gaps in verifier verdict010. Every listed file is read-only until the saved
recheck verdict. T027 and later implementation remain paused. T028's protected
source checkpoint is separate and cannot modify this candidate.

## Repair chain

| Artifact | SHA-256 |
|---|---|
| `001-owner-to-coordinator.prompt.md` | `e9d4036002883fef4d94973d5f4519e5729b67afc26b3854bfbb6fbbae880115` |
| `009-coordinator-candidate-manifest.md` | `fcb905dcee73958340458b88433e5bd43cc2cc78cfadd37470a19ed50a1efb9f` |
| `010-verifier-to-coordinator.response.md` | `06ded06b8faacd28c19ef80d32288d312cc15563ef83a36140763b5ad71e0b45` |
| `011-coordinator-to-evidence-developer.prompt.md` | `15a8d7cfb74df53053c104bcb295d129187cbcc4c343cc7fc9c3d1c17a070bdc` |
| `011-evidence-developer-to-coordinator.response.md` | `728f145f9f3d48ee77e713fe3d76e5123c8b73e79df4ea738a1f89323577c19f` |

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
| `../../../tests/conformance.rs` | `a685073e0e136f0f16167594833c58ef18dc024d6733a9f1d5014d665b59950d` |
| `../../../tests/cli.rs` | `e0bf26cd96c2c06a221572c748e20818e66638e7227cc789e2f62705acca9197` |
| `../../../schemas/v2/report.schema.json` | `805de93d2a50339370e803b7ff1a9a1b8d34c3028b5b21a1a6f428f591733f90` |
| `../../../schemas/v2/comparison.schema.json` | `4f2dd6886056d335c3d4209c3a47dbfab2685a8424b4b8153d8d02fd77f9a104` |
| `../../../schemas/v2/inspection.schema.json` | `2c7e064d0adb915b77c558a7ebbfade96f7bcf0eee890b881ced1383f0eedaef` |
| `../../../schemas/v2/receipt.schema.json` | `6f20777251015af381ad321030490c9c28996a165a4528e8b739d40227cc9637` |
| `../../../schemas/v2/error.schema.json` | `a1be234949560a90dff9391a0fa9c42de4f0431c5c4e2f20d745730f40eb80d5` |
| `../../../Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `../../../Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |
| `../../../rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |

## Repair evidence

- S15 now inspects both fixed Q1/Q2 rows and asserts complete source models and
  configurations on both comparison sides.
- S24 compares the relocated run after all original inputs are absent and proves
  publication of the actual `comparison.json`.
- S26 binds a real evaluation receipt to the absolute `report.json` path and an
  independent exact-byte SHA-256, while retaining comparison-receipt evidence.
- S27 serializes a real public `E_ID` machine error and proves both sentinels are
  absent while retaining successful inspection and report-privacy evidence.
- M17 asserts exact vocabulary order `[A,B]`; M18 independently derives and
  asserts conditional availability, `0.5/0.5`, zero delta, populations and no
  winner/improvement representation.
- All six focused filters, both matrices, the accepted replay filter and the full
  locked suite pass. Inventory remains 44 library, 11 CLI, 91 conformance and
  zero doc tests. Format and diff checks pass. Clippy reports only the accepted
  17 production plus three duplicate lib-test staged diagnostics.

The same verifier must recheck only these six repaired proofs, candidate identity,
and justified affected regressions. Settled T026 behavior and T028 private-source
work are outside the recheck.
