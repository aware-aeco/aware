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

    static JsonSchema LoadSchema()
    {
        var schemaJson = File.ReadAllText(SchemaPath);
        return JsonSchema.FromText(schemaJson);
    }

    [Fact]
    public void Schema_Is_Valid_JsonSchema()
    {
        var schema = LoadSchema();
        Assert.NotNull(schema);
    }

    [Fact]
    public void Minimal_Valid_IR_Passes()
    {
        var schema = LoadSchema();
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
        var schema = LoadSchema();
        var ir = JsonDocument.Parse("""{ "host": "test" }""");
        var result = schema.Evaluate(ir.RootElement);
        Assert.False(result.IsValid, "missing required fields should fail");
    }

    [Fact]
    public void Schema_Rejects_Typo_In_Field_Name()
    {
        var schema = LoadSchema();
        var ir = JsonDocument.Parse("""
        {
          "host": "test", "host_version": "1.0",
          "source": { "kind": "scrape", "urls": [], "extracted_at": "2026-01-01T00:00:00Z" },
          "types": [],
          "metadata": { "page_count": 0, "type_count": 0, "method_count": 0, "event_count": 0, "property_count": 0 },
          "extra_field_we_did_not_declare": "should be rejected"
        }
        """);
        var result = schema.Evaluate(ir.RootElement);
        Assert.False(result.IsValid, "schema should reject unknown top-level fields");
    }

    [Fact]
    public void Schema_Rejects_Method_Returns_Without_Type_And_Doc()
    {
        var schema = LoadSchema();
        var ir = JsonDocument.Parse("""
        {
          "host": "test", "host_version": "1.0",
          "source": { "kind": "scrape", "urls": [], "extracted_at": "2026-01-01T00:00:00Z" },
          "types": [{
            "name": "T", "namespace": "N", "kind": "class", "summary": "s", "doc_url": "https://x.example/T",
            "interfaces": [], "constructors": [], "properties": [], "events": [], "enum_values": [],
            "methods": [{
              "name": "M", "signature": "void M()", "summary": "s", "params": [],
              "returns": {},
              "throws": [], "events_related": [],
              "doc_url": "https://x.example/T.M"
            }]
          }],
          "metadata": { "page_count": 0, "type_count": 1, "method_count": 1, "event_count": 0, "property_count": 0 }
        }
        """);
        var result = schema.Evaluate(ir.RootElement);
        Assert.False(result.IsValid, "schema should reject Method.returns missing type+doc");
    }

    [Fact]
    public void Schema_Accepts_Method_Returns_As_Null()
    {
        var schema = LoadSchema();
        var ir = JsonDocument.Parse("""
        {
          "host": "test", "host_version": "1.0",
          "source": { "kind": "scrape", "urls": [], "extracted_at": "2026-01-01T00:00:00Z" },
          "types": [{
            "name": "T", "namespace": "N", "kind": "class", "summary": "s", "doc_url": "https://x.example/T",
            "interfaces": [], "constructors": [], "properties": [], "events": [], "enum_values": [],
            "methods": [{
              "name": "M", "signature": "void M()", "summary": "s", "params": [],
              "returns": null,
              "throws": [], "events_related": [],
              "doc_url": "https://x.example/T.M"
            }]
          }],
          "metadata": { "page_count": 0, "type_count": 1, "method_count": 1, "event_count": 0, "property_count": 0 }
        }
        """);
        var result = schema.Evaluate(ir.RootElement);
        Assert.True(result.IsValid, "schema should accept Method.returns as null for void methods");
    }

    [Fact]
    public void Schema_Rejects_Event_Without_HandlerThread()
    {
        var schema = LoadSchema();
        var ir = JsonDocument.Parse("""
        {
          "host": "test", "host_version": "1.0",
          "source": { "kind": "scrape", "urls": [], "extracted_at": "2026-01-01T00:00:00Z" },
          "types": [{
            "name": "T", "namespace": "N", "kind": "class", "summary": "s", "doc_url": "https://x.example/T",
            "interfaces": [], "constructors": [], "methods": [], "properties": [], "enum_values": [],
            "events": [{
              "name": "E", "delegate": "EDelegate", "signature": "void EDelegate()",
              "summary": "s", "doc_url": "https://x.example/T.E"
            }]
          }],
          "metadata": { "page_count": 0, "type_count": 1, "method_count": 0, "event_count": 1, "property_count": 0 }
        }
        """);
        var result = schema.Evaluate(ir.RootElement);
        Assert.False(result.IsValid, "schema should reject Event missing handler_thread and interacts_with");
    }

    [Fact]
    public void Schema_Rejects_Type_Without_Methods_Array()
    {
        var schema = LoadSchema();
        var ir = JsonDocument.Parse("""
        {
          "host": "test", "host_version": "1.0",
          "source": { "kind": "scrape", "urls": [], "extracted_at": "2026-01-01T00:00:00Z" },
          "types": [{
            "name": "T", "namespace": "N", "kind": "class",
            "summary": "s", "doc_url": "https://x.example/T",
            "interfaces": [], "constructors": [], "properties": [], "events": [], "enum_values": []
          }],
          "metadata": { "page_count": 0, "type_count": 1, "method_count": 0, "event_count": 0, "property_count": 0 }
        }
        """);
        var result = schema.Evaluate(ir.RootElement);
        Assert.False(result.IsValid, "schema should reject Type missing methods array (producer must emit [])");
    }
}
