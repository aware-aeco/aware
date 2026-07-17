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
    static System.Text.Json.Nodes.JsonNode? Run(string code) =>
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
        Assert.Equal("STA", apartment!.GetValue<string>());
    }

    [Fact]
    public void Awaited_Continuation_Stays_On_Sta_Thread()
    {
        // Without a pumping SynchronizationContext the continuation of a
        // top-level await resumes on the MTA thread pool — exactly the
        // vendor-fatal shape the STA thread exists to prevent.
        var apartment = Run(
            "await System.Threading.Tasks.Task.Delay(25);\n" +
            "return System.Threading.Thread.CurrentThread.GetApartmentState().ToString();");
        Assert.Equal("STA", apartment!.GetValue<string>());
    }

    [Fact]
    public void Script_Return_Value_Marshals_Back()
    {
        Assert.Equal(3, Run("return 1 + 2;")!.GetValue<int>());
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
    public void Async_Void_Straggler_Does_Not_Kill_The_Process()
    {
        // A script can fire async-void work and return before its awaited
        // continuation lands. The continuation then Posts to a pump that has
        // already shut down — that Post must run inline, never throw (a throw
        // here is an unhandled exception that kills the sidecar).
        var result = Run(
            "async void Fire() { await System.Threading.Tasks.Task.Delay(80); }\n" +
            "Fire();\n" +
            "return 42;");
        Assert.Equal(42, result!.GetValue<int>());
        // Keep the process alive long enough for the straggler to land; if
        // Post threw, this test run would crash rather than fail an assert.
        System.Threading.Thread.Sleep(300);
    }

    [Fact]
    public void Disarm_Is_A_Single_Claim()
    {
        Program.ArmLastResortReceipt("exec", null, null);
        Assert.True(Program.DisarmLastResortReceipt());   // first claimant wins
        Assert.False(Program.DisarmLastResortReceipt());  // losers are told so
    }

    [Fact]
    public void LastResort_Receipt_Emits_Once_When_Armed_And_Never_When_Disarmed()
    {
        var sw = new StringWriter();
        var prior = Console.Out;
        var priorExitCode = Environment.ExitCode;
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

            // A fired fallback also forces a failing status, and the normal
            // path (a would-be second claimant) is told to suppress its receipt.
            Assert.Equal(2, Environment.ExitCode);
            Assert.False(Program.DisarmLastResortReceipt());
        }
        finally
        {
            Console.SetOut(prior);
            Program.DisarmLastResortReceipt();
            Environment.ExitCode = priorExitCode; // don't fail the test host
        }
    }
}
