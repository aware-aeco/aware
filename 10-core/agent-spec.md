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
version:      0.1.0                # required · substrate semver — NEVER the vendor SDK version
sdk-target:   2025.0               # optional · the vendor SDK / product version this agent targets
display-name: Tekla Structures     # optional · human-readable for UIs
description: |                     # required · one paragraph
  Watches the active Tekla model and exposes Tekla Open API commands.
  Auto-generated from Tekla Structures 2025.0 DLLs.

# Operational shape
stateful: true                     # required · true|false (see "Stateful vs Stateless")
status: available                  # optional · available (default) | planned | requires-runtime
# minimum-cli-version: 0.136.0     # required only with status: requires-runtime

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

### `version` vs `sdk-target`

These are **two different numbers**. Do not collapse them.

- **`version`** — the substrate's own semver for this agent. New agents start at `0.1.0`. Bumped when the agent's manifest, skills, or commands change in our repo. Format: `MAJOR.MINOR.PATCH`.
- **`sdk-target`** — the vendor SDK / product version this agent reflects. Optional (utility and file-format agents don't have one). Free-form string so it can mirror whatever the vendor uses (`2025.0`, `25.1.5.1431`, `v2`, `8.31.26126.13431`).

Generated agents (`aware build agent --from-nuget`, `--from-openapi`, etc.) always emit `version: 0.1.0` and put the vendor pin in `sdk-target`. The substrate report displays both: `<agent> v<version> · <vendor> · SDK <sdk-target>`.

Why: conflating them confused users — they'd see `tekla v2025.0.1` and read it as "Tekla SDK 2025.0.1" (which doesn't exist). Splitting the fields makes the vendor pin honest and lets the substrate revision evolve independently.

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

### Curated vs reflected commands

Every command declares a `category:` in the manifest:

| Category | What it is | Where it comes from | What a composer can rely on |
|---|---|---|---|
| `curated` | A workflow verb a practitioner actually invokes. Has typed `inputs:` / `outputs:`, worked examples, and a skill describing when to use it. | Hand-written by the agent curator. | Stable contract, deterministic behavior, replayable, idempotent (or explicitly declared otherwise). |
| `reflected` | A leaf-level API method auto-generated from a vendor SDK / OpenAPI / decompile. Description may be the raw vendor doc-comment. | Generated by `aware build agent --from-*`. | Wide coverage as an escape hatch — but a composer (human or AI) must read the vendor docs to use it correctly. |

**Why the distinction matters.** Auto-generation produces breadth (Revit's 7,647 methods, Rhino's 6,954) but no curation; without the category, the catalogue gets diluted and search becomes unusable. The Tekla curated agent (31 craft skills, 3 workflow commands) is the gold standard for `curated`; every NuGet-generated agent ships with `reflected` first, and curators promote individual methods to `curated` over time by adding `inputs:` / `outputs:` schemas, examples, and skills.

**Default.** If `category:` is omitted, the CLI infers it from provenance: anything with a `generated-from:` block on the agent manifest defaults to `reflected`; everything else defaults to `curated`.

**Required fields for `category: curated`:**
- `inputs:` — typed schema (list of `{ name, type, required, description, example }`)
- `outputs:` — typed schema (same shape)
- A `commands/<name>.md` file with the human-readable elaboration plus at least one worked example
- A skill (in `skills/`) that explains when to reach for this command vs alternatives

**Required fields for `category: reflected`:** the description can be a raw doc-comment. No examples required. The agent's `skills/` may still cover the namespace at a high level, but per-method skills are not mandatory.

**Filtering.** `aware tree <agent> --curated`, `aware search <term> --curated`, `aware agent describe <id>` all support filtering or weighting by category. Defaults show curated first, reflected as a collapsed escape hatch.

### Declared effect (`mode-basis`)

A command's `mode:` (`read` / `write`, see [App Spec § Safety contract](./app-spec.md)) can reach a node by five routes, and only one of them is the agent author saying what the command does. AWARE records which route a mode took — its **basis** — so a later step (carrying an approved workflow forward to a new agent version, #628) can tell a promise from a guess:

| Basis | When | A declaration? |
|---|---|---|
| `declared` | The command states `mode:` and is **not** `mode-overridable`. | **Yes** — the only one. |
| `overridable-default` | The command is `mode-overridable: true` and the node states nothing, so the manifest's `mode:` applies as a conservative default (Tekla's `exec`). | No |
| `node-override` | The command is `mode-overridable` and the node states its own `mode:` — or the command is not in the manifest and the node states one. The workflow author said so, not the agent. | No |
| `inferred-name` | No `mode:` anywhere; inferred from the command-name convention (`*.create`, `insert`, … are write; the rest read). | No |
| `fallback-write` | The command is not in the manifest and the node states nothing: compile locks `write` for safety. | No |
| `inherited` | The agent is app-backed (`exposes-as-agent`). Its synthesized `mode:` is written at the app boundary (defaulting to `read`) and is never a declaration; the real effect is the backing app's — the highest mode over every dispatchable node of the backing app, each judged against **that app's own approved pins** — and `write` outright when the app's author wrote `mode: write` on the exposed command. | No (it is the backing app's) |

There is no separate manifest key for "declared": the explicit, non-overridable `mode:` **is** the declaration. An agent that wants its read verbs to count as declared reads states `mode: read` on them.

`aware agent describe <id> --json` reports, per command, `mode`, `mode-basis` (`declared`, `overridable`, `inferred` or `inherited` — no node exists there, so the five node routes collapse to these four words) and `mode-overridable`. An app-backed agent's rows add `inherited-from: <backing app>` and `inherited-read-only` (every node of the backing app is a declared read under its own approved pins), or `inherited-detail` saying why the backing app could not be judged (not installed, no current approval, a lock the run would refuse as inconsistent — `E_APP_LOCK_INVALID` — or approved agents not stored), in which case `mode` is the synthesized boundary mode.

**A workflow is declared read-only** iff every dispatchable node (frozen subtrees excluded, `do:` bodies included) is eligible as read under **both** the pins it was approved on and the pins it would move to: an agent node whose mode is `read` on basis `declared` in both; an `inline`, `assert` or `compare` step; or an app-backed node whose inherited effect is read-only and whose pin does not move (an agent that is app-backed under **either** set of pins counts as app-backed: a wrapper that becomes a plain agent, or the reverse, is a moved wrapper) — and in every case not a node that calls a model at run time (`runtime-model`), and never a `sweep`, `approve`, `snapshot` or `model-lock` step. This is stricter than "every command the workflow calls is read on that agent": one `exec` left at its write default, or one verb whose `read` was only inferred from its name, makes the whole workflow a person's decision. The word is **declared**: it comes from manifest declarations, never from anything observed at run time, and every label built on it says "declared read-only".

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

## Runnability (`status`)

Both an agent and an individual command can declare `status: available` (the default), `status: planned`, or `status: requires-runtime`. `planned` means *declared but not yet runnable* — the contract is published so apps can be authored against it, but the implementation isn't shipped yet. Apps that reference a `planned` agent or command are rejected at **validate / compile** (`E_APP_AGENT_UNAVAILABLE` / `E_APP_COMMAND_UNAVAILABLE`) rather than failing at run with a confusing dispatch error.

`requires-runtime` means the implementation ships inside AWARE itself and is runnable only when the current CLI satisfies the agent-level, strict-SemVer `minimum-cli-version`. That field is required when either the agent or one of its commands uses `requires-runtime`, and is rejected as unused otherwise. Local install, registry install, app validation/compile, and app run all fail closed below the minimum (`E_AGENT_RUNTIME_TOO_OLD` at install and `E_APP_AGENT_UNAVAILABLE` through an app). At or above the minimum, the status is runnable. A CLI predating the enum value, including v0.135.0, rejects `requires-runtime` while deserializing the manifest; it cannot ignore the version field and route through an older generic transport.

- **Agent-level** (`status:` at the top of the manifest) — the whole agent isn't runnable (e.g. no shipped/installable transport binary).
- **Command-level** (`status:` on a command) — the agent is runnable but a *specific* command isn't wired yet (e.g. a REST agent whose read commands work but whose multi-step / binary upload awaits an implementation). An agent can be partially runnable.

---

## Transport

Required: at least one transport **the runtime can dispatch** — `cli`, `rest`, `app` or `builtin`. Optional: more than one, plus `mcp`, which is not dispatchable on its own (see [Which one runs](#which-one-runs-priority-order)).

| Transport | When to use | Format |
|---|---|---|
| `cli` | Default. Works in any agentic CLI (claude-code, codex, opencode). | A binary in `runtime/cli/` + a stable JSON response envelope. |
| `mcp` | When the agent needs to work in non-CLI hosts (Claude Desktop, Cursor) or wants streaming/server-pushed events. | An MCP server config in `runtime/mcp-server.json`. |
| `rest` | When the underlying tool is already a REST API and a REST shim is simpler than a CLI. | An OpenAPI spec or hand-written shim. |
| `builtin` | A `_core` utility the runtime handles **in-process** — no host binary to ship or install (e.g. `html-report`'s generic renderer). | A present-but-empty `builtin: {}` block; dispatch routes by agent id to a runtime handler. |
| `app` | **Not hand-written.** Synthesized by `aware app install` onto the agent manifest of an `exposes-as-agent: true` app. | `app: { backed-by: <app-id> }`; dispatch runs that app's node chain instead of spawning a binary. |

The contract every transport satisfies: **structured input → structured output, with errors as data, not exceptions.** The CLI envelope is documented in [`response-envelope.md`](./response-envelope.md) (forthcoming).

For a REST command whose successful body is too large for a node output, declare
`response: artifact-stream` on that command. Only HTTP 200 with
`application/x-ndjson` is spooled; the output keeps the ordinary REST
`{status:200,headers,body}` envelope, but `body.artifact` is a run-scoped
`aware.artifact-ref/v1` descriptor (app, instance, runId, opaque id, byte count,
SHA-256 and content type). No response path or raw body enters the trace.
The caller must supply an owner-reserved positive source-byte budget through
trusted process context; ordinary REST commands keep their existing behavior.

### Which one runs (priority order)

An agent may declare several transports; exactly one of them dispatches. The order is:

```
cli  >  rest  >  app  >  builtin
```

The highest-priority DECLARED transport wins, and it is the only one that runs — a manifest carrying both `builtin:` and `cli:` runs as `cli`, and its builtin handler is never reached.

This is not a detail of the runtime. It decides which nested `requires:` pins are enforced, what `aware agent catalog` publishes as an agent's `transport:`, and which app compositions validate — so anything asking "how does this agent run?" must answer with THIS order, or it will contradict what actually happens (#215, #349, #366).

**`app:` identifies a synthesized manifest, so do not hand-write one.** `aware app uninstall <app>` and `aware app rename` treat any `agents/<id>/` whose manifest declares `app: { backed-by: <that app> }` as the copy they generated, and **delete it** — that is a question about provenance, deliberately not about dispatch, so adding `cli:` beside it does not protect the folder. AWARE never emits that combination (the synthesizer writes `app:` alone), and a hand-written one is out of spec.

**`mcp` is not in the order, and is not dispatchable on its own.** It describes how a non-CLI host (Claude Desktop, Cursor) reaches the agent; the AWARE runtime cannot invoke it. Declare it *alongside* a runnable transport, as the example above does. An `mcp`-only agent is rejected at validate/install (`E_AGENT_MCP_ONLY`) rather than being admitted and then failing at run.

---

## Connection probe (`probe:`)

An agent may declare **one** probe: a fixed call that proves a connection reaches the real host
or account, run by `aware agent probe <agent>` (#617). It is data in the manifest, inside the
manifest digest, so the tekla probe is tekla's manifest and not a host's code.

```yaml
probe:
  command: model-info        # a command whose OWN effective mode is read
  inputs: {}                 # literal values only — no templating, no caller input
  describe: "Reads the name of the model open in Tekla Structures."   # plain English, <= 160 chars
  kind: host                 # host | account
  rest:                      # rest transport only — and required there
    origin: https://openidconnect.googleapis.com   # exact origin: https, no port/userinfo/path
  reports:                   # RFC 6901 pointers into the transport's ACTUAL result
    summary: /model_name     #   cli: the bridge's JSON receipt
    identity: /host          #   rest: {status, headers, body} — so /body/...
    stable-id: /host_pid
    host-version: /host_version
```

The grammar is **closed** — an unknown key anywhere in the block is refused — and it is checked
by `aware agent validate`, by both install routes, and again at probe time
(`E_PROBE_INVALID`, with the broken rule as `details.reason`):

- `command` exists, is runnable (`status: available`), has `lifecycle: single`, resolves to
  mode `read`, and is **not** `mode-overridable` (a caller-determined mode proves nothing about
  what a fixed input does — `exec` can never be a probe). Model-extraction and
  artifact-streaming commands are refused.
- `inputs` are literals that are declared by the command, match its declared type, and cover
  every `required` input.
- `kind: account` requires `reports.identity` **and** `reports.stable-id`: an account probe that
  cannot say which account it reached verifies nothing.
- A probe is supported on the `cli` and `rest` transports. A `rest` probe requires
  `rest.origin`, its command must declare `method:` + `path:` with a safe method (`GET` or
  `HEAD` — a command's inferred mode comes from its name, which says nothing about a
  `DELETE`), and the URL that path resolves to must sit on exactly that origin. `rest:` on a `cli` probe is refused.

**Trust is not declared.** A manifest can claim anything, so nothing in it can make a probe
*reviewed*. `aware agent probe` reports `reviewed: true` only when the installed bundle's digest
equals the `bundle-digest` a freshly fetched official registry index records for that
agent@version — the agent arrived unmodified from the registry, whose manifests are reviewed in
`aware-aeco/aware` PRs. A local, built, grafted or edited agent, an offline machine, or a custom
registry is `reviewed: false`.

Generated agents get no automatic probe; `aware build agent … --probe <command>` writes one at
build time, after the same validation, and it is unreviewed.

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

## Engineering envelope (v0.21)

Persona audit (structural engineer, **dealbreaker**): *"There is no answer to 'who is liable when your app gets this wrong.' AWARE must produce a calculation package whose every input is auditable, every code-revision pinned, every solver build hashed, every assumption flagged, and every step replayable by a third party with no access to the running engineer's machine."*

v0.21 introduces the **engineering envelope** — a manifest extension on agents (declares what they pin) + an app-spec extension (binds the pins to a run) + a `signed-output` primitive that produces a chain-of-custody record.

> **Status: schema published, behaviour `planned`.** The `engineering:` blocks below are
> typed — the manifest loader deserializes them on both the agent and the app side, and an
> app's is carried verbatim into the lockfile — but **nothing in the runtime acts on them
> yet**. `signed-output` is weaker still: `Node` has no such field and the app manifest sets
> no `deny_unknown_fields`, so the directive is silently discarded at parse rather than
> rejected. This is `planned` in exactly the sense [§ Runnability](#runnability-status)
> gives the word: the contract is published so agents and apps can be authored against it,
> the implementation is not shipped. Concretely:
>
> | Piece | State |
> |---|---|
> | Agent `engineering.pinnable` | parsed, then never read (`cli/src/manifest/agent.rs`) |
> | App `engineering.pins` | parsed; copied into the lock verbatim — **not** checked against the agent's declared pinnable set |
> | `engineering.output-seal` | parsed; no `.aware-receipt.json` is written |
> | `signed-output` node | **not a node kind** — unmapped on `Node`, so serde discards it silently; not even rejected |
> | `aware.units()` | not implemented — see [Unit-typed numerics](#unit-typed-numerics) |
> | `aware app reproduce` | not wired — see [`cli-roadmap.md`](./cli-roadmap.md) |
>
> Signed receipts themselves *do* ship, as the separate `aware key` + `aware receipt
> sign|verify` command groups (ed25519, v0.26/v0.27). They are simply not reachable from
> this envelope's `output-seal` yet.
>
> Read the rest of this section as the contract the envelope will satisfy, not as a
> description of what `aware app run` does today.

### Agent-side declaration

Engineering agents (`tsd-26`, `idea-statica-26`, `csi-api`, `allplan-2025`, etc.) MAY declare an `engineering:` block:

```yaml
engineering:
  pinnable:
    - id: code-of-practice
      description: Design code + revision + national annex
      required: true
      example: 'eurocode-3@2022+uk-na'
    - id: section-catalogue
      description: Steel section catalogue revision
      required: true
      example: 'bs-4-1@2005'
    - id: material-catalogue
      description: Concrete / steel / timber grade catalogue revision
      required: true
      example: 'en-10025-2@2019'
    - id: psi-factors
      description: Eurocode combination factors (ψ_0, ψ_1, ψ_2)
      required: false
      example: 'en-1990@2002+uk-na-2002'
    - id: solver-build
      description: Underlying solver binary hash
      required: true
      example: 'tsd-26.0.3-build-19834'
```

*Planned:* the agent's transport binary is responsible for resolving these pins at runtime against the actual installed product, and the values appear in the provenance log (per-run record). **Not implemented today** — `pinnable` is parsed and never read, and the provenance writer records no engineering pins.

### App-side declaration

Apps that produce engineer-signed deliverables MUST pin the envelope explicitly:

```yaml
engineering:
  pins:
    code-of-practice:    'eurocode-3@2022+uk-na'
    section-catalogue:   'en-10365@2017'
    material-catalogue:  'en-10025-2@2019'
    psi-factors:         'en-1990@2002+uk-na-2002'
    solver-build:        'tsd-26.0.3-build-19834'
  output-seal:
    artifact:    '{{ calc-pack.path }}'
    operator:    '{{ run.operator }}'
    credential:  '{{ secrets.ceng-seal }}'  # optional digital cert
```

*Planned:* at install time, `aware app install` resolves the pins against the installed engineering agent's declared pinnable values, and a mismatch fails the install with a clear error pointing at which pin doesn't match. **Not implemented today** — the pins are parsed and copied into the lockfile, and no comparison against the agent's `pinnable` set happens at install, compile or run. An app whose pins name values the agent never declared installs and runs exactly as if they matched.

### Unit-typed numerics

**Planned — `aware.units()` does not exist.** Nothing in the runtime or in `atoms/`
implements it, and the example below cannot run for a second, independent reason: the
inline expression engine (`cli/src/runtime/inline.rs`) is a *predicate* evaluator — paths,
literals, `==`, `!=`, `&&`, `||` — with no function-call or arithmetic syntax at all, and
`validate` rejects any inline node whose `kind` is not `predicate` with `E_APP_INLINE_KIND`, a `predicate` carrying no executable `code:` body with `E_APP_INLINE_NO_BODY` (#554) — including one written as an `atom://` reference, since no resolver ships yet — and a top-level `predicate` that reads a field while no `connections:` entry feeds it with `E_APP_PREDICATE_NO_INPUT` (#557).
So an app carrying the `map` node below fails at validate/compile, before any missing
helper could be reached. Shipping this means giving the expression engine calls and
arithmetic first; it is tracked under [v0.21 in the roadmap](./cli-roadmap.md).

The contract the helper is intended to satisfy — the `{{ ... }}` template engine gains
optional unit tags, and values carry SI units that the engine refuses to combine wrongly:

```yaml
- id: load
  inline:
    kind: map
    description: Compute floor load (UK convention)
    code: |
      const dead = aware.units('4.0 kN/m^2');
      const live = aware.units('2.5 kN/m^2');
      return aware.units(dead.plus(live));   // returns "6.5 kN/m^2"
```

Mixing kN/m² with kN/m or m would throw at run time, not produce silent garbage. The expression engine would carry the SI base + derived units library, so that per-discipline conventions (kip, klf, kN/m) round-trip cleanly. Until that ships, an app needing unit-safe arithmetic must do it inside an agent command, where real code runs.

### `signed-output` topology node

**Planned — `signed-output` is not a node kind.** `Node` (`cli/src/manifest/app.rs`) has no
field for it and the app manifest sets no `deny_unknown_fields`, so an app declaring one is
not rejected — the directive is **silently dropped at parse**, and the node seals nothing.
The `engineering.output-seal` block above *is* typed, but is never read, so no
`.aware-receipt.json` is written by a run today either. What *does* ship is the lower-level
pair this node would build on: `aware key` and `aware receipt sign|verify` (ed25519).

The end-of-run step that produces a chain-of-custody record:

```yaml
- id: seal
  signed-output:
    artifact: '{{ calc-pack.path }}'
    envelope: '{{ engineering.pins }}'
    operator: '{{ run.operator }}'
    credential: '{{ secrets.ceng-seal }}'
```

Output: a `*.aware-receipt.json` next to the artifact containing:
- artifact SHA-256
- envelope pins (all of them)
- operator id (CEng/PE reference if provided)
- credential signature (if cert present)
- run id + timestamp
- agent versions used
- expression-engine version

A checker years from now can re-create the receipt against a fresh AWARE install + the same app file + the same pins. Mismatch = the calc is no longer reproducible; investigate.

### What this guarantees (once the envelope executes)

These are the guarantees the envelope is designed to deliver. None of them holds today —
see the status note at the top of this section.

- **Code revisions are visible.** No silent EC3:2005 → EC3:2022 drift.
- **Section catalogues are visible.** No silent BS 4-1 → EN 10365 drift.
- **Solver builds are hashed.** A patch release of TSD that quietly changed an LTB rule shows up as a different hash.
- **Output is sealed.** The receipt JSON is the engineer's accountability artefact.

### What this does NOT guarantee

- That the engineer's assumptions were correct
- That the loads input were the right loads
- That the model topology matched the as-built structure
- That an LLM-composed app didn't misinterpret a clause

Those remain professional-judgment matters — AWARE's job is to make the inputs *visible*, not to make them *correct*.

### Why this exists

Engineer audit, section H: *"My insurer (Zurich, in my case) won't underwrite a calc whose decisive step ran in `aware app run`."* The engineering envelope is the answer — the receipt is what the underwriter sees if they ever ask, "what did this code, this section catalogue, this solver build produce?" and the answer is a SHA + a signed envelope, not "we don't know any more."

---

## Versioning

- **Semver.** `<major>.<minor>.<patch>`.
- **Auto-generated agents** version-track their source. `tekla@2025.0.1` mirrors `Tekla Structures 2025.0` (patch is the agent's own iteration).
- **Apps pin agent versions** by `agent@minor.x` (default), `agent@major.x` (loose), or `agent@exact-semver` (strict). See [App Spec](./app-spec.md).
- **Breaking changes** require a major bump *and* a `BREAKING.md` file in the agent folder describing what moved.

---

## Installation

```bash
aware agent install tekla                       # latest
aware agent install tekla@2025.0.1              # an exact version
aware agent install aware-aeco                  # bundle (installs multiple agents)
aware agent list                                # show what's installed
aware agent describe tekla                      # manifest summary + skill index
aware agent describe tekla --available          # …from the registry, with EVERY version it has
aware agent update tekla                        # re-pull the newest
aware agent update tekla@2025.0.1               # move to a named version — including an older one
aware agent skill tekla drawing-identity        # print the skill file
aware agent uninstall tekla
```

A version after `@` is an **exact registry version**, not a range. `tekla@2025.0.x` is not an install spec: the registry resolves a version by exact key lookup, so a wildcard finds nothing. Ranges are an *app pinning* syntax (`requires:` in [App Spec](./app-spec.md)), which is a different question — "which versions may satisfy this app" rather than "which one do I want on disk". This document advertised the wildcard form for some time and the installer never supported it (#363).

Each registry release binds that public key to the identity inside its payload with two required fields: `manifest-agent` and `manifest-version`. The release key and manifest version are separate axes: for example, a product-facing registry key such as `tekla@2025.0.1` may intentionally bind a manifest whose own substrate version is `0.1.5`. The binding makes that relationship explicit, so a moving or incorrectly republished archive cannot silently turn one exact release into another.

Both installation and update enforce these bindings for official, cached, and custom registries alike. Missing, partial, blank, or invalid bindings refuse before download. After extraction, the payload manifest must declare exactly the bound agent and version or the operation refuses before promotion. Registry agent identities use the same portable form on every operating system: ASCII letters, digits, `.`, `_`, and `-`, beginning with a letter or digit, with no trailing dot or space and no Windows reserved stem. Manifest versions are strict SemVer. The bound `manifest-agent` must be the registry key itself, that key followed by a non-empty dotted suffix (for a versioned implementation such as `allplan-2024.0`), or an explicit `alias-of` target. A rename entry's `alias-of` target must equal its bound `manifest-agent`; the old registry key may differ because that difference is the declared migration.

`update <id>@<version>` is how an installed agent reaches a version that is not the newest — `install` refuses while a copy is on disk, and before #363 `update` took no version, so the only route was `uninstall` then `install`, which destroys a locally-installed agent before failing. The swap is **atomic** (see the journaled swap below): the new copy is fetched and validated before the installed one is touched, so naming a version the registry does not have — or an agent that came from a local folder — refuses and leaves what you have alone.

Installation drops the agent folder under `~/.aware/agents/<name>/` and auto-generates host plugins under `~/.<host>/plugins/aware-aeco/` for each agentic CLI present on the machine.

### Where an agent came from is recorded

Installation also writes `~/.aware/agents/<name>/.aware-install.yaml`, saying whether the agent came from the **registry** (with the separately bound registry key/version, manifest agent/version, expected digest, installed digest, and official-source verdict) or from a **local folder** (with the path). It is metadata about the install, not about the agent — distinct from the manifest's own `provenance:` block, which records how the agent was *authored*. The deterministic `sha256:` digest covers a domain-separated, path-sorted tree of UTF-8 relative paths, lengths, and raw file bytes; the receipt itself is excluded and symlink/reparse indirection is rejected.

Registry authoring hashes the canonical stage-0 Git blob bytes, not checkout-translated bytes (for example CRLF produced by `core.autocrlf`). Staged additions and changes are included; unstaged, untracked, renamed, conflicted, symlink, and gitlink content is refused with an actionable staging error. This binds the release digest to the bytes the official repository archive contains while installed verification independently hashes the raw extracted tree.

A release that depends on code in the AWARE binary uses an immutable GitHub commit archive URL of the exact form `https://github.com/<owner>/<repo>/archive/<40-lowercase-hex-commit>.tar.gz`. Its `subdir` must begin with GitHub's matching `<repo>-<commit>/` archive root. The registry parser rejects mismatched repositories, abbreviated or uppercase object names, branch archives paired with a commit-shaped root, and a pinned URL paired with any other root. Local reindexing reads and hashes that subtree from the named local Git commit, so a later checkout edit cannot rewrite historical catalog metadata. The pinned commit must already be an ancestor of the fetched base/default branch, selected from `GITHUB_BASE_REF`, the pushed `GITHUB_REF_NAME`, `origin/HEAD`, or the explicit `AWARE_REGISTRY_BASE_REF` override. Comparing only with the authoring `HEAD` is forbidden because a pull request's own commits satisfy that check before a squash-and-delete merge makes them unreachable. CI fetches full history before checking these pins.

Only an index fetched fresh from AWARE's exact built-in HTTPS registry endpoint can produce `verified: true`. Overrides (`AWARE_REGISTRY`), file registries, offline/stale caches, local installs, legacy receipts, missing digests, and modified trees report an explicit unverified reason. For a digest-bearing official release, the staged tree and mandatory receipt are verified before atomic promotion; `--force` never bypasses integrity. Verification compares the fresh index's manifest binding with the receipt and the installed manifest in addition to comparing all three digests, so an attested tree cannot be credited to a different semantic release. `aware agent describe` reports this bundle provenance. It deliberately does **not** attest an external PATH/managed executable or remote REST service: those execution surfaces require their own attestation.

`aware agent update` reads it and **refuses to replace a locally-installed agent**, because the registry's copy would overwrite work that exists nowhere else. Pass `--force` to take the registry's version anyway. An install that predates the marker has no record, so `update` infers instead: if the installed version is one the registry publishes it proceeds, and if it is not it refuses and says it is inferring (#370). `--all --force` **skips** the local installs and updates the rest, naming what it left alone (#374). Local/legacy markers remain best-effort and unverified; an official verified receipt is mandatory and transactional.

### Approved versions are kept side by side (the agent store)

`~/.aware/agents/<id>/` is the **current, mutable working copy**: install, update, build and the skill builder rewrite it, and `agent list`, `describe`, `probe` and compile read it. An app approved against one version must not start running another just because the agent was updated, so AWARE also keeps an **immutable, content-addressed store** of every agent version an app was compiled against or that was ever current (#626):

```
~/.aware/agent-store-v2/<id>/<tree-hex>/<receipt-key>/   # one immutable package
    manifest.yaml, skills/, …, .aware-install.yaml      #   the copied tree, receipt included
    .aware-package.yaml                                 #   { agent, version, digest, receipt-key, snapshotted-at }
~/.aware/agent-store-v2/<id>/<tree-hex>/.tmp-<random>/  # an in-progress snapshot; every reader ignores it
```

- `<tree-hex>` is the 64-hex body of the bundle's `sha256:` tree digest (the one above). The digest excludes both the install receipt and `.aware-package.yaml`, so a package's digest equals the working copy it was taken from. `<receipt-key>` is the sha256 hex of the receipt bytes, or `no-receipt`: the same bytes installed from an official registry and from a local folder are two packages with two provenances, never one directory whose receipt depends on install order.
- **Snapshots are the only writer.** A snapshot validates the id (a plain segment) and the digest (`sha256:` + 64 lowercase hex) before forming any path; copies the tree into a temp directory inside the digest container (so the final rename is same-directory); fsyncs every file; re-hashes the copy — its digest and receipt key must still equal those computed before the copy, otherwise the working copy changed mid-copy and the snapshot retries once, then refuses (`E_AGENT_STORE_CHANGED`); and publishes with one rename. Durability: Unix fsyncs the package, digest and id directories; Windows renames with `MoveFileExW(MOVEFILE_WRITE_THROUGH)`. An existing package with the same name is verified (fresh digest, receipt key, manifest identity and `.aware-package.yaml`) and reused; one that does not verify makes the snapshot **refuse**, naming it (`E_AGENT_STORE_INVALID`). AWARE never renames, rewrites, repairs or deletes a store package.
- **When snapshots are taken.** `agent install` and `agent update` (registry and local) snapshot the **staged** tree before promoting it, so a snapshot failure installs or replaces nothing. `agent update`, before removing anything, also snapshots **every** directory the swap would remove — `agents/<id>/` and, for a suffixed or renamed payload, `agents/<new-id>/` too — and any failure refuses the update with `agents/` untouched (an outgoing directory with no loadable manifest has nothing a lock could approve: it is replaced without a snapshot). `app compile` snapshots every agent it pins first and compiles from the stored manifests. `app run` snapshots a pinned current copy that has none yet (an install from before the store existed).
- `aware agent uninstall` removes only the working copy. The store is left alone: its packages become unreachable (a run of an app that needs the agent refuses as not installed) and removing them is garbage collection's job (#629, which relies on the run leases below). Until then packages accumulate — at most one per distinct version an app was compiled against or that was current. `aware agent list --json` shows each agent's `stored: [{version, digest}]`, and `unreadable-stored: [{path, reason}]` for any store entry whose record cannot be read (display only; whether an app runs is `aware app check`'s answer — see [App Spec](./app-spec.md)).
- **A lock is replaced atomically** (#628). `aware app compile` writes `<app>.lock` to a temp file in the same directory, fsyncs it, and moves it over the old lock in one step (Windows `MoveFileExW(MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH)`; Unix `rename(2)` then an fsync of the directory). A run reading the lock at preflight sees the old approval or the new one, never a torn mix. A write that fails before the move leaves the old lock byte-identical; if only the durability step after the move fails (the Unix directory fsync), compile still succeeds — the new lock **is** in effect — and warns that a power loss could bring back the previous one, rather than claiming the write failed.
- **`agent-store-v2/` and the legacy `agent-store/`** (#627-b). AWARE 0.149–0.151 kept the store in `agent-store/` and took no run leases. From 0.153 every store reader and writer uses `agent-store-v2/`, so nothing an older CLI does can depend on a package there — which is what makes it the only store garbage collection (#629) may ever touch. The legacy `agent-store/` is **never written or deleted** by a newer CLI. On first store access (and whenever an older CLI has added a package since — the legacy tree is listed by name, three levels, on every access), each legacy package is verified in place and copied into `agent-store-v2/` under the store lock held exclusive, keeping its `.aware-package.yaml` byte for byte (so its `snapshotted-at` is kept), verified again and published with one no-replace rename; a package that does not verify is not imported and is reported once. The record is `agent-store-control/legacy-import.json`. Hard links are never used. If the two stores are not physically distinct plain directories (a link or junction, or one directory under both names) every store operation refuses with `E_AGENT_STORE_ALIASED`.
- **Run leases** (#627-b). Every real `aware app run` (`--dry-run` included; `--simulate` dispatches nothing from the store and takes none) writes `agent-store-control/leases/<run-id>.lease` — `{format: "aware.agent-lease/v1", run-id, app, instance, pid, started-at, cli-version, packages: [{agent, version, digest, receipt-key, root, via?}]}`, every package the run resolved, backing apps' packages included (`via` names the app-backed agent) — and holds an OS lock on it for as long as the run's process lives. It is written while the store reference lock is held (taken before the run reads its approval), so nothing can see a resolved package without its lease. A lock dies with its process (kill, crash, power loss): a lease file whose lock can be taken is stale — no heartbeat, no expiry, no PID guess. `aware agent leases [--json]` lists `leases` (live), `stale` and `unreadable` ones; read-only. `aware agent uninstall <id>` during a run says how many runs still use it; they keep running on their stored copy.
- **Last-needed stamps** (#627-b). `agent-store-control/refs/<id>/<tree-hex>.last-needed` records when a package was last needed by something that let go of it: a run acquiring and releasing its lease, a working copy swapped out, a lock replaced by `app compile` or `app migrate promote|revert`, an app uninstalled, a candidate discarded. It only moves forward (serialized per digest by `<tree-hex>.flock`). A released lease file is deleted only after its packages are stamped. A stamp is availability, never correctness: GC (#629) keeps an unreferenced package for its recovery window counted from `max(snapshotted-at, last-needed)`.
- **The reference table** (#629-a). `aware agent refs [--recovery-window <n>{s,m,h,d}] [--json]` lists every package in `agent-store-v2/` with every reason something still needs it, and so what garbage collection may do with it. Read-only. A package is **kept** while anything references it without expiry: its agent's working copy has exactly those bytes (`current`; a working copy that cannot be hashed keeps *every* package of that agent, `current-unhashable`), an approved `<app>.lock` pins it (`approved-lock`; the pin rule a run uses: `agent-digests`, else `agent-bundle-pins` — a version-only pin references nothing stored), a migration candidate pins it or moves a pin away from it (`candidate-lock`, `candidate-base`), a promotion that has not finished stages it (`promotion-in-progress`), a run holds it (`lease`), or a run ended without releasing its lease (`stale-lease-unstamped`, until GC stamps it and removes the lease). It is **in-window** while only expiring references remain: an archived approval (`.aware-approvals/<hex>.lock`, from the time it was archived), the original approval or a successor's `from` pins in a carried-forward lock's record (`approval-original`, `successor-from`, from the promotion that replaced them), and its own recent use (`recent`, from `max(snapshotted-at, last-needed)`). Those keep it for the recovery window, and for as long as a `HOLD.<app>` exists in the folder. Otherwise it is **removable**. The window is `--recovery-window`, else `agent-store.recovery-window` in `config.yaml`, else 30 days; a malformed one is an error (`E_AGENT_REFS_WINDOW_INVALID`), never a silent default.
  - **Where locks are found.** Every folder under `AWARE_HOME/apps/` and under every folder a front door registered with `aware agent refs roots add <dir> [--label <who>]` (`agent-store-control/roots.yaml`; `roots remove <dir>`, `roots list`), walked up to 8 levels without following links or junctions (reported as `links-not-followed`) and skipping dot-folders, `node_modules`, `target`. In each folder: its `*.lock` files when an app source (`.flo`/`.app`) sits beside them, `.aware-migration/*.candidate.lock` and `*.evidence.json`, `.aware-approvals/*.lock` and `.aware-approvals/.txn/*/*.lock`. A lock AWARE cannot see is not protected: after GC its pin is reported not installed, and nothing else is ever run in its place.
  - **It fails closed.** Anything it could not read that might have held a reference is a blocker and makes the table `complete: false`: a lock, candidate, archive or staged lock that does not parse, an approval record of a format this AWARE cannot read or whose chain is inconsistent, a folder or the store that cannot be listed, a registered folder that is missing, a run lease that cannot be read, a malformed `roots.yaml`. GC removes nothing while the table is incomplete. The command itself still succeeds: it is a report.
  - A package that fails verification is listed under `invalid-packages`: kept as evidence while referenced, otherwise removable after the window. Interrupted snapshots (`.tmp-*`), interrupted removals (`.trash-*`) and unrecognised names are listed under `leftovers`; unrecognised names are never removed. The legacy `agent-store/`'s size is reported under `legacy-store`.
- Threat model: the store protects approved bytes from AWARE's **own** writers. Hand-editing files under `agent-store-v2/` (or the legacy `agent-store/`) is out of scope, exactly as hand-editing `bridges/` is; such an edit is caught by the next run's digest check, which refuses rather than runs other bytes. On a power loss before Windows has made the rename durable, the worst case is that an outgoing package is missing: the run then refuses with `E_APP_LOCK_AGENT_PIN_MISMATCH` (compile again), never runs different bytes.

### Installs, updates and uninstalls swap the working copy atomically (#627)

Every writer of a working copy — `agent install` (registry and local), `agent update` (including the second directory of a suffixed or renamed payload), `agent uninstall`, and the synthesized agent an `exposes-as-agent` app writes or removes on `app install`, `app uninstall`, `app rename` and `app duplicate` — changes `agents/<id>/` through one **journaled swap**. `agents/<id>/` is therefore always a complete old tree or a complete new tree: never a half-deleted or half-copied one.

```
~/.aware/agents/.aware-swap/<txn>/          # one swap; same volume as agents/<id>, two levels deep
    incoming/                               #   the new tree: staged, digest-checked, receipted, snapshotted
    intent.json                             #   ids (sorted), new name, incoming digest, outgoing ids + digests
    journal.log                             #   `pending <step>` before each rename, `done <step>` after
    outgoing-<id>/                          #   an old tree, moved aside whole
~/.aware/agents/.aware-swap/locks/<id>.flock # the agent's swap lock
~/.aware/agent-store-control/store.flock    # the store reference lock
```

- **Staging is per swap and on the same volume.** The new tree is staged in a fresh `incoming/` (never a shared `cache/` staging name), so two installs of one id cannot delete each other's staging, and the final move is always a same-volume rename. Staging, digest checks, the receipt and the snapshot of the new tree all happen before anything under `agents/` is touched; any failure removes the staging and changes nothing.
- **Order.** The swap takes the swap lock of every id it touches, **exclusive, in sorted order** (locks are keyed by the case-folded id, so on a case-insensitive filesystem `Alpha` and `alpha` are one directory and one lock). A directory is replaced only if the name the swap was given resolves to it on this filesystem — compared by filesystem identity, never by a case-folded guess — so on a case-sensitive filesystem an unrelated `agents/Alpha` is never touched by `update alpha`; every directory selected is judged by the update's local-install check under its own name, and only then re-checks what it is about to replace (an install re-checks the id is still free; an update re-runs its "would this destroy a local install?" check for both directories) and snapshots every outgoing copy. It then writes `intent.json` (temp file + rename, fsynced), and for each outgoing id writes and fsyncs `pending out <id>`, moves `agents/<id>` to `outgoing-<id>` (a no-replace rename: Windows `MoveFileExW(MOVEFILE_WRITE_THROUGH)`, Unix `rename(2)` after checking the name is free, then fsyncs of both directories), and writes `done out <id>`; then `pending in <name>`, the move of `incoming/` to `agents/<name>`, `done in <name>`; then `commit`, and deletes the transaction (and with it the old copies). An uninstall has no `incoming/`; it commits once its copy is moved aside.
- **A failed rename puts everything back.** If any move fails (an open handle on Windows, a permission fault), every copy already moved aside is renamed back (`pending restore` / `done restore`), the staging is dropped, and the command fails with the original error; `agents/` is as it was. If a rename succeeded and what follows it fails (a directory sync, a journal line), the swap settles itself the same way recovery would before it returns the error, so `agents/<id>` is never left moved aside. A finished transaction is retired in one step (renamed to `.done-<txn>`, a name no reader treats as a swap) before it is deleted, so a kill during the delete can never make it look unfinished again; `aware doctor` removes leftover `.done-*` directories.
- **Recovery.** A swap interrupted by a crash, a kill or a power loss is recovered by the next command that takes any of its swap locks — an install, update or uninstall of one of its ids, a run or compile that reads one of them, or `aware doctor` — holding the swap locks of the transaction's **whole id set**. Recovery does not trust the journal alone: each `pending` step without its `done` is reconciled from the paths. The swap **commits** if `commit` was recorded, or the move-in physically happened (the incoming tree is gone and `agents/<name>` hashes to the intent's incoming digest), or — for an uninstall — every outgoing tree is out of `agents/`: the moved-aside copies are deleted. Otherwise it **rolls back the whole set**: every moved-aside copy is renamed back, the incoming tree is dropped. A state recovery cannot explain (both the moved-aside copy and `agents/<id>` present, a moved-aside copy missing, `agents/<name>` hashing to neither tree) refuses with `E_AGENT_SWAP_RECOVERY`, names the transaction, and changes nothing. A finished transaction whose final delete failed is inert and is removed later.
- **Readers.** A run's preflight (while it hashes and snapshots a pinned working copy) and `app compile` (while it snapshots the agents it pins) hold the agent's swap lock **shared**, after first recovering any interrupted swap naming that agent. So a run or compile never reads a tree that is being replaced, never needs `aware doctor` or another writer to see an interrupted agent again, and — for a lock compiled before digest pins existed, which runs a snapshot of the working copy — never snapshots a partial tree. A writer waits for readers holding the lock, and a reader waits for a swap in progress; a swap holds the lock for milliseconds (the renames), not for the download or the snapshot. `aware app check` holds the store reference lock shared (so the approval, its archives and the store packages it judges are one consistent view) but takes no swap lock; when an interrupted swap names an agent it checks, it first finishes that swap exactly as the run would, so it never reports an agent "not installed" that the next run would recover and run. Readers that take no lock — `agent list`, `agent describe`, a front door reading `agents/` directly — may, for the instant between a swap's two renames, see the agent absent; they never see a partial tree.
- **The store reference lock.** Every operation that creates or relies on a reference into the agent store — a snapshot, an install, update or uninstall, `app compile` (with or without `--front-door`) and `app inspect`, `app check`, every `app migrate` verb, `app install`, `uninstall`, `rename` and `duplicate`, the recovery in `aware doctor`, and `app run` from **before** it reads the app's source and `<app>.lock` until every pinned agent is resolved to a verified store package — holds `agent-store-control/store.flock` **shared**. Only garbage collection (#629) will take it exclusive, and only with a bounded wait, so it never blocks a run. **Lock order everywhere: the store lock first, then swap locks in sorted id order.** The swap-lock API requires a held store guard, so the order is enforced where the code is compiled. All locks are OS file locks that die with the process: a crash or kill never leaves anything locked. Lock files are named `*.flock`, never `*.lock`, which in AWARE always means an approval.
- `agents/.aware-swap/` is never an agent: discovery skips it, and no agent id can name it.

### The executable contract of a pin (`aware.contract-diff/v1`)

When an approved workflow could move from one stored version of an agent to another (#628), the question is not "did the package change" — every release changes its version and changelog — but "would a run of THIS workflow hand the executor different instructions". An agent package is mostly not executable code: a CLI agent's bridge program belongs to the installed bridge, and REST/builtin agents are executed by AWARE's own code from manifest fields. So the **executable contract** of a pin, for one workflow, is:

1. **The run-relevant manifest projection**, compared on the raw YAML value — so a key this CLI does not know is compared too (fail safe) — with keys in canonical form (sorted at every depth). Excluded, because `aware app run` never reads them: `version`, `display-name`, `description`, `keywords`, `homepage`, `vendor`, `license`, `provenance`, `skills`, `probe`. `probe` is read only by `aware agent probe`; a change to it is reported as `probe-changed` and not counted. A unit test scans the run path (`cli/src/runtime/`, except the probe's own module) and fails if it starts reading any excluded key, with a negative control proving the scan finds such a read.
2. **The called commands**: each command a non-frozen node of the workflow calls, compared whole except its top-level `description` — schemas, `mode`, `mode-overridable`, `lifecycle`, `status`, `category`, `method`, `path`, `response`, `no-auth`, `model-extraction`, and any key not listed here. A command no node calls is reported under `ignored.commands-not-called` and not counted.
3. **The executable files**: the sha256 of every package file except `manifest.yaml` (covered above) and a documentation allowlist — `skills/**`, `commands/**/*.md`, the root-level files `CHANGELOG.md`, `README`, `README.*`, `LICENSE` and `LICENSE.*` (a directory named `README` or `LICENSE` is not documentation), and the install metadata `.aware-install.yaml` / `.aware-package.yaml`. Everything else counts, `atoms/` included. Documentation differences are reported under `ignored.doc-files`.
4. **The executor identity**, as dispatch resolves it at evaluation time — for a CLI agent, the program `aware app run` hands to the operating system (the same function decides it for both): a managed bridge in `<AWARE_HOME>/bridges`, a bundled transport next to `aware`, or `transport.cli.binary` as written. Bytes are claimed **only where AWARE fixed the file** (`resolution: fixed-path` — a bridge, a bundled transport, or an absolute `binary:`), whose sha256 is recorded. A bare name the operating system searches for (`os-search`) or a relative path it resolves against the run's working directory (`relative-to-cwd`) is reported as written with `sha256: null` and the detail "resolved by the operating system at run time; not pinned": AWARE makes no byte claim for a file it does not choose, and two pins naming the same program the same way have the same executor. `aware-cli <version>` for REST and builtin agents; the backing app for an app-backed agent. If the old pin would resolve to a different executor, agent-level `executor` is reported as changed.
5. **The plan changes**: compiled-node fields of the moved agent's nodes — `mode`, `output-schema`, `runtime-model`, `model-pin` — that differ between the approved lock and a candidate lock.

The contract is **unchanged** iff (1), (2), (3) and (5) show no difference and the executor is the same. One diff per moved agent:

```json
{ "format": "aware.contract-diff/v1", "agent": "tekla",
  "from": {"version": "0.1.5", "digest": "sha256:…"}, "to": {"version": "0.1.6", "digest": "sha256:…"},
  "unchanged": true,
  "agent-level": {"changed": []},
  "commands": [{"command": "exec", "nodes": ["read-model"], "unchanged": true, "changes": []}],
  "executable-files": {"added": [], "removed": [], "changed": []},
  "probe-changed": true,
  "executor": {"kind": "cli", "binary": "aware-tekla", "program": "…/bridges/aware-tekla.exe", "resolution": "fixed-path", "sha256": "sha256:…"},
  "plan-changes": [],
  "ignored": {"doc-files": ["CHANGELOG.md", "…"], "commands-not-called": ["model-info"]} }
```

`unchanged` says the run instructions are byte-identical by inspection — nothing was executed. It is never a claim that results are the same on any model or account. Comparing two packages reads only verified store packages; nothing is written.

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
