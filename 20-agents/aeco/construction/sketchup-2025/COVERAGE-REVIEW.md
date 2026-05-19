# Coverage review — sketchup-2025.0

**Verdict:** PASS
**Reviewer:** codex-rescue, run 2026-05-19T05:30Z
**IR source:** `cli-sidecar/Ingest/Output/sketchup-2025.0.ir.json` (sha256: `a89e7bc5ac2094a1dda98b74acda052e77beea31c48dd5a37eb324bbb4f786ab`)

## Step 1 — Type enumeration
- Vendor index: `https://ruby.sketchup.com/_index.html` (YARD-generated)
- Vendor type count: 155
- Catalog type count: 155
- **missing_types_count:** 0 ✓

Enumeration: parsed every `href="*.html"` from `_index.html`, dropped the
YARD-internal pages (`_index.html`, `class_list.html`, `method_list.html`,
`top-level-namespace.html`, `frames.html`, `index.html`) and the file-doc
pages (`file.*.html` — ReleaseNotes, LayOut overview, etc.). The remaining
155 paths were mapped to type names by replacing `/` with `::` (per YARD's
folder-as-namespace convention).

## Step 2 — Deep-check (50 random types)
- Seed: `681475013` (= `sha256(IR)[:8]` as int32)
- Sample size: 50 / 155 random types
- Sampled: `Sketchup::FrameChangeObserver, Sketchup::PagesObserver, Sketchup::Entities, Sketchup::UVHelper, Sketchup::OptionsProvider, Sketchup::ArcCurve, Geom::OrientedBounds2d, Sketchup::Animation, Geom::Bounds2d, Sketchup::ImageRep, Sketchup::Page, Geom::UTM, Sketchup::Dimension, SketchupExtension, Sketchup, Sketchup::Console, Layout::ReferenceEntity, Sketchup::Tools, Geom::LatLong, Sketchup::Group, UI::Toolbar, Sketchup::ViewObserver, Sketchup::AttributeDictionaries, Sketchup::Http::Response, Sketchup::LineStyles, Sketchup::Layer, Sketchup::Pages, Sketchup::Material, UI::HtmlDialog, Sketchup::Axes, Layout::ConnectionPoint, Sketchup::InputPoint, Sketchup::AttributeDictionary, Sketchup::Behavior, Sketchup::Licensing, Sketchup::DimensionRadial, Layout::LockedLayerError, Layout::LayerInstance, Sketchup::MaterialsObserver, Layout::FormattedText, Sketchup::Tool, Sketchup::Vertex, Layout::Image, Sketchup::InstancePath, Geom, Sketchup::Texture, Sketchup::DefinitionsObserver, Layout::Style, Sketchup::ComponentDefinition, Sketchup::Classifications`
- Per-type checks:
  - YARD title shape verified: `<h1>Class: Foo::Bar</h1>` / `<h1>Module: ...</h1>` /
    `<h1>Exception: ...</h1>` (with note-spans for Abstract/Deprecated stripped
    per the YARD convention documented in `EXTRACTION-NOTES.md`).
  - Spot-check of one method (or property, or constant) per type — name
    appears in vendor page (`#{name}-instance_method` anchor or `\b{name}\b`).
- **deep_check_pass_rate:** 50/50 ✓

## Step 3 — Behavioral spot-check (20 random methods)
- Same seeded RNG continued (no re-seed).
- 20 methods sampled across the 50 deep-checked types.
- Catalog summary/remarks populated from YARD `div.docstring > div.discussion`
  blocks. Vendor pages re-fetched for catalog-empty cases to confirm vendor
  has no prose (true no-prose-vendor counts as present per protocol).
- **behavior_present_rate:** 20/20 ✓

## Step 4 — Schema validation
- IR validated against `cli-sidecar/Ingest/Schema/host-coverage.schema.json` via
  `aware coverage validate` → status `ok`.
- All 155 catalog files validated against
  `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json` (Draft 2020-12) via
  jsonschema 4.26.0.
- Files validated: 156 (155 catalog + 1 IR)
- **schema_violations:** 0 ✓

## Reviewer notes / known caveats
- Title shape `Class: Foo::Bar` (kind word LEADS with a colon separator) is
  NOT the Sandcastle `Foo Class` convention — the strict-verify-strict.ps1
  script reports `kind-word-missing` for every YARD-sourced type. This is a
  verifier-shape mismatch, NOT an IR defect (per protocol's vendor-specific
  guidance section).
- Ruby has no .NET-style events. All `events[]` arrays in the catalog are
  legitimately empty. Observer-callback methods (`onSaveModel`,
  `onTransactionStart`, etc.) stay in `methods[]` per EXTRACTION-NOTES.
- The 2025.0 and 2026.0 IRs share the unified YARD source at
  `https://ruby.sketchup.com/` (no version selector exists). The two IRs
  carry identical type content; only `host_version` differs. This is the
  intended pattern for unified-doc vendors per the B4 brief.

## Re-run command
`aware coverage review sketchup-2025.0`
