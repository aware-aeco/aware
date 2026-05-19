# tekla-tedds-26 EXTRACTION NOTES

This agent is generated from a coverage IR scraped from `developer.tekla.com`. These notes capture **how** the IR was produced so the extraction can be reproduced — and so the next reviewer knows what trade-offs the parser made.

## Source

- **Site:** `https://developer.tekla.com`
- **Version root:** `https://developer.tekla.com/doc/tekla-tedds/2026/64124`
- **Extraction date:** 2026-05-19
- **Source kind:** `scrape` (HttpClient + AngleSharp)

## Reproduction

```bash
# From repo root — run the sidecar via stdin envelope.
echo '{"op":"coverage-extract","args":{"vendor":"tedds","version":"26.0","out_path":"cli-sidecar/Ingest/Output/tedds-26.0.ir.json"}}' \
  | cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe

# Regenerate the agent from the IR:
echo '{"op":"coverage-generate","args":{"ir_path":"cli-sidecar/Ingest/Output/tedds-26.0.ir.json","out_dir":"20-agents/aeco/engineering/tekla-tedds-26","agent_id":"tekla-tedds-26.0","vendor":"trimble","vertical":"engineering"}}' \
  | cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe
```

## Naming caveat

See [tekla-tedds-25/EXTRACTION-NOTES.md](../tekla-tedds-25/EXTRACTION-NOTES.md). The agent is published as `tekla-tedds-26` to avoid collision with the existing `tsd-26` slug (which is for Tekla Structural Designer, a different Trimble product).

## Crawl strategy

Identical to the tekla-tedds-25 extraction — only the root URL changes (2026 uses `/doc/tekla-tedds/2026/64124` rather than 2025's `/doc/tekla-tedds/2025/41820`). See [tekla-tedds-25 EXTRACTION-NOTES.md](../tekla-tedds-25/EXTRACTION-NOTES.md) for the full crawl strategy, DocFX selector tables, and per-member parsing detail.

## Version delta vs 2025

The TSD API surface is **identical** between 2025 and 2026:

- 33 types, 77 methods, 71 properties, 0 events — same as 2025.
- 3 namespaces — same as 2025 (`Tekla.Structural.ExpressoAddIn`, `Tekla.Structural.InteropAssemblies.Tedds`, `Tekla.Structural.InteropAssemblies.TeddsCalc`).
- All type names, method names, parameter names, and return types match 2025.

The only differences are:

- **Numeric doc-node IDs.** The IDs in the `/doc/tekla-tedds/<ver>/<slug>-<id>` URLs differ between versions (e.g. `aliasattribute-41824` in 2025 vs `aliasattribute-64128` in 2026). These are version-stamped node IDs, not API content changes.
- **Assembly metadata.** The underlying `.dll`s (`Tedds.TeddsIA.dll`, `Tedds.ExprInterop.dll`, etc.) carry different version stamps internally; the doc pages emit the same `<h6><strong>Assembly</strong>: Tedds.X.dll</h6>` text but the binary they refer to is different.
- **Extracted-at timestamp** in `source.extracted_at` (current UTC at extraction time).

We still publish two separate agents (rather than aliasing 26 → 25) because:
- Operators need an honest version pin in `aware.lock`.
- A future patch release could add types without changing the slug, and the agent should reflect what's documented at that specific URL.
- The `aware coverage diff <a> <b>` workflow assumes one IR per documented version.

## Coverage stats

| Metric | Count |
|---|---|
| Types | 33 |
| Methods (incl. ctors) | 77 |
| Properties | 71 |
| Events | 0 |
| Page count | 37 |
| Schema violations | 0 |
| Strict-verify pass rate | 33/33 (100%) |

## Verification

```powershell
# Strict-sample verification (33/33 PASS).
& cli-sidecar/Ingest/Output/verify-types-strict.ps1 \
    -IrPath cli-sidecar/Ingest/Output/tedds-26.0.ir.json -Count 33

# Schema validation (4 catalog files + IR — all pass).
dotnet test cli-sidecar/Ingest/Generator/Tests --filter "Tedds"
```

## Engineering envelope

- **Code revision:** see `git rev-parse HEAD` at extraction time.
- **Sidecar version:** 0.5.1
- **CLI version:** 0.30.0
- **Generator version:** 0.30.0
- **Schema version:** host-coverage v1.
