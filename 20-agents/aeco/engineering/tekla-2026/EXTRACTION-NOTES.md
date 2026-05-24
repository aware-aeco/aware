# tekla-2026 EXTRACTION NOTES

This agent is generated from a coverage IR scraped from `developer.tekla.com`. These notes capture **how** the IR was produced so the extraction can be reproduced — and so the next reviewer knows what trade-offs the parser made.

## Source

- **Site:** `https://developer.tekla.com`
- **Version root:** `https://developer.tekla.com/doc/tekla-structures/2026/tekla-structures-64304`
- **Extraction date:** 2026-05-19 (re-extracted from scratch after the codex-coverage FAIL feedback identified ctor-name + generic-bracket + returns-key + Property-Value/Return-Value parser defects)
- **Source kind:** `scrape` (HttpClient + AngleSharp; see "Why not Playwright" below)

## Reproduction

```bash
# From repo root — run the sidecar via stdin envelope.
echo '{"op":"coverage-extract","args":{"vendor":"tekla","version":"2026.0","out_path":"cli-sidecar/Ingest/Output/tekla-2026.0.ir.json"}}' \
  | cli-sidecar/bin/Release/net10.0/win-x64/publish/aware-sidecar.exe

# Or use the restart-on-stall wrapper (recommended — Tekla's CDN throttles after ~3k member fetches per session):
pwsh -File cli-sidecar/Ingest/Output/extract-with-restart.ps1 -Version 2026.0

# Regenerate the agent from the IR:
AWARE_SIDECAR=cli-sidecar/bin/Release/net10.0/win-x64/publish/aware-sidecar.exe \
  cli/target/release/aware.exe coverage generate tekla 2026.0 \
  --from-ir cli-sidecar/Ingest/Output/tekla-2026.0.ir.json \
  --vendor trimble --vertical engineering
```

The extractor is at `cli-sidecar/Ingest/Extractors/Tekla/` (Extractor.cs + PageParser.cs + MemberPageParser.cs). The sidecar verb is `coverage-extract` (registered in `cli-sidecar/Program.cs`).

## Crawl strategy — two-pass

**Pass 1: Type-page crawl** (one HTTP fetch per type).

1. Start from the version-pinned root namespace URL (`/doc/tekla-structures/<year>/tekla-structures-<id>`).
2. The root page sidebar `<section id="block-trimble2017-devcenter-apipackagenavigation">` contains a `<ul>` tree with every `Tekla.Structures.*` namespace link. We extract anchors whose text ends in ` Namespace`.
3. For each namespace landing page, the type tables `<table class="members" id="classList|interfaceList|structureList|enumerationList|delegateList">` carry every type's link in the second `<td>` of each row.
4. Each type page is fetched in turn (with a fan-out semaphore of 4 to keep load polite). `PageParser.Parse(html, url)` returns a `TypeInfo` with: name, namespace, kind, summary, remarks, base (most-direct ancestor from the inheritance hierarchy), doc_url, plus member rows (name + one-line summary) under constructors / methods / properties / events / enum_values.
5. A second sequential retry pass recovers any URLs that returned a transient `OperationCanceledException` during the concurrent first pass.

**Pass 2: Per-member enrichment** (one HTTP fetch per member page).

6. After the type-page pass, the extractor queues every member URL captured under each type. Tekla's docs have one `/topic/en/<id>/<guid>` page per member where the full signature, parameter types, return type, exception list, and remarks live.
7. The enrichment loop fetches member pages with a fan-out semaphore of 8, snapshotting the IR every 200 enrichments so the run is resumable if interrupted. Idempotency: members already populated (signature != placeholder) are skipped on restart.
8. `MemberPageParser.Parse(html, url, ...)` fills the placeholder fields with real values: method `signature`, `params[].type/doc/optional`, `returns.type/doc`, `throws[]`, property `type`, event `delegate`/`signature`/`handler_thread`, plus method/property `remarks`.

The restart-on-stall wrapper (`cli-sidecar/Ingest/Output/extract-with-restart.ps1`) babysits the sidecar process and restarts it if the log stalls for >240s without a new progress line — Tekla's CDN throttles aggressively after a few thousand requests per IP, and snapshot-based resumability lets a fresh process pick up where the previous one left off.

## CSS selectors used (type pages)

| What | Selector |
|------|----------|
| Page body root | `div.topicContent#TopicContent` |
| Type title (h1 ending in " Class", "Enumeration", ...) | `h1` inside `td.titleColumn` |
| Direct-child type summary | `div.summary` direct child of `div.topicContent` |
| Namespace anchor | `strong` containing "Namespace:" then next `a` sibling |
| Inheritance hierarchy | `span.collapsibleRegionTitle` containing "Inheritance Hierarchy" → next sibling `.collapsibleSection` → `span.nolink` (last is base) |
| Constructor table | `table.members#constructorList` |
| Method table | `table.members#methodList` |
| Property table | `table.members#propertyList` |
| Event table | `table.members#eventList` |
| Enum value table | `table.members#enumMemberList` |
| Namespace-page class/enum/interface/etc. tables | `table.members#classList`, `#interfaceList`, `#structureList`, `#enumerationList`, `#delegateList` |
| Member row name | `td:nth-child(2) a` (under CleanInlineText) |
| Member row summary | `td:nth-child(3) div.summary` (under CleanInlineText) |
| Member-page link (used by Pass 2) | `td:nth-child(2) a[href^="/topic/"]` |

## CSS selectors used (member pages, Pass 2)

| What | Selector |
|------|----------|
| Member title | `h1` inside `td.titleColumn` |
| Member kind word | trailing word in h1 ("Method", "Property", "Event", "Constructor") |
| C# syntax block | `div.codeSnippetContainerCode[id$="_code_Div1"] > pre` |
| Parameters section | `h4.subHeading` text="Parameters" → following `dl.parameterList` (one `<dt>` per param, `<dd>` for doc) |
| Return value section | `h4.subHeading` text="Return Value" → following `<div class="paramDescription">` |
| Exceptions table | `table.exception` with `<tr>` rows of `(type, when)` |
| Remarks section | `span.collapsibleRegionTitle` text="Remarks" → next `.collapsibleSection` |

## Extraction errors log

If an extraction run produces errors (HTTP failure, parse failure, retry-exhausted), they are appended one per line to a dedicated log file alongside the IR output:

```
cli-sidecar/Ingest/Output/tekla-2026.0-errors.log
```

Format (tab-separated, one record per line):

```
<utc-iso8601>\t<phase>\t<url>\t<exception-message>
```

Phase values currently emitted:

| phase | meaning |
|-------|---------|
| `namespace-fetch` | A namespace landing page fetch failed during Phase 2 type-URL harvest. |
| `type-page-fetch` | A type page fetch failed during Phase 3 concurrent crawl. |
| `type-page-retry-final` | After two retries in the sequential post-pass, the type page still wouldn't load. |
| `member-page-fetch` | A per-member page fetch (with `FetchWithFreshScraper` + 1 retry) finally failed. |
| `member-page-parse` | The page loaded but `MemberPageParser` threw an unexpected exception. |

The file is **created lazily on the first error**, never pre-created empty. Resumed runs append rather than clobber so the history of throttle events across restarts is preserved. The current 2026 run produced **0 errors** end-to-end (single uninterrupted process), so no log file exists on disk today. A future re-run that hits a fresh CDN throttle will lazily create the file with one line per failure.

## Concurrency tuning

The Tekla extractor uses three tunable constants (`Extractor.cs`):

| constant | Tekla value | rationale |
|----------|-------------|-----------|
| `TypePageConcurrency` | 4 | Single shared HttpClient pool; staying conservative on the first ~1320 type pages keeps us comfortably below the throttle threshold. Empirically: 4 in-flight requests completed the type-page pass in ~3 minutes with 0 retries. |
| `MemberPageConcurrency` | 8 | Per-fetch fresh `HttpScraper` instances (one HttpClient each). 8 is the empirical sweet spot — 4 under-utilises the per-IP request budget, 16 trips the per-CDN throttle at ~1k fetches. At 8 we completed 12,417 fetches in a single uninterrupted run on the tekla-2026 pass. |
| `SnapshotEvery` | 200 | A 5-6 MB IR write every 200 enrichments costs ~50-100 ms (well under 1% wall-clock overhead at 8-way concurrency). 200 also gives a generous resumability window: a `kill -9` mid-run loses at most ~40 s of work. |

The `extract-with-restart.ps1` wrapper exposes `StallTimeoutSec` (default 240s) which is sized for the Tekla CDN's typical pause-after-throttle window. See the wrapper's header comment for vendor-specific tuning guidance.

**Why these numbers — for B2-B14 implementers:** Tekla's CDN throttles aggressively after ~3k fetches per IP. Concurrency=8 with per-fetch fresh HttpScraper instances empirically stays below the throttle threshold longer than concurrency=16; per-fetch dispose-on-completion also avoids the long-lived-pool progressive-throttling pattern we observed under shared-scraper testing (100 fetches then stall). For B2-B14 vendors:

1. Measure throughput vs throttle-time on a first run (the dedicated errors log makes this easy — each throttle event lands as a `*-fetch` row with a timestamp).
2. If you see >10 throttle events per 1000 fetches, halve `MemberPageConcurrency` and re-run.
3. If you see <1 throttle event per 5000 fetches AND fetches are not CPU-bound on the parser, double `MemberPageConcurrency`.
4. Document the chosen value + the measurement that informed it in the vendor's EXTRACTION-NOTES.md.

## Parser quirks

- **Language-specific delimiters via inline `<script>`**: Tekla's docs use a pattern like `<span class="nolink">System<span id="LSTAB654CE3_0"></span><script>AddLanguageSpecificTextSet("LSTAB654CE3_0?cs=.|vb=.|cpp=::|nu=.|fs=.");</script>Object</span>`. In a real browser the script substitutes the C# delimiter (`.`) into the empty `<span id="LST...">`. Without JS we synthesize the dot ourselves: when traversing inline text, any element with `id` starting with `LST` is replaced with `.`. This is in `CleanInlineText` / `CleanNoLinkText` in `PageParser.cs`. Without this, every base type would be `SystemObject` (missing dot) and method names containing generics would carry the raw `AddLanguageSpecificTextSet("...")` script text — both of which downstream consumers would have to deal with separately.

- **URL inconsistency on namespace pages**: Most types link to canonical `/doc/tekla-structures/2026/<slug>-<id>` URLs from the sidebar tree, but the namespace landing pages still link to legacy `/topic/en/18/47/<guid>` URLs for many types. Both URLs serve the same content, so the extractor treats them as equivalent — `doc_url` in the IR carries whichever form was on the source page.

- **Empty `<div class="summary">`**: A small number of types (12 in 2026.0) ship with no description authored on the vendor side. The IR schema (and the IRReader validator) require a non-empty summary, so we substitute a precise placeholder `"(No description provided in vendor docs for <ns>.<name>.)"`. The placeholder is intentionally honest — it surfaces the omission rather than fabricating prose.

- **`object`-typed properties are legitimate, not placeholders.** `.NET` collection types implement `ICollection.SyncRoot { get; }` which returns `System.Object` per the BCL contract. Tekla's vendor docs faithfully render `Type: Object` for these properties. The IR's `type="object"` for SyncRoot (and a small number of similar `ICollection`/`IEnumerable` implementations) is **correct**, not a parser placeholder.

- **Why not Playwright**: Phase A shipped a Playwright-based `Scraper` for vendor sites that hydrate content client-side. For the Tekla docs site every relevant DOM tree is pre-rendered in the initial HTML response — Playwright is overkill and, crucially, **Playwright's .NET transport uses reflection-based JSON serialization internally**, which `<PublishAot>true</PublishAot>` disables. `Playwright.CreateAsync()` throws `InvalidOperationException: Reflection-based serialization has been disabled` on the NativeAOT-published sidecar. We added `cli-sidecar/Ingest/Extractors/Common/HttpScraper.cs` as a pure-HttpClient alternative and use it for Tekla. **Sites that genuinely need JS hydration will need a separate AOT story for Playwright (open question for B4–B14).** This contradicts the Phase A handoff lesson #4 — that lesson was wrong.

## Skipped pages

Counted as `skip` in the extractor log: pages whose `<h1>` ends in a non-type word ("Methods", "Properties", "Constructor", "Namespace", etc.). These are reachable from type tables because a method-overload landing page uses the same `<a>` cell shape. **The crawler intentionally does not follow these as types** — they would be duplicate entries (the methods are already enumerated in their parent type's method table).

The 2026 extraction produced **0 skipped pages**.

## Hard failures

None. All 1320 type URLs and all 12,417 member-page URLs were fetched and parsed successfully on the re-extraction. The auto-restart wrapper kicked in 0 times (the 2026 extraction completed in a single uninterrupted sidecar process, running in parallel with 2025 against the same CDN).

## IR / catalog counts

- **Vendor-published namespaces (from sidebar):** 29
- **Unique type URLs discovered:** 1320
- **Types in IR:** 1320
- **Types in generated catalog:** 1320
- **Skipped (non-type) pages:** 0
- **Hard failures (types):** 0

## Member-page enrichment results

| field | filled / total | % |
|-------|---------------:|--:|
| Methods + constructors with real signature | 8079 / 8079 | 100% |
| Properties with non-`object` type | 4166 / 4178 | 99.7% |
| Events with non-`EventHandler` delegate | 160 / 160 | 100% |
| **Total members enriched** | **12405 / 12417** | **99.9%** |

The 12 properties remaining at `type="object"` are `SyncRoot` / `ICollection.SyncRoot` properties that **legitimately** return `System.Object` per the .NET BCL contract — they are not parser failures. A spot check confirms each such property's vendor page does say `Type: Object`. The remaining 162 properties that were `object` in the previous extraction now carry their real types thanks to the Property Value/Return Value subheading-fallback parser fix shipped alongside this re-extraction.

## Strict 50-type self-verification

A deterministic sample of 50 types was drawn from the IR using a Fisher-Yates shuffle seeded by the SHA-256 hash of `tekla-2026.0.ir.json` (script: `cli-sidecar/Ingest/Output/verify-types-strict.ps1`). Each sample was checked at the **type level** (name + kind word present in HTML, with generic-parameter stripping for types like `CustomObservableCollection<T>`) AND the **member level** (3 random methods, 3 random properties, 1 random event each pass the placeholder check: signature != `name(...)`, property type != `object`, event delegate != bare `EventHandler`).

Result: **50 / 50 PASS** (seed = 925498160, IR sha256 = `b729fb3000157aa4902985c086ea786dc563c7b74ecad092835918e8f31bef57`).

The full output is at `cli-sidecar/Ingest/Output/tekla-2026-strict-verify.txt`.

## Known limitations / follow-up work

1. **Interfaces list is currently populated from the C# syntax-block declaration line** (e.g. `class Foo : BaseClass, IBar, IBaz` → interfaces = `["IBar", "IBaz"]`). This is the Tekla-page-template's only deterministic source for implemented interfaces — Tekla doesn't itemize them in a dedicated section.
2. **Multi-layer inheritance**: types that inherit through multiple layers (`Foo -> AbstractFoo -> BaseFoo -> Object`) record only the most-direct ancestor in `base`. This matches the IR schema (a single string), but the chain information is lost. Acceptable per Phase A schema design.
3. **12 properties without explicit type** (the SyncRoot caveat above) — these are correctly typed as `object` per the .NET BCL contract; no further enrichment possible from vendor docs.
4. **Catalog filename sanitization**: generic type names like `Enum<E>` and `CustomObservableCollection<T>` are rewritten in the catalog FILENAME to be filesystem-safe (`Enum_of_E.json`, `CustomObservableCollection_of_T.json`). The JSON BODY inside the file carries the real type name verbatim. Downstream consumers should read `name` from the JSON, not parse it out of the filename.
