# Explicit-null correction coordination response

I accept the owner's concrete pre-freeze finding. Ordinary `Option<T>` decoding in
the accepted private wire DTOs can erase the distinction between an omitted field
and a present JSON null. That is material at this admission boundary: a null
`episode_ids` can otherwise take the select-all path, and a null typed scoring or
observation field can otherwise be treated as unavailable.

I have paused the frozen-review dispatch and issued the bounded developer
correction at `003-coordinator-to-developer.prompt.md`. It requires a regression
through the real decode/admission entry, the smallest presence-sensitive wire
change if reproduced, and coverage of every optional typed wire field sharing the
defect. Omitted and valid present values must remain legal; explicit null typed
scalars, objects, arrays, and strings must fail with `E_SCHEMA`. Opaque `input` and
opaque configuration values retain legal nulls, including nested nulls.

The developer must preserve the eight passing T004-T006 named filters and the
three accepted T002 named filters, rerun the combined checks, and report exact
changed hashes and warning classes. No T007-or-later work, alternate decoder,
permissive mode, blanket opaque-null ban, public API, suppression, or fake caller
is authorized.

The initial manifest 002 records the candidate immediately before this owner
finding and is retained as historical evidence. It is not being sent to the
verifier. After the correction, I will publish a new superseding manifest and a
combined verifier dispatch that explicitly checks the repaired omission-versus-
null distinction while reusing unchanged T002 evidence. The normal combined
checkpoint's one verifier-driven repair allowance remains unused.

