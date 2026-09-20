# Complete the frozen T020 candidate from a fresh context

Role/model: fresh sole implementation developer, `gpt-5.6-terra`, high
reasoning, `fork_turns: none`. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #184 — Build Validator’s concrete multi-label core](http://localhost:3006/1/cards/184).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t018-t020/010-t020-replacement-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat, return only
the response path, SHA-256 and terse status. Do not edit governance, plans,
dispatches, artifact-index, session notes, acceptance records or Fizzy. Do not
delegate or restart the assignment.

## Read first and fixed boundary

Read completely before editing:

- `/Users/dowwie/.codex/AGENTS.md`, repository `AGENTS.md`, and
  `/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md`;
- owner prompts001 and 009, coordinator correction006 and response008;
- `docs/plans/validator/tasks/T020.json`, its two normative specification
  sections, and `docs/plans/validator/execution-contract.md`;
- developer responses005 and 007, T019 response004 and its independent
  `tests/fixtures/multi-label/expected.json`;
- the complete current three schemas, conformance/CLI tests and directly affected
  production types/application code before editing.

This is a focused completion of the existing T020 candidate. Preserve all valid
working code and passing evidence. Do not restart T020, redesign the evaluator,
broaden the schemas beyond the concrete current documents, or begin T021/T022.

Governing/frozen hashes:

| Artifact | SHA-256 |
|---|---|
| owner reassessment009 | `f24e5683306ac62874ff00106e6f141166f67420ef58c25a9b63b09bb293b09d` |
| coordinator freeze008 | `88750cc6b8b541bbce36d8ce6534505dc5704817eaa10659f7b6bfa6e073e5d7` |
| T020 task | `22bf755ee6eccf5efa2abeb01a350ce92c5bff17953a4b6bb55c519c253cf50a` |
| v1 specification | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| data model | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| execution contract | `cdd917974df4fd505f91d165039988ca01d0936ad2e7703da47f30c7b3041e8f` |
| `src/model/multi_label.rs` | `a4f2f7fa678c525f1c3aed34fb1c3931c887fe3359aa13cd7396cbeee228be46` |
| `src/evaluation/multi_label.rs` | `013f014ed1300a4e2173bc0a4a187af53717079c7d80a37575932e242287257b` |
| `src/evaluation.rs` | `172ab7db055ca9483e3b64593411bb9e64c1983eecdbd06013fe0f6ce1a483ff` |
| `src/app.rs` | `d08e50c7ab32bffa0e9791bdcb32ffa27f8c60e0ed8fc586b23d643e18588ca0` |
| `src/cli.rs` | `ae1d7945252b98eb7b7f5c7312e67661485dddb317fffa4e26f00dd0dfb81695` |
| `src/lib.rs` | `8690583df0fffe301262a476fc39d021148cca08c8860f7dfe0ce03b9698c9c7` |
| `src/artifacts.rs` | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |
| check schema | `7d76d4c9a24322db67e44ab4ce9e03bb13cfc2a60aeb82520a2289b4d7efdb57` |
| report schema | `750802861c4c1187a93a8095230b0b85608db06673ec29a6ee185ecc6555b4c9` |
| inspection schema | `e90023cfec6726802ceff8ffd5524c3184b04b30ea8056d003761256666b4e0e` |
| `tests/conformance.rs` | `a18391bdfbf5a1f9aee9d6ae4a4bf1ee841bd154a76644a33192bd3f3914a07d` |
| `tests/cli.rs` | `c20061b8d01ff67df9bb11d99883caa300147df7fdefd6d907bf425ba1811400` |
| T019 independent oracle | `a65de157415f75567cb907f2dd45ae3d524dcb552dce7c10ef9ca87efbfc63ec` |

Stop and save the exact mismatch before editing if any fixed hash differs.

## Primary write scope and preserved implementation

Primary writes are limited to:

- `schemas/v2/check.schema.json`;
- `schemas/v2/report.schema.json`;
- `schemas/v2/inspection.schema.json`;
- `tests/conformance.rs`;
- `tests/cli.rs`.

The current multi-label scorer, closed app dispatch, shared artifact publication,
stored-run rebuild and relocated inspect path are working. A minimal production
edit in the current T020 artifact list is allowed only after a focused test
reproduces a violation of the same T020 contract. Record the requirement,
reproduction and changed paths. Do not refactor, alter tolerances, widen exports,
add dependencies, suppress warnings or manufacture a consumer.

## Exact completion criteria

1. Rebuild the three task-owned schemas from the actual concrete serialized
   single-label and multi-label documents. Each alternative must require all
   concrete fields, close every owned nested object with
   `additionalProperties: false`, model legal null/status/special-value branches,
   constrain counts/UUIDs/enums and reject foreign-task keys. Preserve the prior
   accepted single-label contract. For every positive real document, add focused
   invalid mutations proving missing required nested fields, unexpected nested
   keys, mixed task-family fields and invalid null/status combinations fail.
2. Expand `multi_label_hard_oracles` through public `evaluate` and the published
   report to cover the independent T019 cases: the two-row `[A,B,C]` rational
   aggregate (`N=G=2`, exact `1/2`, TP/FP/FN `1/1/1`, TN `3`, micro F1 `1/2`,
   macro F1 `1/3`, Hamming `1/3`); population scopes/units; answered empty set;
   one-abstention case; all-abstained `no_answered_predictions`; empty selection
   `no_data`; null abstention evidence; and separate equal raw/final evidence.
   Assert values/statuses/ratios, not merely array lengths or nonzero selection.
3. Correct `equal_counts_distinct_exact_sets` to two real two-row published runs:
   expected `[{A,B},{A,B}]`, predictions `[{}, {A,B}]` versus `[{A},{B}]` (or the
   equivalent T019 frozen fixture pair). Prove every label TP/FP/FN/TN aggregate is
   equal while exact-set accuracy is `1/2` versus `0`; retain episode evidence.
   Do not change the independent expected values to fit implementation output.
4. Complete `marginal_loss_and_bins` through published reports: finite abstention
   `[.8,.7]/{A}` oracle; absent-at-zero zero loss without `0*ln(0)`; present-at-zero
   and absent-at-one positive infinity/status; one-label marginal `.8`/present
   Brier `.04` distinct from accepted single-label categorical `[.8,.2]` Brier
   `.08`; and per-label bin assignment/positive counts at `0`, `.1`, `1`. Assert
   selected `N` and `N*K` label-decision populations, units/scopes/statuses and
   unchanged marginals.
5. Preserve `shared_commands_multi_label` as the real binary check/evaluate/
   relocated-inspect scenario. Add the missing relevant exact opaque payload,
   privacy, one-document stdout and existing-output refusal/unchanged-tree
   assertions, reusing accepted shared artifact evidence instead of copying its
   full failure matrix. Validate real check, receipt, stored report and inspection
   documents against their strict schemas. Do not add multi-label comparison or
   threshold execution.

## Evidence and return

List the correct Cargo targets, then run all six exact commands with nonzero counts:

```text
cargo test --locked --lib multi_label_checked_admission -- --nocapture
cargo test --locked --lib cross_task_boundaries -- --nocapture
cargo test --locked --test conformance multi_label_hard_oracles -- --nocapture
cargo test --locked --test conformance equal_counts_distinct_exact_sets -- --nocapture
cargo test --locked --test conformance marginal_loss_and_bins -- --nocapture
cargo test --locked --test cli shared_commands_multi_label -- --nocapture
```

Then run once on the complete candidate:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
git diff --check
```

Apply owner001/correction006 exactly: previously authorized staged private symbols
remain allowed; only a new unavoidable T020 policy/DTO member needs an exact real
T021/T022 consumer. No ordinary warning class, suppression, fake use or visibility
expansion is allowed. Report the exact residual inventory with existing/new status.

Response010 must be complete: criterion-to-production/schema/test map, independent
derivations, exact list/command/exit/count/actual results, final hashes, production
changes and reproductions if any, residual lint inventory and confirmation that no
T021/T022 work occurred. Any incomplete return ends this attempt and requires owner
reassessment; do not return a progress-only sketch.
