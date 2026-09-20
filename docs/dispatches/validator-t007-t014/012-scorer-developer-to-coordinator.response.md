# T010 schema-developer completion handoff

T010 is complete as a local implementation milestone. It is not a verifier
verdict or acceptance.

Published Draft 2020-12 schemas use
`https://json-schema.org/draft/2020-12/schema` in all three files. This dialect
supports local `$defs`/`oneOf` closed tagged variants without external references;
all references are local `#/$defs/...` references.

| Artifact | SHA-256 |
|---|---|
| `Cargo.toml` | `e257b5d70c78f0dd7465093aad8d092db99c35ffa28227298271512faf49bec6` |
| `Cargo.lock` | `86caa4cd0470cc4d1f8d4d8dedbdfccb1c43e42b9f0a5a119343e35b256dd74e` |
| `schemas/v2/golden.schema.json` | `e485f124035f881e5d488f7d398e6253925cb9669a3d5a7529d7a362097cca24` |
| `schemas/v2/predictions.schema.json` | `d7091cac68d91ba35e1f2352201087c7ded6f8a2d549db1ca9b141a9a552bd35` |
| `schemas/v2/config.schema.json` | `27e9660efaad6944238acb0688bda56410ddd5afd2b88b92dfaf87ac3492ce7c` |
| `tests/conformance.rs` | `c0b1a3720d402b96f76a9591a6d92ae5eca1f7732a5f2c75241a8d7afc7163d6` |

`jsonschema 0.37.4` is the sole new dev dependency; `cargo generate-lockfile`
fetched and locked its resolved graph. No production dependency was added.

Coverage is in `input_schema_contract`: legal single-label and multi-label golden
documents, legal prediction/config documents, and rejected wrong version, missing
opaque input, malformed digest, unknown outcome tag, incomplete reject policy,
and extra policy field. The filter directly invokes the schema engine, as the
owner clarification requires; it does not use the single-label runtime.

The schemas deliberately leave these runtime boundaries unclaimed: label
membership, exact probability/threshold vocabulary keys, categorical sum and
finite-number semantics, task-to-artifact compatibility, source existence/kind,
digest equality, duplicate JSON keys, alignment/ID uniqueness across records, and
preparation evidence index bounds. The test includes a shape-valid abstention
prediction whose source reference remains a runtime check.

Final checks: `cargo fmt --all -- --check` exit 0; `cargo check --locked` exit
0; `cargo test --locked --test conformance input_schema_contract -- --nocapture`
exit 0 with 1 passing test; `cargo test --locked --lib` exit 0; `git diff --check`
exit 0. The first locked conformance attempt exited 101 because the new lockfile
did not yet exist; `cargo generate-lockfile` then succeeded with network access.
Current `cargo check` reports 161 existing incomplete-consumer dead-code warnings;
none were suppressed. `shasum` emitted only its host locale fallback warning.

The repository remains untracked. Per the dispatch, I did not modify the project
index or Fizzy; the coordinator must reconcile the new docs response/index duty.
