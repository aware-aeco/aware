# CLI Roadmap

Phased delivery for the `aware` CLI. Each phase is a real shipping unit — the binary that comes out is useful even if the next phase never lands.

The phases compound. Phase N requires everything Phase N−1 shipped. Don't skip ahead.

---

## Implementation status — 2026-05-20

**The phase write-ups below are the original plans. The implementation has run well ahead of the narrative** — the `aware` CLI now ships command surfaces through ~v0.27, and most content phases are done. This section is the authoritative status; the per-phase sections are kept for their design rationale, not as a live to-do list. Verified against `cli/src/main.rs` dispatch, `validate.rs`, `runtime/`, the agent catalogue, and `30-apps/_examples/`.

| Phase | Status | Evidence / note |
|---|---|---|
| v0.1 foundation | ✅ shipped | all read-only commands real |
| v0.2 install + validate | ✅ shipped | `agent`/`app install\|uninstall\|validate`, lockfile |
| v0.3 runtime | ✅ single-shot | `app run\|stop\|logs`, orchestrator + provenance; streaming/stateful-agent transport (`invoke_stream`) still stubbed |
| v0.4 auth + plugins | ✅ shipped | `connect`/`disconnect`, plugin generators |
| v0.5 builders | ✅ shipped | `build --from-*`, `skill` group (`--from-com` is a Windows-only guard) |
| v0.10 curation framework | ✅ shipped | `category:`, `tree\|search --curated` |
| v0.11 safety contract | ✅ shipped | `safety:` block + `E_APP_WRITE_WITHOUT_SAFETY`, `app explain`, `--dry-run` (CLI `app rollback` not yet wired) |
| v0.12 comms depth | ✅ shipped | microsoft-365 23 verbs + google-workspace 24 (at target); full per-verb command-doc coverage |
| v0.13 SSO auth | ✅ shipped | device-code grant (`auth/device.rs`) |
| v0.14 coordination | ✅ shipped | navisworks, solibri, bcf-file, ifc-inspector present |
| v0.15 document control | ✅ agents present | bluebeam-studio, procore, aconex, acc-docs (curation depth varies — see backlog) |
| v0.16 fabricator | ✅ agents present | tekla-powerfab, peddinghaus-translator |
| v0.17 visualization | ✅ shipped | rhino-8/grasshopper-8 curated; enscape-prep, twinmotion-prep |
| v0.18 spec authoring | ✅ agents present | nbs-chorus, avitru-speclink, csi-masterformat |
| v0.19 substrate primitives | 🟡 partial | parsed + validated, but only `assert:` executes at runtime — for-each/compare/sweep/approve/snapshot/model-lock (+ schedule) return `NotYetImplemented` pending v0.19.x |
| v0.20 named atoms | ✅ shipped | 33 atoms — 20 cross-cutting (`atoms/`) + 5 tekla + 8 revit (matches app-spec) |
| v0.21 engineering envelope | ✅ shipped | agent-spec `engineering:`, `signed-output`, units (CLI `app reproduce` not yet wired) |
| v0.22 persona apps | ✅ shipped | all 5 reference `.flo` present |
| v0.23 decalog #9 | ✅ shipped | "AI composes the plan; deterministic code is the plan" |
| v0.24 lockfile + Glass Box | ✅ shipped | `app compile\|inspect`, `app_lock.rs` |
| v0.25 voice packs | ✅ shipped | `voice` command group |
| v0.26/v0.27 signed receipts | ✅ shipped | `key` + `receipt` command groups (ed25519) |
| vendor `*.exec` | 🟡 4 of N | tekla, rhino, revit, sketchup live; allplan/archicad/navisworks catalog-only |

### Actually remaining (the real backlog)

1. **Catalogue curation tail** — the persona-audit #1 gap. Cleared so far: `slack`, `acc-issues`, `acc-account-admin`, `aps-data-management` (now have craft skills); `microsoft-365` + `google-workspace` now have full per-verb command docs. Still open for the largest reflected agents: `csi-api` (4282 cmds / 2 skills), `autocad-2026` (~4.4k / ~77), `revit-2026` (~7.6k / ~53).
2. **Command-doc coverage** — a cross-agent curated-grade polish: many curated verbs still lack a `commands/<verb>.md` (e.g. tekla 3 docs/6 verbs). Not phase-specific.
3. **Vendor `*.exec` fan-out** — Allplan, Archicad, Navisworks (blocked on host install/license).
4. **`aware agent publish`** — the one remaining CLI stub.
5. **`aware connect` providers** — Autodesk/APS + Slack aren't wired into `aware connect` (only trimble-connect/microsoft-365/google-workspace); their agents' auth is token-out-of-band until added.
6. **Agent dedup** — sketchup + navisworks each appear under both `architecture/` and `construction/`; many version-pairs. Needs a consolidation decision.
7. **Runtime execution gaps** — `CliInvoker::invoke_stream` (streaming/stateful-agent transport) and the remaining v0.19 primitives return `NotYetImplemented`. Done at runtime: `assert:` (v0.19), `for-each` (#124), `compare` (#127). Remaining primitives: `sweep` / `approve` / `snapshot` / `model-lock`. Single-shot + for-each + compare apps run fully; these complete the runtime.
8. **v1.0 gates** — external contributor agent, public install paths (winget/brew), a third-party registry app.

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

# Post-v0.5 — Practitioner-driven phases

After v0.5 shipped, five practitioners (BIM manager, designer, architect, structural engineer, detailer) audited the substrate end-to-end on 2026-05-17. Their consolidated feedback lives in [`99-feedback/2026-05-17-persona-audit/CONSOLIDATED.md`](../99-feedback/2026-05-17-persona-audit/CONSOLIDATED.md). The phases below are derived from that audit, ordered by which gaps block adoption hardest.

The bones (text composition, decalog discipline, CLI ergonomics, hand-curated Tekla agent) are sound. The catalogue is ~25% done relative to the manifesto's pitch, the architecture-side agents are reflection-grade not workflow-grade, the runtime has no safety contract for live-model writes, and the fabricator-downstream half of AECO is invisible. These phases close those gaps.

---

## v0.10 — Curation framework + first wave (Revit)

**Goal:** establish the `category: curated | reflected` distinction so the catalogue can hold both hand-written workflow verbs and auto-generated API-method coverage without diluting search. Hand-curate ~10 Revit workflow verbs as the exemplary pattern.

**Why first:** unanimous audit finding #1. Auto-generated agents with 7,647 commands are unusable as shipped; the Tekla curated agent (31 craft skills) proves curation works.

**Spec changes:**
- Extend `agent-spec.md` with a `category: curated | reflected` field on commands
- Define `inputs:` / `outputs:` schema requirement for curated commands (so AI composes deterministically)
- Document the curation pattern + how to promote a reflected command to curated

**CLI changes:**
- `aware tree <agent> --curated` filters to curated commands only
- `aware search <term> --curated` same
- `aware agent describe <id>` highlights curated count vs reflected count

**Content (first wave on revit-2026):**
- `sheet.list` — filter by IssuedFor / Revision / Discipline
- `sheet.export-pdfs` — batch PDF export with naming template + PDF/A
- `sheet.stamp` — date / revision / transmittal stamp on PDF set
- `revision.bump` — next revision per sheet, sets dates, locks revision
- `view.capture-bitmap` — named view → PNG at resolution
- `schedule.export` — schedule to CSV/Excel
- `schedule.find-rows-missing` — QA pass for missing field values
- `element.by-parameter-value` — typed filter over `FilteredElementCollector`
- `parameter.bulk-write` — Excel-keyed-by-mark write
- `link.reload-all` — reload-all-links-and-report-status

**Realistic effort:** 3–4 days.

---

## v0.11 — Safety contract for live-model writes

**Goal:** make write-mode nodes refuse to run unless they declare an explicit `safety:` block — transaction-group, snapshot-before-write, rollback strategy, worksharing pre-flight, UDA audit stamp. Add `--dry-run` mode.

**Why second:** unanimous audit finding #2. Five out of five personas refused live writes today. Without this, AWARE stays read-only.

**Spec changes:**
- Extend `app-spec.md` with `safety:` block on write-mode nodes (required for any node that mutates external state)
- `safety.transaction-group: <name>` — wrap all writes in one rollback-able transaction
- `safety.snapshot: <path>` — eTransmit / save-as before any write
- `safety.worksharing: { check: true, fail-if-other-user: true }` — Revit-style pre-flight
- `safety.audit-stamp: { uda-prefix: AWARE_ }` — touched objects get `AWARE_RUN_ID`, `AWARE_APP`, `AWARE_OPERATOR` UDAs
- `safety.rollback-token: <expr>` — destructive commands return tokens; CLI can replay-in-reverse

**CLI changes:**
- `aware app run <app> --dry-run` — preview proposed transactions without committing
- `aware app rollback <app> --run-id <id>` — undo a previous run using the rollback token chain
- `aware app explain <app>` — pretty-print: reads X, writes Y, sends to Z, requires permissions [...]

**Engine changes:**
- Reject write nodes missing `safety:` at validation time
- Pre-flight worksharing checks before any write attempt
- Snapshot to `~/.aware/snapshots/<app>/<timestamp>/` before write nodes run

**Realistic effort:** 4–5 days. Heavy spec + engine work.

---

## v0.12 — Cross-cutting comms depth (Microsoft-365 + Google-Workspace)

**Goal:** every persona's Monday-morning report flows through Teams / Outlook / Planner / Calendar / SharePoint / Excel ranges / Gmail / Drive / Sheets / Chat. Current `microsoft-365` agent has 4 commands; expand to ~30. Mirror on `google-workspace`.

**Why third:** unanimous audit finding #3. Universal across personas — the comms layer every killer app needs.

**Content (microsoft-365):**
- Teams: `chat.post-message`, `channel.post-message`, `channel.post-with-card`, `channel.post-with-screenshot`
- Outlook: `mail.send`, `mail.send-with-attachment`, `mail.search`, `calendar.create-event`, `calendar.find-availability`
- Planner: `task.create`, `task.list`, `task.assign`, `task.complete`
- SharePoint: `list.read`, `list.write`, `folder.list`, `folder.upload`
- Excel: `range.read`, `range.write`, `worksheet.list`, `table.append-row`

**Content (google-workspace):** parallel set — Gmail, Drive, Sheets, Chat, Calendar, Tasks.

**Realistic effort:** 2–3 days. Mostly OpenAPI-driven scaffolding + curated commands.

---

## v0.13 — SSO-friendly auth + honest install story

**Goal:** make `aware connect` survive enterprise SSO flows. Document the install paths honestly so enterprise IT understands which one to use.

**Why:** half of the audit's "IT department blocks adoption" finding. The other half — Authenticode-signed MSI — is **deferred** at user's direction: AWARE is an open-source project and the npm / curl-pipe install paths already bypass MSI entirely. Code-signing the MSI is operationally expensive (cert procurement) and not strictly required for the project to work. Tracked as a parked follow-up; if it ever becomes blocking, SignPath.io's free OSS tier is the route.

**Code changes:**
- `aware connect <integration>` adds `--tenant-sso` flag that opens default browser to the tenant SSO endpoint, bypasses dev-console requirement
- New `aware connect <integration> --device-code` for headless / IT-managed workstations (RFC 8628 device authorization grant)
- Document the SSO patterns per integration (M365, Google Workspace, ACC, Trimble Connect) in `cli-spec.md`

**Doc changes:**
- `cli/README.md` ranks install paths: **npm/pnpm/yarn/bun (recommended)** → curl-pipe → PowerShell → MSI (last; mention SmartScreen warning + click-through)
- Manifesto links to the curl-pipe install for "enterprise without IT escalation"

**Realistic effort:** 1–2 days. Pure code + docs.

---

## v0.14 — Coordination toolchain

**Goal:** Navisworks Manage 2026 + Solibri Office + BCF 3.0 file format + IFC inspector (IfcOpenShell-style). Half of BIM-manager day-to-day.

**Why:** BIM-manager dealbreaker.

**Agents to add:**
- `navisworks-2026` — built via `aware build agent --from-nuget Autodesk.Navisworks.Api@2026`
- `solibri-office` — REST API (Solibri Anywhere has docs); plus curated rule-set workflow verbs
- `bcf-file` — file-format agent (read+write BCF-XML 2.1 / 3.0). Pure Rust, no host software needed.
- `ifc-inspector` — wraps `web-ifc`'s parser for entity-count, GUID list, Psets extraction (separate from `web-ifc` viewer agent)

**Realistic effort:** 3–4 days.

---

## v0.15 — Document control

**Goal:** Bluebeam Revu Studio + Procore (architect side) + Aconex + ACC Docs (not just Issues).

**Why:** architect + BIM-manager dealbreakers.

**Agents to add:**
- `bluebeam-studio` — REST API (Bluebeam Studio Prime)
- `procore-architect` — REST API, Drawings + Submittals + RFIs + ASIs surfaces
- `aconex` — Oracle Aconex REST API
- `acc-docs` — Autodesk Construction Cloud Docs API (mirror of `acc-issues`, but files surface)

**Realistic effort:** 3–4 days.

---

## v0.16 — Fabricator downstream

**Goal:** Tekla EPM (PowerFab) + Peddinghaus Direct Tools / Peddimat translator + curated NC-export-phase verb on Tekla.

**Why:** detailer dealbreaker. Without EPM, AWARE is missing 40% of the fabricator's day.

**Agents to add:**
- `tekla-powerfab` — REST API per Trimble's official developer portal at developer.tekla.com/tekla-powerfab (free signup; product historically called "Tekla EPM")
- `peddinghaus-translator` — file-format agent, DSTV `.nc1` → Peddimat `.asc` conversion + manifest CSV

**Curation added to existing `tekla` agent:**
- `drawing-issue-pack` — top detailer requested verb
- `nc-export-phase` — DSTV / Peddimat / DXF
- `bolt-list` / `weld-list` / `part-list` / `assembly-list`
- `drawings-by-status`, `drawings-affected-by-revision`
- `uda-set` / `uda-get` / `uda-copy`

**Realistic effort:** 4–5 days.

---

## v0.17 — Visualization (honest answer)

**Goal:** address the designer dealbreaker without inventing fictional APIs. Enscape + Twinmotion + V-Ray have no public scriptable APIs. The honest answer: AWARE prepares the model + routes frames; user renders outside.

**Spec changes:**
- Manifesto + docs explicitly state which viz tools AWARE drives directly vs prepares-for (the latter is fine; just say so)

**Content:**
- Curate Rhino-8 + Grasshopper-8 (cut from 6,954 / 5,181 raw commands to 50 each):
  - `view.capture-named` — named view → PNG with consistent settings
  - `layer.set-state` — load a saved layer state for consistent rendering
  - `make-2d` — flatten to 2D for diagrams
  - `gh.solve` — execute a Grasshopper definition with input parameters
  - `gh.sweep` — parametric sweep over a GH input range
- New `render-queue` primitive in app-spec — long-running heavy compute with progress, retry, resource hints

**Agents to add:**
- `enscape-prep` — drives Rhino/Revit to prep a scene for Enscape; user opens Enscape and renders
- `twinmotion-prep` — same shape for Twinmotion

**Realistic effort:** 3–4 days.

---

## v0.18 — Spec authoring

**Goal:** NBS Chorus + Avitru SpecLink + CSI MasterFormat primitives.

**Why:** architect gap. Half the deliverable for US firms; UK firms run NBS.

**Agents to add:**
- `nbs-chorus` — REST API
- `avitru-speclink` — REST API
- `csi-masterformat` — file-format agent for spec section identification + cross-referencing

**Realistic effort:** 2–3 days.

---

## v0.19 — Substrate primitives

**Goal:** add the workflow primitives every persona asked for. These extend the app-spec, not the agents.

**Spec changes:**
- `for-each: <list-expr>` — iterate over a static list (vs stream-watch)
- `compare: { a, b, by }` — diff two snapshots, emit changes
- `sweep: { var, range, capture }` — parametric sweep
- `approve: { gate, channel, timeout }` — human-in-the-loop pause
- `schedule: { cron, timezone }` — time-triggered runs (not just event-watch)
- `assert: { expr, on-fail }` — pre-flight gate, abort on false
- `snapshot: { path }` — freeze model state to immutable artifact
- `model-lock: { acquire-timeout, write-budget }` — single-writer with rate limit

**Engine changes:**
- All eight primitives implemented in `cli::runtime::orchestrator`
- Each primitive has a worked test against the reference apps
- `aware app run --dry-run` exercises gates without committing

**Realistic effort:** 5–6 days. The substrate's biggest extension since v0.3.

---

## v0.20 — Named reusable atoms

**Goal:** replace inline JavaScript glue with named, reusable, auditable atoms. "No-code" becomes honest.

**Spec changes:**
- `atoms/` subfolder per agent (and at substrate level for cross-cutting)
- Each atom: `id`, `kind: predicate|map|reduce`, `inputs`, `outputs`, `code` (typed; not free-text)
- App `.flo` references atoms by ID: `filter: atom://tekla/is-ready-for-issue`

**Content:**
- 20 cross-cutting atoms: `is-newer-than`, `group-by`, `sort-by`, `unique`, `pluck`, `count`, `sum`, `avg`, `min`, `max`, `at-least`, `at-most`, `format-date`, `path-join`, `naming-template`, `kebab-to-pascal`, `pascal-to-kebab`, `csv-row-build`, `json-path`, `regex-match`
- 10 Tekla atoms: `is-ready-for-issue`, `is-welded`, `is-bolted`, `mark-format`, `phase-name`, etc.
- 10 Revit atoms: `is-issued-for-construction`, `sheet-number-format`, `revision-letter-next`, etc.

**Realistic effort:** 2–3 days.

---

## v0.21 — Engineering envelope

**Goal:** code-revision pinning, solver-build hashing, model-schema-version pin, digital-seal hooks, typed units. Unlock PE-stamped deliverables.

**Why:** structural-engineer dealbreaker.

**Spec changes:**
- Agent manifest gains `engineering:` block declaring code revision, NA, solver build hash, section catalogue revision
- Lockfile pins all of the above alongside agent versions
- New `units:` type on templating — numbers carry dimension (`4.0 kN/m²`, not `4.0`)
- `signed-output:` directive on app — hash + sign + append to JSONL

**CLI changes:**
- `aware app reproduce <app> --run-id <id>` — re-run a frozen run against the pinned engineering envelope
- `aware app sign-output <run-id> --seal <pe-credential>` — chain-of-custody for engineer's stamp

**Realistic effort:** 4–5 days.

---

## v0.22 — Per-persona reference apps + first-hour guides

**Goal:** every persona has a worked end-to-end app that runs against their installed tools, plus a 60-second demo in the manifesto.

**Content:**
- `30-apps/_examples/bim-monday-audit.flo` — Revit + ACC + M365 + html-report
- `30-apps/_examples/designer-monday-shots.flo` — Rhino + M365 + html-report
- `30-apps/_examples/architect-sheet-status.flo` — Revit + ACC + M365
- `30-apps/_examples/engineer-peer-review-delta.flo` — TSD + snapshot + html-report + M365
- `30-apps/_examples/detailer-issue-pack.flo` — Tekla + Trimble Connect + M365

**Docs:**
- `cli/README.md` gains a 60-second demo per persona
- `00-vision/manifesto.md` adds 4 more demos beside the fabricator one

**Realistic effort:** 3–4 days.

---

## v0.23 — Decalog amendment

**Goal:** add the 9th structural truth (the existing decalog has 8, not 10 — "decalog" is a naming convention, not a count).

**Spec changes:**
- `00-vision/decalog.md` — add truth #9: **"AI composes the plan; deterministic code is the plan."**
- Restrict `think-node` / `smart-node` from validation, approval, code-check, or safety-critical paths in apps
- App-spec `assert:` section calls out the constraint structurally

**Realistic effort:** 1 day.

---

## v0.24 — Minimum Receipt (`<app>.lock` + Glass Box viewer)

**Goal:** ship the artifact engineers actually read instead of the AI's prose. Compile every app to a deterministic, type-resolved, agent-version-pinned `<app>.lock` sidecar; render it visually via `aware app inspect`.

**Why next:** the persona audit + the 4-agent verification brainstorm both converged on the same insight — engineers will never trust prose composed by an AI; they will trust a deterministic, signed, multi-voice-reviewed artifact the AI's prose compiled into. The lockfile is the foundation; v0.25 layers panel review on top.

**Substrate-correct framing (mandatory):** the lockfile sidecar is named `<app-name>.lock`, never `.flo.lock`. The substrate is extension-agnostic — `.flo` is FloLess's extension; `.app`, `.flow`, `.aware`, and custom extensions all compile to the same lockfile shape.

**Spec changes (`10-core/app-spec.md`):**
- New "Lockfile sidecar" section defining the `.lock` format
- Resolved fields: every template `{{ }}` → concrete value; every agent reference → pinned version; every node's input/output schema → resolved type; every write-mode node tagged
- The lockfile is **the contract** — what the runtime executes, what the engineer reads, what the diff tool diffs

**CLI changes:**
- `aware app compile <app>` — explicit compile step; emits `<app>.lock` next to the source file
- `aware app inspect <app>` — opens Glass Box (single-file HTML viewer) in the user's default browser
- `aware app validate <app>` — runs compile + emits `<app>.lock` as a side effect (replaces the silent-pass behavior)
- `aware app run <app>` — refuses to execute unless a fresh `.lock` matches the source app's content hash (no drift between read + execution)

**Glass Box viewer (single HTML file, no external runtime):**
- Topology rendered as a flowchart (vanilla JS + SVG; mirrors the `aware diagram` + force-graph playground)
- Each edge labeled with the actual data shape that flows through it (`Stream<SheetId>`, `Object{rows: array}`, etc.)
- Write-mode nodes glow red; read-mode nodes gray
- Thumbnail panel showing which model elements get touched (per-agent integration)
- Side panel: "47 elements touched, 1 workset (yours), 0 elements owned by other users, est. 4min"

**Realistic effort:** 2–3 weeks.

---

## v0.25 — Reviewed Receipt (panel + voice packs)

**Goal:** every write-mode node gets a `panel:` block — N voices, different models in different domain hats, all must agree before any commit happens. Voice packs become a new distribution primitive (forkable, version-pinnable, citable embodiments of institutional knowledge).

**Why this is the killer demo:** the 60-second video that reshapes the industry is a structural engineer running an app + watching one model (GPT-5 wearing a UK code-compliance hat) dissent against another (Claude wearing a structural-engineer hat) with a specific BS 9999 §6.2 citation. Today no AI-for-AEC tool does that. With this primitive, "trust the AI" becomes "N different models, in N different domain hats, each owning one slice of the risk."

**Spec changes:**
- `panel:` block on any write-mode node in the app file
- Required: `voices:` (list), `require:` (unanimous / majority / quorum-N), `on-dissent:` (halt / log-only / warn)
- Each voice declares: `model`, `system-prompt` (or `voice-pack:` reference), `jurisdiction` (optional)

**New primitive: voice packs.**
- `aware voice install @ise/uk-structural-reviewer@2025` pulls a markdown system-prompt + reference-codes folder published by an institution / authoring engineer
- Decalog #6 says "ship agents, not apps." This generalises: **ship voices.**
- Voice packs live at `~/.aware/voices/<scope>/<id>/`
- Forkable (git semantics), version-pinnable (`@2025`), citable (each pack has a DOI-like identifier)

**Receipt artifact:**
- Every panel-gated commit produces a JSONL receipt:
  - lockfile hash
  - panel transcripts (including any dissents)
  - the actual writes that committed
  - timestamp + operator signature
- Receipts land at `~/.aware/receipts/<app>/<run-id>.jsonl`
- Re-emitted next to the artifact if `engineering.output-seal` (v0.21) is set

**CLI changes:**
- `aware voice install <pack>` — same shape as agent install
- `aware voice list` / `describe <pack>` / `uninstall <pack>`
- `aware app run` — when a panel-gated node is hit, the CLI prints each voice's verdict live, halts on dissent (default)

**Composes with v0.11 (safety contract) + v0.19 (substrate primitives) + v0.21 (engineering envelope) + v0.23 (decalog #9):** the panel doesn't replace those; it's the multi-voice review layer that runs *before* the deterministic write executes.

**Realistic effort:** 3–4 weeks. Heavy on protocol design for voice-pack distribution + panel orchestration.

---

## v0.26 — Stamped Receipt (reference cases + signed JSONL) — *queued, ship when 2–3 firms pilot*

Adds reference-cases-as-contract (engineers' existing `\\fileserver\Tender\2024\Q3\Final` folders become CI gates) + ed25519-signed JSONL receipt. PE-stamp-grade. Insurance-grade. Building-control-grade.

Defer until adoption signals demand it. Don't ship the "stamped" tier until 2–3 engineering firms say "we'd pilot this if our last 5 tender packages still pass."

---

## Vendor `*.exec` fan-out — bringing each AECO host to live-write parity

The `*.exec` verb is the structural primitive that closes the catalog → live-host loop in one call. The orchestrator reads the agent's catalog (knowledge), drafts a script, ships it through `*.exec`, sees the host's response. Same receipt vocabulary across vendors so orchestrator UX is uniform.

### v0.31 — `tekla.exec` (Tekla Structures 2026, .NET 4.8 + Roslyn) — *shipped*

Roslyn scripting over Tekla's classic-framework Open API. Sidecar at `cli-tekla/` (net48). 13/20 PASS on live-Tekla drill. Shape: `{verb, code, args}` stdin-JSON, AWARE-shaped receipt on stdout.

### v0.32 — `rhino.exec` (Rhino 8, .NET 10 + rhinocode CLI) — *shipped*

Wraps McNeel's `rhinocode` CLI which speaks named-pipe to Rhino's `ScriptServer`. Sidecar at `cli-rhino/` (net10). C# 9 top-level statements wrapped around the user body. Identical receipt vocabulary to Tekla; only `host` differs.

### v0.34 — `sketchup.exec` (SketchUp 2026, .NET 10 + in-process Ruby bridge)

**Goal:** bring SketchUp to live-write parity with Tekla and Rhino.

**Why this is non-trivial:** Trimble ships no out-of-process CLI bridge for SketchUp. The only public extensibility surface is the in-process Ruby 3.2 interpreter. Therefore the sidecar can't shell out — it must invent the bridge.

**Architecture:**
- `cli-sketchup/` (.NET 10 single-file binary, mirrors cli-rhino's shape)
- `cli-sketchup/BridgeAssets/aware_sketchup_bridge.rb` + `aware_sketchup_bridge/core.rb` — an auto-loaded SketchUp extension that opens a TCP listener on 127.0.0.1:8765 (first free in 8765..8865), drains an inbox queue on SketchUp's main thread via `UI.start_timer(0.05, true)`, evaluates user Ruby against `Sketchup.active_model`, returns a JSON receipt over the socket.
- Wire protocol: 4-byte big-endian length prefix + UTF-8 JSON body. Unambiguous, fast, no escaping concerns.
- Discovery: each SketchUp PID writes `%TEMP%/aware-sketchup/<pid>.json` so the sidecar can find which port belongs to which instance.
- Bridge install: `aware-sketchup --install-bridge` copies the loader + support into `%APPDATA%\SketchUp\SketchUp <year>\SketchUp\Plugins\`. One-time setup; SketchUp auto-loads on next launch.

**Verbs (identical contract to cli-tekla / cli-rhino):**
- `exec` — evaluate Ruby against the active model
- `list-instances` — enumerate live bridges via the discovery folder
- `send-status` — convenience verb that ships `Sketchup.set_status_text(msg)` through `exec`
- `launch` — spawn SketchUp.exe (optionally opens a `.skp` file)
- `close` — clean shutdown (save + CloseMainWindow) or `force: true` (Process.Kill)

**Live-drill target:** 13/20 PASS against SketchUp 2026, matching Tekla's v0.31 bar.

**Realistic effort:** 1–2 days. Shipped at this writing; live drill pending one-time human dismissal of the SketchUp 2026 Welcome dialog (CEF-rendered, blocks plugin loading on first launch only).

### v0.35+ — Future fan-out

The same shape applies to every vendor whose API surface is scriptable in-process: Allplan (CSharp scripts), Archicad (JavaScript via PythonPalette), Revit (Python/C# via pyRevit or RvtPy), Navisworks (.NET via NwApi). Each lands as a new `cli-<vendor>/` directory with the same {verb, code, args} contract and AWARE-shaped receipts.

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

## Parked / deferred ideas

Logged here so they don't keep coming up, but not strictly rejected the way the anti-roadmap is. Each could revive when a concrete consumer asks for it.

### Official Docker base image (`ghcr.io/aware-aeco/aware:0.x`)

**Priority:** low — no concrete consumer today.

**What:** Publish an official Docker image on each release. Downstream packagers (FloLess, third-party SaaS, internal IT) base their containers on it; demo / CI / "try-without-installing" use cases also benefit.

**Cost:** ~1 day of CI work — Dockerfile + GitHub Actions release job. Near-zero ongoing maintenance.

**Why parked:** Modest value with no concrete consumer today. Persona audit didn't flag it. v0.10–v0.23 catalogue gaps are higher leverage. Add it when FloLess (or another downstream product) starts shipping a containerized AWARE-based product and would actually consume the image.

**Discussed:** 2026-05-17, during a "should AWARE support team deployment?" brainstorm. Conclusion: team deployment is overwhelmingly a downstream concern (FloLess et al), not an AWARE substrate concern. AWARE stays local-first per decalog #4; the Docker image is the only piece that might legitimately belong upstream. Host CAD software (Tekla / Revit / Rhino) can never run on the team box — it stays on the engineer's Windows machine — which further narrows what a team server would even be useful for.

---

## Notes for the implementer

Read [`cli-spec.md`](./cli-spec.md) before writing any command. Read the [`decalog`](../00-vision/decalog.md) when you're tempted to add a feature that "would be nice." Read the parent [CLAUDE.md](../CLAUDE.md) for the engineering rules that apply across every session.

The shape of `~/.aware/` is documented in `cli-spec.md` — never invent new directories without updating both the spec AND this roadmap.

When a phase feels long, that IS the work. Don't ship a half-implemented phase and call it v0.3.
