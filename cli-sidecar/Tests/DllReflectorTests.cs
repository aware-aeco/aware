using AwareSidecar.Protocol;
using AwareSidecar.Reflection;
using Xunit;

namespace AwareSidecar.Tests;

public class DllReflectorTests
{
    private static string FixturePath()
    {
        // The fixture assembly is referenced as a project, so its build output
        // is copied next to this test assembly.
        var here = Path.GetDirectoryName(typeof(DllReflectorTests).Assembly.Location)!;
        return Path.Combine(here, "FixtureAssembly.dll");
    }

    [Fact]
    public void ReflectsFixtureAssemblyTypesAndMethods()
    {
        var path = FixturePath();
        Assert.True(File.Exists(path), $"fixture not at {path}");
        var agent = DllReflector.Reflect(new[] { path }, agentIdOverride: "fixture");
        Assert.Equal("fixture", agent.Id);

        // Commands: at least one per public method on Greeter (Greet, CountOccurrences)
        var commandNames = agent.Commands.Select(c => c.Name).ToList();
        Assert.Contains("greeter-greet", commandNames);
        Assert.Contains("greeter-count-occurrences", commandNames);
    }

    [Fact]
    public void XmlDocSummariesPopulateCommandDescriptions()
    {
        var path = FixturePath();
        var agent = DllReflector.Reflect(new[] { path }, null);
        var greet = agent.Commands.FirstOrDefault(c => c.Name == "greeter-greet");
        Assert.NotNull(greet);
        Assert.Contains("greeting", greet!.Description, StringComparison.OrdinalIgnoreCase);
    }

    [Fact]
    public void SkillsAreEmittedPerNamespace()
    {
        var path = FixturePath();
        var agent = DllReflector.Reflect(new[] { path }, null);
        Assert.NotEmpty(agent.Skills);
        // FixtureAssembly.Demo namespace → kebab → "fixtureassembly-demo.md"
        Assert.Contains(agent.Skills, s => s.Filename.Contains("demo"));
    }

    [Fact]
    public void EmptyGlobsThrows()
    {
        Assert.Throws<InvalidOperationException>(() => DllReflector.Reflect(new[] { "C:/nonexistent-12345-*.dll" }, null));
    }
}
