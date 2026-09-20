# T029 independent preparation verdict

Verdict: **Revise**.

The canonical data, native mappings, observations, selections, byte
reproducibility, permissions, and installed-binary shape checks pass. One
required preparation-provenance binding is incomplete: the source evidence does
not directly preserve either route's raw native response artifacts, and the
Score preparation descriptor therefore does not index the raw artifacts from
which its derived outcomes were produced.

## Frozen identity and review boundary

The reviewed identities match the complete developer handoff:

| Artifact | SHA-256 |
|---|---|
| `scripts/acceptance/prepare_chord.py` | `17e14e786a6ec2ec9d6d653396932fe877f8b38f7deb9d97e9a646fa3ba11956` |
| `tests/test_prepare_chord.py` | `5c1ee68c62217f16b8fef830134266cbb2eb6da36cf0c00f267ad70968ff97ec` |
| accepted `source-manifest.json` | `860300bc288ad1f46c98631f78ad353066d7798a54c182ac5b60d8a1b693444e` |
| accepted `source-id-map.json` | `a987df05727164c382f0ea404bd4f465aa9dbaf7fb3f6e8c74d5bedac5cd32b4` |
| complete developer response005 | `c3f5c2e3bc28f9aecf0e69aac09127b9826731a7b4b33c4a8d93aec2f3806da9` |
| installed T027 executable | `4bacbc5947bb70218725916965439837b2afa53d16a04f37ef00a31509a85dde` |

I read no T030 dispatch, oracle implementation, expected result, or performance
result. I did not evaluate performance, run a Rust suite, edit the candidate or
protected inputs, or broaden the review beyond T029.

## F1 — raw native artifacts are absent from the evidence bindings

`source_definition` constructs the same seven-entry evidence shape for each
route at `scripts/acceptance/prepare_chord.py:246-254`. Those entries are the
copied preparation script, accepted source manifest, accepted ID map, route
manifest, requests, references, and preparation receipt. Neither list contains a
raw native response file. The Score descriptor at lines 328-332 indexes only
entries `[0, 1, 2, 6]`, which resolve exactly to:

```text
preparation-script.py
../chord630/source-manifest.json
../chord630/source-id-map.json
preparation-receipt.json
```

An independent set intersection against the accepted manifest reproduced:

```text
choice evidence_paths=7 raw_paths=630 direct_raw_bindings=0
score  evidence_paths=7 raw_paths=630 direct_raw_bindings=0
```

The accepted source manifest inventories and hashes the raw files, but it does
not contain or preserve their bytes. This does not satisfy the ratified
prediction contract at `docs/specs/validator-v1.md:206-213`, which requires the
files referenced by `preparation.evidence_indices` together to preserve the
deterministic transformation, raw source artifacts, and receipt. Owner prompt001
is more explicit at lines 56-60: the Score indices must actually bind the script,
raw native source artifacts, and receipt, and source evidence may reference the
accepted protected raw files without copying the whole bundle into the prepared
output directory.

The installed executable admits the current files because it correctly checks
the declared paths, indices, shapes, and hashes; the specification explicitly
does not ask Validator to certify whether provenance claims are substantively
complete. Passing `check` therefore does not close this preparation-author
obligation.

The smallest correction is confined to evidence construction and dependent
canonical regeneration:

1. Deterministically append each route's 630 accepted
   `native_route_raw_response` frozen paths to that route's source `evidence`
   array, referencing the protected files through their existing sibling paths.
   No raw file needs to be copied into the preparation output.
2. Add every Score raw-response evidence index to the Score
   `preparation.evidence_indices` alongside the script and receipt bindings.
   Retaining the manifest and persisted map bindings is appropriate.
3. Regenerate the affected full predictions, route subsets, and receipt; rerun
   the same two-directory byte comparison, exact hash reconciliation,
   permissions checks, and eight installed-binary `check` invocations.

No mapping, observation, selection, golden-data, configuration, or Validator
change is indicated.

## Passing reproduced criteria

### Population and raw routes

- The persisted map has 630 unique source IDs and UUIDs: 603 labeled and 27
  withheld/unscored. The exclusions independently split into 23 `WITHHELD` and
  4 `UNSCORED` records.
- Choice requests, Score requests, both reference snapshots, both raw-response
  collections, the map, and the manifest all have the same 630 source IDs.
- Both routes recover exactly 630 unique native answers from the accepted raw
  base64 envelopes. No historical parsed-projection failure became abstention or
  filtering.
- `golden.json` contains exactly the 603 reviewed records. Every input state is
  byte-canonically equal between the Choice and Score requests, both references
  agree, and all 27 excluded mapped records and their original reason objects are
  present in the receipt.

### Native mappings, sources, and observations

- Every Choice outcome equals the explicit frozen mapping
  `DIFFERENT_CANONICAL -> NO_MATCH`, `UNCERTAIN -> UNCERTAIN`, and
  `SAME_CANONICAL -> MATCH`.
- Every Score outcome equals an independent
  `bisect_right((0.5, 1.5), score)` derivation. Four source rows have the exact
  `0.5` tie and map to `UNCERTAIN`; the synthetic check also proves `1.5` maps to
  `MATCH` even though no reviewed source row has that exact value.
- The scalar route preserves all 130 reviewed cases in which its derived outcome
  differs from the native distribution argmax. No argmax substitution occurs.
- Choice retains its native categorical distribution and reported confidence.
  Score retains its scalar, native categorical distribution, reported
  confidence, and `same_subject`, `same_capability`, and
  `material_difference` Bernoulli observations exactly.
- Both source kinds are `classifier`. Their actual recorded model, request
  questions, route name, mapping, and prompt/configuration paths match their own
  accepted requests and remain distinct between routes.
- No prediction row has top-level scoring `probabilities` or `confidence`. The
  receipt contains exactly four omission disclosures—probabilities and
  confidence for each route—with all 603 affected UUIDs and nonblank reasons.
- The Score descriptor's method, version, and scalar configuration are otherwise
  exact and its current indices are valid and in range. F1 is solely the missing
  raw-artifact binding.

### Golden binding and selections

- The exact whole-golden SHA-256 is
  `9a8896ea9afefd9bef0e3610595d4469d617fadd371529fd344f88437aa0389b`.
  Both full prediction artifacts and all six subsets bind that digest.
- The labeled selections independently reproduce 383 old-random records, 220
  old-supplement records, and the first 50 labeled source IDs in ascending exact
  source-ID order mapped through the unchanged UUID map.
- Each configuration's ID order is exact. Every subset has precisely that order,
  contains the full route rows without mutation, retains the route source
  definition, and remains bound to the whole golden bytes.

### Reproducibility, hashes, and permissions

Two new mode-0700 sibling output directories were prepared independently. The
base conversion and all three selection invocations completed in each. All 15
files were byte-identical across both runs and the delivered bundle, and all
hashes match response005:

| Artifact | SHA-256 |
|---|---|
| `golden.json` | `9a8896ea9afefd9bef0e3610595d4469d617fadd371529fd344f88437aa0389b` |
| `choice.json` | `cd5531e4d5b3c4e302b79e466aabfe9c104a9689f3f328c8f3e2d0e2827ad5f9` |
| `score.json` | `8873a8e594c41431ba1718142ebd0294900e39c55dbcb2468f78cade232459d4` |
| `config-full.json` | `4bf3016eacdc6d6cfe559a4182929d4ddff6ffed08b23a66edc3a54a728ed95b` |
| `config-random400.json` | `faa707f9e91dc2a17befac4ad922f5a989ae4f9b779210972f794e660180719c` |
| `config-supplements230.json` | `52556988eef040f685237ede92230039489fca3eea6fb1549cc4a0f6ad7fe653` |
| `config-targeted.json` | `a5142ec439eb6407c54e5840af6bcbad39b924813c52a25132b2a351bf4f0951` |
| `preparation-script.py` | `17e14e786a6ec2ec9d6d653396932fe877f8b38f7deb9d97e9a646fa3ba11956` |
| `preparation-receipt.json` | `bb90f91995908e20b48f4237dae01c9a1e10e173cf33bd2a34e27da117d00b9e` |
| `choice-random400.json` | `a9cc635b7787a0130fdf7d1a83cfa85049d2a6aea7041b6e7f9fcb243ed15842` |
| `choice-supplements230.json` | `57a29ac116f092250c5f51bb2c394b00f1bab935001f9657424c57f6a92aed7e` |
| `choice-targeted.json` | `5918f6d62091bade85fea9cd87341bfc014a2e87e1f9c819587108ee74c2043d` |
| `score-random400.json` | `21b8b425f99914d4727feacf135cf8cf30c9cc276924c425a3661162a19d1417` |
| `score-supplements230.json` | `cdc3fc7f63004bd0b17ceaa939245602959cba48dc5af137c4738c6c2fea8983` |
| `score-targeted.json` | `f87154986ad147eff2350e798f2d5508b231f25692399eaf0edda2084e1bdba8` |

The accepted manifest and ID-map hashes were identical before and after review.
The delivered and reproduced roots are mode `0700`; every canonical file is mode
`0600`. Temporary proof directories were removed after their context-managed
checks.

### Focused public and installed-binary checks

`python3.14 -m py_compile scripts/acceptance/prepare_chord.py` and
`python3.14 -m unittest tests/test_prepare_chord.py` both exit 0. The two public
tests directly cover the frozen Score ties and explicit Choice mapping without a
private-data dependency.

The accepted installed executable reports `validator 0.1.0` at the frozen hash
above. Eight `check` invocations exit 0: Choice and Score against the full
configuration, plus both routes against random400, supplements230, and targeted
configurations and their exactly aligned subsets.

## Decision

The candidate is not Ready because F1 leaves the preparation descriptor and
source evidence unable to preserve the exact raw native inputs required for
auditable derivation. The correction is evidence-only and bounded; every other
T029 observable criterion reproduced successfully.
