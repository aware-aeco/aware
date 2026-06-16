---
name: steel-detailer-eu-lookup-usage
description: Use when the user wants deterministic, machine-readable rule values from the EU/Eurocode recommended-values database — for checker workflows, scripting, or when a hard provenance guarantee is needed. Explains how to invoke the lookup command and what it returns.
---

# Deterministic lookup command (EU / Eurocode recommended values)

The `steel-detailer-eu` agent ships a **`lookup` CLI command** (`aware-steel-detailer-eu.exe`)
that queries a verified, versioned rules database without any LLM. It is the hard,
lock-able provenance layer below the advisory skills.

## CLI contract

```
aware-steel-detailer-eu lookup --rule <id>
aware-steel-detailer-eu lookup --category <cat>
aware-steel-detailer-eu lookup --list
aware-steel-detailer-eu describe
```

**Exit codes:** `0` = found (or listing), `1` = not found, `2` = error.

**Output:** JSON to stdout matching the schema:

```json
{
  "id": "partial.gamma.M2.connectors.recommended",
  "category": "partial-factors",
  "rule": "Partial factor γM2 — bolts, welds, pins, plates in bearing (EN recommended)",
  "value": "1.25",
  "units": "dimensionless",
  "citation": "EN 1993-1-8:2005 §6.1 (JRC EUR 27346 p.13)",
  "source_quote": "JRC EUR 27346 §1.2.1: γM2 = 1.25 (recommended value for connections)",
  "ndp": true,
  "found": true
}
```

**`ndp: true`** means this is an NDP — the EN recommended value; a country NA may differ.
The caller MUST propagate this flag to the end user.

When `found: false`, all fields except `id` are `null` — the caller **must refuse or flag**,
not interpolate.

## Available categories

- `bolts` — end/edge distances (e1/e2), pitches (p1/p2), hole clearances, categories,
  preload formula, slip factors (μ, ks), shear αv
- `welds` — throat–leg (a = 0.7s), directional-method formula, βw factors (all grades),
  fu values (EN 1993-1-1 Table 3.1), minimum effective length
- `partial-factors` — EN recommended γM0, γM1, γM2, γM3, γM3,ser (all NDP-flagged)
- `materials` — S235/S275/S355/S420/S460 fy by thickness and fu (EN 1993-1-1 Table 3.1)

Run `aware-steel-detailer-eu lookup --list` for all rule IDs.

## Rules database location

`~/.aware/agents/steel-detailer-eu/rules/bs-en-1993-eu.json` — human-readable JSON;
each rule contains citation, source quote, and `ndp` boolean.

## How to use the lookup result

The `lookup` command is a standalone deterministic CLI (decalog #9 — no LLM in the run path): invoke it directly, or from a checker script, and consume its typed JSON.

```
[model read] → [lookup bolt.pitch.min.p1] → [compare & report NDP warning if ndp:true]
```

If `found: false`, the consumer reports "rule not in verified database" and does NOT
fall back to inference.

> The agent is `status: planned`: today the lookup is a standalone CLI (build it per the install note below). Composing it as a first-class node in a runnable AWARE `.app` is planned — it lights up when the agent becomes `available`.

## Install note

Binary: `~/.aware/bin/aware-steel-detailer-eu.exe` (build from
`20-agents/aeco/engineering/steel-detailer-lookup/` via `cargo build --release`; the
same Rust project produces the AISC, UK, and EU binaries). Rules file installed by
`aware agent install steel-detailer-eu`.

## Source

- Rules database: `20-agents/aeco/engineering/steel-detailer-eu/rules/bs-en-1993-eu.json`
  (verified 2026-06-15; values from EN 1993-1-8:2005 via JRC EUR 27346 + eurocodeapplied.com).
- CLI source: `20-agents/aeco/engineering/steel-detailer-lookup/src/main.rs` (Rust, no LLM).
