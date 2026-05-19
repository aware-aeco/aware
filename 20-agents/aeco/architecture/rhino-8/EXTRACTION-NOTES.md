# rhino-8 EXTRACTION NOTES

This agent is generated from a coverage IR scraped from the McNeel rhinocommon-api-docs github
repository (branch `gh-pages` â€” current Rhino 8.31). These notes capture **how** the IR was
produced so the extraction can be reproduced.

## Source

- **Site:** `https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/`
- **Version root:** `https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/R_Project_RhinoCommon.htm`
- **Branch:** `gh-pages` (Rhino 8.31.26126.13430, Last updated: 2026-05-06 â€” the live github.io content for `mcneel.github.io/rhinocommon-api-docs/api/RhinoCommon/`)
- **Extraction date:** _filled in after extraction_
- **Source kind:** `scrape` (HttpClient + AngleSharp)

## Why this URL â€” version availability

See the rationale at `../rhino-7/EXTRACTION-NOTES.md` (same source story). Summary: the
`gh-pages` branch hosts the live Rhino 8 docs (8.31 current). The new SPA at
`developer.rhino3d.com/api/rhinocommon/` only serves V9 and ignores `?version=` queries; the
github raw `gh-pages` path is the only authoritative source for Rhino 8 docs.

## Reproduction

```bash
echo '{"op":"coverage-extract","args":{"vendor":"rhino","version":"8.0","out_path":"cli-sidecar/Ingest/Output/rhino-8.0.ir.json"}}' \
  | cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe

# Or with auto-restart wrapper:
pwsh -File cli-sidecar/Ingest/Output/extract-rhino-with-restart.ps1 -Version 8.0

# Regenerate the agent:
AWARE_SIDECAR=cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe \
  cli/target/release/aware.exe coverage generate rhino 8.0 \
  --from-ir cli-sidecar/Ingest/Output/rhino-8.0.ir.json \
  --vendor mcneel --vertical architecture
```

The extractor is at `cli-sidecar/Ingest/Extractors/Rhino/` (Extractor.cs + PageParser.cs +
MemberPageParser.cs). The sidecar verb is `coverage-extract`.

## Crawl strategy, selectors, parser quirks

Identical to `../rhino-7/EXTRACTION-NOTES.md`. Both versions are Sandcastle-generated and share
the same DOM. The only difference is the base URL.

## IR / catalog counts

- **Vendor-published namespaces (from sidebar or root):** _see Pass 1 log_
- **Types in IR:** 1399
- **Types in generated catalog:** 1399 (1:1)
- **Page count (HTTP fetches):** 1438

## Member-page enrichment results

| field | filled / total | % |
|-------|---------------:|--:|
| Methods with real signature | 16907 / 16907 | 100% |
| Constructors with real signature | 802 / 802 | 100% |
| Properties with non-`object` type | 8904 / 8936 | 99.6% |
| Events with non-`EventHandler` delegate | 158 / 158 | 100% |
| **Total members enriched** | **26771 / 26803** | **99.9%** |

Extracted at: 2026-05-19T02:49:47.7517985Z

## Concurrency tuning

Identical to rhino-7. github raw serves both branches from the same CDN.

## Extraction errors log

Path: `cli-sidecar/Ingest/Output/rhino-8.0-errors.log` (lazily created on first error).

## Known limitations / follow-up work

Same shape as `../rhino-7/EXTRACTION-NOTES.md`.
