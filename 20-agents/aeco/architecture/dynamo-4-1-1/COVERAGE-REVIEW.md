# Coverage review — dynamo-4.1.1

**Verdict:** FAIL
**Reviewer:** codex-rescue, run 2026-05-19T10:15Z
**IR source:** `cli-sidecar/Ingest/Output/dynamo-4.1.1.ir.json` (sha256: `970372f3802691415b434e47666f78ec705dea2458db6fe364f7049b81c34b51`)

## Step 1 — Type enumeration

- Vendor sources (hybrid — NuGet DLL + XML doc + GitHub source tree):
  - NuGet package: `DynamoVisualProgramming.Core 4.1.1.4941` at
    `https://www.nuget.org/packages/DynamoVisualProgramming.Core/4.1.1.4941`
  - GitHub tree: `https://api.github.com/repos/DynamoDS/Dynamo/git/trees/v4.1.1?recursive=1`
- Vendor type count: 707 (every public type across 9 DLLs via PE
  metadata reflection — same surface as 4.1.0)
- Catalog type count: 701
- IR unique FQNs (after deduplication): 701
- **missing_types_count:** 6 ✗

### Missing types (catalog filename collisions) — same as 4.1.0

Eight IR types collide on filename due to empty-namespace + non-unique
bare-name combination. Same root cause + same affected types as
`dynamo-4.1.0`:

| Bare name | IR entries | True declaring type | Catalog file |
|---|---|---|---|
| `State` | 5 | (1) `Dynamo.Scheduler.TaskStateChangedEventArgs.State`, (2) `Dynamo.Graph.Nodes.CodeBlockNode.State`, (3) `Dynamo.PackageManager.PackageDownloadHandle.State`, (4) `Dynamo.PackageManager.PackageUploadHandle.State`, (5) `ProtoCore.DebugServices.ExecutionStateEventArgs.State` | `State.json` (1 only) |
| `Operation` | 3 | (1) `Dynamo.Models.ViewOperationEventArgs.Operation`, (2-3) `Dynamo.Models.RecordableCommands.*.Operation` (× 2 nested) | `Operation.json` (1 only) |

Net loss: 6 distinct public vendor nested-enum types from the catalog.

### Remediation

See `dynamo-4.1.0/COVERAGE-REVIEW.md` — same MetadataReflector fix
applies (qualify nested-type namespaces with declaring-type FQN).
This is the same surface bug in both versions; a single extractor fix
resolves both.

## Step 2 — Deep-check (N=50 random types)

- Seed: `386102003` (= `sha256(IR)[:8]` as int32)
- Sample size: 50 / 707 random types
- Sampled: `UploadType, Dynamo.Graph.Nodes.Variable, Dynamo.Graph.Nodes.ScopedNodeModel, SingleRunTraceData, Dynamo.Logging.ILogMessage, ProtoCore.DSASM.OpCode, DynamoModelState, ProtoCore.SyntaxAnalysis.ImperativeAstVisitor<TResult>, Dynamo.Scheduler.AsyncTask, Dynamo.Engine.FunctionDescriptorParams, DynamoUtilities.XmlHelper, ProtoCore.AST.AssociativeAST.LanguageBlockNode, State, CreateAnnotationCommand, Dynamo.Graph.Nodes.NodeModelExtensions, Dynamo.Search.SearchElements.DragDropNodeSearchElementInfo, Dynamo.Configuration.ViewExtensionSettings, Dynamo.Visualization.DefaultRenderPackageFactory, Dynamo.Search.LuceneSearch, Dynamo.Graph.Nodes.FunctionCallBase<TController, TDescriptor>, Dynamo.Search.SearchElements.NodeModelSearchElement, Dynamo.Graph.Nodes.InPortDescriptionsAttribute, Dynamo.Migration.WorkspaceMigrationAttribute, Dynamo.Graph.Nodes.ZeroTouch.ZeroTouchVarArgNodeController<T>, ProtoCore.AST.ImperativeAST.InlineConditionalNode, Dynamo.Engine.IFunctionDescriptor, ProtoCore.CallSite, ProtoCore.Lang.Obj, ProtoCore.AST.ImperativeAST.IdentifierNode, StatementType, ProtoCore.AST.AssociativeAST.DoubleNode, Dynamo.Core.CrashPromptArgs, Dynamo.Graph.ConnectorPinModel, ProtoCore.Mirror.MirrorObject, DynamoInstallDetective.InstalledProductLookUp, ProtoCore.AST.AssociativeAST.DynamicBlockNode, Dynamo.Graph.ILocatable, Dynamo.PackageManager.Interfaces.IFileCompressor, ProtoCore.DSASM.MetaData, ProtoCore.Exceptions.ReplicationCaseNotCurrentlySupported, ProtoCore.Lang.ContinuationStructure, DynamoUtilities.PathHelper, Dynamo.Utilities.Point2D, ProtoCore.Runtime.WarningID, ProtoCore.CompileTime.Context, Dynamo.Graph.Workspaces.WorkspaceReadConverter, Dynamo.Wpf.Interfaces.LayoutSpecification, ProtoCore.Properties.Resources, Dynamo.Graph.SaveContext, Dynamo.Engine.Profiling.IProfilingExecutionTimeData`
- Per-type checks:
  - Type name + kind verified against GitHub source `.cs` files (URL form:
    `https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/<path>.cs`
    — extractor uses the `RC4.1.1_master` branch ref since no `v4.1.1`
    tag exists at the moment, per the `EXTRACTION-NOTES.md`).
  - Catalog `methods`/`properties`/`events`/`enum_values` count + names
    line up with the IR's counterparts.
- Initial run: 49/50 PASS. One FAIL analysed below:

| Sampled | Reported issue | Disposition |
|---|---|---|
| `State` (empty namespace) | `enum-values-count-mismatch(ir=6,catalog=5)` | **Symptom of Step 1 defect.** The IR's first `State` entry has 6 enum values (`Dynamo.Scheduler.TaskStateChangedEventArgs.State` — `CompletionHandled,Discarded,ExecutionCompleted,ExecutionFailed,ExecutionStarting,Scheduled`), but `catalog/State.json` holds the *last-written* `State` (the ProtoCore `ExecutionStateEventArgs.State` with 5 values: `ExecutionBegin,ExecutionBreak,ExecutionEnd,ExecutionResume,Invalid`). Counted as FAIL — the catalog has lost the sampled type's data. |

- **deep_check_pass_rate:** 49/50 ✗ (the 1 FAIL is downstream of the
  Step 1 filename collision; the underlying IR data is intact)

## Step 3 — Behavioral spot-check (N=20 methods)

- Same seeded RNG continued (no re-seed).
- 20 methods sampled across the 50 deep-checked types.
- Catalog `summary`/`remarks` populated from XML doc / extractor
  fallback. Behavior PRESENT counts the fallback summary as
  documented per protocol.
- **behavior_present_rate:** 20/20 ✓

## Step 4 — Schema validation

- IR validated against `cli-sidecar/Ingest/Schema/host-coverage.schema.json`
  via `aware coverage validate` → status `ok`.
- All 701 catalog files validated against
  `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json` (Draft 2020-12)
  via `jsonschema 4.26.0`.
- Files validated: 702 (701 catalog + 1 IR)
- **schema_violations:** 0 ✓

## Reviewer summary

- Step 1: **FAIL** (6 missing types from catalog — same root cause as
  4.1.0)
- Step 2: **FAIL** (49/50 — the 1 FAIL is downstream of Step 1's
  filename collision; the catalog's `State.json` contains the wrong
  variant for the sampled type)
- Step 3: **PASS** (20/20)
- Step 4: **PASS** (0 violations across 702 files)

Verdict is **FAIL** on Steps 1 + 2.

## Re-run command

`aware coverage review dynamo-4.1.1`
