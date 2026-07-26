// bake-scene verb — materialize a canonical millimetre scene as source-owned
// native Revit elements. WRITE: needs Revit open with a project model and the
// AwareRevit add-in loaded. The embedded BakeSceneScript runs through the SAME
// Roslyn path as `exec`, so it shares target resolution, the pipe transport,
// and the receipt envelope.

using System.Text.Json;
using System.Text.Json.Nodes;

namespace AwareRevit.Sidecar.Verbs;

internal static class BakeScene
{
    public static async Task<int> RunAsync(JsonNode? input, string? pipeNameOverride = null)
    {
        var scene = input?["scene"];
        if (scene is null)
        {
            Console.WriteLine(Receipts.GenericFail("bake-scene",
                "missing required field: scene (the 3D scene to bake into the model)").ToJsonString());
            return 2;
        }

        var version = input?["version"]?.GetValue<string>();

        // Identity of THIS materialization: the canonical scene plus the Revit
        // version it is being baked into. Stamped onto every element the bake
        // creates, so a later bake can tell an element it owns from one an older
        // bake of the same source produced.
        var materializationHash = ComputeMaterializationHash(scene, version);

        // Deliberately NO status announce. Tekla's bake-scene puts "adding N
        // objects..." on the status bar; Revit's only equivalent is a MODAL
        // TaskDialog, which would block the bake it is announcing. See the
        // comment at the top of BakeSceneScript.Body.
        var execInput = new JsonObject
        {
            ["verb"] = "exec",
            ["version"] = version,
            ["host_pid"] = input?["host_pid"]?.DeepClone(),
            // The script owns its own Revit Transaction (one Undo step for the
            // whole bake, with a real rollback on failure), so the exec host must
            // NOT wrap it in another one — nested transactions throw.
            ["transaction"] = "none",
            ["code"] = BakeSceneScript.Code,
            ["args"] = new JsonObject
            {
                ["scene"] = scene.DeepClone(),
                ["materializationHash"] = materializationHash,
            },
        };

        // Run through exec, then re-label the envelope as this verb's.
        using var sw = new StringWriter();
        var originalOut = Console.Out;
        Console.SetOut(sw);
        int execExit;
        try { execExit = await Exec.RunAsync(execInput, pipeNameOverride); }
        finally { Console.SetOut(originalOut); }

        var receipt = JsonNode.Parse(sw.ToString().Trim()) as JsonObject;
        if (receipt is null)
        {
            Console.WriteLine(Receipts.GenericFail("bake-scene",
                "exec produced no receipt for bake-scene").ToJsonString());
            return execExit == 0 ? 2 : execExit;
        }
        receipt["verb"] = "bake-scene";

        // A bake that REFUSED is not a successful command. `exec` reports whether the SCRIPT ran,
        // and a script that returns {ok:false} ran perfectly — so left alone the envelope says
        // ok:true with exit 0 while the model received nothing. A caller that trusts the envelope
        // (floless.app does, to decide whether to tell the user their members landed) would report
        // a successful insert into a model that refused it. Mirror the SketchUp verb: a receipt
        // that says it failed makes the command fail.
        //
        // Only an explicit `false` flips the verdict — a missing nested `ok` leaves exec's own
        // result standing rather than inventing a failure.
        var exit = ApplyBakeVerdict(receipt, execExit);
        Console.WriteLine(receipt.ToJsonString());
        return exit;
    }

    /// <summary>
    /// Fold the nested bake receipt's verdict into the top-level envelope, returning the exit code
    /// the command should report. Mutates <paramref name="receipt"/> in place.
    /// Extracted so the rule is testable without a live Revit on the other end of the pipe.
    /// </summary>
    internal static int ApplyBakeVerdict(JsonObject receipt, int execExit)
    {
        if (execExit != 0) return execExit;   // exec already failed; its verdict stands
        var refused = receipt["result"] is JsonObject nested
                   && nested["ok"] is JsonValue okValue
                   && okValue.TryGetValue<bool>(out var okFlag)
                   && !okFlag;
        if (!refused) return execExit;
        receipt["ok"] = false;
        receipt["error"] = "bake-scene returned ok:false; inspect the structured result receipt";
        return 2;
    }

    /// <summary>Identity of a materialization: the canonical scene JSON bound to
    /// the Revit version it targets. Mirrors cli-tekla's
    /// ComputeBakeMaterializationHash, with the Revit tag so the two hosts can
    /// never collide on a hash.</summary>
    internal static string ComputeMaterializationHash(JsonNode scene, string? version)
    {
        var payload = "revit-scene-materializer-v1\0"
            + (version ?? "running-host")
            + "\0"
            + scene.ToJsonString(new JsonSerializerOptions { WriteIndented = false });
        using var sha = System.Security.Cryptography.SHA256.Create();
        var digest = sha.ComputeHash(System.Text.Encoding.UTF8.GetBytes(payload));
        return string.Concat(digest.Select(b => b.ToString("x2", System.Globalization.CultureInfo.InvariantCulture)));
    }
}
