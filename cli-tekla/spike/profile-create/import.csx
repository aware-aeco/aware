// SUPERSEDED (owner ruling 2026-11-19, aware#858): the clb/uel import route is dropped; the shipped command is `aware-tekla profile-create`, alias route only. Kept as spike history, not shipped; it imports a whole folder, so run it only against a scratch environment.
// A2 SPIKE, experiments E2-E4 (aware#858). EXPERIMENTAL, WRITES THE PROFILE CATALOGUE. Run only through run-spike.ps1 -Write.
// One call does one thing (args.action): "import-uel" (CatalogHandler.ImportParametricProfileItems), "import-lis"
// (ImportLibraryProfileItems), "save" (CatalogHandler.SaveProfileDatabase), "library-copy" (LibraryProfileItem.Copy) or
// "library-delete" (LibraryProfileItem.Delete, only for a name this spike made). Every write is refused unless ALL of these hold:
//   1. the open model is a throwaway A2SPIKE* model;
//   2. the folder Tekla reads the catalogue from (XS_PROFDB) is under args.scratch_root, i.e. a scratch copy of the
//      environment, unless args.shared_catalog_ok is exactly "MASTER-OK:<model folder name>" (the master's written OK);
//   3. args.backup_hashes lists SHA-256 for profdb.bin and profitab.inp in XS_PROFDB (and any model-folder copies) and every
//      one still matches the file on disk, which proves a backup was taken from exactly the state about to be written;
//   4. every name in args.expect_absent is absent from the catalogue (Import "overrides ... without any warnings",
//      so the silent-overwrite check has to happen here);
//   5. every created or imported name starts with "AWZ", so nothing real can be matched by accident.
using System;
using System.Collections;
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
if (!leaf.StartsWith("A2SPIKE", StringComparison.Ordinal)) throw new Exception("Refusing: '" + leaf + "' is not a throwaway A2SPIKE* model.");

Func<string, string> full = p => Path.GetFullPath(p).TrimEnd('\\') + "\\";
Func<string, string> sha = path =>
{
    using (var s = SHA256.Create()) using (var f = File.OpenRead(path))
        return String.Concat(s.ComputeHash(f).Select(b => b.ToString("x2")));
};
Func<string, string> argText = key => (args != null && args.TryGetValue(key, out value)) ? value as string : null;

string action = argText("action");
string scratchRoot = argText("scratch_root");
string importDir = argText("import_dir");
if (String.IsNullOrWhiteSpace(action) || String.IsNullOrWhiteSpace(scratchRoot) || !Path.IsPathRooted(scratchRoot))
    throw new Exception("args needs action and an absolute scratch_root.");

// Guard 2: which catalogue folder is in play.
string profDb = "";
try { Tekla.Structures.TeklaStructuresSettings.GetAdvancedOption("XS_PROFDB", ref profDb); } catch { profDb = ""; }
profDb = (profDb ?? "").Trim();
if (profDb.Length == 0) throw new Exception("Tekla did not say where XS_PROFDB is, so the write target is unknown.");
bool isolated = profDb.Split(';').All(part => part.Trim().Length > 0 && full(part.Trim()).StartsWith(full(scratchRoot), StringComparison.OrdinalIgnoreCase));
if (!isolated && argText("shared_catalog_ok") != "MASTER-OK:" + leaf)
    throw new Exception("Refusing: XS_PROFDB (" + profDb + ") is not under the scratch root, so this write would land in a shared catalogue. It needs the master's OK.");

// Guard 3: a backup was taken from exactly this state.
var hashes = new Dictionary<string, string>(StringComparer.OrdinalIgnoreCase);
if (args != null && args.TryGetValue("backup_hashes", out value) && value is IDictionary<string, object> dict)
    foreach (var pair in dict) hashes[pair.Key] = Convert.ToString(pair.Value);
if (hashes.Count == 0) throw new Exception("Refusing: args.backup_hashes is empty, so no backup is proven.");
var mustCover = new List<string>();
foreach (var part in profDb.Split(';'))
    foreach (var name in new[] { "profdb.bin", "profitab.inp" })
    {
        var path = Path.Combine(part.Trim(), name);
        if (File.Exists(path)) mustCover.Add(path);
    }
foreach (var name in new[] { "profdb.bin", "profitab.inp" })
{
    var path = Path.Combine(modelPath, name);
    if (File.Exists(path)) mustCover.Add(path);
}
foreach (var path in mustCover)
{
    string recorded;
    if (!hashes.TryGetValue(path, out recorded)) throw new Exception("Refusing: no backup hash recorded for " + path);
    if (!String.Equals(recorded, sha(path), StringComparison.OrdinalIgnoreCase)) throw new Exception("Refusing: " + path + " changed since its backup hash was taken.");
}

// Guards 4 and 5: nothing to overwrite, and only spike names.
var names = new List<string>();
if (args != null && args.TryGetValue("expect_absent", out value) && value is IList wanted)
    foreach (var raw in wanted) { var n = raw as string; if (!String.IsNullOrWhiteSpace(n)) names.Add(n); }
if (names.Count == 0) throw new Exception("Refusing: args.expect_absent must name every profile this call creates.");
foreach (var n in names)
{
    if (!n.StartsWith("AWZ", StringComparison.Ordinal)) throw new Exception("Refusing: '" + n + "' does not start with AWZ.");
    bool taken = false;
    try { taken = new ParametricProfileItem().SelectByPrefix(n) || new ParametricProfileItem().Select(n) || new LibraryProfileItem().Select(n); } catch { }
    if (taken && action != "library-delete" && action != "save") throw new Exception("Refusing: '" + n + "' already exists in the catalogue; Tekla would overwrite it silently.");
}

var catalog = new CatalogHandler();
bool returned = false;
string note = "";
switch (action)
{
    case "import-uel":
    case "import-lis":
        if (String.IsNullOrWhiteSpace(importDir) || !full(importDir).StartsWith(full(scratchRoot), StringComparison.OrdinalIgnoreCase) || !Directory.Exists(importDir))
            throw new Exception("args.import_dir must be an existing folder under scratch_root.");
        note = "import folder holds: " + String.Join(", ", Directory.GetFiles(importDir).Select(Path.GetFileName));
        returned = action == "import-uel" ? catalog.ImportParametricProfileItems(importDir) : catalog.ImportLibraryProfileItems(importDir);
        break;
    case "save":
        returned = catalog.SaveProfileDatabase();
        break;
    case "library-copy":
    {
        var from = argText("source_name");
        if (String.IsNullOrWhiteSpace(from)) throw new Exception("args.source_name is required.");
        var source = new LibraryProfileItem();
        if (!source.Select(from)) throw new Exception("No library profile named '" + from + "'.");
        returned = source.Copy(names[0]);
        break;
    }
    case "library-delete":
    {
        var doomed = new LibraryProfileItem();
        if (!doomed.Select(names[0])) throw new Exception("No library profile named '" + names[0] + "'.");
        returned = doomed.Delete();
        break;
    }
    default:
        throw new Exception("Unknown action '" + action + "'.");
}

// Read back what Tekla now says about each name. Nothing here proves the geometry: that is section-measure's job.
var after = new List<object>();
foreach (var n in names)
{
    bool prefixExists = false, libraryExists = false, parametricSelects = false;
    string parameterString = "", profileString = "";
    try
    {
        var p = new ParametricProfileItem();
        prefixExists = p.SelectByPrefix(n);
        if (prefixExists) { parameterString = p.ParameterString ?? ""; try { profileString = p.CreateProfileString() ?? ""; } catch { } }
    }
    catch { }
    try { libraryExists = new LibraryProfileItem().Select(n); } catch { }
    try { parametricSelects = new ParametricProfileItem().Select(n); } catch { }
    after.Add(new Dictionary<string, object>
    {
        { "name", n }, { "prefix_exists", prefixExists }, { "library_exists", libraryExists }, { "parametric_string_selects", parametricSelects },
        { "parameter_string", parameterString }, { "profile_string_with_defaults", profileString },
    });
}
// Which catalogue files changed on disk because of this call (so the report says where the write landed).
var changed = new List<object>();
foreach (var path in mustCover)
    changed.Add(new Dictionary<string, object> { { "file", path }, { "sha256_before", hashes[path] }, { "sha256_after", File.Exists(path) ? sha(path) : "" } });

return new Dictionary<string, object>
{
    { "experiment", "E2-E4-" + action }, { "model_path", modelPath }, { "xs_profdb", profDb }, { "isolated_under_scratch_root", isolated },
    { "api_returned", returned }, { "note", note }, { "after", after }, { "catalogue_files", changed },
};
