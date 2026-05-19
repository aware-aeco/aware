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
            // exec, send-status, and any verb that takes structured input → always need stdin
            if (wantsStdin || verb is "exec" or "send-status")
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

        // Discover rhinocode once. All verbs share the client.
        RhinocodeClient client;
        try { client = new RhinocodeClient(); }
        catch (FileNotFoundException e)
        {
            Console.WriteLine(Receipts.GenericFail(verb, e.Message).ToJsonString());
            return 3;
        }

        return verb switch
        {
            "exec"           => Exec.Run(client, stdinJson),
            "list-instances" => ListInstances.Run(client),
            "send-status"    => SendStatus.Run(client, stdinJson),
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

            Exit codes:
              0  success
              1  no matching Rhino instance running
              2  API call failed / bad args / unknown verb
              3  rhinocode.exe not found (install Rhino 8.11+ or set RHINOCODE_EXE)

            Prerequisites:
              - Rhino 8.11+ installed
              - In Rhino, run `StartScriptServer` once per session
              - For multi-instance routing, pass `rhino_id` in the input JSON
            """);
    }
}
