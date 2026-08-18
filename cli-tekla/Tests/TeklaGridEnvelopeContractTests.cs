using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.CSharp.Scripting;
using Xunit;

namespace AwareTekla.Tests;

public sealed class TeklaGridEnvelopeContractTests
{
    readonly TeklaGridEnvelopeContract _contract = new();

    [Fact]
    public void ExactRectangularAxesRemainExact()
    {
        var result = _contract.Evaluate(
            new[]
            {
                Axis("X-A", "x", 0, "A", 0, 3000),
                Axis("X-B", "x", 6000, "B", 0, 3000),
                Axis("Y-1", "y", 0, "1", 0, 6000),
                Axis("Y-2", "y", 3000, "2", 0, 6000),
            },
            Levels());

        Assert.True(result.IsSupported);
        Assert.False(result.ExpandsAuthoredExtents);
        Assert.Equal(0, result.XFamilyStartMm);
        Assert.Equal(3000, result.XFamilyEndMm);
        Assert.Equal(0, result.YFamilyStartMm);
        Assert.Equal(6000, result.YFamilyEndMm);
    }

    [Fact]
    public void IndependentAxesUseTheTightestNonTruncatingSharedEnvelope()
    {
        var result = _contract.Evaluate(
            new[]
            {
                Axis("X-A", "x", 0, "A", -100, 3500),
                Axis("X-B", "x", 6000, "B", -200, 3200),
                Axis("Y-1", "y", 0, "1", -500, 6500),
                Axis("Y-2", "y", 3000, "2", -250, 6250),
            },
            Levels());

        Assert.True(result.IsSupported);
        Assert.True(result.ExpandsAuthoredExtents);
        Assert.Equal(-200, result.XFamilyStartMm);
        Assert.Equal(3500, result.XFamilyEndMm);
        Assert.Equal(-500, result.YFamilyStartMm);
        Assert.Equal(6500, result.YFamilyEndMm);
    }

    [Fact]
    public void CoordinateSpansAreContainedEvenWhenAuthoredSegmentsAreShorter()
    {
        var result = _contract.Evaluate(
            new[]
            {
                Axis("X-A", "x", -1000, "A", 100, 200),
                Axis("X-B", "x", 7000, "B", 100, 200),
                Axis("Y-1", "y", -500, "1", 50, 100),
                Axis("Y-2", "y", 4000, "2", 50, 100),
            },
            Levels());

        Assert.True(result.IsSupported);
        Assert.Equal(-500, result.XFamilyStartMm);
        Assert.Equal(4000, result.XFamilyEndMm);
        Assert.Equal(-1000, result.YFamilyStartMm);
        Assert.Equal(7000, result.YFamilyEndMm);
    }

    [Theory]
    [InlineData("single-family", "tekla-grid-single-family-unsupported")]
    [InlineData("duplicate-axis", "tekla-grid-duplicate-axis-offset-unsupported")]
    [InlineData("duplicate-level", "tekla-grid-duplicate-elevation-unsupported")]
    [InlineData("blank-label", "tekla-grid-blank-label-unsupported")]
    [InlineData("overflow", "tekla-grid-derived-envelope-unsupported")]
    public void TeklaOnlyLimitsAreExhaustivelyClassifiable(string scenario, string expectedCode)
    {
        IReadOnlyList<GridAxisExtentContract> axes;
        IReadOnlyList<GridLevelContract> levels = Levels();

        switch (scenario)
        {
            case "single-family":
                axes = new[] { Axis("X-A", "x", 0, "A", 0, 1000) };
                break;
            case "duplicate-axis":
                axes = RectangularAxes().Concat(new[] { Axis("X-C", "x", 0, "C", 0, 3000) }).ToArray();
                break;
            case "duplicate-level":
                axes = RectangularAxes();
                levels = Levels().Concat(new[] { new GridLevelContract("Z-0-copy", 0, "datum") }).ToArray();
                break;
            case "blank-label":
                axes = new[]
                {
                    Axis("X-A", "x", 0, " ", 0, 3000),
                    Axis("Y-1", "y", 0, "1", 0, 6000),
                };
                break;
            case "overflow":
                axes = new[]
                {
                    Axis("X-A", "x", 0, "A", 0, double.MaxValue),
                    Axis("Y-1", "y", -double.MaxValue, "1", 0, 1),
                };
                break;
            default:
                throw new InvalidOperationException(scenario);
        }

        var result = _contract.Evaluate(axes, levels);

        Assert.False(result.IsSupported);
        Assert.Equal(expectedCode, result.Code);
    }

    [Fact]
    public void UnsupportedEvaluationDoesNotPoisonAnUnrelatedGrid()
    {
        var unsupported = _contract.Evaluate(
            new[] { Axis("X-A", "x", 0, "A", 0, 1000) },
            Levels());
        var supported = _contract.Evaluate(RectangularAxes(), Levels());

        Assert.False(unsupported.IsSupported);
        Assert.True(supported.IsSupported);
    }

    [Fact]
    public void CanonicalBakeScriptCompilesAgainstEverySupportedInstalledSdkWhenRequired()
    {
        if (!string.Equals(
                Environment.GetEnvironmentVariable("AWARE_TEKLA_REQUIRE_SUPPORTED_SDKS"),
                "1",
                StringComparison.Ordinal))
            return;

        foreach (var version in new[] { "2025.0", "2026.0" })
        {
            var install = Path.Combine(@"C:\Program Files\Tekla Structures", version);
            Assert.True(Directory.Exists(install), $"Required Tekla {version} SDK is not installed at {install}");
            var (references, _) = Program.ResolveTeklaReferences(install);
            Assert.NotEmpty(references);

            var script = CSharpScript.Create<object>(
                BakeSceneScript.Code,
                Program.CreateScriptOptions(references),
                typeof(ExecGlobals));
            var errors = script.Compile()
                .Where(diagnostic => diagnostic.Severity == DiagnosticSeverity.Error)
                .ToArray();
            Assert.True(
                errors.Length == 0,
                $"BakeSceneScript.Code does not compile against Tekla {version}:\n{string.Join("\n", errors.Select(error => error.ToString()))}");
        }
    }

    static GridAxisExtentContract Axis(
        string id,
        string direction,
        double offsetMm,
        string label,
        double startMm,
        double endMm) =>
        new(id, direction, offsetMm, label, startMm, endMm);

    static IReadOnlyList<GridAxisExtentContract> RectangularAxes() =>
        new[]
        {
            Axis("X-A", "x", 0, "A", 0, 3000),
            Axis("X-B", "x", 6000, "B", 0, 3000),
            Axis("Y-1", "y", 0, "1", 0, 6000),
            Axis("Y-2", "y", 3000, "2", 0, 6000),
        };

    static IReadOnlyList<GridLevelContract> Levels() =>
        new[]
        {
            new GridLevelContract("Z-0", 0, "+0"),
            new GridLevelContract("Z-3000", 3000, "+3000"),
        };
}
