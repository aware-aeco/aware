using System.Text.Json;
using System.Text.Json.Serialization;

namespace AwareSidecar;

// ---------------------------------------------------------------------------
// Source-generation context — required for NativeAOT / trim-safe serialization.
// Add every top-level response/error type here.
// ---------------------------------------------------------------------------
[JsonSerializable(typeof(OkResponse))]
[JsonSerializable(typeof(ErrorResponse))]
[JsonSourceGenerationOptions(WriteIndented = false, PropertyNamingPolicy = JsonKnownNamingPolicy.CamelCase)]
internal partial class SidecarJsonContext : JsonSerializerContext { }

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------
internal sealed class OkResponse
{
    public bool Ok { get; init; } = true;
    public string Version { get; init; } = Program.Version;
    public string Op { get; init; } = "";
    public EchoData Data { get; init; } = new();
}

internal sealed class EchoData
{
    public string Note { get; init; } = "";
    // Raw JSON element — serialized as-is; JsonElement is handled natively by STJ.
    public JsonElement Input { get; init; }
}

internal sealed class ErrorResponse
{
    public bool Ok { get; init; } = false;
    public string Version { get; init; } = Program.Version;
    public string Error { get; init; } = "";
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------
internal static class Program
{
    internal const string Version = "0.5.1";

    public static int Main(string[] args)
    {
        try
        {
            // Read one JSON line from stdin
            string? line = Console.In.ReadLine();
            if (string.IsNullOrWhiteSpace(line))
            {
                WriteError("stdin closed with no input");
                return 2;
            }

            // For Task 1: parse the input as a JSON object and echo it back inside a response envelope.
            // Real op dispatch comes in Tasks 2-5.
            using JsonDocument doc = JsonDocument.Parse(line);
            string op = doc.RootElement.TryGetProperty("op", out JsonElement opEl)
                ? (opEl.GetString() ?? "unknown")
                : "unknown";

            OkResponse response = new()
            {
                Op = op,
                Data = new EchoData
                {
                    Note = "task-1 scaffold: echoing input. Real handlers land in tasks 2-5.",
                    Input = doc.RootElement.Clone(),
                },
            };

            Console.WriteLine(JsonSerializer.Serialize(response, SidecarJsonContext.Default.OkResponse));
            return 0;
        }
        catch (Exception ex)
        {
            WriteError($"unhandled: {ex.Message}");
            return 1;
        }
    }

    private static void WriteError(string message)
    {
        ErrorResponse resp = new() { Error = message };
        Console.WriteLine(JsonSerializer.Serialize(resp, SidecarJsonContext.Default.ErrorResponse));
    }
}
