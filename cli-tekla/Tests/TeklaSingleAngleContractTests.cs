using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using Xunit;

namespace AwareTekla.Tests;

/// <summary>
/// Pins the plain-angle plan to the canonical section geometry by rebuilding where
/// Tekla actually seats an `L` and comparing the result, vertex for vertex, with
/// the outline AWARE's own consumers draw (`profileShape` kind `L` in
/// `cli/src/render/viewer_3d.rs`). A plan that drops the mirror, mirrors across the
/// wrong axis, or forgets the vertical branch's half turn fails here rather than in
/// a screenshot.
/// </summary>
public sealed class TeklaSingleAngleContractTests
{
    const double Tolerance = 1e-9;

    [Theory]
    [InlineData(150, 100, 10, 0)]
    [InlineData(150, 100, 10, 30)]
    [InlineData(150, 100, 10, -90)]
    [InlineData(150, 100, 10, 180)]
    [InlineData(150, 100, 10, 12.345)]
    [InlineData(150, 100, 10, -137.5)]
    [InlineData(100, 75, 8, 47)]
    [InlineData(100, 100, 8, 0)]
    [InlineData(100, 100, 8, 30)]
    [InlineData(88.9, 63.5, 6.35, -12)]
    public void ThePlannedPartReproducesTheCanonicalAngle(double d, double b, double t, double roll)
    {
        // Both zero-frame branches: reversing a column's axis mirrors across the
        // other axis, so a plan that ignores the branch point-reflects the member.
        foreach (var verticalZeroFrame in new[] { false, true })
        {
            var plan = TeklaSingleAngleContract.CreatePlan(d, b, t, roll, verticalZeroFrame);
            AssertSameOutline(CanonicalAngle(d, b, t), MemberFrameOutline(plan, d, b, t, roll, verticalZeroFrame));
        }
    }

    [Fact]
    public void WithoutTheMirrorTheHeelLandsOnTheWrongSide()
    {
        // The bug itself, as a test: building the authored profile forward at the
        // canonical roll is what the sink used to do, and it is exactly the figure
        // viewer-3d, IFC and Rhino do NOT draw. Without this, dropping the mirror
        // from CreatePlan would leave every assertion above still green if the
        // simulation were ever rewritten to match it.
        foreach (var roll in new[] { 0d, 30d, -90d })
        {
            var unmirrored = ForwardBuildOutline(150, 100, 10, roll);
            Assert.False(
                SameOutline(CanonicalAngle(150, 100, 10), unmirrored),
                "an unmirrored Tekla L must not match the canonical angle");
            Assert.False(
                TeklaSingleAngleContract.HeelIsOnTheCanonicalSide(AsPairs(unmirrored), 0.1),
                "the handedness check must reject what the sink used to bake");
        }
    }

    [Fact]
    public void AnEqualLegAngleIsAlsoMisSeatedWithoutTheMirror()
    {
        // An equal L is not chiral, so the mirror is recoverable by roll — but it is
        // still not free. `My` of an equal angle is that angle turned a quarter turn,
        // so the unmirrored bake comes out 90 degrees off, heel pointing elsewhere.
        var unmirrored = ForwardBuildOutline(100, 100, 8, 0);
        Assert.False(SameOutline(CanonicalAngle(100, 100, 8), unmirrored));
        Assert.False(TeklaSingleAngleContract.HeelIsOnTheCanonicalSide(AsPairs(unmirrored), 0.1));

        var plan = TeklaSingleAngleContract.CreatePlan(100, 100, 8, 0, false);
        AssertSameOutline(CanonicalAngle(100, 100, 8), MemberFrameOutline(plan, 100, 100, 8, 0, false));
    }

    [Fact]
    public void AVerticalMemberCarriesTheHalfTurnAProjectedOneDoesNot()
    {
        foreach (var roll in new[] { 0d, 20d, -115d })
        {
            var projected = TeklaSingleAngleContract.CreatePlan(150, 100, 10, roll, false);
            var vertical = TeklaSingleAngleContract.CreatePlan(150, 100, 10, roll, true);
            var expected = TeklaMemberRollContract.NormalizeDegrees(projected.CanonicalRollDegrees + TeklaSingleAngleContract.VerticalFrameCorrectionDegrees);
            Assert.Equal(expected, vertical.CanonicalRollDegrees, 9);
        }
    }

    [Fact]
    public void DroppingTheVerticalHalfTurnPointReflectsTheColumn()
    {
        // Proves the correction is load-bearing rather than decorative: feed the
        // vertical branch the projected branch's roll and the member comes out
        // point-reflected, which is the failure #438 saw on live columns.
        var projected = TeklaSingleAngleContract.CreatePlan(150, 100, 10, 20, false);
        var withoutCorrection = CanonicalAngle(150, 100, 10)
            .Select(MirrorAcrossY)
            .Select(p => Rotate(p, projected.CanonicalRollDegrees))
            .Select(MirrorAcrossX)
            .Select(p => Rotate(p, -20))
            .ToArray();
        Assert.False(SameOutline(CanonicalAngle(150, 100, 10), withoutCorrection));
    }

    [Fact]
    public void TheMirrorIsUnconditionalBecauseTeklasParametricLIsAlwaysTheOtherHand()
    {
        foreach (var verticalZeroFrame in new[] { false, true })
            Assert.True(TeklaSingleAngleContract.CreatePlan(150, 100, 10, 0, verticalZeroFrame).ReversedAxis);
    }

    [Theory]
    [InlineData(150, 100, 10)]
    [InlineData(100, 100, 8)]
    [InlineData(88.9, 63.5, 6.35)]
    public void SectionOutlineIsTheCanonicalAngleTheSinkMustReproduce(double d, double b, double t)
    {
        // The bake compares inserted solids against this, so it has to be the same
        // figure `profileShape` defines — checked against the independent
        // transcription rather than against the production code that built it.
        var plan = TeklaSingleAngleContract.CreatePlan(d, b, t, 0, false);
        var actual = plan.SectionOutline.Select(p => (X: p[0], Y: p[1])).ToArray();
        Assert.Equal(6, actual.Length);
        AssertSameOutline(CanonicalAngle(d, b, t), actual);
    }

    [Theory]
    [InlineData(150, 100, 10, "L150*100*10")]
    [InlineData(100, 100, 8, "L100*100*8")]
    [InlineData(88.9, 63.5, 6.35, "L88.9*63.5*6.35")]
    public void ParametricProfileIsTheSharpCornerAngleTheDescriptorImplies(double d, double b, double t, string expected)
    {
        Assert.Equal(expected, TeklaSingleAngleContract.CreatePlan(d, b, t, 0, false).ParametricProfile);
    }

    [Fact]
    public void ParametricProfileStaysInvariantUnderACommaDecimalCulture()
    {
        // A dropped InvariantCulture yields `L88,9*63,5*6,35`, which would silently
        // stop matching the resolved profile and so skip the outline comparison.
        var original = CultureInfo.CurrentCulture;
        try
        {
            CultureInfo.CurrentCulture = new CultureInfo("pl-PL");
            Assert.Equal("L88.9*63.5*6.35", TeklaSingleAngleContract.CreatePlan(88.9, 63.5, 6.35, 0, false).ParametricProfile);
        }
        finally
        {
            CultureInfo.CurrentCulture = original;
        }
    }

    [Theory]
    [InlineData(150, 100, 100)]
    [InlineData(150, 100, 150)]
    [InlineData(0, 100, 10)]
    [InlineData(150, 0, 10)]
    [InlineData(150, 100, 0)]
    [InlineData(double.NaN, 100, 10)]
    [InlineData(150, double.PositiveInfinity, 10)]
    public void DegenerateDimensionsAreRejected(double d, double b, double t)
    {
        Assert.Throws<ArgumentException>(() => TeklaSingleAngleContract.CreatePlan(d, b, t, 0, false));
    }

    [Fact]
    public void NonFiniteRollIsRejected()
    {
        Assert.Throws<ArgumentException>(() => TeklaSingleAngleContract.CreatePlan(150, 100, 10, double.NaN, false));
    }

    [Fact]
    public void HandednessSurvivesTheFilletsACatalogAngleCarries()
    {
        // The reason the bake cannot lean on the outline comparison alone: a catalog
        // angle is the canonical figure with its inner corner rounded and its leg
        // tips tapered, so most of its vertices move. The heel test must still read.
        var filleted = FilletedAngle(150, 100, 10);
        Assert.False(SameOutline(CanonicalAngle(150, 100, 10), filleted));
        Assert.True(TeklaSingleAngleContract.HeelIsOnTheCanonicalSide(AsPairs(filleted), 0.1));
        Assert.False(TeklaSingleAngleContract.HeelIsOnTheCanonicalSide(
            AsPairs(filleted.Select(MirrorAcrossY).ToArray()), 0.1));
    }

    [Fact]
    public void HandednessToleratesTheModelsOwnVertexNoise()
    {
        // The two leg tips are `b` apart, so a tenth of a millimetre of read-back
        // noise must not decide the answer either way.
        var noisy = CanonicalAngle(150, 100, 10)
            .Select((p, index) => (X: p.X + (index % 2 == 0 ? 0.05 : -0.05), Y: p.Y + (index % 3 == 0 ? 0.05 : -0.05)))
            .ToArray();
        Assert.True(TeklaSingleAngleContract.HeelIsOnTheCanonicalSide(AsPairs(noisy), 0.1));
    }

    [Theory]
    [InlineData(150, 100, 10)]
    [InlineData(100, 100, 8)]
    // `t > b/2`: the vertical leg's tip crosses the envelope's mid-plane, so reading
    // handedness off that mid-plane calls a correctly seated part mirrored. The
    // descriptor permits any `t < min(d,b)`, so this is in contract, not a freak.
    [InlineData(100, 100, 60)]
    [InlineData(150, 100, 80)]
    public void HandednessReadsTheTopEdgeRatherThanTheEnvelopeMidPlane(double d, double b, double t)
    {
        var canonical = CanonicalAngle(d, b, t);
        Assert.True(TeklaSingleAngleContract.HeelIsOnTheCanonicalSide(AsPairs(canonical), 0.1));
        Assert.False(TeklaSingleAngleContract.HeelIsOnTheCanonicalSide(
            AsPairs(canonical.Select(MirrorAcrossY)), 0.1));
    }

    [Fact]
    public void HandednessRefusesASectionWithNoHeelToRead()
    {
        // A rectangle's top edge spans the whole envelope, so neither hand is a
        // truthful answer. Returning one would be reported to the bake as a verdict.
        (double X, double Y)[] rectangle = [(-50d, -50d), (50d, -50d), (50d, 50d), (-50d, 50d)];
        var refused = Assert.Throws<ArgumentException>(
            () => TeklaSingleAngleContract.HeelIsOnTheCanonicalSide(AsPairs(rectangle), 0.1));
        Assert.Contains("both sides", refused.Message);

        // And the other way: a top edge that reaches neither side is not an angle either.
        (double X, double Y)[] floating = [(-50d, -50d), (50d, -50d), (10d, 50d), (-10d, 50d)];
        Assert.Contains("neither side", Assert.Throws<ArgumentException>(
            () => TeklaSingleAngleContract.HeelIsOnTheCanonicalSide(AsPairs(floating), 0.1)).Message);
    }

    [Fact]
    public void HandednessRefusesASectionItCannotJudge()
    {
        // A section with no width across the rolled X has no mid-plane, so there is
        // no honest answer — returning either bool would be a coin toss the bake
        // would read as a verdict.
        var degenerate = new[] { new[] { 5d, -10d }, new[] { 5d, 0d }, new[] { 5d, 10d } };
        Assert.Throws<ArgumentException>(() => TeklaSingleAngleContract.HeelIsOnTheCanonicalSide(degenerate, 0.1));

        Assert.Throws<ArgumentException>(() => TeklaSingleAngleContract.HeelIsOnTheCanonicalSide(null!, 0.1));
        Assert.Throws<ArgumentException>(() => TeklaSingleAngleContract.HeelIsOnTheCanonicalSide(
            new[] { new[] { 0d, 0d }, new[] { 1d, 1d } }, 0.1));
        Assert.Throws<ArgumentException>(() => TeklaSingleAngleContract.HeelIsOnTheCanonicalSide(
            AsPairs(CanonicalAngle(150, 100, 10)), -1));
        Assert.Throws<ArgumentException>(() => TeklaSingleAngleContract.HeelIsOnTheCanonicalSide(
            new[] { new[] { 0d, 0d }, new[] { 1d, double.NaN }, new[] { 2d, 2d } }, 0.1));
    }

    [Theory]
    [InlineData(0, 0, 0, 0, 0, 1000, true)]
    [InlineData(0, 0, 0, 0, 0, -1000, true)]
    [InlineData(0, 0, 0, 1000, 0, 0, false)]
    [InlineData(0, 0, 0, 1000, 0, 1000, false)]
    public void VerticalSeedFrameDetectionMatchesTheRollContract(double x0, double y0, double z0, double x1, double y1, double z1, bool expected)
    {
        Assert.Equal(expected, TeklaMemberRollContract.UsesVerticalSeedFrame(new[] { x0, y0, z0 }, new[] { x1, y1, z1 }));
    }

    /// <summary>
    /// AWARE's canonical single `angle`, in its own envelope: the vertical leg
    /// (length `d`) on -X, the flat leg (length `b`) along -Y. Transcribed from
    /// `profileShape` kind `L`, independently of the contract under test.
    /// </summary>
    static (double X, double Y)[] CanonicalAngle(double d, double b, double t)
    {
        double halfWidth = b / 2d, halfDepth = d / 2d;
        return new[]
        {
            (-halfWidth, -halfDepth), (halfWidth, -halfDepth), (halfWidth, -halfDepth + t),
            (-halfWidth + t, -halfDepth + t), (-halfWidth + t, halfDepth), (-halfWidth, halfDepth),
        };
    }

    /// <summary>
    /// The canonical figure with its inner corner rounded and its leg tips tapered —
    /// a stand-in for the catalog geometry the sharp-corner outline cannot describe.
    /// </summary>
    static (double X, double Y)[] FilletedAngle(double d, double b, double t)
    {
        double halfWidth = b / 2d, halfDepth = d / 2d, fillet = t;
        return new[]
        {
            (-halfWidth, -halfDepth), (halfWidth, -halfDepth), (halfWidth, -halfDepth + t / 2d),
            (halfWidth - fillet, -halfDepth + t), (-halfWidth + t + fillet, -halfDepth + t),
            (-halfWidth + t, -halfDepth + t + fillet), (-halfWidth + t, halfDepth - fillet),
            (-halfWidth + t / 2d, halfDepth), (-halfWidth, halfDepth),
        };
    }

    /// <summary>
    /// What the sink used to build: the authored profile on the forward axis at the
    /// canonical roll, i.e. Tekla's own `L` rolled into place with no mirror.
    ///
    /// The two rotations cancel, and that IS the chirality claim rather than a
    /// redundant step: whatever roll the author asks for, an unmirrored `L` lands in
    /// the same wrong place in the member's own frame, so no roll recovers it.
    /// </summary>
    static (double X, double Y)[] ForwardBuildOutline(double d, double b, double t, double theta) =>
        CanonicalAngle(d, b, t)
            .Select(MirrorAcrossY)
            .Select(p => Rotate(p, theta))
            .Select(p => Rotate(p, -theta))
            .ToArray();

    /// <summary>
    /// Rebuilds where Tekla actually puts the part, in the member's rolled section frame.
    ///
    /// A forward beam at canonical roll phi shows Rot(phi) applied to Tekla's own L;
    /// a reversed beam at canonical roll psi shows that mirrored — across Y on the
    /// projected branch, across X on the vertical seed branch. Tekla's L is itself
    /// the mirror of AWARE's canonical angle: the live 2026 B-rep probe on
    /// `L100*75*8` put the vertical leg on +X. Dividing out the member roll `theta`
    /// leaves the member-frame outline the canonical contract specifies.
    /// </summary>
    static (double X, double Y)[] MemberFrameOutline(TeklaSingleAnglePlan plan, double d, double b, double t, double theta, bool verticalZeroFrame)
    {
        IEnumerable<(double X, double Y)> section = CanonicalAngle(d, b, t)
            .Select(MirrorAcrossY)
            .Select(p => Rotate(p, plan.CanonicalRollDegrees));
        if (plan.ReversedAxis)
            section = verticalZeroFrame ? section.Select(MirrorAcrossX) : section.Select(MirrorAcrossY);
        return section.Select(p => Rotate(p, -theta)).ToArray();
    }

    static (double X, double Y) MirrorAcrossY((double X, double Y) p) => (-p.X, p.Y);

    static (double X, double Y) MirrorAcrossX((double X, double Y) p) => (p.X, -p.Y);

    static (double X, double Y) Rotate((double X, double Y) p, double degrees)
    {
        var radians = degrees * Math.PI / 180d;
        double cos = Math.Cos(radians), sin = Math.Sin(radians);
        return (p.X * cos - p.Y * sin, p.X * sin + p.Y * cos);
    }

    static double[][] AsPairs(IEnumerable<(double X, double Y)> section) =>
        section.Select(p => new[] { p.X, p.Y }).ToArray();

    static bool SameOutline(IReadOnlyList<(double X, double Y)> expected, IReadOnlyList<(double X, double Y)> actual) =>
        expected.Count == actual.Count
        && expected.All(want => actual.Any(got => Math.Abs(got.X - want.X) <= Tolerance && Math.Abs(got.Y - want.Y) <= Tolerance));

    static void AssertSameOutline(IReadOnlyList<(double X, double Y)> expected, IReadOnlyList<(double X, double Y)> actual)
    {
        Assert.Equal(expected.Count, actual.Count);
        foreach (var want in expected)
            Assert.True(
                actual.Any(got => Math.Abs(got.X - want.X) <= Tolerance && Math.Abs(got.Y - want.Y) <= Tolerance),
                $"missing vertex ({want.X}, {want.Y}); got {string.Join(" ", actual.Select(p => $"({p.X:0.####},{p.Y:0.####})"))}");
    }
}
