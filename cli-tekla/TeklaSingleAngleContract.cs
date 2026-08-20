using System;
using System.Collections.Generic;
using System.Globalization;

namespace AwareTekla;

/// <summary>
/// How the Tekla sink must build one plain canonical `angle` member so the
/// finished part matches the descriptor every other sink draws.
/// </summary>
public sealed class TeklaSingleAnglePlan
{
    internal TeklaSingleAnglePlan(bool reversedAxis, double canonicalRollDegrees, string parametricProfile, double envelopeWidthMm, double envelopeDepthMm, double[][] sectionOutline)
    {
        ReversedAxis = reversedAxis;
        CanonicalRollDegrees = canonicalRollDegrees;
        ParametricProfile = parametricProfile;
        EnvelopeWidthMm = envelopeWidthMm;
        EnvelopeDepthMm = envelopeDepthMm;
        SectionOutline = sectionOutline;
    }

    /// <summary>Section bounding box across the member's rolled X — the flat leg, `b`.</summary>
    public double EnvelopeWidthMm { get; }

    /// <summary>Section bounding box across the member's rolled Y — the vertical leg, `d`.</summary>
    public double EnvelopeDepthMm { get; }

    /// <summary>
    /// True when the beam must be built on the reversed `to`→`from` axis.
    ///
    /// Always true today, and stated as data rather than assumed by the caller
    /// because it is a claim about Tekla, not about angles: reversing is the only
    /// way to mirror a section in Tekla, and the mirror is needed only for as long
    /// as Tekla's parametric `L` seats its vertical leg opposite the canonical
    /// descriptor. The bake-time comparison against
    /// <see cref="SectionOutline"/> is what actually holds that — a catalog whose
    /// `L` seats the canonical way fails there rather than baking backwards.
    /// </summary>
    public bool ReversedAxis { get; }

    /// <summary>
    /// Canonical roll for the axis order <see cref="ReversedAxis"/> selects, to be
    /// handed to <see cref="TeklaMemberRollContract.CreatePlan"/> with that same order.
    /// </summary>
    public double CanonicalRollDegrees { get; }

    /// <summary>
    /// The sharp-corner parametric Tekla profile this descriptor implies, e.g.
    /// `L150*100*10`. The full <see cref="SectionOutline"/> comparison is meaningful
    /// only for a member whose resolved profile is exactly this: a catalog angle
    /// carries fillets and a root radius, so its solid never reproduces a
    /// sharp-corner outline even when it is seated correctly.
    /// </summary>
    public string ParametricProfile { get; }

    /// <summary>
    /// The six section vertices the finished part must occupy, as `[x,y]` in the
    /// member's rolled section frame — the canonical figure every sink draws:
    /// the vertical (`d`) leg on -X, the flat (`b`) leg along -Y.
    ///
    /// This exists so the bake can PROVE the mirror rather than assume it. The
    /// arithmetic rests on two empirical facts about Tekla that no unit test can
    /// pin — a test's model of Tekla is the same model this plan was derived from,
    /// so flipping either fact moves both sides together and the suite still
    /// passes. Reading the inserted solid back and comparing it against this
    /// outline is what turns the assumptions into an invariant.
    /// </summary>
    public double[][] SectionOutline { get; }
}

/// <summary>
/// Host-neutral interpretation of the canonical scene `angle` descriptor for the
/// Tekla sink.
///
/// AWARE's canonical `angle` puts the vertical (`d`) leg on -X of the section
/// frame and the flat (`b`) leg along -Y — `profileShape` kind `L` in
/// `cli/src/render/viewer_3d.rs`, and `IFCLSHAPEPROFILEDEF` in
/// `cli/src/render/ifc.rs`. Tekla's parametric `L h*b*t` puts its vertical leg on
/// **+X** instead, which is the mirror. Building the authored profile at the
/// canonical roll therefore lands the heel on the opposite side from viewer-3d,
/// IFC and Rhino, and an L is chiral whenever its legs are unequal, so no roll
/// about the member axis recovers it (see issue #439).
///
/// So the sink mirrors, and the descriptor stays authoritative. That is the
/// substrate's own rule rather than a preference: a sink "must never silently
/// substitute a different section" (`scene-schema.md`), and a chirality-flipped
/// angle is a different section. It is also what
/// <see cref="TeklaDoubleAngleContract"/> already does — every pair it builds
/// mirrors exactly one leg to reach the same canonical figure — so leaving plain
/// angles unmirrored would make one sink disagree with itself about what
/// `shape:"angle"` means.
///
/// The mirror arithmetic is that contract's, unchanged and for the same two
/// live-probed reasons:
///
/// 1. At canonical roll 0 Tekla's parametric `L h*b*t` places its vertical leg on
///    +X and its flat leg at -Y — the mirror of the canonical descriptor.
/// 2. Building a beam on the reversed `to`→`from` axis mirrors its section, but
///    across which axis depends on the canonical zero frame's branch. On the
///    projected branch zero X flips and zero Y survives, so the section mirrors
///    across Y — exactly the mirror wanted. On the vertical seed branch zero X
///    survives and zero Y flips, so it mirrors across X instead; the two differ by
///    a half turn, which is why a vertical member carries an extra 180 degrees.
///    Without that term a column comes out point-reflected: heel on the correct
///    side but pointing the other way along the member.
/// </summary>
public static class TeklaSingleAngleContract
{
    /// <summary>
    /// Half turn that converts the vertical seed branch's mirror-across-X into the
    /// mirror-across-Y this contract needs (fact 2).
    /// </summary>
    public const double VerticalFrameCorrectionDegrees = 180d;

    /// <summary>
    /// How far a read-back section's bounding box may sit from the descriptor's own
    /// before the bake refuses it.
    ///
    /// Looser than the 0.1 mm the outline comparison uses, and deliberately: that one
    /// compares a sharp-corner parametric profile against the figure it was derived
    /// from, while this one compares REAL catalog geometry, which is only nominally
    /// its designation. It stays far tighter than it needs to be to do its job —
    /// catalog angle leg lengths step in whole millimetres and usually by five or
    /// more, so any actual size confusion misses by a wide margin.
    /// </summary>
    public const double EnvelopeToleranceMm = 1d;

    /// <param name="verticalZeroFrame">
    /// <see cref="TeklaMemberRollContract.UsesVerticalSeedFrame"/> for this member's axis.
    /// </param>
    public static TeklaSingleAnglePlan CreatePlan(double d, double b, double t, double rollDegrees, bool verticalZeroFrame)
    {
        RequirePositive(d, nameof(d));
        RequirePositive(b, nameof(b));
        RequirePositive(t, nameof(t));
        // Same non-degeneracy test the canonical consumers apply before they promise
        // a nominal profile: `viewer_3d`'s `t < min(d,b)` and `ifc`'s `t < d && t < b`.
        if (t >= Math.Min(d, b))
            throw new ArgumentException("angle thickness must leave a non-degenerate leg (t < min(d,b))", nameof(t));
        if (!IsFinite(rollDegrees))
            throw new ArgumentException("member rot must be a finite JSON number", nameof(rollDegrees));

        var roll = TeklaMemberRollContract.NormalizeDegrees(rollDegrees);
        var correction = verticalZeroFrame ? VerticalFrameCorrectionDegrees : 0d;
        var canonicalRoll = TeklaMemberRollContract.NormalizeDegrees(-roll + correction);
        var parametricProfile = "L" + Millimetres(d) + "*" + Millimetres(b) + "*" + Millimetres(t);
        return new TeklaSingleAnglePlan(true, canonicalRoll, parametricProfile, b, d, SectionOutline(d, b, t));
    }

    /// <summary>
    /// The section's bounding box as `[width, depth]` across the member's rolled X
    /// and Y, for comparison against <see cref="TeklaSingleAnglePlan.EnvelopeWidthMm"/>
    /// and <see cref="TeklaSingleAnglePlan.EnvelopeDepthMm"/>.
    ///
    /// This is what holds a CATALOG angle to the descriptor. The full outline
    /// comparison cannot: a catalog angle's fillets move most of its vertices, so it
    /// runs only for the sharp-corner parametric `L`. Handedness alone would let a
    /// descriptor of 150×100×10 resolve to a differently sized catalog angle and
    /// commit — the legs would be on the right sides and the wrong lengths. A
    /// bounding box survives fillets, because they are cut into the inner corner and
    /// the leg tips while the outer faces still reach the nominal leg lengths.
    ///
    /// What it does NOT catch, for a catalog profile, is a wrong leg THICKNESS: `t`
    /// does not move the bounding box, and reading it off the solid means measuring
    /// the free end of the vertical leg, which is exactly where a toe radius makes
    /// the measurement unreliable. `insertBeam` already proves the resolved profile
    /// string is the requested one, so what remains is a descriptor disagreeing with
    /// its own profile about thickness — the same exposure every other `xsection`
    /// shape in this sink carries today.
    /// </summary>
    public static double[] MeasureEnvelope(IReadOnlyList<double[]> sectionXy)
    {
        var bounds = Bounds(sectionXy);
        return new[] { bounds[1] - bounds[0], bounds[3] - bounds[2] };
    }

    /// <summary>
    /// Whether a section read back from the model has its heel on the canonical
    /// side — the one question this fix exists to settle, asked in a way that holds
    /// for any angle profile.
    ///
    /// The full <see cref="TeklaSingleAnglePlan.SectionOutline"/> comparison is the
    /// stronger proof but it only applies to a sharp-corner parametric `L`; a
    /// catalog angle's fillets move most of its vertices. What survives both is the
    /// vertical leg: it spans the section's full depth and only `t` of its width, so
    /// the section's TOP edge reaches the -X side of the envelope for a canonical
    /// angle and the +X side for a mirrored one — while the bottom edge, the flat
    /// leg, spans the whole width either way and so says nothing. Fillets are cut
    /// into the inner corner, never the leg tips, so they leave this untouched.
    ///
    /// Which side the top edge REACHES, not which side of the envelope mid-plane it
    /// falls on: those agree only while `t &lt; b/2`, and the descriptor permits any
    /// `t &lt; min(d,b)`. A thick-legged angle's top edge legitimately crosses the
    /// mid-plane, and reading that as a mirror would fail a correctly seated part.
    /// </summary>
    /// <param name="sectionXy">Section vertices as `[x,y]` in the member's rolled frame.</param>
    /// <param name="toleranceMm">
    /// How far below the topmost vertex still counts as the top edge, and how near
    /// an envelope side counts as reaching it. Absorbs the model's own vertex noise.
    /// </param>
    /// <exception cref="ArgumentException">
    /// When the top edge reaches both sides or neither, so there is no heel to read.
    /// A section this contract cannot judge must not be reported as either hand: the
    /// bake would take the answer as a verdict.
    /// </exception>
    public static bool HeelIsOnTheCanonicalSide(IReadOnlyList<double[]> sectionXy, double toleranceMm)
    {
        if (!IsFinite(toleranceMm) || toleranceMm < 0d)
            throw new ArgumentException("tolerance must be a finite non-negative number", nameof(toleranceMm));

        var bounds = Bounds(sectionXy);
        double minX = bounds[0], maxX = bounds[1], maxY = bounds[3];

        if (maxX - minX <= toleranceMm)
            throw new ArgumentException("section has no measurable width across the member's rolled X", nameof(sectionXy));

        double topMinX = double.MaxValue, topMaxX = double.MinValue;
        foreach (var vertex in sectionXy)
        {
            if (vertex[1] < maxY - toleranceMm)
                continue;
            topMinX = Math.Min(topMinX, vertex[0]);
            topMaxX = Math.Max(topMaxX, vertex[0]);
        }

        var reachesMinX = topMinX <= minX + toleranceMm;
        var reachesMaxX = topMaxX >= maxX - toleranceMm;
        if (reachesMinX == reachesMaxX)
            throw new ArgumentException(
                "section's top edge reaches " + (reachesMinX ? "both sides" : "neither side") +
                " of its envelope, so it has no readable angle heel", nameof(sectionXy));
        return reachesMinX;
    }

    /// <summary>
    /// AWARE's canonical single angle, in its own envelope: the vertical leg
    /// (length `d`, thickness `t`) on -X, the flat leg (length `b`, thickness `t`)
    /// along -Y. Same figure as `profileShape` kind `L` in
    /// `cli/src/render/viewer_3d.rs`, which is the sink-independent definition.
    /// </summary>
    static double[][] SectionOutline(double d, double b, double t)
    {
        double halfWidth = b / 2d, halfDepth = d / 2d;
        return new[]
        {
            new[] { -halfWidth, -halfDepth },
            new[] { halfWidth, -halfDepth },
            new[] { halfWidth, -halfDepth + t },
            new[] { -halfWidth + t, -halfDepth + t },
            new[] { -halfWidth + t, halfDepth },
            new[] { -halfWidth, halfDepth },
        };
    }

    /// <summary>Section bounding box as `[minX, maxX, minY, maxY]`.</summary>
    static double[] Bounds(IReadOnlyList<double[]> sectionXy)
    {
        if (sectionXy is null || sectionXy.Count < 3)
            throw new ArgumentException("a section needs at least three vertices", nameof(sectionXy));

        double minX = double.MaxValue, maxX = double.MinValue, minY = double.MaxValue, maxY = double.MinValue;
        foreach (var vertex in sectionXy)
        {
            if (vertex is null || vertex.Length < 2 || !IsFinite(vertex[0]) || !IsFinite(vertex[1]))
                throw new ArgumentException("every section vertex must be a finite [x,y] pair", nameof(sectionXy));
            minX = Math.Min(minX, vertex[0]);
            maxX = Math.Max(maxX, vertex[0]);
            minY = Math.Min(minY, vertex[1]);
            maxY = Math.Max(maxY, vertex[1]);
        }
        return new[] { minX, maxX, minY, maxY };
    }

    static string Millimetres(double value) => value.ToString("0.############", CultureInfo.InvariantCulture);

    static void RequirePositive(double value, string name)
    {
        if (!IsFinite(value) || value <= 0d)
            throw new ArgumentException("angle " + name + " must be a finite positive number", name);
    }

    static bool IsFinite(double value) => !double.IsNaN(value) && !double.IsInfinity(value);
}
