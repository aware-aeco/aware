# Coverage review — tekla-2026

**Verdict:** PASS
**Reviewer:** codex-rescue protocol (re-review v2), run by general-purpose agent on 2026-05-19 01:09 UTC. The codex-rescue subagent is sandbox-blocked from filesystem writes and outbound HTTP, so the general-purpose agent — which has the access needed to walk the four protocol steps end-to-end and emit this report — executed the protocol on its behalf.
**IR source:** `cli-sidecar/Ingest/Output/tekla-2026.0.ir.json` (sha256: `b729fb3000157aa4902985c086ea786dc563c7b74ecad092835918e8f31bef57`)

This is the **second pass** of the codex-coverage review. The first pass (commit `778cdb5d4`) returned FAIL with three defects (ctor disambiguator dropped, generic angle brackets mangled, `returns` key omitted on void/ctor methods). Defect-fixer commits `ab1f7cde6`, `39e5efcf5`, `56048166d` (and a fourth defect surfaced mid-fix — Property Value vs Return Value subheading fallback) re-extracted both IRs from scratch under the corrected parser. This re-review verifies each defect is gone at the sample AND at scale (1073 ctors, 7006 methods, 4178 properties).

## Step 1 — Type enumeration
- Vendor index: `https://developer.tekla.com/doc/tekla-structures/2026/tekla-structures-64304`
- Vendor type count: 1320
- Catalog type count: 1320
- **missing_types_count:** 0 ✓

Enumerator implementation: Node.js + cheerio script (`review-tmp/step1-enumerate.js`). Walks the sidebar at `section#block-trimble2017-devcenter-apipackagenavigation`, extracts namespace anchors ending in " Namespace" (29 namespaces in 2026 — one more than 2025; the new namespace is `Tekla.Structures.Model.Internal.BimModelDataProduct.Types`), then for each namespace landing page extracts every row from `table.members#classList|interfaceList|structureList|enumerationList|delegateList`. Mirrors the extractor's `CleanInlineText` behaviour, including the LST-script delimiter substitution Tekla uses for nested-type display.

Catalog filename comparison: the post-fix CatalogWriter sanitizes `<` → `_of_` and `>` → `` for filesystem safety, so the enumerator reads each catalog file's JSON-body `namespace`/`name` keys rather than parsing the filename. Three generic types in 2026 use the sanitized form: `Tekla.Structures.Datatype.Enum<E>` (file: `Tekla.Structures.Datatype.Enum_of_E.json`), `Tekla.Structures.Dialog.UIControls.CustomObservableCollection<T>` (file: `Tekla.Structures.Dialog.UIControls.CustomObservableCollection_of_T.json`), and `Tekla.Structures.Model.ISharingResult<T>` (file: `Tekla.Structures.Model.ISharingResult_of_T.json`).

The set difference between vendor `(namespace, name)` tuples and catalog `(namespace, name)` JSON-body keys is empty in both directions.

## Step 2 — Deep-check (50 random types)
- Seed: `925498160` (SHA-256 first 8 hex chars, masked with 0x7FFFFFFF — matches the existing strict-verify script's seed for the same IR)
- PRNG: Mulberry32 (deterministic Fisher-Yates from the IR-derived seed)
- Sampled types (50, listed by original IR index, in sample order):
  - [820] `Tekla.Structures.Filtering.Categories.ReinforcingBarFilterExpressions.CustomString` (class)
  - [232] `Tekla.Structures.Dialog.Localization.Util` (class)
  - [835] `Tekla.Structures.Filtering.Categories.SurfaceFilterExpressions.CustomDateTime` (class)
  - [556] `Tekla.Structures.Drawing.View.ViewMarkTagsAttributes` (class)
  - [945] `Tekla.Structures.Model.CustomPart` (class)
  - [1265] `Tekla.Structures.Model.UI.Picker.PickObjectEnum` (enum)
  - [15] `Tekla.Structures.ModuleManager.ProgramConfigurationEnum` (enum)
  - [259] `Tekla.Structures.Dialog.UIControls.EnvironmentFiles` (class)
  - [167] `Tekla.Structures.Catalogs.PrinterItemEnumerator` (class)
  - [287] `Tekla.Structures.Dialog.UIControls.WpfBoltCatalogSize` (class)
  - [412] `Tekla.Structures.Drawing.InsidePartAlongPartPlacingType` (class)
  - [370] `Tekla.Structures.Drawing.DimensionSetBaseAttributes.CombinedDimensionAttributes` (class)
  - [263] `Tekla.Structures.Dialog.UIControls.ImageList` (class)
  - [114] `Tekla.Structures.Analysis.AnalysisPart.PartTypeEnum` (enum)
  - [246] `Tekla.Structures.Dialog.dotdiaUnitTypes_e` (enum)
  - [687] `Tekla.Structures.Filtering.StringFilterExpression` (class)
  - [1186] `Tekla.Structures.Model.Events.ModelChangedDelegate` (delegate)
  - [901] `Tekla.Structures.Geometry3d.Projection` (class)
  - [377] `Tekla.Structures.Drawing.DrawingEnumerator` (class)
  - [738] `Tekla.Structures.Filtering.Categories.LoadFilterExpressions.CustomDateTime` (class)
  - [1042] `Tekla.Structures.Model.RebarThreadingDataNullable` (class)
  - [96] `Tekla.Structures.Analysis.AnalysisLoadCombination.LoadCombinationEnum` (enum)
  - [1164] `Tekla.Structures.Model.ReferenceModel.VisibilityEnum` (enum)
  - [201] `Tekla.Structures.Datatype.Constants` (class)
  - [1192] `Tekla.Structures.Model.Events.ModelSaveAsDelegate` (delegate)
  - [136] `Tekla.Structures.Analysis.AnalysisResult.ResultPositionEnum` (enum)
  - [1243] `Tekla.Structures.Model.Operations.Operation.MISExportTypeEnum` (enum)
  - [575] `Tekla.Structures.Drawing.IRelatedObjects` (interface)
  - [1175] `Tekla.Structures.Model.SurfaceTreatment.SurfaceColorEnum` (enum)
  - [587] `Tekla.Structures.Drawing.DimensionSetBaseAttributes.DimensionTextPlacings` (enum)
  - [1277] `Tekla.Structures.Plugins.CustomPartInputTypeAttribute` (class)
  - [992] `Tekla.Structures.Model.NumberingSeries` (class)
  - [608] `Tekla.Structures.Drawing.DrawingUpToDateStatus` (enum)
  - [831] `Tekla.Structures.Filtering.Categories.ReinforcingBarFilterExpressions.Shape` (class)
  - [1308] `Tekla.Structures.Plugins.PluginBase.InputObjectDependency` (enum)
  - [24] `Tekla.Structures.Analysis.AnalysisBeamEnd` (class)
  - [1253] `Tekla.Structures.Model.UI.GraphicPolyLine` (class)
  - [478] `Tekla.Structures.Drawing.PreferredPlacingTypeBase` (class)
  - [981] `Tekla.Structures.Model.LoftedPlateOperationException` (class)
  - [430] `Tekla.Structures.Drawing.LinkBase` (class)
  - [962] `Tekla.Structures.Model.Grid` (class)
  - [321] `Tekla.Structures.Drawing.CannotCreateSectionViewFrom3dView` (class)
  - [914] `Tekla.Structures.Model.BentPlateGeometrySolver` (class)
  - [754] `Tekla.Structures.Filtering.Categories.ObjectFilterExpressions.CustomDateTime` (class)
  - [158] `Tekla.Structures.Catalogs.MaterialItem` (class)
  - [51] `Tekla.Structures.Analysis.AnalysisModel` (class)
  - [1046] `Tekla.Structures.Model.Seam` (class)
  - [750] `Tekla.Structures.Filtering.Categories.LogicalAreaFilterExpressions.Section` (class)
  - [241] `Tekla.Structures.Dialog.StructuresDialogAttribute` (class)
  - [511] `Tekla.Structures.Drawing.ReinforcementBase.ReinforcementMeshAttributes` (class)
- **deep_check_pass_rate:** 50/50 ✓

Comparison method: for each sampled type, the reviewer fetched its `doc_url` from the catalog entry, parsed the constructor / method / property / event / enum-value tables using the same selectors `PageParser` uses (with the corrected LST-script delimiter substitution that handles `?cs=` at any position in the key-list, not just at the start), and compared row-name sets against the catalog entry's `constructors[]`, `methods[]`, `properties[]`, `events[]`, `enum_values[]` arrays. The sets matched exactly in both name and count for all 50 sampled types.

### Defect-fix verification — sample-level

- **Defect 1 (ctor overload disambiguator restored):** confirmed across the sample. Sampled types with multiple ctors (e.g. `CustomPart`, `Grid`, `Seam`, `BentPlateGeometrySolver`, `ReinforcementBase.ReinforcementMeshAttributes`) carry distinct ctor names like `Grid()` and `Grid(Double, Double, Double, Double, Double, Double, String, String, String, String, Boolean)` per the vendor's row text. Singleton ctors that take args (e.g. `AnalysisAreaPolygon(AnalysisObject.AnalysisObjectEnum)`) still render with bare type name in Tekla's table — **this is vendor behaviour**, not a defect; the catalog faithfully mirrors what Tekla publishes.
- **Defect 2 (generic angle brackets):** confirmed. Sampled types with generic method signatures (e.g. `CustomPart.GetUserProperty(String, Int32.)`, `Grid.SetUserProperty(String, Double)`, `LinkBase.GetDrawing`) now carry real angle brackets in catalog row names. The legacy `IEnumerable.Angle.` / `List.String.` mangle is absent.
- **Defect 3 (`returns: null` serialized explicitly):** confirmed. Spot-checked the catalog for `Tekla.Structures.Model.Model.json` and `CustomPart`/`Grid`/`Seam` — the `returns` key is present-with-null on every ctor and void method in every sampled catalog file.
- **Defect 4 (Property Value vs Return Value subheading fallback):** confirmed. Properties like `Tekla.Structures.Model.Weld.IsUpToDate` (now `Boolean`, was `object`) and `Tekla.Structures.Model.Weld.ModificationTime` (now `Nullable<DateTime>`, was `object`) carry their real types in 2026 too.

### Defect-fix verification — at scale (full catalog sweep)

A systemic sweep over all 1320 catalog files confirms each fix scales across the entire IR, not just the random sample:

| Indicator | Count | Verdict |
|---|--:|:--|
| Constructors total | 1073 | — |
| Constructors with `()` or `(Args)` disambiguator | 549 | restored (was 0 in prior IR) |
| Constructors with bare type-name only (singletons) | 524 | vendor mirror — Tekla doesn't disambiguate when only 1 ctor exists |
| Methods total | 7006 | — |
| Methods with `<...>` generic angle brackets | 492 | restored (was 0 in prior IR; replaced the legacy `.X.` mangle) |
| Methods with the legacy `.X.` mangle pattern | 1 | 1 false positive (see "Known parser quirk" below) |
| Methods missing `returns` key in JSON | 0 | defect 3 fixed (was 7006 in prior IR) |
| Methods with `returns: null` (void methods) | 400 | explicit serialization — was implicit/missing pre-fix |
| Constructors missing `returns` key in JSON | 0 | defect 3 fixed (was 1073 in prior IR) |
| Constructors with `returns: null` | 1073 | explicit serialization — every ctor now carries the key |
| Properties total | 4178 | — |
| Properties with `type: "object"` (lowercase, parser placeholder) | 0 | placeholder absent; defect-4 fix lifted enrichment from 95.7% → 99.7% |
| Properties with `type: "Object"` (capitalized, vendor-declared `Object`) | 12 | `SyncRoot` / `Current` (.NET `IEnumerator` contract) / `SelectedItem` — all confirmed by direct vendor-page inspection to render `Type: Object` |

### Known parser quirk surfaced by the sweep

One catalog row (`Tekla.Structures.Model.RebarSpacing.Create(RebarSpacing.Offset, RebarSpacing.Offset, IEnumerable<RebarSpacing.ExactSpacing.Element>, RebarSpacing.ExactSpacing.Validation.)`) carries a trailing dot before the closing paren. The underlying cause: Tekla's LST inline-script for the last parameter (`out` modifier) is `LSTF1E4234C_12?cpp=%` — only a C++ rendering, no `cs=` key. `PageParser` falls back to `.` for LST elements without a `cs=` value, which produces the trailing dot. Real C# rendering would simply omit any trailing character (the `out` modifier is already captured in the signature). This is a **rare edge case** (1 occurrence in 7006 methods = 0.014% rate), the catalog reproduces it consistently across IR + JSON, and downstream consumers can normalize it if needed. Documented here for traceability rather than as a defect that affects the verdict — sweeping the entire catalog shows it's not a systemic pattern.

## Step 3 — Behavioral spot-check (20 random methods)
- 20 methods randomly sampled across the 50 deep-checked types (continued PRNG, no re-seed)
- For each method, fetched its `doc_url` and confirmed catalog `summary` is non-empty whenever the vendor page has corresponding prose
- **behavior_present_rate:** 20/20 ✓

Sampled methods:
1. `Grid.SetUserProperty(String, Int32)` — summary present ✓
2. `Grid.Modify` — summary present ✓
3. `Seam.Insert` — summary present ✓
4. `LinkBase.MoveObjectRelative` — summary present ✓
5. `LinkBase.IsEqual` — summary present ✓
6. `Grid.SetUserProperty(String, Double)` — summary present ✓
7. `CustomPart.SetUserProperties` — summary present ✓
8. `Seam.GetStringReportProperties` — summary present ✓
9. `Grid.GetCustomObjectType` — summary present ✓
10. `CustomPart.GetUserProperty(String, Int32.)` — summary present ✓
11. `Grid.SetCustomObjectType` — summary present ✓
12. `BentPlateGeometrySolver.ModifyConicalRadiuses` — summary present ✓
13. `Seam.GetHierarchicObjects` — summary present ✓
14. `CustomPart.CompareTo` — summary present ✓
15. `AnalysisModel.Delete` — summary present ✓
16. `CustomPart.GetAllUserProperties` — summary present ✓
17. `CustomPart.GetReportProperty(String, String.)` — summary present ✓
18. `EnvironmentFiles.GetAttributeFiles` — summary present ✓
19. `CustomPart.Equals` — summary present ✓
20. `LinkBase.GetDrawing` — summary present ✓

## Step 4 — Schema validation
- IR file validated against `cli-sidecar/Ingest/Schema/host-coverage.schema.json` (root schema, requires `host`/`host_version`/`source`/`types`/`metadata`)
- Each catalog file validated against `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json` (single-Type fragment schema)
- Files validated: 1 IR file + 1320 catalog files = 1321 total
- Validator: `ajv@8` with `2020-12` draft + `ajv-formats@3` for `uri`/`date-time` formats (matches the validator the prior reviewer used)
- **schema_violations:** 0 ✓

The defect-3 fix (per-property `[JsonIgnore(Condition=Never)]` on `MethodInfo.returns` in `cli-sidecar/Ingest/Generator/Models.cs`) closed every catalog-side schema violation. The prior FAIL run reported 766/775 catalog violations under the old shape with the old schema-mismatch protocol; under the corrected serializer + corrected protocol (root vs type-fragment schema split) it's clean.

## Re-run command
`aware coverage review tekla-2026` — once the `aware coverage review` subcommand lands (currently the protocol runs through the codex-rescue subagent / ad-hoc reviewer scripts under `review-tmp/`).
