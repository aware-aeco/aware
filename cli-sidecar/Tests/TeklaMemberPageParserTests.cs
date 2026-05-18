// Tests for the Tekla MemberPageParser. We cache the live HTML once in <repo>/cli-sidecar/Tests/fixtures/
// so the suite is fully offline + deterministic. The fixtures are committed alongside the test so a
// future run reproduces the exact bytes used here.

using AngleSharp.Html.Parser;
using AwareSidecar.Ingest.Extractors.Tekla;
using Xunit;

namespace AwareSidecar.Ingest.Tests;

public class TeklaMemberPageParserTests
{
    static string LoadFixture(string name)
    {
        var here = Path.GetDirectoryName(typeof(TeklaMemberPageParserTests).Assembly.Location)!;
        // Tests/bin/Debug/net9.0/  → up three to Tests/, then /fixtures
        var path = Path.Combine(here, "fixtures", name);
        if (!File.Exists(path))
        {
            // CI / publish layouts may differ. Walk up to Tests/ root.
            var walk = here;
            for (int i = 0; i < 6 && walk is not null; i++)
            {
                var probe = Path.Combine(walk, "Tests", "fixtures", name);
                if (File.Exists(probe)) { path = probe; break; }
                walk = Path.GetDirectoryName(walk);
            }
        }
        return File.ReadAllText(path);
    }


    [Fact]
    public void Method_Equals_Parses_Real_Signature_And_Return()
    {
        // Fixture is /topic/en/18/47/0e1a8744-a12a-74b6-4bcd-6f67e2c54fc2 — Assertion.Equals
        var html = LoadFixture("tekla-method-equals.html");
        var r = MemberPageParser.ParseMethod(html, isCtor: false);
        Assert.NotNull(r);
        // Tekla page: `public override bool Equals(\n)` — but it carries one parameter `obj`
        // in the Parameters dl. The syntax block itself is empty inside the parens for this
        // particular Tekla quirk; we report what the page actually says.
        Assert.Contains("Equals", r!.signature);
        // Equals takes one parameter: obj of type System.Object (rendered "System.Object").
        Assert.Single(r.@params);
        Assert.Equal("obj", r.@params[0].name);
        Assert.Equal("System.Object", r.@params[0].type);
        // Return value is Boolean.
        Assert.NotNull(r.returns);
        Assert.Equal("Boolean", r.returns!.type);
    }

    [Fact]
    public void Method_FetchModelObjects_Parses_Two_Params_With_Generics_And_Optional()
    {
        // Fixture: Model.FetchModelObjects(List<string>, bool) — two params, the second is Optional.
        var html = LoadFixture("tekla-method-fetchmodelobjects.html");
        var r = MemberPageParser.ParseMethod(html, isCtor: false);
        Assert.NotNull(r);
        Assert.Contains("FetchModelObjects", r!.signature);
        Assert.Equal(2, r.@params.Count);
        Assert.Equal("Guids", r.@params[0].name);
        Assert.Contains("List", r.@params[0].type);
        Assert.Contains("String", r.@params[0].type);
        Assert.Equal("SelectInstances", r.@params[1].name);
        Assert.True(r.@params[1].optional);
        // Return type is List<ModelObject>.
        Assert.NotNull(r.returns);
        Assert.Contains("List", r.returns!.type);
        Assert.Contains("ModelObject", r.returns.type);
    }

    [Fact]
    public void Method_WithException_Parses_Throws()
    {
        // Fixture: AssemblyDrawing.Insert — has an Exceptions section.
        var html = LoadFixture("tekla-method-assemblydrawing-insert.html");
        var r = MemberPageParser.ParseMethod(html, isCtor: false);
        Assert.NotNull(r);
        Assert.NotEmpty(r!.throws);
        Assert.Contains(r.throws, ex => ex.type.Contains("CannotInsertDrawingException", StringComparison.Ordinal));
    }

    [Fact]
    public void Property_DetailedMessage_Parses_String_Type_And_ReadOnly()
    {
        var html = LoadFixture("tekla-property-detailedmessage.html");
        var r = MemberPageParser.ParseProperty(html);
        Assert.NotNull(r);
        Assert.Equal("String", r!.type);
        Assert.True(r.getter);
        Assert.False(r.setter);
    }

    [Fact]
    public void Event_AttributesLoadedFromModel_Parses_Delegate_Type()
    {
        var html = LoadFixture("tekla-event-attributesloaded.html");
        var r = MemberPageParser.ParseEvent(html);
        Assert.NotNull(r);
        // Tekla event has delegate "System.EventHandler" — appears in the "Value" subHeading.
        Assert.Contains("EventHandler", r!.@delegate);
    }

    [Fact]
    public void Constructor_Model_Parses_Empty_Param_List()
    {
        var html = LoadFixture("tekla-ctor-model.html");
        var r = MemberPageParser.ParseMethod(html, isCtor: true);
        Assert.NotNull(r);
        Assert.Contains("Model", r!.signature);
        Assert.Empty(r.@params);
    }

    // ── B1 follow-up: coverage for parser-fragile / under-tested member shapes ─────

    [Fact]
    public void Method_VoidReturn_Has_Null_Returns()
    {
        // Fixture: a `public void CommitChanges()` page with NO "Return Value" subHeading.
        // ExtractReturnValue must return null and the enrichment's `returns` must be null.
        // (Real-vendor case: hundreds of Tekla mutation methods return void.)
        var html = LoadFixture("tekla-method-void-return.html");
        var r = MemberPageParser.ParseMethod(html, isCtor: false);
        Assert.NotNull(r);
        Assert.Contains("CommitChanges", r!.signature);
        Assert.Null(r.returns);
        Assert.Empty(r.@params);
    }

    [Fact]
    public void Method_OptionalParam_Renders_Default_And_Marks_Optional()
    {
        // Fixture: `public int Sample(int Count = 10)` — single optional default-valued param.
        // The Parameters dl carries "(Optional)" on the <dt>; ParamInfo.optional must be true.
        // The default value "10" currently lives in the signature string (a known limitation
        // recorded in EXTRACTION-NOTES.md); we assert what the parser actually emits today
        // rather than over-specifying.
        var html = LoadFixture("tekla-method-optional-params.html");
        var r = MemberPageParser.ParseMethod(html, isCtor: false);
        Assert.NotNull(r);
        Assert.Single(r!.@params);
        Assert.Equal("Count", r.@params[0].name);
        Assert.True(r.@params[0].optional);
        // The "= 10" lives in the signature line (Tekla's vendor docs always carry the literal).
        Assert.Contains("= 10", r.signature);
    }

    [Fact]
    public void Method_MultipleExceptions_Captures_Each_With_When_Description()
    {
        // Fixture: two-row Exceptions table (IOException + UnauthorizedAccessException).
        // Each ThrowsInfo must carry a real non-empty `when` description.
        var html = LoadFixture("tekla-method-throws-multi.html");
        var r = MemberPageParser.ParseMethod(html, isCtor: false);
        Assert.NotNull(r);
        Assert.Equal(2, r!.throws.Count);

        var io = r.throws.FirstOrDefault(t => t.type.Contains("IOException", StringComparison.Ordinal));
        Assert.NotNull(io);
        Assert.False(string.IsNullOrEmpty(io!.when));
        Assert.Contains("stream", io.when, StringComparison.OrdinalIgnoreCase);

        var unauth = r.throws.FirstOrDefault(t => t.type.Contains("UnauthorizedAccessException", StringComparison.Ordinal));
        Assert.NotNull(unauth);
        Assert.False(string.IsNullOrEmpty(unauth!.when));
        Assert.Contains("permission", unauth.when, StringComparison.OrdinalIgnoreCase);
    }

    [Fact]
    public void LstSubstitution_Renders_Dot_Across_Three_Contexts()
    {
        // Fixture's .summary div embeds three LST contexts:
        //   1. between two spans:     <span>Tekla<LST/><script>cs=.</script>Structures</span>
        //   2. inside an <a> anchor:  <a>Foo<LST/><script>cs=.</script>Bar</a>
        //   3. inside a typename:     <span class="nolink">Foo<LST/><script>cs=.</script>BarEnum</span>
        // CleanInlineText must produce "Tekla.Structures :: Foo.Bar :: Foo.BarEnum." — never
        // "AddLanguageSpecificTextSet(...)" leak or "TeklaStructures" (missing dot).
        var html = LoadFixture("tekla-lst-substitution-fixture.html");
        using var doc = new HtmlParser().ParseDocument(html);
        var summary = doc.QuerySelector("div.summary");
        Assert.NotNull(summary);
        // MemberPageParser.CleanInlineText is internal — visible here via InternalsVisibleTo.
        var rendered = MemberPageParser.CleanInlineText(summary!);

        // 1. dot between plain spans
        Assert.Contains("Tekla.Structures", rendered);
        // 2. dot inside an <a> link
        Assert.Contains("Foo.Bar", rendered);
        // 3. dot inside a nolink type span
        Assert.Contains("Foo.BarEnum", rendered);

        // And critically — no script body leakage and no empty-substitution failure.
        Assert.DoesNotContain("AddLanguageSpecificTextSet", rendered);
        Assert.DoesNotContain("TeklaStructures", rendered);
    }

    [Fact]
    public void Property_GenericType_Renders_Angle_Brackets()
    {
        // Fixture: `public IList<ModelObject> Selected { get; }` — type rendered via LST
        // angle-bracket substitution (cs=< / cs=>). The Property Value subHeading carries
        // the rendered "IList<ModelObject>" with real ASCII angle brackets after substitution.
        var html = LoadFixture("tekla-property-real-type.html");
        var r = MemberPageParser.ParseProperty(html);
        Assert.NotNull(r);
        Assert.Equal("IList<ModelObject>", r!.type);
        Assert.True(r.getter);
        Assert.False(r.setter);
    }

    [Fact]
    public void Event_FullyQualifiedDelegate_Is_Captured()
    {
        // Fixture: event with delegate `Tekla.Structures.Model.Events.ModelChangedDelegate`
        // (four LST-dotted segments). The "Value" subHeading section carries the dotted name.
        // The parser must capture the full FQN — a bare "EventHandler" would mean the
        // placeholder from PageParser was never overwritten.
        var html = LoadFixture("tekla-event-real-delegate.html");
        var r = MemberPageParser.ParseEvent(html);
        Assert.NotNull(r);
        Assert.Equal("Tekla.Structures.Model.Events.ModelChangedDelegate", r!.@delegate);
        Assert.NotEqual("EventHandler", r.@delegate);
    }
}
