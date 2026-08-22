# CLI Spec

The contract the `aware` binary satisfies. This is the implementation target — not a wishlist. Phased delivery is documented in [`cli-roadmap.md`](./cli-roadmap.md).

## Surface

```
aware
├── --help                              show top-level help
├── --version                           print version + git sha
├── --config <path>                     override config file location
│
├── agent ...                           manage installed agents
│   ├── list                            print installed agents (table)
│   ├── describe <agent>                manifest summary + skill index (curated/reflected counts)
│   ├── skill <agent> <skill-name>      print a skill's content
│   ├── install <agent>[@version]       fetch from registry or local path
│   ├── uninstall <agent>
│   ├── update <agent>                  re-pull latest matching version
│   ├── validate <path>                 schema + structure checks
│   └── publish <path>                  open a PR to the GitHub registry
│
├── tree <agent>[/<class>]              text tree of an agent's commands grouped by class
│   ├── --curated                       filter to hand-written workflow verbs only
│   └── --reflected                     filter to auto-generated API methods only
│
├── search <term>                       cross-agent substring search over command names + descriptions
│   ├── --limit <n>                     max results per agent (default 5; 0 = unlimited)
│   ├── --agent <id>                    restrict to one agent
│   ├── --curated                       filter to curated workflow verbs only
│   └── --reflected                     filter to reflected API methods only
│
├── app ...                             manage installed apps
│   ├── list                            print installed apps (table)
│   ├── show <app>                      print topology (ASCII) + provenance
│   ├── install <path-or-name>          register an app
│   ├── uninstall <app>
│   ├── run <app> [--instance <id>] [--input <kv>...]       execute
│   ├── stop <app> [--instance <id>]    stop a running app
│   ├── validate <path>                 schema + cycle + cap checks
│   ├── export <app> <output-path>      copy the app file out
│   ├── logs <app> [--instance <id>] [--tail]   read execution traces
│   └── artifact <app> <id> --output <path>     copy a run-owned large artifact
│
├── connect <integration>               provision OAuth credentials (default: browser-paste)
│   ├── --as <alias>                    named credential for multi-account
│   ├── --refresh                       force refresh
│   ├── --scopes <s1,s2>                add scopes (Google etc.)
│   ├── --from-file <path>              load token from file (CI / service accounts)
│   ├── --from-env                      load token from AWARE_TOKEN_<INTEGRATION>
│   ├── --oauth                         PKCE loopback flow (registered OAuth app)
│   ├── --device-code                   RFC 8628 device-code flow (headless / IT-managed) (v0.13)
│   └── --tenant <id-or-domain>         M365 tenant override (v0.13)
│
├── disconnect <integration> [--as <alias>]    delete credential file
│
├── skill ...                           skill-builder commands
│   ├── create <agent> <skill-name>     new skill via skill-creator
│   ├── port <source> <target-agent>    port from external source
│   ├── modify <agent> <skill-name>     refine existing
│   └── eval <agent> <skill-name>       run skill-creator eval
│
├── build agent                         agent-builder (the meta primitive)
│   ├── --from-dlls <path>
│   ├── --from-nuget <pkg>[@version]
│   ├── --from-openapi <url-or-path>
│   ├── --from-com <progid>
│   ├── --from-cli <binary>
│   ├── --from-headers <path>
│   ├── --from-python <module>
│   ├── --from-csharp <path>            C# source (.cs file/dir/glob) via aware-roslyn
│   ├── --reference-dir <dir>          extra ref-DLL dir for --from-csharp (repeatable)
│   ├── --from-csproj <path>           C# project (.csproj) via MSBuildWorkspace (needs .NET SDK)
│   ├── --from-sln <path>              C# solution (.sln) via MSBuildWorkspace (needs .NET SDK)
│   ├── --decompile                     opt-in, license-checked
│   ├── --tier-strategy <auto|all-1|all-2>
│   └── --output <agent-id>
│
├── sidecar ...                         manage AWARE runtime sidecars
│   ├── list [--json]                   status of every managed sidecar
│   ├── install <id>                    download one named sidecar
│   ├── repair --installed              refresh all installed stale managed sidecars
│   └── uninstall <id>                  remove one managed sidecar
│
└── doctor                              health check — config, creds, hosts, registry
```

### Large command outputs

An agent command may materialize a large result as a run-owned artifact rather than embed it in a
`node-output` JSONL event. Its ordinary node output is then a small descriptor:

```json
{ "$aware-artifact": { "id": "read-model-<uuid>.json", "mediaType": "application/json", "bytes": 305623149, "items": 2993 } }
```

`id` is opaque and scoped to the app, instance, and run. A local consumer resolves it with
`aware app artifact <app> <id> --run-id <run> --output <path>`; it must never treat an artifact id
as a filesystem path. This keeps JSONL replay bounded while a producer writes data incrementally and
lets a renderer load or batch-read the resulting file without duplicating the payload in the trace.

### Progressive large outputs

A run-owned artifact bounds the trace; it does not make the result *early*. A `single`-lifecycle
command has exactly one output, emitted after the process exits, so a consumer of a 40-second read
can draw nothing for 40 seconds even though the first geometry existed after one. The progress
channel is the second, write-only channel that fixes that.

**The runtime** passes a CLI transport `AWARE_PROGRESS_FILE` — a path inside the same run-owned
artifact directory as `AWARE_ARTIFACT_DIR`, unique per invocation — whenever it will mirror what is
written there. It tails that file while the command runs and writes each valid record into the trace
as a `node-progress` event, flushed on arrival:

```json
{ "kind": "node-progress", "ts": "…", "run_id": "…", "node": "read",
  "data": { "phase": "batch", "seq": 7, "done": 700, "total": 2993,
            "artifact": { "id": "read-model-<uuid>.seg-00007.json", "mediaType": "application/json",
                          "bytes": 1048231, "items": 100, "seq": 7 } } }
```

**A producer** appends newline-delimited `{"$aware-progress": { … }}` records to that path. The
runtime mirrors a record only if it is a JSON object carrying a non-empty `phase` string; is at most
**8 KiB**; and, when it carries an `artifact` descriptor, names an id that passes the same fence
`aware app artifact` applies. Anything else — engine noise, a half-flushed line, an oversized record —
is skipped silently: the channel is advisory and the node output remains the authoritative result.
The size cap is what preserves #402's guarantee, since a channel that mirrored arbitrary JSON would
let a producer stream the payload back into the trace one "progress" record at a time. For the same
reason at most **10,000 records per invocation** are mirrored, and at most **16 MiB** of the channel
is read at all — the second budget covers a channel whose lines are all rejected, which the first
never sees. Past either, the runtime emits one `phase: "progress-suppressed"` record and stops
listening, so a consumer can tell that from a producer that simply went quiet.

`phase` is producer-defined; the conventional ladder for a geometry producer is `parse` →
`tessellate` → `batch` (once per delivered segment) → `complete`. `done`/`total` are optional
counters and `message` optional prose; both are rendered by `aware app logs`.

**Segments** are the delivery mechanism. A record's `artifact` block names an ordered slice that is
already durable in the run-owned artifact directory, retrievable **while the run continues** through
the ordinary `aware app artifact <app> <id> --run-id <run> --output <path>`. Nothing new is needed to
consume it: a consumer reads the run id `aware app run` prints before the first node starts, tails
the trace (`aware app logs <app> --tail`), and fetches each segment as it is announced. Neither the
trace nor the runtime ever holds a mesh.

Semantics a consumer may rely on:

- **Ordering** — records reach the trace in the order the producer wrote them; `seq` numbers segments
  from 1.
- **Durability** — a segment is announced only after it is completely written and renamed into
  place, so an announced id always resolves to a whole document.
- **Cancellation and failure** — segments and progress records already written survive a cancelled
  or failed run. They live beside the trace and are removed with it; nothing else prunes them. A
  producer killed mid-segment leaves an unannounced temporary file, which is never retrievable.
- **No resume** — the channel carries no restart protocol. A consumer that loses its place re-reads
  the announced segments, or falls back to the complete artifact named by the final node output.
- **Opt-in at the producer** — a command that has no reason to segment keeps emitting one
  `$aware-artifact` (or an ordinary inline output) and publishes no records. Small outputs are
  untouched by all of this.

### What "latest" means

`install <agent>` and `update <agent>` with no `@version` resolve to the greatest version by **SemVer §11 precedence** — not the greatest string. `1.10.1` outranks `1.9.0`, and `2025.0.10` outranks `2025.0.2`, which matters because the registry publishes calendar-shaped versions. A release outranks its own prereleases (`1.0.0` > `1.0.0-rc.1`), and build metadata carries no precedence at all (§10).

A version key that is not strict SemVer ranks **below** every key that is: it can still be asked for by name (`install <agent>@<that-key>`), but it never resolves as "latest" — a key nothing can reason about must not be what an unpinned install fetches.

A version after `@` is an **exact key**, not a range. Ranges are an app-pinning syntax; see [Agent Spec](./agent-spec.md) § Installation.

## Filesystem layout (what `aware` reads / writes)

```
~/.aware/
├── config.yaml                         # user config (default editor, default prompts, etc.)
├── agents/
│   └── <agent-id>/                     # installed agent (manifest + skills + commands)
├── apps/
│   └── <app-id>/                       # installed app
│       ├── <app-id>.<ext>              # app source; <ext> is .app (recommended), .flo, etc.
│       ├── lockfile.yaml               # pinned agent versions resolved at install
│       └── instances/<id>/state/       # per-instance state (stateful apps)
├── credentials/                        # encrypted; OS keychain on Mac/Linux, DPAPI on Windows
│   ├── trimble-connect.json
│   ├── microsoft-365.json
│   └── google-workspace.<alias>.json
├── permissions/
│   └── <app-id>.yaml                   # user's Allow / Always-allow / Deny decisions
├── logs/
│   ├── <app-id>/<instance-id>/<run-id>.jsonl    # provenance trail per run
│   └── <app-id>/<instance-id>/<run-id>.artifacts/   # that run's large outputs, addressed by
│                                                    # `aware app artifact` — the whole result, any
│                                                    # progressive segments, and each invocation's
│                                                    # progress channel. Removed with the run's logs
├── cache/
│   └── registry-index.json             # last-known agent registry index
└── plugins/                            # generated for each agentic CLI host
    ├── claude-code/aware-aeco/
    ├── codex/aware-aeco/
    └── opencode/aware-aeco/
```

## Response envelope

All commands that produce structured output use the same JSON envelope when `--json` is passed:

```json
{
  "ok": true | false,
  "data": <command-specific payload>,
  "error": {
    "code": "<machine-readable-code>",
    "message": "<human-readable>",
    "details": <command-specific debug info>
  } | null,
  "meta": {
    "cli-version": "0.1.0",
    "command": "agent describe",
    "duration-ms": 42
  }
}
```

Without `--json`, output is human-readable text. The envelope shape is stable across versions; `data` payload may evolve per command (semver applies).

## Exit codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General failure (see stderr) |
| 2 | Misuse / bad args |
| 3 | Validation failed (manifest, app, or schema) |
| 4 | Network error (registry, OAuth, agent runtime) |
| 5 | Permission denied (user refused, or capability not granted) |
| 6 | Auth expired (refresh required) |
| 7 | Agent / app not found |
| 8 | Conflict (already installed, name collision) |
| 64+ | Reserved for future use |

## Command details

### `aware agent list`

Print a table of installed agents.

```
$ aware agent list
ID                       VERSION    KIND              SKILLS  COMMANDS
tekla                    2025.0.1   tekla agent       31      3
trimble-connect          2.4.0      trimble-connect   7       3
microsoft-365            1.0.0      microsoft 365     4       4
google-workspace         1.0.0      google workspace  4       5
html-report              1.0.0      utility           1       1
aware-agent-builder      0.1.0      meta              5       1
aware-skill-builder      0.1.0      meta              6       4
```

Flags: `--json`, `--filter <kw>`, `--sort <name|version|skills>`.

### `aware agent describe <agent>`

Print the agent's manifest summary + skill index + command list.

```
$ aware agent describe tekla
agent:        tekla
version:      2025.0.1
description:  Watches the active Tekla model...
stateful:     true
vendor:       trimble
license:      Apache-2.0
transport:    cli

commands:
  watch              start    Subscribe to ModelObjectChanged events
  insert             single   Create a ConnectionPart at a world-space location
  save-attributes    single   Run the Akit save-as pattern

skills (31):
  - drawing-identity.md
  - event-threading.md
  - coordinate-systems.md
  - events-and-clashes.md
  - application-library.md
  ...
```

### `aware app run <app>`

The heaviest command. Loads the app file, resolves agent dependencies via the lockfile, starts any stateful agents, wires connections, and either:
- Returns immediately (one-shot app with only stateless nodes)
- Blocks until stopped (long-running app with stateful nodes)

Output streams to stdout per run; full trace written to `~/.aware/logs/<app>/<instance>/<run>.jsonl`.

```
$ aware app run welded-to-tc --instance fab-east \
    --input tc-project-id="...." \
    --input tc-folder-id="...."

✓ Loaded welded-to-tc@0.3.1 (instance: fab-east)
✓ Resolved: tekla@2025.0.1 + trimble-connect@2.4.0
✓ Permissions confirmed (cached from previous run)
✓ Started stateful node: tekla-watch
  ... waiting for Tekla events ...

[14:03:42] tekla-watch    event   A-104 (welded)
[14:03:43] filter-welded  pass    A-104
[14:03:43] tc-upload      info    POST /folders/..../files
[14:03:44] tc-upload      ok      A-104 uploaded (file_id=f_8u2k1)

^C   # user pressed Ctrl-C
✓ Stopping...
✓ Stopped: tekla-watch
✓ Final state saved to ~/.aware/apps/welded-to-tc/instances/fab-east/state/
```

### `aware connect <integration>`

Provision OAuth credentials. PKCE flow (no client secret needed on-device).

```
$ aware connect trimble-connect

Opening Trimble OAuth in your default browser...
✓ Waiting for callback on http://localhost:7421/callback
   (signed in as: pawel@bimstudio.io)
✓ Received auth code
✓ Exchanged for tokens
✓ Encrypted to ~/.aware/credentials/trimble-connect.json
✓ Done.

The trimble-connect agent can now make authenticated calls.
```

Subsequent commands transparently use the credential. Refresh happens automatically inside `aware app run`.

`connect` covers the integrations AWARE ships an OAuth client for, and validates its
`INTEGRATION` argument against that list. For a handle AWARE runs no OAuth flow for, use
`aware credential` below.

### `aware credential put|delete|status <handle>`

Provision, rotate and revoke an **opaque** credential — a generic REST bearer or API key
AWARE stores but does not mint. The REST transport already resolves any handle an agent
declares in `auth: { scheme: bearer, secret: <handle> }`; this is the supported way to put
one there, so callers that generate their own short-lived tokens never write
`~/.aware/credentials/` by hand.

The secret is read from stdin (default), `--from-file`, or `--from-env`
(`AWARE_TOKEN_<HANDLE>`) — **never from argv**, where it would land in shell history and in
every process listing on the machine.

```
$ printf %s "$TOKEN" | aware credential put floless-workspace --as session
✓ stored credential floless-workspace.session (OS keychain or ~/.aware/credentials fallback)

$ aware --json credential status floless-workspace --as session
{"status":"present","handle":"floless-workspace.session"}

$ printf %s "$NEW_TOKEN" | aware --json credential put floless-workspace --as session
{"status":"rotated","handle":"floless-workspace.session"}

$ aware --json credential delete floless-workspace --as session
{"status":"revoked","handle":"floless-workspace.session"}
```

Contract:

- **`handle`** is the string an agent manifest names in `auth.secret`. With `--as <alias>`
  the stored handle is `<handle>.<alias>`, which is what the manifest must then carry — the
  command echoes it back as `handle` so a caller never has to derive it.
- **`put` replaces atomically.** The next REST invocation uses the new value; a concurrent
  one sees either the whole old credential or the whole new one, never a torn read.
- **`delete` is idempotent and fails closed.** An absent handle is success (`absent`); a
  removal that could not be completed is an error, never a success that leaves the
  credential readable.
- **`status` never prints the secret** and always exits 0 — it reports `present`, `missing`
  or `unusable` in its field, so a script branches on
  `aware --json credential status <handle> | jq -r .status`, not on the exit code.
  `present` answers the question the caller is actually asking — *would the REST transport
  authenticate with this?* — so it is decided by the transport's own resolver, not by a
  second reader with its own idea of what counts. `unusable` means something **is** stored
  and no usable secret came out of it: corrupt JSON, an unreachable keychain, or a blank
  value. A blank credential is never `present`; the runtime treats it as absent too, rather
  than sending a bare `Authorization: Bearer` and reporting a successful run.
- **Storage stays AWARE's.** Whether the bytes live in the OS keychain or the
  `~/.aware/credentials/` fallback is not the caller's concern and is not part of this
  contract.
- Handles are lowercase `a-z0-9`, `-`, `_`, dot-separated, starting and ending
  alphanumeric. A registered OAuth integration is refused and points at `aware connect`,
  which owns its refresh token; the `oauth-app.` prefix is reserved for BYO client secrets.

### `aware doctor`

Health check. No mutations. Useful before filing a bug.

```
$ aware doctor

CLI:
  ✓ aware v0.1.0 (built 2026-04-15 abc123f)
  ✓ Config at ~/.aware/config.yaml

Filesystem:
  ✓ ~/.aware/ writable
  ✓ ~/.aware/credentials/ has correct permissions (0700)

Agents:
  ✓ 7 installed
  ⚠ tekla@2025.0.1 — host software not detected (Tekla Structures not on PATH)

Apps:
  ✓ 2 installed (welded-to-tc, qa-drawings-to-tekla)

Credentials:
  ✓ trimble-connect       valid    expires in 23 minutes (will auto-refresh)
  ✓ microsoft-365         valid    expires in 41 minutes
  ✗ google-workspace      missing  run: aware connect google-workspace

Plugins (host-side):
  ✓ ~/.claude/plugins/aware-aeco/   in sync
  ✗ ~/.codex/plugins/aware-aeco/    out of date — run: aware plugins regenerate

Registry:
  ✓ aware-aeco/aware (last index pull: 4 hours ago)
```

### `aware sidecar list --json`

Returns AWARE's complete managed-sidecar catalogue using the standard response
envelope. Consumers use this contract instead of carrying their own list of
sidecar IDs or inspecting the `~/.aware/bridges/` filesystem layout.

```json
{
  "ok": true,
  "data": {
    "schema-version": 1,
    "runtime-version": "0.120.0",
    "sidecars": [
      {
        "id": "connection-reader",
        "binary": "aware-connection-reader",
        "description": "Connection Reader (Node + web-ifc WASM; extract steel connections from IFC)",
        "status": "stale",
        "installed-version": "0.119.0",
        "repair-eligible": true
      }
    ]
  },
  "error": null,
  "meta": { "cli-version": "0.120.0", "command": "sidecar list", "duration-ms": 1 }
}
```

`status` is owned by AWARE and is one of:

| Status | Meaning | `repair --installed` |
|---|---|---|
| `current` | A managed copy and its version marker match the running CLI. | Skipped. |
| `stale` | A managed copy exists but its marker is absent or from another CLI version. | Refreshed. |
| `legacy` | Only an unmanaged PATH copy was found. | Never overwritten; use `sidecar install <id>` to migrate deliberately. |
| `missing` | No copy was found. | Never installed implicitly. |

`aware sidecar repair --installed` snapshots every `stale` entry in this
catalogue and refreshes each with the currently running CLI's release asset. It
does not accept a consumer-supplied tool list, install absent sidecars, or alter
legacy PATH copies. A caller queries `sidecar list --json` again after repair to
observe the authoritative result.

## Out of scope for the CLI itself

These belong to other components, not the CLI binary:

- **A visual authoring canvas** (a separate downstream project). Reads the same files (the installed apps under `~/.aware/apps/`, agent manifests). Separate executable, separate license.
- **The registry web service.** v0 uses GitHub-hosted JSON; later replaced by a hosted service. CLI just consumes the index.
- **Hosted execution.** AWARE is local-first. Cloud execution is a separate product layer if/when it ships.

## Versioning

- The CLI follows semver: `<major>.<minor>.<patch>`.
- The agent spec, app spec, and CLI spec evolve **together** under the same major version. v0.x is pre-stable; signatures may change. v1.0 = locked.
- Breaking changes to any spec require a `BREAKING.md` note + major bump.

## Verification before commit (for CLI work)

- [ ] `cargo fmt --all` passes (no diff)
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `cargo test` passes (unit + integration)
- [ ] New commands have at least one integration test under `cli/tests/`
- [ ] Help text (`aware <cmd> --help`) is informative — read it back and confirm
- [ ] Exit codes match the table above
- [ ] If the command touches the filesystem, the path is documented in this spec
