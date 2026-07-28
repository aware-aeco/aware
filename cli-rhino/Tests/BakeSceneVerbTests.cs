using System.Text.Json.Nodes;
using AwareRhino;
using AwareRhino.Verbs;
using Xunit;

namespace AwareRhino.Tests;

public class BakeSceneVerbTests : IDisposable
{
    readonly string _dumpDir;
    string? _scriptPath;
    string? _resultPath;

    public BakeSceneVerbTests()
    {
        _dumpDir = Path.Combine(
            Path.GetTempPath(), $"aware-rhino-bake-test-{Guid.NewGuid():N}");
        Directory.CreateDirectory(_dumpDir);
        Environment.SetEnvironmentVariable("AWARE_STUB_DUMP_DIR", _dumpDir);
    }

    public void Dispose()
    {
        Environment.SetEnvironmentVariable("AWARE_BAKE_TIMEOUT_MS", null);
        Environment.SetEnvironmentVariable("AWARE_STUB_DUMP_DIR", null);
        if (_scriptPath is not null) try { File.Delete(_scriptPath); } catch { /* best effort */ }
        if (_resultPath is not null) try { File.Delete(_resultPath); } catch { /* best effort */ }
        try { Directory.Delete(_dumpDir, recursive: true); } catch { /* best effort */ }
    }

    static string StubVariant(string fileName) => Path.Combine(
        Path.GetDirectoryName(typeof(BakeSceneVerbTests).Assembly.Location)!,
        "stub-rhinocode",
        fileName);

    const string Minimal = """
        {
          "scene":{
            "meta":{"units":"mm","sourceId":"drop-a","sceneHash":"scene-a"},
            "elements":[
              {"id":"M1","kind":"member","from":[0,0,0],"to":[1000,0,0],"section":{"w":200,"d":300}}
            ],
            "operations":[],
            "referenceSystems":[]
          },
          "rhino_id":"rhinocode_remotepipe_75029",
          "host_pid":31204,
          "version":"8.13"
        }
        """;

    [Theory]
    [InlineData("""{"rhino_id":42}""", "rhino_id")]
    [InlineData("""{"rhino_id":"  "}""", "rhino_id")]
    [InlineData("""{"rhino_id":null}""", "rhino_id")]
    [InlineData("""{"version":8}""", "version")]
    [InlineData("""{"version":""}""", "version")]
    [InlineData("""{"version":null}""", "version")]
    [InlineData("""{"host_pid":"31204"}""", "host_pid")]
    public void SuppliedMalformedSelectorFailsClosed(string json, string selector)
    {
        var ok = BakeScene.TrySelectors(
            JsonNode.Parse(json),
            out var rhinoId,
            out var hostPid,
            out var version,
            out var error);

        Assert.False(ok);
        Assert.Null(rhinoId);
        Assert.Null(hostPid);
        Assert.Null(version);
        Assert.Contains(selector, error);
    }

    [Fact]
    public void MutatingTimeoutReportsUncertainStateAndReconciliationGuidance()
    {
        Environment.SetEnvironmentVariable("AWARE_BAKE_TIMEOUT_MS", "250");
        var client = new RhinocodeClient(
            rhinocodeExeOverride: StubVariant("rhinocode-silent.cmd"));

        using var sw = new StringWriter();
        var original = Console.Out;
        int exit;
        try
        {
            Console.SetOut(sw);
            exit = BakeScene.Run(client, JsonNode.Parse(Minimal));
        }
        finally
        {
            Console.SetOut(original);
        }

        var receipt = JsonNode.Parse(sw.ToString().Trim()) as JsonObject;
        Assert.Equal(2, exit);
        Assert.NotNull(receipt);
        Assert.False(receipt!["ok"]!.GetValue<bool>());
        Assert.Equal("uncertain", receipt["commit_state"]!.GetValue<string>());
        Assert.Contains("same sourceId", receipt["recovery"]!.GetValue<string>());
        Assert.Contains("may still be applying", receipt["error"]!.GetValue<string>());

        _scriptPath = File.ReadAllText(
            Path.Combine(_dumpDir, "last-script-path.txt")).Trim();
        Assert.True(File.Exists(_scriptPath),
            "an uncertain fire-and-forget completion must keep its exact script/result channel");
        var resultLine = File.ReadLines(_scriptPath)
            .Single(line => line.StartsWith("__aware_result_path =", StringComparison.Ordinal));
        var literal = resultLine[(resultLine.LastIndexOf(" or ", StringComparison.Ordinal) + 4)..];
        _resultPath = System.Text.Json.JsonSerializer.Deserialize<string>(literal);
        Assert.NotNull(_resultPath);
    }
}
