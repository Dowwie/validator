# T028 independent source-bundle verdict

Verdict: **Ready**.

The protected Chord630 source bundle satisfies the frozen T028 contract. I found
no criterion failure or unresolved blocker. This verdict covers the source-only
freeze; it does not accept the bundle on the owner's behalf and does not authorize
T029 or T030.

## Frozen identity

- Protected root:
  `/Users/dowwie/.local/share/validator/acceptance/chord630/`.
- `source-manifest.json`: SHA-256
  `860300bc288ad1f46c98631f78ad353066d7798a54c182ac5b60d8a1b693444e`,
  720,587 bytes.
- `source-id-map.json`: SHA-256
  `a987df05727164c382f0ea404bd4f465aa9dbaf7fb3f6e8c74d5bedac5cd32b4`,
  432,460 bytes.
- Developer checkpoint response: SHA-256
  `b5388afd357e04383c487bc11d1e2c92c0e9a56b55e035decbf6a105b8945fa3`.
- Both protected metadata documents retain
  `candidate_not_yet_independently_reviewed_or_owner_accepted`; no acceptance
  state was changed during review.

All three hashes match the frozen review dispatch exactly.

## Independent reconciliation

I used a fresh Python 3.14.6 stdlib probe outside the repository. It read JSON and
decoded saved response envelopes as data; it did not import or execute any copied
source, instruction, or prompt.

### Population, reasons, and cohorts

Fresh enumeration found the same 630 unique source IDs in each Score and Choice
saved manifest, request collection, reference collection, raw-result route, the
protected manifest's 630-case cohort inventory, and the ID map. The Score and
Choice reference snapshots are exactly equal.

The independently counted reference states are:

| State | Count |
|---|---:|
| `REVIEWED` | 603 |
| `WITHHELD` | 23 |
| `UNSCORED` | 4 |
| Total | 630 |

Thus `603 + 23 + 4 = 630`, and the 27 non-labeled IDs are exactly the manifest's
27 withheld/unscored IDs. For every one of those 27 records, both saved
alternate and baseline state/reason fields match the original reference
provenance exactly; every record retains at least one original non-null reason.
The ID map retains those same reason pairs and dispositions.

Reference-derived cohort membership is exactly 400 `old_random400` and 230
`old_supplements230`. For all 630 IDs, the manifest cohort row and ID-map row
match the original cohort, component, reference disposition, and label-presence
state. These links are unique and unambiguous.

### Exact files and source stability

The manifest has 1,353 unique source paths and 1,353 unique relative frozen paths.
For every entry I independently checked:

- the route belongs to its recorded source root;
- the source and frozen objects are regular, non-symlink files;
- source size, recorded size, and frozen size are equal;
- source SHA-256, recorded SHA-256, and frozen SHA-256 are equal;
- the relative frozen path is non-escaping and resolves inside the protected root.

All 1,353 current external source hashes were captured before the population,
route, UUID, and mapping checks and recomputed afterward. The complete pre/post
maps are equal, proving that the external source files remained unchanged during
the review. The manifest and ID-map bytes also remained unchanged.

The copied inventory totals 1,353 files and 275,887,658 bytes. The manifest's 23
route/role count-and-byte cells all equal a fresh aggregation. Route totals are:

| Route | Files | Bytes | Reproduced ordered path/hash/size digest |
|---|---:|---:|---|
| Score | 666 | 136,769,418 | `77f36ebdeab7e6750291aeb839015dc7beba62362632f92ff20a2f73c0afd5d4` |
| Choice | 662 | 135,138,364 | `04df635d94e4727cfb152166be2e1e38add3c8073d3d868c7b8ea7ad1c70a953` |
| Choice input/configuration | 25 | 3,979,876 | `1db20e8b07be659ba2734cc4cfa3288c5bd674bf7e208189c5707b93763eb1a4` |

The protected tree contains exactly those 1,353 frozen files plus the two
metadata files: 1,355 files and 277,040,705 bytes, with no unrecorded file. Its
inventory uses only the eight documented evidence roles: requests, references,
raw native responses, route/projection evidence, prompts/configuration, manifests
and runtime provenance, indispensable provenance, and saved source snapshots.
There are no copied r04 attempt results, unrelated experiment-tree files, or
paths outside the three declared source roots. This establishes the T028-T030
necessity boundary as far as the saved role/path inventory can establish it.

### Native-route recovery

Each external attempt directory independently contains exactly 630 result
records, and those exact records are the 630 frozen raw-response entries for its
route. Every record has a unique bound source ID from the common 630-ID set and
retains the exact request associated with that ID.

I base64-decoded every raw response as JSON and checked the native identity answer
without reinterpreting or scoring it. All 630 Choice responses contain a native
`choice` in the saved three-label vocabulary. All 630 Score responses contain a
finite numeric native `score`. Therefore both complete native routes remain
recoverable.

Fresh status counts also reproduce the historical projection limitation:

| Route | Raw native records | Parsed valid | Historical malformed |
|---|---:|---:|---:|
| Choice | 630 | 627 | 3 |
| Score | 630 | 347 | 283 |

The malformed historical projections were retained as limitations; I did not
repair, relabel, or score them.

### Persistent identities

`source-id-map.json` contains exactly the common 630 source IDs and 630 distinct,
RFC-conforming version-7 UUIDs. Each row matches the reference-derived cohort,
component, disposition, reason pair, and current-label-presence state. The saved
method declares Python 3.14.6 stdlib `uuid.uuid7()`, 630 newly assigned IDs, and
zero preserved source UUIDs, consistent with the saved absence evidence.

The map hash and bytes were unchanged between the beginning and end of review,
and the manifest records the required resumption contract: this first persisted
map must be reused unchanged on reconciliation. I generated no UUIDs.

### Scalar mapping

The frozen mapping source exactly matches its current saved source at SHA-256
`5caf27be243b2f745395397b469e8d75d16f6da10d7d812dc78564327e527258`.
Static AST inspection, without executing that source, establishes:

- labels `NO_MATCH`, `UNCERTAIN`, `MATCH` in that order;
- boundaries `0.5` and `1.5`;
- `route_score` indexes the labels with
  `bisect_right(SCORE_BOUNDARIES, score)`.

The saved Score manifest and protected mapping record both identify Python
3.14.6. A direct stdlib `bisect_right` probe under the provisioned Python 3.14.6
runtime confirms the boundary ties: `0.5 -> UNCERTAIN` and `1.5 -> MATCH`.

### Permissions, ownership, and containment

The protected tree has 10 directories, all mode `0700`, and 1,355 files, all mode
`0600`. Every directory and file is owned by UID 501. A fresh walk found zero
symlinks. Every manifest frozen path resolves below the protected root, and the
actual bundle file set equals the declared frozen set plus the two metadata files.
No frozen path points back to a moving experiment tree. The manifest neither
records itself as a source entry nor contains its own SHA-256 literal, so there is
no manifest self-hash.

## Commands and exits

| Command/probe | Exit | Result |
|---|---:|---|
| `shasum -a 256` on the checkpoint, manifest, and map; `stat -f` on protected metadata | 0 | All dispatched identities, sizes, modes, and UID match. |
| `python3.14 /tmp/t028_verify.py` | 0 | Independent 1,353-file source/copy verification, all set joins, reasons/cohorts, raw recovery, UUIDs, mapping, permissions, containment, and pre/post stability passed. |
| Independent NUL-delimited ordered source-path/hash/size digest probe | 0 | All three recorded route digests reproduced exactly. |
| Independent manifest `copy_summary` aggregation | 0 | 23 cells checked; zero mismatches. |
| `ruby docs/plans/validator/verify-plan.rb` | 0 | 35 tasks, 433 source blocks, 63 conformance cases, 12 structure checks, 8 ACs, 5 DoD clauses, local links, and artifact index passed. |
| Public `docs/artifact-index.md` relative-link resolution | 0 | 432 links checked, 431 unique targets, zero missing. |
| `git diff --check` | 0 | No output. |

The repository currently presents its existing project tree as untracked, so the
last command has no tracked diff to inspect; the required command nevertheless
passes, and the two public structure/link checks independently cover the public
handoff boundary. No Rust build was required or run.

## Decision

The protected identity, populations, original non-label reasons, cohorts, both
raw native routes, UUID persistence, saved scalar mapping, all source/frozen
hashes, permissions, ownership, containment, candidate status, and bounded source
selection satisfy T028. The only retained limitation is the documented historical
parsed-projection shortfall, for which all 630 exact native records remain
recoverable on both routes. The candidate is **Ready** for coordinator handoff and
owner acceptance.
