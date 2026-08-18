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
        Assert.Contains("created object read-back failed", code);
        Assert.Contains("ownership read-back differs from the request", code);
        Assert.Contains("commit-state-uncertain", code);
        Assert.Contains("active_view = doc.Views.ActiveView", code);
        Assert.Contains("isinstance(active_view, Rhino.Display.RhinoPageView)", code);
        Assert.Contains("elif not active_view.ActiveViewport.ZoomExtents()", code);
        Assert.Contains("view-frame-failed", code);
        Assert.Contains("Rhino.Geometry.Extrusion()", code);
        Assert.Contains("System.Drawing.Color.FromArgb(164, 169, 172)", code);
        Assert.Contains("ObjectColorSource.ColorFromLayer", code);
        Assert.Contains("extrusion.SetPathAndUp(", code);
        Assert.Contains("extrusion.SetOuterProfile(outer_curve, True)", code);
        Assert.Contains("extrusion.AddInnerProfile(inner_curve)", code);
        Assert.Contains("extrusion.ToBrep()", code);
        Assert.Contains("created.Geometry", code);
        Assert.Contains("\"document read-back\"", code);
        Assert.Contains("VolumeMassProperties.Compute(brep)", code);
        Assert.Contains("\"local extrusion\", False", code);
        Assert.Contains("\"transformed extrusion\", False", code);
        Assert.DoesNotContain("measured_profile_areas", code);
        Assert.DoesNotContain("measured_profile_area", code);
        Assert.Contains("\"document read-back\",\n            True", code);
        Assert.Contains("\"volumeBasis\"", code);
        Assert.DoesNotContain("304.8", code);
        Assert.DoesNotContain("25.4", code);
        Assert.DoesNotContain("real_thickness", code);
    }

    [Fact]
    public void EmbeddedMaterializerConsumesOnlyTheNormalizedProfilePlan()
    {
        var code = BakeSceneScript.Code;

        Assert.Contains("profile_plan = supported_row.get(\"profile\")", code);
        Assert.Contains("geometry_revision != \"rhino-profile-v3\"", code);
        Assert.Contains("if shape == \"i\":", code);
        Assert.Contains("if shape == \"channel\":", code);
        Assert.Contains("if shape == \"angle\":", code);
        Assert.Contains("if shape == \"rhs\":", code);
        Assert.Contains("dimensions = profile[\"dimensions\"]", code);
        Assert.DoesNotContain("shape_of", code);
        Assert.DoesNotContain("element.get(\"xsection\")", code);
        Assert.DoesNotContain("meta.get(\"profile\")", code[..code.IndexOf("element_meta =", StringComparison.Ordinal)]);
        Assert.DoesNotContain("Rhino.Geometry.Surface.CreateExtrusion", code);
        Assert.DoesNotContain("CapPlanarHoles", code);
    }

    [Fact]
    public void DirectExtrusionAndUndoSafetyOrderingArePinned()
    {
        var code = BakeSceneScript.Code;

        var begin = code.IndexOf("undo_serial = doc.BeginUndoRecord", StringComparison.Ordinal);
        var zeroCheck = code.IndexOf("if not undo_serial:", begin, StringComparison.Ordinal);
        var layerRead = code.IndexOf("layer = doc.Layers.FindName", zeroCheck, StringComparison.Ordinal);
        var layerWrite = code.IndexOf("doc.Layers.Add(layer)", layerRead, StringComparison.Ordinal);
        var objectWrite = code.IndexOf("doc.Objects.AddBrep", layerWrite, StringComparison.Ordinal);
        Assert.True(begin >= 0);
        Assert.True(begin < zeroCheck);
        Assert.True(zeroCheck < layerRead);
        Assert.True(layerRead < layerWrite);
        Assert.True(layerWrite < objectWrite);

        var localValidation = code.IndexOf("\"local extrusion\"", StringComparison.Ordinal);
        var transform = code.IndexOf("brep.Transform(transform)", localValidation, StringComparison.Ordinal);
        var transformedValidation = code.IndexOf("\"transformed extrusion\"", transform, StringComparison.Ordinal);
        var add = code.IndexOf("doc.Objects.AddBrep", transformedValidation, StringComparison.Ordinal);
        var readback = code.IndexOf("\"document read-back\"", add, StringComparison.Ordinal);
        var retire = code.IndexOf("for old in prior:", readback, StringComparison.Ordinal);
        Assert.True(localValidation < transform);
        Assert.True(transform < transformedValidation);
        Assert.True(transformedValidation < add);
        Assert.True(add < readback);
        Assert.True(readback < retire);
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
        Assert.Equal("AWARE_BAKE_V2:" + hash, ownership["marker"]!.GetValue<string>());
        Assert.Equal("rhino-profile-v3", ownership["geometryRevision"]!.GetValue<string>());

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
    public void AZeroUndoSerialAbortsAsUndoUnavailableBeforeAnyMutation()
    {
        var code = BakeSceneScript.Code;

        // The guard must reject the zero serial by raising the *coded* failure,
        // with no document write between Rhino's refusal and the abort.
        var begin = code.IndexOf("undo_serial = doc.BeginUndoRecord", StringComparison.Ordinal);
        var zeroCheck = code.IndexOf("if not undo_serial:", begin, StringComparison.Ordinal);
        var raise = code.IndexOf("raise BakeFailure(", zeroCheck, StringComparison.Ordinal);
        var reason = code.IndexOf("\"undo-unavailable\"", raise, StringComparison.Ordinal);
        Assert.True(begin >= 0);
        Assert.True(begin < zeroCheck);
        Assert.True(zeroCheck < raise);
        Assert.True(raise < reason);

        // No mutation may appear between BeginUndoRecord and the abort — the
        // whole point of #324 is that a zero serial cannot reach a write.
        var betweenBeginAndRaise = code[begin..raise];
        Assert.DoesNotContain("doc.Layers.Add", betweenBeginAndRaise);
        Assert.DoesNotContain("doc.Objects.AddBrep", betweenBeginAndRaise);
        Assert.DoesNotContain("doc.Objects.Delete", betweenBeginAndRaise);

        // ...and the first mutation of any kind still sits after the abort.
        var firstLayerWrite = code.IndexOf("doc.Layers.Add(layer)", raise, StringComparison.Ordinal);
        var firstObjectWrite = code.IndexOf("doc.Objects.AddBrep", raise, StringComparison.Ordinal);
        Assert.True(raise < firstLayerWrite);
        Assert.True(raise < firstObjectWrite);
    }

    [Fact]
    public void CodedFailuresKeepTheirReceiptCodeAndOthersStayMaterializationFailed()
    {
        var code = BakeSceneScript.Code;

        // The rollback path must read the code off the coded failure rather than
        // stamping every exception `materialization-failed`; anything uncoded
        // keeps the original default.
        Assert.Contains("class BakeFailure(Exception):", code);
        Assert.Contains("scene_level = isinstance(ex, BakeFailure)", code);
        Assert.Contains(
            "cause_code = ex.code if scene_level else \"materialization-failed\"",
            code);

        var causeCode = code.IndexOf("scene_level = isinstance(ex, BakeFailure)", StringComparison.Ordinal);
        var perPlanRow = code.IndexOf("cause_code if cause else \"batch-aborted\"", causeCode, StringComparison.Ordinal);
        Assert.True(causeCode >= 0);
        Assert.True(causeCode < perPlanRow);

        // `scene` is a legal record id — IsValidId reserves nothing — so a
        // document-level refusal must not be pinned onto a member that happens
        // to be called `scene`. The plan match is suppressed for scene-level
        // failures, and the synthetic row hardcodes the id rather than reusing
        // active_id.
        Assert.Contains("cause = not scene_level and plan[\"id\"] == active_id and not cause_seen", code);
        Assert.Contains("\"scene\" if scene_level else active_id, \"scene\", \"failed\", cause_code, str(ex)", code);
    }

    [Fact]
    public void ProgramAndProjectShipTheBakeVerbAndScript()
    {
        Assert.NotEmpty(BakeSceneScript.Code);
        Assert.Equal("bake-scene", BakeScene.Verb);
    }
}
