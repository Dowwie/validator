# Validator T015 verified replay and inspection dispatch 002

Role/model: fresh sole developer `/root/coordinator/replay_developer`,
`gpt-5.6-terra`, reasoning `high`, context inheritance `none`.
Repository: `/Users/dowwie/MyProjects/validator`.
Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t015-t017/002-replay-developer-to-coordinator.response.md`.
Fizzy: [Card 183](http://localhost:3006/1/cards/183).

Complete **T015 only**. You are the sole implementation/test writer. Do not
delegate or begin comparison/T016, fixed T017 fixtures, intersection, multi-label,
policy expansion, or T018+.

## Required orientation and frozen input

Read in full before editing:

- `/Users/dowwie/.codex/AGENTS.md`, repository `AGENTS.md`,
  `/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md`, and the relevant
  manage-dev-team developer boundary in its `SKILL.md`;
- `docs/plans/validator/tasks/T015.json`;
- `docs/specs/validator-v1.md` sections **Evidence bindings and replay**, **CLI and
  run artifacts**, and **Machine interface**;
- `docs/specs/validator-data-model.md#source-organization-and-ownership`;
- owner decision `docs/dispatches/validator-t015-t017/001-owner-to-coordinator.prompt.md`;
- repaired frozen manifest
  `docs/dispatches/validator-t007-t014/036-coordinator-repaired-manifest.md` and
  Ready verdict037.

Confirm the complete manifest036/manifest033 candidate before editing. Key hashes:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |
| `src/app.rs` | `819627406e44e184df13d8baf4c1feb115ea0444cf9b7518f83afd2b9e02b875` |
| `src/artifacts.rs` | `14a7383a204c88a7922dedc6159d2b7ad4819b395bab3a8c2185e83ad39aef61` |
| `src/cli.rs` | `608ab90dcda6cd36f8b4d03575f47efc8f2a5977480a0c4bb670aa37933a0fbe` |
| `src/lib.rs` | `b0c5d00e000747368fc9e18e6459c61417dbbf24f06ac58077e9c4cf382ceac7` |
| `src/model/common.rs` | `5dd0c2652439434ba9496a503c6317d8775cdbc90d8ded7b7dac5d351005d4f3` |
| `src/model/single_label.rs` | `899963586c722a37af0ce0dd8f28d03b86423c8c9bbce431b14280ae8a6cee98` |
| `src/validation.rs` | `5a76b0ccc8bd838c13e8671cfb063b27888f0f9b20ecb6e5fa6c26a03c8b49bf` |
| `src/validation/wire.rs` | `46a42903c2edbb556b1ae1ee7b3c3d9642e562e6b2de2a6d63a852c584f6a3b3` |
| `tests/cli.rs` | `3b0cc62faedcb00e46e76a407b3bdf384da90b4596d593a4bf53da46273a567e` |
| `tests/conformance.rs` | `982f128ba6a1b66e3071113b3c05a7fd37ef7269d20a7fa8f34644bb596075a2` |

The repository is unborn/untracked. Do not stage, commit, clean, reset, modify
`.zvec-grep`, edit governance/index/Fizzy/session records, or start/stop caffeinate
PID 84732.

## Verified replay boundary

Add one contained stored-run loader/verifier in `src/artifacts.rs` and coordinate it
from `src/app.rs`. It must:

- resolve `report.json`, the three snapshot paths, and every stored evidence path
  only under the supplied run directory; reject absolute/parent escapes and any
  symlink whose canonical target leaves the canonical run root;
- read exact stored bytes once and verify manifest kind/path/SHA-256 for golden,
  predictions, config and evidence;
- derive the expected evidence bindings from the stored prediction snapshot:
  sources in UTF-8 byte order, source-local array indices, original path strings,
  consecutive `evidence/n.bin` paths, and one entry per submitted position,
  including repeated paths. Verify missing/extra/duplicate/swapped/inconsistent
  bindings and report source evidence rewrites as `E_PROVENANCE`;
- treat original evidence paths only as compared provenance strings. Never resolve,
  stat, open, canonicalize or otherwise dereference them during replay;
- rerun the existing strict validation and pure single-label evaluator over stored
  snapshots, then reconstruct the typed report with the stored run identity/time
  and verified bindings. Compare structural fields, ordering, counts, statuses and
  discrete values exactly; compare finite numerical result fields only with the
  fixed fixture tolerance. Reject inconsistent stored results before inspection.

Do not trust a stored report to construct evaluator inputs. Do not create a second
validator/evaluator, JSON canonicalization scheme, fallback mode, or generalized
filesystem framework. Keep the verified run representation private to app/artifact
and shaped for T016 reuse without implementing comparison now.

## Explicit inspection and opaque fidelity

Add the small documented library API and CLI command:

```text
validator inspect --run RUN_DIR --episode UUID
```

Inspection first completes verified replay, then accepts only a selected episode
ID. Return exactly one versioned `kind: "inspection"`, `status: "complete"` JSON
document with run identity, episode ID, full stored opaque input, expected target,
the exact original prediction record, recomputed final decision/evidence needed by
the specified inspection contract, and recorded configuration. Routine reports and
errors remain payload-free.

Preserve opaque JSON through the established RawValue boundary; never round-trip
input/prediction/config through `f64` or a lossy generic numeric representation.
Owning regressions must cover integer `9007199254740993`, opaque `1e400`, explicit
null, and literal object key `$serde_json::private::Number` without collision or
spelling/value loss. Minimal adaptation of the retained domain input type is allowed
only for this actual inspection consumer.

Create strict Draft 2020-12 `schemas/v2/inspection.schema.json`. Keep the public API
to the required options/result types; no public replay internals or test facade.
CLI errors remain one safe schema-valid JSON stdout document with typed exits.

## Required named evidence

Implement and run:

```sh
cargo test --locked --test cli relocated_run_inspection -- --nocapture
cargo test --locked --test conformance replay_binding_and_result_tampering -- --nocapture
```

`relocated_run_inspection` must use the built binary: evaluate an evidence-bearing
run, relocate it, make every original input/evidence path unavailable, inspect a
selected episode successfully, validate actual stdout against the inspection
schema, and prove the four opaque-value regressions above. It must also prove an
unselected/unknown episode fails safely and routine report/error output has no raw
payload.

`replay_binding_and_result_tampering` must exercise real replay and distinguish at
least: changed snapshot byte/digest, changed evidence byte, missing/extra/duplicate/
swapped binding, wrong original string/index/ordinal stored path, report source
rewrite mismatch, stored result/status/count tampering, and a stored symlink escape.
Every case must fail before returning inspection, with the required typed category,
no original-path lookup, and no external service/credential dependency. Include a
successful relocated replay control.

Also run:

```sh
cargo fmt --all -- --check
cargo test --all-features --locked
cargo build --release --locked --bin validator
cargo clippy --all-targets --all-features --locked -- -D warnings
git diff --check
```

The named filters, format, all tests, release and diff must pass. Run exact Clippy
and record its real status/symbols. Required T015 consumers should reduce staged
warnings where honest; only the prior enumerated remainder may stay. No ordinary/new
warning, suppression, fake read/consumer or broadened export is covered.

## Allowed writes and response

Allowed: `src/artifacts.rs`, `src/app.rs`, `src/cli.rs`, `src/lib.rs`, minimal
inspection/replay model and validation wiring in existing `src/model*.rs` and
`src/validation*.rs`, `schemas/v2/inspection.schema.json`, `tests/conformance.rs`,
`tests/cli.rs`, and the required response. Do not edit report/input schemas unless
a reproduced current T015 contract defect requires the smallest correction and is
reported first. Do not add dependencies unless an established primitive is truly
missing; report that concrete need before modifying Cargo.

Save a complete response before returning. Include exact input/output hashes,
criterion-to-code/test locations, every command/exit/nonzero count, containment/
binding/tamper cases, recomputation comparison rule, opaque-value byte/value proof,
CLI schema/exits/privacy evidence, staged Clippy inventory and unresolved failures.
Return early only for a concrete reproduced scope/architecture blocker with the
smallest contract-preserving decision.
