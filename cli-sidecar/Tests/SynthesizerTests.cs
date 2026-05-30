using AwareReader;
using Xunit;

namespace AwareSidecar.Tests;

// #180 PR-B: multi-assembly reflection (MetadataReflector.ReflectSet + cross-assembly
// TypeIndex) and the AgentSynthesizer (coherent agent across a set, not first-wins).
// Exercised against the real two-DLL fixture graph: FixtureAssembly (the plug-in) +
// FixtureDataAssembly (PluginBase + [Plugin]/[StructuresField] + the DemoData contract).
public class SynthesizerTests
{
    private static string BinDir() =>
        Path.GetDirectoryName(typeof(SynthesizerTests).Assembly.Location)!;

    private static string[] FixtureSet() =>
    [
        Path.Combine(BinDir(), "FixtureAssembly.dll"),
        Path.Combine(BinDir(), "FixtureDataAssembly.dll"),
    ];

    [Fact]
    public void ReflectSetBuildsCrossAssemblyIndex()
    {
        var set = MetadataReflector.ReflectSet(FixtureSet());

        // Both assemblies are present.
        Assert.Equal(2, set.Assemblies.Count);

        // The index spans both DLLs: the plug-in lives in FixtureAssembly, its data
        // contract + PluginBase live in FixtureDataAssembly.
        Assert.True(set.TypeIndex.ContainsKey("FixtureAssembly.TeklaLike.DemoPlugin"),
            "plug-in type missing from cross-assembly index");
        Assert.True(set.TypeIndex.ContainsKey("FixtureData.DemoData"),
            "data contract (other assembly) missing from cross-assembly index");
        Assert.True(set.TypeIndex.ContainsKey("Tekla.Structures.Plugins.PluginBase"),
            "PluginBase (other assembly) missing from cross-assembly index");
    }

    [Fact]
    public void DemoPluginConstructorParamResolvesAcrossAssemblies()
    {
        // The cross-assembly join the Tekla recipe depends on (PR-D): the plug-in's ctor
        // parameter type (DemoData) lives in a DIFFERENT assembly and must resolve via the index.
        var set = MetadataReflector.ReflectSet(FixtureSet());
        var plugin = set.TypeIndex["FixtureAssembly.TeklaLike.DemoPlugin"];
        var ctorParamType = plugin.Constructors
            .SelectMany(c => c.Parameters)
            .Select(p => p.Type)
            .First(t => t.EndsWith("DemoData", StringComparison.Ordinal));
        Assert.True(set.TypeIndex.ContainsKey(ctorParamType),
            $"ctor param type {ctorParamType} should resolve in the cross-assembly index");
        var data = set.TypeIndex[ctorParamType];
        Assert.Contains(data.Fields, f => f.Name == "Length");
    }

    [Fact]
    public void SynthesizerChoosesPrimaryAssemblyForId()
    {
        // FixtureAssembly contributes more public types than FixtureDataAssembly, so it is
        // the primary — id is derived from it, not from whichever assembly came first.
        var set = MetadataReflector.ReflectSet(FixtureSet());
        var agent = AgentSynthesizer.Synthesize(set);
        Assert.Equal("fixture-assembly", agent.Id);
    }

    [Fact]
    public void SynthesizerRespectsIdOverride()
    {
        var set = MetadataReflector.ReflectSet(FixtureSet());
        var agent = AgentSynthesizer.Synthesize(set, new SynthesisOptions { AgentIdOverride = "custom-id" });
        Assert.Equal("custom-id", agent.Id);
    }

    [Fact]
    public void SynthesizerEmitsCommandsAcrossTheWholeSet()
    {
        var set = MetadataReflector.ReflectSet(FixtureSet());
        var agent = AgentSynthesizer.Synthesize(set);
        var names = agent.Commands.Select(c => c.Name).ToList();
        // From FixtureAssembly (Greeter, a plain non-plug-in type) — present across the set.
        // (The Tekla recipe rewrites the plug-in's per-method commands; see TeklaRecipeTests.)
        Assert.Contains("greeter-greet", names);
        // Command names are unique and ordinal-sorted (determinism).
        Assert.Equal(names.Count, names.Distinct().Count());
        Assert.Equal(names.OrderBy(n => n, StringComparer.Ordinal).ToList(), names);
    }

    [Fact]
    public void SynthesizerCarriesDocSummariesIntoCommandDescriptions()
    {
        var set = MetadataReflector.ReflectSet(FixtureSet());
        var docs = new Dictionary<string, string>
        {
            ["M:FixtureAssembly.Demo.Greeter.Greet(System.String)"] = "Returns a greeting for the given name.",
        };
        var agent = AgentSynthesizer.Synthesize(set, new SynthesisOptions { DocSummaries = docs });
        var greet = agent.Commands.First(c => c.Name == "greeter-greet");
        Assert.Contains("greeting", greet.Description, StringComparison.OrdinalIgnoreCase);
    }
}
