using System.Diagnostics;
using AwareSidecar.Headers;
using Xunit;

namespace AwareSidecar.Tests;

public class HeaderParserTests
{
    private static bool ClangAvailable()
    {
        try
        {
            var psi = new ProcessStartInfo("clang", "--version")
            {
                RedirectStandardOutput = true,
                RedirectStandardError = true,
                UseShellExecute = false,
            };
            using var p = Process.Start(psi);
            if (p == null) return false;
            p.WaitForExit(5000);
            return p.ExitCode == 0;
        }
        catch { return false; }
    }

    private static string FixturePath()
    {
        // The fixture lives in the test project; it gets copied to the test output dir at build time
        var here = Path.GetDirectoryName(typeof(HeaderParserTests).Assembly.Location)!;
        return Path.Combine(here, "fixtures", "sample.h");
    }

    [Fact]
    public void MissingClangReturnsClearError()
    {
        if (ClangAvailable())
        {
            // Can't really test the "missing" path when clang is present.
            return;
        }
        var ex = Assert.Throws<InvalidOperationException>(() =>
            HeaderParser.Parse(new[] { FixturePath() }, null));
        Assert.Contains("clang", ex.Message);
    }

    [Fact]
    public void ParsesSampleHeaderWhenClangPresent()
    {
        if (!ClangAvailable())
        {
            // Skip silently when clang isn't installed
            return;
        }
        var fixture = FixturePath();
        Assert.True(File.Exists(fixture), $"fixture not at {fixture}");
        var agent = HeaderParser.Parse(new[] { fixture }, agentIdOverride: "sample");
        Assert.Equal("sample", agent.Id);
        // Expect at least the three free functions we declared
        var names = agent.Commands.Select(c => c.Name).ToList();
        Assert.Contains("sample-add", names);
        Assert.Contains("sample-mul", names);
        Assert.Contains("sample-greet", names);
    }

    [Fact]
    public void EmptyGlobThrows()
    {
        if (!ClangAvailable()) return;
        Assert.Throws<InvalidOperationException>(() =>
            HeaderParser.Parse(new[] { "C:/this-pattern-matches-nothing-*.h" }, null));
    }
}
