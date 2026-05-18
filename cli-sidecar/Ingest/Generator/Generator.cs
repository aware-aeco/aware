// Generator — orchestrate the three writers (Manifest / Skill / Catalog) into
// a complete on-disk agent layout under <out-dir>/.
//
// Inputs: a path to a host-coverage IR JSON file (validated by IRReader) plus
// the agent metadata fields required by the manifest (id / vendor / vertical).
// Outputs: <out>/manifest.yaml, <out>/skills/*.md, <out>/catalog/*.json.
//
// AOT note: this calls IRReader and CatalogWriter which both use reflection-
// based System.Text.Json. The annotations propagate up to callers (Program.cs
// dispatches with the same warning codes suppressed at the project level).
// The Generator itself is pure orchestration — no JSON serialization happens
// here, but the call graph requires the attributes anyway.

using System.Diagnostics.CodeAnalysis;

namespace AwareSidecar.Ingest;

public sealed record GenerateRequest(
    string IrPath,
    string OutputDir,
    string AgentId,
    string Vendor,
    string Vertical);

public sealed record GenerateResult(
    string ManifestPath,
    List<string> SkillPaths,
    List<string> CatalogPaths,
    int TotalArtefactsWritten);

public static class Generator
{
    [RequiresUnreferencedCode("Calls IRReader/CatalogWriter which use reflection-based JSON.")]
    [RequiresDynamicCode("Calls IRReader/CatalogWriter which may construct generic types at runtime.")]
    public static GenerateResult Generate(GenerateRequest req)
    {
        var ir = IRReader.ReadFromFile(req.IrPath);
        Directory.CreateDirectory(req.OutputDir);
        Directory.CreateDirectory(Path.Combine(req.OutputDir, "skills"));
        Directory.CreateDirectory(Path.Combine(req.OutputDir, "catalog"));

        var manifestPath = Path.Combine(req.OutputDir, "manifest.yaml");
        File.WriteAllText(manifestPath,
            ManifestWriter.Render(ir, req.AgentId, req.Vendor, req.Vertical));

        var skillPaths = new List<string>();
        foreach (var (name, body) in SkillWriter.RenderAll(ir))
        {
            var p = Path.Combine(req.OutputDir, "skills", name);
            File.WriteAllText(p, body);
            skillPaths.Add(p);
        }

        var catalogPaths = new List<string>();
        foreach (var (name, body) in CatalogWriter.RenderAll(ir))
        {
            var p = Path.Combine(req.OutputDir, "catalog", name);
            File.WriteAllText(p, body);
            catalogPaths.Add(p);
        }

        return new GenerateResult(
            manifestPath,
            skillPaths,
            catalogPaths,
            1 + skillPaths.Count + catalogPaths.Count);
    }
}
