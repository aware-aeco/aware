using System.Diagnostics;
using System.IO.Compression;
using AwareSidecar.Protocol;
using AwareReader;

namespace AwareSidecar.Decompile;

public static class IlspyShell
{
    private static readonly HashSet<string> PermissiveLicenses = new(StringComparer.OrdinalIgnoreCase)
    {
        "MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause", "Unlicense", "ISC", "0BSD"
    };

    /// <summary>Decompile a .nupkg's DLLs via ilspycmd, emit an agent.</summary>
    /// <exception cref="InvalidOperationException">ilspycmd missing or license blocked.</exception>
    public static GeneratedAgent Decompile(string packagePath, string version, string? agentIdOverride, bool acceptLicense)
    {
        if (!File.Exists(packagePath))
            throw new InvalidOperationException($"nupkg not found: {packagePath}");

        if (!IsIlspyAvailable())
            throw new InvalidOperationException("ilspycmd not found on PATH. Install with: dotnet tool install -g ilspycmd");

        // Extract nupkg to a scratch dir
        var scratch = Path.Combine(Path.GetTempPath(), $"aware-decompile-{Guid.NewGuid():N}");
        Directory.CreateDirectory(scratch);
        try
        {
            ZipFile.ExtractToDirectory(packagePath, scratch);

            // Find + check license from .nuspec
            var nuspecFile = Directory.EnumerateFiles(scratch, "*.nuspec").FirstOrDefault();
            string license = "UNKNOWN";
            string? description = null;
            string? pkgId = null;
            if (nuspecFile != null)
            {
                var nuspecText = File.ReadAllText(nuspecFile);
                license = ExtractLicense(nuspecText);
                description = ExtractDescription(nuspecText);
                pkgId = ExtractId(nuspecText);
            }
            if (!PermissiveLicenses.Contains(license) && !acceptLicense)
            {
                throw new InvalidOperationException($"package license {license} is not permissive. Re-run with --accept-license to proceed.");
            }

            // Find DLLs under lib/*/
            var libDir = Path.Combine(scratch, "lib");
            var dllFiles = Directory.Exists(libDir)
                ? Directory.EnumerateFiles(libDir, "*.dll", SearchOption.AllDirectories).ToList()
                : new List<string>();

            // Decompile each DLL
            var skills = new List<GeneratedSkill>();
            foreach (var dll in dllFiles)
            {
                var decompiled = RunIlspyCmd(dll);
                if (string.IsNullOrWhiteSpace(decompiled)) continue;
                var stem = Path.GetFileNameWithoutExtension(dll).ToLowerInvariant();
                var skillName = $"{stem}-decompiled.md";
                var body = BuildDecompileSkillBody(pkgId?.ToLowerInvariant() ?? "decompiled", stem, Path.GetFileName(dll), license, decompiled);
                skills.Add(new GeneratedSkill { Filename = skillName, Body = body });
            }

            var id = agentIdOverride ?? pkgId?.ToLowerInvariant() ?? "decompiled-pkg";
            return new GeneratedAgent
            {
                Id = id,
                Version = version,
                Description = description ?? pkgId ?? "Decompiled NuGet package",
                License = license,
                Stateful = false,
                Commands = Array.Empty<GeneratedCommand>(),
                Skills = skills.OrderBy(s => s.Filename, StringComparer.Ordinal).ToArray(),
            };
        }
        finally
        {
            try { Directory.Delete(scratch, recursive: true); } catch { /* best effort */ }
        }
    }

    internal static string BuildDecompileSkillBody(string pkgId, string stem, string filename, string license, string decompiled)
    {
        return
            $"---\n" +
            $"name: {pkgId}-{stem}\n" +
            $"description: Decompiled API surface from {filename} (license: {license})\n" +
            $"next-action: aware skill modify {pkgId} {stem}-decompiled\n" +
            $"---\n" +
            "\n" +
            $"# {stem} (decompiled)\n" +
            "\n" +
            "> ℹ️ This skill contains raw decompiled C#. To summarize it into\n" +
            $"> structured AWARE-style notes, run `aware skill modify {pkgId} {stem}-decompiled`\n" +
            "> from your agentic CLI (Claude Code / Codex). Ask your AI to read the raw\n" +
            "> decompiled output below and rewrite this skill as a normal AWARE skill\n" +
            "> (rule first, then explain) following the `agent-spec.md` conventions.\n" +
            "\n" +
            "## Raw decompiled output\n" +
            "\n" +
            "```csharp\n" +
            $"{decompiled}\n" +
            "```\n";
    }

    private static bool IsIlspyAvailable()
    {
        try
        {
            var psi = new ProcessStartInfo("ilspycmd", "--help")
            {
                RedirectStandardOutput = true,
                RedirectStandardError = true,
                UseShellExecute = false,
            };
            using var p = Process.Start(psi);
            if (p == null) return false;
            p.WaitForExit(5000);
            return p.ExitCode == 0 || p.ExitCode == 1; // some CLIs return 1 for --help
        }
        catch { return false; }
    }

    private static string RunIlspyCmd(string dllPath)
    {
        var psi = new ProcessStartInfo("ilspycmd", $"\"{dllPath}\"")
        {
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            UseShellExecute = false,
        };
        using var p = Process.Start(psi);
        if (p == null) return "";
        var output = p.StandardOutput.ReadToEnd();
        p.WaitForExit(60000);
        return output;
    }

    private static string ExtractLicense(string nuspec)
    {
        var idx = nuspec.IndexOf("<license", StringComparison.Ordinal);
        if (idx < 0) return "UNKNOWN";
        var end = nuspec.IndexOf("</license>", idx, StringComparison.Ordinal);
        if (end < 0) return "UNKNOWN";
        var chunk = nuspec.Substring(idx, end - idx);
        var contentStart = chunk.IndexOf('>');
        if (contentStart < 0) return "UNKNOWN";
        return chunk.Substring(contentStart + 1).Trim();
    }

    private static string? ExtractDescription(string nuspec)
    {
        var start = nuspec.IndexOf("<description>", StringComparison.Ordinal);
        if (start < 0) return null;
        var end = nuspec.IndexOf("</description>", start, StringComparison.Ordinal);
        if (end < 0) return null;
        return nuspec.Substring(start + 13, end - start - 13).Trim();
    }

    private static string? ExtractId(string nuspec)
    {
        var start = nuspec.IndexOf("<id>", StringComparison.Ordinal);
        if (start < 0) return null;
        var end = nuspec.IndexOf("</id>", start, StringComparison.Ordinal);
        if (end < 0) return null;
        return nuspec.Substring(start + 4, end - start - 4).Trim();
    }
}
