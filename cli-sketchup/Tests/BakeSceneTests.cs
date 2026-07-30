using System.IO;
using System.Net;
using System.Net.Sockets;
using System.Text;
using System.Text.Json.Nodes;
using AwareSketchup;
using AwareSketchup.Verbs;
using Xunit;

namespace AwareSketchup.Tests;

public class BakeSceneTests : IDisposable
{
    readonly string _discoveryDir;

    public BakeSceneTests()
    {
        _discoveryDir = Path.Combine(Path.GetTempPath(), $"aware-sketchup-bake-test-{Guid.NewGuid():N}");
        Directory.CreateDirectory(_discoveryDir);
    }

    public void Dispose()
    {
        try { Directory.Delete(_discoveryDir, recursive: true); } catch { }
    }

    // ── unit conversion ──────────────────────────────────────────────────────

    [Fact]
    public void MillimetreInchConversionRoundTrips()
    {
        foreach (var mm in new[] { 0.001, 1.0, 25.4, 3000.0, 152.4, 12345.6789 })
        {
            var inches = BakeScene.MmToInches(mm);
            Assert.Equal(mm, BakeScene.InchesToMm(inches), 9);
        }

        // The anchor the whole scene rides on: 25.4 mm is exactly one SketchUp
        // internal unit, and a 3 m member is 118.11" — the same numbers Ruby's
        // Numeric#mm produces inside the host.
        Assert.Equal(1.0, BakeScene.MmToInches(25.4), 12);
        Assert.Equal(118.110236220472, BakeScene.MmToInches(3000.0), 9);
        Assert.Equal(25.4, BakeScene.InchesToMm(1.0), 12);
    }

    [Fact]
    public void SketchupToleranceIsExpressedInInchesNotMillimetres()
    {
        // 0.001" is ~0.0254 mm. A 0.01 mm member is legal in the scene and
        // degenerate in SketchUp, which is exactly why the check lives in inch
        // space — comparing 0.01 against 0.001 in millimetres would pass it.
        Assert.True(BakeScene.MmToInches(0.01) <= BakeScene.SketchupToleranceInches);
        Assert.False(BakeScene.MmToInches(0.1) <= BakeScene.SketchupToleranceInches);
        Assert.Equal(0.0254, BakeScene.InchesToMm(BakeScene.SketchupToleranceInches), 9);
    }

    // ── id rules ─────────────────────────────────────────────────────────────

    [Fact]
    public void IdsMustBeTrimmedBoundedAndControlCharacterFree()
    {
        Assert.True(BakeScene.IsValidId("m1"));
        Assert.True(BakeScene.IsValidId("beam:B-12/end0"));
        Assert.True(BakeScene.IsValidId(new string('x', 256)));

        Assert.False(BakeScene.IsValidId(null));
        Assert.False(BakeScene.IsValidId(""));
        Assert.False(BakeScene.IsValidId("   "));
        Assert.False(BakeScene.IsValidId(" m1"));
        Assert.False(BakeScene.IsValidId("m1 "));
        Assert.False(BakeScene.IsValidId("m\t1"));
        Assert.False(BakeScene.IsValidId("m\u00001"));
        Assert.False(BakeScene.IsValidId("m\u007f1"));
        Assert.False(BakeScene.IsValidId(new string('x', 257)));
        // 256 is a BYTE budget, not a character count.
        Assert.False(BakeScene.IsValidId(new string('é', 129)));
        Assert.True(BakeScene.IsValidId(new string('é', 128)));
    }

    [Fact]
    public void DuplicateIdsAcrossCollectionsAreRefused()
    {
        var pf = BakeScene.Validate(JsonNode.Parse("""
            {"meta":{"units":"mm","sourceId":"s","sceneHash":"h"},
             "elements":[{"id":"dup","kind":"member"}],
             "operations":[{"id":"dup","kind":"weld"}]}
            """));

        Assert.False(pf.Ok);
        Assert.Contains(pf.Failed, r => Code(r) == "duplicate-id" && Id(r) == "dup");
    }

    [Fact]
    public void InvalidIdIsReportedAgainstTheSceneWhenThereIsNoIdToNameItBy()
    {
        var pf = BakeScene.Validate(JsonNode.Parse("""
            {"meta":{"units":"mm","sourceId":"s","sceneHash":"h"},
             "elements":[{"kind":"member"},{"id":" padded ","kind":"member"}]}
            """));

        Assert.False(pf.Ok);
        Assert.Contains(pf.Failed, r => Code(r) == "invalid-id" && Id(r) == "scene");
        Assert.Contains(pf.Failed, r => Code(r) == "invalid-id" && Id(r) == " padded ");
    }

    // ── envelope ─────────────────────────────────────────────────────────────

    [Fact]
    public void MillimetresAreTheOnlyAcceptedUnits()
    {
        Assert.True(BakeScene.Validate(Scene("\"units\":\"mm\",")).Ok);
        Assert.True(BakeScene.Validate(Scene("\"units\":\"MM\",")).Ok);
        Assert.True(BakeScene.Validate(Scene("")).Ok);  // absent means legacy millimetres

        var feet = BakeScene.Validate(Scene("\"units\":\"ft\","));
        Assert.False(feet.Ok);
        Assert.Contains(feet.Failed, r => Code(r) == "unsupported-units");
    }

    [Fact]
    public void SourceIdAndSceneHashAreMandatory()
    {
        var noSource = BakeScene.Validate(JsonNode.Parse("""{"meta":{"units":"mm","sceneHash":"h"},"elements":[]}"""));
        Assert.Contains(noSource.Failed, r => Code(r) == "missing-source-id");

        var noHash = BakeScene.Validate(JsonNode.Parse("""{"meta":{"units":"mm","sourceId":"s"},"elements":[]}"""));
        Assert.Contains(noHash.Failed, r => Code(r) == "missing-scene-hash");
    }

    [Fact]
    public void NonObjectSceneAndNonArrayCollectionsAreRefused()
    {
        var notObject = BakeScene.Validate(JsonNode.Parse("[]"));
        Assert.Contains(notObject.Failed, r => Code(r) == "invalid-scene");

        var badCollection = BakeScene.Validate(JsonNode.Parse("""
            {"meta":{"units":"mm","sourceId":"s","sceneHash":"h"},"elements":{},"operations":3}
            """));
        Assert.Contains(badCollection.Failed,
            r => Code(r) == "invalid-collection" && Message(r).Contains("scene.elements"));
        Assert.Contains(badCollection.Failed,
            r => Code(r) == "invalid-collection" && Message(r).Contains("scene.operations"));

        var badRecord = BakeScene.Validate(JsonNode.Parse("""
            {"meta":{"units":"mm","sourceId":"s","sceneHash":"h"},"elements":["nope"]}
            """));
        Assert.Contains(badRecord.Failed, r => Code(r) == "invalid-record");
    }

    [Fact]
    public void ARefusedBatchAccountsForEverySupportedRecord()
    {
        // One bad record must not leave the caller guessing whether the good ones
        // quietly landed: every supported id gets its own row.
        var pf = BakeScene.Validate(JsonNode.Parse("""
            {"meta":{"units":"ft","sourceId":"s","sceneHash":"h"},
             "elements":[{"id":"m1","kind":"member"},{"id":"m2","kind":"member"}]}
            """));

        Assert.False(pf.Ok);
        Assert.Contains(pf.Failed, r => Id(r) == "m1" && Code(r) == "batch-aborted");
        Assert.Contains(pf.Failed, r => Id(r) == "m2" && Code(r) == "batch-aborted");
    }

    // ── kind classification ──────────────────────────────────────────────────

    [Fact]
    public void MembersAreMaterializedAndEveryOtherKindIsReportedUnsupported()
    {
        var pf = BakeScene.Validate(JsonNode.Parse("""
            {"meta":{"units":"mm","sourceId":"s","sceneHash":"h"},
             "elements":[{"id":"m1","kind":"member"},{"id":"m2","kind":"box"},{"id":"m3","kind":"line"},
                         {"id":"noKind"},
                         {"id":"p1","kind":"plate"},{"id":"b1","kind":"bolt-shank"},{"id":"z1","kind":"gizmo"}],
             "operations":[{"id":"w1","kind":"weld"}],
             "referenceSystems":[{"id":"g1","kind":"structural-grid"}]}
            """));

        Assert.True(pf.Ok);
        Assert.Equal(new[] { "m1", "m2", "m3", "noKind" }, pf.Supported.Select(Id).ToArray());
        // An element with no kind defaults to `member`, as in the Tekla sink.
        Assert.Contains(pf.Supported, s => Id(s) == "noKind" && Kind(s) == "member");

        Assert.Contains(pf.Unsupported, r => Id(r) == "p1" && Code(r) == "unsupported-kind");
        Assert.Contains(pf.Unsupported, r => Id(r) == "b1" && Code(r) == "unsupported-kind");
        Assert.Contains(pf.Unsupported, r => Id(r) == "z1" && Code(r) == "unsupported-kind");
        Assert.Contains(pf.Unsupported, r => Id(r) == "w1" && Code(r) == "unsupported-operation");
        Assert.Contains(pf.Unsupported, r => Id(r) == "g1" && Code(r) == "unsupported-reference-system");
        Assert.All(pf.Unsupported, r => Assert.Equal("unsupported", Status(r)));
    }

    /// <summary>
    /// EVERY record the canonical scene carries gets exactly one row — including the ones nested
    /// inside another record. A plate's holes and a bolt instance's hole effects used to be absent
    /// from all three arrays, which made a connection model unexportable: the consumer reconciles
    /// the receipt against the scene and refuses a bake that cannot say what became of a record
    /// (aware-aeco/aware#326). This asserts the ids as a SET, so a future collection that is added
    /// to the scene and forgotten here fails rather than passing unnoticed.
    /// </summary>
    [Fact]
    public void EveryNestedSceneRecordIsClassified()
    {
        var pf = BakeScene.Validate(JsonNode.Parse("""
            {"meta":{"units":"mm","sourceId":"s","sceneHash":"h"},
             "elements":[{"id":"m1","kind":"member"},
                         {"id":"p1","kind":"plate","holes":[{"id":"p1-h0"},{"id":"p1-h1"}]}],
             "operations":[{"id":"ba1","kind":"bolt-array",
                            "instances":[{"id":"ba1-i0","holeEffects":[{"id":"ba1-i0-hp"},{"id":"ba1-i0-hm"}]},
                                         {"id":"ba1-i1","holeEffects":[{"id":"ba1-i1-hp"}]}]}],
             "referenceSystems":[{"id":"g1","kind":"structural-grid",
                                  "axes":[{"id":"g1-a"}],"levels":[{"id":"g1-l"}]}]}
            """));

        Assert.True(pf.Ok);
        var classified = pf.Supported.Select(Id)
            .Concat(pf.Unsupported.Select(Id))
            .Concat(pf.Failed.Select(Id))
            .ToArray();
        Assert.Equal(classified.Length, classified.Distinct().Count());
        Assert.Equal(
            new[] { "ba1", "ba1-i0", "ba1-i0-hm", "ba1-i0-hp", "ba1-i1", "ba1-i1-hp",
                    "g1", "g1-a", "g1-l", "m1", "p1", "p1-h0", "p1-h1" },
            classified.OrderBy(x => x, StringComparer.Ordinal).ToArray());

        // The kind is the vocabulary the consumer reconciles against, and it is Tekla's: a hole —
        // whether it belongs to a plate or to a bolt instance — is an `opening`.
        foreach (var openingId in new[] { "p1-h0", "p1-h1", "ba1-i0-hp", "ba1-i0-hm", "ba1-i1-hp" })
            Assert.Contains(pf.Unsupported, r => Id(r) == openingId && Kind(r) == "opening"
                                                && Code(r) == "unsupported-parent");
        Assert.Contains(pf.Unsupported, r => Id(r) == "ba1-i0" && Kind(r) == "bolt-instance");
        Assert.Contains(pf.Unsupported, r => Id(r) == "g1-a" && Kind(r) == "grid-axis");
        Assert.Contains(pf.Unsupported, r => Id(r) == "g1-l" && Kind(r) == "grid-level");
    }

    /// <summary>
    /// The other half of the same contract: a row for an id the scene never declared is as
    /// unreconcilable as a missing one, so a nested collection is only walked on the kind that
    /// canonically owns it. `holes` belong to a plate and `instances` to a bolt array — nowhere else.
    /// </summary>
    [Fact]
    public void NestedCollectionsAreOnlyReadOffTheKindThatOwnsThem()
    {
        var pf = BakeScene.Validate(JsonNode.Parse("""
            {"meta":{"units":"mm","sourceId":"s","sceneHash":"h"},
             "elements":[{"id":"m1","kind":"member","holes":[{"id":"not-a-record"}]}],
             "operations":[{"id":"w1","kind":"weld","instances":[{"id":"also-not-a-record"}]}]}
            """));

        Assert.True(pf.Ok);
        var classified = pf.Supported.Select(Id).Concat(pf.Unsupported.Select(Id)).Concat(pf.Failed.Select(Id));
        Assert.Equal(new[] { "m1", "w1" }, classified.OrderBy(x => x, StringComparer.Ordinal).ToArray());
    }

    /// <summary>A nested record with a duplicate or malformed id is caught by the same gate as a
    /// top-level one — ids are unique across the WHOLE scene, nesting included.</summary>
    [Fact]
    public void NestedRecordsShareTheSceneWideIdGate()
    {
        var pf = BakeScene.Validate(JsonNode.Parse("""
            {"meta":{"units":"mm","sourceId":"s","sceneHash":"h"},
             "elements":[{"id":"p1","kind":"plate","holes":[{"id":"dup"},{"id":"dup"},{"id":" untrimmed "}]}]}
            """));

        Assert.False(pf.Ok);
        Assert.Contains(pf.Failed, r => Id(r) == "dup" && Code(r) == "duplicate-id");
        Assert.Contains(pf.Failed, r => Code(r) == "invalid-id");
    }

    /// <summary>
    /// A rejected bolt-instance id does not take its hole effects down with it. The effects are
    /// records in their own right, so they still get their rows AND still enter the scene-wide id
    /// set — otherwise a duplicate among the effects would slip through behind a bad instance id.
    /// This is what cli-tekla and cli-rhino already do.
    /// </summary>
    [Fact]
    public void HoleEffectsAreStillClassifiedWhenTheirInstanceIdIsRejected()
    {
        var pf = BakeScene.Validate(JsonNode.Parse("""
            {"meta":{"units":"mm","sourceId":"s","sceneHash":"h"},
             "elements":[{"id":"ba1-i0","kind":"member"}],
             "operations":[{"id":"ba1","kind":"bolt-array",
                            "instances":[{"id":"ba1-i0","holeEffects":[{"id":"eff-a"},{"id":"eff-b"}]},
                                         {"id":"ba1-i1","holeEffects":[{"id":"eff-b"}]}]}]}
            """));

        // The instance id collides with the member's, so the instance itself is a failure...
        Assert.False(pf.Ok);
        Assert.Contains(pf.Failed, r => Id(r) == "ba1-i0" && Code(r) == "duplicate-id");
        // ...but its effects are classified anyway, and the duplicate BETWEEN two instances' effects
        // is still caught — both of which the old `continue` would have lost.
        Assert.Contains(pf.Unsupported, r => Id(r) == "eff-a" && Kind(r) == "opening");
        Assert.Contains(pf.Unsupported, r => Id(r) == "eff-b" && Kind(r) == "opening");
        Assert.Contains(pf.Failed, r => Id(r) == "eff-b" && Code(r) == "duplicate-id");
    }

    [Fact]
    public void ANestedCollectionThatIsNotAnArrayFailsTheBatch()
    {
        var pf = BakeScene.Validate(JsonNode.Parse("""
            {"meta":{"units":"mm","sourceId":"s","sceneHash":"h"},
             "elements":[{"id":"p1","kind":"plate","holes":{"id":"nope"}}]}
            """));

        Assert.False(pf.Ok);
        Assert.Contains(pf.Failed, r => Id(r) == "p1" && Code(r) == "invalid-collection");
    }

    // ── receipt shape ────────────────────────────────────────────────────────

    [Fact]
    public void RowCarriesIdKindStatusAndOmitsEmptyCodeAndMessage()
    {
        var bare = BakeScene.Row("m1", "member", "emitted");
        Assert.Equal("m1", bare["id"]!.GetValue<string>());
        Assert.Equal("member", bare["kind"]!.GetValue<string>());
        Assert.Equal("emitted", bare["status"]!.GetValue<string>());
        Assert.Null(bare["code"]);
        Assert.Null(bare["message"]);

        var full = BakeScene.Row("m1", "member", "failed", "invalid-geometry", "boom");
        Assert.Equal("invalid-geometry", full["code"]!.GetValue<string>());
        Assert.Equal("boom", full["message"]!.GetValue<string>());
    }

    [Fact]
    public void EnvelopeAlwaysCarriesAllFourReceiptArrays()
    {
        var env = BakeScene.Envelope(false, new JsonArray(), new JsonArray(), new JsonArray(), new JsonArray());
        Assert.False(env["ok"]!.GetValue<bool>());
        Assert.NotNull(env["emitted"] as JsonArray);
        Assert.NotNull(env["failed"] as JsonArray);
        Assert.NotNull(env["unsupported"] as JsonArray);
        Assert.NotNull(env["warnings"] as JsonArray);
    }

    [Fact]
    public void BakeResultFailKeepsTheStructuredReceiptOnTheBakeSceneVerb()
    {
        var body = BakeScene.Envelope(false, new JsonArray(), new JsonArray { BakeScene.Row("m1", "member", "failed", "invalid-geometry", "boom") },
                                      new JsonArray(), new JsonArray());
        var receipt = Receipts.BakeResultFail(body, "26.0", 4242, "4242", "");

        Assert.False(receipt["ok"]!.GetValue<bool>());
        Assert.Equal("bake-scene", receipt["verb"]!.GetValue<string>());
        Assert.Equal("sketchup", receipt["host"]!.GetValue<string>());
        Assert.Equal("invalid-geometry", receipt["result"]!["failed"]![0]!["code"]!.GetValue<string>());
    }

    [Fact]
    public void ExecReceiptsKeepTheirVerbWhenNoneIsPassed()
    {
        Assert.Equal("exec", Receipts.ExecOk(null, "26.0", 1, "1", "")["verb"]!.GetValue<string>());
        Assert.Equal("exec", Receipts.ExecFail("boom", "", "")["verb"]!.GetValue<string>());
        Assert.Equal("bake-scene", Receipts.ExecOk(null, "26.0", 1, "1", "", "bake-scene")["verb"]!.GetValue<string>());
    }

    [Fact]
    public void SidecarUnsupportedRowsAreMergedIntoTheBridgeReceipt()
    {
        // Unsupported kinds never reach the Ruby, so without this merge they would
        // vanish from the receipt — silently dropped, which the contract forbids.
        var bridge = JsonNode.Parse("""
            {"ok":true,"emitted":[{"id":"m1","kind":"member","status":"emitted"}],
             "failed":[],"unsupported":[],"warnings":[]}
            """);
        var sidecar = new JsonArray { BakeScene.Row("p1", "plate", "unsupported", "unsupported-kind", "nope") };

        var merged = BakeScene.MergeUnsupported(bridge, sidecar) as JsonObject;
        Assert.NotNull(merged);
        var rows = merged!["unsupported"] as JsonArray;
        Assert.Single(rows!);
        Assert.Equal("p1", Id(rows![0]));
    }

    [Fact]
    public void MergeSurvivesANonObjectBridgeResultWithoutLosingUnsupportedRows()
    {
        var sidecar = new JsonArray { BakeScene.Row("p1", "plate", "unsupported", "unsupported-kind", "nope") };
        var merged = BakeScene.MergeUnsupported(JsonNode.Parse("\"surprise\""), sidecar) as JsonObject;

        Assert.NotNull(merged);
        Assert.False(merged!["ok"]!.GetValue<bool>());
        Assert.Equal("p1", Id((merged["unsupported"] as JsonArray)![0]));
        Assert.Contains(merged["failed"] as JsonArray ?? new JsonArray(), r => Code(r) == "invalid-record");
    }

    // ── retire-and-replace ───────────────────────────────────────────────────

    [Fact]
    public void OnlyGroupsOwnedByTheSameSourceAreRetired()
    {
        var marker = BakeSceneOwnership.BuildMarker(new string('a', 64));

        // Owned by this bake -> retired. Every record the incoming scene still
        // carries is re-created fresh in the same operation, so retiring the whole
        // prior set is what makes a record the scene DROPPED disappear.
        Assert.Equal(BakeSceneOwnership.Retirement.Retire,
            BakeSceneOwnership.Decide(new BakeSceneOwnership.OwnedCandidate("src-a", "m1", "hash", marker), "src-a"));

        // Owned by a DIFFERENT source -> never touched. This is the property that
        // stops one drag-out's bake from erasing another's (or another app's) work.
        Assert.Equal(BakeSceneOwnership.Retirement.KeepForeign,
            BakeSceneOwnership.Decide(new BakeSceneOwnership.OwnedCandidate("src-b", "m1", "hash", marker), "src-a"));

        // Never stamped -> the user's own geometry.
        Assert.Equal(BakeSceneOwnership.Retirement.KeepUnowned,
            BakeSceneOwnership.Decide(new BakeSceneOwnership.OwnedCandidate(null, null, null, null), "src-a"));
    }

    [Fact]
    public void APartiallyOrBadlyStampedEntityIsNeverRetired()
    {
        var marker = BakeSceneOwnership.BuildMarker(new string('a', 64));

        Assert.Equal(BakeSceneOwnership.Retirement.KeepUnowned,
            BakeSceneOwnership.Decide(new BakeSceneOwnership.OwnedCandidate("src-a", "", "hash", marker), "src-a"));
        Assert.Equal(BakeSceneOwnership.Retirement.KeepUnowned,
            BakeSceneOwnership.Decide(new BakeSceneOwnership.OwnedCandidate("src-a", "m1", "  ", marker), "src-a"));
        Assert.Equal(BakeSceneOwnership.Retirement.KeepUnowned,
            BakeSceneOwnership.Decide(new BakeSceneOwnership.OwnedCandidate("src-a", "m1", "hash", "AWARE_BAKE_V1:not-hex"), "src-a"));
        Assert.Equal(BakeSceneOwnership.Retirement.KeepUnowned,
            BakeSceneOwnership.Decide(new BakeSceneOwnership.OwnedCandidate("src-a", "m1", "hash", "something-else"), "src-a"));
    }

    [Fact]
    public void AnEmptyIncomingSourceIdRetiresNothing()
    {
        // The one mistake that would turn a bake into "erase the model": an empty
        // source id matching every entity that was never stamped.
        var marker = BakeSceneOwnership.BuildMarker(new string('a', 64));
        foreach (var empty in new string?[] { null, "", "   " })
        {
            Assert.Equal(BakeSceneOwnership.Retirement.KeepUnowned,
                BakeSceneOwnership.Decide(new BakeSceneOwnership.OwnedCandidate("", "m1", "h", marker), empty));
            Assert.Equal(BakeSceneOwnership.Retirement.KeepUnowned,
                BakeSceneOwnership.Decide(new BakeSceneOwnership.OwnedCandidate(null, null, null, null), empty));
        }
    }

    [Fact]
    public void OwnershipMarkerIsNamespacedAndHashShaped()
    {
        var marker = BakeSceneOwnership.BuildMarker(new string('9', 64));
        Assert.StartsWith("AWARE_BAKE_V1:", marker);
        Assert.True(BakeSceneOwnership.IsOwnershipMarker(marker));

        Assert.False(BakeSceneOwnership.IsOwnershipMarker(null));
        Assert.False(BakeSceneOwnership.IsOwnershipMarker(""));
        Assert.False(BakeSceneOwnership.IsOwnershipMarker("AWARE_BAKE_V1:" + new string('a', 63)));
        Assert.False(BakeSceneOwnership.IsOwnershipMarker("AWARE_BAKE_V1:" + new string('A', 64)));  // uppercase hex
        Assert.False(BakeSceneOwnership.IsOwnershipMarker("prefixed AWARE_BAKE_V1:" + new string('a', 64)));
        Assert.False(BakeSceneOwnership.IsOwnershipMarker("AWARE_BAKE_V1:" + new string('a', 64) + "\ntrailing"));
    }

    [Fact]
    public void MaterializationHashIsStableAndHostVersionSensitive()
    {
        var scene = JsonNode.Parse("""{"meta":{"sourceId":"s","sceneHash":"h"},"elements":[]}""")!;
        var first = BakeScene.ComputeMaterializationHash(scene, "26.0");

        Assert.Equal(first, BakeScene.ComputeMaterializationHash(scene, "26.0"));
        Assert.NotEqual(first, BakeScene.ComputeMaterializationHash(scene, "25.0"));
        Assert.Equal(64, first.Length);
        Assert.True(BakeSceneOwnership.IsOwnershipMarker(BakeSceneOwnership.BuildMarker(first)));
    }

    // ── the injected Ruby ────────────────────────────────────────────────────

    [Fact]
    public void TheMaterializerIsEmbeddedInTheSidecar()
    {
        var code = BakeSceneScript.Code;
        Assert.True(code.Length > 1000, "the embedded bake-scene materializer looks empty");
        Assert.Contains("AWARE bake-scene materializer for SketchUp", code);
    }

    [Fact]
    public void TheRubyReadsTheOwnershipVocabularyFromArgsAndHardcodesNoneOfIt()
    {
        // The decision rule that Decide() proves only holds in the host if the Ruby
        // matches on the SAME dictionary and keys. It does because it reads every one
        // of them out of args.ownership — so this asserts the two cannot drift.
        var code = BakeSceneScript.Code;
        foreach (var key in new[] { "dictionary", "sourceIdKey", "recordIdKey", "sceneHashKey",
                                    "markerKey", "marker", "markerPattern", "tag", "sourceId" })
            Assert.Contains($"own['{key}']", code);

        Assert.DoesNotContain(BakeSceneOwnership.MarkerPrefix, code);
        Assert.DoesNotContain(BakeSceneOwnership.SourceIdKey, code);
        Assert.DoesNotContain(BakeSceneOwnership.RecordIdKey, code);
        Assert.DoesNotContain(BakeSceneOwnership.SceneHashKey, code);
        Assert.DoesNotContain(BakeSceneOwnership.MarkerKey, code);
    }

    [Fact]
    public void TheRubyRefusesAnEmptyOwnershipSourceIdBeforeScanning()
    {
        Assert.Contains("args.ownership.sourceId is required", BakeSceneScript.Code);
    }

    [Fact]
    public void TheRubyAvoidsTheBlockScopeFootgunsTheBridgeWrapperImposes()
    {
        // The bridge evaluates the materializer inside `->(args) { ... }`, which
        // makes three otherwise-ordinary Ruby constructs wrong here — and a Ruby
        // PARSER cannot see the first one (`dynamic constant assignment` is raised
        // by CRuby's compiler, not the parser), so it only ever surfaces on a live
        // SketchUp. Pin all three:
        //   CONST = ...   -> SyntaxError: dynamic constant assignment
        //   def / class   -> leaks a method onto Object for the whole session
        //   proc/Proc.new -> `return` becomes non-local and escapes the bridge
        foreach (var line in BakeSceneScript.Code.Split('\n'))
        {
            var trimmed = line.TrimStart();
            if (trimmed.StartsWith('#')) continue;
            Assert.False(System.Text.RegularExpressions.Regex.IsMatch(trimmed, @"^[A-Z][A-Za-z0-9_]*\s*(=[^=~]|\|\|=)"),
                $"constant assignment is a SyntaxError inside the bridge's lambda: {line}");
            Assert.False(System.Text.RegularExpressions.Regex.IsMatch(trimmed, @"^(def|class|module)\s"),
                $"def/class/module inside the bridge's lambda leaks into the SketchUp session: {line}");
        }

        Assert.DoesNotContain("Proc.new", BakeSceneScript.Code);
        Assert.DoesNotContain(" proc {", BakeSceneScript.Code);
        Assert.DoesNotContain(" proc do", BakeSceneScript.Code);
    }

    [Fact]
    public void TheWholeBakeIsOneSketchupOperationWithAnAbortOnFailure()
    {
        var code = BakeSceneScript.Code;
        Assert.Equal(1, Occurrences(code, "model.start_operation("));
        Assert.Equal(1, Occurrences(code, "model.commit_operation"));
        Assert.Equal(1, Occurrences(code, "model.abort_operation"));
        // start_operation(name, true): disable_ui, so a whole drop is one undo step.
        Assert.Contains("model.start_operation(\"AWARE bake-scene #{scene_name}\", true)", code);
    }

    [Fact]
    public void TheRubyConvertsUnitsWithSketchupsOwnHelpersAndProvesTheRoundTrip()
    {
        var code = BakeSceneScript.Code;
        Assert.Contains(".mm", code);
        Assert.Contains(".to_mm", code);
        Assert.Contains("host-unit-conversion", code);
    }

    [Fact]
    public void TheRubyReusesTheSharedValidationCodes()
    {
        var code = BakeSceneScript.Code;
        foreach (var c in new[] { "host-unavailable", "invalid-scene", "unsupported-units",
                                  "missing-source-id", "missing-scene-hash", "batch-aborted",
                                  "invalid-geometry", "materialization-failed" })
            Assert.Contains(c, code);
    }

    // ── verb dispatch ────────────────────────────────────────────────────────

    [Fact]
    public void AUtf8BomOnStdinIsTrimmedBeforeParsing()
    {
        // `$json | aware-sketchup bake-scene --json-stdin` — the invocation the
        // command doc shows — prefixes a BOM from PowerShell, and System.Text.Json
        // rejects it outright. Every stdin verb here reads through TrimJsonBom.
        Assert.Equal("{\"verb\":\"bake-scene\"}", Program.TrimJsonBom("\uFEFF{\"verb\":\"bake-scene\"}"));
        Assert.Equal("{}", Program.TrimJsonBom("{}"));
        Assert.Equal("", Program.TrimJsonBom("\uFEFF"));
    }

    [Fact]
    public void MissingSceneIsACallerError()
    {
        using var sw = new StringWriter();
        int exit = Capture(sw, () => BakeScene.Run(JsonNode.Parse("""{"verb":"bake-scene"}"""),
                                                   new SketchupClient(_discoveryDir, _ => false)));
        Assert.Equal(2, exit);
        var receipt = JsonNode.Parse(sw.ToString().Trim())!;
        Assert.False(receipt["ok"]!.GetValue<bool>());
        Assert.Equal("bake-scene", receipt["verb"]!.GetValue<string>());
        Assert.Contains("missing required field: scene", receipt["error"]!.GetValue<string>());
    }

    [Fact]
    public void AnInvalidSceneIsRefusedBeforeAnyInstanceIsResolved()
    {
        // Exit 2 (not 1) proves the refusal came from preflight: with no live
        // instance in the discovery dir, host resolution would have exited 1.
        using var sw = new StringWriter();
        const string feetScene = """
            {"meta":{"units":"ft","sourceId":"s","sceneHash":"h"},"elements":[{"id":"m1","kind":"member"}]}
            """;
        int exit = Capture(sw, () => BakeScene.Run(Request(feetScene),
                                                   new SketchupClient(_discoveryDir, _ => false)));

        Assert.Equal(2, exit);
        var receipt = JsonNode.Parse(sw.ToString().Trim())!;
        Assert.False(receipt["ok"]!.GetValue<bool>());
        Assert.Contains(receipt["result"]!["failed"]!.AsArray(), r => Code(r) == "unsupported-units");
        Assert.Contains(receipt["result"]!["failed"]!.AsArray(), r => Code(r) == "batch-aborted");
    }

    [Fact]
    public void NoLiveInstanceIsADispatchFailure()
    {
        using var sw = new StringWriter();
        int exit = Capture(sw, () => BakeScene.Run(Request(ValidScene),
                                                   new SketchupClient(_discoveryDir, _ => false)));

        Assert.Equal(1, exit);
        Assert.Contains("no SketchUp instance", JsonNode.Parse(sw.ToString().Trim())!["error"]!.GetValue<string>());
    }

    [Fact]
    public void ASuccessfulBakeShipsTheMaterializerAndOwnershipToTheBridge()
    {
        JsonObject? seen = null;
        var (client, serverTask, pid) = StubBridge(
            req => { seen = req; return """{"ok":true,"result":{"ok":true,"emitted":[{"id":"m1","kind":"member","status":"emitted"}],"failed":[],"unsupported":[],"warnings":[]},"stdout_log":""}"""; });

        using var sw = new StringWriter();
        int exit = Capture(sw, () => BakeScene.Run(Request(ValidScene, pid, "FloLess"), client, timeoutMs: 10_000));
        serverTask.Wait(TimeSpan.FromSeconds(10));

        Assert.Equal(0, exit);
        Assert.NotNull(seen);
        // The bridge only speaks `exec`; bake-scene rides that path with the
        // injected materializer as its code, exactly like Tekla's Roslyn script.
        Assert.Equal("exec", seen!["type"]!.GetValue<string>());
        Assert.Equal(BakeSceneScript.Code, seen["code"]!.GetValue<string>());

        var args = seen["args"]!;
        Assert.Equal("FloLess", args["label"]!.GetValue<string>());
        Assert.Equal("m1", Id(args["supported"]!.AsArray()[0]));
        Assert.Equal("AWARE", args["ownership"]!["dictionary"]!.GetValue<string>());
        Assert.Equal("src-a", args["ownership"]!["sourceId"]!.GetValue<string>());
        Assert.True(BakeSceneOwnership.IsOwnershipMarker(args["ownership"]!["marker"]!.GetValue<string>()));
        Assert.Equal(BakeSceneOwnership.MarkerPattern, args["ownership"]!["markerPattern"]!.GetValue<string>());
        // The scene is handed over untouched — millimetres are the contract; the
        // host does the inch conversion.
        Assert.Equal("mm", args["scene"]!["meta"]!["units"]!.GetValue<string>());

        var receipt = JsonNode.Parse(sw.ToString().Trim())!;
        Assert.True(receipt["ok"]!.GetValue<bool>());
        Assert.Equal("bake-scene", receipt["verb"]!.GetValue<string>());
        Assert.Equal("m1", Id(receipt["result"]!["emitted"]!.AsArray()[0]));
    }

    [Fact]
    public void ABakeThatReportsOkFalseExitsNonZeroAndKeepsItsReceipt()
    {
        var (client, serverTask, pid) = StubBridge(
            _ => """{"ok":true,"result":{"ok":false,"emitted":[],"failed":[{"id":"m1","kind":"member","status":"failed","code":"invalid-geometry","message":"boom"}],"unsupported":[],"warnings":[]},"stdout_log":""}""");

        using var sw = new StringWriter();
        int exit = Capture(sw, () => BakeScene.Run(Request(ValidScene, pid), client, timeoutMs: 10_000));
        serverTask.Wait(TimeSpan.FromSeconds(10));

        Assert.Equal(2, exit);
        var receipt = JsonNode.Parse(sw.ToString().Trim())!;
        Assert.False(receipt["ok"]!.GetValue<bool>());
        Assert.Equal("invalid-geometry", Code(receipt["result"]!["failed"]!.AsArray()[0]));
    }

    [Fact]
    public void UnsupportedKindsSurviveIntoTheDeliveredReceipt()
    {
        var (client, serverTask, pid) = StubBridge(
            _ => """{"ok":true,"result":{"ok":true,"emitted":[],"failed":[],"unsupported":[],"warnings":[]},"stdout_log":""}""");

        using var sw = new StringWriter();
        const string plateOnly = """
            {"meta":{"units":"mm","sourceId":"src-a","sceneHash":"h"},"elements":[{"id":"p1","kind":"plate"}]}
            """;
        int exit = Capture(sw, () => BakeScene.Run(Request(plateOnly, pid), client, timeoutMs: 10_000));
        serverTask.Wait(TimeSpan.FromSeconds(10));

        Assert.Equal(0, exit);
        var rows = JsonNode.Parse(sw.ToString().Trim())!["result"]!["unsupported"]!.AsArray();
        Assert.Single(rows);
        Assert.Equal("p1", Id(rows[0]));
        Assert.Equal("unsupported-kind", Code(rows[0]));
    }

    // ── helpers ──────────────────────────────────────────────────────────────

    /// <summary>A minimal but complete member scene: a 3 m W12X26 beam along +X.</summary>
    const string ValidScene = """
        {"meta":{"units":"mm","sourceId":"src-a","sceneHash":"h1"},
         "elements":[{"id":"m1","kind":"member","from":[0,0,0],"to":[3000,0,0],
                      "section":{"w":102,"d":310},"meta":{"profile":"W12X26"}}]}
        """;

    static JsonNode? Request(string sceneJson, int? pid = null, string? label = null)
    {
        var head = "{\"verb\":\"bake-scene\"";
        if (pid is not null) head += ",\"sketchup_id\":\"" + pid + "\"";
        if (label is not null) head += ",\"label\":\"" + label + "\"";
        return JsonNode.Parse(head + ",\"scene\":" + sceneJson + "}");
    }

    static JsonNode? Scene(string unitsFragment) => JsonNode.Parse(
        "{\"meta\":{" + unitsFragment + "\"sourceId\":\"s\",\"sceneHash\":\"h\"},\"elements\":[]}");

    static string Id(JsonNode? r) => r?["id"]?.GetValue<string>() ?? "";
    static string Kind(JsonNode? r) => r?["kind"]?.GetValue<string>() ?? "";
    static string Status(JsonNode? r) => r?["status"]?.GetValue<string>() ?? "";
    static string Code(JsonNode? r) => r?["code"]?.GetValue<string>() ?? "";
    static string Message(JsonNode? r) => r?["message"]?.GetValue<string>() ?? "";

    static int Occurrences(string haystack, string needle)
    {
        int n = 0, i = 0;
        while ((i = haystack.IndexOf(needle, i, StringComparison.Ordinal)) >= 0) { n++; i += needle.Length; }
        return n;
    }

    static int Capture(StringWriter sw, Func<int> body)
    {
        var original = Console.Out;
        Console.SetOut(sw);
        try { return body(); }
        finally { Console.SetOut(original); }
    }

    (SketchupClient client, Task serverTask, int pid) StubBridge(Func<JsonObject, string> respond)
    {
        var listener = new TcpListener(IPAddress.Loopback, 0);
        listener.Start();
        var port = ((IPEndPoint)listener.LocalEndpoint).Port;
        var pid = System.Diagnostics.Process.GetCurrentProcess().Id;

        File.WriteAllText(Path.Combine(_discoveryDir, $"{pid}.json"),
            $"{{\"pid\":{pid},\"port\":{port},\"version\":\"26.0\",\"started_at\":\"2026-01-01T00:00:00Z\"}}");

        var serverTask = Task.Run(() =>
        {
            using var conn = listener.AcceptTcpClient();
            using var stream = conn.GetStream();
            var raw = SketchupClient.ReadLengthPrefixed(stream, timeoutMs: 5000);
            var request = (JsonNode.Parse(Encoding.UTF8.GetString(raw)) as JsonObject)!;
            SketchupClient.WriteLengthPrefixed(stream, Encoding.UTF8.GetBytes(respond(request)));
            listener.Stop();
        });

        return (new SketchupClient(_discoveryDir, p => p == pid), serverTask, pid);
    }
}
