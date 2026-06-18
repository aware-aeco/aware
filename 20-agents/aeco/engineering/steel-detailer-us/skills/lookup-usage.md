---
name: steel-detailer-us-lookup-usage
description: Use when the user wants deterministic, machine-readable rule values from the AISC database — for checker workflows, scripting, or when you need a hard provenance guarantee. Explains how to invoke the lookup command and what it returns.
---

# Deterministic lookup command (AISC)

The `steel-detailer-us` agent ships a **`lookup` CLI command** (`aware-steel-detailer-us.exe`) that queries a verified, versioned rules database without any LLM. It is the hard, lock-able provenance guarantee that sits below the advisory skill layer.

## CLI contract

```
aware-steel-detailer-us lookup --rule <id>
aware-steel-detailer-us lookup --category <cat>
aware-steel-detailer-us lookup --list
aware-steel-detailer-us describe
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

**`sections` rules carry an extra `properties` object** with typed numeric fields, so a
consumer reads machine values directly (not by parsing the `value` string):

```json
{
  "id": "section.W16X26",
  "category": "sections",
  "value": "26 lb/ft; depth d = 15.7 in; area A = 7.68 in²",
  "units": "imperial (lb/ft, in, in²)",
  "properties": { "type": "W", "weight_plf": 26.0, "depth_in": 15.7,
                  "width_in": 5.5, "area_in2": 7.68, "web_in": 0.25, "flange_in": 0.345 },
  "citation": "AISC Shapes Database v15.0 (US)",
  "found": true
}
```

The rule `id` is `section.<AISC label>` (e.g. `section.HSS6X6X3/8`); uppercase the `x` in a
drawing designation before the lookup. `wall_in` (HSS/pipe design wall) appears instead of
`web_in`/`flange_in` for tubes; **`wall_in` and the HSS/pipe `weight_plf` are the AISC
design-wall basis** (0.93× nominal for A500; nominal for A1085 — see `section-designations`).
The bulk of weights/depths are *not* derivable from the designation (HSS, angles, pipe) —
that is exactly why this lookup exists.

## Available categories

- `bolts` — spacing, edge distances, hole sizes, pretension values
- `welds` — fillet sizes, throat, length limits, PJP throat
- `connection-strength` — bearing and tearout nominal strength equations (§J3.11)
- `materials` — preferred ASTM grades and Fy/Fu by member type
- `sections` — section properties (weight/ft, depth, width, area, thicknesses) for every
  AISC shape, from the AISC Shapes Database (W, M, S, HP, C, MC, L, 2L, WT/MT/ST, HSS,
  Pipe). `lookup --rule section.<label>` or `--category sections`.

Run `aware-steel-detailer-us lookup --list` for all rule IDs (66 connection rules +
the ~2,090-shape AISC section table).

## Rules database location

- `~/.aware/agents/steel-detailer-us/rules/aisc-360-22.json` — the 66 curated,
  hand-verified connection rules (citation + source quote each).
- `~/.aware/agents/steel-detailer-us/rules/aisc-shapes-v15.json` — the generated AISC
  section table (`sections` category), merged into the same lookup at runtime. Missing is
  fine (connection rules still work); present-but-invalid is a hard error.

## How to use the lookup result

The `lookup` command is a standalone deterministic CLI (decalog #9 — no LLM in the run path): invoke it directly, or from a checker script, and consume its typed JSON. A checker compares extracted model values against `result.value`:

```
[model read] → [lookup bolt.spacing.min] → [compare & report]
```

If `found: false`, the consumer reports "rule not in verified database" and does NOT fall back to inference.

> The agent is `status: planned`: today the lookup is a standalone CLI (build it per the install note below). Composing it as a first-class node in a runnable AWARE `.app` is planned — it lights up when the agent becomes `available`.

## Install note

Binary: `~/.aware/bin/aware-steel-detailer-us.exe` (build from `20-agents/aeco/engineering/steel-detailer-lookup/` via `cargo build --release`). Requires `~/.aware/bin/` on PATH. Both rules files (`aisc-360-22.json` + `aisc-shapes-v15.json`) are installed by `aware agent install steel-detailer-us`.

## Source

- Connection rules: `20-agents/aeco/engineering/steel-detailer-us/rules/aisc-360-22.json` (verified 2026-06-14; all rules traced to free AISC / RCSC documents).
- Section table: `.../rules/aisc-shapes-v15.json` — generated from the AISC Shapes Database v15.0 (US) by `steel-detailer-lookup/tools/gen_sections.py` (vendored source CSV under `steel-detailer-lookup/data/`). Section geometry is edition-stable; v16.0 supersedes v15.0 with no change to existing shapes' dimensional properties.
- CLI source: `20-agents/aeco/engineering/steel-detailer-lookup/src/main.rs` (Rust, no LLM in the run path; decalog #9 compliant).
