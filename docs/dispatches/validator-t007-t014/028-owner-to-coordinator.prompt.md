# Finish T014 evidence and correct the premature clean-lint boundary

Role/model: existing Sol-high coordinator, retained context.
Required complete acknowledgment:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t007-t014/028-coordinator-to-owner.response.md`.

The owner read response026, escalation027 including the Clippy diagnostics, the
actual CLI tests, and the relevant strict DTO/validation/model code. Preserve the
working implementation and complete the existing T014 requirements below.

## Sequencing decision

The earlier owner-imposed warning-free T014/T017 boundary was premature: the plan
deliberately constructs shared private contracts before their production consumers
in T015-T021. Moving those consumers ahead of the architecture gate or deleting
and recreating accepted contracts would add risk without improving the product.
Do neither. The full warning-free gate remains mandatory at T027 and T035, after
the required runtime paths exist. This is a correction to intermediate execution
sequencing, not a relaxation of the final specification or a passed Clippy result.

At T014 and T017, still run the exact unmodified warning-denied Clippy command and
save its real exit status and diagnostics. Only the existing `dead_code` symbols
enumerated in escalation027 may remain temporarily unresolved. No new warning,
other lint class, compile error, test failure, or build failure is covered. Reconcile
the exact remaining symbol inventory rather than relying on an aggregate count.
All format, complete locked tests, release build and task-specific behavior gates
must pass. No lint flags, allow/expect attributes, underscore/dead reads, artificial
consumers, expanded exports, fallback paths, removed assertions or test weakening.
The independent review must explicitly state this staged Clippy limitation. Do not
describe the checkpoint or product as warning-free or fully complete.

The DTO version/tag guards genuinely validate input during deserialization; do not
delete fields or loosen admission merely to clear their diagnostics. Their final
representation must be reconciled as part of the actual T018/T021 admission work
and the T027 structure gate, without suppression or fake consumers. Retained opaque
episode data must remain faithful for T015 inspection. Existing byte-preservation
and magic-number/raw-value regressions remain mandatory.

Correct one unsupported mapping: T029 is a Python preparation script, not a Rust
consumer of `PreparationDescriptor::new` or `SourceDefinition::new`. Their Value-to-
RawValue convenience wrappers are not a reason to postpone clean lint beyond T027.
At the existing model/admission integration task, use the actual checked RawValue
constructor and keep test-only conveniences in tests when needed. Similarly,
`Decoded` duplicates bytes now owned by InputArtifacts. Removal of that obsolete
storage/accessors is authorized when touching the decoding boundary, preserving
all exact-byte/strict-admission assertions. Do not start a separate cleanup project.

Update the charter/execution-contract and affected T014/T017 plan wording so the
intermediate sequencing rule and unchanged mandatory T027/T035 clean gates agree.
Record the durable decision in session notes; keep actionable resolution ownership
and next actions in Fizzy. Specifications and numerical requirements are unchanged.

## Required bounded completion now

Resume the sole developer on T014 only. Correct the four ordinary Clippy findings
in escalation027, and any further ordinary current-code style findings exposed
after them, using small behavior-preserving fixes. No scoring formula changes.

Complete the seven concrete process-evidence gaps already enumerated in
escalation027: validate actual stdout against the three machine schemas; missing
input exit3; existing output exit3 with destination preservation/no partial success;
invalid input exit2; unknown flags/missing values/duplicate or unsupported settings;
exact submitted snapshot and evidence bytes; and a secret-bearing failed operation
whose stdout/stderr do not leak the sentinel. Use the real built executable and
valid bases with isolated errors. Fix any actual requirement violation these cases
expose. Existing success, hash, help/version and numerical cases remain.

Finish every named T007-T014 filter and the unchanged fmt/Clippy/all-tests/release
commands. The response must distinguish passing behavior/build checks from the
actual residual Clippy failure. Record exact cases, results, hashes and residual
symbols, with no unsupported completion claim.

Once this bounded evidence is complete and only the expressly staged dead-code
diagnostics remain, freeze the candidate and proceed to the one already-authorized
independent T007-T014 review. This decision adds no review and does not reset the
combined repair allowance. Verifier findings must cite actual current requirements,
reproductions and consequences; speculative hardening or the declared staging
limitation alone does not justify a new repair loop.

Restore active tracking, index/link instructions and responses, and preserve
caffeinate PID84732. Return any actual new contract conflict to the owner promptly.
