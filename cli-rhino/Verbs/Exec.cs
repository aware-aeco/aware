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
                    // Case (a) with no match — before giving up, try to auto-start
                    // the Script Server on any running Rhino.exe process (mimics
                    // typing `_StartScriptServer` in the command bar) and retry.
                    if (ScriptServerHelper.TryEnsureScriptServer(client))
                        resolved = ResolveInstance(client, version, rhinoId);

                    if (resolved is null)
                    {
                        Console.WriteLine(Receipts.ExecFail(
                            $"no running Rhino instance matches version='{version}'",
                            "", "").ToJsonString());
                        return 1;
                    }
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
                // No file ever appeared within the timeout. This is an infra
                // fault — surface the rhinocode exit code + stderr so callers can
                // distinguish it from a script-reported failure. Don't assert
                // "script did not run": it may have run and failed to write.
                var msg = exit != 0
                    ? (FirstLine(stderr) ?? $"rhinocode exited {exit}; result file not written")
                    : "result file not written within timeout (script may have failed before writing)";
                Console.WriteLine(Receipts.ExecFail(
                    msg, stderr.Trim(), stdout.Trim(), exit, stderr.Trim()).ToJsonString());
                return 2;
            }

            JsonNode? fileNode = null;
            Exception? parseEx = null;
            try { fileNode = JsonNode.Parse(fileText); }
            catch (Exception e) { parseEx = e; }

            // A file that exists but doesn't parse (truncated / non-JSON) — or
            // parses to something other than an object — is a malformed-result
            // fault. Surface the contents so the failure is diagnosable.
            if (fileNode is not JsonObject fileObj)
            {
                var why = parseEx is not null ? parseEx.Message : "result file is not a JSON object";
                Console.WriteLine(Receipts.ExecFail(
                    $"result file JSON parse failed: {why}", fileText, stdout.Trim(), exit, stderr.Trim())
                    .ToJsonString());
                return 2;
            }

            // The file carries the AWARE {ok,...} shape produced by the wrapper.
            // Read `ok` tolerantly — a non-bool there means the file is
            // structurally wrong, which we treat as a parse failure rather than
            // letting a hard cast leak to the top-level handler.
            if (fileObj["ok"] is not JsonValue okVal || !okVal.TryGetValue<bool>(out var ok))
            {
                Console.WriteLine(Receipts.ExecFail(
                    "result file JSON parse failed: missing or non-boolean 'ok' field",
                    fileText, stdout.Trim(), exit, stderr.Trim()).ToJsonString());
                return 2;
            }

            if (ok)
            {
                // DeepClone detaches the node from fileObj — assigning a still-
                // parented node into the receipt's JsonObject throws.
                var resultNode = fileObj["result"]?.DeepClone();
                Console.WriteLine(Receipts.ExecOk(
                    resultNode, resolvedVersion, resolvedPid, rhinoId, stdout.Trim()).ToJsonString());
                return 0;
            }
            else
            {
                // Script ran but reported failure. This is a script-fault, not
                // infra — leave exit_code null so callers can tell them apart.
                var error = TolerantString(fileObj["error"]) ?? "script reported failure";
                var stack = TolerantString(fileObj["stack"]) ?? "";
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

    // How long a non-empty-but-unparseable result file must stay byte-for-byte
    // unchanged before we accept it as "settled" and hand it back for an
    // accurate parse-failed receipt (rather than polling to the full timeout
    // and then falsely reporting "not written").
    const int SettledMs = 200;

    // Polls for the result file because `rhinocode script` returns before the
    // in-Rhino script finishes writing it. Returns:
    //   - the file's text as soon as it exists and parses as complete JSON
    //     (the happy path — guards against reading a half-written file), OR
    //   - the file's text once it exists, is non-empty, and has been stable
    //     (size + mtime unchanged) for SettledMs without ever parsing — so the
    //     caller can emit "result file JSON parse failed" with the contents
    //     instead of contradicting the filesystem, OR
    //   - null on timeout (file never appeared, or never settled).
    // Poll interval is tight (10ms) for the common fast case.
    internal static string? WaitForResultFile(string resultPath, int timeoutMs)
    {
        var deadline = DateTime.UtcNow.AddMilliseconds(timeoutMs);
        long lastLen = -1;
        DateTime lastWrite = DateTime.MinValue;
        DateTime stableSince = DateTime.MaxValue;

        while (true)
        {
            string? text = null;
            try
            {
                var fi = new FileInfo(resultPath);
                if (fi.Exists)
                {
                    // Read with shared access so we don't collide with the writer.
                    using var fs = new FileStream(resultPath, FileMode.Open, FileAccess.Read, FileShare.ReadWrite);
                    using var reader = new StreamReader(fs);
                    text = reader.ReadToEnd();

                    if (!string.IsNullOrWhiteSpace(text))
                    {
                        // Complete write ⇒ done.
                        try { JsonNode.Parse(text); return text; }
                        catch { /* not parseable yet — track whether it's settled */ }

                        // Track byte-length + mtime; reset the stable clock on any change.
                        var len = fs.Length;
                        var mtime = fi.LastWriteTimeUtc;
                        if (len != lastLen || mtime != lastWrite)
                        {
                            lastLen = len;
                            lastWrite = mtime;
                            stableSince = DateTime.UtcNow;
                        }
                        else if (DateTime.UtcNow - stableSince >= TimeSpan.FromMilliseconds(SettledMs))
                        {
                            // Non-empty, unchanged for SettledMs, still won't
                            // parse ⇒ accept it so the caller reports the truth.
                            return text;
                        }
                    }
                }
            }
            catch (IOException) { /* writer still has it locked — keep polling */ }

            if (DateTime.UtcNow >= deadline) return null;
            Thread.Sleep(10);
        }
    }

    static int ParseTimeoutMs(string? raw, int fallback)
    {
        if (!string.IsNullOrWhiteSpace(raw) && int.TryParse(raw, out var v) && v > 0) return v;
        return fallback;
    }

    // Reads a node as a string only when it actually is one. Returns null for
    // null / non-string nodes rather than throwing on a bad cast.
    static string? TolerantString(JsonNode? node)
        => node is JsonValue jv && jv.TryGetValue<string>(out var s) ? s : null;

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
