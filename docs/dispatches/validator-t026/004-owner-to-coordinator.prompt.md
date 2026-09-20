# Complete the missing T026 replay matrix dimensions before handoff

Existing Sol-high coordinator; repository `/Users/dowwie/MyProjects/validator`.
Save acknowledgment to `docs/dispatches/validator-t026/004-coordinator-to-owner.response.md`.

Owner inspected the exact test inventory discrepancy and its underlying shared
scenario. Restoring the old filter through a wrapper is correct, but the current
matrix still does not fulfill output1. Do not accept a passing wrapper as the
complete required safety matrix or advance to output2 on that basis.

Concrete current evidence in `tests/conformance.rs`:
`artifact_adversarial_matrix` calls `replay_binding_and_result_tampering_scenario`.
That scenario constructs only `replay_run()` (single-label), and each mutation
asserts only `inspect_replay(&run).is_ok()` equals a boolean. It does not exercise
multi-label computed-float/count/configuration replay, precise error codes, or
failed comparison publication for those corruptions. Those are explicit parts of
owner001 and developer002, not additional review scope. `case_s25` calling the
same scenario does not add the missing paths.

## Bounded completion sequence

Preserve the useful current work and all exact named filters. Have the current
sole developer complete the replay matrix first, before rerunning the full local
boundary or writing a completion claim:

1. Use actual single-label and multi-label run fixtures with appropriate evidence.
   Reuse existing setup and mutation helpers; make only the small explicit
   task-specific choices needed for the two computed metric paths. No generic
   test framework, dynamic metric discovery or duplicate production logic.
2. For each required corruption, assert the precise spec-derived public error
   category through verified inspect and comparison, and assert no comparison
   output directory is published. Retain the valid unmodified control. Make
   missing/extra/duplicate bindings and the other already-assigned provenance,
   path, snapshot, source-array and result mutations genuinely discriminate their
   intended condition rather than merely exercise any failure.
3. For each task, prove a computed metric within declared tolerance succeeds and
   count/recorded configuration/status corruption fails with its precise category.
   Recorded metric-shaped configuration remains exact. The multi-label metric
   path must be actual multi-label output, not a substituted single-label key.
4. Save a concise checkpoint result identifying both task paths and tested
   mutation/error/publication outcomes. If this exposes a product violation,
   preserve the failure and make only the already-authorized owning-layer fix;
   coordinate required artifact/input mapping before the edit.

Then reconcile the remainder of output1's publication/privacy/receipt/relocation
obligations collectively against the existing and new assertions. Reusing existing
complete tests is welcome; identify the actual assertions proving each requirement.
Do not duplicate CLI metric evidence already proved through the API, but do not
substitute API error categories for required real process exits. A named wrapper
is not itself new behavioral proof.

The expected conformance inventory is81 after preserving the original filter,
adding the matrix and six S23-S28 filters; the CLI inventory is11 unless a necessary
new exact case changes it and is explained. Finish the existing local checks and
full response002 only when the original output1 contract is satisfied. Record any
earlier partial handoff honestly rather than overwriting its history. No extra
independent review is requested; the combined T026 review remains later. This is a
bounded completion of the original assignment, not a fresh repair allowance.

Index/link this decision and acknowledgment, and route any consequential worker
follow-up in a new numbered prompt. Keep Card188's first step incomplete until
the full output is supported. T027 and output2 remain unstarted meanwhile.
