using AwareReader;
using AwareReader.Recipes;
using Xunit;

namespace AwareSidecar.Tests;

// #180 PR-D: the Tekla model-plugin recipe. Exercised against the two-DLL fixture graph —
// DemoPlugin (FixtureAssembly) : PluginBase + [Plugin], taking DemoData (FixtureDataAssembly,
// the cross-assembly contract) via ctor; DemoData's [StructuresField] double/int/string fields
// become the emitted insert command's typed inputs.
public class TeklaRecipeTests
{
    private static string BinDir() =>
        Path.GetDirectoryName(typeof(TeklaRecipeTests).Assembly.Location)!;

    private static ReflectedSet FixtureSet() =>
        MetadataReflector.ReflectSet(
        [
            Path.Combine(BinDir(), "FixtureAssembly.dll"),
            Path.Combine(BinDir(), "FixtureDataAssembly.dll"),
        ]);

    private static GeneratedAgent SynthFixture() => AgentSynthesizer.Synthesize(FixtureSet());

    [Fact]
    public void RecipeDetectsPluginSet()
    {
        var recipe = RecipeRegistry.Detect(FixtureSet());
        Assert.NotNull(recipe);
        Assert.Equal("tekla-plugin", recipe!.Name);
    }

    [Fact]
    public void EmitsInsertCommandAndDropsPerMethodRun()
    {
        var agent = SynthFixture();
        var names = agent.Commands.Select(c => c.Name).ToList();
        Assert.Contains("insert-demo-plugin", names);
        Assert.DoesNotContain("demo-plugin-run", names);
    }

    [Fact]
    public void InsertCommandIsWriteModeWithTypedStructuresFieldInputs()
    {
        var agent = SynthFixture();
        var insert = agent.Commands.First(c => c.Name == "insert-demo-plugin");
        Assert.Equal("write", insert.Mode);
        Assert.Equal("single", insert.Lifecycle);

        // The picked points + one input per [StructuresField], typed double/int/string.
        Assert.Contains(insert.Inputs, i => i.Name == "input-points" && !i.Optional);
        Assert.Contains(insert.Inputs, i => i.Name == "length" && i.Type == "number");
        Assert.Contains(insert.Inputs, i => i.Name == "name" && i.Type == "string");
        Assert.Contains(insert.Inputs, i => i.Name == "count" && i.Type == "integer");
    }

    [Fact]
    public void MatchesViaIntermediateBaseChain()
    {
        // IntermediatePlugin : MidBase : PluginBase — the chain is walked through MidBase
        // (resolved via the cross-assembly index), so it is recognized too.
        var names = SynthFixture().Commands.Select(c => c.Name).ToList();
        Assert.Contains("insert-intermediate-plugin", names);
        Assert.DoesNotContain("intermediate-plugin-run", names);
    }

    [Fact]
    public void DoesNotMatchPluginBaseSubclassWithoutPluginAttribute()
    {
        // DerivedNoAttr : PluginBase but lacks [Plugin] — left as a plain per-method command.
        var names = SynthFixture().Commands.Select(c => c.Name).ToList();
        Assert.Contains("derived-no-attr-run", names);
        Assert.DoesNotContain("insert-derived-no-attr", names);
    }

    [Fact]
    public void DoesNotMatchPluginAttributeWithoutPluginBase()
    {
        // OrphanPlugin has [Plugin] but no PluginBase base — not a model plug-in.
        var names = SynthFixture().Commands.Select(c => c.Name).ToList();
        Assert.Contains("orphan-plugin-do-thing", names);
        Assert.DoesNotContain("insert-orphan", names);
    }

    [Fact]
    public void NoRecipeWhenSetHasNoPlugins()
    {
        // The data assembly alone has PluginBase + DemoData but no [Plugin] type → no recipe.
        var set = MetadataReflector.ReflectSet([Path.Combine(BinDir(), "FixtureDataAssembly.dll")]);
        Assert.Null(RecipeRegistry.Detect(set));
    }
}
