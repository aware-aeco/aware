// aware-sketchup — runtime sidecar that bridges the AWARE {verb,code,args}
// stdin-JSON contract to SketchUp's in-process Ruby API via a TCP socket
// hosted by an auto-loaded SketchUp extension (BridgeAssets/aware_sketchup_bridge.rb).
//
// Verb shape mirrors cli-tekla and cli-rhino:
//   exec | list-instances | send-status | launch | close
//
// Plus sidecar-only management flags:
//   --install-bridge     copy Ruby bridge into the user's SketchUp Plugins folder
//   --uninstall-bridge   remove it
//   --bridge-status      report whether bridge is present + up-to-date
//   --version            print version + exit
//   --help               print help + exit

using System.Text.Json.Nodes;
using AwareSketchup.Verbs;

namespace AwareSketchup;

internal static class Program
{
    public const string Version = "0.34.0";

    static int Main(string[] args)
    {
        Console.InputEncoding  = new System.Text.UTF8Encoding(false);
        Console.OutputEncoding = new System.Text.UTF8Encoding(false);

        try { return Run(args); }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"aware-sketchup: unhandled error: {ex.Message}");
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
        if (args[0] is "--version")
        {
            Console.WriteLine(Version);
            return 0;
        }

        // Sidecar-management flags (don't touch SketchUp at all).
        switch (args[0])
        {
            case "--install-bridge":
                return BridgeInstaller.Install(args);
            case "--uninstall-bridge":
                return BridgeInstaller.Uninstall(args);
            case "--bridge-status":
                return BridgeInstaller.Status(args);
        }

        // Two invocation styles (same as cli-tekla / cli-rhino):
        //   aware-sketchup <verb> [--json-stdin]
        //   aware-sketchup --json-stdin                (verb embedded in JSON body)
        string verb;
        JsonNode? stdinJson = null;

        if (args[0].StartsWith("--", StringComparison.Ordinal))
        {
            string buf;
            try { buf = Console.In.ReadToEnd(); }
            catch (Exception e)
            {
                Console.Error.WriteLine($"aware-sketchup: stdin not readable: {e.Message}");
                return 2;
            }
            try { stdinJson = JsonNode.Parse(buf); }
            catch (Exception e)
            {
                Console.Error.WriteLine($"aware-sketchup: stdin not JSON: {e.Message}");
                return 2;
            }
            verb = stdinJson?["verb"]?.GetValue<string>() ?? "";
            if (string.IsNullOrEmpty(verb))
            {
                Console.Error.WriteLine("aware-sketchup: stdin JSON has no `verb` field and no verb on CLI");
                return 2;
            }
        }
        else
        {
            verb = args[0];
            var wantsStdin = args.Skip(1).Any(a => a == "--json-stdin");
            // exec, send-status, close are always stdin-driven for their params.
            if (wantsStdin || verb is "exec" or "send-status" or "close" or "launch")
            {
                string buf;
                try { buf = Console.In.ReadToEnd(); }
                catch (Exception e)
                {
                    Console.Error.WriteLine($"aware-sketchup: stdin not readable: {e.Message}");
                    return 2;
                }
                if (!string.IsNullOrWhiteSpace(buf))
                {
                    try { stdinJson = JsonNode.Parse(buf); }
                    catch (Exception e)
                    {
                        Console.Error.WriteLine($"aware-sketchup: stdin not JSON: {e.Message}");
                        return 2;
                    }
                }
            }
        }

        return verb switch
        {
            "exec"           => Exec.Run(stdinJson),
            "list-instances" => ListInstances.Run(),
            "send-status"    => SendStatus.Run(stdinJson),
            "launch"         => Launch.Run(stdinJson),
            "close"          => Close.Run(stdinJson),
            _                => Unknown(verb),
        };
    }

    static int Unknown(string verb)
    {
        Console.Error.WriteLine($"aware-sketchup: unknown verb '{verb}'. Try --help.");
        return 2;
    }

    static void PrintHelp()
    {
        Console.WriteLine("""
            aware-sketchup — SketchUp 2026 sidecar (in-process Ruby bridge via TCP)

            Usage:
              aware-sketchup <verb> [--json-stdin]
              aware-sketchup --json-stdin                  (verb embedded in JSON body)

            Verbs:
              exec             Evaluate a Ruby script against Sketchup.active_model
              list-instances   Print running SketchUp instances (PID + version + port + model)
              send-status      Display a status-bar message in SketchUp
              launch           Spawn a fresh SketchUp process (optionally opens a model)
              close            Save + close one (or all) SketchUp instance(s)

            Bridge management:
              --install-bridge [--target-year YYYY]    Copy Ruby bridge into Plugins/
              --uninstall-bridge [--target-year YYYY]  Remove it
              --bridge-status [--target-year YYYY]     Report install / version status

            Exit codes:
              0  success
              1  no matching SketchUp instance running / bridge not loaded
              2  bad args / unknown verb / I/O error / sidecar timeout
              3  SketchUp install not found (launch / install-bridge)

            Prerequisites:
              - SketchUp 2026 installed (or pass --target-year for older)
              - aware_sketchup_bridge.rb installed via --install-bridge once
              - SketchUp restarted after install (extension auto-loads on next launch)
            """);
    }
}
