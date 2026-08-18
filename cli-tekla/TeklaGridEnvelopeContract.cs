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
    public GridChildRealizationContract(
        string id,
        string kind,
        string realizedBy,
        string family,
        string nativeLabel,
        double offsetMm)
    {
        Id = id;
        Kind = kind;
        RealizedBy = realizedBy;
        Family = family;
        NativeLabel = nativeLabel;
        OffsetMm = offsetMm;
    }

    public string Id { get; }
    public string Kind { get; }
    public string RealizedBy { get; }
    public string Family { get; }
    public string NativeLabel { get; }
    public double OffsetMm { get; }
}

public sealed class GridLabelTokenMapping
{
    public GridLabelTokenMapping(
        string id,
        string family,
        string authoredLabel,
        string nativeLabel)
    {
        Id = id;
        Family = family;
        AuthoredLabel = authoredLabel;
        NativeLabel = nativeLabel;
    }

    public string Id { get; }
    public string Family { get; }
    public string AuthoredLabel { get; }
    public string NativeLabel { get; }
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
        IReadOnlyList<GridChildRealizationContract> children,
        IReadOnlyList<GridLabelTokenMapping> labelTokenMappings)
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
        LabelTokenMappings = labelTokenMappings;
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
    public IReadOnlyList<GridLabelTokenMapping> LabelTokenMappings { get; }

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

    public Dictionary<string, object>? CreateLabelTokenWarning()
    {
        if (LabelTokenMappings.Count == 0) return null;
        return new Dictionary<string, object>(StringComparer.Ordinal)
        {
            ["id"] = GridId,
            ["kind"] = "structural-grid",
            ["status"] = "warning",
            ["code"] = "tekla-grid-label-tokenized",
            ["message"] = "Tekla's parent Grid label grammar cannot carry spaces, so multi-word axis or elevation labels were mapped to deterministic native tokens.",
            ["mappings"] = LabelTokenMappings.Select(mapping =>
                (object)new Dictionary<string, object>(StringComparer.Ordinal)
                {
                    ["id"] = mapping.Id,
                    ["family"] = mapping.Family,
                    ["authoredLabel"] = mapping.AuthoredLabel,
                    ["nativeLabel"] = mapping.NativeLabel,
                }).ToArray(),
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
    const int WorkaroundLabelMaxUtf16CodeUnits = 40;

    sealed class LabelAllocation
    {
        internal LabelAllocation(
            IReadOnlyDictionary<string, string> nativeLabels,
            IReadOnlyList<GridLabelTokenMapping> mappings)
        {
            NativeLabels = nativeLabels;
            Mappings = mappings;
        }

        internal IReadOnlyDictionary<string, string> NativeLabels { get; }
        internal IReadOnlyList<GridLabelTokenMapping> Mappings { get; }
    }

    public TeklaGridEnvelopeResult Evaluate(
        IReadOnlyList<GridAxisExtentContract> axes,
        IReadOnlyList<GridLevelContract> levels,
        double originZMm,
        string resolvedHostVersion)
    {
        if (axes is null) throw new ArgumentNullException(nameof(axes));
        if (levels is null) throw new ArgumentNullException(nameof(levels));

        var xs = axes.Where(axis => axis.Direction == "x").OrderBy(axis => axis.OffsetMm).ToList();
        var ys = axes.Where(axis => axis.Direction == "y").OrderBy(axis => axis.OffsetMm).ToList();

        if (xs.Count == 0 || ys.Count == 0)
            return Unsupported(
                "tekla-grid-single-family-unsupported",
                "Tekla native Grid requires both X and Y axis families.");

        if (!TryAllocateLabels(axes, levels, resolvedHostVersion, out _))
            return Unsupported(
                "tekla-grid-label-token-unsupported",
                "Tekla native Grid requires each label to be one token; resolved Tekla 2026 additionally supports exactly two control-free tokens separated by one ASCII space through a deterministic native-token mapping of at most 40 UTF-16 code units.");

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
        double originZMm,
        string resolvedHostVersion)
    {
        if (string.IsNullOrWhiteSpace(gridId)) throw new ArgumentException("Grid id is required.", nameof(gridId));
        var envelope = Evaluate(axes, levels, originZMm, resolvedHostVersion);
        if (!envelope.IsSupported)
            throw new InvalidOperationException($"{envelope.Code}: {envelope.Message}");

        if (!TryAllocateLabels(axes, levels, resolvedHostVersion, out var allocation))
            throw new InvalidOperationException("Grid label allocation changed after successful evaluation.");

        var xs = axes.Where(axis => axis.Direction == "x").OrderBy(axis => axis.OffsetMm).ToList();
        var ys = axes.Where(axis => axis.Direction == "y").OrderBy(axis => axis.OffsetMm).ToList();
        var zs = levels.OrderBy(level => level.ElevationMm).ToList();
        var children = xs
            .Select(axis => new GridChildRealizationContract(
                axis.Id, "grid-axis", gridId, "x", allocation.NativeLabels[axis.Id], axis.OffsetMm))
            .Concat(ys.Select(axis => new GridChildRealizationContract(
                axis.Id, "grid-axis", gridId, "y", allocation.NativeLabels[axis.Id], axis.OffsetMm)))
            .Concat(zs.Select(level => new GridChildRealizationContract(
                level.Id, "grid-level", gridId, "z", allocation.NativeLabels[level.Id], level.ElevationMm - originZMm)))
            .ToArray();

        return new TeklaGridMaterializationPlan(
            gridId,
            envelope,
            Spacing(xs.Select(axis => axis.OffsetMm)),
            Spacing(ys.Select(axis => axis.OffsetMm)),
            Spacing(zs.Select(level => level.ElevationMm - originZMm)),
            string.Join(" ", xs.Select(axis => allocation.NativeLabels[axis.Id])),
            string.Join(" ", ys.Select(axis => allocation.NativeLabels[axis.Id])),
            string.Join(" ", zs.Select(level => allocation.NativeLabels[level.Id])),
            xs[0].OffsetMm,
            xs[xs.Count - 1].OffsetMm,
            ys[0].OffsetMm,
            ys[ys.Count - 1].OffsetMm,
            ys[0].OffsetMm - envelope.XFamilyStartMm,
            envelope.XFamilyEndMm - ys[ys.Count - 1].OffsetMm,
            xs[0].OffsetMm - envelope.YFamilyStartMm,
            envelope.YFamilyEndMm - xs[xs.Count - 1].OffsetMm,
            children,
            allocation.Mappings);
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

    static bool TryAllocateLabels(
        IReadOnlyList<GridAxisExtentContract> axes,
        IReadOnlyList<GridLevelContract> levels,
        string resolvedHostVersion,
        out LabelAllocation allocation)
    {
        var orderedFamilies = new[]
        {
            axes.Where(axis => axis.Direction == "x").OrderBy(axis => axis.OffsetMm)
                .Select(axis => (axis.Id, Family: "x", axis.Label)).ToArray(),
            axes.Where(axis => axis.Direction == "y").OrderBy(axis => axis.OffsetMm)
                .Select(axis => (axis.Id, Family: "y", axis.Label)).ToArray(),
            levels.OrderBy(level => level.ElevationMm)
                .Select(level => (level.Id, Family: "z", level.Label)).ToArray(),
        };
        var nativeLabels = new Dictionary<string, string>(StringComparer.Ordinal);
        var mappings = new List<GridLabelTokenMapping>();
        var supportsTwoTokenMapping = resolvedHostVersion?.StartsWith("2026.", StringComparison.Ordinal) == true;

        foreach (var family in orderedFamilies)
        {
            var reserved = new HashSet<string>(
                family.Where(item => IsExactLabelToken(item.Label)).Select(item => item.Label),
                StringComparer.Ordinal);

            for (var position = 0; position < family.Length; position++)
            {
                var item = family[position];
                if (IsExactLabelToken(item.Label))
                {
                    nativeLabels[item.Id] = item.Label;
                    continue;
                }
                if (!supportsTwoTokenMapping || !IsSupportedTwoTokenLabel(item.Label))
                {
                    allocation = null!;
                    return false;
                }

                var candidate = item.Label.Replace(' ', '_');
                if (candidate.Length > WorkaroundLabelMaxUtf16CodeUnits)
                {
                    allocation = null!;
                    return false;
                }
                for (var attempt = 0; !reserved.Add(candidate); attempt++)
                {
                    var suffix = $"~{position + 1}-{attempt + 1}";
                    candidate = item.Label.Replace(' ', '_') + suffix;
                    if (candidate.Length > WorkaroundLabelMaxUtf16CodeUnits)
                    {
                        allocation = null!;
                        return false;
                    }
                }
                nativeLabels[item.Id] = candidate;
                mappings.Add(new GridLabelTokenMapping(item.Id, item.Family, item.Label, candidate));
            }
        }

        allocation = new LabelAllocation(nativeLabels, mappings);
        return true;
    }

    static bool IsExactLabelToken(string label) =>
        !string.IsNullOrEmpty(label)
        && !label.Any(char.IsWhiteSpace)
        && !label.Any(char.IsControl);

    static bool IsSupportedTwoTokenLabel(string label)
    {
        if (string.IsNullOrEmpty(label)
            || label.Length > WorkaroundLabelMaxUtf16CodeUnits
            || label.Any(char.IsControl)
            || label.Count(character => character == ' ') != 1
            || label.Any(character => character != ' ' && char.IsWhiteSpace(character)))
            return false;
        var separator = label.IndexOf(' ');
        return separator > 0 && separator < label.Length - 1;
    }

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
