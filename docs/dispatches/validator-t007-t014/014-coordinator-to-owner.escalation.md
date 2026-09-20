# T010 incomplete corrected-handoff escalation 014

The corrected T010 schemas implement the requested shape fixes, but the
superseding response 013 claims complete regression coverage that is not present
on disk. Under the atomic incomplete-handoff rule, I have frozen T010, paused the
developer, and not dispatched T011.

## Reconciled implementation state

The corrected schema hashes match response 013:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `e257b5d70c78f0dd7465093aad8d092db99c35ffa28227298271512faf49bec6` |
| `Cargo.lock` | `86caa4cd0470cc4d1f8d4d8dedbdfccb1c43e42b9f0a5a119343e35b256dd74e` |
| `schemas/v2/golden.schema.json` | `97d811a192274f1ee0703e5dff475723301fdcc68c20b031d3b22431968b777b` |
| `schemas/v2/predictions.schema.json` | `ef4cf1ccfd80653afc63501fe98900fe897c9eea86e862ae72344e4b2bafab3a` |
| `schemas/v2/config.schema.json` | `c740c7cd2c1381398b3121ee161b1d995a23ceb9dd4325bd73cd0073f66d65c3` |
| `tests/conformance.rs` | `09ef6a87d7c944dca0a94fb566e6837f0b55acffeb0049e68a3b5031d12617ff` |
| response 013 | `7588e0e77ec1cb1f85a1240b2a1aa8ac2195e2d26671b45b733da282d502b888` |

The schemas now use exact `type: "labels"`, enforce golden task/target pairing,
require scored-choice question IDs, split observation bounds, reject whitespace-
only contract strings/map keys, and require nonempty probability/threshold maps.
The direct `input_schema_contract` filter executes one test and passes.

## Missing required evidence

`tests/conformance.rs` remains 74 lines and contains the same small initial matrix.
It does not include the correction-013 regressions for:

- rejecting `label_set` and accepting exact `labels` outcomes/targets;
- rejecting single-label/labels and multi-label/class golden mismatches;
- rejecting scored-choice sources without `question_id`;
- scalar versus bounded observation values and closed observation field variants;
- whitespace-only labels, IDs/keys, models, descriptions, question IDs,
  preparation fields, evidence paths, observation option names, and thresholds;
- empty probability maps and empty threshold maps;
- extra fields in golden and prediction containers;
- legal multi-label prediction and `label_thresholds` configuration documents.

It also does not include the requested shape-valid but semantically invalid case
that proves a named runtime-only boundary. The existing positive prediction uses
a declared source and therefore does not demonstrate its claimed unknown-source
runtime responsibility.

Response 013 says these cases were corrected and that the direct filter covers
them, but no such assertions exist. A single passing filter with missing required
cases does not complete T010.

## Reassessment request

No design, dependency, or tool blocker exists. The smallest correction is to keep
the three corrected schemas and require the same developer to add the exact
missing conformance cases, then rerun only the schema filter, format, library
regression, and diff check. This is a bounded evidence completion; it should not
restart schema design, add another dependency, or advance to T011 before the
saved test matrix exists. Please authorize that one completion attempt or choose
a staffing correction. The combined T007-T014 review boundary and repair allowance
remain unchanged. Task-owned caffeinate PID 84732/session 29011 is still active.
