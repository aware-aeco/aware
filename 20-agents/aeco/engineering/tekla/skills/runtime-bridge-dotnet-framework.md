---
name: tekla-runtime-bridge-dotnet-framework
description: This skill should be used when writing or modifying any code that talks to a running Tekla Structures instance — sidecars, bridges, plugins, ROT-bind helpers, anything that loads Tekla.Structures.Model.dll or related assemblies at runtime. Encodes the hard rule that Tekla's runtime DLLs target .NET Framework 4.x and the patterns required to load them correctly into an external process.
---

# Tekla runtime bridge — .NET Framework rule

**Tekla Open API runtime DLLs target .NET Framework 4.x.** Even Tekla 2026 (which has moved many internal components to modern .NET) keeps the public Open API surface on classic Framework. The image runtime version on `Tekla.Structures.Model.dll` reads `v4.0.30319` — confirm any time you doubt this with:

```powershell
[Reflection.Assembly]::ReflectionOnlyLoadFrom('C:\Program Files\Tekla Structures\<ver>\bin\Tekla.Structures.Model.dll').ImageRuntimeVersion
```

## The rule

**Any code that calls into Tekla at runtime MUST target `net48` or older.** Not `net9.0`. Not `net10.0`. Not netstandard with a hope-and-prayer compat shim. The .NET Framework runtime is what Tekla's connection-to-running-Tekla machinery (.NET Remoting, COM interop) was built against; modern .NET can sometimes load classic-framework DLLs but Tekla's IPC paths break silently.

**Symptom of the violation:** `Model.GetConnectionStatus()` or the first `Operation.*` call throws a bare `FileNotFoundException` with no path. The DLL loaded fine; the connection machinery failed.

```csharp
// WRONG — sidecar targets net9.0
// <TargetFramework>net9.0</TargetFramework>
// Result: FileNotFoundException from inside Model.GetConnectionStatus()

// RIGHT — sidecar targets net48
// <TargetFramework>net48</TargetFramework>
// Result: Connection attaches; Operation.DisplayPrompt() lands in status bar
```

## The dual-runtime layout (Tekla 2026+)

Tekla 2026 ships **two copies** of each Open API DLL:

```
C:\Program Files\Tekla Structures\2026.0\bin\
├── Tekla.Structures.Model.dll          # .NET 8/9 build (internal Tekla)
├── Tekla.Structures.Datatype.dll       # .NET 8/9 build
└── Net48Runtime\
    ├── Tekla.Structures.dll            # .NET Framework 4.8 build  ← THIS one
    ├── Tekla.Structures.Model.dll      # .NET Framework 4.8 build
    └── Tekla.Structures.Datatype.dll   # .NET Framework 4.8 build
```

The `bin/` copy targets the modern .NET runtime Tekla now uses internally. The `Net48Runtime/` copy is what external Open API consumers need. **Probe both directories in your AssemblyResolve handler, prefer `Net48Runtime/` when present.**

Tekla 2025 and earlier only have `bin/` (single runtime). Probe code must handle both layouts.

## AssemblyResolve handler pattern

Wire the handler in `Main` **before** any Tekla type is referenced — including from indirect calls. Move Tekla-using code into a separate method marked `[MethodImpl(MethodImplOptions.NoInlining)]` so the JIT cannot pre-load Tekla types when JIT-ing Main itself:

```csharp
using System;
using System.IO;
using System.Reflection;
using System.Runtime.CompilerServices;

class Program
{
    static string _teklaBinDir = "";

    static int Main(string[] args)
    {
        _teklaBinDir = DiscoverTeklaBin();   // process enum or argv
        AppDomain.CurrentDomain.AssemblyResolve += ResolveTeklaAssembly;
        return RunWithTekla(args);            // separate method, see below
    }

    [MethodImpl(MethodImplOptions.NoInlining)]   // <- the load-bearing attribute
    static int RunWithTekla(string[] args)
    {
        // First Tekla type reference here, AFTER AssemblyResolve is wired.
        var model = new Tekla.Structures.Model.Model();
        // ...
        return 0;
    }

    static Assembly? ResolveTeklaAssembly(object? sender, ResolveEventArgs e)
    {
        var name = new AssemblyName(e.Name).Name;
        if (string.IsNullOrEmpty(name)) return null;
        foreach (var probe in new[]
        {
            Path.Combine(_teklaBinDir, "Net48Runtime"),   // 2026+ first
            _teklaBinDir,                                  // 2025 fallback
        })
        {
            var dll = Path.Combine(probe, $"{name}.dll");
            if (File.Exists(dll)) return Assembly.LoadFrom(dll);
        }
        return null;
    }
}
```

## Why the JIT-inlining trick matters

If you put `new Tekla.Structures.Model.Model()` inline in `Main`, the JIT compiles `Main` as one block. While doing so it eagerly resolves all referenced types — and Tekla types need their assemblies loaded BEFORE your `AssemblyResolve` handler is even registered, because the handler registration is itself one of the IL instructions in `Main`. The result: failure during JIT-compile of `Main`, before any of your code runs.

`[MethodImpl(MethodImplOptions.NoInlining)]` forces the JIT to compile the Tekla-using method lazily, on first call — at which point your `AssemblyResolve` handler has already been registered.

## Build-time vs runtime

This rule is **only** about runtime. The build-time AWARE sidecar (`cli-sidecar/`) targets `.NET 9` because it does metadata-only reflection via `MetadataLoadContext` — it never executes managed code from Tekla DLLs, just reads the IL signatures. Don't conflate them. Build-time = `net9.0`; runtime = `net48`.

## Tekla's IPC wire protocol — what you'll see in errors

When the Open API attaches, it uses **Trimble.Remoting** — Trimble's in-house .NET Remoting fork that runs over **memory-mapped-file (MMF) named pipes**, not classic TCP. Useful to know because the error messages you'll see during debugging directly name the IPC primitives:

| Channel name pattern | What it is |
|---|---|
| `Tekla.Structures.Model-TeklaStructures-Console:<full-version>` (e.g. `:2026.0.59721.0`) | The Open API client ↔ Tekla model channel |
| `Trimble.Remoting.IO.MmfPipeStream` | The pipe stream class |
| `Trimble.Remoting.IO.PipeClient` | Client-side wrapper |

**Diagnostic translation table:**

| Error you see | What it actually means |
|---|---|
| `FileNotFoundException` from inside `Model.GetConnectionStatus()` (no path) | .NET runtime mismatch — you're on net9, switch to net48 |
| `FileNotFoundException` with path `<dll>` from `AssemblyResolve` | Your resolver candidate doesn't exist; check `Net48Runtime/` subdir |
| `RemotingIOException: Cannot connect to remoting service ...` wrapping `FileNotFoundException` from `MemoryMappedFile.OpenExisting` | Tekla process is running but hasn't created its IPC MMF yet — you're polling too early, wait for `GetConnectionStatus() == true` |
| `MissingMethodException` on `Operation.DisplayPrompt` etc. | Version mismatch — loaded a DLL from wrong Tekla version |

The MMF-pipe detail matters for two reasons:
1. **You can't probe the IPC from outside the loaded DLL.** No tool like `tcping` will tell you "Tekla is ready" — only the actual API call attempt does. Hence the polling pattern in [headless-startup-and-shutdown.md](headless-startup-and-shutdown.md).
2. **MMF channels are PID-scoped.** Each Tekla process registers its own MMF with the channel name including its version. Two different versions = two different channels = no collision. Same-version-two-instances would collide on the MMF name — which is part of why Tekla refuses to start a second instance of the same version.

## Likely applies to other classic-framework hosts

The same rule probably applies to anything Trimble / Autodesk shipped before they migrated to modern .NET internally:

- **Revit Open API** — classic .NET Framework, all add-ins target net48
- **AutoCAD ObjectARX .NET wrappers** — classic .NET Framework
- **Navisworks .NET API** — classic .NET Framework

Verify per host before writing the sidecar. Don't assume.
