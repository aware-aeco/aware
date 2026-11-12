using System.Text.Json.Nodes;
using AwareRhino;
using Xunit;

namespace AwareRhino.Tests;

/// <summary>
/// #818: a scene member's own `meta.description` and `meta.properties` reach Rhino as object user
/// text, under the IFC writer's exact rules, refused before Rhino is ever contacted.
/// </summary>
public class MemberLabelTests
{
    static BakeSceneRules.Preflight Validate(JsonObject memberMeta)
    {
        var scene = new JsonObject
        {
            ["meta"] = new JsonObject { ["units"] = "mm", ["sourceId"] = "drop-a", ["sceneHash"] = "scene-a" },
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
        return BakeSceneRules.Validate(scene);
    }

    static JsonObject? SupportedLabels(BakeSceneRules.Preflight pf, string id) =>
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
                description = (string?)plan?["description"]?["value"];
                set = (string?)plan?["properties"]?["name"];
                if (plan?["properties"]?["entries"] is JsonArray entries)
                {
                    foreach (var entry in entries)
                    {
                        // The user-text key is `<set>.<key>`.
                        var key = (string)entry![0]!;
                        Assert.StartsWith(set + ".", key);
                        properties.Add(new(key[(set!.Length + 1)..], (string)entry[1]!));
                    }
                }
                if (description is not null) Assert.Equal("Description", (string?)plan!["description"]!["key"]);
            }
            var mismatch = AwareMemberLabelCases.Mismatch(c, (string?)refusal?["message"], description, set, properties);
            Assert.True(mismatch is null, mismatch);
        }
    }

    [Fact]
    public void AMemberWithoutLabelsSendsNoPlanToRhino()
    {
        var pf = Validate(new JsonObject { ["profile"] = "W16X26" });
        Assert.True(pf.Ok, pf.Failed.ToJsonString());
        Assert.Null(SupportedLabels(pf, AwareMemberLabelCases.MemberId));
        Assert.Null(SupportedLabels(pf, "C1"));
    }

    [Theory]
    [InlineData("AWARE")]
    [InlineData("aware")]
    public void ASetNamedLikeTheBakesOwnPrefixIsReportedNotWritten(string name)
    {
        // `AWARE.PROFILE` and `AWARE.BAKE.*` are this sink's own stamps; a set called AWARE would write
        // `AWARE.<key>` keys beside them, and `AWARE.PROFILE` among them.
        var pf = Validate(new JsonObject
        {
            ["profile"] = "W16X26",
            ["description"] = "HOLD",
            ["properties"] = new JsonObject { ["name"] = name, ["properties"] = new JsonObject { ["PROFILE"] = "x" } },
        });
        Assert.True(pf.Ok, pf.Failed.ToJsonString());
        var plan = SupportedLabels(pf, AwareMemberLabelCases.MemberId)!;
        Assert.Null(plan["properties"]);
        Assert.Equal("HOLD", (string?)plan["description"]!["value"]);
        var miss = Assert.IsType<JsonObject>(Assert.Single(plan["notWritten"]!.AsArray()));
        Assert.Equal("meta.properties", (string?)miss["field"]);
        Assert.Contains("user-text prefix bake-scene keeps its own stamps under", (string?)miss["reason"]);
    }

    [Fact]
    public void AMalformedLabelRefusesTheBatchBeforeRhinoIsContacted()
    {
        var pf = Validate(new JsonObject { ["profile"] = "W16X26", ["description"] = "" });
        Assert.False(pf.Ok);
        Assert.Contains(pf.Failed.OfType<JsonObject>(), r => (string?)r["id"] == "C1" && (string?)r["code"] == "batch-aborted");
        var envelope = BakeSceneRules.Envelope(pf);
        Assert.Empty(envelope["emitted"]!.AsArray());
    }

    // ── the in-Rhino materializer ────────────────────────────────────────────

    static int At(string code, string needle)
    {
        var at = code.IndexOf(needle, StringComparison.Ordinal);
        Assert.True(at >= 0, "BakeSceneScript.py no longer contains: " + needle);
        return at;
    }

    [Fact]
    public void TheMaterializerWritesUserTextBeforeAddingAndClaimsOnlyWhatReadsBack()
    {
        var code = BakeSceneScript.Code;
        Assert.True(At(code, "label_outcome = stage_labels(attrs, plan.get(\"labels\"))")
                    < At(code, "object_id = doc.Objects.AddBrep(plan[\"brep\"], attrs)"));
        Assert.True(At(code, "for miss in report_labels(emitted_row, created, label_outcome):")
                    > At(code, "created = doc.Objects.FindId(object_id)"));
        // A label that reads back differently is taken off the object again, and proved gone.
        At(code, "cleaned.DeleteUserString(key)");
        At(code, "if not doc.Objects.ModifyAttributes(created, cleaned, True):");
        At(code, "any(after.Attributes.GetUserString(key) is not None for key in remove)");
        At(code, "if created_attrs.GetUserString(key) == value:\n            emitted_row[\"description\"] = True");
        At(code, "emitted_row[\"propertySet\"] = name");
        At(code, "emitted_row[\"labelsNotWritten\"] = misses");
        // A set Rhino partly refuses leaves none of its keys on the attributes.
        At(code, "for key in staged_keys + [refused]:\n                attrs.DeleteUserString(key)");
        // The not-written warnings reach only a bake that succeeded.
        Assert.True(At(code, "warnings.extend(label_warnings)") > At(code, "doc.EndUndoRecord(undo_serial)"));
    }
}
