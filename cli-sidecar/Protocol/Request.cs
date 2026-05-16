using System.Text.Json.Serialization;

namespace AwareSidecar.Protocol;

/// <summary>Outer envelope read off stdin.</summary>
public sealed class RequestEnvelope
{
    [JsonPropertyName("op")] public string Op { get; set; } = "";
    [JsonPropertyName("args")] public System.Text.Json.JsonElement Args { get; set; }
}

public sealed class ReflectDllsArgs
{
    [JsonPropertyName("globs")] public string[] Globs { get; set; } = Array.Empty<string>();
    [JsonPropertyName("agent_id")] public string? AgentId { get; set; }
}

public sealed class DecompileArgs
{
    [JsonPropertyName("package_path")] public string PackagePath { get; set; } = "";
    [JsonPropertyName("version")] public string Version { get; set; } = "";
    [JsonPropertyName("agent_id")] public string? AgentId { get; set; }
    [JsonPropertyName("accept_license")] public bool AcceptLicense { get; set; }
}

public sealed class FromComArgs
{
    [JsonPropertyName("progid")] public string ProgId { get; set; } = "";
    [JsonPropertyName("agent_id")] public string? AgentId { get; set; }
}

public sealed class FromHeadersArgs
{
    [JsonPropertyName("files")] public string[] Files { get; set; } = Array.Empty<string>();
    [JsonPropertyName("agent_id")] public string? AgentId { get; set; }
}
