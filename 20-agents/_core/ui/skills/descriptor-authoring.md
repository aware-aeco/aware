---
name: ui-descriptor-authoring
description: Compose declarative UI descriptors, write UI descriptor JSON, custom panels, dashboard composition, stat cards, data tables, declarative dashboard, prompt-composed UI, UI blocks, panel descriptor, host-rendered UI, server-driven UI, ui validate, ui catalog, ui render.
---

# Descriptor Authoring Guide

This is the guide for an AI (in Claude Code, Codex, or any AWARE host) that wants to
compose a **UI descriptor**: a small JSON document describing panels of typed blocks.
You write the *description*; a deterministic engine validates and renders it — you never
write HTML, CSS, or code. This is the same division of labor as `.flo` compose → compile.

## The contract

- **You compose, AWARE validates/renders.** Check your descriptor with `ui.validate`
  before handing it to a host. Discover what you may compose with `ui.catalog` —
  the catalog and the validator are built from the same table, so the catalog is
  always the truth.
- **Hosts render chrome.** A descriptor declares structure and intent (panels, blocks,
  slots, actions); the HOST decides pixels — it renders blocks with its own design
  system. `ui.render` exists as a deterministic *fallback* (self-contained HTML,
  inline CSS, no JS) for hosts without a renderer and for `.flo` apps emitting a
  composed dashboard.
- **Forward compatibility.** An unknown block type is a `validate` WARNING and renders
  as a visible placeholder — never an error. So a descriptor written for a newer
  schema degrades gracefully on an older renderer, and you may not rely on
  unknown-type behavior for anything except a placeholder.
- **No raw HTML, ever.** Every string you put in a descriptor is escaped at render
  time. `text` blocks are plain text with paragraph breaks; HTML in any field shows
  up as literal text, not markup.

## Schema v1

```json
{
  "version": 1,
  "panels": [
    {
      "id": "string (required, unique, [a-z0-9-]+)",
      "slot": "string (required — host-interpreted, e.g. \"dashboard\")",
      "title": "string (required)",
      "blocks": [ { "type": "...", "...": "per-type fields" } ]
    }
  ]
}
```

- `version` must be the integer `1`.
- `panels` is an ordered list; each panel is an ordered list of blocks. There is no
  layout math in v1 — order is the layout.
- `slot` is a free string the HOST interprets ("dashboard", "sidebar", "run-summary",
  …). It declares *where the panel wants to live*; the host decides what that means.
  Ask the host (or its docs) which slot names it honors.
- `id` must be unique across panels and match `[a-z0-9-]+` — hosts key panels by it.

## Block types (v1)

Validation is strict on declared fields (wrong type / missing required = error) and
lenient on extras (unknown extra field on a known block = warning).

### `stat` — a single key metric

| field | type | required | |
|-------|------|----------|---|
| `label` | string | yes | short metric name |
| `value` | string \| number | yes | the metric value |
| `hint` | string | no | fine print under the value |

```json
{ "type": "stat", "label": "Total weight", "value": "12,450 kg", "hint": "phase 2 only" }
```

Consecutive `stat` blocks are rendered as one row of cards by the fallback renderer —
lead a panel with 2–4 of them for a summary strip.

### `table` — a data table bound to a named payload

| field | type | required | |
|-------|------|----------|---|
| `source` | string | yes | key into the render-time `data` map |
| `columns` | array\<string\> | no | columns to show, in this order (omit = all keys) |
| `sort` | string | no | row sort key (numbers numeric, else string form; missing sorts last) |
| `sort-desc` | boolean | no | sort descending (needs `sort`) |

`columns`/`sort` reuse html-report's semantics (#210). The descriptor names the data
(`source`); the actual payload arrives separately in `render`'s `data` input. A
`source` with no payload at render time shows a "no data for 'X'" placeholder — your
descriptor can outlive any one run.

```json
{ "type": "table", "source": "parts", "columns": ["name", "profile", "weight"],
  "sort": "weight", "sort-desc": true }
```

### `text` — escaped prose

| field | type | required | |
|-------|------|----------|---|
| `content` | string | yes | plain text; blank lines separate paragraphs |

No raw HTML pass-through, ever — markup is escaped to literal text.

### `report` — embed a generic auto-render of a payload

| field | type | required | |
|-------|------|----------|---|
| `source` | string | yes | key into the render-time `data` map |
| `title` | string | no | sub-heading above the embedded report |

Embeds html-report's generic auto-render of that payload: array of objects → table,
object → field/value table, scalar → value card. Use `report` when you don't know
the payload's shape; use `table` when you want column/sort control.

### `action` — a declared intent (host wires the behavior)

| field | type | required | |
|-------|------|----------|---|
| `label` | string | yes | button label |
| `action-id` | string | yes | host-interpreted identifier of the intent |
| `inputs` | object | no | inputs the host passes when triggering it |

An `action` declares *what the user should be able to trigger* — it does not say how.
The fallback renderer emits an INERT disabled-styled button carrying
`data-action-id` (and `data-inputs`); a real host binds its own behavior (e.g.
relaying the intent to the terminal AI, or running a `.flo` app). Never assume an
action executes anything by itself.

```json
{ "type": "action", "label": "Re-run BOM", "action-id": "rerun-bom", "inputs": { "phase": 2 } }
```

## The commands

```sh
# What may I compose? (machine-readable block contracts)
aware agent invoke ui catalog

# Is this descriptor valid? (errors/warnings with JSON paths)
aware agent invoke ui validate --inputs '{"descriptor": { ... }}'
aware agent invoke ui validate --inputs @descriptor-args.json

# Fallback-render it (optionally with data payloads, optionally to a file)
aware agent invoke ui render --inputs '{"descriptor": { ... }, "data": { ... }, "output-path": "panel.html"}'
```

In a `.flo` app, the same commands compose as nodes (`agent: ui`, `command: render`),
with upstream node outputs flowing into `data`.

## Full worked example

A dashboard panel for a Tekla BOM run — summary strip, the parts table, a prose note,
and a re-run intent:

```json
{
  "version": 1,
  "panels": [
    {
      "id": "bom-overview",
      "slot": "dashboard",
      "title": "BOM — Phase 2",
      "blocks": [
        { "type": "stat", "label": "Parts", "value": 128 },
        { "type": "stat", "label": "Total weight", "value": "12,450 kg" },
        { "type": "stat", "label": "Status", "value": "OK", "hint": "model synced" },
        { "type": "table", "source": "parts",
          "columns": ["name", "profile", "weight"],
          "sort": "weight", "sort-desc": true },
        { "type": "text",
          "content": "Weights are net values from the live model.\n\nRe-run after model changes to refresh." },
        { "type": "report", "source": "run-summary", "title": "Run summary" },
        { "type": "action", "label": "Re-run BOM", "action-id": "rerun-bom",
          "inputs": { "phase": 2 } }
      ]
    }
  ]
}
```

Rendered with:

```json
{
  "descriptor": { "...": "the descriptor above" },
  "data": {
    "parts": [
      { "name": "Beam B-104", "profile": "HEA300", "weight": 412.5 },
      { "name": "Column C-12", "profile": "HEB400", "weight": 980.0 }
    ],
    "run-summary": { "app": "tekla-bom-by-phase", "phase": 2, "duration-ms": 8421 }
  }
}
```

## Authoring checklist

1. `ui.catalog` first if unsure of the vocabulary — never guess field names.
2. Keep panels small and purposeful: a summary strip (`stat`s), one or two data
   blocks (`table`/`report`), prose only where it earns its place (`text`),
   intents as `action`s.
3. Bind data by NAME (`source`), never inline rows into the descriptor — the
   descriptor is the stable description, payloads change per run.
4. `ui.validate` before shipping; fix errors, review warnings (typos surface as
   "unknown field" warnings).
5. Don't fight the host: slots and actions are its territory — declare intent,
   let it decide presentation and behavior.
