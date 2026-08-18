using Xunit;
using System.Text.Json.Nodes;
using System;

namespace AwareTekla.Tests;

public sealed class CommitPolicyTests
{
    [Fact]
    public void BridgeTestHostIs64BitForTekla2026Compatibility()
    {
        Assert.True(Environment.Is64BitProcess);
    }

    [Fact]
    public void BakeScript_ParsesWithNoSyntaxErrors()
    {
        // The bake script is a raw string compiled by Roslyn only at bake time — a syntax slip (e.g. a
        // `//` comment swallowing the rest of a dense one-liner) otherwise surfaces ONLY on a live Tekla.
        // Parse it here as a C# script so brace/syntax errors fail the build instead. Parsing is
        // syntax-only, so it needs no Tekla assemblies.
        var tree = Microsoft.CodeAnalysis.CSharp.CSharpSyntaxTree.ParseText(
            BakeSceneScript.Code,
            new Microsoft.CodeAnalysis.CSharp.CSharpParseOptions(
                kind: Microsoft.CodeAnalysis.SourceCodeKind.Script));
        var errors = new System.Collections.Generic.List<string>();
        foreach (var d in tree.GetDiagnostics())
            if (d.Severity == Microsoft.CodeAnalysis.DiagnosticSeverity.Error)
                errors.Add(d.ToString());
        Assert.True(errors.Count == 0,
            "BakeSceneScript.Code has syntax errors:\n" + string.Join("\n", errors));
    }

    [Fact]
    public void BakeScript_ClassifiesNestedRecordsEvenWhenTheParentIdIsRejected()
    {
        // #327: a rejected parent id must withhold only the parent's OWN row, never its nested
        // collection — the children are records in their own right, and skipping them drops them
        // from the receipt AND from the scene-wide id set, so a duplicate beneath a bad parent
        // escapes detection. The Tekla rules live in script text with no executable harness here,
        // so the property is pinned on the source: the early `continue` that took the children
        // down with the parent must be gone from all three top-level loops.
        var code = BakeSceneScript.Code;

        Assert.DoesNotContain("if (!acceptId(id,kind)) continue;", code);
        Assert.DoesNotContain("if(!acceptId(id,kind))continue;", code);

        // The parent's own classification is now gated on acceptance...
        Assert.Contains("var accepted = acceptId(id,kind);", code);
        Assert.Contains("if(acceptId(id,kind)) {", code);
        Assert.Contains("if(acceptId(id,kind)){", code);

        // ...while every nested walk still runs. Each must sit AFTER its parent's gate,
        // outside it — reachable whatever the parent id did.
        var elementAccept = code.IndexOf("var accepted = acceptId(id,kind);", StringComparison.Ordinal);
        var holes = code.IndexOf("var holes=list(el,\"holes\");", elementAccept, StringComparison.Ordinal);
        var instances = code.IndexOf("var ins=list(op,\"instances\");", holes, StringComparison.Ordinal);
        var axes = code.IndexOf("var axes=list(rf,\"axes\");", instances, StringComparison.Ordinal);
        var levels = code.IndexOf("var levels=list(rf,\"levels\");", axes, StringComparison.Ordinal);
        Assert.True(elementAccept >= 0);
        Assert.True(holes > elementAccept);
        Assert.True(instances > holes);
        Assert.True(axes > instances);
        Assert.True(levels > axes);

        // The nested ids still enter the scene-wide set, which is what makes a duplicate
        // among them detectable at all.
        Assert.Contains("if(acceptId(hid,\"opening\"))", code);
        Assert.Contains("if(acceptId(aid,\"grid-axis\"))", code);
        Assert.Contains("if(acceptId(lid,\"grid-level\"))", code);
    }

    [Fact]
    public void ExecRetainsAutomaticCommitPolicy()
    {
        Assert.Equal(
            Program.ScriptCommitPolicy.Automatic,
            Program.CommitPolicyForVerb("exec"));
    }

    [Fact]
    public void BakeSceneOwnsItsCommitBoundary()
    {
        Assert.Equal(
            Program.ScriptCommitPolicy.ScriptOwned,
            Program.CommitPolicyForVerb("bake-scene"));
    }

    [Fact]
    public void FailedBakeReceiptPropagatesThroughTheCommandFailurePath()
    {
        var failed = JsonNode.Parse("{\"ok\":false,\"failed\":[{\"id\":\"scene\"}]}");

        Assert.True(Program.ScriptResultReportsFailure("bake-scene", failed));
        Assert.False(Program.ScriptResultReportsFailure("exec", failed));
        Assert.False(Program.ScriptResultReportsFailure(
            "bake-scene",
            JsonNode.Parse("{\"ok\":true}")));
    }

    [Fact]
    public void JsonBomIsRemovedForExplicitVerbPipelines()
    {
        Assert.Equal("{\"version\":\"2026.0\"}", Program.TrimJsonBom("\uFEFF{\"version\":\"2026.0\"}"));
    }

    [Fact]
    public void MaterializationHashIsStableAndVersionSensitive()
    {
        var scene = JsonNode.Parse("{\"meta\":{\"sourceId\":\"s\",\"sceneHash\":\"h\"},\"elements\":[]}")!;
        var first = Program.ComputeBakeMaterializationHash(scene, "2025.0");

        Assert.Equal("tekla-connection-materializer-v2", Program.BakeMaterializerIdentity);
        Assert.Equal(first, Program.ComputeBakeMaterializationHash(scene, "2025.0"));
        Assert.NotEqual(first, Program.ComputeBakeMaterializationHash(scene, "2026.0"));
        Assert.Equal(64, first.Length);
    }

    [Fact]
    public void BakeSceneRequiresArrayCollectionsBeforeMutation()
    {
        Assert.Contains("scene.elements must be an array", BakeSceneScript.Code);
        Assert.Contains("scene.operations must be an array", BakeSceneScript.Code);
        Assert.Contains("scene.referenceSystems must be an array", BakeSceneScript.Code);
    }

    [Fact]
    public void BakeSceneRetiresOnlyNamespacedOwnedObjects()
    {
        Assert.Contains("AWARE_SRC_V1:", BakeSceneScript.Code);
        Assert.Contains("AWARE_SCN_V1:", BakeSceneScript.Code);
        Assert.Contains("SetUserProperty(\"USER_FIELD_1\",sourceKey)", BakeSceneScript.Code);
        Assert.Contains("SetUserProperty(\"USER_FIELD_3\",sceneKey)", BakeSceneScript.Code);
        Assert.Contains("(src==sourceKey||src==sourceId)", BakeSceneScript.Code);
        Assert.Contains("AWARE_BAKE_V1:", BakeSceneScript.Code);
        Assert.Contains("owner.Length==78", BakeSceneScript.Code);
        Assert.Contains("owner.Skip(14).All(c=>", BakeSceneScript.Code);
    }

    [Fact]
    public void BakeSceneUsesGlobalCoordinatesAndRestoresTheUsersWorkPlane()
    {
        Assert.Contains("new TransformationPlane()", BakeSceneScript.Code);
        Assert.Contains("SetCurrentTransformationPlane(previousScenePlane)", BakeSceneScript.Code);
    }

    [Fact]
    public void StableRecordIdentityUsesAndVerifiesALosslessDigest()
    {
        Assert.Contains("AWARE_RID_V1:", BakeSceneScript.Code);
        Assert.Contains("sid!=recordKey(item.Value)", BakeSceneScript.Code);
    }

    [Fact]
    public void BoltChildrenAreGeometryCheckedBeforeRealization()
    {
        Assert.Contains("shank axis must pass through the instance point", BakeSceneScript.Code);
        Assert.Contains("validateAxialChild", BakeSceneScript.Code);
        Assert.Contains("claimedBoltChildren", BakeSceneScript.Code);
    }

    [Fact]
    public void RelationshipsCannotTargetBoltArrayRealizedChildren()
    {
        Assert.Contains(
            "bolt participants must reference independently materialized physical parts",
            BakeSceneScript.Code);
        Assert.Contains(
            "fillet weld participants must be independently materialized parts",
            BakeSceneScript.Code);
        Assert.Contains(
            "boolean target must resolve to an independently materialized part",
            BakeSceneScript.Code);
        Assert.Contains("realizedChildren.ContainsKey(pa)", BakeSceneScript.Code);
    }

    [Fact]
    public void BoltControlFieldsAreValidatedBeforeMutation()
    {
        Assert.Contains("boltType must be `shop` or `site`", BakeSceneScript.Code);
        Assert.Contains("threadInMaterial must be boolean", BakeSceneScript.Code);
        Assert.Contains("components.\"+field+\" must be boolean", BakeSceneScript.Code);
    }

    [Fact]
    public void WeldBooleanControlsAreValidatedBeforeMutation()
    {
        Assert.Contains(
            "fillet weld around and shop fields must be boolean",
            BakeSceneScript.Code);
        Assert.Contains("AroundWeld=(bool)op[\"around\"]", BakeSceneScript.Code);
        Assert.Contains("ShopWeld=(bool)op[\"shop\"]", BakeSceneScript.Code);
    }

    [Fact]
    public void AuthoredNativeFastenerSlotsAreMappedOrRejectedBeforeMutation()
    {
        Assert.Contains(
            "components.bolt must be true because every bolt instance authors shankId and headId",
            BakeSceneScript.Code);
        Assert.Contains("exceeds Tekla BoltArray's \"+maxSlots+\" native slots", BakeSceneScript.Code);
        Assert.Contains(
            "every bolt instance must author the same \"+field+\" cardinality",
            BakeSceneScript.Code);
        Assert.Contains("Nut1=nutSlots>=1,Nut2=nutSlots>=2", BakeSceneScript.Code);
        Assert.Contains(
            "Washer1=washerSlots>=1,Washer2=washerSlots>=2,Washer3=washerSlots>=3",
            BakeSceneScript.Code);
        Assert.Contains(
            "must agree with the authored \"+component.Item1+\"Ids cardinality",
            BakeSceneScript.Code);
    }

    [Fact]
    public void NativeBoltContractIsAuthoritativelyReadBackBeforeRetirement()
    {
        Assert.Contains(
            "native BoltArray parameter/participant read-back differs from the authored bolt contract",
            BakeSceneScript.Code);
        Assert.Contains("selected.PartToBoltTo.Identifier.GUID", BakeSceneScript.Code);
        Assert.Contains("selected.BoltStandard", BakeSceneScript.Code);
        Assert.Contains("nativeById[id]=selected", BakeSceneScript.Code);
    }

    [Fact]
    public void NativeBoltMapsAndReadsBackEveryAuthoredPly()
    {
        Assert.DoesNotContain("effects.Count!=2", BakeSceneScript.Code);
        Assert.Contains("effects.Count<2||effects.Count>5", BakeSceneScript.Code);
        Assert.Contains("AddOtherPartToBolt", BakeSceneScript.Code);
        Assert.Contains("GetOtherPartsToBolt", BakeSceneScript.Code);
        Assert.Contains("Hole3=otherParticipantIds.Count>=1", BakeSceneScript.Code);
        Assert.Contains("Hole4=otherParticipantIds.Count>=2", BakeSceneScript.Code);
        Assert.Contains("Hole5=otherParticipantIds.Count>=3", BakeSceneScript.Code);
    }

    [Fact]
    public void NativeBoltLiveFixtureCoversTwoAndFivePlyBoundaries()
    {
        var fixturePath = System.IO.Path.GetFullPath(System.IO.Path.Combine(
            AppContext.BaseDirectory,
            "..", "..", "..", "Fixtures", "native-bolt-ply-boundaries-scene.json"));
        var fixture = JsonNode.Parse(System.IO.File.ReadAllText(fixturePath))!;
        var operations = fixture["scene"]!["operations"]!.AsArray();

        Assert.Equal(2, operations[0]!["instances"]![0]!["holeEffects"]!.AsArray().Count);
        Assert.Equal(5, operations[1]!["instances"]![0]!["holeEffects"]!.AsArray().Count);
    }

    [Fact]
    public void IndependentGridAxisExtentsUseOneVerifiedNativeEnvelope()
    {
        var code = BakeSceneScript.Code;

        Assert.Contains("gridEnvelope.Evaluate(axisContracts,levelContracts,number(origin[2]))", code);
        Assert.Contains("gridEnvelope.CreatePlan(pair.Key,axisContracts,levelContracts,number(origin[2]))", code);
        Assert.Contains("plan.CreateExpansionWarning()", code);
        Assert.Contains("CoordinateX=plan.CoordinateX", code);
        Assert.Contains("ExtensionLeftX=plan.ExtensionLeftX", code);
        Assert.Contains("Grid origin/coordinate/label/envelope/magnetism read-back", code);
        Assert.Contains("realizedReferences.TryGetValue(id,out realizedBy)", code);
        Assert.DoesNotContain("nativeById[x.Item3]=g", code);
        Assert.DoesNotContain("tekla-grid-axis-extents-unsupported", code);

        var commit = code.IndexOf("if(!m.CommitChanges(\"AWARE bake-scene", StringComparison.Ordinal);
        var publishWarnings = code.IndexOf("warnings.AddRange(gridWarningJournal.PublishAfterCommit())", StringComparison.Ordinal);
        Assert.True(commit >= 0);
        Assert.True(publishWarnings > commit);
    }

    [Fact]
    public void TeklaOnlyGridLimitsDoNotAbortUnrelatedSupportedRecords()
    {
        var code = BakeSceneScript.Code;

        Assert.Contains("unsupportedGridEnvelopes", code);
        Assert.Contains("gridEnvelope.CreateUnsupportedRows", code);
        Assert.Contains("supportedOrder.RemoveAll", code);
        Assert.Contains("referenceById.Remove(gridId)", code);
    }

    [Fact]
    public void NativeGridRequiresAnAuthoredElevationDatum()
    {
        Assert.Contains("levels==null||levels.Count==0", BakeSceneScript.Code);
        Assert.Contains("non-empty levels", BakeSceneScript.Code);
    }

    [Fact]
    public void CanonicalGridBoundsAndAxisExtentsStayPreflightValidated()
    {
        var code = BakeSceneScript.Code;

        Assert.Contains("off<minX||off>maxX", code);
        Assert.Contains("off<minY||off>maxY", code);
        Assert.Contains("start/end extents must be finite and increasing", code);
    }

    [Fact]
    public void WorkPlaneRestoreFailuresStayInsideTheStructuredReceipt()
    {
        Assert.Contains("bool workPlaneRestored=!scenePlaneChanged", BakeSceneScript.Code);
        Assert.Contains("commit-state-uncertain", BakeSceneScript.Code);
        Assert.DoesNotContain(
            "finally {\r\n    if(scenePlaneChanged&&!scenePlaneHandler.SetCurrentTransformationPlane",
            BakeSceneScript.Code);
    }

    [Fact]
    public void LegacyNativeCountsCountDistinctTeklaObjects()
    {
        Assert.Contains("int nativeCount=nativeRecordIdByGuid.Count", BakeSceneScript.Code);
        Assert.Contains("created=nativeCount", BakeSceneScript.Code);
        Assert.Contains("native=nativeCount", BakeSceneScript.Code);
        Assert.Contains("role=str(e,\"group\").ToLowerInvariant()", BakeSceneScript.Code);
        Assert.Contains(
            "legacyMemberRole(e)!=\"column\"&&legacyMemberRole(e)!=\"brace\"",
            BakeSceneScript.Code);
    }

    [Fact]
    public void ExactProfilesAreAuthoritativelyReadBackBeforeRetirement()
    {
        Assert.Contains("authoritative Beam GUID read-back failed", BakeSceneScript.Code);
        Assert.Contains("authoritative ContourPlate GUID read-back failed", BakeSceneScript.Code);
        Assert.Contains("differs from requested", BakeSceneScript.Code);
        Assert.Contains("parts[id]=selected;profileById[id]=resolved;return selected", BakeSceneScript.Code);
    }

    [Fact]
    public void PriorNativeRelationshipsRetireBeforeTheirParticipantParts()
    {
        Assert.Contains(
            "old is BaseWeld||old is BoltGroup||old is BooleanPart?0:old is Part?2:1",
            BakeSceneScript.Code);
        Assert.Contains("prior source-owned object remains after retirement", BakeSceneScript.Code);
    }

    [Theory]
    [InlineData(false, false)]
    [InlineData(true, true)]
    public void FailureCleanupNeverCommitsAfterPriorSetRetirementStarts(
        bool retirementStarted,
        bool reconciliationRequired)
    {
        Assert.Equal(
            reconciliationRequired,
            Program.FailureDisposition(retirementStarted)
                == Program.BakeFailureDisposition.LeaveStateForSourceReconciliation);
    }
}
