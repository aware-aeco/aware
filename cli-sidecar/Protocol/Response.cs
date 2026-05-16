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
}

public sealed class ErrorResponse
{
    [JsonPropertyName("ok")] public bool Ok { get; set; } = false;
    [JsonPropertyName("version")] public string Version { get; set; } = "";
    [JsonPropertyName("op")] public string? Op { get; set; }
    [JsonPropertyName("error")] public string Error { get; set; } = "";
}
