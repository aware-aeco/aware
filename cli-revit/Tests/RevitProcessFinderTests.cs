using AwareRevit.Sidecar;
using Xunit;

namespace AwareRevit.Tests;

public class RevitProcessFinderTests
{
    [Fact]
    public void ExtractVersionFromPath_HappyPath_2026()
    {
        var v = RevitProcessFinder.ExtractVersionFromPath(
            @"C:\Program Files\Autodesk\Revit 2026\Revit.exe");
        Assert.Equal("2026", v);
    }

    [Fact]
    public void ExtractVersionFromPath_HappyPath_2025()
    {
        var v = RevitProcessFinder.ExtractVersionFromPath(
            @"C:\Program Files\Autodesk\Revit 2025\Revit.exe");
        Assert.Equal("2025", v);
    }

    [Fact]
    public void ExtractVersionFromPath_ReturnsNullOnUnrecognized()
    {
        Assert.Null(RevitProcessFinder.ExtractVersionFromPath(@"C:\nope\Revit.exe"));
    }

    [Fact]
    public void Filter_ByVersion_MatchesSubstring()
    {
        var all = new List<RevitInstance>
        {
            new(1, "2025", "p1", false),
            new(2, "2026", "p2", true),
        };
        var matches = RevitProcessFinder.Filter(all, version: "2026", pid: null);
        Assert.Single(matches);
        Assert.Equal(2, matches[0].Pid);
    }

    [Fact]
    public void Filter_ByPid_MatchesExact()
    {
        var all = new List<RevitInstance>
        {
            new(1, "2025", "p1", false),
            new(2, "2026", "p2", true),
        };
        var matches = RevitProcessFinder.Filter(all, version: null, pid: 1);
        Assert.Single(matches);
        Assert.Equal("2025", matches[0].Version);
    }

    [Fact]
    public void Filter_NoCriteria_ReturnsAll()
    {
        var all = new List<RevitInstance>
        {
            new(1, "2025", "p1", false),
            new(2, "2026", "p2", true),
        };
        var matches = RevitProcessFinder.Filter(all, version: null, pid: null);
        Assert.Equal(2, matches.Count);
    }

    [Fact]
    public void PipeNameFor_FormatsAsExpected()
    {
        Assert.Equal("aware-revit-12345", RevitProcessFinder.PipeNameFor(12345));
    }
}
