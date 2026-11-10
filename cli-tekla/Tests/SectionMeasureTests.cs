using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.CSharp.Scripting;
using Microsoft.CodeAnalysis.Scripting;
using Xunit;

namespace AwareTekla.Tests;

/// <summary>
/// #799 / #800 (FloLess #3382 A1 and A0m): the parametric listing of `profile-catalog` and the
/// `section-measure` oracle. The pure helpers (labels, name-order symbols, the section-to-face map and the
/// reference point) are executed here as real script code; the Tekla-facing parts are pinned on the source
/// and proved end to end against a live Tekla by the PR's verification.
/// </summary>
public sealed class SectionMeasureTests
{
    private const string BclUsings = "using System;\nusing System.Collections;\nusing System.Collections.Generic;\nusing System.Globalization;\nusing System.Linq;\n";

    private static T Run<T>(string helpers, string expression)
    {
        var options = ScriptOptions.Default
            .AddReferences(typeof(object).Assembly, typeof(Enumerable).Assembly, typeof(Tuple).Assembly)
            .AddImports("System");
        return CSharpScript.EvaluateAsync<T>(BclUsings + helpers + "\nreturn " + expression + ";", options).GetAwaiter().GetResult();
    }

    private static string FitMapsCall(string anchors, string section, string face) =>
        $"fitMaps(new List<double[]> {{ {anchors} }}, new List<double[]> {{ {section} }}, new List<double[]> {{ {face} }})";

    // A 100 x 80 angle in Tekla's profile frame (bounding box centred on the origin), and its outline.
    private const string Angle = "new[]{-40.0,-50.0}, new[]{40.0,-50.0}, new[]{40.0,-42.0}, new[]{-32.0,-42.0}, new[]{-32.0,50.0}, new[]{-40.0,50.0}";

    [Fact]
    public void Scripts_ParseWithNoSyntaxErrors()
    {
        foreach (var (name, code) in new[] { ("SectionMeasureScript", SectionMeasureScript.Code), ("ProfileCatalogScript", ProfileCatalogScript.Code) })
        {
            var tree = Microsoft.CodeAnalysis.CSharp.CSharpSyntaxTree.ParseText(
                code, new Microsoft.CodeAnalysis.CSharp.CSharpParseOptions(kind: SourceCodeKind.Script));
            var errors = tree.GetDiagnostics().Where(d => d.Severity == DiagnosticSeverity.Error).Select(d => d.ToString()).ToList();
            Assert.True(errors.Count == 0, name + " has syntax errors:\n" + string.Join("\n", errors));
        }
    }

    [Fact]
    public void FitMaps_FindsTheOneMapOfAnAsymmetricSection()
    {
        // The face is the angle turned a quarter (x, y) -> (-y, x) and moved by (5, -7).
        var face = "new[]{55.0,-47.0}, new[]{55.0,33.0}, new[]{47.0,33.0}, new[]{47.0,-39.0}, new[]{-45.0,-39.0}, new[]{-45.0,-47.0}";
        var count = Run<int>(SectionMeasureScript.Geometry, FitMapsCall(Angle, Angle, face) + ".Count");
        Assert.Equal(1, count);
        var matrix = Run<string>(SectionMeasureScript.Geometry, "String.Join(\",\", ((double[][])((Dictionary<string, object>)" + FitMapsCall(Angle, Angle, face) + "[0])[\"matrix\"]).SelectMany(r => r))");
        Assert.Equal("0,-1,1,0", matrix);
        var translation = Run<string>(SectionMeasureScript.Geometry, "String.Join(\",\", (double[])((Dictionary<string, object>)" + FitMapsCall(Angle, Angle, face) + "[0])[\"translation\"])");
        Assert.Equal("5,-7", translation);
    }

    [Fact]
    public void FitMaps_FindsNoMapWhenTheFaceIsADifferentShape()
    {
        // Same bounding box as the angle, but the leg is 10 thick instead of 8: no anchor fits within 1e-3 mm.
        var face = "new[]{-40.0,-50.0}, new[]{40.0,-50.0}, new[]{40.0,-40.0}, new[]{-30.0,-40.0}, new[]{-30.0,50.0}, new[]{-40.0,50.0}";
        Assert.Equal(0, Run<int>(SectionMeasureScript.Geometry, FitMapsCall(Angle, Angle, face) + ".Count"));
    }

    [Fact]
    public void FitMaps_ReturnsEveryMapOfASymmetricSection()
    {
        var plate = "new[]{-10.0,-100.0}, new[]{10.0,-100.0}, new[]{10.0,100.0}, new[]{-10.0,100.0}";
        var face = "new[]{-100.0,-10.0}, new[]{100.0,-10.0}, new[]{100.0,10.0}, new[]{-100.0,10.0}";
        Assert.Equal(4, Run<int>(SectionMeasureScript.Geometry, FitMapsCall(plate, plate, face) + ".Count"));
    }

    [Fact]
    public void FitMaps_ProvesOnAnchorsSoADifferentArcTessellationStillMatches()
    {
        // The catalogue rounds a corner with three points, the solid with one: the anchors (unrounded vertices)
        // and the bounding boxes agree, so the map is found although the two point lists differ.
        var anchors = "new[]{0.0,0.0}, new[]{10.0,0.0}, new[]{0.0,10.0}";
        var section = "new[]{0.0,0.0}, new[]{10.0,0.0}, new[]{10.0,6.0}, new[]{9.0,8.0}, new[]{8.0,9.0}, new[]{6.0,10.0}, new[]{0.0,10.0}";
        var face = "new[]{0.0,0.0}, new[]{10.0,0.0}, new[]{10.0,6.0}, new[]{8.5,8.5}, new[]{6.0,10.0}, new[]{0.0,10.0}";
        var maps = Run<int>(SectionMeasureScript.Geometry, FitMapsCall(anchors, section, face) + ".Count");
        Assert.True(maps >= 1, "expected the identity map to fit on the anchors");
    }

    [Fact]
    public void ReferenceFrom_ReadsTheAxisBackIntoTheSectionFrame()
    {
        // Under the quarter turn (x, y) -> (-y, x) with t = (5, -7), the face origin is the section point M^T(-t) = (7, 5):
        // check, M (7, 5) + t = (-5, 7) + (5, -7) = (0, 0).
        var point = Run<string>(SectionMeasureScript.Geometry,
            "String.Join(\",\", referenceFrom(new List<object> { new Dictionary<string, object> { { \"matrix\", new[] { new double[] { 0, -1 }, new double[] { 1, 0 } } }, { \"translation\", new double[] { 5, -7 } } } }).Item1)");
        Assert.Equal("7,5", point);
    }

    [Fact]
    public void ReferenceFrom_StatesNoPointWhenMapsDisagreeOrNoneFit()
    {
        var none = Run<string>(SectionMeasureScript.Geometry, "referenceFrom(new List<object>()).Item2");
        Assert.Contains("could not be matched", none);
        var disagree = Run<bool>(SectionMeasureScript.Geometry,
            "referenceFrom(new List<object> { " +
            "new Dictionary<string, object> { { \"matrix\", new[] { new double[] { 1, 0 }, new double[] { 0, 1 } } }, { \"translation\", new double[] { 0, 0 } } }, " +
            "new Dictionary<string, object> { { \"matrix\", new[] { new double[] { -1, 0 }, new double[] { 0, 1 } } }, { \"translation\", new double[] { 4, 0 } } } }).Item1 == null");
        Assert.True(disagree);
    }

    [Fact]
    public void SignedArea_IsTheShoelaceArea()
    {
        Assert.Equal(1376.0, Run<double>(SectionMeasureScript.Geometry, "Math.Abs(signedArea(new List<double[]> { " + Angle + " }))"));
    }

    [Fact]
    public void Labels_ReadTeklaKeysAsPlainWords()
    {
        Assert.Equal("Flange thickness", Run<string>(ProfileCatalogScript.Labels, "plainLabel(\"albl_Flange_thickness\")"));
        Assert.Equal("Web thickness", Run<string>(ProfileCatalogScript.Labels, "plainLabel(\"WEB_THICKNESS\")"));
        Assert.Equal("", Run<string>(ProfileCatalogScript.Labels, "plainLabel(\"\")"));
        Assert.Equal("mm", Run<string>(ProfileCatalogScript.Labels, "unitOf(\"INPUT_DIMENSION_UNIT\")"));
        Assert.Equal("deg", Run<string>(ProfileCatalogScript.Labels, "unitOf(\"INPUT_ANGLE_UNIT\")"));
        Assert.Equal("other", Run<string>(ProfileCatalogScript.Labels, "unitOf(\"INPUT_FORCE_UNIT\")"));
    }

    [Fact]
    public void Labels_FindTheSymbolsTheNameOrderMentions()
    {
        // The rounding radius r of a welded I is a parameter but not part of its name order.
        Assert.Equal("b,h,s,t", Run<string>(ProfileCatalogScript.Labels, "String.Join(\",\", symbolsIn(\"h-s-t*b\").OrderBy(x => x, StringComparer.Ordinal))"));
        Assert.Equal("Rb,h,ye,ze", Run<string>(ProfileCatalogScript.Labels, "String.Join(\",\", symbolsIn(\"h-Rb[-ye[*ze]]\").OrderBy(x => x, StringComparer.Ordinal))"));
    }

    [Fact]
    public void SectionMeasure_DeletesEveryTemporaryMemberAndNeverCommits()
    {
        // Line endings follow the checkout (CRLF on a Windows clone), so compare on LF.
        var code = SectionMeasureScript.Code.Replace("\r\n", "\n");
        Assert.DoesNotContain("CommitChanges", code);
        Assert.DoesNotContain(".Modify(", code);
        Assert.DoesNotContain(".Save(", code);
        // Every insert is undone in a finally, and a delete that fails is reported, never swallowed silently.
        var finallyAt = code.IndexOf("\n    finally\n    {", StringComparison.Ordinal);
        Assert.True(finallyAt >= 0, "measurePlacement must have a finally block");
        Assert.True(finallyAt > code.IndexOf("beam.Insert()", StringComparison.Ordinal), "the delete must follow the insert in a finally");
        Assert.Contains("gone = beam.Delete();", code.Substring(finallyAt));
        Assert.Contains("cleanupFailures.Add(", code.Substring(finallyAt));
        Assert.Contains("beam.Name = TemporaryName;", code);
        Assert.Contains("const string TemporaryName = \"" + SectionMeasureScript.TemporaryPartName + "\";", code);
        Assert.Contains("const int MaxInserts = " + SectionMeasureScript.MaxInserts + ";", code);
        // A closed model is refused, not answered empty.
        Assert.Contains("!m.GetConnectionStatus()", code);
        // Input is validated before the first insert: the budget check precedes the model work.
        Assert.True(code.IndexOf("insertBudget > MaxInserts", StringComparison.Ordinal) < code.IndexOf("new Beam(", StringComparison.Ordinal));
    }

    [Fact]
    public void Verbs_AreDispatchedAndRunScriptOwned()
    {
        var source = File.ReadAllText(FindProgramSource());
        Assert.Contains("case \"section-measure\":", source);
        var at = source.IndexOf("ExecuteResolvedScript(\"section-measure\", SectionMeasureScript.Code", StringComparison.Ordinal);
        Assert.True(at >= 0, "section-measure must run SectionMeasureScript.Code");
        Assert.StartsWith("ExecuteResolvedScript(\"section-measure\", SectionMeasureScript.Code, version, pid, argsNode, ScriptCommitPolicy.ScriptOwned)", source.Substring(at));
        // profile-catalog forwards the new parametric flag to its script.
        Assert.Contains("[\"parametric\"] = input?[\"parametric\"]?.DeepClone()", source);
        Assert.Contains("GetParametricProfileItems()", ProfileCatalogScript.Code);
        Assert.Contains("ParameterString", ProfileCatalogScript.Code);
    }

    [Fact]
    public void Scripts_CompileAgainstEverySupportedInstalledSdkWhenRequired()
    {
        if (!string.Equals(Environment.GetEnvironmentVariable("AWARE_TEKLA_REQUIRE_SUPPORTED_SDKS"), "1", StringComparison.Ordinal))
            return;
        foreach (var version in new[] { "2025.0", "2026.0" })
        {
            var install = Path.Combine(@"C:\Program Files\Tekla Structures", version);
            Assert.True(Directory.Exists(install), $"Required Tekla {version} SDK is not installed at {install}");
            var (references, _) = Program.ResolveTeklaReferences(install);
            foreach (var (name, code) in new[] { ("SectionMeasureScript", SectionMeasureScript.Code), ("ProfileCatalogScript", ProfileCatalogScript.Code) })
            {
                var script = CSharpScript.Create<object>(code, Program.CreateScriptOptions(references), typeof(ExecGlobals));
                var errors = script.Compile().Where(d => d.Severity == DiagnosticSeverity.Error).ToArray();
                Assert.True(errors.Length == 0, $"{name} does not compile against Tekla {version}:\n{string.Join("\n", errors.Select(e => e.ToString()))}");
            }
        }
    }

    private static string FindProgramSource()
    {
        var dir = new DirectoryInfo(AppContext.BaseDirectory);
        while (dir != null && !File.Exists(Path.Combine(dir.FullName, "cli-tekla.csproj")))
            dir = dir.Parent;
        Assert.NotNull(dir);
        return Path.Combine(dir!.FullName, "Program.cs");
    }
}
