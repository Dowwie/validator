# T016 fresh comparison completion handoff

Completed T016 only from the preserved partial state. No T017, intersection,
multi-label comparison, winner/significance logic, governance, artifact-index,
session-note, or Fizzy work was performed.

## Input identity

The partial-state hashes required by dispatch008 matched before editing:

| Artifact | Starting SHA-256 | Final SHA-256 |
| --- | --- | --- |
| `src/comparison.rs` | `7cf0e1db8c5a5f65c433112c615bc68d6c4de294346ae6b184e7c8a334652429` | `b439555cb5f0f0a81d3b170ca348689d4e9220339d83539bdc3d69773bba830a` |
| `src/model/single_label.rs` | `899963586c722a37af0ce0dd8f28d03b86423c8c9bbce431b14280ae8a6cee98` | `ccb45043880960ffb131ee1aa35f9f78cd4a9f2f83f958ef909d97ba8f68dd2d` |
| `src/artifacts.rs` | `95a8c71d82dd180828e50810a483f94ee8be00826ef26e56cce700719e33068e` | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |
| `src/app.rs` | `43be4af8e98afef8c6f5733932075305cc58ae3d3e24d3718d824e17fbdc9d57` | `ce33a915af602128ed13ca61bd957d946cf8ba092e7210e14a593d969e457136` |
| `src/cli.rs` | `ae1d7945252b98eb7b7f5c7312e67661485dddb317fffa4e26f00dd0dfb81695` | `ae1d7945252b98eb7b7f5c7312e67661485dddb317fffa4e26f00dd0dfb81695` |
| `src/lib.rs` | `8690583df0fffe301262a476fc39d021148cca08c8860f7dfe0ce03b9698c9c7` | `8690583df0fffe301262a476fc39d021148cca08c8860f7dfe0ce03b9698c9c7` |
| `schemas/v2/comparison.schema.json` | `d2ff60da8fbc59f07fb555abe758457d5ac8e3b670f3c4091a4cd8eb3a442035` | `c5639b223643f11ab4d487810e23ed92d9de8268efab2eefe370b1dfc162d6e3` |
| `tests/conformance.rs` | `3159a6af2fe6839e59fac18df8c0a6b549ff7ab9e8916bfcf5a3bdb52a8cd3b6` | `3b3349b14ef0f5130ca859bddbb9fde419fba6aa28c5f74c2b8a94251b2edc31` |
| `tests/cli.rs` | `5fad6fa4f66572f9af4a7ed4d23f9c0474046b0aa6ff0acd7a03fe6cafe4f9e5` | `c374ad731ea697679775f61fb9656fa024cf7e9893f34be17a1fb08694c58f79` |

## Implementation

- `src/model/single_label.rs` adds concrete serializable T016 comparison types:
  identity/sides/population, field-level configuration differences, metric pairs,
  answered-population overlaps, exact correctness categories, typed outcomes,
  the full label-plus-abstention transition grid, and episode rows.
- `src/comparison.rs` is the pure comparison boundary. It verifies golden digest,
  task kind, ordered vocabulary, role, selected IDs, and actual numerical
  semantics (`categorical_sum_tolerance` and `bin_count`), while permitting and
  reporting decision-policy and source definition/configuration differences.
  It collects typed raw/final/probability metrics, emits candidate-minus-baseline
  deltas only for two defined finite values, records null reasons otherwise,
  preserves direction and answered-ID overlap, and checks the category/table
  totals.
- `src/app.rs` supplies fresh comparison identity/timestamp, verifies both stored
  runs through the established replay path, serializes the typed document, and
  publishes it.
- `src/artifacts.rs` factors the private temporary-directory/no-replace/cleanup
  mechanics into one internal publisher used by both five-entry evaluations and
  one-file comparisons. The evaluation hook still runs after `report.json` is
  written, preserving its late-failure test behavior.
- `schemas/v2/comparison.schema.json` is a strict Draft 2020-12 contract for the
  actual typed document, with required fields and `additionalProperties: false`
  at modeled object boundaries.
- `tests/conformance.rs` adds the two named real-application comparison cases.
  `tests/cli.rs` adds the named real-binary receipt/schema/digest case.

## Criterion and evidence mapping

1. `compare` replays both sides before calling the pure boundary; the CLI case
   uses two stored runs.
2. `require_equal` covers golden/task/labels/role/selected IDs/numerical
   semantics and excludes policy decision from rejection.
3. Recursive `collect_differences` compares local source definitions and their
   configuration leaves. The CLI case uses the same local source ID with changed
   source configuration, observation definition, and preparation descriptor.
4. `SingleLabelComparison` carries caller-supplied UUIDv7/time via `app`, run
   report digests, identical scope, exact population, and configuration diffs.
5. `metric_family` produces typed separated families, status/value/population
   pairs, direction, defined-only deltas, null reasons, and answered overlaps.
6. `single_comparison_transitions` asserts all correctness categories, recovered
   and regressed IDs, changed outcomes, all episode fields, and a complete
   class-plus-abstention grid whose total is five.
7. No winner, significance, or improvement field is modeled.
8. `publish_document` is the shared no-replace/private/cleanup seam;
   `publish_no_replace_race` and the CLI receipt verify it.
9. Existing `compare --baseline --candidate --out` dispatch is exercised by
   `cli_compare_receipt`.
10. `cli_compare_receipt` validates the exact published comparison document
    against the new strict schema.

## Commands

| Command | Expected | Actual |
| --- | --- | --- |
| `cargo check --locked` before edits | Compiles with inherited warnings | Exit 0; 26 inherited warnings |
| `cargo test --locked --test conformance single_comparison_transitions -- --nocapture` | Exact T016 transitions | Exit 0; 1 passed |
| `cargo test --locked --test conformance comparison_compatibility_and_deltas -- --nocapture` | Compatibility and paired deltas | Exit 0; 1 passed |
| `cargo test --locked --test cli cli_compare_receipt -- --nocapture` | CLI receipt/schema/digest | Exit 0; 1 passed |
| `cargo test --locked artifacts::tests::publish_no_replace_race -- --nocapture` | Existing publisher safety | Exit 0; 1 passed |
| `cargo test --locked --test conformance replay_binding_and_result_tampering -- --nocapture` | T015 replay regression | Exit 0; 1 passed |
| `cargo fmt --all -- --check` | Formatting clean | Exit 0 |
| `cargo test --all-features --locked` | Full test suite | Exit 0; 39 unit, 5 CLI, 11 conformance tests passed |
| `cargo build --release --locked` | Release build | Exit 0 |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | No new diagnostic class | Exit 101; exact inherited 26 dead-code diagnostics below |
| `git diff --check` | No whitespace errors | Exit 0 |

## Residual diagnostics

The only residual diagnostics are the owner-staged 26 dead-code items inherited
from T014: `checked_mul`; `TaskDefinition` and its four methods; `SingleLabelTask`
and its two methods; `MultiLabelTask` and its two methods; `LabelVocabulary`
multi-label methods; `LabelSet` and two methods; `Episode` and four methods;
`ObservationSet::values`; `EvaluationConfig::policy`; `Population::dataset_digest`;
`MetricUnit::LabelDecision`; unused `MetricResult` constructors/accessors;
`signal_availability`; and the six strict wire DTO field groups. No new lint
class, suppression, fake use, or public expansion was introduced.

No unresolved functional failure or scope conflict remains. This is a local T016
completion handoff only; it is not acceptance and does not begin T017.
