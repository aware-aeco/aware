# Coverage review — archicad-29.0

**Verdict:** PASS
**Reviewer:** codex-rescue, run 2026-05-19T05:30Z
**IR source:** `cli-sidecar/Ingest/Output/archicad-29.0.ir.json` (sha256: `a2407bf985a9b46c30b98edc39a741d55e9cc0d63c9d1b85b709363c387fd8fa`)

## Step 1 — Type enumeration
- Vendor sources (hybrid, JSON-of-truth):
  - Tapir commands: `https://raw.githubusercontent.com/ENZYME-APD/tapir-archicad-automation/main/docs/archicad-addon/command_definitions.js`
  - Tapir schemas: `https://raw.githubusercontent.com/ENZYME-APD/tapir-archicad-automation/main/docs/archicad-addon/common_schema_definitions.js`
  - Graphisoft menu: `https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/content/menutree.json`
- Vendor type count: 229
  - Tapir command categories: 14
  - Tapir shared schema types: 203
  - Graphisoft command groups: 12 (three documentation-only top-level nodes
    `Introduction`, `Command Format`, `General Information` are correctly
    skipped — they carry only `htmldocumentation` / nested sub-groups, no
    direct command children)
- Catalog type count: 229
- **missing_types_count:** 0 ✓

Reviewer used the JSON-source-of-truth path (per `verify-types-strict-archicad.ps1`)
because both Tapir and Graphisoft doc sites are JS-bootstrapped SPAs whose
initial HTML carries no type names — the rendered DOM is hydrated client-side
from the same JSON files the extractor consumes.

## Step 2 — Deep-check (50 random types)
- Seed: `574651385` (= `sha256(IR)[:8]` as int32)
- Sample size: 50 / 229 random types
- Sampled: `Tapir.Schema.ElementClassification, Tapir.Schema.ElementClassificationItemArray, Tapir.Schema.ExecutionResults, Tapir.Schema.AttributePropertyValues, Tapir.Schema.NavigatorItemIdArrayItem, Tapir.Schema.VolumeType, Tapir.Schema.GuidId, Tapir.Schema.AttributeId, Tapir.Schema.UserUndefinedPropertyValue, Archicad.OfficialCommands.AttributeCommands, Tapir.Schema.LibPartDetails, Tapir.Schema.ElementIdArrayItem, Tapir.Schema.PropertyDefinition, Tapir.Schema.LayerCombinationAttributeOrError, Tapir.AdditionalCommands.RevisionManagementCommands, Tapir.Schema.ClassificationSystemIdArrayItem, Tapir.Schema.ElementPropertyValues, Tapir.Schema.AccuracyType, Tapir.Schema.CurtainWallFrameType, Tapir.Schema.ColorRGB, Tapir.Schema.StorySettings, Tapir.Schema.DocumentRevisionId, Tapir.Schema.ConnectedElementsWrapper, Tapir.Schema.ElementGroupParameters, Archicad.OfficialCommands.NavigatorTreeCommands, Tapir.Schema.RevisionChangesArrayItem, Tapir.Schema.ElementOrGroupIds, Tapir.Schema.MeshSkirtType, Tapir.Schema.AttributeIds, Tapir.Schema.NormalSingleEnumPropertyValue, Tapir.Schema.RevisionChangesOfEntities, Tapir.Schema.BoundingBoxes3D, Tapir.AdditionalCommands.DeveloperCommands, Tapir.Schema.ProjectInfoFields, Tapir.Schema.IssueCommentStatus, Tapir.Schema.NavigatorItemIds, Tapir.Schema.ClassificationIdOrError, Tapir.Schema.BuildingMaterialPhysicalPropertiesList, Tapir.Schema.Holes2D, Tapir.Schema.CurtainWallSegmentDetails, Tapir.Schema.StoriesSettings, Tapir.Schema.SetGDLParameterArray, Tapir.Schema.ClassificationId, Tapir.Schema.RevisionIssue, Tapir.Schema.PropertyDetails, Tapir.Schema.ExpressionDefaultValue, Archicad.OfficialCommands.ViewMapCommands, Tapir.Schema.BuildingMaterialPhysicalProperties, Tapir.Schema.FavoritesWrapper, Tapir.Schema.WallDetails`
- Per-type checks: same as v28 — namespace name presence + per-namespace
  cross-checks (Tapir commands vs source command list, Tapir schemas vs
  source schema dict, Graphisoft groups vs menutree walk-derived set).
- **deep_check_pass_rate:** 50/50 ✓

## Step 3 — Behavioral spot-check (20 random methods)
- Same seeded RNG continued (no re-seed).
- 20 methods sampled across the 50 deep-checked types.
- Catalog summaries pass through vendor JSON `description` fields verbatim.
- **behavior_present_rate:** 20/20 ✓

## Step 4 — Schema validation
- IR validated against `cli-sidecar/Ingest/Schema/host-coverage.schema.json` via
  `aware coverage validate` → status `ok`.
- All 229 catalog files validated against
  `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json` (Draft 2020-12) via
  jsonschema 4.26.0.
- Files validated: 230 (229 catalog + 1 IR)
- **schema_violations:** 0 ✓

## Reviewer notes
- v29 is the live Graphisoft reference (confirmed 2026-05-19); v28 IR is a
  reconstruction from this same surface. The two IRs differ only in
  `host_version`; their `types[]` payloads are identical (same `444268` bytes).
  This is the intended "shared-doc-tree" pattern documented in v28's
  EXTRACTION-NOTES.md.

## Re-run command
`aware coverage review archicad-29.0`
