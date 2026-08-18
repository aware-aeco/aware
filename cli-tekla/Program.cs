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
using System.Runtime.InteropServices;
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
    public TeklaGridEnvelopeContract gridEnvelope = new TeklaGridEnvelopeContract();
    public TeklaMemberRollContract memberRoll = new TeklaMemberRollContract();
}

internal static class Program
{
    // ── stdout hygiene (#217) ────────────────────────────────────────────────
    // INVARIANT: stdout carries ONLY protocol JSON (receipts, watch `fired`
    // events) plus the explicitly-requested --help text. Everything else —
    // our own diagnostics, vendor-assembly console noise (Trimble.Remoting
    // prints a multi-line "Connection failed" stack trace to Console.Out
    // while Model is constructed without a live Tekla), and user exec-code
    // Console.WriteLine — belongs on stderr. Main enforces this structurally:
    // it captures the real stdout here, then points Console.Out at stderr,
    // so the only way to reach the protocol stream is an explicit write
    // through `Protocol`.
    //
    // `_protocol` stays null when Main didn't run — the unit tests drive verb
    // handlers directly and capture output via Console.SetOut, so `Protocol`
    // falls back to Console.Out there.
    static TextWriter? _protocol;
    static TextWriter Protocol => _protocol ?? Console.Out;

    static void WriteProtocolLine(string line)
    {
        var w = Protocol;
        w.WriteLine(line);
        w.Flush(); // the receipt must hit the pipe even if we exit right after
    }

    // ── last-resort receipts (#283) ──────────────────────────────────────────
    // Vendor/native code driven by an exec script can take this process down
    // without unwinding managed frames (CatalogHandler on some Tekla setups
    // terminated the sidecar outright). While a script is in flight these
    // fields are "armed"; the AppDomain hooks wired in Main then emit a
    // structured fail receipt as a last resort so the bridge protocol never
    // just goes silent. Interlocked guarantees at most one such receipt
    // (UnhandledException and ProcessExit can both fire on the way down).
    // Hard kills (TerminateProcess, FailFast, stack overflow) remain
    // unreportable from in-process — the aware CLI labels those
    // "terminated without emitting a receipt".
    static int _lastResortArmed; // 0 = disarmed, 1 = armed
    static string _lastResortVerb = "exec";
    static string? _lastResortHostVersion;
    static int? _lastResortHostPid;

    internal static void ArmLastResortReceipt(string verb, string? hostVersion, int? hostPid)
    {
        _lastResortVerb = verb;
        _lastResortHostVersion = hostVersion;
        _lastResortHostPid = hostPid;
        System.Threading.Interlocked.Exchange(ref _lastResortArmed, 1);
    }

    // Claim the single receipt slot. Exactly ONE caller wins the exchange:
    // either the normal completion path (true → it emits the ok/fail receipt)
    // or a last-resort hook (then the normal path gets false and suppresses
    // its own receipt). This is what guarantees the protocol carries exactly
    // ONE receipt per invocation.
    //
    // The claim is taken BEFORE the winner writes its receipt, which leaves a
    // sub-microsecond window: if the CLR is terminated by an async fault after
    // the normal path claims but before EmitExecOk finishes the write, no
    // receipt reaches the pipe. That is deliberate — the alternative
    // (emit-then-claim) is strictly worse: a fault in ITS window appends a
    // second, contradictory receipt, and two JSON objects on stdout make the
    // CLI's whole-stdout parse fail even on a successful exec. The empty-pipe
    // outcome is instead handled cleanly on the CLI side, which labels it
    // "process terminated without emitting a receipt" (invoker.rs).
    internal static bool TryClaimReceipt()
        => System.Threading.Interlocked.Exchange(ref _lastResortArmed, 0) == 1;

    internal static void EmitLastResortReceipt(string message, string stack)
    {
        if (!TryClaimReceipt()) return;
        // An ok:false receipt must never ride a success status
        // (Environment.Exit(0) from vendor code); deliberate non-zero
        // codes are preserved.
        if (Environment.ExitCode == 0) Environment.ExitCode = 2;
        try
        {
            EmitExecFail(message, stack, _lastResortVerb, _lastResortHostVersion, _lastResortHostPid);
        }
        catch { /* the process is dying; nothing safer to do */ }
    }

    [DllImport("kernel32.dll")]
    static extern uint SetErrorMode(uint uMode);

    // Suppress the Windows Error Reporting "Application Error" dialog and the
    // critical-error handler popup (#290). aware-tekla is a HEADLESS sidecar
    // spawned by the aware CLI — an unhandled CLR/native fault (a vendor crash,
    // a wrong-PID Model connect) must die with a receipt or a non-zero exit,
    // never block on a modal dialog that hangs the parent's wait_with_output().
    const uint SEM_FAILCRITICALERRORS = 0x0001;
    const uint SEM_NOGPFAULTERRORBOX = 0x0002;

    static int Main(string[] args)
    {
        // First statement, before anything can fault: go headless. Combined
        // with the last-resort receipt hooks below, a crash then produces a
        // structured receipt (or the CLI's "terminated without a receipt"
        // label) instead of a blocking WER dialog (#290, #283).
        SetErrorMode(SEM_FAILCRITICALERRORS | SEM_NOGPFAULTERRORBOX);

        // Force UTF-8 on stdin/stdout — .NET Framework defaults to the
        // Windows OEM codepage which mangles em-dashes, smart quotes,
        // accented chars etc when JSON travels in either direction.
        Console.InputEncoding  = new System.Text.UTF8Encoding(encoderShouldEmitUTF8Identifier: false);
        Console.OutputEncoding = new System.Text.UTF8Encoding(encoderShouldEmitUTF8Identifier: false);

        // stdout hygiene (#217): capture the (now UTF-8) real stdout as the
        // protocol stream, then route Console.Out to stderr so nothing that
        // writes through Console — vendor assemblies, user exec code, stray
        // library logging — can corrupt the whole-stdout JSON contract.
        // Must run AFTER the encoding setup above (setting OutputEncoding
        // recreates Console.Out) and BEFORE any Tekla/Roslyn interaction.
        _protocol = Console.Out;
        Console.SetOut(Console.Error);

        // Last-resort receipt hooks (#283) — no-ops unless a script is in
        // flight (see ArmLastResortReceipt). UnhandledException covers vendor
        // exceptions on background threads and corrupted-state exceptions
        // (access violations); ProcessExit covers vendor code calling exit.
        AppDomain.CurrentDomain.UnhandledException += (_, ue) =>
        {
            var ex = ue.ExceptionObject as Exception;
            EmitLastResortReceipt(
                $"{ex?.GetType().Name ?? "UnhandledException"}: " +
                $"{ex?.Message ?? "unknown non-CLR exception"} " +
                "(vendor/native failure during script execution)",
                ex?.StackTrace ?? "");
        };
        AppDomain.CurrentDomain.ProcessExit += (_, _) =>
            EmitLastResortReceipt(
                "sidecar exited during script execution (vendor code called exit?)",
                "");

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
            // Strip leading UTF-8 BOM (U+FEFF) that some PowerShell versions
            // prepend when piping strings to native executables.
            buf = TrimJsonBom(buf);
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
        if (!parsed.JsonStdin && (verb == "exec" || verb == "bake-scene"))
        {
            // Exec / bake-scene are always stdin-driven; force the flag if the caller
            // used the embedded-verb style.
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
            case "bake-scene":
                return BakeScene(parsed);
            case "watch":
                return Watch(parsed);
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
                var input = JsonNode.Parse(TrimJsonBom(Console.In.ReadToEnd()));
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
        WriteProtocolLine(receipt.ToJsonString());
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

    // ── watch ──────────────────────────────────────────────────────────────────
    // A `lifecycle: start` long-running subscription to the Tekla model event
    // stream. Connects to the live model, registers a `ModelObjectChanged`
    // handler (plus `ModelLoad` / `TeklaStructuresExit`), and writes one
    // newline-delimited JSON event per change to stdout — which the runtime's
    // `CliInvoker::invoke_stream` (#172/#173) consumes. Runs until the transport
    // kills the child (its `stop`) or Tekla itself exits.
    //
    // stdout carries ONLY `{"signal":"fired", …}` change events — the runtime
    // forwards every stdout line downstream as a data event, so control records
    // must not go there. Live trigger state (listening↔fired) is observable from
    // the run's NodeStart/NodeOutput events (#143 precedent); listening and
    // model-load are emitted as stderr breadcrumbs (see WriteDiagnostic).
    //
    // Root cause of the original "zero live events" bug (#219): the bridge bound
    // `ModelObjectChanged` with a reflection-emitted DynamicMethod delegate, and
    // Tekla's Open API simply does not deliver to one — `Register()` succeeds but
    // the callback never fires. A REAL-method delegate receives fine (verified
    // live against Tekla 2025 + 2026), so RunWatchLoop now binds via
    // Delegate.CreateDelegate (see BindEventHandler). NO message pump, STA thread,
    // or SynchronizationContext is involved: Tekla raises the handler on its own
    // async thread (Open API Events docs: handlers run "asynchronously" and are
    // "not guaranteed in the same thread where registered"), so the bridge just
    // keeps the (MTA) process alive on a wait handle — the same shape Tekla's own
    // standalone event apps and FloLess's TeklaBridge use. Because the handler
    // can fire on a Tekla worker thread, every stdout write stays serialized
    // under `_watchConsoleLock`.
    internal static int Watch(ParsedArgs args)
    {
        string filter = "all";
        bool selfTest = false;
        string? version = args.Version;
        _watchIncludeDeleted = false;
        _watchOnce = false;
        _watchEmittedOnce = false;

        if (args.JsonStdin)
        {
            string raw;
            try { raw = Console.In.ReadToEnd(); }
            catch (Exception e)
            {
                Console.Error.WriteLine($"aware-tekla watch: stdin not readable: {e.Message}");
                return 2;
            }
            raw = TrimJsonBom(raw);
            if (!string.IsNullOrWhiteSpace(raw))
            {
                JsonNode? input;
                try { input = JsonNode.Parse(raw); }
                catch (Exception e)
                {
                    Console.Error.WriteLine($"aware-tekla watch: stdin not JSON: {e.Message}");
                    return 2;
                }
                filter = input?["filter"]?.GetValue<string>() ?? filter;
                selfTest = ReadBool(input, "self_test") || ReadBool(input, "self-test");
                _watchIncludeDeleted = ReadBool(input, "include_deleted") || ReadBool(input, "include-deleted");
                _watchOnce = ReadBool(input, "once") || ReadBool(input, "one_time") || ReadBool(input, "one-time");
                ParseEventSelection(input?["events"]);
                version ??= input?["version"]?.GetValue<string>();
            }
            else ParseEventSelection(null);
        }
        else ParseEventSelection(null);

        filter = (filter ?? "all").Trim().ToLowerInvariant();
        _watchFilter = filter;
        _watchDebug = Environment.GetEnvironmentVariable("AWARE_TEKLA_WATCH_DEBUG") == "1";

        // Offline self-test path: exercise the full listening → filter → fired
        // emit pipeline with synthetic changes, no live Tekla. This is what the
        // bridge's own end-to-end check drives (the analogue of #173's fake
        // streamer, but through the real watch code).
        if (selfTest || Environment.GetEnvironmentVariable("AWARE_TEKLA_WATCH_SELFTEST") == "1")
            return RunWatchSelfTest(filter);

        // Target selection — mirror send-status/close: honor --pid / --version
        // (the stdin `version` folds into --version), fail fast on no match, and
        // refuse an ambiguous target rather than binding to an arbitrary running
        // instance. A watch attaches to a single live model via the Open API
        // connection, so >1 match is always ambiguous here (no --all fan-out).
        if (string.IsNullOrEmpty(args.Version) && !string.IsNullOrEmpty(version))
            args.Version = version;
        var running = EnumerateRunningTeklas();
        var targets = FilterTargets(running, args);
        if (targets.Count == 0)
        {
            Console.Error.WriteLine(running.Count == 0
                ? "aware-tekla watch: no Tekla instance running — start Tekla with a model open. (error.tekla-not-running)"
                : $"aware-tekla watch: requested target not running (found: {string.Join(", ", running.Select(t => t.Version))}). (error.tekla-not-running)");
            return 1;
        }
        if (targets.Count > 1)
        {
            Console.Error.WriteLine(
                $"aware-tekla watch: ambiguous target ({targets.Count} Tekla instances match) — " +
                "a watch subscribes to a single model. Narrow with --pid <N> or --version <X.Y>.");
            return 4;
        }

        // exePath is …/<version>/bin/TeklaStructures.exe — its directory is bin/.
        var binDir = Path.GetDirectoryName(targets[0].ExePath)!;
        WireResolver(binDir);
        var originalCwd = Environment.CurrentDirectory;
        Environment.CurrentDirectory = binDir;
        try
        {
            object model;
            try { model = ConstructTeklaModel(binDir); }
            catch (Exception e)
            {
                Console.Error.WriteLine(
                    $"aware-tekla watch: could not load Tekla.Structures.Model: {e.Message} " +
                    "(error.tekla-not-running)");
                return 1;
            }

            var modelType = model.GetType();
            var connected = (bool)(modelType.GetMethod("GetConnectionStatus")?.Invoke(model, null) ?? false);
            if (!connected)
            {
                Console.Error.WriteLine(
                    "aware-tekla watch: Tekla is running but the Open API connection couldn't attach. " +
                    "Is a model open? (error.tekla-not-running)");
                return 1;
            }

            if (_watchDebug)
            {
                string? modelName = null;
                try
                {
                    var info = modelType.GetMethod("GetInfo")?.Invoke(model, null);
                    modelName = info?.GetType().GetProperty("ModelName")?.GetValue(info)?.ToString();
                }
                catch { /* best effort */ }
                WriteDiagnostic(new JsonObject
                {
                    ["signal"]       = "debug",
                    ["msg"]          = "connected",
                    ["model"]        = modelName,
                    ["delivered_at"] = DateTime.UtcNow.ToString("o"),
                });
            }

            // Run the event loop on a dedicated STA thread with a message pump —
            // the configuration Tekla's own standalone event sample uses. UI-thread
            // events (SelectionChange, ViewClosed, …) are delivered through the
            // message queue and only fire while that thread pumps; worker-thread
            // events (ModelObjectChanged) deliver regardless. MTA-without-a-pump
            // (the prior shape) got ModelObjectChanged but never SelectionChange.
            return RunWatchLoopOnStaThread(modelType.Assembly, filter);
        }
        finally
        {
            Environment.CurrentDirectory = originalCwd;
        }
    }

    // Marshal RunWatchLoop onto a dedicated STA thread (Tekla's standalone event
    // apps are STA + message-pumped). The Events instance is created, registered,
    // and pumped all on this one thread so UI events have a pump to ride. Faults
    // rethrow on the caller so Main's top-level handler still reports them.
    [MethodImpl(MethodImplOptions.NoInlining)]
    static int RunWatchLoopOnStaThread(Assembly modelAsm, string filter)
    {
        int rc = 0;
        Exception? fault = null;
        var t = new System.Threading.Thread(() =>
        {
            try { rc = RunWatchLoop(modelAsm, filter); }
            catch (Exception e) { fault = e; }
        })
        {
            IsBackground = false,
            Name = "aware-tekla-watch",
        };
        t.SetApartmentState(System.Threading.ApartmentState.STA);
        t.Start();
        t.Join();
        if (fault is not null) throw fault;
        return rc;
    }

    // Pump the Windows message queue on the current (STA) thread until `stop` is
    // signaled. MsgWaitForMultipleObjects wakes on a new message OR on `stop`
    // (index 0, so stop wins) — no busy spin — and a bounded tick is a liveness
    // safety net. This is what carries Tekla's UI-thread events to our handlers.
    static void PumpMessagesUntil(System.Threading.WaitHandle stop)
    {
        var handles = new[] { stop.SafeWaitHandle.DangerousGetHandle() };
        while (true)
        {
            uint r = MsgWaitForMultipleObjects(1, handles, false, PUMP_TICK_MS, QS_ALLINPUT);
            if (r == WAIT_OBJECT_0) break;   // `stop` signaled
            if (r == WAIT_FAILED) break;     // bail rather than spin
            while (PeekMessage(out var msg, IntPtr.Zero, 0, 0, PM_REMOVE))
            {
                if (msg.Message == WM_QUIT) { GC.KeepAlive(stop); return; }
                TranslateMessage(ref msg);
                DispatchMessage(ref msg);
            }
        }
        GC.KeepAlive(stop);
    }

    const uint PUMP_TICK_MS  = 200;
    const uint QS_ALLINPUT   = 0x04FF;
    const uint PM_REMOVE     = 0x0001;
    const uint WAIT_OBJECT_0 = 0x00000000;
    const uint WAIT_FAILED   = 0xFFFFFFFF;
    const uint WM_QUIT       = 0x0012;

#pragma warning disable CS0649 // fields populated by the P/Invoke marshaller, not C#
    [StructLayout(LayoutKind.Sequential)]
    struct NativeMessage
    {
        public IntPtr Hwnd;
        public uint   Message;
        public IntPtr WParam;
        public IntPtr LParam;
        public uint   Time;
        public int    PtX;
        public int    PtY;
    }
#pragma warning restore CS0649

    [DllImport("user32.dll", SetLastError = true)]
    static extern uint MsgWaitForMultipleObjects(
        uint nCount, IntPtr[] pHandles, bool bWaitAll, uint dwMilliseconds, uint dwWakeMask);

    [DllImport("user32.dll")]
    [return: MarshalAs(UnmanagedType.Bool)]
    static extern bool PeekMessage(
        out NativeMessage lpMsg, IntPtr hWnd, uint wMsgFilterMin, uint wMsgFilterMax, uint wRemoveMsg);

    [DllImport("user32.dll")]
    [return: MarshalAs(UnmanagedType.Bool)]
    static extern bool TranslateMessage(ref NativeMessage lpMsg);

    [DllImport("user32.dll")]
    static extern IntPtr DispatchMessage(ref NativeMessage lpMsg);

    // Register the model event handlers, then block until Tekla exits (or the
    // transport kills the child). Tekla delivers ModelObjectChanged to an
    // out-of-process subscriber on its own async thread — NOT via a message pump,
    // and not necessarily on the registering thread (per the Open API Events
    // docs). Runs on an STA thread with a Win32 message pump (see
    // RunWatchLoopOnStaThread / PumpMessagesUntil) — the configuration Tekla's own
    // standalone event sample uses. Worker-thread events (ModelObjectChanged)
    // deliver regardless of the pump; UI-thread events (SelectionChange, …) are
    // posted to the message queue and only fire while it is pumped.
    //
    // Two hard requirements:
    //  1. (#219) the handler must be a REAL-method delegate — Tekla does not invoke
    //     a reflection-emitted DynamicMethod delegate. ModelObjectChanged binds to
    //     OnModelObjectChanged(object) by contravariance; generic events bind to a
    //     per-event instance emitter (see BindEventHandler / BindGenericEvent).
    //  2. the STA message pump above, for UI-thread events.
    //
    // Kept NoInlining for the same JIT/AssemblyResolve ordering reason as exec.
    [MethodImpl(MethodImplOptions.NoInlining)]
    static int RunWatchLoop(Assembly modelAsm, string filter)
    {
        var eventsType = modelAsm.GetType("Tekla.Structures.Model.Events")
            ?? throw new InvalidOperationException("Tekla.Structures.Model.Events type not found");
        var eventsInstance = Activator.CreateInstance(eventsType)
            ?? throw new InvalidOperationException("Could not construct Tekla.Structures.Model.Events()");

        _watchStopSignal = new System.Threading.ManualResetEventSlim(false);

        var changedEvent = eventsType.GetEvent("ModelObjectChanged");
        var loadEvent    = eventsType.GetEvent("ModelLoad");
        var exitEvent    = eventsType.GetEvent("TeklaStructuresExit");

        // Bind real static methods (see BindEventHandler). ModelObjectChanged's
        // List<ChangeData> binds to OnModelObjectChanged(object) contravariantly;
        // the no-arg lifecycle events bind to their void() signallers.
        // ModelObjectChanged keeps its rich `fired` handler; ModelLoad and
        // TeklaStructuresExit are infrastructure (a stderr breadcrumb and the stop
        // signal) and are always bound. The `fired` stream is gated on selection.
        var changedHandler = IsEventSelected("ModelObjectChanged")
            ? BindEventHandler(changedEvent, nameof(OnModelObjectChanged)) : null;
        var loadHandler    = BindEventHandler(loadEvent, nameof(SignalModelLoad));
        var exitHandler    = BindEventHandler(exitEvent, nameof(SignalWatchStop));

        if (changedHandler is not null) changedEvent!.AddEventHandler(eventsInstance, changedHandler);
        if (loadHandler is not null) loadEvent!.AddEventHandler(eventsInstance, loadHandler);
        if (exitHandler is not null) exitEvent!.AddEventHandler(eventsInstance, exitHandler);

        // Every OTHER selected Tekla event is bound generically (real-method
        // closed-delegate emitters — see BindGenericEvent) and streamed as a
        // `{"signal":"event", …}` record. `events: all` (or a name list) widens
        // coverage to the whole Events surface; the default is ModelObjectChanged.
        var genericHandlers = new List<(EventInfo Ev, Delegate Handler)>();
        var boundNames = new List<string>();
        foreach (var ev in eventsType.GetEvents())
        {
            var nm = ev.Name;
            if (nm is "ModelObjectChanged" or "ModelLoad" or "TeklaStructuresExit") continue;
            if (!IsEventSelected(nm)) continue;
            var h = BindGenericEvent(ev);
            if (h is null) continue;
            ev.AddEventHandler(eventsInstance, h);
            genericHandlers.Add((ev, h));
            boundNames.Add(nm);
        }

        eventsType.GetMethod("Register")?.Invoke(eventsInstance, null);

        if (_watchDebug)
            WriteDiagnostic(new JsonObject
            {
                ["signal"]        = "debug",
                ["msg"]           = "registered",
                ["apartment"]     = System.Threading.Thread.CurrentThread.GetApartmentState().ToString(),
                ["changed_bound"] = changedHandler is not null,
                ["generic_events"] = string.Join(",", boundNames),
                ["delivered_at"]  = DateTime.UtcNow.ToString("o"),
            });

        EmitListening(filter);

        try
        {
            // Pump the Windows message queue until TeklaStructuresExit sets the
            // signal — this is what delivers UI-thread events (SelectionChange,
            // ViewClosed, …) to this STA thread. The runtime's streaming transport
            // stops a watcher by killing the child process (see
            // cli/src/runtime/invoker.rs), so a hard kill is the common exit;
            // reacting to Tekla's own exit lets us shut down cleanly when the host
            // goes away instead of lingering as a zombie watcher.
            PumpMessagesUntil(_watchStopSignal.WaitHandle);
        }
        finally
        {
            try
            {
                if (changedHandler is not null) changedEvent!.RemoveEventHandler(eventsInstance, changedHandler);
                if (loadHandler is not null) loadEvent!.RemoveEventHandler(eventsInstance, loadHandler);
                if (exitHandler is not null) exitEvent!.RemoveEventHandler(eventsInstance, exitHandler);
                foreach (var (ev, h) in genericHandlers) ev.RemoveEventHandler(eventsInstance, h);
                eventsType.GetMethod("UnRegister")?.Invoke(eventsInstance, null);
            }
            catch { /* best-effort cleanup — the host may already be gone */ }
            _watchStopSignal = null;
        }
        return 0;
    }

    // Bind a Tekla event to a REAL static method via Delegate.CreateDelegate.
    // CreateDelegate's relaxed (variant) matching lets a method with `object`
    // parameters bind to Tekla's concrete delegate types (e.g. the
    // List<ChangeData> of ModelObjectChanged) by reference contravariance, and
    // no-arg signallers bind to no-arg event delegates. Crucially the result is
    // a real-method delegate, which — unlike a reflection-emitted DynamicMethod —
    // Tekla actually invokes (#219). Returns null if the signatures are
    // incompatible on this Tekla version (best-effort, never throws).
    internal static Delegate? BindEventHandler(EventInfo? eventInfo, string methodName)
    {
        if (eventInfo?.EventHandlerType is null) return null;
        var method = typeof(Program).GetMethod(
            methodName, BindingFlags.NonPublic | BindingFlags.Public | BindingFlags.Static);
        if (method is null) return null;
        try { return Delegate.CreateDelegate(eventInfo.EventHandlerType, method); }
        catch { return null; }
    }

    // Bind any Tekla event to a generic emitter. CRITICAL (#219 follow-up): the
    // handler must be the SAME delegate shape Tekla actually invokes — a plain
    // INSTANCE-method delegate (Target = an object), exactly like the Open API
    // samples' `events.SelectionChange += handler.OnX` and FloLess's
    // TeklaEventManager. An earlier version used a *closed static* delegate
    // (Delegate.CreateDelegate(type, eventName, staticMethod), Target = a string);
    // ModelObjectChanged still fired (it uses an open static delegate) but the
    // no-payload events like SelectionChange never did. Here each event gets a
    // GenericEventEmitter instance that carries the event name, and we bind one of
    // its instance methods by parameter shape (reference args bind to `object` by
    // contravariance; value-typed shapes use an exact-typed method). Exotic shapes
    // we don't model (e.g. (int,int,enum)) are skipped — best-effort, never throws.
    internal static Delegate? BindGenericEvent(EventInfo ev)
    {
        var invoke = ev.EventHandlerType?.GetMethod("Invoke");
        if (invoke is null) return null;
        var ps = invoke.GetParameters();
        string? methodName = ps.Length switch
        {
            0 => nameof(GenericEventEmitter.Emit0),
            1 when !ps[0].ParameterType.IsValueType => nameof(GenericEventEmitter.Emit1),
            1 when ps[0].ParameterType == typeof(int) => nameof(GenericEventEmitter.EmitInt),
            3 when ps[0].ParameterType == typeof(string)
                && ps[1].ParameterType == typeof(string)
                && ps[2].ParameterType == typeof(bool) => nameof(GenericEventEmitter.EmitCmd),
            _ => null,
        };
        if (methodName is null) return null;
        var m = typeof(GenericEventEmitter).GetMethod(
            methodName, BindingFlags.Public | BindingFlags.Instance);
        if (m is null) return null;
        try { return Delegate.CreateDelegate(ev.EventHandlerType!, new GenericEventEmitter(ev.Name), m); }
        catch { return null; }
    }

    // Per-event emitter: an instance object carrying the event name, bound to a
    // Tekla event via an INSTANCE-method delegate (the delegate shape Tekla
    // invokes — see BindGenericEvent). The delegate keeps this instance alive.
    internal sealed class GenericEventEmitter
    {
        readonly string _name;
        public GenericEventEmitter(string name) => _name = name;
        public void Emit0() => EmitEventRecord(_name, null);
        public void Emit1(object? a) => EmitEventRecord(_name, DescribeArg(a));
        public void EmitInt(int a) => EmitEventRecord(_name, JsonValue.Create(a));
        public void EmitCmd(string command, string param, bool status) =>
            EmitEventRecord(_name, new JsonObject
            {
                ["command"] = command,
                ["param"]   = param,
                ["active"]  = status,
            });
    }

    // A non-`fired` event record on the stdout data stream — same channel as
    // `fired`, discriminated by `signal`/`event` so downstream nodes can route.
    static void EmitEventRecord(string eventName, JsonNode? data)
    {
        WriteJsonLine(new JsonObject
        {
            ["signal"]       = "event",
            ["event"]        = eventName,
            ["data"]         = data,
            ["host"]         = "tekla",
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        });
    }

    // Best-effort, allocation-light description of a single event argument without
    // re-entering Tekla: strings pass through, collections report their count, and
    // an object exposing an Identifier.GUID (or GUID) reports type + guid.
    internal static JsonNode? DescribeArg(object? a)
    {
        if (a is null) return null;
        if (a is string s) return JsonValue.Create(s);
        if (a is System.Collections.ICollection col) return new JsonObject { ["count"] = col.Count };
        var guid = TryGetGuid(a) ?? a.GetType().GetProperty("GUID")?.GetValue(a)?.ToString();
        if (!string.IsNullOrEmpty(guid))
            return new JsonObject { ["type"] = a.GetType().Name, ["guid"] = guid };
        return JsonValue.Create(a.ToString());
    }

    // ── event selection (`events` config) ───────────────────────────────────────
    // Which Tekla events the watch streams. Default: ModelObjectChanged only (the
    // historical behavior). `events: "all"` widens to the whole Events surface;
    // `events: ["ModelSave", "model-object-changed", …]` selects by name (kebab or
    // PascalCase, case-insensitive). Names are stored normalized (alphanumerics,
    // lower-cased) so spelling/casing variations all match.
    internal static bool _watchEventsAll;
    internal static HashSet<string> _watchEvents = new(StringComparer.Ordinal);

    internal static void ParseEventSelection(JsonNode? node)
    {
        _watchEventsAll = false;
        _watchEvents = new HashSet<string>(StringComparer.Ordinal);

        void Add(string? raw)
        {
            if (string.IsNullOrWhiteSpace(raw)) return;
            if (raw!.Trim().Equals("all", StringComparison.OrdinalIgnoreCase)) { _watchEventsAll = true; return; }
            _watchEvents.Add(NormalizeEventName(raw));
        }

        if (node is JsonArray arr)
        {
            foreach (var n in arr)
            {
                try { Add(n?.GetValue<string>()); } catch { /* skip non-string entries */ }
            }
        }
        else if (node is not null)
        {
            try { Add(node.GetValue<string>()); } catch { /* not a string scalar */ }
        }

        // Default to the model-change stream when nothing usable was supplied.
        if (!_watchEventsAll && _watchEvents.Count == 0)
            _watchEvents.Add(NormalizeEventName("ModelObjectChanged"));
    }

    internal static bool IsEventSelected(string teklaEventName) =>
        _watchEventsAll || _watchEvents.Contains(NormalizeEventName(teklaEventName));

    internal static string NormalizeEventName(string s) =>
        new string(s.Where(char.IsLetterOrDigit).ToArray()).ToLowerInvariant();

    // Emit `listening`, then synthetic `fired` events covering the object kinds
    // the filter discriminates, so the filter + emit path is verifiable without
    // a live Tekla. Returns 0 (clean exit), mirroring a finite real run.
    static int RunWatchSelfTest(string filter)
    {
        EmitListening(filter);
        var synthetic = new (string Type, string Change)[]
        {
            ("Assembly", "added"),
            ("Weld", "added"),
            ("BoltArray", "added"),
            ("Beam", "modified"),
            ("Beam", "removed"),
        };
        int n = 0;
        foreach (var (type, change) in synthetic)
        {
            if (change == "removed" && !_watchIncludeDeleted) continue;
            if (!WatchFilterMatches(filter, type)) continue;
            var ev = BuildWatchEvent(
                $"00000000-0000-0000-0000-{n:D12}",
                $"{type.Substring(0, 1)}/{n + 1}",
                type,
                change,
                null);
            WriteJsonLine(ev);
            n++;
        }
        return 0;
    }

    // The typeless ModelObjectChanged handler. `changes` is a
    // `List<ChangeData>` (an IEnumerable); each item is a ChangeData with
    // `Type` (ChangeTypeEnum) and `Object` (a ModelObject carrying identifier
    // info only — Select() hydrates the rest). One bad change must never kill
    // the stream, so each is wrapped.
    internal static void OnModelObjectChanged(object? changes)
    {
        if (_watchDebug)
            WriteDiagnostic(new JsonObject
            {
                ["signal"]       = "debug",
                ["msg"]          = "ModelObjectChanged fired",
                ["apartment"]    = System.Threading.Thread.CurrentThread.GetApartmentState().ToString(),
                ["payload_type"] = changes?.GetType().FullName,
                ["delivered_at"] = DateTime.UtcNow.ToString("o"),
            });
        if (changes is not System.Collections.IEnumerable list) return;
        foreach (var cd in list)
        {
            if (cd is null) continue;
            try { EmitChange(cd); }
            catch (Exception e)
            {
                lock (_watchConsoleLock)
                    Console.Error.WriteLine($"aware-tekla watch: skipped a change: {e.Message}");
            }
        }
    }

    static void EmitChange(object changeData)
    {
        var t = changeData.GetType();
        int changeTypeVal = Convert.ToInt32(t.GetProperty("Type")?.GetValue(changeData) ?? 1);
        string change = MapChangeType(changeTypeVal);
        if (change == "removed" && !_watchIncludeDeleted) return;

        var mo = t.GetProperty("Object")?.GetValue(changeData);
        if (mo is null) return;

        // The .NET runtime type (Beam, Assembly, Weld, BoltArray, …) is the
        // filter key — available without a DB round-trip.
        string typeName = mo.GetType().Name;
        if (!WatchFilterMatches(_watchFilter, typeName)) return;

        string? guid = TryGetGuid(mo);
        string? mark = null;
        JsonNode? geometry = null;
        // A removed object can't be Select()ed; emit identity only.
        if (change != "removed")
        {
            TrySelect(mo);
            mark = TryGetMark(mo);
            geometry = TryGetGeometry(mo);
        }

        WriteJsonLine(BuildWatchEvent(guid, mark, typeName, change, geometry));
    }

    // Map Tekla's ChangeData.ChangeTypeEnum to the stream's `change` enum.
    //   OBJECT_INSERT(0)→added  OBJECT_DELETE(2)→removed
    //   OBJECT_MODIFY(1) / USERPROPERTY_CHANGED(3)→modified
    internal static string MapChangeType(int changeTypeValue) => changeTypeValue switch
    {
        0 => "added",
        2 => "removed",
        _ => "modified",
    };

    // Map the manifest `filter` enum to a predicate on the changed object's
    // runtime type. `welded`/`bolted` match weld/bolt objects (a welded
    // connection fires a ModelObjectChanged for its Weld object); `assembly`
    // matches Assembly objects. Drawing changes are NOT surfaced by
    // ModelObjectChanged (they come from Tekla.Structures.Drawing.Events), so a
    // `drawing` filter is intentionally not offered here — see manifest. An
    // unknown filter passes everything rather than silently dropping the stream.
    internal static bool WatchFilterMatches(string filter, string typeName)
    {
        switch ((filter ?? "all").Trim().ToLowerInvariant())
        {
            case "all":      return true;
            case "assembly": return typeName.Equals("Assembly", StringComparison.OrdinalIgnoreCase);
            case "welded":   return typeName.IndexOf("Weld", StringComparison.OrdinalIgnoreCase) >= 0;
            case "bolted":   return typeName.IndexOf("Bolt", StringComparison.OrdinalIgnoreCase) >= 0;
            default:         return true;
        }
    }

    internal static JsonObject BuildWatchEvent(
        string? guid, string? mark, string typeName, string change, JsonNode? geometry)
    {
        return new JsonObject
        {
            ["signal"]       = "fired",
            ["guid"]         = guid,
            ["mark"]         = mark,
            ["type"]         = typeName,
            ["change"]       = change,
            ["geometry"]     = geometry,
            ["host"]         = "tekla",
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
    }

    static void EmitListening(string filter)
    {
        WriteDiagnostic(new JsonObject
        {
            ["signal"]       = "listening",
            ["host"]         = "tekla",
            ["verb"]         = "watch",
            ["filter"]       = filter,
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        });
    }

    // A `fired` change event on the stdout data stream — the runtime treats each
    // stdout line as a data event and propagates it downstream.
    static void WriteJsonLine(JsonNode node)
    {
        // Worker-thread events can fire concurrently; serialize stdout writes.
        lock (_watchConsoleLock)
        {
            // One-shot mode (`once`): emit the FIRST record, then suppress any
            // further records and signal the pump to unwind — an event-driven
            // snapshot. The suppression guard matters because a single
            // ModelObjectChanged batch can carry several changes that would
            // otherwise all be written before the stop signal is observed.
            if (_watchOnce && _watchEmittedOnce) return;
            var w = Protocol;
            w.WriteLine(node.ToJsonString());
            w.Flush();
            if (_watchOnce)
            {
                _watchEmittedOnce = true;
                SignalWatchStop(); // unregister + exit the message pump
            }
        }
    }

    // A lifecycle breadcrumb on stderr — NOT the stdout data stream. The runtime
    // forwards every stdout line downstream as a change event, so a control
    // record there would fire connected nodes with no guid/mark/type/change
    // before any real model change. Listening/model-load state is instead
    // observable from the run's NodeStart/NodeOutput events (the UI-facing
    // listening↔fired signal) plus these stderr breadcrumbs for logs.
    static void WriteDiagnostic(JsonNode node)
    {
        lock (_watchConsoleLock)
        {
            Console.Error.WriteLine(node.ToJsonString());
            Console.Error.Flush();
        }
    }

    static bool ReadBool(JsonNode? input, string key)
    {
        try { return input?[key]?.GetValue<bool>() ?? false; }
        catch { return false; }
    }

    static string? TryGetGuid(object modelObject)
    {
        try
        {
            var id = modelObject.GetType().GetProperty("Identifier")?.GetValue(modelObject);
            var guid = id?.GetType().GetProperty("GUID")?.GetValue(id);
            return guid?.ToString();
        }
        catch { return null; }
    }

    static void TrySelect(object modelObject)
    {
        try { modelObject.GetType().GetMethod("Select", Type.EmptyTypes)?.Invoke(modelObject, null); }
        catch { /* deleted / unreadable — caller emits identity only */ }
    }

    static string? TryGetMark(object modelObject)
    {
        // identity is the position Mark — never Name (see drawing-identity skill).
        foreach (var prop in new[] { "ASSEMBLY_POS", "PART_POS", "MARK" })
        {
            try
            {
                var m = modelObject.GetType().GetMethod(
                    "GetReportProperty", new[] { typeof(string), typeof(string).MakeByRefType() });
                if (m is null) return null;
                object?[] callArgs = { prop, "" };
                if (m.Invoke(modelObject, callArgs) is bool ok && ok
                    && callArgs[1] is string s && !string.IsNullOrEmpty(s))
                    return s;
            }
            catch { /* try the next property */ }
        }
        return null;
    }

    static JsonNode? TryGetGeometry(object modelObject)
    {
        // NOTE on coordinates: Solid.MinimumPoint/MaximumPoint are returned in
        // Tekla's CURRENT transformation plane, which equals world only when the
        // user hasn't changed the work plane. We deliberately do NOT switch the
        // session to the global plane to "normalize" here — that's a global,
        // non-thread-safe mutation of the user's live Tekla session, unacceptable
        // from a passive worker-thread watcher. The contract (manifest/watch.md)
        // documents the box as current-plane accordingly.
        try
        {
            var getSolid = modelObject.GetType().GetMethod("GetSolid", Type.EmptyTypes);
            var solid = getSolid?.Invoke(modelObject, null);
            if (solid is null) return null;
            var min = solid.GetType().GetProperty("MinimumPoint")?.GetValue(solid);
            var max = solid.GetType().GetProperty("MaximumPoint")?.GetValue(solid);
            var minJson = PointToJson(min);
            var maxJson = PointToJson(max);
            if (minJson is null || maxJson is null) return null;
            return new JsonObject { ["min"] = minJson, ["max"] = maxJson };
        }
        catch { return null; }
    }

    internal static JsonNode? PointToJson(object? point)
    {
        if (point is null) return null;
        try
        {
            var x = ReadCoord(point, "X");
            var y = ReadCoord(point, "Y");
            var z = ReadCoord(point, "Z");
            // If any coordinate can't be read, return null rather than a
            // misleading all-zero box.
            if (x is null || y is null || z is null) return null;
            return new JsonObject { ["x"] = x.Value, ["y"] = y.Value, ["z"] = z.Value };
        }
        catch { return null; }
    }

    // Tekla.Structures.Geometry3d.Point exposes X/Y/Z as public *fields*, not
    // properties — read the field first, then fall back to a property for any
    // wrapper that exposes them differently. Returns null when neither exists.
    static double? ReadCoord(object point, string name)
    {
        var t = point.GetType();
        var field = t.GetField(name);
        if (field is not null) return Convert.ToDouble(field.GetValue(point) ?? 0.0);
        var prop = t.GetProperty(name);
        if (prop is not null) return Convert.ToDouble(prop.GetValue(point) ?? 0.0);
        return null;
    }

    // Wire the AssemblyResolve probe (Net48Runtime first, then bin) so the .NET
    // loader can find Tekla.Structures.* siblings at runtime. Idempotent via the
    // shared `_resolverWired` guard.
    static void WireResolver(string binDir)
    {
        if (_resolverWired) return;
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

    internal static void SignalModelLoad()
    {
        // Lifecycle breadcrumb (stderr), not a data event — see WriteDiagnostic.
        WriteDiagnostic(new JsonObject
        {
            ["signal"]       = "status",
            ["change"]       = "model-loaded",
            ["host"]         = "tekla",
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        });
    }

    internal static void SignalWatchStop() => _watchStopSignal?.Set();

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

        string stdin = TrimJsonBom(Console.In.ReadToEnd());
        JsonNode? input;
        try { input = JsonNode.Parse(stdin); }
        catch (Exception e) { EmitExecError($"stdin not JSON: {e.Message}"); return 2; }

        string? code    = input?["code"]?.GetValue<string>();
        string? version = input?["version"]?.GetValue<string>() ?? args.Version;
        var argsNode    = input?["args"] as JsonObject;
        if (string.IsNullOrEmpty(code))
        {
            EmitExecError("missing required field: code (the C# script to compile + run)");
            return 2;
        }
        if (!TryReadPid(input, args, out var pid, out var pidError))
        {
            EmitExecFail(pidError!, "", "exec", null, null);
            return 2;
        }

        return ExecuteResolvedScript("exec", code!, version, pid, argsNode, ScriptCommitPolicy.Automatic);
    }

    // ── bake-scene ─────────────────────────────────────────────────────────────
    // Create native Tekla members (a Beam per scene element) from a generic 3D scene
    // — the bridge-shipped, first-class form of the steel "bake into Tekla" operation
    // (previously inlined as a FloLess app exec script). WRITE: needs Tekla open with
    // a model. Runs the embedded BakeSceneCode through the same Roslyn path as `exec`,
    // so it shares host resolution, the assembly resolver, and the post-script commit.
    static int BakeScene(ParsedArgs args)
    {
        if (!args.JsonStdin)
        {
            EmitExecError("bake-scene requires --json-stdin (or pass the request JSON via stdin)");
            return 2;
        }

        string stdin = TrimJsonBom(Console.In.ReadToEnd());
        JsonNode? input;
        try { input = JsonNode.Parse(stdin); }
        catch (Exception e) { EmitExecError($"stdin not JSON: {e.Message}"); return 2; }

        var scene = input?["scene"];
        if (scene is null)
        {
            EmitExecError("missing required field: scene (the 3D scene to bake into the model)");
            return 2;
        }
        string? version = input?["version"]?.GetValue<string>() ?? args.Version;
        if (!TryReadPid(input, args, out var pid, out var pidError))
        {
            EmitExecFail(pidError!, "", "bake-scene", null, null);
            return 2;
        }
        var qaGuard = string.Equals(
            Environment.GetEnvironmentVariable("AWARE_TEKLA_QA_GUARD"),
            "1",
            StringComparison.Ordinal);
        if (!TryResolveExpectedModelPath(
                input,
                Environment.GetEnvironmentVariable("AWARE_TEKLA_EXPECT_MODEL_PATH"),
                qaGuard,
                out var expectedModelPath,
                out var modelPathError))
        {
            EmitExecFail(modelPathError!, "", "bake-scene", null, null);
            return 2;
        }

        // What the user sees in Tekla while the bake compiles and runs. An optional `label` lets the
        // caller say who is doing this — the substrate has no name of its own to offer, and must not
        // borrow the scene's, which names the MODEL rather than its producer. It rides beside `scene`
        // rather than inside it, so the scene hash (and materialization identity) is unaffected.
        string? label = input?["label"]?.GetValue<string>();
        int objectCount = scene["elements"] is JsonArray sceneElements ? sceneElements.Count : 0;
        string plural = objectCount == 1 ? "" : "s";
        string announce = string.IsNullOrWhiteSpace(label)
            ? $"Adding {objectCount} object{plural} to this model..."
            : $"{label}: adding {objectCount} object{plural} to this model...";

        // Hand the scene to the embedded bake script as the `args.scene` global; `label` rides along
        // so the completion message the script emits on commit matches this one.
        var argsNode = new JsonObject
        {
            ["scene"] = scene.DeepClone(),
            ["label"] = label,
        };
        if (expectedModelPath is not null)
            argsNode["expectedModelPath"] = expectedModelPath;
        return ExecuteResolvedScript("bake-scene", BakeSceneCode, version, pid, argsNode, ScriptCommitPolicy.ScriptOwned, announce);
    }

    internal static bool TryResolveExpectedModelPath(
        JsonNode? input,
        string? environmentPath,
        bool qaGuard,
        out string? resolvedPath,
        out string? error)
    {
        resolvedPath = null;
        error = null;
        string? requestPath = null;
        if (input is JsonObject request && request.ContainsKey("expectedModelPath"))
        {
            if (request["expectedModelPath"] is not JsonValue value
                || !value.TryGetValue<string>(out requestPath))
            {
                error = "invalid `expectedModelPath`: expected a non-empty string";
                return false;
            }
            requestPath = requestPath.Trim();
            if (requestPath.Length == 0)
            {
                error = "invalid `expectedModelPath`: expected a non-empty string";
                return false;
            }
        }

        environmentPath = environmentPath?.Trim();
        if (environmentPath?.Length == 0)
            environmentPath = null;
        try
        {
            requestPath = requestPath is null
                ? null
                : TeklaSceneInputContract.CanonicalModelDirectoryPath(requestPath);
            environmentPath = environmentPath is null
                ? null
                : TeklaSceneInputContract.CanonicalModelDirectoryPath(environmentPath);
        }
        catch (Exception exception)
        {
            error = $"invalid expected Tekla model path: {exception.Message}";
            return false;
        }

        if (requestPath is not null
            && environmentPath is not null
            && !string.Equals(requestPath, environmentPath, StringComparison.OrdinalIgnoreCase))
        {
            error = "conflicting expected Tekla model paths in the request and AWARE_TEKLA_EXPECT_MODEL_PATH";
            return false;
        }
        resolvedPath = requestPath ?? environmentPath;
        if (qaGuard && resolvedPath is null)
        {
            error = "AWARE_TEKLA_QA_GUARD=1 requires expectedModelPath or AWARE_TEKLA_EXPECT_MODEL_PATH";
            return false;
        }
        return true;
    }

    internal static string TrimJsonBom(string input) => input.TrimStart('\uFEFF');

    // Resolve the target PID for exec/bake: the stdin JSON `pid` (how floless.app passes it) takes
    // precedence, falling back to the `--pid` CLI flag. A `pid` field that is PRESENT but not a valid
    // integer is a caller error, not an absent pid — reject it (error out), because silently ignoring
    // it would run against whatever single host is up despite the caller's assertion, which is unsafe
    // for a write script (#290 review). Returns false + `error` on a malformed pid.
    internal static bool TryReadPid(JsonNode? input, ParsedArgs args, out int? pid, out string? error)
    {
        error = null;
        // Distinguish a PRESENT `pid` key from an absent one: `input["pid"]` returns null for BOTH a
        // missing property and an explicit `"pid": null`, so check key presence. A present key must
        // be a valid integer — null / string / float / out-of-range are caller errors, not "absent"
        // (a dynamic target that resolved to null must fail, not silently write to the sole host).
        if (input is JsonObject obj && obj.ContainsKey("pid"))
        {
            if (obj["pid"] is JsonValue v && v.TryGetValue<int>(out var p))
            {
                pid = p;
                return true;
            }
            pid = null;
            error = obj["pid"] is null
                ? "invalid `pid`: expected an integer, got null"
                : $"invalid `pid`: expected an integer, got {obj["pid"]!.ToJsonString()}";
            return false;
        }
        pid = args.Pid;
        return true;
    }

    internal const string BakeMaterializerIdentity = "tekla-connection-materializer-v3";

    internal static string ComputeBakeMaterializationHash(JsonNode scene, string? version)
    {
        var payload = BakeMaterializerIdentity + "\0"
            + (version ?? "running-host")
            + "\0"
            + scene.ToJsonString(new JsonSerializerOptions { WriteIndented = false });
        using var sha = System.Security.Cryptography.SHA256.Create();
        var digest = sha.ComputeHash(System.Text.Encoding.UTF8.GetBytes(payload));
        return string.Concat(digest.Select(b => b.ToString("x2", System.Globalization.CultureInfo.InvariantCulture)));
    }

    internal enum ScriptCommitPolicy
    {
        Automatic,
        ScriptOwned,
    }

    internal enum BakeFailureDisposition
    {
        DeleteStagingAndCommitCleanup,
        LeaveStateForSourceReconciliation,
    }

    internal static BakeFailureDisposition FailureDisposition(bool priorSetRetirementStarted) =>
        priorSetRetirementStarted
            ? BakeFailureDisposition.LeaveStateForSourceReconciliation
            : BakeFailureDisposition.DeleteStagingAndCommitCleanup;

    internal static ScriptCommitPolicy CommitPolicyForVerb(string verb) =>
        string.Equals(verb, "bake-scene", StringComparison.Ordinal)
            ? ScriptCommitPolicy.ScriptOwned
            : ScriptCommitPolicy.Automatic;

    // Shared core for the script-running verbs (`exec`, `bake-scene`): resolve the
    // Tekla host for `version`, wire the assembly resolver, run `code` via Roslyn with
    // `argsNode` exposed as the `args` global, and emit the standard exec receipt.
    static int ExecuteResolvedScript(
        string verb,
        string code,
        string? version,
        int? pid,
        JsonObject? argsNode,
        ScriptCommitPolicy? commitPolicy = null,
        string? announce = null)
    {

        // Find the running Tekla instance (if any) to populate host_pid and
        // host_version in the receipt — matches the cli-rhino receipt shape so
        // downstream orchestrators see a consistent envelope across vendors.
        // Best-effort: if no Tekla is running (smoke-test path), pid stays null.
        int? hostPid = null;
        string? hostVersion = version;
        // A running Tekla instance is version-locked: the Open API can only connect to the
        // instance that is actually open, so DLL resolution must bind to ITS version, not the
        // requested one (#264) — otherwise a request for a non-running major can never connect
        // ("No live Tekla model …") even with a model open. The requested `version` is only a
        // fallback for the no-instance (smoke-test) path. `resolveVersion` carries the chosen
        // version out.
        string? resolveVersion = version;
        ExecTarget target;
        try
        {
            var (rawCount, instances) = DiscoverTeklas();
            target = ResolveExecTarget(pid, version, rawCount, instances);
        }
        catch (Exception e)
        {
            // Fail CLOSED: if we can't enumerate the running Tekla processes we cannot establish
            // that exactly one is live, and proceeding to new Model() risks the ambiguous connection
            // / CLR crash this guard exists to prevent (#290 review). Refuse with a receipt.
            EmitExecFail($"could not enumerate Tekla instances: {e.Message}", "", verb, hostVersion, hostPid);
            return 4;
        }

        switch (target.Kind)
        {
            case ExecTargetKind.Ambiguous:
            case ExecTargetKind.NotRunning:
                // Refuse before touching the Open API — a roulette connection with more than one
                // instance live is what CLR-crashes the sidecar (#290). Exit 4 = ambiguous target.
                EmitExecFail(target.Message, "", verb, hostVersion, hostPid);
                return 4;
            case ExecTargetKind.Resolved:
                hostPid = target.Instance!.Pid;
                hostVersion = target.Instance.Version;
                resolveVersion = target.Instance.Version;
                break;
            case ExecTargetKind.NoHost:
                // Smoke-test path: no live host, requested version drives DLL resolution.
                break;
        }

        if (string.Equals(verb, "bake-scene", StringComparison.Ordinal)
            && argsNode?["scene"] is JsonNode bakeScene)
        {
            // Hash against the resolved running host version, not the caller's stale
            // request. The canonical scene includes every requested profile/operation/
            // grid input, so exact Insert/read-back selects one deterministic result.
            argsNode["materializationHash"] = ComputeBakeMaterializationHash(bakeScene, hostVersion);
        }

        // Resolve the Tekla install dir for the version we'll connect to (the running instance
        // when one is open, else the requested version). Standard path + registry. Missing-install
        // is non-fatal: the script may not reference Tekla types (smoke-test path returns primitives).
        string? hostInstall = string.IsNullOrEmpty(resolveVersion) ? null : DiscoverTeklaInstall(resolveVersion!);
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
        // Armed: if vendor/native code kills the process mid-script, the
        // AppDomain hooks in Main still emit a fail receipt (#283).
        ArmLastResortReceipt(verb, hostVersion, hostPid);
        try
        {
            var resultNode = RunScriptOnStaThread(
                code!,
                probedReferences,
                argsNode,
                probedDir,
                hostPid,
                commitPolicy ?? CommitPolicyForVerb(verb),
                announce);
            // Losing the disarm race means a last-resort hook already emitted
            // a fail receipt (a background-thread fault) — ours is suppressed.
            if (!TryClaimReceipt()) return 2;
            if (ScriptResultReportsFailure(verb, resultNode))
            {
                EmitExecResultFail(resultNode, verb, hostVersion, hostPid);
                return 2;
            }
            EmitExecOk(resultNode, verb, hostVersion, hostPid);
            return 0;
        }
        catch (CompilationErrorException ce)
        {
            if (!TryClaimReceipt()) return 2;
            // Script failed to compile — surface diagnostics so the caller
            // (likely an AI) can re-draft.
            var diagnostics = string.Join("\n", ce.Diagnostics.Select(d => d.ToString()));
            EmitExecFail($"compile error: {ce.Message}", diagnostics, verb, hostVersion, hostPid);
            return 2;
        }
        catch (Exception e)
        {
            if (!TryClaimReceipt()) return 2;
            var root = e;
            while (root is TargetInvocationException && root.InnerException is not null)
                root = root.InnerException;
            EmitExecFail(root.GetType().Name + ": " + root.Message, root.StackTrace ?? "",
                         verb, hostVersion, hostPid);
            return 2;
        }
    }

    // Run the Roslyn script on a dedicated STA thread. Tekla's Open API is
    // written for STA standalone apps (the watch path already runs its loop
    // STA + pumped, see RunWatchLoopOnStaThread); catalogue calls
    // (CatalogHandler, #283) have terminated the whole process when driven
    // from the default MTA main thread. Serialization happens ON the STA
    // thread too — a lazy Tekla enumerable serialized after the join would
    // call back into Tekla from the MTA caller.
    //
    // We deliberately do NOT install a single-threaded SynchronizationContext
    // to keep top-level-`await` continuations on this STA thread. Doing so
    // deadlocks any script that synchronously waits on context-capturing async
    // work (`SomeAsync().Result`, `.Wait()`): the blocked STA thread is the
    // only one that could pump the continuation it is waiting for. That hang is
    // unbounded (the parent's wait_with_output has no timeout) — strictly worse
    // than the alternative. So awaited continuations resume on the thread pool
    // (RunAsync's default), and the rare `await …; model.Foo()` that then
    // touches Tekla off-STA is caught by the last-resort receipt rather than
    // hanging the bridge. Tekla's Open API is entirely synchronous, so real
    // exec scripts almost never await at all.
    [MethodImpl(MethodImplOptions.NoInlining)]
    internal static JsonNode? RunScriptOnStaThread(
        string code,
        IReadOnlyList<MetadataReference> teklaReferences,
        JsonObject? argsNode,
        string? teklaBinDir,
        int? expectedPid,
        ScriptCommitPolicy commitPolicy,
        string? announce = null)
    {
        JsonNode? result = null;
        Exception? fault = null;
        var t = new System.Threading.Thread(() =>
        {
            try
            {
                result = SerializeResult(
                    RunScript(code, teklaReferences, argsNode, teklaBinDir, expectedPid, commitPolicy, announce));
            }
            catch (Exception e) { fault = e; }
        })
        {
            IsBackground = false,
            Name = "aware-tekla-exec",
        };
        t.SetApartmentState(System.Threading.ApartmentState.STA);
        t.Start();
        t.Join();
        if (fault is not null)
            // Preserve the script's original stack trace — the exec receipt
            // carries it so an AI caller can re-draft against the real frame.
            System.Runtime.ExceptionServices.ExceptionDispatchInfo.Capture(fault).Throw();
        return result;
    }

    // The canonical scene materializer is shipped with the bridge but isolated in
    // BakeSceneScript so its source-owned staging and commit boundary stay reviewable.
    static readonly string BakeSceneCode = BakeSceneScript.Code;

    internal static ScriptOptions CreateScriptOptions(IReadOnlyList<MetadataReference> teklaReferences)
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
            MetadataReference.CreateFromFile(typeof(object).Assembly.Location),
            MetadataReference.CreateFromFile(typeof(System.Linq.Enumerable).Assembly.Location),
            MetadataReference.CreateFromFile(typeof(System.Collections.Generic.IDictionary<,>).Assembly.Location),
            MetadataReference.CreateFromFile(typeof(System.Dynamic.DynamicObject).Assembly.Location),
            MetadataReference.CreateFromFile(typeof(Microsoft.CSharp.RuntimeBinder.Binder).Assembly.Location),
            // The shipped bake script delegates host-agnostic contract math to
            // public helpers in this assembly. Roslyn receives ExecGlobals as
            // its globals type, but does not automatically add that assembly as
            // a metadata reference for helper DTO names used in script source.
            MetadataReference.CreateFromFile(typeof(ExecGlobals).Assembly.Location),
        };
        refs = refs
            .GroupBy(r => (r as PortableExecutableReference)?.FilePath ?? Guid.NewGuid().ToString())
            .Select(g => g.First())
            .ToList();
        refs.AddRange(teklaReferences);

        return ScriptOptions.Default
            .WithReferences(refs)
            .WithImports(imports)
            .WithEmitDebugInformation(false);
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    static object? RunScript(
        string code,
        IReadOnlyList<MetadataReference> teklaReferences,
        JsonObject? argsNode,
        string? teklaBinDir,
        int? expectedPid,
        ScriptCommitPolicy commitPolicy,
        string? announce = null)
    {
        var options = CreateScriptOptions(teklaReferences);

        // Construct the Tekla Model lazily. If teklaBinDir is null OR Tekla
        // isn't running, the constructor either throws or returns a model
        // with GetConnectionStatus()==false — neither is fatal here: a
        // smoke-test script (`return 1+2;`) never touches `model`. We set
        // model=null and let the script blow up at its own dynamic call site
        // if it tries to use Tekla without a live host.
        object? modelInstance = null;
        if (teklaBinDir is not null)
        {
            // TOCTOU recheck (#290 review): the process set was snapshotted in ExecuteResolvedScript
            // BEFORE DLL probing, bake hashing, and STA thread startup — a window in which the Tekla
            // process set could change. Recheck as late as possible (here, on the STA thread, immediately
            // before the connection). The Open API binds new Model() by VERSION, so the invariant to
            // reassert is per-major, not global:
            //   • Resolved → the target PID must still be live, every process inspectable, and the
            //                target's MAJOR must still have exactly one instance (a same-major sibling
            //                appearing is what new Model() could misattach to; other majors are fine)
            //   • NoHost   → NO Tekla may have started (DLLs are the requested version, not a running
            //                one — a host appearing could version-mismatch)
            {
                var (rawNow, instNow) = DiscoverTeklas();
                bool stillValid;
                if (expectedPid is int want)
                {
                    var match = instNow.Where(i => i.Pid == want).ToList();
                    stillValid = match.Count == 1
                        && rawNow == instNow.Count
                        && instNow.Count(i => i.Version == match[0].Version) == 1;
                }
                else
                {
                    stillValid = rawNow == 0;
                }
                if (!stillValid)
                    throw new InvalidOperationException(
                        "the running Tekla instance set changed between host selection and connect "
                        + "(the target closed, or another instance of its version started). Retry with "
                        + "exactly one instance of the target Tekla version open (other versions are fine).");
            }
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

        // Announce BEFORE compiling. Compiling this script is the long pole of a bake (seconds),
        // and it happens with a live Tekla connection already in hand — so a message emitted from
        // inside the script only appears once that wait is already over, flashing by just before the
        // work finishes. Emitted here it covers the whole wait. Cosmetic: never fail a bake over it.
        Action<string>? sayStatus = null;
        if (!string.IsNullOrEmpty(announce) && modelInstance is not null)
        {
            try
            {
                var opType = modelInstance.GetType().Assembly
                    .GetType("Tekla.Structures.Model.Operations.Operation");
                var prompt = opType?.GetMethod("DisplayPrompt", new[] { typeof(string) });
                if (prompt is not null)
                {
                    sayStatus = msg => { try { prompt.Invoke(null, new object[] { msg }); } catch { } };
                    sayStatus(announce!);
                }
            }
            catch { /* a status message is never worth failing a bake for */ }
        }

        var script = CSharpScript.Create<object>(
            code,
            options: options,
            globalsType: typeof(ExecGlobals));

        // Compile + execute. Top-level `await` continuations resume on the
        // thread pool (no captured SynchronizationContext) — see
        // RunScriptOnStaThread for why a single-threaded pump is NOT installed
        // (it would deadlock sync-over-async scripts).
        object? returnValue;
        try
        {
            var state = script.RunAsync(globals).GetAwaiter().GetResult();
            returnValue = state.ReturnValue;
        }
        catch
        {
            // The script never returned, so its own failure prompt never ran. Do not leave the
            // caller's "adding..." claim standing over a model where nothing happened.
            sayStatus?.Invoke("Nothing was added.");
            throw;
        }

        // Tekla "transaction-commit" — flush changes to the database. We
        // only do this if a real Model was constructed AND the connection is
        // live (Tekla running, model open). Without the connection check
        // CommitChanges throws into the dead remoting channel.
        if (commitPolicy == ScriptCommitPolicy.Automatic && modelInstance is not null)
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

    internal static (IReadOnlyList<MetadataReference> refs, string? probedDir) ResolveTeklaReferences(string? hostInstall)
    {
        if (hostInstall is null) return (Array.Empty<MetadataReference>(), null);

        // Honour the manifest's dll-probe-paths: Net48Runtime first, then bin.
        var binDir = Path.Combine(hostInstall, "bin");
        if (!Directory.Exists(binDir))
            return (Array.Empty<MetadataReference>(), null);

        var probePaths = new[] { Path.Combine(binDir, "Net48Runtime"), binDir };
        var found = new List<MetadataReference>();
        var seen = new HashSet<string>(StringComparer.OrdinalIgnoreCase);
        foreach (var probe in probePaths)
        {
            if (!Directory.Exists(probe)) continue;

            foreach (var path in Directory
                .EnumerateFiles(probe, "Tekla.Structures*.dll", SearchOption.TopDirectoryOnly)
                .OrderBy(Path.GetFileName, StringComparer.OrdinalIgnoreCase))
            {
                var name = Path.GetFileName(path);
                if (seen.Contains(name)) continue;

                try
                {
                    // Tekla's bin also contains native helpers such as
                    // Tekla.Structures.Native.DbvDatabase.dll. Roslyn can only consume
                    // managed PE metadata, so validate before admitting a reference.
                    _ = AssemblyName.GetAssemblyName(path);
                    found.Add(MetadataReference.CreateFromFile(path));
                    seen.Add(name);
                }
                catch (BadImageFormatException) { /* native DLL — not a Roslyn reference */ }
                catch (FileLoadException) { /* invalid or unsupported managed image */ }
                catch (IOException) { /* raced/missing/unreadable install file */ }
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

    internal static Dictionary<string, object?> JsonObjectToDictionary(JsonObject? obj)
    {
        var dict = new Dictionary<string, object?>(StringComparer.Ordinal);
        if (obj is null) return dict;
        foreach (var kvp in obj)
        {
            dict[kvp.Key] = JsonNodeToObject(kvp.Value);
        }
        return dict;
    }

    internal static object? JsonNodeToObject(JsonNode? node)
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

    internal static bool ScriptResultReportsFailure(string verb, JsonNode? result)
    {
        return string.Equals(verb, "bake-scene", StringComparison.Ordinal)
            && result is JsonObject resultObject
            && resultObject["ok"] is JsonValue okValue
            && okValue.TryGetValue<bool>(out var ok)
            && !ok;
    }

    static void EmitExecOk(JsonNode? result, string verb, string? hostVersion = null, int? hostPid = null)
    {
        var receipt = new JsonObject
        {
            ["ok"] = true,
            ["result"] = result,
            ["host"] = "tekla",
            ["host_version"] = hostVersion,
            ["host_pid"] = hostPid,
            ["verb"] = verb,
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
        WriteProtocolLine(receipt.ToJsonString());
    }

    static void EmitExecResultFail(JsonNode? result, string verb, string? hostVersion = null, int? hostPid = null)
    {
        var receipt = new JsonObject
        {
            ["ok"] = false,
            ["error"] = $"{verb} returned ok:false; inspect the structured result receipt",
            ["result"] = result,
            ["host"] = "tekla",
            ["host_version"] = hostVersion,
            ["host_pid"] = hostPid,
            ["verb"] = verb,
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
        WriteProtocolLine(receipt.ToJsonString());
    }

    static void EmitExecFail(string message, string stack, string verb, string? hostVersion = null, int? hostPid = null)
    {
        var receipt = new JsonObject
        {
            ["ok"] = false,
            ["error"] = message,
            ["stack"] = stack,
            ["host"] = "tekla",
            ["host_version"] = hostVersion,
            ["host_pid"] = hostPid,
            ["verb"] = verb,
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
        WriteProtocolLine(receipt.ToJsonString());
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

    internal static JsonNode? SerializeResult(object? result)
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
        string stdin = TrimJsonBom(Console.In.ReadToEnd());
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
        // Launch Tekla with the DEFAULT (dialog-enabled) error mode. A child
        // captures the parent's process error mode at creation, so without this
        // the user-facing TeklaStructures.exe would inherit the sidecar's
        // headless SEM_NOGPFAULTERRORBOX and silently swallow its OWN startup /
        // runtime error dialogs (#283 review). Restore ours right after.
        var priorErrorMode = SetErrorMode(0);
        Process? p;
        try { p = Process.Start(psi); }
        finally { SetErrorMode(priorErrorMode); }
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
        WriteProtocolLine(receipt.ToJsonString());
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
        // Help is the one non-JSON stdout payload — it only prints when the
        // caller explicitly asked for it (no args / --help), never during a
        // protocol exchange, so routing it through Protocol keeps the
        // historical stdout behavior without weakening the invariant.
        WriteProtocolLine("""
            aware-tekla — Tekla Open API sidecar

            Usage:
              aware-tekla <verb> [flags] [--json-stdin]

            Verbs:
              send-status      Display a transient message in Tekla's status bar
              list-instances   Print running Tekla instances (PID + version)
              launch           Start a Tekla instance via Bypass.ini (headless)
              close            Save + clean-shutdown a Tekla instance (Open API + ModelSave event)
              exec             Compile + run an ad-hoc C# script against the active model
              bake-scene       Materialize source-owned native parts, operations, and grids (write)
              watch            Stream ModelObjectChanged events as newline-delimited JSON (lifecycle: start)

            Flags:
              --version <X.Y>   Target a specific Tekla version (e.g. 2026.0)
              --pid <N>         Target a specific Tekla PID
              --json-stdin      Read inputs as JSON from stdin

            watch (stdin JSON):
              { "filter": "all|welded|bolted|assembly",   // default all (model-object changes)
                "include_deleted": false,                  // emit OBJECT_DELETE too
                "self_test": false }                       // synthetic events, no live Tekla
              Emits {"signal":"listening"} first, then one {"signal":"fired", guid, mark,
              type, change, geometry} line per matching change. Runs until Tekla exits or the
              caller stops (kills) the process.

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

    internal sealed class ParsedArgs
    {
        public string? Version;
        public int? Pid;
        public bool JsonStdin;
        public bool All;
    }

    internal static ParsedArgs ParseArgs(string[] args)
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

    internal sealed class TeklaInstance
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

    static List<TeklaInstance> EnumerateRunningTeklas() => DiscoverTeklas().instances;

    // Enumerate once, returning BOTH the raw TeklaStructures.exe process count and the subset we
    // could actually inspect (read a version from the exe path). The raw count is load-bearing for
    // exec safety (#290): a process we couldn't inspect — MainModule unreadable because it runs at a
    // different elevation, or an unrecognized custom install path — still exists, and new Model()
    // could attach to it. exec therefore treats rawCount > inspected as unsafe rather than assuming
    // the instances it happened to see are all of them.
    internal static (int rawCount, List<TeklaInstance> instances) DiscoverTeklas()
    {
        int mySession;
        try { mySession = Process.GetCurrentProcess().SessionId; }
        catch { mySession = -1; } // unknown → don't filter (count all), the safe default

        // The Open API's remoting channel (a session-local memory-mapped file) only reaches Tekla
        // instances in the SAME Windows session as this sidecar. On RDP / multi-user hosts,
        // GetProcessesByName returns Tekla from EVERY session, so an unreachable other-session
        // instance must not count toward ambiguity or be selectable (#290 review). Exclude only
        // processes we can POSITIVELY place in a different session; an unreadable SessionId errs
        // toward counting (a false ambiguous just asks the user to close others, whereas dropping a
        // reachable instance would risk the wrong-model connect).
        var reachable = new List<Process>();
        foreach (var p in Process.GetProcessesByName("TeklaStructures"))
        {
            bool differentSession;
            try { differentSession = mySession >= 0 && p.SessionId != mySession; }
            catch { differentSession = false; }
            if (!differentSession) reachable.Add(p);
        }

        var instances = new List<TeklaInstance>();
        foreach (var p in reachable)
        {
            try
            {
                var path = p.MainModule?.FileName;
                if (path is null) continue;

                // Parse version from path: "C:/Program Files/Tekla Structures/2026.0/bin/TeklaStructures.exe"
                var version = ExtractVersionFromPath(path);
                if (version is null) continue;

                instances.Add(new TeklaInstance(p.Id, version, path));
            }
            catch
            {
                // Inaccessible process — counted in rawCount, absent from instances.
            }
        }
        return (reachable.Count, instances);
    }

    internal enum ExecTargetKind
    {
        NoHost,      // nothing running — smoke-test path (types resolve; live calls report disconnected)
        Resolved,    // exactly one safe target to run against
        Ambiguous,   // >1 live instance — the out-of-process Open API can't be bound to one; refuse
        NotRunning,  // an explicit --pid named a target that isn't among the running instances
    }

    internal readonly struct ExecTarget
    {
        public ExecTargetKind Kind { get; }
        public TeklaInstance? Instance { get; }
        public string Message { get; }
        public ExecTarget(ExecTargetKind kind, TeklaInstance? instance, string message)
        {
            Kind = kind;
            Instance = instance;
            Message = message;
        }
    }

    // Decide which running Tekla an exec/bake will connect to (refines #290/#292). The Open API's
    // out-of-process `new Model()` binds by VERSION, not PID: loading a major's Tekla.Structures.*
    // assemblies connects to THAT major's instance (#264). So different majors run side by side and
    // are disambiguable by a --version/--pid selector; the only irreducible ambiguity is two instances
    // of the SAME major (identical DLLs — new Model() can't tell them apart), or a process we couldn't
    // inspect (unknown major — could collide with the target). So:
    //   • 0 processes                         → NoHost   (smoke-test; requested version drives DLLs)
    //   • any uninspected process             → Ambiguous (unknown major could collide — close all but one)
    //   • exactly 1 inspected instance        → Resolved (bind DLLs to ITS version — #264)
    //   • >1 of the selected major            → Ambiguous (version-bound API can't pick one same-major sibling)
    //   • >1 major, no --version/--pid         → Ambiguous (name one; different majors are fine side by side)
    //   • a selector picks one unique major   → Resolved
    // An explicit --pid/--version is honoured as the router. Pure + internal so it is unit-tested
    // without a live Tekla.
    internal static ExecTarget ResolveExecTarget(
        int? pid, string? requestedVersion, int rawProcessCount, IReadOnlyList<TeklaInstance> instances)
    {
        // Nothing running. An explicit --pid asked for a live target, so say it isn't there rather
        // than silently taking the smoke-test path.
        if (rawProcessCount == 0)
        {
            return pid is int p0
                ? new ExecTarget(ExecTargetKind.NotRunning, null,
                    $"requested pid {p0} is not running; no Tekla is running.")
                : new ExecTarget(ExecTargetKind.NoHost, null, "no running Tekla instance");
        }

        // An UNINSPECTABLE process is still genuinely unsafe: we can't read its version, so it could be
        // the SAME major as our target, and new Model() (which binds by version, not PID) could attach to
        // it. Keep refusing whenever any process couldn't be inspected. (#290 review.)
        if (instances.Count < rawProcessCount)
        {
            return new ExecTarget(ExecTargetKind.Ambiguous, null,
                $"{rawProcessCount} Tekla processes are running but {rawProcessCount - instances.Count} "
                + "could not be inspected (a different elevation, or an unrecognized install path). "
                + "aware-tekla can't prove which one new Model() would attach to — close all but one and retry.");
        }

        // Exactly one inspected instance: bind to it. Its version drives DLL resolution even if a
        // different version was requested (#264) — the requested version is only a fallback. An explicit
        // --pid is still asserted (it must name the single running instance).
        if (instances.Count == 1)
        {
            var single = instances[0];
            if (pid is int pOnly && single.Pid != pOnly)
                return new ExecTarget(ExecTargetKind.NotRunning, null,
                    $"requested pid {pOnly} is not running; the only live Tekla is pid {single.Pid} ({single.Version}).");
            return new ExecTarget(ExecTargetKind.Resolved, single, "");
        }

        // The Open API is VERSION-LOCKED: loading a major's Tekla.Structures.* assemblies makes new Model()
        // connect to THAT major's running instance (#264). So DIFFERENT majors are disambiguable — the only
        // irreducible ambiguity is two instances of the SAME major (their DLLs are identical, so new Model()
        // can't tell them apart). Resolve to a single instance whenever a --pid or --version selects one
        // major uniquely; refuse only on same-major multiplicity. (Corrects #290/#292's blanket refusal.)
        List<TeklaInstance> candidates;
        if (pid is int p)
        {
            candidates = instances.Where(i => i.Pid == p).ToList();
            if (candidates.Count == 0)
                return new ExecTarget(ExecTargetKind.NotRunning, null,
                    $"requested pid {p} is not among the running Tekla instances ("
                    + string.Join(", ", instances.Select(i => $"pid {i.Pid} ({i.Version})")) + ").");
        }
        else if (!string.IsNullOrEmpty(requestedVersion))
        {
            candidates = instances.Where(i => i.Version == requestedVersion).ToList();
            if (candidates.Count == 0)
                return new ExecTarget(ExecTargetKind.NotRunning, null,
                    $"no running Tekla {requestedVersion} (running: "
                    + string.Join(", ", instances.Select(i => $"pid {i.Pid} ({i.Version})")) + ").");
        }
        else
        {
            candidates = instances.ToList();
        }

        var target = candidates[0];
        // Same-major multiplicity is the real ambiguity: >1 instance of the target's major means new Model()
        // (version-bound, not PID-bound) could attach to either. Refuse and say which major to thin out.
        int sameMajor = instances.Count(i => i.Version == target.Version);
        if (sameMajor > 1)
        {
            return new ExecTarget(ExecTargetKind.Ambiguous, null,
                $"{sameMajor} instances of Tekla {target.Version} are running ("
                + string.Join(", ", instances.Where(i => i.Version == target.Version).Select(i => $"pid {i.Pid}")) + "). "
                + "The Open API binds new Model() by version, so it can't choose between same-version instances — "
                + $"close all but one Tekla {target.Version} instance and retry.");
        }
        // Multiple DIFFERENT majors with no selector: we can't pick for the caller, but any single one is
        // reachable — tell them to name it. (A pid/version selector above would have resolved it.)
        if (candidates.Count > 1)
        {
            return new ExecTarget(ExecTargetKind.Ambiguous, null,
                $"{instances.Count} Tekla instances of different majors are running ("
                + string.Join(", ", instances.Select(i => $"pid {i.Pid} ({i.Version})")) + "). "
                + "Pass --version <X.Y> or --pid <N> to pick one — different majors run side by side fine.");
        }

        return new ExecTarget(ExecTargetKind.Resolved, target, "");
    }

    internal static string? ExtractVersionFromPath(string path)
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
        WriteProtocolLine(obj.ToJsonString());
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
        string stdin = TrimJsonBom(Console.In.ReadToEnd());
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
            // Drain stdout and stderr concurrently. #217 routes all vendor
            // noise to the child's stderr, so a sequential ReadToEnd on
            // stdout could deadlock: the child blocks writing into a full
            // stderr pipe buffer while we block waiting for stdout EOF.
            var soTask = child.StandardOutput.ReadToEndAsync();
            var seTask = child.StandardError.ReadToEndAsync();
            child.WaitForExit();
            string stdout = soTask.GetAwaiter().GetResult();
            string stderr = seTask.GetAwaiter().GetResult();
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
        WriteProtocolLine(allReceipt.ToJsonString());
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
        WriteProtocolLine(receipt.ToJsonString());
    }

    internal static List<TeklaInstance> FilterTargets(List<TeklaInstance> all, ParsedArgs args)
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

    // ── watch state ────────────────────────────────────────────────────────────
    // Worker-thread events emit through `WriteJsonLine`, serialized on this lock.
    static readonly object _watchConsoleLock = new object();
    // Set by the TeklaStructuresExit handler to unblock RunWatchLoop.
    static System.Threading.ManualResetEventSlim? _watchStopSignal;
    // The active filter + delete policy, read by the worker-thread handler.
    // Internal so the test assembly can drive OnModelObjectChanged directly.
    internal static string _watchFilter = "all";
    internal static bool _watchIncludeDeleted;
    // One-shot mode (watch input `once`/`one_time`): when true, the watch emits on
    // the FIRST fired/event record then stops (unregister + exit the pump) — an
    // event-driven snapshot. Default false = continuous (fire on every event).
    // Internal so the test assembly can clear the process-global flag between runs.
    internal static bool _watchOnce;
    // Tripped after the first emitted record in one-shot mode, so a multi-change
    // batch (one ModelObjectChanged carrying several changes) still yields exactly
    // one record before the pump unwinds. Reset at each Watch() entry.
    static bool _watchEmittedOnce;
    // Diagnostic gate (AWARE_TEKLA_WATCH_DEBUG=1) — emits stderr breadcrumbs for
    // event-delivery debugging (#219). Off in normal runs.
    internal static bool _watchDebug;

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
