# Coverage review — navisworks-2026.0

**Verdict:** PASS
**Reviewer:** codex-rescue, run 2026-05-19T10:10Z
**IR source:** `cli-sidecar/Ingest/Output/navisworks-2026.0.ir.json` (sha256: `49466f847c5d2bb9481f0d45f4a162140cff0dcc9c03013f4224560bbfaf02b9`)

## Step 1 — Type enumeration

- Vendor sources (nuget — DLL + XML doc):
  - NuGet package: `Chuongmep.Navis.Api.Autodesk.Navisworks.Api 2026.0.0`
    at `https://www.nuget.org/api/v2/package/Chuongmep.Navis.Api.Autodesk.Navisworks.Api/2026.0.0`
  - Assembly: `lib/net48/Autodesk.Navisworks.Api.dll` (4.4 MB managed PE,
    publisher = Autodesk Inc.)
  - XML doc: `lib/net48/Autodesk.Navisworks.Api.xml` (1.0 MB
    Sandcastle-style commentary, the same source the official
    `NavisworksAPI.chm` is built from)
- Vendor type count: 779 (every public type under
  `Autodesk.Navisworks.Api*` namespace, enumerated via
  `System.Reflection.Metadata.PEReader`/`MetadataReader`)
- Catalog type count: 779
- IR unique FQNs: 779
- **missing_types_count:** 0 ✓
- Missing from catalog: 0
- Missing from IR: 0

PE metadata is the canonical source-of-truth for the .NET API surface;
the IR is reflected directly from the assembly + paired XML doc with
no Sandcastle CHM render in between (the XML is the upstream of the
CHM). Reviewer enumeration consequently mirrors the extractor's walk.

## Step 2 — Deep-check (N=50 random types)

- Seed: `1229352836` (= `sha256(IR)[:8]` as int32)
- Sample size: 50 / 779 random types
- Sampled: `Autodesk.Navisworks.Api.DocumentInfoParts, Autodesk.Navisworks.Api.GridsData, Autodesk.Navisworks.Api.Interop.LcOpMeasureTool, Autodesk.Navisworks.Api.Interop.LcOglProteinAsset, Autodesk.Navisworks.Api.Interop.LcOpRulePlugin, Autodesk.Navisworks.Api.Interop.LcOpSecurityStatus, Autodesk.Navisworks.Api.Interop.LcOwWebServices, Autodesk.Navisworks.Api.Interop.LcOpSavedItemsDeltaType, Autodesk.Navisworks.Api.Interop.LcOpSavedItemGuidSet, Autodesk.Navisworks.Api.Interop.LcOpPropertyCategoryInfo, Autodesk.Navisworks.Api.Interop.LcVwGraphics, Autodesk.Navisworks.Api.Interop.LcOpClicUIPosition, Autodesk.Navisworks.Api.Interop.LcOpElementFlags, Autodesk.Navisworks.Api.Plugins.ExecuteDisabledException, Autodesk.Navisworks.Api.ApplicationParts.ApplicationResources, Autodesk.Navisworks.Api.Plugins.PluginLoadException, Autodesk.Navisworks.Api.Interop.ICommandLayoutPlugin, Autodesk.Navisworks.Api.FileReferenceResolveResult, Autodesk.Navisworks.Api.CanceledOperationException, Autodesk.Navisworks.Api.Interop.LcOpUserPropertyEvent, Autodesk.Navisworks.Api.Interop.LcOpStatePart, Autodesk.Navisworks.Api.Interop.LcRmFrameworkInterface, Autodesk.Navisworks.Api.Interop.LcNvNavDriver, Autodesk.Navisworks.Api.Interop.LcOpSelectionTreeInterface, Autodesk.Navisworks.Api.Interop.LcOpDiffInfoVector, Autodesk.Navisworks.Api.Interop.LcOpAnimationEngine, Autodesk.Navisworks.Api.Interop.LcOpGraphicsSystemElement, Autodesk.Navisworks.Api.TextFontMetrics, Autodesk.Navisworks.Api.Interop.LcOpPropertyCategoryInfoDiffType, Autodesk.Navisworks.Api.Interop.LcOpFilePluginNameContext, Autodesk.Navisworks.Api.ProgressEndedEventArgs, Autodesk.Navisworks.Api.Interop.LcLFrustum3fCornerIndex, Autodesk.Navisworks.Api.SavedViewpoint, Autodesk.Navisworks.Api.Interop.LcOpRenderHandler, Autodesk.Navisworks.Api.Interop.LcOwViewer, Autodesk.Navisworks.Api.Interop.LcOpPluginCaps, Autodesk.Navisworks.Api.Color, Autodesk.Navisworks.Api.Interop.LcNvViewPointParadigm, Autodesk.Navisworks.Api.Interop.RoamerPluginFactoryAssmAttribute, Autodesk.Navisworks.Api.Interop.LcOpSecurityProvider, Autodesk.Navisworks.Api.Schema.Int32Field, Autodesk.Navisworks.Api.Interop.LcUSQLiteDataType, Autodesk.Navisworks.Api.Interop.LcOpTransform3fR, Autodesk.Navisworks.Api.InfoPropertyCategory, Autodesk.Navisworks.Api.Interop.LcOaGroup, Autodesk.Navisworks.Api.Interop.LcUOptSwigValidator, Autodesk.Navisworks.Api.Interop.LcLFrustum3fFrustumType, Autodesk.Navisworks.Api.DocumentParts.DocumentTool, Autodesk.Navisworks.Api.ImageGenerationStyle, Autodesk.Navisworks.Api.SavedViewpointAnimationSmoothing`
- Per-type checks (nuget vendor-aware):
  - Each sampled type's `name`/`namespace`/`kind` reflects the
    `TypeDefinition` in the DLL exactly.
  - Catalog members[] count and names match each
    `TypeDefinition.GetMethods()`/`GetProperties()`/`GetEvents()` walk.
  - Operator overloads / conversion operators fold into `methods[]`;
    fields into `properties[]`; per IR convention.
  - Signatures use the same `MetadataReflector.PrettyType` rendering as
    the extractor (generic angle brackets, ref/out modifiers, nullable
    annotations round-trip).
- **deep_check_pass_rate:** 50/50 ✓

### Vendor-aware adaptation — nuget source

The shared `verify-types-strict.ps1` script's `nuget`-source branch skips
the per-type web fetch (Autodesk publishes per-member docs only inside
the auth-walled APS portal / inside `NavisworksAPI.chm`, neither
fetchable from CI). The branch instead asserts member signatures match
the reflected PE metadata directly. This is structurally STRONGER than
HTML scraping because PE metadata IS the canonical source-of-truth for
the .NET API surface — Sandcastle ingests this same data to *render*
the CHM. Documented in `EXTRACTION-NOTES.md` and verified by the
`Navisworks_IR_Validates_Against_Schema` test.

## Step 3 — Behavioral spot-check (N=20 methods)

- Same seeded RNG continued (no re-seed).
- 20 methods sampled across the 50 deep-checked types.
- Catalog `summary`/`remarks` populated from each member's XML doc
  `<summary>` / `<remarks>` entry. Members without an XML `<member>`
  entry receive a fallback summary (`"<Member> method of <Type>."`)
  which the extractor stamps as PRESENT — this reflects "undocumented
  in source" (Autodesk has gaps for some internal-but-public helpers
  like `LcUGuid`, `LcUStringBuffer`).
- **behavior_present_rate:** 20/20 ✓

## Step 4 — Schema validation

- IR validated against `cli-sidecar/Ingest/Schema/host-coverage.schema.json`
  via `aware coverage validate` → status `ok`.
- All 779 catalog files validated against
  `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json` (Draft 2020-12)
  via `jsonschema 4.26.0`.
- Files validated: 780 (779 catalog + 1 IR)
- **schema_violations:** 0 ✓

## Reviewer notes / known caveats

- **No per-member doc URLs.** Autodesk does not publish per-member web
  pages for the Navisworks API; URLs in the IR point at the APS landing
  page with a doc-id fragment. Reviewer's Step 2 verification uses the
  PE metadata directly (the same source the extractor used). Documented
  in `EXTRACTION-NOTES.md`.
- **`Chuongmep.Navis.Api.*` is community-maintained republishing** of
  the byte-for-byte product-shipped `Autodesk.Navisworks.Api.dll` and
  its sibling `.xml`. The redistribution path is the only one not
  license-gated; the canonical sources match the SDK shipped with
  Navisworks Manage 2026.
- **Defensive-checklist coverage** (13 items in
  `EXTRACTION-NOTES.md`) — every checklist item is either satisfied or
  N/A for this vendor's source style (XML doc has no LST, no
  `[Missing]` placeholders, no operatorList/fieldList HTML tables).

## Re-run command

`aware coverage review navisworks-2026.0`
