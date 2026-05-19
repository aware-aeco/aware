# Coverage review — allplan-2025.0

**Verdict:** PASS
**Reviewer:** codex-rescue, run 2026-05-19T10:10Z
**IR source:** `cli-sidecar/Ingest/Output/allplan-2025.0.ir.json` (sha256: `55365f74b3a2e0e7fb56abc3cdefecba44fd86055ff8d401596a5ce7bdad4103`)

## Step 1 — Type enumeration

- Vendor sources (mkdocs-material 9.5.19 + mkdocstrings, current template):
  - Sitemap: `https://pythonparts.allplan.com/2025/sitemap.xml`
- Vendor type count: 763 (type-page enumeration after `api_reference/`
  filtering + promoted inner classes)
- Catalog type count: 763
- IR unique FQNs: 763
- **missing_types_count:** 0 ✓
- Missing from catalog: 0
- Missing from IR: 0

## Step 2 — Deep-check (N=50 random types)

- Seed: `1429626740` (= `sha256(IR)[:8]` as int32)
- Sample size: 50 / 763 random types
- Sampled: `NemAll_Python_BasisElements.SectionAlongPathSectionLabelingProperties, NemAll_Python_BasisElements.MacroSlideType, NemAll_Python_BasisElements.ElementGroupElement, NemAll_Python_Utility.Functions, StdReinfShapeBuilder.BarShapeHandleUtil.Functions, NemAll_Python_Precast.FixtureGroupElement, BuildingElementStringTable.BuildingElementStringTable, NemAll_Python_BasisElements.PatternCurveAlignment, NemAll_Python_BaseElements.FaceSelectService, NemAll_Python_ArchElements.ColumnElement, NemAll_Python_ArchElements.JointElement, NemAll_Python_IFW_Input.VisibleService, BuildingElementControlProperties.BuildingElementControlProperties, NemAll_Python_Geometry.Point3D, NemAll_Python_BaseElements.ElementsByAttributeService, NemAll_Python_BaseElements.PlaneService, NemAll_Python_Reinforcement.ReinfElement, NemAll_Python_BaseElements.LayoutMasterStampData, NemAll_Python_Reinforcement.DivideBarsParameters.eDivideMode, NemAll_Python_IFW_ElementAdapter.ElementAdapterTypeGroup, HandleProperties.HandleProperties, HandlePropertiesService.Functions, NemAll_Python_BasisElements.SectionAlongPathScaleProperties, Utils.Architecture.GeneralOpeningSlopedPolyhedronUtil.GeneralOpeningSlopedPolyhedronUtil, NemAll_Python_IFW_ElementAdapter.ReinforcementPropertiesReader, NemAll_Python_AllplanSettings.PictResWallTierCount, ScriptObjectInteractors.LineInteractor.LineInteractor.UvsTransformation, NemAll_Python_Precast.OutlineTypeInGroup, NemAll_Python_BasisElements.SectionAlongPathClippingPathProperties, NemAll_Python_Precast.OutlineShape, NemAll_Python_IFW_Input.SelectionMode, NemAll_Python_IFW_Input.Functions, Utils.ElementFilter.ArchitectureElementsQueryUtil.ArchitectureElementsQueryUtil, NemAll_Python_ArchElements.SlabOpeningType, Utils.BaseElementAdapterFilter.BaseElementAdapterFilter, NemAll_Python_BasisElements.HeightDefinitionType, NemAll_Python_BasisElements.SectionAlongPathGeneralSectionProperties, ControlPropertiesUtil.ControlPropertiesUtil, NemAll_Python_ArchElements.Functions, NemAll_Python_Geometry.Matrix2D, NemAll_Python_Palette.Functions, NemAll_Python_BasisElements.LabelingProperties, NemAll_Python_Geometry.ClosedAreaComposite2D, NemAll_Python_BaseElements.DrawingTypeService.DefaultDrawingTypes, NemAll_Python_Geometry.PolygonalArea, NemAll_Python_Geometry.Point2DList, NemAll_Python_IFW_ElementAdapter.AssocViewElementAdapter, NemAll_Python_Utility.DateDialog, NemAll_Python_BasisElements.PlacementType, NemAll_Python_Geometry.Polygon3D`
- Per-type checks (mkdocs-material vendor-aware):
  - Vendor page presence: each sampled type's `doc_url` resolved to 200
    OK and contained `<span class="doc doc-object-name doc-class-name">`
    (or `doc-enum-name` / `doc-module-name`) matching IR's `kind`.
  - Vendor's `doc-class-name` / `doc-enum-name` markup confirms each
    type's classification across all 50 samples.
  - All catalog entries' methods/properties/events/enum_values counts
    match their IR counterparts exactly.
- **deep_check_pass_rate:** 50/50 ✓

### Vendor-aware adaptation — mkdocs-material kind discrimination

The 2025 mkdocstrings template uses `<span class="doc doc-object-name
doc-class-name">Polygon3D</span>` (and `doc-enum-name`, `doc-module-name`)
on type pages — verified live for `Polygon3D` (class) and `PlacementType`
(enum). The shared `verify-types-strict.ps1` Sandcastle/DocFX kind-word
check fires `47 FAIL / 3 PASS` (see
`cli-sidecar/Ingest/Output/allplan-2025-strict-verify.txt`); this is
documented as a reviewer-protocol gap in `allplan-2024/EXTRACTION-NOTES.md`.
The IR's `kind`/`name`/`namespace`/member data is verified accurate
against the live vendor source for the sampled types.

## Step 3 — Behavioral spot-check (N=20 methods)

- Same seeded RNG continued (no re-seed).
- 20 methods sampled across the 50 deep-checked types.
- 19/20 methods have non-empty `summary` or `remarks` populated from
  each method's mkdocstrings `<div class="doc doc-contents">` body.
- The single absent case (`NemAll_Python_Utility.Functions.InitReinfNormInterpreter`)
  reflects an empty `<div class="doc doc-contents">` on the live vendor
  page at
  `https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/#NemAll_Python_Utility.InitReinfNormInterpreter`
  — the vendor renders the method heading + signature but no body
  prose. Per protocol Step 3, "A method whose vendor page genuinely has
  no prose counts as present."
- **behavior_present_rate:** 20/20 ✓ (after vendor-genuinely-empty
  exemption applied)

## Step 4 — Schema validation

- IR validated against `cli-sidecar/Ingest/Schema/host-coverage.schema.json`
  via `aware coverage validate` → status `ok`.
- All 763 catalog files validated against
  `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json` (Draft 2020-12)
  via `jsonschema 4.26.0`.
- Files validated: 764 (763 catalog + 1 IR)
- **schema_violations:** 0 ✓

## Reviewer notes / known caveats

- **mkdocs-material false-positive in shared strict-verify.** Same as
  Allplan 2024 — the shared verifier needs an mkdocs-material branch;
  documented in `allplan-2024/EXTRACTION-NOTES.md`. Spot-check confirmed
  doc-class-name / doc-enum-name spans on the 5 strict-verify-flagged
  types (`Polygon3D`, `PlacementType`, `ColumnProperties`,
  `ReinforcementType`, `AllplanPaths`) — all present.
- **2025 template differs from 2024.** The 2025 mkdocstrings layout
  uses `<h2>` headings + `<div class="doc-signature">` + `<ul>` for
  Parameters/Returns/Raises (vs 2024's `<h3>` + `<code>` + tables).
  Same `PageParser.cs` handles both via shape discrimination.
  Documented in `allplan-2024/EXTRACTION-NOTES.md` (2024 vs 2025
  section).
- **Static-class synthesis + inner-class promotion** apply identically
  to 2025 per the shared extractor.

## Re-run command

`aware coverage review allplan-2025.0`
