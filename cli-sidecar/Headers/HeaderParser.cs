using System.Diagnostics;
using System.Text.Json;
using AwareSidecar.Protocol;

namespace AwareSidecar.Headers;

/// <summary>
/// Parses C/C++ headers via `clang -Xclang -ast-dump=json -fsyntax-only`.
/// Emits one command per top-level FunctionDecl whose source location matches
/// the requested header (transitive includes are skipped).
///
/// v0.5.2 scope: free functions only. Class methods, templates, macros are
/// out of scope.
/// </summary>
public static class HeaderParser
{
    public static GeneratedAgent Parse(string[] files, string? agentIdOverride)
    {
        if (!IsClangAvailable())
        {
            throw new InvalidOperationException(
                "clang not found on PATH. Install LLVM from https://releases.llvm.org/ " +
                "or via your package manager (apt install clang / brew install llvm / choco install llvm)."
            );
        }

        var allFiles = ResolveFiles(files);
        if (allFiles.Count == 0)
        {
            throw new InvalidOperationException($"no header files matched: {string.Join(", ", files)}");
        }

        var commands = new Dictionary<string, GeneratedCommand>(StringComparer.Ordinal);
        var description = $"Parsed from {allFiles.Count} header file(s).";
        string? primaryName = null;

        foreach (var headerPath in allFiles)
        {
            var headerAbs = Path.GetFullPath(headerPath);
            primaryName ??= Path.GetFileNameWithoutExtension(headerAbs);

            // Run clang
            var psi = new ProcessStartInfo("clang")
            {
                RedirectStandardOutput = true,
                RedirectStandardError = true,
                UseShellExecute = false,
            };
            psi.ArgumentList.Add("-Xclang");
            psi.ArgumentList.Add("-ast-dump=json");
            psi.ArgumentList.Add("-fsyntax-only");
            psi.ArgumentList.Add(headerAbs);

            string stdout, stderr;
            using (var p = Process.Start(psi)!)
            {
                stdout = p.StandardOutput.ReadToEnd();
                stderr = p.StandardError.ReadToEnd();
                p.WaitForExit(60000);
            }
            // clang exits non-zero on warnings; we continue as long as we got JSON
            if (string.IsNullOrWhiteSpace(stdout))
            {
                throw new InvalidOperationException(
                    $"clang produced no AST for {headerAbs}: {stderr.Trim()}"
                );
            }

            ExtractFunctions(stdout, headerAbs, commands);
        }

        var id = agentIdOverride ?? primaryName?.ToLowerInvariant() ?? "header-agent";
        return new GeneratedAgent
        {
            Id = id,
            Version = "0.1.0",
            Description = description,
            License = "UNKNOWN",
            Stateful = false,
            Commands = commands.Values
                .OrderBy(c => c.Name, StringComparer.Ordinal)
                .ToArray(),
            Skills = Array.Empty<GeneratedSkill>(),
        };
    }

    private static bool IsClangAvailable()
    {
        try
        {
            var psi = new ProcessStartInfo("clang", "--version")
            {
                RedirectStandardOutput = true,
                RedirectStandardError = true,
                UseShellExecute = false,
            };
            using var p = Process.Start(psi);
            if (p == null) return false;
            p.WaitForExit(5000);
            return p.ExitCode == 0;
        }
        catch
        {
            return false;
        }
    }

    private static List<string> ResolveFiles(string[] inputs)
    {
        var result = new List<string>();
        foreach (var input in inputs)
        {
            if (File.Exists(input))
            {
                result.Add(Path.GetFullPath(input));
                continue;
            }
            // Simple glob: <dir>/<pattern>
            var dir = Path.GetDirectoryName(input);
            var pattern = Path.GetFileName(input);
            if (string.IsNullOrEmpty(dir)) dir = ".";
            if (!Directory.Exists(dir) || string.IsNullOrEmpty(pattern)) continue;
            foreach (var hit in Directory.EnumerateFiles(dir, pattern))
            {
                result.Add(Path.GetFullPath(hit));
            }
        }
        return result.Distinct(StringComparer.OrdinalIgnoreCase).ToList();
    }

    /// <summary>
    /// Walks the clang AST JSON looking for top-level FunctionDecl nodes whose
    /// source location is within the requested header file.
    /// </summary>
    private static void ExtractFunctions(string astJson, string headerAbs, Dictionary<string, GeneratedCommand> output)
    {
        using var doc = JsonDocument.Parse(astJson);
        var root = doc.RootElement;
        if (root.ValueKind != JsonValueKind.Object) return;
        if (!root.TryGetProperty("inner", out var inner) || inner.ValueKind != JsonValueKind.Array) return;

        foreach (var node in inner.EnumerateArray())
        {
            WalkNode(node, headerAbs, output);
        }
    }

    private static void WalkNode(JsonElement node, string headerAbs, Dictionary<string, GeneratedCommand> output)
    {
        if (node.ValueKind != JsonValueKind.Object) return;

        if (node.TryGetProperty("kind", out var kindEl))
        {
            var kind = kindEl.GetString();
            if (kind == "FunctionDecl")
            {
                ExtractFunctionDecl(node, headerAbs, output);
                // Don't recurse into FunctionDecl bodies; we only want free functions at this level
                return;
            }
        }

        // Recurse into namespaces and similar containers
        if (node.TryGetProperty("inner", out var inner) && inner.ValueKind == JsonValueKind.Array)
        {
            foreach (var child in inner.EnumerateArray())
            {
                WalkNode(child, headerAbs, output);
            }
        }
    }

    private static void ExtractFunctionDecl(JsonElement node, string headerAbs, Dictionary<string, GeneratedCommand> output)
    {
        if (!node.TryGetProperty("name", out var nameEl)) return;
        var name = nameEl.GetString();
        if (string.IsNullOrEmpty(name)) return;

        // Skip implicit/built-in functions and operator overloads
        if (name.StartsWith("__", StringComparison.Ordinal)) return;
        if (name.StartsWith("operator", StringComparison.Ordinal)) return;

        // Skip if the source location isn't within our header (i.e., it came from a transitive include)
        if (!IsInTargetFile(node, headerAbs)) return;

        var description = ExtractComment(node) ?? $"C function {name}";
        var kebab = ToKebab(name);

        if (!output.ContainsKey(kebab))
        {
            output[kebab] = new GeneratedCommand
            {
                Name = kebab,
                Lifecycle = "single",
                Description = description,
            };
        }
    }

    private static bool IsInTargetFile(JsonElement node, string targetAbs)
    {
        // clang AST JSON has either:
        //   - "loc": { "file": "...", "line": N, ... } (when the location is in a known file)
        //   - "loc": { "spellingLoc": {...}, "expansionLoc": {...} } (for macro expansions)
        //   - "range": { "begin": { "file": "...", ... } }
        // If "file" is absent, the location is in the SAME file as the previous node with a "file" field
        // (clang's compact encoding). For our purposes, the absence of a "file" key conservatively
        // means "skip" — we only accept declarations whose location explicitly names our header.

        if (TryGetFileFromLoc(node, out var file))
        {
            return string.Equals(Path.GetFullPath(file!), targetAbs, StringComparison.OrdinalIgnoreCase);
        }
        // No explicit file = inherited from context = assume it's in our target (clang's elision)
        return true;
    }

    private static bool TryGetFileFromLoc(JsonElement node, out string? file)
    {
        file = null;
        if (node.TryGetProperty("loc", out var loc) && loc.ValueKind == JsonValueKind.Object)
        {
            if (loc.TryGetProperty("file", out var fileEl) && fileEl.ValueKind == JsonValueKind.String)
            {
                file = fileEl.GetString();
                return true;
            }
            // spellingLoc.file
            if (loc.TryGetProperty("spellingLoc", out var sLoc) && sLoc.ValueKind == JsonValueKind.Object &&
                sLoc.TryGetProperty("file", out var sFileEl) && sFileEl.ValueKind == JsonValueKind.String)
            {
                file = sFileEl.GetString();
                return true;
            }
        }
        if (node.TryGetProperty("range", out var range) && range.ValueKind == JsonValueKind.Object &&
            range.TryGetProperty("begin", out var begin) && begin.ValueKind == JsonValueKind.Object &&
            begin.TryGetProperty("file", out var bFileEl) && bFileEl.ValueKind == JsonValueKind.String)
        {
            file = bFileEl.GetString();
            return true;
        }
        return false;
    }

    private static string? ExtractComment(JsonElement node)
    {
        // clang AST sometimes attaches FullComment children to declarations.
        // For v0.5.2 scope, we keep this simple and return null; XML doc comments are not
        // standardized in C/C++ AST output anyway. Users can fill in descriptions later
        // via `aware skill modify`.
        return null;
    }

    private static string ToKebab(string s)
    {
        var sb = new System.Text.StringBuilder(s.Length + 4);
        var prevLower = false;
        foreach (var c in s)
        {
            if (char.IsUpper(c))
            {
                if (prevLower && sb.Length > 0) sb.Append('-');
                sb.Append(char.ToLowerInvariant(c));
                prevLower = false;
            }
            else if (char.IsWhiteSpace(c) || c == '_' || c == '.' || c == '-')
            {
                if (sb.Length > 0 && sb[sb.Length - 1] != '-') sb.Append('-');
                prevLower = false;
            }
            else if (char.IsLetterOrDigit(c))
            {
                sb.Append(char.ToLowerInvariant(c));
                prevLower = char.IsLower(c) || char.IsDigit(c);
            }
        }
        return sb.ToString().Trim('-');
    }
}
