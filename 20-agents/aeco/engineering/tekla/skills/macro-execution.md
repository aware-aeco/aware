---
name: tekla-macro-execution
description: Tekla Open API - Macro and command execution. RunMacro, MacroBuilder, numbering, export operations (IFC, DWG, NC files, reports). Run internal Tekla commands with dotStartAction. Check macro status with IsMacroRunning. This skill should be used when running macros, automating numbering, exporting models, or executing Tekla internal commands.
---

# Tekla Macro and Command Execution (v2025)

## Critical Patterns

- All macro methods are static on `Tekla.Structures.Model.Operations.Operation`
- **MUST call `TeklaStructures.Connect()` before any macro dispatch** (RunMacro / MacroBuilder.Run / dotStartAction). Without it the macro silently no-ops, IsMacroRunning() returns false instantly, and any side-effect file (e.g. a saved attributes .j-file) never appears on disk. No exception is raised — pure silent failure. `using Tekla.Structures;` to bring `TeklaStructures` into scope.
- **Dynamically-generated macros MUST be written into a directory listed in the `XS_MACRO_DIRECTORY` advanced option** — `<modelPath>/macros` is NOT on the search path by default. Resolve the directory at runtime via `TeklaStructuresSettings.GetAdvancedOption("XS_MACRO_DIRECTORY", ref value)` and write to the first writable entry. Don't hardcode `2025.0` or any version-specific path.
- `RunMacro()` executes asynchronously — use `IsMacroRunning()` to wait for completion
- **`Operation.RunMacro` returns `Boolean`** — ALWAYS capture and check the return value. `false` means the macro file could not be resolved/loaded (typically because it isn't on `XS_MACRO_DIRECTORY`). Don't fire-and-forget; you'll have no signal beyond "the side-effect didn't happen" which is much harder to debug
- Macro filenames are without extension, Tekla resolves the path
- `MacroBuilder` provides a fluent API for building macro commands
- Use `dotStartAction()` for internal Tekla commands not exposed as API methods

## Writing Dynamic Macros to XS_MACRO_DIRECTORY

```csharp
using Tekla.Structures;

// Read the macro search path advanced option (semicolon-separated list).
// Typical value: "C:\\ProgramData\\Trimble\\Tekla Structures\\<ver>\\environments\\common\\macros;C:\\Program Files\\Tekla Structures\\<ver>\\bin\\Env\\Common\\Macros"
string macroDirSetting = "";
TeklaStructuresSettings.GetAdvancedOption("XS_MACRO_DIRECTORY", ref macroDirSetting);

// Pick the first existing + writable entry. ProgramData is normally writable;
// Program Files is normally NOT.
string targetDir = null;
foreach (var dir in (macroDirSetting ?? "").Split(new[] { ';' }, StringSplitOptions.RemoveEmptyEntries))
{
    var trimmed = dir.Trim();
    if (Directory.Exists(trimmed) is false) continue;
    try
    {
        var probe = Path.Combine(trimmed, ".__write_probe__");
        File.WriteAllText(probe, "");
        File.Delete(probe);
        targetDir = trimmed;
        break;
    }
    catch { /* not writable — try next */ }
}

if (targetDir is null)
{
    // Surface this as a real error — don't silently fall back to <modelPath>/macros
    // because Operation.RunMacro can't find files there and will silently no-op.
    throw new InvalidOperationException("No writable XS_MACRO_DIRECTORY entry found.");
}

// Write the .cs macro into targetDir, then dispatch by basename:
File.WriteAllText(Path.Combine(targetDir, "MyDynamicMacro.cs"), macroSource);
Operation.RunMacro("MyDynamicMacro.cs");   // Tekla resolves via XS_MACRO_DIRECTORY
```

## Connect Handshake (REQUIRED before any macro dispatch)

```csharp
using Tekla.Structures;                   // for TeklaStructures
using Tekla.Structures.Model;             // for Model
using Tekla.Structures.Model.Operations;  // for Operation.RunMacro / IsMacroRunning

var model = new Model();
if (model.GetConnectionStatus() is false)
{
    // Model isn't loaded — abort.
    return;
}

// Macro-runtime handshake. Distinct from GetConnectionStatus(): that confirms
// the model is open; this primes the macro dispatcher. Skipping it makes
// every subsequent Operation.RunMacro / MacroBuilder.Run silently no-op.
if (TeklaStructures.Connect() is false)
{
    // Macro infrastructure unavailable — abort.
    return;
}

// Now safe to dispatch:
Operation.RunMacro("MyCustomMacro");
while (Operation.IsMacroRunning()) { System.Threading.Thread.Sleep(100); }
```

## Run a Macro

```csharp
// Operation.RunMacro returns Boolean — ALWAYS capture it. False means Tekla
// could not resolve the file against XS_MACRO_DIRECTORY (or otherwise failed
// to load the macro). Without this check the only signal that something went
// wrong is "the side-effect file never appeared", which is hard to diagnose.
var dispatched = Operation.RunMacro("MyCustomMacro.cs");
if (dispatched is false)
{
    throw new InvalidOperationException(
        "Operation.RunMacro returned false — file not on XS_MACRO_DIRECTORY search path?");
}

// Wait for macro to complete
while (Operation.IsMacroRunning())
{
    System.Threading.Thread.Sleep(100);
}
```

## MacroBuilder Fluent API

```csharp
// REQUIRED handshake — without this, macroBuilder.Run() silently no-ops.
if (TeklaStructures.Connect() is false) return;

// Build and run a macro programmatically
var macroBuilder = new MacroBuilder();
macroBuilder.Callback("acmd_display_dialog", "", "main_frame");
macroBuilder.ValueChange("main_frame", "gr_sel_view", "1");
macroBuilder.PushButton("dia_view_zoom_to_fit", "main_frame");
macroBuilder.Run();
```

## Run Numbering

```csharp
// Modified parts numbering
Operation.RunMacro("run_modified_objects_numbering");

// Wait for completion
while (Operation.IsMacroRunning())
{
    System.Threading.Thread.Sleep(100);
}

// Full numbering of all objects
Operation.RunMacro("run_all_objects_numbering");
```

## Export Operations

```csharp
// Export current model to IFC
Operation.RunMacro("ExportIFC");

// Export drawings to DWG
Operation.RunMacro("ExportDrawingDWG");

// Create reports (use dedicated API, not RunMacro)
// Reports go to "Reports" folder under current model path by default
var model = new Model();
var modelPath = model.GetInfo().ModelPath;
var reportsFolder = Path.Combine(modelPath, "Reports");
Directory.CreateDirectory(reportsFolder); // Ensure folder exists

// Report from all objects
var outputPath = Path.Combine(reportsFolder, "AssemblyList.xsr");
Operation.CreateReportFromAll("Assembly_list", outputPath, "Title", "", "");

// Report from selected objects only
var selectedOutputPath = Path.Combine(reportsFolder, "SelectedParts.xsr");
Operation.CreateReportFromSelected("Part_list", selectedOutputPath, "Title", "", "");

// Display the report in Tekla's viewer
Operation.DisplayReport(outputPath);

// Export NC files
Operation.RunMacro("ExportNCFiles");
```

> **Note:** Report creation requires `using System.IO;` for `Path.Combine` and `Directory.CreateDirectory`.

## Internal Commands (dotStartAction)

**Important:** `dotStartAction` lives on `Tekla.Structures.ModelInternal.Operation`, not the public `Tekla.Structures.Model.Operations.Operation`. Use the fully qualified name to avoid ambiguity.

```csharp
// Run internal Tekla action commands
// Requires: using Tekla.Structures.ModelInternal; (or fully qualify)
Tekla.Structures.ModelInternal.Operation.dotStartAction("ZoomToSelected", "");

// Open a dialog
Tekla.Structures.ModelInternal.Operation.dotStartAction("Open3DView", "");

// Refresh model views
Tekla.Structures.ModelInternal.Operation.dotStartAction("RedrawAllViews", "");
```

## Check Macro Status

```csharp
// Check if any macro is currently running
if (Operation.IsMacroRunning())
{
    // Wait or handle busy state
}
```
