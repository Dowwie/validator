# Validator behavior and flow map

Authority: **Ratified**. This map covers protocol phases 1–3. Each behavior has
concrete tasks and physical owners; [physical-map.json](physical-map.json) lists
all exact planned artifacts and writers. [coverage.json](coverage.json) supplies
source-clause and case-level traceability. This is not live task state.

| Behavior | Inputs → observable result | Component / concrete artifacts | Task contracts |
|---|---|---|---|
| Decode strict wire records | UTF-8 bytes → private DTOs or safe parse/schema errors | `validation/wire.rs`, `validation.rs`, `error.rs` | T001, T002 |
| Validate identity and target vocabulary | UUIDs/digests/ordered labels → checked newtypes and targets | `model/common.rs`, `model.rs` | T003, T006, T018 |
| Preserve observations and provenance | Source definitions, observations, preparation indices → checked evidence independent of scoring | `model/common.rs`, `validation.rs` | T004, T011 |
| Admit categorical signals | Complete maps/confidence/profile → original and normalized categorical values, artifact-wide availability | `model/single_label.rs`, `validation.rs` | T005 |
| Admit complete label sets/marginals | Typed sets and complete probability keys → canonical set and unchanged marginals | `model/multi_label.rs`, `validation.rs` | T018 |
| Align a selected population | All validated rows, config, dataset digest → sorted one-to-one aligned rows and selected/unselected IDs | `validation.rs`, concrete evaluation models | T006, T018 |
| Construct honest metric states | Checked counts/finite arithmetic/applicability → typed values, statuses, units, populations and ratios | `evaluation.rs`, `model/common.rs` | T007 |
| Score single-label decisions | Validated rows/legal policy → raw/final matrix, metrics and episode evidence | `evaluation/single_label.rs`, `model/single_label.rs` | T008, T021 |
| Score categorical signals | Working probabilities/confidence and references → losses, argmax, chosen probability, distinct signal bins, ECE | `evaluation/single_label.rs` | T009 |
| Score multi-label decisions | Validated sets/legal policy → raw/final exact-set metrics, answered binary counts and set differences | `evaluation/multi_label.rs`, `model/multi_label.rs` | T019, T021 |
| Score marginals | Complete unnormalized marginals and reference presence → binary losses/means and separate per-label bins | `evaluation/multi_label.rs` | T020 |
| Check without scoring | Paths/options → complete integrity result or structured failure, no output directory | `app.rs`, `cli.rs`, shared admission/I/O | T010–T014 |
| Evaluate and publish | Validated concrete results plus exact source bytes → immutable run and result-file receipt | `app.rs`, `artifacts.rs`, report models/schemas | T011–T014, T020 |
| Verify and inspect | Stored run and selected ID → verified full input and prediction/config evidence | `artifacts.rs`, `app.rs`, inspection schema | T015 |
| Pair compatible runs | Two verified runs → typed metric pairs/deltas, sources, correctness IDs, outcome transitions | `comparison.rs`, concrete comparison models/schema | T016, T022 |
| Restrict paired populations | Explicit intersection → checked restrictions, recomputed metrics, exact exclusions and retained availability | `model/*`, `comparison.rs`, `cli.rs` | T023 |
| Prepare independent acceptance data | Frozen sources and persistent IDs → canonical inputs, exclusion/derivation receipt, independently derived oracle | `scripts/acceptance/*`, protected bundle, public synthetic input/oracle fixtures | T028–T031 |
| Verify usable delivery | Installed binary/docs/schemas and frozen inputs → fresh-agent command/evidence records and full DoD verdict | `README.md`, `docs/usage.md`, indexed dispatches and acceptance record | T017, T024–T027, T032–T035 |

## Complete flows and wiring ownership

1. **Check:** CLI options (T014) → application paths (T014) → exact file and
   evidence reads/digests (T011) → strict DTOs (T002) → shared/task invariants
   (T003–T006/T018/T021) → no-score integrity response (T014). Error propagation
   from each edge preserves T001 categories to the single stdout document.
2. **Evaluate:** the same admission path → concrete pure evaluation
   (T007–T009/T019–T021) → typed report and bound manifest (T013) → complete
   temporary directory and atomic publication (T012) → actual report-file receipt
   (T014). No second validation/scoring implementation exists for CLI formatting.
3. **Inspect:** CLI (T015) → contained snapshot/binding/digest reads (T015) → same
   validation/evaluation → stored-result consistency check (T015) → explicit
   selected input and associated evidence (T015). Original evidence paths never
   participate in this flow.
4. **Compare:** CLI (T016/T023) → verify/recompute each source run (T015) → pure
   compatibility (T016) → optional checked intersection/re-evaluation (T023) →
   concrete paired metrics/transitions (T016/T022) → T012 publication and exact
   comparison-file receipt (T016). Neither source run is mutated.
5. **Practical acceptance:** source inventory and immutable freeze (T028/T031) →
   deterministic native preparation (T029) and independent oracle (T030/T031) →
   tested install/docs (T032) → fresh-agent use of all four flows (T033/T034) →
   criterion-level evidence audit (T035).

These edges have owners; no unassigned adapter, source fetcher, report renderer,
cache, service, database, API server, or deployment platform is required.

## Cross-cutting requirements injected into tasks

| Required concern | Mechanism and owner | Verification |
|---|---|---|
| Privacy and secrets | No payload in routine diagnostics/reports; explicit inspection; protected acceptance/run directories; never credentials in artifacts. T001/T012–T015/T028–T034. | Sentinel leakage/process checks T026; frozen/private paths and output handling reviewed T033–T035. Source configurations remain opaque as required, so preparation must keep credentials out. |
| Strict untrusted input | Duplicate-key detection even inside opaque values; exact tagged fields; checked finite values; no repair/salvage. T002–T006/T018/T021. | Negative input/schema cases T024 plus constructor tests and E-series cases. |
| Integrity and atomicity | Exact digests, deterministic evidence binding, no-replace publish, contained replay, result recomputation. T011/T012/T015. | Race, late failure, tamper, relocation, symlink tests T026. |
| Numerical reproducibility | Binary64 working values, fixed constants/order/tolerances, checked counts, finite/status arithmetic. T001/T005/T007–T009/T018–T021. | Independent oracles/exhaustive cases T025 and practical reconciliation T030/T031. |
| Machine interface | Typed required fields, nine schemas, one stdout document, safe error codes/exits and help/version. T001/T010/T013–T016/T024. | Schema mutation suite and real subprocess checks T024/T027. |
| Evidence lineage and populations | Exact-byte dataset identity, source-local IDs, preparation bindings, selected/answered distinction, explicit intersections. T004/T006/T013/T016/T022/T023. | Source differences, exclusion sets, empty intersections and conditional populations in S/M/E/DM cases. |
| Build/install reliability | Pinned tools/lockfile, module boundaries, locked offline tests, tested host install. T001/T027/T032. | Four required commands, static signature/import review, actual installed-binary acceptance T033/T034. |

No latency/throughput target, distributed availability target, authentication
service, telemetry stack, CI provider, or cloud deployment is in the specification.
Do not invent tasks for them. Borrowing inputs, avoiding duplicate payload storage,
and deterministic iteration are required structure checks, not permission for a
separate optimization project.
