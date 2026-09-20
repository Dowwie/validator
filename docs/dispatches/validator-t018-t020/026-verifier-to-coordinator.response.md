# Focused T018–T020 source-kind repair recheck

Verdict: **Ready**.

The sole authorized repair closes both manifestations of the original
multi-label source-kind finding. The repaired candidate is stable, the two
negative paths and both preservation paths discriminate correctly, no repair
regression was found, and the warning-denied gate retains only the exact 20
owner-authorized staged `dead_code` diagnostics.

## Repaired candidate reconciliation

- Manifest025 SHA-256 recomputed as
  `e26438bd371f261e5d91a05d55312bfa1d329db75dc4e24e761d75d6bd646f11`,
  exactly the dispatch-bound value.
- Original manifest022 remains
  `918fd6e13608940232de86f3bfb4b4dfba81c7d5cf618657225cef260e05c177`
  and original verdict023 remains
  `41ef819eeb12042ebebd2e51f86a52589150b95354fb1d990a575bd12fbf34e8`.
- Repair prompt024 and response024 recomputed as
  `9bff5430af4ef8a1e92ac362aa056bbc430208422854bd77d62d3bf96ae17a59`
  and `6217ad65a6b7f01ce878e52831f2ecf78fb11a82e84370c08d4cca2bba8bbb48`.
- All 51 bound files—the original 49-file candidate with the three repaired
  identities substituted, plus repair prompt/response024—match exactly. The
  only candidate changes are:

  - `src/validation.rs`:
    `c6e40115a863bba54fab8f84b32beba53b56df153de008bb3fbc63e6ea2886fa`
    to `891e6ee4e95b2be1931dd2006d02940308766b8a7b7d486b85f15faafbb69892`;
  - `schemas/v2/report.schema.json`:
    `7215342a2958fd4d9e9735c2837130a02837d205db93b342b9f64d62a882fcf1`
    to `8f7c4fba2c12381e535888bbde5627a0723471c2018f3ab516a7ce661cb590d6`;
  - `tests/conformance.rs`:
    `f184b28026250ec38da1d228d8bf563272142d21c5844ac2ba4e30e9ab8de291`
    to `aab6e45f86a34d01421e26195b9f06bbca195aefd92e3d565ce84d2fcb1c3f00`.

Every other source, schema, test, oracle, dependency, toolchain, governing, and
handoff hash from manifest022 remains exact. In particular app/CLI, report
serialization, artifacts, hard and marginal evaluators, check and inspection
schemas, comparison/policy code, tolerances, exports, and the T019 oracle are
unchanged.

## Finding 1 recheck: task-wide multi-label source admission

`src/validation.rs:234-240` now admits the shared source map and immediately
rejects when any declared source kind is not `Classifier`. This occurs before
the prediction loop at line 242 and before selection/alignment at lines 278–306,
so an unused source cannot bypass the task boundary. Shared source decoding and
single-label admission are unchanged. The earlier referenced-source check remains
as defense within the prediction loop.

I reran the exact empty multi-label reproduction from verdict023 using the real
release binary. The dataset digest remained
`9ae26f6e48ac56939815a6a492609c132f0f6e1b9f84b498a100c54f00213531`.
With an empty prediction list and one unused, otherwise well-formed
`scored_choice` source, `validator check` now returns exit `2` and:

```json
{"schema_version":2,"kind":"error","status":"error","code":"E_CONFIG","stage":"configuration","affected_ids":[],"message":"configuration is invalid"}
```

Using the same empty multi-label dataset and prediction list with one unused,
well-formed `classifier` source returns exit `0`, reports `source_count: 1`,
`prediction_count: 0`, and `selected_count: 0`. This proves the correction is a
source-kind constraint rather than blanket rejection of unused sources.

The owning regression in `validation::tests::multi_label_checked_admission`
constructs the same empty unused-scored-choice case and asserts
`DiagnosticCode::Config`; the prior referenced-source negative remains intact.
Finding 1 is resolved.

## Finding 2 recheck: report-schema task/source coherence

The multi-label conditional at `schemas/v2/report.schema.json:27` now constrains
each source through `classifierSource`; that definition at line 38 composes the
existing shared `source` definition with `kind: classifier`. The base source
definition remains the single source of configuration, observation, evidence,
question-ID, and preparation constraints. The repair therefore narrows only the
multi-label kind relation and neither duplicates nor weakens those contracts.

I validated the exact foreign-source report published by the pre-repair
reproduction against the repaired Draft 2020-12 schema. It now rejects. Changing
only that source to `classifier` and removing its scored-choice question ID makes
the same multi-label report valid.

I also produced a real one-row single-label `scored_choice` run with categorical
probabilities, reported confidence, and required `question_id: "q"`. Evaluation
returned exit `0`, and its published report validates against the repaired schema
with the scored-choice source retained. The conformance suite independently
contains both discriminators: `multi_label_hard_oracles` rejects the otherwise
well-formed foreign source mutation, and `single_report_schema` accepts the legal
single-label scored-choice form. Finding 2 is resolved.

## Serialized command evidence

| Command | Exit | Independent result |
|---|---:|---|
| `cargo test --locked --lib multi_label_checked_admission -- --nocapture` | 0 | 1 passed, 43 filtered |
| `cargo test --locked --lib cross_task_boundaries -- --nocapture` | 0 | 1 passed, 43 filtered |
| `cargo test --locked --test conformance multi_label_hard_oracles -- --nocapture` | 0 | 1 passed, 14 filtered |
| `cargo test --locked --test conformance single_report_schema -- --nocapture` | 0 | 1 passed, 14 filtered |
| `cargo test --locked --test cli shared_commands_multi_label -- --nocapture` | 0 | 1 passed, 6 filtered |
| `cargo fmt --all -- --check` | 0 | passed |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | exactly the same 20 staged production `dead_code` diagnostics; no ordinary/new class |
| `cargo test --all-features --locked` | 0 | 44 library, 7 CLI, 15 conformance, 0 doc failures |
| `cargo build --release --locked --bin validator` | 0 | passed |
| `git diff --check` | 0 | passed |

The independently rebuilt repaired release binary SHA-256 is
`420ac50f1759c423b40131913f837f11fa4d8768b23f46ce3c90623f359d7da4`.

Clippy's 20 production diagnostics are byte-for-byte-source-identical groups to
verdict023: task-definition wrappers in `src/model.rs`; LabelSet, Episode,
ObservationSet, EvaluationConfig, and MetricResult members in
`src/model/common.rs`; `signal_availability` in `src/model/single_label.rs`; and
strict schema-version/input/later-policy DTO fields in `src/validation/wire.rs`.
No suppression, fake consumer, visibility expansion, tolerance change, ordinary
warning, or new warning class appears.

## Scope and verdict

This focused recheck reused verdict023's settled T018–T020 evidence and assessed
only the two repaired findings and justified regressions. It did not reopen T021,
T022, future staged-warning cleanup, or unrelated schema/audit work. Both findings
are corrected on the exact frozen repaired candidate, the legal neighboring cases
remain supported, and no repair-caused regression is known.

**Ready**: recommend owner acceptance of repaired manifest025 for the T018–T020
checkpoint, retaining the already-recorded 20-diagnostic staged limitation and
the future clean gates.
