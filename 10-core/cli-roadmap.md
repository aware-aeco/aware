# CLI Roadmap

Phased delivery for the `aware` CLI. Each phase is a real shipping unit — the binary that comes out is useful even if the next phase never lands.

The phases compound. Phase N requires everything Phase N−1 shipped. Don't skip ahead.

---

## v0.1 — Read-only foundation

**Goal:** a single binary that explores everything already in `~/.aware/`. No installs, no execution, no network. Pure exploration.

**Why first:** validates the manifest + skill loaders against real fixtures (the 7 agents + 2 apps already in the repo). Lays the Rust scaffolding without touching async, network, or auth.

**Commands**:
- `aware --version`
- `aware --help`
- `aware doctor` (limited — config + filesystem only)
- `aware agent list`
- `aware agent describe <agent>`
- `aware agent skill <agent> <skill-name>`
- `aware app list`
- `aware app show <app>` (ASCII topology + provenance)

**Internal modules to land**:
- `cli::args` — clap derive-based parser
- `cli::config` — load / write `~/.aware/config.yaml`
- `cli::paths` — resolve `~/.aware/...` cross-platform
- `cli::manifest` — `Agent`, `App`, `Skill`, `Command` structs + `serde_yaml` deserialization
- `cli::render` — table + tree formatters for human output
- `cli::envelope` — JSON envelope (`Ok`/`Err`/`meta`) for `--json`

**Definition of done**:
- `cargo build --release` produces `aware` (or `aware.exe` on Windows)
- All commands exit 0 on the fixtures already in this repo
- Integration tests exercise every command surface
- `aware doctor` reports the 7 installed agents and 2 apps correctly
- README quickstart shows the install + first-run flow

**Realistic effort:** 1–2 days of focused work.

---

## v0.2 — Install + validate

**Goal:** users can install agents and apps from sources. Schema validation catches errors before broken artifacts land on disk.

**Why second:** without install, the CLI only works against hand-placed files. Install + validate unblocks distribution.

**Commands added**:
- `aware agent install <agent>[@version]` (from registry index OR local path)
- `aware agent install <bundle>` (e.g. `aware-aeco`)
- `aware agent uninstall <agent>`
- `aware agent update <agent>`
- `aware agent validate <path>`
- `aware app install <path-or-name>`
- `aware app uninstall <app>`
- `aware app validate <path>`

**Internal modules added**:
- `cli::registry` — fetch + cache registry index from GitHub
- `cli::install` — clone (agent) / copy (app) into `~/.aware/`
- `cli::validate` — schema checks (cycles, missing fields, capability mismatches)
- `cli::lockfile` — write `lockfile.yaml` per installed app pinning resolved agent versions

**Definition of done**:
- Installing `aware-aeco` from this very repo populates `~/.aware/agents/` correctly
- Validators reject:
  - Missing required manifest fields
  - Cycles in app `connections:`
  - App-declared agent versions not satisfiable by installed agents
  - Skills referenced in manifest but absent from `skills/`
- `aware doctor` now also checks installed-agent integrity

**Realistic effort:** 2–3 days.

---

## v0.3 — Runtime (the hard one)

**Goal:** apps actually execute. The substrate stops being documentation; it becomes operational.

**Why third:** the highest-leverage feature. After this, the demo in the manifesto literally works.

**Commands added**:
- `aware app run <app>` — execute the topology
- `aware app stop <app>` — terminate running app
- `aware app logs <app>` — read execution traces

**Internal modules added**:
- `cli::runtime::orchestrator` — load `.flo`, build the topology graph, schedule execution
- `cli::runtime::template` — `{{ }}` substitution against inputs / upstream outputs / secrets
- `cli::runtime::lifecycle` — start / stop stateful agents, marshal events between nodes
- `cli::runtime::provenance` — write `.jsonl` execution traces under `~/.aware/logs/`
- `cli::runtime::agent_invoker` — call into agent transports (CLI binary or MCP server)

**Definition of done**:
- `aware app run welded-to-tc` executes against a mocked Tekla + TC and produces a valid trace
- Stateful agents (tekla-watch) get `start` lifecycle hook; Ctrl+C cleanly stops them
- Fan-in topology (qa-drawings-to-tekla) sync-joins inputs correctly
- Templating substitutes upstream outputs and secrets (via `~/.aware/credentials/`)
- Provenance trace is replayable: `aware app logs <app> --replay <run-id>` shows what happened

**Realistic effort:** 4–6 days. The runtime is the substantive engineering. Test it against the two reference apps before considering it done.

---

## v0.4 — Auth + host plugins

**Goal:** credentials flow works end-to-end. Host plugins auto-regenerate so claude-code / codex / opencode users see the AWARE agents without manual setup.

**Why fourth:** runtime works against mocked services in v0.3; this phase makes it work against real OAuth-protected endpoints. And the host-plugin generators close the "AWARE shows up in your terminal AI" loop.

**Commands added**:
- `aware connect <integration>` — PKCE OAuth flow with localhost loopback
- `aware connect <integration> --refresh` — force refresh
- `aware disconnect <integration>`
- `aware plugins regenerate` — rebuild host plugin folders from installed agents

**Internal modules added**:
- `cli::auth::pkce` — auth-code + PKCE flow, localhost callback listener
- `cli::auth::keychain` — encryption via OS keyring (Mac Keychain / Windows DPAPI / Linux libsecret)
- `cli::auth::refresh` — transparent refresh in the runtime path
- `cli::plugins::claude_code` — generate `~/.claude/plugins/aware-aeco/`
- `cli::plugins::codex` — generate `~/.codex/plugins/aware-aeco/`
- `cli::plugins::opencode` — generate `~/.opencode/plugins/aware-aeco/`

**Definition of done**:
- `aware connect trimble-connect` survives a real OAuth flow against `app.connect.trimble.com`
- Tokens encrypted at rest; never leaked to logs
- `aware app run` with a real `aware connect`-ed integration completes a real upload
- Host plugin regeneration is idempotent + diff-stable

**Realistic effort:** 2–3 days.

---

## v0.5 — Builders

**Goal:** `aware agent build` and `aware skill ...` from the meta-primitive specs become functional. The substrate becomes self-growing.

**Why last:** these are the highest-value features for the long-term ecosystem, but they require the rest of the substrate (validation, install, runtime, auth) to be in place to test against.

**Commands added**:
- `aware build agent --from-dlls <path>`
- `aware build agent --from-nuget <pkg>[@version]`
- `aware build agent --from-openapi <url-or-path>`
- `aware build agent --from-cli <binary>`
- `aware build agent --from-com <progid>`
- `aware build agent --from-headers <path>`
- `aware build agent --from-python <module>`
- `aware build agent --decompile` (license-checked)
- `aware skill create <agent> <skill-name>`
- `aware skill port <source> <target-agent>`
- `aware skill modify <agent> <skill-name>`
- `aware skill eval <agent> <skill-name>`

**Internal modules added**:
- `cli::builder::dlls` — PowerShell reflection wrapper (Windows-only path; gated)
- `cli::builder::nuget` — fetch package, unzip, extract DLLs + XML docs
- `cli::builder::openapi` — parse spec, generate command shells
- `cli::builder::decompile` — invoke `ilspycmd`, AI-summarize bodies, never retain decompiled source
- `cli::skill_builder` — delegate to Anthropic `skill-creator`, apply AWARE conventions

**Definition of done**:
- `aware build agent --from-nuget Tekla.Structures.Model@2025.0.0` regenerates the Tekla agent's API-reference skills (round-tripping the round-2.6 ports)
- `aware skill port` produces an output identical to (or better than) the manual port done in round 2.6
- Eval mode reports trigger accuracy
- License-aware extraction blocks decompilation on proprietary packages

**Realistic effort:** 4–5 days. Heavy on integration; lots of moving parts.

---

## v1.0 — Stable

After v0.5, all five spec documents (decalog, manifesto, agent-spec, app-spec, cli-spec) and the CLI implementation are frozen for at least one major version.

**Required for v1.0**:
- All five v0.x phases shipped
- 100+ skills across the agents
- At least one external contributor agent merged to the registry
- Public install paths working on Windows (winget), Mac (brew), Linux (curl-pipe)
- A real third-party app published to the registry by someone other than the AWARE team

**Not required for v1.0**:
- The FloLess canvas (separate project)
- A hosted registry (GitHub-hosted is sufficient)
- Cloud execution (local-first is the thesis)

---

## Anti-roadmap (things we are NOT building)

These come up; the answer is no. Documented here so they don't keep coming up.

| Idea | Why no |
|---|---|
| A new chat host competing with claude-code / codex | Decalog #4: ride existing CLIs. Building yet another chat is fragmentation. |
| A proprietary protocol competing with MCP | Decalog #3: don't gate. MCP is the transport; AWARE is the AECO domain layer. |
| A hosted runtime as the default | Decalog #2: AI is the runtime. Hosted execution is an optional add-on later; local-first is the substrate. |
| A graphical app authoring canvas inside the CLI | Out of scope. The CLI is for execution + management; FloLess (separate project) is the visual authoring canvas. |
| Cloud-stored credentials | Decalog #4 + security. Credentials encrypted on the user's machine, period. |

---

## Notes for the implementer

Read [`cli-spec.md`](./cli-spec.md) before writing any command. Read the [`decalog`](../00-vision/decalog.md) when you're tempted to add a feature that "would be nice." Read the parent [CLAUDE.md](../CLAUDE.md) for the engineering rules that apply across every session.

The shape of `~/.aware/` is documented in `cli-spec.md` — never invent new directories without updating both the spec AND this roadmap.

When a phase feels long, that IS the work. Don't ship a half-implemented phase and call it v0.3.
