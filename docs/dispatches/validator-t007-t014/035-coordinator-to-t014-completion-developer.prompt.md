# Validator T013/T014 focused repair dispatch 035

Role/model: retained sole developer
`/root/coordinator/t014_completion_developer`, `gpt-5.6-terra`, reasoning `high`,
retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/035-t014-completion-developer-to-coordinator.response.md`.
Fizzy: [Card 183](http://localhost:3006/1/cards/183).

This is the one authorized repair/recheck cycle for frozen manifest033. Read the
complete independent `Revise` verdict at
`docs/dispatches/validator-t007-t014/034-verifier-to-coordinator.response.md`
(SHA-256 `bb1fc45987ded8236ef9a7e244dfc2f8ef0782d826a32bab7832cea0c732236d`)
and frozen manifest033. Fix **only** findings 1 and 2. You are sole writer; do not
delegate, begin T015+, change any scoring formula/tolerance, or clean unrelated code.

Confirm the manifest033 product hashes before editing, especially:

| Path | SHA-256 |
|---|---|
| `src/app.rs` | `1be35a7228f0a4d7bb4d5449e088f3914a0b3c9f28a0c3866a4466dbbb256fcf` |
| `src/model/single_label.rs` | `c97ae399fe81896e0e552e0152e99db53addb14fa72043f7c68048f4bfb61e1b` |
| `schemas/v2/report.schema.json` | `a99d4ab2bfc85180b9ce7619e2399ed721b7e5a60d98459efc6935578e6fdb0b` |
| `tests/cli.rs` | `fc03a9d04dbe0c58faab9574ef9eb68d8f4347d6d43d6d5424ae55e7e6cdd111` |
| `tests/conformance.rs` | `16bc7d8eb4644491ae86f30a85a5f3a69f8b471e8106351184b67e30809f9637` |

Do not edit governance/index/Fizzy/session records, stage/commit/clean/reset, modify
`.zvec-grep`, or start/stop caffeinate PID 84732.

## Finding 1: truthful check normalization diagnostics

`check` must report the actual number of admitted categorical rows whose submitted
vector differs from its normalized working vector and the maximum absolute
submitted-sum error. It must remain validation-only: no scoring, report assembly,
ID/time generation, or publication.

Create the smallest shared non-scoring helper owned with the checked single-label
model that derives these diagnostics from admitted evaluation rows. Use it from
both `src/app.rs` check construction and T013 report assembly so the two outputs
cannot drift. Remove the report's duplicated derivation. Use checked count
conversion and preserve the existing numerical tolerance/normalization behavior.

Extend the existing built-binary `cli_check_evaluate` filter with a valid
within-tolerance non-unit categorical vector such as `[0.7, 0.2, 0.1000000005]`.
Assert schema-valid check stdout reports `normalized_count: 1` and the actual
nonzero maximum sum error within the fixed fixture tolerance. Preserve its
no-write property. Do not derive the expected error through production code.

## Finding 2: minimum-two report vocabulary

Change only the single-label task branch in `schemas/v2/report.schema.json` so
`task.labels` requires at least two unique labels. Update the handcrafted positive
report in `single_report_schema` to a structurally consistent two-label document,
then add one discriminating mutation proving a one-label report is rejected. Do
not broaden or redesign other report/schema fields.

## Allowed writes

- `src/model/single_label.rs` for the shared non-scoring diagnostic helper and
  report reuse;
- `src/app.rs` for truthful check fields;
- `tests/cli.rs` for the actual built-binary check regression;
- `schemas/v2/report.schema.json` for `minItems: 2` only;
- `tests/conformance.rs` for the valid two-label fixture and one-label negative;
- the required response file.

## Required checks and handoff

Run at minimum:

```sh
cargo test --locked --test cli cli_check_evaluate -- --nocapture
cargo test --locked --test conformance single_report_schema -- --nocapture
cargo test --locked --lib report_sources_and_privacy -- --nocapture
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
git diff --check
```

The three focused filters, format, full tests, release, and diff must exit 0.
Clippy must retain only the exact owner-staged inventory, with no new/ordinary
warning. Save the complete response before returning with input/output hashes,
criterion-to-code/test mapping, commands/exits/counts, exact numeric assertion,
schema mutation result, and Clippy reconciliation. This checkpoint remains not
warning-free; T027/T035 retain that gate. Stop after the handoff for focused
independent recheck.
