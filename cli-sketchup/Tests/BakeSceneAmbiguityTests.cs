using System.Text.Json.Nodes;
using AwareSketchup;
using Xunit;

namespace AwareSketchup.Tests;

/// <summary>
/// bake-scene must never GUESS which model to write to.
///
/// `SketchupClient.Resolve` matches a version filter by prefix and takes the first hit, which is
/// harmless for a read. But bake-scene is retire-and-replace: an arbitrary pick between two
/// matching sessions would create members in one model while DELETING source-owned groups from
/// whichever was enumerated first. Resolve already refuses to guess when no filter is given; a
/// filter that still matches several is that same ambiguity wearing a filter.
///
/// Found by Codex review.
/// </summary>
public class BakeSceneAmbiguityTests : IDisposable
{
    readonly string _discoveryDir;

    public BakeSceneAmbiguityTests()
    {
        _discoveryDir = Path.Combine(Path.GetTempPath(), $"aware-sketchup-ambig-{Guid.NewGuid():N}");
        Directory.CreateDirectory(_discoveryDir);
    }

    public void Dispose()
    {
        try { Directory.Delete(_discoveryDir, recursive: true); } catch { /* best effort */ }
        GC.SuppressFinalize(this);
    }

    void WriteInstance(int pid, int port, string version) =>
        File.WriteAllText(Path.Combine(_discoveryDir, $"{pid}.json"), new JsonObject
        {
            ["pid"] = pid,
            ["port"] = port,
            ["version"] = version,
            ["model_path"] = null,
            ["started_at"] = "2026-07-26T00:00:00Z",
        }.ToJsonString());

    static JsonObject Request(string? version = null, string? sketchupId = null)
    {
        var req = new JsonObject
        {
            ["verb"] = "bake-scene",
            ["scene"] = new JsonObject
            {
                ["meta"] = new JsonObject
                {
                    ["name"] = "t",
                    ["units"] = "mm",
                    ["sourceId"] = "sha256:" + new string('a', 64),
                    ["sceneHash"] = "sha256:" + new string('b', 64),
                },
                ["elements"] = new JsonArray(),
                ["operations"] = new JsonArray(),
                ["referenceSystems"] = new JsonArray(),
            },
        };
        if (version is not null) req["version"] = version;
        if (sketchupId is not null) req["sketchup_id"] = sketchupId;
        return req;
    }

    string RunCapturingStdout(JsonObject request, out int exit)
    {
        using var sw = new StringWriter();
        var original = Console.Out;
        Console.SetOut(sw);
        try { exit = Verbs.BakeScene.Run(request, new SketchupClient(_discoveryDir, pidAlive: _ => true)); }
        finally { Console.SetOut(original); }
        return sw.ToString();
    }

    [Fact]
    public void TwoInstancesMatchingTheVersionPrefix_AreRefused()
    {
        WriteInstance(1001, 8765, "26.1.189");
        WriteInstance(1002, 8766, "26.1.240");   // same "26.1" prefix, different session

        var output = RunCapturingStdout(Request(version: "26.1"), out var exit);

        Assert.Equal(1, exit);
        Assert.Contains("ambiguous target", output);
        Assert.Contains("1001", output);
        Assert.Contains("1002", output);
        Assert.Contains("sketchup_id", output);
        // It must refuse BEFORE touching a model: no receipt claiming anything was baked.
        Assert.DoesNotContain("\"created\"", output);
    }

    [Fact]
    public void AnExplicitSketchupId_IsNeverAmbiguous()
    {
        WriteInstance(1001, 8765, "26.1.189");
        WriteInstance(1002, 8766, "26.1.240");

        // Picks 1002 unambiguously; it then fails to CONNECT (no bridge on that port), which is a
        // different failure — the point is that it is not refused as ambiguous.
        var output = RunCapturingStdout(Request(version: "26.1", sketchupId: "1002"), out _);

        Assert.DoesNotContain("ambiguous target", output);
    }

    [Fact]
    public void OneMatchingInstance_IsNotRefused()
    {
        WriteInstance(1001, 8765, "26.1.189");
        WriteInstance(1002, 8766, "25.0.100");   // different prefix — not a match

        var output = RunCapturingStdout(Request(version: "26.1"), out _);

        Assert.DoesNotContain("ambiguous target", output);
    }
}

/// <summary>
/// A watchdog timeout is NOT "nothing happened".
///
/// The bridge answers from its own thread while the Ruby materializer keeps running on SketchUp's
/// main thread, so the script can still reach commit_operation after the reply. A receipt implying
/// an untouched model would be a guess dressed as a fact. Found by Codex review.
/// </summary>
public class BakeSceneTimeoutHonestyTests
{
    [Theory]
    [InlineData("bridge timeout after 90000ms")]
    [InlineData("request timed out")]
    [InlineData("WATCHDOG fired")]
    public void ATimeout_SaysTheOutcomeIsUnknownAndHowToRecover(string error)
    {
        var described = Verbs.BakeScene.DescribeBridgeFailure(error);
        Assert.Contains(error, described);
        Assert.Contains("may or may not", described);
        Assert.Contains("same sourceId", described);
    }

    [Theory]
    [InlineData("NoMethodError: undefined method `foo'")]
    [InlineData("bridge reported failure with no error message")]
    public void ANonTimeoutFailure_IsPassedThroughUnembellished(string error)
    {
        // The rescue already aborted the operation there; adding "may or may not" would make a
        // clean, known failure sound uncertain.
        Assert.Equal(error, Verbs.BakeScene.DescribeBridgeFailure(error));
    }

    [Theory]
    [InlineData(null)]
    [InlineData("")]
    [InlineData("   ")]
    public void AMissingErrorStillReadsAsAFailure(string? error)
    {
        Assert.Contains("bridge reported failure", Verbs.BakeScene.DescribeBridgeFailure(error));
    }
}
