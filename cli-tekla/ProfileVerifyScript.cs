namespace AwareTekla;

/// <summary>
/// `profile-verify` (aware#858): the read-back half of `profile-create`. Tekla reads `profitab.inp` only at start, so
/// this runs after Tekla has been restarted and proves, from Tekla itself, that the alias registered and measures like
/// the shape it aliases. READ-ONLY: it enumerates the parametric catalogue and asks `CrossSection.Select` for
/// each sample name; it inserts nothing and commits nothing.
///
/// A prefix that is in the file but not in Tekla (the file was written and Tekla not restarted, or the prefix was too
/// long and registered broken) is reported as `not-registered`, with any near-miss entry the enumerator shows, because
/// `SelectByPrefix` cannot see a broken entry.
/// </summary>
internal static class ProfileVerifyScript
{
    internal const int MaxSamples = 8;

    internal const string Usings = """
using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using Tekla.Structures.Catalogs;
using Tekla.Structures.Model;
""";

    internal const string Body = """

object value;
string prefix = null, reference = null;
if (args != null && args.TryGetValue("prefix", out value)) prefix = value as string;
if (args != null && args.TryGetValue("reference", out value)) reference = value as string;
if (String.IsNullOrEmpty(prefix)) throw new Exception("prefix is required");
var samples = new List<string>();
if (args != null && args.TryGetValue("samples", out value) && value is IList list)
    foreach (var s in list) { var t = s as string; if (!String.IsNullOrWhiteSpace(t)) samples.Add(t.Trim()); }
if (samples.Count == 0) throw new Exception("samples is required: the part of a name after the prefix, e.g. \"400*200\"");

string profileDatabase = "";
try { Tekla.Structures.TeklaStructuresSettings.GetAdvancedOption("XS_PROFDB", ref profileDatabase); } catch { profileDatabase = ""; }

// Enumerate: SelectByPrefix cannot see a broken half-registration, the enumerator does.
int total = 0;
var exact = new List<object>();
var nearMiss = new List<object>();
{
    var items = new CatalogHandler().GetParametricProfileItems();
    while (items.MoveNext())
    {
        var item = items.Current as ParametricProfileItem;
        if (item == null || item.ProfilePrefix == null) continue;
        total++;
        var p = item.ProfilePrefix;
        var row = new Dictionary<string, object> { { "prefix", p }, { "type", item.ProfileItemType.ToString() }, { "subtype", item.ProfileItemSubType.ToString() }, { "parameter_string", item.ParameterString ?? "" }, { "clb", item.IsCLBProfile } };
        if (String.Equals(p, prefix, StringComparison.Ordinal)) exact.Add(row);
        else if (p.StartsWith(prefix, StringComparison.Ordinal) || prefix.StartsWith(p, StringComparison.Ordinal) && p.Length >= 4) nearMiss.Add(row);
    }
}
var selector = new ParametricProfileItem();
bool selected = selector.SelectByPrefix(prefix);
string selectedType = selected ? selector.ProfileItemType.ToString() : null;
string parameterString = selected ? selector.ParameterString : null;

Func<string, Dictionary<string, object>> measure = name =>
{
    var r = new Dictionary<string, object> { { "name", name } };
    try
    {
        var section = new CrossSection(name);
        if (!section.Select(false, 0, 1000.0)) { r["status"] = "no-section"; return r; }
        var pts = section.OuterSurfacePoints.Select(q => new[] { q.X, q.Y }).ToList();
        double area = 0;
        for (int k = 0; k < pts.Count; k++) { var a = pts[k]; var b = pts[(k + 1) % pts.Count]; area += a[0] * b[1] - b[0] * a[1]; }
        r["status"] = "ok";
        r["area"] = Math.Abs(area) / 2;
        r["outer"] = pts;
        r["holes"] = section.InnerSurfacePoints.Count;
    }
    catch (Exception ex) { r["status"] = "error"; r["message"] = ex.Message; }
    return r;
};

var rows = new List<object>();
bool allMatch = selected && exact.Count == 1;
foreach (var s in samples)
{
    var alias = measure(prefix + s);
    Dictionary<string, object> refer = null;
    bool match = false;
    double maxDev = -1;
    if (!String.IsNullOrEmpty(reference))
    {
        refer = measure(reference + s);
        if ((string)alias["status"] == "ok" && (string)refer["status"] == "ok")
        {
            var a = (List<double[]>)alias["outer"]; var b = (List<double[]>)refer["outer"];
            if (a.Count == b.Count)
            {
                maxDev = 0;
                for (int k = 0; k < a.Count; k++) maxDev = Math.Max(maxDev, Math.Max(Math.Abs(a[k][0] - b[k][0]), Math.Abs(a[k][1] - b[k][1])));
                match = maxDev <= 1e-6 && Math.Abs((double)alias["area"] - (double)refer["area"]) <= 1e-6 && (int)alias["holes"] == (int)refer["holes"];
            }
        }
    }
    else match = (string)alias["status"] == "ok";
    if (!match) allMatch = false;
    rows.Add(new Dictionary<string, object> { { "sample", s }, { "alias", alias }, { "reference", refer }, { "matches_reference", match }, { "max_vertex_deviation_mm", maxDev < 0 ? (object)null : maxDev } });
}
string status = exact.Count == 0 ? "not-registered" : (allMatch ? "verified" : "mismatch");
return new Dictionary<string, object>
{
    { "status", status },
    { "prefix", prefix },
    { "reference", reference },
    { "profile_database", profileDatabase },
    { "parametric_total", total },
    { "registered", exact },
    { "near_miss_entries", nearMiss },
    { "select_by_prefix", selected },
    { "type", selectedType },
    { "parameter_string", parameterString },
    { "samples", rows },
    { "note", exact.Count == 0 ? "Tekla does not list this prefix. If profitab.inp was written after Tekla started, restart Tekla (it reads the file only at start); otherwise see near_miss_entries." : "" },
};
""";

    internal const string Code = Usings + "\n" + Body;
}
