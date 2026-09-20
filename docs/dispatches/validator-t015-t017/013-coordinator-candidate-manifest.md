# T015-T017 combined candidate manifest

Status: frozen candidate for the single independent T015-T017 review. This is
not an acceptance, release, warning-free or project-completion claim.

## Governing boundary

Owner prompt001 authorizes the atomic T015 replay/inspection, T016 identical-
population single-label comparison and exact T017 steel-thread sequence, followed
by one combined independent review. T018 remains blocked until owner acceptance.

The governing hashes are:

| Artifact | SHA-256 |
|---|---|
| owner prompt001 | `db1ba26b41b411478550ec18e29e93240ff46ea4196b4cd094d186183a1b2142` |
| T015 task | `6b995d0db5d3ceea4f25062dc4b772dbeafc2a9911cf818314313ba8a7da7fca` |
| T016 task | `f5395e18f7b8599ef99a2f277372bc90b8f4e21675d9ab7824f6bc91dc5da838` |
| T017 task | `8ce5fccc8389c5bc15b44fbb148ba822ea576d7d44cfaa70407d20d63c5cfbe4` |
| v1 specification | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| data model | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| master plan | `0be8489b0cf5873ca40ae22b8b5fd39425631390d3d151ad3484e59b6c29aafb` |
| execution contract | `e389163c6aa6e26b0800849293f61b5d718bad7edfdd55aad5ab3ee00aed94f2` |

## Implementation handoffs

| Milestone | Artifact | SHA-256 | Status carried into review |
|---|---|---|---|
| T015 replay/inspection | response002 | `9e1b78221884ecbcab9dfa60e2677bf0b0b279978261f40172d1f511c5fc231c` | Complete subject to correction005. |
| T015 exact computed paths | response005 | `914cfd24a936595720febcb1dbad30bbb81a27f0200d1410acfc6615300a4238` | Complete full-path collision correction. |
| T016 concrete comparison | response010 | `888e4646fe2455acaf690231642c43c152cfe0011546bb34d42cbee0dfb10c3e` | Complete typed boundary subject to compatibility evidence011. |
| T016 compatibility evidence | response011 | `9a74991343606f64728ed7147bdedaee9de5cef1420e9ee1c100392e3f3818db` | Complete valid-run and private-axis evidence. |
| T017 steel thread | response012 | `72a1d10b3e18bf6b8e644957a820a4686510606a46f41648510621e7e40b7015` | Candidate evidence complete, pending this review. |

Response008 is retained history but was superseded by owner009/correction010; it
is not candidate authority. Response010 and response011 together define final
T016 evidence.

## Frozen production and schema files

| Artifact | SHA-256 |
|---|---|
| `src/artifacts.rs` | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |
| `src/app.rs` | `28418611219e65eb32ffcdf6ae470f48bca944f68112e9997ee87c1f134b7ad7` |
| `src/cli.rs` | `ae1d7945252b98eb7b7f5c7312e67661485dddb317fffa4e26f00dd0dfb81695` |
| `src/comparison.rs` | `a0288c9d79f116b03925802a52771dff0e4894506a23701b9b15e6cb36b02961` |
| `src/evaluation.rs` | `dbee2b6c74f60d5375545b746dbfea3ecc311ad55f3183ab02bdc260157bf891` |
| `src/evaluation/single_label.rs` | `fd271a77be2c1c9aa244749cea9a026cc9efb3e0c00f5a263df428c82caaa19d` |
| `src/error.rs` | `5e47f1afde84a682e6d33744133820a43c14a761383717fc91012c63b330abcb` |
| `src/lib.rs` | `8690583df0fffe301262a476fc39d021148cca08c8860f7dfe0ce03b9698c9c7` |
| `src/main.rs` | `ca701aa73cb1d687d9ff086e654bd9b08e6c285a91c9577c3b2d1eb7fc54a3c0` |
| `src/model.rs` | `ceffb61ab5e523fa4bf3161d5db5a6fcd1276137d951982da4382edb79c58c66` |
| `src/model/common.rs` | `5dd0c2652439434ba9496a503c6317d8775cdbc90d8ded7b7dac5d351005d4f3` |
| `src/model/single_label.rs` | `5cff2210c215bc84e6a95777649a6e0700a3547056fbb8a93365f9d1e6b1a019` |
| `src/validation.rs` | `5a76b0ccc8bd838c13e8671cfb063b27888f0f9b20ecb6e5fa6c26a03c8b49bf` |
| `src/validation/wire.rs` | `46a42903c2edbb556b1ae1ee7b3c3d9642e562e6b2de2a6d63a852c584f6a3b3` |
| `schemas/v2/check.schema.json` | `675f6b1faa3fd67b5b42bc42122f7ccf916925920dd59d14ddee886fa90804ad` |
| `schemas/v2/comparison.schema.json` | `24002dce44ffe08c9900c71a2db3a5c81e7b5b274bd05fd647c46468bf2c75d6` |
| `schemas/v2/error.schema.json` | `a1be234949560a90dff9391a0fa9c42de4f0431c5c4e2f20d745730f40eb80d5` |
| `schemas/v2/inspection.schema.json` | `5e7990c05d9f87a29768c73d6d39a1579d39a9c842ce790cb780ff616182fcfa` |
| `schemas/v2/receipt.schema.json` | `d820751593f82ac68cbca2ed381b305383d96d388dee417abcefec03ed3d1c00` |
| `schemas/v2/report.schema.json` | `4ff9c9f1ad59cd2ed73965c9cf154209e6eaf1b6871691c8d5128ca4cbd7ff46` |

## Frozen test, fixture and candidate-evidence files

| Artifact | SHA-256 |
|---|---|
| `tests/conformance.rs` | `bdf526a6a60fde605c6c65e5691fd7b4e397642b0166cf216b1a169f05d3e353` |
| `tests/cli.rs` | `425568409236bc3f6a834a424916b678e2db1c78cf4a5e487d3f6a4f22050dc0` |
| `tests/fixtures/steel-thread/golden.json` | `eacd3362fc6bf571ed9c3547cefbf9190ad79adcc198b047e6c03fa6387387ce` |
| `tests/fixtures/steel-thread/baseline.json` | `e7fa154c47520c4324a3a932ee04904eff26f04058077257498dac37f2679f53` |
| `tests/fixtures/steel-thread/candidate.json` | `3961f960b452cc0aa7e959e77e243f3159390cd42e179b3d8fd467be0b6f1307` |
| `tests/fixtures/steel-thread/config.json` | `3ef2d28326921d818de66bb063037ab861146b514cea5e54de659b14616018b0` |
| `tests/fixtures/steel-thread/expected.json` | `c48b86dd0d416ca496da11e9f87b8e60435d6dd2203f0f62b448f064f0b8ac7e` |
| `tests/fixtures/steel-thread/source-a/evidence.bin` | `37b2f4a1187d75c02c91fd75440bdfae04770a7cd9bd1e5beceb04e06d1c10f2` |
| `tests/fixtures/steel-thread/source-b/evidence.bin` | `717249018fddf920f110df342456d22e4f846f470576883375d934c74f97e309` |
| `docs/acceptance/validator-v1.md` | `98dba75ec4834090132908d28d7066d05ec47918bd180853639597f9ad34cc65` |
| `target/release/validator` | `6fbc83da5f0ec589105802dfdeaf6a3a2e8504b111409164d0c01eed9536ae09` |

The binary hash is reproducibility evidence for the developer run, not a source
artifact. The verifier rebuilds and records its observed binary identity.

## Candidate gates and explicit limit

Developer evidence records:

- T015 focused replay/inspection cases pass;
- all three T016 named filters plus private compatibility-axis evidence pass;
- `steel_thread_end_to_end` passes 1/1;
- format, 40 unit + 6 CLI + 11 conformance tests, locked release build, plan and
  diff checks pass;
- warning-denied Clippy exits 101 only for the exact 24 owner-staged production
  dead-code diagnostics. This candidate is not warning-free. The T027/T035 clean
  gates remain mandatory.

The independent reviewer must recalculate the four-row oracle, inspect the
production paths and schemas, rerun the justified focused/full gates, and return
`Ready`, `Revise` or `Blocked`. A `Revise` finding must name the violated current
criterion, location, reproduction, consequence and smallest correction. No
intersection, multi-label, T018+, release or general audit belongs to this review.
