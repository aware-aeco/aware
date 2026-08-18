using System;
using System.Collections.Generic;
using System.Globalization;
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

public sealed class GridChildRealizationContract
{
    public GridChildRealizationContract(string id, string kind, string realizedBy)
    {
        Id = id;
        Kind = kind;
        RealizedBy = realizedBy;
    }

    public string Id { get; }
    public string Kind { get; }
    public string RealizedBy { get; }
}

/// <summary>
/// Host-agnostic description of every Tekla Grid property the bake script writes.
/// Keeping the translation here makes coordinate-family swaps, extension mistakes,
/// and child-realization drift executable in tests without loading Tekla.
/// </summary>
public sealed class TeklaGridMaterializationPlan
{
    internal TeklaGridMaterializationPlan(
        string gridId,
        TeklaGridEnvelopeResult envelope,
        string coordinateX,
        string coordinateY,
        string coordinateZ,
        string labelX,
        string labelY,
        string labelZ,
        double minXOffsetMm,
        double maxXOffsetMm,
        double minYOffsetMm,
        double maxYOffsetMm,
        double extensionLeftX,
        double extensionRightX,
        double extensionLeftY,
        double extensionRightY,
        IReadOnlyList<GridChildRealizationContract> children)
    {
        GridId = gridId;
        Envelope = envelope;
        CoordinateX = coordinateX;
        CoordinateY = coordinateY;
        CoordinateZ = coordinateZ;
        LabelX = labelX;
        LabelY = labelY;
        LabelZ = labelZ;
        MinXOffsetMm = minXOffsetMm;
        MaxXOffsetMm = maxXOffsetMm;
        MinYOffsetMm = minYOffsetMm;
        MaxYOffsetMm = maxYOffsetMm;
        ExtensionLeftX = extensionLeftX;
        ExtensionRightX = extensionRightX;
        ExtensionLeftY = extensionLeftY;
        ExtensionRightY = extensionRightY;
        Children = children;
    }

    public string GridId { get; }
    public TeklaGridEnvelopeResult Envelope { get; }
    public string CoordinateX { get; }
    public string CoordinateY { get; }
    public string CoordinateZ { get; }
    public string LabelX { get; }
    public string LabelY { get; }
    public string LabelZ { get; }
    public double MinXOffsetMm { get; }
    public double MaxXOffsetMm { get; }
    public double MinYOffsetMm { get; }
    public double MaxYOffsetMm { get; }
    public double ExtensionLeftX { get; }
    public double ExtensionRightX { get; }
    public double ExtensionLeftY { get; }
    public double ExtensionRightY { get; }
    public IReadOnlyList<GridChildRealizationContract> Children { get; }

    public Dictionary<string, object>? CreateExpansionWarning()
    {
        if (!Envelope.ExpandsAuthoredExtents) return null;
        return new Dictionary<string, object>(StringComparer.Ordinal)
        {
            ["id"] = GridId,
            ["kind"] = "structural-grid",
            ["status"] = "warning",
            ["code"] = "tekla-grid-axis-extents-expanded",
            ["message"] = "Tekla uses one shared rectangular grid envelope, so one or more native grid lines extend beyond their authored startMm/endMm values.",
            ["xFamilyStartMm"] = Envelope.XFamilyStartMm,
            ["xFamilyEndMm"] = Envelope.XFamilyEndMm,
            ["yFamilyStartMm"] = Envelope.YFamilyStartMm,
            ["yFamilyEndMm"] = Envelope.YFamilyEndMm,
        };
    }
}

/// <summary>
/// Makes lossy warnings transactional: callers can queue them during preflight,
/// but only the post-commit success path can publish them into a receipt.
/// </summary>
public sealed class TeklaGridWarningJournal
{
    readonly List<Dictionary<string, object>> _pending = new();

    public int PendingCount => _pending.Count;

    public void Queue(Dictionary<string, object> warning)
    {
        if (warning is null) throw new ArgumentNullException(nameof(warning));
        _pending.Add(warning);
    }

    public IReadOnlyList<Dictionary<string, object>> PublishAfterCommit()
    {
        var published = _pending.ToArray();
        _pending.Clear();
        return published;
    }

    public void Abort() => _pending.Clear();
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
        IReadOnlyList<GridLevelContract> levels,
        double originZMm)
    {
        if (axes is null) throw new ArgumentNullException(nameof(axes));
        if (levels is null) throw new ArgumentNullException(nameof(levels));

        var xs = axes.Where(axis => axis.Direction == "x").OrderBy(axis => axis.OffsetMm).ToList();
        var ys = axes.Where(axis => axis.Direction == "y").OrderBy(axis => axis.OffsetMm).ToList();

        if (xs.Count == 0 || ys.Count == 0)
            return Unsupported(
                "tekla-grid-single-family-unsupported",
                "Tekla native Grid requires both X and Y axis families.");

        if (axes.Any(axis => !IsLabelToken(axis.Label))
            || levels.Any(level => !IsLabelToken(level.Label)))
            return Unsupported(
                "tekla-grid-label-token-unsupported",
                "Tekla native Grid requires every axis and level label to be one whitespace-free token.");

        if (HasDuplicate(xs.Select(axis => axis.OffsetMm))
            || HasDuplicate(ys.Select(axis => axis.OffsetMm)))
            return Unsupported(
                "tekla-grid-duplicate-axis-offset-unsupported",
                "Tekla native Grid cannot preserve duplicate offsets within one axis family.");

        if (HasDuplicate(levels.Select(level => level.ElevationMm)))
            return Unsupported(
                "tekla-grid-duplicate-elevation-unsupported",
                "Tekla native Grid cannot preserve duplicate elevation levels.");

        var normalizedLevels = levels.Select(level => level.ElevationMm - originZMm).ToList();
        if (HasNonFiniteSpacing(xs.Select(axis => axis.OffsetMm))
            || HasNonFiniteSpacing(ys.Select(axis => axis.OffsetMm))
            || HasNonFiniteSpacing(normalizedLevels))
            return Unsupported(
                "tekla-grid-derived-spacing-unsupported",
                "Tekla native Grid coordinate spacing overflows for the authored axes or levels.");

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

    public TeklaGridMaterializationPlan CreatePlan(
        string gridId,
        IReadOnlyList<GridAxisExtentContract> axes,
        IReadOnlyList<GridLevelContract> levels,
        double originZMm)
    {
        if (string.IsNullOrWhiteSpace(gridId)) throw new ArgumentException("Grid id is required.", nameof(gridId));
        var envelope = Evaluate(axes, levels, originZMm);
        if (!envelope.IsSupported)
            throw new InvalidOperationException($"{envelope.Code}: {envelope.Message}");

        var xs = axes.Where(axis => axis.Direction == "x").OrderBy(axis => axis.OffsetMm).ToList();
        var ys = axes.Where(axis => axis.Direction == "y").OrderBy(axis => axis.OffsetMm).ToList();
        var zs = levels.OrderBy(level => level.ElevationMm).ToList();
        var children = axes
            .Select(axis => new GridChildRealizationContract(axis.Id, "grid-axis", gridId))
            .Concat(levels.Select(level => new GridChildRealizationContract(level.Id, "grid-level", gridId)))
            .ToArray();

        return new TeklaGridMaterializationPlan(
            gridId,
            envelope,
            Spacing(xs.Select(axis => axis.OffsetMm)),
            Spacing(ys.Select(axis => axis.OffsetMm)),
            Spacing(zs.Select(level => level.ElevationMm - originZMm)),
            string.Join(" ", xs.Select(axis => axis.Label)),
            string.Join(" ", ys.Select(axis => axis.Label)),
            string.Join(" ", zs.Select(level => level.Label)),
            xs[0].OffsetMm,
            xs[xs.Count - 1].OffsetMm,
            ys[0].OffsetMm,
            ys[ys.Count - 1].OffsetMm,
            ys[0].OffsetMm - envelope.XFamilyStartMm,
            envelope.XFamilyEndMm - ys[ys.Count - 1].OffsetMm,
            xs[0].OffsetMm - envelope.YFamilyStartMm,
            envelope.YFamilyEndMm - xs[xs.Count - 1].OffsetMm,
            children);
    }

    public IReadOnlyList<Dictionary<string, object>> CreateUnsupportedRows(
        string gridId,
        IReadOnlyList<GridAxisExtentContract> axes,
        IReadOnlyList<GridLevelContract> levels,
        TeklaGridEnvelopeResult reason)
    {
        if (reason.IsSupported) throw new ArgumentException("A supported grid has no unsupported rows.", nameof(reason));
        var rows = new List<Dictionary<string, object>>
        {
            Receipt(gridId, "structural-grid", reason.Code, reason.Message),
        };
        rows.AddRange(axes.Select(axis => Receipt(axis.Id, "grid-axis", "unsupported-parent", reason.Message)));
        rows.AddRange(levels.Select(level => Receipt(level.Id, "grid-level", "unsupported-parent", reason.Message)));
        return rows;
    }

    static bool HasDuplicate(IEnumerable<double> values)
    {
        var ordered = values.OrderBy(value => value).ToList();
        for (var i = 1; i < ordered.Count; i++)
            if (Math.Abs(ordered[i] - ordered[i - 1]) <= DuplicateToleranceMm)
                return true;
        return false;
    }

    static bool HasNonFiniteSpacing(IEnumerable<double> values)
    {
        var ordered = values.OrderBy(value => value).ToList();
        if (ordered.Any(value => !IsFinite(value))) return true;
        for (var i = 1; i < ordered.Count; i++)
            if (!IsFinite(ordered[i] - ordered[i - 1]))
                return true;
        return false;
    }

    static bool IsLabelToken(string label) =>
        !string.IsNullOrEmpty(label) && !label.Any(char.IsWhiteSpace);

    static bool IsFinite(double value) => !double.IsNaN(value) && !double.IsInfinity(value);

    static string Spacing(IEnumerable<double> values)
    {
        var ordered = values.OrderBy(value => value).ToList();
        if (ordered.Count == 0) return "0";
        var tokens = new List<string> { Millimetres(ordered[0]) };
        for (var i = 1; i < ordered.Count; i++)
            tokens.Add(Millimetres(ordered[i] - ordered[i - 1]));
        return string.Join(" ", tokens);
    }

    static string Millimetres(double value) =>
        value.ToString("0.############", CultureInfo.InvariantCulture);

    static Dictionary<string, object> Receipt(
        string id,
        string kind,
        string code,
        string message) =>
        new(StringComparer.Ordinal)
        {
            ["id"] = id,
            ["kind"] = kind,
            ["status"] = "unsupported",
            ["code"] = code,
            ["message"] = message,
        };

    static TeklaGridEnvelopeResult Unsupported(string code, string message) =>
        new(false, code, message, 0, 0, 0, 0, false);
}
