using System.Collections.Generic;
using System.Linq;
using Xunit;

namespace AwareTekla.Tests;

public class FilterTargetsTests
{
    static List<Program.TeklaInstance> SampleInstances() => new()
    {
        new Program.TeklaInstance(100, "2026.0", @"C:\Program Files\Tekla Structures\2026.0\bin\TeklaStructures.exe"),
        new Program.TeklaInstance(200, "2025.0", @"C:\Program Files\Tekla Structures\2025.0\bin\TeklaStructures.exe"),
        new Program.TeklaInstance(300, "2026.0", @"C:\Program Files\Tekla Structures\2026.0\bin\TeklaStructures.exe"),
    };

    [Fact]
    public void Pid_TakesPrecedence_AndMatchesExactlyOne()
    {
        var result = Program.FilterTargets(SampleInstances(), new Program.ParsedArgs { Pid = 200 });
        Assert.Single(result);
        Assert.Equal(200, result[0].Pid);
    }

    [Fact]
    public void Version_MatchesAllInstancesOfThatVersion()
    {
        var result = Program.FilterTargets(SampleInstances(), new Program.ParsedArgs { Version = "2026.0" });
        Assert.Equal(2, result.Count);
        Assert.All(result, i => Assert.Equal("2026.0", i.Version));
    }

    [Fact]
    public void Pid_WinsOverVersion_WhenBothSet()
    {
        // pid 100 is a 2026.0 instance; version filter would match two, pid narrows to one.
        var result = Program.FilterTargets(SampleInstances(),
            new Program.ParsedArgs { Pid = 100, Version = "2025.0" });
        Assert.Single(result);
        Assert.Equal(100, result[0].Pid);
    }

    [Fact]
    public void NoSelectors_ReturnsAll()
    {
        var result = Program.FilterTargets(SampleInstances(), new Program.ParsedArgs());
        Assert.Equal(3, result.Count);
    }

    [Fact]
    public void Pid_WithNoMatch_ReturnsEmpty()
    {
        var result = Program.FilterTargets(SampleInstances(), new Program.ParsedArgs { Pid = 999 });
        Assert.Empty(result);
    }
}
