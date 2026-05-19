# allplan-2025 EXTRACTION NOTES

This agent is generated from a coverage IR scraped from Nemetschek's Allplan
PythonParts API documentation at https://pythonparts.allplan.com/2025/. These
notes capture **how** the IR was produced so the extraction can be reproduced
— and so the next reviewer knows what trade-offs the parser made.

## Source

- **Site:** `https://pythonparts.allplan.com/2025/`
- **Version root:** `https://pythonparts.allplan.com/2025/sitemap.xml`
- **Sitemap entries:** 795 total; **725 type-page URLs** after api_reference filtering.
- **Extraction date:** 2026-05-19
- **Source kind:** `scrape` (HttpClient + AngleSharp; no Playwright)
- **Templating engine:** mkdocs-material 9.x + mkdocstrings (newer revision than 2024)

## Why this URL — version availability

Nemetschek publishes the PythonParts API documentation at
`pythonparts.allplan.com` with per-major-version trees:

| Path | What |
|---|---|
| `/2024/` | Allplan 2024 — sibling extractor (older mkdocstrings template) |
| `/2025/` | Allplan 2025 PythonParts API — this extractor |
| `/latest/` | Redirects to current production version |

We pin to `/2025/` explicitly.

This is the **Python plugin API surface** — what a PythonPart script
invokes from inside the running Allplan app.

## Reproduction

```bash
# From repo root — run the sidecar via stdin envelope.
echo '{"op":"coverage-extract","args":{"vendor":"allplan","version":"2025.0","out_path":"cli-sidecar/Ingest/Output/allplan-2025.0.ir.json"}}' \
  | cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe

# Regenerate the agent from the IR:
AWARE_SIDECAR=cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe \
  cli/target/release/aware.exe coverage generate allplan 2025.0 \
  --from-ir cli-sidecar/Ingest/Output/allplan-2025.0.ir.json \
  --vendor nemetschek --vertical architecture
```

The extractor is at `cli-sidecar/Ingest/Extractors/Allplan/` (Extractor.cs +
PageParser.cs). The sidecar verb is `coverage-extract` (registered in
`cli-sidecar/Program.cs`).

## Crawl strategy — single-pass (mkdocs inlines all members)

Same shape as the 2024 sibling. mkdocstrings packs every attribute, method,
and overload onto **ONE HTML page per type** — no per-member detail pages,
no enrichment pass.

**Pass 1: Sitemap-driven type-URL discovery** (one HTTP fetch).

1. Fetch `https://pythonparts.allplan.com/2025/sitemap.xml`.
2. Parse `<loc>` elements; rewrite `/latest/` → `/2025/`.
3. Filter to API reference pages. **For 2025 we DO include 2-segment
   `<Section>/<Module>/` URLs** because they carry inline module-level
   free functions (see "2024 vs 2025 differences" below).

**Pass 2: Type-page crawl** (4-way concurrent, single shared HttpScraper).

4. Fetch each type page; run `PageParser.Parse()`. Snapshot every 100 types.

## 2025 mkdocs-material layout — what the parser handles

The 2025 site uses the **current mkdocstrings template** which differs
significantly from 2024:

```html
<div class="doc doc-object doc-class">
  <h1 id="NemAll_Python_X.Module.Class" class="doc doc-heading">
    <span class="doc doc-object-name doc-class-name">Class</span>
  </h1>
  <p class="doc doc-class-full-path">
    Canonical path: <code>NemAll_Python_X.Module.Class</code>
  </p>
  <p class="doc doc-class-bases">Bases: <code>BaseClass</code></p>

  <div class="doc doc-children">
    <div class="doc doc-object doc-attribute">
      <h2 id="...AttrName" class="doc doc-heading">
        <span class="doc doc-object-name doc-attribute-name">AttrName</span>
        <small class="doc doc-label doc-label-writable">writable</small>
      </h2>
      <div class="doc-signature highlight">
        <pre><code>AttrName: SomeType</code></pre>           <!-- separate signature block -->
      </div>
      <div class="doc-contents">...description...</div>
    </div>

    <div class="doc doc-object doc-function">
      <h2 id="...MethodName" class="doc doc-heading">
        <span class="doc doc-object-name doc-function-name">MethodName</span>
      </h2>
      <div class="doc-signature">
        <pre><code>MethodName(arg: Type) -> RetType</code></pre>
      </div>
      <div class="doc-contents">
        <p>description...</p>
        <p><span class="doc-section-title">Parameters:</span></p>
        <ul>
          <li class="doc-section-item field-body">
            <b><code>arg</code></b> (<code>Type</code>) – <div class="doc-md-description">...</div>
          </li>
        </ul>
        <p><span class="doc-section-title">Returns:</span></p>
        <ul>...</ul>
      </div>
    </div>

    <div class="doc doc-object doc-function">              <!-- overload wrapper -->
      <h2>OverloadedMethod (overloaded label)</h2>
      <div class="doc-overloads">
        <div class="doc-signature">          <!-- variant 1 signature -->
          <pre><code>OverloadedMethod(a: Type1)</code></pre>
        </div>
        <div class="doc doc-object doc-function doc-function-overload">
          <div class="doc-contents">...variant 1 body...</div>
        </div>
        <div class="doc-signature">          <!-- variant 2 signature -->
          <pre><code>OverloadedMethod(a: Type1, b: Type2)</code></pre>
        </div>
        <div class="doc doc-object doc-function doc-function-overload">
          <div class="doc-contents">...variant 2 body...</div>
        </div>
      </div>
    </div>
  </div>
</div>
```

Key structural differences from 2024:

- Member headings are **h2** (not h3); name lives in `<span class="doc-object-name">`,
  not in `<code>` within the heading.
- The signature is a separate `<div class="doc-signature">` sibling of the heading.
- Parameters / Returns / Raises use `<ul><li class="doc-section-item">`, not `<table>`.
- Overload groups use `<div class="doc-overloads">` with alternating
  `<div class="doc-signature">` and `<div class="doc-function-overload">` siblings.
- Namespace landing pages (`<Section>/<Module>/`) use
  `<div class="doc doc-object doc-module">` with `<h1 id="FQN">` and inline
  module-level functions. There are NO separate `_functions/` pages in 2025.

The parser supports BOTH 2024 and 2025 layouts via shape detection. See
`PageParser.cs` for the dual-format logic.

## IR mapping decisions

Identical to the 2024 sibling — see `allplan-2024/EXTRACTION-NOTES.md` for
the table.

Specific to 2025:

- **Module landing pages = static-class for module-level functions.** The
  h1 inside `doc-module` carries the FQN (e.g. `id="Utils.PythonPart"`). We
  use the id as the namespace to disambiguate cross-section collisions
  (e.g. `GeneralScripts/PythonPart/` vs `Utils/PythonPart/`).

## IR / catalog counts

- **Vendor-published type URLs in sitemap (after filter):** 725
- **Types extracted (including promoted inner classes):** 763
  - class: 528
  - enum: 192
  - interface: 3
  - static-class: 40 (vs 14 in 2024 — the namespace landing pages add 26 more)
- **Methods:** 5,783 (vs 5,162 in 2024 — larger surface)
- **Properties:** 1,364 (vs 1,151 in 2024)
- **Events:** 0
- **Page count (HTTP fetches):** 726 (725 type pages + 1 sitemap)
- **Hard errors:** 0 after retry

## Strict-verifier limitation (same as 2024)

The shared `cli-sidecar/Ingest/Output/verify-types-strict.ps1` script reports
49/50 FAIL with `kind-word-missing` because mkdocs-material renders class
pages with the title in `<span class="doc-object-name doc-class-name">`,
not as the Sandcastle/DocFX phrasing `<TypeName> Class`. The IR data is
correct; the verifier protocol needs an mkdocs-material branch.

See `allplan-2024/EXTRACTION-NOTES.md` for the full discussion.

## Known limitations / follow-up work

Same as 2024 — see `allplan-2024/EXTRACTION-NOTES.md` for the full list.

Specific to 2025:

1. **Larger surface than 2024.** 2025 adds 26 namespace-landing static-class
   entries that 2024 served via dedicated `_functions/` pages. The wire
   format is the same (an IR `static-class` with a `methods[]` list); only
   the source URL pattern changed.

2. **Type-annotated signatures.** 2025's mkdocstrings emits Python type
   hints inline in the function signature (e.g.
   `AddToMinMax(minMax: MinMax2D, point: Point2D)`). The IR `signature`
   captures these verbatim. This is RICHER than 2024 — 2024 emits only
   `AddToMinMax(minMax, point)` with type info available only via the
   parameter table. Both versions populate `params[].type` correctly.
