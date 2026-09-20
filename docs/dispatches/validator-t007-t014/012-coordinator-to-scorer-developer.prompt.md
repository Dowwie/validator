# Validator T010 atomic schema-developer dispatch 012

Role/model: retained sole developer `/root/coordinator/scorer_developer`,
`gpt-5.6-terra`, reasoning `high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/012-scorer-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator's verified single-label backbone](http://localhost:3006/1/cards/183).

T009 is reconciled as a complete local milestone after the exact ECE correction.
Complete **T010 only** next. This remains inside the combined T007-T014 candidate;
there is no independent review or acceptance at this unit. You remain the sole
implementation/test writer. Do not delegate or spawn agents.

## Required context and current identity

Read in full:

- `/Users/dowwie/.codex/AGENTS.md`, repository `AGENTS.md`, and the already-read
  manage-dev-team and Rust best-practices skills;
- `docs/plans/validator/tasks/T010.json`;
- `docs/specs/validator-v1.md` sections **Canonical golden dataset**,
  **Canonical prediction artifact**, **Evaluation configuration**, and **Machine
  interface**, including all closed target/source/observation/probability/policy
  variants those sections define;
- owner clarification
  `docs/dispatches/validator-t007-t014/002-owner-to-coordinator.prompt.md`: T010's
  schema-only conformance test may directly use the schema engine, and legal
  multi-label alternatives must not be routed through the single-label runtime.

The complete T009 handoff is
`docs/dispatches/validator-t007-t014/011-scorer-developer-to-coordinator.response.md`,
SHA-256 `70adec8312203e5193204ca4450e5d0bcf3ce7b110d6723912dd205096763627`.
Confirm these exact inputs before editing:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `981783137668727726799ca99ee963f5333978cd6090ad9953b6d237a8e2c621` |
| `Cargo.lock` | `10e432596f9710c5aee0cac125b3ad9d7bbe4d3360e203315af0a628f83b5bb3` |
| `src/lib.rs` | `bd89b6b7148e8ef25e64d28b465218a1f4afd90b8f95e6f92ed3a510de69aade` |
| `src/model/common.rs` | `a9f823207e1fd1aaa6181b5594e5d2a8ac4df150216b7299901b52c98a3ae63b` |
| `src/model/single_label.rs` | `93a9ba28d911841a8d89249ce19846cb001d8dda217961ec95faa94043b22343` |
| `src/evaluation.rs` | `3b6e74d440fb76d539b3fbd338c774e81062d3d4c86e1717b3c02b6696b2ddaa` |
| `src/evaluation/single_label.rs` | `d631caff112d4cea8ec74be14da0cefe067226f23f7f600f0ce5d3b172613b33` |
| `tests/fixtures/single-label/expected.json` | `99cb21c88d4d497b8d805537d9e2648304b9aab34ea748ea6752f350641728c5` |

Preserve the 33-test library baseline. The repository is unborn/untracked. Do not
stage, commit, clean, reset, modify `.zvec-grep`, edit project governance/index/
Fizzy records, or start/stop task-owned caffeinate PID 84732.

## Exact outcome

Publish strict wire-v2 schemas at:

- `schemas/v2/golden.schema.json`;
- `schemas/v2/predictions.schema.json`;
- `schemas/v2/config.schema.json`.

Use one explicit published JSON Schema dialect consistently, with `$schema` in
each file and only local `#/$defs/...` references. State the exact dialect URI and
brief rationale in the response so the coordinator can record the decision in
`session-notes.md`.

- Golden schema covers both complete `single_label` and `multi_label` task/target
  alternatives, UUID-shaped IDs, required opaque `input`, closed surrounding
  objects, exact schema version 2, task label cardinality, and uniqueness rules
  expressible in JSON Schema. Opaque input remains unconstrained JSON and may be
  null, scalar, array, or object.
- Prediction schema covers class, label-set, and abstention outcomes; categorical
  and label-marginal probability wrappers; confidence; both source kinds; evidence;
  preparation; all observation definitions and value variants; exact digest/version
  shapes; and closed tagged/container objects. Source/preparation `configuration`
  remains an opaque object. Observations never imply or require top-level scoring
  probabilities/confidence.
- Config schema covers exact version, nonblank population, role, optional unique
  UUID episode IDs including an explicit empty array, optional parent run ID, and
  closed `as_recorded`, `reject_below`, and `label_thresholds` decision variants.
- Closed variants and containers reject unsupported tags, wrong field sets/types,
  explicit null where omission is required, and extra fields. Legal multi-label
  shapes must validate directly against the schema engine now even though their
  runtime evaluator is not implemented until T018.
- Do not encode false promises for constraints needing runtime/cross-document
  knowledge: label membership/exact probability key coverage, task compatibility
  across separate files, source references/kind rules, digest equality, duplicate
  JSON keys, exact ID alignment, categorical sums/normalization, full finite-number
  semantics, threshold map equality to the vocabulary, and other cross-record
  invariants. Enumerate these boundaries in the response and test at least one
  shape-valid document whose semantic defect remains a runtime responsibility.

## Tests and dependency scope

Create `tests/conformance.rs` with the real exact filter
`input_schema_contract`. It may load the three published schema files and use a
test-only JSON Schema engine directly; do not route schema alternatives through
the current single-label application/runtime. The filter must execute a nonzero
test and cover representative legal single-label and multi-label documents plus
negative version/tag/field/type/extra-field/target/probability/observation/policy
cases. Assert each intended accept/reject outcome with diagnostic case names.

`Cargo.toml`/`Cargo.lock` may change only to add one established, minimal test-only
JSON Schema validator and its required feature settings because the expected T001
dev dependency is absent from the current manifest. Pin the resolved graph in
`Cargo.lock`; no production dependency or schema-generation framework is allowed.

Allowed writes are exactly the three schemas, `tests/conformance.rs`, the minimal
Cargo manifest/lock change above, and the required response. Do not modify Rust
production modules, existing fixtures, other schemas/tests, specifications, plans,
session notes, index, Fizzy, T011+ behavior, or private data.

Run:

```sh
cargo fmt --all -- --check
cargo check --locked
cargo test --locked --test conformance input_schema_contract -- --nocapture
cargo test --locked --lib
git diff --check
```

Confirm the exact conformance filter executes one real test. Do not run full
Clippy/all-target/release gates by habit; T014 owns them. Report dependency fetch/
offline status and current unrelated dead-code warnings exactly; do not suppress
warnings or add fake uses.

## Required response

Continue until all three schemas, the schema-only conformance matrix, dependency
lock, format, compile, and regression checks are complete. Save the response before
returning. Include exact input/output hashes, chosen dialect/rationale, coverage
map, explicit runtime-only boundary list, every command/exit/nonzero test count,
dependency change, warning status, and any unresolved failure.

Return early only for a concrete reproduced blocker with the exact command and
smallest decision. An incomplete return without a blocker is escalated without
reissuing the same instruction.
