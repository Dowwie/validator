# T030 independent Chord630 hard-label oracle

Status: complete candidate for independent review. This is not owner acceptance
and does not start T029.

## Delivered artifacts and identities

| Artifact | SHA-256 |
| --- | --- |
| `scripts/acceptance/oracle_chord.py` | `43ad79ffa33132f48b3fad739414ab3c44b108641c908220b20da660590e6438` |
| `/Users/dowwie/.local/share/validator/acceptance/chord630/expected.json` | `d2098ee0ece60f40561b26d3dcf6517a78dffdbb9c156012cb3a4b0a0f43089e` |
| `source-manifest.json` before and after | `860300bc288ad1f46c98631f78ad353066d7798a54c182ac5b60d8a1b693444e` |
| `source-id-map.json` before and after | `a987df05727164c382f0ea404bd4f465aa9dbaf7fb3f6e8c74d5bedac5cd32b4` |

The protected bundle directory is owned by `dowwie` with mode `0700`; published
`expected.json` is owned by `dowwie` with mode `0600`. The oracle no longer
changes an existing output parent directory. It creates a missing output parent
with private permissions and limits permission changes to the output file.

## Method and output contract

The script verifies the fixed manifest and ID-map digests, validates every input
path stays within the bundle, and hash-checks every raw response and reference it
uses against the manifest. It decodes all 630 Choice and 630 Score base64 native
responses as data; it never imports or executes copied source, Validator, or
preparation code.

Choice uses `answers.identity.choice`. Score uses `answers.identity.score` and
the frozen labels and boundaries: `NO_MATCH`, `UNCERTAIN`, `MATCH`; `0.5` and
`1.5`; `bisect_right`, so the ties map to `UNCERTAIN` and `MATCH` respectively.
It does not substitute distribution argmax.

`expected.json` is schema version 1. It records input identities, mapping and
selection rules, three declared labels plus the mandatory abstention column, all
27 preserved exclusion records/reasons, and four separately named populations:
full 603-label, representative 383-label, challenge 97-label, and the fixed 50
source-ID-order targeted population mapped to UUIDs. Each population contains
Choice and Score `3 x 4` matrices, supports, predicted supports, count identities,
per-class metrics, exact numerator/denominator operands, paired transition
matrix/counts/UUIDs, and no clock-derived field. It also contains deterministically
selected known failures: the first ten labeled cases with either route wrong after
ascending exact source-ID order. This handoff intentionally contains no payload or
ID list.

## Independent results

Rows in each compact matrix are actual `DIFFERENT_CANONICAL`, `UNCERTAIN`,
`SAME_CANONICAL`; columns are declared in that same order. Every abstention column
is zero. These are direct counts, so they also provide each class's exact support,
prediction support, true-positive, false-negative, and false-positive operands.

| Population | Choice matrix | Score matrix | Choice D/E/U | Score D/E/U | changed/recovered/regressed |
| --- | --- | --- | --- | --- | --- |
| Full (603) | `[[265,29,23],[88,99,10],[10,1,78]]` | `[[191,115,11],[55,140,2],[1,29,59]]` | `442/161/0` | `390/213/0` | `157/42/94` |
| Representative (383) | `[[182,18,20],[60,58,5],[6,1,33]]` | `[[134,77,9],[40,82,1],[1,12,27]]` | `273/110/0` | `243/140/0` | `96/25/55` |
| Challenge (97) | `[[26,4,1],[17,23,3],[2,0,21]]` | `[[16,14,1],[8,34,1],[0,4,19]]` | `70/27/0` | `69/28/0` | `25/11/12` |
| Targeted (50) | `[[33,0,1],[4,5,0],[1,0,6]]` | `[[26,8,0],[4,5,0],[0,2,5]]` | `44/6/0` | `36/14/0` | `10/0/8` |

All metrics are defined because every selected population has data and answers.
For every route: abstention rate is `0/1`, coverage is `1/1`, selective accuracy
equals accuracy, and selective risk equals wrong-class rate. The remaining exact
aggregate operands are:

| Population | Route | accuracy / selective accuracy | wrong rate / selective risk | macro F1 |
| --- | --- | --- | --- | --- |
| Full | Choice | `442/603` | `161/603` | `600413/831300` |
| Full | Score | `130/201` | `71/201` | `43509547/65515086` |
| Representative | Choice | `273/383` | `110/383` | `44789/66150` |
| Representative | Score | `243/383` | `140/383` | `1237576/1916145` |
| Challenge | Choice | `70/97` | `27/97` | `11791/15960` |
| Challenge | Score | `69/97` | `28/97` | `4517/6270` |
| Targeted | Choice | `22/25` | `3/25` | `209/252` |
| Targeted | Score | `18/25` | `7/25` | `11/16` |

The separate read-only aggregate derivation decoded the raw frozen answers itself,
without importing the oracle. It recomputed and matched every listed population
matrix, all four transition matrices, and changed/recovered/regressed counts.

The historical route differences are expected: the frozen manifest records all
630 raw native records despite 3 historical Choice and 283 historical Score parsed
projections being malformed. This oracle recovers native records from their saved
raw responses, scores only the 603 reviewed references, reports the 27 exclusions,
and uses frozen scalar routing rather than an argmax. Its full, representative,
challenge, and targeted populations therefore must not be forced to match earlier
all-source, differently parsed, or differently routed projections.

## Evidence

The required interface was run twice into newly created private temporary
directories:

```sh
python3.14 scripts/acceptance/oracle_chord.py \
  --bundle /Users/dowwie/.local/share/validator/acceptance/chord630 \
  --out <private-temp>/expected.json
```

Both commands exited `0`; `cmp` exited `0`; both outputs had SHA-256
`d2098ee0ece60f40561b26d3dcf6517a78dffdbb9c156012cb3a4b0a0f43089e` and
mode `0600`; both temporary directories had mode `0700`. Before and after, `/tmp`
was root-owned with raw mode `41777` (`drwxrwxrwt`, i.e. `1777`), and it was not
changed. The prescribed canonical invocation also exited `0` and produced the
published hash above.

Focused checks passed:

```text
python3.14 -m py_compile scripts/acceptance/oracle_chord.py  # exit 0
/Users/dowwie/.pyenv/versions/3.12.1/bin/ruff check scripts/acceptance/oracle_chord.py  # exit 0
```

No public synthetic fixture was added: this is a tightly bounded private frozen
case and the exercised behavior is fully checked by the two real-data runs and
independent aggregate decoder.

## Criterion mapping and limitations

- The frozen-case source accounting requirement is met by exact 630/603/27 checks,
  persisted map join, exclusion preservation, and source before/after hashes.
- Separate native Choice and frozen scalar-mapped Score hard decisions are met by
  the two raw decoders and required `bisect_right` mapping.
- Hard-decision accounting is met by each matrix's mandatory abstention column,
  supports, identities, exact fractions, and the separately derived match.
- Representative, challenge, known-failure, and fixed targeted-pass requirements
  are met by the persisted deterministic selections; the targeted set is a partial
  demonstration and is not representative or optimized.

The output evaluates only the accepted frozen reference snapshot. Its labels are
not independent truth, it does not establish unseen-data performance, and it does
not certify Validator acceptance. No probability or confidence metric is claimed
because this T030 oracle is limited to required hard labels.
