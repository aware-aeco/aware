using System.Text.Json.Nodes;

namespace AwareRhino.Verbs;

internal static class BakeScene
{
    internal const string Verb = "bake-scene";

    internal static JsonObject BuildOwnership(string sourceId, string materializationHash)
        => new()
        {
            ["sourceIdKey"] = BakeSceneRules.SourceIdKey,
            ["recordIdKey"] = BakeSceneRules.RecordIdKey,
            ["sceneHashKey"] = BakeSceneRules.SceneHashKey,
            ["markerKey"] = BakeSceneRules.MarkerKey,
            ["sourceId"] = sourceId,
            ["marker"] = BakeSceneRules.BuildMarker(materializationHash),
            ["layer"] = BakeSceneRules.LayerName,
            ["geometryRevision"] = CrossSectionProfile.GeometryRevision,
        };

    public static int Run(RhinocodeClient client, JsonNode? input)
    {
        var scene = input?["scene"];
        if (scene is null)
        {
            var missing = new BakeSceneRules.Preflight();
            missing.Failed.Add(BakeSceneRules.Row("scene", "scene", "failed",
                "invalid-scene", "missing required field: scene"));
            Console.WriteLine(Receipts.BakeResultFail(BakeSceneRules.Envelope(missing)).ToJsonString());
            return 2;
        }

        var preflight = BakeSceneRules.Validate(scene);
        if (!preflight.Ok)
        {
            Console.WriteLine(Receipts.BakeResultFail(BakeSceneRules.Envelope(preflight)).ToJsonString());
            return 2;
        }

        if (!TrySelectors(input, out var rhinoId, out var hostPid, out var version, out var selectorError))
        {
            Console.WriteLine(Receipts.ExecFail(selectorError!, "", "", verb: Verb).ToJsonString());
            return 2;
        }

        IReadOnlyList<BakeSceneRules.RhinoInstance> instances;
        try
        {
            var (exit, stdout, stderr) = client.RunListJson();
            if (exit != 0)
            {
                Console.WriteLine(Receipts.ExecFail(
                    $"rhinocode list exited {exit}: {stderr.Trim()}", "", "",
                    exitCode: exit, stderrLog: stderr.Trim(), verb: Verb).ToJsonString());
                return 2;
            }
            var raw = BakeSceneRules.ParseInstances(stdout);
            if (raw.Count == 0 && ScriptServerHelper.TryEnsureScriptServer(client))
            {
                var retry = client.RunListJson();
                if (retry.exitCode != 0)
                {
                    Console.WriteLine(Receipts.ExecFail(
                        $"rhinocode list retry exited {retry.exitCode}: {retry.stderr.Trim()}", "", "",
                        exitCode: retry.exitCode, stderrLog: retry.stderr.Trim(), verb: Verb).ToJsonString());
                    return 2;
                }
                raw = BakeSceneRules.ParseInstances(retry.stdout);
            }
            instances = BakeSceneRules.ReadInstances(raw);
        }
        catch (Exception ex)
        {
            Console.WriteLine(Receipts.ExecFail(
                $"Rhino instance discovery failed: {ex.Message}", ex.StackTrace ?? "", "",
                verb: Verb).ToJsonString());
            return 2;
        }

        var target = BakeSceneRules.ResolveTarget(instances, rhinoId, hostPid, version);
        if (!target.Ok)
        {
            Console.WriteLine(Receipts.ExecFail(target.Error!, "", "", verb: Verb).ToJsonString());
            return 1;
        }
        var instance = target.Instance!;
        var hash = BakeSceneRules.ComputeMaterializationHash(scene, instance.Version);

        var execInput = new JsonObject
        {
            ["verb"] = "exec",
            ["code"] = BakeSceneScript.Code,
            ["rhino_id"] = instance.RhinoId,
            ["version"] = instance.Version,
            ["args"] = new JsonObject
            {
                ["scene"] = scene.DeepClone(),
                ["supported"] = preflight.Supported.DeepClone(),
                ["unsupported"] = preflight.Unsupported.DeepClone(),
                ["ownership"] = BuildOwnership(preflight.SourceId, hash),
                ["materializationHash"] = hash,
                ["label"] = TryString(input, "label") ?? "",
            },
        };

        using var capture = new StringWriter();
        var original = Console.Out;
        int execExit;
        try
        {
            Console.SetOut(capture);
            execExit = Exec.Run(client, execInput, ExecWaitPolicy.MutatingBake);
        }
        finally
        {
            Console.SetOut(original);
        }

        var text = capture.ToString().Trim();
        JsonObject? receipt;
        try { receipt = JsonNode.Parse(text) as JsonObject; }
        catch { receipt = null; }
        if (receipt is null)
        {
            Console.WriteLine(Receipts.ExecFail(
                "exec produced no parseable receipt for bake-scene", "", text,
                hostVersion: instance.Version, hostPid: instance.Pid,
                rhinoId: instance.RhinoId, verb: Verb).ToJsonString());
            return 2;
        }

        receipt["verb"] = Verb;
        receipt["host_version"] = instance.Version;
        receipt["host_pid"] = instance.Pid;
        receipt["rhino_id"] = instance.RhinoId;
        if (execExit != 0)
        {
            Console.WriteLine(receipt.ToJsonString());
            return execExit;
        }

        var baked = receipt["result"] is JsonObject result
                    && result["ok"] is JsonValue okNode
                    && okNode.TryGetValue<bool>(out var ok)
                    && ok;
        if (!baked)
        {
            receipt["ok"] = false;
            receipt["error"] = "bake-scene returned ok:false; inspect the structured result receipt";
            Console.WriteLine(receipt.ToJsonString());
            return 2;
        }

        Console.WriteLine(receipt.ToJsonString());
        return 0;
    }

    internal static bool TrySelectors(JsonNode? input, out string? rhinoId, out int? hostPid,
                                      out string? version, out string? error)
    {
        rhinoId = null;
        version = null;
        hostPid = null;
        error = null;
        if (input is not JsonObject obj) return true;
        if (!TryOptionalSelectorString(obj, "rhino_id", out rhinoId, out error)
            || !TryOptionalSelectorString(obj, "version", out version, out error))
            return false;
        if (!obj.ContainsKey("host_pid")) return true;
        if (obj["host_pid"] is JsonValue value
            && value.TryGetValue<int>(out var pid)
            && pid > 0)
        {
            hostPid = pid;
            return true;
        }
        error = "invalid host_pid: expected a positive integer";
        return false;
    }

    static bool TryOptionalSelectorString(JsonObject obj, string key, out string? selector,
                                          out string? error)
    {
        selector = null;
        error = null;
        if (!obj.ContainsKey(key)) return true;
        if (obj[key] is JsonValue value
            && value.TryGetValue<string>(out var text)
            && !string.IsNullOrWhiteSpace(text))
        {
            selector = text.Trim();
            return true;
        }
        error = $"invalid {key}: expected a non-empty string";
        return false;
    }

    static string? TryString(JsonNode? input, string key)
    {
        if (input?[key] is not JsonValue value) return null;
        if (value.TryGetValue<string>(out var text) && !string.IsNullOrWhiteSpace(text))
            return text.Trim();
        return null;
    }
}
