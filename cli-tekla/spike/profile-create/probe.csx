// A2 SPIKE, experiment E0 (aware#858). READ-ONLY. EXPERIMENTAL: not a shipped verb, run through `exec` by run-spike.ps1.
// What it does: says which model is open, which folders Tekla reads its profile catalogue from, which catalogue
// files exist there (size, time, SHA-256), and which parametric shapes the open environment already has. It writes nothing.
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
// Identity guard: the spike only ever runs in a throwaway model whose folder name starts with A2SPIKE.
if (!leaf.StartsWith("A2SPIKE", StringComparison.Ordinal))
    throw new Exception("Refusing: the open model is '" + leaf + "', not a throwaway A2SPIKE* model.");

Func<string, string> sha = path =>
{
    using (var s = SHA256.Create()) using (var f = File.OpenRead(path))
        return String.Concat(s.ComputeHash(f).Select(b => b.ToString("x2")));
};

// 1. The advanced options that decide where the catalogue is read from.
var options = new Dictionary<string, object>();
foreach (var name in new[] { "XS_PROFDB", "XS_FIRM", "XS_PROJECT", "XS_SYSTEM", "XS_DIR", "XS_RUNPATH", "XS_INP", "XS_USER" })
{
    string text = "";
    bool ok = false;
    try { ok = Tekla.Structures.TeklaStructuresSettings.GetAdvancedOption(name, ref text); } catch { ok = false; }
    options[name] = new Dictionary<string, object> { { "found", ok }, { "value", text ?? "" } };
}

// 2. Candidate folders: every ';' separated entry of those options, plus the model folder and its profile sub-folders.
var folders = new List<string>();
Action<string> addFolder = p =>
{
    if (String.IsNullOrWhiteSpace(p)) return;
    var entry = p.Trim();
    if (!folders.Contains(entry, StringComparer.OrdinalIgnoreCase)) folders.Add(entry);
};
foreach (var pair in options)
{
    var text = (string)((Dictionary<string, object>)pair.Value)["value"];
    foreach (var part in text.Split(';')) addFolder(part);
}
addFolder(modelPath);
addFolder(Path.Combine(modelPath, "ParametricProfiles"));
addFolder(Path.Combine(modelPath, "ExportedProfiles"));
addFolder(Path.Combine(modelPath, "profil"));

var patterns = new[] { "profdb.bin", "profitab.inp", "profile_properties.ail", "*.clb", "*.uel", "*.lis" };
var files = new List<object>();
foreach (var folder in folders)
{
    if (!Directory.Exists(folder)) { files.Add(new Dictionary<string, object> { { "folder", folder }, { "exists", false } }); continue; }
    foreach (var pattern in patterns)
        foreach (var path in Directory.GetFiles(folder, pattern, SearchOption.TopDirectoryOnly).OrderBy(x => x, StringComparer.OrdinalIgnoreCase))
        {
            var info = new FileInfo(path);
            files.Add(new Dictionary<string, object>
            {
                { "folder", folder }, { "file", info.Name }, { "bytes", info.Length },
                { "modified_utc", info.LastWriteTimeUtc.ToString("o") }, { "sha256", sha(path) },
            });
        }
}

// 3. Parametric shapes the environment already has (user-defined ones in detail), and an optional prefix check.
var sketched = new List<object>();
int total = 0, builtIn = 0, clb = 0, sketchedCount = 0, multi = 0;
int limit = 40;
if (args != null && args.TryGetValue("limit", out value) && value != null) limit = Convert.ToInt32(value);   // JSON integers arrive as int
var en = new CatalogHandler().GetParametricProfileItems();
while (en.MoveNext())
{
    var item = en.Current as ParametricProfileItem;
    if (item == null) continue;
    total++;
    var type = item.ProfileItemType.ToString();
    if (type != "PROFILE_USER_PARAMETRIC") { builtIn++; continue; }
    if (item.IsCLBProfile) clb++;
    if (item.IsSketchedUserParametric) sketchedCount++;
    if (item.IsMultiCrossSectionUserParametric) multi++;
    if (sketched.Count < limit)
        sketched.Add(new Dictionary<string, object>
        {
            { "prefix", item.ProfilePrefix ?? "" }, { "parameter_string", item.ParameterString ?? "" },
            { "sketched", item.IsSketchedUserParametric }, { "clb", item.IsCLBProfile },
            { "hard_coded", item.IsHardcodedProfile }, { "multi_cross_section", item.IsMultiCrossSectionUserParametric },
            { "cross_sections", item.NumberOfCrossSections },
        });
}
var prefixChecks = new List<object>();
if (args != null && args.TryGetValue("prefixes", out value) && value is System.Collections.IList wanted)
    foreach (var raw in wanted)
    {
        var prefix = raw as string;
        if (String.IsNullOrWhiteSpace(prefix)) continue;
        bool asPrefix = false, asLibrary = false, asParametric = false;
        try { asPrefix = new ParametricProfileItem().SelectByPrefix(prefix); } catch { }
        try { asLibrary = new LibraryProfileItem().Select(prefix); } catch { }
        try { asParametric = new ParametricProfileItem().Select(prefix); } catch { }
        prefixChecks.Add(new Dictionary<string, object> { { "name", prefix }, { "prefix_exists", asPrefix }, { "library_exists", asLibrary }, { "parametric_string_selects", asParametric } });
    }

return new Dictionary<string, object>
{
    { "experiment", "E0-probe" },
    { "model_path", modelPath },
    { "advanced_options", options },
    { "catalogue_files", files },
    { "parametric_total", total }, { "parametric_built_in", builtIn }, { "parametric_user", total - builtIn },
    { "user_sketched", sketchedCount }, { "user_clb", clb }, { "user_multi_cross_section", multi },
    { "user_parametric_sample", sketched },
    { "prefix_checks", prefixChecks },
};
