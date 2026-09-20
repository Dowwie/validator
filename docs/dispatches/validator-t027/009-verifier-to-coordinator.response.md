# T027 independent structural/offline verdict

Verdict: **Revise**.

The frozen candidate passes every required behavioral filter, inventory check,
format/lint/test/build gate, and the network-denied offline suite. Its checked
task construction, opaque-input ownership, strict wire admission, task-specific
evaluation types, inspection replay boundary, and DM01-DM12 behavior are supported
by direct source review and execution. One current-contract module-direction
defect remains: both task model modules depend on the filesystem-owned
`artifacts::InputArtifacts` type. That violates the normative package boundary
and prevents a Ready verdict.

## Frozen identity

Before review, candidate manifest008 had SHA-256
`1fb631264e5b6a396031be184ab6eb320786529cb53ff00aa976f50d77d01fa5`.
Every artifact hash listed in manifest008 reconciled. After all review commands,
manifest008 retained the same SHA-256 and every listed artifact still matched;
the post-run reconciliation reported zero mismatches.

I read the T027 contract, owner prompts001/004, both complete developer responses,
the linked specification and data-model sections, the execution contract, and the
accepted T026 source-field baseline. I also applied completion prompts002/003's
scope and stopping rule. Review stayed within the changed model, validation,
inspection and test code, necessary callers, twelve DM rows, exact inventory, and
listed gates.

## Required finding

### F1 — model depends on the I/O/artifact module

- **Requirement:** `docs/specs/validator-v1.md:1085-1092` requires `app` to
  coordinate the modules and states that `model` must not depend on application,
  scoring, or I/O modules. The responsibility table at lines 1073-1078 assigns
  coordination to `app`, typed records/reports to `model`, and filesystem reads,
  hashing, copying, integrity checks, and publication to `artifacts`.
- **Location:** `src/model/single_label.rs:6` imports
  `crate::artifacts::InputArtifacts`; its `assemble_report` signature at
  `src/model/single_label.rs:761` accepts `artifacts: &InputArtifacts` at line
  767. `src/model/multi_label.rs:3` has the same import, and its
  `assemble_report` at `src/model/multi_label.rs:477` accepts the same artifacts
  type at line 483. `src/app.rs:202-210` and `src/app.rs:229-237` pass the
  filesystem module's aggregate directly into those model functions; replay
  callers do likewise.
- **Reproduction:**

  ```text
  rg -n 'use crate::artifacts::InputArtifacts|artifacts: &InputArtifacts' \
    src/model/single_label.rs src/model/multi_label.rs
  ```

  Actual result: two production model imports and two production report-builder
  parameters cross from `model` into `artifacts`.
- **Expected:** task model/report construction may receive model-owned values,
  while `app` owns orchestration and `artifacts` owns filesystem state. No model
  module imports a type from the I/O module.
- **Actual:** both task model modules have a compile-time dependency on the
  private filesystem/artifact aggregate solely to read its three model-owned
  snapshots when constructing the report manifest.
- **Consequence:** the actual intra-crate dependency direction contradicts a
  T027 acceptance requirement that Cargo cannot enforce. It couples canonical
  model/report code to the filesystem ownership module and makes the candidate's
  claim that module directions remain intact false. Passing behavior and Clippy
  cannot establish this static property.
- **Smallest correction:** remove the two `InputArtifacts` imports and change the
  two report-builder signatures to receive only the three required model-owned
  `ArtifactSnapshot` references (or an equivalent model-owned manifest value).
  Have `app` extract and pass those values from `InputArtifacts` at the existing
  evaluation/replay call sites. Preserve report JSON, publication, replay,
  schemas, tests, dependencies, and all numerical behavior. Then rerun the
  focused report/replay tests and the listed T027 gates.

## Structure and ownership review

- `validation/wire.rs` keeps the DTOs private, uses explicit tagged enums and
  `deny_unknown_fields`, retains opaque JSON in `RawValue`, rejects duplicate
  keys recursively before Serde decoding, and preserves arbitrary-precision raw
  numbers. The three decoded top-level documents carry concrete `u8`
  `schema_version` fields, and each decode entry enforces version 2 before the
  value is exposed.
- `TaskDefinition`, `SingleLabelTask`, and `MultiLabelTask` perform checked
  vocabulary admission. `validation` constructs the appropriate closed task
  wrapper and rejects the opposite target/output/policy family. No token-only
  construction, discard consumer, task trait, registry, plugin hook, or public
  wire DTO was introduced.
- Every admitted golden row becomes exactly one `Episode<Target>` owning its ID,
  raw opaque input, and concrete target. The aligned rows own the episode;
  evaluators borrow IDs and targets. Evaluation evidence and comparison results
  contain task outputs/observations and do not clone opaque input. `inspect`
  revalidates stored snapshots, verifies evidence, recomputes and compares the
  report, then clones only the selected row's exact `RawValue` for public
  disclosure.
- Single-label and multi-label targets, outputs, policies, aligned rows,
  evaluation objects, evaluators, probability representations, and result types
  remain concrete and distinct. Evaluator signatures accept only their matching
  evaluation type. Strict source-kind, observation-definition/value,
  preparation-binding, scoring-probability, policy, and alignment checks remain
  on admission.
- `validation`, both evaluators, and comparison remain free of filesystem access
  in the scoped code. `app` performs validation, evaluation, replay
  recomputation, inspection, comparison orchestration, and publication;
  `artifacts` owns filesystem reads, exact snapshots, evidence copying,
  integrity verification, and no-replace publication. `lib.rs`, `main.rs`, and
  `cli.rs` retain the small documented library API and thin binary dispatch.
  The sole boundary failure is F1 above.
- Removed helpers documented by response002 were conveniences or test-only
  accessors. Required production types remain in production and have real callers;
  no required model was hidden only to satisfy Clippy.

## DM and offline proof

Direct inspection of each exact filter confirmed that it calls complete public
behavior with discriminating assertions, rather than treating the wrapper name as
proof. Static claims in DM01/DM02 were separately checked against concrete
signatures and callers.

| Filter | Scoped evidence |
|---|---|
| `structure_dm01` | Both task workflows run through public evaluate/report paths; direct caller review confirms shared validation/app/publication paths. |
| `structure_dm02` | Cross-kind targets, probability families, and policies reject; concrete evaluator and checked-evaluation signatures are task-specific. |
| `structure_dm03` | Reordered multi-label rows/labels preserve scores and canonical output; duplicates reject before `LabelSet`. |
| `structure_dm04` | Independent marginals are retained and scored; the categorical interpretation rejects. |
| `structure_dm05` | Answered empty set, abstention, missing row, and missing marginal key produce four distinct outcomes/errors. |
| `structure_dm06` | Empty intersections preserve admitted signal availability for present, absent, and mixed probability families in both task kinds. |
| `structure_dm07` | Abstention evidence survives report/inspection; selected versus raw-answered signal populations and IDs are asserted. |
| `structure_dm08` | Observed categorical `[0.5,0.49]` round-trips, while the identical scoring vector rejects with `E_PROBABILITY`. |
| `structure_dm09` | Scalar/auxiliary observations, definitions, source references, and preparation evidence persist without creating scoring availability. |
| `structure_dm10` | Unknown/mismatched/invalid observation definitions and broken preparation bindings reject before publication/scoring. |
| `structure_dm11` | Equal per-label aggregate counts coexist with distinct `1/2` versus `0/2` exact-set accuracy using episode records. |
| `structure_dm12` | Duplicate/partial labels, mixed task families, unsupported confidence/outcome fields, and malformed shapes reject without extensions. |

Each `structure_dm01` through `structure_dm12` command executed exactly one test,
reported 102 filtered-out conformance tests, and exited 0. `offline_cli_contract`
executed exactly one CLI test, reported 11 filtered-out CLI tests, and exited 0.
Its component scenarios invoke Cargo's `CARGO_BIN_EXE_validator` process on
temporary saved public synthetic inputs and cover check/evaluate/inspect/compare
for both task kinds without an installed binary, credentials, private data, or a
network dependency.

## Inventory and gates

`cargo test --all-features --locked -- --list` exited 0 and listed exactly 44
library tests, 12 CLI tests, 103 conformance tests, zero binary tests, and zero doc
tests. A direct reconciliation of every CLI/conformance verification ID in
T001-T027 found `planned_public_ids=111`, `listed=111`, `missing=0`.

All literal final commands were run serially and exited 0:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
sandbox-exec -p '(version 1) (allow default) (deny network*)' cargo test --all-features --locked --offline
git diff --check
```

Both full suites reported 44 library, 12 CLI, 103 conformance, zero binary, and
zero doc tests. The sandboxed suite used Cargo offline mode while the macOS profile
denied network access. `ruby docs/plans/validator/verify-plan.rb` also exited 0 and
reported 35 task contracts, 433 source blocks, 63 conformance cases, 12 structure
checks, 8 acceptance criteria, 5 DoD clauses, physical owners, local links, and
the artifact index.

T028-T035 evidence remains explicitly pending and was neither claimed nor treated
as a T027 blocker. No candidate source, test, schema, fixture, plan, governance,
index, acceptance record, session record, or Fizzy state was changed during this
review.
