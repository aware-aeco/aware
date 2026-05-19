# Coverage review — sketchup-2026.0

**Verdict:** PASS
**Reviewer:** codex-rescue, run 2026-05-19T05:30Z
**IR source:** `cli-sidecar/Ingest/Output/sketchup-2026.0.ir.json` (sha256: `269e6ede3e3773a82bf5324107320c20efbb2ad40f670e4f078703a1a2d95f80`)

## Step 1 — Type enumeration
- Vendor index: `https://ruby.sketchup.com/_index.html` (YARD-generated, unified site)
- Vendor type count: 155
- Catalog type count: 155
- **missing_types_count:** 0 ✓

Enumeration: same `_index.html` walk as v2025 (`href="*.html"` minus YARD
internal pages + file-doc pages). The unified YARD site is the same source
for both versions per `EXTRACTION-NOTES.md` and the B4 brief.

## Step 2 — Deep-check (50 random types)
- Seed: `647917278` (= `sha256(IR)[:8]` as int32)
- Sample size: 50 / 155 random types
- Sampled: `Layout, Sketchup::Drawingelement, Sketchup::Face, Layout::Pages, Layout::LockedEntityError, Layout::LinearDimension, Sketchup::LoadHandler, Sketchup::AppObserver, Sketchup::FrameChangeObserver, Sketchup::ExtensionsManager, Sketchup::View, Sketchup::ComponentInstance, Layout::Layer, UI::HtmlDialog, Layout::ReferenceEntity, Sketchup::DimensionRadial, Sketchup::Axes, Length, Sketchup::Tool, Sketchup::ConstructionPoint, Sketchup::TextureWriter, Sketchup::Layers, UI, Sketchup::Vertex, Sketchup::LayersObserver, Sketchup::RenderingOptions, Sketchup::RenderingOptionsObserver, Geom::Transformation, Layout::Path, Sketchup::ShadowInfo, Sketchup::PickHelper, Sketchup::Layer, Sketchup::Licensing::ExtensionLicense, Layout::TableCell, Sketchup::Importer, Layout::SketchUpModel, Sketchup::Material, Sketchup::Image, Sketchup::ConstructionLine, Sketchup::ModelObserver, Sketchup::DefinitionObserver, Layout::TableRow, Sketchup::DefinitionsObserver, Sketchup::LineStyles, Sketchup::PagesObserver, Sketchup::Texture, Sketchup::ToolsObserver, Numeric, Sketchup::EnvironmentsObserver, Sketchup::Environments`
- Per-type checks: YARD title verification + first-member name presence
  (same protocol as v2025).
- **deep_check_pass_rate:** 50/50 ✓

## Step 3 — Behavioral spot-check (20 random methods)
- Same seeded RNG continued (no re-seed).
- 20 methods sampled across the 50 deep-checked types.
- Catalog summary/remarks from YARD `div.docstring` blocks.
- **behavior_present_rate:** 20/20 ✓

## Step 4 — Schema validation
- IR validated against `cli-sidecar/Ingest/Schema/host-coverage.schema.json` via
  `aware coverage validate` → status `ok`.
- All 155 catalog files validated against
  `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json` (Draft 2020-12) via
  jsonschema 4.26.0.
- Files validated: 156 (155 catalog + 1 IR)
- **schema_violations:** 0 ✓

## Reviewer notes
- IR content is identical to v2025.0 (same unified source). The two IRs
  differ only in `host_version`; their `types[]` payloads carry the same
  155 types with the same member counts. This is the agreed convention for
  unified-doc vendors (v0.30 Phase B4).
- YARD title shape, Ruby-events-empty, and constants-as-properties caveats
  all apply identically — see v2025's report and EXTRACTION-NOTES.

## Re-run command
`aware coverage review sketchup-2026.0`
