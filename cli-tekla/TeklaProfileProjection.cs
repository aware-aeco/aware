using System;
using System.Collections.Generic;
using System.Linq;
using System.Globalization;
using System.Text;
using System.Text.Json.Nodes;

namespace AwareTekla;

/// <summary>
/// `profile-project` (FloLess #3382 A0p): the pure half of turning a canonical section (AWARE's scene
/// <c>xsection</c> vocabulary, millimetres) into the name a Tekla environment reads as that section.
///
/// Everything here is Tekla-free so it is unit-tested directly; <see cref="ProfileProjectScript"/> does the
/// Tekla half (which aliases the environment has, and the read-back that proves what a name selects).
///
/// Facts this rests on, measured in a live Tekla 2026.0 in the UK (metric) and USA (imperial and metric
/// roles) environments on 2026-11-11, recorded in the PR:
/// <list type="bullet">
/// <item>Open API profile strings are millimetres in every environment and role; inch text is never
///   written (<c>D1"</c> parses as a 1 mm bar, <c>D1/2</c> as a different shape).</item>
/// <item><c>ParametricProfileItem.CreateProfileString()</c> rounds every value to two decimals
///   (0.001 becomes <c>PL0*100</c>), so the name is composed here, with every value at full precision;
///   Tekla parses and keeps full precision.</item>
/// <item>The name follows Tekla's <c>ParameterString</c> by symbol: the parameter list is in another order
///   (HI lists h, b, s, t, r for the name h-s-t*b).</item>
/// <item>PL takes the larger number as its height whatever the order.</item>
/// <item>Comma decimals and exponents are refused by Tekla; the bridge's own culture can be a comma one.</item>
/// </list>
/// </summary>
public static class TeklaProfileProjection
{
    /// <summary>Smallest and largest size, in millimetres, this verb writes into a name.</summary>
    public const double MinSizeMm = 1e-6;
    public const double MaxSizeMm = 1e9;

    /// <summary>Upper bound on sections in one request.</summary>
    public const int MaxSections = 5000;

    /// <summary>
    /// One Tekla built-in shape a canonical shape maps to. A name proves it selected this shape only when
    /// Tekla's read-back reports exactly <see cref="Type"/>, <see cref="SubType"/> and
    /// <see cref="ParameterString"/>, every named symbol, and no symbol outside <see cref="Named"/> and
    /// <see cref="Hidden"/>.
    /// </summary>
    public sealed class Target(
        string shape,
        string type,
        string subType,
        string parameterString,
        string defaultPrefix,
        IReadOnlyList<string> named,
        IReadOnlyList<string> hidden)
    {
        public string Shape { get; } = shape;
        public string Type { get; } = type;
        public string SubType { get; } = subType;
        public string ParameterString { get; } = parameterString;
        public string DefaultPrefix { get; } = defaultPrefix;
        public IReadOnlyList<string> Named { get; } = named;
        public IReadOnlyList<string> Hidden { get; } = hidden;
    }

    /// <summary>The canonical shapes this verb names, and the Tekla shape each becomes.</summary>
    public static readonly IReadOnlyDictionary<string, Target> Targets = new Dictionary<string, Target>(StringComparer.Ordinal)
    {
        ["rect"] = new("rect", "PROFILE_PL", "PROFILE_PL_DEFAULT", "h*b", "PL", ["h", "b"], []),
        ["i"] = new("i", "PROFILE_I", "PROFILE_I_WELDED_SYMMETRICAL", "h-s-t*b", "HI", ["h", "s", "t", "b"], ["r"]),
        ["tee"] = new("tee", "PROFILE_T", "PROFILE_T_PARAMETRIC", "h-s-t-b", "T", ["h", "s", "t", "b"], ["r"]),
        ["angle"] = new("angle", "PROFILE_FPL", "PROFILE_L_COLD_ROLLED", "h*b*t", "L", ["h", "b", "t"], ["r"]),
        ["channel"] = new("channel", "PROFILE_U", "PROFILE_U_COLD_ROLLED", "h*b*t", "U", ["h", "b", "t"], ["r"]),
        ["rhs"] = new("rhs", "PROFILE_P", "PROFILE_P_RECTANGULAR", "h*b*t", "P", ["h", "b", "t"], ["r"]),
        ["chs"] = new("chs", "PROFILE_PD", "PROFILE_PD_CIRCULAR", "d*t", "PD", ["d", "t"], []),
        ["solid-circle"] = new("solid-circle", "PROFILE_D", "PROFILE_D_CIRCULAR", "d", "D", ["d"], []),
    };

    /// <summary>What one canonical section becomes, before Tekla is asked anything.</summary>
    public sealed class Plan(
        string status,
        string? message,
        Target? target,
        IReadOnlyList<(string Symbol, double Value, string From)> values,
        bool quarterTurn)
    {
        public string Status { get; } = status;
        public string? Message { get; } = message;
        public Target? Target { get; } = target;
        public IReadOnlyList<(string Symbol, double Value, string From)> Values { get; } = values;
        public bool QuarterTurn { get; } = quarterTurn;
    }

    /// <summary>
    /// A size written for a Tekla name: invariant culture, dot decimal, the shortest digits that parse back to
    /// the same double, and never an exponent (Tekla refuses one). Only finite sizes inside
    /// [<see cref="MinSizeMm"/>, <see cref="MaxSizeMm"/>] are written; anything else throws.
    /// </summary>
    public static string FormatSize(double value)
    {
        if (double.IsNaN(value) || double.IsInfinity(value) || value < MinSizeMm || value > MaxSizeMm)
            throw new ArgumentOutOfRangeException(nameof(value), value, $"a size must be a finite number of millimetres from {MinSizeMm.ToString("R", CultureInfo.InvariantCulture)} to {MaxSizeMm.ToString("R", CultureInfo.InvariantCulture)}");
        // The fewest significant digits (15, 16, then 17) that parse back to the same double. .NET Framework's
        // "R" is not used: it does not always round-trip there. G17 always does.
        string text = value.ToString("G17", CultureInfo.InvariantCulture);
        foreach (var digits in new[] { "G15", "G16" })
        {
            var candidate = value.ToString(digits, CultureInfo.InvariantCulture);
            if (double.Parse(candidate, NumberStyles.Float, CultureInfo.InvariantCulture) == value) { text = candidate; break; }
        }
        return ExpandExponent(text);
    }

    /// <summary>Rewrites <c>1.5E-05</c> as <c>0.000015</c> (and <c>1E+15</c> as digits); plain text is returned unchanged.</summary>
    public static string ExpandExponent(string text)
    {
        var e = text.IndexOfAny(new[] { 'E', 'e' });
        if (e < 0) return text;
        var mantissa = text.Substring(0, e);
        var exponent = int.Parse(text.Substring(e + 1), NumberStyles.AllowLeadingSign, CultureInfo.InvariantCulture);
        var negative = mantissa.StartsWith("-", StringComparison.Ordinal);
        if (negative) mantissa = mantissa.Substring(1);
        var dot = mantissa.IndexOf('.');
        var digits = dot < 0 ? mantissa : mantissa.Remove(dot, 1);
        var point = (dot < 0 ? mantissa.Length : dot) + exponent; // position of the decimal point within digits
        string result;
        if (point <= 0) result = "0." + new string('0', -point) + digits;
        else if (point >= digits.Length) result = digits + new string('0', point - digits.Length);
        else result = digits.Substring(0, point) + "." + digits.Substring(point);
        if (result.IndexOf('.') >= 0) result = result.TrimEnd('0').TrimEnd('.');
        result = result.TrimStart('0');
        if (result.Length == 0 || result[0] == '.') result = "0" + result;
        return negative ? "-" + result : result;
    }

    /// <summary>
    /// The name in Tekla's order: the prefix, then each symbol of <paramref name="parameterString"/> replaced
    /// by its value, the separators kept (<c>HI</c> + <c>h-s-t*b</c> → <c>HI400-10-15*200</c>).
    /// </summary>
    public static string ComposeName(string prefix, string parameterString, IReadOnlyDictionary<string, double> values)
    {
        var name = new StringBuilder(prefix);
        var symbol = new StringBuilder();
        void Flush()
        {
            if (symbol.Length == 0) return;
            var key = symbol.ToString();
            if (!values.TryGetValue(key, out var v)) throw new ArgumentException($"no value for the symbol '{key}' of '{parameterString}'");
            name.Append(FormatSize(v));
            symbol.Clear();
        }
        foreach (var ch in parameterString)
        {
            if (char.IsLetterOrDigit(ch) || ch == '_') { symbol.Append(ch); continue; }
            Flush();
            name.Append(ch);
        }
        Flush();
        return name.ToString();
    }

    /// <summary>The symbols of a Tekla parameter string, in name order.</summary>
    public static IReadOnlyList<string> SymbolsOf(string parameterString)
    {
        var found = new List<string>();
        var symbol = new StringBuilder();
        foreach (var ch in parameterString + " ")
        {
            if (char.IsLetterOrDigit(ch) || ch == '_') { symbol.Append(ch); continue; }
            if (symbol.Length > 0) { found.Add(symbol.ToString()); symbol.Clear(); }
        }
        return found;
    }

    /// <summary>
    /// Validates one canonical descriptor and maps it onto its Tekla shape. Never throws: a descriptor that
    /// cannot be named comes back with status <c>invalid</c> or <c>no-shape</c> and a plain reason.
    /// </summary>
    public static Plan PlanSection(JsonNode? canonical)
    {
        static Plan Bad(string status, string message) => new(status, message, null, new List<(string, double, string)>(), false);
        if (canonical is not JsonObject o) return Bad("invalid", "\"canonical\" must be an object such as {\"shape\": \"i\", \"d\": 400, \"bf\": 200, \"tw\": 10, \"tf\": 15}.");
        if (o["shape"] is not JsonValue shapeValue || !shapeValue.TryGetValue<string>(out var shape))
            return Bad("invalid", "\"canonical.shape\" must be one of " + string.Join(", ", Targets.Keys) + ".");
        if (!Targets.TryGetValue(shape, out var target))
            return Bad(shape is "double-angle" ? "no-shape" : "invalid",
                shape is "double-angle"
                    ? "A double angle is two members in Tekla, not one parametric profile; it has no single Tekla name."
                    : "\"canonical.shape\" must be one of " + string.Join(", ", Targets.Keys) + ", not '" + shape + "'.");

        var sizes = new Dictionary<string, double>(StringComparer.Ordinal);
        string? Size(string field)
        {
            var node = o[field];
            if (node is not JsonValue v || !v.TryGetValue<double>(out var d))
                return $"\"canonical.{field}\" must be a number of millimetres.";
            if (double.IsNaN(d) || double.IsInfinity(d) || d < MinSizeMm || d > MaxSizeMm)
                return $"\"canonical.{field}\" must be from {FormatSize(MinSizeMm)} to {FormatSize(MaxSizeMm)} mm, not {d.ToString("R", CultureInfo.InvariantCulture)}.";
            sizes[field] = d;
            return null;
        }
        string[] fields = shape switch
        {
            "rect" => ["w", "d"],
            "i" or "channel" or "tee" => ["d", "bf", "tw", "tf"],
            "angle" or "rhs" => ["d", "b", "t"],
            "chs" => ["od", "t"],
            _ => ["diameter"],
        };
        foreach (var field in fields)
            if (Size(field) is { } problem) return Bad("invalid", problem);

        double S(string f) => sizes[f];
        string? geometry = shape switch
        {
            "i" or "channel" when !(2 * S("tf") < S("d")) => "the flanges are thicker than the section is deep (2·tf must be less than d).",
            "i" or "channel" when !(S("tw") < S("bf")) => "the web is not thinner than the flange is wide (tw must be less than bf).",
            "tee" when !(S("tf") < S("d")) => "the flange is not thinner than the tee is deep (tf must be less than d).",
            "tee" when !(S("tw") < S("bf")) => "the web is not thinner than the flange is wide (tw must be less than bf).",
            "angle" when !(S("t") < Math.Min(S("d"), S("b"))) => "the thickness is not less than both legs (t must be less than d and b).",
            "rhs" when !(2 * S("t") < Math.Min(S("d"), S("b"))) => "the walls leave no hollow (2·t must be less than d and b).",
            "chs" when !(2 * S("t") < S("od")) => "the wall leaves no hollow (2·t must be less than od).",
            _ => null,
        };
        if (geometry != null) return Bad("invalid", "This " + shape + " cannot exist: " + geometry);

        var values = new List<(string, double, string)>();
        bool quarterTurn = false;
        switch (shape)
        {
            case "rect":
                // Tekla's plate always takes the larger number as its height; a wide plate is therefore Tekla's
                // tall plate turned a quarter (see section_transform).
                quarterTurn = S("w") > S("d");
                values.Add(("h", Math.Max(S("w"), S("d")), S("w") > S("d") ? "w" : "d"));
                values.Add(("b", Math.Min(S("w"), S("d")), S("w") > S("d") ? "d" : "w"));
                break;
            case "i":
            case "tee":
                values.Add(("h", S("d"), "d")); values.Add(("s", S("tw"), "tw")); values.Add(("t", S("tf"), "tf")); values.Add(("b", S("bf"), "bf"));
                break;
            case "channel":
                // Tekla's parametric channel is cold-rolled: one thickness for web and flanges.
                if (S("tw") != S("tf"))
                    return Bad("no-shape", $"Tekla's parametric channel has one thickness for web and flanges; this channel has tw {FormatSize(S("tw"))} and tf {FormatSize(S("tf"))} mm.");
                values.Add(("h", S("d"), "d")); values.Add(("b", S("bf"), "bf")); values.Add(("t", S("tw"), "tw"));
                break;
            case "angle":
            case "rhs":
                values.Add(("h", S("d"), "d")); values.Add(("b", S("b"), "b")); values.Add(("t", S("t"), "t"));
                break;
            case "chs":
                values.Add(("d", S("od"), "od")); values.Add(("t", S("t"), "t"));
                break;
            default:
                values.Add(("d", S("diameter"), "diameter"));
                break;
        }
        // Name order, as Tekla writes it.
        var order = SymbolsOf(target.ParameterString);
        var ordered = order.Select(sym => values.Single(v => v.Item1 == sym)).ToList();
        return new Plan("planned", null, target, ordered, quarterTurn);
    }

    /// <summary>
    /// Request-level refusals, before Tekla is touched: the shape of the request, not of any one section
    /// (a section that cannot be named is a per-section status, and the batch continues). Null when the
    /// request is well formed.
    /// </summary>
    public static string? ValidateRequest(JsonNode? input)
    {
        if (input is not JsonObject root) return "the request must be a JSON object with \"sections\".";
        if (root["sections"] is not JsonArray sections) return "\"sections\" must be a list of sections.";
        if (sections.Count == 0) return "\"sections\" is empty; give at least one section.";
        if (sections.Count > MaxSections) return $"\"sections\" has {sections.Count} entries; one request may name at most {MaxSections}.";
        var ids = new HashSet<string>(StringComparer.Ordinal);
        for (int i = 0; i < sections.Count; i++)
        {
            if (sections[i] is not JsonObject s) return $"sections[{i}] must be an object.";
            if (s["id"] is not JsonValue idValue || !idValue.TryGetValue<string>(out var id) || string.IsNullOrWhiteSpace(id))
                return $"sections[{i}].id must be a non-empty text the caller uses to match the answer.";
            if (!ids.Add(id)) return $"sections[{i}].id '{id}' is used twice; each section needs its own id.";
            if (s["canonical"] is null) return $"sections[{i}] ('{id}') has no \"canonical\" descriptor.";
        }
        if (root["prefixes"] is { } prefixes)
        {
            if (prefixes is not JsonObject map) return "\"prefixes\" must be an object such as {\"i\": \"WI\"}.";
            foreach (var entry in map)
            {
                var shape = entry.Key;
                var value = entry.Value;
                if (!Targets.ContainsKey(shape)) return $"\"prefixes\" names '{shape}', which is not one of " + string.Join(", ", Targets.Keys) + ".";
                if (value is not JsonValue pv || !pv.TryGetValue<string>(out var p) || string.IsNullOrWhiteSpace(p))
                    return $"\"prefixes.{shape}\" must be a Tekla prefix (text).";
            }
        }
        return null;
    }

    /// <summary>
    /// The script's input for one section: the plan as plain JSON (numbers as numbers, written by
    /// System.Text.Json's round-trip formatting, so the script reads back the same doubles).
    /// </summary>
    public static JsonObject PlanToArgs(JsonObject section, Plan plan, string? requestedPrefix)
    {
        var o = new JsonObject
        {
            ["id"] = section["id"]!.DeepClone(),
            ["status"] = plan.Status,
            ["quarter_turn"] = plan.QuarterTurn,
        };
        foreach (var echo in new[] { "canonical", "definition", "sizes", "faces", "reference_point" })
            if (section[echo] is { } node) o[echo] = node.DeepClone();
        if (plan.Message != null) o["message"] = plan.Message;
        if (plan.Target is { } t)
        {
            o["target"] = new JsonObject
            {
                ["shape"] = t.Shape, ["type"] = t.Type, ["subtype"] = t.SubType, ["parameter_string"] = t.ParameterString,
                ["default_prefix"] = t.DefaultPrefix,
                ["named"] = new JsonArray(t.Named.Select(n => (JsonNode)n).ToArray()),
                ["hidden"] = new JsonArray(t.Hidden.Select(n => (JsonNode)n).ToArray()),
            };
            o["values"] = new JsonArray(plan.Values.Select(v => (JsonNode)new JsonObject { ["symbol"] = v.Symbol, ["value"] = v.Value, ["from"] = v.From }).ToArray());
            if (requestedPrefix != null) o["requested_prefix"] = requestedPrefix;
        }
        return o;
    }
}
