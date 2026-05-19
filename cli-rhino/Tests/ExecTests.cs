using System.IO;
using System.Text.Json.Nodes;
using AwareRhino;
using AwareRhino.Verbs;
using Xunit;

namespace AwareRhino.Tests;

public class ExecTests : IDisposable
{
    readonly string _dumpDir;

    public ExecTests()
    {
        _dumpDir = Path.Combine(Path.GetTempPath(), $"aware-rhino-exec-test-{Guid.NewGuid():N}");
        Directory.CreateDirectory(_dumpDir);
        Environment.SetEnvironmentVariable("AWARE_STUB_DUMP_DIR", _dumpDir);
    }

    public void Dispose()
    {
        Environment.SetEnvironmentVariable("AWARE_STUB_DUMP_DIR", null);
        try { Directory.Delete(_dumpDir, recursive: true); } catch { /* best effort */ }
    }

    static string StubExePath => Path.Combine(
        Path.GetDirectoryName(typeof(ExecTests).Assembly.Location)!,
        "stub-rhinocode",
        "rhinocode.cmd");

    [Fact]
    public void Run_AgainstStubBinary_ProducesOkReceiptWithStubResult()
    {
        // End-to-end through Exec.Run: wrap user code → write temp .cs →
        // shell out to stub rhinocode → parse stub's canned sentinel block →
        // emit AWARE-shaped receipt. Validates the full pipeline minus rhinocode.
        var client = new RhinocodeClient(rhinocodeExeOverride: StubExePath);
        var input = JsonNode.Parse("""{"verb":"exec","code":"return 42;"}""");

        // Capture Exec.Run's stdout into a string so we can inspect the receipt.
        using var sw = new StringWriter();
        var originalOut = Console.Out;
        Console.SetOut(sw);
        int exit;
        try { exit = Exec.Run(client, input); }
        finally { Console.SetOut(originalOut); }

        Assert.Equal(0, exit);
        var receipt = JsonNode.Parse(sw.ToString().Trim()) as JsonObject;
        Assert.NotNull(receipt);
        Assert.True(receipt!["ok"]?.GetValue<bool>());
        Assert.Equal("rhino", receipt["host"]?.GetValue<string>());
        Assert.Equal("exec", receipt["verb"]?.GetValue<string>());
        // Stub's canned result is {"count": 42} — confirm it round-trips.
        var result = receipt["result"] as JsonObject;
        Assert.NotNull(result);
        Assert.Equal(42, result!["count"]?.GetValue<int>());
        // The "trailing chatter" line from the stub's canned response lands in stdout_log.
        Assert.Contains("trailing chatter", receipt["stdout_log"]?.GetValue<string>() ?? "");
    }

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
