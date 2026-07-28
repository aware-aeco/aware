using System.Text.Json.Nodes;
using AwareRhino;
using Xunit;

namespace AwareRhino.Tests;

public class CrossSectionProfileTests
{
    public static TheoryData<string, double, double, string, double, int> Families => new()
    {
        { """{"shape":"i","d":300,"bf":200,"tw":10,"tf":20}""", 200, 300, "i", 10600, 1 },
        { """{"shape":"channel","d":300,"bf":200,"tw":10,"tf":20}""", 200, 300, "channel", 10600, 1 },
        { """{"shape":"angle","d":300,"b":200,"t":10}""", 200, 300, "angle", 4900, 1 },
        { """{"shape":"rhs","d":300,"b":200,"t":10}""", 200, 300, "rhs", 9600, 2 },
        { """{"shape":"chs","od":200,"t":10}""", 200, 200, "chs", Math.PI * 1900, 2 },
        { """{"shape":"rect","w":200,"d":300}""", 200, 300, "rect", 60000, 1 },
    };

    [Theory]
    [MemberData(nameof(Families))]
    public void CanonicalFamiliesDecodeToExecutableNormalizedPlans(
        string xsection, double w, double d, string shape, double area, int profiles)
    {
        var element = new JsonObject { ["xsection"] = JsonNode.Parse(xsection) };
        var result = CrossSectionProfile.Decode(element, w, d);

        Assert.True(result.Ok, result.Error);
        Assert.Equal(shape, result.Profile!.Shape);
        Assert.Equal(w, result.Profile.Width);
        Assert.Equal(d, result.Profile.Depth);
        Assert.Equal(area, result.Profile.Area, 8);
        Assert.Equal(profiles, result.Profile.ProfileCount);
        Assert.All(result.Profile.Residuals, value => Assert.True(value > 0));
        var normalized = result.Profile.ToJson();
        Assert.Equal("rhino-profile-v2", normalized["revision"]!.GetValue<string>());
        Assert.Equal(2, normalized["capCount"]!.GetValue<int>());
    }

    [Fact]
    public void ISectionToleranceResidualUsesEachFlangeOutstand()
    {
        var element = new JsonObject
        {
            ["xsection"] = JsonNode.Parse(
                """{"shape":"i","d":300,"bf":200,"tw":10,"tf":20}"""),
        };

        var result = CrossSectionProfile.Decode(element, 200, 300);

        Assert.True(result.Ok, result.Error);
        Assert.Contains(95, result.Profile!.Residuals);
        Assert.DoesNotContain(190, result.Profile.Residuals);
    }

    [Fact]
    public void OnlyAnAbsentDescriptorUsesTheLegacyRectangle()
    {
        var result = CrossSectionProfile.Decode(new JsonObject(), 200, 300);
        Assert.True(result.Ok);
        Assert.Equal("rect", result.Profile!.Shape);
        Assert.Equal(60000, result.Profile.Area);

        foreach (var present in new JsonNode?[]
                 {
                     null, JsonValue.Create("bad"), JsonValue.Create(7),
                     new JsonObject(), new JsonObject { ["shape"] = "I" },
                     new JsonObject { ["shape"] = "future" },
                 })
        {
            var element = new JsonObject { ["xsection"] = present };
            var bad = CrossSectionProfile.Decode(element, 200, 300);
            Assert.False(bad.Ok);
            Assert.Null(bad.Profile);
        }
    }

    [Theory]
    [InlineData("""{"shape":"i","d":300,"bf":200,"tw":"10","tf":20}""")]
    [InlineData("""{"shape":"i","d":300,"bf":200,"tw":true,"tf":20}""")]
    [InlineData("""{"shape":"i","d":300,"bf":200,"tw":10}""")]
    [InlineData("""{"shape":"i","d":300,"bf":200,"tw":0,"tf":20}""")]
    [InlineData("""{"shape":"i","d":300,"bf":200,"tw":-1,"tf":20}""")]
    [InlineData("""{"shape":"channel","d":300,"b":200,"tw":10,"tf":20}""")]
    [InlineData("""{"shape":"channel","d":300,"bf":200,"b":200,"tw":10,"tf":20}""")]
    public void MalformedOrAliasDimensionsFailClosed(string xsection)
    {
        var result = CrossSectionProfile.Decode(
            new JsonObject { ["xsection"] = JsonNode.Parse(xsection) }, 200, 300);
        Assert.False(result.Ok);
        Assert.Null(result.Profile);
    }

    [Fact]
    public void NonFiniteProgrammaticJsonNumberFailsClosed()
    {
        var xsection = new JsonObject
        {
            ["shape"] = "rect",
            ["w"] = double.NaN,
            ["d"] = 300,
        };
        var result = CrossSectionProfile.Decode(
            new JsonObject { ["xsection"] = xsection }, 200, 300);
        Assert.False(result.Ok);
    }

    [Theory]
    [InlineData("""{"shape":"i","d":300,"bf":200,"tw":200,"tf":20}""")]
    [InlineData("""{"shape":"i","d":300,"bf":200,"tw":10,"tf":150}""")]
    [InlineData("""{"shape":"channel","d":300,"bf":200,"tw":200,"tf":20}""")]
    [InlineData("""{"shape":"angle","d":300,"b":200,"t":200}""")]
    [InlineData("""{"shape":"rhs","d":300,"b":200,"t":100}""")]
    [InlineData("""{"shape":"chs","od":200,"t":100}""")]
    public void TopologyBoundaryDegeneracyIsRejected(string xsection)
    {
        var d = xsection.Contains("\"chs\"", StringComparison.Ordinal) ? 200 : 300;
        var result = CrossSectionProfile.Decode(
            new JsonObject { ["xsection"] = JsonNode.Parse(xsection) }, 200, d);
        Assert.False(result.Ok);
    }

    [Fact]
    public void EnvelopeToleranceIsScaleAwareAndChsMatchesBothAxes()
    {
        var within = CrossSectionProfile.Decode(
            new JsonObject
            {
                ["xsection"] = JsonNode.Parse(
                    """{"shape":"rect","w":200.0000005,"d":299.9999995}""")
            }, 200, 300);
        Assert.True(within.Ok, within.Error);

        var outside = CrossSectionProfile.Decode(
            new JsonObject
            {
                ["xsection"] = JsonNode.Parse(
                    """{"shape":"rect","w":200.000002,"d":300}""")
            }, 200, 300);
        Assert.False(outside.Ok);

        var nonCircular = CrossSectionProfile.Decode(
            new JsonObject
            {
                ["xsection"] = JsonNode.Parse(
                    """{"shape":"chs","od":200,"t":10}""")
            }, 200, 201);
        Assert.False(nonCircular.Ok);
    }

    [Fact]
    public void ScenePreflightAttachesTheNormalizedPlanToSupportedRows()
    {
        var scene = JsonNode.Parse("""
            {
              "meta":{"sourceId":"s","sceneHash":"h"},
              "elements":[{
                "id":"M","kind":"member","from":[0,0,0],"to":[1000,0,0],
                "section":{"w":200,"d":300},
                "xsection":{"shape":"channel","d":300,"bf":200,"tw":10,"tf":20}
              }]
            }
            """);
        var pf = BakeSceneRules.Validate(scene);
        Assert.True(pf.Ok);
        var profile = pf.Supported[0]!["profile"]!;
        Assert.Equal("channel", profile["shape"]!.GetValue<string>());
        Assert.Equal(200, profile["dimensions"]!["bf"]!.GetValue<double>());
    }
}
