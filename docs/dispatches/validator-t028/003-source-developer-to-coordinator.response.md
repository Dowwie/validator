# T028 protected source-freeze checkpoint

Status: protected candidate bundle published and independently reconciled; paused
before independent T028 review because T026 repair reclaims the sole writer. This
checkpoint neither accepts the bundle nor authorizes T029.

## Protected bundle

- Root: `/Users/dowwie/.local/share/validator/acceptance/chord630/`
- `source-manifest.json`: SHA-256
  `860300bc288ad1f46c98631f78ad353066d7798a54c182ac5b60d8a1b693444e`,
  720,587 bytes, mode `0600`.
- `source-id-map.json`: SHA-256
  `a987df05727164c382f0ea404bd4f465aa9dbaf7fb3f6e8c74d5bedac5cd32b4`,
  432,460 bytes, mode `0600`.
- The root and every descendant directory are mode `0700`; every file is mode
  `0600`. All 1,355 files and directories are owned by the current user (UID
  501). The independent scan found zero symlinks and every frozen path resolves
  below the protected root.

The manifest records 1,353 copied source files totaling 275,887,658 bytes;
including the two generated private metadata files, the bundle is 277,040,705
bytes across 1,355 files. It records source paths, frozen relative paths, roles,
byte sizes, and SHA-256 values for every copied source. It intentionally has no
self-hash.

| Source route | Copied files | Bytes | Ordered source path/hash/size digest |
|---|---:|---:|---|
| Restored Score `v2-typesafe-score630-20260918-r02` | 666 | 136,769,418 | `77f36ebdeab7e6750291aeb839015dc7beba62362632f92ff20a2f73c0afd5d4` |
| Native Choice `v2-typesafe-full630-20260917` | 662 | 135,138,364 | `04df635d94e4727cfb152166be2e1e38add3c8073d3d868c7b8ea7ad1c70a953` |
| Choice instruction/configuration source `v2-typesafe-adjudicator-20260917-r04` | 25 | 3,979,876 | `1db20e8b07be659ba2734cc4cfa3288c5bd674bf7e208189c5707b93763eb1a4` |

The copied material is limited to saved manifests and provenance, full requests,
actual prompts/configuration, references and review/exclusion evidence, saved
source snapshots, selected raw native response records, and the route-recovery
evidence needed by T028–T030. It does not copy the Chord repository or execute
the copied code or instructions.

## Reconciled population and native routes

Fresh enumeration from the external saved Score and Choice manifests, requests,
references, and raw result-record directories found the same 630 unique source
IDs in every collection. References reconcile to 603 `REVIEWED`, 23 `WITHHELD`,
and 4 `UNSCORED`: 603 labeled plus 27 withheld/unscored equals 630. The manifest
contains all 27 source IDs with both saved alternate and baseline reason fields,
reference dispositions, cohort and component. It contains explicit membership
for all 630 cases: 400 `old_random400` and 230 `old_supplements230`.

Both native routes are recoverable from exactly 630 raw result records with a
raw native identity answer in each record. The historical strict parsed
projection had 627 Choice outputs and 347 Score outputs, but its failures were
not treated as missing native outcomes: all 630 Choice responses retain their
native `choice` answer and all 630 Score responses retain their numeric `score`
answer. No metric was calculated.

The frozen scalar route comes from the saved Score source snapshot
`sources/score/source-snapshot/typesafe_adjudicator.py`, with its manifest runtime
evidence (Python 3.14.6; saved dependencies). It uses labels `NO_MATCH`,
`UNCERTAIN`, `MATCH`, boundaries `0.5` and `1.5`, and
`bisect_right(SCORE_BOUNDARIES, score)`: scores below `0.5` route to `NO_MATCH`,
`0.5` to `UNCERTAIN`, `1.5` to `MATCH`, and no probability-argmax substitution is
made.

## Identity persistence and resumption

No valid episode UUID appeared in the saved Score/Choice manifests, requests,
references, or analyses. Python 3.14.6 standard-library `uuid.uuid7()` assigned
630 UUIDv7 values once. The map has exactly 630 unique source IDs and 630 unique
version-7 UUIDs, preserving disposition and cohort/component links. A second
read-only reconciliation read the existing map unchanged, generated no IDs, and
reported no copy drift.

## Commands and verification evidence

| Command/probe | Exit | Evidence |
|---|---:|---|
| `python3.14 - <<'PY'` standard-library source inventory probe | 0 | Both routes: 630 manifest IDs, 630 requests, 630 references, 630 raw result files; 603/23/4 reference-state reconciliation. |
| `python3.14 - <<'PY'` standard-library exact-copy/hash/staged-publish probe | 1 then 0 | First staging-only validation detected intermediate directories created with mode `0755`; all staging directories were corrected to `0700`, rechecked, then atomically renamed. No destination existed before publish and no inconsistent file was overwritten. |
| `python3.14 - <<'PY'` independent read-only reconciliation | 0 | Re-enumerated external source IDs/routes, decoded raw response envelopes without executing copied source, validated every source/frozen SHA-256 and byte size, map/disposition/cohort equality, UUIDv7 versions/uniqueness, permissions, ownership, containment, and no symlinks. |
| `shasum -a 256 source-manifest.json source-id-map.json` | 0 | Reproduced the two metadata hashes above. |
| `find ... -type f | wc -l`; `find ... -type l | wc -l`; `stat -f ...` | 0 | 1,355 files, 0 symlinks, root `0700`, metadata files `0600`, owner UID 501. |

For all 1,353 copied source files, the exact SHA-256 and byte size were captured
before copying, matched their frozen counterpart during copying, and matched a
post-copy rehash of the external Chord source. The later independent reconciliation
again compared every current external source file with its frozen hash/size; all
1,353 matched. The frozen paths are private relative paths, never paths back to
the experiment roots.

## Remaining work and limits

The private candidate is ready for the later independent, read-only T028
inventory/hash/permission/native-recovery review, then owner acceptance. T029
and T030 remain unstarted. The preserved limitation is historical strict parsed
projection failure (3 Choice and 283 Score), retained as evidence rather than
silently salvaged; exact raw response records recover both native routes. No
native outcome, disposition reason, or cohort is missing from the freeze.
