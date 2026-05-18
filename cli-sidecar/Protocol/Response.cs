using System.Text.Json.Serialization;

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
