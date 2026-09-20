# T027 repaired public-build handoff for owner acceptance

The final T027 candidate is frozen at repaired manifest011 SHA-256
`178fccf388382f34aa68c9de5cae669c7ab331e5e5612775a28a1a6c6a3639f9`.
The same independent verifier returned **Ready** in response012 SHA-256
`dbc8eb3a4b7793e57c5fe75fbe4efa1dc3722e4562f5e1fc3377eca683e4123e`.

The base candidate integrated real TaskDefinition/wrapper admission, one
opaque-input-owning Episode per checked row, concrete wire-version enforcement and
public inspection disclosure only after verified replay. Exact nonzero DM01-DM12
and `offline_cli_contract` filters supply required behavior; direct source/
signature/caller review supplies the static type/shared-path evidence.

The sole review finding is repaired: task model report builders accept only
model-owned ArtifactSnapshot references, app extracts them from InputArtifacts,
and model has no I/O/artifacts dependency. Focused report/privacy, replay/tamper,
DM01 and offline CLI checks passed in the Ready recheck.

Final evidence on the exact repaired identities:

- warning-denied Clippy: clean;
- ordinary locked suite: 44 library, 12 CLI, 103 conformance, 0 doc;
- release binary build: pass;
- exact network-denied Cargo-offline suite: the same 44/12/103/0;
- format, diff and plan checks: pass;
- all 111 planned through-T027 public CLI/conformance IDs listed, none missing.

No staged diagnostic remains. T028-T035 are not claimed complete. Card190 awaits
this owner acceptance. T030's source-isolated oracle is separately in progress;
T029 still waits for both this acceptance and a complete T030 author handoff.
