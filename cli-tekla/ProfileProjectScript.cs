namespace AwareTekla;

/// <summary>
/// `profile-project` (FloLess #3382 A0p): the Tekla half of naming a canonical section. The plan (which Tekla
/// shape, which values in which order) is made in <see cref="TeklaProfileProjection"/> before Tekla is asked;
/// this script picks the alias the open environment has, composes the name and proves what that name selects:
/// no library profile of that name (Tekla resolves the library first), and a parametric read-back that reports
/// the intended shape and every value bit for bit. Read-only: it inserts nothing and commits nothing.
/// </summary>
internal static class ProfileProjectScript
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

    internal const string Body = """

var m = model as Model;
if (m == null || !m.GetConnectionStatus()) throw new Exception("No Tekla model is open, so there is no profile catalogue to ask.");
string modelPath = "";
try { modelPath = m.GetInfo().ModelPath; } catch (Exception ex) { throw new Exception("Tekla did not say which model is open: " + ex.Message); }
if (String.IsNullOrWhiteSpace(modelPath)) throw new Exception("Tekla did not say which model is open.");

// Which environment answered: the folder after "\environments\" in the profile database path (as profile-catalog).
string profileDatabase = "";
string environment = "";
try { Tekla.Structures.TeklaStructuresSettings.GetAdvancedOption("XS_PROFDB", ref profileDatabase); } catch { profileDatabase = ""; }
profileDatabase = profileDatabase ?? "";
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

// The environment's parametric catalogue: every prefix with the type and subtype it is listed under.
var listed = new List<Tuple<string, string, string>>(); // prefix, type, subtype
{
    var items = new CatalogHandler().GetParametricProfileItems();
    while (items.MoveNext())
    {
        var item = items.Current as ParametricProfileItem;
        if (item == null || item.ProfilePrefix == null) continue;
        listed.Add(Tuple.Create(item.ProfilePrefix, item.ProfileItemType.ToString(), item.ProfileItemSubType.ToString()));
    }
}

Func<object, double> num = v =>
{
    if (v is double d) return d;
    if (v is float f) return f;
    if (v is int i) return i;
    if (v is long l) return l;
    if (v is decimal dm) return (double)dm;
    throw new Exception("a planned value is not a number");
};
Func<double, string> exact = v => v.ToString("R", CultureInfo.InvariantCulture);

var results = new List<object>();
var named = new List<string>();
foreach (var raw in (IList)args["sections"])
{
    var section = (IDictionary<string, object>)raw;
    var row = new Dictionary<string, object>();
    row["id"] = section["id"];
    foreach (var echo in new[] { "canonical", "definition", "sizes", "faces", "reference_point" })
    {
        object echoed;
        if (section.TryGetValue(echo, out echoed)) row[echo] = echoed;
    }
    results.Add(row);
    var status = (string)section["status"];
    if (status != "planned")
    {
        row["status"] = status;
        row["message"] = section["message"];
        continue;
    }
    try
    {
        var target = (IDictionary<string, object>)section["target"];
        var type = (string)target["type"];
        var subtype = (string)target["subtype"];
        var parameterString = (string)target["parameter_string"];
        var namedSymbols = ((IList)target["named"]).Cast<object>().Select(o => (string)o).ToList();
        var hiddenSymbols = ((IList)target["hidden"]).Cast<object>().Select(o => (string)o).ToList();
        var values = ((IList)section["values"]).Cast<IDictionary<string, object>>().ToList();
        var intended = new Dictionary<string, double>(StringComparer.Ordinal);
        foreach (var v in values) intended[(string)v["symbol"]] = num(v["value"]);

        // Aliases of this shape here: the prefixes the catalogue lists under the same subtype. When it lists none
        // (the catalogue lists the rectangular tube's prefixes under the square form, P h*t), the prefixes of the
        // same type; the read-back below proves which form a name selects either way.
        var sameType = listed.Where(l => l.Item2 == type).ToList();
        var sameSubtype = sameType.Where(l => l.Item3 == subtype).Select(l => l.Item1).Distinct().OrderBy(p => p, StringComparer.Ordinal).ToList();
        var aliases = sameSubtype.Count > 0 ? sameSubtype : sameType.Select(l => l.Item1).Distinct().OrderBy(p => p, StringComparer.Ordinal).ToList();
        object requestedRaw;
        string requested = section.TryGetValue("requested_prefix", out requestedRaw) ? requestedRaw as string : null;
        string defaultPrefix = (string)target["default_prefix"];
        string prefix;
        if (requested != null)
        {
            if (!aliases.Contains(requested))
            {
                row["status"] = "prefix-not-available";
                row["message"] = "This environment does not list '" + requested + "' as a prefix of this shape. Its prefixes are: " + (aliases.Count == 0 ? "none" : String.Join(", ", aliases)) + ".";
                row["tekla"] = new Dictionary<string, object> { { "type", type }, { "subtype", subtype }, { "aliases", aliases } };
                continue;
            }
            prefix = requested;
        }
        else if (aliases.Contains(defaultPrefix)) prefix = defaultPrefix;
        else if (aliases.Count > 0) prefix = aliases[0];
        else
        {
            row["status"] = "no-shape";
            row["message"] = "This environment's catalogue lists no parametric profile of type " + type + ".";
            continue;
        }

        var name = AwareTekla.TeklaProfileProjection.ComposeName(prefix, parameterString, intended);
        row["name"] = name;
        row["tekla"] = new Dictionary<string, object> { { "type", type }, { "subtype", subtype }, { "prefix", prefix }, { "aliases", aliases }, { "parameter_string", parameterString } };
        row["parameters"] = values.Select(v => (object)new Dictionary<string, object> { { "symbol", v["symbol"] }, { "value", num(v["value"]) }, { "from", v["from"] } }).ToList();
        if ((bool)section["quarter_turn"])
            row["section_transform"] = new Dictionary<string, object>
            {
                { "from", "tekla-section" }, { "to", "canonical" },
                { "matrix", new[] { new[] { 0, -1 }, new[] { 1, 0 } } }, { "origin", new[] { 0, 0 } },
                { "note", "canonical = matrix x tekla, about the section origin (the bounding-box centre): Tekla's plate is always taller than wide, so this wide plate is Tekla's plate turned 90 degrees counter-clockwise. The plate is symmetric, so turning it clockwise gives the same outline." },
            };

        // A library profile of this name would be selected instead of the parametric shape.
        if (new LibraryProfileItem().Select(name))
        {
            row["status"] = "library-collision";
            row["message"] = "The environment's profile library has a profile called '" + name + "', and Tekla selects the library profile before a parametric one, so this name would not give the section asked for.";
            continue;
        }
        var back = new ParametricProfileItem();
        if (!back.Select(name))
        {
            row["status"] = "read-back-mismatch";
            row["message"] = "Tekla did not read '" + name + "' as a parametric profile.";
            continue;
        }
        var problems = new List<string>();
        var readType = back.ProfileItemType.ToString();
        var readSubtype = back.ProfileItemSubType.ToString();
        if (back.ProfilePrefix != prefix) problems.Add("prefix '" + back.ProfilePrefix + "', expected '" + prefix + "'");
        if (readType != type) problems.Add("type " + readType + ", expected " + type);
        if (readSubtype != subtype) problems.Add("subtype " + readSubtype + ", expected " + subtype);
        if (back.ParameterString != parameterString) problems.Add("name order '" + back.ParameterString + "', expected '" + parameterString + "'");
        var readParameters = new List<object>();
        var hidden = new List<object>();
        var seen = new Dictionary<string, int>(StringComparer.Ordinal);
        foreach (var rawParameter in back.aProfileItemParameters)
        {
            var p = rawParameter as ProfileItemParameter;
            if (p == null || String.IsNullOrEmpty(p.Symbol)) continue;
            int count; seen.TryGetValue(p.Symbol, out count); seen[p.Symbol] = count + 1;
            var unitType = p.ParameterUnitType.ToString();
            readParameters.Add(new Dictionary<string, object> { { "symbol", p.Symbol }, { "value", p.Value }, { "unit_type", unitType } });
            if (intended.ContainsKey(p.Symbol))
            {
                if (unitType != "INPUT_DIMENSION_UNIT") problems.Add(p.Symbol + " is a " + unitType + ", expected a length");
                // Bit for bit: the name must carry the size exactly, not to a rounding.
                if (BitConverter.DoubleToInt64Bits(p.Value) != BitConverter.DoubleToInt64Bits(intended[p.Symbol]))
                    problems.Add(p.Symbol + " read back as " + exact(p.Value) + ", expected " + exact(intended[p.Symbol]));
            }
            else if (hiddenSymbols.Contains(p.Symbol))
                hidden.Add(new Dictionary<string, object> { { "symbol", p.Symbol }, { "value", p.Value }, { "unit_type", unitType } });
            else problems.Add("an unexpected parameter '" + p.Symbol + "'");
        }
        foreach (var symbol in namedSymbols)
        {
            int count; seen.TryGetValue(symbol, out count);
            if (count != 1) problems.Add(symbol + (count == 0 ? " is missing" : " appears " + count + " times"));
        }
        row["hidden_parameters"] = hidden;
        row["read_back"] = new Dictionary<string, object> { { "prefix", back.ProfilePrefix }, { "type", readType }, { "subtype", readSubtype }, { "parameter_string", back.ParameterString }, { "parameters", readParameters } };
        if (problems.Count > 0)
        {
            row["status"] = "read-back-mismatch";
            row["message"] = "Tekla reads '" + name + "' differently from what was asked: " + String.Join("; ", problems) + ".";
            continue;
        }
        row["status"] = "named";
        if (!named.Contains(name)) named.Add(name);
    }
    catch (Exception ex)
    {
        row["status"] = "error";
        row["message"] = ex.Message;
    }
}

var result = new Dictionary<string, object>
{
    { "model_path", modelPath },
    { "environment", new Dictionary<string, object> { { "name", environment }, { "profile_database", profileDatabase } } },
    { "named_count", named.Count },
    { "sections", results },
};
// Ready to send to section-measure, which measures each name's section, reference point and placement on real
// temporary members. Left out when nothing was named (section-measure refuses an empty list).
if (named.Count > 0) result["measure"] = new Dictionary<string, object> { { "profiles", named } };
return result;
""";

    internal const string Code = Usings + "\n" + Body;
}
