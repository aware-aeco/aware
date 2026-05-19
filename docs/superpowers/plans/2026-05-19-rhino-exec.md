# rhino.exec Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ship the v0.32 `aware-rhino` runtime sidecar — a .NET 10 single-file binary that wraps McNeel's `rhinocode` CLI to give AWARE an `exec / list-instances / send-status` surface against a user-running Rhino 8.

**Architecture:** AWARE-shaped `{verb,code,args}` stdin JSON → `aware-rhino.exe` wraps the user's C# in a temp `.cs` file with a synthetic `__AwareRun(args)` local function so `return X;` works → shells out to `rhinocode script <tempfile> --rhino <pipe>` → captures sentinel-marked result from stdout → emits `{ok,result,host,verb,delivered_at}` receipt identical in shape to tekla.exec.

**Tech Stack:** C# / .NET 10 (net10.0-windows — current LTS, matches the SDK installed on this machine; original spec said net8 but the build machine has only .NET 9 and 10), xUnit + Microsoft.NET.Test.Sdk for tests, System.Text.Json for envelope. No third-party NuGets. Single-file self-contained publish (`win-x64`, ReadyToRun).

**Reference:** Full design in [docs/superpowers/specs/2026-05-19-rhino-exec-design.md](../specs/2026-05-19-rhino-exec-design.md). Tekla analog at [cli-tekla/Program.cs](../../../cli-tekla/Program.cs).

**Important constraints:**
- Rhino 8 is NOT installed on the build machine. Every test in this plan runs WITHOUT a live Rhino — integration tests use a stub `rhinocode.cmd` on PATH. The live 20-prompt drill is documented for the user to run in the morning.
- No `Co-Authored-By: Claude` trailers in commit messages.
- Stage specific files (`git add <path>`); never `git add -A`.
- All work happens on the existing branch `feature/v0.29-runtime-hello-world`.

---

## File structure

```
cli-rhino/
├── cli-rhino.csproj
├── Program.cs
├── Receipts.cs
├── ScriptWrapper.cs
├── RhinocodeClient.cs
├── Verbs/
│   ├── Exec.cs
│   ├── ListInstances.cs
│   └── SendStatus.cs
├── Ingest/
│   ├── run-drill.ps1
│   └── Output/
│       └── prompt-01.json ... prompt-20.json
└── Tests/
    ├── cli-rhino.Tests.csproj
    ├── ScriptWrapperTests.cs
    ├── ReceiptsTests.cs
    ├── RhinocodeClientTests.cs
    └── stub-rhinocode/
        ├── rhinocode.cmd          # Windows stub for integration tests
        └── canned-responses/
            ├── list-ok.json
            └── script-ok.txt
```

Plus:
- `20-agents/aeco/architecture/rhino-8/manifest.yaml` — patched (add `transport.cli.binary: aware-rhino` and an `exec` command entry).
- `docs/superpowers/handoffs/2026-05-19-v032-rhino-exec-ready.md` — handoff doc.

---

## Task 1 — Scaffold the project

**Files:**
- Create: `cli-rhino/cli-rhino.csproj`
- Create: `cli-rhino/Program.cs` (minimal stub — just `--help`)
- Create: `cli-rhino/.gitignore` (one-liner for `bin/` `obj/`)

- [ ] **Step 1: Write `cli-rhino/cli-rhino.csproj`**

```xml
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net10.0-windows</TargetFramework>
    <Nullable>enable</Nullable>
    <ImplicitUsings>enable</ImplicitUsings>
    <RootNamespace>AwareRhino</RootNamespace>
    <AssemblyName>aware-rhino</AssemblyName>
    <LangVersion>latest</LangVersion>
    <TreatWarningsAsErrors>true</TreatWarningsAsErrors>
    <InvariantGlobalization>true</InvariantGlobalization>
  </PropertyGroup>
  <PropertyGroup Condition="'$(Configuration)' == 'Release'">
    <PublishSingleFile>true</PublishSingleFile>
    <IncludeNativeLibrariesForSelfExtract>true</IncludeNativeLibrariesForSelfExtract>
    <PublishReadyToRun>true</PublishReadyToRun>
    <SelfContained>true</SelfContained>
    <RuntimeIdentifier>win-x64</RuntimeIdentifier>
  </PropertyGroup>
</Project>
```

- [ ] **Step 2: Write `cli-rhino/Program.cs`** (minimal — full impl arrives in Task 8)

```csharp
// aware-rhino — runtime sidecar that wraps McNeel's rhinocode CLI for the AWARE
// runtime. Speaks the {verb,code,args} stdin-JSON contract shared with cli-tekla.

namespace AwareRhino;

internal static class Program
{
    static int Main(string[] args)
    {
        Console.InputEncoding  = new System.Text.UTF8Encoding(false);
        Console.OutputEncoding = new System.Text.UTF8Encoding(false);

        if (args.Length == 0 || args[0] is "--help" or "-h")
        {
            PrintHelp();
            return 0;
        }

        Console.Error.WriteLine($"aware-rhino: verb dispatch not yet wired (got '{args[0]}'). Coming in Task 8.");
        return 2;
    }

    static void PrintHelp()
    {
        Console.WriteLine("""
            aware-rhino — Rhino 8 sidecar (wraps McNeel's rhinocode CLI)

            Usage:
              aware-rhino <verb> [--json-stdin]
              aware-rhino --json-stdin            (verb embedded in JSON body)

            Verbs:
              exec             Compile + run an ad-hoc C# script against the active Rhino doc
              list-instances   Print running Rhino instances (PID + version + active doc)
              send-status      Display a transient status-bar message in Rhino

            Prerequisites:
              - Rhino 8.11+ installed (rhinocode.exe must be discoverable)
              - In Rhino, run `StartScriptServer` once per session
              - For multi-instance routing, pass `rhino_id` in the input JSON
            """);
    }
}
```

- [ ] **Step 3: Write `cli-rhino/.gitignore`**

```
bin/
obj/
*.user
```

- [ ] **Step 4: Verify it builds**

Run: `cd cli-rhino && dotnet build -c Debug`
Expected: build succeeds, no warnings.

- [ ] **Step 5: Verify help runs**

Run: `dotnet run --project cli-rhino -- --help`
Expected: prints the help text.

- [ ] **Step 6: Commit**

```bash
git add cli-rhino/cli-rhino.csproj cli-rhino/Program.cs cli-rhino/.gitignore
git commit -m "feat(v0.32): scaffold cli-rhino .NET 10 sidecar project"
```

---

## Task 2 — Receipts.cs (envelope shape)

**Files:**
- Create: `cli-rhino/Receipts.cs`

The receipt shape is identical to tekla.exec's output. One file, no tests yet — tested implicitly by Task 6.

- [ ] **Step 1: Write `cli-rhino/Receipts.cs`**

```csharp
// Receipt envelope used by every aware-rhino verb. Shape is identical to
// cli-tekla's receipts so downstream orchestrators see the same JSON across
// vendors. Compact (no indentation) for transport efficiency.

using System.Text.Json.Nodes;

namespace AwareRhino;

internal static class Receipts
{
    public static JsonObject ExecOk(object? result, string? hostVersion, int? hostPid,
                                    string? rhinoId, string stdoutLog)
    {
        return new JsonObject
        {
            ["ok"]            = true,
            ["result"]        = SerializeForReceipt(result),
            ["host"]          = "rhino",
            ["host_version"]  = hostVersion,
            ["host_pid"]      = hostPid,
            ["rhino_id"]      = rhinoId,
            ["verb"]          = "exec",
            ["stdout_log"]    = stdoutLog,
            ["delivered_at"]  = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject ExecFail(string error, string stack, string stdoutLog)
    {
        return new JsonObject
        {
            ["ok"]            = false,
            ["error"]         = error,
            ["stack"]         = stack,
            ["host"]          = "rhino",
            ["verb"]          = "exec",
            ["stdout_log"]    = stdoutLog,
            ["delivered_at"]  = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject ListOk(JsonArray instances)
    {
        return new JsonObject
        {
            ["status"]       = "ok",
            ["instances"]    = instances,
            ["host"]         = "rhino",
            ["verb"]         = "list-instances",
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject SendStatusOk(string message, string? hostVersion, int? hostPid)
    {
        return new JsonObject
        {
            ["status"]          = "ok",
            ["host"]            = "rhino",
            ["host_version"]    = hostVersion,
            ["host_pid"]        = hostPid,
            ["host_session_id"] = hostPid is { } pid ? $"rhino-{pid}" : null,
            ["verb"]            = "send-status",
            ["verb_result"]     = new JsonObject { ["message"] = message },
            ["delivered_at"]    = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject GenericFail(string verb, string error, string? stack = null)
    {
        var o = new JsonObject
        {
            ["status"]       = "err",
            ["host"]         = "rhino",
            ["verb"]         = verb,
            ["error"]        = error,
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
        if (stack is not null) o["stack"] = stack;
        return o;
    }

    // Pre-parsed result (from the script's sentinel block) is already a JsonNode.
    // For other paths the caller may hand us a raw primitive — keep it.
    static JsonNode? SerializeForReceipt(object? result)
    {
        return result switch
        {
            null         => null,
            JsonNode jn  => jn,
            string s     => JsonValue.Create(s),
            bool b       => JsonValue.Create(b),
            int i        => JsonValue.Create(i),
            long l       => JsonValue.Create(l),
            double d     => JsonValue.Create(d),
            _            => JsonValue.Create(result.ToString()),
        };
    }
}
```

- [ ] **Step 2: Verify it builds**

Run: `dotnet build cli-rhino/cli-rhino.csproj -c Debug`
Expected: build succeeds, no warnings.

- [ ] **Step 3: Commit**

```bash
git add cli-rhino/Receipts.cs
git commit -m "feat(v0.32): rhino receipt envelope shape (mirrors cli-tekla)"
```

---

## Task 3 — ScriptWrapper.cs + unit tests

**Files:**
- Create: `cli-rhino/ScriptWrapper.cs`
- Create: `cli-rhino/Tests/cli-rhino.Tests.csproj`
- Create: `cli-rhino/Tests/ScriptWrapperTests.cs`

This is the most logic-dense file. Pure functions, no I/O — fully testable without Rhino or rhinocode.

- [ ] **Step 1: Write the failing test file first (TDD)** — `cli-rhino/Tests/cli-rhino.Tests.csproj`

```xml
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <TargetFramework>net10.0-windows</TargetFramework>
    <Nullable>enable</Nullable>
    <ImplicitUsings>enable</ImplicitUsings>
    <IsPackable>false</IsPackable>
    <RootNamespace>AwareRhino.Tests</RootNamespace>
    <LangVersion>latest</LangVersion>
  </PropertyGroup>
  <ItemGroup>
    <PackageReference Include="Microsoft.NET.Test.Sdk" Version="17.11.1" />
    <PackageReference Include="xunit" Version="2.9.2" />
    <PackageReference Include="xunit.runner.visualstudio" Version="2.8.2" />
  </ItemGroup>
  <ItemGroup>
    <ProjectReference Include="..\cli-rhino.csproj" />
  </ItemGroup>
</Project>
```

Add `<InternalsVisibleTo>AwareRhino.Tests</InternalsVisibleTo>` to `cli-rhino.csproj` (so tests can reach internal classes). Edit cli-rhino.csproj to add inside `<PropertyGroup>` block:

```xml
<ItemGroup>
  <InternalsVisibleTo Include="AwareRhino.Tests" />
</ItemGroup>
```

- [ ] **Step 2: Write `cli-rhino/Tests/ScriptWrapperTests.cs`**

```csharp
using AwareRhino;
using Xunit;

namespace AwareRhino.Tests;

public class ScriptWrapperTests
{
    [Fact]
    public void SplitUsings_NoUsings_BodyOnly()
    {
        var (usings, body) = ScriptWrapper.SplitUsingsAndBody(
            "var x = 1;\nreturn x;");
        Assert.Empty(usings);
        Assert.Equal("var x = 1;\nreturn x;", body.Trim());
    }

    [Fact]
    public void SplitUsings_OneDirective()
    {
        var (usings, body) = ScriptWrapper.SplitUsingsAndBody(
            "using Rhino.Geometry;\nvar p = new Point3d(0,0,0);\nreturn p;");
        Assert.Single(usings);
        Assert.Contains("using Rhino.Geometry;", usings);
        Assert.DoesNotContain("using Rhino.Geometry", body);
    }

    [Fact]
    public void SplitUsings_MultipleDirectives()
    {
        var (usings, body) = ScriptWrapper.SplitUsingsAndBody(
            "using System.IO;\nusing Rhino;\nusing Rhino.Geometry;\nvar p = new Point3d(0,0,0);");
        Assert.Equal(3, usings.Count);
    }

    [Fact]
    public void SplitUsings_AliasDirective()
    {
        var (usings, body) = ScriptWrapper.SplitUsingsAndBody(
            "using Geo = Rhino.Geometry;\nvar p = new Geo.Point3d(0,0,0);");
        Assert.Single(usings);
        Assert.Contains("using Geo = Rhino.Geometry;", usings);
    }

    [Fact]
    public void SplitUsings_IgnoresUsingStatement()
    {
        // "using (var x = ...)" is a using-STATEMENT, not a directive. Keep it in body.
        var (usings, body) = ScriptWrapper.SplitUsingsAndBody(
            "using (var sw = new System.IO.StringWriter()) { sw.Write(\"x\"); }\nreturn null;");
        Assert.Empty(usings);
        Assert.Contains("using (var sw", body);
    }

    [Fact]
    public void Wrap_ProducesCompilableShape()
    {
        var wrapped = ScriptWrapper.Wrap(
            "using Rhino.Geometry;\nvar p = new Point3d(0,0,0);\nreturn new { x = p.X };");

        // Result must contain preamble sentinels and our __AwareRun shape.
        Assert.Contains("__AWARE_RESULT_BEGIN__", wrapped);
        Assert.Contains("__AWARE_RESULT_END__", wrapped);
        Assert.Contains("static object? __AwareRun(IDictionary<string, object?> args)", wrapped);
        Assert.Contains("var p = new Point3d(0,0,0);", wrapped);
        Assert.Contains("return new { x = p.X };", wrapped);
        // Preamble brings in the Rhino namespace already so user's "using Rhino.Geometry;"
        // should still appear once (deduped against preamble).
        var usingRhinoGeometryCount = System.Text.RegularExpressions.Regex.Matches(
            wrapped, @"^using Rhino\.Geometry;\s*$",
            System.Text.RegularExpressions.RegexOptions.Multiline).Count;
        Assert.Equal(1, usingRhinoGeometryCount);
    }

    [Fact]
    public void Wrap_DedupesPreambleUsings()
    {
        // User supplies a using that's already in the preamble (Rhino).
        var wrapped = ScriptWrapper.Wrap("using Rhino;\nreturn 1;");
        var usingRhinoCount = System.Text.RegularExpressions.Regex.Matches(
            wrapped, @"^using Rhino;\s*$",
            System.Text.RegularExpressions.RegexOptions.Multiline).Count;
        Assert.Equal(1, usingRhinoCount);
    }

    [Fact]
    public void Wrap_PreservesUserBodyVerbatim()
    {
        var body = "// my comment\nvar doc = Rhino.RhinoDoc.ActiveDoc;\nint n = doc.Objects.Count;\nreturn n;";
        var wrapped = ScriptWrapper.Wrap(body);
        Assert.Contains("// my comment", wrapped);
        Assert.Contains("int n = doc.Objects.Count;", wrapped);
        Assert.Contains("return n;", wrapped);
    }

    [Fact]
    public void Wrap_PlacesPreambleUsingsAtTop()
    {
        var wrapped = ScriptWrapper.Wrap("return 1;");
        var idxUsing = wrapped.IndexOf("using System;", StringComparison.Ordinal);
        var idxRun = wrapped.IndexOf("__AwareRun", StringComparison.Ordinal);
        Assert.True(idxUsing >= 0);
        Assert.True(idxRun > idxUsing, "using directives must precede the __AwareRun function");
    }

    [Fact]
    public void Wrap_TopLevelStatementsBeforeLocalFunctions()
    {
        // C# 9 top-level statements MUST come before any type/method declarations
        // (local functions at file bottom are allowed because they belong to the
        // synthetic Program.<Main>$). Top-level call site of __AwareRun must
        // appear before the function definition.
        var wrapped = ScriptWrapper.Wrap("return 1;");
        var idxCall = wrapped.IndexOf("__AwareRun(args)", StringComparison.Ordinal);
        var idxDefn = wrapped.IndexOf("static object? __AwareRun(", StringComparison.Ordinal);
        Assert.True(idxCall >= 0);
        Assert.True(idxDefn >= 0);
        Assert.True(idxCall < idxDefn, "call site must come before local function definition");
    }

    [Fact]
    public void Wrap_EmptyBodyStillCompiles()
    {
        var wrapped = ScriptWrapper.Wrap("");
        Assert.Contains("__AwareRun", wrapped);
        Assert.Contains("return null;", wrapped);  // safe default body
    }

    [Fact]
    public void Wrap_BodyWithoutReturnGetsImplicitNullReturn()
    {
        // If user body has no `return`, we still want __AwareRun to return null
        // rather than be a CS0161 (not all code paths return a value). Wrapper
        // appends a `return null;` if no top-level `return` keyword found.
        var wrapped = ScriptWrapper.Wrap("Console.WriteLine(\"hi\");");
        // Verify there IS a return null somewhere inside __AwareRun.
        // Naive substring check: the wrapper appends a final `return null;` if absent.
        Assert.Contains("Console.WriteLine(\"hi\");", wrapped);
        // Count return statements in the synthetic function — at least 1.
        var runFnStart = wrapped.IndexOf("static object? __AwareRun(", StringComparison.Ordinal);
        var afterFn = wrapped.Substring(runFnStart);
        Assert.Contains("return null;", afterFn);
    }
}
```

- [ ] **Step 3: Confirm tests don't compile yet (ScriptWrapper doesn't exist)**

Run: `dotnet test cli-rhino/Tests/cli-rhino.Tests.csproj`
Expected: build FAILS with "The name 'ScriptWrapper' does not exist".

- [ ] **Step 4: Write `cli-rhino/ScriptWrapper.cs`** (minimal impl to pass tests)

```csharp
// Wraps the user's C# `code` (from {verb:"exec", code: "..."}) into a complete
// .cs file that rhinocode can execute. The wrapped file uses C# 9 top-level
// statements + a synthetic `__AwareRun(args)` local function so the user's
// body can use `return X;` semantics — same UX as cli-tekla's exec.

using System.Text;
using System.Text.RegularExpressions;

namespace AwareRhino;

internal static class ScriptWrapper
{
    // Matches `using <id>(.<id>)*;` and `using <alias> = <id>(.<id>)*;`
    // but NOT `using (var x = ...)` statement form.
    static readonly Regex UsingDirectiveRe = new(
        @"^\s*using\s+(?:[A-Za-z_][A-Za-z0-9_]*\s*=\s*)?[A-Za-z_][A-Za-z0-9_.]*\s*;\s*$",
        RegexOptions.Compiled);

    // Preamble brings in the standard Rhino + BCL namespaces. Dedup against
    // user-supplied directives is by exact string match (C# is case-sensitive).
    static readonly string[] PreambleUsings = new[]
    {
        "using System;",
        "using System.Collections.Generic;",
        "using System.Linq;",
        "using System.IO;",
        "using System.Text.Json;",
        "using System.Text.Json.Serialization;",
        "using Rhino;",
        "using Rhino.DocObjects;",
        "using Rhino.Geometry;",
        "using Rhino.Input;",
        "using Rhino.Input.Custom;",
        "using Rhino.PlugIns;",
        "using Rhino.UI;",
    };

    public static (List<string> usings, string body) SplitUsingsAndBody(string code)
    {
        var usings = new List<string>();
        var bodyLines = new List<string>();
        bool inDirectives = true;  // Stop collecting directives at first non-directive line.

        foreach (var line in code.Replace("\r\n", "\n").Split('\n'))
        {
            if (inDirectives && UsingDirectiveRe.IsMatch(line))
            {
                usings.Add(line.Trim());
            }
            else if (inDirectives && string.IsNullOrWhiteSpace(line))
            {
                // Whitespace between directives is allowed; stays in directives bucket.
                // We don't add it anywhere — body re-starts at first real statement.
            }
            else
            {
                inDirectives = false;
                bodyLines.Add(line);
            }
        }

        return (usings, string.Join("\n", bodyLines));
    }

    public static string Wrap(string userCode)
    {
        var (userUsings, body) = SplitUsingsAndBody(userCode);

        // Dedup: keep preamble usings; add only user usings that aren't already there.
        var preambleSet = new HashSet<string>(PreambleUsings, StringComparer.Ordinal);
        var extraUsings = userUsings.Where(u => !preambleSet.Contains(u)).Distinct().ToList();

        // Defensive: if body has no `return` keyword, append `return null;` so the
        // synthetic function doesn't fail CS0161. Naive keyword search is fine for
        // top-level snippets; nested `return`s inside lambdas count too — false-
        // negatives just append a dead `return null;` which is harmless.
        var bodyHasReturn = Regex.IsMatch(body, @"\breturn\b");
        var bodyWithReturn = bodyHasReturn ? body : (body.TrimEnd() + "\nreturn null;");
        if (string.IsNullOrWhiteSpace(body))
            bodyWithReturn = "return null;";

        var sb = new StringBuilder();
        sb.AppendLine("// AWARE-generated script (do not edit)");
        foreach (var u in PreambleUsings) sb.AppendLine(u);
        foreach (var u in extraUsings) sb.AppendLine(u);
        sb.AppendLine();
        sb.AppendLine("var __awareArgsJson = Environment.GetEnvironmentVariable(\"AWARE_ARGS_JSON\") ?? \"{}\";");
        sb.AppendLine("var args = JsonSerializer.Deserialize<Dictionary<string, object?>>(__awareArgsJson)");
        sb.AppendLine("           ?? new Dictionary<string, object?>();");
        sb.AppendLine();
        sb.AppendLine("var __result = __AwareRun(args);");
        sb.AppendLine("Console.WriteLine(\"__AWARE_RESULT_BEGIN__\");");
        sb.AppendLine("Console.WriteLine(JsonSerializer.Serialize(__result, new JsonSerializerOptions {");
        sb.AppendLine("    WriteIndented = false,");
        sb.AppendLine("    ReferenceHandler = ReferenceHandler.IgnoreCycles,");
        sb.AppendLine("    MaxDepth = 6,");
        sb.AppendLine("}));");
        sb.AppendLine("Console.WriteLine(\"__AWARE_RESULT_END__\");");
        sb.AppendLine();
        sb.AppendLine("static object? __AwareRun(IDictionary<string, object?> args)");
        sb.AppendLine("{");
        foreach (var line in bodyWithReturn.Split('\n'))
            sb.AppendLine("    " + line);
        sb.AppendLine("}");

        return sb.ToString();
    }

    // Result-sentinel markers (constants — used by Exec verb too).
    public const string ResultBeginSentinel = "__AWARE_RESULT_BEGIN__";
    public const string ResultEndSentinel   = "__AWARE_RESULT_END__";
}
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `dotnet test cli-rhino/Tests/cli-rhino.Tests.csproj -v normal`
Expected: 12 tests pass.

- [ ] **Step 6: Commit**

```bash
git add cli-rhino/ScriptWrapper.cs cli-rhino/Tests/cli-rhino.Tests.csproj cli-rhino/Tests/ScriptWrapperTests.cs cli-rhino/cli-rhino.csproj
git commit -m "feat(v0.32): ScriptWrapper splits usings, wraps body in __AwareRun() (+ 12 tests)"
```

---

## Task 4 — RhinocodeClient.cs + stub-binary integration tests

**Files:**
- Create: `cli-rhino/RhinocodeClient.cs`
- Create: `cli-rhino/Tests/RhinocodeClientTests.cs`
- Create: `cli-rhino/Tests/stub-rhinocode/rhinocode.cmd`
- Create: `cli-rhino/Tests/stub-rhinocode/canned-responses/list-ok.json`
- Create: `cli-rhino/Tests/stub-rhinocode/canned-responses/script-ok.txt`

- [ ] **Step 1: Write `cli-rhino/Tests/stub-rhinocode/rhinocode.cmd`** (Windows stub)

```cmd
@echo off
REM Stub rhinocode for AwareRhino integration tests. Echoes a canned response
REM based on the first argument: `list` → list-ok.json, `script` → script-ok.txt.
REM Honors `--rhino <id>` by recording it to STUB_LAST_RHINO_ID file for verification.

set "STUB_DIR=%~dp0canned-responses"

if "%1"=="list" (
    type "%STUB_DIR%\list-ok.json"
    exit /b 0
)

if "%1"=="script" (
    REM args after `script` may include `--rhino <id>` and a script path.
    set "RID="
    set "SCRIPTPATH="
    shift
    :argloop
    if "%~1"=="" goto donesargs
    if "%~1"=="--rhino" (
        set "RID=%~2"
        shift
        shift
        goto argloop
    )
    set "SCRIPTPATH=%~1"
    shift
    goto argloop
    :donesargs

    if not "%RID%"=="" echo %RID% > "%TEMP%\aware-rhino-stub-last-rhino-id.txt"
    if not "%SCRIPTPATH%"=="" echo %SCRIPTPATH% > "%TEMP%\aware-rhino-stub-last-script-path.txt"
    type "%STUB_DIR%\script-ok.txt"
    exit /b 0
)

echo stub-rhinocode: unknown verb '%1' 1>&2
exit /b 99
```

- [ ] **Step 2: Write canned responses**

`cli-rhino/Tests/stub-rhinocode/canned-responses/list-ok.json`:
```json
[
  {
    "pipeId": "rhinocode_remotepipe_75029",
    "processId": 31204,
    "processVersion": "8.13.24296.13001",
    "activeDoc": "C:\\Models\\hello.3dm"
  }
]
```

`cli-rhino/Tests/stub-rhinocode/canned-responses/script-ok.txt`:
```
some pre-result chatter from Rhino
__AWARE_RESULT_BEGIN__
{"count": 42}
__AWARE_RESULT_END__
trailing chatter
```

- [ ] **Step 3: Write `cli-rhino/Tests/RhinocodeClientTests.cs`**

```csharp
using System.IO;
using AwareRhino;
using Xunit;

namespace AwareRhino.Tests;

public class RhinocodeClientTests
{
    static string StubDir => Path.Combine(
        Path.GetDirectoryName(typeof(RhinocodeClientTests).Assembly.Location)!,
        "stub-rhinocode");

    static RhinocodeClient NewStubClient()
    {
        // Force discovery via env var override (RHINOCODE_EXE).
        return new RhinocodeClient(rhinocodeExeOverride: Path.Combine(StubDir, "rhinocode.cmd"));
    }

    [Fact]
    public void Discover_PrefersEnvOverride()
    {
        var c = NewStubClient();
        Assert.True(File.Exists(c.RhinocodeExePath));
        Assert.EndsWith("rhinocode.cmd", c.RhinocodeExePath);
    }

    [Fact]
    public void ListInstances_StubReturnsCannedJson()
    {
        var c = NewStubClient();
        var (exit, stdout, stderr) = c.RunListJson();
        Assert.Equal(0, exit);
        Assert.Contains("rhinocode_remotepipe_75029", stdout);
        Assert.Contains("\"processVersion\"", stdout);
    }

    [Fact]
    public void RunScript_PassesRhinoIdToStub()
    {
        var c = NewStubClient();
        var lastIdFile = Path.Combine(Path.GetTempPath(), "aware-rhino-stub-last-rhino-id.txt");
        if (File.Exists(lastIdFile)) File.Delete(lastIdFile);

        var tempScript = Path.Combine(Path.GetTempPath(), $"aware-rhino-test-{Guid.NewGuid():N}.cs");
        File.WriteAllText(tempScript, "// noop");

        var (exit, stdout, stderr) = c.RunScript(tempScript,
                                                 rhinoId: "rhinocode_remotepipe_75029",
                                                 argsJson: "{}");
        File.Delete(tempScript);

        Assert.Equal(0, exit);
        Assert.True(File.Exists(lastIdFile));
        var recordedId = File.ReadAllText(lastIdFile).Trim();
        Assert.Equal("rhinocode_remotepipe_75029", recordedId);
    }

    [Fact]
    public void RunScript_StdoutContainsSentinelBlock()
    {
        var c = NewStubClient();
        var tempScript = Path.Combine(Path.GetTempPath(), $"aware-rhino-test-{Guid.NewGuid():N}.cs");
        File.WriteAllText(tempScript, "// noop");

        var (exit, stdout, _) = c.RunScript(tempScript, rhinoId: null, argsJson: "{}");
        File.Delete(tempScript);

        Assert.Equal(0, exit);
        Assert.Contains(ScriptWrapper.ResultBeginSentinel, stdout);
        Assert.Contains(ScriptWrapper.ResultEndSentinel, stdout);
    }

    [Fact]
    public void Discover_FallsBackToStandardInstallPath_WhenEnvAndPathMiss()
    {
        // Simulate a machine without rhinocode on PATH and no env override.
        // The constructor takes a discovery delegate so we can inject a
        // fake "file-exists" probe.
        var c = new RhinocodeClient(
            rhinocodeExeOverride: null,
            fileExists: path => path == @"C:\Program Files\Rhino 8\System\rhinocode.exe",
            pathLookup: _ => null);
        Assert.Equal(@"C:\Program Files\Rhino 8\System\rhinocode.exe", c.RhinocodeExePath);
    }

    [Fact]
    public void Discover_Throws_WhenNothingFound()
    {
        var ex = Assert.Throws<FileNotFoundException>(() => new RhinocodeClient(
            rhinocodeExeOverride: null,
            fileExists: _ => false,
            pathLookup: _ => null));
        Assert.Contains("rhinocode", ex.Message);
    }
}
```

- [ ] **Step 4: Wire the stub files to copy into the test output directory** — edit `cli-rhino/Tests/cli-rhino.Tests.csproj` to add (inside the existing root `<Project>`):

```xml
<ItemGroup>
  <None Include="stub-rhinocode\**\*">
    <CopyToOutputDirectory>PreserveNewest</CopyToOutputDirectory>
  </None>
</ItemGroup>
```

- [ ] **Step 5: Confirm tests don't compile (RhinocodeClient missing)**

Run: `dotnet test cli-rhino/Tests/cli-rhino.Tests.csproj`
Expected: build FAILS with "RhinocodeClient does not exist".

- [ ] **Step 6: Write `cli-rhino/RhinocodeClient.cs`**

```csharp
// Discovers and shells out to McNeel's rhinocode CLI. Single place that knows
// about the binary's location and command-line shape. Tests inject overrides
// via the constructor to stub the binary or fake PATH lookups.

using System.Diagnostics;
using System.Text;

namespace AwareRhino;

internal sealed class RhinocodeClient
{
    public string RhinocodeExePath { get; }

    public RhinocodeClient(
        string? rhinocodeExeOverride = null,
        Func<string, bool>? fileExists = null,
        Func<string, string?>? pathLookup = null)
    {
        fileExists ??= File.Exists;
        pathLookup ??= WhereExe;

        // 1. Explicit override (RHINOCODE_EXE env var, or constructor arg from a test).
        var envOverride = rhinocodeExeOverride
            ?? Environment.GetEnvironmentVariable("RHINOCODE_EXE");
        if (!string.IsNullOrEmpty(envOverride) && fileExists(envOverride))
        {
            RhinocodeExePath = envOverride;
            return;
        }

        // 2. Standard install location.
        var stdPath = @"C:\Program Files\Rhino 8\System\rhinocode.exe";
        if (fileExists(stdPath))
        {
            RhinocodeExePath = stdPath;
            return;
        }

        // 3. PATH lookup.
        var found = pathLookup("rhinocode.exe") ?? pathLookup("rhinocode");
        if (!string.IsNullOrEmpty(found) && fileExists(found))
        {
            RhinocodeExePath = found;
            return;
        }

        throw new FileNotFoundException(
            "Could not find rhinocode.exe. Install Rhino 8.11+ (standard path: " +
            @"C:\Program Files\Rhino 8\System\rhinocode.exe), add it to PATH, or set RHINOCODE_EXE env var.");
    }

    public (int exitCode, string stdout, string stderr) RunListJson()
    {
        return Run(new[] { "list", "--json" }, argsJson: null);
    }

    public (int exitCode, string stdout, string stderr) RunScript(
        string scriptPath, string? rhinoId, string argsJson)
    {
        var args = new List<string>();
        // --rhino must come BEFORE the subcommand (it's a global flag).
        if (!string.IsNullOrEmpty(rhinoId))
        {
            args.Add("--rhino");
            args.Add(rhinoId!);
        }
        args.Add("script");
        args.Add(scriptPath);
        return Run(args.ToArray(), argsJson);
    }

    (int exitCode, string stdout, string stderr) Run(string[] args, string? argsJson)
    {
        var psi = new ProcessStartInfo
        {
            FileName               = RhinocodeExePath,
            UseShellExecute        = false,
            RedirectStandardOutput = true,
            RedirectStandardError  = true,
            CreateNoWindow         = true,
            StandardOutputEncoding = Encoding.UTF8,
            StandardErrorEncoding  = Encoding.UTF8,
        };
        foreach (var a in args) psi.ArgumentList.Add(a);

        // Inject AWARE_ARGS_JSON for the user's script to read. Always set (even
        // for `list`) — harmless when unused.
        psi.Environment["AWARE_ARGS_JSON"] = argsJson ?? "{}";

        using var p = Process.Start(psi)
            ?? throw new InvalidOperationException($"Process.Start returned null for {RhinocodeExePath}");
        var stdoutTask = p.StandardOutput.ReadToEndAsync();
        var stderrTask = p.StandardError.ReadToEndAsync();
        p.WaitForExit();
        return (p.ExitCode, stdoutTask.GetAwaiter().GetResult(), stderrTask.GetAwaiter().GetResult());
    }

    // Naive PATH lookup using `where` shipped with Windows.
    static string? WhereExe(string name)
    {
        try
        {
            var psi = new ProcessStartInfo
            {
                FileName               = "where",
                Arguments              = name,
                UseShellExecute        = false,
                RedirectStandardOutput = true,
                RedirectStandardError  = true,
                CreateNoWindow         = true,
            };
            using var p = Process.Start(psi);
            if (p is null) return null;
            var output = p.StandardOutput.ReadToEnd().Trim();
            p.WaitForExit();
            if (p.ExitCode != 0) return null;
            // `where` prints one path per line; first line wins.
            return output.Split('\n')[0].Trim();
        }
        catch
        {
            return null;
        }
    }
}
```

- [ ] **Step 7: Run tests**

Run: `dotnet test cli-rhino/Tests/cli-rhino.Tests.csproj -v normal`
Expected: all tests pass (18 total: 12 wrapper + 6 client).

- [ ] **Step 8: Commit**

```bash
git add cli-rhino/RhinocodeClient.cs cli-rhino/Tests/RhinocodeClientTests.cs cli-rhino/Tests/stub-rhinocode cli-rhino/Tests/cli-rhino.Tests.csproj
git commit -m "feat(v0.32): RhinocodeClient (discovery + shell-out) + stub-rhinocode tests"
```

---

## Task 5 — Verbs/ListInstances.cs

**Files:**
- Create: `cli-rhino/Verbs/ListInstances.cs`

- [ ] **Step 1: Write `cli-rhino/Verbs/ListInstances.cs`**

```csharp
// `list-instances` verb: shells out to `rhinocode list --json` and reshapes
// McNeel's keys (pipeId, processId, processVersion, activeDoc) into our
// snake_case schema (rhino_id, pid, version, active_doc).

using System.Text.Json.Nodes;

namespace AwareRhino.Verbs;

internal static class ListInstances
{
    public static int Run(RhinocodeClient client)
    {
        var (exit, stdout, stderr) = client.RunListJson();
        if (exit != 0)
        {
            Console.WriteLine(Receipts.GenericFail("list-instances",
                $"rhinocode list exited {exit}: {stderr.Trim()}").ToJsonString());
            return 2;
        }

        JsonArray? raw;
        try
        {
            raw = JsonNode.Parse(stdout) as JsonArray;
            if (raw is null) throw new InvalidOperationException("expected JSON array");
        }
        catch (Exception e)
        {
            Console.WriteLine(Receipts.GenericFail("list-instances",
                $"could not parse rhinocode list --json output: {e.Message}").ToJsonString());
            return 2;
        }

        var reshaped = new JsonArray();
        foreach (var item in raw)
        {
            if (item is not JsonObject jo) continue;
            reshaped.Add(new JsonObject
            {
                ["pid"]        = jo["processId"]?.GetValue<int?>(),
                ["version"]    = TrimVersion(jo["processVersion"]?.GetValue<string>()),
                ["rhino_id"]   = jo["pipeId"]?.GetValue<string>(),
                ["active_doc"] = jo["activeDoc"]?.GetValue<string>(),
            });
        }

        Console.WriteLine(Receipts.ListOk(reshaped).ToJsonString());
        return 0;
    }

    // "8.13.24296.13001" → "8.13" so it matches the version filter shape used in `exec`.
    static string? TrimVersion(string? full)
    {
        if (string.IsNullOrEmpty(full)) return null;
        var parts = full!.Split('.');
        return parts.Length >= 2 ? $"{parts[0]}.{parts[1]}" : full;
    }
}
```

- [ ] **Step 2: Verify it builds**

Run: `dotnet build cli-rhino/cli-rhino.csproj`
Expected: build succeeds, no warnings.

- [ ] **Step 3: Commit**

```bash
git add cli-rhino/Verbs/ListInstances.cs
git commit -m "feat(v0.32): list-instances verb (reshapes rhinocode list --json)"
```

---

## Task 6 — Verbs/Exec.cs (the main event)

**Files:**
- Create: `cli-rhino/Verbs/Exec.cs`

- [ ] **Step 1: Write `cli-rhino/Verbs/Exec.cs`**

```csharp
// `exec` verb: wraps the user's C# in a temp .cs file, ships it through
// rhinocode (optionally targeting a specific Rhino instance), parses the
// result between sentinels, emits an AWARE-shaped receipt.

using System.Text.Json.Nodes;

namespace AwareRhino.Verbs;

internal static class Exec
{
    public static int Run(RhinocodeClient client, JsonNode? input)
    {
        var code = input?["code"]?.GetValue<string>();
        if (string.IsNullOrEmpty(code))
        {
            Console.WriteLine(Receipts.ExecFail(
                "missing required field: code (C# script to compile + run)", "", "")
                .ToJsonString());
            return 2;
        }

        var rhinoId = input?["rhino_id"]?.GetValue<string>();
        var version = input?["version"]?.GetValue<string>();
        var argsObj = input?["args"] as JsonObject;
        var argsJson = (argsObj ?? new JsonObject()).ToJsonString();

        // If no rhino_id supplied but version is, resolve via list and pick first match.
        // (If neither, rhinocode picks the default.)
        int? resolvedPid = null;
        string? resolvedVersion = null;
        if (string.IsNullOrEmpty(rhinoId) && !string.IsNullOrEmpty(version))
        {
            try
            {
                var resolved = ResolveByVersion(client, version!);
                if (resolved is null)
                {
                    Console.WriteLine(Receipts.ExecFail(
                        $"no running Rhino instance matches version='{version}'",
                        "", "").ToJsonString());
                    return 1;
                }
                rhinoId          = resolved.Value.rhinoId;
                resolvedPid      = resolved.Value.pid;
                resolvedVersion  = resolved.Value.version;
            }
            catch (Exception e)
            {
                Console.WriteLine(Receipts.ExecFail(
                    $"version lookup via rhinocode list failed: {e.Message}", "", "")
                    .ToJsonString());
                return 2;
            }
        }

        // Wrap, write temp, ship.
        var wrapped = ScriptWrapper.Wrap(code!);
        var tempPath = Path.Combine(Path.GetTempPath(),
            $"aware-rhino-exec-{Guid.NewGuid():N}.cs");
        File.WriteAllText(tempPath, wrapped);

        try
        {
            var (exit, stdout, stderr) = client.RunScript(tempPath, rhinoId, argsJson);

            // Find the sentinel block.
            var (jsonText, otherStdout) = ExtractResultJson(stdout);
            if (jsonText is null)
            {
                // Failure mode 1: rhinocode exited non-zero. Surface stderr.
                if (exit != 0)
                {
                    Console.WriteLine(Receipts.ExecFail(
                        FirstLine(stderr) ?? $"rhinocode exited {exit}",
                        stderr.Trim(), stdout).ToJsonString());
                    return 2;
                }
                // Failure mode 2: rhinocode exited 0 but no sentinel. Likely the
                // user's script threw before writing the result. Surface the
                // last non-empty stdout line as the best clue.
                Console.WriteLine(Receipts.ExecFail(
                    "result sentinel not found in script output (script may have thrown)",
                    stderr.Trim(), stdout).ToJsonString());
                return 2;
            }

            JsonNode? resultNode;
            try { resultNode = JsonNode.Parse(jsonText); }
            catch (Exception e)
            {
                Console.WriteLine(Receipts.ExecFail(
                    $"result JSON parse failed: {e.Message}",
                    jsonText, stdout).ToJsonString());
                return 2;
            }

            Console.WriteLine(Receipts.ExecOk(resultNode, resolvedVersion, resolvedPid, rhinoId, otherStdout).ToJsonString());
            return 0;
        }
        finally
        {
            try { File.Delete(tempPath); } catch { /* best-effort */ }
        }
    }

    // Pulls the JSON between __AWARE_RESULT_BEGIN__ and __AWARE_RESULT_END__ out
    // of stdout. Returns (jsonText, otherStdout) — otherStdout has the sentinel
    // block stripped so the receipt's stdout_log is just the user's noise.
    internal static (string? jsonText, string otherStdout) ExtractResultJson(string stdout)
    {
        var lines = stdout.Replace("\r\n", "\n").Split('\n');
        int beginIdx = -1, endIdx = -1;
        for (int i = 0; i < lines.Length; i++)
        {
            if (beginIdx < 0 && lines[i].Trim() == ScriptWrapper.ResultBeginSentinel)
                beginIdx = i;
            else if (beginIdx >= 0 && lines[i].Trim() == ScriptWrapper.ResultEndSentinel)
            {
                endIdx = i;
                break;
            }
        }
        if (beginIdx < 0 || endIdx < 0) return (null, stdout);

        var jsonLines = new List<string>();
        for (int i = beginIdx + 1; i < endIdx; i++) jsonLines.Add(lines[i]);
        var json = string.Join("\n", jsonLines).Trim();

        var rest = new List<string>();
        for (int i = 0; i < lines.Length; i++)
            if (i < beginIdx || i > endIdx) rest.Add(lines[i]);
        return (json, string.Join("\n", rest).Trim());
    }

    static (string rhinoId, int? pid, string version)? ResolveByVersion(RhinocodeClient client, string version)
    {
        var (exit, stdout, _) = client.RunListJson();
        if (exit != 0) throw new InvalidOperationException($"rhinocode list exited {exit}");
        var arr = JsonNode.Parse(stdout) as JsonArray
                  ?? throw new InvalidOperationException("rhinocode list did not return a JSON array");
        foreach (var item in arr)
        {
            if (item is not JsonObject jo) continue;
            var procVersion = jo["processVersion"]?.GetValue<string>() ?? "";
            // Accept "8" matching "8.13.24296.13001" — startswith.
            if (procVersion.StartsWith(version, StringComparison.Ordinal))
            {
                return (
                    rhinoId: jo["pipeId"]!.GetValue<string>(),
                    pid: jo["processId"]?.GetValue<int?>(),
                    version: TrimVersion(procVersion));
            }
        }
        return null;
    }

    static string TrimVersion(string full)
    {
        var parts = full.Split('.');
        return parts.Length >= 2 ? $"{parts[0]}.{parts[1]}" : full;
    }

    static string? FirstLine(string? s)
    {
        if (string.IsNullOrEmpty(s)) return null;
        var idx = s!.IndexOf('\n');
        return idx < 0 ? s.Trim() : s.Substring(0, idx).Trim();
    }
}
```

- [ ] **Step 2: Add ExecTests.cs** — `cli-rhino/Tests/ExecTests.cs`

```csharp
using AwareRhino.Verbs;
using Xunit;

namespace AwareRhino.Tests;

public class ExecTests
{
    [Fact]
    public void ExtractResultJson_HappyPath()
    {
        var stdout = "noise\n__AWARE_RESULT_BEGIN__\n{\"x\":1}\n__AWARE_RESULT_END__\nmore noise";
        var (json, rest) = Exec.ExtractResultJson(stdout);
        Assert.Equal("{\"x\":1}", json);
        Assert.Contains("noise", rest);
        Assert.Contains("more noise", rest);
        Assert.DoesNotContain("__AWARE_RESULT_", rest);
    }

    [Fact]
    public void ExtractResultJson_MultiLineJson()
    {
        var stdout = "__AWARE_RESULT_BEGIN__\n{\n  \"x\": 1,\n  \"y\": 2\n}\n__AWARE_RESULT_END__";
        var (json, _) = Exec.ExtractResultJson(stdout);
        Assert.NotNull(json);
        Assert.Contains("\"x\": 1", json!);
        Assert.Contains("\"y\": 2", json!);
    }

    [Fact]
    public void ExtractResultJson_NoSentinels_ReturnsNullAndAllStdout()
    {
        var stdout = "boom this script threw\nat line 5";
        var (json, rest) = Exec.ExtractResultJson(stdout);
        Assert.Null(json);
        Assert.Contains("boom this script threw", rest);
    }
}
```

- [ ] **Step 3: Make Exec internals visible to tests** — they should already be via the `InternalsVisibleTo` we set in Task 3. Verify the `internal static` on `ExtractResultJson`.

- [ ] **Step 4: Run tests**

Run: `dotnet test cli-rhino/Tests/cli-rhino.Tests.csproj -v normal`
Expected: 21 tests pass total (12 wrapper + 6 client + 3 exec).

- [ ] **Step 5: Commit**

```bash
git add cli-rhino/Verbs/Exec.cs cli-rhino/Tests/ExecTests.cs
git commit -m "feat(v0.32): exec verb — wrap user code, ship through rhinocode, parse sentinels"
```

---

## Task 7 — Verbs/SendStatus.cs

**Files:**
- Create: `cli-rhino/Verbs/SendStatus.cs`

- [ ] **Step 1: Write `cli-rhino/Verbs/SendStatus.cs`**

```csharp
// `send-status` verb: synthesizes a tiny C# snippet that calls
// Rhino.UI.StatusBar.ShowMessage(args["message"]) and dispatches it through
// the same Exec pipeline. Validates the wrapper end-to-end on every call.

using System.Text.Json.Nodes;

namespace AwareRhino.Verbs;

internal static class SendStatus
{
    public static int Run(RhinocodeClient client, JsonNode? input)
    {
        var message = input?["message"]?.GetValue<string>();
        if (string.IsNullOrEmpty(message))
        {
            Console.WriteLine(Receipts.GenericFail("send-status",
                "missing required field: message").ToJsonString());
            return 2;
        }

        // Build a synthetic exec request and reuse the Exec pipeline.
        var execInput = new JsonObject
        {
            ["verb"]     = "exec",
            ["version"]  = input?["version"]?.GetValue<string>(),
            ["rhino_id"] = input?["rhino_id"]?.GetValue<string>(),
            ["code"]     =
                """
                var msg = args.TryGetValue("message", out var m) ? (string?)m?.ToString() : null;
                if (!string.IsNullOrEmpty(msg))
                {
                    Rhino.UI.StatusBar.ShowMessage(msg);
                }
                return new { delivered = !string.IsNullOrEmpty(msg), message = msg };
                """,
            ["args"]     = new JsonObject { ["message"] = message },
        };

        // Capture the exec receipt and re-shape into a send-status receipt.
        using var sw = new StringWriter();
        var originalOut = Console.Out;
        Console.SetOut(sw);
        int execExit;
        try { execExit = Exec.Run(client, execInput); }
        finally { Console.SetOut(originalOut); }

        var execReceipt = JsonNode.Parse(sw.ToString().Trim()) as JsonObject;
        if (execReceipt is null || execReceipt["ok"]?.GetValue<bool>() != true)
        {
            // Forward exec's error envelope but mark verb as send-status.
            var err = execReceipt?["error"]?.GetValue<string>() ?? "exec failed";
            Console.WriteLine(Receipts.GenericFail("send-status", err,
                execReceipt?["stack"]?.GetValue<string>()).ToJsonString());
            return execExit;
        }

        var version = execReceipt["host_version"]?.GetValue<string>();
        var pid     = execReceipt["host_pid"]?.GetValue<int?>();
        Console.WriteLine(Receipts.SendStatusOk(message!, version, pid).ToJsonString());
        return 0;
    }
}
```

- [ ] **Step 2: Verify it builds**

Run: `dotnet build cli-rhino/cli-rhino.csproj`
Expected: build succeeds, no warnings.

- [ ] **Step 3: Commit**

```bash
git add cli-rhino/Verbs/SendStatus.cs
git commit -m "feat(v0.32): send-status verb (delegates to exec for unified path)"
```

---

## Task 8 — Program.cs full verb dispatch

**Files:**
- Modify: `cli-rhino/Program.cs` (replace the Task-1 stub)

- [ ] **Step 1: Replace `cli-rhino/Program.cs`** with full impl:

```csharp
// aware-rhino — runtime sidecar that wraps McNeel's rhinocode CLI for the AWARE
// runtime. Speaks the {verb,code,args} stdin-JSON contract shared with cli-tekla.

using System.Text.Json.Nodes;
using AwareRhino.Verbs;

namespace AwareRhino;

internal static class Program
{
    static int Main(string[] args)
    {
        Console.InputEncoding  = new System.Text.UTF8Encoding(false);
        Console.OutputEncoding = new System.Text.UTF8Encoding(false);

        try { return Run(args); }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"aware-rhino: unhandled error: {ex.Message}");
            Console.Error.WriteLine(ex.StackTrace ?? "");
            return 2;
        }
    }

    static int Run(string[] args)
    {
        if (args.Length == 0 || args[0] is "--help" or "-h")
        {
            PrintHelp();
            return 0;
        }

        // Two invocation styles (same as cli-tekla):
        //   aware-rhino.exe <verb> [--json-stdin]    (canonical)
        //   aware-rhino.exe --json-stdin             (verb embedded in JSON body)
        string verb;
        JsonNode? stdinJson = null;

        if (args[0].StartsWith("--", StringComparison.Ordinal))
        {
            string buf;
            try { buf = Console.In.ReadToEnd(); }
            catch (Exception e)
            {
                Console.Error.WriteLine($"aware-rhino: stdin not readable: {e.Message}");
                return 2;
            }
            try { stdinJson = JsonNode.Parse(buf); }
            catch (Exception e)
            {
                Console.Error.WriteLine($"aware-rhino: stdin not JSON: {e.Message}");
                return 2;
            }
            verb = stdinJson?["verb"]?.GetValue<string>() ?? "";
            if (string.IsNullOrEmpty(verb))
            {
                Console.Error.WriteLine("aware-rhino: stdin JSON has no `verb` field and no verb on CLI");
                return 2;
            }
        }
        else
        {
            verb = args[0];
            // Read stdin only if --json-stdin was passed somewhere after the verb.
            var wantsStdin = args.Skip(1).Any(a => a == "--json-stdin");
            // exec, send-status, and any verb that takes structured input → always need stdin
            if (wantsStdin || verb is "exec" or "send-status")
            {
                string buf;
                try { buf = Console.In.ReadToEnd(); }
                catch (Exception e)
                {
                    Console.Error.WriteLine($"aware-rhino: stdin not readable: {e.Message}");
                    return 2;
                }
                if (!string.IsNullOrWhiteSpace(buf))
                {
                    try { stdinJson = JsonNode.Parse(buf); }
                    catch (Exception e)
                    {
                        Console.Error.WriteLine($"aware-rhino: stdin not JSON: {e.Message}");
                        return 2;
                    }
                }
            }
        }

        // Discover rhinocode once. All verbs share the client.
        RhinocodeClient client;
        try { client = new RhinocodeClient(); }
        catch (FileNotFoundException e)
        {
            Console.WriteLine(Receipts.GenericFail(verb, e.Message).ToJsonString());
            return 3;
        }

        return verb switch
        {
            "exec"           => Exec.Run(client, stdinJson),
            "list-instances" => ListInstances.Run(client),
            "send-status"    => SendStatus.Run(client, stdinJson),
            _ => Unknown(verb),
        };
    }

    static int Unknown(string verb)
    {
        Console.Error.WriteLine($"aware-rhino: unknown verb '{verb}'. Try --help.");
        return 2;
    }

    static void PrintHelp()
    {
        Console.WriteLine("""
            aware-rhino — Rhino 8 sidecar (wraps McNeel's rhinocode CLI)

            Usage:
              aware-rhino <verb> [--json-stdin]
              aware-rhino --json-stdin            (verb embedded in JSON body)

            Verbs:
              exec             Compile + run an ad-hoc C# script against the active Rhino doc
              list-instances   Print running Rhino instances (PID + version + active doc)
              send-status      Display a transient status-bar message in Rhino

            Exit codes:
              0  success
              1  no matching Rhino instance running
              2  API call failed / bad args / unknown verb
              3  rhinocode.exe not found (install Rhino 8.11+ or set RHINOCODE_EXE)

            Prerequisites:
              - Rhino 8.11+ installed
              - In Rhino, run `StartScriptServer` once per session
              - For multi-instance routing, pass `rhino_id` in the input JSON
            """);
    }
}
```

- [ ] **Step 2: Build + smoke test no-Rhino path**

Run: `cd cli-rhino && dotnet build`
Expected: builds clean.

Smoke (no live Rhino, no rhinocode — should fail with exit 3):
```powershell
'{"verb":"exec"}' | & dotnet run --project cli-rhino -- --json-stdin
```
Expected: stdout JSON receipt with `"status":"err"`, `"error":"Could not find rhinocode.exe..."`, exit 3.

- [ ] **Step 3: Smoke test with stub rhinocode on PATH**

```powershell
$env:RHINOCODE_EXE = "$PWD\cli-rhino\Tests\stub-rhinocode\rhinocode.cmd"
'{"verb":"exec","code":"return 1+2;"}' | & dotnet run --project cli-rhino -- --json-stdin
Remove-Item Env:\RHINOCODE_EXE
```
Expected: stdout receipt with `"ok":true`, `"result": {"count": 42}` (stub returns the canned response regardless of input).

- [ ] **Step 4: Commit**

```bash
git add cli-rhino/Program.cs
git commit -m "feat(v0.32): wire Program.cs verb dispatch (exec/list/send-status)"
```

---

## Task 9 — 20-prompt drill fixtures

**Files:**
- Create: `cli-rhino/Ingest/Output/prompt-01.json` … `prompt-20.json` (20 files)

Each is a complete `{verb, code, args}` JSON payload ready to pipe into `aware-rhino.exe`. Following the table in the spec.

- [ ] **Step 1: Write fixtures 01–05 (read-side)**

`prompt-01.json` — count objects by layer:
```json
{
  "verb": "exec",
  "version": "8",
  "code": "var doc = Rhino.RhinoDoc.ActiveDoc;\nvar buckets = new Dictionary<string, int>();\nforeach (var obj in doc.Objects)\n{\n    var layerName = doc.Layers[obj.Attributes.LayerIndex].FullPath;\n    buckets[layerName] = (buckets.TryGetValue(layerName, out var n) ? n : 0) + 1;\n}\nreturn new { total = doc.Objects.Count, by_layer = buckets };",
  "args": {}
}
```

`prompt-02.json` — GUIDs of selected objects:
```json
{
  "verb": "exec",
  "version": "8",
  "code": "var doc = Rhino.RhinoDoc.ActiveDoc;\nvar guids = new List<string>();\nforeach (var obj in doc.Objects.GetSelectedObjects(false, false))\n{\n    guids.Add(obj.Id.ToString());\n}\nreturn new { count = guids.Count, guids = guids };",
  "args": {}
}
```

`prompt-03.json` — bounding box of all visible geometry:
```json
{
  "verb": "exec",
  "version": "8",
  "code": "var doc = Rhino.RhinoDoc.ActiveDoc;\nvar bbox = Rhino.Geometry.BoundingBox.Empty;\nforeach (var obj in doc.Objects)\n{\n    if (obj.IsHidden) continue;\n    bbox.Union(obj.Geometry.GetBoundingBox(true));\n}\nreturn new {\n    is_valid = bbox.IsValid,\n    min = new { x = bbox.Min.X, y = bbox.Min.Y, z = bbox.Min.Z },\n    max = new { x = bbox.Max.X, y = bbox.Max.Y, z = bbox.Max.Z }\n};",
  "args": {}
}
```

`prompt-04.json` — list layers with counts:
```json
{
  "verb": "exec",
  "version": "8",
  "code": "var doc = Rhino.RhinoDoc.ActiveDoc;\nvar counts = new Dictionary<int, int>();\nforeach (var obj in doc.Objects)\n{\n    counts[obj.Attributes.LayerIndex] = (counts.TryGetValue(obj.Attributes.LayerIndex, out var n) ? n : 0) + 1;\n}\nvar rows = new List<object>();\nforeach (var layer in doc.Layers)\n{\n    rows.Add(new { index = layer.Index, name = layer.FullPath, visible = layer.IsVisible, count = counts.TryGetValue(layer.Index, out var c) ? c : 0 });\n}\nreturn new { layer_count = doc.Layers.Count, layers = rows };",
  "args": {}
}
```

`prompt-05.json` — list block instance definitions and member counts:
```json
{
  "verb": "exec",
  "version": "8",
  "code": "var doc = Rhino.RhinoDoc.ActiveDoc;\nvar rows = new List<object>();\nforeach (var idef in doc.InstanceDefinitions)\n{\n    if (idef == null) continue;\n    var objs = idef.GetObjects();\n    rows.Add(new { name = idef.Name, id = idef.Id.ToString(), member_count = objs?.Length ?? 0 });\n}\nreturn new { definition_count = rows.Count, definitions = rows };",
  "args": {}
}
```

- [ ] **Step 2: Write fixtures 06–12 (write-side)**

`prompt-06.json` — add a line:
```json
{
  "verb": "exec",
  "version": "8",
  "code": "var doc = Rhino.RhinoDoc.ActiveDoc;\nvar p0 = new Rhino.Geometry.Point3d(0, 0, 0);\nvar p1 = new Rhino.Geometry.Point3d(1000, 0, 0);\nvar id = doc.Objects.AddLine(p0, p1);\ndoc.Views.Redraw();\nreturn new { added = id != Guid.Empty, id = id.ToString() };",
  "args": {}
}
```

`prompt-07.json` — add a circle:
```json
{
  "verb": "exec",
  "version": "8",
  "code": "var doc = Rhino.RhinoDoc.ActiveDoc;\nvar plane = Rhino.Geometry.Plane.WorldXY;\nvar circle = new Rhino.Geometry.Circle(plane, 500);\nvar id = doc.Objects.AddCircle(circle);\ndoc.Views.Redraw();\nreturn new { added = id != Guid.Empty, id = id.ToString(), radius = 500 };",
  "args": {}
}
```

`prompt-08.json` — add a sphere:
```json
{
  "verb": "exec",
  "version": "8",
  "code": "var doc = Rhino.RhinoDoc.ActiveDoc;\nvar center = new Rhino.Geometry.Point3d(1000, 1000, 0);\nvar sphere = new Rhino.Geometry.Sphere(center, 250);\nvar id = doc.Objects.AddSphere(sphere);\ndoc.Views.Redraw();\nreturn new { added = id != Guid.Empty, id = id.ToString() };",
  "args": {}
}
```

`prompt-09.json` — add a box:
```json
{
  "verb": "exec",
  "version": "8",
  "code": "var doc = Rhino.RhinoDoc.ActiveDoc;\nvar plane = Rhino.Geometry.Plane.WorldXY;\nvar box = new Rhino.Geometry.Box(plane, new Rhino.Geometry.Interval(0, 1000), new Rhino.Geometry.Interval(0, 1000), new Rhino.Geometry.Interval(0, 500));\nvar id = doc.Objects.AddBox(box);\ndoc.Views.Redraw();\nreturn new { added = id != Guid.Empty, id = id.ToString() };",
  "args": {}
}
```

`prompt-10.json` — create a layer with random color:
```json
{
  "verb": "exec",
  "version": "8",
  "code": "var doc = Rhino.RhinoDoc.ActiveDoc;\nvar rng = new Random();\nvar color = System.Drawing.Color.FromArgb(rng.Next(256), rng.Next(256), rng.Next(256));\nvar layer = new Rhino.DocObjects.Layer { Name = \"AWARE\", Color = color };\nint index = doc.Layers.Add(layer);\nreturn new { added = index >= 0, index = index, color = $\"#{color.R:X2}{color.G:X2}{color.B:X2}\" };",
  "args": {}
}
```

`prompt-11.json` — add 10 random points and group them:
```json
{
  "verb": "exec",
  "version": "8",
  "code": "var doc = Rhino.RhinoDoc.ActiveDoc;\nvar rng = new Random();\nvar ids = new List<Guid>();\nfor (int i = 0; i < 10; i++)\n{\n    var p = new Rhino.Geometry.Point3d(rng.NextDouble() * 1000, rng.NextDouble() * 1000, 0);\n    ids.Add(doc.Objects.AddPoint(p));\n}\nint groupIndex = doc.Groups.Add(ids);\ndoc.Views.Redraw();\nreturn new { group_index = groupIndex, point_count = ids.Count, ids = ids.ConvertAll(g => g.ToString()) };",
  "args": {}
}
```

`prompt-12.json` — set selected objects' layer to "AWARE":
```json
{
  "verb": "exec",
  "version": "8",
  "code": "var doc = Rhino.RhinoDoc.ActiveDoc;\nvar awareLayerIndex = doc.Layers.FindName(\"AWARE\")?.Index ?? -1;\nif (awareLayerIndex < 0) return new { ok = false, error = \"layer 'AWARE' does not exist; create it first (prompt 10)\" };\nint moved = 0;\nforeach (var obj in doc.Objects.GetSelectedObjects(false, false))\n{\n    var attrs = obj.Attributes.Duplicate();\n    attrs.LayerIndex = awareLayerIndex;\n    if (doc.Objects.ModifyAttributes(obj, attrs, false)) moved++;\n}\ndoc.Views.Redraw();\nreturn new { ok = true, moved = moved, target_layer = \"AWARE\" };",
  "args": {}
}
```

- [ ] **Step 3: Write fixtures 13–17 (ops)**

`prompt-13.json` — move selected by (100,100,0):
```json
{
  "verb": "exec",
  "version": "8",
  "code": "var doc = Rhino.RhinoDoc.ActiveDoc;\nvar xform = Rhino.Geometry.Transform.Translation(100, 100, 0);\nint moved = 0;\nforeach (var obj in doc.Objects.GetSelectedObjects(false, false))\n{\n    if (doc.Objects.Transform(obj.Id, xform, true) != Guid.Empty) moved++;\n}\ndoc.Views.Redraw();\nreturn new { moved = moved, dx = 100, dy = 100, dz = 0 };",
  "args": {}
}
```

`prompt-14.json` — rotate selected 45° about Z at origin:
```json
{
  "verb": "exec",
  "version": "8",
  "code": "var doc = Rhino.RhinoDoc.ActiveDoc;\nvar xform = Rhino.Geometry.Transform.Rotation(Math.PI / 4, Rhino.Geometry.Vector3d.ZAxis, Rhino.Geometry.Point3d.Origin);\nint rotated = 0;\nforeach (var obj in doc.Objects.GetSelectedObjects(false, false))\n{\n    if (doc.Objects.Transform(obj.Id, xform, true) != Guid.Empty) rotated++;\n}\ndoc.Views.Redraw();\nreturn new { rotated = rotated, degrees = 45, axis = \"Z\" };",
  "args": {}
}
```

`prompt-15.json` — scale selected by 2 about origin:
```json
{
  "verb": "exec",
  "version": "8",
  "code": "var doc = Rhino.RhinoDoc.ActiveDoc;\nvar xform = Rhino.Geometry.Transform.Scale(Rhino.Geometry.Point3d.Origin, 2.0);\nint scaled = 0;\nforeach (var obj in doc.Objects.GetSelectedObjects(false, false))\n{\n    if (doc.Objects.Transform(obj.Id, xform, true) != Guid.Empty) scaled++;\n}\ndoc.Views.Redraw();\nreturn new { scaled = scaled, factor = 2.0 };",
  "args": {}
}
```

`prompt-16.json` — mirror selected across YZ plane:
```json
{
  "verb": "exec",
  "version": "8",
  "code": "var doc = Rhino.RhinoDoc.ActiveDoc;\nvar xform = Rhino.Geometry.Transform.Mirror(Rhino.Geometry.Plane.WorldYZ);\nint mirrored = 0;\nforeach (var obj in doc.Objects.GetSelectedObjects(false, false))\n{\n    if (doc.Objects.Transform(obj.Id, xform, false) != Guid.Empty) mirrored++;\n}\ndoc.Views.Redraw();\nreturn new { mirrored = mirrored, plane = \"WorldYZ\" };",
  "args": {}
}
```

`prompt-17.json` — boolean union of selected breps:
```json
{
  "verb": "exec",
  "version": "8",
  "code": "var doc = Rhino.RhinoDoc.ActiveDoc;\nvar breps = new List<Rhino.Geometry.Brep>();\nvar pickedIds = new List<Guid>();\nforeach (var obj in doc.Objects.GetSelectedObjects(false, false))\n{\n    if (obj.Geometry is Rhino.Geometry.Brep b) { breps.Add(b); pickedIds.Add(obj.Id); }\n}\nif (breps.Count < 2) return new { ok = false, error = $\"need at least 2 selected breps; got {breps.Count}\" };\nvar tol = doc.ModelAbsoluteTolerance;\nvar result = Rhino.Geometry.Brep.CreateBooleanUnion(breps, tol);\nif (result == null || result.Length == 0) return new { ok = false, error = \"boolean union returned no breps\" };\nforeach (var id in pickedIds) doc.Objects.Delete(id, true);\nvar newIds = new List<string>();\nforeach (var brep in result) newIds.Add(doc.Objects.AddBrep(brep).ToString());\ndoc.Views.Redraw();\nreturn new { ok = true, inputs = pickedIds.Count, outputs = newIds.Count, ids = newIds };",
  "args": {}
}
```

- [ ] **Step 4: Write fixtures 18–20 (file + meta)**

`prompt-18.json` — save as 3dm in temp:
```json
{
  "verb": "exec",
  "version": "8",
  "code": "var doc = Rhino.RhinoDoc.ActiveDoc;\nvar target = System.IO.Path.Combine(System.IO.Path.GetTempPath(), \"aware-drill.3dm\");\nvar options = new Rhino.FileIO.FileWriteOptions { SuppressDialogBoxes = true, IncludeRenderMeshes = true };\nbool ok = doc.WriteFile(target, options);\nreturn new { ok = ok, path = target };",
  "args": {}
}
```

`prompt-19.json` — export selected as STL:
```json
{
  "verb": "exec",
  "version": "8",
  "code": "var doc = Rhino.RhinoDoc.ActiveDoc;\nvar selected = new List<Guid>();\nforeach (var obj in doc.Objects.GetSelectedObjects(false, false)) selected.Add(obj.Id);\nif (selected.Count == 0) return new { ok = false, error = \"no objects selected\" };\nvar target = System.IO.Path.Combine(System.IO.Path.GetTempPath(), \"aware-drill.stl\");\nvar cmd = $\"_-Export \\\"{target}\\\" _Enter _Enter\";\nbool issued = Rhino.RhinoApp.RunScript(cmd, false);\nreturn new { ok = issued, path = target, selected_count = selected.Count };",
  "args": {}
}
```

`prompt-20.json` — list viewports:
```json
{
  "verb": "exec",
  "version": "8",
  "code": "var doc = Rhino.RhinoDoc.ActiveDoc;\nvar rows = new List<object>();\nforeach (var view in doc.Views)\n{\n    var vp = view.ActiveViewport;\n    rows.Add(new {\n        name = view.MainViewport.Name,\n        projection = vp.IsParallelProjection ? \"parallel\" : (vp.IsPerspectiveProjection ? \"perspective\" : \"other\"),\n        camera_location = new { x = vp.CameraLocation.X, y = vp.CameraLocation.Y, z = vp.CameraLocation.Z },\n        target = new { x = vp.CameraTarget.X, y = vp.CameraTarget.Y, z = vp.CameraTarget.Z }\n    });\n}\nreturn new { count = rows.Count, viewports = rows };",
  "args": {}
}
```

- [ ] **Step 5: Commit**

```bash
git add cli-rhino/Ingest/Output/prompt-*.json
git commit -m "feat(v0.32): 20-prompt drill fixtures for live Rhino testing"
```

---

## Task 10 — run-drill.ps1 harness

**Files:**
- Create: `cli-rhino/Ingest/run-drill.ps1`

- [ ] **Step 1: Write `cli-rhino/Ingest/run-drill.ps1`**

```powershell
#!/usr/bin/env powershell
# AWARE v0.32 — rhino.exec 20-prompt drill harness.
#
# Prerequisites:
#   1. Rhino 8.11+ installed, with `StartScriptServer` run in the active session.
#   2. aware-rhino.exe built: `cd cli-rhino && dotnet build -c Release`
#
# Output: cli-rhino/Ingest/Output/drill-summary.md (PASS/FAIL table + receipt JSONs).

param(
    [string]$AwareRhino = (Join-Path $PSScriptRoot "..\bin\Release\net10.0-windows\win-x64\publish\aware-rhino.exe"),
    [string]$FixturesDir = (Join-Path $PSScriptRoot "Output"),
    [string]$SummaryPath = (Join-Path $PSScriptRoot "Output\drill-summary.md")
)

$ErrorActionPreference = "Stop"

if (-not (Test-Path $AwareRhino)) {
    # Fallback to Debug + non-published path
    $debugPath = Join-Path $PSScriptRoot "..\bin\Debug\net10.0-windows\aware-rhino.exe"
    if (Test-Path $debugPath) {
        $AwareRhino = $debugPath
    } else {
        Write-Error "aware-rhino.exe not found at $AwareRhino. Run `dotnet publish -c Release -r win-x64` first."
    }
}

$fixtures = Get-ChildItem -Path $FixturesDir -Filter "prompt-*.json" | Sort-Object Name
if ($fixtures.Count -eq 0) {
    Write-Error "No prompt-*.json fixtures found in $FixturesDir"
}

Write-Host "Drill: $($fixtures.Count) prompts against $AwareRhino"
Write-Host ""

$results = @()
$pass = 0; $fail = 0
foreach ($fix in $fixtures) {
    $name = $fix.BaseName
    Write-Host -NoNewline ("  {0} ... " -f $name)
    $payload = Get-Content $fix.FullName -Raw
    $rawOutput = $payload | & $AwareRhino --json-stdin 2>&1
    $exit = $LASTEXITCODE
    $receipt = $null
    try { $receipt = $rawOutput | ConvertFrom-Json } catch { }

    $ok = $false
    if ($receipt -and $receipt.ok -eq $true) {
        $ok = $true
    } elseif ($receipt -and $receipt.status -eq "ok") {
        $ok = $true
    }
    $status = if ($ok) { "PASS" } else { "FAIL" }
    if ($ok) { $pass++ } else { $fail++ }

    $results += [pscustomobject]@{
        Name      = $name
        Status    = $status
        ExitCode  = $exit
        Receipt   = $rawOutput
    }

    Write-Host $status
}

Write-Host ""
Write-Host "Summary: $pass PASS, $fail FAIL of $($fixtures.Count)"

# Write markdown summary
$lines = @()
$lines += "# rhino.exec drill — $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"
$lines += ""
$lines += "**Sidecar:** $AwareRhino"
$lines += "**Fixtures:** $FixturesDir"
$lines += "**Result:** $pass PASS, $fail FAIL of $($fixtures.Count)"
$lines += ""
$lines += "| # | Prompt | Status | Exit | Result snippet |"
$lines += "|---|---|---|---|---|"
foreach ($r in $results) {
    $snippet = ($r.Receipt -replace "[`r`n]+"," ").Substring(0, [Math]::Min(120, $r.Receipt.Length))
    $lines += "| $($r.Name) | (see fixture) | **$($r.Status)** | $($r.ExitCode) | ``$snippet`` |"
}
$lines += ""
$lines += "## Full receipts"
$lines += ""
foreach ($r in $results) {
    $lines += "### $($r.Name) — $($r.Status)"
    $lines += ""
    $lines += '```json'
    $lines += $r.Receipt
    $lines += '```'
    $lines += ""
}
$lines -join "`n" | Out-File -FilePath $SummaryPath -Encoding utf8
Write-Host "Wrote summary → $SummaryPath"

exit $fail
```

- [ ] **Step 2: Verify the script is syntactically valid PowerShell**

Run: `powershell -NoProfile -Command "& { . cli-rhino/Ingest/run-drill.ps1 -AwareRhino 'does-not-exist' }"`
Expected: errors with "aware-rhino.exe not found" (proves the script parses + reaches the error path; we're not actually running the drill in this step).

- [ ] **Step 3: Commit**

```bash
git add cli-rhino/Ingest/run-drill.ps1
git commit -m "feat(v0.32): drill harness — runs 20 fixtures, writes PASS/FAIL summary"
```

---

## Task 11 — Patch rhino-8 agent manifest

**Files:**
- Modify: `20-agents/aeco/architecture/rhino-8/manifest.yaml`

- [ ] **Step 1: Read the manifest's current `transport` and `commands` sections to locate insertion points**

Run: `head -45 20-agents/aeco/architecture/rhino-8/manifest.yaml`

- [ ] **Step 2: Patch via Edit tool**

a) Change the `transport:` block (currently lacks the exec sidecar entry — file already has `transport.cli.binary: aware-rhino` from Task 1 inspection… verify):

If `transport.cli.binary` is already `aware-rhino`, skip this step. Otherwise apply:

```yaml
transport:
  cli:
    binary: aware-rhino
```

b) Add `exec` command at the top of the `commands:` block (alphabetical ordering is broken in the existing manifest anyway; insert right after `commands:`):

```yaml
commands:
  exec:
    lifecycle: single
    description: |
      Compile + run an ad-hoc C# script against the active Rhino document.
      Input: { code, args, version?, rhino_id? }. Output: { ok, result, ... }.
      The script body is wrapped in `static object __AwareRun(IDictionary<string, object?> args)` —
      use `return X;` to return a value. Requires Rhino 8.11+ with StartScriptServer active.
  list-instances:
    lifecycle: single
    description: List running Rhino instances (pid, version, active doc, rhino_id pipe).
  send-status:
    lifecycle: single
    description: Display a transient status-bar message in Rhino.
  # existing entries below…
```

- [ ] **Step 3: Verify the manifest still parses as YAML**

Run: `python -c "import yaml,sys; yaml.safe_load(open('20-agents/aeco/architecture/rhino-8/manifest.yaml','r',encoding='utf-8')); print('OK')"`
Expected: prints `OK`.

- [ ] **Step 4: Commit**

```bash
git add 20-agents/aeco/architecture/rhino-8/manifest.yaml
git commit -m "feat(v0.32): rhino-8 manifest — wire aware-rhino transport + exec/list/send-status verbs"
```

---

## Task 12 — Handoff doc

**Files:**
- Create: `docs/superpowers/handoffs/2026-05-19-v032-rhino-exec-ready.md`

- [ ] **Step 1: Write the handoff**

```markdown
# v0.32 — rhino.exec ready for live drill (2026-05-19)

The `aware-rhino` runtime sidecar is built, tested, and waiting on the
20-prompt drill against a live Rhino 8. Pawel: when you wake up, follow the
"Run the drill" section below.

## What shipped

- **`cli-rhino/`** — .NET 10 single-file sidecar (~15 MB published). 3 verbs:
  `exec`, `list-instances`, `send-status`. Wraps McNeel's `rhinocode` CLI
  rather than rebuilding the in-Rhino Roslyn bridge ourselves (decisive
  architectural win vs cli-tekla).
- **21 unit + integration tests** — all green; runnable without Rhino installed.
  Stub `rhinocode.cmd` shipped under `cli-rhino/Tests/stub-rhinocode/`.
- **20 prompt fixtures** under `cli-rhino/Ingest/Output/prompt-*.json` —
  same shape as the v0.31 tekla.exec drill, mix of read/write/ops/file/meta.
- **`run-drill.ps1` harness** — iterates fixtures, writes
  `cli-rhino/Ingest/Output/drill-summary.md` with PASS/FAIL table and full
  receipt JSON per prompt.
- **`rhino-8/manifest.yaml`** — wired to `aware-rhino` binary; declares
  `exec/list-instances/send-status` commands.

## What's NOT shipped (deferred to v0.33)

- `launch` verb (Rhino has no Bypass.ini equivalent — needs design work)
- `close` verb (save-dialog handling)
- Headless Roslyn-against-RhinoCommon fallback (geometry-only path)
- Mac/Linux
- Catalog self-patching loop (mechanical port from tekla)

## Run the drill (5 minutes)

```powershell
# 1. Launch Rhino 8 (8.11+), open a model with some objects to play with.
# 2. In Rhino, type at the command prompt:
StartScriptServer

# 3. From the repo root, publish the sidecar (one-time):
cd C:\Users\bimst\source\repos\aware\cli-rhino
dotnet publish -c Release -r win-x64 --self-contained `
  -p:PublishSingleFile=true -p:IncludeNativeLibrariesForSelfExtract=true

# 4. Sanity check: list instances should find your Rhino
'{"verb":"list-instances"}' | & .\bin\Release\net10.0-windows\win-x64\publish\aware-rhino.exe --json-stdin
# Expected: { "status": "ok", "instances": [ { "pid": ..., "version": "8.13", "rhino_id": "rhinocode_remotepipe_..." } ] }

# 5. Run the 20-prompt drill:
powershell .\Ingest\run-drill.ps1

# 6. Inspect results:
notepad .\Ingest\Output\drill-summary.md
```

## How to interpret PASS/FAIL

The harness scores PASS if `receipt.ok == true` (or `receipt.status == "ok"`).
It does NOT check semantic correctness — a wrong-layer move still passes if
rhinocode returned a result. Eyeball the result JSON in the summary for actual
behavior. Same scoring approach as v0.31 tekla.exec.

Prompts 02, 12-17, 19 require selected objects in the doc. Select something
appropriate before running each (or accept "no objects selected" as the
expected result for those — the script handles the empty case gracefully).

## Architectural notes (for the v0.33 planning session)

- rhinocode CLI's `script` subcommand takes a file path, not stdin. We write
  a per-call temp .cs file under `%TEMP%`, delete it after run.
- C# 9 top-level statements are the script's surface. We wrap the user's body
  in `static object? __AwareRun(IDictionary<string,object?> args)` so `return X;`
  works — same UX as tekla.exec.
- `args` ships from sidecar → script via `AWARE_ARGS_JSON` env var; the wrapper
  deserializes into `Dictionary<string,object?>`.
- Results travel back via `__AWARE_RESULT_BEGIN__` / `__AWARE_RESULT_END__`
  sentinel markers on stdout. Anything outside the markers lands in
  `receipt.stdout_log` for diagnostics.
- Compile errors surface as `{ok:false, error:"CS0103: ...", stack:"<diagnostics>"}` —
  the AI can re-draft.
- `RHINOCODE_EXE` env var overrides the discovery search if you need to point
  at a non-standard install.

## Open issues to surface during the drill

1. **Top-level statement ordering** — our wrapper places top-level code BEFORE
   the local function. C# 9 requires this. If any prompt fails with CS8803
   ("Top-level statements must precede namespace and type declarations"), the
   wrapper is buggy.
2. **JSON serialization of Rhino types** — `JsonSerializer` with
   `ReferenceHandler.IgnoreCycles` and `MaxDepth=6` should handle most things,
   but native pointer-backed types (`Brep`, `Mesh`, raw `Curve`) may explode.
   Each fixture projects to JSON-friendly shapes (numbers, strings, dicts).
   If a prompt fails with a serialization stack trace, the fixture needs to
   project harder before returning.
3. **rhinocode --rhino routing** — I'm using the named-pipe ID from
   `list-instances` as the `--rhino` argument. Per McNeel docs this should work;
   if a multi-instance test crosses wires, switch to filtering by `processId`
   instead.

## Branch + PR

- Branch: `feature/v0.29-runtime-hello-world` (the same branch as v0.30/v0.31).
- Commits: see `git log --oneline origin/main..HEAD` — atomic per-task,
  no `Co-Authored-By: Claude` trailers.
- PR opened: see the PR URL in the session output / your GitHub notifications.
- Merge after drill confirms PASS rate ≥ tekla's 13/20.
```

- [ ] **Step 2: Commit**

```bash
git add docs/superpowers/handoffs/2026-05-19-v032-rhino-exec-ready.md
git commit -m "docs(v0.32): handoff doc — drill recipe + run instructions"
```

---

## Task 13 — Build release binary, final smoke, push, open PR

**Files:** none (orchestration only)

- [ ] **Step 1: Publish Release single-file binary**

```powershell
dotnet publish cli-rhino/cli-rhino.csproj -c Release -r win-x64 --self-contained `
  -p:PublishSingleFile=true -p:IncludeNativeLibrariesForSelfExtract=true
```
Expected: produces `cli-rhino/bin/Release/net10.0-windows/win-x64/publish/aware-rhino.exe` (~15 MB).

- [ ] **Step 2: Run the full test suite one more time**

```powershell
dotnet test cli-rhino/Tests/cli-rhino.Tests.csproj -v normal
```
Expected: 21 tests pass, 0 fail.

- [ ] **Step 3: Smoke test against stub**

```powershell
$env:RHINOCODE_EXE = "$PWD\cli-rhino\Tests\stub-rhinocode\rhinocode.cmd"
$payload = Get-Content cli-rhino/Ingest/Output/prompt-01.json -Raw
$payload | & .\cli-rhino\bin\Release\net10.0-windows\win-x64\publish\aware-rhino.exe --json-stdin
Remove-Item Env:\RHINOCODE_EXE
```
Expected: stdout JSON receipt with `"ok":true` and the stub's canned `{"count":42}` result.

- [ ] **Step 4: Push the branch**

```powershell
git push origin feature/v0.29-runtime-hello-world
```

- [ ] **Step 5: Open the PR**

```powershell
gh pr create --title "feat(v0.32): rhino.exec runtime sidecar — wraps rhinocode CLI" --body @'
## Summary

Ships **v0.32 rhino.exec**, the second `*.exec` sidecar (after Tekla's v0.31). Wraps McNeel's `rhinocode` CLI rather than rebuilding the in-Rhino Roslyn bridge ourselves — gives us the same `{verb,code,args}` stdin contract as tekla.exec with ~⅓ the code.

- `cli-rhino/` — .NET 10 single-file sidecar, 3 verbs (`exec`, `list-instances`, `send-status`)
- 20 prompt fixtures under `cli-rhino/Ingest/Output/`
- Drill harness `cli-rhino/Ingest/run-drill.ps1`
- 21 unit + integration tests (all runnable without Rhino installed)
- `rhino-8/manifest.yaml` patched to declare the verbs

Full design in [docs/superpowers/specs/2026-05-19-rhino-exec-design.md](./docs/superpowers/specs/2026-05-19-rhino-exec-design.md). Plan in [docs/superpowers/plans/2026-05-19-rhino-exec.md](./docs/superpowers/plans/2026-05-19-rhino-exec.md). Handoff with drill recipe in [docs/superpowers/handoffs/2026-05-19-v032-rhino-exec-ready.md](./docs/superpowers/handoffs/2026-05-19-v032-rhino-exec-ready.md).

## Test plan

- [x] `dotnet test cli-rhino/Tests` — 21 green (12 wrapper + 6 client + 3 exec)
- [x] `dotnet publish` — single-file `aware-rhino.exe` builds clean on Release
- [x] Stub-rhinocode smoke test — sidecar correctly shapes the receipt envelope
- [ ] **Live 20-prompt drill against running Rhino 8** — Pawel runs in the morning; Rhino isn't installed on the build machine
- [ ] Eyeball semantic correctness of drill results — PASS = matches expected Rhino behavior, not just "ok=true"

## What's NOT in this PR (v0.33 candidates)

- `launch` / `close` verbs (Rhino has no Bypass.ini equivalent; save-dialog handling needs care)
- Headless Roslyn-against-RhinoCommon fallback for geometry-only calcs
- Mac/Linux
- Catalog self-patching loop (mechanical port from tekla)
- Wiring into `aware app install`'s sidecar-distribution flow
'@
```

- [ ] **Step 6: Update auto-memory with v0.32 completion**

Add a memory file pointing to the v0.32 completion. See the handoff's memory-update conventions (don't write code patterns, do write completion state).

---

## Self-review (run after writing this plan)

1. **Spec coverage:** Every section of the design doc maps to a task. Architecture → Task 1+8 (scaffolding + dispatch). Verb contracts → Tasks 5, 6, 7. Wrapper deep-dive → Task 3. rhinocode discovery → Task 4. Tests → Tasks 3, 4, 6. Drill → Tasks 9, 10. Build/distribution → Task 13. Non-goals → captured in handoff (Task 12) and PR body (Task 13). ✅
2. **Placeholder scan:** No TBD/TODO. Every step has either a code block or an exact command. ✅
3. **Type consistency:** `__AwareRun` consistently spelled across wrapper + tests + handoff. `ScriptWrapper.ResultBeginSentinel` referenced from `Exec.cs` is defined in `ScriptWrapper.cs`. `RhinocodeClient` signature consistent across tests + Verbs/Exec.cs + Verbs/SendStatus.cs. ✅

---

## Execution

Per the project's CLAUDE.md and the active goal directive ("work until you accomplish whole plan"), this plan will execute via subagent-driven-development overnight: fresh subagent per task, plan-author reviews each task's diff before proceeding, atomic commits per task.
