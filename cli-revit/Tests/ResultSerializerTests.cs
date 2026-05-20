using System.Text.Json.Nodes;
using AwareRevit.Sidecar;
using Xunit;

namespace AwareRevit.Tests;

public class ResultSerializerTests
{
    [Theory]
    [InlineData(true)]
    [InlineData(false)]
    public void Bool_Roundtrips(bool b)
    {
        var n = ResultSerializerCore.ToJson(b);
        Assert.Equal(b, n!.GetValue<bool>());
    }

    [Fact]
    public void Int_Roundtrips()
    {
        Assert.Equal(7, ResultSerializerCore.ToJson(7)!.GetValue<int>());
    }

    [Fact]
    public void String_Roundtrips()
    {
        Assert.Equal("hi", ResultSerializerCore.ToJson("hi")!.GetValue<string>());
    }

    [Fact]
    public void Dictionary_RecursesValues()
    {
        var d = new Dictionary<string, object?> { ["a"] = 1, ["b"] = "x" };
        var n = (JsonObject)ResultSerializerCore.ToJson(d)!;
        Assert.Equal(1, n["a"]!.GetValue<int>());
        Assert.Equal("x", n["b"]!.GetValue<string>());
    }

    [Fact]
    public void List_RecursesItems()
    {
        var list = new List<object?> { 1, 2, "three" };
        var n = (JsonArray)ResultSerializerCore.ToJson(list)!;
        Assert.Equal(3, n.Count);
    }

    [Fact]
    public void AnonymousType_GetsSerialized()
    {
        var anon = new { x = 1, y = "foo" };
        var n = (JsonObject)ResultSerializerCore.ToJson(anon)!;
        Assert.Equal(1, n["x"]!.GetValue<int>());
        Assert.Equal("foo", n["y"]!.GetValue<string>());
    }

    [Fact]
    public void Null_BecomesJsonNull()
    {
        Assert.Null(ResultSerializerCore.ToJson(null));
    }
}
