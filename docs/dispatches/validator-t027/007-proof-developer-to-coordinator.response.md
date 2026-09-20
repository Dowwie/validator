# T027 output2 exact structure and offline proof handoff

Status: complete. This output changed only `tests/conformance.rs` and `tests/cli.rs`.
No production defect was reproduced, so no production source was edited.

## Starting baseline reconciliation

All ten starting identities in dispatch007 matched before editing, including
output1 response002 and the eight protected production files. The production
hashes remain unchanged:

| Artifact | SHA-256 |
|---|---|
| `src/model.rs` | `992adf37259839f71dbffa931ac1a341894e90f29283bf34f0f393f1a762ce56` |
| `src/model/common.rs` | `f5f626654b33c63255de26c0e4a07ba79696b980042fa240e2741d73e54bfc8c` |
| `src/model/single_label.rs` | `1acd5af54ef01b2cc49d6e8c9c7a973ee04e529e4000e4cf0b2edf8fc1a10222` |
| `src/model/multi_label.rs` | `43e80100dd9a5c59232ac48fe1925c6bd1d4f2192b86b492027052f8776bd873` |
| `src/validation.rs` | `1ae4707c48c2310e45340ccbc24109eee616d74eaf03a664c247905ba840ebbb` |
| `src/validation/wire.rs` | `1a6d6887dc3ed2fd1a02c337592cf43dd97b59c2dcf6f361459e396634f27ee4` |
| `src/app.rs` | `f741c03b17788303c7ae79ff43a11f9605cb63a6bc565a98d40f5f55b131d64b` |

| Changed artifact | Before | After |
|---|---|---|
| `tests/conformance.rs` | `d0be253475cc718ad7ed67d50e9096b7dc2a4a5a1ddccdb096662e06d4f05b00` | `e3e092d6bd6d0a7fff53751e22974c1e3e11904a1c9b4c11e64a5aa3c2f5bc53` |
| `tests/cli.rs` | `042d6933411fe35f0a9c0b8bcfb22645fc1bc6c9eb6bca3f8c68df9cb3232f4a` | `ecd69a603a69c786256dba05d9057a6641d9ed412aae7330bc32400ed80ed99b` |

## Exact behavioral proof

The thirteen added named filters are nonzero. They call complete existing public
API or built-process scenarios with their discriminating assertions; the sole
new inline rejection is the exact `[0.5, 0.49]` scoring vector needed to pair
the retained observed vector with the invalid scoring representation.

| Filter | Behavioral evidence executed |
|---|---|
| `structure_dm01` | `case_s17` and `case_m20`: both task-family public evaluate/report flows preserve ID and canonical-order behavior through shared admission, orchestration, artifact snapshot, and report paths. |
| `structure_dm02` | `case_m14` and `policy_preconditions`: cross-kind target, categorical/marginal probability, and decision-policy inputs reject before scoring. |
| `structure_dm03` | `case_m20` proves reordered multi-label submissions preserve aggregate scores and canonical report order; `case_m13` rejects duplicates before label-set conversion. |
| `structure_dm04` | `case_m05` retains independent marginals and scores them as binary values; `case_m14` rejects categorical vectors in multi-label scoring. |
| `structure_dm05` | `case_m16` distinguishes an answered empty set, abstention, missing row (`E_ALIGNMENT`), and missing marginal key (`E_PROBABILITY`). |
| `structure_dm06` | `empty_intersection_availability` restricts both checked task evaluations to zero shared episodes and proves probability/probability, none/none, and mixed availability retain their applicable `no_data` or `not_applicable` status. |
| `structure_dm07` | `multi_label_abstentions_preserve_wire_evidence_in_report_and_inspection` retains abstention distributions/observations in report and inspection; `decision_policy_boundaries` checks selected versus raw-answered signal population and included IDs. |
| `structure_dm08` | `case_e06` round-trips observed categorical `{A: 0.5, B: 0.49}` without normalizing or scoring it; the inline public evaluation asserts the identical categorical scoring vector returns `E_PROBABILITY`. |
| `structure_dm09` | `case_e05` and `case_e10` retain scalar/auxiliary observations with source definitions and provenance while probability metrics remain not applicable until an actual probability vector is supplied. |
| `structure_dm10` | `case_e11` rejects unknown definitions, definition/value-kind mismatch, invalid values, unknown kinds, and invalid preparation evidence bindings before output publication/scoring. |
| `structure_dm11` | `equal_counts_distinct_exact_sets` proves equal per-label TP/FP/FN/TN aggregates can have exact-set accuracies of `1/2` and `0/2`. |
| `structure_dm12` | `case_m13`, `case_m14`, `case_m15`, and `case_m21` reject duplicate/partial label coverage, mixed target/outcome task kinds, scored-choice and confidence extensions in multi-label, unsupported outcome fields, and malformed wire shapes. |
| `offline_cli_contract` | Uses Cargo's `CARGO_BIN_EXE_validator` executable and saved temporary public synthetic inputs to execute check/evaluate/inspect and compare for both task kinds. No installed binary, credential, network, or private input is used. |

Focused individual commands all exited 0 with one test executed: each
`cargo test --locked --test conformance structure_dm01` through
`structure_dm12 -- --nocapture`, and
`cargo test --locked --test cli offline_cli_contract -- --nocapture`.
The commands reported 102 filtered conformance tests and 11 filtered CLI tests,
so every named filter executed nonzero.

## Required static structure review

`src/cli.rs::run` maps the shared public command names `check`, `evaluate`,
`inspect`, and `compare` directly to the four `src/app.rs` public
operations. `src/app.rs::{check,evaluate,inspect,compare}` receives one closed
`ValidatedTask`/`VerifiedRun` sum type and dispatches explicitly between
single-label and multi-label branches. Both evaluation branches use `admit`,
task-specific pure evaluator/report builders, and the same `publish_run`
artifact operation. Inspection rebuilds the validated run and retrieves the
selected episode input from task-specific aligned rows before disclosure.

`src/validation.rs::validate` dispatches on the private tagged wire `Task` enum;
`validate_single_label` only accepts class targets/categorical outputs and
`validate_multi_label` only accepts label-set targets/marginals. The latter
also rejects every non-classifier source. Both paths construct their concrete
`Episode<Target>` and `Prediction<Output>` rows before their checked
evaluation objects. `SingleLabelEvaluation` and `MultiLabelEvaluation` own
distinct policy, row, output, signal/marginal, and restricted-evaluation types.
Their `restrict` methods preserve already-admitted probability availability when
the intersection becomes empty. No task trait, registry, extension hook, public
wire-model export, or source-text test was introduced.

## Test inventory and mapping reconciliation

`cargo test --all-features --locked -- --list` exits 0 and lists 44 library, 12
CLI, 103 conformance, and 0 doc tests. The accepted pre-output2 inventory was 44
library, 11 CLI, 91 conformance, and 0 doc tests; the additions account for the
expected +1 CLI and +12 conformance names. Every planned T027 name appears in
Cargo's inventory. `ruby docs/plans/validator/verify-plan.rb` exits 0 and
reports 35 task contracts, 433 source blocks, 63 conformance cases, 12 structure
checks, 8 ACs, 5 DoD clauses, physical owners, local links, and artifact index.

## Final gates

All commands exited 0:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
sandbox-exec -p '(version 1) (allow default) (deny network*)' cargo test --all-features --locked --offline
git diff --check
ruby docs/plans/validator/verify-plan.rb
```

The full ordinary and network-denied suites both passed 44 library, 12 CLI, 103
conformance, and 0 doc tests. Clippy completed warning-free with `-D warnings`.
The sandboxed command used Cargo offline mode and denied all network access.

## Out of scope

T027 claims no T028-T035 result: T028 frozen private acceptance inputs/source
identities; T029 native Choice/Score preparation; T030 independent native hard
oracle; T031 synthetic multi-label practical bundle/oracle; T032 installation
and machine workflow; T033/T034 fresh-agent installed-executable acceptance; and
T035 final evidence/definition-of-done audit all remain pending their own
authority and evidence.

