using System.Text.Json.Nodes;
using AwareRhino;
using Xunit;

namespace AwareRhino.Tests;

public class BakeSceneRulesTests
{
    const string Minimal = """
        {
          "meta":{"units":"mm","sourceId":"drop-a","sceneHash":"scene-a"},
          "elements":[
            {"id":"M1","kind":"member","from":[0,0,0],"to":[1000,0,0],"section":{"w":200,"d":300}}
          ],
          "operations":[],
          "referenceSystems":[]
        }
        """;

    [Fact]
    public void ValidMemberPassesAndUsesOneSupportedRow()
    {
        var pf = BakeSceneRules.Validate(JsonNode.Parse(Minimal));
        Assert.True(pf.Ok);
        Assert.Equal("drop-a", pf.SourceId);
        Assert.Equal("scene-a", pf.SceneHash);
        Assert.Single(pf.Supported);
        Assert.Empty(pf.Unsupported);
    }

    [Theory]
    [InlineData("""{"meta":{"units":"ft","sourceId":"s","sceneHash":"h"},"elements":[]}""")]
    [InlineData("""{"meta":{"units":"mm","sourceId":"","sceneHash":"h"},"elements":[]}""")]
    [InlineData("""{"meta":{"units":"mm","sourceId":"s","sceneHash":""},"elements":[]}""")]
    [InlineData("""{"meta":{"units":"mm","sourceId":"s","sceneHash":"h"},"elements":{}}""")]
    public void InvalidEnvelopeFailsBeforeDispatch(string json)
    {
        Assert.False(BakeSceneRules.Validate(JsonNode.Parse(json)).Ok);
    }

    [Theory]
    [InlineData("[0,0,0]", "[0,0,0]", 10, 10)]
    [InlineData("[0,0,0]", "[1,0,0]", 0, 10)]
    [InlineData("[0,0,0]", "[1,0,0]", 10, -1)]
    public void InvalidMemberGeometryFailsWholeBatch(string from, string to, double w, double d)
    {
        var json = """
            {"meta":{"sourceId":"s","sceneHash":"h"},"elements":[
              {"id":"bad","kind":"member","from":FROM_VALUE,"to":TO_VALUE,"section":{"w":W_VALUE,"d":D_VALUE}},
              {"id":"good","kind":"member","from":[0,0,0],"to":[1,0,0],"section":{"w":1,"d":1}}
            ]}
            """.Replace("FROM_VALUE", from).Replace("TO_VALUE", to)
               .Replace("W_VALUE", w.ToString(System.Globalization.CultureInfo.InvariantCulture))
               .Replace("D_VALUE", d.ToString(System.Globalization.CultureInfo.InvariantCulture));
        var scene = JsonNode.Parse(json);
        var pf = BakeSceneRules.Validate(scene);
        Assert.False(pf.Ok);
        Assert.Contains(pf.Failed.OfType<JsonObject>(),
            r => r["id"]!.GetValue<string>() == "good"
                 && r["code"]!.GetValue<string>() == "batch-aborted");
    }

    [Fact]
    public void ReceiptAccountsForEveryUnsupportedNestedRecord()
    {
        var scene = JsonNode.Parse("""
            {
              "meta":{"sourceId":"s","sceneHash":"h"},
              "elements":[
                {"id":"M","kind":"member","from":[0,0,0],"to":[1,0,0],"section":{"w":1,"d":1}},
                {"id":"P","kind":"plate","holes":[{"id":"PH"}]}
              ],
              "operations":[
                {"id":"B","kind":"bolt-array","instances":[
                  {"id":"BI","holeEffects":[{"id":"BH"}]}
                ]}
              ],
              "referenceSystems":[
                {"id":"G","kind":"structural-grid","axes":[{"id":"GX"}],"levels":[{"id":"GL"}]}
              ]
            }
            """);
        var pf = BakeSceneRules.Validate(scene);
        Assert.True(pf.Ok);
        Assert.Single(pf.Supported);
        var rows = pf.Unsupported.OfType<JsonObject>().ToList();
        Assert.Equal(8, rows.Count);
        Assert.Contains(rows, r => r["id"]!.GetValue<string>() == "PH"
                                   && r["kind"]!.GetValue<string>() == "opening");
        Assert.Contains(rows, r => r["id"]!.GetValue<string>() == "BH"
                                   && r["kind"]!.GetValue<string>() == "opening");
        Assert.Contains(rows, r => r["id"]!.GetValue<string>() == "GX"
                                   && r["kind"]!.GetValue<string>() == "grid-axis");
    }

    [Fact]
    public void IdsAreUniqueAcrossTheEntireScene()
    {
        var scene = JsonNode.Parse("""
            {"meta":{"sourceId":"s","sceneHash":"h"},
             "elements":[{"id":"same","kind":"member","from":[0,0,0],"to":[1,0,0],"section":{"w":1,"d":1}}],
             "operations":[{"id":"same","kind":"weld"}]}
            """);
        var pf = BakeSceneRules.Validate(scene);
        Assert.False(pf.Ok);
        Assert.Contains(pf.Failed.OfType<JsonObject>(),
            r => r["code"]!.GetValue<string>() == "duplicate-id");
    }

    [Fact]
    public void OwnershipMarkerIsStrictAndMaterializationHashIsHostBound()
    {
        var scene = JsonNode.Parse(Minimal)!;
        var hash = BakeSceneRules.ComputeMaterializationHash(scene, "8.31");
        Assert.Equal(64, hash.Length);
        Assert.True(BakeSceneRules.IsOwnershipMarker(BakeSceneRules.BuildMarker(hash)));
        Assert.False(BakeSceneRules.IsOwnershipMarker("AWARE_BAKE_V1:" + new string('A', 64)));
        Assert.NotEqual(hash, BakeSceneRules.ComputeMaterializationHash(scene, "8.32"));
    }

    static IReadOnlyList<BakeSceneRules.RhinoInstance> Instances =>
    [
        new("pipe-a", 101, "8.31", "A.3dm"),
        new("pipe-b", 202, "8.31", "B.3dm"),
        new("pipe-c", 303, "8.32", "C.3dm"),
    ];

    [Fact]
    public void ExactRhinoIdWinsAndHostPidMustAgree()
    {
        var exact = BakeSceneRules.ResolveTarget(Instances, "pipe-b", null, null);
        Assert.True(exact.Ok);
        Assert.Equal(202, exact.Instance!.Pid);

        var agrees = BakeSceneRules.ResolveTarget(Instances, "pipe-b", 202, "8.31");
        Assert.True(agrees.Ok);

        var mismatch = BakeSceneRules.ResolveTarget(Instances, "pipe-b", 101, null);
        Assert.False(mismatch.Ok);
    }

    [Fact]
    public void WriteTargetNeverGuesses()
    {
        Assert.False(BakeSceneRules.ResolveTarget(Instances, null, null, null).Ok);
        Assert.False(BakeSceneRules.ResolveTarget(Instances, null, null, "8.31").Ok);
        Assert.Equal(303, BakeSceneRules.ResolveTarget(Instances, null, 303, null).Instance!.Pid);
        Assert.True(BakeSceneRules.ResolveTarget([Instances[0]], null, null, null).Ok);
        Assert.False(BakeSceneRules.ResolveTarget([], null, null, null).Ok);
    }
}
