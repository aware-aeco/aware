# Coverage review — dynamo-4.1.0

**Verdict:** PASS
**Reviewer:** codex-rescue, run 2026-05-19T09:24Z (v2 — re-review after nested-type fix)
**IR source:** `cli-sidecar/Ingest/Output/dynamo-4.1.0.ir.json` (sha256: `74bdc564129aded25526f07bd86afd3554aa85722c2ab37461615aa8c0292cfc`)

## Resolution of prior FAIL (ca4c007b4)

The prior review FAILed Step 1 with `missing_types_count = 6` because the
extractor emitted nested types with an empty IR `namespace` field — five
distinct `State` enums and three distinct `Operation` enums collapsed
onto the same catalog filename (`State.json`, `Operation.json`) and were
silently overwritten by `CatalogWriter`.

Fix landed in commits `c0ec8ecd0` (`AssemblyReflector.cs`) and
`b39925435` (`CatalogWriter.cs`):

- Nested-type IR records now carry `namespace = "<declaring-chain>"`
  (Allplan convention), so each nested enum lands in its own catalog
  file (e.g. `Dynamo.Graph.Nodes.Statement.State.json`).
- `CatalogWriter.WriteFile` now throws `InvalidOperationException` on
  filename collision so this class of silent data loss can never recur.

All eight previously-missing types are now present with distinct
filenames and semantically-correct content (verified below).

## Step 1 — Type enumeration

- Vendor sources (hybrid — NuGet DLL + XML doc + GitHub source tree):
  - NuGet package: `DynamoVisualProgramming.Core 4.1.0.4845` at
    `https://www.nuget.org/packages/DynamoVisualProgramming.Core/4.1.0.4845`
  - GitHub tree: `https://github.com/DynamoDS/Dynamo/tree/v4.1.0`
- Vendor type count: 707 (every public type across 9 DLLs reflected via
  `System.Reflection.Metadata.PEReader`/`MetadataReader`)
- Catalog type count: 707
- IR unique `(namespace, name)` tuples: 707 (44 bare-name duplicates,
  every collision resolved via declaring-chain namespace qualification)
- **missing_types_count:** 0 ✓

### Confirmation: previously-missing types now present

All eight types flagged in the prior review (5 `State` + 3 `Operation`
nested enums) exist as distinct catalog files with correct enum values
and declaring-chain namespaces:

| Catalog file | IR `namespace` | IR `name` | enum values |
|---|---|---|---|
| `Dynamo.Scheduler.TaskStateChangedEventArgs.State.json` | `Dynamo.Scheduler.TaskStateChangedEventArgs` | `State` | `CompletionHandled, Discarded, ExecutionCompleted, ExecutionFailed, ExecutionStarting, Scheduled` |
| `Dynamo.Graph.Nodes.Statement.State.json` | `Dynamo.Graph.Nodes.Statement` | `State` | `Error, Normal, Warning` |
| `Dynamo.PackageManager.PackageDownloadHandle.State.json` | `Dynamo.PackageManager.PackageDownloadHandle` | `State` | `Downloaded, Downloading, Error, Installed, Installing, Uninitialized` |
| `Dynamo.PackageManager.PackageUploadHandle.State.json` | `Dynamo.PackageManager.PackageUploadHandle` | `State` | `Compressing, Copying, Error, Ready, Uploaded, Uploading` |
| `ProtoCore.ExecutionStateEventArgs.State.json` | `ProtoCore.ExecutionStateEventArgs` | `State` | `ExecutionBegin, ExecutionBreak, ExecutionEnd, ExecutionResume, Invalid` |
| `Dynamo.Models.ViewOperationEventArgs.Operation.json` | `Dynamo.Models.ViewOperationEventArgs` | `Operation` | `FitView, ZoomIn, ZoomOut` |
| `Dynamo.Models.DynamoModel.DragSelectionCommand.Operation.json` | `Dynamo.Models.DynamoModel.DragSelectionCommand` | `Operation` | `BeginDrag, EndDrag` |
| `Dynamo.Models.DynamoModel.UndoRedoCommand.Operation.json` | `Dynamo.Models.DynamoModel.UndoRedoCommand` | `Operation` | `Redo, Undo` |

Spot-checked against vendor source via WebFetch:

- `https://raw.githubusercontent.com/DynamoDS/Dynamo/v4.1.0/src/DynamoPackages/PackageDownloadHandle.cs` →
  `PackageDownloadHandle.State` has values `Uninitialized, Downloading,
  Downloaded, Installing, Installed, Error` (same set as catalog).
- `https://raw.githubusercontent.com/DynamoDS/Dynamo/v4.1.0/src/DynamoCore/Models/DynamoModelEventArgs.cs` →
  `ViewOperationEventArgs.Operation` has values `FitView, ZoomIn,
  ZoomOut` (same set as catalog).
- `https://raw.githubusercontent.com/DynamoDS/Dynamo/v4.1.0/src/DynamoCore/Models/RecordableCommands.cs` →
  `DragSelectionCommand.Operation` has `BeginDrag, EndDrag` and
  `UndoRedoCommand.Operation` has `Undo, Redo` (same sets as catalog).

Catalog vs IR parity (computed): 707 IR tuples = 707 catalog FQNs, zero
in either set absent from the other.

## Step 2 — Deep-check (N=50 random types)

- Seed: `1958593892` (= `sha256(IR)[:8]` as int32; new IR sha drives new
  seed)
- Sample size: 50 / 707 random types
- Sampled: `ProtoCore.CallSite, Dynamo.Graph.Nodes.NodeLoaders.INodeFactory<T>, Dynamo.Graph.Nodes.ZeroTouch.DSFunction, ProtoCore.AST.AssociativeAST.AssociativeNode, Dynamo.Graph.Nodes.PortType, Dynamo.Migration.NodeMigrationAttribute, Dynamo.Logging.WarningLevel, ProtoCore.AST.AssociativeAST.AstExtensions, ProtoCore.DesignScriptParser.Scanner, ProtoCore.AST.ImperativeAST.BooleanNode, ProtoCore.DSASM.Mirror.NameNotFoundException, Dynamo.Configuration.GroupStyleItem, Dynamo.Models.DynamoModel.AddModelToGroupCommand, DesignScript.Builtin.StringOverIndexingException, Dynamo.PackageManager.PackageDownloadHandle, Dynamo.Models.DynamoPreferencesData, Dynamo.PackageManager.PackageManagerClient, Dynamo.Utilities.OrderedSet<T>, Dynamo.Graph.Nodes.NodeDescriptionAttribute, Dynamo.Utilities.TypeExtensions, Dynamo.Graph.Nodes.Statement.State, ProtoCore.SyntaxAnalysis.AstReplacer, DynamoShapeManager.Utilities, ProtoCore.AST.AssociativeAST.CommentNode, Dynamo.Configuration.LuceneConfig.NodeFieldsEnum, ProtoCore.DSASM.SymbolNode, Dynamo.PackageManager.LoadPackageParams, ProtoCore.Mirror.RuntimeMirror, ProtoCore.Core.ErrorType, ProtoFFI.PInvokeModuleHelper, Dynamo.Utilities.XmlElementHelper, Dynamo.Scheduler.TaskStateChangedEventArgs, ProtoCore.ExecutionStateEventArgs.State, Dynamo.Configuration.GraphChecksumItem, ProtoCore.DSASM.DSArray, ProtoCore.SyntaxAnalysis.AssociativeAstReplacer, Dynamo.Graph.ILocatable, Dynamo.Engine.FunctionDescriptorParams, Dynamo.Graph.Nodes.VariableInputNodeController, Dynamo.Models.DynamoModel.UpdateModelValueCommand, Dynamo.Logging.DynamoLogger, Dynamo.Utilities.GuidUtility, Dynamo.PackageManager.IFileSystem, Dynamo.Core.IDSDKManager, Dynamo.Engine.ITraceReconciliationProcessor, Dynamo.Scheduler.AsyncTaskExtensions, Dynamo.Models.DynamoModel.ModelBasedRecordableCommand, ProtoFFI.CLRModuleType, Dynamo.Models.DynamoModel, Dynamo.Models.DynamoModel.RunCompletedHandler`
- Per-type checks:
  - Catalog FQN (`namespace` + `name`) matches IR exactly for every
    sampled type.
  - Catalog `kind`, `methods`, `properties`, `events`, `enum_values`
    counts match the IR's counterparts (catalog is generated from the
    same IR, so this is a tautology — proves no extraction-time data
    loss between IR and catalog).
  - Filename↔content consistency verified across **all 707 catalog
    files**, not just the sample (0 mismatches).
  - Two previously-flagged types are explicitly in the sample
    (`Dynamo.Graph.Nodes.Statement.State` at index 21,
    `ProtoCore.ExecutionStateEventArgs.State` at index 33) and both
    pass — confirming the nested-type fix lands the right data.
  - Independent strict-verify run (`cli-sidecar/Ingest/Output/dynamo-4.1.0-strict-verify.txt`,
    seed `1958593892`) passes 50/50 against vendor source via
    `verify-types-strict.ps1` using source-code lowercase-kind-word
    recognition.
- **deep_check_pass_rate:** 50/50 ✓

## Step 3 — Behavioral spot-check (N=20 methods)

- Same seeded RNG continued (no re-seed).
- 20 methods sampled across the 50 deep-checked types.
- Every sampled method has a non-empty `summary` derived from the
  Dynamo NuGet XML doc files. Zero used the extractor's fallback summary
  pattern (`"<Member> method of <Type>."`) — all 20 carry real
  Dynamo-author prose from `<summary>` tags.
- Sampled method examples:
  `ProtoCore.AST.AssociativeAST.AstExtensions::ToImperativeNode(...)`,
  `Dynamo.Models.DynamoModel::InsertFileFromPath(string, bool)`,
  `Dynamo.Utilities.OrderedSet<T>::Overlaps(IEnumerable<T>)`,
  `Dynamo.Core.IDSDKManager::GetAccessToken`, etc.
- **behavior_present_rate:** 20/20 ✓

## Step 4 — Schema validation

- IR validated against `cli-sidecar/Ingest/Schema/host-coverage.schema.json`
  via Python `jsonschema 4.26.0` (Draft 2020-12) → 0 errors.
- All 707 catalog files validated against
  `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json`
  (Draft 2020-12) → 0 errors.
- Files validated: 708 (707 catalog + 1 IR)
- **schema_violations:** 0 ✓

## Reviewer summary

- Step 1: **PASS** (707/707 — the 6 previously-missing types are now
  in the catalog with semantically-correct content)
- Step 2: **PASS** (50/50 deep-check — both previously-fixed nested
  `State` enums explicitly hit the sample and both pass)
- Step 3: **PASS** (20/20 — all method summaries non-empty, all from
  real XML doc prose)
- Step 4: **PASS** (0 violations across 708 files)

Verdict is **PASS**. The nested-type namespace qualification fix
resolves the root cause and the `CatalogWriter` collision-detection
guard prevents the class of defect from recurring silently.

## Re-run command

`aware coverage review dynamo-4.1.0`
