// `exec` verb: wraps the user's Python in a temp .py file, ships it through
// rhinocode (optionally targeting a specific Rhino instance), reads the result
// the script wrote to a temp file, and emits an AWARE-shaped receipt.
//
// Why a result FILE and not stdout: `rhinocode script` runs the script inside
// the already-running Rhino process and returns nothing on stdout. The script
// writes its {ok,result} dict to a path we chose and baked into its source;
// we read that file back here. See ScriptWrapper for the full rationale.

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
                "missing required field: code (Python script to run against the active Rhino doc)", "", "")
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

        // Choose a unique result-file path, wrap the user's Python around it
        // (baking the path + args into the source), write the temp .py, ship it.
        var resultPath = Path.Combine(Path.GetTempPath(),
            $"aware-rhino-result-{Guid.NewGuid():N}.json");
        var wrapped = ScriptWrapper.Wrap(code!, resultPath, argsJson);
        var scriptPath = Path.Combine(Path.GetTempPath(),
            $"aware-rhino-exec-{Guid.NewGuid():N}.py");
        File.WriteAllText(scriptPath, wrapped);

        try
        {
            var (exit, stdout, stderr) = client.RunScript(scriptPath, rhinoId, argsJson, resultPath);

            // `rhinocode script` is FIRE-AND-FORGET: it ships the script to the
            // live Rhino over a named pipe and returns exit 0 *immediately*,
            // before the script has finished running and written its result
            // file (empirically ~100ms later). So we poll for the file rather
            // than checking once. The result file is the only reliable channel —
            // if it never appears within the timeout the script either didn't
            // run or threw before the write.
            //
            // The timeout is generous (default 60s) so long geometry ops don't
            // get cut off; override with AWARE_EXEC_TIMEOUT_MS. We stop early as
            // soon as the file appears AND is fully written (parse succeeds).
            var timeoutMs = ParseTimeoutMs(Environment.GetEnvironmentVariable("AWARE_EXEC_TIMEOUT_MS"), 60_000);
            var fileText = WaitForResultFile(resultPath, timeoutMs);
            if (fileText is null)
            {
                var msg = exit != 0
                    ? (FirstLine(stderr) ?? $"rhinocode exited {exit}; result file not written")
                    : "result file not written; script did not run or threw before writing";
                Console.WriteLine(Receipts.ExecFail(msg, stderr.Trim(), stdout.Trim()).ToJsonString());
                return 2;
            }

            JsonNode? fileNode;
            try { fileNode = JsonNode.Parse(fileText); }
            catch (Exception e)
            {
                Console.WriteLine(Receipts.ExecFail(
                    $"result file JSON parse failed: {e.Message}", fileText, stdout.Trim()).ToJsonString());
                return 2;
            }

            // The file already carries the AWARE {ok,...} shape produced by the
            // wrapper. Map it onto the receipt envelope.
            var fileObj = fileNode as JsonObject;
            var ok = fileObj?["ok"]?.GetValue<bool>() ?? false;
            if (ok)
            {
                // DeepClone detaches the node from fileObj — assigning a still-
                // parented node into the receipt's JsonObject throws.
                var resultNode = fileObj?["result"]?.DeepClone();
                Console.WriteLine(Receipts.ExecOk(
                    resultNode, resolvedVersion, resolvedPid, rhinoId, stdout.Trim()).ToJsonString());
                return 0;
            }
            else
            {
                var error = fileObj?["error"]?.GetValue<string>() ?? "script reported failure";
                var stack = fileObj?["stack"]?.GetValue<string>() ?? "";
                Console.WriteLine(Receipts.ExecFail(error, stack, stdout.Trim()).ToJsonString());
                return 1;
            }
        }
        finally
        {
            // AWARE_KEEP_TEMP=1 leaves the generated .py + result file on disk for
            // debugging a failing exec. Off by default — temp files are deleted.
            if (string.IsNullOrEmpty(Environment.GetEnvironmentVariable("AWARE_KEEP_TEMP")))
            {
                try { File.Delete(scriptPath); } catch { /* best-effort */ }
                try { File.Delete(resultPath); } catch { /* best-effort */ }
            }
            else
            {
                Console.Error.WriteLine($"aware-rhino exec: kept temp script {scriptPath} and result {resultPath}");
            }
        }
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

    // Polls for the result file because `rhinocode script` returns before the
    // in-Rhino script finishes writing it. Returns the file's text once it
    // exists and parses as complete JSON (guards against reading a half-written
    // file), or null on timeout. Poll interval starts tight (10ms) for the
    // common fast case and the loop simply spins until the deadline.
    internal static string? WaitForResultFile(string resultPath, int timeoutMs)
    {
        var deadline = DateTime.UtcNow.AddMilliseconds(timeoutMs);
        while (true)
        {
            if (File.Exists(resultPath))
            {
                // Read with shared access so we don't collide with the writer,
                // and confirm it parses (i.e. the write completed) before using.
                try
                {
                    using var fs = new FileStream(resultPath, FileMode.Open, FileAccess.Read, FileShare.ReadWrite);
                    using var reader = new StreamReader(fs);
                    var text = reader.ReadToEnd();
                    if (!string.IsNullOrWhiteSpace(text))
                    {
                        try { JsonNode.Parse(text); return text; }
                        catch { /* partial write — keep polling */ }
                    }
                }
                catch (IOException) { /* writer still has it locked — keep polling */ }
            }
            if (DateTime.UtcNow >= deadline) return null;
            Thread.Sleep(10);
        }
    }

    static int ParseTimeoutMs(string? raw, int fallback)
    {
        if (!string.IsNullOrWhiteSpace(raw) && int.TryParse(raw, out var v) && v > 0) return v;
        return fallback;
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
