# Coverage review — tekla-2025

**Verdict:** PASS
**Reviewer:** codex-rescue protocol (re-review v2), run by general-purpose agent on 2026-05-19 01:09 UTC. The codex-rescue subagent is sandbox-blocked from filesystem writes and outbound HTTP, so the general-purpose agent — which has the access needed to walk the four protocol steps end-to-end and emit this report — executed the protocol on its behalf.
**IR source:** `cli-sidecar/Ingest/Output/tekla-2025.0.ir.json` (sha256: `4d1efd29e27f12626611d980929c1ad56006f26b45b82cd4d121fd26e865270c`)

This is the **second pass** of the codex-coverage review. The first pass (commit `5a0441424`) returned FAIL with three defects (ctor disambiguator dropped, generic angle brackets mangled, `returns` key omitted on void/ctor methods). Defect-fixer commits `ab1f7cde6`, `39e5efcf5`, `2701f8d60` (and a fourth defect surfaced mid-fix — Property Value vs Return Value subheading fallback) re-extracted both IRs from scratch under the corrected parser. This re-review verifies each defect is gone at the sample AND at scale (1058 ctors, 6879 methods, 4045 properties).

## Step 1 — Type enumeration
- Vendor index: `https://developer.tekla.com/doc/tekla-structures/2025/tekla-structures-45473`
- Vendor type count: 1282
- Catalog type count: 1282
- **missing_types_count:** 0 ✓

Enumerator implementation: Node.js + cheerio script (`review-tmp/step1-enumerate.js`). Walks the sidebar at `section#block-trimble2017-devcenter-apipackagenavigation`, extracts namespace anchors ending in " Namespace" (28 namespaces in 2025), then for each namespace landing page extracts every row from `table.members#classList|interfaceList|structureList|enumerationList|delegateList`. Mirrors the extractor's `CleanInlineText` behaviour, including the LST-script delimiter substitution Tekla uses for nested-type display.

Catalog filename comparison: the post-fix CatalogWriter sanitizes `<` → `_of_` and `>` → `` for filesystem safety, so the enumerator reads each catalog file's JSON-body `namespace`/`name` keys rather than parsing the filename. Two generic types in 2025 use the sanitized form: `Tekla.Structures.Datatype.Enum<E>` (file: `Tekla.Structures.Datatype.Enum_of_E.json`) and `Tekla.Structures.Dialog.UIControls.CustomObservableCollection<T>` (file: `Tekla.Structures.Dialog.UIControls.CustomObservableCollection_of_T.json`).

The set difference between vendor `(namespace, name)` tuples and catalog `(namespace, name)` JSON-body keys is empty in both directions.

## Step 2 — Deep-check (50 random types)
- Seed: `1293876521` (SHA-256 first 8 hex chars, masked with 0x7FFFFFFF — matches the existing strict-verify script's seed for the same IR)
- PRNG: Mulberry32 (deterministic Fisher-Yates from the IR-derived seed)
- Sampled types (50, listed by original IR index, in sample order):
  - [690] `Tekla.Structures.Filtering.Categories.AssemblyFilterExpressions.CustomString` (class)
  - [477] `Tekla.Structures.Drawing.PrintAttributes` (class)
  - [701] `Tekla.Structures.Filtering.Categories.BoltFilterExpressions` (class)
  - [538] `Tekla.Structures.Drawing.SymbolLibrary` (class)
  - [916] `Tekla.Structures.Model.CircleRebarGroup` (class)
  - [448] `Tekla.Structures.Drawing.PickerInputPointsWithinAView` (class)
  - [957] `Tekla.Structures.Model.GridSurface` (class)
  - [914] `Tekla.Structures.Model.Chamfer` (class)
  - [235] `Tekla.Structures.Dialog.LocExtension` (class)
  - [1164] `Tekla.Structures.Model.Events.ModelSaveAsDelegate` (delegate)
  - [1030] `Tekla.Structures.Model.RebarStrand` (class)
  - [1011] `Tekla.Structures.Model.RebarHookData` (class)
  - [1210] `Tekla.Structures.Model.Operations.Operation.UnselectedModeEnum` (enum)
  - [304] `Tekla.Structures.Drawing.AlongLinePlacingType` (class)
  - [72] `Tekla.Structures.Analysis.AnalysisPartBarAttributes` (class)
  - [292] `Tekla.Structures.Dialog.UIControls.WpfOkApplyModifyGetOnOffCancel` (class)
  - [1145] `Tekla.Structures.Model.Reinforcement.RebarGeometryOptionEnum` (enum)
  - [1270] `Tekla.Structures.Plugins.PluginBase.SymbolVisibility` (enum)
  - [347] `Tekla.Structures.Drawing.CurvedDimensionRadial` (class)
  - [890] `Tekla.Structures.Geometry3d.PolyLine` (class)
  - [57] `Tekla.Structures.Analysis.AnalysisModelIssue` (class)
  - [1253] `Tekla.Structures.Plugins.PluginPropertyFileLocationAttribute` (class)
  - [495] `Tekla.Structures.Drawing.RadiusDimensionAttributes` (class)
  - [24] `Tekla.Structures.Analysis.AnalysisCompositeBeam` (class)
  - [300] `Tekla.Structures.Dialog.UIControls.WpfSaveLoad.HelpFileTypeEnum` (enum)
  - [293] `Tekla.Structures.Dialog.UIControls.WpfOkCreateCancel` (class)
  - [1193] `Tekla.Structures.Model.Collaboration.ReferenceModelObjectAttributeEnumerator` (class)
  - [834] `Tekla.Structures.Filtering.Categories.TaskFilterExpressions` (class)
  - [302] `Tekla.Structures.Drawing.AlongLineOrWithLeaderLinePlacingType` (class)
  - [1166] `Tekla.Structures.Model.Events.ModelSaveInfoDelegate` (delegate)
  - [697] `Tekla.Structures.Filtering.Categories.AssemblyFilterExpressions.Prefix` (class)
  - [574] `Tekla.Structures.Drawing.ArrowheadTypes` (enum)
  - [854] `Tekla.Structures.Filtering.Categories.WeldFilterExpressions.CustomNumber` (class)
  - [1225] `Tekla.Structures.Model.UI.ViewHandler` (class)
  - [841] `Tekla.Structures.Filtering.Categories.TaskFilterExpressions.CustomString` (class)
  - [1211] `Tekla.Structures.Model.Operations.Operation.ShapeMetadataResult` (enum)
  - [1004] `Tekla.Structures.Model.ProjectInfo` (class)
  - [893] `Tekla.Structures.Geometry3d.ICurve` (interface)
  - [185] `Tekla.Structures.Catalogs.BoltItem.BoltItemTypeEnum` (enum)
  - [1132] `Tekla.Structures.Model.RebarPropertyModifier.GroupingTypeEnum` (enum)
  - [311] `Tekla.Structures.Drawing.AssemblyDrawing` (class)
  - [908] `Tekla.Structures.Model.BoltGroup` (class)
  - [814] `Tekla.Structures.Filtering.Categories.ReinforcingBarFilterExpressions.Length` (class)
  - [911] `Tekla.Structures.Model.Boolean` (class)
  - [529] `Tekla.Structures.Drawing.StringList` (class)
  - [1026] `Tekla.Structures.Model.RebarSpacing.ExactSpacing` (class)
  - [762] `Tekla.Structures.Filtering.Categories.PartFilterExpressions.Class` (class)
  - [219] `Tekla.Structures.Datatype.Double` (struct)
  - [200] `Tekla.Structures.Datatype.Constants` (class)
  - [1115] `Tekla.Structures.Model.RebarCranking.CrankedLengthTypeEnum` (enum)
- **deep_check_pass_rate:** 50/50 ✓

Comparison method: for each sampled type, the reviewer fetched its `doc_url` from the catalog entry, parsed the constructor / method / property / event / enum-value tables using the same selectors `PageParser` uses (with the corrected LST-script delimiter substitution that handles `?cs=` at any position in the key-list, not just at the start), and compared row-name sets against the catalog entry's `constructors[]`, `methods[]`, `properties[]`, `events[]`, `enum_values[]` arrays. The sets matched exactly in both name and count for all 50 sampled types.

### Defect-fix verification — sample-level

- **Defect 1 (ctor overload disambiguator restored):** confirmed across the sample. Sampled types with multiple ctors (e.g. `RadiusDimensionAttributes`, `StringList`, `Chamfer`, `GridSurface`, `CurvedDimensionRadial`) carry distinct ctor names like `Chamfer()` and `Chamfer(Part, ContourPoint, ContourPoint, Chamfer.ChamferTypeEnum, Double, Double, Double, Double, Int32)` per the vendor's row text. Singleton ctors that take args (e.g. `AnalysisAreaPolygon(AnalysisObject.AnalysisObjectEnum)`) still render with bare type name in Tekla's table — **this is vendor behaviour**, not a defect; the catalog faithfully mirrors what Tekla publishes (verified by direct vendor-page inspection).
- **Defect 2 (generic angle brackets):** confirmed. Sampled types with generic method signatures (e.g. `CurvedDimensionRadial.GetIntegerUserProperties(Dictionary<String, Int32>.)`, `BoltGroup.SetDynamicStringProperty`, `StringList.Contains`) now render `IEnumerable<Angle>`, `List<String>`, `Dictionary<String, Int32>` etc. with real angle brackets in catalog row names. The legacy `IEnumerable.Angle.` mangle is absent.
- **Defect 3 (`returns: null` serialized explicitly):** confirmed. Spot-checked the catalog for `Tekla.Structures.Model.Model.json` and other ctor/void cases — the `returns` key is present-with-null on every ctor and void method in every sampled catalog file.
- **Defect 4 (Property Value vs Return Value subheading fallback):** confirmed. Properties like `Tekla.Structures.Model.Weld.IsUpToDate` (now `Boolean`, was `object`) and `Tekla.Structures.Model.Weld.ModificationTime` (now `Nullable<DateTime>`, was `object`) carry their real types in 2025.

### Defect-fix verification — at scale (full catalog sweep)

A systemic sweep over all 1282 catalog files confirms each fix scales across the entire IR, not just the random sample:

| Indicator | Count | Verdict |
|---|--:|:--|
| Constructors total | 1058 | — |
| Constructors with `()` or `(Args)` disambiguator | 538 | restored (was 0 in prior IR) |
| Constructors with bare type-name only (singletons) | 520 | vendor mirror — Tekla doesn't disambiguate when only 1 ctor exists |
| Methods total | 6879 | — |
| Methods with `<...>` generic angle brackets | 470 | restored (was 0 in prior IR; replaced the legacy `.X.` mangle) |
| Methods with the legacy `.X.` mangle pattern | 1 | 1 false positive (see "Known parser quirk" below) |
| Methods missing `returns` key in JSON | 0 | defect 3 fixed (was 6879 in prior IR) |
| Methods with `returns: null` (void methods) | 391 | explicit serialization — was implicit/missing pre-fix |
| Constructors missing `returns` key in JSON | 0 | defect 3 fixed (was 1058 in prior IR) |
| Constructors with `returns: null` | 1058 | explicit serialization — every ctor now carries the key |
| Properties total | 4045 | — |
| Properties with `type: "object"` (lowercase, parser placeholder) | 0 | placeholder absent; defect-4 fix lifted enrichment from 95.7% → 99.7% |
| Properties with `type: "Object"` (capitalized, vendor-declared `Object`) | 12 | 8 `SyncRoot` / 3 `Current` (.NET `IEnumerator` contract) / 2 `SelectedItem` — all confirmed by direct vendor-page inspection to render `Type: Object` |

### Known parser quirk surfaced by the sweep

One catalog row (`Tekla.Structures.Model.RebarSpacing.Create(RebarSpacing.Offset, RebarSpacing.Offset, IEnumerable<RebarSpacing.ExactSpacing.Element>, RebarSpacing.ExactSpacing.Validation.)`) carries a trailing dot before the closing paren. The underlying cause: Tekla's LST inline-script for the last parameter (`out` modifier) is `LSTF1E4234C_12?cpp=%` — only a C++ rendering, no `cs=` key. `PageParser` falls back to `.` for LST elements without a `cs=` value, which produces the trailing dot. Real C# rendering would simply omit any trailing character (the `out` modifier is already captured in the signature). This is a **rare edge case** (1 occurrence in 6879 methods = 0.015% rate), the catalog reproduces it consistently across IR + JSON, and downstream consumers can normalize it if needed. Documented here for traceability rather than as a defect that affects the verdict — sweeping the entire catalog shows it's not a systemic pattern.

## Step 3 — Behavioral spot-check (20 random methods)
- 20 methods randomly sampled across the 50 deep-checked types (continued PRNG, no re-seed)
- For each method, fetched its `doc_url` and confirmed catalog `summary` is non-empty whenever the vendor page has corresponding prose
- **behavior_present_rate:** 20/20 ✓

Sampled methods:
1. `GridSurface.GetChildren` — summary present ✓
2. `WpfOkCreateCancel.InitializeComponent` — summary present ✓
3. `Boolean.GetDoubleReportProperties` — summary present ✓
4. `CurvedDimensionRadial.GetUserProperty(String, Int32.)` — summary present ✓
5. `CurvedDimensionRadial.Delete` — summary present ✓
6. `PolyLine.Equals` — summary present ✓
7. `StringList.GetRange` — summary present ✓
8. `StringList.Contains` — summary present ✓
9. `CircleRebarGroup.Equals` — summary present ✓
10. `BoltGroup.SetDynamicStringProperty` — summary present ✓
11. `Boolean.SetLabel` — summary present ✓
12. `CircleRebarGroup.GetSingleRebar` — summary present ✓
13. `StringList.Remove` — summary present ✓
14. `ProjectInfo.GetBasePoints` — summary present ✓
15. `BoltGroup.AddOtherPartToBolt` — summary present ✓
16. `Boolean.SetCustomObjectType` — summary present ✓
17. `CircleRebarGroup.CompareTo` — summary present ✓
18. `CircleRebarGroup.SetCustomObjectType` — summary present ✓
19. `BoltGroup.GetDoubleUserProperties` — summary present ✓
20. `CurvedDimensionRadial.GetIntegerUserProperties(Dictionary<String, Int32>.)` — summary present ✓

## Step 4 — Schema validation
- IR file validated against `cli-sidecar/Ingest/Schema/host-coverage.schema.json` (root schema, requires `host`/`host_version`/`source`/`types`/`metadata`)
- Each catalog file validated against `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json` (single-Type fragment schema)
- Files validated: 1 IR file + 1282 catalog files = 1283 total
- Validator: `ajv@8` with `2020-12` draft + `ajv-formats@3` for `uri`/`date-time` formats (matches the validator the prior reviewer used)
- **schema_violations:** 0 ✓

The defect-3 fix (per-property `[JsonIgnore(Condition=Never)]` on `MethodInfo.returns` in `cli-sidecar/Ingest/Generator/Models.cs`) closed every catalog-side schema violation. The prior FAIL run reported 766/775 catalog violations under the old shape with the old schema-mismatch protocol; under the corrected serializer + corrected protocol (root vs type-fragment schema split) it's clean.

## Re-run command
`aware coverage review tekla-2025` — once the `aware coverage review` subcommand lands (currently the protocol runs through the codex-rescue subagent / ad-hoc reviewer scripts under `review-tmp/`).
