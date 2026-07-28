using System.Security.Cryptography;
using System.Text;
using System.Text.Json.Nodes;

namespace AwareRhino;

/// <summary>
/// Host-neutral validation and identity rules for <c>bake-scene</c>. Rhino is
/// still treated as an untrusted execution boundary, so the embedded Python
/// repeats the geometry checks before it mutates the active document.
/// </summary>
internal static class BakeSceneRules
{
    internal const string SourceIdKey = "AWARE.BAKE.SOURCE_ID";
    internal const string RecordIdKey = "AWARE.BAKE.RECORD_ID";
    internal const string SceneHashKey = "AWARE.BAKE.SCENE_HASH";
    internal const string MarkerKey = "AWARE.BAKE.MARKER";
    internal const string LegacyMarkerPrefix = "AWARE_BAKE_V1:";
    internal const string MarkerPrefix = "AWARE_BAKE_V2:";
    internal const string LayerName = "AWARE";

    internal sealed class Preflight
    {
        internal string SourceId { get; set; } = "";
        internal string SceneHash { get; set; } = "";
        internal JsonArray Supported { get; } = [];
        internal JsonArray Failed { get; } = [];
        internal JsonArray Unsupported { get; } = [];
        internal bool Ok => Failed.Count == 0;
    }

    internal sealed record RhinoInstance(string RhinoId, int Pid, string Version, string ActiveDoc);

    internal sealed record TargetResult(RhinoInstance? Instance, string? Error)
    {
        internal bool Ok => Instance is not null && Error is null;
    }

    internal static JsonObject Row(string id, string kind, string status, string? code = null, string? message = null)
    {
        var row = new JsonObject
        {
            ["id"] = id,
            ["kind"] = kind,
            ["status"] = status,
        };
        if (!string.IsNullOrEmpty(code)) row["code"] = code;
        if (!string.IsNullOrEmpty(message)) row["message"] = message;
        return row;
    }

    internal static JsonObject Envelope(Preflight pf)
        => new()
        {
            ["ok"] = false,
            ["sourceId"] = pf.SourceId,
            ["sceneHash"] = pf.SceneHash,
            ["emitted"] = new JsonArray(),
            ["failed"] = pf.Failed.DeepClone(),
            ["unsupported"] = pf.Unsupported.DeepClone(),
            ["warnings"] = new JsonArray(),
        };

    internal static bool IsValidId(string? id)
    {
        if (string.IsNullOrWhiteSpace(id) || id != id.Trim()) return false;
        if (Encoding.UTF8.GetByteCount(id) > 256) return false;
        return id.All(c => (c >= ' ' && c != '\u007f'));
    }

    internal static bool IsOwnershipMarker(string? marker)
    {
        if (marker is null || marker.Length != MarkerPrefix.Length + 64)
            return false;
        var prefixOk = marker.StartsWith(MarkerPrefix, StringComparison.Ordinal)
                       || marker.StartsWith(LegacyMarkerPrefix, StringComparison.Ordinal);
        return prefixOk && marker.AsSpan(MarkerPrefix.Length).ToArray()
            .All(c => (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f'));
    }

    internal static string BuildMarker(string materializationHash)
        => MarkerPrefix + materializationHash;

    internal static string ComputeMaterializationHash(JsonNode scene, string hostVersion)
    {
        var payload = scene.ToJsonString() + "\0rhino\0" + hostVersion
                      + "\0" + CrossSectionProfile.GeometryRevision;
        return Convert.ToHexString(SHA256.HashData(Encoding.UTF8.GetBytes(payload))).ToLowerInvariant();
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
        var units = Str(meta, "units").Trim();
        pf.SourceId = Str(meta, "sourceId").Trim();
        pf.SceneHash = Str(meta, "sceneHash").Trim();
        if (units.Length > 0 && !units.Equals("mm", StringComparison.OrdinalIgnoreCase))
            pf.Failed.Add(Row("scene", "scene", "failed", "unsupported-units",
                "Rhino bake-scene accepts only a canonical millimetre scene"));
        if (pf.SourceId.Length == 0)
            pf.Failed.Add(Row("scene", "scene", "failed", "missing-source-id", "scene.meta.sourceId is required"));
        if (pf.SceneHash.Length == 0)
            pf.Failed.Add(Row("scene", "scene", "failed", "missing-scene-hash", "scene.meta.sceneHash is required"));

        var seen = new HashSet<string>(StringComparer.Ordinal);
        var supported = new List<(string Id, string Kind)>();

        var elements = Collection(scene, "elements", pf);
        if (elements is not null)
        {
            foreach (var raw in elements)
            {
                if (raw is not JsonObject element)
                {
                    pf.Failed.Add(Row("scene", "element", "failed", "invalid-record", "element must be an object"));
                    continue;
                }
                var id = Str(element, "id");
                var kind = Str(element, "kind").Trim().ToLowerInvariant();
                if (kind.Length == 0) kind = "member";
                if (!AcceptId(id, kind, pf, seen)) continue;
                if (kind == "member")
                {
                    supported.Add((id, kind));
                    var profile = ValidateMember(element, id, pf);
                    var supportedRow = new JsonObject { ["id"] = id, ["kind"] = kind };
                    if (profile is not null) supportedRow["profile"] = profile.ToJson();
                    pf.Supported.Add(supportedRow);
                }
                else
                {
                    pf.Unsupported.Add(Row(id, kind, "unsupported", "unsupported-kind",
                        "element kind is not supported by the Rhino member sink"));
                }

                if (kind == "plate")
                    AddChildren(element, "holes", "opening", "plate hole", pf, seen);
            }
        }

        var operations = Collection(scene, "operations", pf);
        if (operations is not null)
        {
            foreach (var raw in operations)
            {
                if (raw is not JsonObject operation)
                {
                    pf.Failed.Add(Row("scene", "operation", "failed", "invalid-record", "operation must be an object"));
                    continue;
                }
                var id = Str(operation, "id");
                var kind = Str(operation, "kind").Trim().ToLowerInvariant();
                if (kind.Length == 0) kind = "operation";
                if (!AcceptId(id, kind, pf, seen)) continue;
                pf.Unsupported.Add(Row(id, kind, "unsupported", "unsupported-operation",
                    "operation kind is not supported by the Rhino member sink"));
                if (kind == "bolt-array")
                {
                    if (operation["instances"] is JsonArray instances)
                    {
                        foreach (var instanceRaw in instances)
                        {
                            if (instanceRaw is not JsonObject instance)
                            {
                                pf.Failed.Add(Row(id, "bolt-instance", "failed", "invalid-record",
                                    "bolt instance must be an object"));
                                continue;
                            }
                            var instanceId = Str(instance, "id");
                            if (AcceptId(instanceId, "bolt-instance", pf, seen))
                                pf.Unsupported.Add(Row(instanceId, "bolt-instance", "unsupported",
                                    "unsupported-parent", "parent bolt array is unsupported"));
                            AddChildren(instance, "holeEffects", "opening", "bolt hole effect", pf, seen);
                        }
                    }
                    else if (operation.ContainsKey("instances"))
                        pf.Failed.Add(Row(id, "bolt-instance", "failed", "invalid-collection",
                            "bolt-array instances must be an array"));
                }
            }
        }

        var references = Collection(scene, "referenceSystems", pf);
        if (references is not null)
        {
            foreach (var raw in references)
            {
                if (raw is not JsonObject reference)
                {
                    pf.Failed.Add(Row("scene", "reference-system", "failed", "invalid-record",
                        "reference system must be an object"));
                    continue;
                }
                var id = Str(reference, "id");
                var kind = Str(reference, "kind").Trim().ToLowerInvariant();
                if (kind.Length == 0) kind = "reference-system";
                if (!AcceptId(id, kind, pf, seen)) continue;
                pf.Unsupported.Add(Row(id, kind, "unsupported", "unsupported-reference-system",
                    "reference system kind is not supported by the Rhino member sink"));
                AddChildren(reference, "axes", "grid-axis", "grid axis", pf, seen);
                AddChildren(reference, "levels", "grid-level", "grid level", pf, seen);
            }
        }

        if (pf.Failed.Count > 0)
        {
            var failedIds = pf.Failed.OfType<JsonObject>()
                .Select(r => Str(r, "id")).ToHashSet(StringComparer.Ordinal);
            foreach (var item in supported)
                if (failedIds.Add(item.Id))
                    pf.Failed.Add(Row(item.Id, item.Kind, "failed", "batch-aborted",
                        "Batch was aborted because scene validation failed."));
        }
        return pf;
    }

    internal static JsonArray ParseInstances(string json)
        => JsonNode.Parse(json) as JsonArray
           ?? throw new InvalidOperationException("rhinocode list did not return a JSON array");

    internal static IReadOnlyList<RhinoInstance> ReadInstances(JsonArray raw)
    {
        var list = new List<RhinoInstance>();
        foreach (var item in raw.OfType<JsonObject>())
        {
            var id = Str(item, "pipeId");
            var version = Str(item, "processVersion");
            var activeDoc = ListInstancesActiveDoc(item["activeDoc"]);
            if (id.Length == 0 || version.Length == 0
                || item["processId"] is not JsonValue pv || !pv.TryGetValue<int>(out var pid))
                continue;
            list.Add(new RhinoInstance(id, pid, TrimVersion(version), activeDoc));
        }
        return list;
    }

    internal static TargetResult ResolveTarget(
        IReadOnlyList<RhinoInstance> all, string? rhinoId, int? hostPid, string? version)
    {
        rhinoId = string.IsNullOrWhiteSpace(rhinoId) ? null : rhinoId.Trim();
        version = string.IsNullOrWhiteSpace(version) ? null : version.Trim();

        IEnumerable<RhinoInstance> matches = all;
        if (rhinoId is not null) matches = matches.Where(i => i.RhinoId == rhinoId);
        if (hostPid is not null) matches = matches.Where(i => i.Pid == hostPid.Value);
        if (version is not null) matches = matches.Where(i => i.Version.StartsWith(version, StringComparison.Ordinal));
        var selected = matches.ToList();

        if (selected.Count == 1) return new(selected[0], null);
        if (selected.Count == 0)
        {
            var selectors = string.Join(", ", new[]
            {
                rhinoId is null ? null : $"rhino_id='{rhinoId}'",
                hostPid is null ? null : $"host_pid={hostPid}",
                version is null ? null : $"version='{version}'",
            }.Where(s => s is not null));
            return new(null, selectors.Length == 0
                ? "no Rhino Script Server session is available"
                : $"no Rhino Script Server session matches {selectors}");
        }

        return new(null,
            $"ambiguous target — {selected.Count} Rhino sessions match; pass exact rhino_id or host_pid "
            + $"(matches: {string.Join(", ", selected.Select(i => $"{i.Pid}/{i.RhinoId}"))})");
    }

    static CrossSectionProfile.Plan? ValidateMember(JsonObject element, string id, Preflight pf)
    {
        if (!Vec3(element["from"]) || !Vec3(element["to"]))
        {
            pf.Failed.Add(Row(id, "member", "failed", "invalid-geometry",
                "member requires finite from/to points in millimetres"));
            return null;
        }
        var from = (JsonArray)element["from"]!;
        var to = (JsonArray)element["to"]!;
        var length2 = Enumerable.Range(0, 3)
            .Select(i => Number(to[i]) - Number(from[i])).Sum(v => v * v);
        var section = element["section"] as JsonObject;
        var w = Number(section?["w"]);
        var d = Number(section?["d"]);
        if (!(length2 > 0) || !(w > 0) || !(d > 0))
        {
            pf.Failed.Add(Row(id, "member", "failed", "invalid-geometry",
                "member requires a non-zero axis and a positive section {w,d}; Rhino never invents a section"));
            return null;
        }
        var decoded = CrossSectionProfile.Decode(element, w, d);
        if (!decoded.Ok)
        {
            pf.Failed.Add(Row(id, "member", "failed", "invalid-xsection", decoded.Error));
            return null;
        }
        return decoded.Profile;
    }

    static bool Vec3(JsonNode? node)
        => node is JsonArray a && a.Count >= 3 && Enumerable.Range(0, 3).All(i => double.IsFinite(Number(a[i])));

    static double Number(JsonNode? node)
        => node is JsonValue v && v.TryGetValue<double>(out var d) ? d : double.NaN;

    static JsonArray? Collection(JsonObject scene, string key, Preflight pf)
    {
        if (!scene.ContainsKey(key)) return [];
        if (scene[key] is JsonArray a) return a;
        pf.Failed.Add(Row("scene", "scene", "failed", "invalid-collection", $"scene.{key} must be an array"));
        return null;
    }

    static void AddChildren(JsonObject parent, string key, string kind, string label,
                            Preflight pf, HashSet<string> seen)
    {
        if (!parent.ContainsKey(key)) return;
        if (parent[key] is not JsonArray children)
        {
            pf.Failed.Add(Row(Str(parent, "id"), kind, "failed", "invalid-collection",
                $"{label} collection must be an array"));
            return;
        }
        foreach (var raw in children)
        {
            if (raw is not JsonObject child)
            {
                pf.Failed.Add(Row(Str(parent, "id"), kind, "failed", "invalid-record",
                    $"{label} must be an object"));
                continue;
            }
            var id = Str(child, "id");
            if (AcceptId(id, kind, pf, seen))
                pf.Unsupported.Add(Row(id, kind, "unsupported", "unsupported-parent",
                    "parent record is unsupported by the Rhino member sink"));
        }
    }

    static bool AcceptId(string id, string kind, Preflight pf, HashSet<string> seen)
    {
        if (!IsValidId(id))
        {
            pf.Failed.Add(Row(id.Length == 0 ? "scene" : id, kind, "failed", "invalid-id",
                "id must be unique, trimmed, 1-256 UTF-8 bytes, and contain no control characters"));
            return false;
        }
        if (!seen.Add(id))
        {
            pf.Failed.Add(Row(id, kind, "failed", "duplicate-id", "id is duplicated in the scene"));
            return false;
        }
        return true;
    }

    internal static string TrimVersion(string full)
    {
        var parts = full.Split('.');
        return parts.Length >= 2 ? $"{parts[0]}.{parts[1]}" : full;
    }

    static string Str(JsonObject? obj, string key)
        => obj?[key] is JsonValue v && v.TryGetValue<string>(out var s) ? s ?? "" : "";

    static string ListInstancesActiveDoc(JsonNode? node)
    {
        if (node is JsonValue value && value.TryGetValue<string>(out var text))
            return text ?? "";
        if (node is JsonObject obj)
        {
            var location = Str(obj, "location");
            return location.Length > 0 ? location : Str(obj, "title");
        }
        return "";
    }
}
