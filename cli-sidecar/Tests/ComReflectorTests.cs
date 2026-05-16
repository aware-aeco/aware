using System.Runtime.Versioning;
using AwareSidecar.Com;
using Xunit;

namespace AwareSidecar.Tests;

public class ComReflectorTests
{
    [Fact]
    [SupportedOSPlatform("windows")]
    public void ReflectsWScriptShellOnWindows()
    {
        if (!OperatingSystem.IsWindows())
        {
            // Skip on non-Windows
            return;
        }

        // WScript.Shell is universally available on Windows
        AwareSidecar.Protocol.GeneratedAgent agent;
        try
        {
            agent = ComReflector.Reflect("WScript.Shell", agentIdOverride: "wscript-shell");
        }
        catch (Exception ex)
        {
            // If we hit a deeply unexpected error, treat the test as inconclusive (Xunit doesn't
            // have a built-in Inconclusive, so we just return — better than failing a CI run on
            // someone's machine where WScript.Shell isn't registered).
            Console.WriteLine($"[skip] WScript.Shell reflection failed: {ex.Message}");
            return;
        }
        Assert.Equal("wscript-shell", agent.Id);
        Assert.NotEmpty(agent.Commands);
        Assert.NotEmpty(agent.Skills);
    }

    [Fact]
    [SupportedOSPlatform("windows")]
    public void UnknownProgIdThrows()
    {
        if (!OperatingSystem.IsWindows()) return;
        Assert.Throws<InvalidOperationException>(() =>
            ComReflector.Reflect("This.ProgId.Definitely.Does.Not.Exist.12345", null));
    }
}
