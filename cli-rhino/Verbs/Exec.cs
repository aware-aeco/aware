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

        // Resolve metadata (pid + version) so the receipt is informative even
        // when the caller supplies an explicit rhino_id. The resolution is
        // best-effort: if `rhinocode list` fails, the exec still proceeds and
        // we just leave pid/version null in the receipt.
        //
        // Cases:
        //   (a) version supplied, rhino_id absent → list to pick first match
        //   (b) rhino_id supplied (with or without version) → list to enrich
        //   (c) neither supplied → no list (rhinocode picks the default)
        int? resolvedPid = null;
        string? resolvedVersion = null;
        if (!string.IsNullOrEmpty(version) || !string.IsNullOrEmpty(rhinoId))
        {
            try
            {
                var resolved = ResolveInstance(client, version, rhinoId);
                if (resolved is null && !string.IsNullOrEmpty(version) && string.IsNullOrEmpty(rhinoId))
                {
                    // Case (a) with no match is a hard error — caller asked for
                    // a specific version and we can't find one.
                    Console.WriteLine(Receipts.ExecFail(
                        $"no running Rhino instance matches version='{version}'",
                        "", "").ToJsonString());
                    return 1;
                }
                if (resolved is not null)
                {
                    rhinoId          = resolved.Value.rhinoId;
                    resolvedPid      = resolved.Value.pid;
                    resolvedVersion  = resolved.Value.version;
                }
                // else: caller supplied rhino_id but list didn't surface it (e.g.
                // the instance died between calls). Proceed best-effort.
            }
            catch (Exception e)
            {
                // Hard error only if caller specifically requested a version
                // (case a). Otherwise log to stderr and proceed with nulls.
                if (!string.IsNullOrEmpty(version) && string.IsNullOrEmpty(rhinoId))
                {
                    Console.WriteLine(Receipts.ExecFail(
                        $"version lookup via rhinocode list failed: {e.Message}", "", "")
                        .ToJsonString());
                    return 2;
                }
                Console.Error.WriteLine(
                    $"aware-rhino exec: rhinocode list (for metadata enrichment) failed: {e.Message}. Proceeding.");
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

    // Picks one entry from `rhinocode list --json`. If `rhinoId` is supplied,
    // match by pipeId (exact). Else if `version` is supplied, match by
    // processVersion startswith. Else null.
    internal static (string rhinoId, int? pid, string version)? ResolveInstance(
        RhinocodeClient client, string? version, string? rhinoId)
    {
        var (exit, stdout, _) = client.RunListJson();
        if (exit != 0) throw new InvalidOperationException($"rhinocode list exited {exit}");
        var arr = JsonNode.Parse(stdout) as JsonArray
                  ?? throw new InvalidOperationException("rhinocode list did not return a JSON array");
        foreach (var item in arr)
        {
            if (item is not JsonObject jo) continue;
            var procVersion = jo["processVersion"]?.GetValue<string>() ?? "";
            var pipeId      = jo["pipeId"]?.GetValue<string>() ?? "";

            bool matchById      = !string.IsNullOrEmpty(rhinoId) && pipeId == rhinoId;
            bool matchByVersion = string.IsNullOrEmpty(rhinoId)
                                  && !string.IsNullOrEmpty(version)
                                  && procVersion.StartsWith(version!, StringComparison.Ordinal);

            if (matchById || matchByVersion)
            {
                return (
                    rhinoId: pipeId,
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
