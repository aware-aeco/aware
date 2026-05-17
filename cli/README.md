# `aware` CLI

The runtime binary for the AWARE substrate.

This folder houses the Rust implementation. The contract it satisfies is in [`10-core/cli-spec.md`](../10-core/cli-spec.md); the phased delivery plan is in [`10-core/cli-roadmap.md`](../10-core/cli-roadmap.md); the engineering rules that apply across every session are in [`CLAUDE.md`](../CLAUDE.md) at the repo root.

## Install

AWARE ships five install paths. Pick by what's already on your machine and what your IT department allows. **For enterprise rollouts where IT controls software installs, prefer the npm or curl-pipe paths** — they don't trigger the SmartScreen warning that blocks unsigned MSIs.

### 1. Node package manager (recommended — Windows / Linux / macOS)

```bash
npm  install -g @aware-aeco/cli      # npm
pnpm add     -g @aware-aeco/cli      # pnpm
yarn global add  @aware-aeco/cli     # yarn (classic)
bun  install -g @aware-aeco/cli      # bun

aware --version
```

All four read from the same npm registry and run the same `postinstall` script (Node built-ins only — no manager-specific deps). Pick whichever you already have. The package itself is ~2 KB; the `postinstall` fetches the platform binary (~10 MB) from the [GitHub Release](https://github.com/aware-aeco/aware/releases) matching the version.

**Why this is the IT-friendly path:** npm/pnpm/yarn/bun are already on most developer workstations and don't require admin rights. No SmartScreen warning, no `--installer-allowed` policy override.

### 2. Curl-pipe (Linux / macOS, no Node required)

```bash
curl -fsSL https://raw.githubusercontent.com/aware-aeco/aware/main/scripts/install.sh | bash
```

Drops the binary at `~/.local/bin/aware` (or `/usr/local/bin/` if writable). No package-manager dependency.

### 3. PowerShell (Windows, no Node required)

```powershell
iex (irm https://raw.githubusercontent.com/aware-aeco/aware/main/scripts/install.ps1)
```

Drops the binary at `%LOCALAPPDATA%\aware\bin\aware.exe` and adds it to the user PATH. No admin rights needed.

### 4. MSI installer (Windows enterprise)

Download `aware-<version>-win-x64.msi` from the [latest GitHub Release](https://github.com/aware-aeco/aware/releases/latest) and run it. Installs system-wide to `C:\Program Files\AWARE\` and adds itself to the system PATH. Uninstall via Apps & Features or `msiexec /x aware-<version>-win-x64.msi`.

**SmartScreen warning:** the MSI is currently unsigned (open-source project; signing an MSI requires a paid Authenticode cert). On first run Windows will warn "unknown publisher" — click *More info → Run anyway*, or pre-stage the cert hash via IT policy. **If your IT department blocks unsigned MSIs at the gate, use path #1 (npm) instead** — it bypasses MSI entirely.

Silent install for IT departments:

```pwsh
msiexec /i aware-<version>-win-x64.msi /qn
```

### 5. From source

```bash
git clone https://github.com/aware-aeco/aware
cd aware
cargo build --release --manifest-path cli/Cargo.toml
dotnet publish cli-sidecar -c Release -r <rid> -p:PublishAot=true
```

### Pinned version

- npm / pnpm / yarn / bun: append `@0.22.0` to the package name (e.g. `pnpm add -g @aware-aeco/cli@0.22.0`)
- curl: `... | bash -s -- --version 0.22.0`
- PowerShell: `$env:AWARE_VERSION = "0.22.0"; iex (...)`

### Tracked as follow-up

- Submit winget manifest to [`microsoft/winget-pkgs`](https://github.com/microsoft/winget-pkgs) (requires a code-signed MSI — deferred; open-source projects routinely ship unsigned and the npm path covers most users)
- Homebrew formula — `brew install aware-aeco/tap/aware`
- ARM Linux (linux-arm64) + Intel macOS (osx-x64)

## Authentication (`aware connect`)

Four flows for getting credentials in. Use the lowest-friction one your provider supports.

| Flow | Flag | When to use |
|---|---|---|
| Browser-paste | `aware connect <integration>` (default) | First-time setup, no OAuth app registered yet. CLI opens the provider's authorize URL; user grants, copy-pastes the access token back. |
| PKCE | `--oauth` | Production. Loopback redirect, no manual paste. Requires `AWARE_OAUTH_<INTEGRATION>_CLIENT_ID` env var pointing at a registered OAuth app. |
| **Device code** (v0.13) | `--device-code` | Headless / IT-managed / restricted-browser workstations. CLI prints a URL + short code; user opens it on any device (phone, separate machine), enters the code, CLI polls until authorization completes. RFC 8628. |
| Token paste | `--from-file <path>` or `--from-env` | Service accounts / CI. Token already obtained out-of-band. |

**Tenant-restricted M365 tenants:** add `--tenant <tenant-id-or-domain>` to substitute the Microsoft `common` endpoint with a tenant-specific one. Required by IT departments that block cross-tenant identity.

```bash
# Standard:
aware connect microsoft-365 --oauth

# Tenant-restricted (M365):
aware connect microsoft-365 --device-code --tenant acme.onmicrosoft.com
```

Tokens encrypt at rest via the OS keychain (Windows DPAPI / macOS Keychain / Linux libsecret).

## Status

**v0.6.0 — distribution release.** The `aware` binary is now installable
via a single curl command on Linux/macOS or a PowerShell one-liner on
Windows. A GitHub Actions release workflow cross-builds for win-x64,
linux-x64, and osx-arm64 on every `v0.6.*` tag and publishes signed
archives to GitHub Releases.

[Full surface list — see prior v0.5 status; nothing new added at the CLI level.]

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
- `aware build agent --from-com <progid>` ← v0.5.2 (via C# sidecar)
- `aware build agent --from-headers <path>` ← v0.5.2 (via C# sidecar)
- `aware skill create <agent> <skill>` ← v0.5
- `aware skill port <source> <target-agent>` ← v0.5
- `aware skill modify <agent> <skill>` ← v0.5
- `aware skill eval <agent> <skill>` ← v0.5

All phases of `cli-roadmap.md` v0.1–v0.5.2 are shipped.

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

**Tier C — fully implemented via C# sidecar (v0.5.1 + v0.5.2):**

```bash
# .NET DLL reflection (v0.5.1) — local install, escape hatch
aware build agent --from-dlls "C:/Program Files/Tekla/bin/*.dll"

# COM TypeLib introspection — Windows only (v0.5.2)
aware build agent --from-com AutoCAD.Application

# C++ header parsing via clang (v0.5.2; free functions only)
aware build agent --from-headers "/usr/include/foo/*.h"

# Decompile + reflect NuGet package (v0.5.1; needs ilspycmd)
aware build agent --decompile --from-nuget Some.Closed.Pkg@1.0.0
```

**Prefer NuGet `--from-nuget` over `--from-dlls` when both are an option.**
NuGet packages with `ref/<tfm>/` ship vendor-curated **reference
assemblies** — metadata-only DLLs that contain only the published
public-API contract, no internals, no method bodies. The aware builder
prefers `ref/` over `lib/` automatically. Local install DLLs work too
(`--from-dlls`) but reflect from the runtime DLLs, which include
internal types the vendor never intended as a contract. Use
`--from-dlls` when the product isn't on NuGet, when the NuGet version
lags the installed one, or when you need a specific local build.

All four tier-C sources invoke the `aware-sidecar` companion binary. See the
sidecar section below for install and build instructions.

### The C# sidecar (v0.5.1 — v0.5.2)

For `--from-dlls`, `--from-com`, `--from-headers`, and `--decompile`, the Rust
CLI invokes a companion binary called `aware-sidecar.exe` (Windows) /
`aware-sidecar` (Linux). The sidecar is a self-contained C# NativeAOT binary
(~8 MB) that handles .NET reflection (via `MetadataLoadContext`), COM TypeLib
introspection (via Win32 P/Invoke), and C++ header parsing (via shelling to
`clang`).

**Install:** drop the binary alongside `aware.exe` in any release tarball,
or set `AWARE_SIDECAR=<path>`.

**Build from source:**

```bash
cd cli-sidecar
# Windows
dotnet publish -c Release -r win-x64 -p:PublishAot=true
# Linux
dotnet publish -c Release -r linux-x64 -p:PublishAot=true
# macOS (arm64)
dotnet publish -c Release -r osx-arm64 -p:PublishAot=true
```

**Per-source extras:**

- `--decompile` needs `ilspycmd`: `dotnet tool install -g ilspycmd`
- `--from-headers` needs `clang` on PATH:
  - Linux: `apt install clang` / `dnf install clang`
  - macOS: included with Xcode CLT
  - Windows: install LLVM from https://releases.llvm.org/

**Platform caveats:**

- `--from-com` is Windows-only.
- `--from-headers` covers free functions and public C++ class methods. Templates emit a
  "skipping template" warning to stderr; static class methods are filtered out.
- `--decompile` skills carry a `next-action` frontmatter field pointing at
  `aware skill modify <agent> <skill>` — open the skill in your agentic CLI and
  ask the AI to summarize the raw decompiled output into a normal AWARE skill.

**Tracked for v0.5.4+:**

- C++ templates and operator overloading for `--from-headers`
- Static class methods (currently filtered out)
- ARM Linux (linux-arm64) and Intel macOS (osx-x64) sidecar builds

**IPC protocol** for sidecar contributors: see [cli-sidecar/README.md](../cli-sidecar/README.md).

### The `next-action` skill convention

Skills generated by `--decompile` now include a `next-action:` frontmatter field
pointing at a follow-up `aware` command. Today only decompile uses it; future
agent generators may emit similar hints. The skill loader ignores unknown
frontmatter fields, so this is forward-compatible with v0.1+ tooling.

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
| Single static binary | Works on every OS without a runtime. Today: npm wrapper + curl-pipe + PowerShell; queued: MSI / winget / brew. |
| Cross-platform native | Windows-first matters for AECO; Rust's Windows story is excellent. |
| Decalog alignment | "No vendor in the loop." A statically-linked binary outlives its build environment. |
| Type safety for manifests | `serde` deserializes YAML → typed structs; schema bugs are compile errors. |
| Async runtime | `tokio` handles the long-running stateful-agent case cleanly (v0.3). |
| Distribution | `cargo install` for devs; `npm install -g @aware-aeco/cli` (or pnpm/yarn/bun) + curl-pipe for users. `winget` / `brew` queued behind MSI + Homebrew formula. |

If `cargo build` ever takes too long, consider splitting the runtime (`v0.3`) into a separate crate.
