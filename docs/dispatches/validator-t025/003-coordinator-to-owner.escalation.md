# T025 blocked: multi-label abstention wire evidence omits required status

Status: **owner scope decision required**. T025 output 1 is paused; T026+ remain
prohibited. No production/schema correction, expected-value change or tolerance
change has been made.

## Independent oracle boundary

The fresh output-1 developer reconciled every starting hash and did not read
`src/evaluation*` or production metric/count helpers to derive expectations. It
built direct test-side enumeration for 301 single-label populations and 64
multi-label answered subset pairs plus 8 whole-abstention cases. The existing
fixture values remain unchanged; F04 is independently reconstructed as accuracy
5/8 and macro-F1 131/210.

`exhaustive_single_label` passes. `exhaustive_multi_label` reaches a successful
public report for the minimal expected-empty-set/predicted-whole-abstention case,
then stops on literal wire evidence.

Complete developer record:

| Artifact | SHA-256 |
|---|---|
| `002-oracle-developer-to-coordinator.response.md` | `43c0dfa849fce1ff3a4625135e1ea0f8b9306628860c75b255203ee4942fb925` |
| `tests/conformance.rs` | `0dff9c6b0fbe51f11cabb9e189b7d86dcf11490a5eae8ad33e0063bd75b8517a` |
| `tests/fixtures/single-label/expected.json` | `1fc52d5a24d7900fbef992b0f19227089d83e38d0656b9809fc08f9ed0dcbfe0` |
| `tests/fixtures/multi-label/expected.json` | `213183a46c0f6c011296ef5087b5c44d3852081bdf20eaa80cbb494d3d2c84f2` |

## Reproduction and contract

Minimal public input uses multi-label vocabulary `[A,B,C]`, expected `labels: []`,
and submitted `{"type":"abstention","reason":"oracle"}` under `as_recorded`.
The independent expected counts are `N=1,U=1,G=0,D=0,E=0`.

Ratified `validator-v1.md` lines 669-673 require that abstention set differences
are null with `status: "abstained"`. Expected report evidence is therefore:

```json
{
  "type": "abstention",
  "reason": "oracle",
  "status": "abstained",
  "matched": null,
  "missed": null,
  "extra": null
}
```

Actual public report evidence is only:

```json
{"type":"abstention","reason":"oracle"}
```

The exact assertion sees
`report["episodes"][0]["raw_outcome"]["status"] == null`. Current
`schemas/v2/report.schema.json` also accepts the bare form.

The checked evaluator already owns the correct internal evidence:
`MultiLabelEpisodeResult` carries `status: Abstained` with `matched/missed/extra`
all `None` (`src/evaluation/multi_label.rs:742-751`). The loss occurs in public
serialization at `src/model/multi_label.rs:630`, which emits only type/reason, and
the report schema's `multiOutcome` reuses the generic bare abstention definition.

## Smallest proposed correction

Authorize one bounded correction before resuming output 1:

1. In `src/model/multi_label.rs`, serialize a multi-label abstention outcome with
   literal `status: "abstained"` and `matched`, `missed`, `extra` set to JSON null,
   preserving `type` and `reason`. Do not change scoring/counting.
2. In `schemas/v2/report.schema.json`, add a closed multi-label abstention
   alternative requiring those exact fields/nulls/status. Preserve the existing
   single-label bare abstention alternative and every answered multi-label shape.
3. Keep the frozen exhaustive assertion as the primary regression and add only the
   smallest schema positive/negative assertion necessary to reject the old bare
   multi-label form. Rerun affected report/schema/exhaustive tests and normal local
   gates.

This proposed scope changes public report serialization and its schema, so it is
outside the current T025 oracle-only write boundary and requires owner approval.
Do not alter expected evidence: the ratified literal spelling is the authority.
After a corrected local handoff, resume the same output-1 enumeration from its
preserved work; do not restart or reset its evidence.
