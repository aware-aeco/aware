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
