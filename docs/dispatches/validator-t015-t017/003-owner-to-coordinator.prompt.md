# Preserve exact replay structure while tolerating only computed finite results

Role/model: existing Sol-high coordinator, retained context.
Required full acknowledgment:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t015-t017/003-coordinator-to-owner.response.md`.

During the T015 owner check-in, a focused read of the current `src/app.rs`
`same_report` implementation revealed a mismatch with the already-dispatched task
interface: it converts every JSON Number to f64 and applies fixture tolerance to
all numbers recursively. T015 explicitly requires structural/status/count
differences to be exact, with tolerance only for finite computed results.

Carry this correction into the active T015 brief before accepting its handoff.
An integer count changed to a nearby fractional value must not pass replay, and
opaque source/preparation configuration numbers are recorded data rather than
computed metrics. Keep those values and their JSON structure exact. Preserve the
existing opaque-number and literal serde-number-key boundary rather than making
unrestricted Value conversion the authority for opaque payload/configuration
equivalence. Do not silently coerce a recorded object into a number.

The smallest required evidence is a legitimate unchanged replay, a nearby tamper
to a count that fails, a nearby tamper to recorded numeric configuration that
fails, and the permitted tolerance behavior for an actual finite computed metric.
Fold these into the existing replay result-tamper evidence; this is not an extra
review gate or an invitation to enumerate every numeric field. Use the prescribed
finite-result tolerances unchanged. Do not introduce a generic comparison engine,
configuration format, approximate matching of counts/metadata, or fallback mode.

The implementation remains in flight; no verdict or repair-cycle charge is being
made. Keep the rest of T015's scope and local evidence contract unchanged, finish
it, and proceed to T016 once complete. Index/link this clarification and save the
acknowledgment. The full T015-T017 independent review remains at the same boundary.
