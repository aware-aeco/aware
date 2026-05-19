// CoverageValidator — Draft 2020-12 JSON Schema validation for host-coverage
// IR files and catalog/*.json fragments.
//
// Wired by the sidecar's `coverage-validate` verb (v0.30 alpha). Replaces the
// out-of-tree `ajv@8` / Python `jsonschema` step in the codex-coverage review
// protocol (docs/superpowers/specs/host-coverage-review-protocol.md, Step 4).
//
// Schemas are read from embedded resources (cli-sidecar.csproj declares them
// via <EmbeddedResource>), so a single `aware-sidecar.exe` file is fully self-
// contained — no separate schema files need to ship alongside the binary. The
// caller may still pass explicit override paths via SchemaRoot / SchemaType
// (useful for the test suite, which sometimes wants to validate against a
// scratched-up schema fragment).
//
// AOT: this file is pure managed code. JsonSchema.Net evaluates schemas via
// JsonElement (no reflection on user types). Assembly.GetManifestResourceStream
// is constant-string-keyed and trims cleanly.

using System.Reflection;
using System.Text.Json;
using Json.Schema;

namespace AwareSidecar.Ingest;

public static class CoverageValidator
{
    /// <summary>
    /// Render the violations of a single Evaluate() call as
    /// "@&lt;instance-path&gt;: &lt;message&gt;" strings, walking the
    /// hierarchical results tree. Mirrors the rendering used by
    /// SchemaTests.Walk(...) so the on-screen output matches what the test
    /// suite shows on schema regressions.
    /// </summary>
    public static List<string> RenderViolations(EvaluationResults result)
    {
        var lines = new List<string>();
        void Walk(EvaluationResults r)
        {
            if (r.Errors is not null)
            {
                foreach (var (key, value) in r.Errors)
                {
                    lines.Add($"@{r.InstanceLocation}: {key} = {value}");
                }
            }
            if (r.Details is not null)
            {
                foreach (var d in r.Details)
                {
                    if (!d.IsValid) Walk(d);
                }
            }
        }
        Walk(result);
        return lines;
    }

    /// <summary>Load a schema from disk (when an explicit path is given) or from
    /// the embedded resource fallback (when the path is null/empty).</summary>
    static JsonSchema LoadSchema(string? overridePath, string embeddedName)
    {
        if (!string.IsNullOrEmpty(overridePath))
        {
            if (!File.Exists(overridePath))
                throw new FileNotFoundException($"schema not found: {overridePath}", overridePath);
            return JsonSchema.FromText(File.ReadAllText(overridePath));
        }
        var asm = typeof(CoverageValidator).Assembly;
        using var stream = asm.GetManifestResourceStream(embeddedName)
            ?? throw new InvalidOperationException(
                $"embedded resource '{embeddedName}' not found in {asm.GetName().Name}. " +
                "Did the cli-sidecar.csproj <EmbeddedResource> entries get removed?");
        using var reader = new StreamReader(stream);
        return JsonSchema.FromText(reader.ReadToEnd());
    }

    /// <summary>
    /// Validate an IR file against the root schema and (optionally) every
    /// catalog/*.json under <paramref name="agentDir"/> against the per-Type
    /// schema. <paramref name="schemaRootPath"/> and <paramref name="schemaTypePath"/>
    /// may be null/empty to use the embedded resources.
    /// </summary>
    public static Protocol.CoverageValidateResult Validate(
        string irPath,
        string? schemaRootPath,
        string? schemaTypePath,
        string? agentDir)
    {
        if (string.IsNullOrEmpty(irPath))
            throw new ArgumentException("ir_path is required", nameof(irPath));
        if (!File.Exists(irPath))
            throw new FileNotFoundException($"IR file not found: {irPath}", irPath);

        var rootSchema = LoadSchema(schemaRootPath, "host-coverage.schema.json");
        var options = new EvaluationOptions { OutputFormat = OutputFormat.Hierarchical };

        var irViolations = new List<string>();
        using (var doc = JsonDocument.Parse(File.ReadAllText(irPath)))
        {
            var result = rootSchema.Evaluate(doc.RootElement, options);
            if (!result.IsValid) irViolations = RenderViolations(result);
        }

        var catalogViolations = new Dictionary<string, List<string>>();
        int catalogValidated = 0;
        if (!string.IsNullOrEmpty(agentDir))
        {
            var catalogDir = Path.Combine(agentDir, "catalog");
            if (Directory.Exists(catalogDir))
            {
                var typeSchema = LoadSchema(schemaTypePath, "host-coverage-type.schema.json");
                foreach (var file in Directory.EnumerateFiles(catalogDir, "*.json", SearchOption.TopDirectoryOnly))
                {
                    catalogValidated++;
                    try
                    {
                        using var doc = JsonDocument.Parse(File.ReadAllText(file));
                        var result = typeSchema.Evaluate(doc.RootElement, options);
                        if (!result.IsValid)
                            catalogViolations[file] = RenderViolations(result);
                    }
                    catch (JsonException jx)
                    {
                        catalogViolations[file] = new List<string> { $"@: invalid JSON = {jx.Message}" };
                    }
                }
            }
        }

        return new Protocol.CoverageValidateResult
        {
            Ok = irViolations.Count == 0 && catalogViolations.Count == 0,
            IrPath = irPath,
            IrViolations = irViolations,
            CatalogViolations = catalogViolations,
            CatalogFilesValidated = catalogValidated,
        };
    }
}
