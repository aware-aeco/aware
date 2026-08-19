using System;
using System.Globalization;

namespace AwareTekla;

/// <summary>
/// One native Tekla part of a canonical `double-angle` pair.
/// </summary>
public sealed class TeklaDoubleAngleLeg
{
    internal TeklaDoubleAngleLeg(string suffix, bool reversedAxis, double canonicalRollDegrees, double offsetMm)
    {
        Suffix = suffix;
        ReversedAxis = reversedAxis;
        CanonicalRollDegrees = canonicalRollDegrees;
        OffsetMm = offsetMm;
    }

    /// <summary>Stable per-leg discriminator; `a` is the +rolled-X leg.</summary>
    public string Suffix { get; }

    /// <summary>
    /// True when the beam must be built on the reversed `to`→`from` axis. Reversing
    /// is the only way to mirror a section in Tekla: an L is chiral whenever its legs
    /// are unequal, so no roll about the member axis can turn one leg of a
    /// back-to-back pair into the other.
    /// </summary>
    public bool ReversedAxis { get; }

    /// <summary>
    /// Canonical roll for this leg's own axis, to be handed to
    /// <see cref="TeklaMemberRollContract.CreatePlan"/> with that same axis order.
    /// </summary>
    public double CanonicalRollDegrees { get; }

    /// <summary>Signed offset from the member axis along the member's rolled X.</summary>
    public double OffsetMm { get; }
}

/// <summary>
/// Canonical `double-angle` plan: exactly two native single-angle parts.
/// </summary>
public sealed class TeklaDoubleAnglePlan
{
    internal TeklaDoubleAnglePlan(string legProfile, double envelopeWidthMm, double envelopeDepthMm, TeklaDoubleAngleLeg[] legs, double[][] sectionOutline)
    {
        LegProfile = legProfile;
        EnvelopeWidthMm = envelopeWidthMm;
        EnvelopeDepthMm = envelopeDepthMm;
        Legs = legs;
        SectionOutline = sectionOutline;
    }

    /// <summary>Parametric Tekla profile of one leg, e.g. `L100*75*8`.</summary>
    public string LegProfile { get; }

    public double EnvelopeWidthMm { get; }
    public double EnvelopeDepthMm { get; }

    /// <summary>Exactly two legs, `a` (+rolled X) then `b` (-rolled X).</summary>
    public TeklaDoubleAngleLeg[] Legs { get; }

    /// <summary>
    /// The section vertices the finished pair must occupy together, as `[x,y]` in the
    /// member's rolled section frame — six per leg, the canonical figure every sink
    /// draws. (At `gap == 0` the facing back vertices coincide, so ten of the twelve
    /// are distinct.)
    ///
    /// This exists so the bake can PROVE the pair rather than assume it. The mirror
    /// and roll arithmetic rests on two empirical facts about Tekla, and no unit test
    /// can pin those: a test's model of Tekla is the same model this plan was derived
    /// from, so flipping either fact moves both sides together and the suite still
    /// passes. Reading the inserted solids back and comparing them against this
    /// outline is what turns the assumptions into an invariant, and it catches a
    /// catalog whose parametric `L` seats differently from the probed one.
    /// </summary>
    public double[][] SectionOutline { get; }
}

/// <summary>
/// Host-neutral interpretation of the canonical scene `double-angle` descriptor
/// for the Tekla sink. Tekla has no native 2L profile — double angles exist there
/// only inside components, whose geometry is decided by environment-specific
/// component defaults rather than by the descriptor. So the sink materializes the
/// pair itself, as two plain parametric single angles.
///
/// Two empirical facts drive the plan, both established by live 2026 B-rep probes.
/// A green unit suite is NOT clearance for either: any test's model of what Tekla
/// does is the same model this plan was derived from, so flipping a fact moves both
/// sides together and every test still passes. What actually holds them is the
/// bake-time comparison of the inserted solids against
/// <see cref="TeklaDoubleAnglePlan.SectionOutline"/> — verified by mutation, not by
/// assertion. The tests pin everything downstream of the facts: the offsets, the
/// per-leg rolls, the chirality assignment, the llbb/slbb swap.
///
/// 1. At canonical roll 0, Tekla's parametric `L h*b*t` places its vertical leg on
///    +X of the canonical section frame and its flat leg at -Y. That is the MIRROR
///    of AWARE's canonical `angle`, whose vertical leg is on -X
///    (`viewer-3d` `profileShape`, kind `L`). The mirror is why leg `a` (+X) is the
///    reversed-axis one for LLBB and the forward one for SLBB — pairing the wrong
///    chirality with a side turns the backs outward and faces the outstanding legs
///    at each other.
/// 2. Building a beam on the reversed `to`→`from` axis mirrors its section — but
///    across which axis depends on the canonical zero frame's branch. On the
///    projected branch zero X flips and zero Y survives, so the section mirrors
///    across Y; on the vertical seed branch zero X survives and zero Y flips, so it
///    mirrors across X instead. The two differ by a half turn, which is why a
///    reversed leg of a vertical member carries an extra 180 degrees: without it the
///    pair comes out point-reflected — both backs still `gap` apart, but one
///    outstanding leg up and the other down. Live-verified on 2026: horizontal,
///    rolled and sloped members passed while both columns failed until this term
///    existed.
///
/// The canonical figure both orientations must reproduce is the two-shape path in
/// `viewer-3d`: a leg of thickness `t` spanning the full envelope depth adjacent to
/// the gap, and an outstanding leg of thickness `t` along the envelope bottom.
/// `llbb` puts the long (`d`) legs back-to-back, `slbb` the short (`b`) legs; in
/// both the pair separates along the section's X axis, never its Y.
/// </summary>
public static class TeklaDoubleAngleContract
{
    /// <summary>Extra roll that lays the long leg flat for a short-legs-back-to-back pair.</summary>
    public const double ShortLegsBackToBackRollDegrees = 90d;

    /// <param name="verticalZeroFrame">
    /// <see cref="TeklaMemberRollContract.UsesVerticalSeedFrame"/> for this member's axis.
    /// </param>
    public static TeklaDoubleAnglePlan CreatePlan(double d, double b, double t, double gap, string orientation, double rollDegrees, bool verticalZeroFrame)
    {
        RequirePositive(d, nameof(d));
        RequirePositive(b, nameof(b));
        RequirePositive(t, nameof(t));
        if (!IsFinite(gap) || gap < 0d)
            throw new ArgumentException("double-angle gap must be a finite non-negative number", nameof(gap));
        if (t >= Math.Min(d, b))
            throw new ArgumentException("double-angle thickness must leave a non-degenerate leg (t < min(d,b))", nameof(t));
        if (!IsFinite(rollDegrees))
            throw new ArgumentException("member rot must be a finite JSON number", nameof(rollDegrees));

        var roll = TeklaMemberRollContract.NormalizeDegrees(rollDegrees);
        var legProfile = "L" + Millimetres(d) + "*" + Millimetres(b) + "*" + Millimetres(t);

        // The leg dimension that runs along the section X axis: the outstanding leg,
        // because the back-to-back leg is the one spanning the envelope depth.
        double acrossMm = orientation switch
        {
            "llbb" => b,
            "slbb" => d,
            _ => throw new ArgumentException("double-angle orientation must be exactly llbb or slbb", nameof(orientation)),
        };
        var envelopeWidth = 2d * acrossMm + gap;
        var envelopeDepth = orientation == "llbb" ? d : b;
        var offset = gap / 2d + acrossMm / 2d;

        // The half turn that turns a mirror-across-X back into a mirror-across-Y,
        // on the one branch where reversing the axis mirrors the other way (fact 2).
        var reversedCorrection = verticalZeroFrame ? 180d : 0d;

        // `a` is always the +X leg. Which axis order carries it follows from fact 1.
        var legs = orientation == "llbb"
            ? new[]
            {
                Leg("a", true, -roll + reversedCorrection, offset),
                Leg("b", false, roll, -offset),
            }
            : new[]
            {
                Leg("a", false, roll - ShortLegsBackToBackRollDegrees, offset),
                Leg("b", true, -roll - ShortLegsBackToBackRollDegrees + reversedCorrection, -offset),
            };

        return new TeklaDoubleAnglePlan(legProfile, envelopeWidth, envelopeDepth, legs, SectionOutline(t, gap, acrossMm, envelopeDepth));
    }

    /// <summary>
    /// The canonical pair outline: a leg of thickness `t` spanning the full envelope
    /// depth on each side of the gap, and an outstanding leg of thickness `t` along
    /// the envelope bottom reaching `outstanding` outwards. Mirror-symmetric about
    /// the section Y axis. Same figure as `double_angle_outlines` in
    /// `cli/src/render/ifc.rs`, which is the sink-independent definition.
    /// </summary>
    static double[][] SectionOutline(double t, double gap, double outstanding, double depth)
    {
        double halfDepth = depth / 2d, halfGap = gap / 2d;
        var left = new[]
        {
            new[] { -halfGap, -halfDepth },
            new[] { -halfGap, halfDepth },
            new[] { -halfGap - t, halfDepth },
            new[] { -halfGap - t, -halfDepth + t },
            new[] { -halfGap - outstanding, -halfDepth + t },
            new[] { -halfGap - outstanding, -halfDepth },
        };
        var outline = new double[left.Length * 2][];
        for (var index = 0; index < left.Length; index++)
        {
            outline[index] = left[index];
            outline[left.Length + index] = new[] { -left[index][0], left[index][1] };
        }
        return outline;
    }

    static TeklaDoubleAngleLeg Leg(string suffix, bool reversedAxis, double rollDegrees, double offsetMm) =>
        new(suffix, reversedAxis, TeklaMemberRollContract.NormalizeDegrees(rollDegrees), offsetMm);

    static string Millimetres(double value) => value.ToString("0.############", CultureInfo.InvariantCulture);

    static void RequirePositive(double value, string name)
    {
        if (!IsFinite(value) || value <= 0d)
            throw new ArgumentException("double-angle " + name + " must be a finite positive number", name);
    }

    static bool IsFinite(double value) => !double.IsNaN(value) && !double.IsInfinity(value);
}
