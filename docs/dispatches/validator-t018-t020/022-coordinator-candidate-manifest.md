# T018-T020 combined candidate manifest

Status: frozen candidate for the single independent T018-T020 review. This is not
owner acceptance, release, warning-free or project-completion evidence. T021 and
later work remains prohibited until owner acceptance.

## Governing boundary

Owner prompt001 authorizes atomic T018 checked multi-label admission, T019 hard
accounting and T020 marginal/shared-command integration followed by one combined
independent review. Prompts006,009,012,015 and020 preserve the same technical
criteria while correcting staged-lint wording, replacement sequencing, atomic
completion, stored-evidence replay and the three ordinary lint defects.

| Artifact | SHA-256 |
|---|---|
| owner prompt001 | `f773ee042dfc850e4666e38a2f15d069fca60da8b9699677a663fb05df69c390` |
| lint correction006 | `d126e8be072422229cb80ab28040af96af1a958b0d04c6e920edf8356671bf53` |
| fresh-context decision009 | `f24e5683306ac62874ff00106e6f141166f67420ef58c25a9b63b09bb293b09d` |
| atomic split012 | `298e74535b4c13c9e83aad2192047bfb6b74f48d3ccd0b908f3edb47937b0d91` |
| stored-inspection decision015 | `99ef73875973eae40d64d92d47a208492232e467d632f7d70c056bc61e1c39d3` |
| mechanical-lint decision020 | `687021f7fe70ac9dd45219d6cf78e7bca9c2a23d7708406902cb7dfbc41c5bf4` |
| T018 task | `c0a07aa8b04bec25733dd20c64dbf9444b6412c90db49c4e8a13736c38100c04` |
| T019 task | `0d6be8fa0d8b48cf4372e2a353c860420b75a54107c30446e21c4efd0cdc11f1` |
| T020 task | `22bf755ee6eccf5efa2abeb01a350ce92c5bff17953a4b6bb55c519c253cf50a` |
| v1 specification | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| data model | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| execution contract | `cdd917974df4fd505f91d165039988ca01d0936ad2e7703da47f30c7b3041e8f` |
| Cargo manifest | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| Cargo lockfile | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |
| pinned toolchain | `7553618f800829444096beea3dce84c3d07585efda504c19adb6787369bbd9e8` |

## Complete implementation/evidence handoffs

Incomplete responses005,007 and010 remain honest history and are superseded for
candidate evidence by the complete atomic handoffs below. They are not hidden or
treated as acceptance authority.

| Milestone | Artifact | SHA-256 |
|---|---|---|
| T018 checked admission | response003 | `034a17855aed0330cce60ebfbc73ebf8125e059ae388e742d360850b278bf079` |
| T019 hard accounting/oracle | response004 | `22e2292f457e9d8ba1d748a1f401f7ba182695465bbcf2c99e1e1b50d7a8376e` |
| complete report schema | response013 | `4cf336a5dafe12ddb59a42f51f51f8c2211ff0a209e3589c0513a6e6547184db` |
| check/inspection schemas | response014 | `c13d013e38755b8a49510d120a308e6b39a784abdcae6fc99b71749e1ca0e83b` |
| stored-evidence replay repair | response016 | `9d53993473218e9b77f4a8043f2dda4e9afc1739f444a6012ad585ebb062993c` |
| numerical/CLI evidence | response018 | `55f141be479a3a09ed0454ab3f82181ba7dcb91e78a4c8c7c769dec3d0b30077` |
| final lint/frozen gates | response021 | `e2265bdb490de0a5e8fd543c671704679b8b2179e1842fb309a6dc7cab31d6b7` |

## Frozen source tree

| Artifact | SHA-256 |
|---|---|
| `src/app.rs` | `3324dad26b9571529bdfb1a75b2bb17a9900bbb8f1a65ad822e7b389b76c6671` |
| `src/artifacts.rs` | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |
| `src/cli.rs` | `ae1d7945252b98eb7b7f5c7312e67661485dddb317fffa4e26f00dd0dfb81695` |
| `src/comparison.rs` | `a0288c9d79f116b03925802a52771dff0e4894506a23701b9b15e6cb36b02961` |
| `src/error.rs` | `5e47f1afde84a682e6d33744133820a43c14a761383717fc91012c63b330abcb` |
| `src/evaluation.rs` | `172ab7db055ca9483e3b64593411bb9e64c1983eecdbd06013fe0f6ce1a483ff` |
| `src/evaluation/multi_label.rs` | `013f014ed1300a4e2173bc0a4a187af53717079c7d80a37575932e242287257b` |
| `src/evaluation/single_label.rs` | `fd271a77be2c1c9aa244749cea9a026cc9efb3e0c00f5a263df428c82caaa19d` |
| `src/lib.rs` | `8690583df0fffe301262a476fc39d021148cca08c8860f7dfe0ce03b9698c9c7` |
| `src/main.rs` | `ca701aa73cb1d687d9ff086e654bd9b08e6c285a91c9577c3b2d1eb7fc54a3c0` |
| `src/model.rs` | `846677319091911cd5f1a9a869aa45700b8ce46caf0923e779f74100c71dd4f6` |
| `src/model/common.rs` | `dfe120adb33f7a773edd5a8c44c8331fb918e417aec52dfc3d1c0a8077138791` |
| `src/model/multi_label.rs` | `d0f8a9c7a6c164ae3d34f6111a31c2b52aeead7021c9cf9b25de7d079d7eb46a` |
| `src/model/single_label.rs` | `27d3abef765549fc3f145ed9588f70993cbaa3b152a22754dc45bd277dd2c573` |
| `src/validation.rs` | `c6e40115a863bba54fab8f84b32beba53b56df153de008bb3fbc63e6ea2886fa` |
| `src/validation/wire.rs` | `3c98ed9c193cb8dff0f889956f58a4198b4d8a956a3da34592b45e5e381d29ad` |

## Frozen schemas, tests and oracle

| Artifact | SHA-256 |
|---|---|
| `schemas/v2/check.schema.json` | `1bfd2fb3a644df1203b043244b38a9ed80d404cae5068901b7f162ea6dd662b9` |
| `schemas/v2/config.schema.json` | `c740c7cd2c1381398b3121ee161b1d995a23ceb9dd4325bd73cd0073f66d65c3` |
| `schemas/v2/error.schema.json` | `a1be234949560a90dff9391a0fa9c42de4f0431c5c4e2f20d745730f40eb80d5` |
| `schemas/v2/golden.schema.json` | `97d811a192274f1ee0703e5dff475723301fdcc68c20b031d3b22431968b777b` |
| `schemas/v2/inspection.schema.json` | `3c32ad02a1376b790294bcd655c8c4ffc711a4d90274546303e60577425fc176` |
| `schemas/v2/predictions.schema.json` | `654908068da5d6b3cf9c666cddf4ebc888f94203a04fb6addd5164fbbb893352` |
| `schemas/v2/receipt.schema.json` | `d820751593f82ac68cbca2ed381b305383d96d388dee417abcefec03ed3d1c00` |
| `schemas/v2/report.schema.json` | `7215342a2958fd4d9e9735c2837130a02837d205db93b342b9f64d62a882fcf1` |
| `tests/conformance.rs` | `f184b28026250ec38da1d228d8bf563272142d21c5844ac2ba4e30e9ab8de291` |
| `tests/cli.rs` | `edaf69f7744fbc2df630e280234041e8f9bd0608ed1c2bdd719187090ca85275` |
| `tests/fixtures/multi-label/expected.json` | `a65de157415f75567cb907f2dd45ae3d524dcb552dce7c10ef9ca87efbfc63ec` |
| developer release binary | `d7878036835693417283483dfa7b87fbe204fe39e592463368c588121bd81b41` |

The binary hash is developer-run reproducibility evidence, not a source artifact.
The verifier rebuilds and records its observed binary identity.

## Candidate gates and explicit limit

Developer evidence records:

- all six exact T018-T020 filters exist, each selects one test and passes;
- full locked tests pass: 44 library, 7 CLI, 15 conformance, 0 doc failures;
- formatting, release build and diff checks pass;
- warning-denied Clippy exits 101 only for the exact 20 owner-authorized staged
  production `dead_code` diagnostics recorded in response021. No ordinary warning,
  suppression, fake use, widened export or changed tolerance remains.

The independent verifier must reconcile every manifest hash, inspect the concrete
admission/accounting/probability/application/schema/replay paths, independently
recalculate the hard and marginal oracles, rerun justified focused/full gates and
return `Ready`, `Revise` or `Blocked`. A `Revise` finding must cite a current
criterion, exact location, reproduction, consequence and smallest correction.
T021 policy execution, T022 multi-label comparison, intersection, future staged-
warning cleanup and unrelated design/style work are outside this review.
