# Save the T028 atomic checkpoint and yield the sole writer

T026's independent review has found a material exact-filter gap and Card188 retains
priority. Finish only the currently running protected-bundle reconciliation/
resumption check, make no new source-copy expansion, then save a complete resumable
checkpoint response to:

`/Users/dowwie/MyProjects/validator/docs/dispatches/validator-t028/003-source-developer-to-coordinator.response.md`.

Record the protected root, exact manifest/map hashes, copied counts/bytes,
permissions/ownership/no-symlink verification, UUID method/counts, 603/27/630 and
route/cohort reconciliation, pre/post source-unchanged proof, completed resumption
checks, remaining T028 work and any gap. Do not expose payloads. Do not start T028
independent review or edit Validator candidate files.

After saving the response, return its path/hash and stop. The coordinator will
resume T028 from this exact checkpoint only after the T026 repair writer is done
and the sole-developer slot is free again. This pause does not reject or accept the
private candidate and does not authorize T029.
