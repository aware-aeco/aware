// DTO sent by aware-revit.exe over the named pipe into AwareRevit.dll.
// Shape matches the AWARE stdin-JSON contract minus the `verb` (the pipe is
// per-verb dispatched on the sidecar side; the add-in only ever runs exec).
namespace AwareRevit.Shared;

public sealed class ExecRequest
{
    public string Id { get; set; } = "";
    public string Code { get; set; } = "";
    public Dictionary<string, object?> Args { get; set; } = new();
    /// <summary>One of: "none" (default), "auto" (wrap in Transaction).</summary>
    public string Transaction { get; set; } = "none";
    public int TimeoutSeconds { get; set; } = 60;
}
