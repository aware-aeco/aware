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

    // ── B1 follow-up — defect 1+2 (codex-coverage FAIL 2026-05-19): per-cs= LST tokens ──

    [Fact]
    public void LstSubstitution_DefaultCtor_Parens_Are_Preserved()
    {
        // Defect 1 — codex-coverage FAIL 2026-05-19 on tekla-2025/2026: vendor renders
        // default constructors as e.g. "TransformationPlane()" with the empty-parens being
        // an LST script with `cs=(|vb=(|...` followed by `)|vb=)|...`. The old parser
        // hardcoded `.` for every LST span and collapsed both parens to dots, leaving
        // "TransformationPlane.." in the catalog — colliding with the non-default ctor
        // overload "TransformationPlane(CoordinateSystem)" and breaking overload disambiguation.
        // The corrected parser reads `cs=(` / `cs=)` from the script body and substitutes
        // the literal parens.
        var html = """
<!DOCTYPE html><html><body><div>
TransformationPlane<span id="LSTAAA_0"></span><script type="text/javascript">AddLanguageSpecificTextSet("LSTAAA_0?cs=(|vb=(|cpp=(|fs=(|nu=(");</script><span id="LSTAAA_1"></span><script type="text/javascript">AddLanguageSpecificTextSet("LSTAAA_1?cs=)|vb=)|cpp=)|fs=)|nu=)");</script>
</div></body></html>
""";
        using var doc = new HtmlParser().ParseDocument(html);
        var div = doc.QuerySelector("div")!;
        var rendered = MemberPageParser.CleanInlineText(div);
        Assert.Contains("TransformationPlane()", rendered);
        Assert.DoesNotContain("TransformationPlane..", rendered);
        Assert.DoesNotContain("AddLanguageSpecificTextSet", rendered);
    }

    [Fact]
    public void LstSubstitution_Generic_Open_Bracket_Is_Decoded()
    {
        // Defect 2 — codex-coverage FAIL 2026-05-19: vendor renders generic angle brackets
        // as LST scripts carrying `cs=&lt;` / `cs=&gt;` (HTML-entity-encoded, since the
        // script body is HTML5 raw-text). The parser must HTML-entity-decode the captured
        // value before substituting — without decoding, the literal text `&lt;` would
        // round-trip into the catalog name (e.g. "IEnumerable&lt;Angle&gt;"). The old
        // parser hardcoded `.` instead, producing "IEnumerable.Angle.".
        var html = """
<!DOCTYPE html><html><body><a href="/x">ToString(IEnumerable<span id="LSTBBB_3"></span><script type="text/javascript">AddLanguageSpecificTextSet("LSTBBB_3?cs=&lt;|vb=(Of |cpp=&lt;|fs=&lt;'|nu=(");</script>Angle<span id="LSTBBB_4"></span><script type="text/javascript">AddLanguageSpecificTextSet("LSTBBB_4?cs=&gt;|vb=)|cpp=&gt;|fs=&gt;|nu=)");</script>)</a></body></html>
""";
        using var doc = new HtmlParser().ParseDocument(html);
        var anchor = doc.QuerySelector("a")!;
        var rendered = MemberPageParser.CleanInlineText(anchor);
        Assert.Equal("ToString(IEnumerable<Angle>)", rendered);
        Assert.DoesNotContain("IEnumerable.Angle.", rendered);
        Assert.DoesNotContain("&lt;", rendered);
        Assert.DoesNotContain("&gt;", rendered);
    }

    [Fact]
    public void LstSubstitution_AmpEntity_Is_Decoded()
    {
        // Belt-and-braces: `cs=&amp;` would only ever appear in unusual signatures (ref-style
        // params via & or similar) but the decoder must still handle it. This pins the
        // entity-decoder behaviour so a future regression on the entity table is caught.
        var html = """
<!DOCTYPE html><html><body><div>Foo<span id="LSTCCC_0"></span><script type="text/javascript">AddLanguageSpecificTextSet("LSTCCC_0?cs=&amp;|vb=&amp;|cpp=&amp;|nu=&amp;|fs=&amp;");</script>Bar</div></body></html>
""";
        using var doc = new HtmlParser().ParseDocument(html);
        var div = doc.QuerySelector("div")!;
        var rendered = MemberPageParser.CleanInlineText(div);
        Assert.Equal("Foo&Bar", rendered);
        Assert.DoesNotContain("&amp;", rendered);
    }

    [Fact]
    public void LstSubstitution_CppOnlyMarker_Renders_Empty_Not_Dot()
    {
        // Codex-coverage review v2 (2026-05-19) flagged: Tekla emits LSTs like
        // `LSTF1E4234C_12?cpp=%` for `out`-param indirection markers — C++ only, no
        // `cs=` key at all. The old behaviour fell back to "." and produced trailing
        // dots like `Validation.` instead of the correct empty rendering. Fix: when
        // the regex finds no `cs=` token, return empty string. Pins MemberPageParser
        // line 506 against regression. 0.015% of methods in 2025/2026 hit this case.
        var html = """
<!DOCTYPE html><html><body><div>Validation<span id="LSTDDD_0"></span><script type="text/javascript">AddLanguageSpecificTextSet("LSTDDD_0?cpp=%");</script></div></body></html>
""";
        using var doc = new HtmlParser().ParseDocument(html);
        var div = doc.QuerySelector("div")!;
        var rendered = MemberPageParser.CleanInlineText(div);
        Assert.Equal("Validation", rendered);
        Assert.DoesNotContain(".", rendered);
    }

    // ── B1 follow-up — defect 1: ctor names preserve overload disambiguator ────

    [Fact]
    public void PageParser_TypePage_Ctor_Name_Carries_Overload_Args()
    {
        // Defect 1 — codex-coverage FAIL 2026-05-19: PageParser.ExtractMethodTable
        // previously set `name = typeName` for ctors, dropping the overload disambiguator
        // (`Foo()` vs `Foo(String)` both stored as "Foo"). The fix keeps the row's
        // anchor-text verbatim as the name, including the parentheses + arg list.
        //
        // Minimal fixture: a type page with TWO ctor rows — default + one-arg. Both
        // rows render with LST `cs=(` / `cs=)` for the parens. The catalog name must
        // contain the disambiguator after parsing.
        var html = """
<!DOCTYPE html><html><body>
<div class="topicContent" id="TopicContent">
<table class="titleTable"><tr><td></td><td><h1>TransformationPlane Class</h1></td></tr></table>
<div class="summary">A plane.</div>
<strong>Namespace:</strong> <a href="/topic/x">Tekla.Structures.Model</a><br>
<div class="collapsibleAreaRegion"><span class="collapsibleRegionTitle">Constructors</span></div>
<div class="collapsibleSection">
<table class="members" id="constructorList">
<tr><th>Name</th><th>Description</th><th>Notes</th></tr>
<tr><td></td>
<td><a href="/topic/ctor-default">TransformationPlane<span id="LSTAAA_0"></span><script type="text/javascript">AddLanguageSpecificTextSet("LSTAAA_0?cs=(|vb=(|cpp=(|fs=(|nu=(");</script><span id="LSTAAA_1"></span><script type="text/javascript">AddLanguageSpecificTextSet("LSTAAA_1?cs=)|vb=)|cpp=)|fs=)|nu=)");</script></a></td>
<td><div class="summary">Default.</div></td>
</tr>
<tr><td></td>
<td><a href="/topic/ctor-args">TransformationPlane<span id="LSTAAA_2"></span><script type="text/javascript">AddLanguageSpecificTextSet("LSTAAA_2?cs=(|vb=(|cpp=(|fs=(|nu=(");</script>CoordinateSystem<span id="LSTAAA_3"></span><script type="text/javascript">AddLanguageSpecificTextSet("LSTAAA_3?cs=)|vb=)|cpp=)|fs=)|nu=)");</script></a></td>
<td><div class="summary">From coordinate system.</div></td>
</tr>
</table>
</div>
</div>
</body></html>
""";
        var result = PageParser.Parse(html, "https://example.com/type");
        Assert.NotNull(result);
        Assert.Equal(2, result!.Type.constructors.Count);
        // Both ctor names must include the disambiguator — NOT "TransformationPlane" twice.
        Assert.Contains(result.Type.constructors, c => c.name == "TransformationPlane()");
        Assert.Contains(result.Type.constructors, c => c.name == "TransformationPlane(CoordinateSystem)");
        // And they must be distinct (the collision was the bug).
        Assert.NotEqual(result.Type.constructors[0].name, result.Type.constructors[1].name);
    }

    [Fact]
    public void Property_Falls_Back_From_PropertyValue_To_ReturnValue_Subheading()
    {
        // Tekla docs are inconsistent — every property documented as overriding a
        // base property (e.g. `ModelObject.IsUpToDate`, `ModelObject.ModificationTime`)
        // uses `<h4>Return Value</h4>` instead of the canonical `<h4>Property Value</h4>`.
        // The parser must try Property Value first (the common case), then fall back to
        // Return Value when absent — otherwise the type lands as the placeholder `object`
        // in the catalog. Discovered while regenerating the 2025 catalog after the
        // codex-coverage FAIL fix — `BaseRebarModifier.IsUpToDate` (Boolean) was
        // flagged as a placeholder.
        var html = """
<!DOCTYPE html><html><body>
<div class="topicContent" id="TopicContent">
<table class="titleTable"><tr><td></td><td><h1>Foo.IsUpToDate Property</h1></td></tr></table>
<div class="summary">A flag.</div>
<strong>Namespace:</strong> <a href="/topic/x">Tekla.Test</a><br>
<div class="collapsibleAreaRegion"><span class="collapsibleRegionTitle">Syntax</span></div>
<div id="ID1RBSection" class="collapsibleSection">
<div class="codeSnippetContainer">
<div class="codeSnippetContainerCodeContainer">
<div id="ID0EBCA_code_Div1" class="codeSnippetContainerCode" style="display: block"><pre>public bool IsUpToDate { get; }</pre></div>
</div>
</div>
<h4 class="subHeading">Return Value</h4>Type: <span class="nolink">Boolean</span><br>True when the object is up to date.
</div>
</div>
</body></html>
""";
        var r = MemberPageParser.ParseProperty(html);
        Assert.NotNull(r);
        Assert.Equal("Boolean", r!.type);
        Assert.NotEqual("object", r.type);
        Assert.True(r.getter);
        Assert.False(r.setter);
    }

    [Fact]
    public void PageParser_TypePage_Method_Generic_Renders_Angle_Brackets()
    {
        // Defect 2 — codex-coverage FAIL 2026-05-19: type-page method rows with generics
        // (e.g. `ToString(IEnumerable<Angle>)`) lost their angle brackets during the type-
        // page crawl that built the snapshot. Verify via a freshly-parsed type-page that the
        // row name now carries `<Angle>` literally — the fix only takes effect on a fresh
        // crawl (snapshot must be deleted), but the parser-level check here pins the
        // logic going forward.
        var html = """
<!DOCTYPE html><html><body>
<div class="topicContent" id="TopicContent">
<table class="titleTable"><tr><td></td><td><h1>AngleList Class</h1></td></tr></table>
<div class="summary">Angle list helper.</div>
<strong>Namespace:</strong> <a href="/topic/x">Tekla.Structures.Datatype</a><br>
<div class="collapsibleAreaRegion"><span class="collapsibleRegionTitle">Methods</span></div>
<div class="collapsibleSection">
<table class="members" id="methodList">
<tr><th>Name</th><th>Description</th><th>Notes</th></tr>
<tr><td></td>
<td><a href="/topic/method-x">ToString(IEnumerable<span id="LSTCB3882A2_3"></span><script type="text/javascript">AddLanguageSpecificTextSet("LSTCB3882A2_3?cs=&lt;|vb=(Of |cpp=&lt;|fs=&lt;'|nu=(");</script>Angle<span id="LSTCB3882A2_4"></span><script type="text/javascript">AddLanguageSpecificTextSet("LSTCB3882A2_4?cs=&gt;|vb=)|cpp=&gt;|fs=&gt;|nu=)");</script>)</a></td>
<td><div class="summary">Convert.</div></td>
</tr>
</table>
</div>
</div>
</body></html>
""";
        var result = PageParser.Parse(html, "https://example.com/type");
        Assert.NotNull(result);
        Assert.Single(result!.Type.methods);
        Assert.Equal("ToString(IEnumerable<Angle>)", result.Type.methods[0].name);
        Assert.DoesNotContain("IEnumerable.Angle.", result.Type.methods[0].name);
    }
}
