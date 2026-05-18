using System.IO;
using System.Text.Json;
using Json.Schema;
using Xunit;

public class SchemaTests
{
    // The schema file is copied into the test output directory by the .csproj
    // (<None Include="..\..\Schema\host-coverage.schema.json" />), so we can read
    // it directly from AppContext.BaseDirectory without counting `..` segments.
    static readonly string SchemaPath = Path.Combine(
        AppContext.BaseDirectory, "host-coverage.schema.json");

    [Fact]
    public void Schema_Is_Valid_JsonSchema()
    {
        var schemaJson = File.ReadAllText(SchemaPath);
        var schema = JsonSchema.FromText(schemaJson);
        Assert.NotNull(schema);
    }

    [Fact]
    public void Minimal_Valid_IR_Passes()
    {
        var schemaJson = File.ReadAllText(SchemaPath);
        var schema = JsonSchema.FromText(schemaJson);
        var ir = JsonDocument.Parse("""
        {
          "host": "test", "host_version": "1.0",
          "source": { "kind": "scrape", "urls": ["https://example.com"], "extracted_at": "2026-01-01T00:00:00Z" },
          "types": [],
          "metadata": { "page_count": 0, "type_count": 0, "method_count": 0, "event_count": 0, "property_count": 0 }
        }
        """);
        var result = schema.Evaluate(ir.RootElement);
        Assert.True(result.IsValid, "minimal IR should validate");
    }

    [Fact]
    public void Missing_Required_Field_Fails()
    {
        var schemaJson = File.ReadAllText(SchemaPath);
        var schema = JsonSchema.FromText(schemaJson);
        var ir = JsonDocument.Parse("""{ "host": "test" }""");
        var result = schema.Evaluate(ir.RootElement);
        Assert.False(result.IsValid, "missing required fields should fail");
    }
}
