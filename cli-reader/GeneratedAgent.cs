using System.Text.Json.Serialization;

namespace AwareReader;

// Moved from cli-sidecar/Protocol/Agent.cs (#180): the agent shape the build pipeline
// emits is now produced by the shared AgentSynthesizer in this library, so it lives
// alongside the IR + synthesizer rather than in the sidecar's protocol layer. The
// sidecar's SidecarJsonContext still source-generates the JSON for these types via
// `using AwareReader;`.

public sealed class GeneratedAgent
{
    [JsonPropertyName("id")] public string Id { get; set; } = "";
    [JsonPropertyName("version")] public string Version { get; set; } = "0.1.0";
    [JsonPropertyName("description")] public string Description { get; set; } = "";
    [JsonPropertyName("license")] public string License { get; set; } = "UNKNOWN";
    [JsonPropertyName("stateful")] public bool Stateful { get; set; }
    [JsonPropertyName("commands")] public GeneratedCommand[] Commands { get; set; } = Array.Empty<GeneratedCommand>();
    [JsonPropertyName("skills")] public GeneratedSkill[] Skills { get; set; } = Array.Empty<GeneratedSkill>();
}

public sealed class GeneratedCommand
{
    [JsonPropertyName("name")] public string Name { get; set; } = "";
    [JsonPropertyName("lifecycle")] public string Lifecycle { get; set; } = "single";
    [JsonPropertyName("description")] public string Description { get; set; } = "";
    /// <summary>Explicit read/write mode (e.g. "write" for recipe insert commands). Null = infer from name.</summary>
    [JsonPropertyName("mode")] public string? Mode { get; set; }
    /// <summary>Declared inputs (recipe commands carry their parameter contract). Empty for plain commands.</summary>
    [JsonPropertyName("inputs")] public GeneratedInput[] Inputs { get; set; } = Array.Empty<GeneratedInput>();
}

/// <summary>One declared command input. <c>Default</c> is a literal string (never boxed object) for AOT safety.</summary>
public sealed class GeneratedInput
{
    [JsonPropertyName("name")] public string Name { get; set; } = "";
    [JsonPropertyName("type")] public string Type { get; set; } = "";
    [JsonPropertyName("optional")] public bool Optional { get; set; }
    [JsonPropertyName("default")] public string? Default { get; set; }
}

public sealed class GeneratedSkill
{
    [JsonPropertyName("filename")] public string Filename { get; set; } = "";
    [JsonPropertyName("body")] public string Body { get; set; } = "";
}
