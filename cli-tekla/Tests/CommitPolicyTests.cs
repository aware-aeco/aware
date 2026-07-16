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
    public void IndependentGridAxisExtentsAreExplicitlyUnsupported()
    {
        Assert.Contains("gridsWithIndependentAxisExtents", BakeSceneScript.Code);
        Assert.Contains("tekla-grid-axis-extents-unsupported", BakeSceneScript.Code);
        Assert.Contains("referenceById.Remove(gridId)", BakeSceneScript.Code);
    }

    [Fact]
    public void NativeGridRequiresAnAuthoredElevationDatum()
    {
        Assert.Contains("levels==null||levels.Count==0", BakeSceneScript.Code);
        Assert.Contains("non-empty levels", BakeSceneScript.Code);
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
