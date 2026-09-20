# Resolve one changed-feature regression in the current review

Role/model: existing Sol-high coordinator, retained context.
Repository: `/Users/dowwie/MyProjects/validator`.
Required response:
`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t004-t006/008-coordinator-to-owner.response.md`.

The owner read the combined correction handoff. Keep the frozen review active;
do not start another implementation loop before the verifier consolidates its
findings. There is one concrete regression concern from removing arbitrary_precision
that the current acceptance must resolve.

The recursive duplicate pass still invokes deserialize_any on every value
(`src/validation/wire.rs`, DuplicateKeySeed and reject_duplicate_keys). Without
arbitrary_precision, serde_json's ordinary number parsing has binary64 range
limits. Thus a syntactically legal opaque number such as `1e400` may fail in that
pass before its RawValue can be retained. This differs from the already-passing
integer 9007199254740993 case: it tests range, not merely precision. Main-spec
Canonical golden dataset permits arbitrary JSON input; Validation/numerical
rules requires opaque payload/configuration JSON numeric values to be retained,
without applying scoring semantics. The numeric marker correction was required
to preserve all existing opaque-value behavior.

Give the verifier this exact focused concern by saved follow-up. Establish whether
a legal opaque value containing `1e400` (input and configuration) is accepted and
retained through the production decode path. Also check the specified distinction
for a typed scalar observation with `value: 1e400`: invalid observation numeric
values must produce E_OBSERVATION, rather than being silently accepted, turned
into opaque evidence, or mistaken for malformed JSON syntax. Source reference:
main spec machine-interface diagnostic rule at lines 897–902 and the observation
value/range contract. These cases must not change binary64 scoring arithmetic.

If the current path passes, record the evidence without changes. If it fails,
consolidate it with any other demonstrated findings in the current Revise verdict
and use the existing one-repair allowance. Do not blacklist large opaque numbers,
relax duplicate checks, invent a parser/framework, or weaken the observation error
contract. Ordinary bounded raw-value/tagged DTO changes and needed serde features
remain allowed under the prior correction scope. Escalate a repeated failed repair
with the concrete cause and smallest alternative.

This is the specific consequence of a changed parser feature, not authority for
a broad parser/dependency audit. Preserve unchanged evidence and the fixed review
boundary. Save/index/link the response and follow-up in the current Fizzy record.
