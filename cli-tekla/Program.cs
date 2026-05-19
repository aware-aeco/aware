// aware-tekla — Tekla Open API sidecar for the AWARE runtime.
// Spike v0.29: send-status verb only, single-instance Tekla, no ROT binding.
// ROT-bind multi-instance precise routing lands in the hardening pass.
//
// Contract: receives JSON on stdin, emits JSON receipt on stdout, exits 0
// on success per the desktop-host-sidecar-spec. Spawned by `aware app run`
// via cli/src/runtime/invoker.rs CliInvoker.

using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Runtime.CompilerServices;
using System.Text.Json;
using System.Text.Json.Nodes;
using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.CSharp.Scripting;
using Microsoft.CodeAnalysis.Scripting;

namespace AwareTekla;

// Roslyn-script globals — exposed to the user's C# snippet as top-level
// identifiers `model` and `args`. MUST be public (and non-nested) so the
// dynamically-generated Submission#N type that Roslyn produces can access
// it from a different assembly; otherwise we hit TypeAccessException at
// script runtime.
public sealed class ExecGlobals
{
    // `dynamic` so the script can call methods on `model` without
    // cli-tekla itself needing a static reference to Tekla.Structures.Model.
    // The cost is DLR dispatch overhead per call — negligible vs the
    // surrounding Tekla operations.
    public dynamic? model = null;
    public IDictionary<string, object?> args = new Dictionary<string, object?>();
}

internal static class Program
{
    static int Main(string[] args)
    {
        // Force UTF-8 on stdin/stdout — .NET Framework defaults to the
        // Windows OEM codepage which mangles em-dashes, smart quotes,
        // accented chars etc when JSON travels in either direction.
        Console.InputEncoding  = new System.Text.UTF8Encoding(encoderShouldEmitUTF8Identifier: false);
        Console.OutputEncoding = new System.Text.UTF8Encoding(encoderShouldEmitUTF8Identifier: false);

        try
        {
            return Run(args);
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"aware-tekla: unhandled error: {ex.Message}");
            Console.Error.WriteLine(ex.StackTrace ?? "");
            return 2;
        }
    }

    static int Run(string[] args)
    {
        if (args.Length == 0 || args[0] == "--help" || args[0] == "-h")
        {
            PrintHelp();
            return 0;
        }

        // Two invocation styles are supported:
        //   aware-tekla.exe <verb> [flags] [--json-stdin]   (canonical)
        //   aware-tekla.exe --json-stdin                    (verb embedded in JSON body)
        // The second exists so AI orchestrators can ship a single JSON
        // payload that carries everything (verb, version, code, args) —
        // useful when the only CLI knob needed is `--json-stdin` itself.
        string verb;
        if (args[0].StartsWith("--", StringComparison.Ordinal))
        {
            string buf;
            try { buf = Console.In.ReadToEnd(); }
            catch (Exception e)
            {
                Console.Error.WriteLine($"aware-tekla: stdin not readable: {e.Message}");
                return 2;
            }
            JsonNode? peeked;
            try { peeked = JsonNode.Parse(buf); }
            catch (Exception e)
            {
                Console.Error.WriteLine($"aware-tekla: stdin not JSON (when verb omitted on CLI): {e.Message}");
                return 2;
            }
            verb = peeked?["verb"]?.GetValue<string>() ?? "";
            if (string.IsNullOrEmpty(verb))
            {
                Console.Error.WriteLine(
                    "aware-tekla: stdin JSON has no `verb` field, and no verb was passed on the CLI");
                return 2;
            }
            // Re-bind Console.In to a fresh reader holding the same buffer
            // so the verb handler downstream can read it normally.
            Console.SetIn(new StringReader(buf));
        }
        else
        {
            verb = args[0];
            args = args.Skip(1).ToArray();
        }

        var parsed = ParseArgs(args);
        // If --json-stdin wasn't passed explicitly but we took the embedded-
        // verb path above, the body still came through stdin so the flag is
        // implied. The downstream verb handlers check parsed.JsonStdin.
        if (!parsed.JsonStdin && verb == "exec")
        {
            // Exec is always stdin-driven; force the flag if the caller used
            // the embedded-verb style.
            parsed.JsonStdin = true;
        }

        switch (verb)
        {
            case "send-status":
                return SendStatus(parsed);
            case "list-instances":
                return ListInstances();
            case "launch":
                return Launch(parsed);
            case "close":
                return Close(parsed);
            case "exec":
                return Exec(parsed);
            default:
                Console.Error.WriteLine($"aware-tekla: unknown verb '{verb}'. Try --help.");
                return 2;
        }
    }

    // ── close ────────────────────────────────────────────────────────────────
    // Shut Tekla down CLEANLY by default — save via Open API, wait for the
    // ModelSave event to confirm bytes on disk, then CloseMainWindow.
    // Force-kill is gated behind an explicit acknowledgement flag.
    static int Close(ParsedArgs args)
    {
        bool acknowledgeDataLoss = false;
        if (args.JsonStdin)
        {
            try
            {
                var input = JsonNode.Parse(Console.In.ReadToEnd());
                acknowledgeDataLoss = input?["force"]?.GetValue<bool>() ?? false;
                if (string.IsNullOrEmpty(args.Version))
                    args.Version = input?["version"]?.GetValue<string>();
            }
            catch { /* stdin optional for close */ }
        }

        var allInstances = EnumerateRunningTeklas();
        var targets = FilterTargets(allInstances, args);
        if (targets.Count == 0)
        {
            Console.Error.WriteLine(allInstances.Count == 0
                ? "aware-tekla: no Tekla instance running (nothing to close)"
                : $"aware-tekla: requested target not running (found: {string.Join(", ", allInstances.Select(t => t.Version))})");
            return 1;
        }
        if (targets.Count > 1 && !args.All)
        {
            Console.Error.WriteLine(
                $"aware-tekla: ambiguous target ({targets.Count} matches). Use --pid <N> or --all.");
            return 4;
        }

        var results = new JsonArray();
        int failed = 0;
        foreach (var t in targets)
        {
            try
            {
                CloseOne(t, acknowledgeDataLoss);
                results.Add(new JsonObject
                {
                    ["status"]       = "ok",
                    ["host_pid"]     = t.Pid,
                    ["host_version"] = t.Version,
                    ["mode"]         = acknowledgeDataLoss ? "force" : "clean",
                });
            }
            catch (Exception e)
            {
                var root = e;
                while (root is System.Reflection.TargetInvocationException && root.InnerException is not null)
                    root = root.InnerException;
                failed++;
                results.Add(new JsonObject
                {
                    ["status"]       = "err",
                    ["host_pid"]     = t.Pid,
                    ["host_version"] = t.Version,
                    ["error"]        = root.Message,
                });
            }
        }
        var receipt = new JsonObject
        {
            ["status"]   = failed == 0 ? "ok" : (failed == targets.Count ? "err" : "partial"),
            ["host"]     = "tekla",
            ["verb"]     = "close",
            ["targets"]  = results,
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
        Console.WriteLine(receipt.ToJsonString());
        return failed == 0 ? 0 : 2;
    }

    static void CloseOne(TeklaInstance target, bool acknowledgeDataLoss)
    {
        var binDir = Path.GetDirectoryName(target.ExePath)!;
        var modelDllPath = Path.Combine(binDir, "Tekla.Structures.Model.dll");

        // Wire the AssemblyResolve handler (same pattern as send-status).
        if (!_resolverWired)
        {
            var probePaths = new[] { Path.Combine(binDir, "Net48Runtime"), binDir };
            AppDomain.CurrentDomain.AssemblyResolve += (sender, eventArgs) =>
            {
                try
                {
                    var name = new AssemblyName(eventArgs.Name).Name;
                    if (string.IsNullOrEmpty(name)) return null;
                    foreach (var probe in probePaths)
                    {
                        var candidate = Path.Combine(probe, $"{name}.dll");
                        if (File.Exists(candidate)) return Assembly.LoadFrom(candidate);
                    }
                    return null;
                }
                catch { return null; }
            };
            _resolverWired = true;
        }

        var originalCwd = Environment.CurrentDirectory;
        Environment.CurrentDirectory = binDir;
        try
        {
            CloseInner(modelDllPath, target.Pid, acknowledgeDataLoss);
        }
        finally
        {
            Environment.CurrentDirectory = originalCwd;
        }
    }

    static void CloseInner(string modelDllPath, int pid, bool acknowledgeDataLoss)
    {
        // Preload Tekla.Structures.* siblings same as send-status path.
        var binDir = Path.GetDirectoryName(modelDllPath)!;
        var probePaths = new[] { Path.Combine(binDir, "Net48Runtime"), binDir };
        foreach (var name in new[] { "Tekla.Structures.dll", "Tekla.Structures.Datatype.dll", "Tekla.Structures.Model.dll" })
        {
            foreach (var probe in probePaths)
            {
                var p = Path.Combine(probe, name);
                if (File.Exists(p)) { Assembly.LoadFrom(p); break; }
            }
        }

        var modelAsm = AppDomain.CurrentDomain.GetAssemblies()
            .FirstOrDefault(a => a.GetName().Name == "Tekla.Structures.Model")
            ?? Assembly.LoadFrom(modelDllPath);

        var modelType = modelAsm.GetType("Tekla.Structures.Model.Model")!;
        var modelInstance = Activator.CreateInstance(modelType)!;
        var getConnStatus = modelType.GetMethod("GetConnectionStatus");
        var connected = (bool)(getConnStatus!.Invoke(modelInstance, null) ?? false);

        if (!connected)
        {
            // Open API can't attach — clean save impossible. Either force-kill
            // (if explicitly acknowledged) or refuse.
            if (acknowledgeDataLoss)
            {
                Process.GetProcessById(pid).Kill();
                Process.GetProcessById(pid).WaitForExit(15_000);
                return;
            }
            throw new InvalidOperationException(
                "Open API not reachable for clean save. Re-issue with `force=true` to force-kill " +
                "(WILL LOSE UNSAVED MODEL STATE).");
        }

        // Commit + Save with ModelSave-event wait. Don't trust Model.Save's return
        // value as confirmation — the actual disk write is async on Tekla's
        // worker thread.
        var commitChanges = modelType.GetMethod("CommitChanges", Type.EmptyTypes);
        commitChanges?.Invoke(modelInstance, null);

        var eventsType = modelAsm.GetType("Tekla.Structures.Model.Events")!;
        var eventsInstance = Activator.CreateInstance(eventsType)!;
        _modelSaveSignal = new System.Threading.ManualResetEventSlim(false);
        _teklaExitSignal = new System.Threading.ManualResetEventSlim(false);

        // Subscribe to BOTH signals we need: ModelSave (save completed) and
        // TeklaStructuresExit (app shutdown started — inside-Tekla signal).
        var modelSaveEvent = eventsType.GetEvent("ModelSave");
        var teklaExitEvent = eventsType.GetEvent("TeklaStructuresExit");
        Delegate? saveHandler = BuildDynamicHandler(modelSaveEvent, nameof(SignalModelSave));
        Delegate? exitHandler = BuildDynamicHandler(teklaExitEvent, nameof(SignalTeklaExit));
        if (saveHandler != null) modelSaveEvent!.AddEventHandler(eventsInstance, saveHandler);
        if (exitHandler != null) teklaExitEvent!.AddEventHandler(eventsInstance, exitHandler);
        eventsType.GetMethod("Register")?.Invoke(eventsInstance, null);

        try
        {
            // Trigger save. Save() returns false when there's nothing to save
            // (model is already clean) — that's not an error, just means the
            // ModelSave event won't fire. Either way we proceed to close.
            var saveMethod = modelType.GetMethod("Save", Type.EmptyTypes);
            var saveResult = saveMethod?.Invoke(modelInstance, null);
            bool savedSomething = saveResult is bool b && b;

            if (savedSomething && modelSaveEvent != null)
            {
                // Bounded wait for ModelSave-event confirmation. Small / blank
                // models save in <1s; production can take longer. 60s ceiling.
                if (!_modelSaveSignal.Wait(TimeSpan.FromSeconds(60)))
                    throw new TimeoutException(
                        "ModelSave event did not fire within 60s — save may still be in progress");
            }
            // If savedSomething=false the model was already clean on disk;
            // no event will fire and we can close immediately.
        }
        finally
        {
            if (modelSaveEvent != null && saveHandler != null)
                modelSaveEvent.RemoveEventHandler(eventsInstance, saveHandler);
            if (teklaExitEvent != null && exitHandler != null)
                teklaExitEvent.RemoveEventHandler(eventsInstance, exitHandler);
            eventsType.GetMethod("UnRegister")?.Invoke(eventsInstance, null);
            _modelSaveSignal = null;
            _teklaExitSignal = null;
        }

        // Tekla refuses external WM_CLOSE (CloseMainWindow is a no-op for the
        // app-as-a-whole). Use Tekla's own Open-API exit hook instead.
        // Dump Operation methods so we can pick the right one — done once
        // for diagnostics, kept here as a comment for future maintainers:
        //   Operation.ExitTekla()  — closes the running Tekla application
        //   Operation.CloseModel() — closes the current model (model-level)
        // Either may exist depending on the Tekla version; we try Exit
        // variants first and fall back to CloseModel + WM_CLOSE.

        // Wider diagnostic — search every loaded Tekla.* assembly for any
        // type with an exit/close/quit/shutdown static method.
        Console.Error.WriteLine("aware-tekla close: wide scan of Tekla.* assemblies for exit-shaped methods:");
        foreach (var asm in AppDomain.CurrentDomain.GetAssemblies()
                     .Where(a => a.GetName().Name?.StartsWith("Tekla.") == true))
        {
            Type[] types;
            try { types = asm.GetExportedTypes(); }
            catch { continue; }
            foreach (var t in types)
            {
                MethodInfo[] methods;
                try { methods = t.GetMethods(BindingFlags.Public | BindingFlags.Static); }
                catch { continue; }
                foreach (var m in methods)
                {
                    var n = m.Name.ToLowerInvariant();
                    if ((n.Contains("exit") || n.Contains("quit") || n.Contains("shut")
                         || n == "close" || n.StartsWith("closetekla"))
                        && m.GetParameters().Length <= 1)
                    {
                        var ps = string.Join(", ", m.GetParameters().Select(p => p.ParameterType.Name));
                        Console.Error.WriteLine($"   {t.FullName}.{m.Name}({ps})");
                    }
                }
            }
        }

        var operationType = modelAsm.GetType("Tekla.Structures.Model.Operations.Operation");
        bool exitTriggered = false;
        string? methodUsed = null;
        if (operationType != null)
        {
            foreach (var methodName in new[] { "ExitTekla", "Exit", "CloseTekla", "QuitTekla" })
            {
                var m = operationType.GetMethod(methodName, BindingFlags.Public | BindingFlags.Static, null, Type.EmptyTypes, null);
                if (m != null)
                {
                    methodUsed = methodName;
                    try
                    {
                        m.Invoke(null, null);
                        exitTriggered = true;
                    }
                    catch (Exception e)
                    {
                        // Trimble.Remoting commonly throws here because Tekla
                        // shut down the IPC channel mid-call. The exit DID start —
                        // the channel just died before the call returned. We
                        // confirm via Process.HasExited below regardless.
                        var root = e;
                        while (root is System.Reflection.TargetInvocationException && root.InnerException is not null)
                            root = root.InnerException;
                        Console.Error.WriteLine(
                            $"aware-tekla close: Operation.{methodName}() threw {root.GetType().Name} " +
                            $"(this is expected — IPC channel dies mid-call): {root.Message}");
                        exitTriggered = true;  // assume it started
                    }
                    break;
                }
            }
        }

        if (!exitTriggered)
        {
            // No Operation exit method found — last resort.
            Process.GetProcessById(pid).CloseMainWindow();
        }

        // Dual-signal wait: TeklaStructuresExit event (inside-process) OR
        // Process.HasExited (OS-level). Whichever fires first wins.
        var pTarget = Process.GetProcessById(pid);
        var sw = System.Diagnostics.Stopwatch.StartNew();
        while (sw.Elapsed.TotalSeconds < 60)
        {
            if ((_teklaExitSignal?.IsSet ?? false) || pTarget.HasExited) return;
            System.Threading.Thread.Sleep(500);
        }
        throw new TimeoutException(
            $"Tekla did not exit within 60s of Operation.{methodUsed ?? "(none-found)"}(). " +
            "Pass `force=true` if you accept losing any unsaved view state.");
    }

    // ── exec ─────────────────────────────────────────────────────────────────
    // Compile + run an ad-hoc C# script against the live Tekla model using
    // Roslyn scripting (Microsoft.CodeAnalysis.CSharp.Scripting). The bridge
    // between the v0.30 catalog (data) and the live Tekla host (execution).
    //
    // The orchestrator (AI) reads the catalog at
    // 20-agents/aeco/engineering/tekla-2026/catalog/Tekla.Structures.*.json,
    // drafts a C# snippet, and ships it through this verb. The script is
    // compiled against the resolved Tekla.Structures.*.dll assemblies, run
    // with a `Globals` object exposing `model` (a Tekla Model instance) and
    // `args` (the args block from input JSON), and the return value is
    // serialized back as JSON.
    //
    // Transaction semantics: on script success we call `model.CommitChanges()`
    // (Tekla's equivalent of a commit — there is no rollback API; changes
    // simply stay in working memory until the next CommitChanges). On script
    // exception we DO NOT call CommitChanges, so partial in-memory state is
    // not flushed to the database. The next CommitChanges from a subsequent
    // call overwrites it.
    static int Exec(ParsedArgs args)
    {
        if (!args.JsonStdin)
        {
            EmitExecError("exec requires --json-stdin (or pass the request JSON via stdin)");
            return 2;
        }

        string stdin = Console.In.ReadToEnd();
        JsonNode? input;
        try { input = JsonNode.Parse(stdin); }
        catch (Exception e) { EmitExecError($"stdin not JSON: {e.Message}"); return 2; }

        string? code     = input?["code"]?.GetValue<string>();
        string? version  = input?["version"]?.GetValue<string>() ?? args.Version;
        var argsNode     = input?["args"] as JsonObject;
        if (string.IsNullOrEmpty(code))
        {
            EmitExecError("missing required field: code (the C# script to compile + run)");
            return 2;
        }

        // Find the running Tekla instance (if any) to populate host_pid and
        // host_version in the receipt — matches the cli-rhino receipt shape so
        // downstream orchestrators see a consistent envelope across vendors.
        // Best-effort: if no Tekla is running (smoke-test path), pid stays null.
        int? hostPid = null;
        string? hostVersion = version;
        try
        {
            var instances = EnumerateRunningTeklas();
            TeklaInstance? match = null;
            if (!string.IsNullOrEmpty(version))
            {
                match = instances.FirstOrDefault(i => i.Version == version);
            }
            match ??= instances.FirstOrDefault();
            if (match is not null)
            {
                hostPid = match.Pid;
                hostVersion = match.Version;
            }
        }
        catch { /* enumeration failure is non-fatal; pid stays null */ }

        // Resolve Tekla install dir for the requested version (registry +
        // standard path). Missing-install is non-fatal: the script may not
        // reference Tekla types (smoke-test path returns primitives).
        string? hostInstall = string.IsNullOrEmpty(version) ? null : DiscoverTeklaInstall(version!);
        var (probedReferences, probedDir) = ResolveTeklaReferences(hostInstall);

        // Wire AssemblyResolve so Roslyn can load Tekla DLLs at script-runtime
        // (the references list tells Roslyn at compile-time which assemblies
        // to bind against, but the .NET loader needs the resolver at runtime
        // for transitive Tekla.* dependencies).
        if (!_resolverWired && probedDir is not null)
        {
            var probePaths = new[] { Path.Combine(probedDir, "Net48Runtime"), probedDir };
            AppDomain.CurrentDomain.AssemblyResolve += (sender, eventArgs) =>
            {
                try
                {
                    var name = new AssemblyName(eventArgs.Name).Name;
                    if (string.IsNullOrEmpty(name)) return null;
                    foreach (var probe in probePaths)
                    {
                        var candidate = Path.Combine(probe, $"{name}.dll");
                        if (File.Exists(candidate)) return Assembly.LoadFrom(candidate);
                    }
                    return null;
                }
                catch { return null; }
            };
            _resolverWired = true;
        }

        // Run the actual Roslyn compile+execute in a no-inline method so the
        // JIT can't pre-resolve Tekla types before AssemblyResolve is wired.
        // (See skills/runtime-bridge-dotnet-framework.md for why this matters
        // for any code that touches Tekla types.)
        try
        {
            var result = RunScript(code!, probedReferences, argsNode, probedDir);
            EmitExecOk(result, hostVersion, hostPid);
            return 0;
        }
        catch (CompilationErrorException ce)
        {
            // Script failed to compile — surface diagnostics so the caller
            // (likely an AI) can re-draft.
            var diagnostics = string.Join("\n", ce.Diagnostics.Select(d => d.ToString()));
            EmitExecFail($"compile error: {ce.Message}", diagnostics, hostVersion, hostPid);
            return 2;
        }
        catch (Exception e)
        {
            var root = e;
            while (root is TargetInvocationException && root.InnerException is not null)
                root = root.InnerException;
            EmitExecFail(root.GetType().Name + ": " + root.Message, root.StackTrace ?? "",
                         hostVersion, hostPid);
            return 2;
        }
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    static object? RunScript(
        string code,
        IReadOnlyList<MetadataReference> teklaReferences,
        JsonObject? argsNode,
        string? teklaBinDir)
    {
        // Standard usings — enough for catalog-style snippets to stay
        // boilerplate-free. The script writer can add `using ...;` lines of
        // their own at the top of `code` if they need more.
        var imports = new List<string>
        {
            "System",
            "System.Collections.Generic",
            "System.Linq",
        };

        // Tekla usings layer on top — only included when references resolved.
        if (teklaReferences.Count > 0)
        {
            imports.Add("Tekla.Structures");
            imports.Add("Tekla.Structures.Model");
            imports.Add("Tekla.Structures.Model.Operations");
            imports.Add("Tekla.Structures.Geometry3d");
            imports.Add("Tekla.Structures.Drawing");
            imports.Add("Tekla.Structures.Datatype");
        }

        // Build references: BCL essentials + Tekla assemblies (if found).
        var refs = new List<MetadataReference>
        {
            MetadataReference.CreateFromFile(typeof(object).Assembly.Location),       // mscorlib / System.Private.CoreLib
            MetadataReference.CreateFromFile(typeof(System.Linq.Enumerable).Assembly.Location),
            MetadataReference.CreateFromFile(typeof(System.Collections.Generic.IDictionary<,>).Assembly.Location),
            MetadataReference.CreateFromFile(typeof(System.Dynamic.DynamicObject).Assembly.Location),  // System.Core.dll
            MetadataReference.CreateFromFile(typeof(Microsoft.CSharp.RuntimeBinder.Binder).Assembly.Location), // Microsoft.CSharp.dll for `dynamic`
        };
        // De-dup by file path — typeof(...).Assembly.Location can return the
        // same DLL twice on net48 (e.g. mscorlib + System.Collections both
        // live in mscorlib).
        refs = refs
            .GroupBy(r => (r as PortableExecutableReference)?.FilePath ?? Guid.NewGuid().ToString())
            .Select(g => g.First())
            .ToList();
        refs.AddRange(teklaReferences);

        var options = ScriptOptions.Default
            .WithReferences(refs)
            .WithImports(imports)
            // Allow `await` at top level — handy for snippets that need it.
            // Roslyn scripts default to non-async but enabling allows both.
            .WithEmitDebugInformation(false);

        // Construct the Tekla Model lazily. If teklaBinDir is null OR Tekla
        // isn't running, the constructor either throws or returns a model
        // with GetConnectionStatus()==false — neither is fatal here: a
        // smoke-test script (`return 1+2;`) never touches `model`. We set
        // model=null and let the script blow up at its own dynamic call site
        // if it tries to use Tekla without a live host.
        object? modelInstance = null;
        if (teklaBinDir is not null)
        {
            try
            {
                modelInstance = ConstructTeklaModel(teklaBinDir);
            }
            catch (Exception e)
            {
                // Don't kill exec — let smoke tests still run. Surface the
                // failure on stderr for diagnostic purposes.
                Console.Error.WriteLine(
                    $"aware-tekla exec: could not construct Tekla.Structures.Model.Model — " +
                    $"{e.GetType().Name}: {e.Message} (smoke-test scripts that don't use `model` still work)");
            }
        }

        // Globals — exposed to the script as top-level identifiers `model`
        // and `args`. `model` is dynamic so the script can call any method
        // on it via DLR without us needing a compile-time Tekla reference.
        var argsDict = JsonObjectToDictionary(argsNode);
        var globals = new ExecGlobals { model = modelInstance, args = argsDict };

        var script = CSharpScript.Create<object>(
            code,
            options: options,
            globalsType: typeof(ExecGlobals));

        // Compile + execute.
        var state = script.RunAsync(globals).GetAwaiter().GetResult();
        var returnValue = state.ReturnValue;

        // Tekla "transaction-commit" — flush changes to the database. We
        // only do this if a real Model was constructed AND the connection is
        // live (Tekla running, model open). Without the connection check
        // CommitChanges throws into the dead remoting channel.
        if (modelInstance is not null)
        {
            var modelType = modelInstance.GetType();
            var getStatus = modelType.GetMethod("GetConnectionStatus");
            var connected = (bool)(getStatus?.Invoke(modelInstance, null) ?? false);
            if (connected)
            {
                try
                {
                    var commit = modelType.GetMethod("CommitChanges", Type.EmptyTypes);
                    commit?.Invoke(modelInstance, null);
                }
                catch (Exception ce)
                {
                    // CommitChanges failed — bubble up so we don't silently
                    // lose the user's work in the receipt.
                    var root = ce;
                    while (root is TargetInvocationException && root.InnerException is not null)
                        root = root.InnerException;
                    throw new InvalidOperationException(
                        $"Tekla Model.CommitChanges() failed after script success: {root.Message}", root);
                }
            }
        }

        return returnValue;
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    static object ConstructTeklaModel(string teklaBinDir)
    {
        // Pre-load Tekla.Structures.*.dll in dependency order — see
        // DispatchSendStatusInner for the rationale (loader cache failures).
        var probePaths = new[] { Path.Combine(teklaBinDir, "Net48Runtime"), teklaBinDir };
        foreach (var name in new[] {
            "Tekla.Structures.dll",
            "Tekla.Structures.Datatype.dll",
            "Tekla.Structures.Model.dll",
        })
        {
            foreach (var probe in probePaths)
            {
                var p = Path.Combine(probe, name);
                if (File.Exists(p)) { Assembly.LoadFrom(p); break; }
            }
        }

        var modelAsm = AppDomain.CurrentDomain.GetAssemblies()
            .FirstOrDefault(a => a.GetName().Name == "Tekla.Structures.Model")
            ?? throw new InvalidOperationException(
                "Tekla.Structures.Model.dll could not be loaded — is the requested Tekla version installed?");
        var modelType = modelAsm.GetType("Tekla.Structures.Model.Model")
            ?? throw new InvalidOperationException("Tekla.Structures.Model.Model type not found");

        var modelInstance = Activator.CreateInstance(modelType)
            ?? throw new InvalidOperationException("Could not construct Model() instance");

        // Verify the connection — same pattern as send-status. If Tekla
        // isn't running, GetConnectionStatus returns false; we still
        // return the Model so the script CAN reference Tekla types (just
        // not call live methods).
        var getStatus = modelType.GetMethod("GetConnectionStatus");
        var connected = (bool)(getStatus?.Invoke(modelInstance, null) ?? false);
        if (!connected)
        {
            Console.Error.WriteLine(
                "aware-tekla exec: Tekla.Structures.Model.Model() constructed but " +
                "GetConnectionStatus()==false. Tekla isn't running or no model is open. " +
                "Scripts that only reference types (not state) will still work.");
        }

        return modelInstance;
    }

    static (IReadOnlyList<MetadataReference> refs, string? probedDir) ResolveTeklaReferences(string? hostInstall)
    {
        if (hostInstall is null) return (Array.Empty<MetadataReference>(), null);

        // Honour the manifest's dll-probe-paths: Net48Runtime first, then bin.
        var binDir = Path.Combine(hostInstall, "bin");
        if (!Directory.Exists(binDir))
            return (Array.Empty<MetadataReference>(), null);

        var probePaths = new[] { Path.Combine(binDir, "Net48Runtime"), binDir };
        var wanted = new[]
        {
            "Tekla.Structures.dll",
            "Tekla.Structures.Model.dll",
            "Tekla.Structures.Datatype.dll",
            "Tekla.Structures.Drawing.dll",
            "Tekla.Structures.Geometry3d.Compatibility.dll",
        };

        var found = new List<MetadataReference>();
        var seen = new HashSet<string>(StringComparer.OrdinalIgnoreCase);
        foreach (var name in wanted)
        {
            foreach (var probe in probePaths)
            {
                var p = Path.Combine(probe, name);
                if (File.Exists(p) && seen.Add(name))
                {
                    found.Add(MetadataReference.CreateFromFile(p));
                    break;
                }
            }
        }

        return (found, binDir);
    }

    static string? DiscoverTeklaInstall(string version)
    {
        // Standard path first — Tekla's installer always puts builds here.
        var stdPath = $@"C:\Program Files\Tekla Structures\{version}";
        if (Directory.Exists(stdPath)) return stdPath;

        // Registry fallback — Tekla writes Install Folder per version.
        try
        {
            using var key = Microsoft.Win32.Registry.LocalMachine.OpenSubKey(
                $@"SOFTWARE\Tekla Structures\{version}");
            var installFolder = key?.GetValue("Install Folder") as string;
            if (!string.IsNullOrEmpty(installFolder) && Directory.Exists(installFolder))
                return installFolder;
        }
        catch { /* registry errors → null below */ }

        // WOW6432Node fallback for 32-bit installer entries on 64-bit Windows.
        try
        {
            using var key = Microsoft.Win32.Registry.LocalMachine.OpenSubKey(
                $@"SOFTWARE\WOW6432Node\Tekla Structures\{version}");
            var installFolder = key?.GetValue("Install Folder") as string;
            if (!string.IsNullOrEmpty(installFolder) && Directory.Exists(installFolder))
                return installFolder;
        }
        catch { /* registry errors → null below */ }

        return null;
    }

    static Dictionary<string, object?> JsonObjectToDictionary(JsonObject? obj)
    {
        var dict = new Dictionary<string, object?>(StringComparer.Ordinal);
        if (obj is null) return dict;
        foreach (var kvp in obj)
        {
            dict[kvp.Key] = JsonNodeToObject(kvp.Value);
        }
        return dict;
    }

    static object? JsonNodeToObject(JsonNode? node)
    {
        if (node is null) return null;
        if (node is JsonValue v)
        {
            // Try common primitive shapes; fall back to string.
            if (v.TryGetValue<bool>(out var b)) return b;
            if (v.TryGetValue<int>(out var i)) return i;
            if (v.TryGetValue<long>(out var l)) return l;
            if (v.TryGetValue<double>(out var d)) return d;
            if (v.TryGetValue<string>(out var s)) return s;
            return v.ToString();
        }
        if (node is JsonArray arr)
        {
            var list = new List<object?>();
            foreach (var item in arr) list.Add(JsonNodeToObject(item));
            return list;
        }
        if (node is JsonObject jo) return JsonObjectToDictionary(jo);
        return null;
    }

    static void EmitExecOk(object? result, string? hostVersion = null, int? hostPid = null)
    {
        var receipt = new JsonObject
        {
            ["ok"] = true,
            ["result"] = SerializeResult(result),
            ["host"] = "tekla",
            ["host_version"] = hostVersion,
            ["host_pid"] = hostPid,
            ["verb"] = "exec",
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
        Console.WriteLine(receipt.ToJsonString());
    }

    static void EmitExecFail(string message, string stack, string? hostVersion = null, int? hostPid = null)
    {
        var receipt = new JsonObject
        {
            ["ok"] = false,
            ["error"] = message,
            ["stack"] = stack,
            ["host"] = "tekla",
            ["host_version"] = hostVersion,
            ["host_pid"] = hostPid,
            ["verb"] = "exec",
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
        Console.WriteLine(receipt.ToJsonString());
    }

    static void EmitExecError(string message)
    {
        Console.Error.WriteLine($"aware-tekla exec: {message}");
        Console.Error.WriteLine();
        Console.Error.WriteLine("Required stdin JSON shape:");
        Console.Error.WriteLine("  { \"verb\": \"exec\", \"version\": \"2026.0\",");
        Console.Error.WriteLine("    \"code\": \"return 1+2;\", \"args\": {} }");
        Console.Error.WriteLine();
        Console.Error.WriteLine("`code` is a C# snippet with optional `return X;`. Globals:");
        Console.Error.WriteLine("  dynamic model               // Tekla.Structures.Model.Model (or null)");
        Console.Error.WriteLine("  IDictionary<string,object> args // input args block");
    }

    static JsonNode? SerializeResult(object? result)
    {
        // Defensive serializer — Tekla types are complex (often containing
        // COM proxies or self-referencing children), so we can't trust
        // System.Text.Json's reflection-based default to handle every input.
        if (result is null) return null;

        // Common primitives — emit directly.
        switch (result)
        {
            case bool b:   return JsonValue.Create(b);
            case int i:    return JsonValue.Create(i);
            case long l:   return JsonValue.Create(l);
            case double d: return JsonValue.Create(d);
            case float f:  return JsonValue.Create((double)f);
            case decimal m:return JsonValue.Create(m);
            case string s: return JsonValue.Create(s);
            case Guid g:   return JsonValue.Create(g.ToString());
            case DateTime dt: return JsonValue.Create(dt.ToString("o"));
        }

        // IDictionary<string,*> — preserved as JSON object.
        if (result is System.Collections.IDictionary id)
        {
            var jo = new JsonObject();
            foreach (System.Collections.DictionaryEntry kvp in id)
            {
                jo[kvp.Key?.ToString() ?? ""] = SerializeResult(kvp.Value);
            }
            return jo;
        }

        // IEnumerable (but not string — caught above) — JSON array.
        if (result is System.Collections.IEnumerable e and not string)
        {
            var ja = new JsonArray();
            foreach (var item in e) ja.Add(SerializeResult(item));
            return ja;
        }

        // Tekla.Structures.Identifier has a GUID field — common return shape.
        var t = result.GetType();
        var guidProp = t.GetProperty("GUID");
        if (guidProp is not null)
        {
            var g = guidProp.GetValue(result);
            if (g is not null) return JsonValue.Create(g.ToString());
        }

        // Last resort: ToString() with the type name as a hint. Better than
        // a serialization explosion on COM proxies.
        try
        {
            // Try System.Text.Json first for plain POCOs / records — bounded
            // depth so we don't blow up on cyclic graphs.
            var json = JsonSerializer.Serialize(result, new JsonSerializerOptions
            {
                MaxDepth = 6,
                ReferenceHandler = System.Text.Json.Serialization.ReferenceHandler.IgnoreCycles,
            });
            return JsonNode.Parse(json);
        }
        catch
        {
            return JsonValue.Create($"{t.FullName}: {result}");
        }
    }


    // ── launch ───────────────────────────────────────────────────────────────
    // Spawn a Tekla instance using the headless-startup pattern. Reads
    // optional args from stdin JSON: version, model_path, bypass_ini,
    // license, environment, role. Anything not supplied falls back to a
    // sensible default OR returns a clear "you must supply X" error.
    //
    // Does NOT wait for Open API readiness — that's caller's job, polling
    // send-status. Returns immediately after Tekla process starts so the
    // caller can decide its own readiness deadline.
    static int Launch(ParsedArgs args)
    {
        if (!args.JsonStdin)
        {
            Console.Error.WriteLine("aware-tekla: launch requires --json-stdin");
            return 2;
        }
        string stdin = Console.In.ReadToEnd();
        JsonNode? input;
        try { input = JsonNode.Parse(stdin); }
        catch (Exception e) { Console.Error.WriteLine($"aware-tekla: stdin not JSON: {e.Message}"); return 2; }

        string? version     = input?["version"]?.GetValue<string>() ?? args.Version;
        string? modelPath   = input?["model_path"]?.GetValue<string>();
        string? bypassIni   = input?["bypass_ini"]?.GetValue<string>();
        string? envName     = input?["environment"]?.GetValue<string>();
        string? roleFile    = input?["role"]?.GetValue<string>();
        string? license     = input?["license"]?.GetValue<string>();

        if (string.IsNullOrEmpty(version))
        {
            EmitGuide("missing required field: version (e.g. \"2026.0\")");
            return 2;
        }

        var teklaExe = $@"C:\Program Files\Tekla Structures\{version}\bin\TeklaStructures.exe";
        if (!File.Exists(teklaExe))
        {
            EmitGuide($"Tekla {version} is not installed at {teklaExe}. " +
                      "Install it or pick a version that is. " +
                      "Run `aware-tekla list-instances` to see installed versions.");
            return 3;
        }

        // If caller supplies a bypass_ini path, use it as-is. Else auto-build
        // one from environment + role + license fields.
        if (string.IsNullOrEmpty(bypassIni))
        {
            if (string.IsNullOrEmpty(envName) || string.IsNullOrEmpty(license))
            {
                EmitGuide(
                    "missing fields: when bypass_ini is not provided, you must supply " +
                    "`environment` (e.g. \"blank_project\", \"default\", \"uk\") and " +
                    "`license` (e.g. \"Partner\", \"DIAMOND\", \"FULL\"). " +
                    "Optionally `role` (defaults to role_Engineer.ini in default env). " +
                    "See skills/headless-startup-and-shutdown.md for full reference.");
                return 2;
            }
            var rolePart = string.IsNullOrEmpty(roleFile)
                ? @"%XSDATADIR%\Environments\default\role_Engineer.ini"
                : roleFile!;
            var iniContent =
                "rem AWARE-generated Bypass.ini\r\n" +
                $"set XS_DEFAULT_ENVIRONMENT=%XSDATADIR%\\Environments\\{envName}\\env_{envName}.ini\r\n" +
                $"set XS_DEFAULT_ROLE={rolePart}\r\n" +
                $"set XS_DEFAULT_LICENSE={license}\r\n";
            var tempDir = Path.Combine(Path.GetTempPath(), "aware-tekla");
            Directory.CreateDirectory(tempDir);
            bypassIni = Path.Combine(tempDir, $"Bypass-{version}.ini");
            File.WriteAllText(bypassIni, iniContent);
        }

        var argList = $"-I \"{bypassIni}\"";
        if (!string.IsNullOrEmpty(modelPath))
            argList += $" \"{modelPath}\"";

        var psi = new ProcessStartInfo
        {
            FileName        = teklaExe,
            Arguments       = argList,
            WindowStyle     = ProcessWindowStyle.Maximized,
            UseShellExecute = true,
        };
        var p = Process.Start(psi);
        if (p == null)
        {
            Console.Error.WriteLine("aware-tekla: Process.Start returned null");
            return 2;
        }

        var receipt = new JsonObject
        {
            ["status"]       = "ok",
            ["host"]         = "tekla",
            ["host_version"] = version,
            ["host_pid"]     = p.Id,
            ["verb"]         = "launch",
            ["verb_result"]  = new JsonObject
            {
                ["bypass_ini"]  = bypassIni,
                ["model_path"]  = modelPath ?? "",
                ["maximized"]   = true,
                ["note"]        = "Tekla is starting; poll `send-status` until success to confirm Open API readiness (typically ~30s)",
            },
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
        Console.WriteLine(receipt.ToJsonString());
        return 0;
    }

    static void EmitGuide(string message)
    {
        Console.Error.WriteLine($"aware-tekla launch: {message}");
        Console.Error.WriteLine();
        Console.Error.WriteLine("Required stdin JSON shape:");
        Console.Error.WriteLine("  { \"version\": \"2026.0\", \"environment\": \"blank_project\",");
        Console.Error.WriteLine("    \"license\": \"Partner\", \"model_path\": \"C:/path/to/model\" }");
        Console.Error.WriteLine();
        Console.Error.WriteLine("Or supply a pre-built Bypass.ini directly:");
        Console.Error.WriteLine("  { \"version\": \"2026.0\", \"bypass_ini\": \"C:/path/to/Bypass.ini\",");
        Console.Error.WriteLine("    \"model_path\": \"C:/path/to/model\" }");
    }

    static void PrintHelp()
    {
        Console.WriteLine("""
            aware-tekla — Tekla Open API sidecar

            Usage:
              aware-tekla <verb> [flags] [--json-stdin]

            Verbs:
              send-status      Display a transient message in Tekla's status bar
              list-instances   Print running Tekla instances (PID + version)
              launch           Start a Tekla instance via Bypass.ini (headless)
              close            Save + clean-shutdown a Tekla instance (Open API + ModelSave event)
              exec             Compile + run an ad-hoc C# script against the active model

            Flags:
              --version <X.Y>   Target a specific Tekla version (e.g. 2026.0)
              --pid <N>         Target a specific Tekla PID
              --json-stdin      Read inputs as JSON from stdin

            Exit codes:
              0  success
              1  no matching Tekla instance running
              2  API call failed / bad args / unknown verb
              3  Tekla not installed on this machine
              4  ambiguous target (multiple matches, no --pid)
              5  per-instance routing failed (not implemented in spike)
              6  permission denied
            """);
    }

    sealed class ParsedArgs
    {
        public string? Version;
        public int? Pid;
        public bool JsonStdin;
        public bool All;
    }

    static ParsedArgs ParseArgs(string[] args)
    {
        var p = new ParsedArgs();
        for (int i = 0; i < args.Length; i++)
        {
            switch (args[i])
            {
                case "--version":
                    p.Version = args[++i];
                    break;
                case "--pid":
                    p.Pid = int.Parse(args[++i]);
                    break;
                case "--all":
                    p.All = true;
                    break;
                case "--json-stdin":
                    p.JsonStdin = true;
                    break;
                default:
                    throw new InvalidOperationException($"unknown flag: {args[i]}");
            }
        }
        return p;
    }

    // ── Process discovery ─────────────────────────────────────────────────────

    sealed class TeklaInstance
    {
        public int Pid { get; }
        public string Version { get; }
        public string ExePath { get; }
        public TeklaInstance(int pid, string version, string exePath)
        {
            Pid = pid;
            Version = version;
            ExePath = exePath;
        }
    }

    static List<TeklaInstance> EnumerateRunningTeklas()
    {
        var result = new List<TeklaInstance>();
        foreach (var p in Process.GetProcessesByName("TeklaStructures"))
        {
            try
            {
                var path = p.MainModule?.FileName;
                if (path is null) continue;

                // Parse version from path: "C:/Program Files/Tekla Structures/2026.0/bin/TeklaStructures.exe"
                var version = ExtractVersionFromPath(path);
                if (version is null) continue;

                result.Add(new TeklaInstance(p.Id, version, path));
            }
            catch
            {
                // Inaccessible process — skip silently.
            }
        }
        return result;
    }

    static string? ExtractVersionFromPath(string path)
    {
        // Look for "Tekla Structures/<X.Y>/" segment in the path.
        var parts = path.Replace('\\', '/').Split('/');
        for (int i = 0; i < parts.Length - 1; i++)
        {
            if (parts[i] == "Tekla Structures" && i + 1 < parts.Length)
            {
                // Next segment should be like "2025.0" or "2026.0".
                var candidate = parts[i + 1];
                if (System.Text.RegularExpressions.Regex.IsMatch(candidate, @"^\d{4}\.\d+$"))
                    return candidate;
            }
        }
        return null;
    }

    static int ListInstances()
    {
        var instances = EnumerateRunningTeklas();
        var obj = new JsonObject
        {
            ["status"] = "ok",
            ["instances"] = new JsonArray(
                instances.Select(i => (JsonNode?)new JsonObject
                {
                    ["pid"] = i.Pid,
                    ["version"] = i.Version,
                    ["exe_path"] = i.ExePath,
                }).ToArray()),
        };
        Console.WriteLine(obj.ToJsonString());
        return 0;
    }

    // ── send-status ──────────────────────────────────────────────────────────

    static int SendStatus(ParsedArgs args)
    {
        // Read message from stdin JSON.
        if (!args.JsonStdin)
        {
            Console.Error.WriteLine("aware-tekla: send-status requires --json-stdin");
            return 2;
        }
        string stdin = Console.In.ReadToEnd();
        JsonNode? input;
        try
        {
            input = JsonNode.Parse(stdin);
        }
        catch (Exception e)
        {
            Console.Error.WriteLine($"aware-tekla: stdin not JSON: {e.Message}");
            return 2;
        }
        var message = input?["message"]?.GetValue<string>();
        if (string.IsNullOrEmpty(message))
        {
            Console.Error.WriteLine("aware-tekla: stdin JSON missing 'message' field");
            return 2;
        }

        // Allow stdin JSON to supply `version` too — the AWARE orchestrator
        // sends node.config as stdin JSON, so this is how the app composition
        // selects target version per node. CLI --version flag still wins
        // if both are present (for direct/manual invocations).
        if (string.IsNullOrEmpty(args.Version))
        {
            var stdinVersion = input?["version"]?.GetValue<string>();
            if (!string.IsNullOrEmpty(stdinVersion))
            {
                args.Version = stdinVersion;
            }
        }

        // Pre-flight: find matching Tekla instance(s).
        var allInstances = EnumerateRunningTeklas();
        var targets = FilterTargets(allInstances, args);
        if (targets.Count == 0)
        {
            string detail = allInstances.Count == 0
                ? "no Tekla instance running"
                : $"requested target not running (found: {string.Join(", ", allInstances.Select(t => t.Version))})";
            Console.Error.WriteLine($"aware-tekla: {detail}");
            return 1;
        }
        if (targets.Count > 1 && !args.All)
        {
            Console.Error.WriteLine(
                $"aware-tekla: ambiguous target ({targets.Count} matches). Use --pid <N> or --all.");
            return 4;
        }

        // Dispatch to every target. --all fans out across all matches;
        // single-target case is just the trivial 1-element list. We spawn a
        // separate child process per target so each gets a fresh AppDomain
        // and the right per-version DLL load (cross-version isolation comes
        // for free from loading the matching version's Tekla.Structures.Model.dll).
        if (targets.Count == 1)
        {
            // In-process path — no need for child processes when there's only one.
            var target = targets[0];
            try
            {
                DispatchSendStatus(target, message!);
            }
            catch (Exception e)
            {
                var root = e;
                while (root is System.Reflection.TargetInvocationException && root.InnerException is not null)
                    root = root.InnerException;
                Console.Error.WriteLine($"aware-tekla: API call failed: {root.GetType().Name}: {root.Message}");
                if (root.StackTrace is not null) Console.Error.WriteLine(root.StackTrace);
                return 2;
            }
            EmitReceipt(target, message!);
            return 0;
        }

        // Multi-target (--all) path: spawn one child sidecar per target with
        // --pid scoping, each connects to its own Tekla using its own DLL.
        var combined = new JsonArray();
        int failed = 0;
        var exePath = System.Reflection.Assembly.GetExecutingAssembly().Location;
        foreach (var t in targets)
        {
            var startInfo = new ProcessStartInfo
            {
                FileName               = exePath,
                Arguments              = $"send-status --pid {t.Pid} --json-stdin",
                UseShellExecute        = false,
                RedirectStandardInput  = true,
                RedirectStandardOutput = true,
                RedirectStandardError  = true,
                CreateNoWindow         = true,
            };
            startInfo.StandardOutputEncoding = System.Text.Encoding.UTF8;
            startInfo.StandardErrorEncoding  = System.Text.Encoding.UTF8;
            using var child = Process.Start(startInfo)!;
            var payload = new JsonObject { ["message"] = message }.ToJsonString();
            child.StandardInput.Write(payload);
            child.StandardInput.Close();
            string stdout = child.StandardOutput.ReadToEnd();
            string stderr = child.StandardError.ReadToEnd();
            child.WaitForExit();
            if (child.ExitCode == 0)
            {
                combined.Add(JsonNode.Parse(stdout));
            }
            else
            {
                failed++;
                combined.Add(new JsonObject
                {
                    ["status"]       = "err",
                    ["host"]         = "tekla",
                    ["host_pid"]     = t.Pid,
                    ["host_version"] = t.Version,
                    ["exit_code"]    = child.ExitCode,
                    ["stderr"]       = stderr.Trim(),
                });
            }
        }
        var allReceipt = new JsonObject
        {
            ["status"]   = failed == 0 ? "ok" : (failed == targets.Count ? "err" : "partial"),
            ["host"]     = "tekla",
            ["verb"]     = "send-status",
            ["targets"]  = combined,
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
        Console.WriteLine(allReceipt.ToJsonString());
        return failed == 0 ? 0 : (failed == targets.Count ? 2 : 0);
    }

    static void EmitReceipt(TeklaInstance target, string message)
    {
        // Emit receipt on stdout.
        var receipt = new JsonObject
        {
            ["status"] = "ok",
            ["host"] = "tekla",
            ["host_version"] = target.Version,
            ["host_pid"] = target.Pid,
            ["host_session_id"] = $"tekla-{target.Pid}",
            ["verb"] = "send-status",
            ["verb_result"] = new JsonObject { ["message"] = message },
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
        Console.WriteLine(receipt.ToJsonString());
    }

    static List<TeklaInstance> FilterTargets(List<TeklaInstance> all, ParsedArgs args)
    {
        if (args.Pid is { } pid)
            return all.Where(i => i.Pid == pid).ToList();
        if (args.Version is { } v)
            return all.Where(i => i.Version == v).ToList();
        if (args.All)
            return all;
        return all;
    }

    static bool _resolverWired;

    // Static handle for ModelSave event signaling — Tekla's
    // Events.ModelSaveDelegate is `void(string modelPath)`, so the handler
    // method below has that exact signature so Delegate.CreateDelegate
    // can bind directly.
    static System.Threading.ManualResetEventSlim? _modelSaveSignal;
    static System.Threading.ManualResetEventSlim? _teklaExitSignal;
    static void OnTeklaModelSave(string modelPath) => _modelSaveSignal?.Set();
    internal static void SignalModelSave() => _modelSaveSignal?.Set();
    internal static void SignalTeklaExit() => _teklaExitSignal?.Set();

    // Build a delegate matching the event's actual signature using a
    // DynamicMethod that just calls the named static signaller, ignoring
    // any parameters Tekla's delegate happens to declare. Returns null
    // if the event is missing on this Tekla version.
    static Delegate? BuildDynamicHandler(EventInfo? eventInfo, string signalMethodName)
    {
        if (eventInfo?.EventHandlerType == null) return null;
        var invokeSig = eventInfo.EventHandlerType.GetMethod("Invoke")!;
        var paramTypes = invokeSig.GetParameters().Select(p => p.ParameterType).ToArray();
        var dyn = new System.Reflection.Emit.DynamicMethod(
            $"{eventInfo.Name}Handler", typeof(void), paramTypes, typeof(Program), true);
        var il = dyn.GetILGenerator();
        il.Emit(System.Reflection.Emit.OpCodes.Call,
            typeof(Program).GetMethod(signalMethodName, BindingFlags.NonPublic | BindingFlags.Static)!);
        il.Emit(System.Reflection.Emit.OpCodes.Ret);
        return dyn.CreateDelegate(eventInfo.EventHandlerType);
    }

    static void DispatchSendStatus(TeklaInstance target, string message)
    {
        // Load Tekla.Structures.Model.dll from the target's installation path.
        // For the spike, we connect via the default Open API constructor — this
        // talks to "the" running Tekla, which is the only one we know about.
        var binDir = Path.GetDirectoryName(target.ExePath)!;
        var modelDllPath = Path.Combine(binDir, "Tekla.Structures.Model.dll");
        if (!File.Exists(modelDllPath))
        {
            throw new FileNotFoundException(
                $"Tekla.Structures.Model.dll not found at {modelDllPath}");
        }

        // AssemblyResolve handler MUST be wired before the first Open API call
        // — once the .NET Framework loader fails to find a dependency, it
        // caches the failure and doesn't re-ask the handler. Also change CWD
        // so probing paths include the Tekla bin directory.
        if (!_resolverWired)
        {
            // Tekla 2026 ships dual-runtime: bin/ has .NET 8/9 builds, bin/Net48Runtime/
            // has the legacy .NET Framework 4.8 builds. Our sidecar is net48 so it
            // needs Net48Runtime versions for some assemblies. Probe both, prefer
            // Net48Runtime when present.
            var probePaths = new[]
            {
                Path.Combine(binDir, "Net48Runtime"),
                binDir,
            };
            AppDomain.CurrentDomain.AssemblyResolve += (sender, eventArgs) =>
            {
                try
                {
                    var asmName = new AssemblyName(eventArgs.Name).Name;
                    if (string.IsNullOrEmpty(asmName)) return null;
                    foreach (var probe in probePaths)
                    {
                        var candidate = Path.Combine(probe, $"{asmName}.dll");
                        if (File.Exists(candidate))
                        {
                            return Assembly.LoadFrom(candidate);
                        }
                    }
                    return null;
                }
                catch
                {
                    return null;
                }
            };
            _resolverWired = true;
        }

        // Run the rest of the Tekla interop with CWD = Tekla's bin directory,
        // so the Fusion-style probing also picks up native sibling DLLs.
        var originalCwd = Environment.CurrentDirectory;
        Environment.CurrentDirectory = binDir;
        try
        {
            DispatchSendStatusInner(modelDllPath, message);
        }
        finally
        {
            Environment.CurrentDirectory = originalCwd;
        }
    }

    static void DispatchSendStatusInner(string modelDllPath, string message)
    {
        // PRE-LOAD all Tekla.Structures.* DLLs before the first Model()
        // constructor. The .NET Framework loader caches resolution failures
        // and won't call our AssemblyResolve handler if it has already failed
        // to find an assembly during static field init.
        var binDir = Path.GetDirectoryName(modelDllPath)!;
        var preloadOrder = new[]
        {
            "Tekla.Structures.dll",
            "Tekla.Structures.Datatype.dll",
            "Tekla.Structures.Model.dll",
        };
        foreach (var name in preloadOrder)
        {
            var p = Path.Combine(binDir, name);
            if (File.Exists(p))
            {
                Assembly.LoadFrom(p);
            }
        }

        // Now reflectively grab the Model type from the already-loaded asm.
        var modelAsm = AppDomain.CurrentDomain.GetAssemblies()
            .FirstOrDefault(a => a.GetName().Name == "Tekla.Structures.Model")
            ?? Assembly.LoadFrom(modelDllPath);

        // Establish the Open API connection by instantiating a Model.
        // Tekla Open API initializes its connection-to-running-Tekla machinery
        // when the first Model() is constructed; without this, Operation.* and
        // related static methods throw FileNotFoundException trying to resolve
        // the connection file from an uninitialised state.
        var modelType = modelAsm.GetType("Tekla.Structures.Model.Model")
            ?? throw new InvalidOperationException(
                "Tekla.Structures.Model.Model type not found");
        var modelInstance = Activator.CreateInstance(modelType)
            ?? throw new InvalidOperationException("Could not construct Tekla Model()");

        // Verify the connection actually attached to a running Tekla. The
        // GetConnectionStatus() method returns true once Tekla is reachable.
        var getConnStatus = modelType.GetMethod("GetConnectionStatus",
            BindingFlags.Public | BindingFlags.Instance);
        if (getConnStatus is not null)
        {
            var connected = (bool)(getConnStatus.Invoke(modelInstance, null) ?? false);
            if (!connected)
                throw new InvalidOperationException(
                    "Tekla Model.GetConnectionStatus() returned false — Tekla is running but the Open API connection couldn't attach. Is a model open?");
        }

        // Resolve Tekla.Structures.Model.Operations.Operation.DisplayPrompt(string).
        var operationType = modelAsm.GetType("Tekla.Structures.Model.Operations.Operation")
            ?? throw new InvalidOperationException(
                "Tekla.Structures.Model.Operations.Operation type not found");
        var displayPromptMethod = operationType.GetMethod(
            "DisplayPrompt",
            new[] { typeof(string) })
            ?? throw new InvalidOperationException(
                "Operation.DisplayPrompt(string) method not found");

        displayPromptMethod.Invoke(null, new object[] { message });
    }
}
