# Validator T014 process-evidence completion dispatch 030

Role/model: retained sole developer `/root/coordinator/schema_test_developer`,
`gpt-5.6-terra`, reasoning `high`, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required complete response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/030-schema-test-developer-to-coordinator.response.md`.
Fizzy: [Card 183](http://localhost:3006/1/cards/183).

Your response029 is explicitly partial without a contract blocker. Complete the
unchanged remainder of owner decision028 and coordinator prompt029 now. This is
one bounded continuation of T014, not a new allowance or scope. Remain sole writer,
do not delegate, and stop after the complete handoff. Do not begin T015+.

## Preserved starting identity

Read `028-owner-to-coordinator.prompt.md`, `029-coordinator-to-schema-test-developer.prompt.md`,
and your full `029-schema-test-developer-to-coordinator.response.md`. Confirm:

| Path | SHA-256 |
|---|---|
| `src/app.rs` | `1be35a7228f0a4d7bb4d5449e088f3914a0b3c9f28a0c3866a4466dbbb256fcf` |
| `src/evaluation.rs` | `dbee2b6c74f60d5375545b746dbfea3ecc311ad55f3183ab02bdc260157bf891` |
| `src/evaluation/single_label.rs` | `fd271a77be2c1c9aa244749cea9a026cc9efb3e0c00f5a263df428c82caaa19d` |
| `src/model/common.rs` | `f33f0350845f53d20539f7ac3506b15ddce8409a5a1cfd9c7030352a942c04c2` |
| `src/validation.rs` | `5a76b0ccc8bd838c13e8671cfb063b27888f0f9b20ecb6e5fa6c26a03c8b49bf` |
| `tests/cli.rs` | `d0e255789545846fb57c97c3338cee6344d5cf2f9f915fc3a3df88eaf6d77e4b` |
| `tests/conformance.rs` | `16bc7d8eb4644491ae86f30a85a5f3a69f8b471e8106351184b67e30809f9637` |
| `029-schema-test-developer-to-coordinator.response.md` | `968f178d35978607d85d24b6b6e2b5613302f61bac9ba53e33f86208a61e300a` |

Preserve the completed cleanup. Do not edit governance/index/Fizzy/session records,
stage/commit/clean/reset, modify `.zvec-grep`, or start/stop caffeinate PID 84732.

## Only remaining implementation evidence

Strengthen only the existing three named `tests/cli.rs` filters through the built
binary. Complete every assertion already specified in prompt029:

1. actual check/receipt/error stdout validated with the locked three schemas;
2. missing/unreadable input: exit 3, one safe error JSON;
3. existing destination: exit 3, winner unchanged, no owned partial/temp output;
4. structurally or semantically invalid input: exit 2, one safe error, no output;
5. unknown flags, missing values, duplicates/unsupported settings, and unimplemented
   commands: exit 2; help/version remain successful text exemptions;
6. evidence-bearing success: published golden/predictions/config/evidence bytes are
   exactly the submitted bytes;
7. a failure that actually processes sentinel-bearing input/evidence leaks the
   sentinel to neither stdout nor stderr.

Use isolated valid bases and independent byte/schema checks. Do not claim a generic
JSON parse or path existence is equivalent. Do not add filter names or derive
numerical oracles from production. Write `tests/cli.rs` only unless a new assertion
reproduces an actual T014 behavior defect; if so, report the exact reproduction and
smallest source correction before changing production behavior.

## Complete final matrix

Run every named T007-T014 filter from the task JSON files with a nonzero passing
count. Then run:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked --bin validator
git diff --check
```

All behavior, format, complete test, release, and diff commands must exit 0. Clippy
must be run unmodified and may exit nonzero only for the exact owner-approved staged
`dead_code` symbols. It must contain no ordinary lint and no new symbol beyond the
post-cleanup approved inventory. Save its full exact diagnostics and reconcile each
symbol, not only the aggregate.

Save a complete response before returning. Include all starting/output hashes,
line-level mapping of the seven cases to assertions, every named command/exit/test
count, output-schema validation, exits, byte preservation, sentinel result, final
Clippy status and exact residual symbols. Explicitly state that T014 is not warning-
free and T027/T035 retain that gate. A second partial handoff without a concrete
reproduced blocker must be returned to the owner for staffing reassessment.
