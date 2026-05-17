# AWARE

> **AWARE is what comes after software-as-product. Apps are text — files you can read in Notepad. AI is the runtime that executes them. Open source is what the format does automatically: there's nothing else to a "proprietary" app once it's plain English. No vendor, no installer, no walled garden. AECO is the wedge; the substrate is universal.**

That is the whole thesis.

The eight structural truths it rests on are in **[the decalog](./decalog.md)**. If anything in this document conflicts with the decalog, the decalog wins.

---

## The 60 seconds that prove it

```bash
$ npm install -g @aware-aeco/cli
  ✓ aware CLI installed
  ✓ 11 AECO agents available via `aware agent install`
  ✓ aware-aeco plugin registered with: claude-code, codex

$ claude-code
  ✓ plugin: aware-aeco · 11 agents available

> Watch this Tekla model. When a welded assembly appears,
> upload its drawing to my Trimble Connect fab folder.

  → Picked tekla agent for the trigger.
  → Added inline welded filter.
  → Used trimble-connect's upload command.
  → Wrote ~/.aware/apps/welded-to-tc.flo

  Composed:  [Tekla Watcher] → [Welded Filter] → [TC Uploader]

$ cat ~/.aware/apps/welded-to-tc.flo
  # plain text · readable · editable · forkable · open source by being a file
```

A real Tekla → Trimble Connect integration. One sentence. Plain text. Sixty seconds.

For decades that integration was either *"buy a $40,000 plugin"* or *"hire two devs for six months."* AWARE removes the question.

### Five 60-second demos — one per persona

The fabricator pitch above is the canonical 60 seconds. The 2026-05-17 persona audit asked for one per discipline. Each composes to a `.flo` file in `30-apps/_examples/`:

| Persona | Sentence to Claude / codex | Reference app |
|---|---|---|
| **BIM manager** | *"Every Monday 7am, walk our 4 active Revit projects and post a model-health rollup to Teams. File size, warnings, unplaced rooms, sheets changed this week. Email the PM the same as a PDF."* | [`bim-monday-audit.flo`](../30-apps/_examples/bim-monday-audit.flo) |
| **Designer** | *"Every Monday, open the active Rhino model, capture the SE / NW / aerial named views at 4K, post them to the design-review Teams channel."* | [`designer-monday-shots.flo`](../30-apps/_examples/designer-monday-shots.flo) |
| **Architect** | *"Every Monday 7am, walk all my active Revit projects + their ACC Issues + their Bluebeam Studio Sessions, post a single ball-in-court card to the principals-rollup channel."* | [`architect-sheet-status.flo`](../30-apps/_examples/architect-sheet-status.flo) |
| **Structural engineer** | *"At each peer-review checkpoint, compare the current TSD model against the prior snapshot and produce a delta report with full reproducibility receipt. Pin EC3:2022 + UK NA + EN 10365."* | [`engineer-peer-review-delta.flo`](../30-apps/_examples/engineer-peer-review-delta.flo) |
| **Detailer** | *"Given a list of drawing marks + a revision letter, issue the pack: stamp + export PDF + DWG + NC files (Peddinghaus routed) + bolt list + upload to Trimble Connect. Notify the shop floor Teams channel."* | [`detailer-issue-pack.flo`](../30-apps/_examples/detailer-issue-pack.flo) |

Each runs in production via `aware app run <name>`. First-hour walk-throughs live in [`90-onboarding/`](../90-onboarding/) — one per persona.

---

## Why this changes everything (not just AECO)

Software has always been a thing you buy from someone else. A binary. A bundle. A license. A vendor sitting between you and the tool you actually wanted to use.

When apps become text, that whole arrangement collapses:

- The format is human-readable, so there's no need to reverse-engineer it.
- The runtime is AI, so it's not tied to a specific vendor's binary.
- Open source is not a choice; it's the **physics of the format.**
- Distribution is sending the file.

The vendor doesn't disappear because someone fought them — they disappear because the function they performed (translating intent into running code, then charging rent on the translation) is something AI does for free now.

AECO is where we ship first because:

- The software-vendor lock-in is heaviest here ($50k+/seat for the leading tools)
- The tacit knowledge is deepest here (decades of structural-engineering intuition that dies with each retirement)
- The integration pain is most acute (a single fab pipeline often touches 7+ vendors)

The trajectory is well-documented. Big Data Construction's [history of BIM](https://bigdataconstruction.com/history-of-bim/) traces it: CAD → early BIM → open-vs-closed competition → Autodesk consolidation → an open "what now?" today. Their diagnosis matches the decalog: **construction's core problem is data monopoly and opacity, not missing software.** AWARE is the "what now?" — the layer above BIM where the data is text and the runtime is AI.

If AWARE works in AECO, it works anywhere.

---

## The two primitives that make it real

### 1. `aware agent build` — *any software becomes an agent*

Point the agent builder at a DLL folder, a NuGet package, an OpenAPI spec, a COM type library, a Python module, or a CLI's `--help` output. Out comes a working AWARE agent: manifest + skills + commands, license-aware, version-pinnable.

One engineer can wire up the entire AECO toolchain — AutoCAD, Revit, Bentley, Procore, Trimble Connect, Slack, Microsoft 365 — in a week. **Without writing custom integration code for any of them.**

### 2. `exposes-as-agent: true` — *any app becomes a building block*

When you compose agents into an app, you can mark it `exposes-as-agent: true`. It now appears in the registry as if it were an agent. Other people install it with `aware app install @yourname/your-app` and use it as one node in their bigger pipelines, never seeing the internals.

The detailer who spent 20 years figuring out the right Tekla → IDEA StatiCa → connection-validation flow ships `@detailer/connection-validator`. 50 fab shops install it as one node. **Decades of AECO knowledge stops dying with the people who hold it.**

This is **npm for AECO expertise.** And — by the decalog — it works for any vertical, not just AECO.

---

## What AWARE is (technically)

```
aware CLI                            ─ manages everything
  │
  ├── agents          at ~/.aware/agents/        (installed via `aware agent install`)
  ├── apps            at ~/.aware/apps/          (installed via `aware app install`)
  ├── credentials     at ~/.aware/credentials/   (encrypted; `aware connect/disconnect`)
  ├── logs            at ~/.aware/logs/          (execution provenance)
  │
  └── host plugins    at ~/.<host>/plugins/      (auto-generated per CLI)
                          claude-code
                          codex
                          opencode
```

- Every agent is plain markdown skills + commands + an optional runtime adapter
- Every app is plain text
- Every credential is encrypted on disk, never in source
- Every execution leaves a provenance trail you can audit later

## What AWARE is not

- **Not a new chat.** Use claude-code, codex, opencode — whatever you already use.
- **Not a new protocol.** MCP is a transport; AWARE rides on it where it fits.
- **Not a SaaS.** No cloud account required. Local-first. Your project, your files, your machine.
- **Not a low-code platform.** The composition format is plain text. Power users edit it directly.
- **Not Zapier for AECO.** Zapier connects SaaS endpoints; AWARE composes desktop software, REST APIs, and AI judgment into one fabric.

---

## v0 scope

- Apache 2.0 core CLI
- ~12 first-party agents: `tekla`, `revit`, `autocad`, `trimble-connect`, `procore`, `slack`, `file`, `excel`, `email`, `sharepoint`, `think-node`, `aware-agent-builder`
- Reference apps demonstrating composition patterns (linear, fan-in, fan-out)
- Host plugin generators for claude-code, codex, opencode
- GitHub-hosted registry (single source of truth, PR-based contributions)
- Encrypted credential storage

### Out of v0

- Vector / embedding-based RAG (raw grep + skill files are enough for now)
- App-of-apps direct nesting (use app-as-agent instead)
- Hosted registry service (GitHub is enough for year one)
- FloLess visual canvas (separate project; this manifesto is about the substrate)

---

## Contribute

Three ways:

1. **Write an agent.** Use `aware agent build` to bootstrap from any DLL / NuGet / COM / OpenAPI / CLI source, refine the generated skills, submit a PR.
2. **Publish an app.** Compose existing agents into something useful, mark it `exposes-as-agent: true`, push to the registry.
3. **Improve a skill.** Skills are plain markdown. Find a gotcha that bit you, add a paragraph to the relevant skill file. **If you can write a paragraph, you can contribute to AWARE.**

No build system to learn. No CI pipeline to fight. No boilerplate generators. Markdown PRs.

---

## Start

```bash
npm install -g @aware-aeco/cli                           # any OS (Windows / Mac / Linux)
curl -fsSL .../aware-aeco/aware/main/scripts/install.sh | bash   # Mac / Linux, no Node required
iex (irm .../aware-aeco/aware/main/scripts/install.ps1)          # Windows PowerShell, no Node required

claude-code
> compose an aware app that does <whatever you wish existed>
```

`winget install aware-aeco` and `brew install aware-aeco` are queued — they require an MSI installer (Windows) and a Homebrew formula (Mac), both tracked as follow-up phases. See [cli/README.md](../cli/README.md) for the current install matrix.

Welcome to what comes after software.
