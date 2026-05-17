# AWARE

> **AWARE is what comes after software-as-product. Apps are text — files you can read in Notepad. AI is the runtime that executes them. Open source is what the format does automatically: there's nothing else to a "proprietary" app once it's plain English. No vendor, no installer, no walled garden. AECO is the wedge; the substrate is universal.**

— [the statement](./00-vision/manifesto.md). The five structural truths it rests on are in [the decalog](./00-vision/decalog.md).

---

## 60-second install demo

```bash
$ npm install -g @aware-aeco/cli                       # any OS — recommended
  # alternatives that work today:
  #   curl ... install.sh | bash             # Linux / Mac, no Node
  #   iex (irm ... install.ps1)              # Windows PowerShell, no Node

$ claude-code                    # or codex, or opencode
  ✓ plugin: aware-aeco · 11 agents · 58 curated + 133 raw skills

> Watch this Tekla model. When a welded assembly appears,
> upload its drawing to my Trimble Connect fab folder.

  → composed [Tekla Watcher] → [Welded Filter] → [TC Uploader]
  → wrote ~/.aware/apps/welded-to-tc.flo

$ aware app run welded-to-tc
```

That's the whole thing. One sentence in your terminal, one plain-text file, one command to run.

> `winget install aware-aeco` and `brew install aware-aeco` are queued. They need an MSI installer (Windows) and a Homebrew formula (Mac) plus a code-signing cert, all tracked as follow-up phases. The npm + curl-pipe + PowerShell paths above are what works today.

---

## What's in the repo (v0 snapshot)

```
aware-aeco/
├── 00-vision/              # decalog · manifesto · positioning
├── 10-core/                # agent-spec · app-spec · runtime contract
├── 20-agents/              # 7 agents · 58 skills · all Apache 2.0
│   ├── _core/              #   meta-primitives
│   │   ├── aware-agent-builder/      # any software → an agent
│   │   ├── aware-skill-builder/      # author / port / modify / eval skills
│   │   └── html-report/              # utility: render self-contained HTML output
│   └── aeco/
│       ├── engineering/
│       │   ├── tekla/                # 31 curated skills · stateful · .NET-mandated
│       │   ├── tekla-2025/           # 2,999 cmds · NuGet ref/ reflection (Tekla.Structures.Model 2025.0.0)
│       │   ├── tekla-2026/           # 3,179 cmds · NuGet ref/ reflection (Tekla.Structures.Model 2026.0.3)
│       │   ├── csi-api/              # 4,282 cmds · NuGet reflection (CSiAPIv1-All 2.12.0 — SAP2000/ETABS/SAFE/CSiBridge)
│       │   ├── idea-statica-25/      # 111 cmds · NuGet (IdeaStatiCa.OpenModel 25.1.5)
│       │   ├── idea-statica-26/      # 116 cmds · NuGet (IdeaStatiCa.OpenModel 26.0.1)
│       │   ├── tsd-25/               # 440 cmds · NuGet (TeklaStructuralDesigner.RemotingAPI 25.3.0)
│       │   └── tsd-26/               # 440 cmds · NuGet (TeklaStructuralDesigner.RemotingAPI 26.0.1)
│       ├── architecture/
│       │   ├── allplan-2024/         # 1,162 cmds · NuGet (Allplan.BIF.Core 2024.0.1.35)
│       │   ├── allplan-2025/         # 1,200 cmds · NuGet (Allplan.BIF.Core 2025.0.0.15)
│       │   ├── autocad-2025/         # 4,398 cmds · NuGet (AutoCAD.NET 25.1.0)
│       │   ├── autocad-2026/         # 4,413 cmds · NuGet (AutoCAD.NET 26.0.0)
│       │   ├── revit-2025/           # 7,430 cmds · NuGet ref/ reflection (Autodesk.Revit.SDK 2025.0.2.419)
│       │   ├── revit-2026/           # 7,647 cmds · NuGet ref/ reflection (Autodesk.Revit.SDK 2026.0.0.9999)
│       │   ├── rhino-7/              # 5,859 cmds · NuGet ref/ reflection (RhinoCommon 7.38.x)
│       │   ├── rhino-8/              # 6,954 cmds · NuGet ref/ reflection (RhinoCommon 8.31.x)
│       │   ├── grasshopper-7/        # 4,506 cmds · NuGet ref/ reflection (Grasshopper 7.38.x)
│       │   ├── grasshopper-8/        # 5,181 cmds · NuGet ref/ reflection (Grasshopper 8.31.x)
│       │   ├── sketchup-2025/        # 1,684 cmds · YARD docs (SketchUp/ruby-api-docs SU2025.0.3)
│       │   └── sketchup-2026/        # 1,713 cmds · YARD docs (SketchUp/ruby-api-docs SU2026.0)
│       ├── visualization/
│       │   ├── xeokit/               # 361 cmds · TypeScript .d.ts (@xeokit/xeokit-sdk@2.6.109)
│       │   ├── three/                # 2,860 cmds · TypeScript .d.ts (@types/three@0.184.1)
│       │   ├── thatopen-components/  # 337 cmds · TypeScript .d.ts (@thatopen/components@3.4.6 — modern IFC.js)
│       │   ├── web-ifc/              # 120 cmds · TypeScript .d.ts (web-ifc@0.0.77 — WebAssembly IFC parser)
│       │   ├── speckle-viewer/       # 942 cmds · TypeScript .d.ts (@speckle/viewer@2.28.0 — open AECO data viewer)
│       │   ├── itwin-5-8/            # 4,553 cmds · TypeScript .d.ts (@itwin/core-frontend@5.8.5 — Bentley iTwin)
│       │   └── itwin-5-9/            # 4,561 cmds · TypeScript .d.ts (@itwin/core-frontend@5.9.3 — Bentley iTwin)
│       ├── construction/
│       │   ├── trimble-connect/      # 7 skills · stateless · REST
│       │   └── slack/                # 172 cmds · OpenAPI reflection (Slack Web API)
│       └── cross-cutting/
│           ├── microsoft-365/        # 4 skills · Graph REST
│           └── google-workspace/     # 4 skills · Drive/Sheets/Calendar/Gmail
├── 30-apps/                # reference apps
│   └── _examples/
│       ├── welded-to-tc.flo          # 3-node linear · canonical demo
│       └── qa-drawings-to-tekla.flo  # 7-node DAG · fan-in + fan-out
├── 40-diagrams/            # Mermaid + Excalidraw views of the substrate
└── 50-research/            # design notes, prior art, competitive analysis
```

### Stats

| | Count |
|---|---|
| Curated agents | **7** (58 hand-written skills) |
| Reflected agents | **27** (Tekla 2025/2026 + Revit 2025/2026 + Rhino 7/8 + Grasshopper 7/8 + SketchUp 2025/2026 + AutoCAD 2025/2026 + xeokit + three.js + Slack + CSi API + That Open Components + web-ifc + Speckle Viewer + Allplan 2024/2025 + IDEA StatiCa 25/26 + TSD 25/26 + Bentley iTwin 5.8/5.9 — 3,143 raw skills · 77,820 commands · auto-generated by `aware build agent --from-nuget` (NuGet ref/), `--from-yard` (YARD HTML), `--from-npm` (TypeScript .d.ts), `--from-openapi` (REST spec)) |
| Reference apps | **2** |
| Meta-primitives | **3** (agent-builder, skill-builder, html-report) |
| AECO verticals covered | **engineering · architecture · construction · visualization · cross-cutting** |

---

## Read these in order

1. [`00-vision/decalog.md`](./00-vision/decalog.md) — the five structural truths (5 min read)
2. [`00-vision/manifesto.md`](./00-vision/manifesto.md) — what AWARE is, why now, how it ships (10 min)
3. [`10-core/agent-spec.md`](./10-core/agent-spec.md) — how to write an agent
4. [`10-core/app-spec.md`](./10-core/app-spec.md) — how to write an app
5. [`30-apps/_examples/`](./30-apps/_examples/) — two worked apps showing the format end-to-end
6. [`20-agents/_core/aware-skill-builder/`](./20-agents/_core/aware-skill-builder/) — how to write or port a skill
7. [`CONTRIBUTING.md`](./CONTRIBUTING.md) — three ways to contribute, all markdown PRs

---

## Status

**Substrate v0: content-complete.** Decalog, manifesto, specs, 7 reference agents, 58 production skills, 2 reference apps, two meta-primitives, all hosted under Apache 2.0.

**What's still owed for v0.1:**
- The `aware` CLI binary (the runtime that executes the spec). Spec is detailed enough that the implementation is straightforward — but a real engineering project.
- Host-plugin generators for claude-code / codex / opencode.
- Issue tracker, PR review process, contribution badges.

Until the CLI ships, the repo is usable as **documentation + reference content**: contributors can read the substrate, port their own skills via the documented pipeline, write new agents, and compose apps in `.flo` format. The first agentic-CLI user who installs this gets a real productivity boost from the skills alone — the AI consults them when composing AECO code.

---

## License

[Apache 2.0](./LICENSE) — permissive, patent grant, no walled garden. Consistent with the decalog: AWARE's substrate is open by construction.

Commercial apps built on top of AWARE choose their own license. The substrate does not impose one.

[FloLess](https://floless.io) is one such commercial app — a visual canvas for AWARE apps. It is a separate project under its own license.

---

## Watch this repo

The substrate's content is in. The runtime is next. Star the repo to be notified when the CLI ships.
