# tekla-tedds-25 EXTRACTION NOTES

This agent is generated from a coverage IR scraped from `developer.tekla.com`. These notes capture **how** the IR was produced so the extraction can be reproduced — and so the next reviewer knows what trade-offs the parser made.

## Source

- **Site:** `https://developer.tekla.com`
- **Version root:** `https://developer.tekla.com/doc/tekla-tedds/2025/41820`
- **Extraction date:** 2026-05-19
- **Source kind:** `scrape` (HttpClient + AngleSharp)

## Reproduction

```bash
# From repo root — run the sidecar via stdin envelope.
echo '{"op":"coverage-extract","args":{"vendor":"tedds","version":"25.0","out_path":"cli-sidecar/Ingest/Output/tedds-25.0.ir.json"}}' \
  | cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe

# Regenerate the agent from the IR:
echo '{"op":"coverage-generate","args":{"ir_path":"cli-sidecar/Ingest/Output/tedds-25.0.ir.json","out_dir":"20-agents/aeco/engineering/tekla-tedds-25","agent_id":"tekla-tedds-25.0","vendor":"trimble","vertical":"engineering"}}' \
  | cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe
```

## Naming caveat — `tsd-25` already exists

The task brief refers to this vendor as "TSD (Tekla Tedds)". In the AWARE repo the slug `tsd-25` is already in use for **Tekla Structural Designer** (a different Trimble product, generated from the `TeklaStructuralDesigner.RemotingAPI@25.3.0` NuGet package). To avoid the naming collision, this agent is published as **`tekla-tedds-25`** with agent_id `tekla-tedds-25.0`. The sidecar's `coverage-extract` verb accepts both `vendor=tedds` and `vendor=tekla-tedds` so operators can reach it with either spelling.

## Doc-site shape — DocFX, not Sandcastle

Tekla Tedds uses a different doc generator than Tekla Structures:

| Aspect | Tekla Structures (Sandcastle) | Tekla Tedds (DocFX) |
|---|---|---|
| Content wrapper | `<div class="topicContent" id="TopicContent">` | `<article class="content wrap" id="_content">` |
| Title format | `Foo Class` (kind word trails) | `Class Foo` (kind word leads) |
| Member-section header | `<span class="collapsibleRegionTitle">Methods</span>` | `<h3 id="methods">Methods</h3>` |
| Member-table class | `<table class="members" id="methodList">` | `<table class="table table-bordered table-condensed">` |
| Syntax block | `<div id="..._code_Div1" class="codeSnippetContainerCode"><pre>` | `<div class="codewrapper"><pre><code class="language-cs">` |
| Inline placeholders | `<span id="LST.."/>` + `<script>AddLanguageSpecificTextSet(..)</script>` (LST script substitution) | None — plain UTF-8 with entity-decoded `<` / `>` |
| Member-row anchor href | `/topic/en/<lang>/<version>/<guid>` | `/topic/en/<lang>/<version>/<hash>` (302-redirects to `/doc/.../<member>-<id>`) |

Because of these differences, this vendor has its own `cli-sidecar/Ingest/Extractors/Tedds/` parser pair (`PageParser.cs`, `MemberPageParser.cs`) and its own `Extractor.cs`. The Tekla Structures parsers are NOT reused.

## Surface size

- **Types:** 33 (5 classes, 17 interfaces — including the `_IApplicationEvents` COM-style event interface that exposes event handlers as plain methods, 11 enums)
- **Namespaces:** 3
  - `Tekla.Structural.ExpressoAddIn` — 7 types (custom add-in attribute classes + enums)
  - `Tekla.Structural.InteropAssemblies.Tedds` — 15 types (the Tedds Application interop surface)
  - `Tekla.Structural.InteropAssemblies.TeddsCalc` — 11 types (the Tedds Calculator interop surface)
- **Members:** 148 total (0 ctors w/ args + 19 default-ctor-like, 77 methods including ctors, 71 properties, 0 first-class events)

This is roughly **40x smaller** than Tekla Structures (1320 types, 12000+ members). Extraction completes in ~30-90 seconds end-to-end.

## Crawl strategy

Two-pass crawl (mirrors the Tekla Structures pattern):

1. **Phase 1 — namespace discovery (1 fetch).** Fetch the root page `/doc/tekla-tedds/2025/41820`, walk the sidebar tree `#block-trimble2017-devcenter-apipackagenavigation` for top-level `<a>` entries whose anchor text contains a dot (`Tekla.Structural.ExpressoAddIn` etc.) and skip the `API Namespaces` root entry.
2. **Phase 2 — type URL harvest (3 fetches).** For each namespace page, the sidebar tree is auto-expanded for that namespace. Collect every `<a href="/doc/tekla-tedds/<ver>/<slug>-<id>">` whose anchor text is a bare identifier (no dots, no parens) and whose URL doesn't contain `-ctor-` (those are per-overload pages, discovered later via type-page member tables).
3. **Phase 3 — type-page crawl (33 fetches, sequential).** Fetch each type page; run `PageParser.Parse()` to produce a `(TypeInfo, MemberLinks)` pair. Snapshot the partial IR.
4. **Phase 4 — per-member enrichment (148 fetches, sequential).** For each member URL (from the type pages' member tables), fetch the member page; run `MemberPageParser.ParseMethod|Property|Event` to fill real signature + param types + return type. Snapshot the IR every 100 enrichments.

### Concurrency tuning

Sequential — no concurrency. The total fetch budget (~185 pages per version) is small enough that even 1 fetch/sec finishes in 3 minutes. Tekla's CDN serves both Tekla Structures (huge) and Tekla Tedds (tiny) on the same domain; the CDN's per-IP throttle would only matter if we ran TSD extraction in parallel with a Tekla Structures extraction, which the orchestrator does not do.

### Snapshot/resume

Identical to the Tekla extractor: every 100 enrichments the IR is rewritten to disk via a `.partial`-then-rename pattern. A killed run can be resumed by re-running the same `coverage-extract` invocation — members whose IR fields are still in placeholder shape (signature = `Name(...)`, returns = null, throws = []) get re-enqueued; everything else is preserved verbatim.

## Per-member page format (DocFX)

Each member page has a content article with this shape:

```html
<article class="content wrap" id="_content" data-uid="Tekla.Structural.X.Y.Member*">
  <h1 ...>Method Member</h1>          <!-- title prefixed with kind keyword -->
  <div class="markdown level0 summary"></div>
  <div class="markdown level0 conceptual"></div>

  <h4 data-uid="...#Member(args)">Member(args)</h4>     <!-- one per overload -->
  <div class="markdown level1 summary"><p>summary</p></div>
  <h5 class="declaration">Declaration</h5>
  <div class="codewrapper"><pre><code class="language-cs">FULL C# SIGNATURE</code></pre></div>
  <h5 class="parameters">Parameters</h5>
  <table>...3 cols Type|Name|Description...</table>
  <h5 class="returns">Returns</h5>                     <!-- absent on void -->
  <table>...2 cols Type|Description...</table>
  <h5 class="exceptions">Exceptions</h5>               <!-- when documented -->
  <h5 id="..._remarks">Remarks</h5>
  <div class="markdown level1 remarks"><p>...</p></div>

  <!-- repeats with the next overload's h4 -->
</article>
```

Tekla Tedds constructor pages bundle ALL overloads on a single page (`<h1>Constructor AliasAttribute</h1>` followed by two `<h4>AliasAttribute(string)</h4>` + `<h4>AliasAttribute(string, bool)</h4>` blocks). The parser uses the type-page row text (`AliasAttribute(string)` etc.) as the disambiguating hint to pick the right overload's section.

## Per-property page

Property pages encode read-only vs read-write in the C# syntax:

- `ITeddsDocument ActiveDocument { get; }` → getter=true, setter=false
- `bool Visible { get; set; }` → getter=true, setter=true

The property type comes from the `<h5 class="propertyValue">Property Value</h5>` table's first row's first cell. A fallback to `<h5 class="returns">` exists in case any property page uses the wrong heading (none observed in TSD-25 but harmless to keep parity with the Tekla Structures fallback path).

## Enum-value tables

TSD enum pages have a 3-column `Fields` table: `Name | Value | Description`. The first cell holds the bare field name (`Aborted`); the second cell holds `Aborted = -1` or just `Aborted` (no explicit numeric value); the third is the doc string. The parser extracts the integer when present and falls back to a string-typed enum value when not.

## Verification

```powershell
# Strict-sample verification (33/33 PASS).
& cli-sidecar/Ingest/Output/verify-types-strict.ps1 \
    -IrPath cli-sidecar/Ingest/Output/tedds-25.0.ir.json -Count 33

# Schema validation (8 catalog files + IR — all pass).
dotnet test cli-sidecar/Ingest/Generator/Tests --filter "Tedds"
```

## Coverage stats

| Metric | Count |
|---|---|
| Types | 33 |
| Methods (incl. ctors) | 77 |
| Properties | 71 |
| Events | 0 (TSD uses COM-style event interfaces, modelled as methods) |
| Page count | 37 |
| Schema violations | 0 |
| Strict-verify pass rate | 33/33 (100%) |

## Vendor-specific quirks (for future maintainers)

1. **DocFX, not Sandcastle.** Tekla Tedds and Tekla Structures share a domain (`developer.tekla.com`) but use entirely different doc generators. Don't try to reuse the Tekla Structures `PageParser.cs` — the markup is incompatible.
2. **COM event interface.** `_IApplicationEvents` (note the leading underscore — this is a real type name and must round-trip into the IR with the underscore intact) exposes "events" as plain `void` methods that the host invokes when something happens. We catalog it as an interface with methods rather than as a type with `events: [...]`.
3. **Sidebar tree is the source of truth for type URLs.** The namespace body uses `<h3 id="classes"><h4><a href="/topic/en/..."></a></h4>` xref hash URLs (content-hash redirects) for type links. The canonical `/doc/tekla-tedds/<ver>/<slug>-<id>` URLs are only available from the sidebar tree (`#block-trimble2017-devcenter-apipackagenavigation`).
4. **Per-overload bundling on ctor pages.** Constructor pages aggregate every overload onto one page with consecutive `<h4>` blocks. The parser must accept a `memberRowText` hint to disambiguate.
5. **Stable across major versions.** TSD-26's surface is IDENTICAL to TSD-25 — same 33 types, same 77 methods, same 71 properties. Only the underlying assembly version changes. We still publish two separate agents for honest version pinning.

## Engineering envelope

- **Code revision:** see `git rev-parse HEAD` at extraction time (stamped by `aware-agent-builder` into `provenance.code_rev` once that field lands).
- **Sidecar version:** 0.5.1
- **CLI version:** 0.30.0
- **Generator version:** 0.30.0
- **Schema version:** host-coverage v1 (no breaking changes from Phase A foundation).
