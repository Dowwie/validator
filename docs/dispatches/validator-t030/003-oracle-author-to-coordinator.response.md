# T030 oracle author checkpoint

Status: yielded after the bounded script-construction checkpoint so T027 can use
the sole implementation writer. No expected-result computation, comparison, or
reconciliation has started.

## State written

- `scripts/acceptance/oracle_chord.py`
  (`SHA-256: 5d68e52939282e0c3d7436684b68d3551e6de425f8e045e678cffb4a80903c4d`)
- This checkpoint response.

The new script is the narrowly scoped frozen-case oracle. It verifies the two
accepted input identities, checks contained manifest paths and input hashes,
decodes raw base64 JSON as data, derives Choice from the native answer and Score
with the frozen `bisect_right` boundaries, preserves excluded records, and writes
deterministic owner-only output. It does not import or execute Validator,
preparation, or copied source code.

## Input identity and completed checks

- `source-manifest.json` SHA-256:
  `860300bc288ad1f46c98631f78ad353066d7798a54c182ac5b60d8a1b693444e`
- `source-id-map.json` SHA-256:
  `a987df05727164c382f0ea404bd4f465aa9dbaf7fb3f6e8c74d5bedac5cd32b4`
- `python3.14 -m py_compile scripts/acceptance/oracle_chord.py` exited `0`.

Completed criteria at this checkpoint: the required command interface and frozen
mapping semantics are implemented; output computation has intentionally not run.
No private payload or ID list was emitted here.

## Exact resumption action

After T027 releases the sole writer, run:

```sh
python3.14 scripts/acceptance/oracle_chord.py \
  --bundle /Users/dowwie/.local/share/validator/acceptance/chord630 \
  --out /tmp/oracle-chord-a.json
```

Then inspect any script failure, make only the smallest repair required, and run
the prescribed two temporary-output deterministic comparison before publishing
the protected `expected.json` candidate.

No demonstrated oracle blocker exists at this checkpoint.
