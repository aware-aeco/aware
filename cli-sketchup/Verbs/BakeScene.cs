// `bake-scene` verb: materialize a canonical millimetre scene as source-owned
// SketchUp groups — the SketchUp sink for the same host-neutral scene cli-tekla
// bakes into native Tekla parts.
//
// Division of labour with the Ruby (BakeSceneScript.rb):
//
//   here   scene envelope, id rules, kind classification, the ownership
//          vocabulary, the materialization hash. All host-independent, so it is
//          proved by unit tests and refused BEFORE a socket is even opened —
//          the sidecar's version of Tekla's "complete trust-boundary validation
//          before the first Insert".
//   Ruby   geometry, the groups, the ownership stamps, the retirement scan, and
//          the single start_operation/commit_operation that makes a whole drop
//          one SketchUp Undo.
//
// The receipt is the same vocabulary the Tekla sink emits:
// {ok, emitted[], failed[], unsupported[], warnings[]} with rows shaped
// {id, kind, status, code?, message?}.

using System.Security.Cryptography;
using System.Text;
using System.Text.Json;
using System.Text.Json.Nodes;

namespace AwareSketchup.Verbs;

internal static class BakeScene
{
    public const string Verb = "bake-scene";

    /// <summary>
    /// The bridge's own main-thread watchdog answers at 90s, so the sidecar waits
    /// longer than that on purpose: a slow bake should come back as the bridge's
    /// clean timeout receipt, not as a socket the sidecar gave up on.
    /// </summary>
    public const int DefaultTimeoutMs = 120_000;

    public const double MillimetresPerInch = 25.4;

    /// <summary>
    /// SketchUp collapses geometry closer together than roughly 0.001 inch. A
    /// length that is perfectly legal in a millimetre scene can be degenerate
    /// here, which is why the check has to happen in inch space.
    /// </summary>
    public const double SketchupToleranceInches = 1.0e-3;

    /// <summary>Scene millimetres to SketchUp internal inches — the C# mirror of Ruby's <c>Numeric#mm</c>.</summary>
    internal static double MmToInches(double millimetres) => millimetres / MillimetresPerInch;

    /// <summary>SketchUp internal inches back to scene millimetres — the C# mirror of Ruby's <c>Length#to_mm</c>.</summary>
    internal static double InchesToMm(double inches) => inches * MillimetresPerInch;

    /// <summary>Element kinds the SketchUp sink materializes. Everything else is reported as `unsupported`, never dropped.</summary>
    internal static readonly string[] MaterializedKinds = { "member", "line", "box" };

    // ── preflight ────────────────────────────────────────────────────────────

    /// <summary>Outcome of the host-independent scene validation.</summary>
    internal sealed class Preflight
    {
        public string SourceId { get; set; } = "";
        public string SceneHash { get; set; } = "";
        public string SceneName { get; set; } = "Steel from Drawings";

        /// <summary>`[{id, kind}]` in scene order — what the Ruby is asked to materialize.</summary>
        public JsonArray Supported { get; } = new();

        public JsonArray Failed { get; } = new();
        public JsonArray Unsupported { get; } = new();

        public bool Ok => Failed.Count == 0;
    }

    internal static JsonObject Row(string id, string kind, string status, string? code = null, string? message = null)
    {
        var row = new JsonObject
        {
            ["id"]     = id,
            ["kind"]   = kind,
            ["status"] = status,
        };
        if (!string.IsNullOrEmpty(code)) row["code"] = code;
        if (!string.IsNullOrEmpty(message)) row["message"] = message;
        return row;
    }

    /// <summary>
    /// An id must be trimmed, 1–256 UTF-8 bytes, free of control characters.
    /// Same rule as the Tekla sink, so one scene is accepted or refused
    /// identically by both hosts.
    /// </summary>
    internal static bool IsValidId(string? id)
    {
        if (string.IsNullOrWhiteSpace(id)) return false;
        if (id != id.Trim()) return false;
        if (Encoding.UTF8.GetByteCount(id) > 256) return false;
        foreach (var c in id)
            if (c < 32 || c == 127) return false;
        return true;
    }

    internal static Preflight Validate(JsonNode? sceneNode)
    {
        var pf = new Preflight();
        if (sceneNode is not JsonObject scene)
        {
            pf.Failed.Add(Row("scene", "scene", "failed", "invalid-scene", "scene must be an object"));
            return pf;
        }

        var meta = scene["meta"] as JsonObject;
        var name = Str(meta, "name").Trim();
        if (name.Length > 0) pf.SceneName = name;
        var units = Str(meta, "units").Trim();
        pf.SourceId = Str(meta, "sourceId").Trim();
        pf.SceneHash = Str(meta, "sceneHash").Trim();

        var elements   = ReadCollection(scene, "elements", pf);
        var operations = ReadCollection(scene, "operations", pf);
        var references = ReadCollection(scene, "referenceSystems", pf);

        if (units.Length > 0 && !units.Equals("mm", StringComparison.OrdinalIgnoreCase))
            pf.Failed.Add(Row("scene", "scene", "failed", "unsupported-units",
                "SketchUp bake-scene accepts only millimetres"));
        if (string.IsNullOrWhiteSpace(pf.SourceId))
            pf.Failed.Add(Row("scene", "scene", "failed", "missing-source-id",
                "scene.meta.sourceId is required"));
        if (string.IsNullOrWhiteSpace(pf.SceneHash))
            pf.Failed.Add(Row("scene", "scene", "failed", "missing-scene-hash",
                "scene.meta.sceneHash is required"));

        // Ids are unique across the WHOLE scene, not per collection — the receipt
        // addresses every record by id alone, so a collision would make two rows
        // indistinguishable to the caller.
        var seen = new HashSet<string>(StringComparer.Ordinal);
        Classify(elements, "element", "member", MaterializedKinds, "unsupported-kind",
            "element kind is not supported by the SketchUp sink", pf, seen);
        Classify(operations, "operation", "operation", Array.Empty<string>(), "unsupported-operation",
            "operation kind is not supported by the SketchUp sink", pf, seen);
        Classify(references, "reference-system", "reference-system", Array.Empty<string>(), "unsupported-reference-system",
            "reference system kind is not supported by the SketchUp sink", pf, seen);

        if (pf.Failed.Count > 0)
            AppendBatchAborted(pf, "Batch was aborted because scene identity or envelope validation failed.");

        return pf;
    }

    static JsonArray ReadCollection(JsonObject scene, string key, Preflight pf)
    {
        if (!scene.ContainsKey(key)) return new JsonArray();
        if (scene[key] is JsonArray array) return array;
        pf.Failed.Add(Row("scene", "scene", "failed", "invalid-collection", $"scene.{key} must be an array"));
        return new JsonArray();
    }

    static void Classify(JsonArray records, string recordLabel, string defaultKind,
                         string[] materialized, string unsupportedCode, string unsupportedMessage,
                         Preflight pf, HashSet<string> seen)
    {
        foreach (var raw in records)
        {
            if (raw is not JsonObject record)
            {
                pf.Failed.Add(Row("scene", recordLabel, "failed", "invalid-record", $"{recordLabel} must be an object"));
                continue;
            }

            var id = Str(record, "id");
            var kind = Str(record, "kind").ToLowerInvariant();
            if (kind.Length == 0) kind = defaultKind;

            if (!IsValidId(id))
            {
                pf.Failed.Add(Row(string.IsNullOrEmpty(id) ? "scene" : id, kind, "failed", "invalid-id",
                    "id must be unique, trimmed, 1-256 UTF-8 bytes, and contain no control characters"));
                continue;
            }
            if (!seen.Add(id))
            {
                pf.Failed.Add(Row(id, kind, "failed", "duplicate-id", "id is duplicated in the scene"));
                continue;
            }

            if (Array.IndexOf(materialized, kind) >= 0)
                pf.Supported.Add(new JsonObject { ["id"] = id, ["kind"] = kind });
            else
                pf.Unsupported.Add(Row(id, kind, "unsupported", unsupportedCode, unsupportedMessage));
        }
    }

    /// <summary>
    /// A refused batch reports on EVERY supported record, not just the one that
    /// broke — the caller must never have to guess whether an unmentioned record
    /// quietly landed.
    /// </summary>
    static void AppendBatchAborted(Preflight pf, string message)
    {
        var alreadyFailed = new HashSet<string>(StringComparer.Ordinal);
        foreach (var f in pf.Failed)
            if (f is JsonObject row && row["id"] is JsonValue v && v.TryGetValue<string>(out var id))
                alreadyFailed.Add(id);

        foreach (var s in pf.Supported)
        {
            if (s is not JsonObject supported) continue;
            var id = Str(supported, "id");
            if (!alreadyFailed.Add(id)) continue;
            pf.Failed.Add(Row(id, Str(supported, "kind"), "failed", "batch-aborted", message));
        }
    }

    internal static JsonObject Envelope(bool ok, JsonArray emitted, JsonArray failed,
                                        JsonArray unsupported, JsonArray warnings, Preflight? pf = null)
    {
        var envelope = new JsonObject
        {
            ["ok"] = ok,
        };
        if (pf is not null)
        {
            envelope["sourceId"] = pf.SourceId;
            envelope["sceneHash"] = pf.SceneHash;
        }
        envelope["emitted"] = emitted;
        envelope["failed"] = failed;
        envelope["unsupported"] = unsupported;
        envelope["warnings"] = warnings;
        return envelope;
    }

    // ── identity ─────────────────────────────────────────────────────────────

    /// <summary>
    /// Identifies one materialization: the exact scene plus the host that will
    /// realize it. It is stamped (as the ownership marker) onto every group the
    /// bake creates, so a later bake can tell its own prior output from anything
    /// else in the model.
    /// </summary>
    internal static string ComputeMaterializationHash(JsonNode scene, string? hostVersion)
    {
        var payload = "sketchup-scene-materializer-v1\0"
            + (hostVersion ?? "running-host")
            + "\0"
            + scene.ToJsonString(new JsonSerializerOptions { WriteIndented = false });
        var digest = SHA256.HashData(Encoding.UTF8.GetBytes(payload));
        return Convert.ToHexStringLower(digest);
    }

    internal static JsonObject BuildOwnership(string sourceId, string materializationHash) => new()
    {
        ["dictionary"]    = BakeSceneOwnership.DictionaryName,
        ["tag"]           = BakeSceneOwnership.TagName,
        ["sourceIdKey"]   = BakeSceneOwnership.SourceIdKey,
        ["recordIdKey"]   = BakeSceneOwnership.RecordIdKey,
        ["sceneHashKey"]  = BakeSceneOwnership.SceneHashKey,
        ["markerKey"]     = BakeSceneOwnership.MarkerKey,
        ["sourceId"]      = sourceId,
        ["marker"]        = BakeSceneOwnership.BuildMarker(materializationHash),
        ["markerPattern"] = BakeSceneOwnership.MarkerPattern,
    };

    /// <summary>
    /// Folds the sidecar's own `unsupported` rows into the bridge's result, so
    /// one receipt accounts for every record in the scene. Kinds the SketchUp
    /// sink cannot build are classified here and never reach the Ruby, so
    /// without this merge they would be silently absent from the receipt.
    /// </summary>
    internal static JsonNode? MergeUnsupported(JsonNode? result, JsonArray sidecarUnsupported)
    {
        if (sidecarUnsupported.Count == 0) return result;

        var merged = new JsonArray();
        foreach (var row in sidecarUnsupported)
            merged.Add(row?.DeepClone());

        if (result is not JsonObject body)
        {
            // The bridge answered with something other than a receipt object. Do
            // not drop the rows on the floor; hand back a receipt that carries
            // both them and whatever was returned.
            return new JsonObject
            {
                ["ok"] = false,
                ["emitted"] = new JsonArray(),
                ["failed"] = new JsonArray
                {
                    Row("scene", "scene", "failed", "invalid-record",
                        "the SketchUp bridge returned a non-object bake result"),
                },
                ["unsupported"] = merged,
                ["warnings"] = new JsonArray(),
                ["bridge_result"] = result?.DeepClone(),
            };
        }

        if (body["unsupported"] is JsonArray existing)
            foreach (var row in existing)
                merged.Add(row?.DeepClone());

        body["unsupported"] = merged;
        return body;
    }

    // ── verb ─────────────────────────────────────────────────────────────────

    /// <summary>
    /// Turn a bridge-level failure into an honest sentence.
    ///
    /// Most bridge faults mean the bake's rescue already ran abort_operation and the model is
    /// intact. A WATCHDOG TIMEOUT does not: that answer comes from the bridge's own thread while
    /// the materializer is still running on SketchUp's main thread, which the bridge cannot
    /// interrupt, so the script may reach commit_operation after the reply. Reporting a plain
    /// failure there would assert an untouched model we have not checked. Say it is unknown, and
    /// say what makes it recoverable — a retry under the same sourceId reconciles rather than
    /// duplicates, because the bake is retire-and-replace on that identity.
    /// </summary>
    internal static string DescribeBridgeFailure(string? bridgeError)
    {
        var error = string.IsNullOrWhiteSpace(bridgeError)
            ? "bridge reported failure with no error message"
            : bridgeError!;
        var timedOut = error.Contains("timeout", StringComparison.OrdinalIgnoreCase)
                    || error.Contains("timed out", StringComparison.OrdinalIgnoreCase)
                    || error.Contains("watchdog", StringComparison.OrdinalIgnoreCase);
        if (!timedOut) return error;
        return error
            + " — the materializer may still be running on SketchUp's main thread, so the model may"
            + " or may not have received this bake. Check the model before deciding; re-running the"
            + " SAME scene under the same sourceId reconciles it (retire-and-replace) rather than"
            + " inserting a second copy.";
    }

    public static int Run(JsonNode? input, SketchupClient? clientOverride = null, int timeoutMs = DefaultTimeoutMs)
    {
        var sceneNode = input?["scene"];
        if (sceneNode is null)
        {
            Console.WriteLine(Receipts.ExecFail(
                "missing required field: scene (the canonical millimetre scene to bake into the model)",
                "", "", verb: Verb).ToJsonString());
            return 2;
        }

        var pf = Validate(sceneNode);
        if (!pf.Ok)
        {
            // Refused without opening a socket: the model is provably untouched.
            Console.WriteLine(Receipts.BakeResultFail(
                Envelope(false, new JsonArray(), pf.Failed, pf.Unsupported, new JsonArray(), pf),
                null, null, null, "").ToJsonString());
            return 2;
        }

        var sketchupId = Exec.TryStringOrNumber(input, "sketchup_id");
        var version    = Exec.TryString(input, "version");
        var label      = Exec.TryString(input, "label") ?? "";

        var client = clientOverride ?? new SketchupClient();

        // A version filter resolves by PREFIX and takes the first match, which is fine for a read
        // but not for this: bake-scene is retire-and-replace, so picking an arbitrary one of two
        // matching sessions would create members in one model and DELETE source-owned groups from
        // whichever happened to be enumerated first. `Resolve` already refuses to guess when no
        // filter is given; a filter that still matches several is the same ambiguity wearing a
        // filter, so it gets the same refusal. An explicit sketchup_id is always unambiguous.
        if (string.IsNullOrEmpty(sketchupId) && !string.IsNullOrEmpty(version))
        {
            var matching = client.ListInstances()
                .Where(i => i.Version.StartsWith(version!, StringComparison.Ordinal))
                .ToList();
            if (matching.Count > 1)
            {
                Console.WriteLine(Receipts.ExecFail(
                    $"ambiguous target — {matching.Count} SketchUp instances match version='{version}' "
                    + $"(pids: {string.Join(", ", matching.Select(i => i.Pid))}); "
                    + "pass sketchup_id=<pid> to say which model to bake into",
                    "", "", verb: Verb).ToJsonString());
                return 1;
            }
        }

        var inst = client.Resolve(sketchupId, version);
        if (inst is null)
        {
            var all = client.ListInstances();
            string msg = all.Count == 0
                ? "no SketchUp instance found; launch SketchUp and run `aware-sketchup --install-bridge` if you haven't yet"
                : !string.IsNullOrEmpty(sketchupId)
                    ? $"no SketchUp instance with sketchup_id='{sketchupId}'; running: [{string.Join(", ", all.Select(i => i.Pid))}]"
                    : !string.IsNullOrEmpty(version)
                        ? $"no SketchUp instance matches version='{version}'; running: [{string.Join(", ", all.Select(i => i.Version))}]"
                        : $"ambiguous target — {all.Count} SketchUp instances running; pass sketchup_id=<pid> in the request";
            Console.WriteLine(Receipts.ExecFail(msg, "", "", verb: Verb).ToJsonString());
            return 1;
        }

        // Hash against the host that will actually realize the scene, not the
        // caller's requested version — the marker has to identify what landed.
        var materializationHash = ComputeMaterializationHash(sceneNode, inst.Version);

        var request = new JsonObject
        {
            ["type"] = "exec",
            ["code"] = BakeSceneScript.Code,
            ["args"] = new JsonObject
            {
                ["scene"]               = sceneNode.DeepClone(),
                ["label"]               = label,
                ["supported"]           = pf.Supported.DeepClone(),
                ["ownership"]           = BuildOwnership(pf.SourceId, materializationHash),
                ["materializationHash"] = materializationHash,
            },
        };

        JsonNode response;
        try { response = client.SendRequest(inst.Port, request, timeoutMs); }
        catch (TimeoutException te)
        {
            Console.WriteLine(Receipts.ExecFail(
                $"timeout talking to SketchUp bridge on port {inst.Port}: {te.Message}",
                "", "", inst.Version, inst.Pid, inst.Pid.ToString(), Verb).ToJsonString());
            return 2;
        }
        catch (Exception e)
        {
            Console.WriteLine(Receipts.ExecFail(
                $"bridge I/O failed (port {inst.Port}, pid {inst.Pid}): {e.Message}",
                e.StackTrace ?? "", "", inst.Version, inst.Pid, inst.Pid.ToString(), Verb).ToJsonString());
            return 2;
        }

        if (response is not JsonObject bridgeBody)
        {
            Console.WriteLine(Receipts.ExecFail(
                "bridge returned a non-object response", "", response.ToString() ?? "",
                inst.Version, inst.Pid, inst.Pid.ToString(), Verb).ToJsonString());
            return 2;
        }

        var stdoutLog = bridgeBody["stdout_log"]?.GetValue<string>() ?? "";
        bool dispatched = bridgeBody["ok"]?.GetValue<bool?>() ?? false;
        if (!dispatched)
        {
            // The Ruby raised outside its own rescue, or the bridge itself failed. Usually the
            // bake's rescue has already run abort_operation, so the model is intact.
            //
            // EXCEPT on the watchdog timeout. That answer comes from the bridge's own thread while
            // the materializer is still running on SketchUp's MAIN thread — which the bridge cannot
            // interrupt. The script may therefore reach commit_operation AFTER this reply, so
            // "failed" here does not mean "nothing happened", and a receipt implying an untouched
            // model would be a guess dressed as a fact. Say the outcome is unknown, and say what
            // makes it recoverable: a retry under the SAME sourceId reconciles rather than
            // duplicates, because the bake is retire-and-replace on that identity.
            Console.WriteLine(Receipts.ExecFail(
                DescribeBridgeFailure(bridgeBody["error"]?.GetValue<string>()),
                bridgeBody["stack"]?.GetValue<string>() ?? "",
                stdoutLog, inst.Version, inst.Pid, inst.Pid.ToString(), Verb).ToJsonString());
            return 2;
        }

        var result = MergeUnsupported(bridgeBody["result"], pf.Unsupported);
        bool baked = result is JsonObject r && (r["ok"]?.GetValue<bool?>() ?? false);
        if (!baked)
        {
            Console.WriteLine(Receipts.BakeResultFail(
                result, inst.Version, inst.Pid, inst.Pid.ToString(), stdoutLog).ToJsonString());
            return 2;
        }

        Console.WriteLine(Receipts.ExecOk(
            result, inst.Version, inst.Pid, inst.Pid.ToString(), stdoutLog, Verb).ToJsonString());
        return 0;
    }

    static string Str(JsonObject? obj, string key)
    {
        if (obj is null) return "";
        var node = obj[key];
        if (node is null) return "";
        try { return node.GetValue<string>(); }
        catch { return ""; }
    }
}
