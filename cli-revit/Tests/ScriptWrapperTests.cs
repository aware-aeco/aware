using AwareRevit.Sidecar;
using Xunit;

namespace AwareRevit.Tests;

public class ScriptWrapperTests
{
    [Fact]
    public void SplitUsings_NoUsings_BodyOnly()
    {
        var (usings, body) = ScriptWrapper.SplitUsingsAndBody("var x = 1;\nreturn x;");
        Assert.Empty(usings);
        Assert.Equal("var x = 1;\nreturn x;", body.Trim());
    }

    [Fact]
    public void SplitUsings_MixedDirectives_Aliases()
    {
        var (usings, body) = ScriptWrapper.SplitUsingsAndBody(
            "using Autodesk.Revit.DB;\nusing DB = Autodesk.Revit.DB;\nvar x = 1;");
        Assert.Equal(2, usings.Count);
        Assert.DoesNotContain("using ", body);
    }

    [Fact]
    public void SplitUsings_DoesNotMatchUsingStatement()
    {
        var (usings, body) = ScriptWrapper.SplitUsingsAndBody(
            "using (var t = new Transaction(doc)) { t.Start(); }\nreturn null;");
        Assert.Empty(usings);
        Assert.Contains("using (var t", body);
    }

    [Fact]
    public void Wrap_AddsPreambleAndAwareRunFunction()
    {
        var wrapped = ScriptWrapper.Wrap("return 42;");
        Assert.Contains("__AwareRun", wrapped);
        Assert.Contains("using Autodesk.Revit.DB;", wrapped);
        Assert.Contains("return 42;", wrapped);
    }

    [Fact]
    public void Wrap_EmptyBody_DefaultsToReturnNull()
    {
        var wrapped = ScriptWrapper.Wrap("");
        Assert.Contains("return null;", wrapped);
    }

    [Fact]
    public void Wrap_DedupesUsingsAgainstPreamble()
    {
        var wrapped = ScriptWrapper.Wrap("using Autodesk.Revit.DB;\nreturn 1;");
        var count = System.Text.RegularExpressions.Regex.Matches(
            wrapped, @"^using Autodesk\.Revit\.DB;\s*$",
            System.Text.RegularExpressions.RegexOptions.Multiline).Count;
        Assert.Equal(1, count);
    }
}
