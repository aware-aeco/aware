using System.Collections.Generic;
using Xunit;

namespace AwareTekla.Tests;

public class ParseArgsTests
{
    [Fact]
    public void EmptyArgs_ProducesAllDefaults()
    {
        var p = Program.ParseArgs(new string[0]);
        Assert.Null(p.Version);
        Assert.Null(p.Pid);
        Assert.False(p.JsonStdin);
        Assert.False(p.All);
    }

    [Fact]
    public void ParsesVersionFlag()
    {
        var p = Program.ParseArgs(new[] { "--version", "2026.0" });
        Assert.Equal("2026.0", p.Version);
    }

    [Fact]
    public void ParsesPidFlag_AsInt()
    {
        var p = Program.ParseArgs(new[] { "--pid", "12345" });
        Assert.Equal(12345, p.Pid);
    }

    [Fact]
    public void ParsesAllAndJsonStdinSwitches()
    {
        var p = Program.ParseArgs(new[] { "--all", "--json-stdin" });
        Assert.True(p.All);
        Assert.True(p.JsonStdin);
    }

    [Fact]
    public void ParsesCombinedFlags_InAnyOrder()
    {
        var p = Program.ParseArgs(new[] { "--json-stdin", "--version", "2025.0", "--pid", "999" });
        Assert.True(p.JsonStdin);
        Assert.Equal("2025.0", p.Version);
        Assert.Equal(999, p.Pid);
    }

    [Fact]
    public void UnknownFlag_Throws()
    {
        var ex = Assert.Throws<System.InvalidOperationException>(
            () => Program.ParseArgs(new[] { "--bogus" }));
        Assert.Contains("unknown flag", ex.Message);
    }
}
