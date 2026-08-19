using System;
using Xunit;

namespace AwareTekla.Tests;

public sealed class TeklaMemberRollContractTests
{
    readonly TeklaMemberRollContract _contract = new();

    [Theory]
    [InlineData(-540, -180)]
    [InlineData(-360, 0)]
    [InlineData(-180, -180)]
    [InlineData(-0.0, 0)]
    [InlineData(0, 0)]
    [InlineData(180, -180)]
    [InlineData(360, 0)]
    [InlineData(540, -180)]
    public void NormalizesToCanonicalHalfOpenRange(double input, double expected)
    {
        var actual = TeklaMemberRollContract.NormalizeDegrees(input);
        Assert.Equal(expected, actual, 10);
        if (actual == 0d)
            Assert.True(BitConverter.DoubleToInt64Bits(actual) >= 0);
    }

    [Theory]
    [InlineData(0, 0, 0, 0, 1, 82.7, 82.7)]
    [InlineData(0, 0, 0, 0, -1, 82.7, -97.3)]
    [InlineData(0, 0, 1, 0, 0, 82.7, 172.7)]
    [InlineData(0, 0, 1, 2, 3, 82.7, 172.7)]
    public void MapsCanonicalRollToMeasuredTeklaFrontOffset(
        double fx, double fy, double tx, double ty, double tz, double roll, double expectedOffset)
    {
        var plan = _contract.CreatePlan(new[] { fx, fy, 0d }, new[] { tx, ty, tz }, roll);
        Assert.Equal(expectedOffset, plan.NativeFrontOffsetDegrees, 8);
        AssertFrame(plan);
    }

    [Fact]
    public void AxisReversalPreservesTheDirectedRightHandRule()
    {
        var up = _contract.CreatePlan(new[] { 0d, 0d, 0d }, new[] { 0d, 0d, 1000d }, 65.2);
        var down = _contract.CreatePlan(new[] { 0d, 0d, 1000d }, new[] { 0d, 0d, 0d }, 65.2);

        Assert.Equal(65.2, up.NativeFrontOffsetDegrees, 8);
        Assert.Equal(-114.8, down.NativeFrontOffsetDegrees, 8);
        AssertFrame(up);
        AssertFrame(down);
    }

    [Fact]
    public void InclusiveNearVerticalThresholdProducesStableProjectedFrames()
    {
        foreach (var x in new[] { Math.Sqrt(1e-6) * 0.999, Math.Sqrt(1e-6), Math.Sqrt(1e-6) * 1.001 })
        {
            var z = Math.Sqrt(1d - x * x);
            var plan = _contract.CreatePlan(new[] { 0d, 0d, 0d }, new[] { x, 0d, z }, -23.5);
            AssertFrame(plan);
        }
    }

    [Fact]
    public void BranchesFromTheRawAxisSoNormalizationCannotMoveTheSeam()
    {
        // This axis sits exactly on the inclusive threshold, and it is the one input that
        // separates the two readings of the branch test. Taken from the RAW delta as
        // `|q|² <= eps*|d|²` it seeds, matching AWARE's canonical `scene_roll::member_frame`.
        // Taken from the NORMALIZED axis it projects instead — because `Normalize` here
        // multiplies by the reciprocal length while Rust's `normalized3` divides by it, and
        // that one ulp straddles the threshold — leaving the same member with a zero frame
        // 57° from the substrate's. Unlike the Rust side, where both readings agree on every
        // input found, this assertion really does fail if the branch is moved back onto the
        // normalized axis. See issue #432 and `viewer-3d/skills/scene-schema.md`.
        var plan = _contract.CreatePlan(
            new[] { 0d, 0d, 0d },
            new[] { 26.55075466049515, -17.294594180470543, 31686.662128779197 },
            0d);

        // The seeded rule takes zero X from scene +X projected off the axis, so it stays
        // essentially +X; the projected-up rule would put it near [0.198, -0.980, 0].
        Assert.Equal(0.99999964894885041, plan.ZeroX[0], 12);
        Assert.Equal(4.5733451360524881e-07, plan.ZeroX[1], 12);
        Assert.Equal(-0.00083791525035048931, plan.ZeroX[2], 12);
        AssertFrame(plan);
    }

    [Fact]
    public void BranchesByCrossMultiplicationAndNeverByAQuotient()
    {
        // The other rearrangement that is not equivalent in double precision, and the one
        // an implementer is most likely to write after reading "ratio against |d|²" as a
        // division. Here `|q|² = 1.7497609055789547` exceeds `eps*|d|² = 1.7497609055789545`,
        // so the cross-multiplied comparison the contract specifies projects this member,
        // while `|q|²/|d|²` rounds to exactly 1e-6 and would seed it — 64.9° apart.
        var plan = _contract.CreatePlan(
            new[] { 0d, 0d, 0d },
            new[] { 1.1976826775898164, -0.5615310404423273, 1322.784621855746 },
            0d);

        // Projected-up leaves the threshold at exactly (-sin phi, cos phi, 0); seeding
        // would have put zero X within a whisker of +X instead.
        Assert.Equal(0.424506567735086, plan.ZeroX[0], 12);
        Assert.Equal(0.905424858257038, plan.ZeroX[1], 12);
        Assert.Equal(0d, plan.ZeroX[2], 12);
        AssertFrame(plan);
    }

    [Theory]
    [InlineData("FRONT", 10, 10)]
    [InlineData("TOP", -7.3, 82.7)]
    [InlineData("BACK", 0, -180)]
    [InlineData("BELOW", 7.3, -82.7)]
    public void ReconstructsNormalizedNativeReadback(string rotation, double offset, double expected)
    {
        Assert.Equal(expected, TeklaMemberRollContract.EffectiveNativeDegrees(rotation, offset), 8);
    }

    [Fact]
    public void RejectsNonFiniteAnglesAndDegenerateAxes()
    {
        Assert.Throws<ArgumentException>(() => _contract.CreatePlan(new[] { 0d, 0d, 0d }, new[] { 0d, 0d, 1d }, double.NaN));
        Assert.Throws<ArgumentException>(() => _contract.CreatePlan(new[] { 0d, 0d, 0d }, new[] { 0d, 0d, 0d }, 0));
    }

    static void AssertFrame(TeklaMemberRollPlan plan)
    {
        Assert.InRange(Math.Abs(Dot(plan.Axis, plan.RolledX)), 0, 1e-10);
        Assert.InRange(Math.Abs(Dot(plan.Axis, plan.RolledY)), 0, 1e-10);
        Assert.InRange(Math.Abs(Dot(plan.RolledX, plan.RolledY)), 0, 1e-10);
        Assert.Equal(1d, Length(plan.RolledX), 10);
        Assert.Equal(1d, Length(plan.RolledY), 10);
        var handed = Cross(plan.Axis, plan.RolledX);
        Assert.True(Dot(handed, plan.RolledY) >= Math.Cos(TeklaMemberRollContract.AngularToleranceDegrees * Math.PI / 180d));
    }

    static double Dot(double[] a, double[] b) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    static double Length(double[] a) => Math.Sqrt(Dot(a, a));
    static double[] Cross(double[] a, double[] b) => new[]
    {
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    };
}
