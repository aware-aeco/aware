# AWARE

> **AWARE is what comes after software-as-product. Apps are text — files you can read in Notepad. AI is the runtime that executes them. Open source is what the format does automatically: there's nothing else to a "proprietary" app once it's plain English. No vendor, no installer, no walled garden. AECO is the wedge; the substrate is universal.**

— [the statement](./00-vision/manifesto.md). The nine structural truths it rests on are in [the decalog](./00-vision/decalog.md).

---

## 60-second demo

AWARE is the open framework that turns any software into an **agent** — skills + commands your existing AI can compose into plain-text apps. There's no new app to learn: your terminal AI is the runtime.

```bash
$ npm install -g @aware-aeco/cli                       # any OS — recommended
  # alternatives that work today:
  #   curl ... install.sh | bash             # Linux / Mac, no Node
  #   iex (irm ... install.ps1)              # Windows PowerShell, no Node

$ aware agent install tekla              # any software → an agent (skills + commands)
$ aware agent install trimble-connect
  ✓ tekla            · Tekla Open API, version-pinned
  ✓ trimble-connect  · REST

$ claude-code                    # or codex, or opencode — your AI is the runtime
  ✓ plugin: aware-aeco · 66 agents · 3,282 skills

> From this Tekla model, export each welded assembly's drawing
> to my Trimble Connect fab folder.

  → AWARE hands your AI the tekla + trimble-connect skills; it writes the
    integration and saves it as a plain-text app — readable, forkable,
    open source by construction:
  → ~/.aware/apps/welded-to-tc/welded-to-tc.aware

$ aware app run welded-to-tc
```

That's the whole thing: any software becomes an agent, your AI composes those agents, and the result is one plain-text file you run.

> `winget install aware-aeco` and `brew install aware-aeco` are queued. They need an MSI installer (Windows — scaffolded in `packaging/wix/`) and a Homebrew formula (Mac) plus a code-signing cert, all tracked as follow-up phases. The npm + curl-pipe + PowerShell paths above are what works today.

---

## What's in the repo

```
aware-aeco/
├── 00-vision/              # decalog · manifesto · positioning
├── 10-core/                # agent-spec · app-spec · cli-spec · cli-roadmap · runtime contracts
├── 20-agents/              # 66 agents · 17 curated + 49 reflected · all Apache 2.0
│   ├── _core/              #   5 meta-primitives: aware-agent-builder · aware-skill-builder ·
│   │                       #   html-report · http · ui-inspector
│   └── aeco/
│       ├── engineering/    #   16 — tekla (curated) · tekla 25/26 · CSi · IDEA StatiCa 25/26 ·
│       │                   #        TSD 25/26 · Tedds 25/26 · PowerFab · plugin-sdk 25/26 · …
│       ├── architecture/   #   22 — revit 25/26 · autocad 25/26 · rhino 7/8 · grasshopper 7/8 ·
│       │                   #        archicad 28/29 · allplan 24/25 · sketchup 25/26 · navisworks · dynamo · …
│       ├── construction/   #   13 — trimble-connect · procore · ACC issues/docs/admin · APS ·
│       │                   #        aconex · slack · solibri · bluebeam · BCF + IFC inspectors · …
│       ├── visualization/  #    9 — xeokit · three.js · thatopen-components · web-ifc · speckle · iTwin 5.8/5.9 · …
│       └── cross-cutting/  #    3 — microsoft-365 · google-workspace · dropbox
├── 30-apps/_examples/      # 7 reference apps — one per persona + canonical demos
├── 40-diagrams/            # Mermaid + Excalidraw views of the substrate
├── 50-research/            # design notes, prior art, competitive analysis
├── 90-onboarding/          # first-hour walk-throughs, one per persona
├── cli/                    # the `aware` CLI — Rust runtime (shipped)
├── cli-tekla / -revit /    # desktop-host sidecars — stateful, in-process vendor APIs
│   -rhino / -sketchup
├── cli-roslyn / -reader /  # C# source reader (Roslyn) + shared IR reader + sidecar lib
│   -sidecar
├── cli-npm/                # @aware-aeco/cli — the npm wrapper (published)
├── packaging/wix/          # Windows MSI installer (winget path — in progress)
├── scripts/                # install.sh · install.ps1 · agent generators
└── registry-index.json     # the registry — source of truth for installable agents
```

---

## Read these in order

1. [`00-vision/decalog.md`](./00-vision/decalog.md) — the nine structural truths (5 min read)
2. [`00-vision/manifesto.md`](./00-vision/manifesto.md) — what AWARE is, why now, how it ships (10 min)
3. [`10-core/agent-spec.md`](./10-core/agent-spec.md) — how to write an agent
4. [`10-core/app-spec.md`](./10-core/app-spec.md) — how to write an app
5. [`10-core/cli-spec.md`](./10-core/cli-spec.md) — what the CLI does
6. [`30-apps/_examples/`](./30-apps/_examples/) — seven worked apps showing the format end-to-end
7. [`20-agents/_core/aware-skill-builder/`](./20-agents/_core/aware-skill-builder/) — how to write or port a skill
8. [`CONTRIBUTING.md`](./CONTRIBUTING.md) — three ways to contribute, all markdown PRs

---

## Status

**Substrate: content-complete. Runtime: shipped.**

The `aware` CLI is live at **v<!--stat:cli_version-->0.54.0<!--/stat-->** (Rust), published to npm as **`@aware-aeco/cli`**, with curl + PowerShell installers in [`scripts/`](./scripts/). What began as 7 reference agents is now a working substrate:

- **<!--stat:agents_total-->66<!--/stat--> agents** — <!--stat:agents_curated-->17<!--/stat--> hand-written + <!--stat:agents_reflected-->49<!--/stat--> auto-generated from vendor SDKs — **all registered** in [`registry-index.json`](./registry-index.json) and installable today.
- **`aware build agent`** generators: `--from-nuget`, `--from-npm`, `--from-yard`, `--from-openapi`, `--from-csharp` (Roslyn source reader).
- **Desktop-host sidecars** for stateful, in-process vendor APIs: `cli-tekla`, `cli-revit`, `cli-rhino`, `cli-sketchup`.

**In flight:**
- First-class `winget` / `brew` installers (MSI scaffolding in `packaging/wix/`, Homebrew formula, code-signing cert).
- Registering the agents still maturing on disk.

Track the CLI surface in [`10-core/cli-spec.md`](./10-core/cli-spec.md) and the phased plan in [`10-core/cli-roadmap.md`](./10-core/cli-roadmap.md).

---

## License

[Apache 2.0](./LICENSE) — permissive, patent grant, no walled garden. Consistent with the decalog: AWARE's substrate is open by construction.

Commercial apps built on top of AWARE choose their own license. The substrate does not impose one.

---

## Watch this repo

The substrate is in and the runtime shipped. New agents land continuously — [`registry-index.json`](./registry-index.json) is the live list. Star the repo, install the CLI, and compose something:

```bash
npm install -g @aware-aeco/cli
```
