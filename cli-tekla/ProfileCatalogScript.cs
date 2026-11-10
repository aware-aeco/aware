namespace AwareTekla;

/// <summary>
/// Read-only profile catalogue query (#3312). It deliberately lives as a Roslyn script, like
/// <see cref="BakeSceneScript"/>, so the sidecar stays version-neutral across Tekla 2025 and 2026.
/// Nothing is inserted, modified or committed: it only asks the catalogue.
/// </summary>
internal static class ProfileCatalogScript
{
    /// <summary>The script's using directives.</summary>
    internal const string Usings = """
using System;
using System.Collections;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using Tekla.Structures.Catalogs;
using Tekla.Structures.Model;
""";

    /// <summary>Name-order and label helpers (BCL only), separate so tests can execute them.</summary>
    internal const string Labels = """
// Parametric shapes (#799, FloLess #3382 A1): every parametric profile the environment defines, built-in
// and user-defined, with its name order (Tekla's ParameterString) and its parameters. Prefixes that share
// one shape (same type, subtype, order and parameters) are one entry listing all of them, so an alias is
// never mistaken for a different shape. Ordinal sorting throughout: the same environment gives the same bytes.
Func<string, string> unitOf = raw =>
{
    switch (raw)
    {
        case "INPUT_DIMENSION_UNIT":
        case "INPUT_LENGTH_UNIT":
        case "INPUT_DEFORMATION_UNIT": return "mm";
        case "INPUT_ANGLE_UNIT":
        case "INPUT_SECTION_ANGLE_UNIT": return "deg";
        case "INPUT_INTEGER": return "integer";
        case "INPUT_STRING": return "string";
        case "INPUT_BOOLEAN": return "boolean";
        case "INPUT_RATIO_UNIT": return "ratio";
        case "INPUT_FACTOR_UNIT": return "factor";
        default: return "other";
    }
};
// "albl_Flange_thickness" -> "Flange thickness"; a key without the albl_ prefix (a built-in property such as
// "FLANGE_THICKNESS") reads the same way. The raw key is returned beside it, so nothing is lost.
Func<string, string> plainLabel = key =>
{
    if (String.IsNullOrEmpty(key)) return "";
    var text = key.StartsWith("albl_", StringComparison.Ordinal) ? key.Substring(5) : key;
    text = text.Replace('_', ' ').Trim();
    if (text.Length == 0) return "";
    if (text.ToUpperInvariant() == text) text = text.ToLowerInvariant();
    return Char.ToUpperInvariant(text[0]) + text.Substring(1);
};
// The symbols the name order mentions: the identifier runs of the parameter string.
Func<string, HashSet<string>> symbolsIn = order =>
{
    var found = new HashSet<string>(StringComparer.Ordinal);
    var current = new System.Text.StringBuilder();
    foreach (var ch in (order ?? "") + " ")
    {
        if (Char.IsLetterOrDigit(ch) || ch == '_') { current.Append(ch); continue; }
        if (current.Length > 0) { found.Add(current.ToString()); current.Clear(); }
    }
    return found;
};

""";

    internal const string Body = """

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
bool wantParametric = false;
if (args != null && args.TryGetValue("parametric", out value) && value is bool parametricFlag) wantParametric = parametricFlag;

// The destination must be a connected, open model: without it there is no catalogue to ask and no identity to report.
var m = model as Model;
if (m == null || !m.GetConnectionStatus()) throw new Exception("No Tekla model is open, so there is no profile catalogue to ask.");
string modelPath = "";
try { modelPath = m.GetInfo().ModelPath; } catch (Exception ex) { throw new Exception("Tekla did not say which model is open: " + ex.Message); }
if (String.IsNullOrWhiteSpace(modelPath)) throw new Exception("Tekla did not say which model is open.");

// "Insert would accept this name" is exactly: in the library, or a parametric profile string.
// A lookup that throws proves nothing, so it is reported as "error" (never "missing") and the caller does not decide on it.
var resolved = new List<object>();
foreach (var name in requested)
{
    string status = "missing";
    try
    {
        if (new LibraryProfileItem().Select(name)) status = "library";
        else if (new ParametricProfileItem().Select(name)) status = "parametric";
    }
    catch { status = "error"; }
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

var parametric = new List<object>();
int parametricItemCount = 0;
string profileDatabase = "";
string environment = "";
if (wantParametric)
{
    // Which environment answered: the folder after "\environments\" in the profile database path.
    try { Tekla.Structures.TeklaStructuresSettings.GetAdvancedOption("XS_PROFDB", ref profileDatabase); } catch { profileDatabase = ""; }
    profileDatabase = profileDatabase ?? "";
    var marker = "\\environments\\";
    var at = profileDatabase.IndexOf(marker, StringComparison.OrdinalIgnoreCase);
    if (at >= 0)
    {
        var rest = profileDatabase.Substring(at + marker.Length).TrimStart('\\');
        var cut = rest.IndexOf('\\');
        environment = cut >= 0 ? rest.Substring(0, cut) : rest;
    }

    var groups = new Dictionary<string, Dictionary<string, object>>(StringComparer.Ordinal);
    var groupPrefixes = new Dictionary<string, List<string>>(StringComparer.Ordinal);
    var parametricItems = new CatalogHandler().GetParametricProfileItems();
    while (parametricItems.MoveNext())
    {
        var item = parametricItems.Current as ParametricProfileItem;
        if (item == null) continue;
        parametricItemCount++;
        var order = item.ParameterString ?? "";
        var inName = symbolsIn(order);
        var type = item.ProfileItemType.ToString();
        var subType = item.ProfileItemSubType.ToString();
        var parameters = new List<object>();
        var signature = new System.Text.StringBuilder();
        foreach (var raw in item.aProfileItemParameters)
        {
            var parameter = raw as ProfileItemParameter;
            if (parameter == null || String.IsNullOrEmpty(parameter.Symbol)) continue;
            var unitType = parameter.ParameterUnitType.ToString();
            var labelKey = parameter.AlblString ?? "";
            var property = parameter.Property ?? "";
            object defaultValue;
            if (unitType == "INPUT_INTEGER") defaultValue = parameter.IntegerValue;
            else if (unitType == "INPUT_STRING") defaultValue = parameter.StringValue ?? "";
            else defaultValue = parameter.Value;
            parameters.Add(new Dictionary<string, object>
            {
                { "symbol", parameter.Symbol },
                { "label", plainLabel(labelKey.Length > 0 ? labelKey : property) },
                { "label_key", labelKey },
                { "property", property },
                { "unit", unitOf(unitType) },
                { "unit_type", unitType },
                { "default", defaultValue },
                { "in_name", inName.Contains(parameter.Symbol) },
                { "cross_section", parameter.CrossSectionNumber },
            });
            // Everything an entry reports goes into its key, defaults and cross-section included: two prefixes
            // that differ only in a default (a rounding radius the name does not carry) are different shapes.
            var defaultText = defaultValue is double dv ? dv.ToString("R", CultureInfo.InvariantCulture) : Convert.ToString(defaultValue, CultureInfo.InvariantCulture);
            signature.Append(parameter.Symbol).Append('|').Append(property).Append('|').Append(labelKey).Append('|').Append(unitType)
                .Append('|').Append(defaultText).Append('|').Append(parameter.CrossSectionNumber.ToString(CultureInfo.InvariantCulture)).Append(';');
        }
        var groupKey = type + "\n" + subType + "\n" + order + "\n" + (item.IsSketchedUserParametric ? "sketched" : "") + "\n" + signature;
        var prefix = item.ProfilePrefix ?? "";
        if (!groups.ContainsKey(groupKey))
        {
            groups[groupKey] = new Dictionary<string, object>
            {
                { "parameter_string", order },
                { "type", type },
                { "subtype", subType },
                { "family", familyOf(type, subType) },
                { "built_in", type != "PROFILE_USER_PARAMETRIC" },
                { "sketched", item.IsSketchedUserParametric },
                { "parameters", parameters },
            };
            groupPrefixes[groupKey] = new List<string>();
        }
        if (!groupPrefixes[groupKey].Contains(prefix)) groupPrefixes[groupKey].Add(prefix);
    }
    var entries = new List<KeyValuePair<string, Dictionary<string, object>>>();
    foreach (var pair in groups)
    {
        var prefixes = groupPrefixes[pair.Key];
        prefixes.Sort(StringComparer.Ordinal);
        pair.Value["prefixes"] = prefixes;
        entries.Add(new KeyValuePair<string, Dictionary<string, object>>(prefixes[0] + "\n" + pair.Key, pair.Value));
    }
    entries.Sort((a, b) => String.CompareOrdinal(a.Key, b.Key));
    foreach (var entry in entries)
    {
        // One fixed key order for every entry, whatever order the fields were filled in.
        var e = entry.Value;
        parametric.Add(new Dictionary<string, object>
        {
            { "prefixes", e["prefixes"] },
            { "parameter_string", e["parameter_string"] },
            { "type", e["type"] },
            { "subtype", e["subtype"] },
            { "family", e["family"] },
            { "built_in", e["built_in"] },
            { "sketched", e["sketched"] },
            { "parameters", e["parameters"] },
        });
    }
}

var result = new Dictionary<string, object>
{
    { "model_path", modelPath },
    { "resolved", resolved },
    { "catalog_count", catalog.Count },
    { "catalog", catalog },
};
if (wantParametric)
{
    result["environment"] = new Dictionary<string, object> { { "name", environment }, { "profile_database", profileDatabase } };
    result["parametric_item_count"] = parametricItemCount;
    result["parametric_count"] = parametric.Count;
    result["parametric"] = parametric;
}
return result;
""";

    internal const string Code = Usings + "\n" + Labels + "\n" + Body;
}
