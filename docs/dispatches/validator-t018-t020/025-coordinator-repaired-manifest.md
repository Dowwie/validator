# T018-T020 repaired candidate manifest

Status: frozen repaired candidate for the same independent verifier's focused
recheck. This is the sole owner001 finding-driven repair cycle. It is not owner
acceptance and no second repair cycle is implied.

## Bound review and repair

| Artifact | SHA-256 |
|---|---|
| original manifest022 | `918fd6e13608940232de86f3bfb4b4dfba81c7d5cf618657225cef260e05c177` |
| independent Revise verdict023 | `41ef819eeb12042ebebd2e51f86a52589150b95354fb1d990a575bd12fbf34e8` |
| repair prompt024 | `9bff5430af4ef8a1e92ac362aa056bbc430208422854bd77d62d3bf96ae17a59` |
| repair response024 | `6217ad65a6b7f01ce878e52831f2ecf78fb11a82e84370c08d4cca2bba8bbb48` |

## Changed candidate artifacts

Only these three frozen candidate artifacts changed from manifest022:

| Artifact | Original SHA-256 | Repaired SHA-256 |
|---|---|---|
| `src/validation.rs` | `c6e40115a863bba54fab8f84b32beba53b56df153de008bb3fbc63e6ea2886fa` | `891e6ee4e95b2be1931dd2006d02940308766b8a7b7d486b85f15faafbb69892` |
| `schemas/v2/report.schema.json` | `7215342a2958fd4d9e9735c2837130a02837d205db93b342b9f64d62a882fcf1` | `8f7c4fba2c12381e535888bbde5627a0723471c2018f3ab516a7ce661cb590d6` |
| `tests/conformance.rs` | `f184b28026250ec38da1d228d8bf563272142d21c5844ac2ba4e30e9ab8de291` | `aab6e45f86a34d01421e26195b9f06bbca195aefd92e3d565ce84d2fcb1c3f00` |

Every other governing, handoff, Cargo/toolchain, source, schema, test and oracle
hash listed in manifest022 remains exact. In particular `tests/cli.rs`,
`src/app.rs`, hard/marginal evaluators, check/inspection schemas and the independent
T019 oracle are unchanged.

## Repair contract and gates

The repaired validator rejects every declared non-classifier source during
multi-label source admission, including unused fully valid scored-choice sources,
with the existing `E_CONFIG` diagnostic before alignment. Single-label source
behavior is unchanged.

The repaired report schema applies classifier-only source definitions to the
multi-label task branch while retaining valid scored-choice sources, including
required `question_id`, for single-label reports. Focused tests reproduce both
verdict findings and discriminate both repaired branches.

Repair evidence records:

- affected admission, cross-task, multi-label report, single-label report and real
  CLI filters each select one and pass;
- full locked tests pass: 44 library, 7 CLI, 15 conformance, 0 doc failures;
- formatting, release build and diff checks pass;
- warning-denied Clippy exits 101 only for the same exact 20 owner-authorized
  staged production `dead_code` diagnostics, with no ordinary/new warning.

The same verifier must reconcile this manifest and the three changed hashes,
inspect the repair, reproduce rejection of the unused scored-choice artifact and
the multi-label report mutation, confirm legal single-label scored-choice behavior,
rerun affected/justified regression gates and return `Ready`, `Revise` or
`Blocked`. No new audit, T021/T022 behavior or optional cleanup belongs to recheck.
