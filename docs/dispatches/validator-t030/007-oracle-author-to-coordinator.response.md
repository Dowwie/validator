# T030 cohort-selector repair

Status: repaired candidate for the required focused recheck. No other oracle,
T029, or product work was changed.

## Exact change and identities

Only `scripts/acceptance/oracle_chord.py` changed:

- `representative` now selects reviewed `cohort == "old_random400"`.
- `challenge` now selects reviewed `cohort == "old_supplements230"`.
- The emitted selection descriptions name those exact cohort predicates.

| Artifact | Old SHA-256 | New SHA-256 |
| --- | --- | --- |
| Oracle script | `43ad79ffa33132f48b3fad739414ab3c44b108641c908220b20da660590e6438` | `773272be28d6db7f7c6f2440c66f0a1cb6fc709240adf185911acdfc3cebffcd` |
| Protected expected output | `d2098ee0ece60f40561b26d3dcf6517a78dffdbb9c156012cb3a4b0a0f43089e` | `33fad4ba44b782a869cc1f5f6cdc2fb30ae9321588ed49225154b7b67ce1eb03` |
| Source manifest before and after | `860300bc288ad1f46c98631f78ad353066d7798a54c182ac5b60d8a1b693444e` | unchanged |
| Source ID map before and after | `a987df05727164c382f0ea404bd4f465aa9dbaf7fb3f6e8c74d5bedac5cd32b4` | unchanged |

## Corrected cohort evidence

The reviewed partition is exactly `383 + 220 = 603`. The repaired challenge
population has 220 members, ordered by exact source ID before UUID mapping; its
newline-delimited UUID-order digest is
`a6193871f132ae237b1faf6eba61e5a884a1c781fca258613535c5f0d77fc97e`.

Rows are actual `DIFFERENT_CANONICAL`, `UNCERTAIN`, `SAME_CANONICAL`; columns
are those declared labels followed by `ABSTAINED`.

| Route | 220-row matrix | supports | predicted supports | D/E/U | accuracy | wrong rate | macro F1 |
| --- | --- | --- | --- | --- | --- | --- |
| Choice | `[[83,11,3,0],[28,41,5,0],[4,0,45,0]]` | `97/74/49` | `115/52/53` | `169/51/0` | `169/220` | `51/220` | `262945/340578` |
| Score | `[[57,38,2,0],[15,58,1,0],[0,17,32,0]]` | `97/74/49` | `72/113/35` | `147/73/0` | `147/220` | `73/220` | `1365010/1990989` |

For both routes, answered is 220, coverage and all class coverage values are
`1/1`, abstention rate is `0/1`, selective accuracy equals accuracy, and
selective risk equals wrong rate. The corrected Choice-to-Score transition matrix
is `[[72,43,0],[0,52,0],[0,18,35]]`, with 61 changed, 17 recovered and 39
regressed. Its ordered UUID-list digests are respectively:

- changed: `a12c10ceaac93d1e08d944f23a26681b80241d6ef1431cc3138ed6a8a5e82560`
- recovered: `072e6c9325d4da08bd8ed4a33ef2ca1f688524020bf6258bd43cca55398317d0`
- regressed: `fb4470964fcebe5ddfd1762b6e01c12c7c5bc0b8a99517bdfc2e778db44ad6d8`

A direct structural comparison normalized only the required representative
selection wording, removed the replaced challenge object, and proved the old and
new expected objects otherwise identical. This includes full, representative and
targeted results, exclusions, known failures, mappings and all UUID-bearing lists.
No identifier list or private payload appears in this response.

## Reproducibility and permission evidence

Focused checks passed:

```text
python3.14 -m py_compile scripts/acceptance/oracle_chord.py  # exit 0
/Users/dowwie/.pyenv/versions/3.12.1/bin/ruff check scripts/acceptance/oracle_chord.py  # exit 0
```

The required command was run twice into independently created task-owned private
temporary directories:

```sh
python3.14 scripts/acceptance/oracle_chord.py \
  --bundle /Users/dowwie/.local/share/validator/acceptance/chord630 \
  --out <private-temp>/expected.json
```

Both runs exited `0`; `cmp` exited `0`; both output bytes had SHA-256
`33fad4ba44b782a869cc1f5f6cdc2fb30ae9321588ed49225154b7b67ce1eb03`.
Both temporary roots were `0700`; their outputs were `0600`. Before and after,
`/tmp` was root-owned with raw mode `41777` (`drwxrwxrwt`, or `1777`), unchanged.
The canonical protected invocation exited `0`; the existing bundle parent remains
owner-only `0700` and the published output is owner-only `0600`. The manifest and
map hashes above were checked before and after every regeneration.

No blocker remains for the instructed focused recheck.
