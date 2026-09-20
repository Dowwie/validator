# Documentation Management

`docs/artifact-index.md` is the artifact-discovery and authority-routing index. Consult it after
`docs/README.md` when you search for project information. Use the index to identify likely
authoritative sources before broad repository searches; use `rg` to verify and supplement the
index, not to bypass it.

Maintain the index in the same change that alters the artifact set or an artifact's authority:

- Add an entry for every file created under `docs/`.
- Update the entry when a documented artifact moves, is renamed, changes authority status, or
  materially changes purpose.
- Remove the entry when the documented artifact is deleted.
- Index an artifact outside `docs/` when it becomes an accepted input, authoritative dataset,
  immutable run result, governing prompt, operator entry point, or other artifact required to
  resume or verify material work. Do not index transient logs, caches, or replaceable intermediate
  files.

Each entry must link to the exact artifact and state its authority status, purpose, and appropriate
use. Use the authority vocabulary from `docs/README.md`. Do not copy task state into the index;
Fizzy remains authoritative for owners, blockers, dependencies, and next actions.

Before completing work that changes the artifact set or an artifact's authority:

1. Inspect the Git status and diff for created, moved, renamed, and deleted files.
2. Search `docs/artifact-index.md` for every affected path. Add, update, or remove entries in the
   same change.
3. Verify that every changed index link resolves and that no entry points to a deleted or renamed
   artifact.
4. Run `git diff --check` and report the index update in the completion summary.

Treat missing index maintenance as incomplete work.

If a read-only task reveals a missing or stale entry, report the mismatch. Do not mutate the index
unless the user also authorized documentation changes.

# File-based subagent dispatches

All subagent dispatches must be file-based, including one-off reviews, delegated implementation,
and follow-up task instructions. This applies to every agent in the delegation chain, regardless
of the runtime or skill used.

1. Before dispatching, save the complete instructions under `docs/dispatches/<task>/` in a uniquely
   named file such as `001-owner-to-reviewer.prompt.md`. Include scope, source paths, constraints,
   completion criteria, and the required response-file path. Record the requested model, reasoning
   effort, and context-inheritance setting when applicable.
2. Send the subagent the absolute prompt-file path with instructions to read it and save its full
   substantive response to the specified file, such as `001-reviewer-to-owner.response.md`.
   Runtime messages carry file references; they are not the sole record of task instructions.
3. Save changed instructions in a new numbered prompt file before sending a follow-up. Preserve
   earlier dispatches and responses rather than overwriting their history. Require subagents to
   follow this same protocol for any authorized delegation of their own.
4. The subagent must save its complete findings, verdict, or implementation handoff before
   returning the response-file path. The parent must read that file before accepting the handoff.
   A chat response, Fizzy summary, or later reconstruction does not replace these artifacts. If a
   subagent fails before saving its response, record the failure without inventing a result.
5. Index the prompt and response files in `docs/artifact-index.md` and link them from the relevant
   Fizzy record when one exists. Keep task state in Fizzy and exclude secrets from all artifacts.
