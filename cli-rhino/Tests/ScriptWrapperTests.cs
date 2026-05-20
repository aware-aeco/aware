using AwareRhino;
using Xunit;

namespace AwareRhino.Tests;

public class ScriptWrapperTests
{
    const string ResultPath = @"C:\Temp\aware-result.json";
    const string ArgsJson = "{}";

    [Fact]
    public void Wrap_NestsBodyInsideAwareRunFunction()
    {
        var wrapped = ScriptWrapper.Wrap("return 1", ResultPath, ArgsJson);
        Assert.Contains("def __aware_run(args):", wrapped);
        // Body line is indented 4 spaces under the function.
        Assert.Contains("\n    return 1\n", wrapped);
    }

    [Fact]
    public void Wrap_EmitsPythonPreambleImports()
    {
        var wrapped = ScriptWrapper.Wrap("return 1", ResultPath, ArgsJson);
        Assert.Contains("import json, traceback, os", wrapped);
        Assert.Contains("import Rhino", wrapped);
        Assert.Contains("import rhinoscriptsyntax as rs", wrapped);
        Assert.Contains("import scriptcontext as sc", wrapped);
        Assert.Contains("import System", wrapped);
    }

    [Fact]
    public void Wrap_WritesResultToTheGivenPath()
    {
        var wrapped = ScriptWrapper.Wrap("return 1", ResultPath, ArgsJson);
        // The path is embedded as a Python string literal (backslashes escaped).
        Assert.Contains(@"""C:\\Temp\\aware-result.json""", wrapped);
        Assert.Contains("open(__aware_result_path, \"w\", encoding=\"utf-8\")", wrapped);
        Assert.Contains("json.dump(__aware_out, __aware_f, default=str)", wrapped);
    }

    [Fact]
    public void Wrap_BuildsOkAndErrorBranches()
    {
        var wrapped = ScriptWrapper.Wrap("return 1", ResultPath, ArgsJson);
        Assert.Contains("__aware_retval = __aware_run(__aware_args)", wrapped);
        Assert.Contains("\"ok\": True, \"result\": __aware_retval", wrapped);
        Assert.Contains("except Exception as __aware_ex:", wrapped);
        Assert.Contains("\"ok\": False, \"error\": str(__aware_ex), \"stack\": traceback.format_exc()", wrapped);
    }

    [Fact]
    public void Wrap_EmbedsArgsJsonLiteral()
    {
        var wrapped = ScriptWrapper.Wrap("return 1", ResultPath, """{"a":1}""");
        // Embedded literal with the inner quotes escaped for Python.
        Assert.Contains("__aware_args_json = \"{\\\"a\\\":1}\"", wrapped);
        Assert.Contains("__aware_args = json.loads(", wrapped);
    }

    [Fact]
    public void Wrap_PreservesMultiLineBodyIndentation()
    {
        var body = "doc = Rhino.RhinoDoc.ActiveDoc\nn = doc.Objects.Count\nreturn n";
        var wrapped = ScriptWrapper.Wrap(body, ResultPath, ArgsJson);
        Assert.Contains("\n    doc = Rhino.RhinoDoc.ActiveDoc\n", wrapped);
        Assert.Contains("\n    n = doc.Objects.Count\n", wrapped);
        Assert.Contains("\n    return n\n", wrapped);
    }

    [Fact]
    public void Wrap_PreservesNestedIndentation()
    {
        // A user `for` loop body keeps its own 4-space indent ON TOP of the
        // function's 4-space indent → 8 spaces total for the inner line.
        var body = "total = 0\nfor x in range(3):\n    total += x\nreturn total";
        var wrapped = ScriptWrapper.Wrap(body, ResultPath, ArgsJson);
        Assert.Contains("\n    for x in range(3):\n", wrapped);
        Assert.Contains("\n        total += x\n", wrapped);
    }

    [Fact]
    public void Wrap_BodyWithoutReturnGetsImplicitNoneReturn()
    {
        var wrapped = ScriptWrapper.Wrap("x = 1", ResultPath, ArgsJson);
        Assert.Contains("\n    x = 1\n", wrapped);
        // No top-level return in the body → wrapper appends one.
        Assert.Contains("\n    return None\n", wrapped);
    }

    [Fact]
    public void Wrap_EmptyBodyStillProducesValidFunction()
    {
        var wrapped = ScriptWrapper.Wrap("", ResultPath, ArgsJson);
        Assert.Contains("def __aware_run(args):", wrapped);
        Assert.Contains("\n    return None\n", wrapped);
    }

    [Fact]
    public void Wrap_DoesNotEmitCSharpOrSentinels()
    {
        var wrapped = ScriptWrapper.Wrap("return 1", ResultPath, ArgsJson);
        Assert.DoesNotContain("__AWARE_RESULT_BEGIN__", wrapped);
        Assert.DoesNotContain("__AWARE_RESULT_END__", wrapped);
        Assert.DoesNotContain("Console.WriteLine", wrapped);
        Assert.DoesNotContain("using System;", wrapped);
    }

    [Fact]
    public void PyStringLiteral_EscapesBackslashesAndQuotes()
    {
        Assert.Equal(@"""C:\\a\\b.json""", ScriptWrapper.PyStringLiteral(@"C:\a\b.json"));
        Assert.Equal("\"say \\\"hi\\\"\"", ScriptWrapper.PyStringLiteral("say \"hi\""));
        Assert.Equal("\"line1\\nline2\"", ScriptWrapper.PyStringLiteral("line1\nline2"));
    }
}
