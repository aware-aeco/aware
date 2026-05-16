# `aware` CLI

The runtime binary for the AWARE substrate.

This folder houses the Rust implementation. The contract it satisfies is in [`10-core/cli-spec.md`](../10-core/cli-spec.md); the phased delivery plan is in [`10-core/cli-roadmap.md`](../10-core/cli-roadmap.md); the engineering rules that apply across every session are in [`CLAUDE.md`](../CLAUDE.md) at the repo root.

## Status

**v0.5.1 — C# NativeAOT sidecar** lifts v0.5's tier-C stubs. Twenty-seven user-facing
surfaces; `--from-dlls` and `--decompile` now work via the `aware-sidecar` companion
binary (`--from-com` and `--from-headers` still stubbed for v0.5.2+).

- `aware --version` / `aware --help`
- `aware doctor` (CLI / Filesystem / Agents / Apps / Integrity / Running / Credentials)
- `aware agent list` / `agent describe <id>` / `agent skill <id> <skill>`
- `aware agent install <path|id[@version]|bundle>` / `agent uninstall <id>` / `agent update <id>` / `agent validate <path>`
- `aware app list` / `app show <id>`
- `aware app install <path>` / `app uninstall <id>` / `app validate <path>` / `app export <id> <output>`
- `aware app run <app>` / `app stop <app>` / `app logs <app> [--tail | --replay]`
- `aware connect <integration>` (browser-paste default, OAuth via `--oauth`) / `disconnect <integration>` / `plugins regenerate`
- `aware build agent --from-openapi <url-or-path>` ← v0.5 tier A
- `aware build agent --from-cli <binary>` ← v0.5 tier A
- `aware build agent --from-nuget <pkg>@<version>` ← v0.5 tier B (docs-only, no decompile yet)
- `aware build agent --from-python <module>` ← v0.5 tier B
- `aware build agent --from-dlls <path>` ← v0.5.1 (via C# sidecar)
- `aware build agent --decompile --from-nuget <pkg>@<version>` ← v0.5.1 (via C# sidecar)
- `aware build agent --from-com <progid>` ← v0.5.2 stub
- `aware build agent --from-headers <path>` ← v0.5.2 stub
- `aware skill create <agent> <skill>` ← v0.5
- `aware skill port <source> <target-agent>` ← v0.5
- `aware skill modify <agent> <skill>` ← v0.5
- `aware skill eval <agent> <skill>` ← v0.5

All phases of `cli-roadmap.md` v0.1–v0.5.1 are shipped.

### Connect to an integration (v0.4)

**Default: browser-based token paste.** No OAuth-app registration required.

```bash
aware connect trimble-connect
```

This opens a tab in your browser with a password-style form. Get a token from the
provider's website (Trimble Connect / Microsoft 365 / Google Workspace), paste it
into the form, click Save. The token is encrypted to your OS keychain.

**Why a browser form?** When you're using `aware` from inside Claude Code / Codex /
other AI chat tools, pasting a token directly into the chat would leak it into the
conversation. The browser form is a side-channel — the token only ever exists
in your browser tab and the local keychain. It never flows through any AI session.

**Non-interactive paths** (CI, scripts, headless environments):

```bash
# Read from a file you prepared outside the chat
aware connect trimble-connect --from-file ./trimble-token.txt

# Read from an env var (set in your shell or CI secrets)
$env:AWARE_TOKEN_TRIMBLE_CONNECT = "tk_xyz"
aware connect trimble-connect --from-env
```

The `--from-file` path accepts either a plain bearer token (one line) or a JSON
blob with at least an `access_token` field.

**OAuth flow** (advanced — requires AWARE-AECO or you to have registered an
OAuth app at the provider's developer console):

```bash
$env:AWARE_OAUTH_TRIMBLE_CLIENT_ID = "your-registered-client-id"
aware connect trimble-connect --oauth
```

This opens the provider's sign-in page (the `gh auth login` / `az login` UX) and
exchanges the result for a token automatically.

**Multi-account support:**

```bash
aware connect google-workspace --as personal
aware connect google-workspace --as work
```

**Refresh and remove:**

```bash
aware connect trimble-connect --refresh   # OAuth tokens only
aware disconnect trimble-connect
```

**Doctor displays the source:**

```
Credentials:
  ✓ trimble-connect       valid    paste token (user-managed)
  ✓ google-workspace      valid    OAuth, expires in 41m
  ✗ microsoft-365         missing  run: aware connect microsoft-365
```

**Encryption.** Tokens are encrypted at rest by the OS keychain (Windows DPAPI / macOS Keychain / Linux libsecret) under service `aware-aeco`. Never written to logs.

**Refresh.** When the runtime reads a credential, it lazily refreshes if the token expires within 60 seconds. Transparent.

### Host plugins (v0.4)

The `aware-aeco` plugin bundle is auto-generated from installed agents whenever you run `agent install` or `agent uninstall`. It lands under:

- `~/.claude/plugins/aware-aeco/` (first-class — full plugin manifest + per-command markdown)
- `~/.codex/plugins/aware-aeco/` (scaffold — format pending)
- `~/.opencode/plugins/aware-aeco/` (scaffold — format pending)

Force a rebuild anytime:

```bash
aware plugins regenerate
```

Regeneration is idempotent — re-running produces byte-identical output, so you can safely version-control the plugin directories.

For non-default plugin locations (CI, test fixtures, custom setups), override:

```
$env:AWARE_PLUGINS_CLAUDE   = "C:\test\.claude\plugins"
$env:AWARE_PLUGINS_CODEX    = "C:\test\.codex\plugins"
$env:AWARE_PLUGINS_OPENCODE = "C:\test\.opencode\plugins"
```

### Build an agent (v0.5)

`aware build agent` generates an AWARE agent folder under `~/.aware/agents/<id>/`
by reading an existing software product's API surface. Four sources are fully
implemented in v0.5; four more are stubbed pending the v0.5.1 C# NativeAOT
sidecar (which handles .NET reflection cleaner than Rust subprocess shims).

**Tier A — full implementations:**

```bash
# OpenAPI spec (JSON or YAML; URL or local file)
aware build agent --from-openapi https://petstore.swagger.io/v2/swagger.json

# CLI binary (parses `<binary> --help` for subcommands)
aware build agent --from-cli git --output git-agent
```

**Tier B — partial implementations:**

```bash
# NuGet package (fetches the .nupkg, extracts XML docs as skills;
# license-checked — non-permissive licenses need --accept-license)
aware build agent --from-nuget Tekla.Structures.Model@2025.0.0

# Python module (introspects via `python -c`)
aware build agent --from-python requests
```

**Tier C — stubbed for v0.5.1** (will return a clear "needs sidecar" error):

```bash
aware build agent --from-dlls "C:/Program Files/Tekla/bin/*.dll"
aware build agent --from-com AutoCAD.Application
aware build agent --from-headers "/usr/include/foo/*.h"
aware build agent --decompile --from-nuget Some.Closed.Pkg@1.0.0
```

The four tier-C sources need the v0.5.1 C# NativeAOT sidecar binary that
handles .NET reflection / COM type libraries / C++ header parsing more cleanly
than pure-Rust subprocess shims. Tracked separately.

### The C# sidecar (v0.5.1)

For `--from-dlls` and `--decompile`, the Rust CLI invokes a companion binary
called `aware-sidecar.exe`. The sidecar is a self-contained C# NativeAOT binary
(~8 MB) that handles .NET-heavy work (reflection via `MetadataLoadContext`,
decompilation via `ilspycmd`) which is awkward to do cleanly in pure Rust.

**Install:** drop `aware-sidecar.exe` alongside `aware.exe` in any release tarball,
or set `AWARE_SIDECAR=<path>` to point at it explicitly.

**Build from source:**

```bash
cd cli-sidecar
dotnet publish -c Release -r win-x64 -p:PublishAot=true
# → bin/Release/net9.0/win-x64/publish/aware-sidecar.exe
```

**For `--decompile`:** the sidecar also needs `ilspycmd` on PATH:

```bash
dotnet tool install -g ilspycmd
```

**Tracked for v0.5.2:**

- `--from-com` (Windows COM TypeLib interop in the sidecar)
- `--from-headers` (libclang via P/Invoke or `clang.exe` shell)
- Linux + macOS sidecar binaries (`dotnet publish -r linux-x64` / `osx-arm64`)

**IPC protocol** for sidecar contributors: see [cli-sidecar/README.md](../cli-sidecar/README.md).

### Skill commands (v0.5)

Skill scaffolding + porting + a trigger-corpus eval matcher. AI-driven content
generation is delegated to your own agentic CLI (Claude Code, Codex, etc.) —
AWARE provides templates, file moves, and a lightweight eval; it doesn't bundle
an Anthropic SDK.

```bash
# Scaffold a new skill template (frontmatter + TODOs)
aware skill create tekla my-new-skill

# Port a skill between agents (adapts frontmatter name: field)
aware skill port tekla/drawing-identity trimble-connect

# Open a skill in $EDITOR (falls back to notepad on Windows / vi elsewhere)
aware skill modify tekla drawing-identity

# Run a trigger-match eval
#   creates a <skill>.eval.md template on first run; edit it with trigger
#   phrases, then re-run to see match rate
aware skill eval tekla drawing-identity
```

## Build

```bash
cd cli
cargo build                          # debug → target/debug/aware
cargo build --release                # release → target/release/aware
```

First build pulls ~30 crates (clap, serde, tokio, etc.). Subsequent builds are incremental.

## Run

```bash
# Default reads ~/.aware/. Override via AWARE_HOME for testing:
export AWARE_HOME=$(pwd)/test-fixtures/aware

# Read-only (v0.1)
aware doctor                              # filesystem + integrity health check
aware agent list                          # table of installed agents
aware agent describe tekla
aware agent skill tekla drawing-identity
aware app list
aware app show welded-to-tc
aware --json agent list

# Install + validate (v0.2)
aware agent install ./20-agents/aeco/engineering/tekla       # local path
aware agent install tekla                                    # from registry
aware agent install aware-aeco                               # bundle: all 7 agents
aware agent uninstall tekla
aware agent update tekla
aware agent validate ./my-agent
aware app install ./my-app                                   # writes lockfile.yaml
aware app validate ./my-app
aware app export welded-to-tc /tmp/exported.flo
```

### Run an app (v0.3)

```bash
# One-shot app (all nodes stateless): runs to completion, exits
aware app run my-oneshot-app

# Long-running app (any stateful node like a watcher): blocks until Ctrl+C
aware app run welded-to-tc --instance fab-east

# Inputs and config overrides
aware app run welded-to-tc --config tc-project-id=proj-123 --config tc-folder-id=folder-456

# Stop a long-running app from another terminal
aware app stop welded-to-tc --instance fab-east

# Read execution traces
aware app logs welded-to-tc                       # raw JSONL of most recent run
aware app logs welded-to-tc --tail                # follow live
aware app logs welded-to-tc --replay              # pretty-printed replay of most recent run
aware app logs welded-to-tc --replay --run-id r_abc123   # specific run
```

The runtime invokes each agent's CLI transport binary by name (from `transport.cli.binary`).
Agents generated by `aware build agent` produce runnable transport binaries; hand-authored agents
need their own binary on PATH. The orchestrator logic is tested with in-process mocks under the hood.

### Templating

Node configs support Jinja-style substitution:

- `{{ inputs.X }}` — inputs passed via `--config` or app-level config
- `{{ <node-id>.<output-field> }}` — upstream outputs (kebab-case node ids are auto-translated to underscore; use bracket syntax for raw kebab access: `{{ upstream["my-node"].field }}`)
- `{{ secrets.<id> }}` — credentials from `~/.aware/credentials/<id>.json`
- `{{ config.X }}` — app-level config kv from `~/.aware/apps/<app>/config.yaml`

### Provenance

Every run writes a JSONL trace to `~/.aware/logs/<app>/<instance>/<run-id>.jsonl` with events for
`run-start`, `node-start`, `node-output`, `node-error`, `node-stop`, `run-end`. Replay them at any time
with `aware app logs ... --replay`.

By default the CLI reads the registry from
`https://raw.githubusercontent.com/aware-aeco/aware/main/registry-index.json`.
Override with `AWARE_REGISTRY=<https-or-file-url>` for testing or alternative registries.

## Test

```bash
cargo test                           # unit + integration
cargo test -- --nocapture            # show println output
```

## Lint + format

```bash
cargo fmt --all                      # mandatory before commit
cargo clippy --all-targets -- -D warnings
```

Both must pass with zero diff / warnings before merge. CI enforces this once GitHub Actions is set up.

## Layout

```
cli/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs                      # Clap entry + dispatch
│   ├── error.rs                     # AwareError + exit-code mapping
│   └── commands/
│       ├── mod.rs
│       ├── agent.rs                 # aware agent ...
│       ├── app.rs                   # aware app ...
│       ├── connect.rs               # aware connect / disconnect
│       ├── skill.rs                 # aware skill ...
│       ├── build.rs                 # aware build ...
│       └── doctor.rs                # aware doctor
└── tests/
    └── basic.rs                     # CLI smoke tests via assert_cmd
```

## Engineering rules

See [`CLAUDE.md`](../CLAUDE.md). Non-negotiable:

- Verify before answering (read sources, not summaries)
- No corner-cutting (if a source exists, open it)
- Coding is solved (no "can't be done" as first response)
- All skill work routes through Anthropic's `skill-creator`
- Decalog is the design tiebreaker
- `cargo fmt` + `cargo clippy -D warnings` before commit

## Why Rust

| Reason | |
|---|---|
| Single static binary | Works on every OS without a runtime. `winget install aware-aeco`, done. |
| Cross-platform native | Windows-first matters for AECO; Rust's Windows story is excellent. |
| Decalog alignment | "No vendor in the loop." A statically-linked binary outlives its build environment. |
| Type safety for manifests | `serde` deserializes YAML → typed structs; schema bugs are compile errors. |
| Async runtime | `tokio` handles the long-running stateful-agent case cleanly (v0.3). |
| Distribution | `cargo install` works for devs; `winget` / `brew` / `curl-pipe` for users. |

If `cargo build` ever takes too long, consider splitting the runtime (`v0.3`) into a separate crate.
