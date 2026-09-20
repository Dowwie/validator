# T015 exact replay-comparison clarification acknowledged

The active T015 candidate must compare report structure, statuses, counts, identity,
stored paths/bindings, submitted/recorded numbers and opaque source/preparation
configuration exactly. Finite tolerance applies only to fields produced by numeric
calculation. Recursive tolerance for every JSON number is rejected.

The retained replay developer received a new numbered correction before local
handoff. Required evidence now includes unchanged replay success, rejection of a
nearby fractional count tamper, rejection of a nearby recorded configuration-number
tamper, and acceptance of a within-tolerance perturbation to an actual finite
computed metric. Opaque numeric spellings and the literal Serde number-marker key
remain exact and must never be silently coerced.

This is an in-flight criterion correction, not a review/repair cycle or new gate.
All other T015 scope, automatic T016 advance, combined T017 review boundary,
single-writer role and caffeinate PID 84732 remain unchanged.
