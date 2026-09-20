# Repaired T021-T023 candidate manifest

Status: frozen for the same independent verifier's single focused recheck. This
manifest inherits every unchanged artifact and hash from manifest008 and binds the
sole authorized repair cycle to the four changed files below.

## Repair chain

| Artifact | SHA-256 |
|---|---|
| `008-coordinator-candidate-manifest.md` | `344c542a392e5387da2bff7a681dfe7f61b1f81c8975cd3e48dc78efe11a68f4` |
| `009-verifier-to-coordinator.response.md` | `23918686aebea8181d7e49196ca72ae35cfdb9e5793821e52614ecfd140ea75b` |
| `012-coordinator-to-repair-developer.prompt.md` | `6b0c0f93ef78b3e996f397d267f2a6afee1e5b4fc94559f22b735c67d7ba942f` |
| `012-repair-developer-to-coordinator.response.md` | `eca345d053562b72d0e826b1f1ce10a8bb260bd2a9b2636bd85f4f5d8bf1a685` |

## Repaired files

| Artifact | Manifest008 SHA-256 | Repaired SHA-256 |
|---|---|---|
| `src/app.rs` | `aeee8c0bccde849a769b1a3afc02ac626b8b23565559a28d28979e980a3c7391` | `f23a95f6c6f4d0f6c33cd872567ef14526243f053673a973ee0021a94d79012a` |
| `schemas/v2/comparison.schema.json` | `005a099c80add634eaa46a8141a402728173436e8acdfc449e8bfb5f75524597` | `8bc3b8b3c481da31725fd41cadf5a61c8a13839c77e882d3483dce9ec06d3288` |
| `tests/conformance.rs` | `108cd539f5280acfc3bbb73d61cfd23ccaac03ce8545e631ff82950c02cef92e` | `486c14e2f63d2f8a05eacbdd65586e6f07ca0d2de6d8718eb788293332366cd1` |
| `tests/cli.rs` | `9d94d30be6161f00f08e801b93d3062a95aadd4000d6a7f5678613e3e469570d` | `26b4a8aa28ebedcf9121679337700965e783e7aaddb90f50c9f56d871a5dc673` |

All other manifest008 hashes remain exact, including `src/comparison.rs`
`4ed45a737ed4ea49fc3c88728b83156acc9938f696078c193bc53979b530f845`.

## Repaired behavior and evidence

- Both concrete app branches use intersection dispatch whenever the explicit
  option is set, including equal nonempty and equal empty selections.
- The strict schema admits zero-exclusion intersection populations through the
  existing scope conditionals and a top-level `anyOf`, while identical scope keeps
  its exact empty-list/zero-count requirement and task alternatives stay strict.
- Real application and real CLI regressions cover single-label/multi-label × equal
  nonempty/equal empty. Each asserts requested scope, exact compared population,
  zero exclusions, concrete recomputed result/status, schema validity and receipt.
- The three affected T023 filters, full 44+9+21 suite, formatting, locked release
  build and diff check pass. Clippy remains exactly the authorized 17 production
  plus 3 duplicate lib-test staged diagnostics, with no repair-owned warning.

The candidate remains read-only during focused recheck. T024 remains paused.
