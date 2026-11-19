# profile-create

Register a new Tekla profile prefix as an **alias of a section Tekla already loads**, by adding one row to
the environment's `profitab.inp` (the folder named by `XS_PROFDB`). The new prefix then measures and takes
sizes exactly like the shape it aliases.

```text
AWTRI    ! USER ! 0  !   !  2 !  2 !TEKLAINC.TRIANGLE !h*b      <- the row added
```

This is the **alias route only** (aware#858). It never writes a `.clb` or any other Tekla-owned library, and a
shape Tekla does not have is not made here: that is the Item route (aware#859).

```powershell
# 1. plan: writes nothing
'{"mode":"plan","profile_database":"C:\\...\\environments\\default\\Profil\\","prefix":"AWTRI","alias_of":"TEKLAINC.TRIANGLE"}' |
  aware-tekla profile-create --json-stdin
# 2. create: backed-up write (same request with "mode":"create")
# 3. restart Tekla, then read it back with profile-verify
# 4. undo, if needed: '{"mode":"undo","receipt":"<receipt path from step 2>"}'
```

It is plain file I/O, so **no running Tekla is needed to write**. `profile_database` is the folder
`section-measure` reports as `environment.profile_database`.

## Takes effect after Tekla restarts

Tekla reads `profitab.inp` only at start. A running Tekla does not rescan it, so the result always says
`takes_effect: "after Tekla restarts"` and, when a Tekla is running, that it has not seen the row. Run
[`profile-verify`](profile-verify.md) after the restart.

## The file is shared

`profitab.inp` is the environment's profile database: every model on that environment, and the firm if the
folder is shared, sees the new prefix after Tekla restarts. The result names the file (`profile.file`) and
says so (`profile.shared_note`).

## Rules

| rule | detail |
|---|---|
| prefix | upper-case letters, digits, `-`, `_`; 1 to 8 characters; starts with a letter; does not end in a digit. A prefix over 9 characters registers a broken `PROFILE_UNKNOWN` entry that `SelectByPrefix` cannot see, so 8 is the cap. |
| `alias_of` | `LIBRARY.SECTION`. The section must already be named by a `USER` row in this `profitab.inp` (proof that this environment loads it); otherwise `target-unknown`. The new row copies that row's columns and parameter string. `parameter_string` picks between rows that alias the same section. |
| encoding | `profitab.inp` is ISO-8859-1 with CRLF line endings. Every original byte is kept; the row is appended in the file's own line ending. |
| existing prefix | the same definition is `reused-identical` (exit 0, nothing written); a different one is `conflict` (exit 2, nothing written, never overwritten). |

## Modes

| `mode` | what happens |
|---|---|
| `plan` (default) | Checks everything and reports the row and the file hashes. Writes nothing, creates no backup. |
| `create` | Backup, receipt, atomic replace, read-back. |
| `undo` | With `receipt`, restores the exact pre-write bytes. If the file changed since the write it answers `changed-since`; send `force: true` to restore anyway. |

`create` order: read the file and hash it (SHA-256) -> copy it to `%LOCALAPPDATA%\aware\tekla-profile-create\<folder key>\<run id>\profitab.inp.bak` (or under `backup_dir`) and verify the copy -> write `receipt.json` -> check the file is unchanged -> write a temp file next to it and `File.Replace` -> re-read and compare the hash. A failure before the replace leaves the file untouched (`failed` / `backup-failed`). A failure after the replace restores the backup and reports `partial` with `restore: restored`; the receipt records the state.

## Output

`{ok, result, host, verb, delivered_at}` with

```text
result: {
  status: planned | created | reused-identical | undone | conflict | target-unknown | invalid-request
        | changed-since | backup-failed | partial | failed,
  message, written, tekla_running?, takes_effect?,
  profile: {file, profile_database, prefix, alias_of, parameter_string, template_prefix, row, shared_note},
  file_change?: {path, sha256_before, sha256_after, backup, bytes_before, bytes_after},
  receipt?, undo?: {request, instruction}, verify?: {verb, after, request}
}
```

Exit codes: 0 `planned`, `created`, `reused-identical`, `undone`; 2 refused before any write (`invalid-request`,
`target-unknown`, `conflict`, `changed-since`, a damaged backup on undo); 1 a failure (`backup-failed`, `failed`,
`partial`).
