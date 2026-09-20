# T029 focused raw-evidence recheck verdict

Verdict: **Ready**.

The bounded F1 repair is complete. Each route now has exactly 630 direct raw
native-response bindings in the accepted manifest order, every bound path and
SHA-256 matches the protected source entry, and the Score preparation descriptor
indexes all 630 raw inputs together with the existing script, accepted manifest,
accepted ID map, and receipt bindings. The repair regenerates only the ten
evidence-dependent artifacts; the five golden/configuration artifacts remain
byte-identical.

## Review boundary and frozen identities

I read response007 only after reconciling the retained verdict006 identity. I
read no T030 dispatch, oracle implementation, expected result, or performance
result. I did not evaluate performance, run a Rust suite, edit the candidate or
protected bundle, or broaden the review beyond the authorized F1 recheck.

| Artifact | SHA-256 |
|---|---|
| workspace `scripts/acceptance/prepare_chord.py` | `083ad9461c82564a6f0091f31291e8dc9df193f681610f4b3540dd5f67cfa7ed` |
| protected `preparation-script.py` | `083ad9461c82564a6f0091f31291e8dc9df193f681610f4b3540dd5f67cfa7ed` |
| public `tests/test_prepare_chord.py` | `5c1ee68c62217f16b8fef830134266cbb2eb6da36cf0c00f267ad70968ff97ec` |
| retained verdict006 | `daa33712402877c8a0b6ab1102f5bd087fe081ae5d343e49ac3cbbcb9a8eaa13` |
| accepted `source-manifest.json` | `860300bc288ad1f46c98631f78ad353066d7798a54c182ac5b60d8a1b693444e` |
| accepted `source-id-map.json` | `a987df05727164c382f0ea404bd4f465aa9dbaf7fb3f6e8c74d5bedac5cd32b4` |
| accepted release executable | `4bacbc5947bb70218725916965439837b2afa53d16a04f37ef00a31509a85dde` |

## F1 direct raw bindings

For each of `choice` and `score`, I independently filtered the accepted
manifest to entries whose route matches and whose role is
`native_route_raw_response`. Each filter returned exactly 630 entries. In each
delivered full prediction artifact:

- the source evidence array contains 637 entries;
- positions 0 through 6 retain the prior script, manifest, map, route manifest,
  requests, references, and receipt bindings;
- positions 7 through 636 exactly equal the corresponding manifest entries'
  `../chord630/<frozen_path>` values in recorded order;
- all 630 direct paths are unique, resolve to regular protected files, and
  reproduce the SHA-256 stored on their own accepted manifest entries.

The Score `preparation.evidence_indices` value is exactly:

```text
[0, 1, 2, 6, 7, 8, ..., 636]
```

The first four indices therefore retain `preparation-script.py`, the accepted
source manifest, the accepted ID map, and `preparation-receipt.json`; the
remaining 630 indices cover every direct Score raw response in manifest order.
There is no duplicate, omitted, reordered, stale-path, or hash-mismatched raw
binding.

## Protected and reproduced hashes

The five artifacts that must remain unchanged still have their verdict006
hashes:

| Artifact | SHA-256 |
|---|---|
| `golden.json` | `9a8896ea9afefd9bef0e3610595d4469d617fadd371529fd344f88437aa0389b` |
| `config-full.json` | `4bf3016eacdc6d6cfe559a4182929d4ddff6ffed08b23a66edc3a54a728ed95b` |
| `config-random400.json` | `faa707f9e91dc2a17befac4ad922f5a989ae4f9b779210972f794e660180719c` |
| `config-supplements230.json` | `52556988eef040f685237ede92230039489fca3eea6fb1549cc4a0f6ad7fe653` |
| `config-targeted.json` | `a5142ec439eb6407c54e5840af6bcbad39b924813c52a25132b2a351bf4f0951` |

All ten evidence-dependent delivered artifacts have the repaired hashes from
response007:

| Artifact | SHA-256 |
|---|---|
| `choice.json` | `f6683631465bd9f8ea7b0772e75b6699fed8cc161cacc5bd582faa5d32e7f572` |
| `score.json` | `325db741173eec3c33dc5ad038733000ccbbd36df3d19cd4a24b82c92dc15df3` |
| `preparation-script.py` | `083ad9461c82564a6f0091f31291e8dc9df193f681610f4b3540dd5f67cfa7ed` |
| `preparation-receipt.json` | `1b13c4cc86878abca909edcf1efb765cb818363578dbb2a9be09a62f873564b8` |
| `choice-random400.json` | `c8091c703c394617f79fcf30a43f16b56b827e8ac1d55b0e47f3b6009d9d21f4` |
| `choice-supplements230.json` | `54a3c215b808a7edba8f53b14d4754fb4a79061e5df05444ea522cf3ed4e2e36` |
| `choice-targeted.json` | `38e1ec36e757fccb92562af78f80610a753912c5120a4508b828698cf62c0a62` |
| `score-random400.json` | `b56d7059e13e67dcb428654e9a361de32dbab15bfe73afaea14bc30e1d862bb4` |
| `score-supplements230.json` | `40b0ac8f2e062eadd785966f823addf306d43da53a57bd58182499316ef32af1` |
| `score-targeted.json` | `312822f59919c9f4d8f02df2f93e5eae041a1c2794ea7381f3bd23c72b65ba47` |

I ran the base preparation and all three selection invocations independently in
two fresh sibling directories. All 15 canonical files were byte-identical across
both runs and the delivered protected bundle. Both fresh roots were mode `0700`,
every generated canonical file was mode `0600`, the delivered root remains mode
`0700`, and every delivered canonical file remains mode `0600`. The accepted
manifest and ID-map hashes were identical before and after the reproduction.

## Focused checks

`python3.14 -m py_compile scripts/acceptance/prepare_chord.py` and
`python3.14 -m unittest tests/test_prepare_chord.py` both exited 0; the two public
tests passed.

The exact accepted release executable above returned exit 0 and status
`complete` for all eight required paths:

1. Choice full
2. Choice random400
3. Choice supplements230
4. Choice targeted
5. Score full
6. Score random400
7. Score supplements230
8. Score targeted

Each subset check used its aligned generated configuration and the unchanged
whole `golden.json`.

## Decision

F1 is closed. The source evidence now directly preserves both routes' complete
raw native inputs, and the Score descriptor binds every raw input used by its
documented deterministic transformation. All other passing T029 conclusions
from verdict006 remain unchanged and are reused as authorized. No further local
correction is indicated within this recheck boundary.
