# Repair the demonstrated stored-inspection regression

Existing Sol-high coordinator, repository `/Users/dowwie/MyProjects/validator`.
Save acknowledgment to
`docs/dispatches/validator-t018-t020/015-coordinator-to-owner.response.md`.

Owner012 remains governing. During its check/inspection stage, the writer
reproduced `relocated_run_inspection` returning E_IO before schema validation.
Owner inspection of `src/app.rs` confirms `rebuild_report` invokes
`load_evidence_bindings` on stored predictions in both task branches, while the
accepted single-label comparison replay already uses
`stored.verify_evidence(evaluation.sources())`. This is an existing-contract
regression introduced in T020's generalized inspection, not a schema defect or
an optional robustness feature. It also leaves the inspection path without the
required stored-evidence verification.

Extend the active schema developer's sole-writer scope by a new numbered dispatch
to make the smallest `src/app.rs` repair and relevant existing CLI/conformance
regressions. Keep the current context; do not restart orientation. Before Rust
changes read the rust-best-practices skill and the owning functions plus the
existing StoredRun evidence-verification interface.

For both task kinds, inspection must verify contained snapshot/evidence bindings
using the existing stored-run verifier and rebuild from those verified stored
bindings. Original source/evidence paths remain provenance only. Reuse the existing
verification interface; do not duplicate I/O, add a fallback path, remove evidence,
weaken equality or change errors/tolerances. Do not change artifact formats,
comparison semantics, dependencies, policy or public exports.

Preserve the failing reproduction and demonstrate the corrected single-label
relocated inspection with original inputs/evidence unavailable. Add bound evidence
to the existing real multi-label relocated-inspection case and demonstrate the
same behavior plus rejection after stored evidence is tampered with. Use the
existing precise error contract. Reuse relevant existing tamper/binding assertions
rather than creating a new audit suite. Serialize Cargo checks; these tests share
build resources.

This scoped repair permits stage014 to finish its original real-command evidence.
Require a complete response recording the root cause, exact changed paths/hashes,
failed/passed commands, and unchanged report schema/other production files. The
developer may incorporate this repair in its final014 handoff or save a new
response associated with the follow-up, preserving all earlier files. Do not end
the active assignment merely to report the now-authorized scope exception.

After this stage is complete, proceed to owner012's already-authorized fresh
numerical/CLI evidence stage and its full gates. No new product scope or allowance
reset is granted. If another independent scope gap or incomplete return occurs,
record the actual cause and return to the owner. The combined T018-T020 review is
still unopened. Index/link this prompt and acknowledgment, update Card184 with the
specific repair while Working On, and preserve caffeinate.
