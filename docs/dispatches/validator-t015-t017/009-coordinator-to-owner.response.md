# T016 concrete comparison-contract correction acknowledgment

I read the full owner prompt009 and have not admitted T016 or started T017.

The saved response008 candidate does not yet satisfy the ratified concrete data
model. Although it adds typed outer comparison records, `src/comparison.rs` still
accepts generic `Value` reports, recursively discovers metrics by string paths,
stores each family as a vector of string-named metrics, and reconstructs typed
status/population meaning from JSON strings. Required single-label metric fields
therefore remain naming conventions rather than Rust and schema fields.

I am preserving the useful verified-run, source-difference, transition, receipt,
schema and test work while routing one bounded T016 correction to the current
fresh Terra-high sole writer. The correction will:

- feed the pure comparison boundary recomputed typed single-label results and the
  minimum typed verified-run metadata from replay;
- replace dynamic metric discovery with explicit hard-result and categorical
  paired fields, retaining an ordered concrete per-label collection;
- reuse `MetricResult` status, population and checked-arithmetic contracts instead
  of inferring them from JSON strings;
- use checked accumulation for transition/category accounting;
- update the strict schema and the same three named tests to prove actual concrete
  fields, null-delta status cases and the compatibility boundary;
- preserve identical-population single-label scope, exact publication, staged
  warnings and the combined T015-T017 review allowance.

The current response008 handoff SHA-256 is
`836ca1e0c866f898ba168ada621f37b4c875432fd4febc27af959cb1f199d19a`.
It remains an implementation record, not an accepted local milestone. I will read
and reconcile the corrected saved handoff before advancing automatically to the
already-authorized fresh-context T017 assignment.

