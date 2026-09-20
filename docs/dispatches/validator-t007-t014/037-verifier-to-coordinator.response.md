# T007-T014 focused repaired-candidate recheck

## Verdict: Ready

Both findings from response034 are corrected on the repaired frozen candidate.
The focused tests and justified full regressions pass, no second material defect
was found, and the only nonzero gate remains the exact owner-authorized staged
Clippy inventory. I recommend owner acceptance of the repaired T007-T014
checkpoint; this verdict does not authorize integration or advancement.

## Repaired candidate identity

Before review and again after all commands, I verified the bindings in
`036-coordinator-repaired-manifest.md`:

| Binding | Verified SHA-256 |
|---|---|
| `033-coordinator-final-manifest.md` | `49574acc96a5b8520daf3dd68fabe56ddfd08ff21f5bfb566a282a57c9475112` |
| `034-verifier-to-coordinator.response.md` | `bb1fc45987ded8236ef9a7e244dfc2f8ef0782d826a32bab7832cea0c732236d` |
| `035-t014-completion-developer-to-coordinator.response.md` | `65c347cac6d67ffc400347877f0f652c3a0f2f4eb2b63e6827fa2f2f7899d7ab` |
| `src/app.rs` | `819627406e44e184df13d8baf4c1feb115ea0444cf9b7518f83afd2b9e02b875` |
| `src/model/single_label.rs` | `899963586c722a37af0ce0dd8f28d03b86423c8c9bbce431b14280ae8a6cee98` |
| `schemas/v2/report.schema.json` | `4ff9c9f1ad59cd2ed73965c9cf154209e6eaf1b6871691c8d5128ca4cbd7ff46` |
| `tests/cli.rs` | `3b0cc62faedcb00e46e76a407b3bdf384da90b4596d593a4bf53da46273a567e` |
| `tests/conformance.rs` | `982f128ba6a1b66e3071113b3c05a7fd37ef7269d20a7fa8f34644bb596075a2` |

All other 43 inputs from manifest033 remained byte-identical to their original
declared hashes. The repaired candidate did not move during review.

## Finding-by-finding evidence

### Finding 1: truthful non-scoring check normalization diagnostics — corrected

`src/model/single_label.rs:141-170` defines one shared
`normalization_diagnostics` helper over admitted `SingleLabelEvaluation` rows. It
only examines checked categorical submitted/working vectors, uses checked count
increment plus checked `usize` to `u64` conversion, and derives the maximum
absolute submitted-sum error. It does not score, assemble a report, generate a
run identity/time, access the filesystem, or publish.

`src/app.rs:80-109` calls the helper after shared admission and puts its exact
values in `CheckResult`. `src/model/single_label.rs:542-552,810-819` calls the same
helper for report integrity; the previous results-derived duplicate calculation
is gone.

The built-binary regression at `tests/cli.rs:29-35,103-125` supplies
`[0.7,0.2,0.1000000005]`, validates actual check stdout against the check schema,
requires `normalized_count == 1`, requires a nonzero maximum error, and compares
that value with an independently calculated submitted sum error within the fixed
fixture tolerance. It retains the assertion that check creates no run path. The
focused CLI test passed with one test.

### Finding 2: minimum-two single-label report vocabulary — corrected

`schemas/v2/report.schema.json:50` now requires `task.labels.minItems: 2` for the
single-label report branch. The positive handcrafted report at
`tests/conformance.rs:116-154` is coherent across the two-label A/B vocabulary,
three typed matrix columns, two matrix rows, two class entries, and two-element
submitted/working probability vectors. The mutation at lines 173-175 changes only
the task label array to one entry and requires schema rejection. The focused
schema test passed with one test.

The report/privacy unit path also passed, confirming the shared normalization
helper did not regress report assembly or privacy behavior.

## Focused and full command results

| Command | Exit | Result |
|---|---:|---|
| `cargo test --locked --test cli cli_check_evaluate -- --nocapture` | 0 | 1 passed, nonzero count. |
| `cargo test --locked --lib report_sources_and_privacy -- --nocapture` | 0 | 1 passed, nonzero count. |
| `cargo test --locked --test conformance single_report_schema -- --nocapture` | 0 | 1 passed, nonzero count. |
| `cargo fmt --all -- --check` | 0 | Passed. |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | 101 | Exact staged inventory only; no new or ordinary lint. |
| `cargo test --all-features --locked` | 0 | 39 library, 0 binary-unit, 3 CLI, 8 conformance, and 0 doc tests passed. |
| `cargo build --release --locked --bin validator` | 0 | Passed with only the staged warnings. |
| `git diff --check` | 0 | Passed. |
| `ruby docs/plans/validator/verify-plan.rb` | 0 | Passed: 35 tasks, acyclic dependencies, 433 source blocks, 63 conformance cases, 12 structure checks, 8 ACs, 5 DoD clauses, physical owners, local links, and artifact index. |

Per dispatch037, I reused the settled T007-T012 and unaffected T013/T014 evidence
from response034 and did not rerun the other 15 named filters.

## Staged Clippy reconciliation

The warning-denied Clippy command still exits 101 for exactly the same 26 unique
production `dead_code` diagnostics recorded in response032, manifest033, and
response034: `checked_mul`; the staged single/multi-label task definitions and
methods; multi-label vocabulary/set contracts; `Episode`; observation access;
evaluation policy and dataset-digest access; label-decision metrics; metric
accessors; `signal_availability`; and the six strict Serde DTO groups. The test
target reports the same duplicated six-member subset. The new normalization
helper is consumed by both production callers and adds no warning.

No ordinary/new lint class, suppression, fake consumer, widened export, compile
failure, or behavior-test failure appeared. This checkpoint is **not
warning-free**. T027 and T035 retain the mandatory clean Clippy gate.

## Findings

None.

## Residual risks

- The exact 26-production-diagnostic Clippy inventory remains intentionally staged
  under owner decision028; this Ready verdict does not describe the checkpoint as
  warning-free.
- This focused recheck did not review T015+, replay, inspect, comparison, or
  multi-label behavior. Those remain outside this checkpoint.
- Unchanged T007-T012 and unaffected T013/T014 evidence is reused from the
  hash-bound response034 review; this recheck establishes only the two repairs and
  their justified regressions.
