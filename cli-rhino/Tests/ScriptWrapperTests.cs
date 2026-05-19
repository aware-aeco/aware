using AwareRhino;
using Xunit;

namespace AwareRhino.Tests;

public class ScriptWrapperTests
{
    [Fact]
    public void SplitUsings_NoUsings_BodyOnly()
    {
        var (usings, body) = ScriptWrapper.SplitUsingsAndBody(
            "var x = 1;\nreturn x;");
        Assert.Empty(usings);
        Assert.Equal("var x = 1;\nreturn x;", body.Trim());
    }

    [Fact]
    public void SplitUsings_OneDirective()
    {
        var (usings, body) = ScriptWrapper.SplitUsingsAndBody(
            "using Rhino.Geometry;\nvar p = new Point3d(0,0,0);\nreturn p;");
        Assert.Single(usings);
        Assert.Contains("using Rhino.Geometry;", usings);
        Assert.DoesNotContain("using Rhino.Geometry", body);
    }

    [Fact]
    public void SplitUsings_MultipleDirectives()
    {
        var (usings, body) = ScriptWrapper.SplitUsingsAndBody(
            "using System.IO;\nusing Rhino;\nusing Rhino.Geometry;\nvar p = new Point3d(0,0,0);");
        Assert.Equal(3, usings.Count);
    }

    [Fact]
    public void SplitUsings_AliasDirective()
    {
        var (usings, body) = ScriptWrapper.SplitUsingsAndBody(
            "using Geo = Rhino.Geometry;\nvar p = new Geo.Point3d(0,0,0);");
        Assert.Single(usings);
        Assert.Contains("using Geo = Rhino.Geometry;", usings);
    }

    [Fact]
    public void SplitUsings_IgnoresUsingStatement()
    {
        // "using (var x = ...)" is a using-STATEMENT, not a directive. Keep it in body.
        var (usings, body) = ScriptWrapper.SplitUsingsAndBody(
            "using (var sw = new System.IO.StringWriter()) { sw.Write(\"x\"); }\nreturn null;");
        Assert.Empty(usings);
        Assert.Contains("using (var sw", body);
    }

    [Fact]
    public void Wrap_ProducesCompilableShape()
    {
        var wrapped = ScriptWrapper.Wrap(
            "using Rhino.Geometry;\nvar p = new Point3d(0,0,0);\nreturn new { x = p.X };");

        // Result must contain preamble sentinels and our __AwareRun shape.
        Assert.Contains("__AWARE_RESULT_BEGIN__", wrapped);
        Assert.Contains("__AWARE_RESULT_END__", wrapped);
        Assert.Contains("static object? __AwareRun(IDictionary<string, object?> args)", wrapped);
        Assert.Contains("var p = new Point3d(0,0,0);", wrapped);
        Assert.Contains("return new { x = p.X };", wrapped);
        // Preamble brings in the Rhino namespace already so user's "using Rhino.Geometry;"
        // should still appear once (deduped against preamble).
        var usingRhinoGeometryCount = System.Text.RegularExpressions.Regex.Matches(
            wrapped, @"^using Rhino\.Geometry;\s*$",
            System.Text.RegularExpressions.RegexOptions.Multiline).Count;
        Assert.Equal(1, usingRhinoGeometryCount);
    }

    [Fact]
    public void Wrap_DedupesPreambleUsings()
    {
        // User supplies a using that's already in the preamble (Rhino).
        var wrapped = ScriptWrapper.Wrap("using Rhino;\nreturn 1;");
        var usingRhinoCount = System.Text.RegularExpressions.Regex.Matches(
            wrapped, @"^using Rhino;\s*$",
            System.Text.RegularExpressions.RegexOptions.Multiline).Count;
        Assert.Equal(1, usingRhinoCount);
    }

    [Fact]
    public void Wrap_PreservesUserBodyVerbatim()
    {
        var body = "// my comment\nvar doc = Rhino.RhinoDoc.ActiveDoc;\nint n = doc.Objects.Count;\nreturn n;";
        var wrapped = ScriptWrapper.Wrap(body);
        Assert.Contains("// my comment", wrapped);
        Assert.Contains("int n = doc.Objects.Count;", wrapped);
        Assert.Contains("return n;", wrapped);
    }

    [Fact]
    public void Wrap_PlacesPreambleUsingsAtTop()
    {
        var wrapped = ScriptWrapper.Wrap("return 1;");
        var idxUsing = wrapped.IndexOf("using System;", StringComparison.Ordinal);
        var idxRun = wrapped.IndexOf("__AwareRun", StringComparison.Ordinal);
        Assert.True(idxUsing >= 0);
        Assert.True(idxRun > idxUsing, "using directives must precede the __AwareRun function");
    }

    [Fact]
    public void Wrap_TopLevelStatementsBeforeLocalFunctions()
    {
        // C# 9 top-level statements MUST come before any type/method declarations
        // (local functions at file bottom are allowed because they belong to the
        // synthetic Program.<Main>$). Top-level call site of __AwareRun must
        // appear before the function definition.
        var wrapped = ScriptWrapper.Wrap("return 1;");
        var idxCall = wrapped.IndexOf("__AwareRun(args)", StringComparison.Ordinal);
        var idxDefn = wrapped.IndexOf("static object? __AwareRun(", StringComparison.Ordinal);
        Assert.True(idxCall >= 0);
        Assert.True(idxDefn >= 0);
        Assert.True(idxCall < idxDefn, "call site must come before local function definition");
    }

    [Fact]
    public void Wrap_EmptyBodyStillCompiles()
    {
        var wrapped = ScriptWrapper.Wrap("");
        Assert.Contains("__AwareRun", wrapped);
        Assert.Contains("return null;", wrapped);  // safe default body
    }

    [Fact]
    public void Wrap_BodyWithoutReturnGetsImplicitNullReturn()
    {
        // If user body has no `return`, we still want __AwareRun to return null
        // rather than be a CS0161 (not all code paths return a value). Wrapper
        // appends a `return null;` if no top-level `return` keyword found.
        var wrapped = ScriptWrapper.Wrap("Console.WriteLine(\"hi\");");
        // Verify there IS a return null somewhere inside __AwareRun.
        // Naive substring check: the wrapper appends a final `return null;` if absent.
        Assert.Contains("Console.WriteLine(\"hi\");", wrapped);
        // Count return statements in the synthetic function — at least 1.
        var runFnStart = wrapped.IndexOf("static object? __AwareRun(", StringComparison.Ordinal);
        var afterFn = wrapped.Substring(runFnStart);
        Assert.Contains("return null;", afterFn);
    }
}
