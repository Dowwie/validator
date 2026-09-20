# T025 output 1: independent exhaustive hard-decision oracles

## Scope and independence

I read the assigned T025 materials, the ratified numerical and verification
sections, public report schema, existing accepted fixtures, and the conformance
test setup. All pinned starting identities in dispatch002 reconciled exactly
before editing. I did **not** read `src/evaluation*`, production metric/count
helpers, or any production scoring implementation to derive an expectation.

The only calls to the application are test-boundary `validator::evaluate` calls
after the direct counters derive expected values. No production output was used
to construct, tune, or revise a fixture value.

Changed, task-owned files only:

- `tests/conformance.rs`
- `tests/fixtures/single-label/expected.json`
- `tests/fixtures/multi-label/expected.json`

The existing fixture values remain unchanged. The single-label fixture extends
F04 with its independently derived class-F1 rationals; both fixtures record the
bounded enumeration cardinalities.

## Direct derivations

`DirectSingle` builds a literal three-row, four-column integer matrix from raw
`(actual, Option<predicted>)` pairs. Column three is typed abstention. It derives
support, predicted support, TP/FP/FN, `N/D/E/U/G`, class coverage, all six hard
decision rates, direct F1, fixed-schema macro zero-fill, and the relevant status
rules without application code. The assertions prove:

- `N = D + E + U = sum(matrix) = sum(support)`;
- `G = sum(predicted support)`;
- `sum(TP) = D`, `sum(FP) = E`, and `sum(FN) = E + U` through the direct class
  values; and
- empty populations use `no_data`, while nonempty zero answered populations use
  `no_answered_predictions` for selective metrics. An absent class therefore
  remains undefined; an observed but always-missed class has defined F1 zero.

The single-label enumeration uses vocabulary `[A,B,C]`, every population length
0, 1, and 2, every actual sequence, every declared-class-or-abstention decision
sequence, and both record orders for length two. Its independent total is
`1 + 12 + 288 = 301` public evaluations.

The multi-label enumeration uses the eight bit-set subsets of `[A,B,C]`. For
every one of the 64 expected/answered-predicted pairs, it directly derives the
episode totals, the exact-set result, each label's TP/FP/FN/TN, support,
answered support, predicted support, micro-F1, Hamming numerator, and
vocabulary-ordered matched/missed/extra evidence. It verifies every per-label
four-count sum is `G` and all binary counts total `G*K` (three in the one-row
answered cases). It separately runs the eight expected-set whole-abstention
cases and derives `N=1,U=1,G=0,D=E=0`, no answered binary decisions, null set
differences, and `no_answered_predictions` answered metrics. Abstention is never
converted to an empty set.

F04 remains reconstructed directly from
`A→A,A→A,A→B,A→C,B→B,B→B,C→A,C→C`: matrix
`[[2,1,1,0],[0,2,0,0],[1,0,1,0]]`, accuracy `5/8`, class F1 values
`4/7`, `4/5`, `1/2`, and macro-F1 `131/210`.

## Frozen literal contract discrepancy

The single-label exhaustive filter passed. The multi-label exhaustive filter
stops at a real public wire discrepancy and has not been relaxed.

Exact minimal public input constructed by the failing enumeration case:

```json
{
  "golden": {
    "schema_version": 2,
    "task": {"kind": "multi_label", "labels": ["A", "B", "C"]},
    "episodes": [{
      "id": "01995c20-7d00-7000-8000-000000000001",
      "expected": {"type": "labels", "labels": []},
      "input": null
    }]
  },
  "predictions": [{
    "id": "01995c20-7d00-7000-8000-000000000001",
    "source_id": "source",
    "outcome": {"type": "abstention", "reason": "oracle"}
  }],
  "config": {
    "schema_version": 2,
    "population": "exhaustive_multi_label",
    "role": "development",
    "decision": {"type": "as_recorded"}
  }
}
```

Direct expected evidence, from the ratified multi-label hard-decision contract:

```json
{
  "raw_outcome": {
    "type": "abstention",
    "matched": null,
    "missed": null,
    "extra": null,
    "status": "abstained"
  },
  "counts": {"N": 1, "U": 1, "G": 0, "D": 0, "E": 0}
}
```

Actual successful public report evidence is a bare abstention outcome with no
`status` field; the precise test observation is:

```text
report["episodes"][0]["raw_outcome"]["status"] == null
```

The current public report schema also permits the bare abstention form and does
not require `status`. This conflicts with `docs/specs/validator-v1.md` lines
669–673, which require null set differences with `status: "abstained"` for an
abstention. This is not an oracle failure: the expectation follows the literal
ratified contract, and I have not changed a fixture, tolerance, production file,
or schema to hide it. Per dispatch002, I stopped here and leave the discrepancy
for the coordinator/owner's bounded production-or-schema correction decision.

## Commands and results

| Command | Exit | Result |
|---|---:|---|
| `cargo fmt --all` | 0 | Applied required Rust formatting. |
| `cargo test --locked --test conformance exhaustive_single_label -- --nocapture` | 0 | 1 passed; 301 independently derived populations checked. |
| `cargo test --locked --test conformance exhaustive_multi_label -- --nocapture` | 101 | 1 failed at the frozen `raw_outcome.status` literal-contract discrepancy above. |
| `git diff --check` | 0 | No whitespace errors. |

The required list confirmation, F04/accounting regression, full conformance
suite, and later T025 filters were not run after the public mismatch because the
dispatch explicitly requires freezing and stopping on such a discrepancy.

## Candidate hashes

| Artifact | SHA-256 |
|---|---|
| `tests/conformance.rs` | `0dff9c6b0fbe51f11cabb9e189b7d86dcf11490a5eae8ad33e0063bd75b8517a` |
| `tests/fixtures/single-label/expected.json` | `1fc52d5a24d7900fbef992b0f19227089d83e38d0656b9809fc08f9ed0dcbfe0` |
| `tests/fixtures/multi-label/expected.json` | `213183a46c0f6c011296ef5087b5c44d3852081bdf20eaa80cbb494d3d2c84f2` |

Verdict: **blocked on a frozen public contract discrepancy; no production or
schema correction attempted.**
