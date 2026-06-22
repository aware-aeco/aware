using System.Collections.Generic;
using Xunit;

namespace AwareTekla.Tests;

// #264: a live op must bind its Tekla DLLs to the RUNNING instance's version (the only one the
// version-locked Open API can connect to), not the requested version. PickHostInstance is the
// pure selection that drives that — these lock in its behaviour without a live Tekla.
public class PickHostInstanceTests
{
    static Program.TeklaInstance Inst(int pid, string version) =>
        new(pid, version, $@"C:\Program Files\Tekla Structures\{version}\bin\TeklaStructures.exe");

    [Fact]
    public void PrefersExactVersionMatch_WhenRequestedIsRunning()
    {
        var running = new List<Program.TeklaInstance> { Inst(100, "2025.0"), Inst(200, "2026.0") };
        var pick = Program.PickHostInstance("2026.0", running);
        Assert.NotNull(pick);
        Assert.Equal("2026.0", pick!.Version);
        Assert.Equal(200, pick.Pid);
    }

    [Fact]
    public void FallsBackToRunningInstance_WhenRequestedMajorNotRunning()
    {
        // The footgun from the issue: caller asks for 2026.0 but only 2025.0 is open. We must bind
        // to 2025.0 (the instance that can actually connect), not the requested 2026.0.
        var running = new List<Program.TeklaInstance> { Inst(100, "2025.0") };
        var pick = Program.PickHostInstance("2026.0", running);
        Assert.NotNull(pick);
        Assert.Equal("2025.0", pick!.Version);
    }

    [Fact]
    public void UsesFirstRunningInstance_WhenNoVersionRequested()
    {
        var running = new List<Program.TeklaInstance> { Inst(100, "2024.0"), Inst(200, "2025.0") };
        var pick = Program.PickHostInstance(null, running);
        Assert.NotNull(pick);
        Assert.Equal("2024.0", pick!.Version);
    }

    [Fact]
    public void ReturnsNull_WhenNothingRunning()
    {
        // No instance → fall back to the requested version downstream (smoke-test path).
        Assert.Null(Program.PickHostInstance("2026.0", new List<Program.TeklaInstance>()));
        Assert.Null(Program.PickHostInstance(null, new List<Program.TeklaInstance>()));
    }
}
