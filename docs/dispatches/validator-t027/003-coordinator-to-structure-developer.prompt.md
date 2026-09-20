# Proceed after T027 structural ownership synchronization

Continue the same sole-developer output from prompt002. This is the numbered
routing synchronization requested before edits; it does not expand scope or add
another terminal handoff. Required complete response remains
`002-structure-developer-to-coordinator.response.md`.

The pre-edit report is within owner001's structural boundary. Proceed with the
six reported writer files only:

- `src/model.rs`: real checked TaskDefinition/wrapper admission and closed enum
  destructuring; remove obsolete `is_single_label` rather than inventing a fake
  consumer.
- `src/model/common.rs`: Episode-owned checked rows plus removal/test-localization
  of convenience-only accessors without hiding any required production type.
- `src/model/single_label.rs`: Episode-owned rows and direct production
  signal-availability flags; remove the obsolete wrapper.
- `src/model/multi_label.rs`: Episode-owned rows and actual checked task wrapper
  integration.
- `src/validation.rs`: construct concrete checked task definitions and exactly one
  opaque-input-owning Episode per golden row before alignment.
- `src/validation/wire.rs`: concrete private schema-version decoding and explicit
  version-2 enforcement at each decode boundary.

T027 now owns those exact artifacts. Downstream T032/T033/T034 inputs have been
synchronized. Current governance identities are:

| Artifact | SHA-256 |
|---|---|
| `docs/plans/validator/tasks/T027.json` | `63714134b0095d82f2960902d5f72c0d8b3099da65eac2c91f490355696cd56b` |
| `docs/plans/validator/physical-map.json` | `b541760e3dbf298ddffedfa1e1204f1e31221dd304cf7e34f8335a5d6b4a4ed1` |
| `docs/plans/validator/tasks/T032.json` | `cf727905b27c10a1c596e2d48d87295160c424f2a933e7cfe269d954205b2834` |
| `docs/plans/validator/tasks/T033.json` | `7ef7c374e8e5d3d7de5d403071b8caed8fd21def7df75e2d1fc092a2c6e26c7b` |
| `docs/plans/validator/tasks/T034.json` | `3fce16c54662c86c4e98f0423a67b872e04d7e646c6f15ee948829e8ef9ae93d` |

`verify-plan.rb` and `git diff --check` pass after routing.

Do not write app/evaluation/comparison/artifacts/lib, public tests, schemas,
fixtures or Cargo unless a concrete compile/contract failure proves the reported
six-file plan insufficient and you first return exact evidence. Do not put a
required production type behind `cfg(test)`. Prefer direct unit assertions over
new test-only accessors; if a test-only accessor is truly unavoidable, explain
why it is not a disguised production warning suppression.

Compile after the first coherent task/Episode/wire integration, preserve exact
opaque JSON bytes and all accepted behavior, then complete every output1 gate
from prompt002 with warning-denied Clippy exit 0. Save the full response002 before
returning.
