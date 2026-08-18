using System;

namespace AwareTekla;

/// <summary>
/// Host-neutral interpretation of the canonical scene member-roll contract.
/// The returned native offset is measured from Tekla's FRONT profile frame in
/// the global work plane; the canonical frame is right-handed about from→to.
/// </summary>
public sealed class TeklaMemberRollPlan
{
    internal TeklaMemberRollPlan(
        double normalizedDegrees,
        double nativeFrontOffsetDegrees,
        double[] axis,
        double[] zeroX,
        double[] zeroY,
        double[] rolledX,
        double[] rolledY)
    {
        NormalizedDegrees = normalizedDegrees;
        NativeFrontOffsetDegrees = nativeFrontOffsetDegrees;
        Axis = axis;
        ZeroX = zeroX;
        ZeroY = zeroY;
        RolledX = rolledX;
        RolledY = rolledY;
    }

    public double NormalizedDegrees { get; }
    public double NativeFrontOffsetDegrees { get; }
    public double[] Axis { get; }
    public double[] ZeroX { get; }
    public double[] ZeroY { get; }
    public double[] RolledX { get; }
    public double[] RolledY { get; }
}

public sealed class TeklaMemberRollContract
{
    public const double VerticalEpsilonSquared = 1e-6;
    public const double AngularToleranceDegrees = 0.01;

    public TeklaMemberRollPlan CreatePlan(double[] from, double[] to, double degrees)
    {
        RequirePoint(from, nameof(from));
        RequirePoint(to, nameof(to));
        if (!IsFinite(degrees))
            throw new ArgumentException("member rot must be a finite JSON number", nameof(degrees));

        var axis = Normalize(Subtract(to, from), "member axis must have nonzero finite length");
        var up = new[] { 0d, 0d, 1d };
        var upDot = Dot(axis, up);
        var perpendicularSquared = Math.Max(0d, 1d - upDot * upDot);

        double[] zeroX;
        double[] zeroY;
        if (perpendicularSquared <= VerticalEpsilonSquared)
        {
            var seed = new[] { 1d, 0d, 0d };
            zeroX = Normalize(Subtract(seed, Scale(axis, Dot(seed, axis))), "vertical member zero frame is degenerate");
            zeroY = Normalize(Cross(axis, zeroX), "vertical member zero frame is degenerate");
        }
        else
        {
            zeroY = Normalize(Subtract(up, Scale(axis, upDot)), "member zero frame is degenerate");
            zeroX = Normalize(Cross(zeroY, axis), "member zero frame is degenerate");
        }

        var normalized = NormalizeDegrees(degrees);
        var radians = normalized * Math.PI / 180d;
        var rolledX = Add(Scale(zeroX, Math.Cos(radians)), Scale(zeroY, Math.Sin(radians)));
        var rolledY = Add(Scale(zeroY, Math.Cos(radians)), Scale(zeroX, -Math.Sin(radians)));

        // With the global work plane active, Tekla's FRONT profile X axis is
        // -projected-global-Z for every non-vertical member. Exact verticals
        // are the API's singular case: +Z starts at +X and -Z starts at -X.
        // Live 2026 B-rep probes established this for W/I, channel, unequal
        // angle, RHS, legacy rectangle and CHS profiles on all axis families.
        double[] nativeFrontX;
        var horizontalSquared = axis[0] * axis[0] + axis[1] * axis[1];
        if (horizontalSquared <= 1e-20)
            nativeFrontX = axis[2] >= 0d ? new[] { 1d, 0d, 0d } : new[] { -1d, 0d, 0d };
        else
            nativeFrontX = Scale(Normalize(Subtract(up, Scale(axis, upDot)), "Tekla FRONT frame is degenerate"), -1d);

        var nativeBaseDegrees = Math.Atan2(Dot(nativeFrontX, zeroY), Dot(nativeFrontX, zeroX)) * 180d / Math.PI;
        var nativeFrontOffset = NormalizeDegrees(normalized - nativeBaseDegrees);
        return new TeklaMemberRollPlan(normalized, nativeFrontOffset, axis, zeroX, zeroY, rolledX, rolledY);
    }

    public static double NormalizeDegrees(double degrees)
    {
        if (!IsFinite(degrees))
            throw new ArgumentException("angle must be finite", nameof(degrees));
        var normalized = ((degrees % 360d) + 360d) % 360d;
        if (normalized >= 180d)
            normalized -= 360d;
        return normalized == 0d ? 0d : normalized;
    }

    public static double EffectiveNativeDegrees(string rotation, double offsetDegrees)
    {
        if (!IsFinite(offsetDegrees))
            throw new ArgumentException("native rotation offset must be finite", nameof(offsetDegrees));
        var baseDegrees = rotation switch
        {
            "FRONT" => 0d,
            "TOP" => 90d,
            "BACK" => 180d,
            "BELOW" => -90d,
            _ => throw new ArgumentException($"unknown Tekla rotation `{rotation}`", nameof(rotation)),
        };
        return NormalizeDegrees(baseDegrees + offsetDegrees);
    }

    public static bool AnglesMatch(double expected, double actual) =>
        Math.Abs(NormalizeDegrees(actual - expected)) <= AngularToleranceDegrees;

    static void RequirePoint(double[] value, string name)
    {
        if (value is null || value.Length != 3 || !IsFinite(value[0]) || !IsFinite(value[1]) || !IsFinite(value[2]))
            throw new ArgumentException("point must contain exactly three finite coordinates", name);
    }

    static bool IsFinite(double value) => !double.IsNaN(value) && !double.IsInfinity(value);

    static double[] Add(double[] a, double[] b) => new[] { a[0] + b[0], a[1] + b[1], a[2] + b[2] };
    static double[] Subtract(double[] a, double[] b) => new[] { a[0] - b[0], a[1] - b[1], a[2] - b[2] };
    static double[] Scale(double[] a, double factor) => new[] { a[0] * factor, a[1] * factor, a[2] * factor };
    static double Dot(double[] a, double[] b) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    static double[] Cross(double[] a, double[] b) => new[]
    {
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    };

    static double[] Normalize(double[] value, string message)
    {
        var length = Math.Sqrt(Dot(value, value));
        if (!(length > 1e-12) || !IsFinite(length))
            throw new ArgumentException(message);
        return Scale(value, 1d / length);
    }
}
