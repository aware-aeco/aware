// Splits user-supplied C# into `using` directives + body, wraps the body in
// a synthetic local function so `return X;` semantics work. Mirrors cli-tekla
// and cli-rhino's wrappers. The actual compile+run lives in AwareRevit.dll's
// ScriptEngine.cs which copies these literals (the add-in adds Revit-specific
// references that the sidecar doesn't reference).

using System.Text;
using System.Text.RegularExpressions;

namespace AwareRevit.Sidecar;

internal static class ScriptWrapper
{
    static readonly Regex UsingDirectiveRe = new(
        @"^\s*using\s+(?:[A-Za-z_][A-Za-z0-9_]*\s*=\s*)?[A-Za-z_][A-Za-z0-9_.]*\s*;\s*$",
        RegexOptions.Compiled);

    /// <summary>Preamble usings — kept in sync with AwareRevit/ScriptEngine.cs.</summary>
    public static readonly string[] PreambleUsings =
    {
        "using System;",
        "using System.Collections.Generic;",
        "using System.Linq;",
        "using System.IO;",
        "using System.Text.Json;",
        "using Autodesk.Revit.ApplicationServices;",
        "using Autodesk.Revit.Attributes;",
        "using Autodesk.Revit.DB;",
        "using Autodesk.Revit.DB.Structure;",
        "using Autodesk.Revit.UI;",
        "using Autodesk.Revit.UI.Selection;",
    };

    public static (List<string> usings, string body) SplitUsingsAndBody(string code)
    {
        var usings = new List<string>();
        var bodyLines = new List<string>();
        bool inDirectives = true;

        foreach (var line in code.Replace("\r\n", "\n").Split('\n'))
        {
            if (inDirectives && UsingDirectiveRe.IsMatch(line))
            {
                usings.Add(line.Trim());
            }
            else if (inDirectives && string.IsNullOrWhiteSpace(line))
            {
                // Whitespace between directives is allowed; doesn't break the directive block.
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
        var preambleSet = new HashSet<string>(PreambleUsings, StringComparer.Ordinal);
        var extraUsings = userUsings.Where(u => !preambleSet.Contains(u)).Distinct().ToList();

        var hasReturn = Regex.IsMatch(body, @"\breturn\b");
        var bodyOut = hasReturn ? body : (body.TrimEnd() + "\nreturn null;");
        if (string.IsNullOrWhiteSpace(body)) bodyOut = "return null;";

        var sb = new StringBuilder();
        sb.AppendLine("// AWARE-generated script (do not edit)");
        foreach (var u in PreambleUsings) sb.AppendLine(u);
        foreach (var u in extraUsings) sb.AppendLine(u);
        sb.AppendLine();
        sb.AppendLine("static object? __AwareRun(dynamic uiapp, IDictionary<string, object?> args)");
        sb.AppendLine("{");
        foreach (var line in bodyOut.Split('\n')) sb.AppendLine("    " + line);
        sb.AppendLine("}");
        sb.AppendLine();
        sb.AppendLine("return __AwareRun(uiapp, args);");

        return sb.ToString();
    }
}
