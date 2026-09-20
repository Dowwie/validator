# T014 bounded completion handoff

Status: partial bounded completion. The authorized ordinary lint and obsolete-boundary changes are complete; the required seven expanded process assertions and complete final command matrix remain to be run/recorded by the next authorized T014 continuation.

## Completed changes

- Removed the `RunId` clone-on-copy in `src/app.rs`.
- Grouped probability result accumulators in `src/evaluation/single_label.rs` without changing metric formulas, counts, statuses, or populations.
- Replaced the redundant `checked_add` closure and the needless `Ok(...?)` ratio return.
- Removed duplicate decoded byte storage and the obsolete `Decoded::bytes`/`value` accessors. Validation tests now inspect the private DTO directly; exact submitted bytes remain owned by `InputArtifacts`.
- Removed the genuinely unused `ObservationSet::is_empty` initially, then restored it because an existing single-label unit test is its real current consumer.
- Corrected newly exposed ordinary test-style findings: literal boolean assertion and redundant result-pattern assertion.

## Verification actually completed

- `cargo fmt --all` exited 0.
- `cargo test --locked --lib --quiet` exited 0: 39 tests passed.
- `cargo test --locked --test cli cli_check_evaluate -- --nocapture` exited 0: 1 test passed.
- The first post-cleanup `cargo clippy --all-targets --all-features --locked -- -D warnings` exited 101 with only staged dead-code diagnostics plus two ordinary test-style findings; both ordinary findings were corrected afterward. A final Clippy rerun and the full T007–T014 command matrix were not completed before this handoff.

## Staged Clippy inventory

The remaining expected staged diagnostics are the owner-approved later-consumer/Serde groups from escalation027: `checked_mul`; `TaskDefinition`, `SingleLabelTask`, `MultiLabelTask`; multi-label `LabelVocabulary`/`LabelSet`; `Episode`; observation inspection accessors; checked Value convenience constructors; policy/dataset metric-comparison accessors and `MetricUnit::LabelDecision`; `signal_availability`; and strict wire DTO version/tag/input/rejected-policy fields. The obsolete `Decoded` bytes/accessors were removed. The checkpoint is not warning-free; final warning-free gates remain T027/T035.

## Required continuation

Continue T014 only: add the seven owner-directed real binary process assertions to the three existing `tests/cli.rs` filters, rerun every named T007–T014 filter, then run format check, unmodified Clippy, complete locked tests, release build, and diff check. Do not begin T015 or later behavior.

Current changed-file hashes: `src/app.rs` `1be35a7228f0a4d7bb4d5449e088f3914a0b3c9f28a0c3866a4466dbbb256fcf`; `src/evaluation/single_label.rs` `fd271a77be2c1c9aa244749cea9a026cc9efb3e0c00f5a263df428c82caaa19d`; `src/model/common.rs` `f33f0350845f53d20539f7ac3506b15ddce8409a5a1cfd9c7030352a942c04c2`; `src/validation.rs` `5a76b0ccc8bd838c13e8671cfb063b27888f0f9b20ecb6e5fa6c26a03c8b49bf`; `tests/cli.rs` unchanged at `d0e255789545846fb57c97c3338cee6344d5cf2f9f915fc3a3df88eaf6d77e4b`.
