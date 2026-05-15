# Diagrams

Two views of the AWARE substrate.

| File | Purpose | View it |
|---|---|---|
| **[aware-master.mmd](./aware-master.mmd)** | Mermaid source — renders inline on GitHub when embedded in `.md` | Embed in any markdown file with ` ```mermaid `; or paste into [mermaid.live](https://mermaid.live) |
| **[aware-master.excalidraw](./aware-master.excalidraw)** | Excalidraw source — hand-drawn, freeform, the visual blueprint | Open in [excalidraw.com](https://excalidraw.com) → File → Open, or install the Obsidian Excalidraw plugin and open this repo as a vault |

## Conventions

- **Green nodes** = v0 POC scope (the ★ symbol in the Mermaid version)
- **Gray nodes** = post-v0 / future scope
- **Blue substrate** = the always-free, Apache-2.0 core
- **Yellow meta** = `aware-agent-builder` — the agent that makes agents
- **Pink commercial** = apps built on AWARE that ship under their own license (e.g. FloLess)

## When to update which

- **`.mmd` first** when adding/removing high-level boxes — the Mermaid source is the canonical hierarchy, easy to diff in PRs.
- **`.excalidraw` first** when arranging visual layouts, sketching new flows, drafting talks/slides — Excalidraw is freeform and forgiving.
- **Keep both in sync** at release time. The Mermaid is the README-facing view; the Excalidraw is for designers / talks / docs sites.

## Adding a new diagram

Drop new files in this folder. Name by what they show, not by tool:

- `agent-lifecycle.mmd` — install/run/uninstall states
- `app-execution.excalidraw` — what happens when `aware app run` fires
- `topology-examples.mmd` — linear vs DAG vs fan-in patterns

Each new diagram should be linked from this README with a one-line description.
