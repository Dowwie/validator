# Correct T016 to use concrete paired single-label metric structures

Role/model: current sole implementation developer, `gpt-5.6-terra`, high
reasoning, retained context from response008. Do not delegate.

Repository: `/Users/dowwie/MyProjects/validator`.

Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t015-t017/010-comparison-developer-to-coordinator.response.md`.

Save the full substantive correction handoff there before returning. In chat,
return only its path, SHA-256 and terse status. Do not edit governance, dispatch,
plan, artifact-index, session-note or Fizzy files. Do not begin T017.

## Read and fixed input

Read completely before editing:

- `/Users/dowwie/.codex/AGENTS.md`, repository `AGENTS.md`, and
  `/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md`;
- `docs/dispatches/validator-t015-t017/009-owner-to-coordinator.prompt.md`;
- `docs/dispatches/validator-t015-t017/009-coordinator-to-owner.response.md`;
- prompt006, continuation prompt008 and your full response008;
- `docs/specs/validator-data-model.md`, especially lines 64-68 and 215-218;
- T016 JSON, both linked specification sections, current replay/application,
  report/result models, comparison/schema and tests.

The response008 candidate is not admitted. Its final hashes are the fixed starting
point:

| Artifact | SHA-256 |
|---|---|
| response008 | `836ca1e0c866f898ba168ada621f37b4c875432fd4febc27af959cb1f199d19a` |
| `src/comparison.rs` | `b439555cb5f0f0a81d3b170ca348689d4e9220339d83539bdc3d69773bba830a` |
| `src/model/single_label.rs` | `ccb45043880960ffb131ee1aa35f9f78cd4a9f2f83f958ef909d97ba8f68dd2d` |
| `src/artifacts.rs` | `8eb978211f0aaf79534f2fe406dc5f783f97e42388b35d30a23e3f0b14a2046c` |
| `src/app.rs` | `ce33a915af602128ed13ca61bd957d946cf8ba092e7210e14a593d969e457136` |
| `src/cli.rs` | `ae1d7945252b98eb7b7f5c7312e67661485dddb317fffa4e26f00dd0dfb81695` |
| `src/lib.rs` | `8690583df0fffe301262a476fc39d021148cca08c8860f7dfe0ce03b9698c9c7` |
| `schemas/v2/comparison.schema.json` | `c5639b223643f11ab4d487810e23ed92d9de8268efab2eefe370b1dfc162d6e3` |
| `tests/conformance.rs` | `3b3349b14ef0f5130ca859bddbb9fde419fba6aa28c5f74c2b8a94251b2edc31` |
| `tests/cli.rs` | `c374ad731ea697679775f61fb9656fa024cf7e9893f34be17a1fb08694c58f79` |

Stop and report any pre-edit mismatch.

## Required bounded correction

Preserve all valid T015/T016 behavior and correct the concrete data boundary:

1. Feed the pure comparison module the verified, recomputed concrete
   single-label evaluation/results and minimum typed run metadata already produced
   during replay. Minimal private app/model wiring is authorized. A private typed
   input record holding typed references and explicit recorded metadata is
   appropriate. `compare(&Value, ...)`, a wrapper around a report `Value`, or
   recursive metric discovery remains noncompliant.
2. Replace `ComparisonMetricFamily { metrics: Vec<...> }`, `collect_metrics`,
   string-path metric names, string status inference and JSON reconstruction with
   concrete paired structures:
   - raw and final each use the same explicit single-label hard-results pair type,
     populated with distinct raw/final values;
   - required hard fields are explicit Rust fields and required schema properties,
     including accuracy, wrong-class rate, abstention rate, coverage, selective
     accuracy, selective risk, vocabulary-ordered concrete per-label
     precision/recall/F1/coverage entries, and macro-F1;
   - probability uses an explicit categorical pair type with log loss, Brier score
     and argmax accuracy fields;
   - each metric field is a concrete pair with both checked metric results,
     direction, defined-only candidate-minus-baseline delta, null reason otherwise,
     and answered-population details where applicable.
3. Reuse existing `MetricResult`/`MetricStatus`/population contracts and one small
   typed metric-pair helper. Do not infer statuses, units, scopes, numerators,
   denominators or special values from serialized JSON strings. No dynamic metric
   registry, universal comparison framework, new task kind or broad SDK.
4. Preserve explicit recorded policy/source/configuration/observation/preparation
   differences at the proper opaque boundary; preserve category IDs/counts,
   changed outcomes, full typed class-plus-abstention transitions, per-episode
   fields, verified replay, receipts and the shared publisher.
5. Use the existing checked arithmetic for category/transition increments and
   total accounting. Remove unchecked `+= 1`, lossy count casts and unchecked sums
   on this touched path. Return the established typed numeric/accounting diagnostic
   on overflow or invariant failure.
6. Update `comparison.schema.json` so raw/final/probability have explicit required
   metric properties and concrete per-label entries. Keep strict modeled object
   boundaries and validate actual application output.

## Existing evidence, made discriminating

Keep the same exact three task test names and applicable full gates. Update their
assertions for the concrete fields. In addition:

- `comparison_compatibility_and_deltas` must not pass its incompatibility check
  merely because a stored report was tampered and replay rejected it first. Use
  verified inputs or an owning-module typed unit case to prove the actual T016
  compatibility boundary, with concrete coverage of golden/task/ordered labels/
  role/selected IDs/numerical semantics as feasible at their owning boundary.
- Name and assert the exact undefined macro and positive-infinity/null-delta
  metrics required by the task expectation, including their status and reason;
  `any(metric.delta == null)` is insufficient.
- Assert representative direction fields rather than relying on suffix discovery:
  at minimum accuracy/coverage/precision/recall/F1/argmax are higher-is-better;
  wrong-class rate/selective risk/log loss/Brier are lower-is-better; preserve an
  explicit defensible direction for abstention rate.
- Assert the exact same-local-ID source configuration, observation-definition and
  preparation differences already present in the CLI fixture.

Do not add a new test framework or broader audit. These assertions make the
existing required named evidence prove its stated contract.

Run and record prompt006's three exact task filters, relevant typed comparison/
publisher/replay unit regressions, format, full all-features tests, release build,
warning-denied Clippy and `git diff --check`. Only the inherited owner-staged
dead-code diagnostics may remain, with exact residual count/symbols and real exit
status. No suppression, fake consumer, tolerance change or unrelated cleanup.

The response must map every explicit hard/probability field to its Rust type,
schema property and named assertion; record all commands/results and starting/
final hashes; disclose residual diagnostics and any blocker; and confirm no T017,
intersection, multi-label, winner/significance, governance, index or Fizzy work.

