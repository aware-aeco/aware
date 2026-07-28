using System.Text.Json.Nodes;
using AwareRhino;
using AwareRhino.Verbs;
using Xunit;

namespace AwareRhino.Tests;

public class ListInstancesTests
{
    [Fact]
    public void ActiveDocumentAcceptsLegacyStringAndRhinoCode831Object()
    {
        Assert.Equal(@"C:\Models\legacy.3dm",
            ListInstances.ReadActiveDoc(JsonValue.Create(@"C:\Models\legacy.3dm")));
        Assert.Equal(@"C:\Models\current.3dm",
            ListInstances.ReadActiveDoc(JsonNode.Parse(
                """{"title":"current.3dm","location":"C:\\Models\\current.3dm"}""")));
        Assert.Equal("Untitled",
            ListInstances.ReadActiveDoc(JsonNode.Parse(
                """{"title":"Untitled","location":""}""")));
    }

    [Fact]
    public void BakeTargetParserAcceptsRhinoCode831ActiveDocumentObject()
    {
        var raw = JsonNode.Parse("""
            [{
              "pipeId":"rhinocode_remotepipe_40864",
              "processId":40864,
              "processVersion":"8.31.26126.13431",
              "activeDoc":{"title":"","location":""}
            }]
            """) as JsonArray;
        var instances = BakeSceneRules.ReadInstances(raw!);
        var instance = Assert.Single(instances);
        Assert.Equal(40864, instance.Pid);
        Assert.Equal("8.31", instance.Version);
        Assert.Equal("", instance.ActiveDoc);
    }
}
