# T029 F1 raw-evidence repair handoff

Status: complete focused repair candidate; return to the same verifier for the
authorized recheck.

## Scope and isolation

I read verifier verdict006 at its required SHA-256
`daa33712402877c8a0b6ab1102f5bd087fe081ae5d343e49ac3cbbcb9a8eaa13`.
This repair addresses only F1: direct raw native evidence bindings and the
dependent Score preparation indices. I did not read or access any T030/oracle
path, evaluate performance, alter mappings/observations/outcomes/selections,
change golden/configuration artifacts, add tests/tooling, or start T031.

## Change

`scripts/acceptance/prepare_chord.py` now derives each route's direct raw
evidence list from the accepted source manifest in its recorded entry order. It
requires exactly 630 `native_route_raw_response` entries for each route; each
entry must retain its role, unique frozen path, extant regular file, and recorded
SHA-256 before its sibling evidence path is emitted.

Each Choice and Score source has 637 evidence entries: the prior seven entries,
followed by its 630 direct frozen raw-response paths. The Score preparation
descriptor is exactly:

```text
[0, 1, 2, 6, 7, 8, ..., 636]
```

It therefore preserves the copied script, accepted manifest/map, receipt, and
every direct Score raw native response. The 630 raw entries begin at index 7 and
end at index 636; all resolve to the accepted protected files and reproduce their
manifest SHA-256 values.

## Regenerated artifact identities

Only the ten evidence-dependent artifacts changed. The five omitted artifacts
remain byte-identical to response005: `golden.json` and the four configuration
files.

| Artifact | Previous SHA-256 | Repaired SHA-256 |
|---|---|---|
| `choice.json` | `cd5531e4d5b3c4e302b79e466aabfe9c104a9689f3f328c8f3e2d0e2827ad5f9` | `f6683631465bd9f8ea7b0772e75b6699fed8cc161cacc5bd582faa5d32e7f572` |
| `score.json` | `8873a8e594c41431ba1718142ebd0294900e39c55dbcb2468f78cade232459d4` | `325db741173eec3c33dc5ad038733000ccbbd36df3d19cd4a24b82c92dc15df3` |
| `preparation-script.py` | `17e14e786a6ec2ec9d6d653396932fe877f8b38f7deb9d97e9a646fa3ba11956` | `083ad9461c82564a6f0091f31291e8dc9df193f681610f4b3540dd5f67cfa7ed` |
| `preparation-receipt.json` | `bb90f91995908e20b48f4237dae01c9a1e10e173cf33bd2a34e27da117d00b9e` | `1b13c4cc86878abca909edcf1efb765cb818363578dbb2a9be09a62f873564b8` |
| `choice-random400.json` | `a9cc635b7787a0130fdf7d1a83cfa85049d2a6aea7041b6e7f9fcb243ed15842` | `c8091c703c394617f79fcf30a43f16b56b827e8ac1d55b0e47f3b6009d9d21f4` |
| `choice-supplements230.json` | `57a29ac116f092250c5f51bb2c394b00f1bab935001f9657424c57f6a92aed7e` | `54a3c215b808a7edba8f53b14d4754fb4a79061e5df05444ea522cf3ed4e2e36` |
| `choice-targeted.json` | `5918f6d62091bade85fea9cd87341bfc014a2e87e1f9c819587108ee74c2043d` | `38e1ec36e757fccb92562af78f80610a753912c5120a4508b828698cf62c0a62` |
| `score-random400.json` | `21b8b425f99914d4727feacf135cf8cf30c9cc276924c425a3661162a19d1417` | `b56d7059e13e67dcb428654e9a361de32dbab15bfe73afaea14bc30e1d862bb4` |
| `score-supplements230.json` | `cdc3fc7f63004bd0b17ceaa939245602959cba48dc5af137c4738c6c2fea8983` | `40b0ac8f2e062eadd785966f823addf306d43da53a57bd58182499316ef32af1` |
| `score-targeted.json` | `f87154986ad147eff2350e798f2d5508b231f25692399eaf0edda2084e1bdba8` | `312822f59919c9f4d8f02df2f93e5eae041a1c2794ea7381f3bd23c72b65ba47` |

The workspace script and copied script are both
`083ad9461c82564a6f0091f31291e8dc9df193f681610f4b3540dd5f67cfa7ed`.
The unchanged public regression test remains
`5c1ee68c62217f16b8fef830134266cbb2eb6da36cf0c00f267ad70968ff97ec`.

## Focused verification

All commands below exited 0.

```text
python3.14 -m py_compile scripts/acceptance/prepare_chord.py
python3.14 -m unittest tests/test_prepare_chord.py

python3.14 scripts/acceptance/prepare_chord.py \
  --bundle /Users/dowwie/.local/share/validator/acceptance/chord630 \
  --out /Users/dowwie/.local/share/validator/acceptance/chord630-t029-repair-a.iwg53F
... repair-A --selection config-random400.json
... repair-A --selection config-supplements230.json
... repair-A --selection config-targeted.json
... identical four invocations for chord630-t029-repair-b.GRniPk

target/debug/validator check ... Choice full/random400/supplements230/targeted
target/debug/validator check ... Score full/random400/supplements230/targeted
target/debug/validator check ... final protected Score full
```

The two fresh mode-0700 repair outputs matched on all 15 canonical artifacts.
The pre-replacement reconciliation proved the five golden/configuration files
unchanged, all ten listed evidence-dependent artifacts changed, every repaired
candidate file mode `0600`, and both accepted source metadata hashes unchanged.
The exact ten verified proof-A files replaced their corresponding delivered
protected artifacts; a post-copy comparison with proof-B matched every one.

The direct-binding assertion independently checked both source evidence arrays
against the manifest route/role filter, all 630 resolved paths per route, and
every file SHA-256. It also checked the Score indices exactly span each direct
raw entry. All eight installed-binary `check` invocations returned a complete
result, including the six aligned subset artifacts. The final-root Score full
admission also returned complete with its repaired direct evidence bindings.

The source manifest remains
`860300bc288ad1f46c98631f78ad353066d7798a54c182ac5b60d8a1b693444e`;
the ID map remains
`a987df05727164c382f0ea404bd4f465aa9dbaf7fb3f6e8c74d5bedac5cd32b4`.
The protected root remains mode `0700`; every repaired file is mode `0600`.

## Handoff

No remaining local repair issue is known. This response is a focused F1
candidate for the same verifier's recheck; it is not owner acceptance and does
not assert T030 or performance results. The coordinator retains Fizzy,
artifact-index, and shared-documentation ownership.
