// Wraps the user's Python `code` (from {verb:"exec", code: "..."}) into a
// complete .py file that rhinocode can execute. The user's body ends with a
// `return <value>` statement, so we nest it inside a synthetic
// `__aware_run(args)` function — same `return X` UX as cli-tekla's exec.
//
// IMPORTANT — why the result path is BAKED INTO the source (not an env var):
// `rhinocode script foo.py` does NOT spawn a fresh interpreter. It ships the
// script over a named pipe into the ALREADY-RUNNING Rhino process and runs it
// there. That process inherited Rhino's environment at launch — NOT the
// environment of the short-lived `rhinocode.exe` child we start. So env vars
// we set on the child (AWARE_RESULT_PATH / AWARE_ARGS_JSON) are invisible to
// the script, and the script's stdout never comes back to us either. The only
// channel that works is: bake the values into the source as literals, have the
// script WRITE its result to a file, then read that file back. Proven against
// live Rhino 8.31.

using System.Text;

namespace AwareRhino;

internal static class ScriptWrapper
{
    // Preamble brings in the standard Rhino + stdlib modules most exec scripts
    // need. We do NOT dedup against user imports — Python tolerates redundant
    // `import` statements, and user fixtures use fully-qualified `Rhino.*`.
    static readonly string[] PreambleImports = new[]
    {
        "import json, traceback, os",
        "import Rhino",
        "import rhinoscriptsyntax as rs",
        "import scriptcontext as sc",
        "import System",
    };

    /// <summary>
    /// Builds the full Python script. <paramref name="resultPath"/> is where the
    /// script writes its <c>{ok,result}</c> / <c>{ok:false,error,stack}</c> dict;
    /// <paramref name="argsJson"/> is the JSON the user body reads via its
    /// <c>args</c> parameter. Both are embedded as source literals (see header).
    /// </summary>
    public static string Wrap(string userCode, string resultPath, string argsJson)
    {
        // Normalise newlines, then indent every user line by 4 spaces so it sits
        // inside `def __aware_run(args):`. Blank lines stay blank (no trailing
        // whitespace) to keep the body clean.
        var normalised = (userCode ?? string.Empty).Replace("\r\n", "\n").Replace("\r", "\n");
        var bodyLines = normalised.Split('\n').ToList();

        // Defensive: if the body has no top-level `return`, append one so
        // __aware_run always yields a value. Naive check — a `return` nested in
        // a comment/string would suppress this, but then the user owns the bug.
        bool hasReturn = bodyLines.Any(l =>
        {
            var t = l.TrimStart();
            return t.StartsWith("return ", StringComparison.Ordinal)
                || t == "return"
                || t.StartsWith("return\t", StringComparison.Ordinal);
        });

        var indented = new List<string>();
        foreach (var line in bodyLines)
            indented.Add(string.IsNullOrEmpty(line.TrimEnd()) ? string.Empty : "    " + line);
        if (!hasReturn) indented.Add("    return None");
        // Guard against an entirely-empty body producing an empty function.
        if (indented.All(l => string.IsNullOrEmpty(l))) indented.Add("    return None");

        var sb = new StringBuilder();
        sb.Append("# AWARE-generated script (do not edit)\n");
        foreach (var imp in PreambleImports) sb.Append(imp).Append('\n');
        sb.Append('\n');

        sb.Append("def __aware_run(args):\n");
        foreach (var line in indented) sb.Append(line).Append('\n');
        sb.Append('\n');

        // Embed args + result path as literals. Also read the env var as a
        // fallback in case a future rhinocode variant DOES propagate env.
        sb.Append("__aware_args_json = ").Append(PyStringLiteral(argsJson)).Append('\n');
        sb.Append("__aware_result_path = os.environ.get(\"AWARE_RESULT_PATH\") or ")
          .Append(PyStringLiteral(resultPath)).Append('\n');
        sb.Append("try:\n");
        sb.Append("    __aware_args = json.loads(os.environ.get(\"AWARE_ARGS_JSON\") or __aware_args_json or \"{}\")\n");
        sb.Append("except Exception:\n");
        sb.Append("    __aware_args = {}\n");
        sb.Append('\n');

        sb.Append("try:\n");
        sb.Append("    __aware_retval = __aware_run(__aware_args)\n");
        sb.Append("    __aware_out = {\"ok\": True, \"result\": __aware_retval}\n");
        sb.Append("except Exception as __aware_ex:\n");
        sb.Append("    __aware_out = {\"ok\": False, \"error\": str(__aware_ex), \"stack\": traceback.format_exc()}\n");
        sb.Append('\n');

        // Serialize to a STRING first, with allow_nan=False so NaN/inf raise here
        // (instead of emitting `NaN` which is invalid JSON the C# side chokes on).
        // If the result isn't JSON-serializable, degrade to a guaranteed-
        // serializable error payload rather than writing nothing — writing
        // nothing would make Exec poll the full timeout and falsely report
        // "result file not written". Only after we hold a complete string do we
        // open + write it once (a single complete write also closes the
        // truncated-file race the reader would otherwise hit).
        sb.Append("try:\n");
        sb.Append("    __aware_text = json.dumps(__aware_out, default=str, allow_nan=False)\n");
        sb.Append("except Exception as __aware_ser_ex:\n");
        sb.Append("    __aware_text = json.dumps(\n");
        sb.Append("        {\"ok\": False,\n");
        sb.Append("         \"error\": \"result not JSON-serializable: \" + repr(__aware_ser_ex),\n");
        sb.Append("         \"stack\": traceback.format_exc()},\n");
        sb.Append("        default=str)\n");
        sb.Append('\n');

        sb.Append("with open(__aware_result_path, \"w\", encoding=\"utf-8\") as __aware_f:\n");
        sb.Append("    __aware_f.write(__aware_text)\n");

        return sb.ToString();
    }

    /// <summary>
    /// Renders <paramref name="value"/> as a safe Python single-line string
    /// literal: double-quoted with backslash, quote, and control chars escaped.
    /// Used for both the file path (Windows backslashes) and the args JSON.
    /// </summary>
    internal static string PyStringLiteral(string? value)
    {
        var s = value ?? string.Empty;
        var sb = new StringBuilder(s.Length + 2);
        sb.Append('"');
        foreach (var ch in s)
        {
            switch (ch)
            {
                case '\\': sb.Append("\\\\"); break;
                case '"':  sb.Append("\\\""); break;
                case '\n': sb.Append("\\n"); break;
                case '\r': sb.Append("\\r"); break;
                case '\t': sb.Append("\\t"); break;
                default:
                    if (ch < 0x20)
                        sb.Append("\\x").Append(((int)ch).ToString("x2"));
                    else
                        sb.Append(ch);
                    break;
            }
        }
        sb.Append('"');
        return sb.ToString();
    }
}
