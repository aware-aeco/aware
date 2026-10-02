using System.Text.Json.Nodes;
using Xunit;

namespace AwareTekla.Tests;

// #617: `model-info` is the tekla agent's connection probe. The AWARE probe runtime reads exactly
// two things from a refusal — the kebab-case `code` and `instance_count` — and the declared report
// pointers from a success (/model_name, /host, /host_pid, /host_version). These pin both shapes
// without a live Tekla, so renaming a field the probe reads fails here rather than in the field.
public class ModelInfoTests
{
    static Program.TeklaInstance Inst(int pid, string version) =>
        new(pid, version, $@"C:\Program Files\Tekla Structures\{version}\bin\TeklaStructures.exe");

    [Fact]
    public void Receipt_CarriesEveryFieldTheProbeReports()
    {
        var receipt = Program.BuildModelInfoReceipt(Inst(25068, "2026.0"), "Aware Tests.db1", @"C:\Models\Aware Tests");
        Assert.Equal("ok", receipt["status"]!.GetValue<string>());
        Assert.Equal("tekla", receipt["host"]!.GetValue<string>());
        Assert.Equal(25068, receipt["host_pid"]!.GetValue<int>());
        Assert.Equal("2026.0", receipt["host_version"]!.GetValue<string>());
        Assert.Equal("Aware Tests.db1", receipt["model_name"]!.GetValue<string>());
        Assert.Equal("model-info", receipt["verb"]!.GetValue<string>());
    }

    [Theory]
    [InlineData("host-not-running")]
    [InlineData("host-ambiguous")]
    [InlineData("host-not-connected")]
    [InlineData("model-closed")]
    [InlineData("model-read-failed")]
    public void Failure_IsAStructuredErrReceipt(string code)
    {
        var failure = Program.BuildModelInfoFailure(code, 2, "2026.0", 25068);
        Assert.Equal("err", failure["status"]!.GetValue<string>());
        Assert.Equal(code, failure["code"]!.GetValue<string>());
        Assert.Equal(2, failure["instance_count"]!.GetValue<int>());
        // No free-text field a vendor message could ride out on.
        Assert.False(failure.ContainsKey("message"));
        Assert.False(failure.ContainsKey("error"));
        Assert.False(failure.ContainsKey("stack"));
    }

    [Fact]
    public void TwoSameVersionInstances_AreRefusedNotGuessed()
    {
        // model-info selects its host with exec's rule: two of one major is ambiguous.
        var instances = new System.Collections.Generic.List<Program.TeklaInstance>
        {
            Inst(1, "2026.0"),
            Inst(2, "2026.0"),
        };
        var target = Program.ResolveExecTarget(null, null, instances.Count, instances);
        Assert.Equal(Program.ExecTargetKind.Ambiguous, target.Kind);
    }

    [Fact]
    public void ReadStringMember_ReadsAPropertyOrAField()
    {
        Assert.Equal("a", Program.ReadStringMember(new WithProperty(), "ModelName"));
        Assert.Equal("b", Program.ReadStringMember(new WithField(), "ModelName"));
        Assert.Null(Program.ReadStringMember(null, "ModelName"));
        Assert.Null(Program.ReadStringMember(new WithField(), "ModelPath"));
    }

    public sealed class WithProperty { public string ModelName { get; } = "a"; }
    public sealed class WithField { public string ModelName = "b"; }
}
