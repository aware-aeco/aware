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
    public void Schema_Rejects_Method_Missing_Returns_Key()
    {
        // Defect 3 — codex-coverage FAIL 2026-05-19: System.Text.Json was dropping
        // the `returns` key entirely on void methods + ctors, but the schema requires
        // the key to be present (as null or as ReturnInfo). This test pins that the
        // schema actually does reject the omitted-key shape — otherwise the
        // Catalog_Json_Method/Ctor_Emits_Returns tests in CatalogWriterTests would be
        // testing a schema that no longer enforces the requirement.
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
              "throws": [], "events_related": [],
              "doc_url": "https://x.example/T.M"
            }]
          }],
          "metadata": { "page_count": 0, "type_count": 1, "method_count": 1, "event_count": 0, "property_count": 0 }
        }
        """);
        var result = schema.Evaluate(ir.RootElement);
        Assert.False(result.IsValid, "schema should reject Method that omits the `returns` key entirely (schema requires it as null or ReturnInfo)");
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

    /// <summary>
    /// Validate the Tedds-25 + Tedds-26 catalog files against the per-Type schema. A few
    /// canonical types are spot-checked per version (covering class / interface / enum).
    /// </summary>
    [Theory]
    [InlineData("tekla-tedds-25", "Tekla.Structural.ExpressoAddIn.AliasAttribute.json")]
    [InlineData("tekla-tedds-25", "Tekla.Structural.InteropAssemblies.Tedds.IApplication.json")]
    [InlineData("tekla-tedds-25", "Tekla.Structural.InteropAssemblies.Tedds.TdsCalcStatus.json")]
    [InlineData("tekla-tedds-26", "Tekla.Structural.ExpressoAddIn.AliasAttribute.json")]
    [InlineData("tekla-tedds-26", "Tekla.Structural.InteropAssemblies.Tedds.IApplication.json")]
    [InlineData("tekla-tedds-26", "Tekla.Structural.InteropAssemblies.Tedds.TdsCalcStatus.json")]
    public void Tedds_Catalog_Validates_Against_Type_Schema(string agentDir, string fileName)
    {
        ValidateCatalogFile("engineering", agentDir, fileName);
    }

    /// <summary>
    /// Spot-check the Bluebeam Studio API catalog files against the per-Type schema.
    /// Covers an *Api class, a Request DTO with one+ properties, and a Misc/Jobs-class
    /// reach so each top-level vertical of the IR is exercised.
    /// </summary>
    [Theory]
    [InlineData("bluebeam-v1", "Bluebeam.StudioApi.Sessions.SessionsApi.json")]
    [InlineData("bluebeam-v1", "Bluebeam.StudioApi.Sessions.Models.CreateSessionRequest.json")]
    [InlineData("bluebeam-v1", "Bluebeam.StudioApi.Jobs.ProjectFileJobsApi.json")]
    [InlineData("bluebeam-v1", "Bluebeam.StudioApi.Misc.MiscApi.json")]
    public void Bluebeam_Catalog_Validates_Against_Type_Schema(string agentDir, string fileName)
    {
        ValidateCatalogFile("architecture", agentDir, fileName);
    }

    /// <summary>
    /// Shared per-catalog-file schema validator. Resolves the file path by walking up
    /// from the test binary, then evaluates against the standalone Type schema.
    /// </summary>
    static void ValidateCatalogFile(string vertical, string agentDir, string fileName)
    {
        var schemaPath = Path.Combine(AppContext.BaseDirectory, "host-coverage-type.schema.json");
        if (!File.Exists(schemaPath))
        {
            // Walk up to find the schema if not copied next to test binary.
            var cursor = AppContext.BaseDirectory;
            for (int i = 0; i < 8 && cursor is not null; i++)
            {
                var probe = Path.Combine(cursor, "cli-sidecar", "Ingest", "Schema", "host-coverage-type.schema.json");
                if (File.Exists(probe)) { schemaPath = probe; break; }
                cursor = Path.GetDirectoryName(cursor);
            }
        }
        var schema = JsonSchema.FromText(File.ReadAllText(schemaPath));

        string? Find()
        {
            var cursor = AppContext.BaseDirectory;
            for (int i = 0; i < 8 && cursor is not null; i++)
            {
                var probe = Path.Combine(cursor, "20-agents", "aeco", vertical, agentDir, "catalog", fileName);
                if (File.Exists(probe)) return probe;
                cursor = Path.GetDirectoryName(cursor);
            }
            return null;
        }
        var path = Find();
        if (path is null) return;  // not generated yet — skip rather than fail.
        var fragment = JsonDocument.Parse(File.ReadAllText(path));
        var result = schema.Evaluate(fragment.RootElement, new EvaluationOptions { OutputFormat = OutputFormat.Hierarchical });
        if (!result.IsValid)
        {
            var sb = new System.Text.StringBuilder();
            sb.AppendLine($"Catalog fragment violations in {agentDir}/catalog/{fileName}:");
            void Walk(EvaluationResults r, int depth)
            {
                if (r.Errors is not null)
                {
                    foreach (var (k, v) in r.Errors)
                        sb.AppendLine(new string(' ', depth * 2) + $"@{r.InstanceLocation}: {k} = {v}");
                }
                if (r.Details is not null)
                {
                    foreach (var d in r.Details)
                        if (!d.IsValid) Walk(d, depth + 1);
                }
            }
            Walk(result, 0);
            Assert.Fail(sb.ToString());
        }
    }

    /// <summary>
    /// Validate the Bluebeam Studio API + Navisworks 2026 IRs against the root
    /// schema. The IRs are committed to cli-sidecar/Ingest/Output/ — if either
    /// extractor's output shape ever regresses against the schema, this catches
    /// it before commit.
    /// </summary>
    [Theory]
    [InlineData("bluebeam-v1.ir.json")]
    [InlineData("navisworks-2026.0.ir.json")]
    public void Bluebeam_Navisworks_IR_Validates_Against_Schema(string fileName)
    {
        var schema = LoadSchema();
        string? Find(string here)
        {
            string? cursor = here;
            for (int i = 0; i < 8 && cursor is not null; i++)
            {
                var probe = Path.Combine(cursor, "cli-sidecar", "Ingest", "Output", fileName);
                if (File.Exists(probe)) return probe;
                cursor = Path.GetDirectoryName(cursor);
            }
            return null;
        }
        var path = Find(AppContext.BaseDirectory);
<<<<<<< HEAD
        if (path is null) return;  // IR not generated yet — skip rather than fail.
=======
        if (path is null) return;
>>>>>>> worktree-agent-a5a9cc409d6da5342
        var ir = JsonDocument.Parse(File.ReadAllText(path));
        var result = schema.Evaluate(ir.RootElement, new EvaluationOptions { OutputFormat = OutputFormat.Hierarchical });
        if (!result.IsValid)
        {
            var sb = new System.Text.StringBuilder();
            sb.AppendLine($"Schema violations in {fileName}:");
            void Walk(EvaluationResults r, int depth)
            {
                if (r.Errors is not null)
                {
                    foreach (var (k, v) in r.Errors)
                        sb.AppendLine(new string(' ', depth * 2) + $"@{r.InstanceLocation}: {k} = {v}");
                }
                if (r.Details is not null)
                {
                    foreach (var d in r.Details)
                        if (!d.IsValid) Walk(d, depth + 1);
                }
            }
            Walk(result, 0);
            Assert.Fail(sb.ToString());
        }
    }

    /// <summary>
    /// Validate the Tedds-25 and Tedds-26 IRs against the schema. These are committed to
    /// cli-sidecar/Ingest/Output/ — when the parser changes shape they get re-generated
    /// and this test catches schema regressions before they ship.
    /// </summary>
    [Theory]
    [InlineData("tedds-25.0.ir.json")]
    [InlineData("tedds-26.0.ir.json")]
    public void Tedds_IR_Validates_Against_Schema(string fileName)
    {
        var schema = LoadSchema();
        // The IR files live in cli-sidecar/Ingest/Output/. Walk up from the test binary
        // location to find them; CI + dev layouts both work with this fallback.
        string? Find(string here)
        {
            string? cursor = here;
            for (int i = 0; i < 8 && cursor is not null; i++)
            {
                var probe = Path.Combine(cursor, "cli-sidecar", "Ingest", "Output", fileName);
                if (File.Exists(probe)) return probe;
                cursor = Path.GetDirectoryName(cursor);
            }
            return null;
        }
        var path = Find(AppContext.BaseDirectory);
        if (path is null)
        {
            // IR file not produced yet (e.g. fresh clone). Skip rather than fail —
            // the IR is regenerated by the extractor before commit.
            return;
        }
        var ir = JsonDocument.Parse(File.ReadAllText(path));
        var result = schema.Evaluate(ir.RootElement, new EvaluationOptions { OutputFormat = OutputFormat.Hierarchical });
        if (!result.IsValid)
        {
            var sb = new System.Text.StringBuilder();
            sb.AppendLine($"Schema violations in {fileName}:");
            void Walk(EvaluationResults r, int depth)
            {
                if (r.Errors is not null)
                {
                    foreach (var (k, v) in r.Errors)
                        sb.AppendLine(new string(' ', depth * 2) + $"@{r.InstanceLocation}: {k} = {v}");
                }
                if (r.Details is not null)
                {
                    foreach (var d in r.Details)
                        if (!d.IsValid) Walk(d, depth + 1);
                }
            }
            Walk(result, 0);
            Assert.Fail(sb.ToString());
        }
    }
}
