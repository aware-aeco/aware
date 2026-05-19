// Tests for the IDEA Statica YamlReader + OpenApiParser + MemberPageParser.
//
// The fixtures are excerpted from the openapi-generator output (Connection API +
// RCS API openapi.yaml at tag 26.0.1.2450) plus a handful of representative
// markdown SDK doc pages. Each fixture isolates one structural feature so a
// regression in the parser surfaces against a focused failure.

using AwareSidecar.Ingest.Extractors.IdeaStatica;
using Xunit;

namespace AwareSidecar.Ingest.Tests;

public class IdeaStaticaParserTests
{
    static string LoadFixture(string name)
    {
        var here = Path.GetDirectoryName(typeof(IdeaStaticaParserTests).Assembly.Location)!;
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

    // ── YamlReader fundamentals ────────────────────────────────────────────

    [Fact]
    public void Yaml_Parse_Mini_Mapping()
    {
        // Two top-level keys with a nested mapping under the second.
        const string yaml = """
            openapi: 3.0.4
            info:
              title: Test API
              version: "1.0"
            """;
        var node = YamlReader.Parse(yaml);
        var root = Assert.IsType<YamlNode.Mapping>(node);
        Assert.Equal("3.0.4", root.GetScalar("openapi"));
        var info = root.GetMapping("info");
        Assert.NotNull(info);
        Assert.Equal("Test API", info!.GetScalar("title"));
        Assert.Equal("1.0", info.GetScalar("version"));
    }

    [Fact]
    public void Yaml_Parse_Sequence_Of_Mappings()
    {
        const string yaml = """
            parameters:
            - name: projectId
              in: path
              required: true
            - name: connectionId
              in: path
              required: false
            """;
        var node = YamlReader.Parse(yaml);
        var root = Assert.IsType<YamlNode.Mapping>(node);
        var seq = root.GetSequence("parameters");
        Assert.NotNull(seq);
        Assert.Equal(2, seq!.Items.Count);
        var first = Assert.IsType<YamlNode.Mapping>(seq.Items[0]);
        Assert.Equal("projectId", first.GetScalar("name"));
        Assert.Equal("path", first.GetScalar("in"));
        Assert.Equal("true", first.GetScalar("required"));
    }

    [Fact]
    public void Yaml_Parse_DoubleQuoted_MultiLine_String()
    {
        // openapi-generator wraps long description fields with the backslash-newline
        // continuation pattern. Each continuation line starts with `\ ` literally.
        const string yaml = """
            description: "IDEA StatiCa Connection API, used for the automated design and calculation\
              \ of steel connections."
            """;
        var node = YamlReader.Parse(yaml);
        var root = Assert.IsType<YamlNode.Mapping>(node);
        var desc = root.GetScalar("description");
        Assert.NotNull(desc);
        Assert.Contains("Connection API", desc);
        Assert.Contains("steel connections.", desc);
        // No backslash should remain in the decoded scalar.
        Assert.DoesNotContain("\\", desc!);
    }

    // ── OpenApiParser end-to-end ───────────────────────────────────────────

    [Fact]
    public void OpenApi_Parses_Tiny_Spec_Into_Types()
    {
        var yaml = LoadFixture("idea-statica-mini-spec.yaml");
        var node = YamlReader.Parse(yaml);
        var root = Assert.IsType<YamlNode.Mapping>(node);

        var ctx = new OpenApiParser.SpecContext(
            Document: root,
            SourceUrl: "https://example.test/openapi.yaml",
            DocBaseUrl: "https://example.test/docs/",
            NamespaceForSchemas: "TestApi.Model",
            NamespaceForApis: "TestApi.Api",
            ApiTitle: "Test API",
            ApiVersion: "1.0");
        var types = OpenApiParser.ParseAll(ctx);

        // Expect: SimpleModel (class), Color (enum), ItemApi (synthesized API class).
        Assert.True(types.Count >= 3, $"expected ≥3 types, got {types.Count}");
        Assert.Contains(types, t => t.name == "SimpleModel" && t.kind == "class");
        Assert.Contains(types, t => t.name == "Color" && t.kind == "enum");
        Assert.Contains(types, t => t.name == "ItemApi" && t.kind == "class");

        var simple = types.First(t => t.name == "SimpleModel");
        Assert.Contains(simple.properties, p => p.name == "Id" && p.type == "int");
        Assert.Contains(simple.properties, p => p.name == "Name" && p.type == "string");
        Assert.Contains(simple.properties, p => p.name == "Color");

        var color = types.First(t => t.name == "Color");
        Assert.Equal(3, color.enum_values.Count);
        Assert.Contains(color.enum_values, v => v.name == "red");
        Assert.Contains(color.enum_values, v => v.name == "blue");

        var api = types.First(t => t.name == "ItemApi");
        // GetItem and CreateItem operations are present in the mini spec.
        Assert.Contains(api.methods, m => m.name == "GetItemAsync");
        Assert.Contains(api.methods, m => m.name == "CreateItemAsync");
        var getItem = api.methods.First(m => m.name == "GetItemAsync");
        Assert.NotNull(getItem.returns);
        Assert.Equal("SimpleModel", getItem.returns!.type);
        Assert.Contains(getItem.@params, p => p.name == "itemId" && p.type == "Guid");
    }

    // ── MemberPageParser markdown enrichment ───────────────────────────────

    [Fact]
    public void Markdown_Parses_Model_Page_Properties()
    {
        var md = LoadFixture("idea-statica-model-conconnection.md");
        var result = MemberPageParser.ParseModel(md);
        Assert.NotNull(result);
        // The fixture's "Description" column is blank for most fields, so we don't expect every
        // property to have a description — but the parser should successfully read the table.
        Assert.NotNull(result!.PropertyDescriptions);
    }

    [Fact]
    public void Markdown_Parses_Api_Page_Methods()
    {
        var md = LoadFixture("idea-statica-api-calculation.md");
        var result = MemberPageParser.ParseApi(md);
        Assert.NotNull(result);
        Assert.NotEmpty(result!.MethodsByOperationId);
        // Anchor ids are lowercase operationIds (e.g. "calculate", "getresults").
        Assert.True(result.MethodsByOperationId.ContainsKey("calculate"));
        var calculate = result.MethodsByOperationId["calculate"];
        Assert.False(string.IsNullOrEmpty(calculate.Summary));
        // The Parameters table lists projectId + requestBody — both should land in ParamDocs.
        Assert.True(calculate.ParamDocs.ContainsKey("projectId"));
    }
}
