# tekla-2026 EXTRACTION NOTES

This agent is generated from a coverage IR scraped from
`developer.tekla.com`. These notes capture **how** the IR was produced so the
extraction can be reproduced — and so the next reviewer knows what trade-offs
the parser made.

## Source

- **Site:** `https://developer.tekla.com`
- **Version root:** `https://developer.tekla.com/doc/tekla-structures/2026/tekla-structures-64304`
- **Extraction date:** 2026-05-18 (committed alongside this file)
- **Source kind:** `scrape` (HttpClient + AngleSharp; see "Why not Playwright" below)

## Reproduction

```bash
# From repo root
echo '{"op":"coverage-extract","args":{"vendor":"tekla","version":"2026.0","out_path":"cli-sidecar/Ingest/Output/tekla-2026.0.ir.json"}}' \
  | cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe

# Then regenerate the agent
AWARE_SIDECAR=cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe \
  cli/target/release/aware.exe coverage generate tekla 2026.0 \
  --from-ir cli-sidecar/Ingest/Output/tekla-2026.0.ir.json \
  --vendor trimble --vertical engineering
```

The extractor is at `cli-sidecar/Ingest/Extractors/Tekla/` (Extractor.cs + PageParser.cs).
The sidecar verb is `coverage-extract` (registered in `cli-sidecar/Program.cs`).

## Crawl strategy

1. Start from the version-pinned root namespace URL
   (`/doc/tekla-structures/<year>/tekla-structures-<id>`).
2. The root page sidebar `<section id="block-trimble2017-devcenter-apipackagenavigation">`
   contains a `<ul>` tree with every `Tekla.Structures.*` namespace link. We extract anchors
   whose text ends in ` Namespace`.
3. For each namespace landing page, the type tables
   `<table class="members" id="classList|interfaceList|structureList|enumerationList|delegateList">`
   carry every type's link in the second `<td>` of each row.
4. Each type page is fetched in turn (with a fan-out semaphore of 4 to keep load polite).
   `PageParser.Parse(html, url)` returns a `TypeInfo` (or `null` for non-type pages — see
   Skipped pages below).
5. A second sequential retry pass recovers any URLs that returned a transient
   `OperationCanceledException` during the concurrent first pass.

## CSS selectors used

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
| C# syntax block (delegates) | `div.codeSnippetContainerCode[id$="_code_Div1"] > pre` |

## Parser quirks

- **Language-specific delimiters via inline `<script>`**: Tekla's docs use a pattern like
  `<span class="nolink">System<span id="LSTAB654CE3_0"></span><script>AddLanguageSpecificTextSet("LSTAB654CE3_0?cs=.|vb=.|cpp=::|nu=.|fs=.");</script>Object</span>`.
  In a real browser the script substitutes the C# delimiter (`.`) into the empty `<span id="LST...">`.
  Without JS we synthesize the dot ourselves: when traversing inline text, any element with
  `id` starting with `LST` is replaced with `.`. This is in `CleanInlineText` /
  `CleanNoLinkText` in `PageParser.cs`. Without this, every base type would be
  `SystemObject` (missing dot) and method names containing generics would carry the raw
  `AddLanguageSpecificTextSet("...")` script text — both of which downstream consumers
  would have to deal with separately.

- **URL inconsistency on namespace pages**: Most types link to canonical
  `/doc/tekla-structures/2026/<slug>-<id>` URLs from the sidebar tree, but the namespace
  landing pages still link to legacy `/topic/en/18/47/<guid>` URLs for many types. Both URLs
  serve the same content, so the extractor treats them as equivalent — `doc_url` in the IR
  carries whichever form was on the source page.

- **Empty `<div class="summary">`**: A small number of types (12 in 2026.0) ship with no
  description authored on the vendor side. The IR schema (and the IRReader validator)
  require a non-empty summary, so we substitute a precise placeholder
  `"(No description provided in vendor docs for <ns>.<name>.)"`. The placeholder is
  intentionally honest — it surfaces the omission rather than fabricating prose.

- **Member signatures are not full signatures**: The type pages list members with name +
  short summary only. The full method signature (parameter types + return type) lives on
  per-member pages (`/topic/en/18/47/<guid>` for constructors/methods/properties/events).
  This v1 of the extractor does not walk member pages — for each method we record:
    - `name` = method name (e.g. `Equals`)
    - `signature` = `name + "(...)"` placeholder
    - `summary` = the one-line summary from the type-page row
    - `params` = `[]`
    - `returns` = `null`
    - `throws` = `[]`
  Properties similarly default to `type=object`, `getter=true, setter=true`. Events default
  to `delegate="EventHandler"`, `handler_thread="unknown"`. **A future Phase B pass should
  walk member pages to fill these fields**; the extractor is structured so a per-member
  scrape can be bolted on without touching the type-level orchestration.

- **Why not Playwright**: Phase A shipped a Playwright-based `Scraper` for vendor sites
  that hydrate content client-side. For the Tekla docs site every relevant DOM tree is
  pre-rendered in the initial HTML response — Playwright is overkill and, crucially,
  Playwright's .NET transport uses **reflection-based JSON serialization** internally,
  which `<PublishAot>true</PublishAot>` disables. `Playwright.CreateAsync()` throws
  `InvalidOperationException: Reflection-based serialization has been disabled` on the
  NativeAOT-published sidecar. We added `cli-sidecar/Ingest/Extractors/Common/HttpScraper.cs`
  as a pure-HttpClient alternative and use it for Tekla. Sites that genuinely need JS
  hydration will need a separate AOT story for Playwright.

## Skipped pages

Counted as `skip` in the extractor log: pages whose `<h1>` ends in a non-type word
("Methods", "Properties", "Constructor", "Namespace", etc.). These are reachable from
type tables because a method-overload landing page uses the same `<a>` cell shape.
**The crawler intentionally does not follow these as types** — they would be duplicate
entries (the methods are already enumerated in their parent type's method table).

The 2026 extraction produced **0 skipped pages**.

## Hard failures

None. All 1320 type URLs were fetched and parsed successfully.

## IR / catalog counts

- **Vendor-published namespaces (from sidebar):** 29
- **Unique type URLs discovered:** 1320
- **Types in IR:** 1320
- **Types in generated catalog:** 1320
- **Skipped (non-type) pages:** 0
- **Hard failures:** 0

## 50-type self-verification

A deterministic sample of 50 types was drawn from the IR using a Fisher-Yates
shuffle seeded by the SHA-256 hash of `tekla-2026.0.ir.json` (script:
[`cli-sidecar/Ingest/Output/verify-types.ps1`](../../../../cli-sidecar/Ingest/Output/verify-types.ps1)).
Each sample's `doc_url` was fetched and spot-checked for:

1. The type **name** appears in the rendered HTML.
2. The type's `name + " " + kindWord` (e.g. `"Assertion Class"`,
   `"TemporaryTransparency Enumeration"`) appears verbatim (i.e. the page
   actually IS the type we claim).

Result: **50 / 50 PASS**.

### Sample list + results (deterministic seed = 738096621)

| idx | name | namespace | kind | result |
|-----|------|-----------|------|--------|
| 1130 | Polymesh.PolymeshCheckerFlags | Tekla.Structures.Model | enum | PASS |
| 95 | AnalysisDesignParameterMulti.DesignParIndex | Tekla.Structures.Analysis | enum | PASS |
| 1137 | RebarCranking.CrankSideEnum | Tekla.Structures.Model | enum | PASS |
| 1274 | WeldGeometry | Tekla.Structures.Model.Welding | class | PASS |
| 1242 | Operation.IFCExportViewTypeEnum | Tekla.Structures.Model.Operations | enum | PASS |
| 216 | DistanceList | Tekla.Structures.Datatype | struct | PASS |
| 658 | InputDefinitionFactory | Tekla.Structures.Drawing.Tools | class | PASS |
| 793 | PourObjectFilterExpressions | Tekla.Structures.Filtering.Categories | class | PASS |
| 18 | TeklaStructuresDatabaseTypeEnum | Tekla.Structures | enum | PASS |
| 122 | AnalysisPartCompositeProperties.CompositeBeamWidthEnum | Tekla.Structures.Analysis | enum | PASS |
| 792 | PourBreakFilterExpressions.CustomString | Tekla.Structures.Filtering.Categories | class | PASS |
| 214 | Boolean | Tekla.Structures.Datatype | struct | PASS |
| 32 | AnalysisDecomposedNodeLoad | Tekla.Structures.Analysis | class | PASS |
| 635 | Surfacing.Representation | Tekla.Structures.Drawing | enum | PASS |
| 8 | TeklaStructuresVariables | Tekla.Structures | class | PASS |
| 575 | IPlacing | Tekla.Structures.Drawing | interface | PASS |
| 252 | BoltCatalogStandard | Tekla.Structures.Dialog.UIControls | class | PASS |
| 445 | OpenGraphicObject.OpenGraphicObjectAttributes | Tekla.Structures.Drawing | class | PASS |
| 320 | CannotCreateSectionViewFrom3dView | Tekla.Structures.Drawing | class | PASS |
| 856 | TemplateFilterExpressions.CustomBoolean | Tekla.Structures.Filtering.Categories | class | PASS |
| 1293 | PluginUserInterfaceAttribute | Tekla.Structures.Plugins | class | PASS |
| 909 | BaseWeld | Tekla.Structures.Model | class | PASS |
| 1185 | Events.ModelChangedDelegate | Tekla.Structures.Model | delegate | PASS |
| 879 | CoordinateSystem | Tekla.Structures.Geometry3d | class | PASS |
| 1169 | SharingResultCode | Tekla.Structures.Model | enum | PASS |
| 1312 | FaceEnumerator | Tekla.Structures.Solid | class | PASS |
| 52 | AnalysisModel | Tekla.Structures.Analysis | class | PASS |
| 482 | PrintAttributes | Tekla.Structures.Drawing | class | PASS |
| 372 | DimensionSetBaseAttributes.DimensionFormatAttributes | Tekla.Structures.Drawing | class | PASS |
| 693 | NumericOperatorType | Tekla.Structures.Filtering | enum | PASS |
| 1200 | Events.TsEventOccurredDelegate | Tekla.Structures.Model | delegate | PASS |
| 906 | BaseRebarGroup | Tekla.Structures.Model | class | PASS |
| 1301 | CustomPartBase.CustomPartPositioningType | Tekla.Structures.Plugins | enum | PASS |
| 1029 | RebarMesh | Tekla.Structures.Model | class | PASS |
| 788 | PourBreakFilterExpressions | Tekla.Structures.Filtering.Categories | class | PASS |
| 560 | ViewMarkBasicSymbolAttributes | Tekla.Structures.Drawing | class | PASS |
| 1251 | Color | Tekla.Structures.Model.UI | class | PASS |
| 791 | PourBreakFilterExpressions.CustomDateTime | Tekla.Structures.Filtering.Categories | class | PASS |
| 427 | LineTypeAttributes | Tekla.Structures.Drawing | class | PASS |
| 1146 | RebarLeg.OriginEnum | Tekla.Structures.Model | enum | PASS |
| 317 | BaseLineWithArrowAtStartPointPlacingType | Tekla.Structures.Drawing | class | PASS |
| 408 | HyperLink | Tekla.Structures.Drawing | class | PASS |
| 245 | TeklaProgressBar | Tekla.Structures.Dialog | class | PASS |
| 38 | AnalysisDesignParameter | Tekla.Structures.Analysis | class | PASS |
| 341 | ClosedGraphicObject.ClosedGraphicObjectAttributes | Tekla.Structures.Drawing | class | PASS |
| 215 | Distance | Tekla.Structures.Datatype | struct | PASS |
| 555 | View.ViewMarkSymbolAttributes | Tekla.Structures.Drawing | class | PASS |
| 670 | Events.LayoutEditingModeEnteredDelegate | Tekla.Structures.Drawing.UI | delegate | PASS |
| 376 | Drawing | Tekla.Structures.Drawing | class | PASS |
| 119 | AnalysisPartBarAttributes.ForceStraightSegmentsEnum | Tekla.Structures.Analysis | enum | PASS |

## Known limitations / follow-up work

1. **Method/constructor signatures are placeholders.** A Phase B follow-up should walk
   per-member pages to populate `signature`, `params`, `returns`, `throws`.
2. **Property `type` defaults to `object`** for the same reason.
3. **Event `delegate` defaults to `"EventHandler"`** — vendor pages have the real
   delegate type on the per-member page.
4. **Some inheritance lines are not fully resolved**: types that inherit from a
   `System.*` ancestor are captured (`System.Object`, `System.Exception`, etc.), but
   when the page lists multiple ancestor `nolink` chains the parser uses the LAST one;
   a multi-layer inheritance chain would only record the most-direct ancestor.
5. **Interfaces list is always `[]`** — Tekla's type pages don't itemize implemented
   interfaces in a single dedicated section the way they do for base types; surfacing
   them would require parsing the C# syntax block (e.g. `class Foo : IBar, IBaz`).
