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

**Output:** JSON to stdout, **UTF-8**, matching the schema:

> **Decode the output as UTF-8 explicitly — never let the runtime pick a console codepage.**
> The payload carries non-ASCII characters: `—` in prose, `§` in clause references, and `²`, `≤`, `→`, `×` in expressions.
>
> Getting this wrong fails in one of two ways, and the quiet one is the dangerous one. Both were
> measured against this binary on Windows with the codepage forced to cp1250:
>
> | decoder | what happens | rules affected |
> |---|---|---|
> | **strict** — Python `subprocess` text mode | raises `UnicodeDecodeError`. A wrapper that swallows it returns **no value**, which reads like an empty rules database rather than an encoding fault. | 3 of 68 |
> | **lenient** — Windows PowerShell 5.1 native-command capture | **does not raise.** The text comes back as mojibake — `in²` arrives as `inÂ˛` (`U+00C2 U+02DB`) — so the caller parses corrupted strings and never learns they were corrupted. | **68 of 68** |
>
> Note which column is larger. A strict decoder at least fails loudly on some rules; a lenient one
> quietly corrupts **68 of the 68**. Reading a number out of a corrupted `value`
> string is how a wrong figure reaches a drawing.
>
> To decode correctly:
>
> - **Python** — pass `encoding='utf-8'` to `subprocess.run(...)` / `check_output(...)`, or decode
>   the raw bytes with `.decode('utf-8')`.
> - **Node** — `{ encoding: 'utf8' }` covers `execFile`, `exec` and `spawnSync`. Asynchronous
>   `spawn` needs it on the stream instead: `child.stdout.setEncoding('utf8')`.
> - **PowerShell / .NET** — set `[Console]::OutputEncoding` to UTF-8 before capturing a native
>   command. That does **not** reach `System.Diagnostics.Process`; there, set
>   `ProcessStartInfo.StandardOutputEncoding`. PowerShell 7 already defaults to UTF-8.

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

> The agent is `status: available` as of **aware v0.98.0**, which ships the lookup binary: the command is dispatchable and the agent composes as a first-class node in a runnable AWARE `.app`. On an older CLI the binary is absent and the lookup will fail to spawn — upgrade rather than working around it.

## Install note

Binary: `aware-steel-detailer-eu`, shipped in the aware release archive and MSI from v0.98.0, next
to `aware`, where the CLI resolves it directly — no local build needed (the same Rust
project produces the AISC, UK, and EU binaries). Rules file installed by
`aware agent install steel-detailer-eu` — the binary reads it from
`<AWARE_HOME>/agents/steel-detailer-eu/rules/`, so both steps are needed. To build from
source instead, `cargo build --release` in
`20-agents/aeco/engineering/steel-detailer-lookup/`.

**Invoking it directly.** The `aware` CLI finds this binary on its own (it looks beside
its own executable, then falling back to PATH), so dispatching the agent from an app works on
every install that actually ships the binary. Typing
`aware-steel-detailer-eu ...` as a bare shell command additionally needs it on PATH, which
depends on how aware was installed:

| install method | bare `aware-steel-detailer-eu` on PATH? |
|---|---|
| MSI (Windows) | yes — the install dir is added to system PATH |
| `scripts/install.sh` / `install.ps1` | yes — copied next to `aware` in the install dir |
| npm / pnpm | **no** — it lives in the package-private `binaries/` directory and only `aware` gets a global shim |
| built from source | only if you copy it out of `target/release/` yourself |

If the bare command is not found, dispatch the agent through an app instead of invoking
the binary by hand — that path resolves wherever the binary was shipped alongside `aware`,
which is every install channel above except a from-source build you have not copied out.

## Source

- Rules database: `20-agents/aeco/engineering/steel-detailer-eu/rules/bs-en-1993-eu.json`
  (verified 2026-06-15; values from EN 1993-1-8:2005 via JRC EUR 27346 + eurocodeapplied.com).
- CLI source: `20-agents/aeco/engineering/steel-detailer-lookup/src/main.rs` (Rust, no LLM).
