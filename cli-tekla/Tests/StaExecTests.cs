using System;
using System.IO;
using System.Text.Json.Nodes;
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
    public void Script_Return_Value_Marshals_Back()
    {
        Assert.Equal(3, Run("return 1 + 2;")!.GetValue<int>());
    }

    [Fact]
    public void Top_Level_Await_Completes_And_Returns()
    {
        // A top-level await must run to completion and marshal its value back.
        // (The continuation resumes on the thread pool — we deliberately do NOT
        // install a single-threaded pump, which would deadlock sync-over-async;
        // see RunScriptOnStaThread.)
        Assert.Equal(7, Run(
            "await System.Threading.Tasks.Task.Delay(20);\nreturn 7;")!.GetValue<int>());
    }

    [Fact]
    public void Sync_Over_Async_Does_Not_Deadlock()
    {
        // The exact shape Codex flagged: a finite script that synchronously
        // waits on context-capturing async work. A single-threaded pump would
        // hang here forever (the STA thread blocks on .Result while being the
        // only thread that could pump the continuation). A watchdog thread
        // fails the test instead of hanging the whole run if it regresses.
        JsonNode? result = null;
        var done = new System.Threading.ManualResetEventSlim(false);
        var worker = new System.Threading.Thread(() =>
        {
            result = Run(
                "async System.Threading.Tasks.Task<int> F() " +
                "{ await System.Threading.Tasks.Task.Delay(10); return 1; }\n" +
                "return F().Result;");
            done.Set();
        }) { IsBackground = true };
        worker.Start();
        Assert.True(done.Wait(TimeSpan.FromSeconds(15)), "sync-over-async script deadlocked");
        Assert.Equal(1, result!.GetValue<int>());
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
    public void Receipt_Slot_Is_A_Single_Claim()
    {
        Program.ArmLastResortReceipt("exec", null, null);
        Assert.True(Program.TryClaimReceipt());   // first claimant wins
        Assert.False(Program.TryClaimReceipt());  // losers are told so
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
            Assert.False(Program.TryClaimReceipt());
        }
        finally
        {
            Console.SetOut(prior);
            Program.TryClaimReceipt();
            Environment.ExitCode = priorExitCode; // don't fail the test host
        }
    }
}
