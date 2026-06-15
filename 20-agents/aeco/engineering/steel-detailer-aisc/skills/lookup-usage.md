---
name: steel-detailer-aisc-lookup-usage
description: Use when the user wants deterministic, machine-readable rule values from the AISC database — for checker workflows, scripting, or when you need a hard provenance guarantee. Explains how to invoke the lookup command and what it returns.
---

# Deterministic lookup command (AISC)

The `steel-detailer-aisc` agent ships a **`lookup` CLI command** (`aware-steel-detailer-aisc.exe`) that queries a verified, versioned rules database without any LLM. It is the hard, lock-able provenance guarantee that sits below the advisory skill layer.

## CLI contract

```
aware-steel-detailer-aisc lookup --rule <id>
aware-steel-detailer-aisc lookup --category <cat>
aware-steel-detailer-aisc lookup --list
aware-steel-detailer-aisc describe
```

**Exit codes:** `0` = found (or listing), `1` = not found, `2` = error.

**Output:** JSON to stdout matching the schema:

```json
{
  "id": "bolt.pretension.group120.0.75in",
  "category": "bolts",
  "rule": "Minimum pretension — Group 120, ¾ in. bolt",
  "value": "28 kips",
  "units": "kips",
  "citation": "RCSC 2020 Table 5.2 (= AISC 360-22 Table J3.1)",
  "source_quote": "Table 5.2: ¾ in., Group 120 = 28 kips",
  "found": true
}
```

When `found: false`, all fields except `id` are `null` — the caller **must refuse or flag**, not interpolate.

## Available categories

- `bolts` — spacing, edge distances, hole sizes, pretension values
- `welds` — fillet sizes, throat, length limits, PJP throat
- `connection-strength` — bearing and tearout nominal strength equations (§J3.11)
- `materials` — preferred ASTM grades and Fy/Fu by member type

Run `aware-steel-detailer-aisc lookup --list` for all 66 rule IDs.

## Rules database location

`~/.aware/agents/steel-detailer-aisc/rules/aisc-360-22.json` — human-readable JSON; each rule contains citation + source quote.

## How to use in a workflow

In a `.flo` that checks a model, a `steel-detailer-aisc` node with the `lookup` command returns typed JSON as its `result`. Downstream nodes compare extracted model values against `result.value`:

```
[model read] → [lookup bolt.spacing.min] → [compare & report]
```

If `found: false`, the workflow node reports "rule not in verified database" and does NOT fall back to inference.

## Install note

Binary: `~/.aware/bin/aware-steel-detailer-aisc.exe` (build from `20-agents/aeco/engineering/steel-detailer-lookup/` via `cargo build --release`). Requires `~/.aware/bin/` on PATH. Rules file is installed by `aware agent install steel-detailer-aisc`.

## Source

- Rules database: `20-agents/aeco/engineering/steel-detailer-aisc/rules/aisc-360-22.json` (verified 2026-06-14; all rules traced to free AISC / RCSC documents).
- CLI source: `20-agents/aeco/engineering/steel-detailer-lookup/src/main.rs` (Rust, no LLM in the run path; decalog #9 compliant).
