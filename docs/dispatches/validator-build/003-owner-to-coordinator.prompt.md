# Resolve admission task wiring and type dependencies

Role/model: existing Sol-high coordinator, retain context. Root:
`/Users/dowwie/MyProjects/validator`.
Required response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-build/003-coordinator-to-owner.response.md`.
This changes task/write boundaries only within the authorized T001–T017 outcome.
The owner is updating the plan metadata accordingly; no product behavior changes.

## Decisions and immediate action

The owner read the T003 dispatch and T001 acceptance response. The strict two-file
T003 scope accidentally prevents the required use of an established UUID primitive.
It also makes Prediction<Output> depend on ObservationSet before its owner T004.
These are plan gaps, not developer failures or reasons to write workaround code.

Before more T003 work, issue a new numbered complete developer prompt superseding
its std-only and two-file restrictions. Retain T003's identity/vocabulary/task,
Episode<Target> and Outcome<Target> work and exact tests. Assign Prediction<Output>
to T004 together with its real ObservationSet; do not implement an empty/raw-value
observation placeholder or unplanned T004 behavior in T003. Record T003 acceptance
against the corrected scope, not a missing field that now belongs to T004.

Permit Cargo.toml/Cargo.lock changes for the established UUID crate required by
T003. Prefer the actual standard primitive to a custom UUID parser. Select only
features needed by this task and verify the locked build. Do not add future
runtime/framework features speculatively.

The T002 write area includes src/lib.rs for the module declaration, and manifest/
lock changes for serde_json features actually necessary for exact opaque numeric
values and strict decoding. The same narrow standing routing rule applies to later
tasks: necessary parent module declarations/re-exports and dependency manifests are
part of implementing the named artifacts. They do not permit new behaviors,
public SDK expansion, placeholder runtime uses, or lint suppression. Record every
such file change in the candidate. Updating ordinary wiring is within coordinator
routing authority; semantic/public-contract changes still return to the owner.

## Proportional continuation

Do not stop unrelated ready work or ask the user to reapprove the build. Reuse the
same developer, run local criteria/checks, freeze and independently review the
corrected T003 candidate. The verifier must use the corrected task scope and may
not turn earlier planning inconsistency into an implementation defect.

The owner will update task JSON, physical/dependency input maps, coverage routing,
charter and session notes. You own the numbered worker follow-up, index entries and
Fizzy note. Coordinate if an index write is in flight; the owner is not editing the
index during this correction. Save the required response with the superseding
worker dispatch, active handle and precise implemented scope, then continue.
