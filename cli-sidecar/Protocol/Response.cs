using System.Text.Json.Serialization;
using AwareReader;

namespace AwareSidecar.Protocol;

public sealed class OkResponse
{
    [JsonPropertyName("ok")] public bool Ok { get; set; } = true;
    [JsonPropertyName("version")] public string Version { get; set; } = "";
    [JsonPropertyName("op")] public string Op { get; set; } = "";
    [JsonPropertyName("data")] public ResponseData? Data { get; set; }
}

public sealed class ResponseData
{
    [JsonPropertyName("agent")] public GeneratedAgent? Agent { get; set; }
    [JsonPropertyName("coverage")] public CoverageGenerateResult? Coverage { get; set; }
    /// <summary>
    /// Populated by the `coverage-validate` verb (v0.30 alpha). Carries the IR
    /// validation status + per-catalog-file violations.
    /// </summary>
    [JsonPropertyName("coverage_validate")] public CoverageValidateResult? CoverageValidate { get; set; }
}

public sealed class CoverageGenerateResult
{
    [JsonPropertyName("manifest")] public string Manifest { get; set; } = "";
    [JsonPropertyName("skill_count")] public int SkillCount { get; set; }
    [JsonPropertyName("catalog_count")] public int CatalogCount { get; set; }
    [JsonPropertyName("total")] public int Total { get; set; }
    [JsonPropertyName("skills")] public string[] Skills { get; set; } = Array.Empty<string>();
    [JsonPropertyName("catalogs")] public string[] Catalogs { get; set; } = Array.Empty<string>();
}

public sealed class ErrorResponse
{
    [JsonPropertyName("ok")] public bool Ok { get; set; } = false;
    [JsonPropertyName("version")] public string Version { get; set; } = "";
    [JsonPropertyName("op")] public string? Op { get; set; }
    [JsonPropertyName("error")] public string Error { get; set; } = "";
}

/// <summary>
/// Result of `coverage-validate`. <see cref="Ok"/> is true iff zero IR
/// violations AND zero catalog violations. Violations are rendered as
/// "@&lt;instance-path&gt;: &lt;message&gt;" strings, matching the protocol
/// spec's expected output format.
/// </summary>
public sealed class CoverageValidateResult
{
    [JsonPropertyName("ok")] public bool Ok { get; set; }
    [JsonPropertyName("ir_path")] public string IrPath { get; set; } = "";
    [JsonPropertyName("ir_violations")] public List<string> IrViolations { get; set; } = new();
    /// <summary>
    /// Per-catalog-file violations, keyed by the catalog file path (absolute
    /// or as-given). Empty if no AgentDir was supplied.
    /// </summary>
    [JsonPropertyName("catalog_violations")] public Dictionary<string, List<string>> CatalogViolations { get; set; } = new();
    [JsonPropertyName("catalog_files_validated")] public int CatalogFilesValidated { get; set; }
}
