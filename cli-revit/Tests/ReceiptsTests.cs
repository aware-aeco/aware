using System.Text.Json.Nodes;
using AwareRevit.Sidecar;
using Xunit;

namespace AwareRevit.Tests;

public class ReceiptsTests
{
    [Fact]
    public void ExecOk_HasAllRequiredFields()
    {
        var r = Receipts.ExecOk(JsonNode.Parse("42"), "2026", 1234, "");
        Assert.True(r["ok"]!.GetValue<bool>());
        Assert.Equal("revit", r["host"]!.GetValue<string>());
        Assert.Equal("exec", r["verb"]!.GetValue<string>());
        Assert.Equal("2026", r["host_version"]!.GetValue<string>());
        Assert.Equal(1234, r["host_pid"]!.GetValue<int>());
        Assert.Equal(42, r["result"]!.GetValue<int>());
        Assert.NotNull(r["delivered_at"]);
    }

    [Fact]
    public void ExecFail_SetsOkFalse()
    {
        var r = Receipts.ExecFail("nope", "stack here", "stdout here");
        Assert.False(r["ok"]!.GetValue<bool>());
        Assert.Equal("nope", r["error"]!.GetValue<string>());
        Assert.Equal("stack here", r["stack"]!.GetValue<string>());
        Assert.Equal("stdout here", r["stdout_log"]!.GetValue<string>());
        Assert.Equal("revit", r["host"]!.GetValue<string>());
        Assert.Equal("exec", r["verb"]!.GetValue<string>());
    }

    [Fact]
    public void ListOk_WrapsInstancesArray()
    {
        var arr = new JsonArray { new JsonObject { ["pid"] = 99 } };
        var r = Receipts.ListOk(arr);
        Assert.Equal("ok", r["status"]!.GetValue<string>());
        Assert.Equal("revit", r["host"]!.GetValue<string>());
        Assert.Single((JsonArray)r["instances"]!);
    }

    [Fact]
    public void SendStatusOk_IncludesSessionId()
    {
        var r = Receipts.SendStatusOk("hi", "2026", 7777);
        Assert.Equal("revit-7777", r["host_session_id"]!.GetValue<string>());
        Assert.Equal("hi", r["verb_result"]!["message"]!.GetValue<string>());
    }

    [Fact]
    public void GenericFail_BuildsErrEnvelope()
    {
        var r = Receipts.GenericFail("close", "boom");
        Assert.Equal("err", r["status"]!.GetValue<string>());
        Assert.Equal("close", r["verb"]!.GetValue<string>());
        Assert.Equal("boom", r["error"]!.GetValue<string>());
    }
}
