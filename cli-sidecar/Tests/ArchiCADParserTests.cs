// Tests for the ArchiCAD parsers: Tapir command/schema parser + Graphisoft menutree
// parser + PageParser command-to-IR translation.
//
// Fixtures are minimised slices of the real data sources (committed to keep tests
// offline / deterministic):
//   - archicad-tapir-commands-minimal.js  — 3 commands across 2 categories from Tapir
//   - archicad-tapir-schema-minimal.js    — 4 shared schema types from Tapir
//   - archicad-graphisoft-menutree.json   — 3 commands across 2 groups from Graphisoft
//   - archicad-graphisoft-command-*.json  — one full Graphisoft command JSON

using AwareSidecar.Ingest.Extractors.ArchiCAD;
using Xunit;

namespace AwareSidecar.Ingest.Tests;

public class ArchiCADParserTests
{
    static string LoadFixture(string name)
    {
        var here = Path.GetDirectoryName(typeof(ArchiCADParserTests).Assembly.Location)!;
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
    public void TapirCommandParser_ExtractsCategoriesAndCommands()
    {
        var js = LoadFixture("archicad-tapir-commands-minimal.js");
        var categories = TapirCommandParser.Parse(js);

        Assert.Equal(2, categories.Count);

        var app = categories.First(c => c.Name == "Application Commands");
        Assert.Equal(2, app.Commands.Count);
        var getAddOnVersion = app.Commands.First(c => c.Name == "GetAddOnVersion");
        Assert.Equal("0.1.0", getAddOnVersion.Version);
        Assert.Contains("version of the Tapir", getAddOnVersion.Description);
        Assert.Null(getAddOnVersion.InputScheme); // null in source

        var changeWindow = app.Commands.First(c => c.Name == "ChangeWindow");
        Assert.NotNull(changeWindow.InputScheme);
        Assert.NotNull(changeWindow.OutputScheme);

        var elements = categories.First(c => c.Name == "Element Commands");
        Assert.Single(elements.Commands);
    }

    [Fact]
    public void TapirSchemaParser_ResolvesAllSharedTypes()
    {
        var js = LoadFixture("archicad-tapir-schema-minimal.js");
        var schema = TapirSchemaParser.Parse(js);

        Assert.Equal(4, schema.Count);
        Assert.True(schema.ContainsKey("Elements"));
        Assert.True(schema.ContainsKey("ElementIdArrayItem"));
        Assert.True(schema.ContainsKey("ElementId"));
        Assert.True(schema.ContainsKey("SurfaceType"));

        // Cloned elements must outlive parsing — accessing properties after the
        // local doc disposes should not throw.
        Assert.Equal(
            "The identifier of an element.",
            TapirSchemaParser.ExtractDescription(schema["ElementId"]));
    }

    [Fact]
    public void TapirSchemaParser_SummariseSchemaType_HandlesRefAndPrimitives()
    {
        var js = LoadFixture("archicad-tapir-schema-minimal.js");
        var schema = TapirSchemaParser.Parse(js);

        // ElementId.guid is { "$ref": "#/Guid" } — should return "Guid"
        var idGuid = schema["ElementId"].GetProperty("properties").GetProperty("guid");
        Assert.Equal("Guid", TapirSchemaParser.SummariseSchemaType(idGuid));

        // Elements is { "type": "array", "items": { "$ref": "#/ElementIdArrayItem" } }
        Assert.Equal("ElementIdArrayItem[]", TapirSchemaParser.SummariseSchemaType(schema["Elements"]));

        // SurfaceType is a string enum
        Assert.Equal("string", TapirSchemaParser.SummariseSchemaType(schema["SurfaceType"]));
    }

    [Fact]
    public void PageParser_BuildTapirCategoryType_ProducesStaticClassWithMethods()
    {
        var js = LoadFixture("archicad-tapir-commands-minimal.js");
        var categories = TapirCommandParser.Parse(js);
        var app = categories.First(c => c.Name == "Application Commands");

        var typeInfo = PageParser.BuildTapirCategoryType(app, "https://example.test/");

        Assert.Equal("ApplicationCommands", typeInfo.name);
        Assert.Equal("Tapir.AdditionalCommands", typeInfo.@namespace);
        Assert.Equal("static-class", typeInfo.kind);
        Assert.Equal(2, typeInfo.methods.Count);

        // GetAddOnVersion: no params, returns an object
        var get = typeInfo.methods.First(m => m.name == "GetAddOnVersion");
        Assert.Empty(get.@params);
        Assert.NotNull(get.returns);
        Assert.Equal("0.1.0", get.since);

        // ChangeWindow: 2 params (one required, one optional), returns ExecutionResult ref
        var change = typeInfo.methods.First(m => m.name == "ChangeWindow");
        Assert.Equal(2, change.@params.Count);
        Assert.Equal("windowType", change.@params[0].name);
        Assert.False(change.@params[0].optional);
        Assert.Equal("databaseId", change.@params[1].name);
        Assert.True(change.@params[1].optional);
        Assert.NotNull(change.returns);
        Assert.Equal("ExecutionResult", change.returns!.type);
    }

    [Fact]
    public void PageParser_BuildSharedSchemaType_HandlesObjectAndEnum()
    {
        var js = LoadFixture("archicad-tapir-schema-minimal.js");
        var schema = TapirSchemaParser.Parse(js);

        // ElementId: object → kind="class" with one property
        var elementId = PageParser.BuildSharedSchemaType("ElementId", schema["ElementId"], "https://example.test/");
        Assert.Equal("class", elementId.kind);
        Assert.Single(elementId.properties);
        Assert.Equal("guid", elementId.properties[0].name);
        Assert.Equal("Guid", elementId.properties[0].type);

        // SurfaceType: enum → kind="enum" with enum_values populated
        var surfaceType = PageParser.BuildSharedSchemaType("SurfaceType", schema["SurfaceType"], "https://example.test/");
        Assert.Equal("enum", surfaceType.kind);
        Assert.Equal(4, surfaceType.enum_values.Count);
        Assert.Contains(surfaceType.enum_values, ev => ev.name == "General");
        Assert.Contains(surfaceType.enum_values, ev => ev.name == "Matte");
    }

    [Fact]
    public void PageParser_BuildSharedSchemaType_ArrayTypeRendersItemsProperty()
    {
        var js = LoadFixture("archicad-tapir-schema-minimal.js");
        var schema = TapirSchemaParser.Parse(js);

        var elements = PageParser.BuildSharedSchemaType("Elements", schema["Elements"], "https://example.test/");
        Assert.Equal("class", elements.kind);
        Assert.Single(elements.properties);
        Assert.Equal("Items", elements.properties[0].name);
        Assert.Equal("ElementIdArrayItem[]", elements.properties[0].type);
    }

    [Fact]
    public void GraphisoftMenuParser_ParseMenuTree_FlattensGroupedCommands()
    {
        var json = LoadFixture("archicad-graphisoft-menutree.json");
        var commands = GraphisoftMenuParser.ParseMenuTree(json);

        // Should skip "Introduction" (htmldocumentation, not commanddocumentation).
        Assert.Equal(3, commands.Count);
        Assert.Contains(commands, c => c.CommandName == "ExecuteAddOnCommand" && c.GroupName == "AddOn Commands");
        Assert.Contains(commands, c => c.CommandName == "CreateAttributeFolders" && c.GroupName == "Attribute Commands");

        // FetchUrl should resolve to the content path on the Graphisoft site
        var first = commands[0];
        Assert.StartsWith(GraphisoftMenuParser.ContentBaseUrl, first.FetchUrl);
        Assert.EndsWith(".json", first.FetchUrl);
    }

    [Fact]
    public void GraphisoftMenuParser_ParseCommandJson_ExtractsParamsAndResult()
    {
        var json = LoadFixture("archicad-graphisoft-command-createattrfolders.json");
        var def = GraphisoftMenuParser.ParseCommandJson("CreateAttributeFolders", json);

        Assert.Equal("CreateAttributeFolders", def.Name);
        Assert.Contains("Creates attribute folders", def.Description);
        Assert.NotNull(def.InputScheme);
        Assert.NotNull(def.OutputScheme);

        // Now convert to MethodInfo and verify the params shape
        var method = PageParser.BuildTapirCommandMethod(def, "https://example.test/");
        Assert.Equal("CreateAttributeFolders", method.name);
        Assert.Single(method.@params);
        Assert.Equal("attributeFolders", method.@params[0].name);
        // The items $ref points to APIAttributeTypes.json#/definitions/AttributeFolderCreationParameters;
        // SummariseSchemaType returns the fragment tail.
        Assert.Equal("AttributeFolderCreationParameters[]", method.@params[0].type);

        Assert.NotNull(method.returns);
        // response_parameters has a `executionResults` property → returns shape is "object" with a synth doc.
        Assert.Equal("object", method.returns!.type);
    }

    [Fact]
    public void TapirCommandParser_StripJsWrapper_RejectsMissingPrefix()
    {
        // Defensive: if Tapir restructures the file we MUST fail loudly, not produce a
        // mangled IR. The parser throws on a missing wrapper.
        var bad = "{ \"some\": \"json\" }";
        Assert.Throws<InvalidDataException>(() => TapirCommandParser.StripJsWrapper(bad, "gCommands"));
    }
}
