# Frozen T025 candidate manifest

Status: frozen for the single independent T025 review authorized by owner
prompt001. Every file below remains read-only until a saved verdict or the one
authorized finding-driven correction prompt. T026 and later work remain paused.

## Governing and evidence chain

| Artifact | SHA-256 |
|---|---|
| `001-owner-to-coordinator.prompt.md` | `4c6409c55242078377ff37b87343a41b9b997267d3fbf47dac3d9c889d2f0ab0` |
| `001-coordinator-to-owner.response.md` | `8de15bde5bda74bd78250779e07d9c50c3fba87267d4e70664caff2e24197d68` |
| `002-oracle-developer-to-coordinator.response.md` | `43c0dfa849fce1ff3a4625135e1ea0f8b9306628860c75b255203ee4942fb925` |
| `003-coordinator-to-owner.escalation.md` | `1c945b2cddfae942f329c46833c9fb4eb6052359f94c615dad670217ce997255` |
| `004-owner-to-coordinator.prompt.md` | `f1d8599e210e6d5f1ef3b9a02e53076ebc6f92b4a846df59843f0417e6fe8b3a` |
| `004-coordinator-to-owner.response.md` | `6e2731f5f4cb117f62f2a0cdac23c932feb5c109489387a6a4b0f3910e95fe4f` |
| `005-repair-developer-to-coordinator.response.md` | `90122378998d184c8ec882a9e5e2431809c693e12417f5ad52bf87612d65e5ac` |
| `006-oracle-developer-to-coordinator.response.md` | `a328aeab9ad0f7a529fb77ca0ed494b43982601696ea3d7327dc80026ddfb741` |
| `007-case-developer-to-coordinator.response.md` | `7bf593b4e3d7ffe783e0ac8cb77d908c461a2b776e740e084834a7564d8b6bed` |
| `008-numeric-developer-to-coordinator.response.md` | `3433d9e7eeef3719320bd4f30454f00feb4f0089a42fa0c40cc3f2ee2c7dd442` |

## Governing contracts

| Artifact | SHA-256 |
|---|---|
| `../../specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `../../specs/validator-data-model.md` | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| `../../plans/validator/tasks/T025.json` | `c29765ace2e7859665e290a5a7afad8b71d312ae3ff2ef9c4bb01d6aebdb4de6` |
| `../../plans/validator/coverage.json` | `efe2f425fc975f953c5765d1ea40ed47ffea8f672249d85f4e3eb07c9843e896` |

## Frozen T025 files

| Artifact | SHA-256 |
|---|---|
| `../../../src/model/multi_label.rs` | `3f515a0a9122db963b2e01ca14d5474b4836ba3543a48a4e0766a4459eea0b7c` |
| `../../../schemas/v2/report.schema.json` | `805de93d2a50339370e803b7ff1a9a1b8d34c3028b5b21a1a6f428f591733f90` |
| `../../../schemas/v2/inspection.schema.json` | `2c7e064d0adb915b77c558a7ebbfade96f7bcf0eee890b881ced1383f0eedaef` |
| `../../../tests/conformance.rs` | `5dd2fb2b0b0c6adcdc6150466f475fe4395a4b60b107ae3ab03a4ed0397bee6d` |
| `../../../tests/cli.rs` | `8c7b9c00d777f4a290911dd77199d8e1b9e085cc14fb0dd78a801108c3a10fcf` |
| `../../../tests/fixtures/single-label/expected.json` | `ab93c379b9c0df1d0c0fb7aac7a6df5e7748f3f671d55dd69d173e640090b268` |
| `../../../tests/fixtures/multi-label/expected.json` | `213183a46c0f6c011296ef5087b5c44d3852081bdf20eaa80cbb494d3d2c84f2` |
| `../../../Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `../../../Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |
| `../../../rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |

## Construction evidence

- The production-independent exhaustive oracles pass 301 single-label
  populations plus 64 answered multi-label subset pairs and eight whole
  abstentions.
- F04 is independently reconstructed from raw rows. Class-C F1 serializes the
  normative operands `2/4`, while its mathematical value remains `1/2`; accuracy
  is `5/8` and macro-F1 `131/210`.
- Every exact S01-S12, E01-E04/E07 and M01-M15/M20 filter lists and passes
  independently through the public path. `full_numeric_conformance` executes the
  same independent cases without replacing their exact filters.
- The conformance target lists and passes 74 tests. Full locked tests pass 44
  library, 10 CLI, 74 conformance and zero documentation tests.
- Formatting, locked release build and diff check pass. Warning-denied Clippy
  exits 101 only for the accepted unchanged 17 production plus three duplicate
  lib-test staged diagnostics; no T025 diagnostic exists.
- The authorized whole-abstention repair preserves literal
  `status: "abstained"` and null set differences in report and verified inspection
  output, keeps a legal ordinary class named `ABSTAIN`, and closes the two schemas.

The verifier may read necessary surrounding accepted interfaces and run bounded
checks using the existing target directory. It must not modify this candidate,
rewrite expected values, expand the review into T026/T027 or start private-data
acceptance work.
