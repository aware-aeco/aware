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
            // What Revit objected to during the commit. Previously this went to a
            // dialog nobody was watching; now it reaches the caller (#328/#337).
            IReadOnlyList<string> revitWarnings = Array.Empty<string>();
            if (req.Transaction == "auto")
            {
                // NOT a `using`. Disposing an unfinished transaction makes Revit's
                // destructor roll it back and discard the user's edit, and a Pending
                // commit is exactly "unfinished" — so the scope exit would silently
                // throw away work Revit was still finalizing (#337). Disposal is
                // therefore explicit, and skipped while the transaction is Pending.
                var tx = new Transaction(ui.ActiveUIDocument.Document, "AWARE exec");
                var leftWithRevit = false;
                try
                {
                    tx.Start();
                    // Never let Revit force a modal failure dialog on commit. This runs
                    // unattended over a pipe: a modal blocks Revit's API thread INSIDE an
                    // open transaction, and that is the same thread the pipe handler is
                    // waiting on — so the whole bridge stops answering until a human
                    // clicks OK on a dialog nobody was told about (#328).
                    //
                    // Suppressing the modal only downgrades it to a NON-blocking dialog
                    // though, which is what leaves Commit returning Pending. The
                    // preprocessor is what removes the dialog altogether: it resolves the
                    // failures before Revit shows anything, so the commit finishes
                    // synchronously and the warnings become receipt data (#337).
                    var preprocessor = new AwareFailurePreprocessor();
                    var failureOptions = tx.GetFailureHandlingOptions();
                    failureOptions.SetForcedModalHandling(false);
                    failureOptions.SetClearAfterRollback(true);
                    failureOptions.SetDelayedMiniWarnings(true);
                    failureOptions.SetFailuresPreprocessor(preprocessor);
                    tx.SetFailureHandlingOptions(failureOptions);

                    result = script.RunAsync(globals).GetAwaiter().GetResult().ReturnValue;

                    // Commit REPORTS its outcome rather than always throwing, so a
                    // fire-and-forget call would hand back ok:true for changes the model
                    // does not have.
                    var commitStatus = tx.Commit();
                    revitWarnings = preprocessor.Warnings;

                    if (commitStatus == TransactionStatus.Pending)
                    {
                        // With the preprocessor in place Revit should never be left
                        // waiting on a user, so this is defence in depth. Leave the
                        // transaction alone: RollBack() throws while the document is in
                        // failure mode, and disposing it discards the edit. Revit
                        // resolves the status asynchronously and owns it from here.
                        //
                        // Skipping Dispose() is not sufficient on its own — once this
                        // local falls out of scope the object is unrooted and the GC may
                        // run Revit's finalizer, which rolls back exactly the edit we are
                        // protecting. Suppress the finalizer and hold a durable root.
                        leftWithRevit = true;
                        AwarePendingCommits.LeaveWithRevit(tx);
                        throw new Exception(
                            "Revit left the exec's transaction unresolved (status: Pending) — its "
                            + "failure processing is still running and it now owns the outcome. "
                            + "Whether the change landed is UNKNOWN: inspect the model before "
                            + "retrying, because a blind retry would duplicate it if it did. "
                            + "No further write can start until Revit finalizes this.");
                    }
                    if (commitStatus != TransactionStatus.Committed)
                    {
                        throw new Exception(
                            $"Revit did not commit the exec (transaction status: {commitStatus}). "
                            + "Nothing was written; Revit's failure processing rejected the change"
                            + (preprocessor.RolledBackForErrors
                                ? ": " + string.Join("; ", preprocessor.Errors)
                                : "."));
                    }
                }
                catch
                {
                    // Only roll back a transaction still in our hands. HasEnded() covers
                    // a commit that already resolved to RolledBack, and a Pending one
                    // belongs to Revit — rolling either back here throws on top of the
                    // real fault.
                    if (!leftWithRevit && tx.HasStarted() && !tx.HasEnded()
                        && tx.GetStatus() != TransactionStatus.Pending) tx.RollBack();
                    throw;
                }
                finally
                {
                    // Disposing while Pending would roll back the very edit Revit is
                    // finalizing. Leaking one Transaction is the strictly better failure:
                    // Revit blocks all further writes until it resolves anyway.
                    if (!leftWithRevit) tx.Dispose();
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
                Warnings    = revitWarnings.Count > 0 ? revitWarnings : null,
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
