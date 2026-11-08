namespace AwareTekla;

/// <summary>
/// Read-only profile catalogue query (#3312). It deliberately lives as a Roslyn script, like
/// <see cref="BakeSceneScript"/>, so the sidecar stays version-neutral across Tekla 2025 and 2026.
/// Nothing is inserted, modified or committed: it only asks the catalogue.
/// </summary>
internal static class ProfileCatalogScript
{
    internal const string Code = """
using System;
using System.Collections;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using Tekla.Structures.Catalogs;
using Tekla.Structures.Model;

var inv = CultureInfo.InvariantCulture;
var requested = new List<string>();
object value;
if (args != null && args.TryGetValue("profiles", out value))
{
    var list = value as IList;
    if (list != null)
        foreach (var item in list)
        {
            var name = item as string;
            if (!String.IsNullOrWhiteSpace(name) && !requested.Contains(name)) requested.Add(name);
        }
}
bool wantCatalog = false;
if (args != null && args.TryGetValue("catalog", out value) && value is bool flag) wantCatalog = flag;

// "Insert would accept this name" is exactly: in the library, or a parametric profile string.
var resolved = new List<object>();
foreach (var name in requested)
{
    string status = "missing";
    try { if (new LibraryProfileItem().Select(name)) status = "library"; } catch { }
    if (status == "missing")
    {
        try { if (new ParametricProfileItem().Select(name)) status = "parametric"; } catch { }
    }
    resolved.Add(new Dictionary<string, object> { { "profile", name }, { "status", status } });
}

Func<string, string, string> familyOf = (type, subType) =>
{
    switch (type)
    {
        case "PROFILE_P": return "rectangular-hollow";
        case "PROFILE_PD": return "circular-hollow";
        case "PROFILE_I": return "i-section";
        case "PROFILE_U": return "channel";
        case "PROFILE_T": return "tee";
        case "PROFILE_Z": return "z-section";
        case "PROFILE_FPL": return subType.IndexOf("_L_", StringComparison.Ordinal) >= 0 ? "angle" : "flat";
        default: return "other";
    }
};

var catalog = new List<object>();
if (wantCatalog)
{
    var items = new CatalogHandler().GetLibraryProfileItems();
    while (items.MoveNext())
    {
        var item = items.Current as LibraryProfileItem;
        if (item == null) continue;
        var dims = new Dictionary<string, object>();
        foreach (var raw in item.aProfileItemParameters)
        {
            var parameter = raw as ProfileItemParameter;
            if (parameter == null || String.IsNullOrEmpty(parameter.Symbol)) continue;
            if (parameter.ParameterUnitType.ToString() != "INPUT_DIMENSION_UNIT") continue;
            dims[parameter.Symbol] = Math.Round(parameter.Value, 3);
        }
        catalog.Add(new Dictionary<string, object>
        {
            { "name", item.ProfileName },
            { "family", familyOf(item.ProfileItemType.ToString(), item.ProfileItemSubType.ToString()) },
            { "dims", dims },
        });
    }
}

var m = model as Model;
string modelPath = "";
try { if (m != null) modelPath = m.GetInfo().ModelPath; } catch { }

return new Dictionary<string, object>
{
    { "model_path", modelPath },
    { "resolved", resolved },
    { "catalog_count", catalog.Count },
    { "catalog", catalog },
};
""";
}
