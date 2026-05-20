// Enumerates running Revit instances and reshapes them for list-instances
// and verb-target selection. Pure logic is in static helpers so tests can run
// without a Revit install.

using System.Diagnostics;
using System.Text.RegularExpressions;

namespace AwareRevit.Sidecar;

internal sealed record RevitInstance(int Pid, string Version, string ExePath, bool AddinLoaded);

internal static class RevitProcessFinder
{
    /// <summary>Returns every Revit.exe currently running, with version parsed
    /// from MainModule.FileName. AddinLoaded is set later by probing the pipe.</summary>
    public static List<RevitInstance> Enumerate()
    {
        var result = new List<RevitInstance>();
        foreach (var p in Process.GetProcessesByName("Revit"))
        {
            try
            {
                var path = p.MainModule?.FileName;
                if (path is null) continue;
                var version = ExtractVersionFromPath(path);
                if (version is null) continue;
                result.Add(new RevitInstance(p.Id, version, path, AddinLoaded: false));
            }
            catch
            {
                // Inaccessible process (e.g. cross-session) — skip silently.
            }
        }
        return result;
    }

    /// <summary>Parse "Revit 2026" out of an exe path like
    /// "C:\Program Files\Autodesk\Revit 2026\Revit.exe". Returns "2026" or null.</summary>
    public static string? ExtractVersionFromPath(string path)
    {
        // Normalise to forward slashes for the regex.
        var m = Regex.Match(path.Replace('\\', '/'),
            @"/Autodesk/Revit (\d{4})/", RegexOptions.IgnoreCase);
        return m.Success ? m.Groups[1].Value : null;
    }

    /// <summary>Filter a list by version (exact) and/or pid (exact).
    /// Empty criteria → return everything.</summary>
    public static List<RevitInstance> Filter(List<RevitInstance> all, string? version, int? pid)
    {
        IEnumerable<RevitInstance> q = all;
        if (pid is { } p) q = q.Where(i => i.Pid == p);
        if (!string.IsNullOrEmpty(version)) q = q.Where(i => i.Version == version);
        return q.ToList();
    }

    /// <summary>Per-Revit-process pipe name convention.</summary>
    public static string PipeNameFor(int pid) => $"aware-revit-{pid}";

    /// <summary>Probe whether AwareRevit.dll is loaded by trying to connect to
    /// the per-PID pipe with a tiny timeout. Returns the input instance with
    /// AddinLoaded updated; safe to call without pinging Revit twice.</summary>
    public static async Task<RevitInstance> ProbeAddinAsync(RevitInstance instance,
                                                            TimeSpan probe = default)
    {
        if (probe == default) probe = TimeSpan.FromMilliseconds(300);
        var name = PipeNameFor(instance.Pid);
        try
        {
            using var pipe = new System.IO.Pipes.NamedPipeClientStream(
                ".", name, System.IO.Pipes.PipeDirection.InOut,
                System.IO.Pipes.PipeOptions.Asynchronous);
            await pipe.ConnectAsync((int)probe.TotalMilliseconds);
            return instance with { AddinLoaded = true };
        }
        catch
        {
            return instance with { AddinLoaded = false };
        }
    }
}
