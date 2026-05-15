# Reference apps

Two worked examples that demonstrate the AWARE app composition format end-to-end. Read these alongside [`10-core/app-spec.md`](../../10-core/app-spec.md).

| App | File | Layout | Demonstrates |
|---|---|---|---|
| **Welded → TC Uploader** | [`welded-to-tc.flo`](./welded-to-tc.flo) | linear (3 nodes) | The canonical 60-second-demo composition. Stateful trigger (`tekla.watch`) → inline glue (`predicate`) → stateless sink (`trimble-connect.upload`). Idempotency by Mark. `exposes-as-agent: true`. |
| **QA Drawings to Tekla** | [`qa-drawings-to-tekla.flo`](./qa-drawings-to-tekla.flo) | DAG (7 nodes, 6 edges) | Real AECO pipeline. **Two parallel triggers** (file watcher + Excel watcher), **fan-in** at a Think Node that pairs drawing + spec and validates against tolerance, **fan-out** to two sinks (Tekla insert + Slack notify). |

## What to look for

### In `welded-to-tc.flo`

- The `nodes:` block lists agent invocations + one inline glue step
- Inline glue (`filter-welded`) appears in the topology — no hidden logic
- Templating: `{{ tekla-watch.mark }}` references the upstream node's output
- `exposes-as-agent: true` declares this app as an agent for downstream consumers
- `requires:` pins agent minor versions

### In `qa-drawings-to-tekla.flo`

- `layout: dag` with explicit `row` / `col` per node — the canvas (FloLess) renders this as a grid
- Fan-in: `match-build` has two incoming edges, each with a named `input:` slot (drawing / spec)
- Fan-out: `match-build` has two outgoing edges to different sinks
- `connections:` carry typed labels (`PdfFile`, `Drawing`, `Spec`, `ValidatedJob`)
- App-level skills capture configuration knowledge (`tolerance-policy.md`)

## How to run (once the AWARE CLI ships)

```bash
# Install
aware app install ./welded-to-tc.flo
aware app install ./qa-drawings-to-tekla.flo

# Run
aware app run welded-to-tc -- \
  --tc-project-id "..." \
  --tc-folder-id "..."

aware app run qa-drawings-to-tekla
```

Each `.flo` file is plain text. Read it in Notepad. Edit it by hand. Diff it in git. Share it as an email attachment. That is the whole substrate — text and a runtime.

## Authoring your own

1. Open Claude Code (or codex, opencode) with `aware-aeco` installed
2. Describe what you want in plain English
3. The AI composes the `.flo` file using the installed agents
4. Inspect it visually with FloLess (the commercial canvas) when it ships, or via `aware app show <name>`

See [`CONTRIBUTING.md`](../../CONTRIBUTING.md) for how to publish your apps to the community registry.
