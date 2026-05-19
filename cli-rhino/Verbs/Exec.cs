// `exec` verb: wraps the user's C# in a temp .cs file, ships it through
// rhinocode (optionally targeting a specific Rhino instance), parses the
// result between sentinels, emits an AWARE-shaped receipt.

using System.Text.Json.Nodes;

namespace AwareRhino.Verbs;

internal static class Exec
{
    public static int Run(RhinocodeClient client, JsonNode? input)
    {
        var code = input?["code"]?.GetValue<string>();
        if (string.IsNullOrEmpty(code))
        {
            Console.WriteLine(Receipts.ExecFail(
                "missing required field: code (C# script to compile + run)", "", "")
                .ToJsonString());
            return 2;
        }

        var rhinoId = input?["rhino_id"]?.GetValue<string>();
        var version = input?["version"]?.GetValue<string>();
        var argsObj = input?["args"] as JsonObject;
        var argsJson = (argsObj ?? new JsonObject()).ToJsonString();

        // If no rhino_id supplied but version is, resolve via list and pick first match.
        // (If neither, rhinocode picks the default.)
        int? resolvedPid = null;
        string? resolvedVersion = null;
        if (string.IsNullOrEmpty(rhinoId) && !string.IsNullOrEmpty(version))
        {
            try
            {
                var resolved = ResolveByVersion(client, version!);
                if (resolved is null)
                {
                    Console.WriteLine(Receipts.ExecFail(
                        $"no running Rhino instance matches version='{version}'",
                        "", "").ToJsonString());
                    return 1;
                }
                rhinoId          = resolved.Value.rhinoId;
                resolvedPid      = resolved.Value.pid;
                resolvedVersion  = resolved.Value.version;
            }
            catch (Exception e)
            {
                Console.WriteLine(Receipts.ExecFail(
                    $"version lookup via rhinocode list failed: {e.Message}", "", "")
                    .ToJsonString());
                return 2;
            }
        }

        // Wrap, write temp, ship.
        var wrapped = ScriptWrapper.Wrap(code!);
        var tempPath = Path.Combine(Path.GetTempPath(),
            $"aware-rhino-exec-{Guid.NewGuid():N}.cs");
        File.WriteAllText(tempPath, wrapped);

        try
        {
            var (exit, stdout, stderr) = client.RunScript(tempPath, rhinoId, argsJson);

            // Find the sentinel block.
            var (jsonText, otherStdout) = ExtractResultJson(stdout);
            if (jsonText is null)
            {
                // Failure mode 1: rhinocode exited non-zero. Surface stderr.
                if (exit != 0)
                {
                    Console.WriteLine(Receipts.ExecFail(
                        FirstLine(stderr) ?? $"rhinocode exited {exit}",
                        stderr.Trim(), stdout).ToJsonString());
                    return 2;
                }
                // Failure mode 2: rhinocode exited 0 but no sentinel. Likely the
                // user's script threw before writing the result. Surface the
                // last non-empty stdout line as the best clue.
                Console.WriteLine(Receipts.ExecFail(
                    "result sentinel not found in script output (script may have thrown)",
                    stderr.Trim(), stdout).ToJsonString());
                return 2;
            }

            JsonNode? resultNode;
            try { resultNode = JsonNode.Parse(jsonText); }
            catch (Exception e)
            {
                Console.WriteLine(Receipts.ExecFail(
                    $"result JSON parse failed: {e.Message}",
                    jsonText, stdout).ToJsonString());
                return 2;
            }

            Console.WriteLine(Receipts.ExecOk(resultNode, resolvedVersion, resolvedPid, rhinoId, otherStdout).ToJsonString());
            return 0;
        }
        finally
        {
            try { File.Delete(tempPath); } catch { /* best-effort */ }
        }
    }

    // Pulls the JSON between __AWARE_RESULT_BEGIN__ and __AWARE_RESULT_END__ out
    // of stdout. Returns (jsonText, otherStdout) — otherStdout has the sentinel
    // block stripped so the receipt's stdout_log is just the user's noise.
    internal static (string? jsonText, string otherStdout) ExtractResultJson(string stdout)
    {
        var lines = stdout.Replace("\r\n", "\n").Split('\n');
        int beginIdx = -1, endIdx = -1;
        for (int i = 0; i < lines.Length; i++)
        {
            if (beginIdx < 0 && lines[i].Trim() == ScriptWrapper.ResultBeginSentinel)
                beginIdx = i;
            else if (beginIdx >= 0 && lines[i].Trim() == ScriptWrapper.ResultEndSentinel)
            {
                endIdx = i;
                break;
            }
        }
        if (beginIdx < 0 || endIdx < 0) return (null, stdout);

        var jsonLines = new List<string>();
        for (int i = beginIdx + 1; i < endIdx; i++) jsonLines.Add(lines[i]);
        var json = string.Join("\n", jsonLines).Trim();

        var rest = new List<string>();
        for (int i = 0; i < lines.Length; i++)
            if (i < beginIdx || i > endIdx) rest.Add(lines[i]);
        return (json, string.Join("\n", rest).Trim());
    }

    static (string rhinoId, int? pid, string version)? ResolveByVersion(RhinocodeClient client, string version)
    {
        var (exit, stdout, _) = client.RunListJson();
        if (exit != 0) throw new InvalidOperationException($"rhinocode list exited {exit}");
        var arr = JsonNode.Parse(stdout) as JsonArray
                  ?? throw new InvalidOperationException("rhinocode list did not return a JSON array");
        foreach (var item in arr)
        {
            if (item is not JsonObject jo) continue;
            var procVersion = jo["processVersion"]?.GetValue<string>() ?? "";
            // Accept "8" matching "8.13.24296.13001" — startswith.
            if (procVersion.StartsWith(version, StringComparison.Ordinal))
            {
                return (
                    rhinoId: jo["pipeId"]!.GetValue<string>(),
                    pid: jo["processId"]?.GetValue<int?>(),
                    version: TrimVersion(procVersion));
            }
        }
        return null;
    }

    static string TrimVersion(string full)
    {
        var parts = full.Split('.');
        return parts.Length >= 2 ? $"{parts[0]}.{parts[1]}" : full;
    }

    static string? FirstLine(string? s)
    {
        if (string.IsNullOrEmpty(s)) return null;
        var idx = s!.IndexOf('\n');
        return idx < 0 ? s.Trim() : s.Substring(0, idx).Trim();
    }
}
