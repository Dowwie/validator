# Resolve zero-exclusion intersection in the current review

Existing Sol-high coordinator. Repository `/Users/dowwie/MyProjects/validator`.
Save acknowledgment to
`docs/dispatches/validator-t021-t023/010-coordinator-to-owner.response.md`.

Keep manifest008 frozen and the existing single combined review active. The owner
read full T023 response007 and inspected its exact schema/scope functions. The
handoff explicitly requires nonempty exclusion for intersection documents.
`schemas/v2/comparison.schema.json`'s intersection_population definition requires
one excluded_count >= 1, while `comparison::intersection_scope` computes a normal
set intersection, which can equal both supplied selections.

Give the current verifier a numbered file-based addendum limited to this case:
run explicit `--intersection` on compatible equal selected populations, including
equal empty selections, and check the emitted comparison against its published
schema. The ratified comparison contract permits explicit intersection and imposes
no requirement that selected sets differ or that an exclusion exist. Determine
whether the implementation/schema combination incorrectly rejects a legal result
or emits an invalid document. Inspect both concrete task branches, reusing the
existing test/fixture path and bounded diagnostics.

Include the conclusion and evidence in the existing saved verdict. If reproduced,
report the smallest correction that admits zero-exclusion intersection while
preserving strict identical-scope constraints and task-family discrimination;
do not force artificial exclusion or silently change the requested scope. This
is a current T023 criterion concern, not a new audit or separate review cycle.
No source/schema/test edits during review, no broader diagnostics, no T024 work.
Index/link this prompt/addendum/responses and preserve all existing review bounds.
