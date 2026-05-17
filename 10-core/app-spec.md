# App Spec

How an AWARE app is structured and runs. This is the plain-text composition format.

This document defines what an app *is*. See the [Agent Spec](./agent-spec.md) for what agents are (the building blocks apps compose).

---

## What an app is

An **app** is a named composition of agents (and inline glue) with its own manifest. It can be one-shot (runs once and exits) or long-running (stays up until stopped). It can expose itself as an agent so other apps can call it as a black box.

The app file is **plain text.** Open it in Notepad. Read it. Edit it. Share it. There is nothing else to an app — no binary, no bundle, no proprietary format.

---

## File format

A single file containing YAML-style structured text. The extension is up to the authoring tool:

- `.flo` — FloLess's convention
- `.app` — the recommended generic default
- `.<anything>` — your tool's choice

The substrate doesn't care about the extension. It cares about the **shape** of the contents. The shape is identical across extensions; tools differ only in which one they default to writing.

---

## File anatomy

A minimal app:

```yaml
app:         welded-to-tc
version:     0.3.1
description: Watch Tekla for welded assemblies, upload drawings to TC.

requires:
  - tekla@2025.x
  - trimble-connect@2.x

layout: linear
nodes:
  - id: tekla-watch
    agent: tekla
    command: watch
    config:
      filter: welded

  - id: tc-upload
    agent: trimble-connect
    command: upload
    config:
      project-id: "{{ inputs.tc-project-id }}"
      folder-id:  "{{ inputs.tc-folder-id }}"

connections:
  - from: tekla-watch
    to:   tc-upload
    label: AssemblyEvent
```

That is a complete, runnable app. Three keys (`nodes`, `connections`, `requires`) and a handful of fields.

---

## Manifest fields

```yaml
app:           welded-to-tc            # required · kebab-case id
version:       0.3.1                   # required · semver
display-name:  Welded → TC Uploader    # optional · human-readable
description: |                         # required · one paragraph
  Watches Tekla for new welded assemblies. Uploads their drawings
  to a configured Trimble Connect fab folder. Idempotent by Mark.

# What this app exposes (optional — makes the app an agent)
exposes-as-agent: true
exposed-commands:
  start:
    lifecycle: start
    inputs:
      tc-project-id: string
      tc-folder-id:  string
    outputs:
      type: stream
      schema:
        mark:    string
        file-id: string

# Agent version pins
requires:
  - tekla@2025.x                       # minor-pinned (recommended)
  - trimble-connect@2.x
  - file@1.0.0                         # exact pin

# Capabilities the app needs (inherited from agents + app-level extras)
requires-permissions:
  filesystem:
    - read: ~/.aware/apps/welded-to-tc/state.json
  network:
    - https://app.connect.trimble.com

# Topology shape
layout: linear   # linear | dag

# Nodes — either agent invocations or inline glue
nodes:  ...

# Edges between nodes
connections: ...

# App-level skills (optional) — config knowledge specific to this app
skills:
  - configuring.md
  - troubleshooting.md
```

### Required fields

`app`, `version`, `description`, `nodes`, `connections`, `requires`.

---

## Nodes

A node is either an **agent invocation** or an **inline glue** step. Both appear in the topology; both are inspectable.

### Agent invocation

```yaml
- id: tekla-watch
  agent: tekla                     # the agent id (from manifest.yaml)
  command: watch                   # which command on that agent
  config:                          # parameters for this invocation
    filter: welded
```

Optionally place the node on a grid for DAG layouts:

```yaml
- id: match-build
  agent: smart-node
  command: fan-in-sync
  row: 2                           # for layout: dag
  col: 3
  config: { ... }
```

### Inline glue

For tiny operations (filters, maps, reshapes) that don't deserve a dedicated agent:

```yaml
- id: filter-welded
  inline:
    kind: predicate                # predicate | map | shape
    description: Keep only welded assemblies
    code: |
      e => e.AssemblyType == AssemblyType.Welded
```

Inline glue lives in the app file. It is visible in the topology and inspectable in the canvas — **no hidden logic.**

---

## Connections

A connection is a directed edge between two node ids, with an optional label naming the data type flowing through it.

```yaml
- from: tekla-watch
  to:   tc-upload
  label: AssemblyEvent
```

For DAGs:
- A node may have **many incoming** edges (fan-in)
- A node may have **many outgoing** edges (fan-out)
- The graph must be **acyclic** (the orchestrator rejects cycles at load time)

Fan-in nodes (multiple inputs) must declare which input each incoming edge maps to:

```yaml
- from: pdf-extract
  to:   match-build
  label: Drawing
  input: drawing                   # named slot on the target

- from: excel-lookup
  to:   match-build
  label: Spec
  input: spec
```

---

## Templating

Inputs to a node can reference:

- **App inputs:** `{{ inputs.<name> }}`
- **Upstream outputs:** `{{ <node-id>.<output-name> }}`
- **Environment / credentials:** `{{ secrets.<credential-id> }}` (resolved against `~/.aware/credentials/`)
- **App config:** `{{ config.<key> }}` (resolved against `~/.aware/apps/<name>/config.yaml`)

Templating happens at composition time (when the orchestrator prepares to run a step), not at app authoring time. The app file stays untouched across runs.

```yaml
- id: tc-upload
  agent: trimble-connect
  command: upload
  config:
    project-id:       "{{ inputs.tc-project-id }}"
    folder-id:        "{{ inputs.tc-folder-id }}"
    file:             "{{ tekla-watch.drawing-bytes }}"
    idempotency-key:  "{{ tekla-watch.mark }}"
```

---

## Lifecycle

| Shape | Trigger | `aware app run` behavior |
|---|---|---|
| **One-shot** | All nodes stateless; one trigger that fires once. | Executes once, prints results, exits. |
| **Long-running** | At least one stateful node (e.g. a watcher). | Stays up. `aware app stop <name>` to terminate. Persists state to `~/.aware/apps/<name>/state/`. |

The shape is **inferred** from the agents in `nodes:`. If any node's agent is `stateful: true`, the app is long-running.

---

## Safety contract (write-mode nodes)

The 2026-05-17 persona audit unanimously identified live-model writes as the highest-risk surface in AWARE. The architect, structural engineer, BIM manager, and detailer all refused live writes until the substrate has a *structural* safety contract — not policy. v0.11 introduces one.

### The rule

Any node that mutates state outside the app — Revit model, Tekla model, cloud CDE folder, OAuth-protected API that writes — **MUST** declare a `safety:` block. `aware app validate` refuses to install an app missing `safety:` on a write-mode node. `aware app run` refuses to execute one.

A node is *write-mode* if either:
- the underlying command declares `mode: write` in its manifest entry, OR
- the command's name conventionally implies a write (e.g. `*.create`, `*.update`, `*.delete`, `*.bump`, `*.stamp`, `*.reload-all`, `*.bulk-write`, `*.insert`).

Pure-read nodes (e.g. `sheet.list`, `view.capture-bitmap`, `element.by-parameter-value`) do not need `safety:`.

### The block

```yaml
- id: bump-rev
  agent: revit-2026
  command: revision.bump
  inputs:
    sheet-numbers: ["A-100","A-101"]
    description: "Issued for Tender"
  safety:
    transaction-group: tender-bump          # rollback boundary
    snapshot: true                           # eTransmit/save-as before write
    snapshot-path: ~/.aware/snapshots/{{ run.id }}/  # default; override here
    worksharing:
      check: true                            # pre-flight: model is checked out by current user
      fail-if-other-user: true               # abort if any touched element is owned by someone else
    audit-stamp:
      uda-prefix: AWARE_                     # write AWARE_RUN_ID / AWARE_APP / AWARE_OPERATOR UDAs on touched objects
    rollback-token: "{{ revit-2026.tx.token }}"  # token returned by the write — feeds `aware app rollback`
```

### Field semantics

| Field | Type | Required | Notes |
|---|---|---|---|
| `transaction-group` | string | yes | Names the rollback boundary. Multiple write nodes sharing a `transaction-group` rollback together. |
| `snapshot` | boolean | yes | If `true`, the agent's transport binary MUST save a snapshot of the target document/folder before mutating. If `false`, the app author has explicitly acknowledged the no-undo risk. |
| `snapshot-path` | path | no | Override the default. Defaults to `~/.aware/snapshots/<app>/<run-id>/<node-id>/`. |
| `worksharing.check` | boolean | no | Pre-flight check that the document is in a writable state by the current user. Default `true` for Revit / Tekla / Allplan agents; `false` otherwise. |
| `worksharing.fail-if-other-user` | boolean | no | Default `true`. Set `false` only if you want concurrent writes with another active user (almost never). |
| `audit-stamp.uda-prefix` | string | no | Per-object UDA / property prefix recording the run. Default `AWARE_`. Agents that support UDA writes (Revit, Tekla, Allplan, IDEA) MUST honor this. |
| `audit-stamp.fields` | array | no | Subset of `{run-id, app, operator, timestamp}` to write. Default: all four. |
| `rollback-token` | template-expr | no | Records the token returned by the write — used by `aware app rollback <app> --run-id <id>` to undo. |

### `--dry-run` mode

`aware app run <app> --dry-run` exercises the full DAG **without** committing any write-mode side effects. Each write-mode node emits a structured `would-write:` block in the trace instead of calling the agent's mutation transport. The dry-run trace is replayable but rolls back nothing (because nothing was written).

Dry-run is mandatory **before** any production write run against a real Revit / Tekla central file. The CLI prints a one-line summary at the end:

```
$ aware app run tender-issue --dry-run
✓ dry-run complete: would write 47 sheets to PDF, would bump 47 Revit revisions, would post 1 Teams message
  no model touched; nothing to rollback
```

### Rollback

`aware app rollback <app> --run-id <id>` walks the JSONL provenance trace in reverse, replaying each write-mode node's `rollback-token` against its agent. Agents that don't support rollback (e.g. one-way external posts like Slack messages) MUST declare `rollback: unsupported` on the command — apps using such a node cannot be rolled back automatically.

### What this does NOT promise

- It does **not** guarantee that a vendor tool's transactional semantics are perfectly mirrored. If Revit's `TransactionGroup.Assimilate()` silently dropped a transaction, AWARE inherits that bug.
- It does **not** prevent a misconfigured app from doing damage — it makes the configuration *visible at validation time*.
- It does **not** sign the engineer's stamp. That's v0.21 (engineering envelope).

### Why this exists

The architect persona wrote (audit, section H): *"You don't need to solve this with policy. You solve it with structure: make write-mode nodes require an explicit `safety:` block declaring the transaction group, the snapshot path, and the rollback strategy — and refuse to run a write node that doesn't declare it. That single rule would make AWARE the first AI-for-AEC tool I'd recommend without caveat."* This section is that rule.

### Instances

```bash
aware app run welded-to-tc                                  # default instance
aware app run welded-to-tc --instance fab-east              # named instance
aware app run welded-to-tc --instance fab-west
```

Each instance has its own state directory: `~/.aware/apps/<name>/instances/<instance-id>/`. Same app file, different running contexts.

---

## App-level skills

When an app is shipped to other users, they often need to know **how to configure it for their project**. App-level skills capture that:

```
welded-to-tc/
├── welded-to-tc.flo                # the composition
├── manifest.yaml                   # if needed (extends the .flo)
└── skills/
    ├── configuring.md              # "Set tc-project-id in config.yaml..."
    └── troubleshooting.md          # "If uploads fail with 429..."
```

App-level skills are the same format as agent skills — plain markdown, dual-use (human + AI).

---

## `exposes-as-agent`

The decalog-level killer feature. When an app sets `exposes-as-agent: true`, the registry treats it like an agent. Other apps install it via `aware app install <name>` and call it from their own `nodes:` block as if it were a regular agent.

```yaml
# In my-bigger-pipeline.flo
- id: validate
  agent: pawel/welded-to-tc        # an app, exposed-as-agent
  command: start
  config:
    tc-project-id: "..."
    tc-folder-id: "..."
```

The caller doesn't see the internals. The internal topology runs; the exposed outputs flow back out.

### `exposed-commands`

When `exposes-as-agent: true`, the app must declare which commands it surfaces:

```yaml
exposes-as-agent: true
exposed-commands:
  start:
    lifecycle: start
    inputs:  ...
    outputs: ...

  list-pending:                    # additional commands the app exposes
    lifecycle: single
    inputs:  ...
    outputs: ...
```

The orchestrator synthesizes an agent manifest from this block. To callers, the app looks exactly like a hand-written agent. (See [Agent Spec → "The callable contract"](./agent-spec.md#the-callable-contract).)

### Constraints

For v0:
- An app exposing-as-agent **cannot itself compose** another exposed-as-agent app from the same machine. (Prevents accidental recursion.) Deeper hierarchies come post-v0.
- An exposed app's stateful internal agents are managed when the calling app starts/stops the exposed command.
- An exposed app **does** inherit the union of its internal agents' `requires-permissions`. The caller approves the full union, not each internal agent separately.

---

## Versioning

Apps follow semver. Breaking changes (changed inputs/outputs on exposed commands, removed nodes that callers depend on) require a major bump.

Apps **pin agent versions** in `requires:`. Three pinning levels:

| Pin | Example | Meaning |
|---|---|---|
| Exact | `tekla@2025.0.1` | This specific patch version. Used for reproducibility. |
| Minor (recommended) | `tekla@2025.x` | Any patch within 2025.0. The default. |
| Loose | `tekla@2025.0` | Any compatible version newer than 2025.0. Use when you actively track upstream. |

The orchestrator resolves pins at install time and locks the resolved versions into `~/.aware/apps/<name>/lockfile.yaml`. Subsequent runs use the lock. `aware app update <name>` re-resolves and updates the lock.

---

## Installation and distribution

```bash
aware app install @pawel/welded-to-tc           # from registry
aware app install ./local-app/                  # from a local folder
aware app install https://github.com/.../app    # from a git url
aware app list
aware app run welded-to-tc
aware app stop welded-to-tc
aware app uninstall welded-to-tc
```

Installation drops the app folder under `~/.aware/apps/<name>/`. Each instance gets its own state under `instances/<instance-id>/`.

---

## When an app file is wrong

An app is wrong if it:
- Hardcodes secrets in the composition file
- References agent commands by version-incompatible signatures
- Has cycles in the connections graph
- Declares stateful nodes without a corresponding `stop` command somewhere reachable
- Hides logic in inline glue without a `description` field

`aware app validate <path-to-app>` runs all these checks before install.
