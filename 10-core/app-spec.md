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
    kind: predicate                # predicate (runnable today); map | shape reserved
    description: Keep only welded assemblies
    code: |
      e => e.AssemblyType == AssemblyType.Welded
```

Inline glue lives in the app file. It is visible in the topology and inspectable in the canvas — **no hidden logic.**

> **Runtime support:** only `kind: predicate` executes today. `map` and `shape` are
> reserved for a future release; `aware app validate` / `compile` reject them up
> front (rather than failing at `run`) so an app never locks against an unrunnable
> inline kind (#160).

### Atom references (v0.20)

The persona audit unanimously flagged inline JavaScript lambdas (`code: | e => e.type == "Welded"`) as "not no-code." v0.20 ships a **named atom library** — typed, versioned, reusable predicates / maps / reduces that an inline-glue block can reference instead of inlining code.

```yaml
- id: recent-issues
  inline:
    kind: predicate
    description: Issues newer than last Friday
    atom: 'atom://generic/is-newer-than'
    inputs:
      item:      '{{ upstream }}'
      threshold: '{{ last-friday.iso }}'
```

URI resolution:

| URI shape | Resolves to |
|---|---|
| `atom://generic/<id>` | `atoms/<id>.yaml` in the substrate root (cross-cutting library) |
| `atom://<agent>/<id>` | `20-agents/.../<agent>/atoms/<id>.yaml` (agent-specific) |
| `atom://app/<id>` | `<app-dir>/atoms/<id>.yaml` (app-local; resolved at runtime against the running app's directory) |

When `atom:` is set, `code:` MUST be omitted. The `inputs:` block under `inline:` binds the atom's named parameters to app-context expressions; `aware app validate` checks that every required atom input has a binding and that no unknown inputs are passed.

The substrate ships **33 atoms** in v0.20 — 20 cross-cutting (`is-newer-than`, `group-by`, `sort-by`, `unique`, `pluck`, `count`, `sum`, `avg`, `min`, `max`, `at-least`, `at-most`, `format-date`, `path-join`, `naming-template`, `kebab-to-pascal`, `pascal-to-kebab`, `csv-row-build`, `json-path`, `regex-match`) + 5 Tekla-specific + 8 Revit-specific. See `atoms/README.md` for authoring guidance.

---

## Substrate primitives (v0.19)

The 2026-05-17 persona audit asked for eight workflow primitives every persona reached for and that the agent surface alone couldn't express. These are **node-level** constructs — they live in the topology, not inside an agent.

### `for-each`

Iterate over a static list. The agent surface is event-driven (streams from `tekla.watch`); for-each handles the "I have 7 active projects, walk all of them" case.

```yaml
- id: rollup
  for-each: '{{ projects.items }}'
  do:
    - agent: revit-2026
      command: sheet.list
      inputs:
        filter-issued-for: '{{ item.target-issued-for }}'
```

Inside `do:`, the `{{ item }}` template variable refers to the current iteration's value. Outputs are collected into an array under the for-each node's id.

### `compare`

Diff two snapshots — model state, sheet list, BOM, anything. Returns added / removed / changed records.

```yaml
- id: changes
  compare:
    a: '{{ last-friday-snapshot.rows }}'
    b: '{{ today-export.rows }}'
    by: mark             # the identity key
    track: [profile, length-mm, weight-kg]
```

Output:

```yaml
added:    [...]    # in b, not in a
removed:  [...]    # in a, not in b
changed:
  - mark: 'A-100-1'
    diffs:
      profile:   { from: 'UB 305x165x40', to: 'UB 305x165x46' }
      length-mm: { from: 6200, to: 6500 }
```

### `sweep`

Parametric sweep over a range — engineer storey-count study, designer panel-depth options.

```yaml
- id: storey-sweep
  sweep:
    var: storeys
    values: [4, 5, 6, 7, 8]
  do:
    - agent: tsd-26
      command: model.set-parameter
      inputs:
        name: TotalStoreys
        value: '{{ var }}'
    - agent: tsd-26
      command: analysis.run
    - agent: tsd-26
      command: utilisation.summary
```

Output: per-step record collected into an array under the sweep node's id.

### `approve`

Human-in-the-loop pause. The runtime posts to a channel + waits for explicit approval before the next node runs.

```yaml
- id: gate
  approve:
    channel: 'teams://Project Acme/coordination'
    prompt: |
      {{ tender-pdfs.written.length }} sheets exported to PDF.
      Approve to upload to ACC Docs?
    timeout-minutes: 240   # 4 hours, then fail
    approvers:
      - principal@acme.com
      - pm@acme.com
```

The runtime posts an Adaptive Card with Approve / Reject buttons. The first approver wins. Timeout = fail (downstream nodes skip).

### `schedule`

Time-triggered runs. The app declares a cron + timezone; the runtime takes over.

```yaml
schedule:
  cron: "0 7 * * MON"     # Mondays at 7am
  timezone: "Europe/London"
  start-date: "2026-06-01"
  end-date: "2027-06-01"
```

The runtime registers a OS-level scheduled task (cron / launchd / Task Scheduler) on first install + clears it on uninstall. Compatible with the `aware app run` command for ad-hoc triggers.

### `assert`

Pre-flight gate — abort the run if a condition is false. Pairs with the v0.11 safety contract.

```yaml
- id: file-size-check
  assert:
    expr: '{{ revit-info.file-size-bytes < 500000000 }}'
    on-fail: |
      Revit file is {{ revit-info.file-size-bytes / 1000000 }} MB.
      Refuses to run automation against files > 500 MB.
```

If the expression evaluates false: the run halts with the `on-fail` message as the error, downstream nodes skip. Used liberally in BIM-manager audit apps.

**Decalog truth #9 constraint** — *AI composes the plan; deterministic code is the plan.* The `expr:` in an `assert:` block MUST be:

- a deterministic expression in the substrate's inline-glue engine (the same engine used by `inline.kind: predicate` blocks), OR
- a named atom reference (`atom://generic/at-least`, `atom://app/is-stable`), OR
- a typed agent command (`compare`, `ifc-inspector.validate.schema`, etc.).

It MUST NOT be a `think-node` / `smart-node` call. `aware app validate` enforces this. Truth #9 is structural: an LLM may *compose* an assert (write the expression in the first place), but the *evaluation* at run time is always deterministic. PE seals and steel deliveries do not survive hallucinations.

### Reading a drawing / PDF / image (compose-time extraction)

The same truth #9 boundary answers *"I have a contract drawing / PDF / schedule screenshot to read."* The reading is an LLM act, so it happens at **compose time**, never in the run path:

1. The terminal AI (composing the app) reads the artifact — a connection schedule, a BOM screenshot, a spec PDF — and **bakes the extracted values into the deterministic app file** (a `for-each` over the parsed rows, literal attribute values, etc.).
2. Execution then runs deterministically against those baked values — no model reads the artifact at run time.

When the extraction is high-stakes or the human should eyeball it before any write, gate the first write-mode node behind an **`approve:`** block (the extracted values appear in the Adaptive Card; the human confirms or rejects before the deterministic write proceeds).

```yaml
# `schedule-rows` is baked in by the terminal AI after reading the screenshot —
# the joint ids and preset names are LITERALS in the file, not a runtime read.
- id: write-attributes
  for-each: '{{ schedule-rows }}'
  do:
    - id: save
      agent: tekla
      command: save-attributes
      config:
        joint-id: '{{ item.joint-id }}'    # from the compose-time extraction
        filename: '{{ item.preset }}'
      safety: { transaction-group: schedule-import, snapshot: true }
```

What this is **not**: a runtime `think-node` that reads the drawing while the app executes. `aware app validate` rejects that for the same structural reason it rejects an LLM `assert:` — the run path is deterministic, so front doors like floless.app should extract up front (optionally behind `approve:`), not defer reading to execution.

### `snapshot`

Freeze model state to an immutable artifact. Pairs with the v0.11 safety contract's `snapshot:` flag but operates at the topology level — the artifact is *named* in the topology and addressable by downstream nodes.

```yaml
- id: pre-issue-snapshot
  snapshot:
    of:
      agent: revit-2026
      target: '{{ project.path }}'
    name: 'pre-tender-{{ run.date }}'
    keep-days: 90
```

Output: snapshot id + path. The downstream `compare` node can reference a prior snapshot by name:

```yaml
- id: delta
  compare:
    a-snapshot: 'pre-tender-2026-05-10'
    b-snapshot: 'pre-tender-{{ run.date }}'
    by: element-id
```

### `model-lock`

Single-writer guarantee on a host model (Revit Central, Tekla model, ArchiCAD Teamwork). Acquired before write nodes; released on completion or failure.

```yaml
- id: lock
  model-lock:
    agent: revit-2026
    target: '{{ project.path }}'
    acquire-timeout-seconds: 60
    write-budget-per-second: 10   # rate-limit writes
    on-conflict: abort            # abort | wait-and-retry
```

If a human user has the model open: `abort` fails fast; `wait-and-retry` polls every 30s up to `acquire-timeout-seconds`. The lock releases automatically when the topology completes (success or failure).

### Composition

Primitives compose: a `for-each` containing an `approve` gate inside a `sweep` is valid. The runtime enforces the v0.11 safety contract on any write-mode node anywhere in the tree.

```yaml
- id: weekly-rollup
  schedule:
    cron: "0 7 * * MON"
    timezone: "Europe/London"
  do:
    - id: each-project
      for-each: '{{ projects.items }}'
      do:
        - id: gate
          assert:
            expr: '{{ item.active }}'
        - id: rollup
          # ... agent invocations ...
```

---

## Lockfile sidecar (v0.24)

Every app compiles to a deterministic sidecar called `<app-name>.lock`. This is the artifact engineers, building-control officers, and insurers actually read — not the AI's prose in the source app file.

The substrate is extension-agnostic: `.flo` is FloLess's extension; `.app`, `.flow`, `.aware` or any other extension all compile to the same lockfile shape. The sidecar is named by the app's id, not the source extension.

### What's resolved in the lockfile

| Resolved at compile time | Left as runtime expression |
|---|---|
| Every agent reference → pinned version (`revit-2026@2026.0.0.9999`) | Upstream-output references (`{{ sheets.sheets }}`) |
| Static `{{ inputs.x }}` substitutions when `inputs:` are defaults | Run-instance secrets (`{{ secrets.acc-token }}`) |
| Every node's input + output schema | Per-iteration `{{ item }}` references inside `for-each` |
| Write-mode tagging on every node | Per-step `{{ var }}` inside `sweep` |
| Safety-contract block defaults expanded | Time-dependent expressions (`{{ run.date }}`) |

The lockfile is plain text (YAML), diffable in git, and readable end-to-end. Engineers do not read the AI's prose; they read the lockfile.

### Shape

```yaml
# tender-issue.lock — compiled from tender-issue.flo
# DO NOT EDIT — regenerated by `aware app compile`.

source-hash:      sha256:abc123...        # SHA-256 of the source app file
compiled-at:      2026-05-17T20:00:00Z
compiler-version: 0.24.0

app:     tender-issue
version: 0.1.0

agent-pins:
  revit-2026:     2026.0.0.9999
  microsoft-365:  1.0.0
  acc-docs:       1.0.0

nodes:
  - id: sheets
    agent: revit-2026
    command: sheet.list
    mode: read
    inputs:
      filter-issued-for: 'Ready for Tender'
    output-schema:
      sheets:
        type: array
        items: { sheet-number, sheet-name, current-revision, issue-date, drawn-by, checked-by }

  - id: bump
    agent: revit-2026
    command: revision.bump
    mode: write
    safety:
      transaction-group: tender-bump
      snapshot: true
      worksharing:
        check: true
        fail-if-other-user: true
    inputs:
      sheet-numbers: '{{ runtime: sheets.sheets[*].sheet-number }}'
      description:   'Issued for Tender'
      issued-by:     'P.Lisowski'
```

### CLI

| Command | What it does |
|---|---|
| `aware app compile <app>` | Explicit compile. Emits `<app>.lock` next to the source file. Fails if validation fails. |
| `aware app validate <app>` | Now also writes `<app>.lock` as a side effect (was: silent pass) |
| `aware app inspect <app>` | Opens Glass Box — a single-file HTML viewer of the lockfile — in the user's default browser |
| `aware app run <app>` | Refuses to execute unless a fresh `.lock` matches the source's `source-hash`. Prompts the user to run `aware app compile` first |

### Why this matters

The persona audit + the verification brainstorm both converged on the same insight: **engineers will never trust prose composed by an AI.** They will trust a deterministic, type-resolved, signed artifact. The lockfile is that artifact.

In git, the lockfile is what the team reviews. The prose source can change freely — the engineer reads the diff between two lockfiles to know what actually changed in the execution plan. The AI's prose is for AI; the lockfile is for the human.

This is the foundation for v0.25 (panel review on write-mode nodes) and v0.26 (signed JSONL receipt for PE-stamp-grade deliverables).

---

## Panel review (v0.25)

The 4-agent verification brainstorm + the persona audit converged on a single insight no existing AI-for-AEC tool delivers: **N different models, in N different domain hats, must agree before any write-mode commit happens.** A structural engineer's killer demo is watching GPT-5 (in a UK code-compliance hat) dissent against Claude (in a structural-engineer hat) with a specific BS 9999 §6.2 citation. Today that flow exists nowhere.

v0.25 ships it as a substrate primitive.

### `panel:` block

Any write-mode node can declare a `panel:` block. When the runtime reaches the node, it convenes the panel; each voice produces a verdict + rationale; if the `require:` quorum isn't met (default `unanimous`), the runtime halts with the dissents recorded in the receipt.

```yaml
- id: bump-revisions
  agent: revit-2026
  command: revision.bump
  inputs:
    sheet-numbers: '{{ sheet-list.sheets[*].sheet-number }}'
    description:   'Issued for Tender'
  safety:
    transaction-group: tender-bump
    snapshot: true
    worksharing: { check: true, fail-if-other-user: true }
  panel:
    require: unanimous              # unanimous | majority | quorum-N
    on-dissent: halt                # halt | log-only | warn
    voices:
      - id: structural-engineer
        model: claude-opus-4-7
        voice-pack: 'atom://voices/aware-aeco/structural-engineer@2026'
      - id: code-compliance
        model: gpt-5
        voice-pack: '@ise/uk-structural-reviewer@2025'
        jurisdiction: UK
      - id: detailer
        model: gemini-2-5-pro
        voice-pack: 'atom://voices/aware-aeco/steel-detailer@2026'
      - id: building-physics
        model: llama-4-405b
        voice-pack: 'atom://voices/aware-aeco/building-physics@2026'
```

### Voice packs — a new distribution primitive

Decalog #6 says "ship agents, not apps." v0.25 generalises: **ship voices.**

A voice pack is a markdown system-prompt + reference-codes folder published by an institution / authoring engineer / firm. Forkable. Version-pinnable. Citable. Independently auditable.

```bash
aware voice install @ise/uk-structural-reviewer@2025
# Resolves @ise → Institution of Structural Engineers (UK)
# Pulls system-prompt.md + reference-codes/*.md
# Stores at ~/.aware/voices/ise/uk-structural-reviewer/2025/
```

Voice pack folder layout:

```
~/.aware/voices/ise/uk-structural-reviewer/2025/
  manifest.yaml           # pack metadata + version
  system-prompt.md        # the LLM's instructions
  references/
    BS-9999-2017.md       # cited code reference
    BS-EN-1993-1-1-2022.md
    ...
  examples/               # optional — sample dissents + rationales
    fire-strategy-gap.md
```

Voice packs compose with the v0.20 atom library: a voice pack can reference atoms (`atom://generic/at-least`) inside its system prompt to enforce deterministic checks during reasoning.

### Receipt artifact

Every panel-gated commit emits a JSONL receipt next to the trace:

```
~/.aware/receipts/<app>/<run-id>.jsonl
```

Each receipt contains:

```jsonl
{"kind":"panel-convened","node":"bump-revisions","ts":"2026-05-17T17:00:01Z","voices":["structural-engineer","code-compliance","detailer","building-physics"]}
{"kind":"voice-verdict","voice":"structural-engineer","model":"claude-opus-4-7","verdict":"approve","rationale":"Revision bump on 47 sheets..."}
{"kind":"voice-verdict","voice":"code-compliance","model":"gpt-5","verdict":"dissent","rationale":"Revision bump to 'Issued for Tender' on A-100 precedes the fire-strategy approval gate per BS 9999 §6.2. ...","citations":["BS 9999:2017 §6.2"]}
{"kind":"panel-halted","reason":"non-unanimous (3 approve, 1 dissent)","ts":"2026-05-17T17:00:14Z"}
```

When the engineer (or their insurer, building-control officer) needs to reconstruct *why* a write committed (or didn't), they read the receipt. Every dissent is logged forever — even resolved ones — so a post-incident audit can answer "did anyone object?" with a yes/no/transcript.

If the app declares `engineering.output-seal` (v0.21), the JSONL receipt is referenced from the seal's chain-of-custody record.

### CLI

| Command | What it does |
|---|---|
| `aware voice install <pack>` | Install a voice pack from the registry or a local path |
| `aware voice list` | List installed voice packs |
| `aware voice describe <pack>` | Show pack metadata + system prompt + references |
| `aware voice uninstall <pack>` | Remove a voice pack |
| `aware app run <app>` | When a panel-gated node fires, prints each voice's verdict live |
| `aware app explain <app>` | Now also surfaces which nodes have panels + their voice composition |

### Composition

The panel doesn't replace the v0.11 safety contract, v0.19 substrate primitives, v0.21 engineering envelope, or v0.23 deterministic-validator boundary. It's the **multi-voice review layer** that runs *before* the deterministic write executes:

```
write-mode node
  ↓
v0.11 safety pre-flight (worksharing, snapshot, transaction-group)
  ↓
v0.25 panel review (N voices, must agree)
  ↓
v0.19 primitive checks (assert, model-lock)
  ↓
the actual deterministic write
  ↓
v0.21 output-seal receipt (audit artefact)
```

The panel is the **human-in-the-loop slot, automated**. A panel of LLMs in different domain hats is the closest substitute to a real four-eyes review that doesn't require four humans available at app-run time.

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
