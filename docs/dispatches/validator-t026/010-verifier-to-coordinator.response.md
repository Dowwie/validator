# T026 independent combined verification verdict

Verdict: **Revise**.

The frozen candidate is mechanically intact and its complete bounded execution
evidence passes. It is not Ready because six exact named conformance filters do
not assert all behavior assigned to their own ratified row. Owner prompt001 says
that a named case may reuse a complete scenario, but its actual assertions must
cover its entire row. Passing a separate umbrella or CLI filter does not satisfy
that exact-filter requirement.

## Candidate identity and review boundary

The SHA-256 of `009-coordinator-candidate-manifest.md` is exactly
`fcb905dcee73958340458b88433e5bd43cc2cc78cfadd37470a19ed50a1efb9f`.
Every hash listed in manifest009 reconciled before review. I repeated the frozen
candidate hashes after all executions; every source, schema, test, contract,
mapping and toolchain identity remains equal to manifest009.

I read the global and repository instructions, the complete manage-dev-team
skill, T026, owner prompts001/004/006, complete developer responses005/008, both
ratified specifications at every T026 coverage block, and every coverage entry
whose implementation or verification tasks include T026. I inspected the
complete T026-owned tests and the necessary app, comparison, artifact, CLI and
multi-label report paths. I did not inspect T028 work.

## Required revisions

### R1 — `case_s15` does not prove both full origins through inspection and comparison

Requirement: S15 requires two source definitions and correct row references,
with both origins available in inspection and comparison. Prompt008 makes this
explicit as both full origin definitions through verified inspection and both
sources/configurations in comparison.

Location: `tests/conformance.rs:1587-1661`.

Exact input: Q1 has model `retained-A` and configuration
`{"question":"Q1","revision":1}`; Q2 has model `revised-B` and configuration
`{"question":"Q2","revision":2}`. The two fixed episodes reference Q1 and Q2.

Expected: the exact filter verifies both inspected origins and both complete
source definitions/configurations on the comparison sides.

Actual: the filter inspects only the Q2 episode and asserts only
`prediction.source_id == "Q2"` at line 1639. Its comparison assertions check the
baseline source count and the two model strings, but not either comparison-side
configuration and not the candidate-side source definitions. The report
assertions do contain both configurations, but a report assertion is not the
required inspection/comparison assertion.

Consequence: the named S15 filter can pass if inspection loses Q1 or full origin
information, or if comparison mispublishes either source configuration.

Smallest correction: extend this exact filter to inspect both fixed episodes and
assert the origin information the public inspection contract supplies, then
assert the complete Q1/Q2 definitions and configurations on both comparison
sides. If the intended inspection contract requires fields that the current
public result cannot expose, return that exact mismatch for owner scope resolution
instead of weakening S15.

### R2 — `case_s24` verifies relocation inspection but never relocation comparison

Requirement: S24 requires both inspection and comparison to succeed from copied
evidence alone after original sources become unavailable.

Location: `tests/conformance.rs:1476-1491`.

Exact input: a real saved single-label run is moved to `relocated`; the submitted
golden, predictions, config and both original evidence files are then deleted.

Expected: public inspection and public comparison both succeed against the
relocated stored run without any original-path lookup.

Actual: line 1490 asserts only `inspect_replay(&relocated).is_ok()`. The filter
never invokes comparison and therefore exits successfully even if relocated
comparison is broken. The separately executed CLI matrix compares a different
relocated scenario, but `case_s24` neither calls nor reuses that workflow.

Consequence: the required exact S24 filter is not discriminating for half of its
ratified row.

Smallest correction: compare the relocated verified run through the public API,
assert success and publication of `comparison.json`, while the original files
remain absent.

### R3 — `case_s26` does not verify an evaluation receipt

Requirement: S26 requires evaluation and comparison receipts whose `result_path`
names the actual result file and whose `result_sha256` equals an independent hash
of that file's exact bytes.

Location: `tests/conformance.rs:1500-1530`.

Exact input: `replay_run()` publishes an evaluation run, after which the filter
publishes a self-comparison.

Expected: the filter retains and verifies the evaluation receipt against
`report.json`, then independently verifies the comparison receipt against
`comparison.json`.

Actual: `replay_run()` discards the evaluation receipt. Lines 1503-1509 hash the
report but assert only that the digest string has length 64; they never compare it
with an evaluation `result_sha256` or assert the evaluation `result_path`. Only
the comparison receipt is actually verified at lines 1518-1529.

Consequence: `case_s26` passes if evaluation receipts point to the wrong file or
contain the wrong digest.

Smallest correction: retain the real evaluation receipt in this scenario and
assert its absolute `report.json` path and independently calculated SHA-256 before
checking the comparison receipt.

### R4 — `case_s27` does not exercise the required error privacy surface

Requirement: S27 requires opaque input and arbitrary evidence contents to remain
absent from reports and errors, while explicit inspection returns the stored
selected input. Owner prompt001 also names routine stdout/stderr/errors/reports.

Location: `tests/conformance.rs:1533-1572`.

Exact input: the selected episode contains `S27-opaque-input`; the bound evidence
contains `S27-evidence-content`.

Expected: the exact filter verifies neither sentinel appears in a real public
error result and verifies explicit inspection returns the opaque input.

Actual: lines 1558-1560 check only `report.json`; lines 1562-1571 check successful
inspection. The filter triggers no failing operation and examines no error. A
separate CLI matrix checks related error privacy, but `case_s27` does not reuse it.

Consequence: the exact S27 filter can pass while its required error surface leaks
input or evidence content.

Smallest correction: add one real public failure using the sentinel-bound input,
assert the typed error, and assert its machine serialization contains neither
sentinel; retain the existing report and inspection assertions.

### R5 — `case_m17` does not assert vocabulary order

Requirement: M17 and prompt008 require vocabulary-ordered per-label transition
evidence for the fixed `{A}` to `{B}` change against reference `{A,B}`.

Location: `tests/conformance.rs:1946-1993`.

Exact input: vocabulary `[A,B]`, baseline outcome `{A}`, candidate outcome `{B}`.

Expected: `transitions.per_label` is explicitly ordered `[A,B]`, with A
`present -> absent` and B `absent -> present` counts fixed at one.

Actual: the test searches the array with `.find()` independently for A and B at
lines 1977-1989. Reversing the array would still pass every M17 assertion.

Consequence: the named test does not prove the required deterministic vocabulary
ordering.

Smallest correction: assert the exact per-label label sequence before checking
the two transition cells.

### R6 — `case_m18` omits the assigned conditional metric availability/change assertions

Requirement: M18 requires the same selected population, explicit answered
populations and overlap, and conditional metric changes that remain labeled as
descriptive population-conditioned values rather than paired model improvement.
Prompt008 specifically requires conditional metric availability/changes.

Location: `tests/conformance.rs:1997-2035`.

Exact input: the fixed multi-label comparison has baseline answered IDs `{1,3}`,
candidate answered IDs `{1,2}`, and overlap `{1}`.

Expected: the exact filter asserts those populations and overlap plus the
conditional metric statuses/values/delta or other machine fields that establish
the actual conditional change without an improvement claim.

Actual: the filter asserts the two ID lists, counts, overlap count and
`population_scope == "answered"`. It never asserts any conditional metric status,
value or delta and never checks the absence of an overall improvement/winner
claim.

Consequence: M18 can pass with missing or incorrect conditional metric results,
which is the behavior the case exists to preserve.

Smallest correction: assert independently fixed baseline/candidate conditional
metric results and their status/delta fields, plus the relevant absence of an
overall improvement/winner representation.

## Verified behavior and executed evidence

The replay matrix itself uses real single-label and multi-label controls. Its
corruptions exercise snapshots, evidence bytes and symlink containment,
missing/extra/duplicate/swapped bindings, original/index/ordinal/source-array
changes, exact counts/configuration/status, and stored results through both public
inspect and compare with precise code/stage assertions and absent rejected output.
The multi-label computed-path classifier is closed over hard, per-label,
answered-macro, binary-loss/Brier and marginal-bin value families. It contains no
dynamic suffix discovery. Inside-tolerance representatives pass; outside
tolerance and exact operand/count/status/configuration mutations reject. The
constants remain `1e-12` absolute and `1e-10` relative, and the single-label
paths remain present.

The shared production publisher uses an atomic no-replace rename. Its owning tests
deterministically cover concurrent file/directory/symlink creation and a failure
after temporary publication is populated. The real CLI matrices cover both task
kinds, receipts, process exits, relocation, privacy and large-integer inspection.
Those collective checks are useful and pass; they do not replace the incomplete
assertions required inside the six exact filters above.

Every required literal filter executed one nonzero test and exited 0:
`case_s15`, `case_s16`, `case_s18`, `case_s19`, `case_s20`, `case_s22` through
`case_s28`, `case_m17` through `case_m19`, `case_e08`,
`artifact_adversarial_matrix`, `replay_binding_and_result_tampering`, and
`publication_and_privacy_matrix`.

Inventory commands exited 0 and listed exactly 91 conformance tests and 11 CLI
tests. `cargo test --all-features --locked` exited 0 with 44 library, 11 CLI, 91
conformance and zero doc tests passing. Current warning-denied Clippy exited 101
with exactly the accepted 17 production diagnostics plus the three duplicate
lib-test diagnostics and no new diagnostic.

No source, schema, test, fixture, governance, plan, index, session note,
acceptance record or Fizzy state was edited during this review. T028 was not
inspected. A focused correction of these exact-filter assertions, followed by a
new frozen manifest and bounded recheck, is sufficient; no T027 work, private
source review, generic audit framework or duplicate full workflow is requested.
