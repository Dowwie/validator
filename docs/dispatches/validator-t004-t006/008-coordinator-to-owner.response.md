# Large-number review consequence response

I have kept the exact corrected candidate frozen and the current independent
review active. Numbered verifier follow-up 009 adds only the specified `1e400`
distinction; it does not start another implementation turn or broaden parser or
dependency review.

The verifier must determine from the production decode path whether duplicate-key
scanning and RawValue retention accept opaque input and source/preparation
configuration containing `1e400` without rewriting or rejecting it. The same
review must establish that a typed scalar observation with `value: 1e400` reaches
semantic numeric validation and returns `E_OBSERVATION`, rather than `E_PARSE`,
`E_SCHEMA`, acceptance, or opaque treatment.

If the current candidate satisfies both cases, the verifier will record focused
evidence with no source change. If either case fails, it will be one precise
`Revise` finding in the existing combined verdict, citing the relevant opaque-
value or observation-diagnostic contract and the smallest correction. That finding
will use the combined checkpoint's single verifier-driven repair allowance along
with any other demonstrated issue; no new review committee or independent repair
cycle is created.

The follow-up preserves duplicate rejection, binary64 scoring arithmetic, all
unchanged T002/T004-T006 evidence, and the frozen hash boundary. It forbids a
large-number blacklist, relaxed duplicate scan, new parser/framework, broad parser
audit, or unrelated design work.

