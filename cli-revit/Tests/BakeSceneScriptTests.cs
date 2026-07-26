// Tests for the bake-scene script assembly + the sidecar verb that ships it.
//
// The Revit-specific half of the bake only exists as script text, so it is
// checked two ways here: it must PARSE as a C# script, and — on a machine with
// Revit installed — it must COMPILE semantically against the real RevitAPI, the
// same way the in-Revit ScriptEngine will compile it. No Revit is launched and
// nothing is executed; this is a compile, not a run.

using System.IO.Pipes;
using System.Text.Json;
using System.Text.Json.Nodes;
using AwareRevit.Shared;
using AwareRevit.Sidecar;
using AwareRevit.Sidecar.Verbs;
using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.CSharp;
using Microsoft.CodeAnalysis.CSharp.Scripting;
using Microsoft.CodeAnalysis.Scripting;
using Xunit;

namespace AwareRevit.Tests;

/// <summary>Console.SetOut is process-global, so every test class that captures
/// stdout has to share one collection — otherwise xUnit's cross-class
/// parallelism interleaves two captures and each reads the other's receipt.</summary>
[CollectionDefinition(ConsoleCaptureCollection.Name, DisableParallelization = true)]
public class ConsoleCaptureCollection
{
    public const string Name = "console-capture";
}

[Collection(ConsoleCaptureCollection.Name)]
public class BakeSceneScriptTests
{
    static readonly JsonSerializerOptions JsonOpts = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower,
        DefaultIgnoreCondition = System.Text.Json.Serialization.JsonIgnoreCondition.WhenWritingNull,
    };

    // ── the script is assembled from one copy of the rules ───────────────────

    [Fact]
    public void Code_SplicesTheEmbeddedRulesSourceBetweenHeaderAndBody()
    {
        var rules = BakeSceneScript.LoadRulesSource();
        Assert.Contains("internal static class AwareBakeRules", rules);
        Assert.Contains("MmPerFoot = 304.8", rules);
        Assert.Equal(BakeSceneScript.Header + rules + BakeSceneScript.Body, BakeSceneScript.Code);
    }

    static readonly System.Text.RegularExpressions.Regex UsingDirective =
        new(@"^\s*using\s+(?:[A-Za-z_][A-Za-z0-9_]*\s*=\s*)?[A-Za-z_][A-Za-z0-9_.]*\s*;\s*$");

    [Fact]
    public void Code_DeclaresEveryUsingBeforeAnyOtherCode()
    {
        // A C# script rejects a `using` DIRECTIVE that follows a declaration or a
        // statement, so the header must own them all. (`using (var tx = ...)` is a
        // statement, not a directive — hence the regex rather than a prefix test.)
        var lines = BakeSceneScript.Code.Replace("\r\n", "\n").Split('\n');
        var firstNonDirective = Array.FindIndex(lines, l =>
            !UsingDirective.IsMatch(l) && l.Trim().Length > 0);
        Assert.True(firstNonDirective > 0, "the script must start with using directives");
        Assert.DoesNotContain(lines.Skip(firstNonDirective), l => UsingDirective.IsMatch(l));
    }

    /// <summary>Revit has no status bar and TaskDialog is MODAL — announcing the
    /// bake would block the bake, inside an open Transaction, on the API thread.
    /// Tekla's bake-scene announces; Revit's deliberately does not.</summary>
    [Fact]
    public void Code_NeverPopsATaskDialogOrOtherModalDuringTheBake()
    {
        // The comment explaining WHY names TaskDialog; the code must not call it.
        var executable = string.Join("\n", BakeSceneScript.Code
            .Replace("\r\n", "\n")
            .Split('\n')
            .Where(l => !l.TrimStart().StartsWith("//", StringComparison.Ordinal)));

        Assert.DoesNotContain("TaskDialog", executable);
        Assert.DoesNotContain("MessageBox", executable);
        Assert.Contains("SetForcedModalHandling(false)", executable);
    }

    /// <summary>mm → Revit's internal decimal feet happens in exactly one place.
    /// A second copy of 304.8 in the Revit half is how a unit bug gets in.</summary>
    [Fact]
    public void Body_NeverConvertsUnitsItself()
    {
        Assert.DoesNotContain("304.8", BakeSceneScript.Body);
        Assert.Contains("AwareBakeRules.MmToFeet(", BakeSceneScript.Body);
    }

    [Fact]
    public void Code_RunsTheWholeBakeInOneTransaction()
    {
        var code = BakeSceneScript.Code;
        Assert.Equal(1, CountOccurrences(code, "new Transaction("));
        Assert.Contains("tx.Commit();", code);
        Assert.Contains("tx.RollBack();", code);
    }

    static int CountOccurrences(string haystack, string needle)
    {
        var count = 0;
        var at = haystack.IndexOf(needle, StringComparison.Ordinal);
        while (at >= 0)
        {
            count++;
            at = haystack.IndexOf(needle, at + needle.Length, StringComparison.Ordinal);
        }
        return count;
    }

    // ── the script compiles ──────────────────────────────────────────────────

    [Fact]
    public void Code_ParsesAsACSharpScript()
    {
        var tree = CSharpSyntaxTree.ParseText(
            BakeSceneScript.Code,
            CSharpParseOptions.Default
                .WithKind(SourceCodeKind.Script)
                .WithLanguageVersion(LanguageVersion.Latest));

        var errors = tree.GetDiagnostics()
            .Where(d => d.Severity == DiagnosticSeverity.Error)
            .Select(d => d.ToString())
            .ToList();
        Assert.True(errors.Count == 0, string.Join("\n", errors));
    }

    /// <summary>Compile the shipped script against the REAL Revit API, exactly as
    /// the in-Revit ScriptEngine will. Skipped (passes trivially) on a machine
    /// without Revit installed — the parse test above still covers syntax there.</summary>
    [Fact]
    public void Code_CompilesAgainstTheRealRevitApi_WhenRevitIsInstalled()
    {
        var revitDir = FindRevitInstall();
        if (revitDir is null) return; // no Revit on this machine — nothing to compile against

        var references = new List<MetadataReference>();
        void AddFromType(Type t)
        {
            var location = t.Assembly.Location;
            if (!string.IsNullOrEmpty(location) && File.Exists(location))
                references.Add(MetadataReference.CreateFromFile(location));
        }
        AddFromType(typeof(object));
        AddFromType(typeof(System.Linq.Enumerable));
        AddFromType(typeof(IDictionary<,>));
        AddFromType(typeof(System.Dynamic.DynamicObject));
        AddFromType(typeof(JsonSerializer));
        references.Add(MetadataReference.CreateFromFile(Path.Combine(revitDir, "RevitAPI.dll")));
        references.Add(MetadataReference.CreateFromFile(Path.Combine(revitDir, "RevitAPIUI.dll")));

        // ScriptEngine supplies `uiapp` and `args` as globals; declare them as
        // locals instead so this compile needs no compile-time Revit reference in
        // the test assembly. The nullability of `args` is irrelevant — a script
        // compiles with the nullable context off.
        const string globalsStub = "UIApplication uiapp = null;\nIDictionary<string, object> args = null;\n";
        var code = BakeSceneScript.Header + globalsStub
            + BakeSceneScript.LoadRulesSource() + BakeSceneScript.Body;

        var options = ScriptOptions.Default
            .WithReferences(references)
            .WithImports(
                "System", "System.Collections.Generic", "System.Linq", "System.IO", "System.Text.Json",
                "Autodesk.Revit.ApplicationServices", "Autodesk.Revit.Attributes", "Autodesk.Revit.DB",
                "Autodesk.Revit.DB.Structure", "Autodesk.Revit.UI", "Autodesk.Revit.UI.Selection")
            .WithEmitDebugInformation(false);

        var diagnostics = CSharpScript.Create<object>(code, options).Compile();
        var errors = diagnostics
            .Where(d => d.Severity == DiagnosticSeverity.Error)
            .Select(d => d.ToString())
            .ToList();
        Assert.True(errors.Count == 0, string.Join("\n", errors));
    }

    /// <summary>A directory holding RevitAPI.dll + RevitAPIUI.dll: a local Revit
    /// install first, else Autodesk's reference-assembly package if it happens to
    /// be in the NuGet cache. Null when neither is present.</summary>
    static string? FindRevitInstall()
    {
        var programFiles = Environment.GetEnvironmentVariable("ProgramW6432")
                           ?? Environment.GetFolderPath(Environment.SpecialFolder.ProgramFiles);
        foreach (var year in new[] { "2026", "2025" })
        {
            var dir = Path.Combine(programFiles, "Autodesk", $"Revit {year}");
            if (HasRevitApi(dir)) return dir;
        }

        var packages = Path.Combine(
            Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), ".nuget", "packages", "autodesk.revit.sdk");
        if (Directory.Exists(packages))
        {
            foreach (var version in Directory.EnumerateDirectories(packages).OrderByDescending(d => d))
            {
                var dir = Path.Combine(version, "ref", "net8.0");
                if (HasRevitApi(dir)) return dir;
            }
        }
        return null;
    }

    static bool HasRevitApi(string dir) =>
        File.Exists(Path.Combine(dir, "RevitAPI.dll")) && File.Exists(Path.Combine(dir, "RevitAPIUI.dll"));

    // ── the verb ─────────────────────────────────────────────────────────────

    const string MinimalScene = """
    {
      "meta": { "name": "Bay 1", "units": "mm", "sourceId": "src-a", "sceneHash": "hash-a" },
      "elements": [
        { "id": "m1", "kind": "member", "from": [0,0,0], "to": [5000,0,0], "meta": { "profile": "W16x50" } }
      ]
    }
    """;

    [Fact]
    public async Task BakeSceneRunAsync_ShipsTheEmbeddedScriptWithSceneAndHashAndOwnsItsTransaction()
    {
        var pipeName = $"aware-revit-test-{Guid.NewGuid():N}";
        ExecRequest? captured = null;
        var serverTask = Task.Run(async () =>
        {
            using var server = new NamedPipeServerStream(pipeName, PipeDirection.InOut, 1,
                PipeTransmissionMode.Byte, PipeOptions.Asynchronous);
            await server.WaitForConnectionAsync();
            var reqJson = await PipeFrame.ReadAsync(server);
            captured = JsonSerializer.Deserialize<ExecRequest>(reqJson!, JsonOpts);
            var resp = new ExecResponse
            {
                Id = captured!.Id,
                Ok = true,
                Result = JsonNode.Parse("""{"ok":true,"created":1,"emitted":[{"id":"m1","kind":"member","status":"emitted"}]}"""),
                StdoutLog = "",
                HostVersion = "2026",
                HostPid = 4242,
            };
            await PipeFrame.WriteAsync(server, JsonSerializer.Serialize(resp, JsonOpts));
        });

        var input = new JsonObject
        {
            ["verb"] = "bake-scene",
            ["version"] = "2026",
            ["scene"] = JsonNode.Parse(MinimalScene),
        };

        var (exit, receipt) = await RunCapturing(() => BakeScene.RunAsync(input, pipeName));
        await serverTask;

        Assert.Equal(0, exit);
        Assert.NotNull(captured);
        Assert.Equal(BakeSceneScript.Code, captured!.Code);
        // The script opens its own Revit Transaction; a second, outer one throws.
        Assert.Equal("none", captured.Transaction);
        Assert.True(captured.Args.ContainsKey("scene"));
        Assert.True(captured.Args.ContainsKey("materializationHash"));
        var hash = captured.Args["materializationHash"]?.ToString();
        Assert.Equal(64, hash!.Length);

        Assert.NotNull(receipt);
        Assert.True(receipt!["ok"]!.GetValue<bool>());
        Assert.Equal("bake-scene", receipt["verb"]!.GetValue<string>());
        Assert.Equal("revit", receipt["host"]!.GetValue<string>());
        Assert.Equal(1, receipt["result"]!["created"]!.GetValue<int>());
    }

    [Fact]
    public async Task BakeSceneRunAsync_RefusesARequestWithNoScene()
    {
        var (exit, receipt) = await RunCapturing(() => BakeScene.RunAsync(
            new JsonObject { ["verb"] = "bake-scene" }, "unused-pipe"));

        Assert.Equal(2, exit);
        Assert.NotNull(receipt);
        Assert.Equal("err", receipt!["status"]!.GetValue<string>());
        Assert.Equal("bake-scene", receipt["verb"]!.GetValue<string>());
        Assert.Contains("scene", receipt["error"]!.GetValue<string>());
    }

    [Fact]
    public async Task BakeSceneRunAsync_RelabelsAFailedExecReceiptAsBakeScene()
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
                Error = "InvalidOperationException: no document",
                Stack = "stub-stack",
                StdoutLog = "",
                HostVersion = "2026",
                HostPid = 4242,
            };
            await PipeFrame.WriteAsync(server, JsonSerializer.Serialize(resp, JsonOpts));
        });

        var input = new JsonObject
        {
            ["verb"] = "bake-scene",
            ["scene"] = JsonNode.Parse(MinimalScene),
        };
        var (exit, receipt) = await RunCapturing(() => BakeScene.RunAsync(input, pipeName));
        await serverTask;

        Assert.Equal(2, exit);
        Assert.NotNull(receipt);
        Assert.False(receipt!["ok"]!.GetValue<bool>());
        Assert.Equal("bake-scene", receipt["verb"]!.GetValue<string>());
        Assert.Contains("no document", receipt["error"]!.GetValue<string>());
    }

    static async Task<(int exit, JsonObject? receipt)> RunCapturing(Func<Task<int>> run)
    {
        using var sw = new StringWriter();
        var originalOut = Console.Out;
        Console.SetOut(sw);
        int exit;
        try { exit = await run(); }
        finally { Console.SetOut(originalOut); }
        return (exit, JsonNode.Parse(sw.ToString().Trim()) as JsonObject);
    }

    // ── materialization identity ─────────────────────────────────────────────

    [Fact]
    public void ComputeMaterializationHash_IsStableAndBindsSceneToHostVersion()
    {
        var scene = JsonNode.Parse(MinimalScene)!;
        var other = JsonNode.Parse(MinimalScene.Replace("W16x50", "W18X35"))!;

        var a = BakeScene.ComputeMaterializationHash(scene, "2026");
        var b = BakeScene.ComputeMaterializationHash(JsonNode.Parse(MinimalScene)!, "2026");
        Assert.Equal(a, b);
        Assert.Equal(64, a.Length);

        Assert.NotEqual(a, BakeScene.ComputeMaterializationHash(scene, "2025"));
        Assert.NotEqual(a, BakeScene.ComputeMaterializationHash(scene, null));
        Assert.NotEqual(a, BakeScene.ComputeMaterializationHash(other, "2026"));
    }
}
