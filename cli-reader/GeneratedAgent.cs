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
}

public sealed class GeneratedSkill
{
    [JsonPropertyName("filename")] public string Filename { get; set; } = "";
    [JsonPropertyName("body")] public string Body { get; set; } = "";
}
