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
                try
                {
                    result = script.RunAsync(globals).GetAwaiter().GetResult().ReturnValue;
                    tx.Commit();
                }
                catch
                {
                    if (tx.HasStarted()) tx.RollBack();
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
