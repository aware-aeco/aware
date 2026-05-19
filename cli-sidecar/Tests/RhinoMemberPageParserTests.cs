// Tests for the Rhino MemberPageParser. Fixtures are cached HTML pulled from the McNeel
// raw-github mirror at branch 7 (Rhino 7.19 — the LTS final). Committed fixtures keep the
// suite offline + deterministic across machines.

using AwareSidecar.Ingest.Extractors.Rhino;
using Xunit;

namespace AwareSidecar.Ingest.Tests;

public class RhinoMemberPageParserTests
{
    static string LoadFixture(string name)
    {
        var here = Path.GetDirectoryName(typeof(RhinoMemberPageParserTests).Assembly.Location)!;
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

    [Fact]
    public void Method_DistanceTo_Parses_Signature_Param_Return()
    {
        var html = LoadFixture("rhino-method-distanceto.html");
        var r = MemberPageParser.ParseMethod(html, isCtor: false);
        Assert.NotNull(r);
        Assert.Contains("DistanceTo", r!.signature);
        // DistanceTo takes a Point3d parameter and returns Double.
        Assert.NotEmpty(r.@params);
        Assert.Contains(r.@params, p => p.name == "other");
        Assert.NotNull(r.returns);
        Assert.Equal("Double", r.returns!.type);
    }

    [Fact]
    public void Operator_Addition_Is_Parsed_As_Method()
    {
        // The page title ends in " Operator". The Tekla parser would reject this; the Rhino parser
        // accepts " Operator" alongside " Method" for non-ctor calls.
        var html = LoadFixture("rhino-method-op-addition.html");
        var r = MemberPageParser.ParseMethod(html, isCtor: false);
        Assert.NotNull(r);
        Assert.NotNull(r!.returns);
        Assert.Contains("Point3d", r.returns!.type);
        Assert.NotEmpty(r.@params);
    }

    [Fact]
    public void Operator_Page_Is_Rejected_For_IsCtor()
    {
        // Constructor parse must NOT swallow operator pages — title ends in " Operator", not
        // " Constructor".
        var html = LoadFixture("rhino-method-op-addition.html");
        var r = MemberPageParser.ParseMethod(html, isCtor: true);
        Assert.Null(r);
    }

    [Fact]
    public void Conversion_Operator_Is_Parsed_As_Method()
    {
        // Sandcastle renders implicit/explicit conversion operators with h1 ending in
        // " Conversion (FromType to ToType)" rather than " Operator" or " Method". The leading
        // LST tag's `nu=` reads "Explicit" or "Implicit" depending on direction. Either way,
        // the trailing kind word is " Conversion" — the parser accepts that as a method-like
        // member.
        var html = LoadFixture("rhino-method-op-conversion.html");
        var r = MemberPageParser.ParseMethod(html, isCtor: false);
        Assert.NotNull(r);
        // The signature should reference the source type (GH_ScriptVariant) and target type
        // (Boolean) somewhere.
        Assert.False(string.IsNullOrEmpty(r!.signature));
        Assert.NotNull(r.returns);
    }

    [Fact]
    public void Constructor_Point3d_Parses()
    {
        var html = LoadFixture("rhino-ctor-point3d.html");
        var r = MemberPageParser.ParseMethod(html, isCtor: true);
        Assert.NotNull(r);
        // Constructor signature carries the type name.
        Assert.Contains("Point3d", r!.signature);
        // Constructor has no return value (void in IR semantics).
        Assert.Null(r.returns);
    }

    [Fact]
    public void Property_IsValid_Parses_Type_And_Getter()
    {
        var html = LoadFixture("rhino-property-isvalid.html");
        var r = MemberPageParser.ParseProperty(html);
        Assert.NotNull(r);
        Assert.Equal("Boolean", r!.type);
        Assert.True(r.getter);
    }

    [Fact]
    public void Field_Epsilon_Parses_Type_Getter_NoSetter()
    {
        // Field title ends in " Field"; ParseProperty must recognise it AND emit setter=false.
        var html = LoadFixture("rhino-field-epsilon.html");
        var r = MemberPageParser.ParseProperty(html);
        Assert.NotNull(r);
        Assert.True(r!.getter);
        Assert.False(r.setter);
        // Epsilon is a Double constant.
        Assert.Equal("Double", r.type);
    }

    [Fact]
    public void Event_AppSettingsChanged_Parses_Delegate()
    {
        var html = LoadFixture("rhino-event-appsettings.html");
        var r = MemberPageParser.ParseEvent(html);
        Assert.NotNull(r);
        Assert.False(string.IsNullOrEmpty(r!.@delegate));
        // The delegate type is some EventHandler shape — must NOT collapse to the bare placeholder
        // "EventHandler" alone (that's the unenriched type-page row).
        Assert.NotEqual("EventHandler", r.@delegate);
    }
}
