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

**Output:** JSON to stdout, **UTF-8**, matching the schema:

> **Decode it as UTF-8 explicitly.** The output carries non-ASCII characters — § (clause refs), `—`, `²`, `≤` and `γ` — from the
> EN 1993-1-8 / BS clause references. A caller that lets its runtime pick the console codepage instead will **throw while
> decoding**, and the usual shape of that bug is a wrapper swallowing the error and returning *no
> value* — a silent failure that reads like a missing rule rather than an encoding problem.
>
> Measured across all 49 rule ids: **2** fail to decode under cp1250 while every one is
> clean UTF-8. Few, but the ones that break are ordinary rules, not exotica.
>
> In Python pass `encoding='utf-8'` to `subprocess.run(...)`/`check_output(...)`; in Node set
> `{ encoding: 'utf8' }`; in PowerShell set `[Console]::OutputEncoding` before capturing.

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

## How to use the lookup result

The `lookup` command is a standalone deterministic CLI (decalog #9 — no LLM in the run path): invoke it directly, or from a checker script, and consume its typed JSON. A checker compares extracted model values against `result.value`:

```
[model read] → [lookup bolt.edge.min.e1] → [compare & report]
```

If `found: false`, the consumer reports "rule not in verified database" and does NOT fall back to inference.

> The agent is `status: available` as of **aware v0.98.0**, which ships the lookup binary: the command is dispatchable and the agent composes as a first-class node in a runnable AWARE `.app`. On an older CLI the binary is absent and the lookup will fail to spawn — upgrade rather than working around it.

## Install note

Binary: `aware-steel-detailer-uk`, shipped in the aware release archive and MSI from v0.98.0, next to `aware`, where the CLI resolves it directly — no local build needed (the same project produces all three region binaries). Rules file is installed by `aware agent install steel-detailer-uk` — the binary reads it from `<AWARE_HOME>/agents/steel-detailer-uk/rules/`, so both steps are needed. To build from source instead, `cargo build --release` in `20-agents/aeco/engineering/steel-detailer-lookup/`.

**Invoking it directly.** The `aware` CLI finds this binary on its own (it looks beside
its own executable), so dispatching the agent from an app works on every install. Typing
`aware-steel-detailer-uk ...` as a bare shell command additionally needs it on PATH, which
depends on how aware was installed:

| install method | bare `aware-steel-detailer-uk` on PATH? |
|---|---|
| MSI (Windows) | yes — the install dir is added to system PATH |
| `scripts/install.sh` / `install.ps1` | yes — copied next to `aware` in the install dir |
| npm / pnpm | **no** — it lives in the package-private `binaries/` directory and only `aware` gets a global shim |
| built from source | only if you copy it out of `target/release/` yourself |

If the bare command is not found, dispatch the agent through an app instead of invoking
the binary by hand — that path always resolves.

## Source

- Rules database: `20-agents/aeco/engineering/steel-detailer-uk/rules/bs-en-1993-uk.json` (verified 2026-06-14; traced to SCI P358, steelconstruction.info, SCI P363 — free sources).
- CLI source: `20-agents/aeco/engineering/steel-detailer-lookup/src/main.rs` (Rust, no LLM in the run path; decalog #9 compliant).
