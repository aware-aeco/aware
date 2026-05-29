using System;
using System.IO;
using System.Linq;
using System.Text.Json.Nodes;
using Xunit;

namespace AwareTekla.Tests;

/// <summary>
/// Pure-logic tests for the <c>watch</c> verb's change-type mapping, filter
/// predicate, and event shape — no live Tekla required.
/// </summary>
public class WatchLogicTests
{
    [Theory]
    [InlineData(0, "added")]    // OBJECT_INSERT
    [InlineData(1, "modified")] // OBJECT_MODIFY
    [InlineData(2, "removed")]  // OBJECT_DELETE
    [InlineData(3, "modified")] // USERPROPERTY_CHANGED
    [InlineData(99, "modified")] // unknown → modified (never drops the event)
    public void MapChangeType_MapsTeklaEnumToStreamEnum(int value, string expected)
    {
        Assert.Equal(expected, Program.MapChangeType(value));
    }

    [Theory]
    [InlineData("all", "Beam", true)]
    [InlineData("all", "Weld", true)]
    [InlineData("assembly", "Assembly", true)]
    [InlineData("assembly", "Beam", false)]
    [InlineData("welded", "Weld", true)]
    [InlineData("welded", "PolygonWeld", true)]
    [InlineData("welded", "BoltArray", false)]
    [InlineData("bolted", "BoltArray", true)]
    [InlineData("bolted", "BoltGroup", true)]
    [InlineData("bolted", "Weld", false)]
    [InlineData("drawing", "GADrawing", true)]
    [InlineData("drawing", "Beam", false)]
    [InlineData("ALL", "Beam", true)]            // case-insensitive
    [InlineData("unknown-filter", "Beam", true)] // unknown → pass-through, not silent drop
    public void WatchFilterMatches_DiscriminatesByObjectKind(string filter, string typeName, bool expected)
    {
        Assert.Equal(expected, Program.WatchFilterMatches(filter, typeName));
    }

    [Fact]
    public void BuildWatchEvent_HasFiredSignalAndDeclaredFields()
    {
        var geom = new JsonObject { ["min"] = new JsonObject { ["x"] = 0.0 } };
        var ev = Program.BuildWatchEvent("g-1", "A/1", "Assembly", "added", geom);

        Assert.Equal("fired", ev["signal"]!.GetValue<string>());
        Assert.Equal("g-1", ev["guid"]!.GetValue<string>());
        Assert.Equal("A/1", ev["mark"]!.GetValue<string>());
        Assert.Equal("Assembly", ev["type"]!.GetValue<string>());
        Assert.Equal("added", ev["change"]!.GetValue<string>());
        Assert.Equal("tekla", ev["host"]!.GetValue<string>());
        Assert.NotNull(ev["geometry"]);
        Assert.False(string.IsNullOrEmpty(ev["delivered_at"]!.GetValue<string>()));
    }

    [Fact]
    public void BuildWatchEvent_NullableFieldsSerializeAsJsonNull()
    {
        var ev = Program.BuildWatchEvent(guid: null, mark: null, "Beam", "removed", geometry: null);
        // A removed object emits identity-only: guid/mark/geometry are JSON null,
        // not missing — downstream sinks can rely on a stable shape.
        Assert.True(ev.ContainsKey("guid"));
        Assert.Null(ev["guid"]);
        Assert.Null(ev["mark"]);
        Assert.Null(ev["geometry"]);
        Assert.Equal("removed", ev["change"]!.GetValue<string>());
    }
}

/// <summary>
/// End-to-end test of the <c>watch</c> verb's self-test mode through the real
/// <see cref="Program.Watch"/> entry point: drives stdin/stdout/stderr exactly
/// as the runtime's streaming transport does, and asserts that stdout carries
/// ONLY <c>fired</c> data events (control records like <c>listening</c> go to
/// stderr so they never propagate downstream). The analogue of #173's
/// fake-streamer e2e, but exercising the actual bridge code instead of a stub
/// binary — and Tekla-free.
///
/// These redirect process-global Console.In/Out/Error, so the collection
/// disables parallelization to keep concurrent tests from clobbering the redirect.
/// </summary>
[Collection("WatchConsole")]
public class WatchSelfTestEndToEndTests
{
    static (JsonNode[] stdoutFired, JsonNode[] stderrDiag) RunWatch(string stdinJson)
    {
        var originalIn = Console.In;
        var originalOut = Console.Out;
        var originalErr = Console.Error;
        try
        {
            Console.SetIn(new StringReader(stdinJson));
            var outW = new StringWriter();
            var errW = new StringWriter();
            Console.SetOut(outW);
            Console.SetError(errW);

            int rc = Program.Watch(new Program.ParsedArgs { JsonStdin = true });
            Assert.Equal(0, rc); // self-test exits cleanly, mirroring a finite real run

            return (ParseLines(outW.ToString()), ParseLines(errW.ToString()));
        }
        finally
        {
            Console.SetIn(originalIn);
            Console.SetOut(originalOut);
            Console.SetError(originalErr);
        }
    }

    static JsonNode[] ParseLines(string text) => text
        .Split('\n')
        .Select(l => l.Trim())
        .Where(l => l.Length > 0)
        .Select(l => JsonNode.Parse(l)!)
        .ToArray();

    [Fact]
    public void SelfTest_StdoutCarriesOnlyFiredEvents_ListeningGoesToStderr()
    {
        var (stdoutFired, stderrDiag) = RunWatch(@"{""self_test"":true,""filter"":""all""}");

        // stdout: only `fired` data events (no control records that would
        // propagate downstream as a phantom change).
        Assert.True(stdoutFired.Length >= 4, $"expected several fired events, got {stdoutFired.Length}");
        Assert.All(stdoutFired, e => Assert.Equal("fired", e["signal"]!.GetValue<string>()));
        Assert.DoesNotContain(stdoutFired, e => e["signal"]!.GetValue<string>() == "listening");
        // Deletes are dropped by default (include_deleted=false).
        Assert.DoesNotContain(stdoutFired, e => e["change"]!.GetValue<string>() == "removed");

        // stderr: the listening breadcrumb, carrying the resolved filter.
        var listening = Assert.Single(stderrDiag, e => e["signal"]!.GetValue<string>() == "listening");
        Assert.Equal("all", listening["filter"]!.GetValue<string>());
    }

    [Fact]
    public void SelfTest_WeldedFilter_EmitsOnlyWeldEvents()
    {
        var (fired, _) = RunWatch(@"{""self_test"":true,""filter"":""welded""}");

        Assert.NotEmpty(fired);
        Assert.All(fired, e =>
            Assert.Contains("Weld", e["type"]!.GetValue<string>(), StringComparison.OrdinalIgnoreCase));
    }

    [Fact]
    public void SelfTest_IncludeDeleted_EmitsRemovedEvents()
    {
        var (fired, _) = RunWatch(@"{""self_test"":true,""filter"":""all"",""include_deleted"":true}");
        Assert.Contains(fired, e => e["change"]!.GetValue<string>() == "removed");
    }

    [Fact]
    public void SelfTest_DefaultsToAll_WhenNoFilterGiven()
    {
        var (fired, stderrDiag) = RunWatch(@"{""self_test"":true}");
        Assert.NotEmpty(fired);
        var listening = Assert.Single(stderrDiag, e => e["signal"]!.GetValue<string>() == "listening");
        Assert.Equal("all", listening["filter"]!.GetValue<string>());
    }
}

/// Drives the worker-thread <see cref="Program.OnModelObjectChanged"/> handler
/// with fake ChangeData-shaped objects (duck-typed via the same reflection the
/// real handler uses: a <c>Type</c> enum property + an <c>Object</c> carrying an
/// <c>Identifier.GUID</c>). Covers the reflection extraction mechanics — property
/// names, enum→int mapping, IEnumerable iteration, filter-by-runtime-type — that
/// the synthetic self-test path can't reach. Console-global, so serialized.
[Collection("WatchConsole")]
public class WatchHandlerReflectionTests
{
    enum FakeChangeType { OBJECT_INSERT = 0, OBJECT_MODIFY = 1, OBJECT_DELETE = 2 }

    sealed class FakeIdentifier
    {
        public Guid GUID { get; }
        public FakeIdentifier(Guid g) => GUID = g;
    }

    // Runtime type name == "Weld"/"Beam"/… drives the filter, mirroring how the
    // real handler reads `modelObject.GetType().Name`.
    class Beam { public FakeIdentifier Identifier { get; } = new FakeIdentifier(Guid.NewGuid()); }
    sealed class Weld : Beam { }

    sealed class FakeChangeData
    {
        public FakeChangeType Type { get; }
        public object Object { get; }
        public FakeChangeData(FakeChangeType t, object o) { Type = t; Object = o; }
    }

    static string[] Capture(string filter, bool includeDeleted, System.Collections.IEnumerable changes)
    {
        var originalOut = Console.Out;
        try
        {
            Program._watchFilter = filter;
            Program._watchIncludeDeleted = includeDeleted;
            var sw = new StringWriter();
            Console.SetOut(sw);
            Program.OnModelObjectChanged(changes);
            return sw.ToString()
                .Split('\n')
                .Select(l => l.Trim())
                .Where(l => l.Length > 0)
                .ToArray();
        }
        finally
        {
            Console.SetOut(originalOut);
            Program._watchFilter = "all";
            Program._watchIncludeDeleted = false;
        }
    }

    [Fact]
    public void ExtractsGuidTypeAndChange_FromReflectedChangeData()
    {
        var beamGuid = Guid.NewGuid();
        var changes = new System.Collections.Generic.List<object>
        {
            new FakeChangeData(FakeChangeType.OBJECT_INSERT, new Beam()),
            new FakeChangeData(FakeChangeType.OBJECT_MODIFY, new Weld { }),
        };

        var lines = Capture("all", includeDeleted: false, changes);
        Assert.Equal(2, lines.Length);

        var beam = JsonNode.Parse(lines[0])!;
        Assert.Equal("fired", beam["signal"]!.GetValue<string>());
        Assert.Equal("Beam", beam["type"]!.GetValue<string>());
        Assert.Equal("added", beam["change"]!.GetValue<string>());
        Assert.False(string.IsNullOrEmpty(beam["guid"]!.GetValue<string>()));

        var weld = JsonNode.Parse(lines[1])!;
        Assert.Equal("Weld", weld["type"]!.GetValue<string>());
        Assert.Equal("modified", weld["change"]!.GetValue<string>());
    }

    [Fact]
    public void HonorsFilter_DroppingNonMatchingTypes()
    {
        var changes = new System.Collections.Generic.List<object>
        {
            new FakeChangeData(FakeChangeType.OBJECT_INSERT, new Beam()),
            new FakeChangeData(FakeChangeType.OBJECT_INSERT, new Weld()),
        };
        var lines = Capture("welded", includeDeleted: false, changes);
        var only = Assert.Single(lines);
        Assert.Equal("Weld", JsonNode.Parse(only)!["type"]!.GetValue<string>());
    }

    [Fact]
    public void DropsDeletes_UnlessIncludeDeleted()
    {
        var changes = new System.Collections.Generic.List<object>
        {
            new FakeChangeData(FakeChangeType.OBJECT_DELETE, new Beam()),
        };
        Assert.Empty(Capture("all", includeDeleted: false, changes));

        var lines = Capture("all", includeDeleted: true, changes);
        var only = Assert.Single(lines);
        Assert.Equal("removed", JsonNode.Parse(only)!["change"]!.GetValue<string>());
    }

    [Fact]
    public void NonEnumerablePayload_EmitsNothing_AndDoesNotThrow()
    {
        Assert.Empty(Capture("all", includeDeleted: false, new System.Collections.Generic.List<object>()));
    }
}

[CollectionDefinition("WatchConsole", DisableParallelization = true)]
public class WatchConsoleCollection { }
