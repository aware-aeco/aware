using System.Text.Json.Nodes;
using AwareRevit.Sidecar.Verbs;
using Xunit;

namespace AwareRevit.Tests;

/// <summary>
/// A bake that REFUSED must not report success.
///
/// `exec` answers "did the script run?", and a script returning {ok:false} ran perfectly — so
/// without folding the nested verdict in, the envelope reads ok:true / exit 0 while the model
/// received nothing. floless.app decides from that envelope whether to tell the user their members
/// landed, so the gap would surface as "Inserted 4 members into Revit" against a model that
/// refused them. Found by review, reproduced against live Revit (a scene missing sourceId).
/// </summary>
public class BakeSceneVerdictTests
{
    static JsonObject Envelope(bool? nestedOk, bool topOk = true)
    {
        var receipt = new JsonObject { ["ok"] = topOk, ["verb"] = "bake-scene" };
        var result = new JsonObject { ["created"] = 0 };
        if (nestedOk is not null) result["ok"] = nestedOk.Value;
        receipt["result"] = result;
        return receipt;
    }

    [Fact]
    public void RefusedBake_FailsTheCommand()
    {
        var receipt = Envelope(nestedOk: false);
        var exit = BakeScene.ApplyBakeVerdict(receipt, execExit: 0);
        Assert.Equal(2, exit);
        Assert.False(receipt["ok"]!.GetValue<bool>());
        Assert.Contains("ok:false", receipt["error"]!.GetValue<string>());
    }

    [Fact]
    public void SuccessfulBake_StaysSuccessful()
    {
        var receipt = Envelope(nestedOk: true);
        var exit = BakeScene.ApplyBakeVerdict(receipt, execExit: 0);
        Assert.Equal(0, exit);
        Assert.True(receipt["ok"]!.GetValue<bool>());
        Assert.Null(receipt["error"]);
    }

    [Fact]
    public void MissingNestedOk_LeavesExecsVerdictAlone()
    {
        // Inventing a failure from an absent field would be its own silent bug.
        var receipt = Envelope(nestedOk: null);
        var exit = BakeScene.ApplyBakeVerdict(receipt, execExit: 0);
        Assert.Equal(0, exit);
        Assert.True(receipt["ok"]!.GetValue<bool>());
    }

    [Fact]
    public void NonObjectResult_LeavesExecsVerdictAlone()
    {
        var receipt = new JsonObject { ["ok"] = true, ["result"] = "not an object" };
        Assert.Equal(0, BakeScene.ApplyBakeVerdict(receipt, execExit: 0));
        Assert.True(receipt["ok"]!.GetValue<bool>());
    }

    [Fact]
    public void NonBooleanNestedOk_DoesNotThrowAndDoesNotFlip()
    {
        var receipt = new JsonObject
        {
            ["ok"] = true,
            ["result"] = new JsonObject { ["ok"] = "false" },   // a STRING, not a bool
        };
        Assert.Equal(0, BakeScene.ApplyBakeVerdict(receipt, execExit: 0));
        Assert.True(receipt["ok"]!.GetValue<bool>());
    }

    [Theory]
    [InlineData(1)]
    [InlineData(2)]
    public void AlreadyFailingExec_KeepsItsOwnExitCode(int execExit)
    {
        var receipt = Envelope(nestedOk: false, topOk: false);
        Assert.Equal(execExit, BakeScene.ApplyBakeVerdict(receipt, execExit));
    }
}
