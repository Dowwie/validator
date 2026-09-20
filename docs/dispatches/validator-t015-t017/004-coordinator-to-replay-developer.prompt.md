# Validator T015 exact replay-comparison correction 004

Continue your active T015 assignment and required response002. Read owner prompt
`003-owner-to-coordinator.prompt.md` and acknowledgment
`003-coordinator-to-owner.response.md`. This correction is binding before handoff.

The current `src/app.rs::same_report` behavior that converts all JSON numbers to
`f64` and applies fixture tolerance recursively violates T015. Replace it with the
smallest explicit comparison boundary:

- structure, array/object membership and ordering, strings, booleans, nulls,
  statuses, counts, IDs, paths, ordinals, hashes, raw/submitted values, policy
  constants, and source/preparation/opaque configuration are exact;
- a count changed from an integer to a nearby fractional number must fail, even if
  the numeric difference is within tolerance;
- recorded numeric configuration must remain exact and must not be converted to
  an approximately equal number;
- only actual finite computed result fields may use the existing absolute/relative
  tolerances. Define this boundary explicitly by typed field or exact report path,
  not by the fact that a JSON node happens to be numeric;
- preserve arbitrary-precision/opaque number spelling and literal
  `$serde_json::private::Number` objects. Do not make unrestricted Value conversion
  authoritative for opaque configuration equivalence or coerce that object to a
  number.

Fold four discriminating cases into the existing
`replay_binding_and_result_tampering` filter:

1. legitimate unchanged recomputation succeeds;
2. a nearby fractional tamper to an integer count fails;
3. a nearby numeric tamper inside recorded source/preparation configuration fails;
4. a within-tolerance perturbation of one actual finite computed metric succeeds.

Use valid bases and unchanged prescribed tolerances. Do not enumerate every number,
create a generic comparison engine/configuration format, relax any other equality,
add fallback behavior, or alter the rest of T015. Run the named filter and disclose
the exact comparison boundary/cases in the required response002.
