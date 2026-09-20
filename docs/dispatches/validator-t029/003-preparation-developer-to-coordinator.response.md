# T029 preparation developer checkpoint

Status: paused at the coordinator-directed allowed-source orientation boundary;
T029 is not complete.

## Scope and isolation

I read the T029 governing assignment, the coordinator's hold instruction, the
relevant canonical prediction, observation, preparation, practical-case, and
dataset-preparation contracts, the published golden/prediction/config/receipt
schemas, and the accepted T028 source-freeze record. I did not read, list,
search, import, or otherwise access any T030/oracle material, expected result,
or prior blocked response. I did not inspect `scripts/acceptance/` broadly.

## Allowed-source orientation evidence

Read-only inspection of the accepted protected metadata at
`/Users/dowwie/.local/share/validator/acceptance/chord630/` reproduced:

| Artifact | SHA-256 | Observed summary |
|---|---|---|
| `source-manifest.json` | `860300bc288ad1f46c98631f78ad353066d7798a54c182ac5b60d8a1b693444e` | 1,353 source entries; 630 cohort memberships; 27 withheld/unscored records; documented Choice and Score recovery and scalar mapping. |
| `source-id-map.json` | `a987df05727164c382f0ea404bd4f465aa9dbaf7fb3f6e8c74d5bedac5cd32b4` | 630 persisted case records and UUIDv7 assignment metadata. |

The frozen evidence establishes 603 labeled records, 27 withheld/unscored
records, the same 630 source IDs across collections, and the documented Score
boundaries/ties. The contracts require distinct Choice and Score classifier
artifacts, each bound to identical whole-gold bytes, with observation-only
numeric evidence and disclosed omitted scoring probabilities/confidence.

## Files and commands

No T029 implementation file existed before this checkpoint, and none was
created. In particular, `scripts/acceptance/prepare_chord.py` remains absent.
No protected output was created or published, no preparation run was started,
and no test or performance evaluation was run.

Successful read-only commands:

```text
sed -n ... 003-coordinator-to-preparation-developer.prompt.md                 exit 0
sed -n ... 004-coordinator-to-preparation-developer.prompt.md                 exit 0
sed -n ... relevant specification and schema files                            exit 0
python3.14 read-only manifest/map shape and SHA-256 inspection                 exit 0
rg -n ... T029 plan boundary                                                    exit 0
git status --short scripts/acceptance/prepare_chord.py tests                  exit 0
```

The only new file is this coordinator checkpoint. Per the T029 scope allocation,
the coordinator retains ownership of shared documentation, the artifact index,
and Fizzy; this response does not modify them.

## Resume condition

Await coordinator routing after the T030 scoped verdict. Resume only if directed;
the first implementation action remains a small isolated native-input parsing
unit in `scripts/acceptance/prepare_chord.py`, followed by no protected
publication until the full T029 contract permits it.
