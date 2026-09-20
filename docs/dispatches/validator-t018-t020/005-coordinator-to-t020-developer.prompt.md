# Implement T020 marginal scoring and real shared-command integration

Role/model: fresh sole implementation developer, `gpt-5.6-terra`, high
reasoning, `fork_turns: none`. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.
Fizzy: [Card #184 — Build Validator’s concrete multi-label core](http://localhost:3006/1/cards/184).

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t018-t020/005-t020-developer-to-coordinator.response.md`.

Save the full substantive handoff there before returning. In chat, return only
the response path, SHA-256 and terse status. Do not edit governance, plans,
dispatches, artifact-index, session notes, acceptance records or Fizzy.

## Read first and fixed input

Read completely before editing:

- `/Users/dowwie/.codex/AGENTS.md`, repository `AGENTS.md`, and
  `/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md`;
- `docs/plans/validator/tasks/T020.json`;
- `Multi-label probability metrics and diagnostics`, `CLI and run artifacts`,
  `Evidence bindings and replay`, and the multi-label required evidence rows in
  `docs/specs/validator-v1.md`;
- `Reports and metric reuse` in `docs/specs/validator-data-model.md`;
- `docs/plans/validator/execution-contract.md` and owner prompt001;
- T018 response003 and T019 response004, then the complete current model,
  evaluators, validator, application, artifact/replay, CLI, schema and integration
  test code before editing.

T018 and T019 are reconciled local milestones. T020 completes their real
application boundary and is the last implementation task before one combined
independent review. Do not begin T021 policy execution, T022 multi-label paired
comparison, or later work.

Starting hashes:

| Artifact | SHA-256 |
|---|---|
| T020 contract | `22bf755ee6eccf5efa2abeb01a350ce92c5bff17953a4b6bb55c519c253cf50a` |
| v1 specification | `c0ce1c3f5e729ad510091e6c33d2b2b74c6f79f269ea7b76d7707364b54b79a7` |
| data model | `9b8ff48e452c7de0dec85d86da07797a33eb263ad165e230c0d744fa1bfc70bb` |
| execution contract | `cdd917974df4fd505f91d165039988ca01d0936ad2e7703da47f30c7b3041e8f` |
| T019 handoff | `22e2292f457e9d8ba1d748a1f401f7ba182695465bbcf2c99e1e1b50d7a8376e` |
| `src/model.rs` | `846677319091911cd5f1a9a869aa45700b8ce46caf0923e779f74100c71dd4f6` |
| `src/model/common.rs` | `dfe120adb33f7a773edd5a8c44c8331fb918e417aec52dfc3d1c0a8077138791` |
| `src/model/multi_label.rs` | `df2b98c78f4ebed9945be256b0371b06705729e3053ef4319b5a878cdc392081` |
| `src/evaluation.rs` | `172ab7db055ca9483e3b64593411bb9e64c1983eecdbd06013fe0f6ce1a483ff` |
| `src/evaluation/multi_label.rs` | `2d6ec1dd615a8aac4882a6330b5e08319bc1385aa9f1e70b462c116bd4ea8e4c` |
| `src/validation.rs` | `c6e40115a863bba54fab8f84b32beba53b56df153de008bb3fbc63e6ea2886fa` |
| `src/validation/wire.rs` | `3c98ed9c193cb8dff0f889956f58a4198b4d8a956a3da34592b45e5e381d29ad` |
| `src/app.rs` | `28418611219e65eb32ffcdf6ae470f48bca944f68112e9997ee87c1f134b7ad7` |
| `src/cli.rs` | `ae1d7945252b98eb7b7f5c7312e67661485dddb317fffa4e26f00dd0dfb81695` |
| `src/lib.rs` | `8690583df0fffe301262a476fc39d021148cca08c8860f7dfe0ce03b9698c9c7` |
| `src/artifacts.rs` | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |
| `schemas/v2/check.schema.json` | `675f6b1faa3fd67b5b42bc42122f7ccf916925920dd59d14ddee886fa90804ad` |
| `schemas/v2/report.schema.json` | `4ff9c9f1ad59cd2ed73965c9cf154209e6eaf1b6871691c8d5128ca4cbd7ff46` |
| `schemas/v2/inspection.schema.json` | `5e7990c05d9f87a29768c73d6d39a1579d39a9c842ce790cb780ff616182fcfa` |
| `schemas/v2/receipt.schema.json` | `d820751593f82ac68cbca2ed381b305383d96d388dee417abcefec03ed3d1c00` |
| `schemas/v2/error.schema.json` | `a1be234949560a90dff9391a0fa9c42de4f0431c5c4e2f20d745730f40eb80d5` |
| `tests/conformance.rs` | `bdf526a6a60fde605c6c65e5691fd7b4e397642b0166cf216b1a169f05d3e353` |
| `tests/cli.rs` | `425568409236bc3f6a834a424916b678e2db1c78cf4a5e487d3f6a4f22050dc0` |
| multi-label hard oracle | `a65de157415f75567cb907f2dd45ae3d524dcb552dce7c10ef9ca87efbfc63ec` |

Stop and report an exact mismatch before editing.

## Allowed write scope

Own only the T020 artifacts in its ratified task contract:

- `src/model/multi_label.rs`, `src/evaluation/multi_label.rs`, and the necessary
  `src/evaluation.rs` closed dispatch;
- `src/app.rs`, `src/cli.rs`, and `src/lib.rs` for one shared concrete task
  dispatch across check/evaluate/inspect and stored-run replay;
- `schemas/v2/check.schema.json`, `report.schema.json`, and
  `inspection.schema.json` for strict concrete multi-label alternatives;
- `tests/conformance.rs` and `tests/cli.rs` for real public API/process evidence.

Minimal existing shared accessors or `src/artifacts.rs` changes are permitted only
if the real multi-label path demonstrates they are necessary; preserve one shared
publication/replay implementation and report the exception. Receipt and error
schemas are task-neutral and must remain unchanged unless an actual failing T020
document proves a mismatch; stop and report before expanding them. Do not add
dependencies, test-only public facades, source inclusion, widened SDK exports,
duplicate I/O/publication paths, dynamic task registries or generic evaluator
frameworks.

`compare` remains the accepted single-label behavior. T022 owns paired multi-label
comparison; do not implement it here and do not claim it in `shared_commands_multi_label`.
Stored-run replay needed to verify/inspect one multi-label run is required now.

## Marginal metrics and signals

1. Score complete marginals over every selected episode `N`, including whole-row
   abstentions. Never silently select probability-bearing rows. If the family is
   unavailable, preserve explicit applicability/status rather than an empty result.
2. For each label, compute binary log loss from reference presence using `-ln(p)`
   for present and stable `-ln_1p(-p)` for absent, and binary Brier `(p-y)^2`.
   Branch on the observed outcome; never evaluate `0*ln(0)`, clip, normalize or
   repair. An observed outcome assigned zero probability is positive infinity via
   the shared status contract; any infinite label loss makes the mean infinite.
3. Per-label probability metrics use `N` selected `label_decision`s. Overall means
   use `N*K` selected `label_decision`s. Mean binary Brier lies in `[0,1]` and is
   distinct from categorical Brier. No joint-set probability, argmax, confidence,
   maximum-probability diagnostic, ECE or statistical claim exists for multi-label.
4. Produce ten fixed bins separately for every label using the existing boundary
   rule. Each bin has boundaries, count, positive count, mean probability and
   observed positive rate; empty means/rates are null. Pair positivity with the
   reference label, not prediction correctness. Do not aggregate labels.
5. Required discriminators include: `[0.8,0.7]` against `{A}` gives mean Brier
   `0.265` and mean log loss `(-ln(.8)-ln(.3))/2`, even for abstention; absent labels
   at zero have zero loss; present-at-zero and absent-at-one are explicit infinity;
   one-label marginal `0.8`/present has Brier `.04` while single-label categorical
   `[0.8,0.2]` has `.08`; bin values at `0`, `.1`, and `1` use declared boundaries
   and reference positivity. Use exact analytical assertions where representable
   and the ratified fixture tolerances only for transcendent floating results.

## Closed application, artifacts and schemas

1. Replace the remaining single-label-only application match with the existing
   private closed `ValidatedTask::{SingleLabel, MultiLabel}` dispatch. Both tasks
   use the same file loading, evidence binding, immutable publication, receipt,
   strict stdout/error and source/privacy behavior. Preserve all accepted
   single-label outputs and tests.
2. `check` admits both task kinds without scoring or publication. Its Rust output
   and strict schema must select concrete task-tagged integrity alternatives so a
   multi-label document never requires single-label normalization or confidence
   fields, and neither variant accepts foreign fields.
3. `evaluate` dispatches to the concrete evaluator, creates a complete typed
   multi-label report, serializes it, and publishes it through the existing atomic
   run path. The report's `task`, `raw`, `final`, `probability`, `signals`, and
   sorted `episodes` are task-tagged concrete alternatives with required fields,
   exact population counts/scopes/units/statuses and no single-label-only keys.
   Preserve source definitions/counts/composition, artifact bindings, policy,
   integrity and opaque payload privacy.
4. Stored multi-label runs must verify exact artifacts/evidence, rebuild their
   report through the same closed concrete path, and support `inspect`. Inspection
   returns the selected episode's original opaque input, expected target,
   prediction, final outcome, observations, marginal probability evidence and
   recorded configuration. Its strict schema selects concrete task alternatives.
   Existing replay equality rules must compare structural/count/recorded data
   exactly and apply configured float tolerance only to actual computed multi-label
   metric/bin paths.
5. Update only the three task-owned schemas with closed `oneOf` or equivalent
   concrete alternatives. Keep `additionalProperties: false`, required fields,
   exact enums, null/status branches and nonnegative/count/UUID constraints. Add
   positive real documents and discriminating negative schema cases that reject
   foreign single-label/multi-label keys, absent required fields and permissive
   near misses. Do not defer actual T020 document validity to T024.
6. Current policy is only `as_recorded`; raw and final remain equal. Do not execute
   `label_thresholds`, add a placeholder success variant or coerce unsupported
   policy. T021 owns threshold execution.

## Exact required evidence

All six exact filters must exist, list, and run nonzero on their specified targets:

```text
cargo test --locked --lib multi_label_checked_admission -- --nocapture
cargo test --locked --lib cross_task_boundaries -- --nocapture
cargo test --locked --test conformance multi_label_hard_oracles -- --nocapture
cargo test --locked --test conformance equal_counts_distinct_exact_sets -- --nocapture
cargo test --locked --test conformance marginal_loss_and_bins -- --nocapture
cargo test --locked --test cli shared_commands_multi_label -- --nocapture
```

The two hard conformance filters must call the real public application API and
assert the hand-authored T019 oracle/status/evidence values; do not recreate a
private evaluator facade. `marginal_loss_and_bins` must use the real evaluate/
published-report path and cover all finite/infinite/scaling/bin cases above.
`shared_commands_multi_label` must run the real binary for check, evaluate and
inspect, validate stdout against the strict schemas, verify the stored report,
exercise relocated stored-run replay, and assert the unchanged exact publication/
privacy/failure behavior relevant to the new task. It must not test or claim
paired multi-label comparison.

Capture the six names through the correct Cargo list targets and confirm each
filtered command selects the expected nonzero count. Also run the full combined
boundary gates:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
git diff --check
```

T019 recorded 69 library plus 11 library-test `dead_code` diagnostics and no
ordinary warning. Real T020 application/report/schema consumers should remove the
T019 portion. Any residual diagnostic is stageable only when the exact private
symbol has a real T021 or T022 consumer already named by the ratified plan. No
ordinary/new warning, suppression, fake use, cleanup-only detour or widened export
is permitted. T027 and T035 remain warning-free gates.

## Handoff

The response must map every T020 criterion and normative clause to concrete types,
schemas and named real-path assertions; document the exact closed dispatch and
shared publication/replay reuse; give analytical marginal derivations; record each
exact command, exit, listed/selected test count and actual result; list starting
and final hashes for every changed/relied-on artifact; give any exact residual
Clippy count/symbol/owning-task map; disclose blockers; and confirm no T021/T022+
implementation, multi-label comparison, governance/index/Fizzy work. Completion
freezes a candidate for one independent combined T018-T020 review; it is not owner
acceptance.
