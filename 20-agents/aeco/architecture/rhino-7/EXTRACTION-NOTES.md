# rhino-7 EXTRACTION NOTES

This agent is generated from a coverage IR scraped from the McNeel rhinocommon-api-docs github
repository (branch `7` â€” Rhino 7.19, the LTS final). These notes capture **how** the IR was
produced so the extraction can be reproduced â€” and so the next reviewer knows what trade-offs
the parser made.

## Source

- **Site:** `https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/`
- **Version root:** `https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/R_Project_RhinoCommon.htm`
- **Branch:** `7` (Rhino 7.19.22165.13000, Last updated: 6/15/2022 â€” the final Rhino 7 release before Rhino 8)
- **Extraction date:** _filled in after extraction_
- **Source kind:** `scrape` (HttpClient + AngleSharp)

## Why this URL â€” version availability

McNeel publishes the RhinoCommon API docs in two places:

| Where | What | Versions |
|---|---|---|
| `mcneel.github.io/rhinocommon-api-docs/` (github pages) | Rendered Sandcastle HTML | Current 8.x only (via the `gh-pages` branch) |
| `developer.rhino3d.com/api/rhinocommon/` (SPA) | Modern Quasar-based viewer | V9 only (`?version=` query is ignored â€” always serves V9) |
| `github.com/mcneel/rhinocommon-api-docs` branches `5`/`6`/`7`/`8` | Raw Sandcastle HTML per version | One branch per major version (3, 4, 5, 6, 7, 8) |

The branch-per-version layout is the only place where Rhino 7 docs are still served. The
SPA at `developer.rhino3d.com` does NOT honor `?version=7` â€” it always serves V9. The
`raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/...` URLs are stable, version-
specific, and serve the same Sandcastle HTML the github.io site served when Rhino 7 was current.

The `gh-pages` branch is the live github.io content; today it's Rhino 8.31. We use it for the
sibling rhino-8 agent, and the dedicated `7` branch for rhino-7.

## Reproduction

```bash
# From repo root â€” run the sidecar via stdin envelope.
echo '{"op":"coverage-extract","args":{"vendor":"rhino","version":"7.0","out_path":"cli-sidecar/Ingest/Output/rhino-7.0.ir.json"}}' \
  | cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe

# Or use the restart-on-stall wrapper (recommended if you hit github raw throttling):
pwsh -File cli-sidecar/Ingest/Output/extract-rhino-with-restart.ps1 -Version 7.0

# Regenerate the agent from the IR:
AWARE_SIDECAR=cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe \
  cli/target/release/aware.exe coverage generate rhino 7.0 \
  --from-ir cli-sidecar/Ingest/Output/rhino-7.0.ir.json \
  --vendor mcneel --vertical architecture
```

The extractor is at `cli-sidecar/Ingest/Extractors/Rhino/` (Extractor.cs + PageParser.cs +
MemberPageParser.cs). The sidecar verb is `coverage-extract` (registered in
`cli-sidecar/Program.cs`).

## Crawl strategy â€” two-pass

Identical shape to the Tekla extractor (see `tekla-2026/EXTRACTION-NOTES.md`):

**Pass 1: Type-page crawl** (one HTTP fetch per type).

1. Fetch the version-pinned root URL `R_Project_RhinoCommon.htm`.
2. Parse `<table class="members" id="namespaceList">` for namespace URLs.
3. For each namespace landing page, walk the type tables `<table class="members"
   id="classList|interfaceList|structureList|enumerationList|delegateList">` to collect every
   type URL.
4. Each type page is fetched with a fan-out semaphore of 4 (RhinoCommon's CDN â€” GitHub raw â€” is
   more lenient than Tekla's, so 4 is conservative). `PageParser.Parse(html, url, baseUrl)`
   returns a `TypeInfo` with: name, namespace, kind, summary, remarks, base, doc_url, plus
   member rows under constructors / methods / properties / events / enum_values.
5. Sequential retry pass for transient failures.

**Pass 2: Per-member enrichment** (one HTTP fetch per member page).

6. The extractor queues every member URL captured under each type. RhinoCommon's docs have one
   `/(M_|P_|E_|F_|Overload_)<Namespace>_<Type>_<Member>.htm` page per member.
7. The enrichment loop fetches with fan-out 8, snapshotting the IR every 200 enrichments.
8. `MemberPageParser.ParseMethod/ParseProperty/ParseEvent` fills the placeholders with real
   signature / params / returns / throws / property type / event delegate.

## Differences from Tekla â€” RhinoCommon Sandcastle quirks

The Sandcastle template family is the same, but RhinoCommon's docs differ from Tekla's in
several ways that the extractor handles:

| Quirk | Handling |
|---|---|
| Title kind is **"Struct"** (singular) on some structs and **"Structure"** on others | `PageParser.ParseTitle` accepts both " Struct" and " Structure" â†’ IR `kind="struct"` |
| Type pages expose an extra `table#operatorList` for operator overloads | `PageParser` extracts operatorList rows and merges them into `methods[]` (operators are static methods semantically) |
| Type pages expose an extra `table#fieldList` for public fields | Folded into `properties[]` with `setter=false` (fields are read-only at the public API surface) |
| Member detail pages use h1 endings " Operator" and " Field" beyond Tekla's " Method"/" Constructor"/" Property"/" Event" | `MemberPageParser.ParseMethod` accepts " Operator"; `ParseProperty` accepts " Field" and emits `setter=false` |
| Property detail pages sometimes use "Field Value" heading | `MemberPageParser.ParseProperty` falls back from "Property Value" to "Return Value" to "Field Value" |
| LST language-specific text placeholders sometimes have only `cpp=::` and `nu=.` keys (no `cs=`) on operator member pages | `MemberPageParser.ExtractCsVariant` falls back to `nu=` when `cs=` is absent. Without this fallback type names in operator pages would collapse: `Point3d.Addition` â†’ `Point3dAddition`. The Tekla parser defaulted to "" for missing `cs=`, which is correct for the `cpp=%`-only out-param case but wrong for the separator case |
| Namespace anchors live in `table.members#namespaceList` on the root page | Handled by `Extractor.ExtractNamespaceUrls` |
| Doc hrefs are relative-walked (`../html/N_...htm`) on the root page and bare (`N_...htm`) on namespace pages | `AbsoluteUrl` uses `Uri` math to handle both forms cleanly |

## CSS selectors used

Identical to Tekla's verified set in `tekla-2026/EXTRACTION-NOTES.md`. The RhinoCommon extension
points (operator/field tables, " Operator"/" Field" h1 endings) are noted above.

## Concurrency tuning

| constant | value | rationale |
|---|---|---|
| `TypePageConcurrency` | 2 | github raw rate-limits IPs aggressively for sustained traffic. At 4+ concurrent we observed 429 storms after ~5k fetches. 2 is empirically below the trigger point for single-vendor runs. |
| `MemberPageConcurrency` | 4 | Tekla's CDN tolerated 8; github raw's secondary rate limit is more restrictive â€” we measured a 429 storm at 8-way concurrency. 4 keeps us below the trigger point. Fresh `HttpScraper` per fetch avoids long-lived-pool degradation. |
| `SnapshotEvery` | 200 | Standard â€” ~1% wall-clock overhead, ~40s resumability window |
| `FetchWithFreshScraper` retries | 3 with backoff 1.5s/4s/10s | Github raw's 429s clear within 1-10 seconds. The exponential backoff rides out transient throttle events; persistent 429s exhaust retries and land in the errors log for a follow-up resume pass. |

The `extract-rhino-with-restart.ps1` wrapper exposes `StallTimeoutSec` (default 240s) for
auto-restart on CDN stall.

**Two-pass extraction is normal.** github raw's rate limit causes a first-pass error rate of
1-15% depending on parallel pressure. After the first pass completes (the wrapper sees the
`done:` line), a quick second invocation of the wrapper resumes from snapshot, re-queues all
placeholder members, and completes them with 0 errors (the throttle has cleared by then).
This is the same flow Tekla uses (where the wrapper's restart-on-stall behavior also drives
re-passes). For Rhino/Grasshopper, the second-pass invocation can be manual when running
multiple extractions in parallel â€” see `resume-extract.ps1`.

## Extraction errors log

Errors land at `cli-sidecar/Ingest/Output/rhino-7.0-errors.log` (same shape as Tekla's). Created
lazily on first error; missing file means no errors occurred.

## IR / catalog counts

- **Vendor-published namespaces (from sidebar or root):** _see Pass 1 log_
- **Types in IR:** 1154
- **Types in generated catalog:** 1154 (1:1)
- **Page count (HTTP fetches):** 1192

## Member-page enrichment results

| field | filled / total | % |
|-------|---------------:|--:|
| Methods with real signature | 15073 / 15073 | 100% |
| Constructors with real signature | 673 / 673 | 100% |
| Properties with non-`object` type | 7725 / 7757 | 99.6% |
| Events with non-`EventHandler` delegate | 151 / 151 | 100% |
| **Total members enriched** | **23622 / 23654** | **99.9%** |

Extracted at: 2026-05-19T02:45:48.9224911Z

## Known limitations / follow-up work

1. Sandcastle types are the curated PUBLIC API surface. Internal types not visible in the docs
   are not in the IR.
2. Operator overloads are folded into `methods[]` per the IR schema (no dedicated `operators[]`
   slot). Their row name carries the operator name + arg signature so they remain disambiguated.
3. Fields are folded into `properties[]` with `setter=false`. The original "field vs property"
   distinction is lost in the IR but the catalog summary text preserves the semantic intent (the
   vendor's prose says "field"/"constant"/"property" as appropriate).
4. Generic type filenames use `_of_T` for `<T>` in the catalog (same convention as Tekla).
