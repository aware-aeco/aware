// Tests for the Tekla MemberPageParser. We cache the live HTML once in <repo>/cli-sidecar/Tests/fixtures/
// so the suite is fully offline + deterministic. The fixtures are committed alongside the test so a
// future run reproduces the exact bytes used here.

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
}
