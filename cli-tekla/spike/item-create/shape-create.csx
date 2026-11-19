// A5 SPIKE, experiments E1-E2 and E5-E8 (aware#859). EXPERIMENTAL, WRITES THE MODEL'S SHAPE CATALOGUE. Run only through
// run-spike.ps1 -Write. One call does one thing (args.action):
//   "insert"                  ShapeItem.Insert()                                   (refuses a duplicate geometry)
//   "insert-allow-duplicates" ShapeItem.InsertUsingNormalsAllowDuplicates()        (keeps a duplicate geometry; may need vertex normals)
//   "insert-or-match"         ShapeItem.InsertOrGetGuidsOfShapesWithMatchingGeometry (reports the shapes that already match)
//   "insert-same-name"        Insert() under a name that already exists, to learn whether Tekla overwrites it silently
//   "set-metadata"            SetMetadata (the AWARE digest and sizes, so a later run can tell what a shape is)
//   "rename" / "delete"       Rename / Delete of a shape this spike made
//   "export-tsc"              ShapeItem.Export to an explicit folder under the scratch root
//   "import-tsc"              CatalogHandler.ImportShapeItems(folder)               (overrides silently, so names are checked first)
//   "import-geometry"         CatalogHandler.ImportShapeItemsFromGeometryFiles(folder)  (.ifc/.step/.dxf...; file name = shape name)
// Every write is refused unless ALL of these hold:
//   1. the open model is a throwaway A5SPIKE* model;
//   2. the model folder (where the shape catalogue lives) and XS_PROFDB are both under args.scratch_root, unless
//      args.shared_catalog_ok is exactly "MASTER-OK:<model folder name>" (the master's written OK);
//   3. args.backup_hashes lists SHA-256 for every file now in the model's Shapes\ and ShapeGeometries\ folders, no file is
//      missing from it, and every one still matches, which proves a backup was taken from exactly the state about to change;
//   4. every name this call creates is absent from the catalogue (expect_absent), and every name it acts on that must
//      exist is present (expect_present). Tekla's Import "overrides the existing shape items without any warnings", so the
//      silent-overwrite check has to happen here;
//   5. every shape name starts with "AWI" and is at most 1023 characters (Tekla's documented maximum) long, so nothing real can be matched.
// The solid is built from args.solid = { kind:"prism", outer:[[y,z]...], holes:[[[y,z]...]...], length } (a section swept
// along X), which is the shape of the production input (a section swept along a member); any other solid needs a new builder.
using System;
using System.Collections;
using System.Collections.Generic;
using System.Globalization;
using System.IO;
using System.Linq;
using System.Security.Cryptography;
using Tekla.Structures.Catalogs;
using Tekla.Structures.Model;

object value;
var m = model as Model;
if (m == null || !m.GetConnectionStatus()) throw new Exception("No Tekla model is open.");
var modelPath = m.GetInfo().ModelPath;
var leaf = Path.GetFileName(modelPath.TrimEnd('\\', '/'));
if (!leaf.StartsWith("A5SPIKE", StringComparison.Ordinal)) throw new Exception("Refusing: '" + leaf + "' is not a throwaway A5SPIKE* model.");

Func<string, string> full = p => Path.GetFullPath(p).TrimEnd('\\') + "\\";
Func<string, string> sha = path =>
{
    using (var s = SHA256.Create()) using (var f = File.OpenRead(path))
        return String.Concat(s.ComputeHash(f).Select(b => b.ToString("x2")));
};
Func<string, string> argText = key => (args != null && args.TryGetValue(key, out value)) ? value as string : null;
Func<string, List<string>> argNames = key =>
{
    var list = new List<string>();
    if (args != null && args.TryGetValue(key, out value) && value is IList raw)
        foreach (var n in raw) { var t = n as string; if (!String.IsNullOrWhiteSpace(t)) list.Add(t); }
    return list;
};

// Tekla documents 1023 as the longest shape name; the constant on ShapeItem is not public, so it is repeated here.
const int MaxShapeNameLength = 1023;
string action = argText("action");
string scratchRoot = argText("scratch_root");
if (String.IsNullOrWhiteSpace(action) || String.IsNullOrWhiteSpace(scratchRoot) || !Path.IsPathRooted(scratchRoot))
    throw new Exception("args needs action and an absolute scratch_root.");

// Guard 2: the shape catalogue is the model's own, so the model folder must be inside the scratch root; the profile
// catalogue (XS_PROFDB) is not written by this script but must also be the scratch copy, so a surprise cannot reach it.
string profDb = "";
try { Tekla.Structures.TeklaStructuresSettings.GetAdvancedOption("XS_PROFDB", ref profDb); } catch { profDb = ""; }
profDb = (profDb ?? "").Trim();
bool modelIsolated = full(modelPath).StartsWith(full(scratchRoot), StringComparison.OrdinalIgnoreCase);
bool profDbIsolated = profDb.Length > 0 && profDb.Split(';').All(part => part.Trim().Length > 0 && full(part.Trim()).StartsWith(full(scratchRoot), StringComparison.OrdinalIgnoreCase));
if ((!modelIsolated || !profDbIsolated) && argText("shared_catalog_ok") != "MASTER-OK:" + leaf)
    throw new Exception("Refusing: the model folder (" + modelPath + ") or XS_PROFDB (" + profDb + ") is not under the scratch root. It needs the master's OK.");

// Guard 3: a backup was taken from exactly this state.
Func<Dictionary<string, string>> catalogueNow = () =>
{
    var now = new Dictionary<string, string>(StringComparer.OrdinalIgnoreCase);
    foreach (var sub in new[] { "Shapes", "ShapeGeometries" })
    {
        var dir = Path.Combine(modelPath, sub);
        if (Directory.Exists(dir)) foreach (var path in Directory.GetFiles(dir, "*", SearchOption.AllDirectories)) now[path] = sha(path);
    }
    return now;
};
var backup = new Dictionary<string, string>(StringComparer.OrdinalIgnoreCase);
if (args != null && args.TryGetValue("backup_hashes", out value) && value is IDictionary<string, object> dict)
    foreach (var pair in dict) backup[pair.Key] = Convert.ToString(pair.Value);
if (String.IsNullOrWhiteSpace(argText("backup_id"))) throw new Exception("Refusing: args.backup_id is empty, so no backup is proven.");
var before = catalogueNow();
foreach (var pair in before)
{
    string recorded;
    if (!backup.TryGetValue(pair.Key, out recorded)) throw new Exception("Refusing: no backup hash recorded for " + pair.Key);
    if (!String.Equals(recorded, pair.Value, StringComparison.OrdinalIgnoreCase)) throw new Exception("Refusing: " + pair.Key + " changed since its backup hash was taken.");
}

// Guards 4 and 5: nothing to overwrite silently, and only spike names.
var absent = argNames("expect_absent");
var present = argNames("expect_present");
foreach (var name in absent.Concat(present))
{
    if (!name.StartsWith("AWI", StringComparison.Ordinal)) throw new Exception("Refusing: '" + name + "' does not start with AWI.");
    if (name.Length > MaxShapeNameLength) throw new Exception("Refusing: '" + name + "' is longer than " + MaxShapeNameLength + " characters.");
}
foreach (var name in absent)
    if (new ShapeItem().Select(name)) throw new Exception("Refusing: a shape named '" + name + "' already exists, and Tekla would overwrite it silently.");
foreach (var name in present)
    if (!new ShapeItem().Select(name)) throw new Exception("Refusing: expected a shape named '" + name + "' to exist.");
if (absent.Count + present.Count == 0) throw new Exception("Refusing: args.expect_absent or args.expect_present must name every shape this call touches.");

// ---- the solid: a section swept along X, built as a FacetedBrep ----
Func<List<double[]>, double> shoelace = ring =>
{
    double a = 0;
    for (int i = 0; i < ring.Count; i++) { var p = ring[i]; var q = ring[(i + 1) % ring.Count]; a += p[0] * q[1] - q[0] * p[1]; }
    return a / 2.0;
};
Func<object, List<double[]>> ringOf = raw =>
{
    var ring = new List<double[]>();
    foreach (var pt in (IEnumerable)raw) { var c = ((IEnumerable)pt).Cast<object>().Select(x => Convert.ToDouble(x, CultureInfo.InvariantCulture)).ToArray(); ring.Add(new[] { c[0], c[1] }); }
    return ring;
};
IDictionary<string, object> spec = null;
object solidArg;
if (args != null && args.TryGetValue("solid", out solidArg)) spec = solidArg as IDictionary<string, object>;
FacetedBrep sentBrep = null;
double expectedVolume = 0, solidLength = 0;
double[] expectedBox = null;
var sentVertices = new List<double[]>();
var polymeshProblems = new List<object>();
bool polymeshValid = false;
bool needsSolid = action.StartsWith("insert", StringComparison.Ordinal);
if (needsSolid)
{
    if (spec == null || !spec.ContainsKey("outer") || !spec.ContainsKey("length")) throw new Exception("args.solid needs outer and length.");
    var outer = ringOf(spec["outer"]);
    var holes = new List<List<double[]>>();
    if (spec.TryGetValue("holes", out value) && value is IEnumerable hs) foreach (var h in hs) holes.Add(ringOf(h));
    solidLength = Convert.ToDouble(spec["length"], CultureInfo.InvariantCulture);
    if (outer.Count < 3 || solidLength <= 0) throw new Exception("args.solid needs an outer ring with 3 or more points and a positive length.");
    if (shoelace(outer) < 0) outer.Reverse();                                    // outer ring counter-clockwise seen from +X
    foreach (var h in holes) if (shoelace(h) > 0) h.Reverse();                   // hole rings clockwise
    string innerWinding = argText("inner_winding") ?? "opposite";                  // E2 tries both: how a cap's inner loops wind
    var rings = new List<List<double[]>> { outer }; rings.AddRange(holes);
    int total = rings.Sum(r => r.Count);
    var vertices = new List<Tekla.Structures.Geometry3d.Vector>();
    for (int level = 0; level < 2; level++)
        foreach (var r in rings) foreach (var p in r) vertices.Add(new Tekla.Structures.Geometry3d.Vector(level == 0 ? 0.0 : solidLength, p[0], p[1]));
    Func<int, int, int> vid = (level, k) => level * total + k;
    var faces = new List<int[]>();
    var inner = new Dictionary<int, int[][]>();
    Func<int, int, int[]> ringIds = (level, ring) => Enumerable.Range(0, rings[ring].Count).Select(i => vid(level, rings.Take(ring).Sum(r => r.Count) + i)).ToArray();
    // Cap at x = 0 faces -X, so its outer loop runs clockwise seen from +X; the cap at x = L faces +X and runs counter-clockwise.
    // Hole rings are stored clockwise (seen from +X). "opposite" gives a cap's inner loops the reverse winding of its outer loop.
    faces.Add(ringIds(0, 0).Reverse().ToArray());
    var holeIds0 = new List<int[]>();
    for (int h = 1; h < rings.Count; h++) holeIds0.Add(innerWinding == "same" ? ringIds(0, h) : ringIds(0, h).Reverse().ToArray());
    if (holeIds0.Count > 0) inner[0] = holeIds0.ToArray();
    int capEnd = faces.Count;
    faces.Add(ringIds(1, 0));
    var holeIds1 = new List<int[]>();
    for (int h = 1; h < rings.Count; h++) holeIds1.Add(innerWinding == "same" ? ringIds(1, h).Reverse().ToArray() : ringIds(1, h));
    if (holeIds1.Count > 0) inner[capEnd] = holeIds1.ToArray();
    // Side faces: one quad per ring edge. For a counter-clockwise outer ring this order faces outwards; a clockwise hole ring
    // makes the same order face into the hole, which is outwards from the material.
    for (int r = 0; r < rings.Count; r++)
    {
        int start = rings.Take(r).Sum(x => x.Count);
        int n = rings[r].Count;
        for (int i = 0; i < n; i++)
            faces.Add(new[] { vid(0, start + i), vid(0, start + (i + 1) % n), vid(1, start + (i + 1) % n), vid(1, start + i) });
    }
    sentBrep = new FacetedBrep(vertices.ToArray(), faces.ToArray(), inner);
    foreach (var v in vertices) sentVertices.Add(new[] { v.X, v.Y, v.Z });
    double area = Math.Abs(shoelace(outer)) - holes.Sum(h => Math.Abs(shoelace(h)));
    expectedVolume = area * solidLength;
    var ys = outer.Select(p => p[0]); var zs = outer.Select(p => p[1]);
    expectedBox = new[] { 0.0, ys.Min(), zs.Min(), solidLength, ys.Max(), zs.Max() };
    // Tekla's own check of the geometry before it is offered to the catalogue.
    var problems = new List<KeyValuePair<int, Polymesh.PolymeshHealthCheckEnum>>();
    polymeshValid = Polymesh.Validate(sentBrep, Polymesh.PolymeshCheckerFlags.All, ref problems);
    if (problems != null) foreach (var pr in problems) polymeshProblems.Add(new Dictionary<string, object> { { "face", pr.Key }, { "problem", pr.Value.ToString() } });
}

// ---- read-back of one shape through the API ----
Func<string, Dictionary<string, object>> describe = name =>
{
    var row = new Dictionary<string, object> { { "name", name } };
    var s = new ShapeItem();
    bool found = false;
    try { found = s.Select(name); } catch (Exception ex) { row["select_error"] = ex.Message; }
    row["found"] = found;
    if (!found) return row;
    row["guid"] = s.Guid; row["geometry_guid"] = s.GeometryGuid; row["fingerprint"] = s.Fingerprint;
    row["is_solid"] = s.IsSolid; row["brep_type"] = s.BrepType.ToString(); row["up_axis"] = s.UpAxis.ToString();
    try { var e = s.Extrema; if (e != null) row["extrema"] = new[] { e.MinPoint.X, e.MinPoint.Y, e.MinPoint.Z, e.MaxPoint.X, e.MaxPoint.Y, e.MaxPoint.Z }; } catch (Exception ex) { row["extrema_error"] = ex.Message; }
    try { row["instance_count"] = s.GetInstanceCount(); } catch (Exception ex) { row["instance_count_error"] = ex.Message; }
    try
    {
        var rb = s.ShapeFacetedBrep;
        var got = new List<double[]>();
        foreach (var v in rb.Vertices) got.Add(new[] { (double)v.X, (double)v.Y, (double)v.Z });
        row["vertex_count"] = got.Count; row["face_count"] = rb.Faces.Count;
        if (sentVertices.Count > 0 && got.Count > 0)
            row["max_vertex_distance_from_sent"] = sentVertices.Max(a => got.Min(b => Math.Sqrt((a[0] - b[0]) * (a[0] - b[0]) + (a[1] - b[1]) * (a[1] - b[1]) + (a[2] - b[2]) * (a[2] - b[2]))));
    }
    catch (Exception ex) { row["brep_error"] = ex.Message; }
    var meta = new Dictionary<string, object>();
    try { foreach (var key in s.GetAllMetadataKeys()) { var t = ShapeMetadataTypeEnum.Text; object v = null; string label = "", desc = ""; if (s.GetMetadata(key, ref t, ref v, ref label, ref desc)) meta[key] = t + ":" + v; } } catch (Exception ex) { meta["error"] = ex.Message; }
    row["metadata"] = meta;
    return row;
};

// ---- the action ----
string name1 = argText("name");
var result = new Dictionary<string, object> { { "experiment", "shape-create:" + action }, { "model_path", modelPath }, { "xs_profdb", profDb } };
result["catalogue_files_before"] = before.Count;
if (needsSolid)
{
    result["polymesh_valid"] = polymeshValid; result["polymesh_problems"] = polymeshProblems;
    result["expected_volume"] = expectedVolume; result["expected_box"] = expectedBox;
}
try
{
    if (action == "insert" || action == "insert-allow-duplicates" || action == "insert-or-match" || action == "insert-same-name")
    {
        if (String.IsNullOrWhiteSpace(name1)) throw new Exception("args.name is required.");
        ShapeUpAxis up = (ShapeUpAxis)Enum.Parse(typeof(ShapeUpAxis), argText("up_axis") ?? "Y_Axis");
        var shape = new ShapeItem { Name = name1, ShapeFacetedBrep = sentBrep, UpAxis = up };
        if (action == "insert") result["returned"] = shape.Insert();
        else if (action == "insert-same-name") result["returned"] = shape.Insert();
        else if (action == "insert-allow-duplicates") result["returned"] = shape.InsertUsingNormalsAllowDuplicates();
        else
        {
            List<string> existing;
            result["returned"] = shape.InsertOrGetGuidsOfShapesWithMatchingGeometry(out existing);
            result["matching_guids"] = existing;
        }
        string digest = argText("digest");
        if (!String.IsNullOrWhiteSpace(digest) && (bool)result["returned"] && action != "insert-same-name")
        {
            var tagged = new ShapeItem();
            if (tagged.Select(name1)) result["set_metadata_returned"] = tagged.SetMetadata("aware.digest", ShapeMetadataTypeEnum.Text, digest, "AWARE digest", "sha256 of the neutral solid this shape was made from");
        }
        result["after"] = describe(name1);
    }
    else if (action == "set-metadata")
    {
        var s = new ShapeItem();
        if (!s.Select(name1)) throw new Exception("No shape named '" + name1 + "'.");
        result["returned"] = s.SetMetadata(argText("key") ?? "aware.digest", ShapeMetadataTypeEnum.Text, argText("text") ?? "", argText("label") ?? "", "");
        result["after"] = describe(name1);
    }
    else if (action == "rename")
    {
        var s = new ShapeItem();
        string newName = argText("new_name");
        if (String.IsNullOrWhiteSpace(newName) || !newName.StartsWith("AWI", StringComparison.Ordinal)) throw new Exception("Refusing: new_name must start with AWI.");
        if (!s.Select(name1)) throw new Exception("No shape named '" + name1 + "'.");
        result["returned"] = s.Rename(newName);
        result["after_old"] = describe(name1); result["after_new"] = describe(newName);
    }
    else if (action == "delete")
    {
        var s = new ShapeItem();
        if (!s.Select(name1)) throw new Exception("No shape named '" + name1 + "'.");
        result["instances_before_delete"] = s.GetInstanceCount();
        result["returned"] = s.Delete();
        result["after"] = describe(name1);
    }
    else if (action == "export-tsc")
    {
        string outDir = argText("out_dir");
        if (String.IsNullOrWhiteSpace(outDir) || !Path.IsPathRooted(outDir) || !full(outDir).StartsWith(full(scratchRoot), StringComparison.OrdinalIgnoreCase))
            throw new Exception("out_dir is outside scratch_root.");
        if (full(outDir).StartsWith(full(modelPath), StringComparison.OrdinalIgnoreCase)) throw new Exception("out_dir must not be inside the model folder.");
        Directory.CreateDirectory(outDir);
        var s = new ShapeItem();
        if (!s.Select(name1)) throw new Exception("No shape named '" + name1 + "'.");
        var chosen = Path.Combine(outDir, name1);                                   // an explicit path, so Tekla does not default to the model folder
        result["returned"] = s.Export(ref chosen);
        result["path_after_call"] = chosen;
        var written = new List<object>();
        foreach (var path in Directory.GetFiles(outDir, "*", SearchOption.AllDirectories))
            written.Add(new Dictionary<string, object> { { "file", path }, { "bytes", new FileInfo(path).Length }, { "sha256", sha(path) } });
        result["files_written"] = written;
    }
    else if (action == "import-tsc" || action == "import-geometry")
    {
        string importDir = argText("import_dir");
        if (String.IsNullOrWhiteSpace(importDir) || !Path.IsPathRooted(importDir) || !full(importDir).StartsWith(full(scratchRoot), StringComparison.OrdinalIgnoreCase))
            throw new Exception("import_dir is outside scratch_root.");
        if (full(importDir).StartsWith(full(modelPath), StringComparison.OrdinalIgnoreCase)) throw new Exception("import_dir must not be inside the model folder.");
        var handler = new CatalogHandler();
        result["returned"] = action == "import-tsc" ? handler.ImportShapeItems(importDir) : handler.ImportShapeItemsFromGeometryFiles(importDir);
        result["after"] = absent.Concat(present).Distinct().Select(describe).ToList();
    }
    else throw new Exception("Unknown action '" + action + "'.");
}
catch (Exception ex)
{
    result["exception"] = ex.GetType().Name + ": " + ex.Message;
}

// What changed on disk, so the snapshot script's verdict can be matched against what Tekla did.
var after = catalogueNow();
result["files_added"] = after.Keys.Where(k => !before.ContainsKey(k)).ToList();
result["files_removed"] = before.Keys.Where(k => !after.ContainsKey(k)).ToList();
result["files_changed"] = after.Keys.Where(k => before.ContainsKey(k) && before[k] != after[k]).ToList();
result["catalogue_files_after"] = after.Count;
result["shape_count_after"] = Enumerable.Range(0, 1).Select(_ => { int c = 0; var e = new CatalogHandler().GetShapeItems(); while (e.MoveNext()) c++; return c; }).First();
return result;
