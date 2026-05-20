using System.IO;
using AwareRhino;
using Xunit;

namespace AwareRhino.Tests;

public class RhinocodeClientTests : IDisposable
{
    // Per-test isolated dump dir so the stub and the test agree on where it
    // writes diagnostic artifacts. Bypasses %TEMP%/Path.GetTempPath() mismatches
    // when those env vars are inconsistent on the build machine.
    readonly string _dumpDir;

    public RhinocodeClientTests()
    {
        _dumpDir = Path.Combine(Path.GetTempPath(), $"aware-rhino-stub-{Guid.NewGuid():N}");
        Directory.CreateDirectory(_dumpDir);
        Environment.SetEnvironmentVariable("AWARE_STUB_DUMP_DIR", _dumpDir);
    }

    public void Dispose()
    {
        Environment.SetEnvironmentVariable("AWARE_STUB_DUMP_DIR", null);
        try { Directory.Delete(_dumpDir, recursive: true); } catch { /* best effort */ }
    }

    static string StubDir => Path.Combine(
        Path.GetDirectoryName(typeof(RhinocodeClientTests).Assembly.Location)!,
        "stub-rhinocode");

    static RhinocodeClient NewStubClient()
    {
        // Force discovery via constructor override.
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
        var lastIdFile = Path.Combine(_dumpDir, "last-rhino-id.txt");

        var tempScript = Path.Combine(_dumpDir, $"aware-rhino-test-{Guid.NewGuid():N}.py");
        File.WriteAllText(tempScript, "# noop");

        var (exit, stdout, stderr) = c.RunScript(tempScript,
                                                 rhinoId: "rhinocode_remotepipe_75029",
                                                 argsJson: "{}");
        File.Delete(tempScript);

        Assert.Equal(0, exit);
        Assert.True(File.Exists(lastIdFile),
            $"stub did not write last-rhino-id.txt to {_dumpDir}. stdout={stdout}; stderr={stderr}");
        var recordedId = File.ReadAllText(lastIdFile).Trim();
        Assert.Equal("rhinocode_remotepipe_75029", recordedId);
    }

    [Fact]
    public void RunScript_WritesResultFileToResultPath_NotStdout()
    {
        // Real rhinocode `script` returns no stdout; the script writes a result
        // file. The stub mirrors that: given AWARE_RESULT_PATH it copies a canned
        // {ok,result} JSON there and prints nothing. Verify the file appears.
        var c = NewStubClient();
        var tempScript = Path.Combine(_dumpDir, $"aware-rhino-test-{Guid.NewGuid():N}.py");
        File.WriteAllText(tempScript, "# noop");
        var resultPath = Path.Combine(_dumpDir, $"aware-rhino-result-{Guid.NewGuid():N}.json");

        var (exit, stdout, _) = c.RunScript(tempScript, rhinoId: null, argsJson: "{}", resultPath: resultPath);
        File.Delete(tempScript);

        Assert.Equal(0, exit);
        Assert.True(string.IsNullOrWhiteSpace(stdout), $"expected empty stdout, got: {stdout}");
        Assert.True(File.Exists(resultPath), $"stub did not write result file to {resultPath}");
        var body = File.ReadAllText(resultPath);
        Assert.Contains("\"ok\": true", body);
        Assert.Contains("\"count\": 42", body);
    }

    [Fact]
    public void Discover_FallsBackToStandardInstallPath_WhenEnvAndPathMiss()
    {
        // Simulate a machine without rhinocode on PATH and no env override.
        // Inject fake "file-exists" probe so only the standard install path is "present".
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
