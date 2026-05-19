# grasshopper-7 EXTRACTION NOTES

This agent is generated from a coverage IR scraped from the McNeel grasshopper-api-docs github
repository (branch `7` â€” Grasshopper for Rhino 7). These notes capture **how** the IR was
produced so the extraction can be reproduced.

## Source

- **Site:** `https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/`
- **Version root:** `https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/723c01da-9986-4db2-8f53-6f3a7494df75.htm`
- **Branch:** `7` (Grasshopper 1.0 for Rhino 7 â€” the SDK frozen against the Rhino 7 final)
- **Extraction date:** _filled in after extraction_
- **Source kind:** `scrape` (HttpClient + AngleSharp)

## Why this URL â€” version availability

The Grasshopper SDK docs follow the same pattern as RhinoCommon:

| Where | What | Versions |
|---|---|---|
| `developer.rhino3d.com/api/grasshopper/` | Sandcastle docs (current) | Live Rhino 8.31 only |
| `github.com/mcneel/grasshopper-api-docs` branches `5`/`6`/`7`/`8` | Raw Sandcastle HTML | One branch per major version |

The branch-per-version layout is the only source for Grasshopper 7 docs.

Path note: Grasshopper's docs live under `api/grasshopper/html/` (not bare `html/` like
RhinoCommon).

## Reproduction

```bash
echo '{"op":"coverage-extract","args":{"vendor":"grasshopper","version":"7.0","out_path":"cli-sidecar/Ingest/Output/grasshopper-7.0.ir.json"}}' \
  | cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe

# Or with auto-restart wrapper:
pwsh -File cli-sidecar/Ingest/Output/extract-grasshopper-with-restart.ps1 -Version 7.0

# Regenerate the agent:
AWARE_SIDECAR=cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe \
  cli/target/release/aware.exe coverage generate grasshopper 7.0 \
  --from-ir cli-sidecar/Ingest/Output/grasshopper-7.0.ir.json \
  --vendor mcneel --vertical architecture
```

The extractor is at `cli-sidecar/Ingest/Extractors/Grasshopper/` â€” it delegates all parsing to
the Rhino PageParser/MemberPageParser (Sandcastle DOM is byte-identical). The orchestrator
overrides only:
- Vendor URL bases (different repo, different path under the branch)
- Root-page namespace discovery (Grasshopper's root only lists namespaces via the TOC sidebar,
  not via a `table#namespaceList`)
- Host name "grasshopper" + errors-log filename

## Crawl strategy â€” two-pass

Same as Rhino's (which is the same as Tekla's). See `../rhino-7/EXTRACTION-NOTES.md` for the
detailed pipeline description.

## Differences from Rhino â€” Grasshopper-specific quirks

| Quirk | Handling |
|---|---|
| The Grasshopper SDK root page has NO `table.members#namespaceList` â€” namespaces are only listed in the LEFT-SIDEBAR TOC (`div#tocNav`) | `Grasshopper.Extractor.ExtractNamespaceUrls` reads namespace anchors from `div#tocNav`, filtering by filename prefix `N_` |
| Root-page filename is a GUID (`723c01da-...htm`) not `R_Project_*.htm` | Hardcoded per-version in `Extractor.KnownVersions` |
| Path layout `api/grasshopper/html/` (not bare `html/`) | Hardcoded per-version in `Extractor.KnownVersions` |

All other DOM shapes â€” type tables, member tables, h1 titles, syntax blocks, parameters lists â€”
match Rhino's verbatim and the Rhino parsers handle them unchanged.

## CSS selectors used

See `../rhino-7/EXTRACTION-NOTES.md`. The only additional Grasshopper-specific selector is
`div#tocNav` for namespace discovery.

## Concurrency tuning

Identical to Rhino (`TypePageConcurrency=4`, `MemberPageConcurrency=8`, `SnapshotEvery=200`).
github raw serves Grasshopper docs from the same CDN as Rhino docs.

## Extraction errors log

Path: `cli-sidecar/Ingest/Output/grasshopper-7.0-errors.log`.

## IR / catalog counts

- **Vendor-published namespaces (from sidebar or root):** _see Pass 1 log_
- **Types in IR:** 650
- **Types in generated catalog:** 650 (1:1)
- **Page count (HTTP fetches):** 674

## Member-page enrichment results

| field | filled / total | % |
|-------|---------------:|--:|
| Methods with real signature | 4847 / 4847 | 100% |
| Constructors with real signature | 544 / 544 | 100% |
| Properties with non-`object` type | 2969 / 2979 | 99.7% |
| Events with non-`EventHandler` delegate | 170 / 170 | 100% |
| **Total members enriched** | **8530 / 8540** | **99.9%** |

Extracted at: 2026-05-19T02:29:37.3625197Z

## Member-page enrichment results

| field | filled / total | % |
|-------|---------------:|--:|
| Methods with real signature | 4847 / 4847 | 100% |
| Constructors with real signature | 544 / 544 | 100% |
| Properties with non-`object` type | 2969 / 2979 | 99.7% |
| Events with non-`EventHandler` delegate | 170 / 170 | 100% |
| **Total members enriched** | **8530 / 8540** | **99.9%** |

Extracted at: 2026-05-19T02:29:37.3625197Z

## Member-page enrichment results

| field | filled / total | % |
|-------|---------------:|--:|
| Methods with real signature | 4847 / 4847 | 100,0% |
| Constructors with real signature | 544 / 544 | 100,0% |
| Properties with non-`object` type | 2969 / 2979 | 99,7% |
| Events with non-`EventHandler` delegate | 170 / 170 | 100,0% |
| **Total members enriched** | **8530 / 8540** | **99,9%** |

Extracted at: 2026-05-19T02:29:37.3625197Z

## Known limitations / follow-up work

1. Sandcastle types are the curated PUBLIC API surface.
2. Operators folded into `methods[]`, fields into `properties[]` (same pattern as Rhino).
3. The Grasshopper SDK is generally smaller than RhinoCommon (~250-300 types vs ~1300+) â€” the
   extraction completes proportionally faster.
