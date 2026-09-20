# T004-T006 checked-admission handoff

Implemented the ordered T004, T005, and T006 checked-admission candidate. No constituent task is claimed independently accepted; it is ready for frozen independent review and owner decision.

## Scope, inputs, and changed hashes

Accepted inputs were preserved, including accepted T002 repair manifest SHA-256 3e0c155c1a64e6ec75d8bf518400fcc3c12d283ec3774de705833eb912843e91. Cargo manifests, lockfile, toolchain, lib/error/main, the wire DTO contract, documentation, Fizzy, and .zvec-grep were not edited. No dependency was added.

| Changed path | Current SHA-256 |
|---|---|
| src/model/common.rs | 6dd18b99bad4be57dbb4c7a1add44b0aa711a37234a0b1c501d207f1dcf79af5 |
| src/model/single_label.rs | 35fe11ada58053273708751631c079984f86e6e6fad3f7d24dcae502c4b92fe5 |
| src/model.rs | ceffb61ab5e523fa4bf3161d5db5a6fcd1276137d951982da4382edb79c58c66 |
| src/validation.rs | 1a085d123a35fd270fbd30359bcde449e0ba645f11b59d9eebe63547dcb0f3f5 |

git diff --check passed. The repository remains unborn/untracked; no staging, committing, reset, or clean was performed.

## T004

common now owns checked source records, closed observation definitions and values, preparation bindings, a real ObservationSet, and generic Prediction output. validation consumes strict T002 DTOs into those records, retaining declared-but-unused sources and opaque source metadata.

The conversion enforces nonblank source/definition identifiers, a question ID for scored choice, valid preparation evidence indices, and definition-kind agreement. Scalars and vectors retain supplied values without normalization or vocabulary binding. Invalid values return E_OBSERVATION; unknown names or mismatches return E_PROVENANCE.

Both named checks passed with exit 0 and one test each:

    cargo test --locked --lib observation_contracts -- --nocapture
    cargo test --locked --lib source_preparation_bindings -- --nocapture

## T005

single_label now owns categorical submitted/normalized representations, reported confidence, as-recorded output, scored-choice admission, and independent artifact-wide signal availability. Classifier choices may differ from argmax; scored choice requires an answered tied-maximum class, categorical distribution, confidence, and source question ID. Signal presence is checked over the submitted artifact before population selection, including abstentions; empty artifacts have neither family.

All named checks passed with exit 0 and one test each:

    cargo test --locked --lib categorical_admission -- --nocapture
    cargo test --locked --lib scored_choice_ties -- --nocapture
    cargo test --locked --lib artifact_signal_completeness -- --nocapture

## T006

validate_single_label is a pure exact-byte entry point. It decodes through T002, binds the declared dataset digest to the caller-supplied digest, validates every dataset/source/prediction/configuration record before selection, then returns closed ValidatedEvaluation::SingleLabel.

The completed evaluation owns checked vocabulary, sources, as-recorded policy, signal availability, population description and role, optional checked parent run ID, UUID-sorted selected/unselected IDs, and private aligned rows. It rejects duplicate dataset/prediction IDs, unknown sources, malformed unselected records, non-as-recorded policy, invalid selection IDs, and missing/extra predictions. Omitted selection selects all; explicit empty selection selects none.

All named checks passed with exit 0 and one test each:

    cargo test --locked --lib population_alignment -- --nocapture
    cargo test --locked --lib validate_before_selection -- --nocapture
    cargo test --locked --lib dataset_digest_binding -- --nocapture

Additional passing regressions are checked_admission_builds_the_closed_evaluation and malformed_artifact_fails_before_population_selection; the latter proves malformed artifact content fails before a bad requested population is considered.

## Verification and gate result

The listed inventory contains 19 tests. These checks passed:

    cargo test --locked --lib -- --list
    cargo fmt --all -- --check
    cargo test --all-features --locked
    cargo build --locked --bin validator
    git diff --check

cargo clippy --all-targets --all-features --locked -- -D warnings exits 101 solely for dead_code: 97 library diagnostics and 24 library-test diagnostics. They are incomplete-consumer records, fields, methods, and conversion helpers in src/model/common.rs, src/model/single_label.rs, src/validation.rs, src/validation/wire.rs, and accepted task wrappers in src/model.rs. There are no unused-import diagnostics or other lint classes. No lint suppression, fake runtime caller, placeholder use, or public export was added.

One local compile correction was made: borrowed outputs could not satisfy the slice signal API before rows moved. A private presence-flag helper now performs the same all-or-none check before ownership transfer. There was no contract ambiguity, external dependency, network operation, or resource blocker.

