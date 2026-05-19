# v0.33 — revit.exec Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ship `aware-revit` runtime sidecar (two binaries — an in-Revit add-in + a CLI driver) with five verbs (`exec`, `list-instances`, `send-status`, `launch`, `close`) at full feature-parity with v0.31 Tekla. Run a 20-prompt live drill against Revit 2026 with ≥13/20 PASS.

**Architecture:** Revit's API is callable only on Revit's main thread inside Revit's process. Therefore: (1) `AwareRevit.dll` — an `IExternalApplication` add-in loaded by Revit that owns a `NamedPipeServerStream` and uses `IExternalEventHandler` to execute user code on the main thread; (2) `aware-revit.exe` — the AWARE-CLI-facing sidecar that discovers running Revit instances and dispatches over named pipes. Both target net8.0-windows (Revit 2026's runtime). See `docs/superpowers/specs/2026-05-19-revit-exec-design.md` for full design rationale.

**Tech Stack:** .NET 8, C#, Roslyn (`Microsoft.CodeAnalysis.CSharp.Scripting`), `NamedPipeServerStream`/`NamedPipeClientStream`, RevitAPI.dll (reference-only, copy-local false), xUnit for tests, PowerShell for drill harness.

---

## File Structure

```
cli-revit/
├── .gitignore                               # bin/ obj/ Output/drill-summary.md
├── Shared/                                  # protocol DTOs shared by both binaries
│   ├── Shared.csproj                        # net8.0 (no -windows)
│   ├── PipeFrame.cs                         # length-prefixed JSON framing
│   ├── ExecRequest.cs                       # {id, verb, code, args, transaction, timeout_seconds}
│   └── ExecResponse.cs                      # {id, ok, result, error, stack, stdout_log, host_version, host_pid}
├── Sidecar/                                 # aware-revit.exe — the CLI-facing binary
│   ├── cli-revit.csproj                     # net8.0-windows; OutputType=Exe; AssemblyName=aware-revit
│   ├── Program.cs                           # argv parsing + verb dispatch
│   ├── PipeClient.cs                        # NamedPipeClientStream wrapper, retries
│   ├── RevitProcessFinder.cs                # enumerate running Revit.exe + version + addin liveness
│   ├── Receipts.cs                          # {ok, result, host, host_pid, host_version, ...} envelope
│   └── Verbs/
│       ├── Exec.cs
│       ├── ListInstances.cs
│       ├── SendStatus.cs
│       ├── Launch.cs
│       └── Close.cs
├── AwareRevit/                              # AwareRevit.dll — loaded by Revit
│   ├── AwareRevit.csproj                    # net8.0-windows; references RevitAPI.dll CopyLocal=false
│   ├── AddInApplication.cs                  # IExternalApplication: OnStartup → boot pipe server
│   ├── ExecuteEventHandler.cs               # IExternalEventHandler: invoke ScriptEngine on main thread
│   ├── PipeServer.cs                        # NamedPipeServerStream listener loop
│   ├── ScriptEngine.cs                      # Roslyn compile+run + ScriptWrapper logic
│   ├── ResultSerializer.cs                  # JSON-safe shapes for Element/XYZ/Parameter etc.
│   └── AwareRevit.addin                     # Revit add-in manifest
├── Tests/                                   # xUnit tests (no Revit required)
│   ├── cli-revit.Tests.csproj
│   ├── PipeFrameTests.cs
│   ├── PipeClientTests.cs                   # in-process pipe-server stub
│   ├── ReceiptsTests.cs
│   ├── ResultSerializerTests.cs
│   ├── RevitProcessFinderTests.cs
│   └── EndToEndTests.cs                     # sidecar process driven against an in-process stub addin
├── Ingest/
│   ├── run-drill.ps1                        # iterate prompt fixtures, write drill-summary.md
│   └── Output/
│       ├── prompt-01.json ... prompt-20.json
│       └── drill-summary.md                 # gitignored, written by harness
└── install-addin.ps1                        # copy AwareRevit.dll + manifest into Revit addin folder
```

Plus a tiny edit to `20-agents/aeco/architecture/revit-2026/manifest.yaml` wiring the runtime verbs.

---

## Task 1: Scaffold the cli-revit directory + Shared protocol library

**Files:**
- Create: `cli-revit/.gitignore`
- Create: `cli-revit/Shared/Shared.csproj`
- Create: `cli-revit/Shared/ExecRequest.cs`
- Create: `cli-revit/Shared/ExecResponse.cs`

- [ ] **Step 1: Create `cli-revit/.gitignore`**

Content:
```
bin/
obj/
Ingest/Output/drill-summary.md
*.user
```

- [ ] **Step 2: Create `cli-revit/Shared/Shared.csproj`**

Content:
```xml
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <TargetFramework>net8.0</TargetFramework>
    <Nullable>enable</Nullable>
    <ImplicitUsings>enable</ImplicitUsings>
    <RootNamespace>AwareRevit.Shared</RootNamespace>
    <AssemblyName>AwareRevit.Shared</AssemblyName>
    <LangVersion>latest</LangVersion>
    <TreatWarningsAsErrors>true</TreatWarningsAsErrors>
  </PropertyGroup>
  <ItemGroup>
    <PackageReference Include="System.Text.Json" Version="8.0.5" />
  </ItemGroup>
</Project>
```

- [ ] **Step 3: Create `cli-revit/Shared/ExecRequest.cs`**

Content:
```csharp
// DTO sent by aware-revit.exe over the named pipe into AwareRevit.dll.
// Shape matches the AWARE stdin-JSON contract minus the `verb` (the pipe is
// per-verb dispatched on the sidecar side; the add-in only ever runs exec).
namespace AwareRevit.Shared;

public sealed class ExecRequest
{
    public string Id { get; set; } = "";
    public string Code { get; set; } = "";
    public Dictionary<string, object?> Args { get; set; } = new();
    /// <summary>One of: "none" (default), "auto" (wrap in Transaction).</summary>
    public string Transaction { get; set; } = "none";
    public int TimeoutSeconds { get; set; } = 60;
}
```

- [ ] **Step 4: Create `cli-revit/Shared/ExecResponse.cs`**

Content:
```csharp
// DTO returned by AwareRevit.dll over the named pipe to aware-revit.exe.
// The sidecar wraps it in the outer AWARE receipt envelope (adds host="revit",
// verb, delivered_at). Keep this dumb — no transport concerns.
namespace AwareRevit.Shared;

public sealed class ExecResponse
{
    public string Id { get; set; } = "";
    public bool Ok { get; set; }
    /// <summary>JSON-shaped result. Serialized by the add-in's ResultSerializer.</summary>
    public object? Result { get; set; }
    public string? Error { get; set; }
    public string? Stack { get; set; }
    public string StdoutLog { get; set; } = "";
    public string? HostVersion { get; set; }
    public int? HostPid { get; set; }
}
```

- [ ] **Step 5: Verify Shared compiles**

Run: `dotnet build cli-revit/Shared/Shared.csproj -v minimal`
Expected: `Build succeeded.`

- [ ] **Step 6: Commit**

Run:
```
git add cli-revit/.gitignore cli-revit/Shared/Shared.csproj cli-revit/Shared/ExecRequest.cs cli-revit/Shared/ExecResponse.cs
git commit -m "feat(v0.33): scaffold cli-revit + shared protocol DTOs"
```

---

## Task 2: Pipe framing (length-prefixed JSON)

**Files:**
- Create: `cli-revit/Shared/PipeFrame.cs`
- Create: `cli-revit/Tests/cli-revit.Tests.csproj`
- Create: `cli-revit/Tests/PipeFrameTests.cs`

- [ ] **Step 1: Create the tests csproj `cli-revit/Tests/cli-revit.Tests.csproj`**

Content:
```xml
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <TargetFramework>net8.0-windows</TargetFramework>
    <Nullable>enable</Nullable>
    <ImplicitUsings>enable</ImplicitUsings>
    <IsPackable>false</IsPackable>
    <RootNamespace>AwareRevit.Tests</RootNamespace>
    <AssemblyName>AwareRevit.Tests</AssemblyName>
    <LangVersion>latest</LangVersion>
  </PropertyGroup>
  <ItemGroup>
    <PackageReference Include="Microsoft.NET.Test.Sdk" Version="17.11.1" />
    <PackageReference Include="xunit" Version="2.9.2" />
    <PackageReference Include="xunit.runner.visualstudio" Version="2.8.2" />
  </ItemGroup>
  <ItemGroup>
    <ProjectReference Include="..\Shared\Shared.csproj" />
  </ItemGroup>
</Project>
```

- [ ] **Step 2: Write the failing test `cli-revit/Tests/PipeFrameTests.cs`**

Content:
```csharp
using System.IO;
using System.Text;
using AwareRevit.Shared;
using Xunit;

namespace AwareRevit.Tests;

public class PipeFrameTests
{
    [Fact]
    public async Task Roundtrip_PreservesJsonPayload()
    {
        var ms = new MemoryStream();
        await PipeFrame.WriteAsync(ms, "{\"id\":\"abc\",\"ok\":true}");
        ms.Position = 0;
        var got = await PipeFrame.ReadAsync(ms);
        Assert.Equal("{\"id\":\"abc\",\"ok\":true}", got);
    }

    [Fact]
    public async Task Roundtrip_UnicodePayload()
    {
        var ms = new MemoryStream();
        var payload = "{\"msg\":\"héllo 日本 🔥\"}";
        await PipeFrame.WriteAsync(ms, payload);
        ms.Position = 0;
        var got = await PipeFrame.ReadAsync(ms);
        Assert.Equal(payload, got);
    }

    [Fact]
    public async Task ReadAsync_OnEmptyStream_ReturnsNull()
    {
        var ms = new MemoryStream();
        var got = await PipeFrame.ReadAsync(ms);
        Assert.Null(got);
    }

    [Fact]
    public async Task WriteAsync_WritesLengthPrefixBigEndian()
    {
        var ms = new MemoryStream();
        await PipeFrame.WriteAsync(ms, "AB");  // 2-byte payload after UTF-8 encoding
        var bytes = ms.ToArray();
        Assert.Equal(6, bytes.Length); // 4-byte length + 2-byte payload
        // Big-endian length = 0x00000002
        Assert.Equal(0, bytes[0]);
        Assert.Equal(0, bytes[1]);
        Assert.Equal(0, bytes[2]);
        Assert.Equal(2, bytes[3]);
        Assert.Equal((byte)'A', bytes[4]);
        Assert.Equal((byte)'B', bytes[5]);
    }
}
```

- [ ] **Step 3: Run tests — expect compile fail (PipeFrame doesn't exist yet)**

Run: `dotnet test cli-revit/Tests/cli-revit.Tests.csproj -v minimal`
Expected: build fails — `PipeFrame` type not found.

- [ ] **Step 4: Create `cli-revit/Shared/PipeFrame.cs`**

Content:
```csharp
// Length-prefixed JSON framing for the aware-revit pipe protocol. Four bytes
// big-endian length, then UTF-8 JSON payload. One frame per request, one frame
// per response. Streams are otherwise opaque — the caller owns the
// NamedPipe{Server,Client}Stream lifecycle.
using System.Buffers.Binary;
using System.Text;

namespace AwareRevit.Shared;

public static class PipeFrame
{
    /// <summary>Read one frame. Returns null on clean EOF (pipe closed cleanly).</summary>
    public static async Task<string?> ReadAsync(Stream s, CancellationToken ct = default)
    {
        var lenBytes = new byte[4];
        int read = 0;
        while (read < 4)
        {
            var n = await s.ReadAsync(lenBytes.AsMemory(read, 4 - read), ct);
            if (n == 0) return read == 0 ? null : throw new EndOfStreamException("partial length prefix");
            read += n;
        }
        var len = BinaryPrimitives.ReadInt32BigEndian(lenBytes);
        if (len < 0 || len > 64 * 1024 * 1024)
            throw new InvalidDataException($"frame length out of range: {len}");

        var payload = new byte[len];
        read = 0;
        while (read < len)
        {
            var n = await s.ReadAsync(payload.AsMemory(read, len - read), ct);
            if (n == 0) throw new EndOfStreamException("payload truncated");
            read += n;
        }
        return Encoding.UTF8.GetString(payload);
    }

    /// <summary>Write one frame. Flushes the stream before returning.</summary>
    public static async Task WriteAsync(Stream s, string json, CancellationToken ct = default)
    {
        var payload = Encoding.UTF8.GetBytes(json);
        var lenBytes = new byte[4];
        BinaryPrimitives.WriteInt32BigEndian(lenBytes, payload.Length);
        await s.WriteAsync(lenBytes, ct);
        await s.WriteAsync(payload, ct);
        await s.FlushAsync(ct);
    }
}
```

- [ ] **Step 5: Run tests again — expect PASS**

Run: `dotnet test cli-revit/Tests/cli-revit.Tests.csproj -v minimal`
Expected: 4 passing tests.

- [ ] **Step 6: Commit**

Run:
```
git add cli-revit/Shared/PipeFrame.cs cli-revit/Tests/cli-revit.Tests.csproj cli-revit/Tests/PipeFrameTests.cs
git commit -m "feat(v0.33): pipe framing (length-prefixed JSON) + 4 tests"
```

---

## Task 3: Sidecar scaffold + receipt envelope

**Files:**
- Create: `cli-revit/Sidecar/cli-revit.csproj`
- Create: `cli-revit/Sidecar/Program.cs`
- Create: `cli-revit/Sidecar/Receipts.cs`
- Create: `cli-revit/Tests/ReceiptsTests.cs`

- [ ] **Step 1: Create `cli-revit/Sidecar/cli-revit.csproj`**

Content:
```xml
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net8.0-windows</TargetFramework>
    <Nullable>enable</Nullable>
    <ImplicitUsings>enable</ImplicitUsings>
    <RootNamespace>AwareRevit.Sidecar</RootNamespace>
    <AssemblyName>aware-revit</AssemblyName>
    <LangVersion>latest</LangVersion>
    <TreatWarningsAsErrors>true</TreatWarningsAsErrors>
    <Version>0.33.0</Version>
  </PropertyGroup>

  <PropertyGroup Condition="'$(Configuration)' == 'Release'">
    <PublishSingleFile>true</PublishSingleFile>
    <IncludeNativeLibrariesForSelfExtract>true</IncludeNativeLibrariesForSelfExtract>
    <SelfContained>true</SelfContained>
    <RuntimeIdentifier>win-x64</RuntimeIdentifier>
  </PropertyGroup>

  <ItemGroup>
    <ProjectReference Include="..\Shared\Shared.csproj" />
  </ItemGroup>

  <ItemGroup>
    <InternalsVisibleTo Include="AwareRevit.Tests" />
  </ItemGroup>
</Project>
```

- [ ] **Step 2: Create `cli-revit/Sidecar/Program.cs` — stub help only**

Content:
```csharp
// aware-revit — runtime sidecar that brokers between AWARE CLI and the
// in-Revit AwareRevit.dll add-in over a named pipe. Speaks the {verb, code,
// args} stdin-JSON contract shared with cli-tekla / cli-rhino.

namespace AwareRevit.Sidecar;

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

        Console.Error.WriteLine($"aware-revit: unknown verb '{args[0]}'. Try --help.");
        return 2;
    }

    static void PrintHelp()
    {
        Console.WriteLine("""
            aware-revit — Revit sidecar (talks to AwareRevit.dll add-in over named pipes)

            Usage:
              aware-revit <verb> [--json-stdin]
              aware-revit --json-stdin            (verb embedded in JSON body)

            Verbs:
              exec             Compile + run an ad-hoc C# script against the active Revit doc
              list-instances   Print running Revit instances (pid + version + addin status)
              send-status      Show a transient TaskDialog message in Revit
              launch           Start a Revit instance (optionally opening a model)
              close            Cleanly close a Revit instance via the add-in

            Exit codes:
              0  success
              1  no matching Revit instance running
              2  bad args / unknown verb / pipe protocol error
              3  Revit not installed at the requested version
              4  ambiguous target (multiple matches, no --pid)
              6  add-in not loaded (pipe connect failed)
            """);
    }
}
```

- [ ] **Step 3: Write failing tests `cli-revit/Tests/ReceiptsTests.cs`**

Content:
```csharp
using System.Text.Json.Nodes;
using AwareRevit.Sidecar;
using Xunit;

namespace AwareRevit.Tests;

public class ReceiptsTests
{
    [Fact]
    public void ExecOk_HasAllRequiredFields()
    {
        var r = Receipts.ExecOk(JsonNode.Parse("42"), "2026", 1234, "");
        Assert.True(r["ok"]!.GetValue<bool>());
        Assert.Equal("revit", r["host"]!.GetValue<string>());
        Assert.Equal("exec", r["verb"]!.GetValue<string>());
        Assert.Equal("2026", r["host_version"]!.GetValue<string>());
        Assert.Equal(1234, r["host_pid"]!.GetValue<int>());
        Assert.Equal(42, r["result"]!.GetValue<int>());
        Assert.NotNull(r["delivered_at"]);
    }

    [Fact]
    public void ExecFail_SetsOkFalse()
    {
        var r = Receipts.ExecFail("nope", "stack here", "stdout here");
        Assert.False(r["ok"]!.GetValue<bool>());
        Assert.Equal("nope", r["error"]!.GetValue<string>());
        Assert.Equal("stack here", r["stack"]!.GetValue<string>());
        Assert.Equal("stdout here", r["stdout_log"]!.GetValue<string>());
        Assert.Equal("revit", r["host"]!.GetValue<string>());
        Assert.Equal("exec", r["verb"]!.GetValue<string>());
    }

    [Fact]
    public void ListOk_WrapsInstancesArray()
    {
        var arr = new JsonArray { new JsonObject { ["pid"] = 99 } };
        var r = Receipts.ListOk(arr);
        Assert.Equal("ok", r["status"]!.GetValue<string>());
        Assert.Equal("revit", r["host"]!.GetValue<string>());
        Assert.Equal(1, ((JsonArray)r["instances"]!).Count);
    }

    [Fact]
    public void SendStatusOk_IncludesSessionId()
    {
        var r = Receipts.SendStatusOk("hi", "2026", 7777);
        Assert.Equal("revit-7777", r["host_session_id"]!.GetValue<string>());
        Assert.Equal("hi", r["verb_result"]!["message"]!.GetValue<string>());
    }

    [Fact]
    public void GenericFail_BuildsErrEnvelope()
    {
        var r = Receipts.GenericFail("close", "boom");
        Assert.Equal("err", r["status"]!.GetValue<string>());
        Assert.Equal("close", r["verb"]!.GetValue<string>());
        Assert.Equal("boom", r["error"]!.GetValue<string>());
    }
}
```

- [ ] **Step 4: Add a `<ProjectReference Include="..\Sidecar\cli-revit.csproj" />` to `cli-revit/Tests/cli-revit.Tests.csproj`**

The csproj already has the Shared project ref; add Sidecar as a second ProjectReference. Modify the existing `<ItemGroup>` containing `Shared.csproj`:

```xml
  <ItemGroup>
    <ProjectReference Include="..\Shared\Shared.csproj" />
    <ProjectReference Include="..\Sidecar\cli-revit.csproj" />
  </ItemGroup>
```

- [ ] **Step 5: Run tests — expect compile fail (Receipts doesn't exist)**

Run: `dotnet test cli-revit/Tests/cli-revit.Tests.csproj -v minimal`
Expected: build fails — `Receipts` type not found.

- [ ] **Step 6: Create `cli-revit/Sidecar/Receipts.cs`**

Content:
```csharp
// Receipt envelope used by every aware-revit verb. Shape mirrors cli-rhino:
// {ok, result, host, host_pid, host_version, verb, stdout_log, delivered_at}
// for exec; analogous shapes for the other verbs. Compact (no indentation)
// for transport efficiency.

using System.Text.Json.Nodes;

namespace AwareRevit.Sidecar;

internal static class Receipts
{
    public static JsonObject ExecOk(JsonNode? result, string? hostVersion, int? hostPid,
                                    string stdoutLog)
    {
        return new JsonObject
        {
            ["ok"]            = true,
            ["result"]        = result,
            ["host"]          = "revit",
            ["host_version"]  = hostVersion,
            ["host_pid"]      = hostPid,
            ["verb"]          = "exec",
            ["stdout_log"]    = stdoutLog,
            ["delivered_at"]  = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject ExecFail(string error, string stack, string stdoutLog)
    {
        return new JsonObject
        {
            ["ok"]           = false,
            ["error"]        = error,
            ["stack"]        = stack,
            ["host"]         = "revit",
            ["verb"]         = "exec",
            ["stdout_log"]   = stdoutLog,
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject ListOk(JsonArray instances)
    {
        return new JsonObject
        {
            ["status"]       = "ok",
            ["instances"]    = instances,
            ["host"]         = "revit",
            ["verb"]         = "list-instances",
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject SendStatusOk(string message, string? hostVersion, int? hostPid)
    {
        return new JsonObject
        {
            ["status"]           = "ok",
            ["host"]             = "revit",
            ["host_version"]     = hostVersion,
            ["host_pid"]         = hostPid,
            ["host_session_id"]  = hostPid is { } pid ? $"revit-{pid}" : null,
            ["verb"]             = "send-status",
            ["verb_result"]      = new JsonObject { ["message"] = message },
            ["delivered_at"]     = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject LaunchOk(int hostPid, string hostVersion, JsonObject verbResult)
    {
        return new JsonObject
        {
            ["status"]       = "ok",
            ["host"]         = "revit",
            ["host_version"] = hostVersion,
            ["host_pid"]     = hostPid,
            ["verb"]         = "launch",
            ["verb_result"]  = verbResult,
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject CloseOk(int hostPid, string hostVersion, string mode)
    {
        return new JsonObject
        {
            ["status"]       = "ok",
            ["host"]         = "revit",
            ["host_version"] = hostVersion,
            ["host_pid"]     = hostPid,
            ["mode"]         = mode,
            ["verb"]         = "close",
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject GenericFail(string verb, string error, string? stack = null)
    {
        var o = new JsonObject
        {
            ["status"]       = "err",
            ["host"]         = "revit",
            ["verb"]         = verb,
            ["error"]        = error,
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
        if (stack is not null) o["stack"] = stack;
        return o;
    }
}
```

- [ ] **Step 7: Run tests — expect 5 ReceiptsTests + 4 PipeFrameTests passing**

Run: `dotnet test cli-revit/Tests/cli-revit.Tests.csproj -v minimal`
Expected: 9 passing tests total.

- [ ] **Step 8: Commit**

Run:
```
git add cli-revit/Sidecar/cli-revit.csproj cli-revit/Sidecar/Program.cs cli-revit/Sidecar/Receipts.cs cli-revit/Tests/cli-revit.Tests.csproj cli-revit/Tests/ReceiptsTests.cs
git commit -m "feat(v0.33): sidecar scaffold + receipt envelope + 5 tests"
```

---

## Task 4: Pipe client + retry loop

**Files:**
- Create: `cli-revit/Sidecar/PipeClient.cs`
- Create: `cli-revit/Tests/PipeClientTests.cs`

- [ ] **Step 1: Write the failing test `cli-revit/Tests/PipeClientTests.cs`**

Content:
```csharp
using System.IO.Pipes;
using AwareRevit.Shared;
using AwareRevit.Sidecar;
using Xunit;

namespace AwareRevit.Tests;

public class PipeClientTests
{
    /// <summary>Spin up an in-process pipe server on a random name and verify the
    /// client can round-trip a request through it.</summary>
    [Fact]
    public async Task SendRequest_RoundTrips()
    {
        var pipeName = $"aware-revit-test-{Guid.NewGuid():N}";
        var serverTask = Task.Run(async () =>
        {
            using var server = new NamedPipeServerStream(pipeName, PipeDirection.InOut, 1,
                PipeTransmissionMode.Byte, PipeOptions.Asynchronous);
            await server.WaitForConnectionAsync();
            var reqJson = await PipeFrame.ReadAsync(server);
            Assert.NotNull(reqJson);
            Assert.Contains("\"abc\"", reqJson);
            // Echo back a canned response.
            await PipeFrame.WriteAsync(server,
                """{"id":"abc","ok":true,"result":42,"host_version":"2026","host_pid":1111}""");
        });

        var client = new PipeClient(pipeName);
        var respJson = await client.SendRequestAsync(
            """{"id":"abc","code":"return 1;"}""",
            timeout: TimeSpan.FromSeconds(5));

        Assert.Contains("\"ok\":true", respJson);
        Assert.Contains("\"host_version\":\"2026\"", respJson);
        await serverTask;
    }

    [Fact]
    public async Task SendRequest_FailsClearly_WhenPipeMissing()
    {
        var client = new PipeClient($"aware-revit-noexist-{Guid.NewGuid():N}");
        await Assert.ThrowsAsync<TimeoutException>(async () =>
            await client.SendRequestAsync("""{"id":"x"}""", timeout: TimeSpan.FromSeconds(1)));
    }
}
```

- [ ] **Step 2: Run tests — expect compile fail (PipeClient missing)**

Run: `dotnet test cli-revit/Tests/cli-revit.Tests.csproj -v minimal`
Expected: build error referencing `PipeClient`.

- [ ] **Step 3: Create `cli-revit/Sidecar/PipeClient.cs`**

Content:
```csharp
// Connects to AwareRevit.dll's per-process named pipe and round-trips a
// length-prefixed JSON request. Connect-retries with 500ms backoff so we
// tolerate a short race between sidecar launch and add-in OnStartup.

using System.IO.Pipes;
using AwareRevit.Shared;

namespace AwareRevit.Sidecar;

internal sealed class PipeClient
{
    public string PipeName { get; }

    public PipeClient(string pipeName)
    {
        PipeName = pipeName;
    }

    /// <summary>Connect + write request + read response. Retries connect every
    /// 500ms until <paramref name="timeout"/> elapses. Throws TimeoutException
    /// if the pipe never appears, EndOfStreamException if the add-in closes the
    /// pipe mid-frame.</summary>
    public async Task<string> SendRequestAsync(string requestJson, TimeSpan timeout)
    {
        using var pipe = new NamedPipeClientStream(".", PipeName, PipeDirection.InOut,
            PipeOptions.Asynchronous);

        var deadline = DateTime.UtcNow + timeout;
        while (true)
        {
            try
            {
                await pipe.ConnectAsync(250);
                break;
            }
            catch (TimeoutException)
            {
                if (DateTime.UtcNow >= deadline)
                    throw new TimeoutException(
                        $"pipe '{PipeName}' did not appear within {timeout.TotalSeconds:0.0}s " +
                        $"(is AwareRevit.dll loaded? check Revit > Add-Ins ribbon)");
                await Task.Delay(250);
            }
        }

        await PipeFrame.WriteAsync(pipe, requestJson);
        var responseJson = await PipeFrame.ReadAsync(pipe)
            ?? throw new EndOfStreamException("pipe closed before response was sent");
        return responseJson;
    }
}
```

- [ ] **Step 4: Run tests — expect 2 PipeClientTests + 9 previous = 11 PASS**

Run: `dotnet test cli-revit/Tests/cli-revit.Tests.csproj -v minimal`
Expected: 11 passing.

- [ ] **Step 5: Commit**

Run:
```
git add cli-revit/Sidecar/PipeClient.cs cli-revit/Tests/PipeClientTests.cs
git commit -m "feat(v0.33): pipe client (connect retries + length-prefixed JSON round-trip)"
```

---

## Task 5: Revit process discovery

**Files:**
- Create: `cli-revit/Sidecar/RevitProcessFinder.cs`
- Create: `cli-revit/Tests/RevitProcessFinderTests.cs`

- [ ] **Step 1: Write the failing test `cli-revit/Tests/RevitProcessFinderTests.cs`**

Content:
```csharp
using AwareRevit.Sidecar;
using Xunit;

namespace AwareRevit.Tests;

public class RevitProcessFinderTests
{
    [Fact]
    public void ExtractVersionFromPath_HappyPath_2026()
    {
        var v = RevitProcessFinder.ExtractVersionFromPath(
            @"C:\Program Files\Autodesk\Revit 2026\Revit.exe");
        Assert.Equal("2026", v);
    }

    [Fact]
    public void ExtractVersionFromPath_HappyPath_2025()
    {
        var v = RevitProcessFinder.ExtractVersionFromPath(
            @"C:\Program Files\Autodesk\Revit 2025\Revit.exe");
        Assert.Equal("2025", v);
    }

    [Fact]
    public void ExtractVersionFromPath_ReturnsNullOnUnrecognized()
    {
        Assert.Null(RevitProcessFinder.ExtractVersionFromPath(@"C:\nope\Revit.exe"));
    }

    [Fact]
    public void Filter_ByVersion_MatchesSubstring()
    {
        var all = new List<RevitInstance>
        {
            new(1, "2025", "p1", false),
            new(2, "2026", "p2", true),
        };
        var matches = RevitProcessFinder.Filter(all, version: "2026", pid: null);
        Assert.Single(matches);
        Assert.Equal(2, matches[0].Pid);
    }

    [Fact]
    public void Filter_ByPid_MatchesExact()
    {
        var all = new List<RevitInstance>
        {
            new(1, "2025", "p1", false),
            new(2, "2026", "p2", true),
        };
        var matches = RevitProcessFinder.Filter(all, version: null, pid: 1);
        Assert.Single(matches);
        Assert.Equal("2025", matches[0].Version);
    }

    [Fact]
    public void Filter_NoCriteria_ReturnsAll()
    {
        var all = new List<RevitInstance>
        {
            new(1, "2025", "p1", false),
            new(2, "2026", "p2", true),
        };
        var matches = RevitProcessFinder.Filter(all, version: null, pid: null);
        Assert.Equal(2, matches.Count);
    }
}
```

- [ ] **Step 2: Run tests — expect compile fail**

Run: `dotnet test cli-revit/Tests/cli-revit.Tests.csproj -v minimal`
Expected: build error referencing `RevitProcessFinder` / `RevitInstance`.

- [ ] **Step 3: Create `cli-revit/Sidecar/RevitProcessFinder.cs`**

Content:
```csharp
// Enumerates running Revit instances and reshapes them for list-instances
// and verb-target selection. Pure logic is in static helpers so tests can run
// without a Revit install.

using System.Diagnostics;
using System.Text.RegularExpressions;

namespace AwareRevit.Sidecar;

internal sealed record RevitInstance(int Pid, string Version, string ExePath, bool AddinLoaded);

internal static class RevitProcessFinder
{
    /// <summary>Returns every Revit.exe currently running, with version parsed
    /// from MainModule.FileName. AddinLoaded is set later by probing the pipe.</summary>
    public static List<RevitInstance> Enumerate()
    {
        var result = new List<RevitInstance>();
        foreach (var p in Process.GetProcessesByName("Revit"))
        {
            try
            {
                var path = p.MainModule?.FileName;
                if (path is null) continue;
                var version = ExtractVersionFromPath(path);
                if (version is null) continue;
                result.Add(new RevitInstance(p.Id, version, path, AddinLoaded: false));
            }
            catch
            {
                // Inaccessible process (e.g. cross-session) — skip silently.
            }
        }
        return result;
    }

    /// <summary>Parse "Revit 2026" out of an exe path like
    /// "C:\Program Files\Autodesk\Revit 2026\Revit.exe". Returns "2026" or null.</summary>
    public static string? ExtractVersionFromPath(string path)
    {
        // Normalise to forward slashes for the regex.
        var m = Regex.Match(path.Replace('\\', '/'),
            @"/Autodesk/Revit (\d{4})/", RegexOptions.IgnoreCase);
        return m.Success ? m.Groups[1].Value : null;
    }

    /// <summary>Filter a list by version (substring) and/or pid (exact).
    /// Empty criteria → return everything.</summary>
    public static List<RevitInstance> Filter(List<RevitInstance> all, string? version, int? pid)
    {
        IEnumerable<RevitInstance> q = all;
        if (pid is { } p) q = q.Where(i => i.Pid == p);
        if (!string.IsNullOrEmpty(version)) q = q.Where(i => i.Version == version);
        return q.ToList();
    }

    /// <summary>Per-Revit-process pipe name convention.</summary>
    public static string PipeNameFor(int pid) => $"aware-revit-{pid}";

    /// <summary>Probe whether AwareRevit.dll is loaded by trying to connect to
    /// the per-PID pipe with a tiny timeout. Returns the input instance with
    /// AddinLoaded updated; safe to call without pinging Revit twice.</summary>
    public static async Task<RevitInstance> ProbeAddinAsync(RevitInstance instance,
                                                            TimeSpan probe = default)
    {
        if (probe == default) probe = TimeSpan.FromMilliseconds(300);
        var name = PipeNameFor(instance.Pid);
        try
        {
            using var pipe = new System.IO.Pipes.NamedPipeClientStream(
                ".", name, System.IO.Pipes.PipeDirection.InOut,
                System.IO.Pipes.PipeOptions.Asynchronous);
            await pipe.ConnectAsync((int)probe.TotalMilliseconds);
            return instance with { AddinLoaded = true };
        }
        catch
        {
            return instance with { AddinLoaded = false };
        }
    }
}
```

- [ ] **Step 4: Run tests — expect 6 new + 11 previous = 17 PASS**

Run: `dotnet test cli-revit/Tests/cli-revit.Tests.csproj -v minimal`
Expected: 17 passing.

- [ ] **Step 5: Commit**

Run:
```
git add cli-revit/Sidecar/RevitProcessFinder.cs cli-revit/Tests/RevitProcessFinderTests.cs
git commit -m "feat(v0.33): Revit process discovery + pipe-probe + 6 tests"
```

---

## Task 6: list-instances verb (sidecar-side)

**Files:**
- Create: `cli-revit/Sidecar/Verbs/ListInstances.cs`
- Modify: `cli-revit/Sidecar/Program.cs`

- [ ] **Step 1: Create `cli-revit/Sidecar/Verbs/ListInstances.cs`**

Content:
```csharp
// list-instances verb — enumerate running Revit.exe processes, probe each
// for the AwareRevit add-in pipe, emit a JSON receipt. Does not invoke any
// Revit API (process listing + cheap pipe probe only).

using System.Text.Json.Nodes;

namespace AwareRevit.Sidecar.Verbs;

internal static class ListInstances
{
    public static async Task<int> RunAsync()
    {
        var raw = RevitProcessFinder.Enumerate();
        var probed = new List<RevitInstance>();
        foreach (var inst in raw)
            probed.Add(await RevitProcessFinder.ProbeAddinAsync(inst));

        var arr = new JsonArray();
        foreach (var i in probed)
        {
            arr.Add(new JsonObject
            {
                ["pid"]          = i.Pid,
                ["version"]      = i.Version,
                ["exe_path"]     = i.ExePath,
                ["addin_loaded"] = i.AddinLoaded,
            });
        }
        Console.WriteLine(Receipts.ListOk(arr).ToJsonString());
        return 0;
    }
}
```

- [ ] **Step 2: Wire `list-instances` into `cli-revit/Sidecar/Program.cs`**

Replace the body of `Main` to dispatch verbs. Find the block:
```csharp
        Console.Error.WriteLine($"aware-revit: unknown verb '{args[0]}'. Try --help.");
        return 2;
```
Replace it with:
```csharp
        // Two invocation styles (same as cli-tekla / cli-rhino):
        //   aware-revit.exe <verb> [--json-stdin]   (canonical)
        //   aware-revit.exe --json-stdin            (verb embedded in JSON body)
        string verb;
        System.Text.Json.Nodes.JsonNode? stdinJson = null;

        if (args[0].StartsWith("--", StringComparison.Ordinal))
        {
            string buf;
            try { buf = Console.In.ReadToEnd(); }
            catch (Exception e)
            {
                Console.Error.WriteLine($"aware-revit: stdin not readable: {e.Message}");
                return 2;
            }
            try { stdinJson = System.Text.Json.Nodes.JsonNode.Parse(buf); }
            catch (Exception e)
            {
                Console.Error.WriteLine($"aware-revit: stdin not JSON: {e.Message}");
                return 2;
            }
            verb = stdinJson?["verb"]?.GetValue<string>() ?? "";
            if (string.IsNullOrEmpty(verb))
            {
                Console.Error.WriteLine("aware-revit: stdin JSON has no `verb` field and no verb on CLI");
                return 2;
            }
        }
        else
        {
            verb = args[0];
            var wantsStdin = args.Skip(1).Any(a => a == "--json-stdin");
            if (wantsStdin || verb is "exec" or "send-status" or "launch" or "close")
            {
                string buf;
                try { buf = Console.In.ReadToEnd(); }
                catch (Exception e)
                {
                    Console.Error.WriteLine($"aware-revit: stdin not readable: {e.Message}");
                    return 2;
                }
                if (!string.IsNullOrWhiteSpace(buf))
                {
                    try { stdinJson = System.Text.Json.Nodes.JsonNode.Parse(buf); }
                    catch (Exception e)
                    {
                        Console.Error.WriteLine($"aware-revit: stdin not JSON: {e.Message}");
                        return 2;
                    }
                }
            }
        }

        return verb switch
        {
            "list-instances" => Verbs.ListInstances.RunAsync().GetAwaiter().GetResult(),
            _ => Unknown(verb),
        };
    }

    static int Unknown(string verb)
    {
        Console.Error.WriteLine($"aware-revit: unknown verb '{verb}'. Try --help.");
        return 2;
```

(The new `static int Unknown` method replaces the old fallback `Console.Error.WriteLine`.)

- [ ] **Step 3: Build the sidecar — expect no errors**

Run: `dotnet build cli-revit/Sidecar/cli-revit.csproj -v minimal`
Expected: `Build succeeded.`

- [ ] **Step 4: Smoke-test list-instances against the real machine**

Run:
```
echo {} | dotnet run --project cli-revit/Sidecar/cli-revit.csproj -- list-instances --json-stdin
```
Expected: a JSON receipt with `"status":"ok"`. If Revit isn't running, `instances` is an empty array.

- [ ] **Step 5: Commit**

Run:
```
git add cli-revit/Sidecar/Verbs/ListInstances.cs cli-revit/Sidecar/Program.cs
git commit -m "feat(v0.33): list-instances verb + Program.cs verb-dispatch"
```

---

## Task 7: ScriptEngine (sidecar-internal port) + tests

The add-in will host its own ScriptEngine (because Roslyn must reference RevitAPI), but the pure splitter logic is identical. To keep the test surface inside the no-Revit Tests project, we expose the splitter in the Sidecar assembly (under `Sidecar/ScriptWrapper.cs`) and the add-in copies the same logic in `AwareRevit/ScriptEngine.cs`. This duplicates ~80 lines of regex but avoids dragging RevitAPI into the test runner.

**Files:**
- Create: `cli-revit/Sidecar/ScriptWrapper.cs`
- Create: `cli-revit/Tests/ScriptWrapperTests.cs`

- [ ] **Step 1: Write the failing tests `cli-revit/Tests/ScriptWrapperTests.cs`**

Content:
```csharp
using AwareRevit.Sidecar;
using Xunit;

namespace AwareRevit.Tests;

public class ScriptWrapperTests
{
    [Fact]
    public void SplitUsings_NoUsings_BodyOnly()
    {
        var (usings, body) = ScriptWrapper.SplitUsingsAndBody("var x = 1;\nreturn x;");
        Assert.Empty(usings);
        Assert.Equal("var x = 1;\nreturn x;", body.Trim());
    }

    [Fact]
    public void SplitUsings_MixedDirectives_Aliases()
    {
        var (usings, body) = ScriptWrapper.SplitUsingsAndBody(
            "using Autodesk.Revit.DB;\nusing DB = Autodesk.Revit.DB;\nvar x = 1;");
        Assert.Equal(2, usings.Count);
        Assert.DoesNotContain("using ", body);
    }

    [Fact]
    public void SplitUsings_DoesNotMatchUsingStatement()
    {
        var (usings, body) = ScriptWrapper.SplitUsingsAndBody(
            "using (var t = new Transaction(doc)) { t.Start(); }\nreturn null;");
        Assert.Empty(usings);
        Assert.Contains("using (var t", body);
    }

    [Fact]
    public void Wrap_AddsPreambleAndAwareRunFunction()
    {
        var wrapped = ScriptWrapper.Wrap("return 42;");
        Assert.Contains("__AwareRun", wrapped);
        Assert.Contains("using Autodesk.Revit.DB;", wrapped);
        Assert.Contains("return 42;", wrapped);
    }

    [Fact]
    public void Wrap_EmptyBody_DefaultsToReturnNull()
    {
        var wrapped = ScriptWrapper.Wrap("");
        Assert.Contains("return null;", wrapped);
    }

    [Fact]
    public void Wrap_DedupesUsingsAgainstPreamble()
    {
        var wrapped = ScriptWrapper.Wrap("using Autodesk.Revit.DB;\nreturn 1;");
        var count = System.Text.RegularExpressions.Regex.Matches(
            wrapped, @"^using Autodesk\.Revit\.DB;\s*$",
            System.Text.RegularExpressions.RegexOptions.Multiline).Count;
        Assert.Equal(1, count);
    }
}
```

- [ ] **Step 2: Run tests — expect compile fail**

Run: `dotnet test cli-revit/Tests/cli-revit.Tests.csproj -v minimal`
Expected: build error — `ScriptWrapper` missing.

- [ ] **Step 3: Create `cli-revit/Sidecar/ScriptWrapper.cs`**

Content:
```csharp
// Splits user-supplied C# into `using` directives + body, wraps the body in
// a synthetic local function so `return X;` semantics work. Mirrors cli-tekla
// and cli-rhino's wrappers. The actual compile+run lives in AwareRevit.dll's
// ScriptEngine.cs which copies these literals (the add-in adds Revit-specific
// references that the sidecar doesn't reference).

using System.Text;
using System.Text.RegularExpressions;

namespace AwareRevit.Sidecar;

internal static class ScriptWrapper
{
    static readonly Regex UsingDirectiveRe = new(
        @"^\s*using\s+(?:[A-Za-z_][A-Za-z0-9_]*\s*=\s*)?[A-Za-z_][A-Za-z0-9_.]*\s*;\s*$",
        RegexOptions.Compiled);

    /// <summary>Preamble usings — kept in sync with AwareRevit/ScriptEngine.cs.</summary>
    public static readonly string[] PreambleUsings =
    {
        "using System;",
        "using System.Collections.Generic;",
        "using System.Linq;",
        "using System.IO;",
        "using System.Text.Json;",
        "using Autodesk.Revit.ApplicationServices;",
        "using Autodesk.Revit.Attributes;",
        "using Autodesk.Revit.DB;",
        "using Autodesk.Revit.DB.Structure;",
        "using Autodesk.Revit.UI;",
        "using Autodesk.Revit.UI.Selection;",
    };

    public static (List<string> usings, string body) SplitUsingsAndBody(string code)
    {
        var usings = new List<string>();
        var bodyLines = new List<string>();
        bool inDirectives = true;

        foreach (var line in code.Replace("\r\n", "\n").Split('\n'))
        {
            if (inDirectives && UsingDirectiveRe.IsMatch(line))
            {
                usings.Add(line.Trim());
            }
            else if (inDirectives && string.IsNullOrWhiteSpace(line))
            {
                // Whitespace between directives is allowed; doesn't break the directive block.
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
        var preambleSet = new HashSet<string>(PreambleUsings, StringComparer.Ordinal);
        var extraUsings = userUsings.Where(u => !preambleSet.Contains(u)).Distinct().ToList();

        var hasReturn = Regex.IsMatch(body, @"\breturn\b");
        var bodyOut = hasReturn ? body : (body.TrimEnd() + "\nreturn null;");
        if (string.IsNullOrWhiteSpace(body)) bodyOut = "return null;";

        var sb = new StringBuilder();
        sb.AppendLine("// AWARE-generated script (do not edit)");
        foreach (var u in PreambleUsings) sb.AppendLine(u);
        foreach (var u in extraUsings) sb.AppendLine(u);
        sb.AppendLine();
        sb.AppendLine("static object? __AwareRun(dynamic uiapp, IDictionary<string, object?> args)");
        sb.AppendLine("{");
        foreach (var line in bodyOut.Split('\n')) sb.AppendLine("    " + line);
        sb.AppendLine("}");
        sb.AppendLine();
        sb.AppendLine("return __AwareRun(uiapp, args);");

        return sb.ToString();
    }
}
```

- [ ] **Step 4: Run tests — expect 6 new + 17 previous = 23 PASS**

Run: `dotnet test cli-revit/Tests/cli-revit.Tests.csproj -v minimal`
Expected: 23 passing.

- [ ] **Step 5: Commit**

Run:
```
git add cli-revit/Sidecar/ScriptWrapper.cs cli-revit/Tests/ScriptWrapperTests.cs
git commit -m "feat(v0.33): ScriptWrapper (splits usings, wraps body) + 6 tests"
```

---

## Task 8: AwareRevit add-in scaffold (csproj + manifest + AddInApplication stub)

We compile this against RevitAPI.dll from the local install and ship the addin manifest. The csproj sets `<Private>false</Private>` on Revit refs so we don't deploy Revit's own DLLs.

**Files:**
- Create: `cli-revit/AwareRevit/AwareRevit.csproj`
- Create: `cli-revit/AwareRevit/AddInApplication.cs`
- Create: `cli-revit/AwareRevit/AwareRevit.addin`

- [ ] **Step 1: Create `cli-revit/AwareRevit/AwareRevit.csproj`**

Content:
```xml
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <TargetFramework>net8.0-windows</TargetFramework>
    <Nullable>enable</Nullable>
    <ImplicitUsings>enable</ImplicitUsings>
    <RootNamespace>AwareRevit.AddIn</RootNamespace>
    <AssemblyName>AwareRevit</AssemblyName>
    <LangVersion>latest</LangVersion>
    <TreatWarningsAsErrors>true</TreatWarningsAsErrors>
    <UseWindowsForms>false</UseWindowsForms>
    <UseWPF>false</UseWPF>
    <!-- We are a library loaded by Revit; no entry point. -->
    <OutputType>Library</OutputType>
    <Version>0.33.0</Version>
    <!-- Revit's loader sets the CWD to the Revit install; we don't need
         OS DLL deployment. CopyLocal=false on Revit refs below. -->
    <CopyLocalLockFileAssemblies>true</CopyLocalLockFileAssemblies>
  </PropertyGroup>

  <ItemGroup>
    <PackageReference Include="Microsoft.CodeAnalysis.CSharp.Scripting" Version="4.12.0" />
    <PackageReference Include="System.Text.Json" Version="8.0.5" />
  </ItemGroup>

  <ItemGroup>
    <ProjectReference Include="..\Shared\Shared.csproj" />
  </ItemGroup>

  <!-- Reference Revit's API assemblies from the installed Revit 2026.
       <Private>false</Private> means "do not copy to output" — Revit
       already owns these in its load context. -->
  <ItemGroup>
    <Reference Include="RevitAPI">
      <HintPath>C:\Program Files\Autodesk\Revit 2026\RevitAPI.dll</HintPath>
      <Private>false</Private>
    </Reference>
    <Reference Include="RevitAPIUI">
      <HintPath>C:\Program Files\Autodesk\Revit 2026\RevitAPIUI.dll</HintPath>
      <Private>false</Private>
    </Reference>
  </ItemGroup>

  <!-- Copy the .addin manifest next to the DLL so install-addin.ps1 can grab both. -->
  <ItemGroup>
    <None Include="AwareRevit.addin">
      <CopyToOutputDirectory>PreserveNewest</CopyToOutputDirectory>
    </None>
  </ItemGroup>
</Project>
```

- [ ] **Step 2: Create `cli-revit/AwareRevit/AwareRevit.addin` (manifest)**

Content:
```xml
<?xml version="1.0" encoding="utf-8"?>
<RevitAddIns>
  <AddIn Type="Application">
    <Name>AWARE Runtime Bridge</Name>
    <!-- install-addin.ps1 rewrites this to the absolute deployed path. -->
    <Assembly>AwareRevit.dll</Assembly>
    <ClientId>4F2A1D6C-9B81-4C7E-A0F5-1E3C58A6D9C0</ClientId>
    <FullClassName>AwareRevit.AddIn.AddInApplication</FullClassName>
    <VendorId>AWRE</VendorId>
    <VendorDescription>AWARE Project — https://github.com/aware-aeco/aware</VendorDescription>
  </AddIn>
</RevitAddIns>
```

- [ ] **Step 3: Create `cli-revit/AwareRevit/AddInApplication.cs` (stub OnStartup only)**

Content:
```csharp
// AwareRevit — Revit 2026 add-in for the AWARE runtime. Loaded by Revit at
// startup via AwareRevit.addin (placed in %APPDATA%\Autodesk\Revit\Addins\2026\).
// OnStartup boots a NamedPipeServerStream listener on "aware-revit-<PID>" and
// registers a single IExternalEventHandler for marshaling user code onto
// Revit's main thread.

using Autodesk.Revit.UI;

namespace AwareRevit.AddIn;

public sealed class AddInApplication : IExternalApplication
{
    PipeServer? _server;
    ExecuteEventHandler? _handler;
    ExternalEvent? _event;

    public Result OnStartup(UIControlledApplication app)
    {
        try
        {
            _handler = new ExecuteEventHandler();
            _event = ExternalEvent.Create(_handler);
            _server = new PipeServer(_handler, _event, GetUIApplication(app));
            _server.Start();
            return Result.Succeeded;
        }
        catch (Exception ex)
        {
            // Don't take Revit down if our boot fails — log to the journal and
            // return Failed so Revit shows our name red in the add-ins ribbon.
            TaskDialog.Show("AWARE", $"OnStartup failed: {ex.Message}\n\n{ex.StackTrace}");
            return Result.Failed;
        }
    }

    public Result OnShutdown(UIControlledApplication app)
    {
        try { _server?.Stop(); } catch { /* best effort */ }
        return Result.Succeeded;
    }

    /// <summary>Bridge from UIControlledApplication (what OnStartup gets) to
    /// UIApplication (what user code needs). Revit doesn't expose UIApplication
    /// directly at OnStartup; we capture it on the first idle event.</summary>
    static UIApplicationProvider GetUIApplication(UIControlledApplication app)
    {
        var provider = new UIApplicationProvider();
        app.Idling += (sender, e) =>
        {
            if (sender is UIApplication ui) provider.Set(ui);
        };
        return provider;
    }
}

/// <summary>Thread-safe holder: the pipe server captures this at boot,
/// and the Idling event populates it later (once Revit has a real UIApplication).</summary>
internal sealed class UIApplicationProvider
{
    UIApplication? _ui;
    readonly object _lock = new();
    public void Set(UIApplication ui)
    {
        lock (_lock) { _ui ??= ui; }
    }
    public UIApplication? Get()
    {
        lock (_lock) { return _ui; }
    }
}
```

- [ ] **Step 4: Build the add-in csproj to verify references resolve**

Run: `dotnet build cli-revit/AwareRevit/AwareRevit.csproj -v minimal`
Expected: build fails — `PipeServer` and `ExecuteEventHandler` types missing (we'll add them in Task 9). The point of this step is to confirm RevitAPI.dll references resolve.

Verify the failure messages mention `PipeServer` / `ExecuteEventHandler` (not `Autodesk.Revit.UI`). If `Autodesk.Revit.UI` is unresolved, fix the HintPath in the csproj.

- [ ] **Step 5: Commit**

Run:
```
git add cli-revit/AwareRevit/AwareRevit.csproj cli-revit/AwareRevit/AwareRevit.addin cli-revit/AwareRevit/AddInApplication.cs
git commit -m "feat(v0.33): AwareRevit add-in scaffold (csproj + addin manifest + OnStartup)"
```

---

## Task 9: ExecuteEventHandler + PipeServer (the add-in's IPC)

**Files:**
- Create: `cli-revit/AwareRevit/ExecuteEventHandler.cs`
- Create: `cli-revit/AwareRevit/PipeServer.cs`

- [ ] **Step 1: Create `cli-revit/AwareRevit/ExecuteEventHandler.cs`**

Content:
```csharp
// IExternalEventHandler — Revit's ONLY safe cross-thread mechanism. The pipe
// server runs on a background thread; when a request arrives it enqueues an
// ExecRequest and calls ExternalEvent.Raise(). Revit calls Execute() on the
// main thread the next time it goes idle. Execute() pops the request, runs
// the ScriptEngine against the live UIApplication, and stashes the response
// for the pipe server to pick up.

using System.Collections.Concurrent;
using Autodesk.Revit.UI;
using AwareRevit.Shared;

namespace AwareRevit.AddIn;

internal sealed class ExecuteEventHandler : IExternalEventHandler
{
    readonly ConcurrentQueue<PendingRequest> _queue = new();

    public string GetName() => "AwareRevit.ExecuteEventHandler";

    /// <summary>Called by the pipe server thread. Returns a Task that completes
    /// when Execute() finishes the work. Holds at most one outstanding request
    /// per UI idle cycle (the queue is drained by Execute()).</summary>
    public Task<ExecResponse> Enqueue(ExecRequest req, ExternalEvent ev,
                                      UIApplicationProvider uiProvider)
    {
        var tcs = new TaskCompletionSource<ExecResponse>();
        _queue.Enqueue(new PendingRequest(req, tcs, uiProvider));
        ev.Raise();
        return tcs.Task;
    }

    public void Execute(UIApplication app)
    {
        while (_queue.TryDequeue(out var pending))
        {
            try
            {
                var resp = ScriptEngine.RunOnMainThread(app, pending.Req);
                pending.Tcs.TrySetResult(resp);
            }
            catch (Exception ex)
            {
                pending.Tcs.TrySetResult(new ExecResponse
                {
                    Id = pending.Req.Id,
                    Ok = false,
                    Error = $"add-in execute crashed: {ex.GetType().Name}: {ex.Message}",
                    Stack = ex.StackTrace ?? "",
                    StdoutLog = "",
                    HostPid = System.Diagnostics.Process.GetCurrentProcess().Id,
                    HostVersion = app.Application.VersionNumber,
                });
            }
        }
    }

    sealed record PendingRequest(
        ExecRequest Req,
        TaskCompletionSource<ExecResponse> Tcs,
        UIApplicationProvider UiProvider);
}
```

- [ ] **Step 2: Create `cli-revit/AwareRevit/PipeServer.cs`**

Content:
```csharp
// NamedPipeServerStream listener inside Revit's process. Owns a worker thread
// that ConnectAsync-loops, reads one length-prefixed JSON request per
// connection, marshals it through the IExternalEventHandler, writes the
// response back, closes the pipe, opens a new one. Single connection at a
// time (the sidecar serializes its requests).

using System.IO.Pipes;
using System.Text.Json;
using Autodesk.Revit.UI;
using AwareRevit.Shared;

namespace AwareRevit.AddIn;

internal sealed class PipeServer
{
    static readonly JsonSerializerOptions JsonOpts = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower,
        DefaultIgnoreCondition = System.Text.Json.Serialization.JsonIgnoreCondition.WhenWritingNull,
    };

    readonly ExecuteEventHandler _handler;
    readonly ExternalEvent _event;
    readonly UIApplicationProvider _uiProvider;
    readonly string _pipeName;
    readonly CancellationTokenSource _cts = new();
    Thread? _listener;

    public PipeServer(ExecuteEventHandler handler, ExternalEvent ev, UIApplicationProvider uiProvider)
    {
        _handler = handler;
        _event = ev;
        _uiProvider = uiProvider;
        var pid = System.Diagnostics.Process.GetCurrentProcess().Id;
        _pipeName = $"aware-revit-{pid}";
    }

    public string PipeName => _pipeName;

    public void Start()
    {
        _listener = new Thread(() => ListenLoop(_cts.Token))
        {
            IsBackground = true,
            Name = "AwareRevit.PipeServer",
        };
        _listener.Start();
    }

    public void Stop()
    {
        _cts.Cancel();
        // The listening NamedPipeServerStream.WaitForConnectionAsync respects
        // the token; the loop exits naturally.
    }

    async void ListenLoop(CancellationToken ct)
    {
        while (!ct.IsCancellationRequested)
        {
            try
            {
                await using var server = new NamedPipeServerStream(
                    _pipeName,
                    PipeDirection.InOut,
                    maxNumberOfServerInstances: 1,
                    PipeTransmissionMode.Byte,
                    PipeOptions.Asynchronous);

                await server.WaitForConnectionAsync(ct);
                await HandleOne(server, ct);
            }
            catch (OperationCanceledException) { /* stopping */ }
            catch (Exception ex)
            {
                // Don't kill the loop on per-connection errors; just log.
                try { System.Diagnostics.Debug.WriteLine($"AwareRevit pipe error: {ex}"); }
                catch { }
                // Short backoff so a broken state doesn't busy-loop.
                try { await Task.Delay(500, ct); } catch { }
            }
        }
    }

    async Task HandleOne(NamedPipeServerStream server, CancellationToken ct)
    {
        var reqJson = await PipeFrame.ReadAsync(server, ct);
        if (reqJson is null) return;

        ExecRequest? req;
        try
        {
            req = JsonSerializer.Deserialize<ExecRequest>(reqJson, JsonOpts);
        }
        catch (Exception ex)
        {
            var bad = new ExecResponse
            {
                Id = "",
                Ok = false,
                Error = $"request JSON parse failed: {ex.Message}",
                StdoutLog = "",
            };
            await PipeFrame.WriteAsync(server, JsonSerializer.Serialize(bad, JsonOpts), ct);
            return;
        }
        if (req is null)
        {
            var bad = new ExecResponse
            {
                Id = "",
                Ok = false,
                Error = "request JSON deserialised to null",
                StdoutLog = "",
            };
            await PipeFrame.WriteAsync(server, JsonSerializer.Serialize(bad, JsonOpts), ct);
            return;
        }

        var resp = await _handler.Enqueue(req, _event, _uiProvider);
        var respJson = JsonSerializer.Serialize(resp, JsonOpts);
        await PipeFrame.WriteAsync(server, respJson, ct);
    }
}
```

- [ ] **Step 3: Build add-in — expect compile fail (`ScriptEngine` missing)**

Run: `dotnet build cli-revit/AwareRevit/AwareRevit.csproj -v minimal`
Expected: build error — `ScriptEngine` type not found.

- [ ] **Step 4: Commit (the add-in won't build yet; that's the next task — keep the commits atomic)**

We commit even though build fails so each PR-review commit is small. The next task introduces ScriptEngine which makes the whole thing build green.

Run:
```
git add cli-revit/AwareRevit/ExecuteEventHandler.cs cli-revit/AwareRevit/PipeServer.cs
git commit -m "feat(v0.33): add-in pipe server + IExternalEventHandler marshalling (build-incomplete; ScriptEngine next)"
```

---

## Task 10: ScriptEngine (Roslyn compile+run in Revit) + ResultSerializer

**Files:**
- Create: `cli-revit/AwareRevit/ScriptEngine.cs`
- Create: `cli-revit/AwareRevit/ResultSerializer.cs`
- Create: `cli-revit/Tests/ResultSerializerTests.cs` (Sidecar mirror — see step 5)

Note: ResultSerializer in the add-in is Revit-aware (knows about Element / XYZ / Parameter). We TDD-test a sidecar-side mirror against `Dictionary`, primitives, and anonymous types so the core logic is covered without dragging RevitAPI into the test runner.

- [ ] **Step 1: Create `cli-revit/AwareRevit/ResultSerializer.cs`**

Content:
```csharp
// JSON-shaping serializer for Revit return values. Defensive: primitives are
// emitted as-is, Element gets {id, category, name, level?}, ElementId becomes
// its integer, XYZ becomes {x,y,z}, Parameter becomes {name,type,value}, and
// unknown types fall back to ToString(). IEnumerable / IDictionary recurse.
// Mirrors the behaviour of cli-tekla's SerializeResult but adds the Revit-
// specific shapes the AI orchestrators reach for most often.

using System.Collections;
using System.Text.Json;
using System.Text.Json.Nodes;
using Autodesk.Revit.DB;

namespace AwareRevit.AddIn;

internal static class ResultSerializer
{
    public static JsonNode? ToJson(object? value)
    {
        if (value is null) return null;
        switch (value)
        {
            case bool b:        return JsonValue.Create(b);
            case int i:         return JsonValue.Create(i);
            case long l:        return JsonValue.Create(l);
            case double d:      return JsonValue.Create(d);
            case float f:       return JsonValue.Create((double)f);
            case decimal m:     return JsonValue.Create(m);
            case string s:      return JsonValue.Create(s);
            case Guid g:        return JsonValue.Create(g.ToString());
            case DateTime dt:   return JsonValue.Create(dt.ToString("o"));
            case JsonNode jn:   return jn;
        }

        // Revit-specific shapes.
        if (value is ElementId eid)
            return JsonValue.Create(eid.Value);

        if (value is XYZ xyz)
            return new JsonObject { ["x"] = xyz.X, ["y"] = xyz.Y, ["z"] = xyz.Z };

        if (value is Parameter p)
            return new JsonObject
            {
                ["name"]  = p.Definition?.Name,
                ["type"]  = p.StorageType.ToString(),
                ["value"] = ParameterValue(p),
            };

        if (value is Element e)
            return new JsonObject
            {
                ["id"]       = e.Id?.Value,
                ["category"] = e.Category?.Name,
                ["name"]     = e.Name,
            };

        if (value is IDictionary id)
        {
            var jo = new JsonObject();
            foreach (DictionaryEntry kvp in id)
                jo[kvp.Key?.ToString() ?? ""] = ToJson(kvp.Value);
            return jo;
        }

        if (value is IEnumerable enumerable and not string)
        {
            var ja = new JsonArray();
            foreach (var item in enumerable) ja.Add(ToJson(item));
            return ja;
        }

        // Anonymous types / records / POCOs → System.Text.Json with cycle protection.
        try
        {
            var json = JsonSerializer.Serialize(value, new JsonSerializerOptions
            {
                MaxDepth = 6,
                ReferenceHandler = System.Text.Json.Serialization.ReferenceHandler.IgnoreCycles,
            });
            return JsonNode.Parse(json);
        }
        catch
        {
            return JsonValue.Create($"{value.GetType().FullName}: {value}");
        }
    }

    static JsonNode? ParameterValue(Parameter p)
    {
        return p.StorageType switch
        {
            StorageType.Integer  => JsonValue.Create(p.AsInteger()),
            StorageType.Double   => JsonValue.Create(p.AsDouble()),
            StorageType.String   => JsonValue.Create(p.AsString() ?? ""),
            StorageType.ElementId => JsonValue.Create(p.AsElementId()?.Value),
            _ => JsonValue.Create(p.AsValueString() ?? ""),
        };
    }
}
```

- [ ] **Step 2: Create `cli-revit/AwareRevit/ScriptEngine.cs`**

Content:
```csharp
// Roslyn-script host that compiles + runs the user's wrapped C# against the
// currently-loaded RevitAPI inside Revit's AppDomain. The references come
// from already-loaded assemblies (Assembly.Location on the in-Revit
// RevitAPI / RevitAPIUI). Globals expose `uiapp` (dynamic UIApplication)
// and `args` (input dict).

using System.Reflection;
using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.CSharp.Scripting;
using Microsoft.CodeAnalysis.Scripting;
using Autodesk.Revit.UI;
using Autodesk.Revit.DB;
using AwareRevit.Shared;

namespace AwareRevit.AddIn;

/// <summary>Roslyn-script globals — exposed to user C# as `uiapp` and `args`.
/// MUST be public + non-nested so Roslyn's dynamically-generated Submission#N
/// type can access it from a different assembly.</summary>
public sealed class ExecGlobals
{
    public dynamic uiapp { get; set; } = null!;
    public IDictionary<string, object?> args { get; set; } = new Dictionary<string, object?>();
}

internal static class ScriptEngine
{
    static readonly string[] PreambleUsings =
    {
        "System",
        "System.Collections.Generic",
        "System.Linq",
        "System.IO",
        "System.Text.Json",
        "Autodesk.Revit.ApplicationServices",
        "Autodesk.Revit.Attributes",
        "Autodesk.Revit.DB",
        "Autodesk.Revit.DB.Structure",
        "Autodesk.Revit.UI",
        "Autodesk.Revit.UI.Selection",
    };

    public static ExecResponse RunOnMainThread(UIApplication ui, ExecRequest req)
    {
        var pid = System.Diagnostics.Process.GetCurrentProcess().Id;
        var ver = ui.Application.VersionNumber;
        var stdoutCapture = new System.IO.StringWriter();
        var originalOut = Console.Out;
        Console.SetOut(stdoutCapture);

        try
        {
            var refs = ResolveReferences();
            var opts = ScriptOptions.Default
                .WithReferences(refs)
                .WithImports(PreambleUsings)
                .WithEmitDebugInformation(false);

            var globals = new ExecGlobals
            {
                uiapp = ui,
                args  = req.Args ?? new Dictionary<string, object?>(),
            };

            var script = CSharpScript.Create<object>(req.Code, opts, typeof(ExecGlobals));
            object? result;
            if (req.Transaction == "auto")
            {
                using var tx = new Transaction(ui.ActiveUIDocument.Document, "AWARE exec");
                tx.Start();
                try
                {
                    result = script.RunAsync(globals).GetAwaiter().GetResult().ReturnValue;
                    tx.Commit();
                }
                catch
                {
                    if (tx.HasStarted()) tx.RollBack();
                    throw;
                }
            }
            else
            {
                result = script.RunAsync(globals).GetAwaiter().GetResult().ReturnValue;
            }

            return new ExecResponse
            {
                Id          = req.Id,
                Ok          = true,
                Result      = ResultSerializer.ToJson(result),
                StdoutLog   = stdoutCapture.ToString(),
                HostVersion = ver,
                HostPid     = pid,
            };
        }
        catch (CompilationErrorException ce)
        {
            var diag = string.Join("\n", ce.Diagnostics.Select(d => d.ToString()));
            return new ExecResponse
            {
                Id          = req.Id,
                Ok          = false,
                Error       = $"compile error: {ce.Message}",
                Stack       = diag,
                StdoutLog   = stdoutCapture.ToString(),
                HostVersion = ver,
                HostPid     = pid,
            };
        }
        catch (Exception ex)
        {
            var root = ex;
            while (root is TargetInvocationException && root.InnerException is not null)
                root = root.InnerException;
            return new ExecResponse
            {
                Id          = req.Id,
                Ok          = false,
                Error       = $"{root.GetType().Name}: {root.Message}",
                Stack       = root.StackTrace ?? "",
                StdoutLog   = stdoutCapture.ToString(),
                HostVersion = ver,
                HostPid     = pid,
            };
        }
        finally
        {
            Console.SetOut(originalOut);
        }
    }

    static List<MetadataReference> ResolveReferences()
    {
        // Use already-loaded Revit + BCL assemblies. Assembly.Location is the
        // safest source here: Revit's loader places its DLLs on disk under
        // C:\Program Files\Autodesk\Revit 2026\, so Location is populated.
        var refs = new List<MetadataReference>();
        void Add(Type t)
        {
            try
            {
                var loc = t.Assembly.Location;
                if (!string.IsNullOrEmpty(loc) && File.Exists(loc))
                    refs.Add(MetadataReference.CreateFromFile(loc));
            }
            catch { /* skip unresolvable */ }
        }

        Add(typeof(object));                       // System.Private.CoreLib
        Add(typeof(System.Linq.Enumerable));        // System.Linq
        Add(typeof(System.Collections.Generic.IDictionary<,>));
        Add(typeof(System.Dynamic.DynamicObject));  // System.Core
        Add(typeof(Microsoft.CSharp.RuntimeBinder.Binder));
        Add(typeof(System.Text.Json.JsonSerializer));
        Add(typeof(Autodesk.Revit.UI.UIApplication));  // RevitAPIUI
        Add(typeof(Autodesk.Revit.DB.Document));       // RevitAPI

        // De-dup by file path.
        return refs
            .GroupBy(r => (r as PortableExecutableReference)?.FilePath ?? Guid.NewGuid().ToString())
            .Select(g => g.First())
            .ToList();
    }
}
```

- [ ] **Step 3: Build the add-in — expect SUCCESS**

Run: `dotnet build cli-revit/AwareRevit/AwareRevit.csproj -v minimal`
Expected: `Build succeeded.`

- [ ] **Step 4: Write sidecar-side mirror tests `cli-revit/Tests/ResultSerializerTests.cs`**

We test the algorithm against pure .NET types (no Revit) by creating a parallel `Sidecar/ResultSerializerCore.cs` that handles primitives/dicts/anonymous-types (the same code paths as the add-in's ResultSerializer minus the Revit shapes). This locks the contract without requiring a Revit dependency.

Content:
```csharp
using System.Text.Json.Nodes;
using AwareRevit.Sidecar;
using Xunit;

namespace AwareRevit.Tests;

public class ResultSerializerTests
{
    [Theory]
    [InlineData(true)]
    [InlineData(false)]
    public void Bool_Roundtrips(bool b)
    {
        var n = ResultSerializerCore.ToJson(b);
        Assert.Equal(b, n!.GetValue<bool>());
    }

    [Fact]
    public void Int_Roundtrips()
    {
        Assert.Equal(7, ResultSerializerCore.ToJson(7)!.GetValue<int>());
    }

    [Fact]
    public void String_Roundtrips()
    {
        Assert.Equal("hi", ResultSerializerCore.ToJson("hi")!.GetValue<string>());
    }

    [Fact]
    public void Dictionary_RecursesValues()
    {
        var d = new Dictionary<string, object?> { ["a"] = 1, ["b"] = "x" };
        var n = (JsonObject)ResultSerializerCore.ToJson(d)!;
        Assert.Equal(1, n["a"]!.GetValue<int>());
        Assert.Equal("x", n["b"]!.GetValue<string>());
    }

    [Fact]
    public void List_RecursesItems()
    {
        var list = new List<object?> { 1, 2, "three" };
        var n = (JsonArray)ResultSerializerCore.ToJson(list)!;
        Assert.Equal(3, n.Count);
    }

    [Fact]
    public void AnonymousType_GetsSerialized()
    {
        var anon = new { x = 1, y = "foo" };
        var n = (JsonObject)ResultSerializerCore.ToJson(anon)!;
        Assert.Equal(1, n["x"]!.GetValue<int>());
        Assert.Equal("foo", n["y"]!.GetValue<string>());
    }

    [Fact]
    public void Null_BecomesJsonNull()
    {
        Assert.Null(ResultSerializerCore.ToJson(null));
    }
}
```

- [ ] **Step 5: Create the sidecar mirror `cli-revit/Sidecar/ResultSerializerCore.cs`**

Content:
```csharp
// Sidecar-side mirror of AwareRevit/ResultSerializer.cs covering the
// non-Revit code paths (primitives, IDictionary, IEnumerable, anonymous
// types). Keeps the shape under test without dragging RevitAPI into the
// test project. The add-in adds Revit-specific shapes on top.

using System.Collections;
using System.Text.Json;
using System.Text.Json.Nodes;

namespace AwareRevit.Sidecar;

internal static class ResultSerializerCore
{
    public static JsonNode? ToJson(object? value)
    {
        if (value is null) return null;
        switch (value)
        {
            case bool b:        return JsonValue.Create(b);
            case int i:         return JsonValue.Create(i);
            case long l:        return JsonValue.Create(l);
            case double d:      return JsonValue.Create(d);
            case float f:       return JsonValue.Create((double)f);
            case decimal m:     return JsonValue.Create(m);
            case string s:      return JsonValue.Create(s);
            case Guid g:        return JsonValue.Create(g.ToString());
            case DateTime dt:   return JsonValue.Create(dt.ToString("o"));
            case JsonNode jn:   return jn;
        }
        if (value is IDictionary id)
        {
            var jo = new JsonObject();
            foreach (DictionaryEntry kvp in id)
                jo[kvp.Key?.ToString() ?? ""] = ToJson(kvp.Value);
            return jo;
        }
        if (value is IEnumerable enumerable and not string)
        {
            var ja = new JsonArray();
            foreach (var item in enumerable) ja.Add(ToJson(item));
            return ja;
        }
        try
        {
            var json = JsonSerializer.Serialize(value, new JsonSerializerOptions
            {
                MaxDepth = 6,
                ReferenceHandler = System.Text.Json.Serialization.ReferenceHandler.IgnoreCycles,
            });
            return JsonNode.Parse(json);
        }
        catch
        {
            return JsonValue.Create($"{value.GetType().FullName}: {value}");
        }
    }
}
```

- [ ] **Step 6: Run tests — expect 7 new + 23 previous = 30 PASS**

Run: `dotnet test cli-revit/Tests/cli-revit.Tests.csproj -v minimal`
Expected: 30 passing.

- [ ] **Step 7: Commit**

Run:
```
git add cli-revit/AwareRevit/ScriptEngine.cs cli-revit/AwareRevit/ResultSerializer.cs cli-revit/Sidecar/ResultSerializerCore.cs cli-revit/Tests/ResultSerializerTests.cs
git commit -m "feat(v0.33): ScriptEngine (Roslyn-in-Revit) + ResultSerializer (Revit-aware) + 7 tests"
```

---

## Task 11: Sidecar exec verb (drives the pipe)

**Files:**
- Create: `cli-revit/Sidecar/Verbs/Exec.cs`
- Modify: `cli-revit/Sidecar/Program.cs` (add exec to the switch)
- Create: `cli-revit/Tests/EndToEndTests.cs` (in-process stub server)

- [ ] **Step 1: Create `cli-revit/Sidecar/Verbs/Exec.cs`**

Content:
```csharp
// exec verb — picks a Revit target, opens a pipe to its AwareRevit.dll add-in,
// ships the request, reshapes the add-in's ExecResponse into the AWARE-shaped
// outer receipt envelope (adds host="revit", verb, delivered_at).

using System.Text.Json;
using System.Text.Json.Nodes;
using AwareRevit.Shared;

namespace AwareRevit.Sidecar.Verbs;

internal static class Exec
{
    static readonly JsonSerializerOptions JsonOpts = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower,
        DefaultIgnoreCondition = System.Text.Json.Serialization.JsonIgnoreCondition.WhenWritingNull,
    };

    public static async Task<int> RunAsync(JsonNode? input, string? pipeNameOverride = null)
    {
        var code = input?["code"]?.GetValue<string>();
        if (string.IsNullOrEmpty(code))
        {
            Console.WriteLine(Receipts.ExecFail(
                "missing required field: code (C# script to compile + run)", "", "")
                .ToJsonString());
            return 2;
        }

        var version = input?["version"]?.GetValue<string>();
        int? hostPid = null;
        if (input?["host_pid"] is JsonNode pidNode && pidNode.GetValueKind() == JsonValueKind.Number)
            hostPid = pidNode.GetValue<int>();
        var transaction = input?["transaction"]?.GetValue<string>() ?? "none";
        var argsObj = input?["args"] as JsonObject ?? new JsonObject();

        // Resolve target. If pipeNameOverride is set (tests), bypass discovery.
        string pipeName;
        string? targetVersion = null;
        int? targetPid = null;
        if (pipeNameOverride is not null)
        {
            pipeName = pipeNameOverride;
        }
        else
        {
            var all = RevitProcessFinder.Enumerate();
            var matches = RevitProcessFinder.Filter(all, version, hostPid);
            if (matches.Count == 0)
            {
                Console.WriteLine(Receipts.ExecFail(
                    all.Count == 0
                        ? "no Revit instance running"
                        : $"requested target not found (running: {string.Join(", ", all.Select(i => i.Version))})",
                    "", "").ToJsonString());
                return 1;
            }
            if (matches.Count > 1)
            {
                Console.WriteLine(Receipts.ExecFail(
                    $"ambiguous target ({matches.Count} matches). Pass `version` or `host_pid` to disambiguate.",
                    "", "").ToJsonString());
                return 4;
            }
            var target = matches[0];
            targetPid = target.Pid;
            targetVersion = target.Version;
            pipeName = RevitProcessFinder.PipeNameFor(target.Pid);
        }

        // Build the inner request DTO.
        var req = new ExecRequest
        {
            Id = Guid.NewGuid().ToString("N"),
            Code = code!,
            Args = JsonObjectToDictionary(argsObj),
            Transaction = transaction,
            TimeoutSeconds = 60,
        };
        var reqJson = JsonSerializer.Serialize(req, JsonOpts);

        // Send over pipe.
        string respJson;
        try
        {
            var client = new PipeClient(pipeName);
            respJson = await client.SendRequestAsync(reqJson, TimeSpan.FromSeconds(30));
        }
        catch (TimeoutException te)
        {
            Console.WriteLine(Receipts.ExecFail(
                te.Message, "", "").ToJsonString());
            return 6;
        }
        catch (Exception e)
        {
            Console.WriteLine(Receipts.ExecFail(
                $"pipe transport failed: {e.Message}", e.StackTrace ?? "", "").ToJsonString());
            return 2;
        }

        // Parse + reshape.
        ExecResponse? resp;
        try { resp = JsonSerializer.Deserialize<ExecResponse>(respJson, JsonOpts); }
        catch (Exception e)
        {
            Console.WriteLine(Receipts.ExecFail(
                $"response JSON parse failed: {e.Message}", "", respJson).ToJsonString());
            return 2;
        }
        if (resp is null)
        {
            Console.WriteLine(Receipts.ExecFail(
                "response deserialised to null", "", respJson).ToJsonString());
            return 2;
        }

        if (resp.Ok)
        {
            // resp.Result was deserialised as a generic object — re-serialise then
            // parse as a JsonNode so the envelope carries proper JSON structure.
            JsonNode? resultNode = null;
            if (resp.Result is not null)
            {
                var resultJson = JsonSerializer.Serialize(resp.Result);
                resultNode = JsonNode.Parse(resultJson);
            }
            Console.WriteLine(Receipts.ExecOk(
                resultNode,
                resp.HostVersion ?? targetVersion,
                resp.HostPid ?? targetPid,
                resp.StdoutLog).ToJsonString());
            return 0;
        }

        Console.WriteLine(Receipts.ExecFail(
            resp.Error ?? "unknown error",
            resp.Stack ?? "",
            resp.StdoutLog).ToJsonString());
        return 2;
    }

    static Dictionary<string, object?> JsonObjectToDictionary(JsonObject obj)
    {
        var d = new Dictionary<string, object?>(StringComparer.Ordinal);
        foreach (var kvp in obj) d[kvp.Key] = JsonNodeToObject(kvp.Value);
        return d;
    }

    static object? JsonNodeToObject(JsonNode? node)
    {
        if (node is null) return null;
        if (node is JsonValue v)
        {
            if (v.TryGetValue<bool>(out var b)) return b;
            if (v.TryGetValue<int>(out var i)) return i;
            if (v.TryGetValue<long>(out var l)) return l;
            if (v.TryGetValue<double>(out var d)) return d;
            if (v.TryGetValue<string>(out var s)) return s;
            return v.ToString();
        }
        if (node is JsonArray arr)
        {
            var list = new List<object?>();
            foreach (var item in arr) list.Add(JsonNodeToObject(item));
            return list;
        }
        if (node is JsonObject jo) return JsonObjectToDictionary(jo);
        return null;
    }
}
```

- [ ] **Step 2: Wire `exec` into `Program.cs`**

Modify the switch in `Program.cs` to add the `exec` case. Replace:
```csharp
        return verb switch
        {
            "list-instances" => Verbs.ListInstances.RunAsync().GetAwaiter().GetResult(),
            _ => Unknown(verb),
        };
```
With:
```csharp
        return verb switch
        {
            "list-instances" => Verbs.ListInstances.RunAsync().GetAwaiter().GetResult(),
            "exec"           => Verbs.Exec.RunAsync(stdinJson).GetAwaiter().GetResult(),
            _ => Unknown(verb),
        };
```

- [ ] **Step 3: Write an end-to-end test using an in-process stub pipe server `cli-revit/Tests/EndToEndTests.cs`**

Content:
```csharp
using System.IO.Pipes;
using System.Text.Json;
using System.Text.Json.Nodes;
using AwareRevit.Shared;
using AwareRevit.Sidecar.Verbs;
using Xunit;

namespace AwareRevit.Tests;

public class EndToEndTests
{
    static readonly JsonSerializerOptions JsonOpts = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower,
        DefaultIgnoreCondition = System.Text.Json.Serialization.JsonIgnoreCondition.WhenWritingNull,
    };

    /// <summary>Spin up an in-process pipe server that mimics AwareRevit.dll:
    /// reads one request, returns a canned ok-receipt. Drive Exec.RunAsync
    /// against it with a known pipe name and verify the envelope.</summary>
    [Fact]
    public async Task ExecRunAsync_AgainstStubAddin_EmitsOkEnvelope()
    {
        var pipeName = $"aware-revit-test-{Guid.NewGuid():N}";
        var serverTask = Task.Run(async () =>
        {
            using var server = new NamedPipeServerStream(pipeName, PipeDirection.InOut, 1,
                PipeTransmissionMode.Byte, PipeOptions.Asynchronous);
            await server.WaitForConnectionAsync();
            var reqJson = await PipeFrame.ReadAsync(server);
            Assert.NotNull(reqJson);
            var req = JsonSerializer.Deserialize<ExecRequest>(reqJson!, JsonOpts);
            Assert.NotNull(req);
            Assert.Equal("return 42;", req!.Code);

            var resp = new ExecResponse
            {
                Id = req.Id,
                Ok = true,
                Result = 42,
                StdoutLog = "stub-noise",
                HostVersion = "2026",
                HostPid = 99999,
            };
            await PipeFrame.WriteAsync(server, JsonSerializer.Serialize(resp, JsonOpts));
        });

        var input = JsonNode.Parse("""{"code":"return 42;"}""");

        using var sw = new StringWriter();
        var originalOut = Console.Out;
        Console.SetOut(sw);
        int exit;
        try { exit = await Exec.RunAsync(input, pipeNameOverride: pipeName); }
        finally { Console.SetOut(originalOut); }

        Assert.Equal(0, exit);
        var receipt = JsonNode.Parse(sw.ToString().Trim()) as JsonObject;
        Assert.NotNull(receipt);
        Assert.True(receipt!["ok"]!.GetValue<bool>());
        Assert.Equal("revit", receipt["host"]!.GetValue<string>());
        Assert.Equal("exec", receipt["verb"]!.GetValue<string>());
        Assert.Equal("2026", receipt["host_version"]!.GetValue<string>());
        Assert.Equal(99999, receipt["host_pid"]!.GetValue<int>());
        Assert.Equal(42, receipt["result"]!.GetValue<int>());
        Assert.Equal("stub-noise", receipt["stdout_log"]!.GetValue<string>());

        await serverTask;
    }

    [Fact]
    public async Task ExecRunAsync_AddinReturnsError_EmitsFailEnvelope()
    {
        var pipeName = $"aware-revit-test-{Guid.NewGuid():N}";
        var serverTask = Task.Run(async () =>
        {
            using var server = new NamedPipeServerStream(pipeName, PipeDirection.InOut, 1,
                PipeTransmissionMode.Byte, PipeOptions.Asynchronous);
            await server.WaitForConnectionAsync();
            var reqJson = await PipeFrame.ReadAsync(server);
            var req = JsonSerializer.Deserialize<ExecRequest>(reqJson!, JsonOpts);

            var resp = new ExecResponse
            {
                Id = req!.Id,
                Ok = false,
                Error = "CS0103: 'doc' missing",
                Stack = "stub-stack",
                StdoutLog = "",
                HostVersion = "2026",
                HostPid = 99999,
            };
            await PipeFrame.WriteAsync(server, JsonSerializer.Serialize(resp, JsonOpts));
        });

        var input = JsonNode.Parse("""{"code":"return doc;"}""");

        using var sw = new StringWriter();
        var originalOut = Console.Out;
        Console.SetOut(sw);
        int exit;
        try { exit = await Exec.RunAsync(input, pipeNameOverride: pipeName); }
        finally { Console.SetOut(originalOut); }

        Assert.Equal(2, exit);
        var receipt = JsonNode.Parse(sw.ToString().Trim()) as JsonObject;
        Assert.NotNull(receipt);
        Assert.False(receipt!["ok"]!.GetValue<bool>());
        Assert.Contains("CS0103", receipt["error"]!.GetValue<string>());
        Assert.Equal("stub-stack", receipt["stack"]!.GetValue<string>());

        await serverTask;
    }
}
```

- [ ] **Step 4: Run tests — expect 2 new + 30 previous = 32 PASS**

Run: `dotnet test cli-revit/Tests/cli-revit.Tests.csproj -v minimal`
Expected: 32 passing.

- [ ] **Step 5: Commit**

Run:
```
git add cli-revit/Sidecar/Verbs/Exec.cs cli-revit/Sidecar/Program.cs cli-revit/Tests/EndToEndTests.cs
git commit -m "feat(v0.33): sidecar exec verb (pipe driver) + 2 end-to-end stub tests"
```

---

## Task 12: send-status, launch, close verbs

**Files:**
- Create: `cli-revit/Sidecar/Verbs/SendStatus.cs`
- Create: `cli-revit/Sidecar/Verbs/Launch.cs`
- Create: `cli-revit/Sidecar/Verbs/Close.cs`
- Modify: `cli-revit/Sidecar/Program.cs`

- [ ] **Step 1: Create `cli-revit/Sidecar/Verbs/SendStatus.cs`**

Content:
```csharp
// send-status verb: synthesises a tiny C# snippet that pops a transient
// TaskDialog (Revit has no status-bar equivalent) and routes through exec.
// Same validation-of-the-whole-pipeline approach as cli-rhino.

using System.IO;
using System.Text.Json.Nodes;
using AwareRevit.Sidecar;

namespace AwareRevit.Sidecar.Verbs;

internal static class SendStatus
{
    public static async Task<int> RunAsync(JsonNode? input)
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
            ["verb"]    = "exec",
            ["version"] = input?["version"]?.GetValue<string>(),
            ["code"]    =
                """
                var msg = args.TryGetValue("message", out var m) ? m?.ToString() : null;
                if (!string.IsNullOrEmpty(msg))
                {
                    Autodesk.Revit.UI.TaskDialog.Show("AWARE", msg);
                }
                return new { delivered = !string.IsNullOrEmpty(msg), message = msg };
                """,
            ["args"]    = new JsonObject { ["message"] = message },
        };

        // Capture exec's stdout, then re-shape into a send-status receipt.
        using var sw = new StringWriter();
        var originalOut = Console.Out;
        Console.SetOut(sw);
        int execExit;
        try { execExit = await Exec.RunAsync(execInput); }
        finally { Console.SetOut(originalOut); }

        var execReceipt = JsonNode.Parse(sw.ToString().Trim()) as JsonObject;
        if (execReceipt is null || execReceipt["ok"]?.GetValue<bool>() != true)
        {
            var err = execReceipt?["error"]?.GetValue<string>() ?? "exec failed";
            Console.WriteLine(Receipts.GenericFail("send-status", err,
                execReceipt?["stack"]?.GetValue<string>()).ToJsonString());
            return execExit;
        }

        var version = execReceipt["host_version"]?.GetValue<string>();
        int? pid = null;
        if (execReceipt["host_pid"] is JsonNode pidNode && pidNode.GetValueKind() == System.Text.Json.JsonValueKind.Number)
            pid = pidNode.GetValue<int>();
        Console.WriteLine(Receipts.SendStatusOk(message!, version, pid).ToJsonString());
        return 0;
    }
}
```

- [ ] **Step 2: Create `cli-revit/Sidecar/Verbs/Launch.cs`**

Content:
```csharp
// launch verb: spawn Revit.exe (and optionally open a model). Mirrors
// cli-tekla's launch -- returns immediately after Process.Start; the caller
// polls list-instances for addin_loaded=true to confirm IPC readiness.

using System.Diagnostics;
using System.IO;
using System.Text.Json.Nodes;

namespace AwareRevit.Sidecar.Verbs;

internal static class Launch
{
    public static int Run(JsonNode? input)
    {
        var version = input?["version"]?.GetValue<string>();
        var modelPath = input?["model_path"]?.GetValue<string>();
        var language = input?["language"]?.GetValue<string>();

        if (string.IsNullOrEmpty(version))
        {
            EmitGuide("missing required field: version (e.g. \"2026\")");
            return 2;
        }

        var revitExe = $@"C:\Program Files\Autodesk\Revit {version}\Revit.exe";
        if (!File.Exists(revitExe))
        {
            EmitGuide($"Revit {version} is not installed at {revitExe}. " +
                      "Install it or pick a version that is. " +
                      "Run `aware-revit list-instances` to see installed versions.");
            return 3;
        }

        var argList = "";
        if (!string.IsNullOrEmpty(language))
            argList += $"/language {language} ";
        if (!string.IsNullOrEmpty(modelPath))
            argList += $"\"{modelPath}\"";

        var psi = new ProcessStartInfo
        {
            FileName        = revitExe,
            Arguments       = argList.Trim(),
            UseShellExecute = true,
            WindowStyle     = ProcessWindowStyle.Maximized,
        };
        var p = Process.Start(psi);
        if (p == null)
        {
            Console.Error.WriteLine("aware-revit: Process.Start returned null");
            return 2;
        }

        var verbResult = new JsonObject
        {
            ["model_path"] = modelPath ?? "",
            ["language"]   = language ?? "",
            ["note"]       = "Revit is starting; poll list-instances until addin_loaded=true (~20s)",
        };
        Console.WriteLine(Receipts.LaunchOk(p.Id, version!, verbResult).ToJsonString());
        return 0;
    }

    static void EmitGuide(string message)
    {
        Console.Error.WriteLine($"aware-revit launch: {message}");
        Console.Error.WriteLine();
        Console.Error.WriteLine("Required stdin JSON shape:");
        Console.Error.WriteLine("  { \"version\": \"2026\",");
        Console.Error.WriteLine("    \"model_path\": \"C:/Models/Project.rvt\" }");
    }
}
```

- [ ] **Step 3: Create `cli-revit/Sidecar/Verbs/Close.cs`**

Content:
```csharp
// close verb: clean shutdown via the add-in (PostCommand(Close) +
// Application.Exit()) followed by a Process.HasExited wait. Force-kill is
// gated behind explicit `force: true`. Mirrors cli-tekla's data-loss
// acknowledgement pattern.

using System.Diagnostics;
using System.Text.Json.Nodes;

namespace AwareRevit.Sidecar.Verbs;

internal static class Close
{
    public static async Task<int> RunAsync(JsonNode? input)
    {
        var version = input?["version"]?.GetValue<string>();
        int? hostPid = null;
        if (input?["host_pid"] is JsonNode pidNode && pidNode.GetValueKind() == System.Text.Json.JsonValueKind.Number)
            hostPid = pidNode.GetValue<int>();
        var force = input?["force"]?.GetValue<bool>() ?? false;

        var all = RevitProcessFinder.Enumerate();
        var matches = RevitProcessFinder.Filter(all, version, hostPid);
        if (matches.Count == 0)
        {
            Console.WriteLine(Receipts.GenericFail("close",
                all.Count == 0 ? "no Revit instance running" : "no matching target").ToJsonString());
            return 1;
        }
        if (matches.Count > 1)
        {
            Console.WriteLine(Receipts.GenericFail("close",
                $"ambiguous target ({matches.Count} matches). Pass version or host_pid.").ToJsonString());
            return 4;
        }
        var target = matches[0];

        if (force)
        {
            try { Process.GetProcessById(target.Pid).Kill(); }
            catch (Exception e)
            {
                Console.WriteLine(Receipts.GenericFail("close", $"Process.Kill failed: {e.Message}").ToJsonString());
                return 2;
            }
            await WaitForExitAsync(target.Pid, TimeSpan.FromSeconds(15));
            Console.WriteLine(Receipts.CloseOk(target.Pid, target.Version, "force").ToJsonString());
            return 0;
        }

        // Clean path: ask the add-in to close docs + exit via an exec request.
        var execInput = new JsonObject
        {
            ["verb"]    = "exec",
            ["host_pid"] = target.Pid,
            ["code"]    =
                """
                // Close active docs then exit Revit. PostCommand schedules on the UI
                // thread; the exec call returns immediately after the post.
                var cmdId = Autodesk.Revit.UI.RevitCommandId
                    .LookupPostableCommandId(Autodesk.Revit.UI.PostableCommand.Close);
                if (cmdId != null) uiapp.PostCommand(cmdId);
                uiapp.Application.Exit();
                return new { exit_posted = true };
                """,
            ["args"]    = new JsonObject(),
        };

        using var sw = new StringWriter();
        var originalOut = Console.Out;
        Console.SetOut(sw);
        int execExit;
        try { execExit = await Exec.RunAsync(execInput); }
        finally { Console.SetOut(originalOut); }

        var execReceipt = JsonNode.Parse(sw.ToString().Trim()) as JsonObject;
        // Exec may fail because Application.Exit() races against the pipe write.
        // Treat any response (or pipe-closed mid-write) as success-pending and
        // verify via Process.HasExited.
        var exited = await WaitForExitAsync(target.Pid, TimeSpan.FromSeconds(30));
        if (!exited)
        {
            Console.WriteLine(Receipts.GenericFail("close",
                "Revit did not exit within 30s of Application.Exit(). " +
                "Re-issue with force=true if data loss is acceptable.").ToJsonString());
            return 2;
        }
        Console.WriteLine(Receipts.CloseOk(target.Pid, target.Version, "clean").ToJsonString());
        return 0;
    }

    static async Task<bool> WaitForExitAsync(int pid, TimeSpan timeout)
    {
        var deadline = DateTime.UtcNow + timeout;
        while (DateTime.UtcNow < deadline)
        {
            try
            {
                var p = Process.GetProcessById(pid);
                if (p.HasExited) return true;
            }
            catch (ArgumentException) { return true; /* process gone */ }
            await Task.Delay(500);
        }
        return false;
    }
}
```

- [ ] **Step 4: Wire all three verbs into `Program.cs`**

Replace the switch with the full surface:
```csharp
        return verb switch
        {
            "list-instances" => Verbs.ListInstances.RunAsync().GetAwaiter().GetResult(),
            "exec"           => Verbs.Exec.RunAsync(stdinJson).GetAwaiter().GetResult(),
            "send-status"    => Verbs.SendStatus.RunAsync(stdinJson).GetAwaiter().GetResult(),
            "launch"         => Verbs.Launch.Run(stdinJson),
            "close"          => Verbs.Close.RunAsync(stdinJson).GetAwaiter().GetResult(),
            _ => Unknown(verb),
        };
```

- [ ] **Step 5: Build the sidecar — expect SUCCESS**

Run: `dotnet build cli-revit/Sidecar/cli-revit.csproj -v minimal`
Expected: `Build succeeded.`

- [ ] **Step 6: Run tests — should still be 32 PASS**

Run: `dotnet test cli-revit/Tests/cli-revit.Tests.csproj -v minimal`
Expected: 32 passing.

- [ ] **Step 7: Commit**

Run:
```
git add cli-revit/Sidecar/Verbs/SendStatus.cs cli-revit/Sidecar/Verbs/Launch.cs cli-revit/Sidecar/Verbs/Close.cs cli-revit/Sidecar/Program.cs
git commit -m "feat(v0.33): send-status / launch / close verbs (full 5-verb surface)"
```

---

## Task 13: install-addin.ps1

**Files:**
- Create: `cli-revit/install-addin.ps1`

- [ ] **Step 1: Create `cli-revit/install-addin.ps1`**

Content:
```powershell
# Installs AwareRevit.dll + AwareRevit.addin into the per-user Revit add-in
# folder. Rewrites the manifest's <Assembly> entry to the absolute deployed
# path so Revit can find the DLL regardless of cwd.
#
# Usage:
#   pwsh .\cli-revit\install-addin.ps1                      # debug build
#   pwsh .\cli-revit\install-addin.ps1 -Configuration Release
#   pwsh .\cli-revit\install-addin.ps1 -RevitVersion 2025   # default 2026

param(
    [string]$Configuration = "Debug",
    [string]$RevitVersion  = "2026"
)

$ErrorActionPreference = "Stop"

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$buildOut = Join-Path $repoRoot "cli-revit\AwareRevit\bin\$Configuration\net8.0-windows"
$dllPath  = Join-Path $buildOut "AwareRevit.dll"
$srcManifest = Join-Path $buildOut "AwareRevit.addin"

if (-not (Test-Path $dllPath)) {
    Write-Error "AwareRevit.dll not found at $dllPath. Build first: dotnet build cli-revit\AwareRevit\AwareRevit.csproj -c $Configuration"
}
if (-not (Test-Path $srcManifest)) {
    Write-Error "AwareRevit.addin not found at $srcManifest. Build first."
}

$addinDir = Join-Path $env:APPDATA "Autodesk\Revit\Addins\$RevitVersion"
if (-not (Test-Path $addinDir)) {
    New-Item -ItemType Directory -Path $addinDir -Force | Out-Null
}

# Deploy DLL + its dependencies (Roslyn etc.) next to the manifest. We put
# them all in a subfolder so they don't pollute the addin root.
$deployDir = Join-Path $addinDir "AwareRevit"
if (Test-Path $deployDir) {
    Get-ChildItem $deployDir -File | Remove-Item -Force
} else {
    New-Item -ItemType Directory -Path $deployDir -Force | Out-Null
}
Get-ChildItem $buildOut -File -Recurse | ForEach-Object {
    $rel = $_.FullName.Substring($buildOut.Length).TrimStart('\','/')
    $dest = Join-Path $deployDir $rel
    $destDir = Split-Path $dest -Parent
    if (-not (Test-Path $destDir)) { New-Item -ItemType Directory -Path $destDir -Force | Out-Null }
    Copy-Item $_.FullName $dest -Force
}

# Rewrite manifest's <Assembly> to absolute deployed path.
$deployedDll = Join-Path $deployDir "AwareRevit.dll"
$destManifest = Join-Path $addinDir "AwareRevit.addin"
$xml = [xml](Get-Content $srcManifest -Raw)
$xml.RevitAddIns.AddIn.Assembly = $deployedDll
$xml.Save($destManifest)

Write-Host "Deployed:"
Write-Host "  Manifest: $destManifest"
Write-Host "  Assembly: $deployedDll"
Write-Host ""
Write-Host "Restart Revit $RevitVersion. The add-in should appear under Add-Ins > External Tools."
Write-Host "If Revit prompts about trust, allow 'Always Load'."
```

- [ ] **Step 2: Commit**

Run:
```
git add cli-revit/install-addin.ps1
git commit -m "feat(v0.33): install-addin.ps1 -- deploys addin + manifest to per-user Revit folder"
```

---

## Task 14: 20-prompt drill fixtures

**Files:**
- Create: `cli-revit/Ingest/Output/prompt-01.json` through `prompt-20.json`

- [ ] **Step 1: Create the Ingest/Output directory**

The directory is created implicitly by the file writes below. The harness expects it.

- [ ] **Step 2: Write prompt-01.json — count elements by category**

Content:
```json
{
  "verb": "exec",
  "version": "2026",
  "code": "var doc = uiapp.ActiveUIDocument.Document;\nvar buckets = new Dictionary<string, int>();\nvar collector = new FilteredElementCollector(doc).WhereElementIsNotElementType();\nforeach (var el in collector)\n{\n    var cat = el.Category?.Name ?? \"<no category>\";\n    buckets[cat] = (buckets.TryGetValue(cat, out var n) ? n : 0) + 1;\n}\nreturn new { total = buckets.Values.Sum(), by_category = buckets };",
  "args": {}
}
```

- [ ] **Step 3: Write prompt-02.json — list selected element ids**

Content:
```json
{
  "verb": "exec",
  "version": "2026",
  "code": "var sel = uiapp.ActiveUIDocument.Selection.GetElementIds();\nvar ids = sel.Select(id => id.Value).ToList();\nreturn new { count = ids.Count, ids = ids };",
  "args": {}
}
```

- [ ] **Step 4: Write prompt-03.json — doc title + path**

Content:
```json
{
  "verb": "exec",
  "version": "2026",
  "code": "var doc = uiapp.ActiveUIDocument.Document;\nreturn new { title = doc.Title, path = doc.PathName, is_workshared = doc.IsWorkshared };",
  "args": {}
}
```

- [ ] **Step 5: Write prompt-04.json — list views**

Content:
```json
{
  "verb": "exec",
  "version": "2026",
  "code": "var doc = uiapp.ActiveUIDocument.Document;\nvar views = new FilteredElementCollector(doc).OfClass(typeof(View)).Cast<View>().Where(v => !v.IsTemplate).Select(v => new { name = v.Name, type = v.ViewType.ToString() }).Take(50).ToList();\nreturn new { count = views.Count, views = views };",
  "args": {}
}
```

- [ ] **Step 6: Write prompt-05.json — list families**

Content:
```json
{
  "verb": "exec",
  "version": "2026",
  "code": "var doc = uiapp.ActiveUIDocument.Document;\nvar fams = new FilteredElementCollector(doc).OfClass(typeof(Family)).Cast<Family>().Select(f => new { name = f.Name, category = f.FamilyCategory?.Name }).Take(50).ToList();\nreturn new { count = fams.Count, families = fams };",
  "args": {}
}
```

- [ ] **Step 7: Write prompt-06.json — list worksets**

Content:
```json
{
  "verb": "exec",
  "version": "2026",
  "code": "var doc = uiapp.ActiveUIDocument.Document;\nif (!doc.IsWorkshared) return new { workshared = false, worksets = new List<object>() };\nvar wsets = new FilteredWorksetCollector(doc).OfKind(WorksetKind.UserWorkset).Select(w => new { id = w.Id.IntegerValue, name = w.Name, owner = w.Owner, is_open = w.IsOpen }).ToList();\nreturn new { workshared = true, count = wsets.Count, worksets = wsets };",
  "args": {}
}
```

- [ ] **Step 8: Write prompt-07.json — list linked models**

Content:
```json
{
  "verb": "exec",
  "version": "2026",
  "code": "var doc = uiapp.ActiveUIDocument.Document;\nvar links = new FilteredElementCollector(doc).OfClass(typeof(RevitLinkInstance)).Cast<RevitLinkInstance>().Select(l => new { name = l.Name, link_doc = l.GetLinkDocument()?.Title ?? \"<unloaded>\" }).ToList();\nreturn new { count = links.Count, links = links };",
  "args": {}
}
```

- [ ] **Step 9: Write prompt-08.json — project info**

Content:
```json
{
  "verb": "exec",
  "version": "2026",
  "code": "var doc = uiapp.ActiveUIDocument.Document;\nvar info = doc.ProjectInformation;\nreturn new { number = info.Number, name = info.Name, address = info.Address, status = info.Status, client_name = info.ClientName };",
  "args": {}
}
```

- [ ] **Step 10: Write prompt-09.json — list levels**

Content:
```json
{
  "verb": "exec",
  "version": "2026",
  "code": "var doc = uiapp.ActiveUIDocument.Document;\nvar levels = new FilteredElementCollector(doc).OfClass(typeof(Level)).Cast<Level>().OrderBy(l => l.Elevation).Select(l => new { name = l.Name, elevation_ft = l.Elevation, id = l.Id.Value }).ToList();\nreturn new { count = levels.Count, levels = levels };",
  "args": {}
}
```

- [ ] **Step 11: Write prompt-10.json — place a wall (TX auto)**

Content:
```json
{
  "verb": "exec",
  "version": "2026",
  "transaction": "auto",
  "code": "var doc = uiapp.ActiveUIDocument.Document;\nvar level = new FilteredElementCollector(doc).OfClass(typeof(Level)).Cast<Level>().FirstOrDefault();\nvar wallType = new FilteredElementCollector(doc).OfClass(typeof(WallType)).Cast<WallType>().FirstOrDefault();\nif (level == null || wallType == null) return new { placed = false, reason = \"no level or wall type available\" };\nvar p1 = new XYZ(0, 0, 0);\nvar p2 = new XYZ(10, 0, 0);\nvar line = Line.CreateBound(p1, p2);\nvar wall = Wall.Create(doc, line, wallType.Id, level.Id, 10.0, 0.0, false, false);\nreturn new { placed = true, id = wall.Id.Value, wall_type = wallType.Name };",
  "args": {}
}
```

- [ ] **Step 12: Write prompt-11.json — place a column (TX auto)**

Content:
```json
{
  "verb": "exec",
  "version": "2026",
  "transaction": "auto",
  "code": "var doc = uiapp.ActiveUIDocument.Document;\nvar level = new FilteredElementCollector(doc).OfClass(typeof(Level)).Cast<Level>().FirstOrDefault();\nvar colSym = new FilteredElementCollector(doc).OfCategory(BuiltInCategory.OST_Columns).WhereElementIsElementType().Cast<FamilySymbol>().FirstOrDefault();\nif (level == null || colSym == null) return new { placed = false, reason = \"no level or column symbol available\" };\nif (!colSym.IsActive) colSym.Activate();\nvar p = new XYZ(20, 20, 0);\nvar col = doc.Create.NewFamilyInstance(p, colSym, level, Autodesk.Revit.DB.Structure.StructuralType.Column);\nreturn new { placed = true, id = col.Id.Value, symbol = colSym.Name };",
  "args": {}
}
```

- [ ] **Step 13: Write prompt-12.json — set a parameter on selection (TX auto)**

Content:
```json
{
  "verb": "exec",
  "version": "2026",
  "transaction": "auto",
  "code": "var doc = uiapp.ActiveUIDocument.Document;\nvar selIds = uiapp.ActiveUIDocument.Selection.GetElementIds().ToList();\nif (selIds.Count == 0) return new { updated = 0, note = \"no selection; select an element with a Comments parameter\" };\nint updated = 0;\nforeach (var id in selIds)\n{\n    var el = doc.GetElement(id);\n    var p = el.LookupParameter(\"Comments\");\n    if (p != null && !p.IsReadOnly && p.StorageType == StorageType.String)\n    {\n        p.Set(\"AWARE touched on \" + System.DateTime.UtcNow.ToString(\"o\"));\n        updated++;\n    }\n}\nreturn new { updated = updated, selected = selIds.Count };",
  "args": {}
}
```

- [ ] **Step 14: Write prompt-13.json — bounding box of selection**

Content:
```json
{
  "verb": "exec",
  "version": "2026",
  "code": "var doc = uiapp.ActiveUIDocument.Document;\nvar view = uiapp.ActiveUIDocument.ActiveView;\nvar selIds = uiapp.ActiveUIDocument.Selection.GetElementIds().ToList();\nif (selIds.Count == 0) return new { bbox = (object)null, note = \"no selection\" };\nvar boxes = selIds.Select(id => doc.GetElement(id).get_BoundingBox(view)).Where(b => b != null).ToList();\nif (boxes.Count == 0) return new { bbox = (object)null, note = \"no bounding box for selected elements in this view\" };\nvar min = new XYZ(boxes.Min(b => b.Min.X), boxes.Min(b => b.Min.Y), boxes.Min(b => b.Min.Z));\nvar max = new XYZ(boxes.Max(b => b.Max.X), boxes.Max(b => b.Max.Y), boxes.Max(b => b.Max.Z));\nreturn new { selected = selIds.Count, min = new { x = min.X, y = min.Y, z = min.Z }, max = new { x = max.X, y = max.Y, z = max.Z } };",
  "args": {}
}
```

- [ ] **Step 15: Write prompt-14.json — list active view categories**

Content:
```json
{
  "verb": "exec",
  "version": "2026",
  "code": "var doc = uiapp.ActiveUIDocument.Document;\nvar view = uiapp.ActiveUIDocument.ActiveView;\nvar elsInView = new FilteredElementCollector(doc, view.Id).WhereElementIsNotElementType().ToList();\nvar cats = elsInView.Select(e => e.Category?.Name ?? \"<none>\").GroupBy(c => c).Select(g => new { category = g.Key, count = g.Count() }).OrderByDescending(x => x.count).ToList();\nreturn new { view_name = view.Name, total_visible_elements = elsInView.Count, categories = cats };",
  "args": {}
}
```

- [ ] **Step 16: Write prompt-15.json — regenerate doc (TX auto, marker change)**

Content:
```json
{
  "verb": "exec",
  "version": "2026",
  "transaction": "auto",
  "code": "var doc = uiapp.ActiveUIDocument.Document;\nvar info = doc.ProjectInformation;\nvar p = info.LookupParameter(\"Project Status\");\nstring before = p?.AsString() ?? \"\";\nif (p != null && !p.IsReadOnly && p.StorageType == StorageType.String) p.Set((before ?? \"\") + \" \");\ndoc.Regenerate();\nreturn new { regenerated = true, marker_was = before };",
  "args": {}
}
```

- [ ] **Step 17: Write prompt-16.json — export active sheet to PDF**

Content:
```json
{
  "verb": "exec",
  "version": "2026",
  "code": "var doc = uiapp.ActiveUIDocument.Document;\nvar view = uiapp.ActiveUIDocument.ActiveView;\nif (view.ViewType != ViewType.DrawingSheet) return new { exported = false, reason = \"active view is not a sheet; open a sheet first\" };\nvar tempDir = System.IO.Path.Combine(System.IO.Path.GetTempPath(), \"aware-revit-pdf\");\nSystem.IO.Directory.CreateDirectory(tempDir);\nvar opts = new PDFExportOptions { FileName = \"aware-export\", Combine = true };\nvar viewSet = new List<ElementId> { view.Id };\nbool ok = doc.Export(tempDir, viewSet, opts);\nreturn new { exported = ok, path = System.IO.Path.Combine(tempDir, \"aware-export.pdf\") };",
  "args": {}
}
```

- [ ] **Step 18: Write prompt-17.json — save the model**

Content:
```json
{
  "verb": "exec",
  "version": "2026",
  "code": "var doc = uiapp.ActiveUIDocument.Document;\nif (string.IsNullOrEmpty(doc.PathName)) return new { saved = false, reason = \"document has no path (save-as required)\" };\ntry { doc.Save(); return new { saved = true, path = doc.PathName }; }\ncatch (System.Exception e) { return new { saved = false, error = e.Message }; }",
  "args": {}
}
```

- [ ] **Step 19: Write prompt-18.json — export to IFC**

Content:
```json
{
  "verb": "exec",
  "version": "2026",
  "code": "var doc = uiapp.ActiveUIDocument.Document;\nvar tempDir = System.IO.Path.Combine(System.IO.Path.GetTempPath(), \"aware-revit-ifc\");\nSystem.IO.Directory.CreateDirectory(tempDir);\nvar opts = new IFCExportOptions();\nopts.FileVersion = IFCVersion.IFC4;\nvar name = (doc.Title ?? \"export\") + \".ifc\";\nbool ok;\nusing (var tx = new Transaction(doc, \"AWARE IFC export\"))\n{\n    tx.Start();\n    ok = doc.Export(tempDir, name, opts);\n    tx.Commit();\n}\nreturn new { exported = ok, path = System.IO.Path.Combine(tempDir, name) };",
  "args": {}
}
```

- [ ] **Step 20: Write prompt-19.json — Revit version + build + add-on count**

Content:
```json
{
  "verb": "exec",
  "version": "2026",
  "code": "var app = uiapp.Application;\nreturn new { version = app.VersionNumber, build = app.VersionBuild, name = app.VersionName, language = app.Language.ToString() };",
  "args": {}
}
```

- [ ] **Step 21: Write prompt-20.json — list all open documents**

Content:
```json
{
  "verb": "exec",
  "version": "2026",
  "code": "var docs = new List<object>();\nforeach (Document d in uiapp.Application.Documents) {\n    docs.Add(new { title = d.Title, path = d.PathName, is_family = d.IsFamilyDocument, is_linked = d.IsLinked });\n}\nreturn new { count = docs.Count, documents = docs };",
  "args": {}
}
```

- [ ] **Step 22: Commit all 20 fixtures**

Run:
```
git add cli-revit/Ingest/Output/prompt-01.json cli-revit/Ingest/Output/prompt-02.json cli-revit/Ingest/Output/prompt-03.json cli-revit/Ingest/Output/prompt-04.json cli-revit/Ingest/Output/prompt-05.json cli-revit/Ingest/Output/prompt-06.json cli-revit/Ingest/Output/prompt-07.json cli-revit/Ingest/Output/prompt-08.json cli-revit/Ingest/Output/prompt-09.json cli-revit/Ingest/Output/prompt-10.json cli-revit/Ingest/Output/prompt-11.json cli-revit/Ingest/Output/prompt-12.json cli-revit/Ingest/Output/prompt-13.json cli-revit/Ingest/Output/prompt-14.json cli-revit/Ingest/Output/prompt-15.json cli-revit/Ingest/Output/prompt-16.json cli-revit/Ingest/Output/prompt-17.json cli-revit/Ingest/Output/prompt-18.json cli-revit/Ingest/Output/prompt-19.json cli-revit/Ingest/Output/prompt-20.json
git commit -m "feat(v0.33): 20-prompt drill fixtures for live Revit 2026 testing"
```

---

## Task 15: Drill harness

**Files:**
- Create: `cli-revit/Ingest/run-drill.ps1`

- [ ] **Step 1: Create `cli-revit/Ingest/run-drill.ps1`**

Content:
```powershell
# AWARE v0.33 - revit.exec 20-prompt drill harness.
#
# Prerequisites:
#   1. Revit 2026 running with a model open.
#   2. AwareRevit.dll add-in installed (run install-addin.ps1).
#   3. aware-revit.exe built (Debug or Release).
#
# Output: cli-revit/Ingest/Output/drill-summary.md (PASS/FAIL + receipt JSONs).
#
# Compatible with Windows PowerShell 5.1 and PowerShell 7+.

param(
    [string]$AwareRevit = $null,
    [string]$FixturesDir = (Join-Path $PSScriptRoot "Output"),
    [string]$SummaryPath = (Join-Path $PSScriptRoot "Output\drill-summary.md")
)

$ErrorActionPreference = "Stop"

if (-not $AwareRevit) {
    $candidates = @(
        (Join-Path $PSScriptRoot "..\Sidecar\bin\Release\net8.0-windows\win-x64\publish\aware-revit.exe"),
        (Join-Path $PSScriptRoot "..\Sidecar\bin\Release\net8.0-windows\aware-revit.exe"),
        (Join-Path $PSScriptRoot "..\Sidecar\bin\Debug\net8.0-windows\aware-revit.exe")
    )
    foreach ($c in $candidates) {
        if (Test-Path $c) { $AwareRevit = $c; break }
    }
}
if (-not $AwareRevit -or -not (Test-Path $AwareRevit)) {
    Write-Error "aware-revit.exe not found. Run 'dotnet publish cli-revit/Sidecar/cli-revit.csproj -c Release -r win-x64' first, or pass -AwareRevit <path>."
}

$fixtures = Get-ChildItem -Path $FixturesDir -Filter "prompt-*.json" | Sort-Object Name
if ($fixtures.Count -eq 0) { Write-Error "No prompt-*.json fixtures found in $FixturesDir" }

Write-Host "Drill: $($fixtures.Count) prompts against $AwareRevit"
Write-Host ""

$results = @()
$pass = 0; $fail = 0
foreach ($fix in $fixtures) {
    $name = $fix.BaseName
    Write-Host -NoNewline ("  {0} ... " -f $name)
    $payload = Get-Content $fix.FullName -Raw
    $rawOutput = $payload | & $AwareRevit --json-stdin 2>&1
    $exit = $LASTEXITCODE
    if ($rawOutput -is [array]) { $rawOutput = ($rawOutput -join "`n") }
    $rawOutput = [string]$rawOutput

    $receipt = $null
    try { $receipt = $rawOutput | ConvertFrom-Json } catch { }

    $ok = $false
    if ($receipt -and ($receipt.ok -eq $true -or $receipt.status -eq "ok")) {
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

$lines = @()
$lines += "# revit.exec drill - $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"
$lines += ""
$lines += "**Sidecar:** $AwareRevit"
$lines += "**Fixtures:** $FixturesDir"
$lines += "**Result:** $pass PASS, $fail FAIL of $($fixtures.Count)"
$lines += ""
$lines += "| # | Status | Exit | Result snippet |"
$lines += "|---|---|---|---|"
foreach ($r in $results) {
    $clean = ($r.Receipt -replace "[`r`n]+", " ")
    $maxLen = [Math]::Min(120, $clean.Length)
    $snippet = $clean.Substring(0, $maxLen)
    $lines += "| $($r.Name) | **$($r.Status)** | $($r.ExitCode) | ``$snippet`` |"
}
$lines += ""
$lines += "## Full receipts"
$lines += ""
foreach ($r in $results) {
    $lines += "### $($r.Name) - $($r.Status)"
    $lines += ""
    $lines += '```json'
    $lines += $r.Receipt
    $lines += '```'
    $lines += ""
}
$lines -join "`n" | Out-File -FilePath $SummaryPath -Encoding utf8
Write-Host "Wrote summary -> $SummaryPath"

exit $fail
```

- [ ] **Step 2: Commit**

Run:
```
git add cli-revit/Ingest/run-drill.ps1
git commit -m "feat(v0.33): drill harness -- 20 fixtures + PASS/FAIL summary"
```

---

## Task 16: Wire runtime verbs into revit-2026 manifest

**Files:**
- Modify: `20-agents/aeco/architecture/revit-2026/manifest.yaml` (top of `commands:` block)

- [ ] **Step 1: Edit the manifest — insert the 5 runtime verbs at the top of the `commands:` map**

Find the existing line:
```yaml
transport:
  cli:
    binary: aware-revit-2026
commands:
```

Change `aware-revit-2026` → `aware-revit` (matches the sidecar `AssemblyName`), and insert the 5 verbs at the top of the commands map:

```yaml
transport:
  cli:
    binary: aware-revit
commands:
  # === v0.33 runtime verbs (aware-revit sidecar) =============================
  exec:
    lifecycle: single
    description: |
      Compile + run an ad-hoc C# script against the active Revit document via
      the in-Revit AwareRevit add-in over a named pipe. Input: { code, args,
      version?, host_pid?, transaction? }. Output: { ok, result, ... }.
      Globals: `dynamic uiapp` (UIApplication), `IDictionary<string,object?> args`.
      Use transaction="auto" to wrap the body in a Transaction (default: no wrap).
      Requires AwareRevit add-in installed (cli-revit/install-addin.ps1).
  list-instances:
    lifecycle: single
    description: |
      List running Revit instances (pid, version, exe_path, addin_loaded).
      addin_loaded=true means the AwareRevit add-in's named pipe is reachable.
  send-status:
    lifecycle: single
    description: |
      Show a transient TaskDialog message in Revit (Revit has no status bar
      equivalent; TaskDialog is the closest attention-getter).
  launch:
    lifecycle: single
    description: |
      Spawn a Revit instance. Optional model_path opens a model. Returns
      immediately after Process.Start; poll list-instances for
      addin_loaded=true to confirm IPC readiness (typically ~20s).
  close:
    lifecycle: single
    description: |
      Cleanly close a Revit instance via the add-in (PostCommand Close +
      Application.Exit), waiting up to 30s for Process.HasExited. Pass
      force=true to bypass (Process.Kill -- WILL LOSE UNSAVED WORK).
  # === Auto-generated catalog commands =======================================
```

- [ ] **Step 2: Verify the manifest still parses**

Run: `python -c "import yaml; yaml.safe_load(open(r'20-agents/aeco/architecture/revit-2026/manifest.yaml', encoding='utf-8'))" && echo OK`

Expected: `OK`.

- [ ] **Step 3: Commit**

Run:
```
git add 20-agents/aeco/architecture/revit-2026/manifest.yaml
git commit -m "feat(v0.33): wire revit-2026 manifest -- exec/list/send-status/launch/close + binary=aware-revit"
```

---

## Task 17: Build everything Release + smoke test list-instances locally

This step proves the pipeline end-to-end against the real machine (without Revit running). The sidecar must build clean and `list-instances` must return an empty-or-real list.

- [ ] **Step 1: Build all three .csproj files in Release**

Run:
```
dotnet build cli-revit/Shared/Shared.csproj -c Release -v minimal
dotnet build cli-revit/AwareRevit/AwareRevit.csproj -c Release -v minimal
dotnet build cli-revit/Sidecar/cli-revit.csproj -c Release -v minimal
```
Expected: all three "Build succeeded.", zero warnings (we have `TreatWarningsAsErrors=true`).

- [ ] **Step 2: Publish the sidecar as a single-file binary**

Run:
```
dotnet publish cli-revit/Sidecar/cli-revit.csproj -c Release -r win-x64 -p:PublishSingleFile=true -p:IncludeNativeLibrariesForSelfExtract=true -v minimal
```
Expected: `aware-revit.exe` appears under `cli-revit/Sidecar/bin/Release/net8.0-windows/win-x64/publish/`.

- [ ] **Step 3: Smoke-test list-instances against the real machine**

Run:
```
$awareRevit = "cli-revit\Sidecar\bin\Release\net8.0-windows\win-x64\publish\aware-revit.exe"
'{"verb":"list-instances"}' | & $awareRevit --json-stdin
```
Expected: JSON receipt with `"status":"ok"` and an `instances` array (empty if Revit isn't running, populated otherwise).

- [ ] **Step 4: Run the full test suite once more**

Run: `dotnet test cli-revit/Tests/cli-revit.Tests.csproj -v minimal`
Expected: 32 passing.

- [ ] **Step 5: Commit if any build-config tweaks were needed (probably nothing to commit)**

Skip if no files changed.

---

## Task 18: Live drill (manual, against running Revit 2026)

Live drill — only doable on the actual machine. Document outcome regardless.

- [ ] **Step 1: Build + install the add-in**

Run:
```
dotnet build cli-revit/AwareRevit/AwareRevit.csproj -c Release -v minimal
pwsh -NoProfile -ExecutionPolicy Bypass -File .\cli-revit\install-addin.ps1 -Configuration Release -RevitVersion 2026
```

- [ ] **Step 2: Launch Revit via the sidecar (or manually)**

Manual: open Revit 2026, open `C:\Program Files\Autodesk\Revit 2026\Samples\Snowdon Towers Sample Architectural.rvt`.
Or via sidecar:
```
'{"verb":"launch","version":"2026","model_path":"C:/Program Files/Autodesk/Revit 2026/Samples/Snowdon Towers Sample Architectural.rvt"}' | & $awareRevit --json-stdin
```
Wait ~20s for Revit's UI to be ready and the add-in to load. Verify via:
```
'{"verb":"list-instances"}' | & $awareRevit --json-stdin
```
Look for `"addin_loaded": true` in the receipt.

- [ ] **Step 3: Run the 20-prompt drill**

Run:
```
pwsh -NoProfile -ExecutionPolicy Bypass -File .\cli-revit\Ingest\run-drill.ps1
```

Drill writes `cli-revit/Ingest/Output/drill-summary.md`.

- [ ] **Step 4: Inspect summary, record PASS count in the handoff doc**

Read the summary, count `**PASS**` rows. Target: ≥13/20.

- [ ] **Step 5: Do not commit drill-summary.md (it's in .gitignore)**

---

## Task 19: Handoff doc + memory entry

**Files:**
- Create: `docs/superpowers/handoffs/2026-05-19-v033-revit-exec-ready.md`
- Create or append: `C:\Users\bimst\.claude\projects\C--Users-bimst-source-repos-aware\memory\project_v033_revit_exec_complete.md`

- [ ] **Step 1: Create the handoff doc**

Content based on the v0.32 handoff shape (intro, what shipped, what's NOT shipped, run-the-drill recipe, interpret PASS/FAIL, receipt shape divergence, architectural notes, open issues, branch+PR section). Substitute Revit-specific details: in-Revit add-in, install-addin.ps1, .addin manifest, pipe-protocol, TX semantics.

- [ ] **Step 2: Create the memory pointer file**

Content (short, pointer-style — MEMORY entries are pointers, not substitutes):
```markdown
# v0.33 — revit.exec shipped (in-Revit add-in + sidecar)

- Branch: `v0.33-revit-exec`. PR: see https://github.com/aware-aeco/aware/pulls.
- Sidecar: `cli-revit/Sidecar/` (.NET 8, single-file). Add-in: `cli-revit/AwareRevit/` (.NET 8, loaded by Revit via `.addin` manifest in `%APPDATA%\Autodesk\Revit\Addins\2026\`).
- 5 verbs: `exec`, `list-instances`, `send-status`, `launch`, `close`.
- Architecture: pipe protocol (named pipe `aware-revit-<PID>`, length-prefixed JSON) because Revit's API is callable only on its main thread inside its process.
- Tests: 32 xUnit; no Revit dependency (in-process pipe stub).
- Live drill: see `docs/superpowers/handoffs/2026-05-19-v033-revit-exec-ready.md` for PASS rate.
- What's owed (next): mirror exec receipt fields (host_pid, host_version, stdout_log) back into cli-tekla for cross-vendor envelope convergence.
- Install: `pwsh cli-revit/install-addin.ps1 -Configuration Release -RevitVersion 2026`.
```

- [ ] **Step 3: Commit the handoff (memory file is outside the repo)**

Run:
```
git add docs/superpowers/handoffs/2026-05-19-v033-revit-exec-ready.md
git commit -m "docs(v0.33): handoff -- 5 verbs shipped, drill recipe, run instructions"
```

---

## Task 20: Code review pass, push, open PR

- [ ] **Step 1: Run `pr-review-toolkit:code-reviewer` agent over the branch diff**

If the skill is not available, skip and rely on the Tests already passing + careful manual review. Address any HIGH-severity findings inline.

- [ ] **Step 2: Push the branch**

Run: `git push -u origin v0.33-revit-exec`

- [ ] **Step 3: Open the PR**

Use `gh pr create --base main --head v0.33-revit-exec --title "feat(v0.33): revit.exec runtime sidecar — in-Revit add-in + .NET sidecar" --body-file <(...)`.

PR body shape (mirror v0.32 PR #66):
- Summary (3 bullets)
- Architecture decision (one paragraph)
- Receipt shape (one paragraph)
- Test plan (xUnit count, in-process stub coverage, live drill PASS rate)
- What's NOT included (deferred list)

- [ ] **Step 4: Verify PR URL is returned**

Run: `gh pr view --json url -q .url`
Capture the URL for the final report.

---

## Spec Coverage Self-Review

Cross-checked the plan against `docs/superpowers/specs/2026-05-19-revit-exec-design.md`:

- Two-binary architecture (add-in + sidecar) → Tasks 8-10 (add-in), Tasks 3-6+11-12 (sidecar).
- 5 verbs (`exec`, `list-instances`, `send-status`, `launch`, `close`) → Task 6 (list), Task 11 (exec), Task 12 (the other three).
- Pipe protocol (length-prefixed JSON, per-PID name) → Task 2 (framing), Task 4 (client), Task 9 (server).
- Roslyn host inside Revit referencing in-process RevitAPI → Task 10 (`ScriptEngine.cs`).
- Defensive result serializer (Revit-aware) → Task 10 (`ResultSerializer.cs`).
- Transaction semantics (`transaction: "auto"` opt-in) → Task 10 (`ScriptEngine.cs` wraps in Transaction when requested) + Tasks 10/11/12/15 drill fixtures use the flag.
- Receipt shape (`host_pid`, `host_version`, `stdout_log` — strictly additive over cli-tekla) → Task 3 (`Receipts.cs`).
- Tests w/o Revit installed (in-process pipe-server stub) → Task 11 (`EndToEndTests.cs`).
- 20-prompt drill + harness → Tasks 14-15.
- Manifest wiring → Task 16.
- install-addin.ps1 → Task 13.
- Handoff doc + memory pointer → Task 19.
- PR open → Task 20.

No gaps. Plan covers every requirement of the spec.
