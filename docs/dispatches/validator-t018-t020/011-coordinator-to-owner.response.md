# T020 second reassessment handoff

Status: owner reassessment required under owner prompt009. The authorized fresh
replacement returned response010 incomplete. I have not restarted either writer,
begun independent review or frozen a candidate. Card184 is returning to Waiting
with T020 still its first incomplete step.

## Bound attempt and current state

| Artifact | SHA-256 |
|---|---|
| owner reassessment009 | `f24e5683306ac62874ff00106e6f141166f67420ef58c25a9b63b09bb293b09d` |
| coordinator acknowledgment009 | `11e9f061e76cba6b70c82f706f7356fd91e493f2801f2a8fe3bd5e7413748960` |
| replacement prompt010 | `6b45aa0cd5d375e2ec82f4f83f838f9364ed0b31a02d1361709bc15f3f432769` |
| incomplete response010 | `9902012dfd3f7fabfcb064eecf1ce2fda0e21040c15ce6a8aa738a66a0f63fb6` |
| check schema | `42f2a7d390e53af59159d85254273fc929af25d871b28fa10d56cc415b877362` |
| report schema | `ec1e649325b7e621f16a471989b2a956fdfc1d6fab8a83263e2351b3ae904ecf` |
| inspection schema | `7cf676b2393907a4ce6079260d44935be9123a809efd8ad074e659d53c2c72e0` |
| `tests/conformance.rs` | `a18391bdfbf5a1f9aee9d6ae4a4bf1ee841bd154a76644a33192bd3f3914a07d` |
| `tests/cli.rs` | `c20061b8d01ff67df9bb11d99883caa300147df7fdefd6d907bf425ba1811400` |

Production remains at the response008 hashes. The replacement changed only the
three schemas; no tests, Rust source, dependency, tolerance, public API, T021/T022
behavior or governance artifact was changed.

## Preserved evidence and exact failure

- The rewritten check schema closes task and integrity structures with concrete
  task alternatives. The inspection schema closes root/identity/target/prediction/
  outcome alternatives while retaining opaque input/configuration values.
- The report schema parses and accepts the working multi-label report.
- `multi_label_hard_oracles`, `marginal_loss_and_bins`, and
  `shared_commands_multi_label` each still pass one selected real-path test against
  the rewritten documents.
- `single_report_schema` fails exactly because deleting
  `raw.accuracy.population_unit` remains schema-valid. Thus the new report hard
  branch has not preserved the already accepted required metric contract. This is
  a demonstrated contract regression, not an optional audit finding.
- The replacement made no test changes. The incomplete T019 hard cases,
  equal-count discriminator, marginal endpoint/scaling/bin cases and CLI
  refusal/privacy assertions remain exactly as recorded in response008/owner009.
- Warning-denied Clippy, full locked tests and release build were correctly not
  claimed while the strict schema gate fails.

## Smallest changed approach for owner decision

The remaining assignment still bundles a dense report-schema reconstruction with
four independent discriminating test families. The two fresh writers repeatedly
returned after a partial checkpoint rather than completing that bundle. The
smallest changed approach is to admit an atomic schema-only task first: one fresh
Terra-high/fork-none writer restores the concrete single-label report branch from
the accepted pre-T020 schema, adds the concrete multi-label branch without
weakening it, and finishes only focused positive/negative schema tests. Its local
gate is all schema-focused conformance filters, especially `single_report_schema`,
plus the existing real multi-label positive reports.

After that complete schema artifact is reconciled, a separate fresh context can
complete only the already enumerated hard/marginal/CLI assertions and run the six
filters/full gates. This splits by the demonstrated failure boundary, preserves
single-writer serialization and does not reset the T020 checkpoint allowance,
change technical criteria or broaden review. If the owner selects a different
approach, the same frozen hashes above remain the resume point.

This handoff requests the required owner sequencing/context decision only. The
single combined independent T018-T020 review remains unopened.
