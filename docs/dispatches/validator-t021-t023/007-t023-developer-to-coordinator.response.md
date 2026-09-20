# T023 implementation handoff

Status: complete local implementation and verification. This handoff implements
only T023 validated intersection comparison; it does not open T024 work.

## Delivered behavior

- `Population::restrict` is a consuming checked operation: it sorts its request,
  rejects duplicate or non-selected IDs with `E_ID`, preserves the original
  digest, role, parent/run policy and complete dataset partition, and makes the
  requested IDs the new checked selection.
- Both concrete validated evaluations consume that operation, move their
  original vocabulary, source definitions, legal policy, aligned rows and
  artifact-wide availability into a single rebuilt evaluation. Single-label
  keeps its original `SignalAvailability`; multi-label keeps its original
  `marginals_available` value. No prediction JSON is synthesized or decoded.
- `--intersection` first checks all original compatibility facts except selected
  IDs, derives the UUID-sorted common population and each deterministic excluded
  list, then restricts and re-evaluates both concrete structures. Default
  comparison still requires equal selections. Mixed task kinds still reject.
- Comparison sides retain original run/report identities but now calculate
  source definitions, source counts and composition from the restricted
  compared rows. Typed raw/final answered populations and overlaps continue to
  come from the recomputed typed results.
- The comparison schema has strict population alternatives for `identical` and
  `intersection` scope, including non-empty exclusion on actual intersection
  documents. It retains concrete single-label/multi-label discrimination and
  top-level `additionalProperties: false` rules.
- The CLI recognizes a flag-only `--intersection`; duplicate, unknown, or
  value-taking uses reject as `E_SCHEMA` without publication.

## Criterion and evidence map

| Criterion | Production path | Concrete assertion |
|---|---|---|
| V091-V096, D049-D052 | `Population::restrict`; concrete evaluation `restrict` methods; `app::compare` | Restriction consumes checked rows, preserves policy/provenance/vocabulary/availability, rejects arbitrary IDs, and recomputes rather than subtracting aggregates. |
| V097-V100, S19-S20 | `comparison::intersection_scope`, compatibility helpers, typed comparison assembly | Default selection mismatch returns `E_COMPARISON`; explicit scope has sorted common/excluded IDs/counts; original run IDs/report digests remain in sides; source attribution is restricted-population attribution. |
| V210-V216, D053-D055 | Original availability carried through restriction and existing concrete evaluators | Empty shared selections have hard `no_data`; signal/marginal families independently retain `no_data` if originally present and `not_applicable` if absent; all undefined deltas are null with a side-status reason. |
| V217-V222, AC6 | `ComparisonOptions.intersection`, CLI parser, comparison schema, atomic publisher | API/CLI route real verified/replayed runs to schema-valid immutable `comparison.json` and a digest-bearing receipt. |

`intersection_recomputation` uses common ID `...0002`, baseline exclusion
`...0001`, candidate exclusion `...0003` for each task family. The single-label
raw accuracy is recomputed to baseline `0.0`, candidate `1.0`; the multi-label
final exact-match accuracy is independently recomputed to the same values. Both
source policies are active (`reject_below` and `label_thresholds` respectively),
and the compared side source count is exactly `{"source":1}`.

`empty_intersection_availability` covers both tasks for all three required
cells: present/present gives `no_data`/`no_data`; absent/absent gives
`not_applicable`/`not_applicable`; present/absent gives
`no_data`/`not_applicable`. In every cell hard metrics are `no_data`, probability
or marginal deltas are JSON null, and `delta_reason` names the side status.

`cli_intersection` proves default mismatch rejection, explicit flag success,
schema-valid publication, receipt SHA-256, exact exclusions, immutable-output
refusal, and duplicate/unknown/missing-value option rejection.

## Verification

All commands ran in `/Users/dowwie/MyProjects/validator`.

| Command | Exit | Result |
|---|---:|---|
| `cargo fmt --all -- --check` | 0 | Passed. |
| `cargo test --locked --test conformance intersection_recomputation -- --nocapture` | 0 | 1 passed, 20 filtered. |
| `cargo test --locked --test conformance empty_intersection_availability -- --nocapture` | 0 | 1 passed, 20 filtered. |
| `cargo test --locked --test cli cli_intersection -- --nocapture` | 0 | 1 passed, 8 filtered. |
| `cargo test --all-features --locked` | 0 | 44 library, 9 CLI, 21 conformance and 0 doc tests passed. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Exactly the authorized staged dead-code inventory: 17 production diagnostics and 3 matching lib-test duplicates; no T023 diagnostic. |
| `cargo build --release --locked` | 0 | Passed. |
| `git diff --check` | 0 | Passed. |

The 17 production staged diagnostics are unchanged: `src/model.rs` at 18, 27,
52, 58, 70 and 76; `src/model/common.rs` at 495, 515, 523, 793, 1058 and
1295; `src/model/single_label.rs` at 115; and `src/validation/wire.rs` at 15,
32, 46 and 228. The three lib-test duplicates are the established wire DTO
dead-code diagnostics. No suppression, fake use, visibility widening, or cleanup
was introduced.

## Hash reconciliation

Every starting identity listed in prompt007 matched before editing, including
T023, both ratified specifications, the execution contract, owner prompt001,
the final T022 reconciliation and every listed source/schema/test artifact.

| Artifact | Final SHA-256 |
|---|---|
| `src/model/common.rs` | `5b91a1aaad30127f63030d885338e8d54d807d5682cf3c077525102167c0e9cc` |
| `src/model/single_label.rs` | `4ec2c89ce43f30a79bdb8054c106608257613491415710e56f3139cfbf748ea0` |
| `src/model/multi_label.rs` | `a0d6b55a7930b4acb1c9379d98b77d66eddc3386788eba3fdfb351cb28402c70` |
| `src/comparison.rs` | `4ed45a737ed4ea49fc3c88728b83156acc9938f696078c193bc53979b530f845` |
| `src/app.rs` | `aeee8c0bccde849a769b1a3afc02ac626b8b23565559a28d28979e980a3c7391` |
| `src/cli.rs` | `ed6424fa545b1ccb8418cfc008abe466c7953b9ecad17b0c2f494fc2d4c06969` |
| `schemas/v2/comparison.schema.json` | `005a099c80add634eaa46a8141a402728173436e8acdfc449e8bfb5f75524597` |
| `tests/conformance.rs` | `108cd539f5280acfc3bbb73d61cfd23ccaac03ce8545e631ff82950c02cef92e` |
| `tests/cli.rs` | `9d94d30be6161f00f08e801b93d3062a95aadd4000d6a7f5678613e3e469570d` |

The response itself is intentionally not indexed: prompt007 expressly reserves
artifact-index, governance, plans, session notes, acceptance records and Fizzy
for the coordinator.
