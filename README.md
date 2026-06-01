# AWARE

> **AWARE is what comes after software-as-product. Apps are text — files you can read in Notepad. AI is the runtime that executes them. Open source is what the format does automatically: there's nothing else to a "proprietary" app once it's plain English. No vendor, no installer, no walled garden. AECO is the wedge; the substrate is universal.**

— [the statement](./00-vision/manifesto.md). The nine structural truths it rests on are in [the decalog](./00-vision/decalog.md).

---

## 60-second install demo

```bash
$ npm install -g @aware-aeco/cli                       # any OS — recommended
  # alternatives that work today:
  #   curl ... install.sh | bash             # Linux / Mac, no Node
  #   iex (irm ... install.ps1)              # Windows PowerShell, no Node

$ claude-code                    # or codex, or opencode
  ✓ plugin: aware-aeco · 66 agents · 3,282 skills

> Watch this Tekla model. When a welded assembly appears,
> upload its drawing to my Trimble Connect fab folder.

  → composed [Tekla Watcher] → [Welded Filter] → [TC Uploader]
  → wrote ~/.aware/apps/welded-to-tc.flo

$ aware app run welded-to-tc
```

That's the whole thing. One sentence in your terminal, one plain-text file, one command to run.

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
├── 30-apps/_examples/      # 7 reference apps (.flo) — one per persona + canonical demos
├── 40-diagrams/            # Mermaid + Excalidraw views of the substrate
├── 50-research/            # design notes, prior art, competitive analysis
├── 90-onboarding/          # first-hour walk-throughs, one per persona
├── cli/                    # the `aware` CLI — Rust runtime · v0.53.0 (shipped)
├── cli-tekla / -revit /    # desktop-host sidecars — stateful, in-process vendor APIs
│   -rhino / -sketchup
├── cli-roslyn / -reader /  # C# source reader (Roslyn) + shared IR reader + sidecar lib
│   -sidecar
├── cli-npm/                # @aware-aeco/cli — the npm wrapper (published)
├── packaging/wix/          # Windows MSI installer (winget path — in progress)
├── scripts/                # install.sh · install.ps1 · agent generators
└── registry-index.json     # the registry — source of truth for installable agents
```

### Stats — as of 2026-06-01 (regenerate from `registry-index.json` + the tree)

| | Count |
|---|---|
| Agents | **<!--stat:agents_total-->66<!--/stat-->** — all registered & installable |
| — curated (hand-written skills) | **<!--stat:agents_curated-->17<!--/stat-->** |
| — reflected (auto-generated: NuGet / npm / YARD / OpenAPI) | **<!--stat:agents_reflected-->49<!--/stat-->** |
| Skills | **<!--stat:skills-->3,282<!--/stat-->** |
| API commands | **<!--stat:commands-->46,813<!--/stat-->** command files · **<!--stat:catalog-->12,162<!--/stat-->** catalog entries |
| Reference apps | **<!--stat:apps-->7<!--/stat-->** (`.flo`) |
| Meta-primitives | **<!--stat:meta_primitives-->5<!--/stat-->** (agent-builder · skill-builder · html-report · http · ui-inspector) |
| Disciplines | engineering · architecture · construction · visualization · cross-cutting |

> Numbers grow as agents land. [`registry-index.json`](./registry-index.json) is the source of truth — every agent in the tree is registered.

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

The `aware` CLI is live at **v0.53.0** (Rust), published to npm as **`@aware-aeco/cli`**, with curl + PowerShell installers in [`scripts/`](./scripts/). What began as 7 reference agents is now a working substrate:

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

[FloLess](https://floless.io) is one such commercial app — a visual canvas for AWARE apps. It is a separate project under its own license.

---

## Watch this repo

The substrate is in and the runtime shipped. New agents land continuously — [`registry-index.json`](./registry-index.json) is the live list. Star the repo, install the CLI, and compose something:

```bash
npm install -g @aware-aeco/cli
```
