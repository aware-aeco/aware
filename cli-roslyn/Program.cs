using System.Text.Json;
using System.Text.Json.Serialization;
using AwareReader;
using AwareRoslyn;

[assembly: System.Runtime.CompilerServices.InternalsVisibleTo("aware-roslyn.Tests")]

namespace AwareRoslyn;

/// <summary>
/// aware-roslyn entry point. One JSON request in on stdin, one JSON response out on stdout
/// (same envelope shape as aware-sidecar so the Rust CLI parses both identically). The only
/// op is <c>reflect-csharp</c>: reflect C# source (.cs / .csproj / .sln) → AwareReader IR →
/// AgentSynthesizer → GeneratedAgent.
/// </summary>
internal static class Program
{
    private const string Version = "0.1.0";

    private static readonly JsonSerializerOptions Json = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
        PropertyNameCaseInsensitive = true,
        DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull,
    };

    public static int Main()
    {
        try
        {
            var line = Console.In.ReadToEnd();
            if (string.IsNullOrWhiteSpace(line))
            {
                EmitError(null, "stdin closed with no input");
                return 2;
            }

            var envelope = JsonSerializer.Deserialize<RequestEnvelope>(line, Json);
            if (envelope is null || string.IsNullOrEmpty(envelope.Op))
            {
                EmitError(null, "could not parse request envelope");
                return 2;
            }

            if (envelope.Op != "reflect-csharp")
            {
                EmitError(envelope.Op, $"unknown op: {envelope.Op}");
                return 2;
            }

            var args = envelope.Args.Deserialize<ReflectCsharpArgs>(Json);
            if (args is null || args.Paths.Length == 0)
            {
                EmitError(envelope.Op, "reflect-csharp requires non-empty 'paths'");
                return 2;
            }

            try
            {
                var reflection = RoslynReader.ReflectPaths(args.Paths, args.References ?? Array.Empty<string>());
                var agent = AgentSynthesizer.Synthesize(
                    reflection.Set,
                    new SynthesisOptions
                    {
                        AgentIdOverride = args.AgentId,
                        DocSummaries = reflection.DocSummaries,
                    });
                EmitOk(envelope.Op, agent);
                return 0;
            }
            catch (Exception ex)
            {
                EmitError(envelope.Op, ex.Message);
                return 1;
            }
        }
        catch (JsonException ex)
        {
            EmitError(null, $"invalid JSON: {ex.Message}");
            return 2;
        }
        catch (Exception ex)
        {
            EmitError(null, $"unhandled: {ex.Message}");
            return 1;
        }
    }

    private static void EmitOk(string op, GeneratedAgent agent)
    {
        var resp = new OkResponse { Ok = true, Version = Version, Op = op, Data = new ResponseData { Agent = agent } };
        Console.WriteLine(JsonSerializer.Serialize(resp, Json));
    }

    private static void EmitError(string? op, string message)
    {
        var resp = new ErrorResponse { Ok = false, Version = Version, Op = op, Error = message };
        Console.WriteLine(JsonSerializer.Serialize(resp, Json));
    }
}

internal sealed class RequestEnvelope
{
    [JsonPropertyName("op")] public string Op { get; set; } = "";
    [JsonPropertyName("args")] public JsonElement Args { get; set; }
}

internal sealed class ReflectCsharpArgs
{
    // snake_case wire keys, matching the existing sidecar protocol (e.g. agent_id).
    [JsonPropertyName("paths")] public string[] Paths { get; set; } = Array.Empty<string>();
    [JsonPropertyName("references")] public string[]? References { get; set; }
    [JsonPropertyName("agent_id")] public string? AgentId { get; set; }
}

internal sealed class OkResponse
{
    public bool Ok { get; set; }
    public string Version { get; set; } = "";
    public string Op { get; set; } = "";
    public ResponseData? Data { get; set; }
}

internal sealed class ResponseData
{
    public GeneratedAgent? Agent { get; set; }
}

internal sealed class ErrorResponse
{
    public bool Ok { get; set; }
    public string Version { get; set; } = "";
    public string? Op { get; set; }
    public string Error { get; set; } = "";
}
