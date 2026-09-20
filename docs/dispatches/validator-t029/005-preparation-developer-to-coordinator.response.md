# T029 native preparation handoff

Status: complete local preparation candidate; independent review and owner
acceptance remain pending.

## Scope, authority, and isolation

This completion implements only T029's frozen native preparation. It uses the
accepted T028 protected bundle and persisted identity map; it does not calculate
or compare performance, access expected metrics, inspect an oracle/T030 file, or
start T031. The accepted source hashes remained:

| Protected artifact | SHA-256 |
|---|---|
| `source-manifest.json` | `860300bc288ad1f46c98631f78ad353066d7798a54c182ac5b60d8a1b693444e` |
| `source-id-map.json` | `a987df05727164c382f0ea404bd4f465aa9dbaf7fb3f6e8c74d5bedac5cd32b4` |

The source map partitions the 630 saved source IDs into 603 labeled records and
27 withheld/unscored records. The canonical golden artifact contains only the
603 reviewed records; the private receipt preserves each excluded mapped ID,
cohort/component, disposition, and original baseline/alternate reason evidence.

## Delivery

`scripts/acceptance/prepare_chord.py` is the deterministic, frozen-case
converter. It verifies the accepted metadata digests, requires a private sibling
output directory, refuses to replace an existing canonical artifact, decodes all
630 raw base64 response envelopes for each route, and rejects missing/duplicate
records, mismatched state/reference collections, malformed required native
observations, changed source counts, or an unrecognized label.

The converter emits the complete task artifact set at
`/Users/dowwie/.local/share/validator/acceptance/chord630/`:

- full `golden.json`, `choice.json`, and `score.json`;
- full, random400, supplements230, and first-50-targeted configurations;
- Choice and Score subsets for random400, supplements230, and targeted;
- `preparation-script.py` and `preparation-receipt.json` as bound provenance.

The labelled cohort selections contain 383 `old_random400` records and 220
`old_supplements230` records. The targeted selection is the first 50 labeled
source IDs in ascending exact string order, mapped through the unchanged UUID
map. All subset artifacts preserve the full golden digest and have IDs exactly in
their configuration order.

The common canonical vocabulary is `NO_MATCH`, `UNCERTAIN`, and `MATCH`.
Choice's saved classes are mapped explicitly from its source vocabulary; Score is
routed solely by the accepted `bisect_right([0.5, 1.5], score)` rule, including
the documented ties. Score's categorical distribution is never substituted for
its scalar route. Both artifacts use `classifier` sources that retain the actual
route model, question configuration, and prompt/configuration evidence paths.

No top-level scoring `probabilities` or `confidence` is emitted. Native
distributions and confidence remain typed observations. Score also retains its
original scalar and the three native Bernoulli observations. The receipt has four
complete 603-ID omission disclosures: probabilities and confidence for each
route, with the observation-only reasons. The Score preparation descriptor binds
the copied preparation script, accepted source manifest/map, and receipt.

## Exact final artifact hashes

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

The workspace script hash is the copied script hash above. The public synthetic
tie/mapping regression test is
`tests/test_prepare_chord.py`, SHA-256
`5c1ee68c62217f16b8fef830134266cbb2eb6da36cf0c00f267ad70968ff97ec`.

## Verification evidence

All commands below exited 0.

```text
python3.14 -m py_compile scripts/acceptance/prepare_chord.py
python3.14 -m unittest tests/test_prepare_chord.py

python3.14 scripts/acceptance/prepare_chord.py \
  --bundle /Users/dowwie/.local/share/validator/acceptance/chord630 \
  --out /Users/dowwie/.local/share/validator/acceptance/chord630-t029-proof-a.DvFSqS
... same proof-A invocation with --selection config-random400.json
... same proof-A invocation with --selection config-supplements230.json
... same proof-A invocation with --selection config-targeted.json
... identical four invocations for chord630-t029-proof-b.d8mvOd

python3.14 private alignment/native-route assertion script
cargo build --quiet
target/debug/validator check ... choice.json ... config-full.json
target/debug/validator check ... score.json ... config-full.json
target/debug/validator check ... each Choice/Score subset with its three config files
target/debug/validator check ... final protected score.json with config-full.json
cargo fmt --check
cargo test --quiet
git diff --check
```

The two fresh protected proof outputs and the delivered bundle matched on every
one of the 15 canonical files and receipt hashes in the table. The private
assertions verified all observations, omitted scoring fields, explicit selection
order/counts, full-gold bindings, source configuration differences, native
Choice mapping, Score scalar routing, 27 exclusion entries, output permissions,
and unchanged manifest/map bytes. The accepted executable admitted both full
routes and every required subset; the repository validation completed 44 unit,
12 CLI, and 103 conformance tests without failures. New protected output files
are mode `0600`; their root remains `0700`.

## Limitations and handoff

This is preparation evidence only. It does not establish hard-label metrics,
performance, reference truth, T030's independent oracle results, a fresh
operator acceptance, or T031 work. The documented historical parsed-projection
failures remain represented by raw-envelope recovery; no missing native outcome
was converted to abstention or filtered from the claimed complete routes.

The coordinator owns Fizzy, artifact-index, and shared-documentation updates for
this candidate and its dispatch records. No such shared file was modified here.
