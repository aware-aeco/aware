// Roslyn-script host that compiles + runs the user's wrapped C# against the
// currently-loaded RevitAPI inside Revit's AppDomain. The references come
// from already-loaded assemblies (Assembly.Location on the in-Revit
// RevitAPI / RevitAPIUI). Globals expose `uiapp` (dynamic UIApplication)
// and `args` (input dict).

using System.Reflection;
using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.CSharp.Scripting;
using Microsoft.CodeAnalysis.Scripting;
using Autodesk.Revit.UI;
using Autodesk.Revit.DB;
using AwareRevit.Shared;

namespace AwareRevit.AddIn;

/// <summary>Roslyn-script globals — exposed to user C# as `uiapp` and `args`.
/// MUST be public + non-nested so Roslyn's dynamically-generated Submission#N
/// type can access it from a different assembly.
///
/// `uiapp` is statically typed as Autodesk.Revit.UI.UIApplication (not `dynamic`)
/// so that LINQ extension methods + lambdas work directly off it. The cost is
/// a compile-time dependency on RevitAPIUI in the script's reference set, but
/// we add that anyway in ResolveReferences().</summary>
public sealed class ExecGlobals
{
    public UIApplication uiapp { get; set; } = null!;
    public IDictionary<string, object?> args { get; set; } = new Dictionary<string, object?>();
}

internal static class ScriptEngine
{
    static readonly string[] PreambleUsings =
    {
        "System",
        "System.Collections.Generic",
        "System.Linq",
        "System.IO",
        "System.Text.Json",
        "Autodesk.Revit.ApplicationServices",
        "Autodesk.Revit.Attributes",
        "Autodesk.Revit.DB",
        "Autodesk.Revit.DB.Structure",
        "Autodesk.Revit.UI",
        "Autodesk.Revit.UI.Selection",
    };

    public static ExecResponse RunOnMainThread(UIApplication ui, ExecRequest req)
    {
        var pid = System.Diagnostics.Process.GetCurrentProcess().Id;
        var ver = ui.Application.VersionNumber;
        var stdoutCapture = new System.IO.StringWriter();
        var originalOut = Console.Out;
        Console.SetOut(stdoutCapture);

        try
        {
            var refs = ResolveReferences();
            var opts = ScriptOptions.Default
                .WithReferences(refs)
                .WithImports(PreambleUsings)
                .WithEmitDebugInformation(false);

            var globals = new ExecGlobals
            {
                uiapp = ui,
                args  = req.Args ?? new Dictionary<string, object?>(),
            };

            var script = CSharpScript.Create<object>(req.Code, opts, typeof(ExecGlobals));
            object? result;
            if (req.Transaction == "auto")
            {
                using var tx = new Transaction(ui.ActiveUIDocument.Document, "AWARE exec");
                tx.Start();
                // Never let Revit force a modal failure dialog on commit. This runs
                // unattended over a pipe: a modal blocks Revit's API thread INSIDE an
                // open transaction, and that is the same thread the pipe handler is
                // waiting on — so the whole bridge stops answering until a human
                // clicks OK on a dialog nobody was told about (#328). bake-scene has
                // guarded this since it shipped; exec, the more general write path,
                // did not. Anything warning-worthy — an off-axis brace, geometry
                // joined out from under a dimension — trips it.
                var failureOptions = tx.GetFailureHandlingOptions();
                failureOptions.SetForcedModalHandling(false);
                failureOptions.SetClearAfterRollback(true);
                failureOptions.SetDelayedMiniWarnings(true);
                tx.SetFailureHandlingOptions(failureOptions);
                try
                {
                    result = script.RunAsync(globals).GetAwaiter().GetResult().ReturnValue;
                    // Commit REPORTS its outcome rather than always throwing, and
                    // suppressing the modal above is exactly what makes the quiet
                    // outcomes reachable: Revit's failure processing now resolves
                    // on its own and can roll back — or leave the transaction
                    // pending — without raising. Treating Commit as
                    // fire-and-forget would hand back ok:true for changes the
                    // model does not have. bake-scene guards this the same way.
                    var commitStatus = tx.Commit();
                    if (commitStatus == TransactionStatus.Pending)
                    {
                        // Revit's failure processing has not finished, so the
                        // change may still land. Do NOT claim nothing was
                        // written — that would send the caller into a retry that
                        // duplicates the edit if the commit later succeeds.
                        throw new Exception(
                            "Revit has not resolved the exec's transaction (status: Pending) — its "
                            + "failure processing is still running, so the change may yet be "
                            + "committed. Check the model before retrying; a retry may duplicate it.");
                    }
                    if (commitStatus != TransactionStatus.Committed)
                    {
                        throw new Exception(
                            $"Revit did not commit the exec (transaction status: {commitStatus}). "
                            + "Nothing was written; this is usually Revit's failure processing "
                            + "rejecting the change or a warning resolving to a rollback.");
                    }
                }
                catch
                {
                    // Only roll back a transaction still in our hands. HasEnded()
                    // covers the commit that already resolved to RolledBack, and
                    // a Pending one belongs to Revit's failure processing —
                    // rolling either back here throws on top of the real fault.
                    // Same guard bake-scene uses.
                    if (tx.HasStarted() && !tx.HasEnded()
                        && tx.GetStatus() != TransactionStatus.Pending) tx.RollBack();
                    throw;
                }
            }
            else
            {
                result = script.RunAsync(globals).GetAwaiter().GetResult().ReturnValue;
            }

            return new ExecResponse
            {
                Id          = req.Id,
                Ok          = true,
                Result      = ResultSerializer.ToJson(result),
                StdoutLog   = stdoutCapture.ToString(),
                HostVersion = ver,
                HostPid     = pid,
            };
        }
        catch (CompilationErrorException ce)
        {
            var diag = string.Join("\n", ce.Diagnostics.Select(d => d.ToString()));
            return new ExecResponse
            {
                Id          = req.Id,
                Ok          = false,
                Error       = $"compile error: {ce.Message}",
                Stack       = diag,
                StdoutLog   = stdoutCapture.ToString(),
                HostVersion = ver,
                HostPid     = pid,
            };
        }
        catch (Exception ex)
        {
            var root = ex;
            while (root is TargetInvocationException && root.InnerException is not null)
                root = root.InnerException;
            return new ExecResponse
            {
                Id          = req.Id,
                Ok          = false,
                Error       = $"{root.GetType().Name}: {root.Message}",
                Stack       = root.StackTrace ?? "",
                StdoutLog   = stdoutCapture.ToString(),
                HostVersion = ver,
                HostPid     = pid,
            };
        }
        finally
        {
            Console.SetOut(originalOut);
        }
    }

    static List<MetadataReference> ResolveReferences()
    {
        // Use already-loaded Revit + BCL assemblies. Assembly.Location is the
        // safest source here: Revit's loader places its DLLs on disk under
        // C:\Program Files\Autodesk\Revit 2026\, so Location is populated.
        var refs = new List<MetadataReference>();
        void Add(Type t)
        {
            try
            {
                var loc = t.Assembly.Location;
                if (!string.IsNullOrEmpty(loc) && File.Exists(loc))
                    refs.Add(MetadataReference.CreateFromFile(loc));
            }
            catch { /* skip unresolvable */ }
        }

        Add(typeof(object));                       // System.Private.CoreLib
        Add(typeof(System.Linq.Enumerable));        // System.Linq
        Add(typeof(System.Collections.Generic.IDictionary<,>));
        Add(typeof(System.Dynamic.DynamicObject));  // System.Core
        Add(typeof(Microsoft.CSharp.RuntimeBinder.Binder));
        Add(typeof(System.Text.Json.JsonSerializer));
        Add(typeof(Autodesk.Revit.UI.UIApplication));  // RevitAPIUI
        Add(typeof(Autodesk.Revit.DB.Document));       // RevitAPI

        // De-dup by file path.
        return refs
            .GroupBy(r => (r as PortableExecutableReference)?.FilePath ?? Guid.NewGuid().ToString())
            .Select(g => g.First())
            .ToList();
    }
}
