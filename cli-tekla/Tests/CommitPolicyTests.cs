using Xunit;
using System.Text.Json.Nodes;
using System;
using System.Linq;

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

        Assert.Equal("tekla-connection-materializer-v4", Program.BakeMaterializerIdentity);
        Assert.Equal(first, Program.ComputeBakeMaterializationHash(scene, "2025.0"));
        Assert.NotEqual(first, Program.ComputeBakeMaterializationHash(scene, "2026.0"));
        Assert.Equal(64, first.Length);
    }

    [Fact]
    public void MaterializationPayloadV4DiffersFromTheMemberRollPayloadV3()
    {
        var scene = JsonNode.Parse("{\"meta\":{\"sourceId\":\"s\",\"sceneHash\":\"h\"},\"elements\":[]}")!;
        var current = Program.ComputeBakeMaterializationHash(scene, "2026.0");
        var oldPayload = "tekla-connection-materializer-v3\0" + "2026.0\0"
            + scene.ToJsonString(new System.Text.Json.JsonSerializerOptions { WriteIndented = false });
        using var sha = System.Security.Cryptography.SHA256.Create();
        var old = string.Concat(sha.ComputeHash(System.Text.Encoding.UTF8.GetBytes(oldPayload))
            .Select(value => value.ToString("x2", System.Globalization.CultureInfo.InvariantCulture)));

        Assert.NotEqual(old, current);
    }

    [Fact]
    public void ResolvedBakeContextOverwritesForgedCallerCapabilities()
    {
        var scene = JsonNode.Parse("{\"meta\":{\"sourceId\":\"s\",\"sceneHash\":\"h\"},\"elements\":[]}")!;
        var args = new JsonObject
        {
            ["resolvedHostVersion"] = "2026.0",
            ["materializationHash"] = "forged",
        };

        Program.ApplyResolvedBakeContext(args, scene, "2025.0");

        Assert.Equal("2025.0", args["resolvedHostVersion"]!.GetValue<string>());
        Assert.Equal(
            Program.ComputeBakeMaterializationHash(scene, "2025.0"),
            args["materializationHash"]!.GetValue<string>());
    }

    [Fact]
    public void BakeSceneRequiresArrayCollectionsBeforeMutation()
    {
        Assert.Contains("scene.elements must be an array", BakeSceneScript.Code);
        Assert.Contains("scene.operations must be an array", BakeSceneScript.Code);
        Assert.Contains("scene.referenceSystems must be an array", BakeSceneScript.Code);
    }

    [Fact]
    public void BakeSceneExecutesTheMemberRollAndBrepVerificationContract()
    {
        var code = BakeSceneScript.Code;
        Assert.Contains("memberRoll.CreatePlan", code);
        Assert.Contains("member rot must be a finite JSON number", code);
        Assert.Contains("SceneUpIsAbsentOrExactZ", code);
        Assert.Contains("CanonicalModelDirectoryPath", code);
        Assert.Contains("EffectiveNativeDegrees", code);
        Assert.Contains("solidVertices", code);
        Assert.Contains("native B-rep section orientation differs", code);
        Assert.Contains("nativeRotationOffset", code);

        var guard = code.IndexOf("unexpected-model-path", StringComparison.Ordinal);
        var workPlaneChange = code.IndexOf(
            "SetCurrentTransformationPlane(new TransformationPlane())",
            StringComparison.Ordinal);
        Assert.True(guard >= 0, "expected-model guard must exist");
        Assert.True(workPlaneChange >= 0, "global work-plane change must exist");
        Assert.True(guard < workPlaneChange, "model-path mismatch must fail before work-plane mutation");
    }

    [Fact]
    public void CanonicalTeeRidesTheNamedProfilePathLikeEveryOtherRolledSection()
    {
        // THE REFUSAL THIS REPLACES, and why it is gone rather than merely deleted.
        //
        // The sink used to throw on `xshape == "tee"`, and its own message named the condition for
        // lifting it: "supply an exact native profile after verifying its catalog geometry". That
        // verification has now been done against a live Tekla 2026 UK environment, by asking the
        // catalogue itself through `CatalogHandler.GetLibraryProfileItems()` rather than by assuming:
        //
        //   UKT  ->  UKT102*127*11, UKT102*127*13, UKT102*152*13
        //   TEE  ->  TEE102*102*12, TEE102*127*11, TEE102*127*13
        //
        // Those are byte-identical to the names the producer emits, so a tee is not a guess at a
        // catalogue entry — it IS the catalogue entry. Refusing it meant a UK model carrying any UKT
        // or TEE member could not be exported at all.
        //
        // A tee needs no special plan the way an angle and a double angle do (those are rebuilt from
        // legs). It is one named profile with a roll, exactly like an I or a channel, so it takes the
        // same path they do.
        var code = BakeSceneScript.Code;
        Assert.DoesNotContain("if(xshape==\"tee\")throw", code);
        Assert.DoesNotContain("is explicitly unsupported by the Tekla sink", code);

        // What must NOT have been lost with it: a tee is still held to an EXACT profile name, and is
        // still planned before any Insert. Nothing may fabricate a section for it.
        Assert.Contains("member requires a nonzero finite axis and exact profile", code);
        Assert.Contains("memberRollPlans[id]=memberRoll.CreatePlan(fromArr,toArr,roll)", code);
    }

    [Fact]
    public void CanonicalDoubleAngleMaterializesAsTwoPlannedSingleAngles()
    {
        var code = BakeSceneScript.Code;
        // Planned before the first Insert, so a degenerate descriptor fails
        // preflight rather than leaving one lone angle in the model.
        Assert.Contains("AwareTekla.TeklaDoubleAngleContract.CreatePlan(", code);
        Assert.Contains("memberRollPlans[id+\"#\"+leg.Suffix]=memberRoll.CreatePlan(leg.ReversedAxis?toArr:fromArr,leg.ReversedAxis?fromArr:toArr,leg.CanonicalRollDegrees)", code);
        // Both legs run through insertBeam, so both inherit its exact-profile
        // read-back, ownership tagging and B-rep roll verification. Asserted as the
        // separate decisions they are, so a rename fails one legibly rather than all.
        Assert.Contains("insertBeam(id,id+\"#\"+leg.Suffix,", code);
        // The native profile is ALWAYS the derived leg profile. The authored `2L…`
        // designation must never reach Tekla — some environment's catalog may well
        // resolve it, and then the bake silently builds a section nobody chose.
        Assert.Contains("\"member\",anglePlan.LegProfile,", code);
        Assert.Contains("leg.ReversedAxis?legTail:legHead,leg.ReversedAxis?legHead:legTail", code);
        Assert.Contains("move(head,rolledX,leg.OffsetMm)", code);
        Assert.DoesNotContain("xshape==\"double-angle\")throw", code);
    }

    [Fact]
    public void DoubleAngleGeometryIsProvedAgainstTheCanonicalOutlineBeforeCommit()
    {
        var code = BakeSceneScript.Code;
        // The per-leg roll verification proves each leg against its own plan and says
        // nothing about the pair. Without this the two empirical Tekla facts the plan
        // rests on are unfalsifiable: a test's model of Tekla is the model the plan
        // came from, so flipping one moves both sides and the suite stays green.
        Assert.Contains("anglePlan.SectionOutline", code);
        Assert.Contains("dot(d3,rolledX),dot(d3,rolledY)", code);
        Assert.Contains("the inserted double-angle pair does not match the canonical", code);
        // Both directions: a missing vertex means the pair is misplaced, an extra one
        // means the resolved profile is not the sharp-cornered section assumed.
        Assert.Contains("wanted.Any(w=>!sectionSeen.Any(", code);
        Assert.Contains("sectionSeen.Any(q=>!wanted.Any(", code);
    }

    [Fact]
    public void DoubleAngleEnvelopeMustAgreeWithTheAuthoredSection()
    {
        var code = BakeSceneScript.Code;
        // IFC and Rhino both refuse a double angle whose envelope disagrees with
        // `section`; before this the Tekla sink accepted the mismatch and baked the
        // xsection, so a typo'd thickness shipped under an untouched designation.
        Assert.Contains("anglePlan.EnvelopeWidthMm", code);
        Assert.Contains("anglePlan.EnvelopeDepthMm", code);
        Assert.Contains("disagrees with the authored section", code);
    }

    [Fact]
    public void DoubleAngleLiveFixtureCoversBothBranchesWithAndWithoutRoll()
    {
        var scene = JsonNode.Parse(System.IO.File.ReadAllText(FixturePath("double-angle-scene.json")))!;
        var members = scene["scene"]!["elements"]!.AsArray()
            .Where(element => element!["xsection"]?["shape"]?.GetValue<string>() == "double-angle")
            .ToArray();

        Assert.Contains(members, m => m!["orientation"] is null && m["xsection"]!["orientation"]!.GetValue<string>() == "llbb");
        Assert.Contains(members, m => m!["xsection"]!["orientation"]!.GetValue<string>() == "slbb");
        Assert.Contains(members, m => m!["xsection"]!["gap"]!.GetValue<double>() == 0);
        Assert.Contains(members, m => m!["xsection"]!["d"]!.GetValue<double>() == m["xsection"]!["b"]!.GetValue<double>());

        // The vertical zero-frame branch must be driven WITH a roll as well as
        // without: at rot 0 the reversed leg's `-rot + 180` collapses to a bare 180,
        // so a sign error on that term passes every unrolled column.
        var verticals = members.Where(m => IsVertical(m!)).ToArray();
        Assert.Contains(verticals, m => m!["rot"] is null);
        Assert.Contains(verticals, m => m!["rot"] is not null && m["rot"]!.GetValue<double>() != 0);
        // And downward as well as upward, since the seed branch picks its native
        // reference from the axis sign.
        Assert.Contains(verticals, m => m!["from"]![2]!.GetValue<double>() < m["to"]![2]!.GetValue<double>());
        Assert.Contains(verticals, m => m!["from"]![2]!.GetValue<double>() > m["to"]![2]!.GetValue<double>());

        // Non-vertical coverage: a plain horizontal, a rolled one, and a sloped one.
        var horizontals = members.Where(m => !IsVertical(m!)).ToArray();
        Assert.Contains(horizontals, m => m!["rot"] is not null && m["rot"]!.GetValue<double>() != 0);
        Assert.Contains(horizontals, m => m!["from"]![2]!.GetValue<double>() != m["to"]![2]!.GetValue<double>());
    }

    [Fact]
    public void DoubleAngleLiveFixturePlansCleanlyAndMatchesItsAuthoredEnvelopes()
    {
        var scene = JsonNode.Parse(System.IO.File.ReadAllText(FixturePath("double-angle-scene.json")))!;
        foreach (var element in scene["scene"]!["elements"]!.AsArray())
        {
            var section = element!["xsection"];
            if (section?["shape"]?.GetValue<string>() != "double-angle") continue;

            double[] from = [.. element["from"]!.AsArray().Select(v => v!.GetValue<double>())];
            double[] to = [.. element["to"]!.AsArray().Select(v => v!.GetValue<double>())];
            // Exactly the arguments the bake script derives, so the manual live pass
            // and the automated contract cannot drift apart unnoticed.
            var plan = TeklaDoubleAngleContract.CreatePlan(
                section["d"]!.GetValue<double>(),
                section["b"]!.GetValue<double>(),
                section["t"]!.GetValue<double>(),
                section["gap"]!.GetValue<double>(),
                section["orientation"]!.GetValue<string>(),
                element["rot"]?.GetValue<double>() ?? 0,
                TeklaMemberRollContract.UsesVerticalSeedFrame(from, to));

            var id = element["id"]!.GetValue<string>();
            Assert.Equal(element["section"]!["w"]!.GetValue<double>(), plan.EnvelopeWidthMm, 9);
            Assert.Equal(element["section"]!["d"]!.GetValue<double>(), plan.EnvelopeDepthMm, 9);
            Assert.False(string.IsNullOrWhiteSpace(plan.LegProfile), $"{id}: leg profile");
            Assert.NotEqual(element["profile"]!.GetValue<string>(), plan.LegProfile);
        }
    }

    [Fact]
    public void SingleAngleIsBuiltMirroredAndProvedAgainstTheCanonicalSection()
    {
        // The bake script is a raw string, so nothing type-checks this wiring. These
        // are the load-bearing pieces: without the reversed axis the heel lands on
        // the mirror of what viewer-3d, IFC and Rhino draw, and without the read-back
        // comparison a catalog seated the other way commits silently. See #439.
        var code = BakeSceneScript.Code;
        Assert.Contains("TeklaSingleAngleContract.CreatePlan", code);
        Assert.Contains("memberRollPlans[id+\"#L\"]", code);
        Assert.Contains("singlePlan.ReversedAxis?tail:head", code);
        Assert.Contains("HeelIsOnTheCanonicalSide", code);
        Assert.Contains("singlePlan.ParametricProfile", code);
        // Handedness alone would let a descriptor resolve to a differently sized
        // catalog angle: right sides, wrong leg lengths. The envelope is what holds a
        // catalog profile to the descriptor, before and after insertion.
        Assert.Contains("TeklaSingleAngleContract.MeasureEnvelope", code);
        Assert.Contains("TeklaSingleAngleContract.EnvelopeToleranceMm", code);
        // Pre-flight envelope agreement lives in the contract, so an incomplete
        // authored `section` is refused rather than silently skipping the check.
        Assert.Contains("TeklaSingleAngleContract.RequireAuthoredEnvelope", code);
        // The receipt must still report the AUTHORED roll, not the reversed one the
        // part is actually built at, and must say that the axis was reversed.
        Assert.Contains("memberRollById[id]=basePlan.NormalizedDegrees", code);
        Assert.Contains("r[\"reversedAxis\"]=emittedSinglePlan.ReversedAxis", code);
    }

    [Fact]
    public void SingleAngleLiveFixtureCoversBothBranchesWithAndWithoutRoll()
    {
        var members = SingleAngleMembers();

        // The vertical zero-frame branch must be driven WITH a roll as well as
        // without: at rot 0 the `-rot + 180` correction collapses to a bare 180, so a
        // sign error on that term passes every unrolled column.
        var verticals = members.Where(m => IsVertical(m!)).ToArray();
        Assert.Contains(verticals, m => m!["rot"] is null);
        Assert.Contains(verticals, m => m!["rot"] is not null && m["rot"]!.GetValue<double>() != 0);
        // And downward as well as upward, since the seed branch picks its native
        // reference from the axis sign.
        Assert.Contains(verticals, m => m!["from"]![2]!.GetValue<double>() > m["to"]![2]!.GetValue<double>());

        // Non-vertical coverage: a plain horizontal, a rolled one, and a sloped one.
        var horizontals = members.Where(m => !IsVertical(m!)).ToArray();
        Assert.Contains(horizontals, m => m!["rot"] is null && m["from"]![2]!.GetValue<double>() == m["to"]![2]!.GetValue<double>());
        Assert.Contains(horizontals, m => m!["rot"] is not null && m["rot"]!.GetValue<double>() != 0);
        Assert.Contains(horizontals, m => m!["from"]![2]!.GetValue<double>() != m["to"]![2]!.GetValue<double>());

        // An equal-leg angle as well as unequal ones. It is not chiral, so the mirror
        // is a quarter turn rather than unrecoverable — still the wrong seating.
        Assert.Contains(members, m => m!["xsection"]!["d"]!.GetValue<double>() == m["xsection"]!["b"]!.GetValue<double>());
        Assert.Contains(members, m => m!["xsection"]!["d"]!.GetValue<double>() != m["xsection"]!["b"]!.GetValue<double>());
    }

    [Fact]
    public void SingleAngleLiveFixturePlansCleanlyAndMatchesItsAuthoredEnvelope()
    {
        foreach (var element in SingleAngleMembers())
        {
            var section = element!["xsection"]!;
            double[] from = [.. element["from"]!.AsArray().Select(v => v!.GetValue<double>())];
            double[] to = [.. element["to"]!.AsArray().Select(v => v!.GetValue<double>())];
            double d = section["d"]!.GetValue<double>(), b = section["b"]!.GetValue<double>();
            // Exactly the arguments the bake script derives, so the manual live pass
            // and the automated contract cannot drift apart unnoticed.
            var plan = TeklaSingleAngleContract.CreatePlan(
                d, b, section["t"]!.GetValue<double>(),
                element["rot"]?.GetValue<double>() ?? 0,
                TeklaMemberRollContract.UsesVerticalSeedFrame(from, to));

            var id = element["id"]!.GetValue<string>();
            Assert.True(plan.ReversedAxis, $"{id}: must be built on the reversed axis");
            // The bake refuses an angle whose authored `section` is incomplete or
            // disagrees with the descriptor, so every fixture member must clear that
            // gate through the same call the bake makes.
            TeklaSingleAngleContract.RequireAuthoredEnvelope(
                plan,
                element["section"]!["w"]!.GetValue<double>(),
                element["section"]!["d"]!.GetValue<double>());
            Assert.Equal(b, plan.EnvelopeWidthMm, 9);
            Assert.Equal(d, plan.EnvelopeDepthMm, 9);
            // Every fixture member is authored with the sharp-corner parametric `L`,
            // so the live bake runs the FULL outline comparison on each rather than
            // falling back to the fillet-tolerant heel test alone.
            Assert.Equal(element["profile"]!.GetValue<string>(), plan.ParametricProfile);
            // The figure the bake compares against must itself read as canonical.
            Assert.True(TeklaSingleAngleContract.HeelIsOnTheCanonicalSide(plan.SectionOutline, 0.1), $"{id}: canonical outline");
        }
    }

    static JsonNode?[] SingleAngleMembers()
    {
        var scene = JsonNode.Parse(System.IO.File.ReadAllText(FixturePath("double-angle-scene.json")))!;
        var members = scene["scene"]!["elements"]!.AsArray()
            .Where(element => element!["xsection"]?["shape"]?.GetValue<string>() == "angle")
            .ToArray();
        // A fixture that quietly stopped carrying single angles would pass every
        // assertion below by vacuity.
        Assert.NotEmpty(members);
        return members;
    }

    static string FixturePath(string name) => System.IO.Path.GetFullPath(System.IO.Path.Combine(
        AppContext.BaseDirectory, "..", "..", "..", "Fixtures", name));

    static bool IsVertical(JsonNode member)
    {
        double[] from = [.. member["from"]!.AsArray().Select(v => v!.GetValue<double>())];
        double[] to = [.. member["to"]!.AsArray().Select(v => v!.GetValue<double>())];
        return TeklaMemberRollContract.UsesVerticalSeedFrame(from, to);
    }

    [Fact]
    public void DoubleAnglePairIsReportedAsTwoNativesRatherThanOne()
    {
        var code = BakeSceneScript.Code;
        Assert.Contains("r[\"nativeGuids\"]=legGuids", code);
        Assert.Contains("tekla-double-angle-materialized-as-pair", code);
    }

    [Fact]
    public void NativeConnectionsRefuseADoubleAngleParticipant()
    {
        var code = BakeSceneScript.Code;
        Assert.Contains("so a native connection to it is ambiguous", code);
        // Assert the guard's own field array, not the bare field names: every one of
        // those literals appears elsewhere in the script for unrelated reasons, so
        // per-name assertions still passed with the array trimmed to the bolt pair —
        // and a weld to a double angle would then silently bind to leg `a` alone.
        Assert.Contains(
            "foreach(var field in new[]{\"partToBoltTo\",\"partToBeBolted\",\"mainId\",\"secondaryId\",\"targetId\"})rejectDoubleAngleParticipant(field,str(op,field));",
            code);
        Assert.Contains("rejectDoubleAngleParticipant(\"holeEffects.targetId\"", code);
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

        Assert.Contains("gridEnvelope.Evaluate(axisContracts,levelContracts,number(origin[2]),resolvedHostVersion)", code);
        Assert.Contains("gridEnvelope.CreatePlan(pair.Key,axisContracts,levelContracts,number(origin[2]),resolvedHostVersion)", code);
        Assert.Contains("plan.CreateExpansionWarning()", code);
        Assert.Contains("plan.CreateLabelTokenWarning()", code);
        Assert.Contains("CoordinateX=plan.CoordinateX", code);
        Assert.Contains("CoordinateZ=plan.CoordinateZ", code);
        Assert.Contains("LabelZ=plan.LabelZ", code);
        Assert.Contains("ExtensionLeftX=plan.ExtensionLeftX", code);
        Assert.Contains("Grid origin/coordinate/label/envelope/magnetism read-back", code);
        Assert.Contains("Math.Abs(dot(normal,expectedAxis))>=1-1e-9", code);
        Assert.Contains("native Grid automatic plane label/coordinate/orientation association", code);
        Assert.Contains("realizedReferences.TryGetValue(id,out realizedBy)", code);
        Assert.DoesNotContain("nativeById[x.Item3]=g", code);
        Assert.DoesNotContain("tekla-grid-axis-extents-unsupported", code);

        var gridInsert = code.IndexOf("// Structural grids are inserted", StringComparison.Ordinal);
        var gridVerificationEnd = code.IndexOf("// GUID/tag read-back proves", StringComparison.Ordinal);
        Assert.True(gridInsert >= 0, "parent Grid insertion block must exist");
        Assert.True(gridVerificationEnd > gridInsert, "Grid verification block must have a bounded end");
        var gridBlock = code.Substring(gridInsert, gridVerificationEnd - gridInsert);
        Assert.DoesNotContain("nativePlane.Modify", gridBlock);
        Assert.DoesNotContain("new GridPlane", gridBlock);

        var commit = code.IndexOf("if(!m.CommitChanges(\"AWARE bake-scene", StringComparison.Ordinal);
        var classifyRecords = code.IndexOf("foreach(var item in supportedOrder){string id=item.Item1;string kind=item.Item2;ModelObject o=null", StringComparison.Ordinal);
        var publishWarnings = code.IndexOf("warnings.AddRange(gridWarningJournal.PublishAfterCommit())", StringComparison.Ordinal);
        Assert.True(commit >= 0);
        Assert.True(classifyRecords > commit);
        Assert.True(publishWarnings > classifyRecords);
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
