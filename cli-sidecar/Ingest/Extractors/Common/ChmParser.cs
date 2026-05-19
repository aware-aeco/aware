// ChmParser — shared helper used by every vendor extractor whose source-of-truth
// is a Microsoft Compiled HTML Help (CHM) file. Autodesk ships its Revit, AutoCAD,
// and Navisworks SDK API references as CHMs, so any Tier-B extractor (DLL-driven
// vendor with CHM-shipped docs) decompiles via this class first.
//
// Strategy: shell out to `hh.exe -decompile <dir> <chm>`, the Windows OS built-in
// HTML Help decompiler. It writes the constituent .htm/.html files into <dir>.
// We then enumerate the directory and yield (filename, html) tuples for downstream
// per-vendor parsers — the same shape as Scraper's output, so the IRBuilder pipeline
// doesn't care whether the source was web or CHM.
//
// Windows-only. `hh.exe` is shipped with every supported Windows SKU; the binary
// has no Linux/macOS equivalent. Extractors that use this helper document the same
// platform constraint, and their tests skip cleanly on non-Windows runners.
//
// Re-decompile idempotency: each unique CHM gets its own extract dir under %TEMP%
// (named by basename), and `hh.exe` overwrites existing files on each run. We don't
// purge the dir between runs — if you find stale-file bugs in downstream parsers,
// add `Directory.Delete(_extractDir, recursive: true)` before the CreateDirectory
// call below.
//
// Encoding: `File.ReadAllText` defaults to UTF-8 with BOM detection. Autodesk CHMs
// are usually Windows-1252 encoded; non-ASCII glyphs may decode incorrectly here.
// For A8 we ship the simple variant; vendor extractors that hit garbled output can
// swap to `File.ReadAllBytes` + an explicit encoding in their own wrapper.
//
// NativeAOT: only uses System.Diagnostics.Process and System.IO. Both are fully
// AOT-compatible — no reflection, no source generators required.

using System.Diagnostics;

namespace AwareSidecar.Ingest.Extractors.Common;

/// <summary>
/// Decompiles a CHM (Microsoft Compiled HTML Help) file into a directory of
/// HTML pages using `hh.exe -decompile` (Windows-built-in tool).
/// Then yields the (filename, html) tuples for downstream parsing.
/// </summary>
public sealed class ChmParser
{
    readonly string _chmPath;
    readonly string _extractDir;

    public ChmParser(string chmPath)
    {
        _chmPath = chmPath ?? throw new ArgumentNullException(nameof(chmPath));
        _extractDir = Path.Combine(Path.GetTempPath(), "aware-chm-" + Path.GetFileNameWithoutExtension(chmPath));
    }

    public IEnumerable<(string Filename, string Html)> Decompile()
    {
        Directory.CreateDirectory(_extractDir);
        var psi = new ProcessStartInfo("hh.exe", $"-decompile \"{_extractDir}\" \"{_chmPath}\"")
        {
            UseShellExecute = false,
            CreateNoWindow = true,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
        };
        var p = Process.Start(psi)!;
        p.WaitForExit(120_000);
        if (p.ExitCode != 0)
            throw new InvalidOperationException(
                $"hh.exe decompile failed (exit {p.ExitCode}): {p.StandardError.ReadToEnd()}");

        foreach (var html in Directory.GetFiles(_extractDir, "*.htm*", SearchOption.AllDirectories))
            yield return (Path.GetFileName(html), File.ReadAllText(html));
    }
}
