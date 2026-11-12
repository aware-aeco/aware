// #818: a scene member's own `meta.description` reaches Revit as the element's instance Comments
// parameter, under the IFC writer's exact rules; its property set has no place in Revit and the
// receipt says so rather than dropping it.

using System.Text.Json;
using System.Text.Json.Nodes;
using AwareRevit.Sidecar;
using Xunit;

namespace AwareRevit.Tests;

public class MemberLabelTests
{
    static AwareBakeRules.ParsedScene Parse(JsonObject memberMeta)
    {
        var scene = new JsonObject
        {
            ["meta"] = new JsonObject { ["units"] = "mm", ["sourceId"] = "src-a", ["sceneHash"] = "hash-a" },
            ["elements"] = new JsonArray
            {
                new JsonObject
                {
                    ["id"] = "C1", ["kind"] = "member", ["from"] = new JsonArray(0, 0, 0), ["to"] = new JsonArray(0, 0, 3600),
                    ["meta"] = new JsonObject { ["profile"] = "W10X49" },
                },
                new JsonObject
                {
                    ["id"] = AwareMemberLabelCases.MemberId, ["kind"] = "member",
                    ["from"] = new JsonArray(0, 0, 3600), ["to"] = new JsonArray(6000, 0, 3600),
                    ["meta"] = memberMeta,
                },
            },
        };
        return AwareBakeRules.ParseScene(JsonDocument.Parse(scene.ToJsonString()).RootElement.Clone());
    }

    [Fact]
    public void TheSinkAcceptsAndRefusesExactlyTheMembersTheIfcWriterDoes()
    {
        foreach (var c in AwareMemberLabelCases.Load())
        {
            var parsed = Parse(c.Meta);
            var refusal = parsed.Failed.FirstOrDefault(r => (string)r["code"] == AwareMemberLabels.RefusalCode);
            var member = parsed.Members.FirstOrDefault(m => m.Id == AwareMemberLabelCases.MemberId);
            if (refusal is not null)
            {
                Assert.Equal(AwareMemberLabelCases.MemberId, refusal["id"]);
                Assert.Null(member);
                // The refused member fails the batch before the transaction: the other one is aborted.
                Assert.False(parsed.Ok);
            }
            else
            {
                Assert.True(parsed.Ok, c.Name + ": " + string.Join("; ", parsed.Failed.Select(r => r["message"])));
                Assert.NotNull(member);
                // meta.profile stays exactly the catalogue key, labels or not.
                Assert.Equal(AwareMemberLabelCases.Profile, member!.Profile);
            }
            var mismatch = AwareMemberLabelCases.Mismatch(c, (string?)refusal?["message"],
                member?.Labels.Description, member?.Labels.PropertySetName, member?.Labels.Properties);
            Assert.True(mismatch is null, mismatch);
        }
    }

    sealed class FakeComments
    {
        public string? Value;
        public int MaxLength = int.MaxValue;
        public bool KeepsEverything;
        public bool Set(string value)
        {
            if (KeepsEverything && Value is not null && value.Length == 0) return true;
            Value = value.Length > MaxLength ? value[..MaxLength] : value;
            return true;
        }
    }

    static AwareMemberLabelSet Labels(string metaJson)
    {
        var parsed = Parse(JsonNode.Parse(metaJson)!.AsObject());
        Assert.True(parsed.Ok);
        return parsed.Members.Single(m => m.Id == AwareMemberLabelCases.MemberId).Labels;
    }

    [Fact]
    public void TheDescriptionIsWrittenIntoCommentsAndThePropertySetIsReportedNotWritten()
    {
        var comments = new FakeComments();
        var outcome = AwareMemberLabels.Write(
            Labels("{\"profile\":\"W16X26\",\"description\":\"HOLD: size pending from others\","
                + "\"properties\":{\"name\":\"FloLess_Member\",\"properties\":{\"Status\":\"HOLD\"}}}"),
            AwareBakeRules.RevitLabelCarrier(true, comments.Set, () => comments.Value!));
        var row = AwareBakeRules.Row("B1", "member", "emitted", "", "");
        AwareMemberLabels.Report(row, outcome);

        Assert.Equal("HOLD: size pending from others", comments.Value);
        Assert.Equal(true, row["description"]);
        Assert.False(row.ContainsKey("propertySet"));
        var miss = Assert.IsType<Dictionary<string, object>>(Assert.Single((List<object>)row["labelsNotWritten"]));
        Assert.Equal("meta.properties", miss["field"]);
        Assert.Equal(AwareBakeRules.PropertySetUnavailable, miss["reason"]);
    }

    [Fact]
    public void AnElementWithoutAWritableCommentsParameterIsNeverClaimed()
    {
        var comments = new FakeComments();
        var outcome = AwareMemberLabels.Write(Labels("{\"profile\":\"W16X26\",\"description\":\"HOLD\"}"),
            AwareBakeRules.RevitLabelCarrier(false, comments.Set, () => comments.Value!));
        var row = AwareBakeRules.Row("B1", "member", "emitted", "", "");
        AwareMemberLabels.Report(row, outcome);

        Assert.Null(comments.Value);
        Assert.False(row.ContainsKey("description"));
        var miss = (Dictionary<string, object>)((List<object>)row["labelsNotWritten"]).Single();
        Assert.Equal("meta.description", miss["field"]);
        Assert.Equal("this element has no writable instance Comments parameter", miss["reason"]);
        var warning = AwareMemberLabels.NotWrittenWarning("B1", "member", "meta.description", (string)miss["reason"]);
        Assert.Equal("member-label-not-written", warning["code"]);
        Assert.Equal("`meta.description` was sent but not written: this element has no writable instance Comments parameter",
            warning["message"]);
    }

    [Fact]
    public void ACommentsValueThatDoesNotReadBackWholeIsRemovedAndNotClaimed()
    {
        var comments = new FakeComments { MaxLength = 4 };
        var outcome = AwareMemberLabels.Write(Labels("{\"profile\":\"W16X26\",\"description\":\"HOLD pending\"}"),
            AwareBakeRules.RevitLabelCarrier(true, comments.Set, () => comments.Value!));
        Assert.False(outcome.DescriptionWritten);
        Assert.Equal("", comments.Value);
        Assert.Contains("read back as 4 of its 12 bytes, so it was removed again", outcome.NotWritten.Single().Value);
    }

    [Fact]
    public void AValueThatCannotBeRemovedAgainIsNeverDescribedAsRemoved()
    {
        // The host shortens the value and then ignores the clear: the receipt must not say it is gone.
        var comments = new FakeComments { MaxLength = 4, KeepsEverything = true };
        var outcome = AwareMemberLabels.Write(Labels("{\"profile\":\"W16X26\",\"description\":\"HOLD pending\"}"),
            AwareBakeRules.RevitLabelCarrier(true, comments.Set, () => comments.Value!));
        Assert.False(outcome.DescriptionWritten);
        Assert.Equal("HOLD", comments.Value);
        var reason = outcome.NotWritten.Single().Value;
        Assert.Contains("could not be removed again, so the model may hold part of it", reason);
        Assert.DoesNotContain("so it was removed again", reason);
    }

    [Fact]
    public void AValueTheHostAltersWithoutShorteningIsDescribedAsDifferent()
    {
        var comments = new FakeComments();
        var outcome = AwareMemberLabels.Write(Labels("{\"profile\":\"W16X26\",\"description\":\"HOLD\"}"),
            AwareBakeRules.RevitLabelCarrier(true, v => comments.Set(v.Length == 0 ? v : "H?LD"), () => comments.Value!));
        Assert.False(outcome.DescriptionWritten);
        Assert.Equal("", comments.Value);
        Assert.Contains("read back differently from what was sent, so it was removed again", outcome.NotWritten.Single().Value);
    }

    [Fact]
    public void AMemberWithoutLabelsGainsNoReceiptField()
    {
        var labels = Labels("{\"profile\":\"W16X26\"}");
        Assert.True(labels.IsEmpty);
        var row = AwareBakeRules.Row("B1", "member", "emitted", "", "");
        AwareMemberLabels.Report(row, AwareMemberLabels.Write(labels, AwareBakeRules.RevitLabelCarrier(true, _ => throw new InvalidOperationException("written"), () => "")));
        Assert.Equal(new[] { "id", "kind", "status" }, row.Keys.ToArray());
    }

    // ── the in-Revit script ──────────────────────────────────────────────────

    [Fact]
    public void TheScriptCarriesTheSharedLabelRulesAndWritesCommentsBeforeCommitting()
    {
        var rules = BakeSceneScript.LoadRulesSource();
        Assert.Contains("internal static class AwareBakeRules", rules);
        Assert.Contains("internal static class AwareMemberLabels", rules);

        var body = BakeSceneScript.Body;
        int At(string needle)
        {
            var at = body.IndexOf(needle, StringComparison.Ordinal);
            Assert.True(at >= 0, "BakeSceneScript.Body no longer contains: " + needle);
            return at;
        }
        var write = At("created.get_Parameter(BuiltInParameter.ALL_MODEL_INSTANCE_COMMENTS)");
        Assert.True(write > At("created.SetEntity(stamp);"));
        Assert.True(write < At("var commitStatus = tx.Commit();"));
        Assert.True(At("AwareMemberLabels.Report(row, labelOutcome);") < At("emitted.Add(row);"));
        // The not-written warnings are published only once the transaction committed.
        Assert.True(At("warnings.AddRange(labelWarnings);") > At("if (commitStatus != TransactionStatus.Committed)"));
    }
}
