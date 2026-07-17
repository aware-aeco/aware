using System.Collections.Generic;
using Xunit;

namespace AwareTekla.Tests;

// #290: exec/bake connect out-of-process via `new Model()`, which the Tekla Open API cannot bind
// to a chosen instance. With more than one Tekla live it may attach to a different instance than
// the one whose DLLs were loaded — a version mismatch there hard-crashes the CLR (0xe0434352).
// ResolveExecTarget is the pure selection that refuses that ambiguity; it also preserves the #264
// single-instance behaviour (bind to the running version, not the requested one). These lock in
// its behaviour without a live Tekla.
public class ResolveExecTargetTests
{
    static Program.TeklaInstance Inst(int pid, string version) =>
        new(pid, version, $@"C:\Program Files\Tekla Structures\{version}\bin\TeklaStructures.exe");

    static List<Program.TeklaInstance> None() => new();

    [Fact]
    public void NoInstance_NoPid_IsSmokeTestPath()
    {
        var t = Program.ResolveExecTarget(null, "2026.0", None());
        Assert.Equal(Program.ExecTargetKind.NoHost, t.Kind);
        Assert.Null(t.Instance);
    }

    [Fact]
    public void NoInstance_WithPid_IsNotRunning()
    {
        // An explicit --pid asks for a live target; nothing is running → clear error, not smoke test.
        var t = Program.ResolveExecTarget(1234, null, None());
        Assert.Equal(Program.ExecTargetKind.NotRunning, t.Kind);
        Assert.Contains("1234", t.Message);
    }

    [Fact]
    public void SingleInstance_NoFilter_Resolves()
    {
        var t = Program.ResolveExecTarget(null, null, new List<Program.TeklaInstance> { Inst(100, "2025.0") });
        Assert.Equal(Program.ExecTargetKind.Resolved, t.Kind);
        Assert.Equal(100, t.Instance!.Pid);
        Assert.Equal("2025.0", t.Instance.Version);
    }

    [Fact]
    public void SingleInstance_RequestedVersionDiffers_StillBindsToRunning()
    {
        // #264: caller asks for 2026.0 but only 2025.0 is open — bind to the running one.
        var t = Program.ResolveExecTarget(null, "2026.0", new List<Program.TeklaInstance> { Inst(100, "2025.0") });
        Assert.Equal(Program.ExecTargetKind.Resolved, t.Kind);
        Assert.Equal("2025.0", t.Instance!.Version);
    }

    [Fact]
    public void SingleInstance_PidMatches_Resolves()
    {
        var t = Program.ResolveExecTarget(100, null, new List<Program.TeklaInstance> { Inst(100, "2025.0") });
        Assert.Equal(Program.ExecTargetKind.Resolved, t.Kind);
        Assert.Equal(100, t.Instance!.Pid);
    }

    [Fact]
    public void SingleInstance_PidMismatch_IsNotRunning()
    {
        // The user explicitly named a PID that isn't the one running — assert it, don't silently
        // run against the other instance.
        var t = Program.ResolveExecTarget(999, null, new List<Program.TeklaInstance> { Inst(100, "2025.0") });
        Assert.Equal(Program.ExecTargetKind.NotRunning, t.Kind);
        Assert.Contains("999", t.Message);
        Assert.Contains("100", t.Message);
    }

    [Fact]
    public void MultipleInstances_DifferentVersions_IsAmbiguous()
    {
        // The exact #290 crash setup: 2025.0 + 2026.0 both open. Refuse rather than roulette-connect.
        var t = Program.ResolveExecTarget(null, null,
            new List<Program.TeklaInstance> { Inst(100, "2025.0"), Inst(200, "2026.0") });
        Assert.Equal(Program.ExecTargetKind.Ambiguous, t.Kind);
        Assert.Null(t.Instance);
        Assert.Contains("2 Tekla instances", t.Message);
        Assert.Contains("100", t.Message);
        Assert.Contains("200", t.Message);
    }

    [Fact]
    public void MultipleInstances_SameVersion_IsStillAmbiguous()
    {
        // Even same-version instances are ambiguous: the API still binds to an unpredictable one, so
        // a write could hit the wrong model. Refuse.
        var t = Program.ResolveExecTarget(null, "2026.0",
            new List<Program.TeklaInstance> { Inst(100, "2026.0"), Inst(200, "2026.0") });
        Assert.Equal(Program.ExecTargetKind.Ambiguous, t.Kind);
    }

    [Fact]
    public void MultipleInstances_PidNamesOne_IsStillAmbiguous()
    {
        // --pid cannot rescue multi-instance: there is no per-PID binding out-of-process, so the API
        // may still attach elsewhere. Refuse regardless of --pid.
        var t = Program.ResolveExecTarget(200, null,
            new List<Program.TeklaInstance> { Inst(100, "2025.0"), Inst(200, "2026.0") });
        Assert.Equal(Program.ExecTargetKind.Ambiguous, t.Kind);
    }
}
