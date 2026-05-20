using System.Text.Json.Nodes;
using AwareSketchup;
using Xunit;

namespace AwareSketchup.Tests;

public class ReceiptsTests
{
    [Fact]
    public void ExecOk_HasRequiredFields()
    {
        var result = JsonNode.Parse("{\"count\":42}");
        var receipt = Receipts.ExecOk(result, "26.0", 12345, "12345", "noise");
        Assert.True(receipt["ok"]!.GetValue<bool>());
        Assert.Equal("sketchup", receipt["host"]!.GetValue<string>());
        Assert.Equal("exec", receipt["verb"]!.GetValue<string>());
        Assert.Equal(12345, receipt["host_pid"]!.GetValue<int>());
        Assert.Equal("26.0", receipt["host_version"]!.GetValue<string>());
        Assert.Equal("12345", receipt["sketchup_id"]!.GetValue<string>());
        Assert.Equal(42, receipt["result"]!["count"]!.GetValue<int>());
        Assert.Equal("noise", receipt["stdout_log"]!.GetValue<string>());
        Assert.NotNull(receipt["delivered_at"]);
    }

    [Fact]
    public void ExecFail_HasErrorAndStack()
    {
        var receipt = Receipts.ExecFail("boom", "stack trace here", "captured stdout");
        Assert.False(receipt["ok"]!.GetValue<bool>());
        Assert.Equal("sketchup", receipt["host"]!.GetValue<string>());
        Assert.Equal("exec", receipt["verb"]!.GetValue<string>());
        Assert.Equal("boom", receipt["error"]!.GetValue<string>());
        Assert.Equal("stack trace here", receipt["stack"]!.GetValue<string>());
        Assert.Equal("captured stdout", receipt["stdout_log"]!.GetValue<string>());
    }

    [Fact]
    public void ListOk_WrapsArray()
    {
        var arr = new JsonArray { new JsonObject { ["pid"] = 1, ["version"] = "26.0", ["port"] = 8765 } };
        var receipt = Receipts.ListOk(arr);
        Assert.Equal("ok", receipt["status"]!.GetValue<string>());
        Assert.Equal("list-instances", receipt["verb"]!.GetValue<string>());
        Assert.Equal(1, (receipt["instances"] as JsonArray)?.Count);
    }

    [Fact]
    public void SendStatusOk_HasHostSessionId()
    {
        var receipt = Receipts.SendStatusOk("hello", "26.0", 12345);
        Assert.Equal("ok", receipt["status"]!.GetValue<string>());
        Assert.Equal("sketchup-12345", receipt["host_session_id"]!.GetValue<string>());
        Assert.Equal("send-status", receipt["verb"]!.GetValue<string>());
        Assert.Equal("hello", receipt["verb_result"]!["message"]!.GetValue<string>());
    }

    [Fact]
    public void SendStatusOk_NullPidHidesSessionId()
    {
        var receipt = Receipts.SendStatusOk("hello", null, null);
        Assert.Null(receipt["host_session_id"]);
    }

    [Fact]
    public void GenericFail_AlwaysHasStatusErr()
    {
        var receipt = Receipts.GenericFail("close", "no instance");
        Assert.Equal("err", receipt["status"]!.GetValue<string>());
        Assert.Equal("sketchup", receipt["host"]!.GetValue<string>());
        Assert.Equal("close", receipt["verb"]!.GetValue<string>());
        Assert.Equal("no instance", receipt["error"]!.GetValue<string>());
        Assert.Null(receipt["stack"]);
    }

    [Fact]
    public void GenericFail_IncludesStackWhenProvided()
    {
        var receipt = Receipts.GenericFail("close", "boom", "trace");
        Assert.Equal("trace", receipt["stack"]!.GetValue<string>());
    }
}
