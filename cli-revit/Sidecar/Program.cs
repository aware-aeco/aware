// aware-revit — runtime sidecar that brokers between AWARE CLI and the
// in-Revit AwareRevit.dll add-in over a named pipe. Speaks the {verb, code,
// args} stdin-JSON contract shared with cli-tekla / cli-rhino.

using System.Text.Json.Nodes;

namespace AwareRevit.Sidecar;

internal static class Program
{
    static int Main(string[] args)
    {
        Console.InputEncoding  = new System.Text.UTF8Encoding(false);
        Console.OutputEncoding = new System.Text.UTF8Encoding(false);

        if (args.Length == 0 || args[0] is "--help" or "-h")
        {
            PrintHelp();
            return 0;
        }

        // Two invocation styles (same as cli-tekla / cli-rhino):
        //   aware-revit.exe <verb> [--json-stdin]   (canonical)
        //   aware-revit.exe --json-stdin            (verb embedded in JSON body)
        string verb;
        JsonNode? stdinJson = null;

        if (args[0].StartsWith("--", StringComparison.Ordinal))
        {
            string buf;
            try { buf = Console.In.ReadToEnd(); }
            catch (Exception e)
            {
                Console.Error.WriteLine($"aware-revit: stdin not readable: {e.Message}");
                return 2;
            }
            try { stdinJson = JsonNode.Parse(buf); }
            catch (Exception e)
            {
                Console.Error.WriteLine($"aware-revit: stdin not JSON: {e.Message}");
                return 2;
            }
            verb = stdinJson?["verb"]?.GetValue<string>() ?? "";
            if (string.IsNullOrEmpty(verb))
            {
                Console.Error.WriteLine("aware-revit: stdin JSON has no `verb` field and no verb on CLI");
                return 2;
            }
        }
        else
        {
            verb = args[0];
            var wantsStdin = args.Skip(1).Any(a => a == "--json-stdin");
            if (wantsStdin || verb is "exec" or "send-status" or "launch" or "close")
            {
                string buf;
                try { buf = Console.In.ReadToEnd(); }
                catch (Exception e)
                {
                    Console.Error.WriteLine($"aware-revit: stdin not readable: {e.Message}");
                    return 2;
                }
                if (!string.IsNullOrWhiteSpace(buf))
                {
                    try { stdinJson = JsonNode.Parse(buf); }
                    catch (Exception e)
                    {
                        Console.Error.WriteLine($"aware-revit: stdin not JSON: {e.Message}");
                        return 2;
                    }
                }
            }
        }

        return verb switch
        {
            "list-instances" => Verbs.ListInstances.RunAsync().GetAwaiter().GetResult(),
            _ => Unknown(verb),
        };
    }

    static int Unknown(string verb)
    {
        Console.Error.WriteLine($"aware-revit: unknown verb '{verb}'. Try --help.");
        return 2;
    }

    static void PrintHelp()
    {
        Console.WriteLine("""
            aware-revit — Revit sidecar (talks to AwareRevit.dll add-in over named pipes)

            Usage:
              aware-revit <verb> [--json-stdin]
              aware-revit --json-stdin            (verb embedded in JSON body)

            Verbs:
              exec             Compile + run an ad-hoc C# script against the active Revit doc
              list-instances   Print running Revit instances (pid + version + addin status)
              send-status      Show a transient TaskDialog message in Revit
              launch           Start a Revit instance (optionally opening a model)
              close            Cleanly close a Revit instance via the add-in

            Exit codes:
              0  success
              1  no matching Revit instance running
              2  bad args / unknown verb / pipe protocol error
              3  Revit not installed at the requested version
              4  ambiguous target (multiple matches, no --pid)
              6  add-in not loaded (pipe connect failed)
            """);
    }
}
