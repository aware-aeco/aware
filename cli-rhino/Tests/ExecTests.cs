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

    static string StubVariant(string fileName) => Path.Combine(
        Path.GetDirectoryName(typeof(ExecTests).Assembly.Location)!,
        "stub-rhinocode",
        fileName);

    static string SilentStubExePath => StubVariant("rhinocode-silent.cmd");
    static string ErrStubExePath => StubVariant("rhinocode-err.cmd");
    static string MalformedStubExePath => StubVariant("rhinocode-malformed.cmd");

    static (int exit, JsonObject? receipt) RunExec(RhinocodeClient client, JsonNode? input)
    {
        using var sw = new StringWriter();
        var originalOut = Console.Out;
        Console.SetOut(sw);
        int exit;
        try { exit = Exec.Run(client, input); }
        finally { Console.SetOut(originalOut); }
        var receipt = JsonNode.Parse(sw.ToString().Trim()) as JsonObject;
        return (exit, receipt);
    }

    [Fact]
    public void Run_AgainstStubBinary_ProducesOkReceiptWithResultFromFile()
    {
        // End-to-end through Exec.Run: wrap user Python → write temp .py → shell
        // out to the stub → stub writes the canned result to AWARE_RESULT_PATH →
        // Exec reads that file → emits AWARE-shaped receipt. Validates the full
        // file-based pipeline minus real Rhino.
        var client = new RhinocodeClient(rhinocodeExeOverride: StubExePath);
        var input = JsonNode.Parse("""{"verb":"exec","code":"return 42"}""");

        var (exit, receipt) = RunExec(client, input);

        Assert.Equal(0, exit);
        Assert.NotNull(receipt);
        Assert.True(receipt!["ok"]?.GetValue<bool>());
        Assert.Equal("rhino", receipt["host"]?.GetValue<string>());
        Assert.Equal("exec", receipt["verb"]?.GetValue<string>());
        // Stub's canned result is {"ok":true,"result":{"count":42}} — confirm it round-trips.
        var result = receipt["result"] as JsonObject;
        Assert.NotNull(result);
        Assert.Equal(42, result!["count"]?.GetValue<int>());
        // Regression guard: the result comes from the FILE, never stdout. Real
        // `rhinocode script` returns no stdout, so stdout_log must be empty. If
        // anyone re-adds stdout scraping at this layer this assertion breaks.
        Assert.Equal("", receipt["stdout_log"]?.GetValue<string>());
    }

    [Fact]
    public void Run_MissingCode_FailsWithExit2()
    {
        var client = new RhinocodeClient(rhinocodeExeOverride: StubExePath);
        var input = JsonNode.Parse("""{"verb":"exec"}""");

        var (exit, receipt) = RunExec(client, input);

        Assert.Equal(2, exit);
        Assert.NotNull(receipt);
        Assert.False(receipt!["ok"]?.GetValue<bool>());
        Assert.Contains("code", receipt["error"]?.GetValue<string>() ?? "");
    }

    [Fact]
    public void Run_WhenScriptWritesNoResultFile_FailsWithInfraError()
    {
        // The silent stub returns exit 0 but writes NOTHING — simulating a script
        // that never ran or threw before the result write. Exec polls for the
        // file and, when it never appears, must report the "result file not
        // written" infra failure rather than a false success. Use a short
        // timeout so the test doesn't sit through the 60s production default.
        Environment.SetEnvironmentVariable("AWARE_EXEC_TIMEOUT_MS", "300");
        try
        {
            var client = new RhinocodeClient(rhinocodeExeOverride: SilentStubExePath);
            var input = JsonNode.Parse("""{"verb":"exec","code":"return 1"}""");

            var (exit, receipt) = RunExec(client, input);

            Assert.Equal(2, exit);
            Assert.NotNull(receipt);
            Assert.False(receipt!["ok"]?.GetValue<bool>());
            Assert.Contains("result file not written", receipt["error"]?.GetValue<string>() ?? "");
            // Infra fault ⇒ the rhinocode exit code is surfaced for machine
            // triage (here 0: the silent stub exited cleanly but wrote nothing).
            Assert.Equal(0, receipt["exit_code"]?.GetValue<int>());
        }
        finally
        {
            Environment.SetEnvironmentVariable("AWARE_EXEC_TIMEOUT_MS", null);
        }
    }

    [Fact]
    public void Run_WhenScriptReportsFailure_ReturnsExit1AndPropagatesErrorAndStack()
    {
        // The err stub writes {"ok":false,"error":"boom","stack":"..."} — i.e.
        // the user script RAN and threw (the most common real non-happy path).
        // Exec must return exit 1 (script-fault, NOT infra fault 2) and forward
        // the script's error + stack onto the receipt.
        var client = new RhinocodeClient(rhinocodeExeOverride: ErrStubExePath);
        var input = JsonNode.Parse("""{"verb":"exec","code":"raise RuntimeError('boom')"}""");

        var (exit, receipt) = RunExec(client, input);

        Assert.Equal(1, exit);
        Assert.NotNull(receipt);
        Assert.False(receipt!["ok"]?.GetValue<bool>());
        Assert.Equal("boom", receipt["error"]?.GetValue<string>());
        Assert.Contains("RuntimeError: boom", receipt["stack"]?.GetValue<string>() ?? "");
        // Script-fault: exit_code stays null so callers can tell it apart from
        // an infra fault (where rhinocode itself errored).
        Assert.True(receipt["exit_code"] is null || receipt["exit_code"]!.GetValueKind() == System.Text.Json.JsonValueKind.Null);
    }

    [Fact]
    public void Run_WhenResultFileIsMalformedJson_FailsWithParseError()
    {
        // The malformed stub writes a non-empty, non-JSON result file. Exec must
        // surface "result file JSON parse failed" (exit 2) with the contents —
        // and WaitForResultFile's settled-but-unparseable terminal state must
        // return promptly rather than polling to the full timeout. A timeout
        // longer than the settle window proves the early return works.
        Environment.SetEnvironmentVariable("AWARE_EXEC_TIMEOUT_MS", "10000");
        try
        {
            var client = new RhinocodeClient(rhinocodeExeOverride: MalformedStubExePath);
            var input = JsonNode.Parse("""{"verb":"exec","code":"return 1"}""");

            var sw = System.Diagnostics.Stopwatch.StartNew();
            var (exit, receipt) = RunExec(client, input);
            sw.Stop();

            Assert.Equal(2, exit);
            Assert.NotNull(receipt);
            Assert.False(receipt!["ok"]?.GetValue<bool>());
            Assert.Contains("result file JSON parse failed", receipt["error"]?.GetValue<string>() ?? "");
            // Must have returned via the settled-file path, well under the 10s timeout.
            Assert.True(sw.ElapsedMilliseconds < 5000,
                $"expected early return on settled-unparseable file, took {sw.ElapsedMilliseconds}ms");
        }
        finally
        {
            Environment.SetEnvironmentVariable("AWARE_EXEC_TIMEOUT_MS", null);
        }
    }
}
