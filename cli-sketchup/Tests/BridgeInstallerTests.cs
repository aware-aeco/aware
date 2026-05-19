using System.IO;
using AwareSketchup;
using Xunit;

namespace AwareSketchup.Tests;

public class BridgeInstallerTests : IDisposable
{
    readonly string _root;

    public BridgeInstallerTests()
    {
        _root = Path.Combine(Path.GetTempPath(), $"aware-sketchup-installer-test-{Guid.NewGuid():N}");
        Directory.CreateDirectory(_root);
    }

    public void Dispose()
    {
        try { Directory.Delete(_root, recursive: true); } catch { }
    }

    [Fact]
    public void DefaultPluginsDir_FollowsAppDataConvention()
    {
        var got = BridgeInstaller.DefaultPluginsDir(2026);
        Assert.Contains("SketchUp", got);
        Assert.Contains("2026", got);
        Assert.Contains("Plugins", got);
    }

    [Fact]
    public void ParseArgs_DefaultsToCurrentYear()
    {
        var (year, dir) = BridgeInstaller.ParseArgs(new[] { "--install-bridge" });
        Assert.Equal(2026, year);
        Assert.Null(dir);
    }

    [Fact]
    public void ParseArgs_ReadsTargetYear()
    {
        var (year, _) = BridgeInstaller.ParseArgs(new[] { "--install-bridge", "--target-year", "2025" });
        Assert.Equal(2025, year);
    }

    [Fact]
    public void ParseArgs_ReadsPluginsDir()
    {
        var (_, dir) = BridgeInstaller.ParseArgs(new[] { "--install-bridge", "--plugins-dir", "C:\\foo" });
        Assert.Equal("C:\\foo", dir);
    }

    [Fact]
    public void Install_CopiesAssets()
    {
        // The build-output BridgeAssets/ folder lives next to the test assembly's
        // own copy of aware-sketchup.dll (we link to the main project).
        var pluginsDir = Path.Combine(_root, "Plugins");

        // Capture stdout so the receipt doesn't bleed into test output.
        using var sw = new StringWriter();
        var original = Console.Out;
        Console.SetOut(sw);
        int exit;
        try
        {
            exit = BridgeInstaller.Install(new[] { "--install-bridge", "--plugins-dir", pluginsDir });
        }
        finally
        {
            Console.SetOut(original);
        }

        Assert.Equal(0, exit);
        Assert.True(File.Exists(Path.Combine(pluginsDir, "aware_sketchup_bridge.rb")));
        Assert.True(File.Exists(Path.Combine(pluginsDir, "aware_sketchup_bridge", "core.rb")));
        // Receipt is ok-shaped.
        Assert.Contains("\"status\":\"ok\"", sw.ToString());
    }

    [Fact]
    public void Uninstall_RemovesAssets()
    {
        var pluginsDir = Path.Combine(_root, "Plugins");
        // Install first.
        using (var sw0 = new StringWriter())
        {
            var original0 = Console.Out;
            Console.SetOut(sw0);
            try { BridgeInstaller.Install(new[] { "--install-bridge", "--plugins-dir", pluginsDir }); }
            finally { Console.SetOut(original0); }
        }
        Assert.True(File.Exists(Path.Combine(pluginsDir, "aware_sketchup_bridge.rb")));

        using var sw = new StringWriter();
        var original = Console.Out;
        Console.SetOut(sw);
        int exit;
        try
        {
            exit = BridgeInstaller.Uninstall(new[] { "--uninstall-bridge", "--plugins-dir", pluginsDir });
        }
        finally
        {
            Console.SetOut(original);
        }
        Assert.Equal(0, exit);
        Assert.False(File.Exists(Path.Combine(pluginsDir, "aware_sketchup_bridge.rb")));
        Assert.False(Directory.Exists(Path.Combine(pluginsDir, "aware_sketchup_bridge")));
    }

    [Fact]
    public void Status_NoInstall_ReportsFalse()
    {
        var pluginsDir = Path.Combine(_root, "empty-plugins");
        Directory.CreateDirectory(pluginsDir);

        using var sw = new StringWriter();
        var original = Console.Out;
        Console.SetOut(sw);
        try { BridgeInstaller.Status(new[] { "--bridge-status", "--plugins-dir", pluginsDir }); }
        finally { Console.SetOut(original); }

        var json = sw.ToString();
        Assert.Contains("\"loader_present\":false", json);
        Assert.Contains("\"support_present\":false", json);
    }

    [Fact]
    public void Status_AfterInstall_ReportsTrue()
    {
        var pluginsDir = Path.Combine(_root, "installed-plugins");
        using (var sw0 = new StringWriter())
        {
            var o0 = Console.Out;
            Console.SetOut(sw0);
            try { BridgeInstaller.Install(new[] { "--install-bridge", "--plugins-dir", pluginsDir }); }
            finally { Console.SetOut(o0); }
        }

        using var sw = new StringWriter();
        var original = Console.Out;
        Console.SetOut(sw);
        try { BridgeInstaller.Status(new[] { "--bridge-status", "--plugins-dir", pluginsDir }); }
        finally { Console.SetOut(original); }

        var json = sw.ToString();
        Assert.Contains("\"loader_present\":true", json);
        Assert.Contains("\"support_present\":true", json);
        Assert.Contains("\"up_to_date\":true", json);
        Assert.Contains("\"installed_version\":\"0.34.0\"", json);
    }

    [Fact]
    public void ReadInstalledVersion_ExtractsConst()
    {
        var tmp = Path.GetTempFileName();
        try
        {
            File.WriteAllText(tmp, "BRIDGE_VERSION = '1.2.3'.freeze\n");
            Assert.Equal("1.2.3", BridgeInstaller.ReadInstalledVersion(tmp));
        }
        finally { File.Delete(tmp); }
    }

    [Fact]
    public void ReadInstalledVersion_HandlesMissingFile()
    {
        Assert.Null(BridgeInstaller.ReadInstalledVersion(Path.Combine(_root, "nope.rb")));
    }
}
