using System;
using System.Collections.Generic;
using System.Text.Json.Nodes;
using Xunit;

namespace AwareTekla.Tests;

public class SerializeResultTests
{
    [Fact]
    public void Null_ReturnsNull()
    {
        Assert.Null(Program.SerializeResult(null));
    }

    [Fact]
    public void String_RoundTrips()
    {
        var node = Program.SerializeResult("hello");
        Assert.Equal("hello", node!.GetValue<string>());
    }

    [Fact]
    public void Int_RoundTrips()
    {
        var node = Program.SerializeResult(7);
        Assert.Equal(7, node!.GetValue<int>());
    }

    [Fact]
    public void Bool_RoundTrips()
    {
        var node = Program.SerializeResult(true);
        Assert.True(node!.GetValue<bool>());
    }

    [Fact]
    public void Guid_SerializesAsString()
    {
        var g = Guid.NewGuid();
        var node = Program.SerializeResult(g);
        Assert.Equal(g.ToString(), node!.GetValue<string>());
    }

    [Fact]
    public void Enumerable_BecomesJsonArray()
    {
        var node = Program.SerializeResult(new List<int> { 1, 2, 3 });
        var arr = Assert.IsType<JsonArray>(node);
        Assert.Equal(3, arr.Count);
        Assert.Equal(1, arr[0]!.GetValue<int>());
    }

    [Fact]
    public void Dictionary_BecomesJsonObject()
    {
        var input = new Dictionary<string, object?> { ["a"] = 1, ["b"] = "two" };
        var node = Program.SerializeResult(input);
        var obj = Assert.IsType<JsonObject>(node);
        Assert.Equal(1, obj["a"]!.GetValue<int>());
        Assert.Equal("two", obj["b"]!.GetValue<string>());
    }

    [Fact]
    public void ObjectWithGuidProperty_SerializesThatGuid()
    {
        var id = new FakeIdentifier { GUID = Guid.NewGuid() };
        var node = Program.SerializeResult(id);
        Assert.Equal(id.GUID.ToString(), node!.GetValue<string>());
    }

    [Fact]
    public void PlainPoco_FallsBackToJsonObject()
    {
        var node = Program.SerializeResult(new Point { X = 4, Y = 9 });
        var obj = Assert.IsType<JsonObject>(node);
        Assert.Equal(4, obj["X"]!.GetValue<int>());
        Assert.Equal(9, obj["Y"]!.GetValue<int>());
    }

    sealed class FakeIdentifier
    {
        public Guid GUID { get; set; }
    }

    sealed class Point
    {
        public int X { get; set; }
        public int Y { get; set; }
    }
}
