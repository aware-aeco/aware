// aware-rhino — runtime sidecar that wraps McNeel's rhinocode CLI for the AWARE
// runtime. Speaks the {verb,code,args} stdin-JSON contract shared with cli-tekla.

using System.Text.Json.Nodes;
using AwareRhino.Verbs;

namespace AwareRhino;

internal static class Program
{
    static int Main(string[] args)
    {
        Console.InputEncoding  = new System.Text.UTF8Encoding(false);
        Console.OutputEncoding = new System.Text.UTF8Encoding(false);

        try { return Run(args); }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"aware-rhino: unhandled error: {ex.Message}");
            Console.Error.WriteLine(ex.StackTrace ?? "");
            return 2;
        }
    }

    static int Run(string[] args)
    {
        if (args.Length == 0 || args[0] is "--help" or "-h")
        {
            PrintHelp();
            return 0;
        }

        // Two invocation styles (same as cli-tekla):
        //   aware-rhino.exe <verb> [--json-stdin]    (canonical)
        //   aware-rhino.exe --json-stdin             (verb embedded in JSON body)
        string verb;
        JsonNode? stdinJson = null;

        if (args[0].StartsWith("--", StringComparison.Ordinal))
        {
            string buf;
            try { buf = Console.In.ReadToEnd(); }
            catch (Exception e)
            {
                Console.Error.WriteLine($"aware-rhino: stdin not readable: {e.Message}");
                return 2;
            }
            try { stdinJson = JsonNode.Parse(buf); }
            catch (Exception e)
            {
                Console.Error.WriteLine($"aware-rhino: stdin not JSON: {e.Message}");
                return 2;
            }
            verb = stdinJson?["verb"]?.GetValue<string>() ?? "";
            if (string.IsNullOrEmpty(verb))
            {
                Console.Error.WriteLine("aware-rhino: stdin JSON has no `verb` field and no verb on CLI");
                return 2;
            }
        }
        else
        {
            verb = args[0];
            // Read stdin only if --json-stdin was passed somewhere after the verb.
            var wantsStdin = args.Skip(1).Any(a => a == "--json-stdin");
            // exec, send-status, launch, close, and any verb that takes structured input → always need stdin
            if (wantsStdin || verb is "exec" or "send-status" or "launch" or "close")
            {
                string buf;
                try { buf = Console.In.ReadToEnd(); }
                catch (Exception e)
                {
                    Console.Error.WriteLine($"aware-rhino: stdin not readable: {e.Message}");
                    return 2;
                }
                if (!string.IsNullOrWhiteSpace(buf))
                {
                    try { stdinJson = JsonNode.Parse(buf); }
                    catch (Exception e)
                    {
                        Console.Error.WriteLine($"aware-rhino: stdin not JSON: {e.Message}");
                        return 2;
                    }
                }
            }
        }

        // `launch` doesn't need rhinocode (it spawns Rhino.exe directly), so
        // discover lazily — failing to find rhinocode shouldn't block a launch.
        RhinocodeClient? client = null;
        if (verb is not "launch")
        {
            try { client = new RhinocodeClient(); }
            catch (FileNotFoundException e)
            {
                Console.WriteLine(Receipts.GenericFail(verb, e.Message).ToJsonString());
                return 3;
            }
        }

        return verb switch
        {
            "exec"           => Exec.Run(client!, stdinJson),
            "list-instances" => ListInstances.Run(client!),
            "send-status"    => SendStatus.Run(client!, stdinJson),
            "launch"         => Launch.Run(stdinJson),
            "close"          => Close.Run(client!, stdinJson),
            _ => Unknown(verb),
        };
    }

    static int Unknown(string verb)
    {
        Console.Error.WriteLine($"aware-rhino: unknown verb '{verb}'. Try --help.");
        return 2;
    }

    static void PrintHelp()
    {
        Console.WriteLine("""
            aware-rhino — Rhino 8 sidecar (wraps McNeel's rhinocode CLI)

            Usage:
              aware-rhino <verb> [--json-stdin]
              aware-rhino --json-stdin            (verb embedded in JSON body)

            Verbs:
              exec             Compile + run an ad-hoc C# script against the active Rhino doc
              list-instances   Print running Rhino instances (PID + version + active doc)
              send-status      Display a transient status-bar message in Rhino
              launch           Spawn a fresh Rhino instance (optionally with a model + auto Script Server)
              close            Shut a running Rhino instance down cleanly (or force-kill with force=true)

            Exit codes:
              0  success
              1  no matching Rhino instance running / didn't exit cleanly
              2  API call failed / bad args / unknown verb
              3  rhinocode.exe (or Rhino.exe for launch) not found

            Prerequisites:
              - Rhino 8.11+ installed
              - For exec/send-status/list-instances: `StartScriptServer` active in Rhino
                (launch auto-starts it via /runscript flag if `start_script_server: true`, default true)
              - For multi-instance routing, pass `rhino_id` in the input JSON
            """);
    }
}
