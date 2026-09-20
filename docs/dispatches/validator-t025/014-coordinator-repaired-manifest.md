# Repaired T025 candidate manifest

Status: frozen for the same verifier's bounded recheck after the single authorized
test-only repair cycle. This manifest supersedes manifest009 only for the
`tests/conformance.rs` identity and the added decision/repair chain. Every listed
file remains read-only. T026 and later work remain paused.

## Governing, finding and repair chain

| Artifact | SHA-256 |
|---|---|
| `001-owner-to-coordinator.prompt.md` | `4c6409c55242078377ff37b87343a41b9b997267d3fbf47dac3d9c889d2f0ab0` |
| `004-owner-to-coordinator.prompt.md` | `f1d8599e210e6d5f1ef3b9a02e53076ebc6f92b4a846df59843f0417e6fe8b3a` |
| `009-coordinator-candidate-manifest.md` | `8071c565c7c4f544cf3072c993df11ea70d3f80a71cbc85c28fd93a99aea6076` |
| `010-coordinator-to-verifier.prompt.md` | `6ce3c2ff6397d67743db9518434104a3ffd8291c4674e6c3bd04f8db5658eb67` |
| `010-verifier-to-coordinator.response.md` | `9b952709641bcbbe89f2be5cc032be318fad1a37483e8b4a58875a67a15f6faa` |
| `011-coordinator-to-numeric-developer.prompt.md` | `fcea3f4e478500d999e438398504e060202915c8acfe3cb92773d0bd6b9a2948` |
| `012-owner-to-coordinator.prompt.md` | `85443e02a34f53108b27f67fefd81ea48606284b2b253e6ac5b139fce07261c9` |
| `012-coordinator-to-owner.response.md` | `8dbd9021f736414e8a3164d41aca71a8c1866fabfdf13272293149c49e122a2d` |
| `013-coordinator-to-numeric-developer.prompt.md` | `1b5206e7291715a3c006aca4d1b4616e3d40c1c0d4a9b8bcd2f5a807068c4772` |
| `011-numeric-developer-to-coordinator.response.md` | `edd3ab7824be3c0c9b0aa6a229ad9efb840a2919a9bcf43bfd4825a4ff163252` |

## Governing contracts

| Artifact | SHA-256 |
|---|---|
| `../../specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `../../specs/validator-data-model.md` | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| `../../plans/validator/tasks/T025.json` | `c29765ace2e7859665e290a5a7afad8b71d312ae3ff2ef9c4bb01d6aebdb4de6` |
| `../../plans/validator/coverage.json` | `efe2f425fc975f953c5765d1ea40ed47ffea8f672249d85f4e3eb07c9843e896` |

## Frozen repaired files

| Artifact | SHA-256 |
|---|---|
| `../../../tests/conformance.rs` | `40b9d231e3c77416d9c0f6bc93784bd309d45fa1cd6fe8a3eea6327f27ed3cd1` |
| `../../../tests/fixtures/single-label/expected.json` | `ab93c379b9c0df1d0c0fb7aac7a6df5e7748f3f671d55dd69d173e640090b268` |
| `../../../tests/fixtures/multi-label/expected.json` | `213183a46c0f6c011296ef5087b5c44d3852081bdf20eaa80cbb494d3d2c84f2` |
| `../../../src/model/multi_label.rs` | `3f515a0a9122db963b2e01ca14d5474b4836ba3543a48a4e0766a4459eea0b7c` |
| `../../../schemas/v2/report.schema.json` | `805de93d2a50339370e803b7ff1a9a1b8d34c3028b5b21a1a6f428f591733f90` |
| `../../../schemas/v2/inspection.schema.json` | `2c7e064d0adb915b77c558a7ebbfade96f7bcf0eee890b881ced1383f0eedaef` |
| `../../../tests/cli.rs` | `8c7b9c00d777f4a290911dd77199d8e1b9e085cc14fb0dd78a801108c3a10fcf` |
| `../../../Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `../../../Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |
| `../../../rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |

## Repair evidence

- Every direct-ratio assertion now checks independently expected population count,
  unit and scope; exhaustive single/multi and S03 pass, with selected versus
  answered episode populations distinguished.
- S12 performs 29 isolated public evaluations. Each asserts count one in its
  independently expected bin and zero in the other nine, with the owner-corrected
  `next_down(0.9)` mapping to bin 9 under binary64 arithmetic.
- M02 asserts both label identities, binary counts/supports and null
  `undefined_zero_denominator` F1 `0/0` with answered population metadata.
- M14 now executes the required multi-label golden class target and receives
  `E_CONFIG` without publication.
- Both exhaustive filters, S03, S12, M02, M14 and the umbrella pass. The inventory
  remains 74 conformance tests. Full locked tests pass 44 library, 10 CLI, 74
  conformance and zero documentation tests. Formatting and diff check pass.
- Warning-denied Clippy retains exactly the accepted 17 production plus three
  duplicate lib-test diagnostics and no repair-owned warning. Unchanged release
  evidence from response008 remains applicable.

The same verifier must recheck only these four corrections and justified affected
regressions, explicitly correct verdict010's illustrative S12 vector, and reuse
all unchanged evidence. No new review cycle or optional coverage is authorized.
