# Frozen T021-T023 candidate manifest

Status: frozen for the single independent T021-T023 review authorized by
`001-owner-to-coordinator.prompt.md`. Implementation, tests and schemas remain
read-only until the verifier saves a verdict or the coordinator issues one
finding-bound repair prompt.

## Governing handoffs

| Artifact | SHA-256 |
|---|---|
| `001-owner-to-coordinator.prompt.md` | `73f5ebe3a192a0baedf9787b55a356d49b03211fa093edfcabf47034cecc15b9` |
| `002-t021-developer-to-coordinator.response.md` | `33be455a22ae14c936121cf28c4e7de1ab0ef1c6fcd7daa20d4a4d75a5d71a6e` |
| `006-t022-developer-to-coordinator.response.md` | `9cd51b915ba9ddd149624f9bb277c9577763182674033da98c4335ba74f956b5` |
| `007-t023-developer-to-coordinator.response.md` | `e8fb439184858803b498d6e34042b1b9d1b04d1b02084364fa02984fc153d8d5` |

## Frozen implementation and evidence

| Artifact | SHA-256 |
|---|---|
| `src/model/common.rs` | `5b91a1aaad30127f63030d885338e8d54d807d5682cf3c077525102167c0e9cc` |
| `src/model/single_label.rs` | `4ec2c89ce43f30a79bdb8054c106608257613491415710e56f3139cfbf748ea0` |
| `src/model/multi_label.rs` | `a0d6b55a7930b4acb1c9379d98b77d66eddc3386788eba3fdfb351cb28402c70` |
| `src/validation.rs` | `fb6148069b4ade27eab57917e0111211274f3aa31c45b4458d3fa1b76a5482a0` |
| `src/evaluation/single_label.rs` | `f0cad620c80d8a1b7cef1847d49179c1b028f33546d40c5fc0b38cf5f002938c` |
| `src/evaluation/multi_label.rs` | `ff11dc9c2afbb5d1387051e245fd9532a90171a8a2c0086e1ea6b0fa1e8de8f3` |
| `src/comparison.rs` | `4ed45a737ed4ea49fc3c88728b83156acc9938f696078c193bc53979b530f845` |
| `src/app.rs` | `aeee8c0bccde849a769b1a3afc02ac626b8b23565559a28d28979e980a3c7391` |
| `src/cli.rs` | `ed6424fa545b1ccb8418cfc008abe466c7953b9ecad17b0c2f494fc2d4c06969` |
| `src/artifacts.rs` | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |
| `schemas/v2/report.schema.json` | `4078be102abd69034932d38bda2e0c9c68e98b9bf3f59aa64cef116867a543d3` |
| `schemas/v2/inspection.schema.json` | `039e6060b88042c908c27a34374592d9f64434aa1aac4725da13146b01c0ac82` |
| `schemas/v2/comparison.schema.json` | `005a099c80add634eaa46a8141a402728173436e8acdfc449e8bfb5f75524597` |
| `tests/conformance.rs` | `108cd539f5280acfc3bbb73d61cfd23ccaac03ce8545e631ff82950c02cef92e` |
| `tests/cli.rs` | `9d94d30be6161f00f08e801b93d3062a95aadd4000d6a7f5678613e3e469570d` |
| `Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |

## Local boundary evidence

- T021: both exact policy filters pass; checked policies, preserved raw results,
  task-bound report/inspection output and pre-score rejections are implemented.
- T022: both exact typed-comparison filters and real CLI comparison pass; strict
  task alternatives accept real outputs and reject foreign/missing fields.
- T023: all three exact intersection filters pass, including both task families,
  the six empty-availability cells and real CLI receipt/exclusion behavior.
- Final T023 gates pass: 44 library, 9 CLI and 21 conformance tests; formatting,
  locked release build and diff check pass.
- Warning-denied Clippy exits 101 only for the owner-authorized staged inventory:
  17 production diagnostics and 3 matching lib-test duplicates. No T021-T023
  diagnostic remains. This is an accepted checkpoint limit, not a clean gate.

T024 and later implementation remain paused. The independent verifier may read and
execute checks against this exact candidate but must not modify source, tests,
schemas, governance or this manifest.
