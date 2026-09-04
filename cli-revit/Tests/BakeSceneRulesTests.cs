// Unit tests for the host-neutral half of bake-scene. AwareBakeRules lives in
// the global namespace because the same source is spliced into the Roslyn script
// that runs inside Revit — so these tests pin the text that mutates the model.

using System.Text.Json;
using Xunit;

namespace AwareRevit.Tests;

public class BakeSceneRulesTests
{
    static JsonElement Json(string raw) => JsonDocument.Parse(raw).RootElement.Clone();

    // ── units ────────────────────────────────────────────────────────────────

    [Fact]
    public void MmToFeet_ConvertsToRevitInternalDecimalFeet()
    {
        Assert.Equal(1.0, AwareBakeRules.MmToFeet(304.8), 12);
        Assert.Equal(0.0, AwareBakeRules.MmToFeet(0.0), 12);
        Assert.Equal(-10.0, AwareBakeRules.MmToFeet(-3048.0), 12);
        // 3600 mm ≈ 11.811 ft — the number a 3.6 m column must land on.
        Assert.Equal(11.81102362204724, AwareBakeRules.MmToFeet(3600.0), 10);
    }

    [Fact]
    public void MmToFeet_RoundTripsThroughFeetToMm()
    {
        Assert.Equal(1234.5, AwareBakeRules.FeetToMm(AwareBakeRules.MmToFeet(1234.5)), 9);
    }

    // ── ownership schema ─────────────────────────────────────────────────────

    /// <summary>The schema GUID is how a later bake recognises the elements an
    /// earlier bake owns. It must parse, and it must never drift.</summary>
    [Fact]
    public void SchemaGuid_IsAStableParsableGuid()
    {
        Assert.Equal(Guid.Parse("9C4C7F1E-1B3A-4E7D-9A2F-6D5B8C0E4A11"),
            Guid.Parse(AwareBakeRules.SchemaGuid));
        Assert.Equal("AwareBakeV1", AwareBakeRules.SchemaName);
        Assert.Equal("AWRE", AwareBakeRules.SchemaVendorId); // AwareRevit.addin <VendorId>
    }

    [Fact]
    public void SchemaFields_CarrySourceSceneMaterializationAndRecordIdentity()
    {
        Assert.Equal(
            new[] { "SourceId", "SceneHash", "MaterializationHash", "RecordId" },
            AwareBakeRules.SchemaFields());
    }

    // ── profile-name normalisation ───────────────────────────────────────────

    [Theory]
    [InlineData("W16x50", "W16X50")]
    [InlineData("w16x50", "W16X50")]
    [InlineData("W16X50", "W16X50")]
    [InlineData(" W 16 x 50 ", "W16X50")]
    [InlineData("hss6x6x1/4", "HSS6X6X1/4")]
    [InlineData("", "")]
    public void NormalizeProfile_UpperCasesAndStripsWhitespace(string input, string expected)
    {
        Assert.Equal(expected, AwareBakeRules.NormalizeProfile(input));
    }

    /// <summary>THE load-bearing case: a live Revit document ships both `W16X26`
    /// and `W16x50`. A case-sensitive lookup silently loses half the members to
    /// the DirectShape fallback, so both must resolve whatever the case.</summary>
    [Fact]
    public void TryResolveProfile_MatchesMixedCaseTypeNamesInTheSameDocument()
    {
        var index = AwareBakeRules.IndexByNormalizedName(new[]
        {
            new KeyValuePair<string, string>("W16X26", "symbol-upper"),
            new KeyValuePair<string, string>("W16x50", "symbol-lower"),
        });

        string match;
        Assert.True(AwareBakeRules.TryResolveProfile(index, "W16X50", out match));
        Assert.Equal("symbol-lower", match);

        Assert.True(AwareBakeRules.TryResolveProfile(index, "W16x50", out match));
        Assert.Equal("symbol-lower", match);

        Assert.True(AwareBakeRules.TryResolveProfile(index, "w16x26", out match));
        Assert.Equal("symbol-upper", match);

        Assert.True(AwareBakeRules.TryResolveProfile(index, " W16 X 26 ", out match));
        Assert.Equal("symbol-upper", match);
    }

    [Fact]
    public void TryResolveProfile_MissesAreReportedNotGuessed()
    {
        var index = AwareBakeRules.IndexByNormalizedName(new[]
        {
            new KeyValuePair<string, string>("W16x50", "symbol-lower"),
        });

        string match;
        Assert.False(AwareBakeRules.TryResolveProfile(index, "W18X35", out match));
        Assert.Null(match);
        Assert.False(AwareBakeRules.TryResolveProfile(index, "", out match));
        Assert.False(AwareBakeRules.TryResolveProfile(index, "   ", out match));
    }

    [Fact]
    public void IndexByNormalizedName_FirstOccurrenceWinsOnCollision()
    {
        var index = AwareBakeRules.IndexByNormalizedName(new[]
        {
            new KeyValuePair<string, string>("W16X50", "first"),
            new KeyValuePair<string, string>("w16x50", "second"),
        });
        Assert.Single(index);
        Assert.Equal("first", index["W16X50"]);
    }

    // ── id validation ────────────────────────────────────────────────────────

    [Theory]
    [InlineData("m1", true)]
    [InlineData("beam-B12/level-2", true)]
    [InlineData("", false)]
    [InlineData("   ", false)]
    [InlineData(" m1", false)]      // untrimmed
    [InlineData("m1 ", false)]      // untrimmed
    [InlineData("m\t1", false)]     // control character
    [InlineData("m\u007f1", false)] // DEL
    public void IsAcceptableId_MatchesTheTeklaSinkRule(string id, bool expected)
    {
        Assert.Equal(expected, AwareBakeRules.IsAcceptableId(id));
    }

    [Fact]
    public void IsAcceptableId_CapsAt256Utf8Bytes()
    {
        Assert.True(AwareBakeRules.IsAcceptableId(new string('a', 256)));
        Assert.False(AwareBakeRules.IsAcceptableId(new string('a', 257)));
        // A 128-character string of 2-byte code points is exactly 256 bytes.
        Assert.True(AwareBakeRules.IsAcceptableId(new string('ä', 128)));
        Assert.False(AwareBakeRules.IsAcceptableId(new string('ä', 129)));
    }

    // ── receipt rows ─────────────────────────────────────────────────────────

    [Fact]
    public void Row_CarriesIdKindStatusAndOmitsEmptyCodeAndMessage()
    {
        var emitted = AwareBakeRules.Row("m1", "member", "emitted", "", "");
        Assert.Equal(new[] { "id", "kind", "status" }, emitted.Keys.ToArray());
        Assert.Equal("m1", emitted["id"]);
        Assert.Equal("member", emitted["kind"]);
        Assert.Equal("emitted", emitted["status"]);

        var failed = AwareBakeRules.Row("m1", "member", "failed", "invalid-id", "why");
        Assert.Equal("invalid-id", failed["code"]);
        Assert.Equal("why", failed["message"]);
    }

    // ── scene parsing / receipt shape ────────────────────────────────────────

    const string TwoMemberScene = """
    {
      "meta": { "name": "Bay 1", "units": "mm", "sourceId": "src-a", "sceneHash": "hash-a" },
      "elements": [
        { "id": "m1", "kind": "member", "from": [0,0,0], "to": [5000,0,0], "meta": { "profile": "W16x50" } },
        { "id": "m2", "kind": "member", "from": [0,0,0], "to": [0,0,3600], "meta": { "profile": "W12X26", "name": "C-1" } }
      ]
    }
    """;

    [Fact]
    public void ParseScene_ReadsMembersProfilesAndIdentity()
    {
        var parsed = AwareBakeRules.ParseScene(Json(TwoMemberScene));

        Assert.True(parsed.Ok);
        Assert.Empty(parsed.Failed);
        Assert.Equal("Bay 1", parsed.Name);
        Assert.Equal("src-a", parsed.SourceId);
        Assert.Equal("hash-a", parsed.SceneHash);
        Assert.Equal(2, parsed.Members.Count);
        Assert.Equal("W16x50", parsed.Members[0].Profile);
        Assert.Equal(5000.0, parsed.Members[0].ToMm[0]);
        Assert.Equal("W12X26", parsed.Members[1].Profile);
        Assert.Equal("C-1", parsed.Members[1].Name);
        Assert.Equal(new[] { "m1", "m2" }, parsed.SupportedOrder.Select(r => r.Id).ToArray());
    }

    [Fact]
    public void ParseScene_RejectsNonMillimetreUnits()
    {
        var parsed = AwareBakeRules.ParseScene(Json("""
        { "meta": { "units": "in", "sourceId": "s", "sceneHash": "h" }, "elements": [] }
        """));
        Assert.False(parsed.Ok);
        Assert.Contains(parsed.Failed, r => (string)r["code"] == "unsupported-units");
    }

    [Fact]
    public void ParseScene_AbsentUnitsMeansLegacyMillimetres()
    {
        var parsed = AwareBakeRules.ParseScene(Json("""
        { "meta": { "sourceId": "s", "sceneHash": "h" }, "elements": [] }
        """));
        Assert.True(parsed.Ok);
    }

    [Fact]
    public void ParseScene_RequiresSourceIdAndSceneHash()
    {
        var parsed = AwareBakeRules.ParseScene(Json("""{ "meta": { "units": "mm" } }"""));
        Assert.False(parsed.Ok);
        Assert.Contains(parsed.Failed, r => (string)r["code"] == "missing-source-id");
        Assert.Contains(parsed.Failed, r => (string)r["code"] == "missing-scene-hash");
    }

    [Fact]
    public void ParseScene_RejectsNonObjectScene()
    {
        var parsed = AwareBakeRules.ParseScene(Json("[]"));
        Assert.False(parsed.Ok);
        Assert.Equal("invalid-scene", (string)parsed.Failed[0]["code"]);
    }

    [Fact]
    public void ParseScene_RejectsDuplicateAndMalformedIds()
    {
        var parsed = AwareBakeRules.ParseScene(Json("""
        {
          "meta": { "units": "mm", "sourceId": "s", "sceneHash": "h" },
          "elements": [
            { "id": "m1", "from": [0,0,0], "to": [1000,0,0], "meta": { "profile": "W16X50" } },
            { "id": "m1", "from": [0,0,0], "to": [1000,0,0], "meta": { "profile": "W16X50" } },
            { "id": " m2", "from": [0,0,0], "to": [1000,0,0], "meta": { "profile": "W16X50" } }
          ]
        }
        """));
        Assert.False(parsed.Ok);
        Assert.Contains(parsed.Failed, r => (string)r["code"] == "duplicate-id");
        Assert.Contains(parsed.Failed, r => (string)r["code"] == "invalid-id");
    }

    [Fact]
    public void ParseScene_RejectsCollectionsThatAreNotArrays()
    {
        var parsed = AwareBakeRules.ParseScene(Json("""
        { "meta": { "units": "mm", "sourceId": "s", "sceneHash": "h" }, "elements": {} }
        """));
        Assert.False(parsed.Ok);
        Assert.Contains(parsed.Failed, r => (string)r["code"] == "invalid-collection");
    }

    [Fact]
    public void ParseScene_RejectsMembersWithoutAFiniteAxisOrProfile()
    {
        var parsed = AwareBakeRules.ParseScene(Json("""
        {
          "meta": { "units": "mm", "sourceId": "s", "sceneHash": "h" },
          "elements": [
            { "id": "zero", "from": [0,0,0], "to": [0,0,0], "meta": { "profile": "W16X50" } },
            { "id": "noprofile", "from": [0,0,0], "to": [1000,0,0] },
            { "id": "nan", "from": [0,0,0], "to": ["oops",0,0], "meta": { "profile": "W16X50" } }
          ]
        }
        """));
        Assert.False(parsed.Ok);
        Assert.Equal(3, parsed.Failed.Count(r => (string)r["code"] == "invalid-geometry"));
        Assert.Empty(parsed.Members);
    }

    /// <summary>Kinds the Revit sink cannot build yet are REPORTED, never dropped.</summary>
    [Fact]
    public void ParseScene_ReportsUnsupportedKindsOperationsAndReferenceSystems()
    {
        var parsed = AwareBakeRules.ParseScene(Json("""
        {
          "meta": { "units": "mm", "sourceId": "s", "sceneHash": "h" },
          "elements": [
            { "id": "m1", "from": [0,0,0], "to": [1000,0,0], "meta": { "profile": "W16X50" } },
            { "id": "p1", "kind": "plate", "holes": [ { "id": "h1" } ] },
            { "id": "b1", "kind": "bolt-shank" }
          ],
          "operations": [ { "id": "w1", "kind": "weld" } ],
          "referenceSystems": [ { "id": "g1", "kind": "structural-grid", "axes": [ { "id": "a1" } ] } ]
        }
        """));

        Assert.True(parsed.Ok);
        Assert.Single(parsed.Members);
        var codes = parsed.Unsupported.Select(r => (string)r["id"]).OrderBy(x => x).ToArray();
        Assert.Equal(new[] { "a1", "b1", "g1", "h1", "p1", "w1" }, codes);
        Assert.Equal("unsupported-kind", (string)parsed.Unsupported.Single(r => (string)r["id"] == "p1")["code"]);
        Assert.Equal("unsupported-parent", (string)parsed.Unsupported.Single(r => (string)r["id"] == "h1")["code"]);
        Assert.Equal("unsupported-operation", (string)parsed.Unsupported.Single(r => (string)r["id"] == "w1")["code"]);
        Assert.Equal("unsupported-reference-system",
            (string)parsed.Unsupported.Single(r => (string)r["id"] == "g1")["code"]);
    }

    /// <summary>
    /// EVERY record the canonical scene carries gets exactly one row — including the ones nested
    /// inside another record. A bolt instance's hole effects sit TWO levels below the operation and
    /// used to be absent from all three arrays, which made a connection model unexportable: the
    /// consumer reconciles the receipt against the scene and refuses a bake that cannot say what
    /// became of a record (aware-aeco/aware#326). The ids are asserted as a SET, so a future
    /// collection that is added to the scene and forgotten here fails rather than passing unnoticed.
    /// </summary>
    [Fact]
    public void ParseScene_ClassifiesEveryNestedRecord()
    {
        var parsed = AwareBakeRules.ParseScene(Json("""
        {
          "meta": { "units": "mm", "sourceId": "s", "sceneHash": "h" },
          "elements": [
            { "id": "m1", "from": [0,0,0], "to": [1000,0,0], "meta": { "profile": "W16X50" } },
            { "id": "p1", "kind": "plate", "holes": [ { "id": "p1-h0" }, { "id": "p1-h1" } ] }
          ],
          "operations": [
            { "id": "ba1", "kind": "bolt-array", "instances": [
                { "id": "ba1-i0", "holeEffects": [ { "id": "ba1-i0-hp" }, { "id": "ba1-i0-hm" } ] },
                { "id": "ba1-i1", "holeEffects": [ { "id": "ba1-i1-hp" } ] } ] }
          ],
          "referenceSystems": [
            { "id": "g1", "kind": "structural-grid", "axes": [ { "id": "g1-a" } ], "levels": [ { "id": "g1-l" } ] }
          ]
        }
        """));

        Assert.True(parsed.Ok);
        var classified = parsed.SupportedOrder.Select(r => r.Id)
            .Concat(parsed.Unsupported.Select(r => (string)r["id"]))
            .Concat(parsed.Failed.Select(r => (string)r["id"]))
            .ToArray();
        Assert.Equal(classified.Length, classified.Distinct().Count());
        Assert.Equal(
            new[] { "ba1", "ba1-i0", "ba1-i0-hm", "ba1-i0-hp", "ba1-i1", "ba1-i1-hp",
                    "g1", "g1-a", "g1-l", "m1", "p1", "p1-h0", "p1-h1" },
            classified.OrderBy(x => x, StringComparer.Ordinal).ToArray());

        // The kind is the vocabulary the consumer reconciles against, and it is Tekla's: a hole —
        // whether it belongs to a plate or to a bolt instance — is an `opening`.
        foreach (var openingId in new[] { "p1-h0", "p1-h1", "ba1-i0-hp", "ba1-i0-hm", "ba1-i1-hp" })
        {
            var row = parsed.Unsupported.Single(r => (string)r["id"] == openingId);
            Assert.Equal("opening", (string)row["kind"]);
            Assert.Equal("unsupported-parent", (string)row["code"]);
        }
        Assert.Equal("bolt-instance", (string)parsed.Unsupported.Single(r => (string)r["id"] == "ba1-i0")["kind"]);
        Assert.Equal("grid-axis", (string)parsed.Unsupported.Single(r => (string)r["id"] == "g1-a")["kind"]);
        Assert.Equal("grid-level", (string)parsed.Unsupported.Single(r => (string)r["id"] == "g1-l")["kind"]);
    }

    /// <summary>
    /// The other half of the same contract: a row for an id the scene never declared is as
    /// unreconcilable as a missing one, so a nested collection is only read off the record family
    /// that canonically owns it — `holes` off a plate, `instances` off a bolt array, `axes`/`levels`
    /// off a reference system.
    /// </summary>
    [Fact]
    public void ParseScene_ReadsNestedCollectionsOnlyOffTheKindThatOwnsThem()
    {
        var parsed = AwareBakeRules.ParseScene(Json("""
        {
          "meta": { "units": "mm", "sourceId": "s", "sceneHash": "h" },
          "elements": [ { "id": "b1", "kind": "bolt-shank", "holes": [ { "id": "not-a-record" } ] } ],
          "operations": [ { "id": "w1", "kind": "weld", "instances": [ { "id": "also-not" } ],
                            "axes": [ { "id": "nor-this" } ] } ],
          "referenceSystems": [ { "id": "g1", "kind": "structural-grid", "axes": [ { "id": "g1-a" } ] } ]
        }
        """));

        Assert.True(parsed.Ok);
        var classified = parsed.SupportedOrder.Select(r => r.Id)
            .Concat(parsed.Unsupported.Select(r => (string)r["id"]))
            .Concat(parsed.Failed.Select(r => (string)r["id"]));
        Assert.Equal(new[] { "b1", "g1", "g1-a", "w1" },
            classified.OrderBy(x => x, StringComparer.Ordinal).ToArray());
    }

    /// <summary>A nested record with a duplicate or malformed id is caught by the same gate as a
    /// top-level one — ids are unique across the WHOLE scene, nesting included.</summary>
    [Fact]
    public void ParseScene_AppliesTheSceneWideIdGateToNestedRecords()
    {
        var parsed = AwareBakeRules.ParseScene(Json("""
        {
          "meta": { "units": "mm", "sourceId": "s", "sceneHash": "h" },
          "operations": [
            { "id": "ba1", "kind": "bolt-array", "instances": [
                { "id": "i0", "holeEffects": [ { "id": "dup" }, { "id": "dup" } ] } ] }
          ]
        }
        """));

        Assert.False(parsed.Ok);
        Assert.Contains(parsed.Failed, r => (string)r["id"] == "dup" && (string)r["code"] == "duplicate-id");
    }

    /// <summary>
    /// A rejected bolt-instance id does not take its hole effects down with it. The effects are
    /// records in their own right, so they still get their rows AND still enter the scene-wide id
    /// set — otherwise a duplicate among the effects would slip through behind a bad instance id.
    /// This is what cli-tekla and cli-rhino already do.
    /// </summary>
    [Fact]
    public void ParseScene_ClassifiesHoleEffectsEvenWhenTheirInstanceIdIsRejected()
    {
        var parsed = AwareBakeRules.ParseScene(Json("""
        {
          "meta": { "units": "mm", "sourceId": "s", "sceneHash": "h" },
          "elements": [ { "id": "ba1-i0", "kind": "plate" } ],
          "operations": [
            { "id": "ba1", "kind": "bolt-array", "instances": [
                { "id": "ba1-i0", "holeEffects": [ { "id": "eff-a" }, { "id": "eff-b" } ] },
                { "id": "ba1-i1", "holeEffects": [ { "id": "eff-b" } ] } ] }
          ]
        }
        """));

        // The instance id collides with the plate's, so the instance itself is a failure...
        Assert.False(parsed.Ok);
        Assert.Contains(parsed.Failed, r => (string)r["id"] == "ba1-i0" && (string)r["code"] == "duplicate-id");
        // ...but its effects are classified anyway, and the duplicate BETWEEN two instances' effects
        // is still caught — both of which the old `continue` would have lost.
        Assert.Contains(parsed.Unsupported, r => (string)r["id"] == "eff-a" && (string)r["kind"] == "opening");
        Assert.Contains(parsed.Unsupported, r => (string)r["id"] == "eff-b" && (string)r["kind"] == "opening");
        Assert.Contains(parsed.Failed, r => (string)r["id"] == "eff-b" && (string)r["code"] == "duplicate-id");
    }

    /// <summary>
    /// The same rule one level UP (#327): a rejected parent id withholds only the parent's own row,
    /// never its nested collection. Otherwise a duplicate hiding in the subtree of a bad parent only
    /// surfaces after the parent is fixed, and the refusal names the bad parent while saying nothing
    /// about what is underneath it.
    /// </summary>
    [Fact]
    public void ParseScene_ClassifiesNestedRecordsEvenWhenTheParentIdIsRejected()
    {
        var parsed = AwareBakeRules.ParseScene(Json("""
        {
          "meta": { "units": "mm", "sourceId": "s", "sceneHash": "h" },
          "elements": [
            { "id": "dup", "kind": "member" },
            { "id": "dup", "kind": "plate", "holes": [ { "id": "h-a" }, { "id": "h-b" } ] }
          ],
          "referenceSystems": [
            { "id": " bad ", "kind": "structural-grid",
              "axes": [ { "id": "ax-a" } ], "levels": [ { "id": "h-b" } ] }
          ]
        }
        """));

        Assert.False(parsed.Ok);
        // The plate's id collides with the member's, so the plate itself fails and
        // contributes no row of its own...
        Assert.Contains(parsed.Failed, r => (string)r["id"] == "dup" && (string)r["code"] == "duplicate-id");
        Assert.DoesNotContain(parsed.Unsupported, r => (string)r["id"] == "dup");
        // ...but its holes are classified anyway, as are the axes beneath a malformed
        // reference-system id.
        Assert.Contains(parsed.Unsupported, r => (string)r["id"] == "h-a" && (string)r["kind"] == "opening");
        Assert.Contains(parsed.Unsupported, r => (string)r["id"] == "ax-a" && (string)r["kind"] == "grid-axis");
        // ...and they still claim ids, so the duplicate ACROSS two rejected parents'
        // subtrees is caught — exactly what the old `continue` dropped.
        Assert.Contains(parsed.Failed, r => (string)r["id"] == "h-b" && (string)r["code"] == "duplicate-id");
    }

    [Fact]
    public void AppendBatchAborted_GivesEverySupportedRecordAVerdict()
    {
        var parsed = AwareBakeRules.ParseScene(Json("""
        {
          "meta": { "units": "cm", "sourceId": "s", "sceneHash": "h" },
          "elements": [
            { "id": "m1", "from": [0,0,0], "to": [1000,0,0], "meta": { "profile": "W16X50" } },
            { "id": "m2", "from": [0,0,0], "to": [0,1000,0], "meta": { "profile": "W16X50" } }
          ]
        }
        """));
        Assert.False(parsed.Ok);

        AwareBakeRules.AppendBatchAborted(parsed, "aborted");
        Assert.Contains(parsed.Failed, r => (string)r["id"] == "m1" && (string)r["code"] == "batch-aborted");
        Assert.Contains(parsed.Failed, r => (string)r["id"] == "m2" && (string)r["code"] == "batch-aborted");
    }

    [Fact]
    public void AppendBatchAborted_DoesNotOverwriteARecordsOwnVerdict()
    {
        var parsed = AwareBakeRules.ParseScene(Json("""
        {
          "meta": { "units": "mm", "sourceId": "s", "sceneHash": "h" },
          "elements": [
            { "id": "m1", "from": [0,0,0], "to": [1000,0,0], "meta": { "profile": "W16X50" } },
            { "id": "bad", "from": [0,0,0], "to": [0,0,0], "meta": { "profile": "W16X50" } }
          ]
        }
        """));
        AwareBakeRules.AppendBatchAborted(parsed, "aborted");
        Assert.Equal("invalid-geometry", (string)parsed.Failed.Single(r => (string)r["id"] == "bad")["code"]);
        Assert.Equal("batch-aborted", (string)parsed.Failed.Single(r => (string)r["id"] == "m1")["code"]);
    }

    // ── column vs beam ───────────────────────────────────────────────────────

    [Theory]
    [InlineData(0, 0, 3600, true)]     // plumb up
    [InlineData(0, 0, -3600, true)]    // plumb down
    [InlineData(5000, 0, 0, false)]    // flat beam
    [InlineData(5000, 0, 300, false)]  // sloped beam
    [InlineData(0, 0, 0, false)]       // degenerate
    [InlineData(10, 0, 3600, true)]    // 0.16 deg off plumb — still a column
    [InlineData(200, 0, 3600, false)]  // 3.2 deg off plumb — framing
    public void IsVerticalAxis_SeparatesColumnsFromFraming(double dx, double dy, double dz, bool expected)
    {
        Assert.Equal(expected, AwareBakeRules.IsVerticalAxis(dx, dy, dz));
    }

    // ── level choice ─────────────────────────────────────────────────────────

    [Fact]
    public void PickLevelIndex_TakesTheNearestLevelAtOrBelow()
    {
        var elevations = new List<double> { 0.0, 10.0, 20.0 };
        Assert.Equal(0, AwareBakeRules.PickLevelIndex(elevations, 0.0));
        Assert.Equal(0, AwareBakeRules.PickLevelIndex(elevations, 9.99));
        Assert.Equal(1, AwareBakeRules.PickLevelIndex(elevations, 10.0));
        Assert.Equal(1, AwareBakeRules.PickLevelIndex(elevations, 19.5));
        Assert.Equal(2, AwareBakeRules.PickLevelIndex(elevations, 1000.0));
    }

    [Fact]
    public void PickLevelIndex_FallsBackToTheLowestLevelBelowTheStack()
    {
        Assert.Equal(0, AwareBakeRules.PickLevelIndex(new List<double> { 5.0, 15.0 }, -20.0));
    }

    [Fact]
    public void PickLevelIndex_ReportsMinusOneWhenTheDocumentHasNoLevel()
    {
        Assert.Equal(-1, AwareBakeRules.PickLevelIndex(new List<double>(), 0.0));
        Assert.Equal(-1, AwareBakeRules.PickLevelIndex(null, 0.0));
    }

    // ── fallback placeholder section ─────────────────────────────────────────

    [Fact]
    public void TryParseNominalSectionMm_ReadsImperialAndMetricDesignations()
    {
        double depth, width;

        Assert.True(AwareBakeRules.TryParseNominalSectionMm("W12X26", out depth, out width));
        Assert.Equal(304.8, depth, 6);   // 12 in
        Assert.Equal(152.4, width, 6);   // placeholder half-depth

        Assert.True(AwareBakeRules.TryParseNominalSectionMm("hss6x6x1/4", out depth, out width));
        Assert.Equal(152.4, depth, 6);
        Assert.Equal(152.4, width, 6);   // the second token IS a width for boxes

        Assert.True(AwareBakeRules.TryParseNominalSectionMm("UB305X165X40", out depth, out width));
        Assert.Equal(305.0, depth, 6);
        Assert.Equal(165.0, width, 6);

        // The unit comes from the PREFIX, never the magnitude: IPE80 is 80 mm and
        // W44 is 44 inches, and a magnitude rule would get both wrong.
        Assert.True(AwareBakeRules.TryParseNominalSectionMm("IPE80", out depth, out width));
        Assert.Equal(80.0, depth, 6);
        Assert.True(AwareBakeRules.TryParseNominalSectionMm("W44X335", out depth, out width));
        Assert.Equal(1117.6, depth, 6);
    }

    [Fact]
    public void TryParseNominalSectionMm_FallsBackWhenTheDesignationIsUnrecognised()
    {
        double depth, width;
        Assert.False(AwareBakeRules.TryParseNominalSectionMm("MYSTERY-42", out depth, out width));
        Assert.Equal(AwareBakeRules.FallbackSectionDepthMm, depth);
        Assert.Equal(AwareBakeRules.FallbackSectionWidthMm, width);

        Assert.False(AwareBakeRules.TryParseNominalSectionMm("", out depth, out width));
        Assert.False(AwareBakeRules.TryParseNominalSectionMm("W", out depth, out width));
        Assert.False(AwareBakeRules.TryParseNominalSectionMm("300", out depth, out width));
    }

    // ── retire-and-replace ───────────────────────────────────────────────────

    static AwareBakeRules.OwnedRecord Owned(long elementId, string sourceId, string recordId) =>
        new AwareBakeRules.OwnedRecord { ElementId = elementId, SourceId = sourceId, RecordId = recordId };

    [Fact]
    public void PlanRetirement_DeletesTheWholePriorSetOfThisSource()
    {
        var plan = AwareBakeRules.PlanRetirement(
            new[] { Owned(1, "src-a", "m1"), Owned(2, "src-a", "m2"), Owned(3, "src-a", "gone") },
            "src-a",
            new[] { "m1", "m2" },
            new long[] { 10, 11 });

        Assert.Equal(new long[] { 1, 2, 3 }, plan.Delete.Select(r => r.ElementId).ToArray());
        Assert.Empty(plan.Keep);
        // A record the incoming scene still carries was just re-created, so its
        // old element is `superseded`; one the scene dropped is `removed`.
        Assert.Equal(AwareBakeRules.ReasonSuperseded, plan.Delete.Single(r => r.RecordId == "m1").Reason);
        Assert.Equal(AwareBakeRules.ReasonSuperseded, plan.Delete.Single(r => r.RecordId == "m2").Reason);
        Assert.Equal(AwareBakeRules.ReasonRemoved, plan.Delete.Single(r => r.RecordId == "gone").Reason);
    }

    /// <summary>The safety property that stops a bake eating the user's model:
    /// another source's elements are NEVER deleted.</summary>
    [Fact]
    public void PlanRetirement_NeverTouchesAnotherSourcesElements()
    {
        var plan = AwareBakeRules.PlanRetirement(
            new[] { Owned(1, "src-a", "m1"), Owned(2, "src-b", "m1"), Owned(3, "", "m1") },
            "src-a",
            new[] { "m1" },
            Array.Empty<long>());

        Assert.Equal(new long[] { 1 }, plan.Delete.Select(r => r.ElementId).ToArray());
        Assert.Equal(new long[] { 2, 3 }, plan.Keep.Select(r => r.ElementId).ToArray());
        Assert.All(plan.Keep, r => Assert.Equal(AwareBakeRules.ReasonForeignSource, r.Reason));
    }

    /// <summary>The other safety property: an element THIS bake just created is
    /// never in the retirement set, even if the ownership collector saw it.</summary>
    [Fact]
    public void PlanRetirement_NeverDeletesWhatThisBakeJustCreated()
    {
        var plan = AwareBakeRules.PlanRetirement(
            new[] { Owned(1, "src-a", "m1"), Owned(99, "src-a", "m1") },
            "src-a",
            new[] { "m1" },
            new long[] { 99 });

        Assert.Equal(new long[] { 1 }, plan.Delete.Select(r => r.ElementId).ToArray());
        Assert.Equal(new long[] { 99 }, plan.Keep.Select(r => r.ElementId).ToArray());
        Assert.Equal(AwareBakeRules.ReasonJustCreated, plan.Keep[0].Reason);
    }

    [Fact]
    public void PlanRetirement_OnAFirstBakeDeletesNothing()
    {
        var plan = AwareBakeRules.PlanRetirement(
            Array.Empty<AwareBakeRules.OwnedRecord>(), "src-a", new[] { "m1" }, Array.Empty<long>());
        Assert.Empty(plan.Delete);
        Assert.Empty(plan.Keep);
    }

    [Fact]
    public void PlanRetirement_OnAnEmptyIncomingSceneStillRetiresThePriorSet()
    {
        var plan = AwareBakeRules.PlanRetirement(
            new[] { Owned(1, "src-a", "m1") }, "src-a", Array.Empty<string>(), Array.Empty<long>());
        Assert.Single(plan.Delete);
        Assert.Equal(AwareBakeRules.ReasonRemoved, plan.Delete[0].Reason);
    }

    // ── the SUPPLIED cross-section ──────────────────────────────────────────
    // Revit used to ignore `xsection` and rebuild a nominal rectangle from the designation, so a
    // `CHS508.0*14.2` became a 508 x 254 box. These pin the outline it builds instead. Point counts
    // are the discriminator a shape can be recognised by, exactly as they are in the live host.

    static System.Collections.Generic.Dictionary<string, double> Dims(params (string, double)[] pairs)
    {
        var d = new System.Collections.Generic.Dictionary<string, double>();
        foreach (var (k, v) in pairs) d[k] = v;
        return d;
    }

    [Fact]
    public void SectionOutline_RoundTubeIsACircleWithABore()
    {
        double[][] outer, inner;
        Assert.True(AwareBakeRules.TrySectionOutlineMm("chs", Dims(("od", 508), ("t", 14.2)), 508, 508, true, out outer, out inner));
        // 48 inscribed segments — the SAME count as floless's viewer and the SketchUp sink, so all
        // three draw one circle rather than three approximations of one.
        Assert.Equal(48, outer.Length);
        Assert.NotNull(inner);
        Assert.Equal(48, inner.Length);
        // Measured, not counted: the outer ring reaches the real radius and the bore is one wall in.
        var rOuter = System.Linq.Enumerable.Max(System.Linq.Enumerable.Select(outer, p => Math.Sqrt(p[0] * p[0] + p[1] * p[1])));
        var rInner = System.Linq.Enumerable.Max(System.Linq.Enumerable.Select(inner, p => Math.Sqrt(p[0] * p[0] + p[1] * p[1])));
        Assert.Equal(254.0, rOuter, 6);
        Assert.Equal(254.0 - 14.2, rInner, 6);
    }

    [Fact]
    public void SectionOutline_EachShapeHasItsOwnSignature()
    {
        double[][] outer, inner;
        Assert.True(AwareBakeRules.TrySectionOutlineMm("i", Dims(("tw", 6.7), ("tf", 11.8)), 165.7, 306.6, false, out outer, out inner));
        Assert.Equal(12, outer.Length);                        // an I traces 12 points
        Assert.Null(inner);

        Assert.True(AwareBakeRules.TrySectionOutlineMm("angle", Dims(("t", 10)), 100, 100, false, out outer, out inner));
        Assert.Equal(6, outer.Length);                         // an L traces 6 — NOT the 4 of a box
        Assert.Null(inner);

        Assert.True(AwareBakeRules.TrySectionOutlineMm("channel", Dims(("tw", 5), ("tf", 8.5)), 50, 100, false, out outer, out inner));
        Assert.Equal(8, outer.Length);

        Assert.True(AwareBakeRules.TrySectionOutlineMm("tee", Dims(("tw", 6), ("tf", 10)), 102, 127, false, out outer, out inner));
        Assert.Equal(8, outer.Length);                         // half an I, not the 12 of a full one

        Assert.True(AwareBakeRules.TrySectionOutlineMm("rhs", Dims(("t", 10)), 100, 100, false, out outer, out inner));
        Assert.Equal(4, outer.Length);
        Assert.NotNull(inner);                                 // a tube is hollow
        Assert.Equal(4, inner.Length);
    }

    [Fact]
    public void SectionOutline_RefusesAShapeTheCatalogueWillNotDraw()
    {
        // An ellipse is carried as data and refused as geometry. Inventing a rectangle for it here is
        // exactly the silent-wrong-answer this change exists to remove, so it must return false and
        // let the caller fall back to a placeholder that SAYS it is one.
        double[][] outer, inner;
        Assert.False(AwareBakeRules.TrySectionOutlineMm("unsupported-custom", Dims(), 150, 75, false, out outer, out inner));
        Assert.Null(outer);
        Assert.False(AwareBakeRules.TrySectionOutlineMm("joist", Dims(), 200, 400, false, out outer, out inner));
        // And a section with no envelope cannot be drawn at any shape.
        Assert.False(AwareBakeRules.TrySectionOutlineMm("i", Dims(("tw", 6)), 0, 300, false, out outer, out inner));
    }

    [Fact]
    public void SectionOutline_ARoundBarIsRoundAndASquareOneIsNot()
    {
        // `rect` covers both a solid bar and a plate; only `section.round` separates them, and the
        // tube branch makes the same distinction. Getting this wrong squares a round bar silently.
        double[][] outer, inner;
        Assert.True(AwareBakeRules.TrySectionOutlineMm("rect", Dims(), 50, 50, true, out outer, out inner));
        Assert.Equal(48, outer.Length);
        Assert.True(AwareBakeRules.TrySectionOutlineMm("rect", Dims(), 50, 50, false, out outer, out inner));
        Assert.Equal(4, outer.Length);
    }

    [Fact]
    public void SectionOutline_FallsBackToAnEstimateOnlyWhenAThicknessIsMissing()
    {
        // The catalogue's own thickness must win. Pinned by measuring the web position: a 6.7 mm web
        // puts the inner faces at +/-3.35, which the estimate (10% of width) would not.
        double[][] outer, inner;
        Assert.True(AwareBakeRules.TrySectionOutlineMm("i", Dims(("tw", 6.7), ("tf", 11.8)), 165.7, 306.6, false, out outer, out inner));
        var webHalf = System.Linq.Enumerable.Min(System.Linq.Enumerable.Select(
            System.Linq.Enumerable.Where(outer, p => p[0] > 0), p => p[0]));
        Assert.Equal(3.35, webHalf, 6);

        // With no `tw` supplied the estimate stands in, and it is a DIFFERENT number — so the first
        // assertion is measuring the catalogue value, not something both paths would produce.
        Assert.True(AwareBakeRules.TrySectionOutlineMm("i", Dims(("tf", 11.8)), 165.7, 306.6, false, out outer, out inner));
        var estimated = System.Linq.Enumerable.Min(System.Linq.Enumerable.Select(
            System.Linq.Enumerable.Where(outer, p => p[0] > 0), p => p[0]));
        Assert.True(Math.Abs(estimated - 3.35) > 1.0, "the estimate must differ from the catalogue web, got " + estimated);
    }
}
