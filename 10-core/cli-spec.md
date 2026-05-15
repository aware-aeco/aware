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
│   ├── describe <agent>                manifest summary + skill index
│   ├── skill <agent> <skill-name>      print a skill's content
│   ├── install <agent>[@version]       fetch from registry or local path
│   ├── uninstall <agent>
│   ├── update <agent>                  re-pull latest matching version
│   ├── validate <path>                 schema + structure checks
│   └── publish <path>                  open a PR to the GitHub registry
│
├── app ...                             manage installed apps
│   ├── list                            print installed apps (table)
│   ├── show <app>                      print topology (ASCII) + provenance
│   ├── install <path-or-name>          register an app
│   ├── uninstall <app>
│   ├── run <app> [--instance <id>] [--config <kv>...]      execute
│   ├── stop <app> [--instance <id>]    stop a running app
│   ├── validate <path>                 schema + cycle + cap checks
│   ├── export <app> <output-path>      copy .flo file out
│   └── logs <app> [--instance <id>] [--tail]   read execution traces
│
├── connect <integration>               provision OAuth credentials (PKCE)
│   ├── --as <alias>                    named credential for multi-account
│   ├── --refresh                       force refresh
│   └── --scopes <s1,s2>                add scopes (Google etc.)
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
│   ├── --decompile                     opt-in, license-checked
│   ├── --tier-strategy <auto|all-1|all-2>
│   └── --output <agent-id>
│
└── doctor                              health check — config, creds, hosts, registry
```

## Filesystem layout (what `aware` reads / writes)

```
~/.aware/
├── config.yaml                         # user config (default editor, default prompts, etc.)
├── agents/
│   └── <agent-id>/                     # installed agent (manifest + skills + commands)
├── apps/
│   └── <app-id>/                       # installed app
│       ├── <app-id>.flo
│       ├── lockfile.yaml               # pinned agent versions resolved at install
│       └── instances/<id>/state/       # per-instance state (stateful apps)
├── credentials/                        # encrypted; OS keychain on Mac/Linux, DPAPI on Windows
│   ├── trimble-connect.json
│   ├── microsoft-365.json
│   └── google-workspace.<alias>.json
├── permissions/
│   └── <app-id>.yaml                   # user's Allow / Always-allow / Deny decisions
├── logs/
│   └── <app-id>/<instance-id>/<run-id>.jsonl    # provenance trail per run
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

The heaviest command. Loads the `.flo`, resolves agent dependencies via the lockfile, starts any stateful agents, wires connections, and either:
- Returns immediately (one-shot app with only stateless nodes)
- Blocks until stopped (long-running app with stateful nodes)

Output streams to stdout per run; full trace written to `~/.aware/logs/<app>/<instance>/<run>.jsonl`.

```
$ aware app run welded-to-tc --instance fab-east \
    --config tc-project-id="...." \
    --config tc-folder-id="...."

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

## Out of scope for the CLI itself

These belong to other components, not the CLI binary:

- **The FloLess visual canvas.** Reads the same files (`~/.aware/apps/*.flo`, agent manifests). Separate executable, separate license.
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
