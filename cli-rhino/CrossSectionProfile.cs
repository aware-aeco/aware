using System.Text.Json;
using System.Text.Json.Nodes;

namespace AwareRhino;

/// <summary>
/// Strict, host-neutral decoder for the canonical scene <c>xsection</c>
/// discriminated union. No RhinoCommon types belong here: the result is
/// executable-testable and is passed to the live bridge as normalized data.
/// </summary>
internal static class CrossSectionProfile
{
    internal const string GeometryRevision = "rhino-profile-v4";

    internal sealed record Plan(
        string Shape,
        double Width,
        double Depth,
        IReadOnlyDictionary<string, double> Dimensions,
        IReadOnlyList<double> Residuals,
        double Area,
        double Perimeter,
        int ProfileCount)
    {
        internal JsonObject ToJson()
        {
            var componentCount = Shape == "double-angle" ? 2 : 1;
            var dimensions = new JsonObject();
            foreach (var (key, value) in Dimensions) dimensions[key] = value;
            var residuals = new JsonArray();
            foreach (var value in Residuals) residuals.Add(value);
            var result = new JsonObject
            {
                ["revision"] = GeometryRevision,
                ["shape"] = Shape,
                ["w"] = Width,
                ["d"] = Depth,
                ["dimensions"] = dimensions,
                ["residuals"] = residuals,
                ["area"] = Area,
                ["perimeter"] = Perimeter,
                ["profileCount"] = ProfileCount,
                ["componentCount"] = componentCount,
                ["capCount"] = 2 * componentCount,
            };
            if (Shape == "double-angle")
            {
                var components = new JsonArray();
                foreach (var outline in DoubleAngleOutlines(Dimensions))
                {
                    var points = new JsonArray();
                    foreach (var (x, y) in outline) points.Add(new JsonArray(x, y));
                    components.Add(points);
                }
                result["components"] = components;
            }
            return result;
        }
    }

    internal sealed record DecodeResult(Plan? Profile, string? Error)
    {
        internal bool Ok => Profile is not null && Error is null;
    }

    internal static DecodeResult Decode(JsonObject element, double sectionWidth, double sectionDepth)
    {
        if (!FinitePositive(sectionWidth) || !FinitePositive(sectionDepth))
            return Fail("section.{w,d} must contain positive finite JSON numbers");

        if (!element.ContainsKey("xsection"))
            return Success(Rect(sectionWidth, sectionDepth));
        if (element["xsection"] is not JsonObject xs)
            return Fail("xsection, when present, must be an object");
        if (!StrictString(xs["shape"], out var shape)
            || shape is not ("i" or "channel" or "angle" or "rhs" or "chs" or "rect" or "tee" or "double-angle"))
            return Fail("xsection.shape must be exactly one of i|channel|angle|rhs|chs|rect|tee|double-angle");

        return shape switch
        {
            "i" => DecodeI(xs, sectionWidth, sectionDepth),
            "channel" => DecodeChannel(xs, sectionWidth, sectionDepth),
            "angle" => DecodeAngle(xs, sectionWidth, sectionDepth),
            "rhs" => DecodeRhs(xs, sectionWidth, sectionDepth),
            "chs" => DecodeChs(xs, sectionWidth, sectionDepth),
            "rect" => DecodeRect(xs, sectionWidth, sectionDepth),
            "tee" => DecodeTee(xs, sectionWidth, sectionDepth),
            "double-angle" => DecodeDoubleAngle(xs, sectionWidth, sectionDepth),
            _ => Fail("unsupported xsection.shape"),
        };
    }

    internal static double EnvelopeTolerance(double actual, double expected)
        => Math.Max(0.000001, 1e-9 * Math.Max(1, Math.Max(Math.Abs(actual), Math.Abs(expected))));

    internal static IReadOnlyList<IReadOnlyList<(double X, double Y)>> DoubleAngleOutlines(
        IReadOnlyDictionary<string, double> dimensions)
    {
        var t = dimensions["t"];
        var gap = dimensions["gap"];
        var back = dimensions["back"];
        var outstanding = dimensions["outstanding"];
        var halfGap = gap / 2;
        var halfBack = back / 2;
        IReadOnlyList<(double X, double Y)> left =
        [
            (-halfGap, -halfBack), (-halfGap, halfBack),
            (-halfGap - t, halfBack), (-halfGap - t, -halfBack + t),
            (-halfGap - outstanding, -halfBack + t),
            (-halfGap - outstanding, -halfBack),
        ];
        var right = left.Reverse().Select(point => (-point.X, point.Y)).ToArray();
        return [left, right];
    }

    static DecodeResult DecodeI(JsonObject xs, double w, double d)
    {
        if (!Dimensions(xs, ["d", "bf", "tw", "tf"], out var v, out var error))
            return Fail(error);
        if (!Matches(v["bf"], w) || !Matches(v["d"], d))
            return Fail("xsection i {bf,d} must match section {w,d}");
        var tw = v["tw"];
        var tf = v["tf"];
        if (!(tw < w) || !(2 * tf < d))
            return Fail("xsection i requires tw < bf and 2*tf < d");
        var points = new[]
        {
            (-w / 2, -d / 2), (w / 2, -d / 2), (w / 2, -d / 2 + tf),
            (tw / 2, -d / 2 + tf), (tw / 2, d / 2 - tf), (w / 2, d / 2 - tf),
            (w / 2, d / 2), (-w / 2, d / 2), (-w / 2, d / 2 - tf),
            (-tw / 2, d / 2 - tf), (-tw / 2, -d / 2 + tf), (-w / 2, -d / 2 + tf),
        };
        return Success(new Plan("i", w, d, v, [tw, tf, (w - tw) / 2, d - 2 * tf],
            2 * w * tf + (d - 2 * tf) * tw, PolygonPerimeter(points), 1));
    }

    static DecodeResult DecodeChannel(JsonObject xs, double w, double d)
    {
        if (xs.ContainsKey("b"))
            return Fail("xsection channel uses canonical bf; the b alias is not accepted");
        if (!Dimensions(xs, ["d", "bf", "tw", "tf"], out var v, out var error))
            return Fail(error);
        if (!Matches(v["bf"], w) || !Matches(v["d"], d))
            return Fail("xsection channel {bf,d} must match section {w,d}");
        var tw = v["tw"];
        var tf = v["tf"];
        if (!(tw < w) || !(2 * tf < d))
            return Fail("xsection channel requires tw < bf and 2*tf < d");
        var points = new[]
        {
            (-w / 2, -d / 2), (w / 2, -d / 2), (w / 2, -d / 2 + tf),
            (-w / 2 + tw, -d / 2 + tf), (-w / 2 + tw, d / 2 - tf),
            (w / 2, d / 2 - tf), (w / 2, d / 2), (-w / 2, d / 2),
        };
        return Success(new Plan("channel", w, d, v, [tw, tf, w - tw, d - 2 * tf],
            2 * w * tf + (d - 2 * tf) * tw, PolygonPerimeter(points), 1));
    }

    static DecodeResult DecodeAngle(JsonObject xs, double w, double d)
    {
        if (!Dimensions(xs, ["d", "b", "t"], out var v, out var error))
            return Fail(error);
        if (!Matches(v["b"], w) || !Matches(v["d"], d))
            return Fail("xsection angle {b,d} must match section {w,d}");
        var t = v["t"];
        if (!(t < Math.Min(w, d)))
            return Fail("xsection angle requires t < min(b,d)");
        var points = new[]
        {
            (-w / 2, -d / 2), (w / 2, -d / 2), (w / 2, -d / 2 + t),
            (-w / 2 + t, -d / 2 + t), (-w / 2 + t, d / 2), (-w / 2, d / 2),
        };
        return Success(new Plan("angle", w, d, v, [t, w - t, d - t],
            t * (w + d - t), PolygonPerimeter(points), 1));
    }

    static DecodeResult DecodeRhs(JsonObject xs, double w, double d)
    {
        if (!Dimensions(xs, ["d", "b", "t"], out var v, out var error))
            return Fail(error);
        if (!Matches(v["b"], w) || !Matches(v["d"], d))
            return Fail("xsection rhs {b,d} must match section {w,d}");
        var t = v["t"];
        if (!(2 * t < Math.Min(w, d)))
            return Fail("xsection rhs requires 2*t < min(b,d)");
        var innerW = w - 2 * t;
        var innerD = d - 2 * t;
        return Success(new Plan("rhs", w, d, v, [t, innerW, innerD],
            w * d - innerW * innerD, 2 * (w + d + innerW + innerD), 2));
    }

    static DecodeResult DecodeChs(JsonObject xs, double w, double d)
    {
        if (!Dimensions(xs, ["od", "t"], out var v, out var error))
            return Fail(error);
        var od = v["od"];
        if (!Matches(od, w) || !Matches(od, d))
            return Fail("xsection chs od must independently match section.w and section.d");
        var t = v["t"];
        if (!(2 * t < od))
            return Fail("xsection chs requires 2*t < od");
        var inner = od - 2 * t;
        return Success(new Plan("chs", w, d, v, [t, inner],
            Math.PI / 4 * (od * od - inner * inner), Math.PI * (od + inner), 2));
    }

    static DecodeResult DecodeRect(JsonObject xs, double w, double d)
    {
        if (!Dimensions(xs, ["w", "d"], out var v, out var error))
            return Fail(error);
        return !Matches(v["w"], w) || !Matches(v["d"], d)
            ? Fail("xsection rect {w,d} must match section {w,d}")
            : Success(Rect(w, d));
    }

    static DecodeResult DecodeTee(JsonObject xs, double w, double d)
    {
        if (!Dimensions(xs, ["d", "bf", "tw", "tf"], out var v, out var error))
            return Fail(error);
        if (!Matches(v["bf"], w) || !Matches(v["d"], d))
            return Fail("xsection tee {bf,d} must match section {w,d}");
        var tw = v["tw"];
        var tf = v["tf"];
        if (!(tw < w) || !(tf < d))
            return Fail("xsection tee requires tw < bf and tf < d");
        var points = new[]
        {
            (-tw / 2, -d / 2), (tw / 2, -d / 2), (tw / 2, d / 2 - tf),
            (w / 2, d / 2 - tf), (w / 2, d / 2), (-w / 2, d / 2),
            (-w / 2, d / 2 - tf), (-tw / 2, d / 2 - tf),
        };
        return Success(new Plan("tee", w, d, v, [tw, tf, (w - tw) / 2, d - tf],
            w * tf + (d - tf) * tw, PolygonPerimeter(points), 1));
    }

    static DecodeResult DecodeDoubleAngle(JsonObject xs, double w, double d)
    {
        if (!Dimensions(xs, ["d", "b", "t"], out var parsed, out var error))
            return Fail(error);
        if (!StrictNumber(xs["gap"], out var gap) || gap < 0)
            return Fail("xsection.gap must be a non-negative finite JSON number");
        if (!StrictString(xs["orientation"], out var orientation)
            || orientation is not ("llbb" or "slbb"))
            return Fail("xsection.orientation must be exactly llbb or slbb");

        var angleDepth = parsed["d"];
        var angleWidth = parsed["b"];
        var t = parsed["t"];
        if (!(t < Math.Min(angleDepth, angleWidth)))
            return Fail("xsection double-angle requires t < min(b,d)");

        var back = orientation == "llbb" ? angleDepth : angleWidth;
        var outstanding = orientation == "llbb" ? angleWidth : angleDepth;
        if (!Matches(2 * outstanding + gap, w) || !Matches(back, d))
            return Fail("xsection double-angle orientation and dimensions must match section {w,d}");

        var dimensions = new Dictionary<string, double>(parsed, StringComparer.Ordinal)
        {
            ["gap"] = gap,
            ["back"] = back,
            ["outstanding"] = outstanding,
        };
        var residuals = new List<double> { t, back - t, outstanding - t };
        if (gap > 0) residuals.Add(gap);
        return Success(new Plan(
            "double-angle",
            w,
            d,
            dimensions,
            residuals,
            2 * t * (back + outstanding - t),
            4 * (back + outstanding),
            1));
    }

    static Plan Rect(double w, double d)
        => new("rect", w, d,
            new Dictionary<string, double> { ["w"] = w, ["d"] = d },
            [w, d], w * d, 2 * (w + d), 1);

    static bool Dimensions(
        JsonObject xs,
        string[] keys,
        out IReadOnlyDictionary<string, double> values,
        out string error)
    {
        var result = new Dictionary<string, double>(StringComparer.Ordinal);
        foreach (var key in keys)
        {
            if (!StrictNumber(xs[key], out var value) || value <= 0)
            {
                values = result;
                error = $"xsection.{key} must be a positive finite JSON number";
                return false;
            }
            result[key] = value;
        }
        values = result;
        error = "";
        return true;
    }

    static bool StrictNumber(JsonNode? node, out double value)
    {
        value = double.NaN;
        return node is JsonValue
               && node.GetValueKind() == JsonValueKind.Number
               && node.AsValue().TryGetValue<double>(out value)
               && double.IsFinite(value);
    }

    static bool StrictString(JsonNode? node, out string value)
    {
        value = "";
        return node is JsonValue
               && node.GetValueKind() == JsonValueKind.String
               && node.AsValue().TryGetValue<string>(out value!)
               && value is not null;
    }

    static bool Matches(double actual, double expected)
        => Math.Abs(actual - expected) <= EnvelopeTolerance(actual, expected);

    static bool FinitePositive(double value) => double.IsFinite(value) && value > 0;

    static double PolygonPerimeter(IReadOnlyList<(double X, double Y)> points)
    {
        double result = 0;
        for (var i = 0; i < points.Count; i++)
        {
            var a = points[i];
            var b = points[(i + 1) % points.Count];
            result += Math.Sqrt(Math.Pow(a.X - b.X, 2) + Math.Pow(a.Y - b.Y, 2));
        }
        return result;
    }

    static DecodeResult Success(Plan plan) => new(plan, null);
    static DecodeResult Fail(string error) => new(null, error);
}
