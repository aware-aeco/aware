# tekla-2025 EXTRACTION NOTES

This agent is generated from a coverage IR scraped from
`developer.tekla.com`. These notes capture **how** the IR was produced so the
extraction can be reproduced — and so the next reviewer knows what trade-offs
the parser made.

## Source

- **Site:** `https://developer.tekla.com`
- **Version root:** `https://developer.tekla.com/doc/tekla-structures/2025/tekla-structures-45473`
- **Extraction date:** 2026-05-18 (committed alongside this file)
- **Source kind:** `scrape` (HttpClient + AngleSharp; see "Why not Playwright" below)

## Reproduction

```bash
# From repo root
echo '{"op":"coverage-extract","args":{"vendor":"tekla","version":"2025.0","out_path":"cli-sidecar/Ingest/Output/tekla-2025.0.ir.json"}}' \
  | cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe

# Then regenerate the agent
AWARE_SIDECAR=cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe \
  cli/target/release/aware.exe coverage generate tekla 2025.0 \
  --from-ir cli-sidecar/Ingest/Output/tekla-2025.0.ir.json \
  --vendor trimble --vertical engineering
```

The extractor is at `cli-sidecar/Ingest/Extractors/Tekla/` (Extractor.cs + PageParser.cs).
The sidecar verb is `coverage-extract` (registered in `cli-sidecar/Program.cs`).

## Crawl strategy

Identical to the tekla-2026 extraction — only the root URL changes:

1. Start from the version-pinned root namespace URL
   (`/doc/tekla-structures/2025/tekla-structures-45473`).
2. The root page sidebar `<section id="block-trimble2017-devcenter-apipackagenavigation">`
   contains a `<ul>` tree with every `Tekla.Structures.*` namespace link.
3. For each namespace landing page, the type tables
   `<table class="members" id="classList|interfaceList|structureList|enumerationList|delegateList">`
   carry every type's link.
4. Each type page is fetched via HttpClient with a fan-out semaphore of 4.

The 2025 sidebar enumerates 28 namespaces (one fewer than 2026 — `Tekla.Structures.Model.Internal.BimModelDataProduct.Types` is new in 2026 and absent from 2025's tree). The 2025 layout otherwise matches 2026 exactly, so the same `PageParser` works without per-version branches.

## CSS selectors used

(Identical to tekla-2026 — see that file for the table.)

## Parser quirks

(Identical to tekla-2026 — same site, same page templates. See the tekla-2026 EXTRACTION-NOTES.md for the full quirks list.)

## Skipped pages

The 2025 extraction produced **0 skipped pages**.

## Hard failures

None. All 1282 type URLs were fetched and parsed successfully.

## IR / catalog counts

- **Vendor-published namespaces (from sidebar):** 28
- **Unique type URLs discovered:** 1282
- **Types in IR:** 1282
- **Types in generated catalog:** 1282
- **Skipped (non-type) pages:** 0
- **Hard failures:** 0

## 50-type self-verification

A deterministic sample of 50 types was drawn from the IR using a Fisher-Yates
shuffle seeded by the SHA-256 hash of `tekla-2025.0.ir.json` (script:
[`cli-sidecar/Ingest/Output/verify-types.ps1`](../../../../cli-sidecar/Ingest/Output/verify-types.ps1)).
Each sample's `doc_url` was fetched and spot-checked for:

1. The type **name** appears in the rendered HTML.
2. The type's `name + " " + kindWord` appears verbatim (i.e. the page actually IS the type we claim).

Result: **50 / 50 PASS**.

### Sample list + results (deterministic seed = 2139606680)

| idx | name | namespace | kind | result |
|-----|------|-----------|------|--------|
| 370 | DimensionSetBaseAttributes.DimensionPlacingAttributes | Tekla.Structures.Drawing | class | PASS |
| 612 | LineTypesEnum | Tekla.Structures.Drawing | enum | PASS |
| 1075 | Beam.BeamTypeEnum | Tekla.Structures.Model | enum | PASS |
| 419 | LevelMark.LevelMarkAttributes | Tekla.Structures.Drawing | class | PASS |
| 155 | DrawingItemEnumerator | Tekla.Structures.Catalogs | class | PASS |
| 922 | ConicalSurface | Tekla.Structures.Model | class | PASS |
| 750 | ObjectFilterExpressions.Phase | Tekla.Structures.Filtering.Categories | class | PASS |
| 366 | DimensionSetBaseAttributes | Tekla.Structures.Drawing | class | PASS |
| 11 | ComponentDefinitionTypeEnum | Tekla.Structures | enum | PASS |
| 403 | HyperLink | Tekla.Structures.Drawing | class | PASS |
| 421 | Line.LineAttributes | Tekla.Structures.Drawing | class | PASS |
| 1120 | RebarEndDetailModifier.EndTypeEnum | Tekla.Structures.Model | enum | PASS |
| 1034 | Reinforcement | Tekla.Structures.Model | class | PASS |
| 32 | AnalysisDecomposedNodeLoad | Tekla.Structures.Analysis | class | PASS |
| 1254 | PluginSymbolVisiblityAttribute | Tekla.Structures.Plugins | class | PASS |
| 723 | ConstructionObjectFilterExpressions.CustomNumber | Tekla.Structures.Filtering.Categories | class | PASS |
| 895 | BaseComponent | Tekla.Structures.Model | class | PASS |
| 415 | LeaderLineAndParentObjectAlongPartPlacingType | Tekla.Structures.Drawing | class | PASS |
| 1146 | Reinforcement.RebarOffsetTypeEnum | Tekla.Structures.Model | enum | PASS |
| 761 | PartFilterExpressions | Tekla.Structures.Filtering.Categories | class | PASS |
| 926 | ConnectiveGeometry | Tekla.Structures.Model | class | PASS |
| 718 | ComponentFilterExpressions.Phase | Tekla.Structures.Filtering.Categories | class | PASS |
| 329 | CannotPerformOperationDrawingMustBeActiveException | Tekla.Structures.Drawing | class | PASS |
| 1247 | InputObjectTypeAttribute | Tekla.Structures.Plugins | class | PASS |
| 369 | DimensionSetBaseAttributes.DimensionFormatAttributes | Tekla.Structures.Drawing | class | PASS |
| 150 | ComponentItemEnumerator | Tekla.Structures.Catalogs | class | PASS |
| 1067 | BaseWeld.WeldElectrodeClassificationEnum | Tekla.Structures.Model | enum | PASS |
| 552 | ViewMarkBasicTagAttributes | Tekla.Structures.Drawing | class | PASS |
| 285 | WpfBoltCatalogSize | Tekla.Structures.Dialog.UIControls | class | PASS |
| 735 | LogicalAreaFilterExpressions | Tekla.Structures.Filtering.Categories | class | PASS |
| 413 | LayoutAttributes | Tekla.Structures.Drawing | class | PASS |
| 425 | LinkFrameAttributes | Tekla.Structures.Drawing | class | PASS |
| 546 | View | Tekla.Structures.Drawing | class | PASS |
| 947 | FacesAtAnObtuseAngleException | Tekla.Structures.Model | class | PASS |
| 591 | DimensionSetBaseAttributes.ShortDimensionTypes | Tekla.Structures.Drawing | enum | PASS |
| 312 | AttributesBase | Tekla.Structures.Drawing | class | PASS |
| 42 | AnalysisDesignParameterMulti | Tekla.Structures.Analysis | class | PASS |
| 1255 | PluginUserInterfaceAttribute | Tekla.Structures.Plugins | class | PASS |
| 1005 | RebarComplexGeometry | Tekla.Structures.Model | class | PASS |
| 453 | PlacingAttributes | Tekla.Structures.Drawing | class | PASS |
| 204 | EnumConverter | Tekla.Structures.Datatype | class | PASS |
| 714 | ComponentFilterExpressions.CustomDateTime | Tekla.Structures.Filtering.Categories | class | PASS |
| 665 | BooleanConstantFilterExpression | Tekla.Structures.Filtering | class | PASS |
| 787 | PourObjectFilterExpressions.CustomDateTime | Tekla.Structures.Filtering.Categories | class | PASS |
| 518 | SectionMarkBase.SectionMarkSymbol | Tekla.Structures.Drawing | class | PASS |
| 19 | TeklaStructuresSettings.InvalidPathCallback | Tekla.Structures | delegate | PASS |
| 40 | AnalysisDesignSettings | Tekla.Structures.Analysis | class | PASS |
| 965 | LoadArea | Tekla.Structures.Model | class | PASS |
| 134 | AnalysisResult.LoadCaseTypeEnum | Tekla.Structures.Analysis | enum | PASS |
| 892 | IBoundingVolume | Tekla.Structures.Geometry3d | interface | PASS |

## Known limitations / follow-up work

Same as tekla-2026 — see that file's "Known limitations / follow-up work" section.
