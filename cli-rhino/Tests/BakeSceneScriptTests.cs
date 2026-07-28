using AwareRhino;
using AwareRhino.Verbs;
using Xunit;

namespace AwareRhino.Tests;

public class BakeSceneScriptTests
{
    [Fact]
    public void EmbeddedMaterializerCarriesTheRhinoSpecificSafetyContract()
    {
        var code = BakeSceneScript.Code;
        Assert.True(code.Length > 10_000);
        Assert.Contains("Rhino.RhinoMath.UnitScale(", code);
        Assert.Contains("Rhino.UnitSystem.Millimeters, doc.ModelUnitSystem", code);
        Assert.Contains("doc.Objects.AddBrep", code);
        Assert.Contains("doc.BeginUndoRecord", code);
        Assert.Contains("doc.EndUndoRecord", code);
        Assert.Contains("doc.Objects.Undelete", code);
        Assert.Contains("doc.Objects.Delete", code);
        Assert.Contains("doc.Layers.FindName", code);
        Assert.Contains("rectangular_outline(width, depth)", code);
        Assert.Contains("created object read-back failed", code);
        Assert.Contains("ownership read-back differs from the request", code);
        Assert.Contains("commit-state-uncertain", code);
        Assert.Contains("active_view = doc.Views.ActiveView", code);
        Assert.Contains("isinstance(active_view, Rhino.Display.RhinoPageView)", code);
        Assert.Contains("elif not active_view.ActiveViewport.ZoomExtents()", code);
        Assert.Contains("view-frame-failed", code);
        Assert.Contains("if capped is None:", code);
        Assert.Contains("not brep.IsSolid", code);
        Assert.Contains("valid closed solid", code);
        Assert.DoesNotContain("304.8", code);
        Assert.DoesNotContain("25.4", code);
        Assert.DoesNotContain("real_thickness", code);
    }

    [Fact]
    public void OwnershipVocabularyIsNamespacedAndInjectedRatherThanRestated()
    {
        var hash = new string('a', 64);
        var ownership = BakeScene.BuildOwnership("drop-a", hash);
        Assert.Equal("AWARE.BAKE.SOURCE_ID", ownership["sourceIdKey"]!.GetValue<string>());
        Assert.Equal("AWARE.BAKE.RECORD_ID", ownership["recordIdKey"]!.GetValue<string>());
        Assert.Equal("AWARE.BAKE.SCENE_HASH", ownership["sceneHashKey"]!.GetValue<string>());
        Assert.Equal("AWARE.BAKE.MARKER", ownership["markerKey"]!.GetValue<string>());
        Assert.Equal("AWARE_BAKE_V1:" + hash, ownership["marker"]!.GetValue<string>());

        Assert.DoesNotContain(BakeSceneRules.SourceIdKey, BakeSceneScript.Code);
        Assert.DoesNotContain(BakeSceneRules.RecordIdKey, BakeSceneScript.Code);
    }

    [Fact]
    public void ViewFramingRunsAfterCommitAndCannotEnterGeometryRollback()
    {
        var code = BakeSceneScript.Code;
        var frame = code.IndexOf("frame_failure = None", StringComparison.Ordinal);
        var commitClosed = code.LastIndexOf("undo_serial = 0", frame, StringComparison.Ordinal);
        var redraw = code.IndexOf("doc.Views.Redraw()", frame, StringComparison.Ordinal);
        var warning = code.IndexOf("if frame_failure:", frame, StringComparison.Ordinal);
        var success = code.IndexOf("return envelope(True", warning, StringComparison.Ordinal);
        var rollback = code.IndexOf("cleanup_ok = True", success, StringComparison.Ordinal);

        Assert.True(commitClosed >= 0);
        Assert.True(commitClosed < frame);
        Assert.True(frame < redraw);
        Assert.True(redraw < warning);
        Assert.True(warning < success);
        Assert.True(success < rollback);
        Assert.Contains("elif not active_view.ActiveViewport.ZoomExtents()", code);
        Assert.Contains("\"view-frame-failed\"", code);
    }

    [Fact]
    public void ProgramAndProjectShipTheBakeVerbAndScript()
    {
        Assert.NotEmpty(BakeSceneScript.Code);
        Assert.Equal("bake-scene", BakeScene.Verb);
    }
}
