# T030 repaired cohort-selector focused recheck

## Verdict: Ready

The repaired candidate fixes the sole required finding from verdict005. The named
populations now use the accepted cohort identities, the labeled partition is
exactly 383 representative plus 220 challenge rows, and the complete repaired
challenge object matches an independent raw-evidence derivation field for field.
Every unaffected conclusion from verdict005 remains valid.

## Repaired identities and scope

| Artifact | Verified SHA-256 |
| --- | --- |
| `scripts/acceptance/oracle_chord.py` | `773272be28d6db7f7c6f2440c66f0a1cb6fc709240adf185911acdfc3cebffcd` |
| protected `expected.json` | `33fad4ba44b782a869cc1f5f6cdc2fb30ae9321588ed49225154b7b67ce1eb03` |
| protected `source-manifest.json` | `860300bc288ad1f46c98631f78ad353066d7798a54c182ac5b60d8a1b693444e` |
| protected `source-id-map.json` | `a987df05727164c382f0ea404bd4f465aa9dbaf7fb3f6e8c74d5bedac5cd32b4` |
| repair response007 | `4398e24f7bc93b1576cf72cfe9fa184a148ba5482817923f8e5afe6a6c407b64` |

The repaired script selects `cohort == "old_random400"` for representative and
`cohort == "old_supplements230"` for challenge; the emitted selection strings
name those same predicates. Replacing only those two predicates and two strings
with their former component-based text reconstructs the original script SHA-256
`43ad79ffa33132f48b3fad739414ab3c44b108641c908220b20da660590e6438`,
confirming the repair is confined to the required four lines.

The protected bundle remains mode `0700`; the repaired expected output remains
mode `0600`.

## Independent 383/220 reconciliation

The accepted references independently yield 383 reviewed `old_random400` rows
and 220 reviewed `old_supplements230` rows. The repaired challenge membership is
ordered by exact source ID before UUID mapping. Its newline-delimited ordered UUID
digest is the required
`a6193871f132ae237b1faf6eba61e5a884a1c781fca258613535c5f0d77fc97e`.

A fresh decoder read only the protected manifest, reference records, map and the
440 relevant native Choice/Score raw responses. Its independently constructed
challenge object was exactly equal to `populations.challenge`, including the
selection string, 220 UUIDs, both full route result objects and the paired object.

Rows are actual `DIFFERENT_CANONICAL`, `UNCERTAIN`, `SAME_CANONICAL`; columns are
those labels followed by `ABSTAINED`.

| Route | Matrix | Supports | Predicted supports | D/E/U | Accuracy | Wrong rate | Macro F1 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Choice | `[[83,11,3,0],[28,41,5,0],[4,0,45,0]]` | `97/74/49` | `115/52/53` | `169/51/0` | `169/220` | `51/220` | `262945/340578` |
| Score | `[[57,38,2,0],[15,58,1,0],[0,17,32,0]]` | `97/74/49` | `72/113/35` | `147/73/0` | `147/220` | `73/220` | `1365010/1990989` |

For both routes, answered is 220, coverage and all class coverage ratios are
`1/1`, abstention rate is `0/1`, selective accuracy equals accuracy, and selective
risk equals wrong rate. All metrics and per-class ratios have status `defined`.

The complete independently matched per-class operands and ratios are:

| Route/class | support/predicted | TP/FN/FP | precision | recall | F1 |
| --- | --- | --- | --- | --- | --- |
| Choice / DIFFERENT | `97/115` | `83/14/32` | `83/115` | `83/97` | `83/106` |
| Choice / UNCERTAIN | `74/52` | `41/33/11` | `41/52` | `41/74` | `41/63` |
| Choice / SAME | `49/53` | `45/4/8` | `45/53` | `45/49` | `15/17` |
| Score / DIFFERENT | `97/72` | `57/40/15` | `19/24` | `57/97` | `114/169` |
| Score / UNCERTAIN | `74/113` | `58/16/55` | `58/113` | `29/37` | `116/187` |
| Score / SAME | `49/35` | `32/17/3` | `32/35` | `32/49` | `16/21` |

The repaired Choice-to-Score matrix is
`[[72,43,0],[0,52,0],[0,18,35]]`, with 61 changed, 17 recovered and 39 regressed
cases. The independently derived ordered UUID-list digests match exactly:

| Set | SHA-256 |
| --- | --- |
| changed | `a12c10ceaac93d1e08d944f23a26681b80241d6ef1431cc3138ed6a8a5e82560` |
| recovered | `072e6c9325d4da08bd8ed4a33ef2ca1f688524020bf6258bd43cca55398317d0` |
| regressed | `fb4470964fcebe5ddfd1762b6e01c12c7c5bc0b8a99517bdfc2e778db44ad6d8` |

## Unchanged object and deterministic runs

The previous frozen expected bytes used by verdict005 still hash to
`d2098ee0ece60f40561b26d3dcf6517a78dffdbb9c156012cb3a4b0a0f43089e`.
After removing the replaced challenge object and normalizing only the required
representative selection-description text, the previous and repaired expected
objects are structurally equal. Thus full, representative, targeted, exclusions,
known failures, method/input fields and every unaffected UUID-bearing list are
unchanged.

Two fresh direct oracle runs exited 0 with empty stdout/stderr. Their bytes were
identical to each other and the protected repaired output, all at SHA-256
`33fad4ba44b782a869cc1f5f6cdc2fb30ae9321588ed49225154b7b67ce1eb03`.
Both task-owned temporary roots remained `0700`; both output files were `0600`.
An existing output parent remained `0750`, proving no parent chmod side effect.
`/tmp` remained root-owned mode `1777`. Manifest and map hashes were unchanged
across the runs. Read-only Ruff passed.

No Validator source/result or T029 preparation implementation/output was read or
used. No broader case, tool or audit was added. The focused repair satisfies the
T030 cohort criterion and no scoped blocker remains.
