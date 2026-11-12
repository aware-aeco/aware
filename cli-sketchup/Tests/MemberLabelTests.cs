using System.Text.Json.Nodes;
using AwareSketchup;
using AwareSketchup.Verbs;
using Xunit;

namespace AwareSketchup.Tests;

/// <summary>
/// #818: a scene member's own `meta.description` and `meta.properties` reach SketchUp as attribute
/// dictionaries on the member's group, under the IFC writer's exact rules, refused before SketchUp is
/// ever contacted.
/// </summary>
public class MemberLabelTests
{
    static BakeScene.Preflight Validate(JsonObject memberMeta)
    {
        var scene = new JsonObject
        {
            ["meta"] = new JsonObject { ["units"] = "mm", ["sourceId"] = "src-a", ["sceneHash"] = "h1" },
            ["elements"] = new JsonArray
            {
                new JsonObject
                {
                    ["id"] = "C1", ["kind"] = "member", ["from"] = JsonNode.Parse("[0,0,0]"), ["to"] = JsonNode.Parse("[0,0,3600]"),
                    ["section"] = new JsonObject { ["w"] = JsonNode.Parse("254"), ["d"] = JsonNode.Parse("254") },
                },
                new JsonObject
                {
                    ["id"] = AwareMemberLabelCases.MemberId, ["kind"] = "member",
                    ["from"] = JsonNode.Parse("[0,0,3600]"), ["to"] = JsonNode.Parse("[6000,0,3600]"),
                    ["section"] = new JsonObject { ["w"] = JsonNode.Parse("139.7"), ["d"] = JsonNode.Parse("398.8") },
                    ["meta"] = memberMeta,
                },
            },
        };
        return BakeScene.Validate(scene);
    }

    static JsonObject? SupportedLabels(BakeScene.Preflight pf, string id) =>
        pf.Supported.OfType<JsonObject>().Single(r => (string?)r["id"] == id)["labels"] as JsonObject;

    [Fact]
    public void TheSinkAcceptsAndRefusesExactlyTheMembersTheIfcWriterDoes()
    {
        foreach (var c in AwareMemberLabelCases.Load())
        {
            var pf = Validate(c.Meta);
            var refusal = pf.Failed.OfType<JsonObject>()
                .FirstOrDefault(r => (string?)r["code"] == AwareMemberLabels.RefusalCode);
            string? description = null, set = null;
            var properties = new List<KeyValuePair<string, string>>();
            if (refusal is not null)
            {
                Assert.Equal(AwareMemberLabelCases.MemberId, (string?)refusal["id"]);
                Assert.False(pf.Ok);
            }
            else
            {
                Assert.True(pf.Ok, c.Name + ": " + pf.Failed.ToJsonString());
                var plan = SupportedLabels(pf, AwareMemberLabelCases.MemberId);
                if (plan?["description"] is JsonObject d)
                {
                    description = (string?)d["value"];
                    Assert.Equal(BakeSceneOwnership.DictionaryName, (string?)d["dictionary"]);
                    Assert.Equal(BakeScene.DescriptionKey, (string?)d["key"]);
                }
                if (plan?["properties"] is JsonObject p)
                {
                    set = (string?)p["name"];
                    Assert.Equal(set, (string?)p["dictionary"]);
                    foreach (var entry in p["entries"]!.AsArray())
                        properties.Add(new((string)entry![0]!, (string)entry[1]!));
                }
            }
            var mismatch = AwareMemberLabelCases.Mismatch(c, (string?)refusal?["message"], description, set, properties);
            Assert.True(mismatch is null, mismatch);
        }
    }

    [Fact]
    public void AMemberWithoutLabelsSendsNoPlanToSketchup()
    {
        var pf = Validate(new JsonObject { ["profile"] = "W16X26" });
        Assert.True(pf.Ok, pf.Failed.ToJsonString());
        Assert.Null(SupportedLabels(pf, AwareMemberLabelCases.MemberId));
        Assert.Null(SupportedLabels(pf, "C1"));
    }

    [Theory]
    [InlineData("AWARE", "ownership stamps")]
    [InlineData("aware", "ownership stamps")]
    [InlineData("dynamic_attributes", "Dynamic Components")]
    [InlineData("Dynamic_Attributes", "Dynamic Components")]
    public void ASetNamedLikeADictionarySomethingElseOwnsIsReportedNotWritten(string name, string why)
    {
        var pf = Validate(new JsonObject
        {
            ["profile"] = "W16X26",
            ["description"] = "HOLD",
            ["properties"] = new JsonObject { ["name"] = name, ["properties"] = new JsonObject { ["sourceId"] = "x" } },
        });
        Assert.True(pf.Ok, pf.Failed.ToJsonString());
        var plan = SupportedLabels(pf, AwareMemberLabelCases.MemberId)!;
        Assert.Null(plan["properties"]);
        Assert.Equal("HOLD", (string?)plan["description"]!["value"]);
        var miss = Assert.IsType<JsonObject>(Assert.Single(plan["notWritten"]!.AsArray()));
        Assert.Equal("meta.properties", (string?)miss["field"]);
        Assert.Contains(why, (string?)miss["reason"]);
    }

    [Fact]
    public void AMalformedLabelRefusesTheBatchBeforeSketchupIsContacted()
    {
        var pf = Validate(new JsonObject { ["profile"] = "W16X26", ["description"] = "" });
        Assert.False(pf.Ok);
        Assert.Contains(pf.Failed.OfType<JsonObject>(), r => (string?)r["id"] == "C1" && (string?)r["code"] == "batch-aborted");
    }

    // ── the in-SketchUp materializer ─────────────────────────────────────────

    static int At(string code, string needle)
    {
        var at = code.IndexOf(needle, StringComparison.Ordinal);
        Assert.True(at >= 0, "BakeSceneScript.rb no longer contains: " + needle);
        return at;
    }

    [Fact]
    public void TheMaterializerWritesLabelsInsideTheOperationAndClaimsOnlyWhatReadsBack()
    {
        var code = BakeSceneScript.Code.Replace("\r\n", "\n");
        var write = At(code, "plan['labelOutcome'] = write_labels.call(grp, plan['labels'])");
        Assert.True(write > At(code, "grp.set_attribute(dict, k_mrk, marker)"));
        Assert.True(write < At(code, "model.commit_operation"));
        // Every value is read back before it counts, and a refused set leaves no dictionary behind.
        At(code, "grp.get_attribute(d_dict, d_key) == d_value");
        At(code, "grp.get_attribute(p_dict, key) == value");
        At(code, "(grp.delete_attribute(p_dict) rescue nil)");
        At(code, "r['description'] = true if outcome['description']");
        At(code, "r['labelsNotWritten'] = outcome['notWritten']");
        // The not-written warnings reach only a bake that committed.
        Assert.True(At(code, "'warnings' => warnings + label_warnings,") > At(code, "committed = true"));
    }
}
