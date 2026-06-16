---
name: steel-detailer-uk-lookup-usage
description: Use when the user wants deterministic, machine-readable rule values from the UK/Eurocode database — for checker workflows, scripting, or when you need a hard provenance guarantee. Explains how to invoke the lookup command and what it returns.
---

# Deterministic lookup command (UK / Eurocode)

The `steel-detailer-uk` agent ships a **`lookup` CLI command** (`aware-steel-detailer-uk.exe`) that queries a verified, versioned rules database without any LLM. It is the hard, lock-able provenance guarantee that sits below the advisory skill layer.

## CLI contract

```
aware-steel-detailer-uk lookup --rule <id>
aware-steel-detailer-uk lookup --category <cat>
aware-steel-detailer-uk lookup --list
aware-steel-detailer-uk describe
```

**Exit codes:** `0` = found (or listing), `1` = not found, `2` = error.

**Output:** JSON to stdout matching the schema:

```json
{
  "id": "partial.gamma.M2.connectors",
  "category": "partial-factors",
  "rule": "Partial factor γM2 — bolts, welds, pins, plates in bearing (UK NA)",
  "value": "1.25",
  "units": "dimensionless",
  "citation": "UK NA to BS EN 1993-1-8 (SCI P358 §1.6)",
  "source_quote": "P358 §1.6: γM2 = 1.25 for connectors ...",
  "found": true
}
```

When `found: false`, all fields except `id` are `null` — the caller **must refuse or flag**, not interpolate.

## Available categories

- `bolts` — edge/end distances (e1/e2), pitches (p1/p2), hole clearances, preload and slip factors
- `welds` — throat–leg relationship, directional method strengths, βw, P358 full-strength throats
- `partial-factors` — UK NA values for γM0, γM1, γM2 (two values), γM3, γM3,ser
- `materials` — S275/S355 fy by thickness, fu, Charpy subgrades (JR/J0/J2/K2)

Run `aware-steel-detailer-uk lookup --list` for all 49 rule IDs.

## Rules database location

`~/.aware/agents/steel-detailer-uk/rules/bs-en-1993-uk.json` — human-readable JSON; each rule contains citation + source quote.

## How to use in a workflow

In an AWARE `.app` that checks a UK/Eurocode model, a `steel-detailer-uk` node with the `lookup` command returns typed JSON as its `result`. Downstream nodes compare extracted model values against `result.value`:

```
[model read] → [lookup bolt.edge.min.e1] → [compare & report]
```

If `found: false`, the workflow node reports "rule not in verified database" and does NOT fall back to inference.

## Install note

Binary: `~/.aware/bin/aware-steel-detailer-uk.exe` (build from `20-agents/aeco/engineering/steel-detailer-lookup/` via `cargo build --release`; the same project produces both region binaries). Requires `~/.aware/bin/` on PATH. Rules file is installed by `aware agent install steel-detailer-uk`.

## Source

- Rules database: `20-agents/aeco/engineering/steel-detailer-uk/rules/bs-en-1993-uk.json` (verified 2026-06-14; traced to SCI P358, steelconstruction.info, SCI P363 — free sources).
- CLI source: `20-agents/aeco/engineering/steel-detailer-lookup/src/main.rs` (Rust, no LLM in the run path; decalog #9 compliant).
