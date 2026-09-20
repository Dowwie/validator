# Validator T014 fresh-context completion dispatch 031

Role/model: fresh sole developer `/root/coordinator/t014_completion_developer`,
`gpt-5.6-terra`, reasoning `high`, context inheritance `none`.
Repository: `/Users/dowwie/MyProjects/validator`.
Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/031-t014-completion-developer-to-coordinator.response.md`.
Fizzy: [Card 183](http://localhost:3006/1/cards/183).

You replace a repeatedly partial worker under the existing owner021 authorization.
You are the sole implementation/test writer. Complete the unchanged remainder of
**T014 only**, save the full response, and stop. Do not delegate or begin T015,
comparison, inspection, replay, multi-label behavior, or later consumers.

## Orientation and authority

Read in full before editing:

- `/Users/dowwie/.codex/AGENTS.md` and repository `AGENTS.md`;
- `/Users/dowwie/.agents/skills/rust-best-practices/SKILL.md`;
- `docs/plans/validator/tasks/T014.json` and the verification arrays in T007-T013;
- `docs/dispatches/validator-t007-t014/028-owner-to-coordinator.prompt.md`;
- `docs/dispatches/validator-t007-t014/029-coordinator-to-schema-test-developer.prompt.md`;
- `docs/dispatches/validator-t007-t014/029-schema-test-developer-to-coordinator.response.md`;
- `docs/dispatches/validator-t007-t014/030-coordinator-interruption.md`;
- the current `tests/cli.rs`, application/CLI code, and three output schemas.

The owner sequencing decision is exact: T014 must pass all behavior, format,
complete locked tests, release build, and diff checks. Run the exact warning-denied
Clippy command and record its real result. Only the exact staged private `dead_code`
inventory already mapped to T015-T021/Serde may remain; no ordinary warning, new
symbol, suppression, fake use, public export, or weakened assertion is allowed.
Fully warning-free gates remain mandatory at T027/T035.

## Current disk identity

Confirm before editing:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |
| `src/app.rs` | `1be35a7228f0a4d7bb4d5449e088f3914a0b3c9f28a0c3866a4466dbbb256fcf` |
| `src/cli.rs` | `608ab90dcda6cd36f8b4d03575f47efc8f2a5977480a0c4bb670aa37933a0fbe` |
| `src/lib.rs` | `b0c5d00e000747368fc9e18e6459c61417dbbf24f06ac58077e9c4cf382ceac7` |
| `src/main.rs` | `ca701aa73cb1d687d9ff086e654bd9b08e6c285a91c9577c3b2d1eb7fc54a3c0` |
| `src/error.rs` | `5e47f1afde84a682e6d33744133820a43c14a761383717fc91012c63b330abcb` |
| `src/evaluation.rs` | `dbee2b6c74f60d5375545b746dbfea3ecc311ad55f3183ab02bdc260157bf891` |
| `src/evaluation/single_label.rs` | `fd271a77be2c1c9aa244749cea9a026cc9efb3e0c00f5a263df428c82caaa19d` |
| `src/model/common.rs` | `f33f0350845f53d20539f7ac3506b15ddce8409a5a1cfd9c7030352a942c04c2` |
| `src/model/single_label.rs` | `c97ae399fe81896e0e552e0152e99db53addb14fa72043f7c68048f4bfb61e1b` |
| `src/validation.rs` | `5a76b0ccc8bd838c13e8671cfb063b27888f0f9b20ecb6e5fa6c26a03c8b49bf` |
| `src/validation/wire.rs` | `46a42903c2edbb556b1ae1ee7b3c3d9642e562e6b2de2a6d63a852c584f6a3b3` |
| `tests/conformance.rs` | `16bc7d8eb4644491ae86f30a85a5f3a69f8b471e8106351184b67e30809f9637` |
| `tests/cli.rs` | `5d46674b46b4ae8d86cc1bb050667f2f78bf69f7eab6d1659660f27c2697a994` |

Do not edit governance/index/Fizzy/session records, stage/commit/clean/reset, modify
`.zvec-grep`, or start/stop caffeinate PID 84732.

## Complete and prove the current CLI filters

Preserve correct in-flight assertions, but independently run and inspect them.
Complete the seven existing process requirements inside the three named filters:

1. Validate actual check, receipt, and error stdout against the locked schemas.
2. Missing/unreadable input exits 3 with one safe schema-valid error document.
3. An existing file/directory destination exits 3, remains byte/type-identical,
   and no call-owned temporary/final partial is left.
4. A structurally/semantically invalid input exits 2, emits one safe schema-valid
   error document, and creates no output.
5. Unknown flags, missing values, duplicate flags/settings, unsupported settings,
   and unimplemented commands exit 2; help/version remain text exemptions.
6. A successful evaluation with a real bound evidence file proves byte-identical
   published golden, predictions, config, and `evidence/0.bin` content.
7. A failed operation must actually validate/load sentinel-bearing input/evidence,
   then prove neither stdout nor stderr contains the sentinel. A parse failure that
   never opens the bundle does not count. An existing-destination failure after
   valid admission/evidence loading is an appropriate bounded reproduction.

Use isolated valid-base mutations and independent schema/byte checks. Preserve the
existing check-no-write, one-document stdout, stderr separation, success/hash,
help/version, numerical, and privacy evidence. Do not add filter names or derive
oracles from production.

Write `tests/cli.rs` only unless an assertion reproduces a real T014 production
defect. If it does, make only the smallest criterion-required source correction
and disclose it; do not redesign the parser/application boundary.

## Full required matrix

Run every exact named T007-T014 verification command from the task JSON files with
a nonzero passing count. Then run:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
git diff --check
```

All commands except the owner-staged Clippy limitation must exit 0. Save complete
Clippy diagnostics and reconcile every residual symbol exactly; it may fail only
for the approved staged `dead_code` set and no other warning.

Save the response before returning. Include exact input/output hashes, line-level
mapping from all seven requirements to assertions, every named command/exit/count,
schema/exits/filesystem/byte/privacy results, complete Clippy status/symbol mapping,
and unresolved failures. Explicitly state the checkpoint is not warning-free and
T027/T035 retain that gate. Do not return another partial response without an exact
reproduced blocker.
