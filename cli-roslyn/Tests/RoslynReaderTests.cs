using AwareReader;
using AwareRoslyn;
using Xunit;

namespace AwareRoslyn.Tests;

// #180 PR-C: the source reader. Writes throwaway .cs files and asserts the produced IR
// matches what the compiled-DLL reader would emit (attributes, fields, doc summaries),
// so the shared AgentSynthesizer + recipe behave identically on source and compiled inputs.
public sealed class RoslynReaderTests : IDisposable
{
    private readonly string _dir;

    public RoslynReaderTests()
    {
        _dir = Path.Combine(Path.GetTempPath(), "aware-roslyn-test-" + Guid.NewGuid().ToString("N"));
        Directory.CreateDirectory(_dir);
    }

    public void Dispose()
    {
        try { Directory.Delete(_dir, recursive: true); } catch { /* best effort */ }
    }

    private string WriteCs(string name, string source)
    {
        var path = Path.Combine(_dir, name);
        File.WriteAllText(path, source);
        return path;
    }

    private const string MarkedSource = """
        namespace Demo
        {
            [System.AttributeUsage(System.AttributeTargets.All)]
            public sealed class DemoMarkerAttribute : System.Attribute
            {
                public DemoMarkerAttribute(string name) { Name = name; }
                public string Name { get; }
                public int Order { get; set; }
            }

            /// <summary>A widget that does widget things.</summary>
            [DemoMarker("widget", Order = 3)]
            public class MarkedWidget
            {
                [DemoMarker("field-marker")] public double Size;

                /// <summary>Configures the widget.</summary>
                public void Configure(int value) { }
            }
        }
        """;

    [Fact]
    public void ReflectsSourceTypesMethodsAndFields()
    {
        var path = WriteCs("Marked.cs", MarkedSource);
        var reflection = RoslynReader.ReflectPaths(new[] { path }, Array.Empty<string>());
        var t = reflection.Set.TypeIndex["Demo.MarkedWidget"];

        Assert.Contains(t.Methods, m => m.Name == "Configure");
        Assert.Contains(t.Fields, f => f.Name == "Size" && f.Type == "System.Double");
    }

    [Fact]
    public void DecodesTypeAndFieldAttributesWithArgs()
    {
        var path = WriteCs("Marked.cs", MarkedSource);
        var reflection = RoslynReader.ReflectPaths(new[] { path }, Array.Empty<string>());
        var t = reflection.Set.TypeIndex["Demo.MarkedWidget"];

        var typeMarker = t.Attributes.First(a => a.TypeName == "Demo.DemoMarkerAttribute");
        Assert.Equal("widget", typeMarker.FixedArguments.Single().Value);
        Assert.Equal("3", typeMarker.NamedArguments.First(n => n.Name == "Order").Value);

        var size = t.Fields.First(f => f.Name == "Size");
        Assert.Contains(size.Attributes, a =>
            a.TypeName == "Demo.DemoMarkerAttribute" && a.FixedArguments.Any(x => x.Value == "field-marker"));
    }

    [Fact]
    public void HarvestsDocSummaries()
    {
        var path = WriteCs("Marked.cs", MarkedSource);
        var reflection = RoslynReader.ReflectPaths(new[] { path }, Array.Empty<string>());
        Assert.True(reflection.DocSummaries.TryGetValue("T:Demo.MarkedWidget", out var ts));
        Assert.Contains("widget things", ts);
        Assert.True(reflection.DocSummaries.TryGetValue("M:Demo.MarkedWidget.Configure(System.Int32)", out var ms));
        Assert.Contains("Configures", ms);
    }

    [Fact]
    public void SynthesizesAgentFromSourceWithSummaries()
    {
        var path = WriteCs("Marked.cs", MarkedSource);
        var reflection = RoslynReader.ReflectPaths(new[] { path }, Array.Empty<string>());
        var agent = AgentSynthesizer.Synthesize(
            reflection.Set, new SynthesisOptions { DocSummaries = reflection.DocSummaries });

        var cmd = agent.Commands.First(c => c.Name == "marked-widget-configure");
        Assert.Contains("Configures", cmd.Description);
    }

    [Fact]
    public void ProjectAndSolutionInputsAreRejectedWithActionableError()
    {
        var ex = Assert.Throws<InvalidOperationException>(
            () => RoslynReader.ReflectPaths(new[] { "Some.csproj" }, Array.Empty<string>()));
        Assert.Contains("--reference-dir", ex.Message);
    }
}
