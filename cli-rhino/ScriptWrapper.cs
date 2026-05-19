// Wraps the user's C# `code` (from {verb:"exec", code: "..."}) into a complete
// .cs file that rhinocode can execute. The wrapped file uses C# 9 top-level
// statements + a synthetic `__AwareRun(args)` local function so the user's
// body can use `return X;` semantics — same UX as cli-tekla's exec.

using System.Text;
using System.Text.RegularExpressions;

namespace AwareRhino;

internal static class ScriptWrapper
{
    // Matches `using <id>(.<id>)*;` and `using <alias> = <id>(.<id>)*;`
    // but NOT `using (var x = ...)` statement form.
    static readonly Regex UsingDirectiveRe = new(
        @"^\s*using\s+(?:[A-Za-z_][A-Za-z0-9_]*\s*=\s*)?[A-Za-z_][A-Za-z0-9_.]*\s*;\s*$",
        RegexOptions.Compiled);

    // Preamble brings in the standard Rhino + BCL namespaces. Dedup against
    // user-supplied directives is by exact string match (C# is case-sensitive).
    static readonly string[] PreambleUsings = new[]
    {
        "using System;",
        "using System.Collections.Generic;",
        "using System.Linq;",
        "using System.IO;",
        "using System.Text.Json;",
        "using System.Text.Json.Serialization;",
        "using Rhino;",
        "using Rhino.DocObjects;",
        "using Rhino.Geometry;",
        "using Rhino.Input;",
        "using Rhino.Input.Custom;",
        "using Rhino.PlugIns;",
        "using Rhino.UI;",
    };

    public static (List<string> usings, string body) SplitUsingsAndBody(string code)
    {
        var usings = new List<string>();
        var bodyLines = new List<string>();
        bool inDirectives = true;  // Stop collecting directives at first non-directive line.

        foreach (var line in code.Replace("\r\n", "\n").Split('\n'))
        {
            if (inDirectives && UsingDirectiveRe.IsMatch(line))
            {
                usings.Add(line.Trim());
            }
            else if (inDirectives && string.IsNullOrWhiteSpace(line))
            {
                // Whitespace between directives is allowed; stays in directives bucket.
                // We don't add it anywhere — body re-starts at first real statement.
            }
            else
            {
                inDirectives = false;
                bodyLines.Add(line);
            }
        }

        return (usings, string.Join("\n", bodyLines));
    }

    public static string Wrap(string userCode)
    {
        var (userUsings, body) = SplitUsingsAndBody(userCode);

        // Dedup: keep preamble usings; add only user usings that aren't already there.
        var preambleSet = new HashSet<string>(PreambleUsings, StringComparer.Ordinal);
        var extraUsings = userUsings.Where(u => !preambleSet.Contains(u)).Distinct().ToList();

        // Defensive: if body has no `return` keyword, append `return null;` so the
        // synthetic function doesn't fail CS0161. Naive keyword search is fine for
        // top-level snippets; nested `return`s inside lambdas count too — false-
        // negatives just append a dead `return null;` which is harmless.
        var bodyHasReturn = Regex.IsMatch(body, @"\breturn\b");
        var bodyWithReturn = bodyHasReturn ? body : (body.TrimEnd() + "\nreturn null;");
        if (string.IsNullOrWhiteSpace(body))
            bodyWithReturn = "return null;";

        var sb = new StringBuilder();
        sb.AppendLine("// AWARE-generated script (do not edit)");
        foreach (var u in PreambleUsings) sb.AppendLine(u);
        foreach (var u in extraUsings) sb.AppendLine(u);
        sb.AppendLine();
        sb.AppendLine("var __awareArgsJson = Environment.GetEnvironmentVariable(\"AWARE_ARGS_JSON\") ?? \"{}\";");
        sb.AppendLine("var args = JsonSerializer.Deserialize<Dictionary<string, object?>>(__awareArgsJson)");
        sb.AppendLine("           ?? new Dictionary<string, object?>();");
        sb.AppendLine();
        sb.AppendLine("var __result = __AwareRun(args);");
        sb.AppendLine("Console.WriteLine(\"__AWARE_RESULT_BEGIN__\");");
        sb.AppendLine("Console.WriteLine(JsonSerializer.Serialize(__result, new JsonSerializerOptions {");
        sb.AppendLine("    WriteIndented = false,");
        sb.AppendLine("    ReferenceHandler = ReferenceHandler.IgnoreCycles,");
        sb.AppendLine("    MaxDepth = 6,");
        sb.AppendLine("}));");
        sb.AppendLine("Console.WriteLine(\"__AWARE_RESULT_END__\");");
        sb.AppendLine();
        sb.AppendLine("static object? __AwareRun(IDictionary<string, object?> args)");
        sb.AppendLine("{");
        foreach (var line in bodyWithReturn.Split('\n'))
            sb.AppendLine("    " + line);
        sb.AppendLine("}");

        return sb.ToString();
    }

    // Result-sentinel markers (constants — used by Exec verb too).
    public const string ResultBeginSentinel = "__AWARE_RESULT_BEGIN__";
    public const string ResultEndSentinel   = "__AWARE_RESULT_END__";
}
