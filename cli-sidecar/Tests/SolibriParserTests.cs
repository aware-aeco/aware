// Tests for the Solibri extractor's distinctive shapes:
//
//   - Tag synthesis (every operation gets `tags: [paths]`)
//   - operationId synthesis (operations lacking `operationId:` get a derived value)
//   - Root-level array schemas (`type: array` at the schema root) → class with Items: List<T>
//   - Inline-map additionalProperties on object properties → Dictionary<string, T>
//   - Single-`paths` API-class assembly with all 26 (or N) operations
//
// The fixtures (cli-sidecar/Tests/fixtures/solibri-mini-spec.yaml) deliberately
// stress these patterns in a small spec so the assertions are focused.

using AwareSidecar.Ingest.Extractors.IdeaStatica;
using SolibriExtractor = AwareSidecar.Ingest.Extractors.Solibri.Extractor;
using Xunit;

namespace AwareSidecar.Ingest.Tests;

public class SolibriParserTests
{
    static string LoadFixture(string name)
    {
        var here = Path.GetDirectoryName(typeof(SolibriParserTests).Assembly.Location)!;
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

    // ── Tag + operationId synthesis ─────────────────────────────────────────

    [Fact]
    public void DeriveOperationId_Get_Ping_Returns_camelcase_ping()
    {
        Assert.Equal("ping", SolibriExtractor.DeriveOperationIdFromVerbAndPath("get", "/ping"));
    }

    [Fact]
    public void DeriveOperationId_Get_MultiSegment_Joins_PascalCased()
    {
        // GET /threed/camera → threedCamera (lowercase first letter, PascalCase tail).
        Assert.Equal("threedCamera", SolibriExtractor.DeriveOperationIdFromVerbAndPath("get", "/threed/camera"));
    }

    [Fact]
    public void DeriveOperationId_Post_PrefixesCreate()
    {
        Assert.Equal("createPresentations", SolibriExtractor.DeriveOperationIdFromVerbAndPath("post", "/presentations"));
    }

    [Fact]
    public void DeriveOperationId_Skips_PathVariables()
    {
        // /info/{guid} → info (variable segment is skipped).
        Assert.Equal("info", SolibriExtractor.DeriveOperationIdFromVerbAndPath("get", "/info/{guid}"));
    }

    // ── End-to-end mini-spec parse ──────────────────────────────────────────

    [Fact]
    public void SolibriExtractor_Synthesises_Single_paths_Class()
    {
        var yaml = LoadFixture("solibri-mini-spec.yaml");
        var root = YamlReader.Parse(yaml);
        var rootMap = Assert.IsType<YamlNode.Mapping>(root);

        // Synthesise tags + ids — same pre-pass the extractor runs.
        SolibriExtractor.SynthesiseTagsAndOperationIds(rootMap);

        // Every operation should now have `tags: [paths]`.
        var paths = rootMap.GetMapping("paths");
        Assert.NotNull(paths);
        var opsExamined = 0;
        foreach (var pathKey in paths!.KeyOrder)
        {
            if (paths.Get(pathKey) is not YamlNode.Mapping verbsMap) continue;
            foreach (var verb in verbsMap.KeyOrder)
            {
                if (verbsMap.Get(verb) is not YamlNode.Mapping op) continue;
                var tags = op.GetSequence("tags");
                Assert.NotNull(tags);
                Assert.Single(tags!.Items);
                var tagScalar = Assert.IsType<YamlNode.Scalar>(tags.Items[0]);
                Assert.Equal("paths", tagScalar.Value);
                opsExamined++;
            }
        }
        Assert.Equal(4, opsExamined);   // /ping GET, /shutdown POST, /models GET, /threed/camera GET

        // Now run the OpenApiParser — should produce ONE `pathsApi` class (the
        // suffix is dropped by FixDocUrls in the extractor, but the parser
        // itself appends it). The mini-spec also has three Model schemas:
        // PingResponse, ModelInfo, ModelInfoList.
        var ctx = new OpenApiParser.SpecContext(
            Document: rootMap,
            SourceUrl: "https://test.example/openapi.yaml",
            DocBaseUrl: "https://test.example/openapi.yaml?schema=",
            NamespaceForSchemas: "Solibri.Rest.Model",
            NamespaceForApis: "Solibri.Rest.Api",
            ApiTitle: "Solibri REST API",
            ApiVersion: "1.0.0");
        var types = OpenApiParser.ParseAll(ctx);
        // 3 schemas + 1 *Api class.
        Assert.Equal(4, types.Count);

        var apiTypes = types.Where(t => t.@namespace == "Solibri.Rest.Api").ToList();
        Assert.Single(apiTypes);
        Assert.Equal("pathsApi", apiTypes[0].name);     // parser-emitted name; extractor renames to "paths"
        Assert.Equal(4, apiTypes[0].methods.Count);     // ping + shutdown + getModels + getCameraState
    }

    // ── Root-level array schema ─────────────────────────────────────────────

    [Fact]
    public void OpenApi_Parses_RootLevel_Array_Schema_Into_Items_Property()
    {
        var yaml = LoadFixture("solibri-mini-spec.yaml");
        var root = YamlReader.Parse(yaml);
        var rootMap = Assert.IsType<YamlNode.Mapping>(root);

        var ctx = new OpenApiParser.SpecContext(
            Document: rootMap,
            SourceUrl: "https://test.example/openapi.yaml",
            DocBaseUrl: "https://test.example/openapi.yaml?schema=",
            NamespaceForSchemas: "Solibri.Rest.Model",
            NamespaceForApis: "Solibri.Rest.Api",
            ApiTitle: "Solibri REST API",
            ApiVersion: "1.0.0");
        var types = OpenApiParser.ParseAll(ctx);

        var modelInfoList = types.First(t => t.name == "ModelInfoList");
        Assert.Equal("class", modelInfoList.kind);
        Assert.Single(modelInfoList.properties);
        var itemsProp = modelInfoList.properties[0];
        Assert.Equal("Items", itemsProp.name);
        Assert.Equal("List<ModelInfo>", itemsProp.type);
        // Remarks should identify the wrapper shape.
        Assert.NotNull(modelInfoList.remarks);
        Assert.Contains("List wrapper", modelInfoList.remarks!);
    }

    // ── Inline-map additionalProperties on object property ──────────────────

    [Fact]
    public void OpenApi_Parses_Inline_Map_AdditionalProperties_As_Dictionary()
    {
        var yaml = LoadFixture("solibri-mini-spec.yaml");
        var root = YamlReader.Parse(yaml);
        var rootMap = Assert.IsType<YamlNode.Mapping>(root);

        var ctx = new OpenApiParser.SpecContext(
            Document: rootMap,
            SourceUrl: "https://test.example/openapi.yaml",
            DocBaseUrl: "https://test.example/openapi.yaml?schema=",
            NamespaceForSchemas: "Solibri.Rest.Model",
            NamespaceForApis: "Solibri.Rest.Api",
            ApiTitle: "Solibri REST API",
            ApiVersion: "1.0.0");
        var types = OpenApiParser.ParseAll(ctx);

        var modelInfo = types.First(t => t.name == "ModelInfo");
        var metadataProp = modelInfo.properties.First(p => p.name == "Metadata");
        Assert.Equal("Dictionary<string, string>", metadataProp.type);
    }
}
