using System.IO;
using System.Text.Json.Nodes;
using AwareRhino;
using AwareRhino.Verbs;
using Xunit;

namespace AwareRhino.Tests;

public class CloseTests : IDisposable
{
    readonly string _dumpDir;

    public CloseTests()
    {
        _dumpDir = Path.Combine(Path.GetTempPath(), $"aware-rhino-close-test-{Guid.NewGuid():N}");
        Directory.CreateDirectory(_dumpDir);
        Environment.SetEnvironmentVariable("AWARE_STUB_DUMP_DIR", _dumpDir);
    }

    public void Dispose()
    {
        Environment.SetEnvironmentVariable("AWARE_STUB_DUMP_DIR", null);
        try { Directory.Delete(_dumpDir, recursive: true); } catch { /* best effort */ }
    }

    static string StubExePath => Path.Combine(
        Path.GetDirectoryName(typeof(CloseTests).Assembly.Location)!,
        "stub-rhinocode",
        "rhinocode.cmd");

    [Fact]
    public void Run_ForceNoTarget_ReturnsErr()
    {
        var client = new RhinocodeClient(rhinocodeExeOverride: StubExePath);
        var input  = new JsonObject { ["force"] = true };

        using var sw = new StringWriter();
        var originalOut = Console.Out;
        Console.SetOut(sw);
        int exit;
        try { exit = Close.Run(client, input); }
        finally { Console.SetOut(originalOut); }

        // Force with no target → rhinocode list returns stub's canned response
        // which won't match a real PID. Either we get "no target identified"
        // or Process.GetProcessById on the stub's fake pid fails. Either way,
        // not a clean exit.
        Assert.NotEqual(0, exit);
        var receipt = JsonNode.Parse(sw.ToString().Trim()) as JsonObject;
        Assert.NotNull(receipt);
        Assert.Equal("err", receipt!["status"]?.GetValue<string>());
    }

    [Fact]
    public void Run_CleanMode_IssuesExitMacroViaRhinocode()
    {
        // Stub's `command` subcommand isn't a known verb — it falls through to
        // the "unknown verb" branch and exits 99. We're testing that Close.Run
        // attempts the macro and surfaces the receipt with the expected shape,
        // not that the stub actually shut down a Rhino.
        var client = new RhinocodeClient(rhinocodeExeOverride: StubExePath);
        var input  = new JsonObject
        {
            ["rhino_id"] = "rhinocode_remotepipe_75029",
            ["save"]     = false,
        };

        using var sw = new StringWriter();
        var originalOut = Console.Out;
        Console.SetOut(sw);
        int exit;
        try { exit = Close.Run(client, input); }
        finally { Console.SetOut(originalOut); }

        var receipt = JsonNode.Parse(sw.ToString().Trim()) as JsonObject;
        Assert.NotNull(receipt);
        Assert.Equal("close", receipt!["verb"]?.GetValue<string>());
        var verbResult = receipt["verb_result"] as JsonObject;
        Assert.NotNull(verbResult);
        Assert.Equal("clean", verbResult!["mode"]?.GetValue<string>());
        Assert.Equal("_-Exit _N", verbResult["macro"]?.GetValue<string>());
        Assert.False(verbResult["save"]?.GetValue<bool>() ?? true);
    }

    [Fact]
    public void Run_CleanMode_SaveTrue_UsesYesMacro()
    {
        var client = new RhinocodeClient(rhinocodeExeOverride: StubExePath);
        var input  = new JsonObject
        {
            ["rhino_id"] = "rhinocode_remotepipe_75029",
            ["save"]     = true,
        };

        using var sw = new StringWriter();
        var originalOut = Console.Out;
        Console.SetOut(sw);
        try { Close.Run(client, input); }
        finally { Console.SetOut(originalOut); }

        var receipt    = JsonNode.Parse(sw.ToString().Trim()) as JsonObject;
        var verbResult = receipt?["verb_result"] as JsonObject;
        Assert.Equal("_-Exit _Y", verbResult!["macro"]?.GetValue<string>());
        Assert.True(verbResult["save"]?.GetValue<bool>() ?? false);
    }
}
