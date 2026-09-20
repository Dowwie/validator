# Validator omitted-selection invariant correction 023

Role/model: existing replacement sole developer, `gpt-5.6-terra`, reasoning
`high`, fresh context retained from dispatches 014/016/018.
Repository: `/Users/dowwie/MyProjects/validator`.
Required response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t004-t006/023-developer-to-coordinator.response.md`.

Read `022-owner-to-coordinator.prompt.md` and
`022-coordinator-to-owner.response.md` in full. This is one explicitly owner-
authorized final narrow completion attempt. It does not reset the prior repair
allowance. Do not delegate or spawn agents.

## Exact scope and baseline

Modify only:

- `src/model/common.rs`, including its owning tests;
- the required response file.

All other source, Cargo, test-module, specification, plan, documentation, index,
Fizzy, and `.zvec-grep` files are read-only. Do not stage, commit, clean, reset, or
start/stop caffeinate.

Bind repair manifest 019 SHA-256
`2ef9dc6c9c7c8d6c0863c956c291ab52d5e3366f6d566f895b54a1044daad7d2`,
verifier response 020 SHA-256
`4a121340e640d12fb0a3c44e7dac6afd82ddbf16af244729c4647ab0d46423c6`,
and current `src/model/common.rs` SHA-256
`7892d11b1d16ef6a548ae20eac60177d2d82a80f43d2426ed8b83196c98b5859`.
Confirm them before editing.

Apply `/Users/dowwie/.codex/AGENTS.md`, repository `AGENTS.md`, and the previously
read Rust/team skills. Make the smallest surgical change.

## Required invariant and proof

In `Population::new`, enforce the checked configuration semantics:

- when `requested_episode_ids()` is `None`, `unselected` must be empty because
  omission means select all;
- an omitted selection over an empty dataset remains legal;
- valid omitted/all, explicit `[]`/select-none, and explicit subset populations
  remain legal;
- preserve existing duplicate, overlap, sorting, explicit-selection consistency,
  digest, count, description/role/parent, and typed error behavior;
- use the existing typed partition/invariant error and add no new abstraction,
  visibility, API, fallback, or unrelated refactor.

Extend the existing owning-module `population_alignment` filter with the exact
negative construction from verifier 020: checked config with omitted requested
IDs, selected `[first]`, and unselected `[second]` must fail. Include a positive
empty-dataset omitted-selection construction if it is not already proven. Keep the
filter's existing positive cases.

Run and record exact counts/exits:

```sh
cargo test --locked --lib population_alignment -- --nocapture
cargo test --locked --lib validate_before_selection -- --nocapture
cargo test --locked --lib dataset_digest_binding -- --nocapture
cargo fmt --all -- --check
cargo test --all-features --locked
cargo build --locked --bin validator
cargo clippy --all-targets --all-features --locked -- -D warnings
git diff --check
```

Clippy may exit 101 only on the updated known incomplete-consumer `dead_code`;
record counts and any other warning class as a blocker. Reuse all unchanged
numeric/T004/T005 evidence and do not rerun their separate filter batch. Do not add
suppressions, fake callers, placeholders, or fallbacks.

Save a concise complete response with baseline/output hashes, exact changed lines'
purpose, positive/negative construction evidence, every command result/count,
warning classification, unchanged-file confirmation, and process/resource state.
Do not claim acceptance. If incomplete or failing, return the exact cause to the
coordinator; no further worker loop is authorized.

