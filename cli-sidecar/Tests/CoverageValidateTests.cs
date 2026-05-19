// Unit tests for the in-process CoverageValidator that backs the sidecar
// `coverage-validate` verb. These bypass the stdin/stdout envelope and exercise
// the static Validate(...) entry point directly — the verb-level switch in
// Program.cs is a thin wrapper that we exercise via the Rust integration test
// suite (cli/tests/coverage_validate.rs).

using System.IO;
using AwareSidecar.Ingest;
using Xunit;

namespace AwareSidecar.Tests;

public class CoverageValidateTests
{
    /// <summary>Resolve the minimal IR fixture used by the Generator tests.</summary>
    static string MinimalIrFixturePath()
    {
        string? cursor = AppContext.BaseDirectory;
        for (int i = 0; i < 8 && cursor is not null; i++)
        {
            var probe = Path.Combine(cursor, "cli-sidecar", "Ingest", "Generator", "Tests", "fixtures", "minimal.ir.json");
            if (File.Exists(probe)) return probe;
            cursor = Path.GetDirectoryName(cursor);
        }
        throw new FileNotFoundException("cannot locate minimal.ir.json fixture");
    }

    [Fact]
    public void Validates_Minimal_Ir_Against_Embedded_Schema()
    {
        // No SchemaRoot supplied → falls back to the embedded resource. This is
        // the canonical path the codex-coverage reviewer will use.
        var result = CoverageValidator.Validate(
            irPath: MinimalIrFixturePath(),
            schemaRootPath: null,
            schemaTypePath: null,
            agentDir: null);

        Assert.True(result.Ok, $"expected ok=true, got violations: {string.Join("; ", result.IrViolations)}");
        Assert.Empty(result.IrViolations);
        Assert.Empty(result.CatalogViolations);
        Assert.Equal(0, result.CatalogFilesValidated);
    }

    [Fact]
    public void Reports_Violations_When_Ir_Is_Broken()
    {
        // Hand-craft an IR missing required top-level fields. The root schema
        // requires host, host_version, source, types, metadata; an empty object
        // should produce 5 "required" violations.
        var tmp = Path.GetTempFileName();
        try
        {
            File.WriteAllText(tmp, "{}");
            var result = CoverageValidator.Validate(tmp, null, null, null);
            Assert.False(result.Ok);
            Assert.NotEmpty(result.IrViolations);
            // The hierarchical walker emits "required" errors at the root path.
            Assert.Contains(result.IrViolations, line => line.Contains("required"));
        }
        finally { File.Delete(tmp); }
    }

    [Fact]
    public void Reports_Violations_When_Method_Returns_Key_Omitted()
    {
        // Pin the regression that motivated Defect 3 in Phase B: System.Text.Json
        // was dropping `returns` on void methods + ctors, but the schema requires
        // the key to be present (as null or as ReturnInfo). Walk the ladder of
        // checks here so the validator catches the same regression at the CLI
        // level, not just inside the Generator tests.
        var tmp = Path.GetTempFileName();
        try
        {
            File.WriteAllText(tmp, """
            {
              "host": "demo", "host_version": "1.0",
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
            var result = CoverageValidator.Validate(tmp, null, null, null);
            Assert.False(result.Ok);
            Assert.NotEmpty(result.IrViolations);
        }
        finally { File.Delete(tmp); }
    }

    [Fact]
    public void Missing_Ir_File_Throws_FileNotFound()
    {
        Assert.Throws<FileNotFoundException>(() =>
            CoverageValidator.Validate(
                irPath: "C:/aware-test-does-not-exist-12345.ir.json",
                schemaRootPath: null,
                schemaTypePath: null,
                agentDir: null));
    }

    [Fact]
    public void Catalog_Validation_Walks_Catalog_Subdirectory()
    {
        // Build a temp "agent" with one bogus catalog/Foo.json. Expect one
        // catalog file validated, one entry in CatalogViolations, ok=false.
        var tmp = Path.Combine(Path.GetTempPath(), $"aware-cv-{Guid.NewGuid():N}");
        var catalog = Path.Combine(tmp, "catalog");
        Directory.CreateDirectory(catalog);
        try
        {
            // IR is fine (minimal fixture) — only the catalog has issues.
            var bogus = Path.Combine(catalog, "Bogus.json");
            File.WriteAllText(bogus, """{"name":"X"}"""); // missing required Type fields

            var result = CoverageValidator.Validate(
                irPath: MinimalIrFixturePath(),
                schemaRootPath: null,
                schemaTypePath: null,
                agentDir: tmp);

            Assert.False(result.Ok);
            Assert.Empty(result.IrViolations);
            Assert.Equal(1, result.CatalogFilesValidated);
            Assert.Single(result.CatalogViolations);
            Assert.Contains(result.CatalogViolations, kv => kv.Key.EndsWith("Bogus.json"));
        }
        finally
        {
            try { Directory.Delete(tmp, recursive: true); } catch { /* best-effort */ }
        }
    }

    [Fact]
    public void Catalog_Validation_With_Missing_Catalog_Dir_Is_Noop()
    {
        // AgentDir is supplied but no catalog/ subdir exists → 0 files
        // validated, no violations, IR-only validation still runs.
        var tmp = Path.Combine(Path.GetTempPath(), $"aware-cv-noop-{Guid.NewGuid():N}");
        Directory.CreateDirectory(tmp);
        try
        {
            var result = CoverageValidator.Validate(
                irPath: MinimalIrFixturePath(),
                schemaRootPath: null,
                schemaTypePath: null,
                agentDir: tmp);

            Assert.True(result.Ok);
            Assert.Equal(0, result.CatalogFilesValidated);
            Assert.Empty(result.CatalogViolations);
        }
        finally
        {
            try { Directory.Delete(tmp, recursive: true); } catch { /* best-effort */ }
        }
    }
}
