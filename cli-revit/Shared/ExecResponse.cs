// DTO returned by AwareRevit.dll over the named pipe to aware-revit.exe.
// The sidecar wraps it in the outer AWARE receipt envelope (adds host="revit",
// verb, delivered_at). Keep this dumb — no transport concerns.
namespace AwareRevit.Shared;

public sealed class ExecResponse
{
    public string Id { get; set; } = "";
    public bool Ok { get; set; }
    /// <summary>JSON-shaped result. Serialized by the add-in's ResultSerializer.</summary>
    public object? Result { get; set; }
    public string? Error { get; set; }
    public string? Stack { get; set; }
    public string StdoutLog { get; set; } = "";
    public string? HostVersion { get; set; }
    public int? HostPid { get; set; }
}
