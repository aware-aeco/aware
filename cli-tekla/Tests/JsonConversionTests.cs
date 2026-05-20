using System.Collections.Generic;
using System.Text.Json.Nodes;
using Xunit;

namespace AwareTekla.Tests;

public class JsonConversionTests
{
    [Fact]
    public void JsonNodeToObject_NullNode_ReturnsNull()
    {
        Assert.Null(Program.JsonNodeToObject(null));
    }

    [Fact]
    public void JsonNodeToObject_Bool_ReturnsBool()
    {
        Assert.Equal(true, Program.JsonNodeToObject(JsonValue.Create(true)));
    }

    [Fact]
    public void JsonNodeToObject_Integer_ReturnsInt()
    {
        // bool is tried before int, so a plain integer should come back as int.
        Assert.Equal(42, Program.JsonNodeToObject(JsonValue.Create(42)));
    }

    [Fact]
    public void JsonNodeToObject_String_ReturnsString()
    {
        Assert.Equal("hello", Program.JsonNodeToObject(JsonValue.Create("hello")));
    }

    [Fact]
    public void JsonNodeToObject_Array_ReturnsListOfConvertedItems()
    {
        var arr = new JsonArray(JsonValue.Create(1), JsonValue.Create("two"), JsonValue.Create(true));
        var result = Assert.IsType<List<object?>>(Program.JsonNodeToObject(arr));
        Assert.Equal(3, result.Count);
        Assert.Equal(1, result[0]);
        Assert.Equal("two", result[1]);
        Assert.Equal(true, result[2]);
    }

    [Fact]
    public void JsonObjectToDictionary_Null_ReturnsEmptyDictionary()
    {
        var dict = Program.JsonObjectToDictionary(null);
        Assert.Empty(dict);
    }

    [Fact]
    public void JsonObjectToDictionary_FlatObject_MapsKeysAndValues()
    {
        var obj = new JsonObject
        {
            ["name"] = "beam",
            ["count"] = 3,
            ["active"] = true,
        };
        var dict = Program.JsonObjectToDictionary(obj);
        Assert.Equal("beam", dict["name"]);
        Assert.Equal(3, dict["count"]);
        Assert.Equal(true, dict["active"]);
    }

    [Fact]
    public void JsonObjectToDictionary_NestedObject_RecursesIntoDictionary()
    {
        var obj = new JsonObject
        {
            ["outer"] = new JsonObject { ["inner"] = "value" },
        };
        var dict = Program.JsonObjectToDictionary(obj);
        var inner = Assert.IsType<Dictionary<string, object?>>(dict["outer"]);
        Assert.Equal("value", inner["inner"]);
    }
}
