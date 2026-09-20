# Correct the oracle output permission boundary before execution

Apply on T030 resumption, before running its saved checkpoint. Record the fix in
the existing next complete T030 response; no extra acknowledgment artifact.

Owner inspected `scripts/acceptance/oracle_chord.py:439-444`. `write_json` calls
`os.chmod(path.parent, 0o700)` unconditionally. The saved resumption example writes
`/tmp/oracle-chord-a.json`; that would attempt to change the shared `/tmp`
directory's permissions. This is a demonstrated unsafe output side effect, not
optional hardening. It must not run as written.

The sole oracle author must remove chmod of arbitrary existing parent directories.
Use a newly created task-owned private temporary directory for reproducibility
runs, retain output files at 0600, and preserve all pre-existing directory modes.
Ensure the canonical protected output remains in its already owner-only bundle.
Verify the shared `/tmp` directory's mode remains unchanged. Do not repair this
by changing system directory ownership or permissions, adding a fallback, or
creating a generic filesystem framework. No other scope expansion.

The independent reviewer owns the separate native-count reconciliation. The
author needs its script's focused correctness/reproducibility checks, not another
maintained oracle/checker implementation. Keep this correction inside the existing
frozen-case task and continue toward its complete handoff.
