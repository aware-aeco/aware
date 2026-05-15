# Agent Spec

How an agent works in AWARE — the contract every agent satisfies, whether hand-written, auto-generated, or synthesized from an app.

This document defines what an agent *is*. The [App Spec](./app-spec.md) defines how agents compose into apps.

---

## What an agent is

An **agent** is a unit of capability — something an app can call. There are three ways an agent comes into existence:

1. **Hand-written.** A contributor authors the manifest, skills, and commands directly.
2. **Auto-generated.** [`aware agent build`](../20-agents/_core/aware-agent-builder/) reflects a DLL, parses an OpenAPI spec, decompiles a NuGet package, or introspects a CLI — and emits a full agent folder.
3. **Synthesized.** An app marked `exposes-as-agent: true` appears in the registry as an agent. Its commands run the app's internal topology.

**All three produce the same shape.** Callers don't need to know which kind they're invoking.

---

## Anatomy

Every agent lives in a folder with this layout:

```
<agent-name>/
├── manifest.yaml             # required — capability declaration
├── skills/                   # required — at least one skill file
│   ├── <topic-1>.md
│   └── <topic-2>.md
├── commands/                 # required — one file per command
│   ├── <command-1>.md
│   └── <command-2>.md
└── runtime/                  # optional — how this agent actually executes
    ├── cli/                  #   default transport: a CLI binary
    └── mcp-server.json       #   optional: MCP server config
```

The folder name = the agent's id. Use kebab-case. Avoid generic names; prefer `tekla` over `cad-tool`, `trimble-connect` over `bim-platform`.

---

## manifest.yaml

The capability declaration. This is the source of truth for everything the registry, CLI, and orchestrator need to know.

```yaml
agent:        tekla                # required · the id (matches folder name)
version:      2025.0.1             # required · semver
display-name: Tekla Structures     # optional · human-readable for UIs
description: |                     # required · one paragraph
  Watches the active Tekla model and exposes Tekla Open API commands.
  Auto-generated from Tekla Structures 2025.0 DLLs.

# Operational shape
stateful: true                     # required · true|false (see "Stateful vs Stateless")

# Cataloguing
vendor: trimble                    # optional · for grouping/filtering
license: MIT                       # required · SPDX identifier
homepage: https://github.com/aware-aeco/agents/tekla
keywords: [aeco, engineering, structural, bim]

# Provenance (set automatically by aware agent build)
provenance:
  generated-by: aware-agent-builder
  generator-version: 0.1.0
  source:
    type: dlls                     # dlls | nuget | openapi | com | cli | headers | python | hand-written
    path: C:/Program Files/Tekla Structures/2025.0/bin/Tekla.*.dll
  generated-at: 2026-05-15T10:23:00Z

# Capabilities this agent requires at runtime
requires:
  filesystem:
    - read: ~/.aware/credentials/tekla.json
  network:
    - localhost:9999               # Tekla COM bridge
  software:
    - tekla-structures@2025.x      # version constraint on host software

# Transport — how the agent is invoked
transport:
  cli:                             # required for non-app agents
    binary: aware-tekla            # name of the CLI binary
  mcp:                             # optional · for non-CLI hosts
    server: aware-tekla-mcp
    transport: stdio

# Commands the agent exposes
commands:
  watch:
    lifecycle: start               # start | stop | single
    description: Subscribe to ModelObjectChanged events
    inputs: {}
    outputs:
      type: stream                 # stream | single
      schema:
        mark:     string
        type:     string
        geometry: object

  insert:
    lifecycle: single
    description: Create a ConnectionPart in the active model
    inputs:
      connection:
        type: object
        ref: ./schemas/connection.json
    outputs:
      type: single
      schema:
        guid: string

# Skills — discovered automatically from skills/*.md
skills:
  - drawing-identity.md
  - event-threading.md
  - coordinate-systems.md
```

### Required fields

`agent`, `version`, `description`, `stateful`, `license`, `transport`, `commands`. Everything else is optional.

---

## Skills

Skills are **plain markdown files** under `skills/`. They are dual-use:
- **Humans** read them to learn the agent.
- **The AI** reads them as context when composing.

There is no special format beyond standard markdown. Conventions:

- **One concern per file.** `drawing-identity.md` covers identity rules; `event-threading.md` covers async/sync behavior. Don't bundle.
- **Lead with the rule, then explain.** *"Drawing identity = Mark, NOT Name. Drawing.Name can repeat..."*
- **Cite sources** when the knowledge comes from a vendor doc. *"Source: developer.tekla.com — verified per release."*
- **Encode judgment, not just facts.** *"For sheets &lt; 10k rows, build an in-memory dictionary. Larger sheets use MATCH/INDEX via COM."*

A skill is good when an AI reading it would compose better code than an AI without it.

---

## Commands

Commands are documented in `commands/*.md`. Each command file describes:

- What the command does (one paragraph)
- Input schema with examples
- Output schema with examples
- Lifecycle (`start`, `stop`, `single`)
- Failure modes and idempotency notes
- Example invocation (CLI / MCP form)

The same information is declared in `manifest.yaml` under `commands:`. The manifest is the machine-readable source; the markdown is the human-readable elaboration.

---

## Stateful vs Stateless

Declared in `manifest.yaml` as `stateful: true|false`.

| Flag | Meaning | Lifecycle |
|---|---|---|
| `stateful: false` | Each call is independent. No memory between calls. | Commands are `single`. |
| `stateful: true` | Holds open connections, subscriptions, or cached state. Must be started and stopped. | Commands include `start` and `stop` lifecycles; may emit events between. |

**Rule of thumb:** agents that *watch / subscribe / listen* are stateful. Agents that *do / call / send / transform* are stateless.

A stateful agent's `start` command may emit a `stream` output (events flow until `stop` is called). A stateless agent's commands always emit `single` outputs (one response per call).

---

## Transport

Required: at least one transport. Optional: more than one.

| Transport | When to use | Format |
|---|---|---|
| `cli` | Default. Works in any agentic CLI (claude-code, codex, opencode). | A binary in `runtime/cli/` + a stable JSON response envelope. |
| `mcp` | When the agent needs to work in non-CLI hosts (Claude Desktop, Cursor) or wants streaming/server-pushed events. | An MCP server config in `runtime/mcp-server.json`. |
| `rest` | When the underlying tool is already a REST API and a REST shim is simpler than a CLI. | An OpenAPI spec or hand-written shim. |

The contract every transport satisfies: **structured input → structured output, with errors as data, not exceptions.** The CLI envelope is documented in [`response-envelope.md`](./response-envelope.md) (forthcoming).

---

## Capabilities and permissions

Declared in `manifest.yaml` under `requires:`. The user is prompted at first install (Claude-style — *"Allow once / Always allow / Deny"*). Approvals are stored in `~/.aware/permissions/`.

Categories:
- `filesystem` — read/write paths
- `network` — hosts/ports
- `software` — host software with version constraints
- `secrets` — credentials by service id (cross-references `~/.aware/credentials/<id>.json`)

If an agent tries to touch something it didn't declare, the orchestrator blocks it and warns the user. **No silent escalation.**

---

## Versioning

- **Semver.** `<major>.<minor>.<patch>`.
- **Auto-generated agents** version-track their source. `tekla@2025.0.1` mirrors `Tekla Structures 2025.0` (patch is the agent's own iteration).
- **Apps pin agent versions** by `agent@minor.x` (default), `agent@major.x` (loose), or `agent@exact-semver` (strict). See [App Spec](./app-spec.md).
- **Breaking changes** require a major bump *and* a `BREAKING.md` file in the agent folder describing what moved.

---

## Installation

```bash
aware agent install tekla                # latest
aware agent install tekla@2025.0.1       # exact
aware agent install tekla@2025.0.x       # minor pinning
aware agent install aware-aeco           # bundle (installs multiple agents)
aware agent list                         # show what's installed
aware agent describe tekla               # manifest summary + skill index
aware agent skill tekla drawing-identity # print the skill file
aware agent uninstall tekla
```

Installation drops the agent folder under `~/.aware/agents/<name>/` and auto-generates host plugins under `~/.<host>/plugins/aware-aeco/` for each agentic CLI present on the machine.

---

## The callable contract

This is what every agent (and every `exposes-as-agent: true` app) appears as to the orchestrator:

```
agent <name>@<version>
  ├── stateful: bool
  ├── commands: { <name>: { lifecycle, inputs, outputs }, … }
  ├── requires: { filesystem, network, software, secrets }
  └── transport: { cli, mcp?, rest? }
```

Apps in the composition layer don't see "agent" vs "app-exposed-as-agent." They see this shape and call it. **That's the whole point.**

---

## When an agent is wrong

An agent is wrong if it:
- Holds secrets in plain text in any file under the agent folder
- Performs destructive operations without declaring the capability
- Silently catches errors and returns success
- Hides behavior in compiled code that isn't summarized in skills
- Requires the user to edit configuration outside `~/.aware/`

Fix it. Open a PR. We are early — there is no installed base yet, breaking changes are cheap.
