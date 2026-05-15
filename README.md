# AWARE

> **AWARE is what comes after software-as-product. Apps are text — files you can read in Notepad. AI is the runtime that executes them. Open source is what the format does automatically: there's nothing else to a "proprietary" app once it's plain English. No vendor, no installer, no walled garden. AECO is the wedge; the substrate is universal.**

— [the statement](./00-vision/manifesto.md). The five structural truths it rests on are in [the decalog](./00-vision/decalog.md).

---

## 60-second install demo

```bash
$ winget install aware-aeco      # Windows
$ brew install aware-aeco        # Mac
$ curl -fsSL https://aware-aeco.dev/install | sh   # Linux / WSL

$ claude-code                    # or codex, or opencode
  ✓ plugin: aware-aeco v0.1 · 12 agents available

> Watch this Tekla model. When a welded assembly appears,
> upload its drawing to my Trimble Connect fab folder.

  → composed [Tekla Watcher] → [Welded Filter] → [TC Uploader]
  → wrote ~/.aware/apps/welded-to-tc.flo

$ aware app run welded-to-tc
```

That's the whole thing. One sentence in your terminal, one plain-text file, one command to run.

---

## Read these in order

1. [`00-vision/decalog.md`](./00-vision/decalog.md) — the five structural truths
2. [`00-vision/manifesto.md`](./00-vision/manifesto.md) — what AWARE is, why now, how it ships
3. [`10-core/agent-spec.md`](./10-core/agent-spec.md) — how to write an agent
4. [`10-core/app-spec.md`](./10-core/app-spec.md) — how to write an app
5. [`CONTRIBUTING.md`](./CONTRIBUTING.md) — three ways to contribute, all markdown PRs

---

## Layout

```
aware-aeco/
├── 00-vision/      # decalog · manifesto · positioning
├── 10-core/        # agent-spec · app-spec · runtime contract
├── 20-agents/      # first-party AECO agents + the agent builder
├── 30-apps/        # reference apps + community examples
├── 40-diagrams/    # framework diagrams (excalidraw + mermaid)
└── 50-research/    # design notes, prior art, competitive analysis
```

---

## License

[Apache 2.0](./LICENSE) — permissive, patent grant, no walled garden. The license is consistent with the decalog: AWARE's substrate is open by construction.

Commercial apps built on top of AWARE choose their own license. The substrate does not impose one.

---

## Status

Pre-v0. Manifesto and specs locked. Reference agents in progress. CLI binary coming. Watch this repo if you want to be in early.
