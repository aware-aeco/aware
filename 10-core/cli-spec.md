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
├── search <term>                       substring search over INSTALLED agents' command names + descriptions
│                                       (not-yet-installed agents live in `agent search`; every result says so)
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
│   └── artifact <app> <id> --output <path> [--max-bytes <n>] copy a run-owned large artifact
│   └── artifact <app> --run-id <id> --usage   measure retained bytes for one run
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
├── credential ...                      manage opaque credentials AWARE does not mint
│   ├── put <handle>                    store/rotate from stdin (or file/env)
│   ├── delete <handle>                 revoke idempotently
│   ├── status <handle>                 report present/missing/unusable
│   └── capabilities                    advertise stable machine-readable fingerprints
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
├── provider ...                        manage signed model-provider packages
│   ├── trust-publisher <key> --publisher-id <id>  enroll an Ed25519 trust root
│   ├── enroll <absolute-directory>     verify a signed closed package
│   ├── select <format-id> <manifest-sha256>       select one enrolled package
│   └── list [--format <format-id>]     list public package/capability metadata
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
When a consumer reserves a sealed-copy budget, `--max-bytes <n>` refuses a source larger than
that positive limit, checks the limit before every bounded write, creates the destination
exclusively and removes it on a failed copy. Omitting the option preserves the legacy copy behavior.
`aware app artifact <app> --instance <instance> --run-id <id> --usage` returns
`{app,instance,runId,bytes,files}` for all immediate regular files in that run's
artifact directory (including a failed-run spool or interrupted candidate), without
exposing any path. It refuses linked/reparse entries. A missing directory measures zero.

An opt-in REST command may instead declare `response: artifact-stream` in its agent manifest.
Its HTTP 200 NDJSON body is spooled in chunks under the current run's artifact directory,
with a hard `AWARE_REPORT_SOURCE_BYTES` ceiling and a required opaque
`AWARE_REPORT_RESERVATION_ID` supplied by the trusted caller. Its small output is
`{status:200,headers:{"content-type":"application/x-ndjson"},body:{artifact:{schemaVersion:
"aware.artifact-ref/v1",app,instance,runId,id,bytes,sha256,contentType}}}`.
Non-200 error bodies are bounded; no successful response body is parsed as one JSON value.
`html-report-stream.render-stream` resolves only a matching current-run descriptor, verifies
size and SHA-256, checks complete NDJSON counts/ordinals/terminal hash, and emits a
small `bundle` descriptor, exact object/property counts and `complete:true`.
Its candidate is atomically published only after a complete render within the trusted
`AWARE_REPORT_RENDER_BYTES` ceiling; `html-report.render` remains unchanged.
Direct `aware agent invoke html-report-stream render-stream` refuses with an app-run
explanation because it has no current-run artifact scope.
One completed source spool and one completed bundle may consume a given run reservation;
durable run-scoped claim files prevent a second streaming node from reusing its byte
partition. Ordinary failures release their claim; crashes retain it and fail closed.

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

### Registry release identity

Every installable registry version entry includes `manifest-agent` and `manifest-version`. These bind the registry key to the identity the extracted `manifest.yaml` must declare; the registry version and manifest version are deliberately separate values. Install and update require both bindings under every trust mode, validate the agent id with one portable filename-safe grammar, validate the manifest version as strict SemVer, and compare both values after extraction before any installed directory is created, removed, or replaced. `manifest-agent` must be the registry key, the key plus a non-empty dotted suffix, or an explicit `alias-of` target; this preserves versioned implementations without permitting an unrelated payload identity. A rename alias additionally requires `alias-of` to equal `manifest-agent`.

Errors before download name the registry key/version and the missing or invalid binding. A payload mismatch names the release, the bound value, and the value the payload actually declared. Bundle install and `agent update --all` inherit the same checks because they call the single-agent install/update paths.

An official bundle is `verified` only when its fresh registry binding, installation receipt, and installed manifest identity agree and its expected, recorded, and installed digests match. Digest equality alone cannot verify a payload under the wrong release name.

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
├── providers/
│   ├── publishers/<key-sha256>.json    # locally trusted Ed25519 public keys
│   ├── packages/<manifest-sha256>.json # verified immutable enrollment records
│   └── selections/<format-id>.json     # active package plus bounded history
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

An agent may return a bounded structured failure with `code`, `phase`, `retryable`, `message`, and `diagnosticId`. A `node-error` JSONL event preserves that object under `structured`. It may also carry an optional `details` string map for non-secret correlation values needed to reconcile an ambiguous side effect (for example `attemptId` and `rfcMessageId`). The field is omitted when absent, so pre-details payloads remain deserializable; it is correlation metadata, not a general debug-payload or secret channel.

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

The heaviest command. It first verifies the installed source against the engineer-approved `<app>.lock`: the lock must be present, parseable, and carry the SHA-256 of the exact raw source bytes. Compilation and runtime each parse and hash one source snapshot, so the compiled plan, approved bytes, and executed app cannot drift between reads. An unsafe `app:` id is rejected before it can become a lock path. A missing (`E_APP_LOCK_MISSING`), unreadable/malformed (`E_APP_LOCK_INVALID`), or mismatched (`E_APP_LOCK_STALE`) lock exits 3 before trace creation or node dispatch and tells the operator to run `aware app compile` again. Before real dispatch, every reachable agent must also match the exact compiled `agent-pins` version (`E_APP_LOCK_AGENT_PIN_MISMATCH`); simulation remains independent of ambient agent versions because it contacts no binary. Source approval applies independently to the top-level app and every app-backed agent it invokes, including `--dry-run` and `--simulate`.

Agent runtime compatibility is checked at both compile and run. An installed manifest with `status: requires-runtime` is dispatchable only when the running CLI is at least its strict-SemVer `minimum-cli-version`; otherwise validation refuses it before trace creation with `E_APP_AGENT_UNAVAILABLE` and names both versions. The same contract gates local and registry install, so an incompatible agent is never promoted into `~/.aware/agents/` by a current CLI.

Verified bundles additionally carry `agent-bundle-pins`, so changed bytes require recompilation. `app run --require-verified-agents` resolves the reachable set before any network or trace creation, snapshots every dispatchable executable bundle once, and refuses with `E_APP_AGENT_BUNDLE_UNVERIFIED` unless its official registry receipt still matches its tree. Frozen subtrees, inline-only graphs, and `--simulate` dispatch nothing and require no registry fetch; dry-run remains gated because read nodes still execute. For the single app-backed hop v0 permits, the approved backing app is traversed and its dispatchable leaf agents are assessed—the synthesized wrapper is routing metadata, not executable provenance. A second app-backed hop is refused. The snapshot is recorded as `verified-at-start` in run-start provenance.

After that gate, it loads the app file, resolves agent dependencies via the lockfile, starts any stateful agents, wires connections, and either:
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

Machine-readable connect surfaces expose capability metadata without exposing
token material. A successful `aware --json connect ...` result, each entry from
`aware --json connect --list [--as <alias>]`, and each credential entry in
`aware doctor --json` includes:

- `scopes`: the granted OAuth scope identifiers, sorted and deduplicated. When a
  successful OAuth token response omits `scope`, the scopes sent in that grant
  request are recorded. User-imported opaque tokens report only scope metadata
  supplied by the imported credential object; AWARE never infers a grant from a
  bearer token.
- `generation`: an opaque random identifier for the stored material grant. A
  fresh connection or import mints a new generation. Refresh preserves it when
  the normalized grant is unchanged and replaces it when the provider reports a
  materially different scope set. Disconnect removes it with the credential.

Credentials stored by older CLI versions remain valid. Their first materializing
read (including `connect --list`, but excluding `doctor`) lazily persists a
generation; repeated reads therefore return the same value. Materialization and
credential rotation are serialized per account across processes, and metadata is
written back to the same backend that supplied the credential. If the metadata
lock or write cannot be completed, the readable credential remains usable and
reports `generation: null` until a later materializing read can persist one;
AWARE never reports an ephemeral generation as stable. Missing or unreadable
credentials likewise report `generation: null` and an empty `scopes` array. The
generation is not derived from access or refresh token bytes and is not an
authenticator.

Lock domains follow storage domains. OS-keyring accounts use a stable per-user
lock namespace resolved from the native current-user identity/location rather
than `HOME`, `XDG_DATA_HOME`, `LOCALAPPDATA`, or `AWARE_HOME`, because the keyring
account itself is global to that user. Credentials-file locks remain inside their
owning `AWARE_HOME`. An operation that may touch both acquires the keyring lock
first and the file lock second; account-derived lock filenames are SHA-256 hashes
rather than user-controlled path segments.

Credential reads, metadata migration, and refresh CAS bind their lock, snapshot,
and write destination to the exact requested account slot. Embedded `integration`
metadata is descriptive and never selects another account. Both base-plus-alias
lookups and direct alias-qualified handles therefore resolve and materialize the
same slot without allowing copied metadata to mutate its original account.

Refresh does not hold the credential lock during provider network I/O. It keeps
the complete raw backend snapshot it started from (distinct from the normalized
view returned to callers), then acquires the account lock,
re-reads the authoritative backend, and compare-and-stores the refresh response
only when that snapshot is unchanged, using the normal credential backend
selection and fallback policy. A concurrent connect, import, or rotation wins;
the stale refresh returns a conflict asking the caller to retry and never
overwrites the newer credential.

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

`aware --json credential capabilities` is the fail-closed discovery surface for callers
that provision generic local-agent credentials. It returns the bare JSON object:

```json
{"schemaVersion":1,"capabilities":["secret.put.v1","secret.revoke.v1"]}
```

`secret.put.v1` means the canonical `credential put <handle>` contract above: raw opaque
secret bytes arrive through stdin by default, never argv, and success returns only
`status` plus the resolved `handle`. `secret.revoke.v1` means the canonical idempotent
`credential delete <handle>` contract. The identifiers describe semantic capabilities;
they do not introduce a second `secret` command group. Callers must check the exact
identifiers rather than infer support from `aware --version`.

### `aware doctor`

Health check. No mutations. Credential inspection does not create lock files,
rewrite legacy credentials, normalize stored bytes, or materialize generation
metadata; a legacy credential therefore reports `generation: null`. Useful
before filing a bug.

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

### `aware provider trust-publisher|enroll|select|list`

These operator commands establish a local, format-neutral trust boundary for model-provider
packages. Package manifests use `aware.model-provider-package/v1`, closed canonical JSON and an
Ed25519 signature over the manifest SHA-256. They declare opaque package, format and capability IDs,
protocol version, source-capture/request/result/artifact/cache contract IDs, one relative launcher,
an AWARE compatibility range and a complete file allowlist with byte counts and SHA-256 receipts.

Enrollment requires an absolute regular directory, a previously trusted publisher and an exact
inventory containing only the manifest, signature and receipted files. Links, reparse points, extra
files, unsafe relative paths, duplicate capabilities/files, incompatible versions, signature failure
and receipt drift are refused. Selection binds an opaque format to an exact enrolled manifest digest
and increments a local generation while retaining at most eight prior digests.

`list --json` exposes package ID/version, format ID, manifest/publisher digests, declared capabilities
and selection status. It never exposes the package root, launcher, file allowlist or provider output.
The `model-reference-reader` protocol-v3 path consumes the same selected record and brackets every
provider invocation with complete package re-verification. A caller must supply a non-empty,
bounded opaque `provider-authorization` value for `preflight`, `fingerprint-source`, `probe`,
`read-model` and `read-snapshot`. AWARE passes that value to the provider unchanged. It has no key,
endpoint or rule that can mint, refresh or interpret the authorization, and it fails before launch
when the value is absent. This is a host authorization boundary, not an AWARE entitlement system.

Protocol v3 captures ordered source namespaces into an immutable private closure, invokes
`discover`, applies the admitted dependency policy, invokes the package's read-only `convert`
operation, and admits only its closed output inventory. Entity, property and relationship JSONL
records are schema-checked, canonicalized, externally sorted and sharded with unique identities and
referential integrity. Geometry is admitted as one or more contiguous GLB tiles; entity ownership and
bounds must name an admitted tile. AWARE signs and publishes a `model-reference-manifest/v2` root,
four family indexes and content-addressed objects. Cancellation or failure before root publication
leaves no visible root.

For protocol-v3 conversion, a provider may refuse incomplete geometry coverage by exiting nonzero
with empty stdout and an exact canonical JSON stderr object containing only `code`, `phase`,
`retryable`, `message` and `diagnosticId`. The reader recognizes only
`reference-model-coverage-incomplete` with `phase: "conversion"`, `retryable: false`, bounded text
and a UUID diagnostic ID. It forwards the code through its own safe error envelope, but replaces
provider-controlled text and diagnostic ID with reader-owned values. Malformed, oversized or
unrecognized stderr remains the generic `reference-provider-failed` error. No partial model is
published in either case.

The compatibility branches are explicit and non-negotiated:

| Provider protocol | Commands | Reader request | Artifact contract |
|---|---|---|---|
| `1` local file | `preflight`, `probe`, `read-model`, `read-snapshot` | existing v1/v2 bytes | frozen five-object legacy package |
| `2` managed cloud | same | existing v2 bytes plus conversion attempt | frozen five-object legacy package |
| `3` enrolled source set | all of the above plus `fingerprint-source` | explicit `model-reference-reader/v3`, package identity, namespaces and opaque authorization | indexed `model-reference-manifest/v2` CAS root |

There is no fallback between protocol 3 and either legacy protocol. Missing v3 fields refuse before
provider I/O; legacy request serialization, cache identity, errors and artifacts remain unchanged.

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
