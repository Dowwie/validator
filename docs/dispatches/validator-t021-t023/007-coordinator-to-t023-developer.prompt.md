# Implement T023 validated intersection recomputation

Role/model: fresh sole implementation developer, `gpt-5.6-terra`, high
reasoning, `fork_turns: none`. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #185 — Build Validator policies and comparison core](http://localhost:3006/1/cards/185).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t021-t023/007-t023-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat return only the
path, SHA-256 and terse status. Do not edit governance, plans, artifact-index,
session notes, acceptance records or Fizzy. Do not delegate.

Read global and repository `AGENTS.md`, the Rust best-practices skill, the complete
ratified T023 task, the full linked comparison and validated-data sections of both
specifications, owner prompt001, and final T021/T022 handoffs. Read current
single-label and multi-label validated structures, evaluators, typed comparison,
application/CLI publication, strict comparison schema and tests before editing.
T024 and later work remain prohibited.

Starting identities use exact paths:

| Artifact | SHA-256 |
|---|---|
| `docs/plans/validator/tasks/T023.json` | `db1f42cc90c937ea82d15426a5f728c9e2efe41596d386aff00c008d5635bef2` |
| `docs/specs/validator-v1.md` | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| `docs/specs/validator-data-model.md` | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| `docs/plans/validator/execution-contract.md` | `cdd917974df4fd505f91d165039988ca01d0936ad2e7703da47f30c7b3041e8f` |
| `docs/dispatches/validator-t021-t023/001-owner-to-coordinator.prompt.md` | `73f5ebe3a192a0baedf9787b55a356d49b03211fa093edfcabf47034cecc15b9` |
| `docs/dispatches/validator-t021-t023/006-t022-developer-to-coordinator.response.md` | `9cd51b915ba9ddd149624f9bb277c9577763182674033da98c4335ba74f956b5` |
| `src/model/common.rs` | `dfe120adb33f7a773edd5a8c44c8331fb918e417aec52dfc3d1c0a8077138791` |
| `src/model/single_label.rs` | `cc6a448ef4ca162aa3353e72084cb93a1f2c91d47e12b086463c8046b279976f` |
| `src/model/multi_label.rs` | `19f6716d1bcf9e33ce47951e676de4973001a2cafe450bdf24cafa79367a1592` |
| `src/validation.rs` | `fb6148069b4ade27eab57917e0111211274f3aa31c45b4458d3fa1b76a5482a0` |
| `src/evaluation/single_label.rs` | `f0cad620c80d8a1b7cef1847d49179c1b028f33546d40c5fc0b38cf5f002938c` |
| `src/evaluation/multi_label.rs` | `ff11dc9c2afbb5d1387051e245fd9532a90171a8a2c0086e1ea6b0fa1e8de8f3` |
| `src/comparison.rs` | `a2cdaa46d4e008261e3b9e75edf55c6e4f54a51737fe6287250e77e14fa8f4b0` |
| `src/app.rs` | `e102572012bc0204a0312b88607914700486bcc020925cd84ab6deaba0810d5f` |
| `src/cli.rs` | `ae1d7945252b98eb7b7f5c7312e67661485dddb317fffa4e26f00dd0dfb81695` |
| `src/artifacts.rs` | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |
| `schemas/v2/comparison.schema.json` | `4bf70998e7ff0117a8a09853213d118cf2e35e6abac11c3ba22e439e8771ab2d` |
| `tests/conformance.rs` | `290dc063b95da1a9f65f5ba79ff6cffeaa84d0d539617d12a056e28523034d62` |
| `tests/cli.rs` | `4df3d3bf0c1e1fa6e75d4710e1f163c5d5cf48a2259554fc3c35db1a04131228` |

Stop and save the exact mismatch before editing. Otherwise implement only T023.

## Required validated restriction and comparison

- Add a checked restriction operation on each existing validated concrete
  evaluation. It accepts only IDs already selected, preserves task, ordered
  vocabulary, legal policy, source/provenance ownership and the original
  artifact-wide `SignalAvailability`, and rebuilds aligned rows once. Do not
  synthesize or re-decode prediction JSON, construct unchecked vectors, subtract
  aggregate results or infer signal applicability from restricted row count.
- Default comparison still rejects unequal selected-ID sets. The explicit
  intersection option permits only that selection difference. Golden digest,
  concrete task, ordered vocabulary, role and numerical semantics remain required;
  mixed task kinds remain rejected.
- Compute sorted common IDs and deterministic baseline/candidate excluded IDs,
  then evaluate both restricted concrete structures with their original policies
  and compare those recomputed results through the existing typed paths.
- Report explicit `intersection` scope, common and both excluded lists/counts,
  source attribution for the compared population, original run/report identities,
  raw/final answered populations/overlap and paired typed results. The targeted
  scope must make no full-population improvement, winner or significance claim.
- Empty intersection is valid. Hard results are `no_data`. For each side and each
  task independently, an originally applicable probability/marginal family becomes
  `no_data`; an originally absent family remains `not_applicable`. Mixed availability
  preserves the different statuses and every undefined paired metric has null delta
  with the correct reason. Observation presence never changes applicability.
- Wire the existing compare application and CLI with explicit `--intersection`,
  including normal duplicate/unknown/missing-value flag behavior, verified replay,
  atomic publication and receipt. Preserve default comparison and both accepted
  task-specific output alternatives.
- Extend the strict comparison schema only for the concrete identical/intersection
  population alternatives required by actual output. Preserve task discrimination,
  required fields and `additionalProperties` rules; validate real API/CLI output.

## Required evidence and gates

Implement and run these exact nonzero real-path filters:

```text
cargo test --locked --test conformance intersection_recomputation -- --nocapture
cargo test --locked --test conformance empty_intersection_availability -- --nocapture
cargo test --locked --test cli cli_intersection -- --nocapture
```

`intersection_recomputation` must prove both concrete task families score the
shared IDs rather than reuse/subtract original aggregates, with exact sorted
common/excluded IDs and original policy/source/provenance retained. The empty case
must cover original probability/probability, label-or-set/label-or-set and mixed
availability for both tasks, with independent statuses and null reasons/deltas.
The CLI case must prove default mismatch rejection, explicit flag success, real
published/schema-valid output, receipt digest, exclusions and immutable output
refusal. Add focused regressions for identical-population default compare and
accepted opaque replay/evidence behavior where affected.

Run `cargo fmt --all -- --check`, all locked tests, warning-denied Clippy, release
build and `git diff --check`. The exact authorized staged inventory may remain;
real T023 consumers should remove only their actual diagnostics. No new warning,
suppression, fake use, export widening, cleanup or future-task implementation.

Response007 must map every criterion to production code and concrete assertions,
state independent expected recomputed values and every empty-availability matrix
cell, list commands/exits/counts, exact final hashes and exact residual lint
inventory. A complete handoff freezes the combined T021-T023 candidate for one
fresh independent Sol-high review; no separate owner gate occurs first.
