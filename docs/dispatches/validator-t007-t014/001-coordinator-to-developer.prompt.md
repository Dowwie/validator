# Validator T007-T014 check/evaluate developer dispatch 001

Role/model: existing replacement sole developer, `gpt-5.6-terra`, reasoning
`high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/001-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator’s verified single-label backbone](http://localhost:3006/1/cards/183).

Implement the exact ordered sequence **T007 -> T008 -> T009 -> T010 -> T011 ->
T012 -> T013 -> T014** as one coherent single-label scoring/publication/application
candidate. You remain the sole implementation/test writer. Do not delegate or
spawn agents. No constituent task is independently accepted in this turn.

## Governing context and accepted baseline

Read in full before editing:

- `/Users/dowwie/.codex/AGENTS.md`, repository `AGENTS.md`,
  `/Users/dowwie/.agents/skills/manage-dev-team/SKILL.md`, and
  `/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md`.
- Task contracts `docs/plans/validator/tasks/T007.json` through `T014.json`, in
  exact order, including every requirement ID, input artifact, named check, and
  common completion check.
- `docs/plans/validator/execution-contract.md`, capability map, physical map, and
  build charter sections governing the coherent T007-T014 sequence.
- Main specification sections: Terms/evidence boundaries; Canonical golden and
  prediction artifacts; Evaluation configuration; Validation/numerical rules;
  Counts and metrics including Single-label hard decisions, Undefined values, and
  Single-label probability/confidence diagnostics; CLI and run artifacts including
  Machine interface and Evidence bindings/replay; Rust project organization,
  package layout/dependencies/tests; and Verification/acceptance cases relevant to
  single-label check/evaluate.
- Data-model Reports/metric reuse, Source organization/ownership, Decode/validate/
  align/score, task-specific structures, and structure acceptance checks.

T004-T006 and their incorporated T002 corrections are owner-accepted at manifest
`docs/dispatches/validator-t004-t006/024-coordinator-final-manifest.md`, SHA-256
`9e9b953930d1d8d334375f3a0ca4f0a22aed5f91eb8cb88cabb32cddc6c1107d`.
Confirm every implementation hash in that manifest before editing. Use the
manifest as the machine-generated unchanged-hash authority; do not retype long
unchanged tables in your response.

The repository is unborn/untracked. Preserve unrelated and accepted files. Do not
stage, commit, clean, reset, modify `.zvec-grep`, or start/stop the task-owned
caffeinate process.

## Construction and test rule

Implement each prerequisite and owning unit tests before its consumer. T008/T009/
T010/T013 conformance tests must ultimately exercise the real small application
API introduced by T014. Do not expose private internals, include production source
in tests, add a test-only facade, placeholder command, fake nonzero test, fallback,
suppression, or unnecessary SDK surface.

Before T014 exists, verify private behavior with owning `#[cfg(test)]` unit tests
and compile checks. Add/run the named conformance and CLI filters only after the
real application API/binary path exists. Preserve all 24 accepted admission tests.

## Combined write scope

You may create/modify only the artifacts owned by T007-T014 and minimal required
parent/dependency wiring:

- `Cargo.toml`, `Cargo.lock` for established minimal runtime/dev dependencies and
  required features;
- `src/model.rs`, `src/model/common.rs`, `src/model/single_label.rs`;
- `src/evaluation.rs`, `src/evaluation/single_label.rs`;
- `src/artifacts.rs`;
- `src/app.rs`, `src/cli.rs`, `src/lib.rs`, `src/main.rs`;
- `src/error.rs` only if the existing stable safe diagnostic/exit mapping needs a
  minimal application/CLI serialization method; preserve all codes/categories;
- `schemas/v2/golden.schema.json`, `predictions.schema.json`, `config.schema.json`,
  `report.schema.json`, `check.schema.json`, `receipt.schema.json`, and
  `error.schema.json`;
- `tests/conformance.rs`, `tests/cli.rs`, and necessary synthetic files under
  `tests/fixtures/single-label/`, including `expected.json`;
- the required response file.

Do not edit specifications, plans, session notes, artifact index, Fizzy, T015+
modules/tests/schemas, multi-label runtime modules, private acceptance data, or
external files. State the chosen explicit JSON Schema dialect/URI and rationale in
your response; the coordinator owns the session decision record/index update.

## T007: checked metric arithmetic and statuses

Implement `MetricResult` and shared checked count/ratio/macro/bin arithmetic in
`src/model/common.rs` and `src/evaluation.rs`.

- Every metric has typed finite value or exact nonnumeric status, population count,
  unit, and selected/answered scope; ratios include exact numerator/denominator.
- Apply availability before no-data, then answered-only/no-answered, then zero-
  denominator precedence. Empty populations are not macro zero-filled. Only the
  two named nonempty macro-F1 metrics zero-fill undefined classes and identify the
  affected labels/classes.
- Positive-infinite log loss uses null plus `+infinity`; unintended nonfinite
  arithmetic is `E_NUMERIC`.
- All counts and `N*K`/micro expressions use checked exact nonnegative arithmetic.

Required filters:
`metric_status_precedence`, `count_overflow_is_error`,
`macro_undefined_classes` (library, nonzero).

## T008: single-label hard decisions

Add typed single-label results/episode evidence and pure evaluation over borrowed
`SingleLabelEvaluation` only.

- Produce raw/final K by K+1 matrices with typed abstention column, totals,
  supports, accuracy, wrong/abstention rates, coverage, selective accuracy/risk,
  per-class precision/recall/F1, macro-F1 and exact accounting identities.
- Wrong A-to-B counts once; support includes abstentions. Literal label `ABSTAIN`
  remains distinct from abstention.
- Episode evidence includes expected/raw/final/correctness/source/observations and
  never opaque input.
- Independent fixture oracles must not be derived from production scoring: F04 is
  accuracy `5/8`, macro-F1 `131/210`; the thread case is accuracy `1/2`, coverage
  `3/4`, macro-F1 `1/2`.

Required conformance filters through the later real app API:
`single_matrix_identities`, `f04_asymmetric_oracle`.

## T009: categorical losses and distinct signals

- Score categorical probability metrics over every selected row including raw
  abstentions; confidence bins use raw answered rows only.
- Preserve submitted versus working vector, deterministic vocabulary-order argmax,
  max/chosen probability, raw choice/argmax disagreement, and confidence.
- Natural log loss and full multiclass Brier use the exact specification; zero true
  probability yields positive-infinity status, and `[1,0,0]` true B has Brier 2.
- Ten bins use `min(floor(10*h),9)` with exact binary64 boundaries and ID-level
  included/excluded evidence. Only max-probability bins produce top-label ECE.
  Vendor confidence is never labeled probability calibration.

Required real-API conformance filters:
`categorical_loss_oracles`, `signal_population_bins`, `bin_boundary_binary64`.

After T007-T009 production scoring and owning unit tests compile/pass, send one
concise milestone with delivered interfaces and actual local checks, then continue.
Do not create a private test facade or claim the later conformance filters early.

## T010: strict canonical input schemas

Publish explicit-dialect JSON Schemas for golden, predictions, and config.

- Cover both declared task alternatives and all closed tagged source, observation,
  scoring, preparation, target, selection, and policy shapes required now; reject
  version/tag/field/type mismatches and extra fields.
- Keep opaque input/config genuinely opaque. Observations never require scoring
  signals. Cross-record/vocabulary/digest/numeric rules remain documented runtime
  checks, not false schema promises.
- Use local references and an established test-only validator. Record exact dialect
  URI/rationale in the response for coordinator session-note entry.

Required real-API conformance filter: `input_schema_contract`.

## T011: exact snapshots and evidence bindings

Implement filesystem work only in `src/artifacts.rs` and model manifest records.

- Read each canonical input exactly once, retain exact bytes, compute exact SHA-256,
  and pass bytes/digests to validation without adding filesystem access there.
- Resolve new-submission evidence paths relative to the prediction artifact;
  declared unused sources/evidence are retained. Sort source IDs by UTF-8 bytes,
  evidence by array index, store every entry separately as `evidence/n.bin` even for
  repeated/basename-collision paths, and bind source/index/original/stored/digest.
- Missing reads are `E_IO`; original paths are never replay dereference behavior.

Required library filters: `evidence_ordinal_binding`, `artifact_exact_bytes`.

## T012: no-replace complete publication

- Build the complete run in an owned temporary sibling, with exact snapshots,
  evidence, schemas/manifest/report as specified, then use a host-supported atomic
  no-replace operation. An exists-then-ordinary-rename sequence is forbidden.
- Existing file/directory/symlink or racing creator remains unchanged and wins;
  destination-exists is typed. Late validation/write failure leaves no final
  success directory and cleans only the owned temporary artifact.
- Protect run-directory permissions on the tested host. Return the absolute report
  path and SHA-256 of exact report bytes. Never overwrite or alter source inputs.

Required library filters: `publish_no_replace_race`, `publish_late_failure`.

## T013: typed reports, check/receipt/error contracts

- Add concrete typed run/check/receipt/error/report models and JSON Schemas with
  every required identity, artifact, source, composition/count, population, task,
  policy, integrity, raw/final/probability/signal, normalization, and sorted episode
  field. No dynamic universal metric map.
- Caller-supplied run UUIDv7/time/digests enter assembly; report assembly performs
  no clock or filesystem access. Source evidence paths are stored bindings and the
  prediction snapshot stays exact.
- Include unused declared sources with zero selected count and all not-applicable
  metric families. Routine reports omit opaque episode input/raw evidence content;
  opaque source/preparation configuration remains only where specified.
- Every emitted JSON document is finite standard JSON and validates its exact
  schema alternative.

Required real-API conformance filters:
`single_report_schema`, `report_sources_and_privacy`.

After T010-T013 schemas, snapshot/evidence loading, typed report assembly, and
no-replace publication unit tests pass at a coherent compile point, send one concise
artifact-publication milestone and continue. Do not claim final integration.

## T014: real application API and thin CLI

- Add a small documented library API accepting explicit paths/options and returning
  typed check/evaluate success or `Diagnostic`, independent of CLI. Wire all prior
  conformance tests through this API; expose only what application callers need.
- `check` loads exact bytes/evidence, validates schema/policy/provenance/alignment,
  returns complete integrity counts/selected IDs, performs no scoring and creates
  no output directory.
- `evaluate` follows the same validation path, scores once, assembles the typed
  report, snapshots exact inputs/evidence, publishes once without replacement, and
  returns a receipt whose absolute `result_path` points to `report.json` and whose
  `result_sha256` hashes its exact bytes.
- App supplies UUIDv7 and UTC RFC3339 time. Binary-only `cli` parses supported
  options and delegates; `main` invokes it and returns exit status only.
- Every operational success/failure emits exactly one schema-version-2 JSON stdout
  document with kind/status; safe stderr separation and stable exit mapping are
  enforced. Help/version are exempt. Unknown args/settings fail. Valid poor/no-data
  results exit 0; process tests cover 0/2/3 and unit error mapping covers 4.

Required real process filters:
`cli_check_evaluate`, `cli_help_version_errors`, `receipt_hash_matches_report`.

## Final verification and handoff

Every one of these 18 task filters must exist, run, and execute a nonzero count:

```text
metric_status_precedence
count_overflow_is_error
macro_undefined_classes
single_matrix_identities
f04_asymmetric_oracle
categorical_loss_oracles
signal_population_bins
bin_boundary_binary64
input_schema_contract
evidence_ordinal_binding
artifact_exact_bytes
publish_no_replace_race
publish_late_failure
single_report_schema
report_sources_and_privacy
cli_check_evaluate
cli_help_version_errors
receipt_hash_matches_report
```

The list contains 18 distinct filters; run the exact task commands from T007-T014
for each (T007 has three, T008 two, T009 three, T010 one, T011 two, T012 two,
T013 two, T014 three). Also rerun the accepted admission filters or full suite so
all 24 baseline tests remain passing.

At the T014 boundary, all of these must exit 0 with no exception:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
git diff --check
```

The interim dead-code allowance ends here. Do not suppress warnings, export
internals, delete accepted behavior, or add fake runtime uses merely to satisfy
Clippy. Resolve warnings through real application consumption and surgical code;
if the clean gate conflicts with accepted/later-only behavior, stop and report the
exact symbols/contract and smallest decision instead of weakening either side.

Save one complete response with separate T007-T014 sections: delivered interfaces,
exact changed hashes (link manifest 024 for unchanged inputs), schema dialect,
independent oracle provenance, every named command/count/result, milestone results,
publication race/late-failure evidence, actual CLI exits/stdout/filesystem effects,
clean final gates, dependencies/features, process/resource state, and any limits.
Do not claim independent verification/owner acceptance. A repeated incomplete
handoff triggers owner reassessment.
