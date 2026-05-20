using System.IO;
using System.Text.Json.Nodes;
using AwareRhino.Verbs;
using Xunit;

namespace AwareRhino.Tests;

public class LaunchTests : IDisposable
{
    readonly string _stubExePath;

    public LaunchTests()
    {
        // Create a tiny stub "Rhino.exe" that just exits 0 — enough to verify
        // launch wires up Process.Start without actually spawning Rhino.
        // On Windows a .cmd file is launchable via UseShellExecute=true.
        _stubExePath = Path.Combine(Path.GetTempPath(),
            $"aware-rhino-launch-stub-{Guid.NewGuid():N}.cmd");
        File.WriteAllText(_stubExePath, "@echo off\r\nexit /b 0\r\n");
    }

    public void Dispose()
    {
        try { File.Delete(_stubExePath); } catch { /* best effort */ }
    }

    [Fact]
    public void Run_MissingRhinoExe_EmitsErrEnvelope()
    {
        var input = JsonNode.Parse("""
            { "rhino_exe": "C:\\definitely\\does\\not\\exist.exe" }
            """);
        using var sw = new StringWriter();
        var originalOut = Console.Out;
        Console.SetOut(sw);
        int exit;
        try { exit = Launch.Run(input); }
        finally { Console.SetOut(originalOut); }

        Assert.Equal(3, exit);
        var receipt = JsonNode.Parse(sw.ToString().Trim()) as JsonObject;
        Assert.NotNull(receipt);
        Assert.Equal("err", receipt!["status"]?.GetValue<string>());
        Assert.Contains("Could not find Rhino.exe", receipt["error"]?.GetValue<string>() ?? "");
    }

    [Fact]
    public void Run_BadModelPath_EmitsErrEnvelope()
    {
        var input = new JsonObject
        {
            ["rhino_exe"]  = _stubExePath,
            ["model_path"] = @"C:\not\a\real\model.3dm",
        };
        using var sw = new StringWriter();
        var originalOut = Console.Out;
        Console.SetOut(sw);
        int exit;
        try { exit = Launch.Run(input); }
        finally { Console.SetOut(originalOut); }

        Assert.Equal(2, exit);
        var receipt = JsonNode.Parse(sw.ToString().Trim()) as JsonObject;
        Assert.NotNull(receipt);
        Assert.Contains("model_path", receipt!["error"]?.GetValue<string>() ?? "");
    }

    [Fact]
    public void Run_StubExe_SpawnsAndReturnsPidReceipt()
    {
        var input = new JsonObject
        {
            ["rhino_exe"]           = _stubExePath,
            ["start_script_server"] = false,
        };
        using var sw = new StringWriter();
        var originalOut = Console.Out;
        Console.SetOut(sw);
        int exit;
        try { exit = Launch.Run(input); }
        finally { Console.SetOut(originalOut); }

        Assert.Equal(0, exit);
        var receipt = JsonNode.Parse(sw.ToString().Trim()) as JsonObject;
        Assert.NotNull(receipt);
        Assert.Equal("ok", receipt!["status"]?.GetValue<string>());
        Assert.Equal("rhino", receipt["host"]?.GetValue<string>());
        Assert.True(receipt["host_pid"]?.GetValue<int>() > 0,
            "host_pid should be > 0 after a successful spawn");
        var verbResult = receipt["verb_result"] as JsonObject;
        Assert.NotNull(verbResult);
        Assert.False(verbResult!["script_server"]?.GetValue<bool>() ?? true,
            "script_server should be false when caller opted out");
    }

    [Fact]
    public void DiscoverRhinoExe_ReturnsNullOrExistingPath()
    {
        var discovered = Launch.DiscoverRhinoExe();
        // Either Rhino is installed (path exists) or not (null). Anything else is a bug.
        if (discovered is not null)
        {
            Assert.True(File.Exists(discovered),
                $"DiscoverRhinoExe returned '{discovered}' but the file does not exist");
        }
    }
}
