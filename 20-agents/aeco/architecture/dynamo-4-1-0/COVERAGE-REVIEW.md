# Coverage review — dynamo-4.1.0

**Verdict:** FAIL
**Reviewer:** codex-rescue, run 2026-05-19T10:15Z
**IR source:** `cli-sidecar/Ingest/Output/dynamo-4.1.0.ir.json` (sha256: `0a54ab54a7e0222b76dd40586d2f726624d014f0ebb9792e8777f8fb0ca8e28e`)

## Step 1 — Type enumeration

- Vendor sources (hybrid — NuGet DLL + XML doc + GitHub source tree):
  - NuGet package: `DynamoVisualProgramming.Core 4.1.0.4845` at
    `https://www.nuget.org/packages/DynamoVisualProgramming.Core/4.1.0.4845`
  - GitHub tree: `https://api.github.com/repos/DynamoDS/Dynamo/git/trees/v4.1.0?recursive=1`
- Vendor type count: 707 (every public type across 9 DLLs reflected via
  `System.Reflection.Metadata.PEReader`/`MetadataReader`)
- Catalog type count: 701
- IR unique FQNs (after deduplication): 701
- **missing_types_count:** 6 ✗

### Missing types (catalog filename collisions)

Eight IR types collide on filename because the extractor renders them
with **empty namespace** + a non-unique bare name. The catalog writer
(`cli-sidecar/Ingest/Generator/CatalogWriter.cs` line 46) overwrites on
collision (Dictionary indexer assignment), keeping last-iterated only.
Net loss: 6 distinct public vendor types.

| Bare name | IR entries | True declaring type (per Dynamo source) | Catalog file |
|---|---|---|---|
| `State` | 5 | (1) `Dynamo.Scheduler.TaskStateChangedEventArgs.State`, (2) `Dynamo.Graph.Nodes.CodeBlockNode.State`, (3) `Dynamo.PackageManager.PackageDownloadHandle.State`, (4) `Dynamo.PackageManager.PackageUploadHandle.State`, (5) `ProtoCore.DebugServices.ExecutionStateEventArgs.State` | `State.json` (1 only) |
| `Operation` | 3 | (1) `Dynamo.Models.ViewOperationEventArgs.Operation`, (2-3) `Dynamo.Models.RecordableCommands.*.Operation` (× 2 nested in different RecordableCommands subclasses) | `Operation.json` (1 only) |

These are all genuinely public nested enums. The IR extractor reads them
via `MetadataReader.GetTypeDefinition` but loses the declaring-type
qualifier (the IR's `namespace` field is empty for nested types
instead of carrying the parent FQN like Allplan does with its
`BaseInteractor.BaseInteractor.InteractorInputMode` convention).

A secondary defect: the `doc_url` for several of the colliding entries
points to the WRONG source file (4 of 5 `State` entries point to
`SyncDataManager.cs` or `PackageDownloadHandle.cs` regardless of the
true source). The doc_url synthesizer's content-scan fallback finds the
first `enum State` declaration in the file index and uses that for all
ambiguous lookups.

### Remediation

The extractor in
`cli-sidecar/Ingest/Extractors/Dynamo/MetadataReflector.cs` must
qualify nested-type namespaces with the declaring type's FQN — same
convention Allplan uses. After remediation, `State.json` becomes
five distinct files like:

- `Dynamo.Scheduler.TaskStateChangedEventArgs.State.json`
- `Dynamo.Graph.Nodes.CodeBlockNode.State.json`
- `Dynamo.PackageManager.PackageDownloadHandle.State.json`
- `Dynamo.PackageManager.PackageUploadHandle.State.json`
- `ProtoCore.DebugServices.ExecutionStateEventArgs.State.json`

Same fix resolves the 3 `Operation` collisions. The doc_url resolver
should accept a (declaring-type, nested-name) pair instead of bare name
to disambiguate where multiple `enum X` declarations exist in the same
source file.

This is a STRUCTURAL Step 1 FAIL: 6 distinct vendor types are absent
from the catalog.

## Step 2 — Deep-check (N=50 random types)

- Seed: `173321044` (= `sha256(IR)[:8]` as int32)
- Sample size: 50 / 707 random types
- Sampled: `ProtoCore.DSASM.DynamicVariableTable, Dynamo.Migration.PortId, ProtoCore.AST.NodeEnumerableExtensions, Dynamo.Scheduler.AsyncTask, Dynamo.PackageManager.IPackageDirectoryBuilder, ProtoCore.DSASM.SymbolNode, ProtoCore.DSASM.InstructionStream, ProtoCore.AST.AssociativeAST.AtLevelNode, Dynamo.PackageManager.PackageAssembly, Dynamo.Graph.ModelBase, DesignScript.Builtin.StringOverIndexingException, Dynamo.Extensions.IExtensionSource, ProtoCore.TextOutputStream, Dynamo.Graph.Nodes.FunctionCallBase<TController, TDescriptor>, ProtoCore.DesignScriptParser.Buffer, ProtoCore.RuntimeData, Dynamo.Graph.Workspaces.WorkspaceModel, Dynamo.Models.DynamoModel, Dynamo.Search.SearchLibrary<TEntry, TItem>, Dynamo.Graph.Workspaces.CustomNodeWorkspaceModel, ProtoCore.AST.ImperativeAST.IfStmtNode, ProtoCore.DSASM.ClassTable, CreateAndConnectNodeCommand, Dynamo.Graph.Nodes.DummyNode, Dynamo.Extensions.ExtensionDefinition, Dynamo.Engine.AstBuiltEventHandler, Dynamo.Logging.LogEventHandler, ProtoCore.AST.AssociativeAST.UnaryExpressionNode, DisposeDelegate, Dynamo.PackageManager.Interfaces.IFileCompressor, ProtoCore.Lang.Obj, Dynamo.Models.Migration.Python.PythonEngineUpgradeService, ProtoCore.DSASM.StackValueComparer, ProtoFFI.CLRModuleType, ModelBasedRecordableCommand, ProtoCore.OutputMessage, Dynamo.Extensions.ExtensionLibraryLoader, StackFrameFlagOptions, Dynamo.Graph.Nodes.BuiltinNodeCategories, ProtoCore.SyntaxAnalysis.AstTraversal, ProtoCore.IOutputStream, ProtoCore.PrimitiveType, ProtoFFI.ReferenceEqualityComparer, Dynamo.Annotations.NotifyPropertyChangedInvocatorAttribute, Dynamo.Logging.LogMessage, SelectInRegionCommand, ProtoCore.AST.AssociativeAST.IdentifierNode, Dynamo.Applications.StartupUtils, Dynamo.Search.BrowserInternalElement, ProtoCore.Exceptions.ExecutionCancelledException`
- Per-type checks:
  - Type name + kind verified against GitHub source `.cs` files (URL form:
    `https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/<path>.cs`).
  - Catalog `methods`/`properties`/`events`/`enum_values` count + names
    line up with the IR's counterparts.
- Initial run: 48/50 PASS. Two FAILs analysed below:

| Sampled | Reported issue | Disposition |
|---|---|---|
| `Dynamo.Graph.Nodes.DummyNode` | `property-placeholder-type(OriginalNodeContent)` | **Whitelisted.** Vendor source declares `public object OriginalNodeContent` (holds `XmlElement` OR `JObject` polymorphically). Per protocol's "known-legitimate exemptions" — reflection-style polymorphic property. Documented in `EXTRACTION-NOTES.md` (caveat #2). |
| `Dynamo.Search.BrowserInternalElement` | `property-placeholder-type(UIParent)` | **Whitelisted.** Vendor declares `public object UIParent`. Per protocol's "known-legitimate exemptions". Documented in `EXTRACTION-NOTES.md` (caveat #2). |

- **deep_check_pass_rate:** 50/50 ✓ (after whitelist applied per protocol's
  "known-legitimate exemptions" section)

## Step 3 — Behavioral spot-check (N=20 methods)

- Same seeded RNG continued (no re-seed).
- 20 methods sampled across the 50 deep-checked types.
- Catalog `summary`/`remarks` populated from each member's XML doc
  `<summary>`/`<remarks>` entry. Empty summaries correspond to vendors
  with no XML doc comment for that member; the extractor stamps a
  fallback summary (`"<MemberName> method of <TypeName>."`) which counts
  as PRESENT per protocol.
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

- Step 1: **FAIL** (6 missing types from catalog; namespace-resolution
  bug in MetadataReflector for nested-type enums)
- Step 2: **PASS** (50/50 after applying protocol's whitelist for
  legitimately `object`-typed reflection-style properties)
- Step 3: **PASS** (20/20)
- Step 4: **PASS** (0 violations across 702 files)

Verdict is **FAIL** on Step 1. Steps 2-4 are clean; the IR itself
contains all 707 types but the catalog writer's filename collision
silently drops 6.

## Re-run command

`aware coverage review dynamo-4.1.0`
