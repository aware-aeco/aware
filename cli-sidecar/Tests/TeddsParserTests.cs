// Tests for the Tekla Tedds PageParser + MemberPageParser. Fixtures live in
// cli-sidecar/Tests/fixtures/tedds-*.html and are committed for deterministic
// offline reproduction.
//
// Each fixture is the verbatim HTML response captured from
//   https://developer.tekla.com/doc/tekla-tedds/<ver>/<slug>-<id>
// on 2026-05-19. The fixtures cover the Tedds-specific quirks the parsers must handle:
//   - DocFX-style markup (article#_content, h1 "Class Foo"/"Interface IFoo", <h5 class="declaration">…)
//   - Constructor pages bundling MULTIPLE overloads on one page (tedds-ctor-aliasattribute)
//   - 3-column enum-value tables (Name|Value|Description)
//   - Property pages with `{ get; }` vs `{ get; set; }` syntax patterns
//   - Method pages with and without a Returns section (void → null returns)

using AngleSharp.Html.Parser;
using AwareSidecar.Ingest.Extractors.Tedds;
using Xunit;

namespace AwareSidecar.Ingest.Tests;

public class TeddsParserTests
{
    static string LoadFixture(string name)
    {
        var here = Path.GetDirectoryName(typeof(TeddsParserTests).Assembly.Location)!;
        var path = Path.Combine(here, "fixtures", name);
        if (!File.Exists(path))
        {
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

    // ── type-page tests ─────────────────────────────────────────────────────

    [Fact]
    public void Type_AliasAttribute_Parses_Class_Kind_And_Namespace()
    {
        var html = LoadFixture("tedds-type-aliasattribute.html");
        var url = "https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-expressoaddin-aliasattribute-41824";
        var r = PageParser.Parse(html, url);
        Assert.NotNull(r);
        Assert.Equal("AliasAttribute", r!.Type.name);
        Assert.Equal("class", r.Type.kind);
        Assert.Equal("Tekla.Structural.ExpressoAddIn", r.Type.@namespace);
        // The fixture page has an empty summary div → parser substitutes a placeholder.
        Assert.NotEmpty(r.Type.summary);
        // Two constructor overloads listed in the Constructors table.
        Assert.Equal(2, r.Type.constructors.Count);
        Assert.Contains("AliasAttribute(string)", r.Type.constructors.Select(c => c.name));
        Assert.Contains("AliasAttribute(string, bool)", r.Type.constructors.Select(c => c.name));
        // Base class is Attribute (from inheritance block).
        Assert.NotNull(r.Type.@base);
        Assert.Contains("Attribute", r.Type.@base!);
    }

    [Fact]
    public void Type_IApplication_Parses_Interface_With_Methods_And_Properties()
    {
        var html = LoadFixture("tedds-type-iapplication.html");
        var url = "https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-tedds-iapplication-41831";
        var r = PageParser.Parse(html, url);
        Assert.NotNull(r);
        Assert.Equal("IApplication", r!.Type.name);
        Assert.Equal("interface", r.Type.kind);
        Assert.Equal("Tekla.Structural.InteropAssemblies.Tedds", r.Type.@namespace);
        // The IApplication interface lists 16 properties and 4 methods in the fixture.
        Assert.True(r.Type.properties.Count >= 10,
            $"expected >=10 properties, got {r.Type.properties.Count}");
        Assert.True(r.Type.methods.Count >= 3,
            $"expected >=3 methods, got {r.Type.methods.Count}");
        // Spot-check a known property + method.
        Assert.Contains("ActiveDocument", r.Type.properties.Select(p => p.name));
        Assert.Contains(r.Type.methods, m => m.name.StartsWith("Move", StringComparison.Ordinal));
        // Member URLs should be present so the enrich pass can fetch them.
        Assert.All(r.Links.Properties, u => Assert.False(string.IsNullOrEmpty(u)));
        Assert.All(r.Links.Methods, u => Assert.False(string.IsNullOrEmpty(u)));
    }

    [Fact]
    public void Type_TdsCalcStatus_Parses_Enum_With_Integer_Values()
    {
        var html = LoadFixture("tedds-type-tdscalcstatus.html");
        var url = "https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-tedds-tdscalcstatus-41841";
        var r = PageParser.Parse(html, url);
        Assert.NotNull(r);
        Assert.Equal("TdsCalcStatus", r!.Type.name);
        Assert.Equal("enum", r.Type.kind);
        Assert.Equal(4, r.Type.enum_values.Count);
        // Names should be the bare field names.
        var names = r.Type.enum_values.Select(v => v.name).ToList();
        Assert.Contains("Aborted", names);
        Assert.Contains("Ok", names);
        // The "Aborted = -1" cell's value parses to -1 as an integer.
        var aborted = r.Type.enum_values.Single(v => v.name == "Aborted");
        Assert.Equal(System.Text.Json.JsonValueKind.Number, aborted.value.ValueKind);
        Assert.Equal(-1, aborted.value.GetInt32());
        var ok = r.Type.enum_values.Single(v => v.name == "Ok");
        Assert.Equal(1, ok.value.GetInt32());
        // Summary text should round-trip.
        Assert.Equal("Calculating completed successfully", ok.summary);
    }

    // ── member-page tests ───────────────────────────────────────────────────

    [Fact]
    public void Constructor_AliasAttribute_Parses_First_Overload_Signature_And_Params()
    {
        var html = LoadFixture("tedds-ctor-aliasattribute.html");
        // Hint matches the type-page row text for the first overload.
        var r = MemberPageParser.ParseMethod(html, isCtor: true, memberRowText: "AliasAttribute(string)");
        Assert.NotNull(r);
        Assert.Contains("AliasAttribute", r!.signature);
        Assert.Contains("string", r.signature);
        Assert.Single(r.@params);
        Assert.Equal("alias", r.@params[0].name);
        Assert.Equal("string", r.@params[0].type);
        // Constructors return void → returns must be null (not a placeholder).
        Assert.Null(r.returns);
    }

    [Fact]
    public void Constructor_AliasAttribute_Parses_Second_Overload_TwoParams()
    {
        var html = LoadFixture("tedds-ctor-aliasattribute.html");
        var r = MemberPageParser.ParseMethod(html, isCtor: true, memberRowText: "AliasAttribute(string, bool)");
        Assert.NotNull(r);
        Assert.Contains("AliasAttribute", r!.signature);
        Assert.Equal(2, r.@params.Count);
        Assert.Equal("alias", r.@params[0].name);
        Assert.Equal("string", r.@params[0].type);
        Assert.Equal("aliasOnly", r.@params[1].name);
        Assert.Equal("bool", r.@params[1].type);
        Assert.Null(r.returns);
    }

    [Fact]
    public void Method_Move_Parses_Void_Signature_And_Two_Params()
    {
        var html = LoadFixture("tedds-method-move-void.html");
        var r = MemberPageParser.ParseMethod(html, isCtor: false, memberRowText: "Move(int, int)");
        Assert.NotNull(r);
        Assert.Contains("Move", r!.signature);
        Assert.Contains("void", r.signature);
        Assert.Equal(2, r.@params.Count);
        Assert.Equal("Left", r.@params[0].name);
        Assert.Equal("int", r.@params[0].type);
        Assert.Equal("Top", r.@params[1].name);
        Assert.Equal("int", r.@params[1].type);
        // Void method → returns must be null per IR schema contract.
        Assert.Null(r.returns);
    }

    [Fact]
    public void Method_GetOutput_Parses_String_Return_Type_And_Doc()
    {
        var html = LoadFixture("tedds-method-getoutput-returns.html");
        var r = MemberPageParser.ParseMethod(html, isCtor: false, memberRowText: "GetOutput(OutputFormat)");
        Assert.NotNull(r);
        Assert.Contains("GetOutput", r!.signature);
        Assert.Contains("string", r.signature);
        Assert.Single(r.@params);
        Assert.Equal("format", r.@params[0].name);
        Assert.Equal("OutputFormat", r.@params[0].type);
        // Non-void method → returns must be populated with type + doc.
        Assert.NotNull(r.returns);
        Assert.Equal("string", r.returns!.type);
        Assert.False(string.IsNullOrEmpty(r.returns.doc));
        // Remarks section should be captured (the fixture has a multi-paragraph Remarks).
        Assert.NotNull(r.remarks);
    }

    [Fact]
    public void Property_ActiveDocument_Parses_ReadOnly_With_ITeddsDocument_Type()
    {
        var html = LoadFixture("tedds-property-activedocument.html");
        var r = MemberPageParser.ParseProperty(html);
        Assert.NotNull(r);
        Assert.Equal("ITeddsDocument", r!.type);
        Assert.True(r.getter);
        Assert.False(r.setter);  // syntax `{ get; }` only.
    }

    // ── extractor URL discovery tests ───────────────────────────────────────

    [Fact]
    public void ExtractNamespaceUrls_From_Root_Page_Finds_All_Three_Namespaces()
    {
        // Synthetic-ish fixture: the root page's sidebar tree is in our type-page fixture's
        // own sidebar (since every doc page renders the full tree). We reuse the type-page
        // fixture to verify the namespace-URL filter works against the real DOM.
        var html = LoadFixture("tedds-type-aliasattribute.html");
        var urls = Extractor.ExtractNamespaceUrls(html, "2025");
        // Expect exactly 3 namespaces under tekla-tedds: ExpressoAddIn, InteropAssemblies.Tedds,
        // InteropAssemblies.TeddsCalc.
        Assert.Equal(3, urls.Count);
        Assert.Contains(urls, u => u.EndsWith("tekla-structural-expressoaddin-41821", StringComparison.Ordinal));
        Assert.Contains(urls, u => u.EndsWith("tekla-structural-interopassemblies-tedds-41822", StringComparison.Ordinal));
        Assert.Contains(urls, u => u.EndsWith("tekla-structural-interopassemblies-teddscalc-41823", StringComparison.Ordinal));
    }

    [Fact]
    public void ExtractTypeUrls_From_Namespace_Page_Filters_Out_Member_Pages()
    {
        // Same trick: the type-page sidebar holds the full nav tree, including the parent
        // namespace's other types. We pass an expanded namespace URL so the function
        // excludes it from the result.
        var html = LoadFixture("tedds-type-aliasattribute.html");
        var nsUrl = "https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-expressoaddin-41821";
        var urls = Extractor.ExtractTypeUrls(html, "2025", nsUrl);
        // Should contain TYPE URLs (no parens in anchor text, no dots) and exclude the namespace itself.
        Assert.DoesNotContain(nsUrl, urls);
        // Spot-check: the ExpressoAddIn types live in the sidebar.
        Assert.Contains(urls, u => u.Contains("aliasattribute-41824", StringComparison.Ordinal));
        Assert.Contains(urls, u => u.Contains("fieldattribute-41825", StringComparison.Ordinal));
        // Constructor overload pages (e.g. -ctor-) MUST be excluded — they should only come
        // from per-type member tables, not the root nav.
        Assert.DoesNotContain(urls, u => u.Contains("-ctor-", StringComparison.Ordinal));
    }
}
