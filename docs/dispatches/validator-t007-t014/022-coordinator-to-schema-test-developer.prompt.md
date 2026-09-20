# Validator T010 fresh test-matrix developer dispatch 022

Role/model: fresh sole developer, `gpt-5.6-terra`, reasoning `high`,
`fork_turns=none`.
Repository: `/Users/dowwie/MyProjects/validator`.
Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/022-schema-test-developer-to-coordinator.response.md`.
Fizzy: [Card 183 — Build Validator's verified single-label backbone](http://localhost:3006/1/cards/183).

Complete the exact remaining T010 test matrix. You are the sole implementation/
test writer. Do not delegate or begin T011.

## Required context

Read in full before editing:

- `/Users/dowwie/.codex/AGENTS.md` and repository `AGENTS.md`;
- `/Users/dowwie/.agents/skills/manage-dev-team/SKILL.md` and
  `/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md`;
- `docs/plans/validator/tasks/T010.json`;
- `docs/specs/validator-v1.md` sections **Canonical golden dataset**,
  **Canonical prediction artifact**, **Evaluation configuration**, and **Machine
  interface**;
- owner scope decisions
  `docs/dispatches/validator-t007-t014/018-owner-to-coordinator.prompt.md` and
  `docs/dispatches/validator-t007-t014/021-owner-to-coordinator.prompt.md`.

Do not read the full dispatch history. Confirm these frozen inputs before editing:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `e257b5d70c78f0dd7465093aad8d092db99c35ffa28227298271512faf49bec6` |
| `Cargo.lock` | `86caa4cd0470cc4d1f8d4d8dedbdfccb1c43e42b9f0a5a119343e35b256dd74e` |
| `schemas/v2/golden.schema.json` | `97d811a192274f1ee0703e5dff475723301fdcc68c20b031d3b22431968b777b` |
| `schemas/v2/predictions.schema.json` | `654908068da5d6b3cf9c666cddf4ebc888f94203a04fb6addd5164fbbb893352` |
| `schemas/v2/config.schema.json` | `c740c7cd2c1381398b3121ee161b1d995a23ceb9dd4325bd73cd0073f66d65c3` |
| `tests/conformance.rs` | `916a7d485f4819b436eb6ef2983e9ceb86a3875cf79561d4c10cb7bd80fe2d17` |

The current exact filter passes, as do 33 library tests. Preserve every existing
valid assertion, the legal scalar 1.33, retained categorical observation sum 0.99,
all five legal observation kinds, legal policies/multi-label shapes, and the one
absent-source runtime-boundary case.

## Only remaining deliverable

Use reusable valid prediction/config bases. Each negative case must change one
relevant field/shape, carry a clear case name, and assert the specification-derived
expected result. Add exactly these five case families in `tests/conformance.rs`:

1. **Isolate bounded value and extras:** an otherwise valid prediction with only
   an out-of-range bernoulli or reported-confidence observation; separate otherwise
   valid documents with an extra envelope field, extra prediction-row field, and
   extra outcome field.
2. **Isolate whitespace model:** an otherwise valid source with only a whitespace-
   only `model`. Retain the existing isolated whitespace source-key regression.
   Inspect the shared nonblank schema reference wiring; do not create exhaustive
   cross-products.
3. **Isolate config rules:** one otherwise valid label-threshold config with only
   an empty `thresholds` map; one otherwise valid config whose decision contains
   only an extra policy field.
4. **Complete probability branches:** an otherwise valid prediction with legal
   nonempty categorical probabilities; an otherwise valid prediction with empty
   label-marginal probabilities. Retain legal label marginals and empty categorical.
5. **Complete observation shapes:** separate otherwise valid predictions for a
   scalar kind carrying `values`, a vector kind carrying `value`, a vector map
   with a whitespace-only key, and empty label-marginal observation values. Retain
   the existing empty categorical observation case.

The existing absent-source case is sufficient; do not add another runtime-only
boundary. Do not add exhaustive combinations for identical shared definitions.

Primary and expected write is `tests/conformance.rs`. Preserve all three schemas.
Change an existing schema only if one listed isolated case actually fails and
proves a specification defect; record the red/green case and minimal schema hash
change. No dependency, framework, production, fixture, documentation/index/Fizzy,
or other test changes. Do not stage, commit, reset, clean, modify `.zvec-grep`, or
start/stop caffeinate PID 84732.

Run:

```sh
cargo fmt --all -- --check
cargo test --locked --test conformance input_schema_contract -- --nocapture
cargo test --locked --lib
git diff --check
```

Save the full response before returning. Map bullets 1-5 to exact case names and
line locations, report each result, exact changed hashes, command exits/nonzero
counts, any demonstrated schema defect, and unresolved failures. A prose or count
claim without the actual assertions is incomplete.
