# v0.30 Phase B Tier A Complete — 2026-05-19

Tier A of v0.30 Phase B is done. **7 vendor extractors × 14 versions PASS** the host-coverage protocol on branch `feature/v0.29-runtime-hello-world`.

## What's shipped

| Vendor | Versions | Source | Strict 50/50 | Schema validation |
|---|---|---|:---:|:---:|
| Tekla Structures (B1) | 2025, 2026 | developer.tekla.com (Sandcastle) | PASS / PASS | 0 violations |
| Rhino (B5) | 7, 8 | github raw mcneel/rhinocommon-api-docs | PASS / PASS | 0 violations |
| Grasshopper (B6) | 7, 8 | github raw mcneel/grasshopper-api-docs | PASS / PASS | 0 violations |
| SketchUp (B4) | 2025, 2026 | ruby.sketchup.com (Ruby YARD) | PASS / PASS | 0 violations |
| ArchiCAD (B7) | 28, 29 | Tapir GitHub + Graphisoft DevPortal | PASS / PASS | 0 violations |
| Tekla Tedds (B9) | 25, 26 | developer.tekla.com/doc/tekla-tedds (DocFX) | PASS / PASS | 0 violations |
| IDEA Statica (B8) | 25, 26 | github idea-statica/ideastatica-public (OpenAPI+md) | PASS / PASS | 0 violations |

**Total:** ~5,500+ types across all catalogs, ~95,000+ enriched members. Each vendor's coverage gated by the 4-step `host-coverage-review-protocol.md` (type enumeration FULL, deep-check N=50, behavior N=20, schema 0 violations).

## Test + build state

- **Sidecar:** `dotnet build` + `dotnet publish -c Release -r win-x64 --self-contained` clean (0 warnings, 0 errors)
- **Tests:** 77 cli-sidecar.Tests + 36 Ingest/Generator/Tests + 1 cargo coverage_e2e — all green
- **Code:** all extractors AOT-clean, no reflection-based JSON, source-gen via `IrJsonContext`

## Infrastructure built during Tier A

- `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json` — standalone catalog Type schema (catalog files validate against this; root IR validates against the original `host-coverage.schema.json`)
- `cli-sidecar/Ingest/Extractors/Common/HttpScraper.cs` — AOT-clean HTTP wrapper (replaces Playwright which is NOT AOT-compatible despite Phase A handoff lesson #4)
- `cli-sidecar/Ingest/Output/extract-with-restart.ps1` — auto-restart wrapper for CDN throttle handling
- `cli-sidecar/Ingest/Output/verify-types-strict.ps1` — strict 50-type sampler with Sandcastle/DocFX/markdown/YAML format dispatch
- `cli-sidecar/Ingest/Output/enrichment-stats.ps1` — member fill-rate reporter
- `docs/superpowers/vendor-impl-brief.md` — shared 13-point defensive checklist for new vendor implementers

## Protocol refinements landed

The codex-coverage protocol grew during Tier A as real defects surfaced:

1. **Schema split** — catalog files validate against `host-coverage-type.schema.json` (not the root IR schema)
2. **Catalog filename sanitization** — `_of_` for `<`, strip `>` (Windows-safe for generic types)
3. **Multi-page enumeration** — sidebar-tree navigation acknowledged (Tekla, others)
4. **SyncRoot whitelist** — `.NET BCL` `ICollection.SyncRoot` properties legitimately `type=object`
5. **LST `cs=` regex** — must accept mid-pipe-list position (`[?|]cs=([^|"]*)`)
6. **Clustered-FAIL investigation** — 3+ same-cause FAILs → spot-check before declaring FAIL
7. **Sandcastle reviewer rules** — `[Missing <summary>]` filter; external/inherited member skip; multiple member tables (operatorList, fieldList, conversionList) fold into one catalog list
8. **DocFX reviewer rules** — kind word LEADS (`Class Foo` not `Foo Class`); alternate spellings (`Enum`/`Struct` vs `Enumeration`/`Structure`)
9. **Hybrid OpenAPI/markdown reviewer rules** — no HTML; tag-based API-class synthesis; Async-suffix mismatch between heading and anchor id
10. **Sample-size adaptation** — vendors with N < 50 types sample full set (Tekla Tedds: 33/33)

## What's owed for v0.30 Phase B completion

**Tier B remaining (5 vendors, ~7 versions):**

- **B10 Allplan (2024 + 2025)** — PythonParts API PDF + `.py` type stubs shipped with SDK. PDF extraction via PdfPig or similar. Vertical: `architecture`.
- **B11 Dynamo (4.1.0 + 4.1.1)** — DynamoBIM.org dev docs + Autodesk Forge API pages. Vertical: `engineering` or `architecture` (Dynamo is the visual-programming scripting layer for Revit).
- **B12 Navisworks (2026)** — CHM-based, same shape as Revit/AutoCAD CHM extraction. Use `Common/ChmParser.cs`. Single version. Vertical: `architecture`.
- **B13 Solibri (single)** — Solibri OSO REST API OpenAPI spec + Help portal. Vertical: `architecture`.
- **B14 Bluebeam (single)** — `developer.bluebeam.com/studio-api/`. Smallest surface. Vertical: `architecture`.

Each follows the proven pattern in `docs/superpowers/vendor-impl-brief.md`. Codex-coverage protocol verdict per agent before commit.

## Open observation worth tracking

The `aware coverage validate` Rust CLI command is currently a JSON-parse stub. Full schema validation against both `host-coverage.schema.json` and `host-coverage-type.schema.json` is performed externally during codex-coverage reviews (via `ajv@8` + `ajv-formats@3` or Python `jsonschema 4.26.0`). A follow-up should wire a `coverage-validate` sidecar verb that does both validations in one shot. Not blocking for Tier B; should land before v0.30 PR.

## Reproduction commands

```bash
# Verify any agent's catalog against the schema:
npx --yes ajv-cli@5 validate -s cli-sidecar/Ingest/Schema/host-coverage-type.schema.json -d "20-agents/aeco/<vertical>/<host>-<ver>/catalog/*.json"

# Run any vendor's extraction end-to-end:
pwsh -File cli-sidecar/Ingest/Output/extract-with-restart.ps1 -Version <ver>   # if a vendor-specific wrapper exists; else use the sidecar verb directly

# Run the strict 50-type verify:
pwsh -File cli-sidecar/Ingest/Output/verify-types-strict.ps1 -IrPath cli-sidecar/Ingest/Output/<host>-<ver>.ir.json
```

## Branch tip at handoff time

`26599b162 fix(v0.30 spec): host-coverage-review-protocol — DocFX + Hybrid OpenAPI vendor rules, sample-size adaptation`

Commits added during Tier A: 36+ (extractors, IRs, agents, reviews, protocol updates, merges).
