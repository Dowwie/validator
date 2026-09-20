# T024 independent verification verdict

## Verdict: Revise

The frozen candidate is reviewable and every manifest identity matches, but the
published comparison, report, and receipt schemas still certify documents that
violate required non-opaque field shapes. This is one T024 schema-completeness
finding with several independently reproduced manifestations. The fourteen
assigned conformance rows, both umbrella filters, the real-binary stdout matrix,
and the scoped test/lint boundary otherwise satisfy this review.

## Frozen-candidate reconciliation

Manifest005 itself hashes to
`502f5f7e4458b11ebdb46ec0a271b3ffe8eb76dd329e2ba7c123a5ccfbbce82c`.
Every inherited/current identity in that manifest matched before review:

| Artifact | Reconciled SHA-256 |
|---|---|
| `001-owner-to-coordinator.prompt.md` | `bfe635ffb06f5ebdeda58610dec181896cb6fd4432726d0355fd583039d87269` |
| `001-coordinator-to-owner.response.md` | `740ca222c8b432b3df70edffd9efd13041ad5b5e683e5a209ad6e6bc5287aae8` |
| `002-schema-developer-to-coordinator.response.md` | `3f00d279951624aca41b3b26a78a686faae90d8938d5fe73249a21ddca91207b` |
| `004-case-developer-to-coordinator.response.md` | `ea7cd976a1a9f798f4a0500d73ac9393cc3a06058c04fb233ed1699f0d8bf145` |
| `../validator-t021-t023/013-coordinator-repaired-manifest.md` | `2cd39b7c27d9b0408ff0903441f3afa975616edf3644f3e502f560d6d26f0bc7` |
| `schemas/v2/check.schema.json` | `1bfd2fb3a644df1203b043244b38a9ed80d404cae5068901b7f162ea6dd662b9` |
| `schemas/v2/comparison.schema.json` | `d0dd16bef3902ac1597bb6538204ffe7c13442c655b4779f9953f6fd5200c8e9` |
| `schemas/v2/config.schema.json` | `c740c7cd2c1381398b3121ee161b1d995a23ceb9dd4325bd73cd0073f66d65c3` |
| `schemas/v2/error.schema.json` | `a1be234949560a90dff9391a0fa9c42de4f0431c5c4e2f20d745730f40eb80d5` |
| `schemas/v2/golden.schema.json` | `97d811a192274f1ee0703e5dff475723301fdcc68c20b031d3b22431968b777b` |
| `schemas/v2/inspection.schema.json` | `039e6060b88042c908c27a34374592d9f64434aa1aac4725da13146b01c0ac82` |
| `schemas/v2/predictions.schema.json` | `654908068da5d6b3cf9c666cddf4ebc888f94203a04fb6addd5164fbbb893352` |
| `schemas/v2/receipt.schema.json` | `d820751593f82ac68cbca2ed381b305383d96d388dee417abcefec03ed3d1c00` |
| `schemas/v2/report.schema.json` | `5500e80464cd87b196f291634c00c073113220747f03898c66406f5106e94ea9` |
| `tests/conformance.rs` | `4b44ea67d2f902e139790a9be339539abd7ee3a90f64052df7092c37b62cc9b7` |
| `tests/cli.rs` | `8c7b9c00d777f4a290911dd77199d8e1b9e085cc14fb0dd78a801108c3a10fcf` |
| `Cargo.toml` | `a75773897bbd009c91c6bc2f04353ec30180a1022e1fc4d486880f030bb02135` |
| `Cargo.lock` | `4a38530c42e420225ae5f28555d7c8c91014f2a6e365f3e4f995109fce223db0` |

The frozen schemas/tests and accepted production baseline remained byte-identical.
The independent mutation probe lived in an external temporary copy.

## Finding: non-opaque result fields remain permissive

**Requirement.** T024 requires all nine schemas to be complete normative machine
contracts, each concrete alternative to prohibit incompatible fields, counts and
metric population fields to be typed, result paths to identify the exact result
file, and no permissive escape outside expressly opaque input/configuration/value
positions. Machine-interface blocks V174-V195, comparison lines 939-968,
AC5/S29, and data-model blocks D058-D060 require typed task-specific result
families and explicit metric populations. Source and preparation configuration
are opaque **objects**: their member values are arbitrary JSON, but the
configuration value itself is not an arbitrary scalar, array, or null.

**Locations and demonstrated admissions.** Against a real emitted comparison and
report, pinned `jsonschema` 0.37.4 accepted all of these mutations:

- `schemas/v2/comparison.schema.json:15` accepts arbitrary contents in
  `baseline/candidate.sources` and `source_counts`, including a blank-key source
  map with null/integer values and a string count.
- `schemas/v2/comparison.schema.json:22-24` accepts a string hard-result count,
  `classes: [null]`, an invented `macro_f1` object, a boolean probability count,
  and `final_outcome_transitions: [null]`. The same untyped pattern continues in
  multi-label macro/episode members at lines 26 and 31.
- `schemas/v2/comparison.schema.json:13` and
  `schemas/v2/report.schema.json:49` accept `population_scope: "raw_answered"`
  on a scalar metric. D059 restricts `raw_answered` to bin-population scope;
  scalar metrics use `selected` or `answered`.
- `schemas/v2/report.schema.json:37` accepts a source
  `configuration: null`; line 42 makes the same arbitrary-value allowance for a
  preparation configuration. Both contracts require an object while preserving
  arbitrary JSON member values. Line 36 also accepts an absolute artifact path
  where the report manifest requires a stored relative path.
- `schemas/v2/receipt.schema.json:15,23` accepts an evaluation receipt whose
  `result_path` is `/tmp/not-report.json`. V175 requires the evaluation branch to
  identify `report.json` and the comparison branch to identify
  `comparison.json`.

The external probe first validated the unmodified emitted documents, applied one
mutation at a time, and asserted that the frozen schema still reported each
mutant valid. It exited 0 with `1 passed`; therefore this is an admission defect,
not blanket schema rejection or an implementation-shaped hypothetical.

**Consequence.** An agent cannot rely on a schema-valid comparison/report/receipt
for typed counts, task-specific result records, metric population scope, opaque
object boundaries, stored artifact paths, or exact result-file selection. That
fails T024's complete-contract outcome and AC5/S29 even though current production
documents happen to validate.

**Smallest correction.** Keep production Rust unchanged. In the three affected
schemas:

1. Replace the comparison's bare `true`, bare object, and untyped array positions
   with closed definitions matching the existing serialized comparison structs:
   constrained source records and nonnegative source counts; typed single/multi
   counts, class/macro records, transition rows, and multi-label episode sets.
2. Restrict scalar metric scope to `selected` or `answered`; retain
   `raw_answered` only in the bin-population definitions that use it.
3. Make report source/preparation `configuration` an object with unconstrained
   member values, preserve the genuinely arbitrary golden `input`, and constrain
   stored manifest paths to their required relative forms.
4. Discriminate receipt paths by operation so evaluation ends in `report.json`
   and comparison ends in `comparison.json` while remaining absolute.
5. Add focused mutations for each repaired position to `all_schema_contracts` or
   the existing focused schema tests. Do not duplicate these assertions in the
   CLI matrix.

## Assigned-case review

I read every complete compound function rather than relying on its name or the
developer summary. Each uses public synthetic inputs and the shared failure helper
checks both the exact diagnostic category and absence of a finalized run.

- S13 covers invalid UUID, duplicate episode and prediction IDs, duplicate
  vocabulary labels, unknown class/field, missing input, and partial categorical
  evidence with the required `E_ID`, `E_LABEL`, `E_SCHEMA`, and `E_PROBABILITY`
  boundaries.
- S14 distinguishes missing/extra rows (`E_ALIGNMENT`) from digest failure
  (`E_PROVENANCE`) and publishes no run.
- S17 proves stable raw/final/probability/sorted episode evidence across episode
  and object-key reordering while exact golden bytes differ.
- S21 accepts declared `UNCERTAIN`, rejects missing/null expected shapes and an
  undeclared class, and makes no reference-settlement claim.
- M16 distinguishes a missing row, answered empty set, explicit abstention, and
  incomplete marginals. M21 preserves a legal unreviewed set while rejecting
  count-map, mixed task, partial-reference, per-label-abstention-shaped, and v1
  wire forms without fallback interpretation.
- E05/E06/E09/E10 retain scalar/observation values and preparation evidence,
  separate observation-only from canonical scoring evidence, preserve
  probability applicability, and make no invented scalar or task conversion.
- E11 preserves the established provenance boundary for unknown observation
  binding/preparation indices and the observation/schema boundaries for numeric
  and kind failures. E12 proves per-row absence without imputation and rejects an
  incomplete scoring family. E13 requires an explicit outcome for both probability
  and scalar-observation rows.
- S29 validates the real report plus typed receipt/error documents and delegates
  the real one-document stdout matrix to its existing CLI owner. Its positive-only
  schema check does not detect the finding above.

## Commands and gate evidence

To avoid cross-contamination from the external mutation probe, all candidate
commands below used a fresh Cargo target directory outside the repository.

- `all_schema_contracts`, `cli_stdout_schema_matrix`, and every exact S13/S14/S17/
  S21/S29, M16/M21, E05/E06/E09-E13 command exited 0 and executed exactly one
  test. Conformance filters reported 35 filtered out; the CLI filter reported 9.
- `cargo test --locked` exited 0 with exactly 44 unit tests, 36 conformance tests,
  10 CLI tests, and 0 documentation tests.
- `cargo clippy --locked --all-targets -- -D warnings` exited 101 with exactly the
  accepted 17 production dead-code diagnostics. The full-suite lib-test build
  reported the expected three duplicate unread-wire-field diagnostics (two
  elided as duplicates). No T024-owned warning, suppression, fake consumer, or
  visibility widening appeared.
- Developer formatting, locked release-build, and diff-check evidence is reusable;
  this finding does not implicate those unchanged gates.

No T025-or-later criterion, numerical reimplementation, private data, optional
metadata request, cosmetic rewrite, or production change is part of this verdict.
