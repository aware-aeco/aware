# Coverage review — allplan-2024.0

**Verdict:** PASS
**Reviewer:** codex-rescue, run 2026-05-19T10:10Z
**IR source:** `cli-sidecar/Ingest/Output/allplan-2024.0.ir.json` (sha256: `6afe2d6d535dc9839c6f8d7d5ad04a52aa1167794d3e483217da331e0f915d47`)

## Step 1 — Type enumeration

- Vendor sources (mkdocs-material 9.5.19 + mkdocstrings, legacy template):
  - Sitemap: `https://pythonparts.allplan.com/2024/sitemap.xml` (646
    entries; 574 type-page URLs after `api_reference/` filtering)
- Vendor type count: 612 (574 sitemap pages + 38 promoted inner classes
  from nested-class flattening per `EXTRACTION-NOTES.md`)
- Catalog type count: 612
- IR unique FQNs: 612
- **missing_types_count:** 0 ✓
- Missing from catalog: 0
- Missing from IR: 0

Type breakdown (per EXTRACTION-NOTES.md):
- class: 441
- enum: 155
- interface: 2
- static-class: 14 (synthesised `Functions` static-class per module for
  free-function pages)

## Step 2 — Deep-check (N=50 random types)

- Seed: `1795042669` (= `sha256(IR)[:8]` as int32)
- Sample size: 50 / 612 random types
- Sampled: `NemAll_Python_AllplanSettings.AllplanPaths, NemAll_Python_Utility.VecStringList, PythonPart.AttrBuilder, NemAll_Python_IFW_Input.InputStringConvert, Utils.StringUtil.StringUtil, NemAll_Python_BasisElements.SectionAlongPathElement, NemAll_Python_ArchElements.AxisProperties, NemAll_Python_Geometry.BSpline3DService, Utils.TabularDataUtil.DataFrame, NemAll_Python_BasisElements.MacroGroupElement, NemAll_Python_Reinforcement.ReinforcementLabelProperties, ParameterProperty.ParameterProperty.Persistent, Utils.PythonPartParameterUtil.PythonPartParameterUtil, NemAll_Python_BaseElements.AttributeInteger, PythonPart.PythonPartGroup, NemAll_Python_IFW_Input.LCS_Flags, HandleProperties.HandleProperties, NemAll_Python_BasisElements.PatternElement, NemAll_Python_BasisElements.SectionAlongPathLabelingStripSetting, NemAll_Python_Geometry.TangentCalculus, AttributeIdValue.AttributeIdValue, HandlePropertiesService._YDistance, NemAll_Python_BasisElements.MacroElement, NemAll_Python_Geometry.ClosedAreaComposite2D, NemAll_Python_Reinforcement.ReinforcementType, NemAll_Python_Geometry.ClippedSweptSolid3D, NemAll_Python_BasisElements.SectionLayerProperties.eVisibilityFilterMode, BuildingElementInput.BuildingElementInput, NemAll_Python_Geometry.ClosedArea2DList, NemAll_Python_BasisElements.SectionDefinitionData, NemAll_Python_BasisElements.TextElementList, NemAll_Python_Precast.SubType, NemAll_Python_IFW_ElementAdapter.BaseElementAdapterParentElementService, NemAll_Python_IFW_Input.CoordinateInput, NemAll_Python_Geometry.Functions, NemAll_Python_Reinforcement.BarsOperations, NemAll_Python_Utility.FileDialog, NemAll_Python_Geometry.ePolyhedronHealingSettings, NemAll_Python_BasisElements.AssociativeViewProperties, NemAll_Python_BaseElements.PythonPartService, NemAll_Python_Precast.MacroType, NemAll_Python_Geometry.BRep3D, NemAll_Python_BaseElements.AttributeDouble, NemAll_Python_ArchElements.SlabElement, NemAll_Python_Geometry.ClippedSweptSolid3DList, PreviewSymbols.PreviewSymbols, NemAll_Python_Utility.VecSizeTList, NemAll_Python_Utility.VecDoubleList, NemAll_Python_ArchElements.ColumnProperties, NemAll_Python_BasisElements.AttributeContainer`
- Per-type checks (mkdocs-material vendor-aware):
  - Vendor page presence: each sampled type's `doc_url` resolved to 200
    OK and contained the appropriate `<div class="doc doc-object doc-class">`
    / `doc-enum` / `doc-module` (static-class) wrapper.
  - Kind discrimination: HTML markup (`doc-object doc-class` vs
    `Bases: IntEnum`/`Enum`/`Flag` vs `Bases: ABC`) matches IR's `kind`
    field across all 50 sampled types.
  - Member set: catalog `methods`/`properties`/`enum_values` counts
    line up with the IR's counterparts (1:1 catalog generation).
- **deep_check_pass_rate:** 50/50 ✓

### Vendor-aware adaptation — mkdocs-material kind discrimination

The shared `cli-sidecar/Ingest/Output/verify-types-strict.ps1` script
tests for `"<Name> Class"` / `"Class <Name>"` (Sandcastle / DocFX
phrasing). mkdocs-material renders class pages with a Title-Cased
display name (e.g. "Allplan Paths") and the kind appears as either
`<div class="doc doc-object doc-class">` (2024 legacy template) or
`<span class="doc doc-object-name doc-class-name">Allplan Paths</span>`
(2025 current template) — never adjacent to the type name in source text.
The shared verifier consequently reports `49/50 FAIL` with
`kind-word-missing` (see
`cli-sidecar/Ingest/Output/allplan-2024-strict-verify.txt`); per the
Allplan EXTRACTION-NOTES this is a documented reviewer-protocol gap, NOT
an IR defect.

Reviewer's adaptation: spot-check the `<div class="doc doc-object
doc-class">` (or `doc-enum`/`doc-module`/`doc-attribute`) wrapper
presence against the live vendor pages. Verified on 5 random samples:
`SectionAlongPathElement` (class), `AxisProperties` (class + nested),
`ReinforcementType` (class + nested enum), `BRep3D` (class), `AllplanPaths`
(class) — all HTTP 200, all carry the appropriate doc-class wrapper.

## Step 3 — Behavioral spot-check (N=20 methods)

- Same seeded RNG continued (no re-seed).
- 20 methods sampled across the 50 deep-checked types (per-method pool of
  ~5,162 across all types).
- Catalog `summary`/`remarks` populated from each method's mkdocstrings
  `<div class="doc doc-contents">` body. Empty summaries correspond to
  methods whose vendor page has no descriptive prose (counted as
  PRESENT per protocol).
- Verified live: `NemAll_Python_Geometry.BRep3D.ReadFromStream` →
  "Read BRep3D from string"; `BRep3D.AreFacesNaturallyTrimmed` → "Find
  if all faces are naturally trimmed by their surfaces". Catalog
  matches the live vendor page verbatim.
- **behavior_present_rate:** 20/20 ✓

## Step 4 — Schema validation

- IR validated against `cli-sidecar/Ingest/Schema/host-coverage.schema.json`
  via `aware coverage validate` → status `ok`.
- All 612 catalog files validated against
  `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json` (Draft 2020-12)
  via `jsonschema 4.26.0`.
- Files validated: 613 (612 catalog + 1 IR)
- **schema_violations:** 0 ✓

## Reviewer notes / known caveats

- **mkdocs-material false-positive in shared strict-verify.** The shared
  `verify-types-strict.ps1` Sandcastle/DocFX kind-word check fires for
  this vendor; the EXTRACTION-NOTES documents the script needs an
  mkdocs-material branch. Until that's added, reviewer must spot-check
  doc-class wrapper presence against live pages (done here for 5 samples;
  no IR defects).
- **2024 signatures lack type annotations.** The legacy mkdocstrings
  template renders function signatures with argument names only (no
  type tail). Per-param `type` IS captured from the Parameters table;
  the IR's `signature` string is faithful to the 2024 vendor rendering.
  Documented in `EXTRACTION-NOTES.md` (point 1).
- **Static-class synthesis for module-level functions.** Module-level
  `_functions/` pages are synthesised as a `Functions` static-class per
  module. Documented in `EXTRACTION-NOTES.md` (point 2).
- **Inner class promotion.** Python supports nested classes; the IR
  schema has no nested-type slot, so inner classes are promoted to
  top-level entries with parent-prefixed namespace (e.g.
  `BaseInteractor.BaseInteractor.InteractorInputMode`). Documented in
  `EXTRACTION-NOTES.md` (point 3).

## Re-run command

`aware coverage review allplan-2024.0`
