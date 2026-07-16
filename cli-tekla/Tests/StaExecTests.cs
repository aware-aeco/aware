using System;
using System.IO;
using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.Scripting;
using Xunit;

namespace AwareTekla.Tests;

// #283 — exec scripts run on a dedicated STA thread (Tekla's Open API is
// written for STA standalone apps; catalogue calls terminated the process
// when driven from the default MTA main thread), and a script that takes the
// process down still leaves a structured fail receipt via the last-resort
// hooks. These tests drive the helpers headlessly (no Tekla references).
public class StaExecTests
{
    static object? Run(string code) =>
        Program.RunScriptOnStaThread(
            code,
            Array.Empty<MetadataReference>(),
            argsNode: null,
            teklaBinDir: null,
            Program.ScriptCommitPolicy.Automatic);

    [Fact]
    public void Script_Executes_On_Sta_Thread()
    {
        var apartment = Run(
            "return System.Threading.Thread.CurrentThread.GetApartmentState().ToString();");
        Assert.Equal("STA", apartment);
    }

    [Fact]
    public void Script_Return_Value_Marshals_Back()
    {
        Assert.Equal(3, Run("return 1 + 2;"));
    }

    [Fact]
    public void Script_Exception_Rethrows_With_Original_Frames()
    {
        var ex = Assert.Throws<InvalidOperationException>(
            () => Run("throw new System.InvalidOperationException(\"boom\");"));
        Assert.Equal("boom", ex.Message);
        // ExceptionDispatchInfo must preserve the script-side frames — the
        // exec receipt carries this stack for the caller to re-draft against.
        Assert.Contains("Submission", ex.StackTrace ?? "");
    }

    [Fact]
    public void Bad_Code_Surfaces_Compilation_Error()
    {
        Assert.Throws<CompilationErrorException>(() => Run("this is not C#"));
    }

    [Fact]
    public void LastResort_Receipt_Emits_Once_When_Armed_And_Never_When_Disarmed()
    {
        var sw = new StringWriter();
        var prior = Console.Out;
        Console.SetOut(sw);
        try
        {
            // Disarmed: nothing is written.
            Program.EmitLastResortReceipt("vendor exit", "");
            Assert.Equal("", sw.ToString());

            // Armed: exactly one ok:false receipt, then re-emission no-ops.
            Program.ArmLastResortReceipt("exec", "2026.0", 1234);
            Program.EmitLastResortReceipt("vendor exit", "");
            Program.EmitLastResortReceipt("vendor exit", "");
            var lines = sw.ToString().Trim().Split('\n');
            Assert.Single(lines);
            Assert.Contains("\"ok\":false", lines[0]);
            Assert.Contains("vendor exit", lines[0]);
            Assert.Contains("2026.0", lines[0]);
        }
        finally
        {
            Console.SetOut(prior);
            Program.DisarmLastResortReceipt();
        }
    }
}
