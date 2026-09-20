# Final repaired T025 candidate manifest

Status: frozen for the same verifier's final focused recheck after the
owner-authorized per-label population-unit correction. This manifest supersedes
manifest014 for `src/evaluation/multi_label.rs`, `tests/conformance.rs` and the
updated governance mappings. Every listed file remains read-only. T026 and later
work remain paused.

## Governing and correction chain

| Artifact | SHA-256 |
|---|---|
| `014-coordinator-repaired-manifest.md` | `2e908298bef177b9af9ffdd30f2562ad0b4a4f612dd43ec9a24a1e18a15a8e8e` |
| `015-verifier-to-coordinator.response.md` | `ce4293236799e897babf0e1c9edb7f8a432775f58ea00c0c1b533af5d37bfc72` |
| `016-owner-to-coordinator.prompt.md` | `0a129c1fc80602aebd2a133b6cfb945e708a9fcfb33511d43b27d19f9f35ce07` |
| `016-coordinator-to-owner.response.md` | `bd3deebd6a2151754ba5e40ec12369d9146654603c83b379b8a329b47623f1f7` |
| `017-coordinator-to-numeric-developer.prompt.md` | `9d781b1ef3e842c9a9a6d6874212907f8ce5804a3c050added15bbfae9c05097` |
| `018-owner-to-coordinator.prompt.md` | `921db6a2deefaab3e8fcdb3faa094c983df4e5d5c5dc4698b6ac913028c6e925` |
| `018-coordinator-to-owner.response.md` | `dc25a6227e323f0d984efb7c514d4d3f63ce07951ffd3ccf174e0ed41fd46389` |
| `019-coordinator-to-numeric-developer.prompt.md` | `268d35b167c8f652f1360ef58c2f0adcb276e8cbc6a43485de1b605170894472` |
| `017-numeric-developer-to-coordinator.response.md` | `7114d8862feb20091412b45d91ce1b0ea400441eddd3022a21c51fb9a89fe08f` |

## Governing contracts and mappings

| Artifact | SHA-256 |
|---|---|
| `../../specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `../../specs/validator-data-model.md` | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| `../../plans/validator/tasks/T025.json` | `7bc49b83a565c7abcbfcebd8e5336903820aec00e185783ad8e449aedceead71` |
| `../../plans/validator/tasks/T027.json` | `a53eed042464e0e0ecb263747f53c2933f508701e10b679a9cc4f685c0a308b7` |
| `../../plans/validator/physical-map.json` | `a15159a581d008a962df99501cc64a7bf708d2c3c6b66b7c92c944410a961563` |
| `../../plans/validator/coverage.json` | `efe2f425fc975f953c5765d1ea40ed47ffea8f672249d85f4e3eb07c9843e896` |

## Frozen final files

| Artifact | SHA-256 |
|---|---|
| `../../../src/evaluation/multi_label.rs` | `c4158facd454e66e66b3f1ddc0bb9976ca07fca6961bf200597ba9b31107b461` |
| `../../../tests/conformance.rs` | `13409645624e53b8a3f2b4e5719dd7ae651527e0c4f886dcb511bfa10bf38bef` |
| `../../../tests/fixtures/single-label/expected.json` | `ab93c379b9c0df1d0c0fb7aac7a6df5e7748f3f671d55dd69d173e640090b268` |
| `../../../tests/fixtures/multi-label/expected.json` | `213183a46c0f6c011296ef5087b5c44d3852081bdf20eaa80cbb494d3d2c84f2` |
| `../../../src/model/multi_label.rs` | `3f515a0a9122db963b2e01ca14d5474b4836ba3543a48a4e0766a4459eea0b7c` |
| `../../../schemas/v2/report.schema.json` | `805de93d2a50339370e803b7ff1a9a1b8d34c3028b5b21a1a6f428f591733f90` |
| `../../../schemas/v2/inspection.schema.json` | `2c7e064d0adb915b77c558a7ebbfade96f7bcf0eee890b881ced1383f0eedaef` |
| `../../../tests/cli.rs` | `8c7b9c00d777f4a290911dd77199d8e1b9e085cc14fb0dd78a801108c3a10fcf` |
| `../../../Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `../../../Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |
| `../../../rust-toolchain.toml` | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |

## Final correction evidence

- Only the three per-label precision/recall/F1 calls changed from
  `answered_episode_metric` to the existing `answered_label_metric`, passing `G`
  as the per-label label-decision population. No new helper or formula exists.
- Raw/final normal answered M01, undefined M02, all-abstained M04 and empty
  selection prove population `G`, unit `label_decision`, scope `answered` for all
  three per-label metrics. Aggregate metrics retain `G*K` label decisions and
  set-level metrics retain episodes.
- M02 explicitly proves present integer-zero numerator and denominator fields for
  each undefined per-label F1.
- The existing persisted report plus real verified-inspection regression proves
  the corrected metadata survives saved/recomputed operation.
- Affected filters, both exhaustive filters, umbrella and persisted-inspection
  filter pass; list remains 74. Full locked tests pass 44 library, 10 CLI, 74
  conformance and zero documentation tests. Format, release and diff pass.
- Warning-denied Clippy retains only the accepted unchanged 17 production plus
  three duplicate lib-test staged diagnostics.

All prior independent numerical, abstention-wire, S12 binary64, M14 target and
schema evidence remains unchanged. The verifier must recheck only the corrected
unit/operand boundary and justified affected regressions, then stop.
