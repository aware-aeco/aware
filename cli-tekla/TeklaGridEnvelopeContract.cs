using System;
using System.Collections.Generic;
using System.Linq;

namespace AwareTekla;

/// <summary>
/// Host-agnostic input for one canonical structural-grid axis. The bake script
/// constructs these records after canonical validation, then delegates every
/// Tekla-specific representability decision to <see cref="TeklaGridEnvelopeContract"/>.
/// </summary>
public sealed class GridAxisExtentContract
{
    public GridAxisExtentContract(
        string id,
        string direction,
        double offsetMm,
        string label,
        double startMm,
        double endMm)
    {
        Id = id;
        Direction = direction;
        OffsetMm = offsetMm;
        Label = label;
        StartMm = startMm;
        EndMm = endMm;
    }

    public string Id { get; }
    public string Direction { get; }
    public double OffsetMm { get; }
    public string Label { get; }
    public double StartMm { get; }
    public double EndMm { get; }
}

public sealed class GridLevelContract
{
    public GridLevelContract(string id, double elevationMm, string label)
    {
        Id = id;
        ElevationMm = elevationMm;
        Label = label;
    }

    public string Id { get; }
    public double ElevationMm { get; }
    public string Label { get; }
}

public sealed class TeklaGridEnvelopeResult
{
    internal TeklaGridEnvelopeResult(
        bool isSupported,
        string code,
        string message,
        double xFamilyStartMm,
        double xFamilyEndMm,
        double yFamilyStartMm,
        double yFamilyEndMm,
        bool expandsAuthoredExtents)
    {
        IsSupported = isSupported;
        Code = code;
        Message = message;
        XFamilyStartMm = xFamilyStartMm;
        XFamilyEndMm = xFamilyEndMm;
        YFamilyStartMm = yFamilyStartMm;
        YFamilyEndMm = yFamilyEndMm;
        ExpandsAuthoredExtents = expandsAuthoredExtents;
    }

    public bool IsSupported { get; }
    public string Code { get; }
    public string Message { get; }

    // X-family grid lines sit at X offsets and run in Y. Y-family grid lines
    // sit at Y offsets and run in X.
    public double XFamilyStartMm { get; }
    public double XFamilyEndMm { get; }
    public double YFamilyStartMm { get; }
    public double YFamilyEndMm { get; }
    public bool ExpandsAuthoredExtents { get; }
}

/// <summary>
/// Computes the tightest native Tekla Grid envelope that cannot truncate any
/// authored axis. This class deliberately has no Tekla references so unit tests
/// execute the exact production algorithm used by the Roslyn bake script.
/// </summary>
public sealed class TeklaGridEnvelopeContract
{
    const double DuplicateToleranceMm = 1e-9;
    const double ExpansionToleranceMm = 1e-9;

    public TeklaGridEnvelopeResult Evaluate(
        IReadOnlyList<GridAxisExtentContract> axes,
        IReadOnlyList<GridLevelContract> levels)
    {
        if (axes is null) throw new ArgumentNullException(nameof(axes));
        if (levels is null) throw new ArgumentNullException(nameof(levels));

        var xs = axes.Where(axis => axis.Direction == "x").OrderBy(axis => axis.OffsetMm).ToList();
        var ys = axes.Where(axis => axis.Direction == "y").OrderBy(axis => axis.OffsetMm).ToList();

        if (xs.Count == 0 || ys.Count == 0)
            return Unsupported(
                "tekla-grid-single-family-unsupported",
                "Tekla native Grid requires both X and Y axis families.");

        if (axes.Any(axis => string.IsNullOrWhiteSpace(axis.Label))
            || levels.Any(level => string.IsNullOrWhiteSpace(level.Label)))
            return Unsupported(
                "tekla-grid-blank-label-unsupported",
                "Tekla native Grid requires a non-blank label for every axis and level.");

        if (HasDuplicate(xs.Select(axis => axis.OffsetMm))
            || HasDuplicate(ys.Select(axis => axis.OffsetMm)))
            return Unsupported(
                "tekla-grid-duplicate-axis-offset-unsupported",
                "Tekla native Grid cannot preserve duplicate offsets within one axis family.");

        if (HasDuplicate(levels.Select(level => level.ElevationMm)))
            return Unsupported(
                "tekla-grid-duplicate-elevation-unsupported",
                "Tekla native Grid cannot preserve duplicate elevation levels.");

        var xFamilyStart = Math.Min(xs.Min(axis => axis.StartMm), ys.Min(axis => axis.OffsetMm));
        var xFamilyEnd = Math.Max(xs.Max(axis => axis.EndMm), ys.Max(axis => axis.OffsetMm));
        var yFamilyStart = Math.Min(ys.Min(axis => axis.StartMm), xs.Min(axis => axis.OffsetMm));
        var yFamilyEnd = Math.Max(ys.Max(axis => axis.EndMm), xs.Max(axis => axis.OffsetMm));

        var envelopeValues = new[]
        {
            xFamilyStart,
            xFamilyEnd,
            yFamilyStart,
            yFamilyEnd,
        };
        var extensions = new[]
        {
            ys[0].OffsetMm - xFamilyStart,
            xFamilyEnd - ys[ys.Count - 1].OffsetMm,
            xs[0].OffsetMm - yFamilyStart,
            yFamilyEnd - xs[xs.Count - 1].OffsetMm,
        };
        if (envelopeValues.Any(value => !IsFinite(value))
            || extensions.Any(value => !IsFinite(value) || value < 0))
            return Unsupported(
                "tekla-grid-derived-envelope-unsupported",
                "The shared native Grid envelope overflows or cannot contain every authored axis.");

        var expands = xs.Any(axis => Math.Abs(axis.StartMm - xFamilyStart) > ExpansionToleranceMm
                                  || Math.Abs(axis.EndMm - xFamilyEnd) > ExpansionToleranceMm)
                   || ys.Any(axis => Math.Abs(axis.StartMm - yFamilyStart) > ExpansionToleranceMm
                                  || Math.Abs(axis.EndMm - yFamilyEnd) > ExpansionToleranceMm);

        return new TeklaGridEnvelopeResult(
            true,
            string.Empty,
            string.Empty,
            xFamilyStart,
            xFamilyEnd,
            yFamilyStart,
            yFamilyEnd,
            expands);
    }

    static bool HasDuplicate(IEnumerable<double> values)
    {
        var ordered = values.OrderBy(value => value).ToList();
        for (var i = 1; i < ordered.Count; i++)
            if (Math.Abs(ordered[i] - ordered[i - 1]) <= DuplicateToleranceMm)
                return true;
        return false;
    }

    static bool IsFinite(double value) => !double.IsNaN(value) && !double.IsInfinity(value);

    static TeklaGridEnvelopeResult Unsupported(string code, string message) =>
        new(false, code, message, 0, 0, 0, 0, false);
}
