# grasshopper-8 EXTRACTION NOTES

This agent is generated from a coverage IR scraped from the McNeel grasshopper-api-docs github
repository (branch `gh-pages` â€” Grasshopper for Rhino 8.31).

## Source

- **Site:** `https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/`
- **Version root:** `https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/723c01da-9986-4db2-8f53-6f3a7494df75.htm`
- **Branch:** `gh-pages` (Grasshopper for Rhino 8.31 â€” the live developer.rhino3d.com/api/grasshopper/ content)
- **Extraction date:** _filled in after extraction_
- **Source kind:** `scrape` (HttpClient + AngleSharp)

## Reproduction

```bash
echo '{"op":"coverage-extract","args":{"vendor":"grasshopper","version":"8.0","out_path":"cli-sidecar/Ingest/Output/grasshopper-8.0.ir.json"}}' \
  | cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe

# Or:
pwsh -File cli-sidecar/Ingest/Output/extract-grasshopper-with-restart.ps1 -Version 8.0

# Regenerate the agent:
AWARE_SIDECAR=cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe \
  cli/target/release/aware.exe coverage generate grasshopper 8.0 \
  --from-ir cli-sidecar/Ingest/Output/grasshopper-8.0.ir.json \
  --vendor mcneel --vertical architecture
```

## Crawl strategy, selectors, parser quirks

Identical to `../grasshopper-7/EXTRACTION-NOTES.md`. The branch-7 and branch-gh-pages Grasshopper
docs share the same Sandcastle template and TOC-only namespace layout.

## IR / catalog counts

- **Vendor-published namespaces (from sidebar or root):** _see Pass 1 log_
- **Types in IR:** 825
- **Types in generated catalog:** 825 (1:1)
- **Page count (HTTP fetches):** 862

## Member-page enrichment results

| field | filled / total | % |
|-------|---------------:|--:|
| Methods with real signature | 11266 / 11266 | 100% |
| Constructors with real signature | 860 / 860 | 100% |
| Properties with non-`object` type | 6622 / 6631 | 99.9% |
| Events with non-`EventHandler` delegate | 511 / 511 | 100% |
| **Total members enriched** | **19259 / 19268** | **100%** |

Extracted at: 2026-05-19T02:39:23.8754914Z

## Concurrency tuning, errors log, limitations

See `../grasshopper-7/EXTRACTION-NOTES.md`.
