# allplan-2024 EXTRACTION NOTES

This agent is generated from a coverage IR scraped from Nemetschek's Allplan
PythonParts API documentation at https://pythonparts.allplan.com/2024/. These
notes capture **how** the IR was produced so the extraction can be reproduced
— and so the next reviewer knows what trade-offs the parser made.

## Source

- **Site:** `https://pythonparts.allplan.com/2024/`
- **Version root:** `https://pythonparts.allplan.com/2024/sitemap.xml`
- **Sitemap entries:** 646 total; **574 type-page URLs** after api_reference filtering.
- **Extraction date:** 2026-05-19
- **Source kind:** `scrape` (HttpClient + AngleSharp; no Playwright)
- **Templating engine:** mkdocs-material 9.5.19 + mkdocstrings

## Why this URL — version availability

Nemetschek publishes the PythonParts API documentation at
`pythonparts.allplan.com` with per-major-version trees:

| Path | What |
|---|---|
| `/2024/` | Allplan 2024 PythonParts API — this extractor |
| `/2025/` | Allplan 2025 — sibling extractor (different layout, see below) |
| `/latest/` | Redirects to current production version (2026 at extraction time) |

We pin to `/2024/` explicitly; the `/latest/` route is moving and can't be
used as a stable archive.

This is the **Python plugin API surface** — the one a PythonPart script
invokes from inside the running Allplan app. It is **separate from** the
.NET `Allplan.BIF.Core` BimPlus integration that the prior `allplan-2024`
agent was generated from (via `--from-nuget`). The two surfaces complement
each other; this v0.30 extraction REPLACES the older one because the
PythonParts API is the canonical in-app scripting interface.

## Reproduction

```bash
# From repo root — run the sidecar via stdin envelope.
echo '{"op":"coverage-extract","args":{"vendor":"allplan","version":"2024.0","out_path":"cli-sidecar/Ingest/Output/allplan-2024.0.ir.json"}}' \
  | cli-sidecar/bin/Release/net10.0/win-x64/publish/aware-sidecar.exe

# Regenerate the agent from the IR:
AWARE_SIDECAR=cli-sidecar/bin/Release/net10.0/win-x64/publish/aware-sidecar.exe \
  cli/target/release/aware.exe coverage generate allplan 2024.0 \
  --from-ir cli-sidecar/Ingest/Output/allplan-2024.0.ir.json \
  --vendor nemetschek --vertical architecture
```

The extractor is at `cli-sidecar/Ingest/Extractors/Allplan/` (Extractor.cs +
PageParser.cs). The sidecar verb is `coverage-extract` (registered in
`cli-sidecar/Program.cs`).

## Crawl strategy — single-pass (mkdocs inlines all members)

mkdocstrings packs the entire class — every attribute, every method, every
overload — onto **ONE HTML page per type**. There is no per-member detail
page. A single fetch produces a fully-populated `TypeInfo`. Compared to the
Sandcastle vendors (Tekla / Rhino / Grasshopper) this saves the Phase 4
enrichment pass entirely.

**Pass 1: Sitemap-driven type-URL discovery** (one HTTP fetch).

1. Fetch `https://pythonparts.allplan.com/2024/sitemap.xml`.
2. Parse `<loc>` elements; rewrite `/latest/` to `/2024/` so the URLs we
   crawl are version-pinned.
3. Filter to API reference pages: include
   `/api_reference/<Section>/<Module>/<Class>/`,
   `/api_reference/<Section>/<Module>/_functions/`,
   `/api_reference/<Section>/<Module>/` (2-segment module index, see
   below). Skip `_navigation_tree`, `<Section>/` roots, non-`api_reference`
   pages (manual / getting_started / release_notes).

**Pass 2: Type-page crawl** (4-way concurrent, single shared HttpScraper).

4. Fetch each type page and run `PageParser.Parse()`. Snapshot every 100
   types. Hard failures get up to 2 retries with 750ms backoff.

## 2024 mkdocs-material layout — what the parser handles

The 2024 site uses the **legacy mkdocstrings template**. Distinctive markers:

```html
<h1 id="display-name">Display Name</h1>                <!-- OUTSIDE doc-class -->
<p>Class full path: <code>NemAll_Python_X.Module.Class</code></p>

<div class="doc doc-object doc-class">
  <p class="doc doc-class-bases">Bases: <code>BaseClass</code></p>

  <div class="doc doc-children">
    <h2>Attributes</h2>
    <div class="doc doc-object doc-attribute">
      <h3 class="doc doc-heading"><code>AttrName: SomeType</code></h3>
      <!-- writable label, description -->
    </div>

    <h2>Functions</h2>
    <div class="doc doc-object doc-function">
      <h3 class="doc doc-heading"><code>MethodName(args)</code></h3>
      <p><span class="doc-section-title">Parameters:</span></p>
      <table>...</table>            <!-- table-based sections -->
      <p><span class="doc-section-title">Returns:</span></p>
      <table>...</table>
    </div>

    <div class="doc doc-object doc-function-overload">     <!-- wrapper -->
      <h3 class="doc doc-heading">name <overload label></h3>
      <div class="doc doc-contents doc-contents-overloads">
        <div class="doc doc-object doc-function">         <!-- variant 1 -->
          <code>name(args1)</code>
          <div class="doc doc-contents">...</div>
        </div>
        <div class="doc doc-object doc-function">         <!-- variant 2 -->
          <code>name(args2)</code>
          ...
        </div>
      </div>
    </div>
  </div>
</div>
```

Module-level `_functions/` pages have no `doc-class` wrapper but DO have a
`<div class="doc doc-object doc-module">` container, with the FQN in
`<p>Module full path: <code>FQN</code></p>` at article level.

## IR mapping decisions

| Vendor element | IR field |
|---|---|
| Class with no Bases | `kind: "class"` |
| Class with `Bases: IntEnum / Enum / Flag / IntFlag / StrEnum` | `kind: "enum"`, attributes → `enum_values[]` |
| Class with `Bases: ABC / ABCMeta` | `kind: "interface"` |
| Module-level functions page | Synthetic `kind: "static-class"` named `Functions` under the module namespace |
| `__init__` | `constructors[]` (signature rendered as `TypeName(args)` per Tekla/Rhino convention) |
| Properties (mkdocstrings `doc-attribute`) | `properties[]` — getter always true, setter true iff `doc-label-writable` is present |
| Single-signature methods | `methods[]` |
| Overloaded methods/ctors | One `MethodInfo` per overload variant; the IR `name` carries the parameter list for disambiguation |
| Inner classes (nested `<h2>Classes</h2>`) | Promoted to top-level `TypeInfo` entries with parent-prefixed namespace (e.g. `BaseInteractor.BaseInteractor.InteractorInputMode`) |

## IR / catalog counts

- **Vendor-published type URLs in sitemap:** 574
- **Types extracted (including promoted inner classes):** 612
  - class: 441
  - enum: 155
  - interface: 2
  - static-class: 14
- **Methods:** 5,162
- **Properties:** 1,151
- **Events:** 0 (Allplan PythonParts has no event surface)
- **Page count (HTTP fetches):** 575 (574 type pages + 1 sitemap)
- **Hard errors:** 0 after retry

## Strict-verifier limitation (false-positive `kind-word-missing`)

The shared `cli-sidecar/Ingest/Output/verify-types-strict.ps1` script tests
for the Sandcastle / DocFX phrasing pattern `<TypeName> Class` / `Class
<TypeName>` on the type page. mkdocs-material renders class pages
differently:

- The page title is a Title-Cased display name (e.g. "Allplan Element"),
  not "AllplanElement Class".
- The fully-qualified name appears in
  `<p>Class full path: <code>FQN</code></p>`.
- The kind appears as the Bases clause or the doc-symbol classes — never
  adjacent to the type name in source text.

As a result, the strict-verify sampler reports `49/50 FAIL` with
`kind-word-missing` for this agent — these are reviewer-protocol gaps, NOT
IR errors. The IR's `kind`, `name`, `namespace`, member counts, parameter
types, and return types are all accurate against the live vendor pages
(spot-verified on 5 random types during development).

**The strict-verify protocol needs an mkdocs-material branch** for future
mkdocs-based vendors (Allplan, anyone else who adopts mkdocstrings). The
fix is a script update — out of scope for this extraction PR per the
vendor-impl-brief's "use as-is" rule for foundation files. Tracked as
follow-up for the protocol owner.

## 2024 vs 2025 — same vendor, two layouts

Nemetschek shipped a substantial mkdocstrings template change between 2024
and 2025. The parser supports BOTH layouts in one codebase via shape
discrimination:

- 2024 (legacy mkdocstrings): h3 headings, `<code>` carries full signatures,
  tables for Parameters/Returns/Raises, doc-function-overload wrappers.
- 2025 (current mkdocstrings): h2 headings, `<div class="doc-signature">`
  separated from heading, `<ul>` with `<li class="doc-section-item">` for
  Parameters/Returns/Raises, `<div class="doc-overloads">` wrapper with
  alternating signature/body siblings.

See `cli-sidecar/Ingest/Extractors/Allplan/PageParser.cs` for the dual-format
parsing logic. The same Extractor.cs orchestrator drives both.

## Known limitations / follow-up work

1. **2024 vendor signature limitation.** 2024's mkdocstrings template
   renders function signatures WITHOUT type annotations (only argument
   names appear in the `<code>` header). Type info IS captured per-param
   from the Parameters table — the IR `params[].type` is correct — but
   the IR `signature` string for 2024 methods lacks the type tail. 2025
   signatures include full type annotations.

2. **Static-class synthesis for module-level functions.** Module-level
   free functions (e.g. `NemAll_Python_Geometry.CalcAngle`) have no
   natural class home in the IR schema. We synthesise a `Functions`
   static-class per module to hold them. This is a lossy round-trip —
   the catalog file is `functions.json` per module rather than the
   vendor's flat free-function tree — but downstream tooling treats it
   identically to any other static-class.

3. **Nested class promotion.** Python supports nested classes (e.g.
   `BaseInteractor.InteractorInputMode`). The IR schema has no
   nested-type slot, so we promote inner classes to top-level entries
   with parent-prefixed namespace. The catalog filename will be
   `interactorinputmode.json` (slugified), and the type's `namespace`
   field carries the parent qualifier `BaseInteractor.BaseInteractor`.

4. **Property setter inference.** mkdocstrings emits a `writable` label
   on settable properties. Properties without the label are treated as
   read-only (`getter=true, setter=false`). The vendor's docs are the
   source of truth here; if a property has both a `@property` getter and
   a `@<name>.setter` decorator but mkdocstrings missed the label, the
   IR will miss the setter too. We have not found any cases in the
   Allplan surface; flag if encountered.
