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

    /// <summary>
    /// Verifies that the resolver sibling-discovery path does not crash when called with a
    /// real DLL that has sibling assemblies in the same directory.  The fixture assembly lives
    /// next to the test runner DLLs, so we expect at least the fixture itself to be present
    /// and reflection to succeed (same coverage as ReflectsFixtureAssemblyTypesAndMethods,
    /// but explicitly exercises the code path added for Bug 1).
    /// </summary>
    [Fact]
    public void SiblingDllsInSameDirectoryAreDiscoveredWithoutError()
    {
        // The fixture DLL sits next to xunit, System.Reflection.MetadataLoadContext, etc.
        // All of those are in the same directory — the resolver should pick them up as
        // siblings and not throw "Could not find assembly".
        var path = FixturePath();
        Assert.True(File.Exists(path), $"fixture not at {path}");

        // A second DLL that lives in the same directory as the fixture
        var here = Path.GetDirectoryName(path)!;
        var anySibling = Directory.EnumerateFiles(here, "*.dll")
            .FirstOrDefault(f => !string.Equals(f, path, StringComparison.OrdinalIgnoreCase));
        Assert.NotNull(anySibling); // test environment must have at least one sibling

        // Passing only the fixture path; the resolver should auto-discover siblings.
        var ex = Record.Exception(() => DllReflector.Reflect(new[] { path }, agentIdOverride: "sibling-test"));
        Assert.Null(ex);
    }

    /// <summary>
    /// Verifies that passing a bare directory path (no glob suffix) auto-expands to *.dll
    /// and successfully reflects the DLLs inside — specifically FixtureAssembly.dll.
    /// This is the UX fix: callers no longer need to append \*.dll manually.
    /// </summary>
    [Fact]
    public void DirectoryPathAutoExpandsToDlls()
    {
        var fixturePath = FixturePath();
        Assert.True(File.Exists(fixturePath), $"fixture not at {fixturePath}");

        // Pass the directory that contains FixtureAssembly.dll — no *.dll suffix.
        var dir = Path.GetDirectoryName(fixturePath)!;
        var agent = DllReflector.Reflect(new[] { dir }, agentIdOverride: "dir-expand-test");

        // The directory contains FixtureAssembly.dll so commands from it must be present.
        var commandNames = agent.Commands.Select(c => c.Name).ToList();
        Assert.Contains("greeter-greet", commandNames);
    }

    /// <summary>
    /// Smoke-tests GetRefPackRoots (indirectly via Reflect) on the current OS:
    /// verifies that at least one packs root either exists or the scan is silently skipped.
    /// On a developer machine with .NET SDK installed this will enumerate ref-pack DLLs;
    /// on a stripped CI agent the directories simply won't exist and nothing should throw.
    /// </summary>
    [Fact]
    public void RefPackScanDoesNotThrowRegardlessOfSdkInstallation()
    {
        var path = FixturePath();
        Assert.True(File.Exists(path), $"fixture not at {path}");
        // If the packs dir is absent the test passes silently; if it is present the resolver
        // must still succeed (it just has more paths).
        var ex = Record.Exception(() => DllReflector.Reflect(new[] { path }, agentIdOverride: "refpack-test"));
        Assert.Null(ex);
    }
}
