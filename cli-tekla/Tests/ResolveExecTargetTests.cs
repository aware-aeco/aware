using System.Collections.Generic;
using System.Text.Json.Nodes;
using Xunit;

namespace AwareTekla.Tests;

// #290: exec/bake connect out-of-process via `new Model()`, which the Tekla Open API cannot bind
// to a chosen instance. With more than one Tekla live it may attach to a different instance than
// the one whose DLLs were loaded — a version mismatch there hard-crashes the CLR (0xe0434352).
// ResolveExecTarget is the pure selection that refuses that ambiguity. Safety keys off the RAW
// process count, not just the instances we could inspect (a process we can't read still exists).
// It also preserves the #264 single-instance behaviour (bind to the running version, not the
// requested one). These lock in its behaviour without a live Tekla.
public class ResolveExecTargetTests
{
    static Program.TeklaInstance Inst(int pid, string version) =>
        new(pid, version, $@"C:\Program Files\Tekla Structures\{version}\bin\TeklaStructures.exe");

    // Convenience: raw count == inspected count (the "all processes inspectable" case).
    static Program.ExecTarget Resolve(int? pid, string? version, List<Program.TeklaInstance> instances) =>
        Program.ResolveExecTarget(pid, version, instances.Count, instances);

    static List<Program.TeklaInstance> None() => new();

    [Fact]
    public void NoProcess_NoPid_IsSmokeTestPath()
    {
        var t = Resolve(null, "2026.0", None());
        Assert.Equal(Program.ExecTargetKind.NoHost, t.Kind);
        Assert.Null(t.Instance);
    }

    [Fact]
    public void NoProcess_WithPid_IsNotRunning()
    {
        // An explicit --pid asks for a live target; nothing is running → clear error, not smoke test.
        var t = Resolve(1234, null, None());
        Assert.Equal(Program.ExecTargetKind.NotRunning, t.Kind);
        Assert.Contains("1234", t.Message);
    }

    [Fact]
    public void SingleInstance_NoFilter_Resolves()
    {
        var t = Resolve(null, null, new List<Program.TeklaInstance> { Inst(100, "2025.0") });
        Assert.Equal(Program.ExecTargetKind.Resolved, t.Kind);
        Assert.Equal(100, t.Instance!.Pid);
        Assert.Equal("2025.0", t.Instance.Version);
    }

    [Fact]
    public void SingleInstance_RequestedVersionDiffers_StillBindsToRunning()
    {
        // #264: caller asks for 2026.0 but only 2025.0 is open — bind to the running one.
        var t = Resolve(null, "2026.0", new List<Program.TeklaInstance> { Inst(100, "2025.0") });
        Assert.Equal(Program.ExecTargetKind.Resolved, t.Kind);
        Assert.Equal("2025.0", t.Instance!.Version);
    }

    [Fact]
    public void SingleInstance_PidMatches_Resolves()
    {
        var t = Resolve(100, null, new List<Program.TeklaInstance> { Inst(100, "2025.0") });
        Assert.Equal(Program.ExecTargetKind.Resolved, t.Kind);
        Assert.Equal(100, t.Instance!.Pid);
    }

    [Fact]
    public void SingleInstance_PidMismatch_IsNotRunning()
    {
        // The user explicitly named a PID that isn't the one running — assert it, don't silently
        // run against the other instance.
        var t = Resolve(999, null, new List<Program.TeklaInstance> { Inst(100, "2025.0") });
        Assert.Equal(Program.ExecTargetKind.NotRunning, t.Kind);
        Assert.Contains("999", t.Message);
        Assert.Contains("100", t.Message);
    }

    [Fact]
    public void MultipleInstances_DifferentVersions_IsAmbiguous()
    {
        // The exact #290 crash setup: 2025.0 + 2026.0 both open. Refuse rather than roulette-connect.
        var t = Resolve(null, null,
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
        var t = Resolve(null, "2026.0",
            new List<Program.TeklaInstance> { Inst(100, "2026.0"), Inst(200, "2026.0") });
        Assert.Equal(Program.ExecTargetKind.Ambiguous, t.Kind);
    }

    [Fact]
    public void MultipleInstances_PidNamesOne_IsStillAmbiguous()
    {
        // --pid cannot rescue multi-instance: there is no per-PID binding out-of-process, so the API
        // may still attach elsewhere. Refuse regardless of --pid.
        var t = Resolve(200, null,
            new List<Program.TeklaInstance> { Inst(100, "2025.0"), Inst(200, "2026.0") });
        Assert.Equal(Program.ExecTargetKind.Ambiguous, t.Kind);
    }

    [Fact]
    public void UninspectableProcess_CountsTowardAmbiguity()
    {
        // #290 review: 2 raw TeklaStructures.exe processes but only one could be inspected (the other
        // runs at a different elevation / custom path). new Model() could still attach to the hidden
        // one, so this must be Ambiguous even though `instances` has a single entry.
        var t = Program.ResolveExecTarget(
            null, null, rawProcessCount: 2, new List<Program.TeklaInstance> { Inst(100, "2025.0") });
        Assert.Equal(Program.ExecTargetKind.Ambiguous, t.Kind);
        Assert.Contains("could not be inspected", t.Message);
    }

    [Fact]
    public void SingleUninspectableProcess_IsAmbiguous_NotSmokeTest()
    {
        // One raw process, none inspectable: we can't determine its version, and new Model() would
        // attach to it — don't fall through to the smoke-test/requested-version path.
        var t = Program.ResolveExecTarget(null, "2026.0", rawProcessCount: 1, None());
        Assert.Equal(Program.ExecTargetKind.Ambiguous, t.Kind);
    }

    [Fact]
    public void MalformedStdinPid_IsRejected()
    {
        // A present-but-non-integer `pid` is a caller error, not an absent pid (#290 review).
        var input = JsonNode.Parse("{\"pid\":\"1234\"}");
        var ok = Program.TryReadPid(input, new Program.ParsedArgs(), out var pid, out var error);
        Assert.False(ok);
        Assert.Null(pid);
        Assert.Contains("invalid", error!);
    }

    [Fact]
    public void AbsentStdinPid_FallsBackToCliFlag()
    {
        var input = JsonNode.Parse("{\"code\":\"return 1;\"}");
        var ok = Program.TryReadPid(input, new Program.ParsedArgs { Pid = 42 }, out var pid, out var error);
        Assert.True(ok);
        Assert.Equal(42, pid);
        Assert.Null(error);
    }

    [Fact]
    public void ValidStdinPid_TakesPrecedenceOverCliFlag()
    {
        var input = JsonNode.Parse("{\"pid\":7}");
        var ok = Program.TryReadPid(input, new Program.ParsedArgs { Pid = 42 }, out var pid, out var error);
        Assert.True(ok);
        Assert.Equal(7, pid);
        Assert.Null(error);
    }
}
