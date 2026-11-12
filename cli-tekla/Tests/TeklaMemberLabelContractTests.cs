using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.Json.Nodes;
using Xunit;

namespace AwareTekla.Tests;

/// <summary>
/// #818: a scene member's own `meta.description` and `meta.properties` reach Tekla as the part's
/// Comment and as UDAs, under the IFC writer's exact rules, and the receipt claims only what landed.
/// </summary>
public sealed class TeklaMemberLabelContractTests
{
    /// <summary>A Tekla part's UDAs as the bake sees them through SetUserProperty/GetUserProperty.</summary>
    sealed class FakeUdas
    {
        public readonly Dictionary<string, string> Values = new(StringComparer.Ordinal);
        public int MaxLength = int.MaxValue;
        public readonly HashSet<string> Refused = new(StringComparer.Ordinal);

        public bool Set(string name, string value)
        {
            if (Refused.Contains(name)) return false;
            Values[name] = value.Length > MaxLength ? value.Substring(0, MaxLength) : value;
            return true;
        }

        public string? Get(string name) => Values.TryGetValue(name, out var value) ? value : null;
    }

    static TeklaMemberLabels Labels(string metaJson)
    {
        var meta = Program.JsonNodeToObject(JsonNode.Parse(metaJson));
        var error = TeklaMemberLabelContract.Validate("B1", meta, out var labels);
        Assert.Null(error);
        return labels;
    }

    static Dictionary<string, object> Bake(TeklaMemberLabels labels, FakeUdas udas, out List<Dictionary<string, object>> warnings)
    {
        var report = TeklaMemberLabelContract.Write(labels, udas.Set, udas.Get);
        var row = new Dictionary<string, object>(StringComparer.Ordinal) { ["id"] = "B1", ["kind"] = "member", ["status"] = "emitted" };
        report.ApplyTo(row);
        warnings = report.Warnings("B1", "member");
        return row;
    }

    const string Hold = "HOLD: size pending from others; approx. wt. 26 lb/ft";

    [Fact]
    public void TheSinkAcceptsAndRefusesExactlyTheMembersTheIfcWriterDoes()
    {
        // The script receives `meta` through Program.JsonNodeToObject, so the cases go through it too.
        foreach (var c in AwareMemberLabelCases.Load())
        {
            var meta = Program.JsonNodeToObject(c.Meta);
            var error = TeklaMemberLabelContract.Validate(AwareMemberLabelCases.MemberId, meta, out var labels);
            var mismatch = AwareMemberLabelCases.Mismatch(c, error, labels.Set.Description, labels.Set.PropertySetName, labels.Set.Properties);
            Assert.True(mismatch == null, mismatch);
        }
    }

    [Fact]
    public void AMemberWithoutMetaOrWithAProfileOnlyCarriesNoLabels()
    {
        Assert.Null(TeklaMemberLabelContract.Validate("B1", null, out var none));
        Assert.True(none.IsEmpty);
        Assert.True(Labels("{\"profile\":\"W16X26\"}").IsEmpty);
        var udas = new FakeUdas();
        var row = Bake(Labels("{\"profile\":\"W16X26\"}"), udas, out var warnings);
        Assert.Empty(udas.Values);
        Assert.Empty(warnings);
        Assert.Equal(new[] { "id", "kind", "status" }, row.Keys.ToArray());
    }

    [Fact]
    public void TheDescriptionGoesIntoThePartsCommentAndThePropertiesIntoUdasNamedByTheirKeys()
    {
        var udas = new FakeUdas();
        var row = Bake(Labels("{\"profile\":\"W16X26\",\"description\":\"" + Hold + "\","
            + "\"properties\":{\"name\":\"FloLess_Member\",\"properties\":{\"Status\":\"HOLD\",\"Source\":\"S-201\"}}}"), udas, out var warnings);

        Assert.Equal(Hold, udas.Values["comment"]);
        Assert.Equal("HOLD", udas.Values["Status"]);
        Assert.Equal("S-201", udas.Values["Source"]);
        Assert.Equal(3, udas.Values.Count);
        Assert.Equal(true, row["description"]);
        Assert.Equal("FloLess_Member", row["propertySet"]);
        Assert.False(row.ContainsKey("labelsNotWritten"));
        Assert.Empty(warnings);
    }

    [Fact]
    public void AValueTeklaShortensIsReportedNotWrittenAndRemovedAgain()
    {
        // Tekla string UDAs have historically been capped near 80 characters. Whatever the cap on a
        // given environment, a description that does not read back whole is never claimed.
        var udas = new FakeUdas { MaxLength = 20 };
        var row = Bake(Labels("{\"description\":\"" + Hold + "\"}"), udas, out var warnings);

        Assert.False(row.ContainsKey("description"));
        Assert.Equal("", udas.Values["comment"]);
        var miss = Assert.IsType<Dictionary<string, object>>(Assert.Single((List<object>)row["labelsNotWritten"]));
        Assert.Equal("meta.description", miss["field"]);
        Assert.Contains("read back as 20 of its 52 bytes", (string)miss["reason"]);
        var warning = Assert.Single(warnings);
        Assert.Equal("member-label-not-written", warning["code"]);
        Assert.Equal("warning", warning["status"]);
        Assert.Equal("B1", warning["id"]);
        Assert.StartsWith("`meta.description` was sent but not written: ", (string)warning["message"]);
    }

    [Fact]
    public void APropertySetIsAllOrNothing()
    {
        var udas = new FakeUdas();
        udas.Refused.Add("Status");
        var row = Bake(Labels("{\"properties\":{\"name\":\"FloLess_Member\",\"properties\":{\"Status\":\"HOLD\",\"Approx\":\"26\",\"Source\":\"S-201\"}}}"),
            udas, out var warnings);

        // Approx and Source (key order) landed before Status was refused, and were cleared again.
        Assert.Equal("", udas.Values["Approx"]);
        Assert.Equal("", udas.Values["Source"]);
        Assert.False(udas.Values.ContainsKey("Status"));
        Assert.False(row.ContainsKey("propertySet"));
        var miss = (Dictionary<string, object>)((List<object>)row["labelsNotWritten"]).Single();
        Assert.Equal("meta.properties", miss["field"]);
        Assert.Contains("property `Status` was not written", (string)miss["reason"]);
        Assert.Contains("the 2 that were written were removed again", (string)miss["reason"]);
        Assert.Single(warnings);
    }

    [Theory]
    [InlineData("USER_FIELD_1")]
    [InlineData("user_field_4")]
    [InlineData("comment")]
    [InlineData("AWARE_REQUESTED_PROFILE")]
    public void APropertyNeverOverwritesAUdaTheBakeOwns(string key)
    {
        var udas = new FakeUdas();
        udas.Values["USER_FIELD_1"] = "AWARE_SRC_V1:owner";
        udas.Values["user_field_4"] = "AWARE_BAKE_V1:marker";
        udas.Values["AWARE_REQUESTED_PROFILE"] = "W16X26";
        var before = new Dictionary<string, string>(udas.Values);
        var row = Bake(Labels("{\"properties\":{\"name\":\"P\",\"properties\":{\"" + key + "\":\"x\"}}}"), udas, out _);

        Assert.Equal(before, udas.Values);
        Assert.False(row.ContainsKey("propertySet"));
        var reason = (string)((Dictionary<string, object>)((List<object>)row["labelsNotWritten"]).Single())["reason"];
        Assert.Contains("property `" + key + "` was not written: it is ", reason);
    }

    [Fact]
    public void ADescriptionAndASetAreReportedApart()
    {
        // One can land while the other does not; each is reported for what it is.
        var udas = new FakeUdas();
        udas.Refused.Add("comment");
        var row = Bake(Labels("{\"description\":\"HOLD\",\"properties\":{\"name\":\"S\",\"properties\":{\"Status\":\"HOLD\"}}}"), udas, out var warnings);
        Assert.False(row.ContainsKey("description"));
        Assert.Equal("S", row["propertySet"]);
        Assert.Equal("meta.description", ((Dictionary<string, object>)((List<object>)row["labelsNotWritten"]).Single())["field"]);
        Assert.Single(warnings);
    }

    // ── the Roslyn materializer calls the contract at the right moments ─────

    static string Script => BakeSceneScript.Code;

    static int At(string needle)
    {
        var at = Script.IndexOf(needle, StringComparison.Ordinal);
        Assert.True(at >= 0, "BakeSceneScript.Code no longer contains: " + needle);
        return at;
    }

    [Fact]
    public void BakeScript_RefusesAMalformedLabelBeforeTheFirstInsert()
    {
        var validate = At("AwareTekla.TeklaMemberLabelContract.Validate(pair.Key,labelMeta,out labels)");
        Assert.True(validate < At("if(failed.Count>0){appendBatchAborted(\"Batch was aborted because another supported record failed preflight.\")"));
        Assert.True(validate < At("labelBeforeInsert(b);"));
        At("failed.Add(row(pair.Key,labelKind,\"failed\",AwareTekla.TeklaMemberLabelContract.RefusalCode,labelError))");
        // Only the kinds the IFC writer labels: member, line and box.
        At("if(labelKind!=\"member\"&&labelKind!=\"line\"&&labelKind!=\"box\")continue;");
    }

    [Fact]
    public void BakeScript_WritesLabelsOnEveryNativePartBeforeCommittingAndReportsThemAfter()
    {
        var write = At("AwareTekla.TeklaMemberLabelContract.Write(pair.Value,");
        Assert.True(write > At("// Physical parts. Children realized by a BoltArray are intentionally not duplicated."));
        Assert.True(write < At("m.CommitChanges(\"AWARE bake-scene \""));
        // Both legs of a double angle carry the labels; every other member is its one part.
        At("if(doubleAngleLegGuids.TryGetValue(id,out labelLegGuids))labelled.AddRange(");
        // Read back from a fresh select, and only a value every part agrees on counts.
        At("var o=m.SelectModelObject(new Identifier(g));string stored=\"\";if(o==null||!o.GetUserProperty(uda,ref stored))return null;");
        var report = At("labelReport.ApplyTo(r);warnings.AddRange(labelReport.Warnings(id,kind));");
        Assert.True(report > At("m.CommitChanges(\"AWARE bake-scene \""));
    }
}
