// aware-rhino — runtime sidecar that wraps McNeel's rhinocode CLI for the AWARE
// runtime. Speaks the {verb,code,args} stdin-JSON contract shared with cli-tekla.

namespace AwareRhino;

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

        Console.Error.WriteLine($"aware-rhino: verb dispatch not yet wired (got '{args[0]}'). Coming in Task 8.");
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

            Prerequisites:
              - Rhino 8.11+ installed (rhinocode.exe must be discoverable)
              - In Rhino, run `StartScriptServer` once per session
              - For multi-instance routing, pass `rhino_id` in the input JSON
            """);
    }
}
