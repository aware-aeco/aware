// A2 SPIKE, experiment E1 (aware#858). EXPERIMENTAL. Exports ONE existing user parametric shape to a scratch folder,
// to learn the file Tekla writes (a .uel for a sketch, a .clb for a clb profile). It changes no catalogue: the only
// thing written is the export file, into args.out_dir, which must be under args.scratch_root.
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
if (!leaf.StartsWith("A2SPIKE", StringComparison.Ordinal)) throw new Exception("Refusing: '" + leaf + "' is not a throwaway A2SPIKE* model.");

Func<string, string> full = p => Path.GetFullPath(p).TrimEnd('\\') + "\\";
string prefix = null, outDir = null, scratchRoot = null;
if (args != null && args.TryGetValue("prefix", out value)) prefix = value as string;
if (args != null && args.TryGetValue("out_dir", out value)) outDir = value as string;
if (args != null && args.TryGetValue("scratch_root", out value)) scratchRoot = value as string;
if (String.IsNullOrWhiteSpace(prefix) || String.IsNullOrWhiteSpace(outDir) || String.IsNullOrWhiteSpace(scratchRoot))
    throw new Exception("args needs prefix, out_dir and scratch_root.");
if (!Path.IsPathRooted(outDir) || !Path.IsPathRooted(scratchRoot)) throw new Exception("out_dir and scratch_root must be absolute.");
if (!full(outDir).StartsWith(full(scratchRoot), StringComparison.OrdinalIgnoreCase)) throw new Exception("out_dir is outside scratch_root.");
// Never export into the model folder: it is where Tekla writes when no path is given, and a folder it reads from.
if (full(outDir).StartsWith(full(modelPath), StringComparison.OrdinalIgnoreCase)) throw new Exception("out_dir must not be inside the model folder.");

Directory.CreateDirectory(outDir);
Func<string, Dictionary<string, string>> snapshot = dir =>
    Directory.Exists(dir)
        ? Directory.GetFiles(dir, "*", SearchOption.TopDirectoryOnly).ToDictionary(p => p, p => new FileInfo(p).Length + "|" + new FileInfo(p).LastWriteTimeUtc.Ticks, StringComparer.OrdinalIgnoreCase)
        : new Dictionary<string, string>();
var modelBefore = snapshot(modelPath);

var item = new ParametricProfileItem();
if (!item.SelectByPrefix(prefix)) throw new Exception("No parametric shape has prefix '" + prefix + "'.");
var info = new Dictionary<string, object>
{
    { "prefix", item.ProfilePrefix ?? "" }, { "parameter_string", item.ParameterString ?? "" },
    { "sketched", item.IsSketchedUserParametric }, { "clb", item.IsCLBProfile }, { "hard_coded", item.IsHardcodedProfile },
    { "multi_cross_section", item.IsMultiCrossSectionUserParametric }, { "cross_sections", item.NumberOfCrossSections },
};
var parameters = new List<object>();
foreach (var raw in item.aProfileItemParameters)
{
    var p = raw as ProfileItemParameter;
    if (p != null)
        parameters.Add(new Dictionary<string, object> { { "symbol", p.Symbol }, { "property", p.Property }, { "unit_type", p.ParameterUnitType.ToString() }, { "value", p.Value }, { "cross_section", p.CrossSectionNumber } });
}

var chosen = Path.Combine(outDir, "export-" + prefix);   // an explicit path, so Tekla does not default to the model folder
bool exported = item.Export(ref chosen);

var written = new List<object>();
foreach (var path in Directory.GetFiles(outDir, "*", SearchOption.AllDirectories))
{
    var fi = new FileInfo(path);
    string hash;
    using (var s = SHA256.Create()) using (var f = File.OpenRead(path)) hash = String.Concat(s.ComputeHash(f).Select(b => b.ToString("x2")));
    var head = "";
    try { head = File.ReadAllText(path); if (head.Length > 6000) head = head.Substring(0, 6000); } catch { }
    written.Add(new Dictionary<string, object> { { "file", path }, { "bytes", fi.Length }, { "sha256", hash }, { "text_head", head } });
}
var modelAfter = snapshot(modelPath);
var strayInModelFolder = modelAfter.Where(kv => !modelBefore.ContainsKey(kv.Key) || modelBefore[kv.Key] != kv.Value).Select(kv => kv.Key).ToList();

return new Dictionary<string, object>
{
    { "experiment", "E1-export" }, { "model_path", modelPath }, { "shape", info }, { "parameters", parameters },
    { "export_returned", exported }, { "path_after_call", chosen }, { "files_written", written },
    { "model_folder_files_changed_by_export", strayInModelFolder },
};
