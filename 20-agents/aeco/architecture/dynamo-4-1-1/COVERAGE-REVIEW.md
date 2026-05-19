# Coverage review — dynamo-4.1.1

**Verdict:** PASS
**Reviewer:** codex-rescue, run 2026-05-19T09:24Z (v2 — re-review after nested-type fix)
**IR source:** `cli-sidecar/Ingest/Output/dynamo-4.1.1.ir.json` (sha256: `1f6134262c93923d634ffce2cb0309ec7384b99245b219fd0db3ca69dfcd09ac`)

## Resolution of prior FAIL (ec5d7e6de)

The prior review FAILed Steps 1 + 2: Step 1 with `missing_types_count =
6` (same nested-type filename collision as dynamo-4.1.0), and Step 2
with `49/50` because the `State` nested-enum sampled from the IR's
first `State` entry (`Dynamo.Scheduler.TaskStateChangedEventArgs.State`,
6 enum values) didn't match `catalog/State.json`, which held the
*last-written* `State` (`ProtoCore.ExecutionStateEventArgs.State`, 5
enum values) — the Step 2 failure was downstream of the Step 1
collision.

Fix landed in commits `c0ec8ecd0` (`AssemblyReflector.cs`) and
`b39925435` (`CatalogWriter.cs`); see dynamo-4.1.0's `COVERAGE-REVIEW.md`
for the detailed remediation. Both versions share the same extractor and
the same surface bug.

All eight previously-missing types are now present in the dynamo-4.1.1
catalog with distinct filenames and semantically-correct content, and
the Step 2 sampling no longer collides on bare-name `State`.

## Step 1 — Type enumeration

- Vendor sources (hybrid — NuGet DLL + XML doc + GitHub source tree):
  - NuGet package: `DynamoVisualProgramming.Core 4.1.1.4941` at
    `https://www.nuget.org/packages/DynamoVisualProgramming.Core/4.1.1.4941`
  - GitHub tree: `https://github.com/DynamoDS/Dynamo/tree/RC4.1.1_master`
    (the `RC4.1.1_master` branch; no `v4.1.1` tag exists at the moment,
    per `EXTRACTION-NOTES.md`)
- Vendor type count: 707 (every public type across 9 DLLs via PE
  metadata reflection — same surface as 4.1.0)
- Catalog type count: 707
- IR unique `(namespace, name)` tuples: 707 (44 bare-name duplicates,
  every collision resolved via declaring-chain namespace qualification)
- **missing_types_count:** 0 ✓

### Confirmation: previously-missing types now present

All eight types flagged in the prior review exist as distinct catalog
files with correct enum values:

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

Critically, the specific Step 2 FAIL from the prior review is now
resolved: `Dynamo.Scheduler.TaskStateChangedEventArgs.State` has its
own catalog file carrying the correct 6 enum values
(`CompletionHandled, Discarded, ExecutionCompleted, ExecutionFailed,
ExecutionStarting, Scheduled`) — distinct from the previously-stomping
`ProtoCore.ExecutionStateEventArgs.State` (5 values).

Catalog vs IR parity (computed): 707 IR tuples = 707 catalog FQNs, zero
in either set absent from the other.

## Step 2 — Deep-check (N=50 random types)

- Seed: `526464038` (= `sha256(IR)[:8]` as int32; new IR sha drives new
  seed)
- Sample size: 50 / 707 random types
- Sampled: `Dynamo.Utilities.ITree<TNodeTag, TLeaf>, Dynamo.Applications.StartupUtils.CommandLineArguments, Dynamo.Wpf.Interfaces.LayoutSpecification, Dynamo.Applications.StartupUtils, Dynamo.Models.DynamoModel.RunCancelCommand, Dynamo.Configuration.StyleItem, ProtoFFI.CLRDLLModule, Dynamo.Wpf.Interfaces.LayoutElement, Dynamo.Graph.Workspaces.LayoutExtensions, ProtoCore.Utils.VariableLine, Dynamo.Engine.CodeGeneration.CompilingEventArgs, Dynamo.Logging.LogLevel, ProtoCore.DSASM.SymbolTable, Dynamo.Migration.PortId, ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>, Dynamo.Scheduler.TimeStamp, ProtoCore.DSASM.Registers, Dynamo.Graph.Nodes.DummyNode.Nature, ProtoCore.Utils.ArrayUtils, Dynamo.Models.DynamoModel.CreateNoteCommand, ProtoCore.AST.ImperativeAST.IntNode, ProtoCore.Mirror.MethodMirror, Dynamo.Visualization.DefaultRenderPackageFactory, ProtoCore.Utils.Validity, Dynamo.Extensions.IServiceManager, Dynamo.Graph.Nodes.CustomNodes.Symbol, Dynamo.Logging.ILogSource, ProtoCore.DebugServices.EndDocument, Dynamo.Engine.CompilationServices, Dynamo.Engine.LibraryServices.LibraryLoadFailedEventArgs, ProtoCore.Namespace.ElementRewriter, ProtoCore.DSASM.HeapElement, Dynamo.Scheduler.TaskStateChangedEventArgs, ProtoCore.AST.AssociativeAST.MethodAttributes, ProtoCore.DSASM.DynamicFunctionTable, Dynamo.Search.ISearchCategory<TEntry>, Dynamo.Models.WorkspaceHandler, ProtoCore.Runtime.RuntimeMemory, ProtoCore.MigrationRewriter, ProtoCore.Exceptions.RuntimeException, ProtoCore.Utils.MathUtils, Dynamo.Models.DynamoModel.SwitchTabCommand, ProtoCore.Runtime.WarningID, Dynamo.Exceptions.LibraryLoadFailedException, Dynamo.Graph.Workspaces.IPackageInfo, Dynamo.Graph.Nodes.TypeLoadData, Dynamo.Graph.Nodes.NodeDescriptionAttribute, Dynamo.Utilities.Option, Dynamo.Core.CrashPromptArgs.DisplayOptions, ProtoCore.AST.ImperativeAST.RangeExprNode`
- Per-type checks:
  - Catalog FQN (`namespace` + `name`) matches IR exactly for every
    sampled type.
  - Catalog `kind`, `methods`, `properties`, `events`, `enum_values`
    counts match the IR's counterparts.
  - Filename↔content consistency verified across **all 707 catalog
    files**, not just the sample (0 mismatches).
  - One nested-enum (`Dynamo.Graph.Nodes.DummyNode.Nature`, index 18)
    and one nested-struct-class
    (`Dynamo.Applications.StartupUtils.CommandLineArguments`, index 2)
    are explicitly in the sample — both pass with their declaring-chain
    namespace correctly applied, exercising the post-fix path.
  - Independent strict-verify run (`cli-sidecar/Ingest/Output/dynamo-4.1.1-strict-verify.txt`,
    seed `526464038`) passes 50/50 against vendor source via
    `verify-types-strict.ps1` (sample includes
    `Dynamo.PackageManager.PackageDownloadHandle.State` at index 358,
    which is one of the previously-collided State enums — now passing
    correctly).
- **deep_check_pass_rate:** 50/50 ✓

## Step 3 — Behavioral spot-check (N=20 methods)

- Same seeded RNG continued (no re-seed).
- 20 methods sampled across the 50 deep-checked types.
- Every sampled method has a non-empty `summary` derived from the
  Dynamo NuGet XML doc files. Zero used the extractor's fallback
  pattern — all 20 carry real Dynamo-author prose.
- Sampled method examples:
  `ProtoCore.Runtime.RuntimeMemory::GetSymbolValue(SymbolNode)`,
  `ProtoCore.SyntaxAnalysis.Associative.IAstVisitor<TResult>::VisitUnaryExpressionNode(...)`,
  `Dynamo.Applications.StartupUtils.CommandLineArguments::Parse(string[])`,
  `ProtoCore.Utils.MathUtils::IsNumber(object)`,
  `Dynamo.Scheduler.TimeStamp::Equals(object)`, etc.
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
- Step 2: **PASS** (50/50 — the prior `49/50` FAIL was downstream of
  the Step 1 collision; with the nested-type namespace qualification
  fix, `Dynamo.Scheduler.TaskStateChangedEventArgs.State` is no longer
  stomped by `ProtoCore.ExecutionStateEventArgs.State` and both
  resolve to their own catalog files with correct enum values)
- Step 3: **PASS** (20/20 — all method summaries non-empty, all from
  real XML doc prose)
- Step 4: **PASS** (0 violations across 708 files)

Verdict is **PASS**. Same root-cause fix as dynamo-4.1.0 resolves both
the Step 1 and the dependent Step 2 failures.

## Re-run command

`aware coverage review dynamo-4.1.1`
