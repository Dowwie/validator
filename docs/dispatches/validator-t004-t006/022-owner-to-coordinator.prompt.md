# Owner reassessment: close the omitted-selection constructor case

Role/model: existing Sol-high coordinator, retained context.
Required response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t004-t006/022-coordinator-to-owner.response.md`.

The owner read the complete replacement handoff, focused verifier020 verdict,
and escalation021. The numeric repair is established. The sole remaining finding
is required by the existing checked population contract, not optional redesign.

Cause: constructor consistency validation covers Some(requested IDs) but omits
the None branch, although None means select all. The changed next action is one
explicit invariant guard plus its direct owning-module negative test, not another
general model rewrite. Its small cost is justified because downstream code relies
on this checked type's selection semantics.

Authorize the existing replacement developer to modify only
`src/model/common.rs` and its owning tests: reject omitted requested IDs with a
nonempty unselected partition, using the existing typed partition error. Preserve
valid omitted/all, empty dataset, explicit-empty and explicit-subset behavior.
No other implementation file or numeric/parser behavior needs to change. If an
actual dependency requires a wider edit, return the smallest concrete reason first.

Run the population_alignment filter (including the new direct construction case),
validate_before_selection, dataset_digest_binding, format, full locked tests,
locked build and Clippy. Record the actual nonzero counts and exits. Reuse the
unchanged numeric/T004/T005 evidence and existing dead-code-only limitation;
do not repeat their separate per-filter command batches. Save a concise complete
developer response, reconcile/freeze the changed file, then obtain one focused
independent recheck of this condition, its positive/negative cases, and unchanged
source identity. No broader rereview or new requirement is authorized.

This is one explicitly owner-authorized narrow completion attempt after
reassessment; it does not reset the coordinator's exhausted repair allowance.
A failed recheck or incomplete return comes directly back to the owner. After a
Ready response, send the exact saved result/hash to the owner for acceptance and
the already-defined T007–T014 next sequence. Keep all source writes paused until
that acceptance, and preserve task-owned caffeinate PID84732/session29011.

Record this decision in the existing Fizzy card and index prompt/response/recheck
artifacts. Manifest019 remains the source baseline. Its exact lib.rs hash is the
authority for that unchanged file; the missing character in the developer014
response's lib.rs hash is a clerical typo, not a changed source or new code defect.
Do not rewrite frozen historical evidence solely to repair that typo.
