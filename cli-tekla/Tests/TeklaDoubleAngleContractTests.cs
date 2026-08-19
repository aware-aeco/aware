using System;
using System.Collections.Generic;
using System.Linq;
using Xunit;

namespace AwareTekla.Tests;

/// <summary>
/// Pins the double-angle plan to the canonical section geometry by rebuilding each
/// leg's outline from the plan and comparing it, vertex for vertex, with the outline
/// AWARE's own IFC emitter writes (`double_angle_outlines` in `cli/src/render/ifc.rs`).
/// A plan that separates the pair on the wrong axis, faces the outstanding legs at
/// each other, or drops the mirror fails here rather than in a screenshot.
/// </summary>
public sealed class TeklaDoubleAngleContractTests
{
    const double Tolerance = 1e-9;

    [Theory]
    [InlineData("llbb", 150, 100, 10, 12, 0)]
    [InlineData("llbb", 150, 100, 10, 12, 30)]
    [InlineData("llbb", 150, 100, 10, 12, -90)]
    [InlineData("llbb", 150, 100, 10, 12, 180)]
    [InlineData("llbb", 150, 100, 10, 12, 12.345)]
    [InlineData("llbb", 100, 100, 8, 0, 0)]
    [InlineData("slbb", 150, 100, 10, 12, 0)]
    [InlineData("slbb", 150, 100, 10, 12, 30)]
    [InlineData("slbb", 150, 100, 10, 12, -90)]
    [InlineData("slbb", 150, 100, 10, 12, 180)]
    [InlineData("slbb", 150, 100, 10, 12, 12.345)]
    [InlineData("slbb", 100, 100, 8, 0, 0)]
    public void EachLegReproducesTheCanonicalOutline(string orientation, double d, double b, double t, double gap, double roll)
    {
        // Both zero-frame branches: a column mirrors across the other axis when its
        // beam is reversed, so a plan that ignores the branch point-reflects the pair.
        foreach (var verticalZeroFrame in new[] { false, true })
        {
            var plan = TeklaDoubleAngleContract.CreatePlan(d, b, t, gap, orientation, roll, verticalZeroFrame);
            var expected = CanonicalOutlines(d, b, t, gap, orientation);

            // `a` is the +X leg, so it must match the canonical right-hand outline.
            AssertSameOutline(expected.Right, MemberFrameOutline(plan.Legs[0], d, b, t, roll, verticalZeroFrame));
            AssertSameOutline(expected.Left, MemberFrameOutline(plan.Legs[1], d, b, t, roll, verticalZeroFrame));
        }
    }

    [Fact]
    public void AVerticalMemberReversesTheMirrorAxisAndSoShiftsItsReversedLegByAHalfTurn()
    {
        foreach (var orientation in new[] { "llbb", "slbb" })
        {
            var projected = TeklaDoubleAngleContract.CreatePlan(150, 100, 10, 12, orientation, 20, false);
            var vertical = TeklaDoubleAngleContract.CreatePlan(150, 100, 10, 12, orientation, 20, true);
            for (var index = 0; index < 2; index++)
            {
                var expected = projected.Legs[index].ReversedAxis
                    ? TeklaMemberRollContract.NormalizeDegrees(projected.Legs[index].CanonicalRollDegrees + 180)
                    : projected.Legs[index].CanonicalRollDegrees;
                Assert.Equal(expected, vertical.Legs[index].CanonicalRollDegrees, 9);
                Assert.Equal(projected.Legs[index].OffsetMm, vertical.Legs[index].OffsetMm, 9);
            }
        }
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

    [Theory]
    [InlineData("llbb", 150, 100, 10, 12, 212, 150)]
    [InlineData("slbb", 150, 100, 10, 12, 312, 100)]
    [InlineData("llbb", 100, 100, 8, 0, 200, 100)]
    [InlineData("slbb", 100, 100, 8, 0, 200, 100)]
    public void EnvelopeMatchesTheCanonicalContract(string orientation, double d, double b, double t, double gap, double width, double depth)
    {
        var plan = TeklaDoubleAngleContract.CreatePlan(d, b, t, gap, orientation, 0, false);
        Assert.Equal(width, plan.EnvelopeWidthMm, 9);
        Assert.Equal(depth, plan.EnvelopeDepthMm, 9);
    }

    [Fact]
    public void LegsSeparateSymmetricallyAndLeaveExactlyTheAuthoredGap()
    {
        foreach (var orientation in new[] { "llbb", "slbb" })
        {
            var plan = TeklaDoubleAngleContract.CreatePlan(150, 100, 10, 12, orientation, 0, false);
            Assert.Equal(2, plan.Legs.Length);
            Assert.Equal("a", plan.Legs[0].Suffix);
            Assert.Equal("b", plan.Legs[1].Suffix);
            Assert.Equal(-plan.Legs[0].OffsetMm, plan.Legs[1].OffsetMm, 9);
            Assert.True(plan.Legs[0].OffsetMm > 0);

            var right = MemberFrameOutline(plan.Legs[0], 150, 100, 10, 0, false);
            var left = MemberFrameOutline(plan.Legs[1], 150, 100, 10, 0, false);
            Assert.Equal(12, right.Min(p => p.X) - left.Max(p => p.X), 9);
        }
    }

    [Fact]
    public void OneLegIsAlwaysMirroredBecauseAnUnequalAngleIsChiral()
    {
        foreach (var orientation in new[] { "llbb", "slbb" })
        {
            var plan = TeklaDoubleAngleContract.CreatePlan(150, 100, 10, 12, orientation, 0, false);
            Assert.Single(plan.Legs, leg => leg.ReversedAxis);
        }
    }

    [Theory]
    [InlineData(150, 100, 10, "L150*100*10")]
    [InlineData(100, 100, 8, "L100*100*8")]
    [InlineData(88.9, 63.5, 6.35, "L88.9*63.5*6.35")]
    public void LegProfileIsTheParametricSingleAngle(double d, double b, double t, string expected)
    {
        Assert.Equal(expected, TeklaDoubleAngleContract.CreatePlan(d, b, t, 10, "llbb", 0, false).LegProfile);
    }

    [Theory]
    [InlineData(150, 100, 10, 12, "LLBB")]
    [InlineData(150, 100, 10, 12, "back-to-back")]
    [InlineData(150, 100, 10, 12, "")]
    public void OrientationTokensAreExactAndLowercase(double d, double b, double t, double gap, string orientation)
    {
        Assert.Throws<ArgumentException>(() => TeklaDoubleAngleContract.CreatePlan(d, b, t, gap, orientation, 0, false));
    }

    [Theory]
    [InlineData(150, 100, 100, 12)]
    [InlineData(150, 100, 150, 12)]
    [InlineData(150, 100, 10, -1)]
    [InlineData(0, 100, 10, 12)]
    [InlineData(150, 0, 10, 12)]
    [InlineData(150, 100, 0, 12)]
    [InlineData(double.NaN, 100, 10, 12)]
    [InlineData(150, 100, 10, double.PositiveInfinity)]
    public void DegenerateDimensionsAreRejected(double d, double b, double t, double gap)
    {
        Assert.Throws<ArgumentException>(() => TeklaDoubleAngleContract.CreatePlan(d, b, t, gap, "llbb", 0, false));
    }

    [Fact]
    public void NonFiniteRollIsRejected()
    {
        Assert.Throws<ArgumentException>(() => TeklaDoubleAngleContract.CreatePlan(150, 100, 10, 12, "llbb", double.NaN, false));
    }


    /// <summary>
    /// AWARE's canonical single `angle`, in its own bounding box: the vertical leg
    /// (length `d`) on -X, the flat leg (length `b`) along -Y.
    /// </summary>
    static (double X, double Y)[] CanonicalAngle(double d, double b, double t)
    {
        double hw = b / 2d, hd = d / 2d;
        return new[]
        {
            (-hw, -hd), (hw, -hd), (hw, -hd + t),
            (-hw + t, -hd + t), (-hw + t, hd), (-hw, hd),
        };
    }

    /// <summary>
    /// The canonical pair, transcribed from `double_angle_outlines` in
    /// `cli/src/render/ifc.rs` — the sink-independent definition this plan must hit.
    /// </summary>
    static ((double X, double Y)[] Left, (double X, double Y)[] Right) CanonicalOutlines(double d, double b, double t, double gap, string orientation)
    {
        var (back, outstanding) = orientation == "llbb" ? (d, b) : (b, d);
        double halfBack = back / 2d, halfGap = gap / 2d;
        var left = new[]
        {
            (X: -halfGap, Y: -halfBack),
            (X: -halfGap, Y: halfBack),
            (X: -halfGap - t, Y: halfBack),
            (X: -halfGap - t, Y: -halfBack + t),
            (X: -halfGap - outstanding, Y: -halfBack + t),
            (X: -halfGap - outstanding, Y: -halfBack),
        };
        return (left, left.Select(p => (X: -p.X, Y: p.Y)).ToArray());
    }

    /// <summary>
    /// Rebuilds where Tekla actually puts a leg, in the member's rolled section frame.
    ///
    /// A forward beam at canonical roll phi shows Rot(phi) applied to Tekla's own L;
    /// a reversed beam at canonical roll psi shows MirrorX(Rot(psi)) applied to it.
    /// Tekla's L is itself MirrorX of AWARE's canonical angle — the live 2026 B-rep
    /// probe on `L100*75*8` put the vertical leg on +X. Dividing out the member roll
    /// `theta` leaves the member-frame outline the canonical contract specifies.
    /// </summary>
    static (double X, double Y)[] MemberFrameOutline(TeklaDoubleAngleLeg leg, double d, double b, double t, double theta, bool verticalZeroFrame)
    {
        // Zero frame: Tekla's own L, rolled, then mirrored if the beam was reversed —
        // across Y on the projected branch, across X on the vertical seed branch.
        var zeroFrame = CanonicalAngle(d, b, t)
            .Select(p => (X: -p.X, Y: p.Y))
            .Select(p => Rotate(p, leg.CanonicalRollDegrees));
        if (leg.ReversedAxis)
            zeroFrame = verticalZeroFrame
                ? zeroFrame.Select(p => (X: p.X, Y: -p.Y))
                : zeroFrame.Select(p => (X: -p.X, Y: p.Y));
        // Divide out the member roll, then offset along the member's rolled X.
        return zeroFrame.Select(p => Rotate(p, -theta)).Select(p => (X: p.X + leg.OffsetMm, Y: p.Y)).ToArray();
    }

    static (double X, double Y) Rotate((double X, double Y) p, double degrees)
    {
        var radians = degrees * Math.PI / 180d;
        double cos = Math.Cos(radians), sin = Math.Sin(radians);
        return (p.X * cos - p.Y * sin, p.X * sin + p.Y * cos);
    }

    static void AssertSameOutline(IReadOnlyList<(double X, double Y)> expected, IReadOnlyList<(double X, double Y)> actual)
    {
        Assert.Equal(expected.Count, actual.Count);
        foreach (var want in expected)
            Assert.True(
                actual.Any(got => Math.Abs(got.X - want.X) <= Tolerance && Math.Abs(got.Y - want.Y) <= Tolerance),
                $"missing vertex ({want.X}, {want.Y}); got {string.Join(" ", actual.Select(p => $"({p.X:0.####},{p.Y:0.####})"))}");
    }
}
