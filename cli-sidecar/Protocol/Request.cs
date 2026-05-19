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

public sealed class CoverageGenerateArgs
{
    [JsonPropertyName("ir_path")] public string IrPath { get; set; } = "";
    [JsonPropertyName("out_dir")] public string OutDir { get; set; } = "";
    [JsonPropertyName("agent_id")] public string AgentId { get; set; } = "";
    [JsonPropertyName("vendor")] public string Vendor { get; set; } = "";
    [JsonPropertyName("vertical")] public string Vertical { get; set; } = "";
}

/// <summary>
/// Coverage-extract verb args (Phase B). Drives a vendor extractor to crawl the
/// vendor's docs site / SDK and write a host-coverage IR JSON to disk.
/// </summary>
public sealed class CoverageExtractArgs
{
    [JsonPropertyName("vendor")] public string Vendor { get; set; } = "";
    [JsonPropertyName("version")] public string Version { get; set; } = "";
    [JsonPropertyName("out_path")] public string OutPath { get; set; } = "";
}

/// <summary>
/// Coverage-validate verb args (v0.30 alpha). Validates a host-coverage IR file
/// against the root schema and, optionally, every catalog/*.json under
/// <paramref name="AgentDir"/> against the per-Type schema.
///
/// Both schemas are resolved from embedded resources in the sidecar binary —
/// see <see cref="AwareSidecar.Ingest.CoverageValidator"/>. Pass null/empty
/// <c>SchemaRoot</c> / <c>SchemaType</c> to use the embedded versions.
/// </summary>
public sealed class CoverageValidateArgs
{
    [JsonPropertyName("ir_path")] public string IrPath { get; set; } = "";
    /// <summary>Override path for the root schema. Empty/null → use embedded resource.</summary>
    [JsonPropertyName("schema_root")] public string? SchemaRoot { get; set; }
    /// <summary>Override path for the per-Type schema. Empty/null → use embedded resource.</summary>
    [JsonPropertyName("schema_type")] public string? SchemaType { get; set; }
    /// <summary>If set, walk &lt;AgentDir&gt;/catalog/*.json and validate each against the Type schema.</summary>
    [JsonPropertyName("agent_dir")] public string? AgentDir { get; set; }
}
