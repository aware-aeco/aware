namespace AwareTekla;

/// <summary>
/// `section-measure` (#800, FloLess #3382 A0m): Tekla's own evaluated cross-section for a batch of profile
/// names, the point its member axis passes through, and where the section sits in world space under each
/// requested position. It is the oracle a caller proves its own section definitions against.
///
/// The section comes from the catalogue (<c>CrossSection.Select</c>), which changes nothing. The reference
/// point and every placement are measured on a real temporary member: inserted, its solid read, and deleted
/// in a <c>finally</c>, never committed. A delete that fails is reported, never hidden. It lives as a
/// Roslyn script, like <see cref="ProfileCatalogScript"/>, so the bridge stays version-neutral.
/// </summary>
internal static class SectionMeasureScript
{
    /// <summary>The Name every temporary member carries, so a person who sees one knows what made it.</summary>
    internal const string TemporaryPartName = "AWARE_SECTION_MEASURE";

    /// <summary>Upper bound on temporary members one request may insert.</summary>
    internal const int MaxInserts = 5000;

    /// <summary>The script's using directives.</summary>
    internal const string Usings = """
using System;
using System.Collections;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using Tekla.Structures;
using Tekla.Structures.Catalogs;
using Tekla.Structures.Geometry3d;
using Tekla.Structures.Model;
using Tekla.Structures.Solid;
""";

    /// <summary>The pure-geometry helpers (BCL only), separate so tests can execute them.</summary>
    internal const string Geometry = """
// Pure geometry, BCL only: executed directly by the unit tests (SectionMeasureTests), not only parsed.
const double MatchTolerance = 1e-3;
// ── Geometry helpers ──────────────────────────────────────────────────────────
Func<double, double> clean = x => x == 0 ? 0 : x; // folds -0 to 0; every other value is Tekla's, unrounded
Func<List<double[]>, double> signedArea = pts =>
{
    double a = 0;
    for (int k = 0; k < pts.Count; k++)
    {
        var p = pts[k];
        var q = pts[(k + 1) % pts.Count];
        a += p[0] * q[1] - q[0] * p[1];
    }
    return a / 2;
};
Func<List<double[]>, object> bounds2 = pts =>
{
    double x0 = Double.MaxValue, y0 = Double.MaxValue, x1 = Double.MinValue, y1 = Double.MinValue;
    foreach (var p in pts) { x0 = Math.Min(x0, p[0]); y0 = Math.Min(y0, p[1]); x1 = Math.Max(x1, p[0]); y1 = Math.Max(y1, p[1]); }
    return new Dictionary<string, object> { { "min", new[] { clean(x0), clean(y0) } }, { "max", new[] { clean(x1), clean(y1) } } };
};
Func<List<double[]>, List<double[]>> asJson2 = pts => pts.Select(p => new[] { clean(p[0]), clean(p[1]) }).ToList();
// The axis-aligned maps (of the eight 90° rotations/reflections, each with a translation) that carry the
// catalogue section onto the measured start face. Tekla tessellates a rounded corner differently in the
// catalogue and in the solid, so a map is proved on the section's exact anchors (see anchorsOf: the
// unrounded vertices and the ends of each rounding arc, all of which both sides share) plus the two
// bounding boxes, every one within MatchTolerance. A symmetric section matches under several maps; all of them are returned.
// The anchors of one loop of defining vertices, each { x, y, rounded (1 or 0), radius }: an unrounded vertex
// itself, and for a rounded vertex the two points where its arc meets the neighbouring edges (at r / tan(θ/2)
// from the corner along each edge, θ being the corner angle). Both of Tekla's tessellations contain those
// points; only the points between them differ. A vertex chamfered any other way contributes no anchor.
Func<List<double[]>, List<double[]>> anchorsOf = loop =>
{
    var anchors = new List<double[]>();
    for (int k = 0; k < loop.Count; k++)
    {
        var v = loop[k];
        if (v[2] == 0) { anchors.Add(new[] { v[0], v[1] }); continue; }
        if (v[2] != 1 || !(v[3] > 0)) continue;
        var prev = loop[(k + loop.Count - 1) % loop.Count];
        var next = loop[(k + 1) % loop.Count];
        double ax = prev[0] - v[0], ay = prev[1] - v[1], bx = next[0] - v[0], by = next[1] - v[1];
        double la = Math.Sqrt(ax * ax + ay * ay), lb = Math.Sqrt(bx * bx + by * by);
        if (la == 0 || lb == 0) continue;
        ax /= la; ay /= la; bx /= lb; by /= lb;
        double theta = Math.Acos(Math.Max(-1, Math.Min(1, ax * bx + ay * by)));
        if (!(theta > 1e-9) || !(theta < Math.PI - 1e-9)) continue;
        double d = v[3] / Math.Tan(theta / 2);
        anchors.Add(new[] { v[0] + d * ax, v[1] + d * ay });
        anchors.Add(new[] { v[0] + d * bx, v[1] + d * by });
    }
    return anchors;
};
var orientations = new List<int[]> {
    new[] { 1, 0, 0, 1 }, new[] { 0, -1, 1, 0 }, new[] { -1, 0, 0, -1 }, new[] { 0, 1, -1, 0 },
    new[] { -1, 0, 0, 1 }, new[] { 1, 0, 0, -1 }, new[] { 0, 1, 1, 0 }, new[] { 0, -1, -1, 0 } };
Func<List<double[]>, List<double[]>, List<double[]>, List<object>> fitMaps = (anchors, section, face) =>
{
    var maps = new List<object>();
    if (anchors.Count == 0 || section.Count == 0 || face.Count == 0) return maps;
    double fx0 = face.Min(p => p[0]), fy0 = face.Min(p => p[1]), fx1 = face.Max(p => p[0]), fy1 = face.Max(p => p[1]);
    foreach (var o in orientations)
    {
        Func<double[], double[]> apply = p => new[] { o[0] * p[0] + o[1] * p[1], o[2] * p[0] + o[3] * p[1] };
        var mapped = section.Select(apply).ToList();
        double tx = fx0 - mapped.Min(p => p[0]), ty = fy0 - mapped.Min(p => p[1]);
        double residual = Math.Max(Math.Abs(mapped.Max(p => p[0]) + tx - fx1), Math.Abs(mapped.Max(p => p[1]) + ty - fy1));
        foreach (var a in anchors.Select(apply))
        {
            double nearest = Double.MaxValue;
            foreach (var q in face) nearest = Math.Min(nearest, Math.Sqrt((a[0] + tx - q[0]) * (a[0] + tx - q[0]) + (a[1] + ty - q[1]) * (a[1] + ty - q[1])));
            residual = Math.Max(residual, nearest);
        }
        if (residual > MatchTolerance) continue;
        maps.Add(new Dictionary<string, object>
        {
            { "matrix", new[] { new double[] { o[0], o[1] }, new double[] { o[2], o[3] } } },
            { "translation", new[] { clean(tx), clean(ty) } },
            { "max_residual", residual },
        });
    }
    return maps;
};

// The maps above, kept only where they also carry the holes: the same number of holes on both sides, each
// mapped catalogue hole's bounding box equal to a distinct face hole's, and every hole anchor on a face hole
// point, all within MatchTolerance. Without this a hollow section whose outline is symmetric but
// whose holes are not would be reported as matching under maps that put the holes in the wrong place.
Func<List<double[]>, List<double[]>, List<double[]>, List<List<double[]>>, List<double[]>, List<List<double[]>>, List<object>> fitSectionMaps =
    (anchors, section, face, sectionHoles, holeAnchors, faceHoles) =>
{
    var kept = new List<object>();
    if (sectionHoles.Count != faceHoles.Count) return kept;
    var facePoints = faceHoles.SelectMany(h => h).ToList();
    foreach (Dictionary<string, object> map in fitMaps(anchors, section, face))
    {
        var mtx = (double[][])map["matrix"];
        var t = (double[])map["translation"];
        Func<double[], double[]> apply = p => new[] { mtx[0][0] * p[0] + mtx[0][1] * p[1] + t[0], mtx[1][0] * p[0] + mtx[1][1] * p[1] + t[1] };
        double residual = (double)map["max_residual"];
        var used = new bool[faceHoles.Count];
        bool ok = true;
        foreach (var hole in sectionHoles)
        {
            var mapped = hole.Select(apply).ToList();
            double x0 = mapped.Min(p => p[0]), y0 = mapped.Min(p => p[1]), x1 = mapped.Max(p => p[0]), y1 = mapped.Max(p => p[1]);
            int best = -1; double bestError = Double.MaxValue;
            for (int k = 0; k < faceHoles.Count; k++)
            {
                if (used[k]) continue;
                var f = faceHoles[k];
                double error = new[] { Math.Abs(f.Min(p => p[0]) - x0), Math.Abs(f.Min(p => p[1]) - y0), Math.Abs(f.Max(p => p[0]) - x1), Math.Abs(f.Max(p => p[1]) - y1) }.Max();
                if (error < bestError) { bestError = error; best = k; }
            }
            if (best < 0 || bestError > MatchTolerance) { ok = false; break; }
            used[best] = true;
            residual = Math.Max(residual, bestError);
        }
        if (!ok) continue;
        foreach (var a in holeAnchors.Select(apply))
        {
            double nearest = Double.MaxValue;
            foreach (var q in facePoints) nearest = Math.Min(nearest, Math.Sqrt((a[0] - q[0]) * (a[0] - q[0]) + (a[1] - q[1]) * (a[1] - q[1])));
            residual = Math.Max(residual, nearest);
        }
        if (residual > MatchTolerance) continue;
        map["max_residual"] = residual;
        kept.Add(map);
    }
    return kept;
};


// The point a member's axis passes through, in the catalogue section's frame. The axis is (0, 0) in the
// face frame; under a map (M, t) that is the section point M^T (0 - t), M being orthogonal. Every matching
// map must give the same point, or no point is stated and the note says why.
Func<List<object>, Tuple<double[], string>> referenceFrom = maps =>
{
    var candidates = new List<double[]>();
    foreach (Dictionary<string, object> map in maps)
    {
        var mtx = (double[][])map["matrix"];
        var t = (double[])map["translation"];
        candidates.Add(new[] { clean(-(mtx[0][0] * t[0] + mtx[1][0] * t[1])), clean(-(mtx[0][1] * t[0] + mtx[1][1] * t[1])) });
    }
    if (candidates.Count == 0)
        return Tuple.Create<double[], string>(null, "The catalogue section could not be matched to the measured member face, so the point its axis passes through is not stated.");
    if (candidates.Any(c => Math.Abs(c[0] - candidates[0][0]) > MatchTolerance || Math.Abs(c[1] - candidates[0][1]) > MatchTolerance))
        return Tuple.Create<double[], string>(null, "The section matches the member face under several orientations that put the axis at different points, so the reference point is not stated.");
    return Tuple.Create<double[], string>(candidates[0], null);
};
""";

    internal const string Body = """
const string TemporaryName = "AWARE_SECTION_MEASURE";
const int MaxInserts = 5000;

object value;
Func<object, double?> num = v =>
{
    if (v == null) return null;
    if (v is double d) return d;
    if (v is float f) return f;
    if (v is int i) return i;
    if (v is long l) return l;
    if (v is decimal m) return (double)m;
    return null;
};

// ── Input ─────────────────────────────────────────────────────────────────────
// A field that is present but of the wrong shape is refused with its name, never silently ignored: a
// dropped "axis" or "placements" would measure something other than what was asked and still say ok.
Func<string, object> field = key =>
{
    object found;
    return args != null && args.TryGetValue(key, out found) ? found : null;
};
var profiles = new List<string>();
var profileField = field("profiles");
if (profileField != null && !(profileField is IList)) throw new Exception("\"profiles\" must be a list of profile names.");
if (profileField is IList profileList)
{
    int index = 0;
    foreach (var item in profileList)
    {
        var name = item as string;
        if (item != null && name == null) throw new Exception("profiles[" + index + "] must be a profile name (text).");
        if (!String.IsNullOrWhiteSpace(name) && !profiles.Contains(name)) profiles.Add(name);
        index++;
    }
}
if (profiles.Count == 0) throw new Exception("section-measure needs at least one profile name in \"profiles\".");
if (field("placements") != null && !(field("placements") is IList)) throw new Exception("\"placements\" must be a list of positions.");
if (field("placement_grid") != null && !(field("placement_grid") is IDictionary<string, object>)) throw new Exception("\"placement_grid\" must be an object of lists.");
if (field("axis") != null && !(field("axis") is IDictionary<string, object>)) throw new Exception("\"axis\" must be an object {\"start\": [x, y, z], \"end\": [x, y, z]}.");
if (field("high_accuracy") != null && !(field("high_accuracy") is bool)) throw new Exception("\"high_accuracy\" must be true or false.");
if (field("material") != null && !(field("material") is string)) throw new Exception("\"material\" must be a material name (text).");

double length = 1000;
if (args != null && args.TryGetValue("length", out value) && value != null)
{
    var parsed = num(value);
    if (parsed == null || !(parsed.Value > 0) || Double.IsInfinity(parsed.Value)) throw new Exception("\"length\" must be a positive number of millimetres.");
    length = parsed.Value;
}
bool highAccuracy = true;
if (args != null && args.TryGetValue("high_accuracy", out value) && value is bool accuracyFlag) highAccuracy = accuracyFlag;
string material = "STEEL_UNDEFINED";
if (args != null && args.TryGetValue("material", out value) && value is string materialText && !String.IsNullOrWhiteSpace(materialText)) material = materialText;

Func<object, string, double[]> readPoint = (raw, where) =>
{
    var list = raw as IList;
    if (list == null || list.Count != 3) throw new Exception(where + " must be [x, y, z] in millimetres.");
    var p = new double[3];
    for (int k = 0; k < 3; k++)
    {
        var c = num(list[k]);
        if (c == null || Double.IsNaN(c.Value) || Double.IsInfinity(c.Value)) throw new Exception(where + " must be [x, y, z] in millimetres.");
        p[k] = c.Value;
    }
    return p;
};

double[] defaultStart = new double[] { 0, 0, 0 };
double[] defaultEnd = new double[] { length, 0, 0 };
if (args != null && args.TryGetValue("axis", out value) && value is IDictionary<string, object> axisSpec)
{
    object s, e;
    if (!axisSpec.TryGetValue("start", out s) || !axisSpec.TryGetValue("end", out e)) throw new Exception("\"axis\" needs both \"start\" and \"end\".");
    defaultStart = readPoint(s, "axis.start");
    defaultEnd = readPoint(e, "axis.end");
}

Func<string, string, string[], string> enumText = (raw, field, allowed) =>
{
    var text = (raw ?? "").Trim().ToUpperInvariant();
    if (Array.IndexOf(allowed, text) < 0) throw new Exception(field + " \"" + raw + "\" is not one of " + String.Join(", ", allowed) + ".");
    return text;
};
var rotations = new[] { "FRONT", "TOP", "BACK", "BELOW" };
var planes = new[] { "LEFT", "MIDDLE", "RIGHT" };
var depths = new[] { "FRONT", "MIDDLE", "BEHIND" };

var placements = new List<Dictionary<string, object>>();
Func<IDictionary<string, object>, string, Dictionary<string, object>> readPlacement = (spec, where) =>
{
    object v;
    Func<string, double> offset = field =>
    {
        object o;
        if (!spec.TryGetValue(field, out o) || o == null) return 0;
        var d = num(o);
        if (d == null || Double.IsNaN(d.Value) || Double.IsInfinity(d.Value)) throw new Exception(where + "." + field + " must be a number.");
        return d.Value;
    };
    var placement = new Dictionary<string, object>
    {
        { "rotation", enumText(spec.TryGetValue("rotation", out v) ? v as string : "FRONT", where + ".rotation", rotations) },
        { "rotation_offset", offset("rotation_offset") },
        { "plane", enumText(spec.TryGetValue("plane", out v) ? v as string : "MIDDLE", where + ".plane", planes) },
        { "plane_offset", offset("plane_offset") },
        { "depth", enumText(spec.TryGetValue("depth", out v) ? v as string : "MIDDLE", where + ".depth", depths) },
        { "depth_offset", offset("depth_offset") },
    };
    double[] start = defaultStart, end = defaultEnd;
    if (spec.TryGetValue("axis", out v) && v != null)
    {
        var axis = v as IDictionary<string, object>;
        object s, e;
        if (axis == null || !axis.TryGetValue("start", out s) || !axis.TryGetValue("end", out e)) throw new Exception(where + ".axis needs both \"start\" and \"end\".");
        start = readPoint(s, where + ".axis.start");
        end = readPoint(e, where + ".axis.end");
    }
    double dx = end[0] - start[0], dy = end[1] - start[1], dz = end[2] - start[2];
    if (Math.Sqrt(dx * dx + dy * dy + dz * dz) < 1) throw new Exception(where + ".axis is shorter than 1 mm.");
    placement["axis"] = new Dictionary<string, object> { { "start", start }, { "end", end } };
    return placement;
};

if (args != null && args.TryGetValue("placements", out value) && value is IList placementList)
{
    int index = 0;
    foreach (var raw in placementList)
    {
        var spec = raw as IDictionary<string, object>;
        if (spec == null) throw new Exception("placements[" + index + "] must be an object.");
        placements.Add(readPlacement(spec, "placements[" + index + "]"));
        index++;
    }
}
if (args != null && args.TryGetValue("placement_grid", out value) && value is IDictionary<string, object> grid)
{
    Func<string, string, List<object>> axisList = (field, fallback) =>
    {
        object o;
        if (!grid.TryGetValue(field, out o) || o == null) return new List<object> { fallback == null ? (object)0.0 : fallback };
        var list = o as IList;
        if (list == null || list.Count == 0) throw new Exception("placement_grid." + field + " must be a non-empty list.");
        return list.Cast<object>().ToList();
    };
    var gr = axisList("rotations", "FRONT");
    var gro = axisList("rotation_offsets", null);
    var gp = axisList("planes", "MIDDLE");
    var gpo = axisList("plane_offsets", null);
    var gd = axisList("depths", "MIDDLE");
    var gdo = axisList("depth_offsets", null);
    // The budget is checked on the grid's size before it is expanded, so an oversized grid is refused
    // at once instead of being built in memory first. Doubles cannot overflow here; longs could.
    double gridSize = (double)gr.Count * gro.Count * gp.Count * gpo.Count * gd.Count * gdo.Count;
    if ((double)profiles.Count * (placements.Count + gridSize + 1) > MaxInserts)
        throw new Exception("This request would insert " + ((double)profiles.Count * (placements.Count + gridSize + 1)).ToString("0", CultureInfo.InvariantCulture) + " temporary members; the limit is " + MaxInserts + ". Split it into smaller batches.");
    int index = 0;
    foreach (var r in gr) foreach (var ro in gro) foreach (var p in gp) foreach (var po in gpo) foreach (var d in gd) foreach (var dof in gdo)
    {
        var spec = new Dictionary<string, object>
        {
            { "rotation", r as string }, { "rotation_offset", ro },
            { "plane", p as string }, { "plane_offset", po },
            { "depth", d as string }, { "depth_offset", dof },
        };
        placements.Add(readPlacement(spec, "placement_grid[" + index + "]"));
        index++;
    }
}
long insertBudget = (long)profiles.Count * (placements.Count + 1);
if (insertBudget > MaxInserts) throw new Exception("This request would insert " + insertBudget + " temporary members; the limit is " + MaxInserts + ". Split it into smaller batches.");

// ── Model ─────────────────────────────────────────────────────────────────────
var m = model as Model;
if (m == null || !m.GetConnectionStatus()) throw new Exception("No Tekla model is open, so there is nothing to measure in.");
string modelPath = "";
try { modelPath = m.GetInfo().ModelPath; } catch (Exception ex) { throw new Exception("Tekla did not say which model is open: " + ex.Message); }
if (String.IsNullOrWhiteSpace(modelPath)) throw new Exception("Tekla did not say which model is open.");
string profileDatabase = "";
try { TeklaStructuresSettings.GetAdvancedOption("XS_PROFDB", ref profileDatabase); } catch { profileDatabase = ""; }
profileDatabase = profileDatabase ?? "";
string environment = "";
{
    var marker = "\\environments\\";
    var at = profileDatabase.IndexOf(marker, StringComparison.OrdinalIgnoreCase);
    if (at >= 0)
    {
        var rest = profileDatabase.Substring(at + marker.Length).TrimStart('\\');
        var cut = rest.IndexOf('\\');
        environment = cut >= 0 ? rest.Substring(0, cut) : rest;
    }
}

Func<IEnumerable<CrossSectionPoint>, List<object>> vertexList = list =>
{
    var o = new List<object>();
    foreach (var c in list)
    {
        var vertex = new Dictionary<string, object> { { "x", clean(c.X) }, { "y", clean(c.Y) } };
        if (c.Chamfer != null && c.Chamfer.Type != Chamfer.ChamferTypeEnum.CHAMFER_NONE)
            vertex["chamfer"] = new Dictionary<string, object> { { "type", c.Chamfer.Type.ToString() }, { "x", clean(c.Chamfer.X) }, { "y", clean(c.Chamfer.Y) } };
        o.Add(vertex);
    }
    return o;
};


// The 2D frame of a section plane, for the member's direction u: v = Z × u (Y for a member along +X),
// or, when u is vertical, v = u × X; w = u × v. A member along +X therefore reads as (Y, Z).
Func<Vector, Vector[]> planeFrame = dir =>
{
    var u = new Vector(dir); u.Normalize();
    Vector v;
    if (Math.Abs(u.Z) < 0.999999) v = new Vector(0, 0, 1).Cross(u);
    else v = u.Cross(new Vector(1, 0, 0));
    v.Normalize();
    var w = u.Cross(v); w.Normalize();
    return new[] { u, v, w };
};

// ── Measure ───────────────────────────────────────────────────────────────────
int inserted = 0, deleted = 0;
var cleanupFailures = new List<object>();

// A run that was killed, or a Tekla that failed between an insert and its delete, can leave a temporary
// member behind; the finally below cannot run then. So every member this verb inserts is written to a
// ledger the moment Tekla accepts it, and struck off once it is deleted. A run starts by removing what the
// ledger still lists for this model, and only that: a member is removed only if this verb recorded it,
// it still exists, and it still carries the reserved name. A part a person made is never touched, whatever
// its name. The ledger lives under %LOCALAPPDATA%\aware\tekla-section-measure, one file per model path.
Func<string, string> ledgerKey = text =>
{
    uint hash = 2166136261;
    foreach (var ch in text.ToLowerInvariant()) { hash ^= ch; hash *= 16777619; }
    return hash.ToString("x8", CultureInfo.InvariantCulture);
};
string ledgerPath = null;
string ledgerError = null;
var pending = new List<string>();
try
{
    var folder = System.IO.Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "aware", "tekla-section-measure");
    System.IO.Directory.CreateDirectory(folder);
    ledgerPath = System.IO.Path.Combine(folder, ledgerKey(modelPath.TrimEnd('\\', '/')) + ".txt");
}
catch (Exception ex) { ledgerError = "The cleanup ledger could not be opened: " + ex.Message; }
Action writeLedger = () =>
{
    if (ledgerPath == null) return;
    try { System.IO.File.WriteAllLines(ledgerPath, pending); }
    catch (Exception ex) { ledgerError = "The cleanup ledger could not be written: " + ex.Message; }
};

int sweptBefore = 0;
var sweepFailures = new List<object>();
if (ledgerPath != null && System.IO.File.Exists(ledgerPath))
{
    var listed = new List<string>();
    try { listed = System.IO.File.ReadAllLines(ledgerPath).Select(l => l.Trim()).Where(l => l.Length > 0).Distinct().ToList(); }
    catch (Exception ex) { ledgerError = "The cleanup ledger could not be read: " + ex.Message; }
    foreach (var text in listed)
    {
        Guid guid;
        if (!Guid.TryParse(text, out guid)) continue;
        ModelObject found = null;
        try { found = m.SelectModelObject(new Identifier(guid)); } catch { found = null; }
        var leftover = found as Beam;
        // Gone already (the model was closed without saving, or a person deleted it): nothing to do.
        if (leftover == null || !String.Equals(leftover.Name, TemporaryName, StringComparison.Ordinal)) continue;
        bool gone = false;
        try { gone = leftover.Delete(); } catch { gone = false; }
        if (gone) sweptBefore++;
        else
        {
            pending.Add(text);
            sweepFailures.Add(new Dictionary<string, object> { { "id", leftover.Identifier.ID }, { "guid", text } });
        }
    }
    writeLedger();
}

// One temporary member: insert, read its start face in the plane frame, delete in finally.
Func<string, Dictionary<string, object>, List<double[]>, List<double[]>, List<List<double[]>>, List<double[]>, Dictionary<string, object>> measurePlacement = (profile, placement, sectionPoints, sectionAnchors, sectionHoles, holeAnchors) =>
{
    var axis = (Dictionary<string, object>)placement["axis"];
    var start = (double[])axis["start"];
    var end = (double[])axis["end"];
    var row = new Dictionary<string, object>
    {
        { "rotation", placement["rotation"] }, { "rotation_offset", placement["rotation_offset"] },
        { "plane", placement["plane"] }, { "plane_offset", placement["plane_offset"] },
        { "depth", placement["depth"] }, { "depth_offset", placement["depth_offset"] },
        { "axis", axis },
    };
    Beam beam = null;
    string insertedGuid = null;
    try
    {
        beam = new Beam(new Point(start[0], start[1], start[2]), new Point(end[0], end[1], end[2]));
        beam.Name = TemporaryName;
        beam.Profile.ProfileString = profile;
        beam.Material.MaterialString = material;
        beam.Position.Rotation = (Position.RotationEnum)Enum.Parse(typeof(Position.RotationEnum), (string)placement["rotation"]);
        beam.Position.RotationOffset = (double)placement["rotation_offset"];
        beam.Position.Plane = (Position.PlaneEnum)Enum.Parse(typeof(Position.PlaneEnum), (string)placement["plane"]);
        beam.Position.PlaneOffset = (double)placement["plane_offset"];
        beam.Position.Depth = (Position.DepthEnum)Enum.Parse(typeof(Position.DepthEnum), (string)placement["depth"]);
        beam.Position.DepthOffset = (double)placement["depth_offset"];
        if (!beam.Insert()) { row["status"] = "insert-refused"; row["message"] = "Tekla refused to create a member with this profile and position."; return row; }
        inserted++;
        // Kept as text now: once the member is deleted Tekla no longer reports its GUID.
        insertedGuid = beam.Identifier.GUID.ToString();
        pending.Add(insertedGuid);
        writeLedger();

        var solid = beam.GetSolid(highAccuracy ? Solid.SolidCreationTypeEnum.HIGH_ACCURACY : Solid.SolidCreationTypeEnum.NORMAL);
        var dir = new Vector(end[0] - start[0], end[1] - start[1], end[2] - start[2]);
        var frame = planeFrame(dir);
        var origin = new Point(start[0], start[1], start[2]);
        Func<Point, double[]> to2 = p => { var d = new Vector(p.X - origin.X, p.Y - origin.Y, p.Z - origin.Z); return new[] { d.Dot(frame[1]), d.Dot(frame[2]) }; };

        // The start face: the planar face whose outward normal points back along the axis and that lies nearest the start.
        Face startFace = null; double startDistance = Double.MaxValue;
        var faces = solid.GetFaceEnumerator();
        while (faces.MoveNext())
        {
            var face = faces.Current as Face;
            if (face == null) continue;
            var n = new Vector(face.Normal); n.Normalize();
            if (n.Dot(frame[0]) > -0.999999) continue;
            var loops0 = face.GetLoopEnumerator();
            double distance = Double.MaxValue;
            if (loops0.MoveNext())
            {
                var vs = (loops0.Current as Loop).GetVertexEnumerator();
                if (vs.MoveNext()) { var p = vs.Current as Point; distance = Math.Abs(new Vector(p.X - origin.X, p.Y - origin.Y, p.Z - origin.Z).Dot(frame[0])); }
            }
            if (distance < startDistance) { startDistance = distance; startFace = face; }
        }
        if (startFace == null) { row["status"] = "no-start-face"; row["message"] = "Tekla's solid for this member has no end face across its axis."; return row; }

        var loops = new List<List<Point>>();
        var loopEnum = startFace.GetLoopEnumerator();
        while (loopEnum.MoveNext())
        {
            var loop = loopEnum.Current as Loop;
            if (loop == null) continue;
            var pts = new List<Point>();
            var vertices = loop.GetVertexEnumerator();
            while (vertices.MoveNext()) { var p = vertices.Current as Point; if (p != null) pts.Add(new Point(p.X, p.Y, p.Z)); }
            if (pts.Count > 0) loops.Add(pts);
        }
        // The outer loop is the one with the largest area; every other loop is a hole.
        int outerIndex = 0; double outerArea = -1;
        for (int k = 0; k < loops.Count; k++)
        {
            var a = Math.Abs(signedArea(loops[k].Select(to2).ToList()));
            if (a > outerArea) { outerArea = a; outerIndex = k; }
        }
        Func<List<Point>, List<double[]>> world = pts => pts.Select(p => new[] { clean(p.X), clean(p.Y), clean(p.Z) }).ToList();
        var outer2 = loops[outerIndex].Select(to2).ToList();
        var holes2 = loops.Where((l, k) => k != outerIndex).Select(l => l.Select(to2).ToList()).ToList();
        double faceArea = Math.Abs(signedArea(outer2)) - holes2.Sum(h => Math.Abs(signedArea(h)));
        row["status"] = "ok";
        row["start_face"] = new Dictionary<string, object>
        {
            { "outer", world(loops[outerIndex]) },
            { "holes", loops.Where((l, k) => k != outerIndex).Select(world).ToList() },
            { "outer_2d", asJson2(outer2) },
            { "holes_2d", holes2.Select(asJson2).ToList() },
            { "bounds_2d", bounds2(outer2) },
            { "area", faceArea },
            { "start_offset", startDistance },
        };
        var min = solid.MinimumPoint; var max = solid.MaximumPoint;
        row["bounds"] = new Dictionary<string, object> { { "min", new[] { clean(min.X), clean(min.Y), clean(min.Z) } }, { "max", new[] { clean(max.X), clean(max.Y), clean(max.Z) } } };
        var cs = beam.GetCoordinateSystem();
        var ax = new Vector(cs.AxisX); ax.Normalize(); var ay = new Vector(cs.AxisY); ay.Normalize();
        row["member_frame"] = new Dictionary<string, object>
        {
            { "origin", new[] { clean(cs.Origin.X), clean(cs.Origin.Y), clean(cs.Origin.Z) } },
            { "axis_x", new[] { clean(ax.X), clean(ax.Y), clean(ax.Z) } },
            { "axis_y", new[] { clean(ay.X), clean(ay.Y), clean(ay.Z) } },
        };
        row["frame_2d"] = new Dictionary<string, object>
        {
            { "v", new[] { clean(frame[1].X), clean(frame[1].Y), clean(frame[1].Z) } },
            { "w", new[] { clean(frame[2].X), clean(frame[2].Y), clean(frame[2].Z) } },
        };
        double teklaArea = 0;
        if (beam.GetReportProperty("PROFILE.CROSS_SECTION_AREA", ref teklaArea)) row["tekla_area"] = teklaArea;
        var maps = fitSectionMaps(sectionAnchors, sectionPoints, outer2, sectionHoles, holeAnchors, holes2);
        row["section_maps"] = maps;
        if (maps.Count == 0) row["section_map_note"] = "No 90-degree rotation or reflection of the catalogue section matches this face within " + MatchTolerance.ToString(CultureInfo.InvariantCulture) + " mm.";
        return row;
    }
    catch (Exception ex)
    {
        row["status"] = "error";
        row["message"] = ex.Message;
        return row;
    }
    finally
    {
        if (beam != null && beam.Identifier != null && beam.Identifier.ID > 0)
        {
            bool gone = false;
            try { gone = beam.Delete(); } catch { gone = false; }
            if (gone) { deleted++; if (insertedGuid != null) { pending.Remove(insertedGuid); writeLedger(); } }
            else cleanupFailures.Add(new Dictionary<string, object> { { "profile", profile }, { "id", beam.Identifier.ID }, { "guid", beam.Identifier.GUID.ToString() } });
        }
    }
};

var neutral = readPlacement(new Dictionary<string, object>(), "reference");
var results = new List<object>();
foreach (var profile in profiles)
{
    var entry = new Dictionary<string, object> { { "profile", profile } };
    string resolution = "missing";
    var parameters = new List<object>();
    try
    {
        var library = new LibraryProfileItem();
        var parametricItem = new ParametricProfileItem();
        ProfileItem resolved = null;
        if (library.Select(profile)) { resolution = "library"; resolved = library; }
        else if (parametricItem.Select(profile)) { resolution = "parametric"; resolved = parametricItem; }
        if (resolved != null)
            foreach (var raw in resolved.aProfileItemParameters)
            {
                var parameter = raw as ProfileItemParameter;
                if (parameter == null || String.IsNullOrEmpty(parameter.Symbol)) continue;
                // The same value selection as profile-catalog: integers and strings live in their own fields.
                var unitType = parameter.ParameterUnitType.ToString();
                object parameterValue;
                if (unitType == "INPUT_INTEGER") parameterValue = parameter.IntegerValue;
                else if (unitType == "INPUT_STRING") parameterValue = parameter.StringValue ?? "";
                else parameterValue = parameter.Value;
                parameters.Add(new Dictionary<string, object> { { "symbol", parameter.Symbol }, { "value", parameterValue }, { "unit_type", unitType } });
            }
        if (resolved is ParametricProfileItem p) { entry["prefix"] = p.ProfilePrefix; entry["parameter_string"] = p.ParameterString; }
    }
    catch { resolution = "error"; }
    entry["resolution"] = resolution;
    entry["parameters"] = parameters;
    if (resolution == "missing")
    {
        entry["status"] = "no-section";
        entry["message"] = "Tekla's catalogue has no profile of this name, neither in the library nor as a parametric profile.";
        results.Add(entry);
        continue;
    }

    List<double[]> sectionPoints = null;
    List<double[]> sectionAnchors = null;
    List<List<double[]>> sectionHoles = null;
    List<double[]> holeAnchors = null;
    try
    {
        var section = new CrossSection(profile);
        if (!section.Select(highAccuracy, 0, length)) throw new Exception("Tekla returned no cross-section for this name.");
        var outerPoints = section.OuterSurfacePoints.Select(p => new[] { p.X, p.Y }).ToList();
        var holePoints = section.InnerSurfacePoints.Select(l => l.Select(p => new[] { p.X, p.Y }).ToList()).ToList();
        var outerVertices = section.OuterSurface.Select(c => new[] { c.X, c.Y }).ToList();
        double area = Math.Abs(signedArea(outerPoints)) - holePoints.Sum(h => Math.Abs(signedArea(h)));
        sectionPoints = outerPoints;
        sectionHoles = holePoints;
        Func<IEnumerable<CrossSectionPoint>, List<double[]>> loopOf = list => list.Select(c =>
        {
            var type = c.Chamfer == null ? Chamfer.ChamferTypeEnum.CHAMFER_NONE : c.Chamfer.Type;
            double kind = type == Chamfer.ChamferTypeEnum.CHAMFER_NONE ? 0 : type == Chamfer.ChamferTypeEnum.CHAMFER_ROUNDING ? 1 : 2;
            return new[] { c.X, c.Y, kind, kind == 1 ? c.Chamfer.X : 0 };
        }).ToList();
        holeAnchors = section.InnerSurfaces.SelectMany(l => anchorsOf(loopOf(l))).ToList();
        sectionAnchors = anchorsOf(loopOf(section.OuterSurface));
        entry["section"] = new Dictionary<string, object>
        {
            { "frame", "tekla-profile" },
            { "high_accuracy", highAccuracy },
            { "outer", new Dictionary<string, object> { { "vertices", vertexList(section.OuterSurface) }, { "points", asJson2(outerPoints) } } },
            { "holes", section.InnerSurfaces.Select((l, k) => (object)new Dictionary<string, object> { { "vertices", vertexList(l) }, { "points", asJson2(holePoints[k]) } }).ToList() },
            { "area", area },
            { "area_vertices", Math.Abs(signedArea(outerVertices)) - section.InnerSurfaces.Sum(l => Math.Abs(signedArea(l.Select(c => new[] { c.X, c.Y }).ToList()))) },
            { "bounds", bounds2(outerPoints) },
        };
    }
    catch (Exception ex)
    {
        entry["status"] = "no-section";
        entry["message"] = "Tekla did not evaluate a cross-section for this name: " + ex.Message;
        results.Add(entry);
        continue;
    }

    // The reference point: where a member positioned MIDDLE / FRONT / MIDDLE with no offsets carries its axis,
    // read back into the catalogue section's frame through the measured map.
    var reference = measurePlacement(profile, neutral, sectionPoints, sectionAnchors, sectionHoles, holeAnchors);
    entry["reference_placement"] = reference;
    var referenceMaps = reference.ContainsKey("section_maps") ? (List<object>)reference["section_maps"] : new List<object>();
    var referenceResult = referenceFrom(referenceMaps);
    entry["reference_point"] = referenceResult.Item1;
    if (referenceResult.Item2 != null) entry["reference_point_note"] = referenceResult.Item2;

    var rows = new List<object>();
    foreach (var placement in placements) rows.Add(measurePlacement(profile, placement, sectionPoints, sectionAnchors, sectionHoles, holeAnchors));
    entry["placements"] = rows;
    bool allOk = (string)reference["status"] == "ok" && rows.All(r => (string)((Dictionary<string, object>)r)["status"] == "ok");
    entry["status"] = allOk ? "ok" : "partial";
    results.Add(entry);
}

return new Dictionary<string, object>
{
    { "model_path", modelPath },
    { "environment", new Dictionary<string, object> { { "name", environment }, { "profile_database", profileDatabase } } },
    { "length", length },
    { "high_accuracy", highAccuracy },
    { "material", material },
    { "placement_count", placements.Count },
    { "profiles", results },
    { "temporary_members", new Dictionary<string, object>
        {
            { "inserted", inserted }, { "deleted", deleted }, { "cleanup_failures", cleanupFailures },
            { "removed_from_earlier_runs", sweptBefore }, { "earlier_run_cleanup_failures", sweepFailures },
            { "ledger_error", ledgerError },
        } },
};
""";

    internal const string Code = Usings + "\n" + Geometry + "\n" + Body;

}
