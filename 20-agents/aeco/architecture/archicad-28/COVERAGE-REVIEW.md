# Coverage review — archicad-28.0

**Verdict:** PASS
**Reviewer:** codex-rescue, run 2026-05-19T05:30Z
**IR source:** `cli-sidecar/Ingest/Output/archicad-28.0.ir.json` (sha256: `d9df2b4cb2c5f39a05089b468920de7db837f1bfb06896e409d894922b20c99d`)

## Step 1 — Type enumeration
- Vendor sources (hybrid, JSON-of-truth):
  - Tapir commands: `https://raw.githubusercontent.com/ENZYME-APD/tapir-archicad-automation/main/docs/archicad-addon/command_definitions.js`
  - Tapir schemas: `https://raw.githubusercontent.com/ENZYME-APD/tapir-archicad-automation/main/docs/archicad-addon/common_schema_definitions.js`
  - Graphisoft menu: `https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/content/menutree.json`
- Vendor type count: 229
  - Tapir command categories: 14
  - Tapir shared schema types: 203
  - Graphisoft command groups: 12 (excluding three documentation-only top-level
    nodes — `Introduction`, `Command Format`, `General Information` — that
    carry only `htmldocumentation` / nested sub-groups and produce no commands;
    the extractor correctly skips them per `GraphisoftMenuParser.WalkMenuItem`)
- Catalog type count: 229
- **missing_types_count:** 0 ✓

Reviewer used the JSON-source-of-truth path (per `verify-types-strict-archicad.ps1`)
because the rendered Tapir docs and Graphisoft JSON Interface site are
JS-bootstrapped SPAs whose initial HTML carries no type names.

## Step 2 — Deep-check (50 random types)
- Seed: `1507797836` (= `sha256(IR)[:8]` as int32)
- Sample size: 50 / 229 random types
- Sampled: `Archicad.OfficialCommands.BasicCommands, Tapir.Schema.NormalVolumePropertyValue, Archicad.OfficialCommands.NavigatorTreeCommands, Tapir.Schema.PropertyValuesOrErrorArray, Tapir.Schema.ErrorItem, Tapir.Schema.NormalLengthPropertyValue, Tapir.Schema.EnumValueIdArrayItem, Tapir.Schema.ElementGroupParameters, Tapir.Schema.MeshDetails, Tapir.Schema.DocumentRevisionId, Tapir.Schema.AccuracyType, Tapir.Schema.ManualZoneGeometry, Tapir.AdditionalCommands.AttributeCommands, Tapir.Schema.CurtainWallFrameDetails, Tapir.Schema.RevisionChangesOfEntities, Tapir.Schema.StorySettings, Tapir.Schema.ExpressionDefaultValue, Tapir.Schema.ZoneBoundariesWrapper, Tapir.Schema.BuildingMaterialPhysicalPropertiesArrayItem, Tapir.Schema.GuidId, Tapir.Schema.Error, Tapir.Schema.Favorites, Tapir.Schema.PropertyValuesArrayItem, Tapir.Schema.PropertyDataType, Tapir.Schema.PropertyGroupArrayItem, Tapir.Schema.ElementId, Archicad.OfficialCommands.ElementGeometryCommands, Tapir.Schema.NormalVolumeListPropertyValue, Tapir.Schema.AttributeHeadersOrError, Tapir.Schema.PropertyDefinitionArrayItem, Tapir.Schema.PropertyGroupId, Tapir.Schema.NormalIntegerListPropertyValue, Tapir.Schema.ExecutionResult, Tapir.Schema.GroupId, Tapir.Schema.BasicDefaultValue, Tapir.Schema.PossibleStringValues, Tapir.Schema.NavigatorItemIdArrayItem, Tapir.Schema.LibraryFileAddition, Tapir.Schema.ExecutionResults, Tapir.Schema.ConnectedElementsOrError, Tapir.Schema.ZoneDetails, Tapir.Schema.SetGDLParameterByIndexDetails, Tapir.Schema.CurtainWallDetails, Tapir.Schema.NormalIntegerPropertyValue, Tapir.Schema.AttributeHeadersWrapper, Tapir.Schema.PropertyIdOrErrorArray, Tapir.Schema.ElementClassificationOrError, Tapir.Schema.Issues, Tapir.Schema.GDLParameterList, Tapir.Schema.RevisionChange`
- Per-type checks:
  - Tapir.AdditionalCommands → category name present in `gCommands[*].name`,
    `commands[*].name` set matches IR `methods[*].name`.
  - Tapir.Schema → top-level schema key present in `gSchemaDefinitions`;
    object properties / enum values cross-checked against schema definition.
  - Archicad.OfficialCommands → group name present in Graphisoft menutree's
    walk-derived group list (per extractor convention).
- **deep_check_pass_rate:** 50/50 ✓

## Step 3 — Behavioral spot-check (20 random methods)
- Same seeded RNG continued (no re-seed).
- 20 methods sampled across the 50 types.
- Catalog summaries are populated from each source JSON's `description` field
  (Tapir and Graphisoft commands) and the schema's inline description; the
  extractor passes vendor prose through verbatim.
- **behavior_present_rate:** 20/20 ✓

## Step 4 — Schema validation
- IR validated against `cli-sidecar/Ingest/Schema/host-coverage.schema.json` via
  `aware coverage validate` → status `ok`.
- All 229 catalog files validated against
  `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json` (Draft 2020-12) via
  jsonschema 4.26.0.
- Files validated: 230 (229 catalog + 1 IR)
- **schema_violations:** 0 ✓

## Reviewer notes / known caveats
- v28 official-command surface is reconstructed from the live v29 Graphisoft
  docs (Graphisoft does not archive older JSON Interface references). Any
  v29-only commands present in the catalog will surface a runtime "command
  not found" against a v28 host. Documented in `EXTRACTION-NOTES.md`.
- Tapir method `since` fields encode the Tapir Add-On version (not the
  ArchiCAD version) that introduced the command — same caveat noted in the
  extractor docs and intentional.

## Re-run command
`aware coverage review archicad-28.0`
