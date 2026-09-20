# T030 independent oracle review verdict

## Verdict: Revise

The frozen candidate has one required, reproducible cohort-selection defect. The
accepted labeled cohort partition is 383 `old_random400` rows and 220
`old_supplements230` rows. The candidate instead defines `challenge` as
`component == "challenge"`, which selects only 97 of the 220 reviewed supplement
rows. Consequently every membership-dependent field under
`populations.challenge` is calculated for the wrong population.

All other scoped oracle entries reconciled independently from the protected raw
native answers, accepted references and persisted map. The deterministic/security
checks also passed. This verdict does not use Validator source, Validator results,
T029 preparation implementation or T029 output.

## Frozen candidate identity

All dispatched identities matched before and after review.

| Artifact | SHA-256 |
| --- | --- |
| `scripts/acceptance/oracle_chord.py` | `43ad79ffa33132f48b3fad739414ab3c44b108641c908220b20da660590e6438` |
| protected `expected.json` | `d2098ee0ece60f40561b26d3dcf6517a78dffdbb9c156012cb3a4b0a0f43089e` |
| protected `source-manifest.json` | `860300bc288ad1f46c98631f78ad353066d7798a54c182ac5b60d8a1b693444e` |
| protected `source-id-map.json` | `a987df05727164c382f0ea404bd4f465aa9dbaf7fb3f6e8c74d5bedac5cd32b4` |
| author response004 | `80a9bebc6386b7e537b2e4ae9aa6d6bddbfb5def167464ba27a3089635f1e480` |

The protected bundle directory was mode `0700`; the three protected JSON files
were mode `0600`. The map contains 630 unique UUIDv7 identifiers, and its source
ID, cohort, component and reference-disposition fields exactly match the 630
accepted reference records.

## Required finding: incomplete challenge cohort

Normative basis: T030 requires the representative and challenge cohorts to remain
distinguishable as explicit selections. Follow-up dispatch006 requires the
accepted T028 labeled partition, 383 plus 220, rather than allowing a 97-case
component subset to stand for the challenge cohort.

The accepted raw evidence has this complete reviewed partition:

| Cohort/component | Reviewed rows |
| --- | ---: |
| `old_random400` / `random` | 383 |
| `old_supplements230` / `domain` | 95 |
| `old_supplements230` / `challenge` | 97 |
| `old_supplements230` / `enrichment_paraphrase` | 10 |
| `old_supplements230` / `enrichment_neighborhood` | 18 |
| **`old_supplements230` total** | **220** |

At `scripts/acceptance/oracle_chord.py:390-391`, both named populations are
selected by `component`. The published `challenge.episode_uuids` exactly equals
the 97 `component=challenge` UUIDs and does not equal the 220
`cohort=old_supplements230` UUIDs. The output therefore omits 95 domain, 10
paraphrase and 18 neighborhood reviewed cases from the named challenge cohort.

The complete 220-member challenge UUID sequence, ordered by exact source ID as
required, has SHA-256
`a6193871f132ae237b1faf6eba61e5a884a1c781fca258613535c5f0d77fc97e`
when encoded as each UUID followed by `\n`. This digest records membership and
order without publishing the protected identifier list.

### Correct challenge hard-label entries

Rows are actual `DIFFERENT_CANONICAL`, `UNCERTAIN`, `SAME_CANONICAL`; columns are
those declared labels followed by mandatory `ABSTAINED`.

| Route | Correct 3 x 4 matrix | Supports | Predicted supports | D/E/U |
| --- | --- | --- | --- | --- |
| Choice | `[[83,11,3,0],[28,41,5,0],[4,0,45,0]]` | `97/74/49` | `115/52/53` | `169/51/0` |
| Score | `[[57,38,2,0],[15,58,1,0],[0,17,32,0]]` | `97/74/49` | `72/113/35` | `147/73/0` |

For both routes, answered is 220, coverage and class coverage are `1/1`, and
abstention rate is `0/1`. Accuracy equals selective accuracy; wrong-class rate
equals selective risk.

| Route | Accuracy / selective accuracy | Wrong rate / selective risk | Macro F1 |
| --- | --- | --- | --- |
| Choice | `169/220` | `51/220` | `262945/340578` |
| Score | `147/220` | `73/220` | `1365010/1990989` |

Every per-class operand and ratio was independently derived as follows.

| Route/class | support/predicted | TP/FN/FP | precision | recall | F1 |
| --- | --- | --- | --- | --- | --- |
| Choice / DIFFERENT | `97/115` | `83/14/32` | `83/115` | `83/97` | `83/106` |
| Choice / UNCERTAIN | `74/52` | `41/33/11` | `41/52` | `41/74` | `41/63` |
| Choice / SAME | `49/53` | `45/4/8` | `45/53` | `45/49` | `15/17` |
| Score / DIFFERENT | `97/72` | `57/40/15` | `19/24` | `57/97` | `114/169` |
| Score / UNCERTAIN | `74/113` | `58/16/55` | `58/113` | `29/37` | `116/187` |
| Score / SAME | `49/35` | `32/17/3` | `32/35` | `32/49` | `16/21` |

All ratios above have status `defined`; there are no undefined classes.

### Correct challenge paired transitions

Rows are Choice and columns are Score in declared-label order. The correct matrix
is `[[72,43,0],[0,52,0],[0,18,35]]`, with 61 changed, 17 recovered and 39
regressed cases.

Using the same newline-delimited, order-preserving UUID encoding, the transition
list SHA-256 values are:

| Set | Count | SHA-256 |
| --- | ---: | --- |
| changed | 61 | `a12c10ceaac93d1e08d944f23a26681b80241d6ef1431cc3138ed6a8a5e82560` |
| recovered | 17 | `072e6c9325d4da08bd8ed4a33ef2ca1f688524020bf6258bd43cca55398317d0` |
| regressed | 39 | `fb4470964fcebe5ddfd1762b6e01c12c7c5bc0b8a99517bdfc2e778db44ad6d8` |

The published 97-row challenge values (`70/27` Choice and `69/28` Score, with
25/11/12 changed/recovered/regressed) are internally correct for that component
subset, but they do not satisfy the required 220-row challenge-cohort selection.

## Independent reconciliation of unaffected entries

A separate stdlib decoder hash-checked all 1,261 consumed manifest entries: 630
Choice raw responses, 630 Score raw responses and the accepted reference file. It
joined those records to all 630 persisted map cases, independently obtained 603
reviewed labels and the exact 27 excluded cases, decoded native Choice values, and
mapped native Score scalars through the frozen `bisect_right` boundaries.

The candidate's complete non-challenge object matched the independently derived
object: schema/method/input identities, labels, all 27 exclusion objects, full,
representative and targeted memberships, both route result objects including all
matrices/supports/operands/statuses, all paired UUID lists, and all ten known
failures. After substituting the required challenge cohort and normalizing only
the representative selector wording, every one of the candidate-versus-derived
differences was beneath `populations.challenge`; there were no non-challenge
value differences.

Compact independent checkpoints are:

| Population | Count | Choice matrix | Score matrix | Choice D/E/U | Score D/E/U | changed/recovered/regressed |
| --- | ---: | --- | --- | --- | --- | --- |
| Full | 603 | `[[265,29,23,0],[88,99,10,0],[10,1,78,0]]` | `[[191,115,11,0],[55,140,2,0],[1,29,59,0]]` | `442/161/0` | `390/213/0` | `157/42/94` |
| Representative | 383 | `[[182,18,20,0],[60,58,5,0],[6,1,33,0]]` | `[[134,77,9,0],[40,82,1,0],[1,12,27,0]]` | `273/110/0` | `243/140/0` | `96/25/55` |
| Targeted first 50 | 50 | `[[33,0,1,0],[4,5,0,0],[1,0,6,0]]` | `[[26,8,0,0],[4,5,0,0],[0,2,5,0]]` | `44/6/0` | `36/14/0` | `10/0/8` |

The full, representative and targeted macro-F1 operands respectively matched
`600413/831300`, `44789/66150`, `209/252` for Choice and
`43509547/65515086`, `1237576/1916145`, `11/16` for Score.

The deterministic known-failure rule also matched exactly: first ten labeled
cases with either route wrong after ascending exact source-ID ordering. Its ten
UUIDs have newline-delimited order digest
`7ea66e829921ed2390b1924522404383d6099fb7e7cf426d6d213a5955b11b72`.
The sorted excluded source-ID sequence has digest
`c55e49fcf7c3155bbbb3279a7347cbd2d3391b78f74dde91482514aa06fbb594`.

The native evidence contains six exact `0.5` Score ties, four in the labeled
population; all map to `UNCERTAIN`. It contains no exact `1.5` case, while both
manifest and candidate code freeze `bisect_right`, which would map `1.5` to
`MATCH`. The complete raw routes recover all 630 cases despite the manifest's
three malformed historical Choice projections and 283 malformed historical Score
projections. No argmax or historical parsed projection was used.

## Determinism, permissions and isolation

Two direct `python3.14 scripts/acceptance/oracle_chord.py --bundle ... --out ...`
runs exited 0 with empty stdout/stderr. Their output bytes matched each other and
the published candidate at SHA-256
`d2098ee0ece60f40561b26d3dcf6517a78dffdbb9c156012cb3a4b0a0f43089e`.
Both task-owned temporary roots remained mode `0700`; both outputs were mode
`0600`. An existing output parent remained `0750` before and after, confirming no
parent chmod side effect. `/tmp` resolves to root-owned `/private/tmp`; it remained
mode `1777` before and after.

The script, manifest and ID-map hashes were unchanged across both runs. In-memory
AST parsing and read-only Ruff both passed. Static inspection found no Validator,
`src/`, T029 preparation or production-result dependency. The output contains no
raw-response, request, prompt, input-context, distribution, confidence or episode-
payload keys. Neither its own digest nor the script digest occurs in the output,
so there is no self-hash cycle.

## Smallest required correction and focused recheck

Change only the two population selectors to the accepted cohort identities:

```python
"representative": [episode for episode in labeled if episode.cohort == "old_random400"],
"challenge": [episode for episode in labeled if episode.cohort == "old_supplements230"],
```

Update the emitted selection descriptions to name those cohort predicates, then
regenerate the protected `expected.json`. The representative numerical entries
remain unchanged; the challenge membership and all nested challenge Choice,
Score and paired-transition entries must become the 220-row values above. No
other source, schema, metric, failure-selection, permission or architecture change
is justified.

A focused recheck should bind the repaired script/output hashes, confirm the
383+220 partition, reconcile the corrected challenge object and transition-list
digests, and repeat the two deterministic permission-safe runs. The current
candidate is not Ready until that correction and recheck complete.
