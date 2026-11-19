// A5 SPIKE, experiment E0 (aware#859). READ-ONLY. EXPERIMENTAL: not a shipped verb, run through `exec` by run-spike.ps1.
// What it does: says which model is open, where Tekla reads its profile catalogue from, which files sit in the model's
// own shape catalogue (Shapes\ and ShapeGeometries\, with size and SHA-256), which shapes Tekla lists, and whether the
// planned AWI* names already exist. It writes nothing.
using System;
using System.Collections.Generic;
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
// Identity guard: the spike only ever runs in a throwaway model whose folder name starts with A5SPIKE.
if (!leaf.StartsWith("A5SPIKE", StringComparison.Ordinal))
    throw new Exception("Refusing: the open model is '" + leaf + "', not a throwaway A5SPIKE* model.");

Func<string, string> sha = path =>
{
    using (var s = SHA256.Create()) using (var f = File.OpenRead(path))
        return String.Concat(s.ComputeHash(f).Select(b => b.ToString("x2")));
};

// 1. The advanced options that decide where profile catalogues are read from (the shape catalogue is not one of them).
var options = new Dictionary<string, object>();
foreach (var name in new[] { "XS_PROFDB", "XS_FIRM", "XS_PROJECT", "XS_SYSTEM", "XS_DIR", "XS_RUNPATH", "XS_INP", "XS_USER" })
{
    string text = "";
    bool ok = false;
    try { ok = Tekla.Structures.TeklaStructuresSettings.GetAdvancedOption(name, ref text); } catch { ok = false; }
    options[name] = new Dictionary<string, object> { { "found", ok }, { "value", text ?? "" } };
}

// 2. The model's own shape catalogue on disk, by name, size, time and hash.
var files = new List<object>();
foreach (var sub in new[] { "Shapes", "ShapeGeometries" })
{
    var dir = Path.Combine(modelPath, sub);
    if (!Directory.Exists(dir)) { files.Add(new Dictionary<string, object> { { "folder", dir }, { "exists", false } }); continue; }
    foreach (var path in Directory.GetFiles(dir, "*", SearchOption.AllDirectories).OrderBy(x => x, StringComparer.OrdinalIgnoreCase))
    {
        var info = new FileInfo(path);
        files.Add(new Dictionary<string, object>
        {
            { "folder", dir }, { "file", info.Name }, { "bytes", info.Length },
            { "modified_utc", info.LastWriteTimeUtc.ToString("o") }, { "sha256", sha(path) },
        });
    }
}

// 3. What Tekla itself lists as shapes (read through the API, not the files).
var shapes = new List<object>();
int limit = 60;
if (args != null && args.TryGetValue("limit", out value) && value != null) limit = Convert.ToInt32(value);
int total = 0;
var en = new CatalogHandler().GetShapeItems();
while (en.MoveNext())
{
    var item = en.Current as ShapeItem;
    if (item == null) continue;
    total++;
    if (shapes.Count >= limit) continue;
    var row = new Dictionary<string, object>
    {
        { "name", item.Name }, { "guid", item.Guid }, { "geometry_guid", item.GeometryGuid },
        { "is_solid", item.IsSolid }, { "brep_type", item.BrepType.ToString() }, { "fingerprint", item.Fingerprint },
    };
    try { row["instance_count"] = item.GetInstanceCount(); } catch (Exception ex) { row["instance_count_error"] = ex.Message; }
    try { var e = item.Extrema; if (e != null) row["extrema"] = new[] { e.MinPoint.X, e.MinPoint.Y, e.MinPoint.Z, e.MaxPoint.X, e.MaxPoint.Y, e.MaxPoint.Z }; } catch { }
    shapes.Add(row);
}

// 4. Optional name check: does a shape with each of these names already exist?
var nameChecks = new List<object>();
if (args != null && args.TryGetValue("names", out value) && value is System.Collections.IList wanted)
    foreach (var raw in wanted)
    {
        var n = raw as string;
        if (String.IsNullOrWhiteSpace(n)) continue;
        bool exists = false;
        try { exists = new ShapeItem().Select(n); } catch { }
        nameChecks.Add(new Dictionary<string, object> { { "name", n }, { "length", n.Length }, { "exists", exists } });
    }

// 5. Items already in the model that point at a shape: their profile string is the shape name.
int brepItems = 0;
var moe = m.GetModelObjectSelector().GetAllObjectsWithType(ModelObject.ModelObjectEnum.BREP);
while (moe.MoveNext()) if (moe.Current is Brep) brepItems++;

return new Dictionary<string, object>
{
    { "experiment", "E0-probe" },
    { "model_path", modelPath },
    { "advanced_options", options },
    { "shape_catalogue_files", files },
    { "shape_count", total }, { "shapes", shapes },
    { "name_checks", nameChecks },
    { "brep_items_in_model", brepItems },
};
