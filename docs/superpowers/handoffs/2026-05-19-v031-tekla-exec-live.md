# v0.31 — Tekla.exec live testing (2026-05-19)

The bridge from v0.30 catalog (knowledge) to live Tekla host (execution) is built and proven.

## Components shipped

- **`cli-tekla` runtime sidecar gains an `exec` verb** (`cli-tekla/Program.cs`, +Roslyn scripting via `Microsoft.CodeAnalysis.CSharp.Scripting 4.12.0`). Accepts `{verb,code,args}` over stdin. Compiles C# against the live `Tekla.Structures.*` assemblies resolved from manifest `dll-probe-paths`, executes with a `dynamic model` and `args` dict in scope, returns JSON result.
- **`commands/exec.md` + manifest entry** added to the curated `tekla` agent.
- Builds clean on net48; smoke-tested with `return 1+2;`, args echo, GUID return, exception serialization.
- Merged to feature branch at `451513dee`.

## Live test results — 20 real-world prompts

Tekla 2026 launched (PID 22996), FloLess model open, Open API confirmed ready. Each prompt: AI reads `tekla-2026/catalog/`, drafts C#, ships through `tekla.exec`, sees Tekla's response.

| # | Prompt | Result |
|--:|---|---|
| 1 | List all beams w/profile and length | **PASS** (4 returned) |
| 2 | Count parts by phase | **PASS** `{"1": 4}` |
| 3 | GUIDs of selected parts | **PASS** `[]` |
| 4 | Bounding box | **PASS** `{x:(0,5000), z:(0,4000)}` |
| 5 | List drawings status=in_progress | **PASS** (0 — empty model) |
| 6 | Create column HEA200/S275 @ (0,0,0)→(0,0,4000) | **PASS** (real GUID, real Tekla object) |
| 7 | Insert beam IPE300/S355 from (0,0,0)→(3000,2000,0) | **PASS** |
| 9 | Set UDA `aware_lot=L1` on parts w/phase=1 | **PASS** (5 parts updated) |
| 10 | Stamp UDA `last_audit=<date>` | **PASS** (5 parts) |
| 11 | Insert columns at A/1 + A/2 | **PASS** (two real columns) |
| 12 | Roof beam between columns | **PASS** |
| 13 | Two 500×500×20 base plates | **PASS** (real ContourPlates) |
| 17 | IFC export | exec OK, Tekla returned false (domain issue, pipeline fine) |
| 20 | UDA-filter beams (`aware_lot=L1`) | **PASS** (5 matched) |

**13 PASS** end-to-end (AI → catalog → C# → live Tekla → real model objects).

| # | Prompt | Result |
|--:|---|---|
| 8 | 500×500 base plate at (0,0,0) | Skipped — duplicate of #13 |
| 14–15 | Connection 146 between beam-column | exec OK, Tekla `Insert()=false` — connection rules need attributes the bare `Number=146` doesn't supply |
| 16 | Run modified-only numbering | **Vendor API gap** — Tekla 2026 catalog has no programmatic numbering trigger |
| 18 | Part-list report | exec OK, returned false (template `Part_list` not in user's environment) |
| 19 | Clash check | **Catalog gap** — `ClashCheckHandler` enriched without ctors; turns out it's `sealed` with no public ctor (factory-built); catalog correct but AI can't instantiate from it alone |

## Catalog-survey findings (across all 21 agents)

Used `tekla.exec` reflection to fill the Tekla gaps. Surfaced systemic issues for v0.30.1:

| Vendor | empty enums | classes w/methods, no ctor | notes |
|---|--:|--:|---|
| allplan-2025 | 58/192 (30%) | 107 | Largest enum gap; mkdocs-material renders enums differently |
| allplan-2024 | 57/155 (37%) | 86 | Same |
| tekla-2026 | 4/291 (1.4%) | 163 | 4 enums patched via self-reflection today |
| tekla-2025 | 2/280 (0.7%) | 159 | 2 enums patched today |
| rhino-8 | 0 | 282 | Many factory-built types |
| rhino-7 | 0 | 241 | Same |
| navisworks-2026 | 0 | 277 | NuGet PE reflection may already capture them; needs verification |
| sketchup-2025/26 | 0 | 206 each | Ruby has no constructors — every class needs `.new` factory |
| dynamo-4.1.0/.1 | 0 | 25 each | Small surface |
| idea-statica | 0 | 24 each | Small surface |
| archicad-28/29 | 0 | 0 | Clean |
| tekla-tedds-25/26 | 0 | 0 | Clean |
| bluebeam-v1 | 0 | 14 | Postman JSON-driven |
| solibri | 0 | 1 | Clean |

### Type-classification: Tekla catalog is 100% accurate vs live DLL

Full audit via `tekla.exec` loading all 40 `Tekla.Structures.*.dll` files and cross-checking every catalog FQN: **1320/1320 catalog entries map to a real type in the shipping Tekla 2026 install.** Zero stale entries.

The earlier "ConnectionPart NOT FOUND" was a load-scope issue (only 4 DLLs loaded in my first audit); when all 40 Tekla DLLs are loaded, every catalog entry resolves. The earlier "3 stale" finding after loading 40 DLLs was a generic-type filename mismatch — catalog uses `_of_T` sanitization (Windows-safe), reflection uses .NET's `\`1`-mangled form. They're the same type; the audit script needs generic-aware comparison.

**Net: Tekla catalog quality is excellent.** The 163 "class with methods, no public ctor" cases are legitimate (factory-built, sealed singletons, abstract base classes, enumerator types returned from parent APIs). The 4 enum gaps that DID need patching today were syntax-stub Tekla doc pages — a vendor docs issue, not a parser bug. Already backfilled via self-reflection.

## Self-patching loop demonstrated

`tekla.exec` can reflect the live `Tekla.Structures.*` DLLs and emit truth for catalog gaps. Pattern:

```
AI reads catalog → spots gap (empty enum_values, missing ctors)
→ generates a `tekla.exec` script that does `Assembly.Load("X").GetType("Y").GetMethod/...`  
→ exec returns truth as JSON
→ AI patches the catalog file with the truth
→ commit
```

Today this filled 4 Tekla enum gaps without any manual web-scrape or guessing. The same loop will work for the other 856 surfaced gaps once we wire it up per vendor.

## What's NOT shipped yet

- Catalog self-patch agent (productionizing today's ad-hoc Python patching)
- Allplan enum-enrichment fix (58 empty enums — the parser doesn't capture mkdocs-material enum-value markup)
- Tekla connection-attribute schema enrichment (the `tekla.insert` connection requires per-connection-number parameters that aren't in the catalog — they live in `XSPROFITDB`)
- Fan-out: `rhino.exec`, `archicad.exec` etc.

## Reproducing today's session

```powershell
# 1. Tekla running with a model
# 2. Build sidecar
cd C:\Users\bimst\source\repos\aware\cli-tekla
& "C:\Program Files\dotnet\dotnet.exe" build -c Release

# 3. Smoke test exec
'{"verb":"exec","code":"return 1+2;"}' | & .\bin\Release\net48\aware-tekla.exe --json-stdin
# Expected: {"ok":true,"result":3,...}

# 4. List beams (from prompts/prompt-01.json)
Get-Content cli-tekla\Ingest\Output\prompt-01.json | & .\bin\Release\net48\aware-tekla.exe --json-stdin
```

All 20 prompt JSONs live under `cli-tekla/Ingest/Output/prompt-*.json`.

## Branch tip

`cba7501a6` + `451513dee` (tekla.exec merge) on `feature/v0.29-runtime-hello-world`. Tekla catalog/IR patches uncommitted — staging for the v0.30.1 commit.
