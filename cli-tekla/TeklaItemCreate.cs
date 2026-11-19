using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;
using System.Linq;
using System.Security.Cryptography;
using System.Text;
using System.Text.Json.Nodes;

namespace AwareTekla;

/// <summary>One member the caller wants sent as an Item: its neutral solid, where it sits, what Tekla needs.</summary>
public sealed class ItemMember
{
    public string Id { get; set; } = "";
    public string Label { get; set; } = "";
    public string? DefinitionDigest { get; set; }
    public PrismSolid Solid { get; set; } = new PrismSolid();
    public string SolidDigest { get; set; } = "";
    public double[] Start { get; set; } = new double[3];
    public double[] End { get; set; } = new double[3];
    public double RollDeg { get; set; }
    public string Material { get; set; } = "Steel_Undefined";
}

/// <summary>A parsed, validated `item-create` request. Built only by <see cref="TeklaItemCreate.Parse"/>.</summary>
public sealed class ItemCreateRequest
{
    /// <summary>"plan" (nothing written), "create" or "delete".</summary>
    public string Mode { get; set; } = "plan";
    public string? ConsentSource { get; set; }
    public string NamePrefix { get; set; } = ItemShapeNames.DefaultPrefix;
    public string? ExpectModelPath { get; set; }
    public string? ExpectModelGuid { get; set; }
    public bool? ExpectShared { get; set; }
    public string? ExpectTeklaVersion { get; set; }
    public List<ItemMember> Members { get; } = new List<ItemMember>();
    /// <summary>delete mode: guids of the Items to delete first.</summary>
    public List<string> ItemGuids { get; } = new List<string>();
    /// <summary>delete mode: names of the shapes to delete.</summary>
    public List<string> ShapeNames { get; } = new List<string>();
    /// <summary>delete mode: the ledger's name for each Item guid; the Item must still carry it.</summary>
    public Dictionary<string, string> ItemProof { get; } = new Dictionary<string, string>(StringComparer.OrdinalIgnoreCase);
    /// <summary>delete mode: the ledger's guid (and geometry guid) for each shape name; the shape must still carry them.</summary>
    public Dictionary<string, (string guid, string? geometryGuid)> ShapeProof { get; } = new Dictionary<string, (string, string?)>(StringComparer.OrdinalIgnoreCase);
    /// <summary>delete mode: delete what the run's ledger does not vouch for anyway (--not-ours-confirmed).</summary>
    public bool NotOursConfirmed { get; set; }
    /// <summary>delete mode: the acknowledgement that the shapes are still used by this many Items.</summary>
    public int? DeleteWithItems { get; set; }
}

/// <summary>The open model as Tekla reports it; what the person saw when they chose to send as an Item.</summary>
public sealed class ItemTarget
{
    public string ModelPath { get; set; } = "";
    public string? ModelGuid { get; set; }
    public bool? SharedModel { get; set; }
    public bool? SingleUserModel { get; set; }
    public string? TeklaVersion { get; set; }
}

/// <summary>A shape in the catalogue, read back.</summary>
public sealed class ShapeInfo
{
    public string Name { get; set; } = "";
    public string? Guid { get; set; }
    public string? GeometryGuid { get; set; }
    /// <summary>The "aware.digest" metadata, when the shape carries it.</summary>
    public string? Digest { get; set; }
    /// <summary>[minX, minY, minZ, maxX, maxY, maxZ] or null when Tekla would not say.</summary>
    public double[]? Extrema { get; set; }
    public List<double[]> Vertices { get; } = new List<double[]>();
    /// <summary>How many Items Tekla says use the shape; null when Tekla would not say.</summary>
    public int? InstanceCount { get; set; }
}

public sealed class ShapeInsertResult
{
    public bool Inserted { get; set; }
    /// <summary>Guids of existing shapes with the same geometry (when nothing was inserted).</summary>
    public List<string> MatchingGuids { get; } = new List<string>();
    public string? Refusal { get; set; }
}

/// <summary>One Item to place.</summary>
public sealed class ItemSpec
{
    public string MemberId { get; set; } = "";
    public string Name { get; set; } = "";
    public string ShapeName { get; set; } = "";
    public double[] Start { get; set; } = new double[3];
    public double[] End { get; set; } = new double[3];
    public double RollDeg { get; set; }
    public string Material { get; set; } = "";
    public string Digest { get; set; } = "";
    public string? DefinitionDigest { get; set; }
    /// <summary>
    /// Set by the host the moment Tekla accepts the Item, before the commit or anything else that can fail, so the
    /// engine can record an Item whose placement then threw.
    /// </summary>
    public string? PlacedGuid { get; set; }
}

/// <summary>An Item as Tekla reports it after it is placed, measured against the shape and the request.</summary>
public sealed class ItemReadback
{
    public bool Exists { get; set; }
    public string? Guid { get; set; }
    public string? Name { get; set; }
    /// <summary>The Item's Profile.ProfileString (the shape name); empty when the shape was deleted.</summary>
    public string? ShapeName { get; set; }
    public double[]? SolidBox { get; set; }
    public double? VolumeFromFaces { get; set; }
    public double? OriginToStartMm { get; set; }
    public double? AxisXDot { get; set; }
    public double? ExtentAlongAxisX { get; set; }
    public double? MaxSolidToShapeMm { get; set; }
    public double? MaxShapeToSolidMm { get; set; }
    public int? ShapeInstanceCount { get; set; }
    public bool? UdaSet { get; set; }
    /// <summary>The guid names a model object that is not a Brep Item (Exists is false); this verb never deletes those.</summary>
    public bool NotBrep { get; set; }
}

public sealed class ItemRef
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
}

/// <summary>
/// Everything `item-create` needs from Tekla, as plain data. The Roslyn script implements it with the Open API;
/// the tests implement it with a fake, which is how a partial failure is exercised without a Tekla.
/// A write method may throw at any point; the engine's ledger says what had been written by then.
/// </summary>
public interface IItemCreateHost
{
    ItemTarget ReadTarget();
    /// <summary>The shape with this name, or null when none exists.</summary>
    ShapeInfo? FindShape(string name);
    ShapeInfo? FindShapeByGuid(string guid);
    /// <summary>Tekla's own validation of the brep (Polymesh.Validate with every check); empty means valid.</summary>
    IReadOnlyList<string> ValidateBrep(PrismBrep brep);
    /// <summary>WRITE: insert the shape, or report the shapes that already have this geometry.</summary>
    ShapeInsertResult InsertShape(string name, PrismBrep brep);
    /// <summary>WRITE: tag the shape with the digest and label. Returns whether Tekla accepted both.</summary>
    bool SetShapeMetadata(string name, string digest, string label);
    /// <summary>WRITE: place one Item and commit. Sets <see cref="ItemSpec.PlacedGuid"/> as soon as Tekla accepts it; returns the guid.</summary>
    string? InsertItem(ItemSpec spec);
    /// <summary>Reads a placed Item and measures it against the request and the shape it names.</summary>
    ItemReadback MeasureItem(ItemSpec spec, string guid);
    ItemReadback ReadItem(string guid);
    IReadOnlyList<ItemRef> ItemsUsingShape(string name);
    /// <summary>WRITE.</summary>
    bool DeleteItem(string guid);
    /// <summary>WRITE: ShapeItem.Delete(), which Tekla performs even while Items use the shape.</summary>
    bool DeleteShape(string name);
    /// <summary>Path to SHA-256 of every file in the model's Shapes and ShapeGeometries folders.</summary>
    IDictionary<string, string> ListCatalogueFiles();
    /// <summary>Copies the catalogue files somewhere safe, verifies the copies and returns the folder; throws when it cannot.</summary>
    string BackupCatalogue(string runId);
}

/// <summary>The file half of a host: the model's catalogue folders, read and backed up without Tekla.</summary>
public abstract class CatalogueFileHost
{
    protected CatalogueFileHost(string modelPath, string? backupRoot = null)
    {
        ModelPath = modelPath;
        BackupRoot = backupRoot ?? Path.Combine(
            Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "aware", "tekla-item-create");
    }

    protected string ModelPath { get; }
    protected string BackupRoot { get; }

    public static readonly string[] CatalogueFolders = { "Shapes", "ShapeGeometries" };

    // A hash is reused while a file's size and write time are unchanged, so listing the catalogue around every shape
    // costs a directory scan, not a re-hash of every file each time.
    readonly Dictionary<string, (long size, long ticks, string hash)> _hashes = new Dictionary<string, (long, long, string)>(StringComparer.OrdinalIgnoreCase);

    public IDictionary<string, string> ListCatalogueFiles()
    {
        var now = new Dictionary<string, string>(StringComparer.OrdinalIgnoreCase);
        foreach (var sub in CatalogueFolders)
        {
            var dir = Path.Combine(ModelPath, sub);
            if (!Directory.Exists(dir)) continue;
            foreach (var path in Directory.GetFiles(dir, "*", SearchOption.AllDirectories))
            {
                var info = new FileInfo(path);
                if (_hashes.TryGetValue(path, out var known) && known.size == info.Length && known.ticks == info.LastWriteTimeUtc.Ticks) { now[path] = known.hash; continue; }
                var hash = Sha256(path);
                _hashes[path] = (info.Length, info.LastWriteTimeUtc.Ticks, hash);
                now[path] = hash;
            }
        }
        return now;
    }

    public string BackupCatalogue(string runId)
    {
        var key = TeklaItemCreate.ModelKey(ModelPath);
        var dir = Path.Combine(BackupRoot, key, runId, "backup");
        Directory.CreateDirectory(dir);
        foreach (var sub in CatalogueFolders)
        {
            var source = Path.Combine(ModelPath, sub);
            if (!Directory.Exists(source)) continue;
            foreach (var file in Directory.GetFiles(source, "*", SearchOption.AllDirectories))
            {
                var relative = file.Substring(ModelPath.TrimEnd('\\', '/').Length).TrimStart('\\', '/');
                var copy = Path.Combine(dir, relative);
                Directory.CreateDirectory(Path.GetDirectoryName(copy)!);
                File.Copy(file, copy, true);
                if (!string.Equals(Sha256(file), Sha256(copy), StringComparison.OrdinalIgnoreCase))
                    throw new IOException("the backup copy of " + file + " does not match it");
            }
        }
        return dir;
    }

    public static string Sha256(string path)
    {
        using var sha = SHA256.Create();
        using var stream = File.OpenRead(path);
        return string.Concat(sha.ComputeHash(stream).Select(b => b.ToString("x2", CultureInfo.InvariantCulture)));
    }
}

/// <summary>
/// `item-create` (aware#859, FloLess #3382 A5): send a member to Tekla as an Item, the route for a shape Tekla's
/// profile catalogue does not have. A shape is added to the open model's own shape catalogue, then an Item
/// (Tekla.Structures.Model.Brep) is placed on it. The pieces here are Tekla-free: the request, the checks, the
/// order of writes, the receipt and the undo. <see cref="TeklaItemCreateScript"/> supplies the Open API half.
///
/// Settled live (Tekla 2026.0, 2026-11-19, spike run 1): the write lands in the model folder only; Tekla does not
/// stretch a shape to the Item's length, so there is one shape per distinct length; <c>Insert()</c> accepts an
/// invalid brep, so Tekla's validator runs first; <c>ShapeItem.Delete()</c> deletes a shape even while Items use
/// it, so a delete is refused until the caller acknowledges the number of Items affected.
/// </summary>
public static class TeklaItemCreate
{
    public const int MaxMembers = 2000;
    public static readonly string[] ConsentSources = { "ui-click", "chat-proposal-card" };

    /// <summary>The tolerance, in mm, within which an existing shape's geometry equals the one wanted.</summary>
    const double GeometryToleranceMm = 1e-3;
    /// <summary>Allowed deviation, in mm, of a placed Item from its shape, its start point and its length.</summary>
    const double PlacementToleranceMm = 1e-2;

    // ── request ───────────────────────────────────────────────────────────────

    /// <summary>
    /// Why the request cannot be run, or null. The text starts with the status the caller sees
    /// (invalid-request, consent-required, invalid-solid, digest-mismatch, length-mismatch). Nothing here touches Tekla.
    /// </summary>
    public static string? Parse(JsonNode? input, out ItemCreateRequest? request)
    {
        request = null;
        if (input is not JsonObject root) return "invalid-request: the request must be a JSON object.";
        var req = new ItemCreateRequest();
        if (NotText(root, "request", "mode", "version", "name_prefix") is { } badRoot) return badRoot;
        var mode = Text(root, "mode") ?? "plan";
        if (mode != "plan" && mode != "create" && mode != "delete") return $"invalid-request: mode '{mode}' is not plan, create or delete.";
        req.Mode = mode;

        if (mode != "plan")
        {
            var source = root["consent"] is JsonObject consent ? Text(consent, "source") : null;
            if (source == null || !ConsentSources.Contains(source))
                return "consent-required: a " + mode + " writes to the open Tekla model; pass consent {\"source\": \"ui-click\"} or {\"source\": \"chat-proposal-card\"} after the person confirmed (an unattended run never calls this).";
            req.ConsentSource = source;
        }

        if (root["target"] is JsonObject target)
        {
            if (NotText(target, "target", "model_path", "model_guid", "tekla_version") is { } badTarget) return badTarget;
            if (target["shared_model"] != null && !(target["shared_model"] is JsonValue sm && sm.TryGetValue<bool>(out _))) return "invalid-request: target.shared_model must be true or false.";
            req.ExpectModelPath = Text(target, "model_path");
            req.ExpectModelGuid = Text(target, "model_guid");
            req.ExpectTeklaVersion = Text(target, "tekla_version");
            if (target["shared_model"] is JsonValue sv && sv.TryGetValue<bool>(out var shared)) req.ExpectShared = shared;
        }
        else if (root["target"] != null) return "invalid-request: target must be an object.";
        if (mode != "plan" && req.ExpectModelPath == null && req.ExpectModelGuid == null)
            return "invalid-request: a " + mode + " must say which model it is for (target.model_path or target.model_guid): the model the person chose.";

        if (root.ContainsKey("name_prefix"))
        {
            var prefix = Text(root, "name_prefix");
            if (ItemShapeNames.ValidatePrefix(prefix) is { } why) return "invalid-request: " + why;
            req.NamePrefix = prefix!;
        }

        if (root.ContainsKey("delete_with_items"))
        {
            if (root["delete_with_items"] is not JsonValue dv || !dv.TryGetValue<double>(out var n) || n < 0 || n != Math.Floor(n) || n > int.MaxValue)
                return "invalid-request: delete_with_items must be a whole number, the count of Items that still use the shape.";
            req.DeleteWithItems = (int)n;
        }

        if (root["not_ours_confirmed"] != null)
        {
            if (root["not_ours_confirmed"] is not JsonValue nv || !nv.TryGetValue<bool>(out var confirmed)) return "invalid-request: not_ours_confirmed must be true or false.";
            if (mode != "delete" && confirmed) return "invalid-request: not_ours_confirmed only belongs to a delete.";
            req.NotOursConfirmed = confirmed;
        }
        if (mode == "delete") return ParseDelete(root, req, out request);
        if (req.DeleteWithItems != null) return "invalid-request: delete_with_items only belongs to a delete.";

        if (root["members"] is not JsonArray members || members.Count == 0) return "invalid-request: members must list at least one member.";
        if (members.Count > MaxMembers) return $"invalid-request: {members.Count} members; one call takes at most {MaxMembers}.";
        var ids = new HashSet<string>(StringComparer.Ordinal);
        for (int i = 0; i < members.Count; i++)
        {
            if (members[i] is not JsonObject m) return $"invalid-request: members[{i}] must be an object.";
            var id = Text(m, "id");
            if (string.IsNullOrWhiteSpace(id) || id!.Length > 128) return $"invalid-request: members[{i}].id must be a non-empty text of at most 128 characters.";
            if (!ids.Add(id)) return $"invalid-request: member id '{id}' is used twice.";
            var who = $"member '{id}'";
            if (NotText(m, who + ": ", "label", "material", "definition_digest", "solid_digest") is { } badMember) return badMember;
            var member = new ItemMember { Id = id, Label = Text(m, "label") ?? id, DefinitionDigest = Text(m, "definition_digest") };
            if (member.Label.Length > 256 || member.Label.Any(char.IsControl)) return $"invalid-request: {who}: label must be at most 256 characters with no control characters.";
            if (m["material"] != null)
            {
                var material = Text(m, "material");
                if (string.IsNullOrWhiteSpace(material) || material!.Length > 64 || material.Any(char.IsControl)) return $"invalid-request: {who}: material must be a text of at most 64 characters.";
                member.Material = material;
            }
            if (ReadSolid(m["solid"], member.Solid, who) is { } solidProblem) return solidProblem;
            member.Solid.Normalize();
            if (member.Solid.Validate(who) is { } invalid) return "invalid-solid: " + invalid;
            member.SolidDigest = member.Solid.Digest();
            var claimed = Text(m, "solid_digest");
            if (claimed != null && !string.Equals(claimed, member.SolidDigest, StringComparison.OrdinalIgnoreCase))
                return $"digest-mismatch: {who}: solid_digest {claimed} is not the digest AWARE computes for the solid ({member.SolidDigest}).";
            if (m["placement"] is not JsonObject placement) return $"invalid-request: {who} has no placement.";
            if (Point(placement["start"]) is not { } start || Point(placement["end"]) is not { } end) return $"invalid-request: {who}: placement.start and placement.end must each be [x, y, z] in millimetres.";
            member.Start = start; member.End = end;
            if (placement["roll_deg"] != null)
            {
                if (placement["roll_deg"] is not JsonValue rv || !rv.TryGetValue<double>(out var roll) || double.IsNaN(roll) || double.IsInfinity(roll) || Math.Abs(roll) > 3600)
                    return $"invalid-request: {who}: placement.roll_deg must be a number of degrees.";
                member.RollDeg = roll;
            }
            var distance = Math.Sqrt(Math.Pow(end[0] - start[0], 2) + Math.Pow(end[1] - start[1], 2) + Math.Pow(end[2] - start[2], 2));
            if (Math.Abs(distance - member.Solid.Length) > GeometryToleranceMm)
                return $"length-mismatch: {who}: the end point is {distance.ToString("R", CultureInfo.InvariantCulture)} mm from the start but the solid is {member.Solid.Length.ToString("R", CultureInfo.InvariantCulture)} mm long. Tekla does not stretch a shape to its Item's length, so the two must agree.";
            req.Members.Add(member);
        }
        request = req;
        return null;
    }

    static string? ParseDelete(JsonObject root, ItemCreateRequest req, out ItemCreateRequest? request)
    {
        request = null;
        // A delete is meant to carry the proof a receipt gives (undo.request): each Item as {guid, name}, each shape as
        // {name, guid, geometry_guid}. A bare guid or name is accepted too, but it has no proof: the run answers
        // not-created-by-this-run and deletes nothing unless the caller confirmed (not_ours_confirmed).
        const string badEntry = "invalid-request: items must be [{\"guid\", \"name\"}] (or a bare guid) and shapes [{\"name\", \"guid\", \"geometry_guid\"}] (or a bare name).";
        if (root["items"] is JsonArray items)
            foreach (var node in items)
            {
                string? g, n = null;
                if (node is JsonValue iv && iv.TryGetValue<string>(out g)) { }
                else if (node is JsonObject io && Text(io, "guid") is { } og && Text(io, "name") is { } on) { g = og; n = on; }
                else return badEntry;
                if (g == null || !Guid.TryParse(g, out _)) return badEntry;
                if (req.ItemGuids.Contains(g, StringComparer.OrdinalIgnoreCase)) continue;
                req.ItemGuids.Add(g);
                if (n != null) req.ItemProof[g] = n;
            }
        else if (root["items"] != null) return badEntry;
        if (root["shapes"] is JsonArray shapes)
            foreach (var node in shapes)
            {
                string? s, sg = null, geo = null;
                if (node is JsonValue sv && sv.TryGetValue<string>(out s)) { }
                else if (node is JsonObject so && Text(so, "name") is { } sn && Text(so, "guid") is { } sgv && Guid.TryParse(sgv, out _)) { s = sn; sg = sgv; geo = Text(so, "geometry_guid"); if (geo != null && !Guid.TryParse(geo, out _)) return badEntry; }
                else return badEntry;
                if (string.IsNullOrEmpty(s) || s!.Length > 1023 || s.Any(char.IsControl)) return "invalid-request: a shape name must be 1 to 1023 characters with no control characters.";
                if (req.ShapeNames.Contains(s, StringComparer.OrdinalIgnoreCase)) continue;
                req.ShapeNames.Add(s);
                if (sg != null) req.ShapeProof[s] = (sg, geo);
            }
        else if (root["shapes"] != null) return badEntry;
        if (req.ItemGuids.Count == 0 && req.ShapeNames.Count == 0) return "invalid-request: a delete names at least one item or shape (the undo of a create lists them).";
        request = req;
        return null;
    }

    static string? ReadSolid(JsonNode? node, PrismSolid solid, string who)
    {
        if (node is not JsonObject s) return $"invalid-solid: {who} has no solid.";
        var kind = Text(s, "kind");
        if (kind != "prism") return $"invalid-solid: {who}: solid.kind '{kind}' is not 'prism', the only solid this verb builds (a section swept along its length).";
        if (s["length"] is not JsonValue lv || !lv.TryGetValue<double>(out var length)) return $"invalid-solid: {who}: solid.length must be a number of millimetres.";
        solid.Length = length;
        var outer = Ring(s["outer"]);
        if (outer == null) return $"invalid-solid: {who}: solid.outer must be a list of [y, z] points.";
        solid.Outer.AddRange(outer);
        if (s["holes"] != null)
        {
            if (s["holes"] is not JsonArray holes) return $"invalid-solid: {who}: solid.holes must be a list of rings.";
            foreach (var h in holes)
            {
                var ring = Ring(h);
                if (ring == null) return $"invalid-solid: {who}: every hole must be a list of [y, z] points.";
                solid.Holes.Add(ring);
            }
        }
        return null;
    }

    static List<double[]>? Ring(JsonNode? node)
    {
        if (node is not JsonArray points) return null;
        var ring = new List<double[]>();
        foreach (var p in points)
        {
            if (p is not JsonArray pair || pair.Count != 2) return null;
            if (pair[0] is not JsonValue a || !a.TryGetValue<double>(out var y) || pair[1] is not JsonValue b || !b.TryGetValue<double>(out var z)) return null;
            ring.Add(new[] { y, z });
        }
        return ring;
    }

    static double[]? Point(JsonNode? node)
    {
        if (node is not JsonArray p || p.Count != 3) return null;
        var c = new double[3];
        for (int i = 0; i < 3; i++)
        {
            if (p[i] is not JsonValue v || !v.TryGetValue<double>(out var d) || double.IsNaN(d) || double.IsInfinity(d) || Math.Abs(d) > 1e9) return null;
            c[i] = d;
        }
        return c;
    }

    /// <summary>A present, non-null field that is not text (a number read as "absent" would silently skip a check).</summary>
    static string? NotText(JsonObject o, string where, params string[] keys)
    {
        foreach (var key in keys)
            if (o[key] != null && !(o[key] is JsonValue v && v.TryGetValue<string>(out _)))
                return "invalid-request: " + (where == "request" ? "" : where.EndsWith(": ", StringComparison.Ordinal) ? where : where + ".") + key + " must be text.";
        return null;
    }

    static string? Text(JsonObject o, string key) => o[key] is JsonValue v && v.TryGetValue<string>(out var s) ? s : null;

    /// <summary>A short stable key for a model folder, for the backup path.</summary>
    public static string ModelKey(string modelPath)
    {
        using var sha = SHA256.Create();
        var normal = modelPath.Trim().TrimEnd('\\', '/').ToLowerInvariant();
        return string.Concat(sha.ComputeHash(Encoding.UTF8.GetBytes(normal)).Take(8).Select(b => b.ToString("x2", CultureInfo.InvariantCulture)));
    }

    /// <summary>The model's own GUID from its TeklaStructuresModel.xml, or null.</summary>
    public static string? ReadModelGuid(string modelPath)
    {
        try
        {
            var file = Path.Combine(modelPath, "TeklaStructuresModel.xml");
            if (!File.Exists(file)) return null;
            var m = System.Text.RegularExpressions.Regex.Match(File.ReadAllText(file), "<Guid>\\s*([0-9a-fA-F-]{36})\\s*</Guid>");
            return m.Success ? m.Groups[1].Value.ToLowerInvariant() : null;
        }
        catch (IOException) { return null; }
        catch (UnauthorizedAccessException) { return null; }
    }

    // ── run ───────────────────────────────────────────────────────────────────

    /// <summary>Parse the request JSON and run it against the host. A request the parser refuses throws ArgumentException.</summary>
    public static Dictionary<string, object?> RunJson(string requestJson, IItemCreateHost host)
    {
        JsonNode? node;
        try { node = JsonNode.Parse(requestJson); }
        catch (System.Text.Json.JsonException e) { throw new ArgumentException("invalid-request: not JSON: " + e.Message); }
        var refusal = Parse(node, out var request);
        if (refusal != null || request == null) throw new ArgumentException(refusal ?? "invalid-request");
        return Run(request, host, null);
    }

    /// <summary>
    /// The one entry point: checks in the order the design fixes, then writes, each step verified before the next.
    /// Always returns the same keys; <c>ok</c> is false for every status except planned, created and deleted.
    /// </summary>
    public static Dictionary<string, object?> Run(ItemCreateRequest req, IItemCreateHost host, string? runId)
    {
        runId ??= DateTime.UtcNow.ToString("yyyyMMdd'T'HHmmssfff'Z'", CultureInfo.InvariantCulture) + "-" + Guid.NewGuid().ToString("N").Substring(0, 6);
        var receipt = new Dictionary<string, object?>
        {
            ["status"] = "failed", ["ok"] = false, ["mode"] = req.Mode, ["run_id"] = runId,
            ["target"] = null, ["shapes"] = new List<object?>(), ["items"] = new List<object?>(),
            ["new_shape_count"] = 0, ["warnings"] = new List<object?>(),
            ["ledger"] = new List<object?>(), ["backup"] = null,
            ["exists_afterwards"] = new List<object?>(), ["undo"] = null,
        };
        var ledger = new List<LedgerEntry>();
        string? backupDir = null;
        IDictionary<string, string>? filesBefore = null;
        bool writing = false;
        List<ShapePlan>? plansSoFar = null;
        try
        {
            var target = host.ReadTarget();
            receipt["target"] = TargetRow(target);
            if (TargetChanged(req, target) is { } changed)
            {
                receipt["status"] = "target-changed";
                receipt["message"] = changed + " Nothing was written.";
                return receipt;
            }

            if (req.Mode == "delete") return RunDelete(req, host, runId, receipt, ledger);

            // One shape per distinct solid (a new length is a new shape); two members of the same solid share one.
            var plans = new List<ShapePlan>();
            plansSoFar = plans;
            var byDigest = new Dictionary<string, ShapePlan>(StringComparer.Ordinal);
            foreach (var m in req.Members)
            {
                if (!byDigest.TryGetValue(m.SolidDigest, out var plan))
                {
                    plan = new ShapePlan { Digest = m.SolidDigest, Solid = m.Solid, Name = ItemShapeNames.For(req.NamePrefix, m.SolidDigest), Label = m.Label };
                    if (ItemShapeNames.Validate(plan.Name) is { } why) throw new InvalidOperationException(why);
                    byDigest[m.SolidDigest] = plan;
                    plans.Add(plan);
                }
                plan.Members.Add(m);
                if (m.RollDeg != 0)
                    Warn(receipt, $"member '{m.Id}': roll_deg {m.RollDeg.ToString("R", CultureInfo.InvariantCulture)} is applied as Tekla's rotation offset about the member axis; its sign and its mapping from a FloLess roll were not yet proved live (only roll 0 was), so check the placed Item.");
            }
            receipt["new_shape_count"] = 0;

            // Check 6: an existing shape under the derived name is either this solid (reuse) or a conflict. Never overwritten.
            var conflicts = new List<object?>();
            foreach (var plan in plans)
            {
                var existing = host.FindShape(plan.Name);
                if (existing == null) { plan.Status = "would-create"; continue; }
                var difference = Differences(plan, existing);
                if (difference == null) { plan.Status = "reused-identical"; plan.Existing = existing; }
                else
                {
                    plan.Status = "conflict"; plan.Existing = existing;
                    conflicts.Add(new Dictionary<string, object?> { ["name"] = plan.Name, ["why"] = difference, ["existing"] = ShapeRow(existing), ["wanted_digest"] = plan.Digest, ["wanted_box"] = plan.Solid.Box });
                }
            }
            receipt["shapes"] = plans.Select(p => (object?)ShapePlanRow(p)).ToList();
            receipt["new_shape_count"] = plans.Count(p => p.Status == "would-create");
            if (conflicts.Count > 0)
            {
                receipt["status"] = "conflict";
                receipt["conflicts"] = conflicts;
                receipt["message"] = "A shape already holds a derived name with different content. Nothing was written, and nothing was overwritten.";
                return receipt;
            }

            // Check 8: Tekla's own validation of every brep about to be offered, before the first write.
            foreach (var plan in plans.Where(p => p.Status == "would-create"))
            {
                var problems = host.ValidateBrep(plan.Solid.BuildBrep());
                if (problems.Count > 0)
                {
                    receipt["status"] = "invalid-solid";
                    receipt["message"] = $"Tekla's validator rejects the solid for '{plan.Name}' (member '{plan.Members[0].Id}'): " + string.Join("; ", problems) + ". Nothing was written.";
                    return receipt;
                }
            }

            if (req.Mode == "plan")
            {
                receipt["status"] = "planned"; receipt["ok"] = true;
                receipt["items"] = req.Members.Select(m => (object?)new Dictionary<string, object?> { ["member_id"] = m.Id, ["status"] = "would-create", ["shape"] = ItemShapeNames.For(req.NamePrefix, m.SolidDigest) }).ToList();
                receipt["message"] = $"Would add {plans.Count(p => p.Status == "would-create")} shape(s) to the model's shape catalogue (one per distinct length), reuse {plans.Count(p => p.Status == "reused-identical")}, and place {req.Members.Count} Item(s). A shape with the same geometry under another name can only be found by trying the insert, so it shows up on create.";
                return receipt;
            }

            // Check 7: a verified copy of the catalogue files, before the first write.
            if (plans.Any(p => p.Status == "would-create"))
            {
                try { backupDir = host.BackupCatalogue(runId); }
                catch (Exception e)
                {
                    receipt["status"] = "backup-failed";
                    receipt["message"] = "The catalogue could not be backed up (" + e.Message + "), so nothing was written.";
                    return receipt;
                }
                receipt["backup"] = new Dictionary<string, object?> { ["folder"] = backupDir };
            }
            filesBefore = host.ListCatalogueFiles();

            writing = true;
            bool mismatch = false;
            foreach (var plan in plans)
            {
                if (plan.Status == "reused-identical") { plan.EffectiveName = plan.Name; continue; }
                var before = host.ListCatalogueFiles();
                var entry = new LedgerEntry { Kind = "shape", Name = plan.Name };
                ShapeInsertResult inserted;
                try { inserted = host.InsertShape(plan.Name, plan.Solid.BuildBrep()); }
                catch (Exception)
                {
                    // Tekla may have written before it threw. The name was absent before this call (checked above), so a
                    // shape under it now is this run's: record it, then let the failure through.
                    ShapeInfo? probe = null;
                    try { probe = host.FindShape(plan.Name); } catch (Exception) { /* the receipt's read-back says so */ }
                    if (probe != null)
                    {
                        entry.Guid = probe.Guid; entry.GeometryGuid = probe.GeometryGuid;
                        ledger.Add(entry);
                        plan.Status = "created"; plan.Guid = probe.Guid; plan.GeometryGuid = probe.GeometryGuid;
                        try { entry.Files = AddedFiles(before, host.ListCatalogueFiles()); } catch (Exception) { /* listed as unledgered below */ }
                    }
                    throw;
                }
                if (inserted.Inserted)
                {
                    // Recorded the moment Tekla says it wrote, before anything else can fail (even listing the files).
                    ledger.Add(entry);
                    plan.Status = "created";
                    try { entry.Files = AddedFiles(before, host.ListCatalogueFiles()); } catch (Exception) { /* listed as unledgered below */ }
                    var info = host.FindShape(plan.Name);
                    if (info != null) { entry.Guid = info.Guid; entry.GeometryGuid = info.GeometryGuid; }
                    plan.Guid = entry.Guid; plan.GeometryGuid = entry.GeometryGuid;
                    plan.MetadataSet = host.SetShapeMetadata(plan.Name, plan.Digest, plan.Label);
                    if (plan.MetadataSet != true)
                    {
                        // Without its digest the shape cannot be told from somebody else's on a later call (it would read as a conflict).
                        mismatch = true;
                        plan.Message = "Tekla did not accept the aware.digest tag on this shape; it was created, but a later call cannot recognise it as this verb's.";
                        Warn(receipt, $"shape '{plan.Name}': " + plan.Message);
                    }
                    var back = host.FindShape(plan.Name);
                    var difference = back == null ? "Tekla cannot find the shape it just inserted" : GeometryDifference(plan, back);
                    if (difference != null)
                    {
                        plan.Status = "read-back-mismatch"; plan.Message = difference;
                        throw new StopRun("read-back-mismatch", $"The shape '{plan.Name}' reads back differently from what was sent: {difference}. No Item was placed on it.");
                    }
                    plan.EffectiveName = plan.Name;
                }
                else if (inserted.MatchingGuids.Count > 0)
                {
                    ShapeInfo? same = null;
                    foreach (var guid in inserted.MatchingGuids) { same = host.FindShapeByGuid(guid); if (same != null) break; }
                    if (same == null) throw new InvalidOperationException("Tekla reports a shape with this geometry (guid " + inserted.MatchingGuids[0] + ") but cannot select it by that guid.");
                    plan.Status = "reused-by-geometry"; plan.Existing = same; plan.EffectiveName = same.Name;
                    plan.Message = $"Tekla already holds a shape with exactly this geometry, '{same.Name}'; it is reused and nothing was added for '{plan.Name}'.";
                }
                else throw new InvalidOperationException("Tekla did not insert the shape '" + plan.Name + "'" + (inserted.Refusal == null ? "." : ": " + inserted.Refusal));
            }

            var itemRows = new List<object?>();
            bool stopPlacing = false;
            foreach (var plan in plans)
                foreach (var m in plan.Members)
                {
                    if (stopPlacing)
                    {
                        itemRows.Add(new Dictionary<string, object?> { ["member_id"] = m.Id, ["shape"] = plan.EffectiveName, ["status"] = "not-placed" });
                        continue;
                    }
                    var spec = new ItemSpec
                    {
                        MemberId = m.Id, Name = m.Label, ShapeName = plan.EffectiveName!, Start = m.Start, End = m.End,
                        RollDeg = m.RollDeg, Material = m.Material, Digest = plan.Digest, DefinitionDigest = m.DefinitionDigest,
                    };
                    string? placedGuid = null;
                    try { placedGuid = host.InsertItem(spec); }
                    finally
                    {
                        // Recorded even when the call throws after Tekla accepted the Item (a failed commit, say).
                        var known = placedGuid ?? spec.PlacedGuid;
                        if (known != null) ledger.Add(new LedgerEntry { Kind = "item", Name = m.Label, Guid = known, MemberId = m.Id, ShapeName = plan.EffectiveName });
                    }
                    if (placedGuid == null) throw new InvalidOperationException($"Tekla did not place the Item for member '{m.Id}'.");
                    var placed = host.MeasureItem(spec, placedGuid);
                    var verification = Verify(m, placed, plan.EffectiveName!, out var itemOk);
                    if (!itemOk) { mismatch = true; stopPlacing = true; }
                    if (placed.UdaSet == false) Warn(receipt, $"member '{m.Id}': Tekla did not accept the AWARE_* user properties on the Item; it is still labelled by its name, but a later delete cannot use them to recognise it.");
                    itemRows.Add(new Dictionary<string, object?>
                    {
                        ["member_id"] = m.Id, ["item_guid"] = placedGuid, ["shape"] = plan.EffectiveName, ["status"] = itemOk ? "created" : "read-back-mismatch",
                        ["verification"] = verification,
                    });
                    receipt["items"] = new List<object?>(itemRows);
                }
            receipt["shapes"] = plans.Select(p => (object?)ShapePlanRow(p)).ToList();
            receipt["new_shape_count"] = plans.Count(p => p.Status == "created");
            receipt["items"] = itemRows;
            receipt["status"] = mismatch ? "read-back-mismatch" : "created";
            receipt["ok"] = !mismatch;
            if (mismatch) receipt["message"] = "Something written does not read back as asked (see shapes[].message and items[].verification). What was written stays in the model and nothing claims a match" + (stopPlacing ? "; placing stopped at the first Item that did not match, and the members after it have status not-placed" : "") + ". Check it, or use the undo.";
            FillAfterwards(receipt, ledger, host);
            RefreshLedgerHashes(ledger, host);
            FillLedger(receipt, ledger, backupDir);
            return receipt;
        }
        catch (Exception e)
        {
            // Anything after the first write is a partial result: say exactly what exists, offer the exact undo, never retry.
            string status = e is StopRun stop ? stop.Status : "failed";
            string message = e.Message;
            var unledgered = new List<object?>();
            if (writing)
            {
                try
                {
                    var after = host.ListCatalogueFiles();
                    var known = new HashSet<string>(ledger.SelectMany(l => l.Files.Select(f => f.Path)), StringComparer.OrdinalIgnoreCase);
                    foreach (var f in AddedFiles(filesBefore!, after)) if (!known.Contains(f.Path)) unledgered.Add(new Dictionary<string, object?> { ["path"] = f.Path, ["sha256"] = f.Sha256 });
                }
                catch (Exception) { /* the receipt below says the read-back failed */ }
                if (ledger.Count > 0 || unledgered.Count > 0) status = status == "read-back-mismatch" ? status : "partial";
            }
            receipt["status"] = status; receipt["ok"] = false; receipt["error"] = message;
            receipt["unledgered_files"] = unledgered;
            if (unledgered.Count > 0) Warn(receipt, "Tekla wrote catalogue files that the run did not record; they are listed in unledgered_files and are NOT part of the undo. Restore them from the backup with Tekla closed.");
            if (writing)
            {
                if (plansSoFar != null) receipt["shapes"] = plansSoFar.Select(p => (object?)ShapePlanRow(p)).ToList();
                FillAfterwards(receipt, ledger, host);
                RefreshLedgerHashes(ledger, host);
                FillLedger(receipt, ledger, backupDir);
            }
            return receipt;
        }
    }

    // ── delete (also the undo of a create) ────────────────────────────────────

    static Dictionary<string, object?> RunDelete(ItemCreateRequest req, IItemCreateHost host, string runId, Dictionary<string, object?> receipt, List<LedgerEntry> ledger)
    {
        // Phase 1, read only: what is there, what the run's ledger vouches for, and which Items would be left holding a deleted shape.
        var listed = new HashSet<string>(req.ItemGuids, StringComparer.OrdinalIgnoreCase);
        var shapeRows = new List<object?>();
        var affected = new List<object?>();
        int remaining = 0;
        var present = new List<string>();
        var allowedLeft = new Dictionary<string, int>(StringComparer.OrdinalIgnoreCase);
        var notOurs = new List<Dictionary<string, object?>>();
        foreach (var name in req.ShapeNames)
        {
            var info = host.FindShape(name);
            if (info == null) { shapeRows.Add(new Dictionary<string, object?> { ["name"] = name, ["status"] = "absent" }); continue; }
            // Vouched for only when the shape under this name is still the one (guid, geometry guid) the run wrote.
            string? reason = null;
            if (!req.ShapeProof.TryGetValue(name, out var proof)) reason = "missing";
            else if (!string.Equals(info.Guid, proof.guid, StringComparison.OrdinalIgnoreCase) || (proof.geometryGuid != null && !string.Equals(info.GeometryGuid, proof.geometryGuid, StringComparison.OrdinalIgnoreCase))) reason = "guid-mismatch";
            if (reason != null)
                notOurs.Add(new Dictionary<string, object?>
                {
                    ["kind"] = "shape", ["name"] = info.Name, ["guid"] = info.Guid, ["reason"] = reason,
                    ["why"] = reason == "missing" ? "no ledger proof was given for it" : "its guid is not the one the run recorded (" + proof.guid + "): a different shape now holds that name",
                });
            var users = host.ItemsUsingShape(name);
            var others = users.Where(u => !listed.Contains(u.Guid)).ToList();
            // The instance count is Tekla's own; the users list is read off the model. The larger one is the one to respect.
            int left = Math.Max(others.Count, (info.InstanceCount ?? 0) - users.Count(u => listed.Contains(u.Guid)));
            if (info.InstanceCount == null) Warn(receipt, $"Tekla would not count the Items that use '{name}'; only a scan of the model's Items was used.");
            remaining += left;
            present.Add(name);
            allowedLeft[name] = left;
            shapeRows.Add(new Dictionary<string, object?> { ["name"] = name, ["status"] = "present", ["guid"] = info.Guid, ["instance_count"] = info.InstanceCount, ["items_left_after_listed_are_deleted"] = left });
            foreach (var u in others) affected.Add(new Dictionary<string, object?> { ["item_guid"] = u.Guid, ["name"] = u.Name, ["shape"] = name });
        }
        var skipNotBrep = new HashSet<string>(StringComparer.OrdinalIgnoreCase);
        foreach (var guid in req.ItemGuids)
        {
            var item = host.ReadItem(guid);
            if (item.NotBrep)
            {
                skipNotBrep.Add(guid);
                notOurs.Add(new Dictionary<string, object?> { ["kind"] = "item", ["guid"] = guid, ["reason"] = "not-a-brep", ["why"] = "the guid names a model object that is not a Brep Item; this verb never deletes it" });
                continue;
            }
            if (!item.Exists) continue;
            string? reason = null;
            if (!req.ItemProof.TryGetValue(guid, out var proofName)) reason = "missing";
            else if (!string.Equals(item.Name, proofName, StringComparison.Ordinal)) reason = "name-mismatch";
            if (reason != null)
                notOurs.Add(new Dictionary<string, object?>
                {
                    ["kind"] = "item", ["name"] = item.Name, ["guid"] = guid, ["reason"] = reason,
                    ["why"] = reason == "missing" ? "no ledger proof was given for it" : "its name is not the one the run recorded (" + proofName + ")",
                });
        }
        receipt["shapes"] = shapeRows;
        receipt["items_using_shapes"] = affected;
        receipt["instance_count_remaining"] = remaining;
        if (notOurs.Count > 0)
            receipt["not_created_by_this_run"] = notOurs.Cast<object?>().ToList();
        if (notOurs.Count > 0 && !req.NotOursConfirmed)
        {
            receipt["status"] = "not-created-by-this-run";
            receipt["message"] = "The ledger of the run that wrote these does not vouch for everything this delete names (see not_created_by_this_run: each with its name, guid and why). Nothing was deleted. If the person confirms they want them deleted anyway, repeat the call with --not-ours-confirmed (not_ours_confirmed: true).";
            return receipt;
        }
        if (remaining > 0)
        {
            if (req.DeleteWithItems == null)
            {
                receipt["status"] = "shape-in-use";
                receipt["message"] = $"{remaining} Item(s) still use the shape(s) to delete. Tekla would delete a shape anyway and leave those Items without their profile. Nothing was deleted. To go on, pass delete_with_items = {remaining} (or --delete-with-items {remaining}), or list those Items to delete them first.";
                return receipt;
            }
            if (req.DeleteWithItems != remaining)
            {
                receipt["status"] = "delete-count-mismatch";
                receipt["message"] = $"delete_with_items is {req.DeleteWithItems} but {remaining} Item(s) still use the shape(s). The acknowledgement must name the current count. Nothing was deleted.";
                return receipt;
            }
        }
        else if (req.DeleteWithItems != null && req.DeleteWithItems != 0)
        {
            receipt["status"] = "delete-count-mismatch";
            receipt["message"] = $"delete_with_items is {req.DeleteWithItems} but no Item will still use the shape(s) once the listed Items are deleted. Nothing was deleted.";
            return receipt;
        }
        if (notOurs.Count > 0)
        {
            // Recorded only now that the call goes ahead: the receipt says what was deleted on the person's say-so alone.
            receipt["override"] = "not-ours-confirmed";
            receipt["override_applied_to"] = notOurs.Where(n => (string?)n["reason"] != "not-a-brep").Cast<object?>().ToList();
            if (skipNotBrep.Count > 0) Warn(receipt, "Items that are not Brep Items were not deleted: " + string.Join(", ", skipNotBrep) + ".");
        }

        // Phase 2, the writes: a verified copy of the catalogue first, then Items, then shapes.
        string? backupDir = null;
        bool wrote = false;
        try
        {
            if (present.Count > 0)
            {
                try { backupDir = host.BackupCatalogue(runId); }
                catch (Exception e)
                {
                    receipt["status"] = "backup-failed";
                    receipt["message"] = "The catalogue could not be backed up (" + e.Message + "), so nothing was deleted.";
                    return receipt;
                }
                receipt["backup"] = new Dictionary<string, object?> { ["folder"] = backupDir };
            }
            var deletedItems = new List<object?>();
            var deletedShapes = new List<object?>();
            receipt["items"] = deletedItems; receipt["deleted_shapes"] = deletedShapes;   // filled as it goes, so a failure still lists what is gone
            foreach (var guid in req.ItemGuids)
            {
                if (skipNotBrep.Contains(guid)) { deletedItems.Add(new Dictionary<string, object?> { ["item_guid"] = guid, ["status"] = "skipped-not-a-brep" }); continue; }
                var item = host.ReadItem(guid);
                if (!item.Exists) { deletedItems.Add(new Dictionary<string, object?> { ["item_guid"] = guid, ["status"] = "absent" }); continue; }
                wrote = true;
                bool ok = host.DeleteItem(guid);
                deletedItems.Add(new Dictionary<string, object?> { ["item_guid"] = guid, ["status"] = ok ? "deleted" : "not-deleted" });
                if (!ok) throw new InvalidOperationException("Tekla did not delete the Item " + guid + ".");
            }
            foreach (var name in present)
            {
                // The guard is checked again at the write: Items may have been placed since it was first read.
                var nowCount = Math.Max(host.ItemsUsingShape(name).Count, host.FindShape(name)?.InstanceCount ?? 0);
                if (nowCount > allowedLeft[name])
                    throw new StopRun("delete-count-mismatch", $"{nowCount} Item(s) now use '{name}', more than the {allowedLeft[name]} the delete was acknowledged for. It was not deleted.");
                var before = host.ListCatalogueFiles();
                wrote = true;
                bool ok = host.DeleteShape(name);
                var after = host.ListCatalogueFiles();
                var removed = before.Keys.Where(k => !after.ContainsKey(k)).ToList();
                deletedShapes.Add(new Dictionary<string, object?> { ["name"] = name, ["status"] = ok ? "deleted" : "not-deleted", ["files_removed"] = removed });
                if (!ok) throw new InvalidOperationException("Tekla did not delete the shape '" + name + "'.");
            }
            if (remaining > 0)
                Warn(receipt, $"{remaining} Item(s) now have no shape. Tekla keeps their last solid but they no longer name a profile; they are listed in items_using_shapes.");
            receipt["status"] = "deleted"; receipt["ok"] = true;
            return receipt;
        }
        catch (Exception e)
        {
            receipt["status"] = wrote ? "partial" : e is StopRun stopped ? stopped.Status : "failed"; receipt["ok"] = false; receipt["error"] = e.Message;
            if (backupDir != null) receipt["backup"] = new Dictionary<string, object?> { ["folder"] = backupDir };
            var left = new List<object?>();
            foreach (var guid in req.ItemGuids) { try { if (host.ReadItem(guid).Exists) left.Add(new Dictionary<string, object?> { ["kind"] = "item", ["guid"] = guid }); } catch (Exception) { left.Add(new Dictionary<string, object?> { ["kind"] = "item", ["guid"] = guid, ["read_back"] = "failed" }); } }
            foreach (var name in present) { try { if (host.FindShape(name) != null) left.Add(new Dictionary<string, object?> { ["kind"] = "shape", ["name"] = name }); } catch (Exception) { left.Add(new Dictionary<string, object?> { ["kind"] = "shape", ["name"] = name, ["read_back"] = "failed" }); } }
            receipt["exists_afterwards"] = left;
            receipt["undo"] = "A delete has no undo through this verb: restore the shape files from the backup folder with Tekla closed (a deleted Item cannot be restored; it can only be placed again).";
            return receipt;
        }
    }

    // ── helpers ───────────────────────────────────────────────────────────────

    sealed class StopRun : Exception
    {
        public StopRun(string status, string message) : base(message) { Status = status; }
        public string Status { get; }
    }

    sealed class LedgerEntry
    {
        public string Kind = "";
        public string Name = "";
        public string? Guid;
        public string? GeometryGuid;
        public string? MemberId;
        public string? ShapeName;
        public List<FileRow> Files = new List<FileRow>();
    }

    sealed class FileRow
    {
        public string Path = "";
        public string Sha256 = "";
    }

    sealed class ShapePlan
    {
        public string Name = "";
        public string? EffectiveName;
        public string Digest = "";
        public string Label = "";
        public PrismSolid Solid = new PrismSolid();
        public List<ItemMember> Members = new List<ItemMember>();
        public string Status = "would-create";
        public ShapeInfo? Existing;
        public string? Guid;
        public string? GeometryGuid;
        public bool? MetadataSet;
        public string? Message;
    }

    static List<FileRow> AddedFiles(IDictionary<string, string> before, IDictionary<string, string> after) =>
        after.Where(kv => !before.ContainsKey(kv.Key) || !string.Equals(before[kv.Key], kv.Value, StringComparison.OrdinalIgnoreCase))
             .Select(kv => new FileRow { Path = kv.Key, Sha256 = kv.Value }).OrderBy(f => f.Path, StringComparer.OrdinalIgnoreCase).ToList();

    static void Warn(Dictionary<string, object?> receipt, string text) => ((List<object?>)receipt["warnings"]!).Add(text);

    static string? TargetChanged(ItemCreateRequest req, ItemTarget actual)
    {
        string Norm(string? p) => (p ?? "").Trim().TrimEnd('\\', '/').ToLowerInvariant();
        if (req.ExpectModelPath != null && Norm(req.ExpectModelPath) != Norm(actual.ModelPath))
            return $"The open model is '{actual.ModelPath}', not '{req.ExpectModelPath}' that the person chose.";
        if (req.ExpectModelGuid != null && !string.Equals(req.ExpectModelGuid, actual.ModelGuid, StringComparison.OrdinalIgnoreCase))
            return $"The open model's GUID is {actual.ModelGuid ?? "unknown"}, not {req.ExpectModelGuid}: it is not the model the person chose.";
        if (req.ExpectShared != null && actual.SharedModel != null && req.ExpectShared != actual.SharedModel)
            return $"The open model is {(actual.SharedModel == true ? "a shared (multi-user)" : "not a shared")} model, which is not what the person was told.";
        if (req.ExpectTeklaVersion != null && actual.TeklaVersion != null && !actual.TeklaVersion.StartsWith(req.ExpectTeklaVersion, StringComparison.OrdinalIgnoreCase))
            return $"Tekla is version {actual.TeklaVersion}, not {req.ExpectTeklaVersion}.";
        return null;
    }

    static Dictionary<string, object?> TargetRow(ItemTarget t) => new Dictionary<string, object?>
    {
        ["model_path"] = t.ModelPath, ["model_guid"] = t.ModelGuid, ["shared_model"] = t.SharedModel, ["single_user_model"] = t.SingleUserModel,
        ["tekla_version"] = t.TeklaVersion, ["write_level"] = "model",
        ["shared_note"] = t.SharedModel == true
            ? "This is a shared (multi-user) model: the shapes added here are seen by everyone on it."
            : t.SharedModel == false ? "Single-user model: the shapes are in this model only." : "Tekla did not say whether the model is shared.",
    };

    static Dictionary<string, object?> ShapeRow(ShapeInfo s) => new Dictionary<string, object?>
    {
        ["name"] = s.Name, ["guid"] = s.Guid, ["geometry_guid"] = s.GeometryGuid, ["digest"] = s.Digest, ["extrema"] = s.Extrema, ["instance_count"] = s.InstanceCount,
    };

    static Dictionary<string, object?> ShapePlanRow(ShapePlan p)
    {
        var row = new Dictionary<string, object?>
        {
            ["name"] = p.EffectiveName ?? p.Name, ["derived_name"] = p.Name, ["digest"] = p.Digest, ["length_mm"] = p.Solid.Length, ["member_ids"] = p.Members.Select(m => m.Id).ToList(),
            ["status"] = p.Status, ["guid"] = p.Guid ?? p.Existing?.Guid, ["geometry_guid"] = p.GeometryGuid ?? p.Existing?.GeometryGuid,
            ["metadata_set"] = p.MetadataSet, ["message"] = p.Message,
        };
        return row;
    }

    /// <summary>Why an existing shape is not this plan's solid, or null when it is the same (digest, box and every vertex).</summary>
    static string? Differences(ShapePlan plan, ShapeInfo existing)
    {
        if (!string.Equals(existing.Name, plan.Name, StringComparison.Ordinal))
            return $"Tekla matched the name case-insensitively with '{existing.Name}'.";
        if (existing.Digest == null) return "it carries no aware.digest, so it was not made by this verb for this solid.";
        if (!string.Equals(existing.Digest, plan.Digest, StringComparison.OrdinalIgnoreCase)) return $"its aware.digest is {existing.Digest}, not {plan.Digest}.";
        return GeometryDifference(plan, existing);
    }

    static string? GeometryDifference(ShapePlan plan, ShapeInfo info)
    {
        var box = plan.Solid.Box;
        var tolerance = GeometryToleranceMm + 2e-7 * Math.Max(1.0, box.Max(Math.Abs));
        if (info.Extrema == null || info.Extrema.Length != 6) return "Tekla gave no extents for it.";
        for (int i = 0; i < 6; i++)
            if (Math.Abs(info.Extrema[i] - box[i]) > tolerance) return $"its extents differ from the solid's (index {i}: {info.Extrema[i].ToString("R", CultureInfo.InvariantCulture)} vs {box[i].ToString("R", CultureInfo.InvariantCulture)}).";
        var sent = plan.Solid.BuildBrep().Vertices;
        if (info.Vertices.Count != sent.Count) return $"it has {info.Vertices.Count} vertices, the solid {sent.Count}.";
        double worst = Math.Max(Dev(sent, info.Vertices), Dev(info.Vertices, sent));
        if (worst > tolerance) return $"a vertex is {worst.ToString("R", CultureInfo.InvariantCulture)} mm from the solid's.";
        return null;
    }

    static double Dev(List<double[]> source, List<double[]> target) =>
        source.Max(a => target.Min(b => Math.Sqrt(Math.Pow(a[0] - b[0], 2) + Math.Pow(a[1] - b[1], 2) + Math.Pow(a[2] - b[2], 2))));

    /// <summary>Compare what Tekla reports for a placed Item with what was asked; every number is in the output.</summary>
    static Dictionary<string, object?> Verify(ItemMember m, ItemReadback r, string expectedShape, out bool ok)
    {
        var v = new Dictionary<string, object?>();
        var problems = new List<object?>();
        double? boxDeviation = r.MaxSolidToShapeMm == null && r.MaxShapeToSolidMm == null ? (double?)null : Math.Max(r.MaxSolidToShapeMm ?? 0, r.MaxShapeToSolidMm ?? 0);
        double? ratio = r.VolumeFromFaces == null ? (double?)null : r.VolumeFromFaces.Value / m.Solid.Volume;
        v["box_max_deviation_mm"] = boxDeviation; v["volume_ratio"] = ratio; v["origin_to_start_mm"] = r.OriginToStartMm; v["axis_dot"] = r.AxisXDot;
        v["extent_along_axis_mm"] = r.ExtentAlongAxisX; v["shape_instance_count"] = r.ShapeInstanceCount; v["uda_set"] = r.UdaSet;
        if (boxDeviation == null || boxDeviation > PlacementToleranceMm) problems.Add("the solid does not coincide with the shape moved by Tekla's own coordinate system");
        if (ratio == null || Math.Abs(ratio.Value - 1) > 1e-5) problems.Add("the volume Tekla's faces give is not the section area times the length");
        if (r.OriginToStartMm == null || r.OriginToStartMm > PlacementToleranceMm) problems.Add("the shape's origin is not on the start point");
        if (r.AxisXDot == null || r.AxisXDot < 1 - 1e-6) problems.Add("the shape's X axis does not run start to end");
        if (r.ExtentAlongAxisX == null || Math.Abs(r.ExtentAlongAxisX.Value - m.Solid.Length) > PlacementToleranceMm) problems.Add("the Item is not as long as the solid");
        if (!string.Equals(r.ShapeName, expectedShape, StringComparison.Ordinal)) problems.Add("the Item does not name the expected shape (" + expectedShape + ")");
        v["problems"] = problems;
        ok = problems.Count == 0;
        return v;
    }

    /// <summary>
    /// A shape's files are hashed when Tekla says it inserted them, which is before the digest is written into its record;
    /// the receipt's sha256_after must describe what is left on disk, so every ledgered file is hashed again at the end.
    /// </summary>
    static void RefreshLedgerHashes(List<LedgerEntry> ledger, IItemCreateHost host)
    {
        // A shape whose guid could not be read when it was written is read again, so the undo can carry its proof.
        foreach (var entry in ledger.Where(l => l.Kind == "shape" && l.Guid == null))
        {
            try { var s = host.FindShape(entry.Name); if (s != null) { entry.Guid = s.Guid; entry.GeometryGuid = s.GeometryGuid; } }
            catch (Exception) { /* stays without proof; the receipt says so */ }
        }
        IDictionary<string, string> now;
        try { now = host.ListCatalogueFiles(); }
        catch (Exception) { return; }
        foreach (var entry in ledger)
            foreach (var file in entry.Files)
                if (now.TryGetValue(file.Path, out var hash)) file.Sha256 = hash;
    }

    static void FillLedger(Dictionary<string, object?> receipt, List<LedgerEntry> ledger, string? backupDir)
    {
        receipt["ledger"] = ledger.Select(l => (object?)new Dictionary<string, object?>
        {
            ["kind"] = l.Kind, ["name"] = l.Name, ["guid"] = l.Guid, ["geometry_guid"] = l.GeometryGuid, ["member_id"] = l.MemberId, ["shape"] = l.ShapeName,
            ["files"] = l.Files.Select(f => (object?)new Dictionary<string, object?> { ["path"] = f.Path, ["sha256_after"] = f.Sha256 }).ToList(),
        }).ToList();
        if (backupDir != null) receipt["backup"] = new Dictionary<string, object?> { ["folder"] = backupDir };
        // The proof the delete checks: each Item's guid and name, each shape's name, guid and geometry guid, as recorded.
        var items = ledger.Where(l => l.Kind == "item" && l.Guid != null).Select(l => (object?)new Dictionary<string, object?> { ["guid"] = l.Guid, ["name"] = l.Name }).ToList();
        var shapes = ledger.Where(l => l.Kind == "shape" && l.Guid != null).Select(l => (object?)new Dictionary<string, object?> { ["name"] = l.Name, ["guid"] = l.Guid, ["geometry_guid"] = l.GeometryGuid }).ToList();
        var unprovable = ledger.Where(l => l.Kind == "shape" && l.Guid == null).Select(l => (object?)l.Name).ToList();
        if (items.Count == 0 && shapes.Count == 0 && unprovable.Count == 0) { receipt["undo"] = null; return; }
        if (unprovable.Count > 0) receipt["shapes_without_proof"] = unprovable;
        receipt["undo"] = new Dictionary<string, object?>
        {
            ["what"] = "Deletes exactly what this run wrote, in this order: the Items listed, then the shapes listed (only shapes this run added; a shape it reused is left alone), each only if it still matches the guid and name recorded here. Run it as a second, explicit call with the person's consent; it is never done automatically." + (unprovable.Count > 0 ? " Shapes in shapes_without_proof could not be read back, so they are not in this request: remove their files from the backup comparison with Tekla closed." : ""),
            // Bound to the model this run wrote to: shape names are the same in every model, so an undo run against another
            // open model must be refused (target-changed), not delete that model's identical shapes.
            ["request"] = new Dictionary<string, object?>
            {
                ["mode"] = "delete", ["items"] = items, ["shapes"] = shapes,
                ["target"] = new Dictionary<string, object?> { ["model_path"] = ((Dictionary<string, object?>)receipt["target"]!)["model_path"], ["model_guid"] = ((Dictionary<string, object?>)receipt["target"]!)["model_guid"] },
                ["consent"] = new Dictionary<string, object?> { ["source"] = "ui-click" },
            },
            ["restore_files_instead"] = backupDir == null ? null : "With Tekla closed, remove the files named in ledger[].files from the model's Shapes and ShapeGeometries folders; the pre-write copy is in " + backupDir + ".",
        };
    }

    /// <summary>What exists now, by read-back from Tekla rather than from intention.</summary>
    static void FillAfterwards(Dictionary<string, object?> receipt, List<LedgerEntry> ledger, IItemCreateHost host)
    {
        var rows = new List<object?>();
        foreach (var l in ledger)
        {
            try
            {
                if (l.Kind == "shape")
                {
                    var s = host.FindShape(l.Name);
                    rows.Add(new Dictionary<string, object?> { ["kind"] = "shape", ["name"] = l.Name, ["exists"] = s != null, ["guid"] = s?.Guid, ["instance_count"] = s?.InstanceCount });
                }
                else
                {
                    var i = host.ReadItem(l.Guid!);
                    rows.Add(new Dictionary<string, object?> { ["kind"] = "item", ["guid"] = l.Guid, ["member_id"] = l.MemberId, ["exists"] = i.Exists, ["shape"] = i.ShapeName });
                }
            }
            catch (Exception e)
            {
                rows.Add(new Dictionary<string, object?> { ["kind"] = l.Kind, ["name"] = l.Name, ["guid"] = l.Guid, ["read_back"] = "failed: " + e.Message });
            }
        }
        receipt["exists_afterwards"] = rows;
    }
}
