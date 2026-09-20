# Jev proof-of-concept execution record

Authority: **Decision record**. This is a bounded execution result, not an owner
acceptance or production-accuracy claim. The implementation is described in the
[example README](../../examples/jev-poc/README.md); machine results and exact source
hashes are in [outcomes.json](../../examples/jev-poc/outcomes.json).

## Executed

The [GitHub Actions run](https://github.com/Dowwie/validator/actions/runs/35527639676) applied the proposed example changes to
triggering commit `68417f5b1fcf392aa1d654b9e686b6abb3e4610e` and exercised the pinned Rust toolchain.
The compiled worktree's exact source hashes are retained in outcomes.json; the
triggering commit itself contains the transfer boundary, not a claim that all
applied source files were already committed there. The later evidence commit
adds this record and outputs without changing the tested application source.

* 162 Rust tests passed, including the three new CLI help tests.
* 40 Python contract tests passed.
* All 10 synthetic adapter profiles completed native check/evaluate,
  inspection, identical comparison, relocated inspection, and tamper rejection.
* Six recorded jgrep arms and four compatible native paired comparisons completed.
* Sift capture passed Node syntax and dry-run checks. No live model calls were made.
* 846 reference proposals were generated; **zero new human approvals** were created.

Formatting, warning-denied Clippy, and the native build ran before the examples.
Logs and full native run bundles are retained in the workflow artifact; the
published upstream bytes and this numerical outcome summary are also committed
so the offline replay does not depend on artifact retention. The artifact may
expire; rerun the documented command to regenerate complete run bundles.

## Recorded Jev replay, not new inference

The source is `keltokhy/jgrep@fdceb6bdf79165a133667b8e57f3b7244545f0a2`. Gold is the upstream author's
20-case handwritten diagnostic. It is not held out, not a production sample, and
has not acquired new independent human approval here. Diff judgments and current-
function judgments are separate tasks with different reference labels.

| Recorded input mode | TP | FP | FN | TN | Case accuracy |
|---|---:|---:|---:|---:|---:|
| `added_lines` | 6 | 3 | 2 | 9 | 0.750 |
| `diff_hunks` | 8 | 2 | 0 | 10 | 0.900 |
| `diff_lines` | 8 | 6 | 0 | 6 | 0.700 |
| `function_line_context` | 10 | 1 | 0 | 9 | 0.950 |
| `function_lines` | 9 | 3 | 1 | 7 | 0.800 |
| `functions` | 10 | 1 | 0 | 9 | 0.950 |

Case decisions were reconstructed from all published scored rows using the
original any-record-at-or-above-0.5 rule. The published summary ID sets were used
only as an independent cross-check. A record's probability, or their maximum,
was not relabeled as a calibrated case probability. Validator's counts and
accuracy were then checked against separately reconstructed case accounting.

| Comparison (baseline → candidate) | Recovered cases | Regressed cases | Changed decisions |
|---|---:|---:|---:|
| `added_lines-vs-diff_hunks` | 4 | 1 | 5 |
| `diff_lines-vs-diff_hunks` | 5 | 1 | 6 |
| `function_line_context-vs-functions` | 0 | 0 | 0 |
| `function_lines-vs-functions` | 3 | 0 | 3 |

The complete-hunk contrast changes both input unit and question framing; it is
not a clean single-variable ablation. These results establish faithful replay
and actionable disagreements, not a universal improvement in Jev. Inspect the
retained false-positive and false-negative IDs in outcomes.json before choosing
a new experiment. No reference labels were changed to make an arm look better.

## Flexibility established

Application-specific adapters preserve native outcomes and auxiliary evidence.
Mixed originating models/configurations receive distinct source definitions.
Partial scoring families are disclosed and retained as observations without
dropping cases, inventing values, or weakening Validator's all-or-none rule.
Rounded categorical distributions are not silently normalized. Score-like fit,
selected-category probability, minimum-stage probability and native confidence
remain different quantities.

The only Rust behavior change is CLI discovery: all four commands are listed,
per-command `--help`/`-h` works, and required output destinations are explicit.
The numerical core, schemas, metrics and persisted run contracts are unchanged.
Unknown commands still return the existing machine error contract.

## Not completed

There are no new Jev accuracy measurements for Sift, Foreman, Upwork, tax, generic
classification, filing, compaction, routing or jev-align. Their current outcomes
are source-shaped synthetic adapter proofs and pending-reference proposals.
The live Sift runner is provided but not live-tested. Other application-native
capture runners and full upstream runtime integration tests are not implemented.

The optional new jgrep cases, tax page-kind extension, router policy-fixture suite,
full-registry tax capture, independent semantic review, production distributions,
and optimization experiments remain unexecuted. The router draft uses a family-
preserving 39/21 split rather than forcing the planned 40/20. Template correlation
and synthetic-reference bias limit every drafted corpus.

The next substantive proof is a human-reviewed Sift subset through the provided
bounded capture runner, followed by native run → review → recommend. No task
management, autonomous remediation, gold revision, or provider telemetry was added.

## Local sandbox continuation — 2026-09-20

The same PR's CI-built executable was downloaded and run in the local sandbox;
its SHA-256 is `bf33ab4a1ca16ee7cf985ff6d83bacf58344b452580639ddd0ce4c7ad0898dab`.
The build archive identifies the PR test merge as
`04611af0966609ced70315ea4123011714562f24`; its head was `e1fc38e`.
The archive digest was verified before extraction. This sandbox has Python and
Node but no Rust compiler, direct external DNS, or configured Jev/provider key.
That did not prevent execution of the downloaded native program.

Locally executed: all 40 existing Python contract tests, all ten native adapter
lifecycle fixtures, the six published jgrep replays and four comparisons, plus
**58 new assertions against actual upstream application implementations**:
24 router policy cases, 18 Sift classify/provider checks, and 16 Foreman
schema/parser/assessment checks. All 80 proposed Sift inputs and 60 proposed
Foreman observations passed their actual upstream schemas. The resulting native
fixture captures completed six Validator evaluations, three paired comparisons,
inspection, receipt-hash verification and relocated inspection.

The native runtime bundle was obtained from
[run 35528954816](https://github.com/Dowwie/validator/actions/runs/35528954816),
with source archive SHA-256
`3c387662c51560694d231a8590ecbd3a788de81474f1d31a9f2a92f57b62d0c6`.
It contains clean pinned public checkouts and locked Node dependencies, not keys.
Sift and router execute their installed dependencies. Foreman uses real upstream
Pydantic types and the upstream-supported injected-client path; its fake replies
do not exercise a live TypeSafe SDK transport. No workers or file-moving flows run.

The primary router fixture suite passes. Four separate malformed-input probes
show three cases where missing, string, or out-of-range confidence still permits
a downgrade. Foreman accepts a finite 8.0 through normalization and clamps it to
1.0. These are low-level boundary findings, not proof of production exposure or
Jev errors. Full probe outputs are retained in
[the native outcomes](../../examples/jev-poc/native/outcomes.json).

No new model inference or human reference approval occurred. These runs close the
router's planned deterministic-fixture slice and add real application-boundary
coverage for Sift and Foreman. They do not complete live accuracy experiments,
remaining capture runners, semantic reference review, or optional dataset expansion.
The measurement core and third-party source remain unchanged.
