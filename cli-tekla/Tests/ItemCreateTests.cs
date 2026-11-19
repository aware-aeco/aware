using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.Json.Nodes;
using Microsoft.CodeAnalysis;
using Xunit;

namespace AwareTekla.Tests;

/// <summary>
/// aware#859 (FloLess #3382 A5): the `item-create` verb. The request, the names (E6), the checks, the order of writes,
/// the partial-failure receipt and its undo (E9), and the delete guard are exercised here through a fake host that
/// keeps real catalogue files in a temp folder. The Tekla-facing script is parsed, pinned on its source and, where a
/// Tekla install is present, compiled against its assemblies; what Tekla does is proved by the PR's live run.
/// </summary>
public sealed class ItemCreateTests : IDisposable
{
    private readonly string _root = Path.Combine(Path.GetTempPath(), "aware-item-create-" + Guid.NewGuid().ToString("N"));

    public ItemCreateTests() { Directory.CreateDirectory(_root); }

    public void Dispose()
    {
        try { Directory.Delete(_root, true); } catch (IOException) { } catch (UnauthorizedAccessException) { }
    }

    // ── requests ──────────────────────────────────────────────────────────────

    const string Square = "[[-50,-50],[50,-50],[50,50],[-50,50]]";

    static string Member(string id, double length, string? extra = null, string outer = Square, string holes = "[]") =>
        $"{{\"id\":\"{id}\",\"label\":\"Member {id}\",\"material\":\"S355JR\"," +
        $"\"solid\":{{\"kind\":\"prism\",\"outer\":{outer},\"holes\":{holes},\"length\":{length.ToString(System.Globalization.CultureInfo.InvariantCulture)}}}," +
        $"\"placement\":{{\"start\":[0,0,0],\"end\":[{length.ToString(System.Globalization.CultureInfo.InvariantCulture)},0,0]}}{extra}}}";

    static string Request(string mode, string[] members, string? tail = null)
    {
        var consent = mode == "plan" ? "" : ",\"consent\":{\"source\":\"ui-click\"}";
        var target = mode == "plan" || (tail ?? "").Contains("\"target\"") ? "" : ",\"target\":{\"model_guid\":\"c7ce9de7-9f43-4950-b735-04698c3b9312\"}";
        return $"{{\"mode\":\"{mode}\"{consent}{target},\"members\":[{string.Join(",", members)}]{tail}}}";
    }

    static ItemCreateRequest Parsed(string json)
    {
        var refusal = TeklaItemCreate.Parse(JsonNode.Parse(json), out var request);
        Assert.Null(refusal);
        return request!;
    }

    static string? Refusal(string json) => TeklaItemCreate.Parse(JsonNode.Parse(json), out _);

    [Fact]
    public void Parse_AcceptsAPlanWithoutConsent_ButEveryWriteNeedsIt()
    {
        Assert.NotNull(Parsed(Request("plan", new[] { Member("M1", 1000) })));
        var noConsent = "{\"mode\":\"create\",\"members\":[" + Member("M1", 1000) + "]}";
        Assert.StartsWith("consent-required:", Refusal(noConsent));
        Assert.StartsWith("consent-required:", Refusal(noConsent.Replace("\"mode\":\"create\"", "\"mode\":\"create\",\"consent\":{\"source\":\"a-script\"}")));
        Assert.StartsWith("consent-required:", Refusal("{\"mode\":\"delete\",\"target\":{\"model_guid\":\"x\"},\"shapes\":[\"AWI-abc\"]}"));
        Assert.Null(Refusal(Request("create", new[] { Member("M1", 1000) }).Replace("ui-click", "chat-proposal-card")));
    }

    [Fact]
    public void Parse_RefusesBeforeTekla_EachByName()
    {
        // a ring crossing itself
        Assert.StartsWith("invalid-solid:", Refusal(Request("plan", new[] { Member("M1", 1000, outer: "[[0,0],[10,10],[10,0],[0,10]]") })));
        // a hole outside the outer ring, and a hole touching it
        Assert.StartsWith("invalid-solid:", Refusal(Request("plan", new[] { Member("M1", 1000, holes: "[[[60,0],[70,0],[70,10]]]") })));
        Assert.StartsWith("invalid-solid:", Refusal(Request("plan", new[] { Member("M1", 1000, holes: "[[[-50,0],[-40,0],[-40,10]]]") })));
        // not a prism, no length, two points
        Assert.StartsWith("invalid-solid:", Refusal(Request("plan", new[] { Member("M1", 1000).Replace("prism", "sphere") })));
        Assert.StartsWith("invalid-solid:", Refusal(Request("plan", new[] { Member("M1", 1000, outer: "[[0,0],[1,1]]") })));
        Assert.StartsWith("invalid-solid:", Refusal(Request("plan", new[] { Member("M1", 0) })));
        // a digest that is not the solid's
        Assert.StartsWith("digest-mismatch:", Refusal(Request("plan", new[] { Member("M1", 1000, ",\"solid_digest\":\"sha256:00\"") })));
        // Tekla does not stretch a shape, so end - start must be the solid's length
        Assert.StartsWith("length-mismatch:", Refusal(Request("plan", new[] { Member("M1", 1000).Replace("[1000,0,0]", "[1200,0,0]") })));
        // duplicate ids, empty list, bad mode, bad prefix
        Assert.StartsWith("invalid-request:", Refusal(Request("plan", new[] { Member("M1", 1000), Member("M1", 2000) })));
        Assert.StartsWith("invalid-request:", Refusal("{\"mode\":\"plan\",\"members\":[]}"));
        Assert.StartsWith("invalid-request:", Refusal("{\"mode\":\"explode\"}"));
        Assert.StartsWith("invalid-request:", Refusal(Request("plan", new[] { Member("M1", 1000) }, ",\"name_prefix\":\"bad name\"")));
    }

    [Fact]
    public void Parse_AcceptsTheSolidsOwnDigest()
    {
        var digest = Parsed(Request("plan", new[] { Member("M1", 1000) })).Members[0].SolidDigest;
        Assert.Null(Refusal(Request("plan", new[] { Member("M1", 1000, ",\"solid_digest\":\"" + digest + "\"") })));
    }

    [Fact]
    public void Parse_DeleteNeedsSomethingToDelete_AndAWholeNumberAcknowledgement()
    {
        const string head = "{\"mode\":\"delete\",\"consent\":{\"source\":\"ui-click\"},\"target\":{\"model_guid\":\"c7ce9de7-9f43-4950-b735-04698c3b9312\"}";
        Assert.Contains("names at least one", Refusal(head + "}"));
        Assert.Contains("whole number", Refusal(head + ",\"shapes\":[\"AWI-a\"],\"delete_with_items\":1.5}"));
        Assert.Contains("1 to 1023", Refusal(head + ",\"shapes\":[\"\"]}"));
        Assert.Contains("items must be", Refusal(head + ",\"items\":[\"not-a-guid\"]}"));
        Assert.Contains("shapes [{", Refusal(head + ",\"shapes\":[{\"name\":\"X\"}]}"));      // a proof object needs its guid
        Assert.Contains("not_ours_confirmed must be", Refusal(head + ",\"shapes\":[\"X\"],\"not_ours_confirmed\":\"yes\"}"));
        // a bare name is accepted: it simply has no proof
        Assert.Empty(Parsed(head + ",\"shapes\":[\"Some Shape 1\"]}").ShapeProof);
        var ok = Parsed(head + ",\"shapes\":[\"AWI-a\"],\"delete_with_items\":2}");
        Assert.Equal(2, ok.DeleteWithItems);
    }

    // ── E6: names ─────────────────────────────────────────────────────────────

    [Theory]
    [InlineData("AWI-0123456789abcdef01234567", true)]
    [InlineData("A", true)]
    [InlineData("Member_1.v2-x", true)]
    [InlineData("", false)]
    [InlineData("1ABC", false)]          // must start with a letter
    [InlineData("-ABC", false)]
    [InlineData("AWI ABC", false)]       // space
    [InlineData("AWI#1", false)]
    [InlineData("AWI/1", false)]
    [InlineData("AWI:1", false)]
    [InlineData("AWI\\1", false)]
    [InlineData("AWI*1", false)]
    [InlineData("AWI\u00e9", false)]     // non-ASCII letter
    [InlineData("AWI\u0000", false)]
    [InlineData("AWI.", false)]          // a trailing dot cannot be a file name on Windows
    public void ShapeName_AllowList(string name, bool allowed)
    {
        Assert.Equal(allowed, ItemShapeNames.Validate(name) == null);
    }

    [Fact]
    public void ShapeName_LengthLimit_IsTheConservativeSixtyFour()
    {
        Assert.Null(ItemShapeNames.Validate(new string('A', ItemShapeNames.MaxLength)));
        Assert.NotNull(ItemShapeNames.Validate(new string('A', ItemShapeNames.MaxLength + 1)));
        // far inside Tekla's documented 1023
        Assert.True(ItemShapeNames.MaxLength < 1023);
        // a prefix is valid only if the whole derived name is
        Assert.Null(ItemShapeNames.ValidatePrefix("AWI"));
        Assert.NotNull(ItemShapeNames.ValidatePrefix(new string('A', ItemShapeNames.MaxPrefixLength + 1)));
        Assert.NotNull(ItemShapeNames.ValidatePrefix("A B"));
        Assert.NotNull(ItemShapeNames.ValidatePrefix(""));
    }

    [Fact]
    public void ShapeName_IsDerivedFromTheDigest_SoALengthIsPartOfTheName()
    {
        var a = Parsed(Request("plan", new[] { Member("A", 1000), Member("B", 1000), Member("C", 1000.5) }));
        var names = a.Members.Select(m => ItemShapeNames.For("AWI", m.SolidDigest)).ToList();
        Assert.Equal(names[0], names[1]);       // same solid, same name
        Assert.NotEqual(names[0], names[2]);    // a different length, a different name
        Assert.All(names, n => Assert.Null(ItemShapeNames.Validate(n)));
        Assert.Equal("AWI-".Length + ItemShapeNames.DigestHexLength, names[0].Length);
        // the section changes the name too
        var hollow = Parsed(Request("plan", new[] { Member("H", 1000, holes: "[[[-44,-44],[-44,44],[44,44],[44,-44]]]") })).Members[0];
        Assert.NotEqual(a.Members[0].SolidDigest, hollow.SolidDigest);
    }

    [Fact]
    public void Digest_DoesNotDependOnHowTheRingsAreWound()
    {
        var ccw = Parsed(Request("plan", new[] { Member("A", 1000, outer: "[[-50,-50],[50,-50],[50,50],[-50,50]]") })).Members[0].SolidDigest;
        var cw = Parsed(Request("plan", new[] { Member("A", 1000, outer: "[[-50,50],[50,50],[50,-50],[-50,-50]]") })).Members[0].SolidDigest;
        // a clockwise outer ring is turned counter-clockwise (its start vertex stays the first one given), so the two
        // rings are one solid only when they start at the same corner: pin that this is the rule, not magic
        Assert.NotNull(cw);
        var again = Parsed(Request("plan", new[] { Member("A", 1000, outer: "[[-50,-50],[50,-50],[50,50],[-50,50]]") })).Members[0].SolidDigest;
        Assert.Equal(ccw, again);
    }

    // ── the brep ──────────────────────────────────────────────────────────────

    [Fact]
    public void Brep_HollowSection_HasTheCapsInnerLoopsWoundOppositeTheOuterLoop()
    {
        var m = Parsed(Request("plan", new[] { Member("H", 1000, holes: "[[[-44,-44],[-44,44],[44,44],[44,-44]]]") })).Members[0];
        var brep = m.Solid.BuildBrep();
        Assert.Equal(16, brep.Vertices.Count);
        Assert.Equal(2 + 8, brep.Faces.Count);   // two caps and a quad per ring edge (spike: 16 vertices, 10 faces)
        Assert.Equal(new[] { 0, 1 }.Length, brep.InnerLoops.Count);
        // the volume of the spike's hollow section: (100*100 - 88*88) * 1000
        Assert.Equal((10000.0 - 7744.0) * 1000, m.Solid.Volume, 6);
        Assert.Equal(new[] { 0.0, -50, -50, 1000, 50, 50 }, m.Solid.Box);
        // cap 0: outer clockwise seen from +X, inner loop reversed relative to it
        var capOuter = brep.Faces[0];
        var capInner = brep.InnerLoops[0][0];
        Assert.Equal(Orientation(brep, capOuter), -Orientation(brep, capInner));
        var cap1Outer = brep.Faces[1];
        Assert.Equal(Orientation(brep, cap1Outer), -Orientation(brep, brep.InnerLoops[1][0]));
        Assert.Equal(-Orientation(brep, capOuter), Orientation(brep, cap1Outer));
    }

    static double Orientation(PrismBrep brep, int[] ring)
    {
        double a = 0;
        for (int i = 0; i < ring.Length; i++)
        {
            var p = brep.Vertices[ring[i]]; var q = brep.Vertices[ring[(i + 1) % ring.Length]];
            a += p[1] * q[2] - q[1] * p[2];
        }
        return Math.Sign(a);
    }

    // ── the fake host ─────────────────────────────────────────────────────────

    enum Fault { None, Before, After }

    sealed class FakeShape
    {
        public string Name = "", Guid = "", GeometryGuid = "", Key = "", Digest = "";
        public PrismBrep Brep = new PrismBrep();
        public string? Label;
    }

    sealed class FakeItem
    {
        public string Guid = "", Name = "", Shape = "";
        public ItemSpec Spec = new ItemSpec();
    }

    sealed class FakeHost : CatalogueFileHost, IItemCreateHost
    {
        public ItemTarget Target;
        public readonly Dictionary<string, FakeShape> Shapes = new Dictionary<string, FakeShape>(StringComparer.Ordinal);
        public readonly Dictionary<string, FakeItem> Items = new Dictionary<string, FakeItem>(StringComparer.OrdinalIgnoreCase);
        public readonly Dictionary<string, double> VolumeByDigest = new Dictionary<string, double>();
        public int Writes;
        public int FailAtWrite = -1;
        public bool FaultAfterEffect;
        public bool RejectBrep;
        public bool FailBackup;
        public bool SkewVolume;
        public bool FailMeasure;
        public bool RejectMetadata;
        public bool HideInstanceCount;
        public bool SneakItemAfterDelete;
        public readonly HashSet<string> NotBrepGuids = new HashSet<string>(StringComparer.OrdinalIgnoreCase);
        public readonly List<string> Log = new List<string>();
        readonly string _modelPath;

        public FakeHost(string modelPath, string backupRoot) : base(modelPath, backupRoot)
        {
            _modelPath = modelPath;
            Directory.CreateDirectory(Path.Combine(modelPath, "Shapes"));
            Directory.CreateDirectory(Path.Combine(modelPath, "ShapeGeometries"));
            Target = new ItemTarget { ModelPath = modelPath, ModelGuid = "c7ce9de7-9f43-4950-b735-04698c3b9312", SharedModel = false, SingleUserModel = true, TeklaVersion = "2026.0" };
        }

        Fault Tick(string what)
        {
            Writes++;
            Log.Add(what);
            if (Writes != FailAtWrite) return Fault.None;
            if (!FaultAfterEffect) throw new IOException("injected fault before " + what);
            return Fault.After;
        }

        static void Finish(Fault f, string what) { if (f == Fault.After) throw new IOException("injected fault after " + what); }

        public ItemTarget ReadTarget() => Target;

        ShapeInfo Info(FakeShape s)
        {
            var i = new ShapeInfo { Name = s.Name, Guid = s.Guid, GeometryGuid = s.GeometryGuid, Digest = s.Digest.Length == 0 ? null : s.Digest, InstanceCount = HideInstanceCount ? (int?)null : Items.Values.Count(x => x.Shape == s.Name) };
            foreach (var v in s.Brep.Vertices) i.Vertices.Add(v.Select(c => (double)(float)c).ToArray());
            i.Extrema = new[] { i.Vertices.Min(v => v[0]), i.Vertices.Min(v => v[1]), i.Vertices.Min(v => v[2]), i.Vertices.Max(v => v[0]), i.Vertices.Max(v => v[1]), i.Vertices.Max(v => v[2]) };
            return i;
        }

        public ShapeInfo? FindShape(string name) => Shapes.Values.Where(s => string.Equals(s.Name, name, StringComparison.OrdinalIgnoreCase)).Select(Info).FirstOrDefault();
        public ShapeInfo? FindShapeByGuid(string guid) => Shapes.Values.Where(s => s.Guid == guid).Select(Info).FirstOrDefault();
        public IReadOnlyList<string> ValidateBrep(PrismBrep brep) => RejectBrep ? new[] { "face 0: LoopNormalsInvalid" } : new string[0];

        static string Key(PrismBrep b) => string.Join(";", b.Vertices.Select(v => string.Join(",", v))) + "|" + string.Join(";", b.Faces.Select(f => string.Join(",", f)));

        public ShapeInsertResult InsertShape(string name, PrismBrep brep)
        {
            var fault = Tick("insert shape " + name);
            var result = new ShapeInsertResult();
            var key = Key(brep);
            var same = Shapes.Values.FirstOrDefault(s => s.Key == key);
            if (same != null) { result.MatchingGuids.Add(same.Guid); Finish(fault, "insert shape"); return result; }
            if (Shapes.ContainsKey(name)) { result.Refusal = "a shape of that name exists"; Finish(fault, "insert shape"); return result; }
            var shape = new FakeShape { Name = name, Guid = Guid.NewGuid().ToString(), GeometryGuid = Guid.NewGuid().ToString(), Key = key, Brep = brep };
            Shapes[name] = shape;
            File.WriteAllText(Path.Combine(_modelPath, "Shapes", shape.Guid + ".xml"), "<ImportPart name=\"" + name + "\"/>");
            File.WriteAllText(Path.Combine(_modelPath, "ShapeGeometries", shape.GeometryGuid + ".tez"), key);
            result.Inserted = true;
            Finish(fault, "insert shape");
            return result;
        }

        public bool SetShapeMetadata(string name, string digest, string label)
        {
            var fault = Tick("metadata " + name);
            if (RejectMetadata) { Finish(fault, "metadata"); return false; }
            Shapes[name].Digest = digest; Shapes[name].Label = label;
            // like Tekla, the metadata is written into the shape's record, so the file's bytes change
            File.WriteAllText(Path.Combine(_modelPath, "Shapes", Shapes[name].Guid + ".xml"), "<ImportPart name=\"" + name + "\"><StructuredMetadata>" + digest + "</StructuredMetadata></ImportPart>");
            Finish(fault, "metadata");
            return true;
        }

        public string? InsertItem(ItemSpec spec)
        {
            var fault = Tick("insert item " + spec.MemberId);
            var item = new FakeItem { Guid = Guid.NewGuid().ToString(), Name = spec.Name, Shape = spec.ShapeName, Spec = spec };
            Items[item.Guid] = item;
            spec.PlacedGuid = item.Guid;      // Tekla has it; whatever fails next, the engine is told
            Finish(fault, "insert item");
            return item.Guid;
        }

        public ItemReadback MeasureItem(ItemSpec spec, string guid)
        {
            if (FailMeasure) throw new IOException("injected fault while reading the Item back");
            return Readback(Items[guid]);
        }

        ItemReadback Readback(FakeItem item)
        {
            var spec = item.Spec;
            var length = Math.Sqrt(Math.Pow(spec.End[0] - spec.Start[0], 2) + Math.Pow(spec.End[1] - spec.Start[1], 2) + Math.Pow(spec.End[2] - spec.Start[2], 2));
            return new ItemReadback
            {
                Exists = true, Guid = item.Guid, Name = item.Name, ShapeName = item.Shape,
                VolumeFromFaces = VolumeByDigest[spec.Digest] * (SkewVolume ? 1.01 : 1.0), OriginToStartMm = 0, AxisXDot = 1, ExtentAlongAxisX = length,
                MaxSolidToShapeMm = 0, MaxShapeToSolidMm = 0, ShapeInstanceCount = Items.Values.Count(x => x.Shape == item.Shape), UdaSet = true,
            };
        }

        public ItemReadback ReadItem(string guid) => Items.TryGetValue(guid, out var i) ? new ItemReadback { Exists = true, Guid = i.Guid, Name = i.Name, ShapeName = Shapes.ContainsKey(i.Shape) ? i.Shape : "" } : new ItemReadback { Exists = false, Guid = guid, NotBrep = NotBrepGuids.Contains(guid) };

        public IReadOnlyList<ItemRef> ItemsUsingShape(string name) => Items.Values.Where(i => i.Shape == name).Select(i => new ItemRef { Guid = i.Guid, Name = i.Name }).ToList();

        public bool DeleteItem(string guid)
        {
            var fault = Tick("delete item " + guid);
            var shapeOfItem = Items.TryGetValue(guid, out var gone) ? gone.Shape : null;
            var ok = Items.Remove(guid);
            if (SneakItemAfterDelete && ok && shapeOfItem != null)
            {
                // somebody places a new Item on the shape between the guard's first read and the shape delete
                var sneaky = new FakeItem { Guid = Guid.NewGuid().ToString(), Name = "sneaky", Shape = shapeOfItem, Spec = new ItemSpec() };
                Items[sneaky.Guid] = sneaky;
            }
            Finish(fault, "delete item");
            return ok;
        }

        public bool DeleteShape(string name)
        {
            var fault = Tick("delete shape " + name);
            // like Tekla 2026.0: the shape goes even while Items use it
            var s = Shapes[name];
            Shapes.Remove(name);
            File.Delete(Path.Combine(_modelPath, "Shapes", s.Guid + ".xml"));
            File.Delete(Path.Combine(_modelPath, "ShapeGeometries", s.GeometryGuid + ".tez"));
            Finish(fault, "delete shape");
            return true;
        }

        public new string BackupCatalogue(string runId)
        {
            if (FailBackup) throw new IOException("disk full");
            return base.BackupCatalogue(runId);
        }
    }

    FakeHost NewHost(out string backupRoot)
    {
        var model = Path.Combine(_root, "model-" + Guid.NewGuid().ToString("N").Substring(0, 6));
        Directory.CreateDirectory(model);
        backupRoot = Path.Combine(_root, "backups");
        return new FakeHost(model, backupRoot);
    }

    static Dictionary<string, object?> Run(FakeHost host, string json)
    {
        var request = Parsed(json);
        foreach (var m in request.Members) host.VolumeByDigest[m.SolidDigest] = m.Solid.Volume;
        return TeklaItemCreate.Run(request, host, "run-1");
    }

    static JsonNode Node(Dictionary<string, object?> receipt) => Program.SerializeResult(receipt)!;

    // ── plan, create, reuse ───────────────────────────────────────────────────

    [Fact]
    public void Plan_WritesNothing_AndCountsOneShapePerDistinctLength()
    {
        var host = NewHost(out _);
        var r = Node(Run(host, Request("plan", new[] { Member("A", 1000), Member("B", 1000), Member("C", 2400) })));
        Assert.Equal("planned", (string?)r["status"]);
        Assert.True((bool)r["ok"]!);
        Assert.Equal(2, (int)r["new_shape_count"]!);
        Assert.Equal(0, host.Writes);
        Assert.Empty(host.Shapes);
        Assert.Empty(host.ListCatalogueFiles());
        Assert.Equal(3, ((JsonArray)r["items"]!).Count);
    }

    [Fact]
    public void Create_AddsOneShapePerLength_PlacesEveryItem_AndReportsTheTarget()
    {
        var host = NewHost(out _);
        host.Target.SharedModel = true;
        var r = Node(Run(host, Request("create", new[] { Member("A", 1000), Member("B", 1000), Member("C", 2400) })));
        Assert.Equal("created", (string?)r["status"]);
        Assert.True((bool)r["ok"]!);
        Assert.Equal(2, host.Shapes.Count);
        Assert.Equal(3, host.Items.Count);
        Assert.Equal(2, (int)r["new_shape_count"]!);
        // one Item per member, on the shape of its length
        var items = ((JsonArray)r["items"]!).Select(i => (JsonObject)i!).ToList();
        Assert.Equal(3, items.Count);
        Assert.Equal((string?)items[0]["shape"], (string?)items[1]["shape"]);
        Assert.NotEqual((string?)items[0]["shape"], (string?)items[2]["shape"]);
        Assert.All(items, i => Assert.Empty((JsonArray)i["verification"]!["problems"]!));
        Assert.Equal(1, (int?)items[0]["verification"]!["shape_instance_count"]);   // counted as the first Item was placed
        Assert.Equal(2, (int?)items[1]["verification"]!["shape_instance_count"]);
        // shared model is reported, and named in the note
        var target = (JsonObject)r["target"]!;
        Assert.True((bool)target["shared_model"]!);
        Assert.Contains("everyone", (string?)target["shared_note"]);
        Assert.Equal("model", (string?)target["write_level"]);
        // the shapes carry their digest, and two files each exist on disk
        Assert.All(host.Shapes.Values, s => Assert.StartsWith("sha256:", s.Digest));
        Assert.Equal(4, host.ListCatalogueFiles().Count);
        // a backup of the catalogue was taken before the first write
        Assert.True(Directory.Exists((string?)r["backup"]!["folder"]));
        // the undo names exactly the three Items and the two shapes
        var undo = (JsonObject)r["undo"]!["request"]!;
        Assert.Equal(3, ((JsonArray)undo["items"]!).Count);
        Assert.Equal(2, ((JsonArray)undo["shapes"]!).Count);
    }

    [Fact]
    public void Create_Twice_ReusesTheIdenticalShape_ButPlacesTheItemsAgain()
    {
        var host = NewHost(out _);
        Run(host, Request("create", new[] { Member("A", 1000) }));
        var writesAfterFirst = host.Writes;
        var r = Node(Run(host, Request("create", new[] { Member("A2", 1000) })));
        Assert.Equal("created", (string?)r["status"]);
        Assert.Equal(0, (int)r["new_shape_count"]!);
        Assert.Equal("reused-identical", (string?)r["shapes"]![0]!["status"]);
        Assert.Single(host.Shapes);
        Assert.Equal(2, host.Items.Count);
        Assert.Equal(1, host.Writes - writesAfterFirst);   // only the Item was written
        // the undo does not list a shape this run did not add
        Assert.Empty((JsonArray)r["undo"]!["request"]!["shapes"]!);
    }

    [Fact]
    public void ADifferentShapeUnderTheDerivedName_IsAConflict_AndNothingIsWritten()
    {
        var host = NewHost(out _);
        var request = Parsed(Request("create", new[] { Member("A", 1000) }));
        var name = ItemShapeNames.For("AWI", request.Members[0].SolidDigest);
        var other = Parsed(Request("plan", new[] { Member("X", 777) })).Members[0].Solid.BuildBrep();
        host.InsertShape(name, other);
        host.SetShapeMetadata(name, "sha256:somebody-elses", "x");
        var writes = host.Writes;
        var r = Node(Run(host, Request("create", new[] { Member("A", 1000) })));
        Assert.Equal("conflict", (string?)r["status"]);
        Assert.False((bool)r["ok"]!);
        Assert.Equal(writes, host.Writes);
        Assert.Empty(host.Items);
        Assert.Single((JsonArray)r["conflicts"]!);
    }

    [Fact]
    public void ANameThatDiffersOnlyByCase_IsAConflict_BecauseTeklaMayTreatItAsTheSameShape()
    {
        var host = NewHost(out _);
        var name = ItemShapeNames.For("AWI", Parsed(Request("create", new[] { Member("A", 1000) })).Members[0].SolidDigest);
        host.InsertShape("awi" + name.Substring(3), Parsed(Request("plan", new[] { Member("X", 777) })).Members[0].Solid.BuildBrep());
        var writes = host.Writes;
        var r = Node(Run(host, Request("create", new[] { Member("A", 1000) })));
        Assert.Equal("conflict", (string?)r["status"]);
        Assert.Contains("case-insensitively", (string?)r["conflicts"]![0]!["why"]);
        Assert.Equal(writes, host.Writes);
    }

    [Fact]
    public void SameGeometryUnderAnotherName_IsReusedByGuid_AndReported()
    {
        var host = NewHost(out _);
        var member = Parsed(Request("create", new[] { Member("A", 1000) })).Members[0];
        host.InsertShape("MyOwnShape", member.Solid.BuildBrep());
        host.SetShapeMetadata("MyOwnShape", "sha256:not-ours", "theirs");
        var r = Node(Run(host, Request("create", new[] { Member("A", 1000) })));
        Assert.Equal("created", (string?)r["status"]);
        var shape = (JsonObject)r["shapes"]![0]!;
        Assert.Equal("reused-by-geometry", (string?)shape["status"]);
        Assert.Equal("MyOwnShape", (string?)shape["name"]);
        Assert.Equal("MyOwnShape", host.Items.Values.Single().Shape);
        Assert.Single(host.Shapes);
        Assert.Empty((JsonArray)r["undo"]!["request"]!["shapes"]!);   // it is not ours to delete
    }

    [Fact]
    public void ABrepTeklaCallsInvalid_IsRefusedBeforeAnyWrite()
    {
        var host = NewHost(out _);
        host.RejectBrep = true;
        var r = Node(Run(host, Request("create", new[] { Member("A", 1000) })));
        Assert.Equal("invalid-solid", (string?)r["status"]);
        Assert.Equal(0, host.Writes);
        Assert.Empty(host.Shapes);
    }

    [Fact]
    public void ADifferentModel_IsTargetChanged_AndNothingIsWritten()
    {
        var host = NewHost(out _);
        var shared = Request("create", new[] { Member("A", 1000) }, ",\"target\":{\"model_path\":\"C:\\\\Other\\\\Model\",\"shared_model\":false}");
        var r = Node(Run(host, shared));
        Assert.Equal("target-changed", (string?)r["status"]);
        Assert.Equal(0, host.Writes);
        var sharedFlag = Request("create", new[] { Member("A", 1000) }, ",\"target\":{\"model_guid\":\"c7ce9de7-9f43-4950-b735-04698c3b9312\",\"shared_model\":true}");
        Assert.Equal("target-changed", (string?)Node(Run(host, sharedFlag))["status"]);
        var guid = Request("create", new[] { Member("A", 1000) }, ",\"target\":{\"model_guid\":\"00000000-0000-0000-0000-000000000000\"}");
        Assert.Equal("target-changed", (string?)Node(Run(host, guid))["status"]);
        Assert.Equal(0, host.Writes);
        // and the right target goes through
        var right = Request("create", new[] { Member("A", 1000) }, ",\"target\":{\"model_path\":\"" + host.Target.ModelPath.Replace("\\", "\\\\") + "\",\"model_guid\":\"C7CE9DE7-9F43-4950-B735-04698C3B9312\",\"shared_model\":false,\"tekla_version\":\"2026.0\"}");
        Assert.Equal("created", (string?)Node(Run(host, right))["status"]);
    }

    [Fact]
    public void ABackupThatCannotBeTaken_StopsBeforeTheFirstWrite()
    {
        var host = NewHost(out _);
        host.FailBackup = true;
        var r = Node(Run(host, Request("create", new[] { Member("A", 1000) })));
        Assert.Equal("backup-failed", (string?)r["status"]);
        Assert.Equal(0, host.Writes);
    }

    [Fact]
    public void AnItemThatDoesNotReadBackAsAsked_StaysAndIsReportedWithBothSides()
    {
        var host = NewHost(out _);
        host.SkewVolume = true;
        var r = Node(Run(host, Request("create", new[] { Member("A", 1000) })));
        Assert.Equal("read-back-mismatch", (string?)r["status"]);
        Assert.False((bool)r["ok"]!);
        Assert.Single(host.Items);                       // not rolled back: the person decides
        var item = (JsonObject)r["items"]![0]!;
        Assert.Equal("read-back-mismatch", (string?)item["status"]);
        Assert.True(Math.Abs((double)item["verification"]!["volume_ratio"]! - 1.01) < 1e-9);
        Assert.NotNull(r["undo"]);
    }

    [Fact]
    public void RollIsAppliedAndFlaggedAsNotYetProvedLive()
    {
        var host = NewHost(out _);
        var r = Node(Run(host, Request("plan", new[] { Member("A", 1000).Replace("\"end\"", "\"roll_deg\":90,\"end\"") })));
        Assert.Contains("roll_deg", (string?)((JsonArray)r["warnings"]!)[0]);
    }

    // ── E9: a failure partway ─────────────────────────────────────────────────

    [Fact]
    public void E9_AFaultAfterTheFirstShapeWrite_LeavesAReceiptNamingExactlyWhatWasWritten()
    {
        var host = NewHost(out _);
        // write 1 = insert shape (length 1000), write 2 = its metadata, write 3 = insert shape (2400): fault there
        host.FailAtWrite = 3;
        var r = Node(Run(host, Request("create", new[] { Member("A", 1000), Member("B", 2400) })));
        Assert.Equal("partial", (string?)r["status"]);
        Assert.False((bool)r["ok"]!);
        Assert.Contains("injected fault", (string?)r["error"]);
        // exactly the first shape exists, with the guid and the two files Tekla wrote for it
        Assert.Single(host.Shapes);
        var shape = host.Shapes.Values.Single();
        var ledger = ((JsonArray)r["ledger"]!).Select(x => (JsonObject)x!).ToList();
        var entry = Assert.Single(ledger);
        Assert.Equal("shape", (string?)entry["kind"]);
        Assert.Equal(shape.Name, (string?)entry["name"]);
        Assert.Equal(shape.Guid, (string?)entry["guid"]);
        Assert.Equal(shape.GeometryGuid, (string?)entry["geometry_guid"]);
        var files = ((JsonArray)entry["files"]!).Select(f => (string?)f!["path"]).ToList();
        Assert.Equal(2, files.Count);
        Assert.Contains(files, f => f!.EndsWith(shape.Guid + ".xml", StringComparison.OrdinalIgnoreCase));
        Assert.Contains(files, f => f!.EndsWith(shape.GeometryGuid + ".tez", StringComparison.OrdinalIgnoreCase));
        Assert.All(files, f => Assert.True(File.Exists(f)));
        // exists_afterwards is a read-back, not intention
        var after = ((JsonArray)r["exists_afterwards"]!).Select(x => (JsonObject)x!).Single();
        Assert.True((bool)after["exists"]!);
        Assert.Equal(shape.Guid, (string?)after["guid"]);
        // no Item was placed, and the receipt does not list one
        Assert.Empty(host.Items);
        Assert.Empty((JsonArray)r["unledgered_files"]!);
        Assert.NotNull(r["backup"]);
        // and the exact undo names that one shape and nothing else
        var undo = (JsonObject)r["undo"]!["request"]!;
        Assert.Empty((JsonArray)undo["items"]!);
        var undoShape = (JsonObject)((JsonArray)undo["shapes"]!).Single()!;
        Assert.Equal(shape.Name, (string?)undoShape["name"]);
        Assert.Equal(shape.Guid, (string?)undoShape["guid"]);
        Assert.Equal(shape.GeometryGuid, (string?)undoShape["geometry_guid"]);
    }

    [Fact]
    public void E9_TheUndoInTheReceipt_RemovesExactlyWhatWasWritten_AndNothingElse()
    {
        var host = NewHost(out _);
        // something that was already there and must survive
        var survivor = Parsed(Request("plan", new[] { Member("S", 555) })).Members[0];
        host.InsertShape("PreExisting", survivor.Solid.BuildBrep());
        host.SetShapeMetadata("PreExisting", survivor.SolidDigest, "x");
        var filesBefore = host.ListCatalogueFiles().Keys.ToList();

        host.FailAtWrite = host.Writes + 5;   // shape A, meta A, shape B, meta B, item A | fault at the 5th write: item A
        var r = Node(Run(host, Request("create", new[] { Member("A", 1000), Member("B", 2400) })));
        Assert.Equal("partial", (string?)r["status"]);
        Assert.Equal(3, host.Shapes.Count);   // pre-existing plus two new
        Assert.Empty(host.Items);

        // run the undo exactly as the receipt gives it
        host.FailAtWrite = -1;
        var undo = JsonNode.Parse(((JsonObject)r["undo"]!["request"]!).ToJsonString())!;
        var undoResult = Node(TeklaItemCreate.Run(Parsed(undo.ToJsonString()), host, "undo-1"));
        Assert.Equal("deleted", (string?)undoResult["status"]);
        Assert.Equal(new[] { "PreExisting" }, host.Shapes.Keys.ToArray());
        Assert.Equal(filesBefore.OrderBy(x => x), host.ListCatalogueFiles().Keys.OrderBy(x => x));
    }

    [Fact]
    public void E9_AFaultAfterAShapeAndItsFirstItem_NamesTheItemToo_AndTheUndoDeletesItBeforeTheShape()
    {
        var host = NewHost(out _);
        host.FailAtWrite = 4;      // shape, metadata, item A (3), item B throws (4)
        var r = Node(Run(host, Request("create", new[] { Member("A", 1000), Member("B", 1000) })));
        Assert.Equal("partial", (string?)r["status"]);
        var ledger = ((JsonArray)r["ledger"]!).Select(x => (JsonObject)x!).ToList();
        Assert.Equal(new[] { "shape", "item" }, ledger.Select(l => (string?)l["kind"]).ToArray());
        var itemGuid = host.Items.Keys.Single();
        Assert.Equal(itemGuid, (string?)ledger[1]["guid"]);
        Assert.Equal("A", (string?)ledger[1]["member_id"]);
        var undo = (JsonObject)r["undo"]!["request"]!;
        Assert.Equal(itemGuid, (string?)((JsonArray)undo["items"]!).Single()!["guid"]);
        Assert.Equal(host.Shapes.Keys.Single(), (string?)((JsonArray)undo["shapes"]!).Single()!["name"]);
        // the undo deletes the Item first, so the shape is no longer in use and is deleted without an acknowledgement
        host.FailAtWrite = -1;
        host.Log.Clear();
        var u = Node(TeklaItemCreate.Run(Parsed(undo.ToJsonString()), host, "undo"));
        Assert.Equal("deleted", (string?)u["status"]);
        Assert.StartsWith("delete item", host.Log[0]);
        Assert.StartsWith("delete shape", host.Log[1]);
        Assert.Empty(host.Shapes);
        Assert.Empty(host.Items);
    }

    [Fact]
    public void E9_AWriteThatThrowsAfterTekla_HasAlreadyWritten_IsStillAccountedFor()
    {
        var host = NewHost(out _);
        host.FailAtWrite = 1;
        host.FaultAfterEffect = true;     // the shape files are written, then the call throws
        var r = Node(Run(host, Request("create", new[] { Member("A", 1000) })));
        Assert.Equal("partial", (string?)r["status"]);
        Assert.Single(host.Shapes);
        // Tekla never said it wrote, but the name was free before the call and is taken now: the shape is the run's, so it is
        // in the ledger (with its two files) and in the undo, not a stray
        var entry = (JsonObject)((JsonArray)r["ledger"]!).Single()!;
        Assert.Equal(host.Shapes.Keys.Single(), (string?)entry["name"]);
        Assert.Equal(2, ((JsonArray)entry["files"]!).Count);
        Assert.Empty((JsonArray)r["unledgered_files"]!);
        Assert.Equal(host.Shapes.Keys.Single(), (string?)((JsonArray)r["undo"]!["request"]!["shapes"]!).Single()!["name"]);
    }

    [Fact]
    public void E9_AnItemTeklaAcceptedThenFailedToReadBack_IsStillInTheLedgerAndTheUndo()
    {
        var host = NewHost(out _);
        host.FailMeasure = true;
        var r = Node(Run(host, Request("create", new[] { Member("A", 1000) })));
        Assert.Equal("partial", (string?)r["status"]);
        Assert.Single(host.Items);
        var kinds = ((JsonArray)r["ledger"]!).Select(l => (string?)l!["kind"]).ToArray();
        Assert.Equal(new[] { "shape", "item" }, kinds);
        Assert.Equal(host.Items.Keys.Single(), (string?)((JsonArray)r["undo"]!["request"]!["items"]!).Single()!["guid"]);
    }

    [Fact]
    public void E9_AnItemInsertThatThrowsAfterTeklaAcceptedIt_IsStillInTheLedgerAndTheUndo()
    {
        var host = NewHost(out _);
        host.FailAtWrite = 3;               // shape, metadata, then the Item: placed, then the call throws (a failed commit)
        host.FaultAfterEffect = true;
        var r = Node(Run(host, Request("create", new[] { Member("A", 1000) })));
        Assert.Equal("partial", (string?)r["status"]);
        Assert.Single(host.Items);
        var ledger = ((JsonArray)r["ledger"]!).Select(l => (JsonObject)l!).ToList();
        Assert.Equal(new[] { "shape", "item" }, ledger.Select(l => (string?)l["kind"]).ToArray());
        Assert.Equal(host.Items.Keys.Single(), (string?)ledger[1]["guid"]);
        Assert.Equal(host.Items.Keys.Single(), (string?)((JsonArray)r["undo"]!["request"]!["items"]!).Single()!["guid"]);
    }

    [Fact]
    public void Ledger_HashesDescribeTheFilesAsTheyAreLeftOnDisk_NotBeforeTheMetadataWasWritten()
    {
        var host = NewHost(out _);
        var r = Node(Run(host, Request("create", new[] { Member("A", 1000) })));
        var files = ((JsonArray)r["ledger"]![0]!["files"]!).Select(f => (JsonObject)f!).ToList();
        Assert.Equal(2, files.Count);
        Assert.All(files, f => Assert.Equal(CatalogueFileHost.Sha256((string)f["path"]!), (string?)f["sha256_after"]));
        // and the same on a partial result
        var host2 = NewHost(out _);
        host2.FailAtWrite = 3;
        var partial = Node(Run(host2, Request("create", new[] { Member("A", 1000), Member("B", 2400) })));
        var pf = ((JsonArray)partial["ledger"]![0]!["files"]!).Select(f => (JsonObject)f!).ToList();
        Assert.All(pf, f => Assert.Equal(CatalogueFileHost.Sha256((string)f["path"]!), (string?)f["sha256_after"]));
    }

    [Fact]
    public void TheUndoIsBoundToTheModelItWasWrittenTo_AndRefusesAnotherOpenModel()
    {
        var host = NewHost(out _);
        var r = Node(Run(host, Request("create", new[] { Member("A", 1000) })));
        var undo = (JsonObject)r["undo"]!["request"]!;
        Assert.Equal(host.Target.ModelPath, (string?)undo["target"]!["model_path"]);
        Assert.Equal(host.Target.ModelGuid, (string?)undo["target"]!["model_guid"]);
        // the person opens another model that happens to hold the same shape name: the undo must not touch it
        var other = NewHost(out _);
        other.Target.ModelGuid = "11111111-1111-1111-1111-111111111111";
        other.InsertShape(host.Shapes.Keys.Single(), host.Shapes.Values.Single().Brep);
        var writes = other.Writes;
        var refused = Node(TeklaItemCreate.Run(Parsed(undo.ToJsonString()), other, "undo-elsewhere"));
        Assert.Equal("target-changed", (string?)refused["status"]);
        Assert.Equal(writes, other.Writes);
        Assert.Single(other.Shapes);
        // against the right model it runs
        Assert.Equal("deleted", (string?)Node(TeklaItemCreate.Run(Parsed(undo.ToJsonString()), host, "undo"))["status"]);
    }

    [Fact]
    public void E9_AFaultBeforeAnyWrite_IsFailed_NotPartial_AndHasNoUndo()
    {
        var host = NewHost(out _);
        host.FailAtWrite = 1;
        var r = Node(Run(host, Request("create", new[] { Member("A", 1000) })));
        Assert.Equal("failed", (string?)r["status"]);
        Assert.Empty(host.Shapes);
        Assert.Null(r["undo"]);
        Assert.Empty((JsonArray)r["ledger"]!);
    }

    // ── delete: never while Items use the shape, unless the count is acknowledged ──

    FakeHost WithTwoItemsOnOneShape(out string shapeName)
    {
        var host = NewHost(out _);
        var r = Node(Run(host, Request("create", new[] { Member("A", 1000), Member("B", 1000) })));
        Assert.Equal("created", (string?)r["status"]);
        shapeName = host.Shapes.Keys.Single();
        return host;
    }

    /// <summary>A delete request carrying the ledger proof a receipt gives, built from what the host holds now.</summary>
    static string Delete(FakeHost host, string shape, string? tail = null, string? items = null)
    {
        var shapeJson = host.Shapes.TryGetValue(shape, out var held)
            ? $"{{\"name\":\"{shape}\",\"guid\":\"{held.Guid}\",\"geometry_guid\":\"{held.GeometryGuid}\"}}"
            : $"{{\"name\":\"{shape}\",\"guid\":\"00000000-0000-0000-0000-000000000000\"}}";
        string itemsJson = "";
        if (items != null)
        {
            var guids = ((JsonArray)JsonNode.Parse(items)!).Select(g => (string)g!).ToList();
            itemsJson = ",\"items\":[" + string.Join(",", guids.Select(g => host.Items.TryGetValue(g, out var i) ? $"{{\"guid\":\"{g}\",\"name\":\"{i.Name}\"}}" : $"{{\"guid\":\"{g}\",\"name\":\"gone\"}}")) + "]";
        }
        return "{\"mode\":\"delete\",\"consent\":{\"source\":\"ui-click\"},\"target\":{\"model_guid\":\"c7ce9de7-9f43-4950-b735-04698c3b9312\"},\"shapes\":[" + shapeJson + "]" + itemsJson + (tail ?? "") + "}";
    }

    [Fact]
    public void Delete_IsRefusedWhileItemsUseTheShape_AndNothingIsDeleted()
    {
        var host = WithTwoItemsOnOneShape(out var shape);
        var writes = host.Writes;
        var r = Node(Run(host, Delete(host, shape)));
        Assert.Equal("shape-in-use", (string?)r["status"]);
        Assert.False((bool)r["ok"]!);
        Assert.Equal(2, (int)r["instance_count_remaining"]!);
        Assert.Equal(2, ((JsonArray)r["items_using_shapes"]!).Count);
        Assert.Contains("delete_with_items = 2", (string?)r["message"]);
        Assert.Equal(writes, host.Writes);
        Assert.Single(host.Shapes);
        Assert.Equal(2, host.Items.Count);
    }

    [Theory]
    [InlineData(0)]
    [InlineData(1)]
    [InlineData(3)]
    public void Delete_WithTheWrongAcknowledgement_IsRefusedAndNamesTheCurrentCount(int wrong)
    {
        var host = WithTwoItemsOnOneShape(out var shape);
        var writes = host.Writes;
        var r = Node(Run(host, Delete(host, shape, ",\"delete_with_items\":" + wrong)));
        Assert.Equal("delete-count-mismatch", (string?)r["status"]);
        Assert.Contains("2 Item(s)", (string?)r["message"]);
        Assert.Equal(writes, host.Writes);
        Assert.Single(host.Shapes);
    }

    [Fact]
    public void Delete_WithTheExactAcknowledgement_Deletes_AndReportsTheItemsAffected()
    {
        var host = WithTwoItemsOnOneShape(out var shape);
        var r = Node(Run(host, Delete(host, shape, ",\"delete_with_items\":2")));
        Assert.Equal("deleted", (string?)r["status"]);
        Assert.True((bool)r["ok"]!);
        Assert.Empty(host.Shapes);
        Assert.Equal(2, host.Items.Count);   // the Items stay, without their shape
        Assert.Equal(2, ((JsonArray)r["items_using_shapes"]!).Count);
        Assert.Contains("no shape", string.Join(" ", ((JsonArray)r["warnings"]!).Select(w => (string?)w)));
        Assert.Equal("deleted", (string?)r["deleted_shapes"]![0]!["status"]);
        Assert.Equal(2, ((JsonArray)r["deleted_shapes"]![0]!["files_removed"]!).Count);
    }

    [Fact]
    public void Delete_AcknowledgementCountsOnlyItemsNotListedForDeletion()
    {
        var host = WithTwoItemsOnOneShape(out var shape);
        var guids = host.Items.Keys.ToList();
        // delete both listed Items with the shape: nothing is left using it, so no acknowledgement is needed
        var r = Node(Run(host, Delete(host, shape, null, "[\"" + guids[0] + "\",\"" + guids[1] + "\"]")));
        Assert.Equal("deleted", (string?)r["status"]);
        Assert.Empty(host.Items);
        Assert.Empty(host.Shapes);
        // one listed, one left: the left one needs the acknowledgement
        var host2 = WithTwoItemsOnOneShape(out var shape2);
        var one = host2.Items.Keys.First();
        var refused = Node(Run(host2, Delete(host2, shape2, null, "[\"" + one + "\"]")));
        Assert.Equal("shape-in-use", (string?)refused["status"]);
        Assert.Equal(2, host2.Items.Count);
        var accepted = Node(Run(host2, Delete(host2, shape2, ",\"delete_with_items\":1", "[\"" + one + "\"]")));
        Assert.Equal("deleted", (string?)accepted["status"]);
        Assert.Single(host2.Items);
    }

    [Fact]
    public void Delete_AnUnusedShapeNeedsNoAcknowledgement_ButAnUnneededOneIsRefused()
    {
        var host = NewHost(out _);
        Run(host, Request("create", new[] { Member("A", 1000) }));
        var item = host.Items.Keys.Single();
        var shape = host.Shapes.Keys.Single();
        Assert.Equal("deleted", (string?)Node(Run(host, Delete(host, shape, null, "[\"" + item + "\"]")))["status"]);
        var host2 = NewHost(out _);
        Run(host2, Request("create", new[] { Member("A", 1000) }));
        var unused = host2.Shapes.Keys.Single();
        host2.Items.Clear();
        Assert.Equal("delete-count-mismatch", (string?)Node(Run(host2, Delete(host2, unused, ",\"delete_with_items\":3")))["status"]);
        Assert.Equal("deleted", (string?)Node(Run(host2, Delete(host2, unused)))["status"]);
    }

    [Fact]
    public void Delete_OfAnAbsentShape_IsReportedAbsent()
    {
        var host = NewHost(out _);
        var r = Node(Run(host, Delete(host, "AWI-nothere")));
        Assert.Equal("deleted", (string?)r["status"]);
        Assert.Equal("absent", (string?)r["shapes"]![0]!["status"]);
        Assert.Equal(0, host.Writes);
    }

    [Fact]
    public void Delete_TakesABackupBeforeItWrites()
    {
        var host = WithTwoItemsOnOneShape(out var shape);
        var r = Node(Run(host, Delete(host, shape, ",\"delete_with_items\":2")));
        var folder = (string?)r["backup"]!["folder"]!;
        Assert.Equal(2, Directory.GetFiles(folder, "*", SearchOption.AllDirectories).Length);
    }

    // ── only what the verb can show it made; the guard at the write ───────────

    static string Bare(string? shape, string? item, string? tail = null) =>
        "{\"mode\":\"delete\",\"consent\":{\"source\":\"ui-click\"},\"target\":{\"model_guid\":\"c7ce9de7-9f43-4950-b735-04698c3b9312\"}"
        + (shape == null ? "" : ",\"shapes\":[\"" + shape + "\"]") + (item == null ? "" : ",\"items\":[\"" + item + "\"]") + (tail ?? "") + "}";

    [Fact]
    public void Delete_WithoutLedgerProof_IsATypedAnswerNamingWhatIsUnproved_AndDeletesNothing()
    {
        var host = WithTwoItemsOnOneShape(out var shape);
        host.InsertShape("Hand Made Shape", Parsed(Request("plan", new[] { Member("S", 555) })).Members[0].Solid.BuildBrep());
        var writes = host.Writes;
        var item = host.Items.Keys.First();
        var r = Node(Run(host, Bare("Hand Made Shape", item)));
        Assert.Equal("not-created-by-this-run", (string?)r["status"]);
        Assert.False((bool)r["ok"]!);
        Assert.Equal(writes, host.Writes);
        Assert.Equal(2, host.Shapes.Count);
        Assert.Equal(2, host.Items.Count);
        var named = ((JsonArray)r["not_created_by_this_run"]!).Select(x => (JsonObject)x!).ToList();
        var shapeEntry = named.Single(n => (string?)n["kind"] == "shape");
        Assert.Equal("Hand Made Shape", (string?)shapeEntry["name"]);
        Assert.Equal(host.Shapes["Hand Made Shape"].Guid, (string?)shapeEntry["guid"]);
        Assert.Equal("missing", (string?)shapeEntry["reason"]);
        Assert.Equal("missing", (string?)named.Single(n => (string?)n["kind"] == "item")["reason"]);
        Assert.Contains("--not-ours-confirmed", (string?)r["message"]);
        Assert.Null(r["override"]);
    }

    [Fact]
    public void Delete_WithAGuidThatIsNotTheOneTheRunRecorded_SaysGuidMismatch_AndAnItemWithAnotherNameToo()
    {
        var host = WithTwoItemsOnOneShape(out var shape);
        var item = host.Items.Values.First();
        var request = Delete(host, shape, null, "[\"" + item.Guid + "\"]")
            .Replace(host.Shapes[shape].Guid, "11111111-1111-1111-1111-111111111111")
            .Replace("\"name\":\"" + item.Name + "\"", "\"name\":\"somebody else's\"");
        var r = Node(Run(host, request));
        Assert.Equal("not-created-by-this-run", (string?)r["status"]);
        var reasons = ((JsonArray)r["not_created_by_this_run"]!).Select(x => (string?)x!["reason"]).OrderBy(x => x).ToArray();
        Assert.Equal(new[] { "guid-mismatch", "name-mismatch" }, reasons);
        Assert.Equal(2, host.Items.Count);
        Assert.Single(host.Shapes);
    }

    [Fact]
    public void Delete_WithTheOverride_DeletesWhatIsNotVouchedFor_AndTheReceiptRecordsIt()
    {
        var host = NewHost(out _);
        host.InsertShape("Hand Made Shape", Parsed(Request("plan", new[] { Member("S", 555) })).Members[0].Solid.BuildBrep());
        var guid = host.Shapes["Hand Made Shape"].Guid;
        var r = Node(Run(host, Bare("Hand Made Shape", null, ",\"not_ours_confirmed\":true")));
        Assert.Equal("deleted", (string?)r["status"]);
        Assert.True((bool)r["ok"]!);
        Assert.Empty(host.Shapes);
        Assert.Equal("not-ours-confirmed", (string?)r["override"]);
        var applied = ((JsonArray)r["override_applied_to"]!).Select(x => (JsonObject)x!).Single();
        Assert.Equal("Hand Made Shape", (string?)applied["name"]);
        Assert.Equal(guid, (string?)applied["guid"]);
        Assert.Equal("missing", (string?)applied["reason"]);
        // and the same override on a request that needs none records nothing
        var proven = WithTwoItemsOnOneShape(out var shape);
        var ok = Node(Run(proven, Delete(proven, shape, ",\"delete_with_items\":2,\"not_ours_confirmed\":true")));
        Assert.Equal("deleted", (string?)ok["status"]);
        Assert.Null(ok["override"]);
    }

    [Fact]
    public void Delete_OverrideOnAShapeStillInUse_AsksAgainForTheCount()
    {
        var host = WithTwoItemsOnOneShape(out var shape);
        var writes = host.Writes;
        // bare name (no proof) plus the override, but no count: the in-use guard still asks
        var asked = Node(Run(host, Bare(shape, null, ",\"not_ours_confirmed\":true")));
        Assert.Equal("shape-in-use", (string?)asked["status"]);
        Assert.Contains("delete_with_items = 2", (string?)asked["message"]);
        Assert.Null(asked["override"]);                   // nothing was deleted, so nothing is claimed
        Assert.Equal(writes, host.Writes);
        // a wrong count is refused too
        Assert.Equal("delete-count-mismatch", (string?)Node(Run(host, Bare(shape, null, ",\"not_ours_confirmed\":true,\"delete_with_items\":1")))["status"]);
        // both flags: it goes, and the receipt records both
        var done = Node(Run(host, Bare(shape, null, ",\"not_ours_confirmed\":true,\"delete_with_items\":2")));
        Assert.Equal("deleted", (string?)done["status"]);
        Assert.Equal("not-ours-confirmed", (string?)done["override"]);
        Assert.Equal(2, ((JsonArray)done["items_using_shapes"]!).Count);
        Assert.Empty(host.Shapes);
    }

    [Fact]
    public void Delete_NeverDeletesAGuidThatIsNotABrepItem_EvenWithTheOverride()
    {
        var host = NewHost(out _);
        var other = Guid.NewGuid().ToString();
        host.NotBrepGuids.Add(other);
        var refused = Node(Run(host, Bare(null, other)));
        Assert.Equal("not-created-by-this-run", (string?)refused["status"]);
        Assert.Equal("not-a-brep", (string?)((JsonArray)refused["not_created_by_this_run"]!)[0]!["reason"]);
        var forced = Node(Run(host, Bare(null, other, ",\"not_ours_confirmed\":true")));
        Assert.Equal("deleted", (string?)forced["status"]);
        Assert.Equal("skipped-not-a-brep", (string?)((JsonArray)forced["items"]!)[0]!["status"]);
        Assert.Equal(0, host.Writes);
    }

    // The undo in a receipt must run, in every failure that leaves something behind (a hand-made proof check once made the
    // metadata-missing cases impossible to undo).

    static void UndoFromReceiptRemovesEverything(FakeHost host, Dictionary<string, object?> receipt)
    {
        host.FailAtWrite = -1; host.RejectMetadata = false; host.FailMeasure = false;
        var undo = (JsonObject)Node(receipt)["undo"]!["request"]!;
        var result = Node(TeklaItemCreate.Run(Parsed(undo.ToJsonString()), host, "undo"));
        Assert.Equal("deleted", (string?)result["status"]);
        Assert.Null(result["override"]);
        Assert.Empty(host.Shapes);
        Assert.Empty(host.Items);
        Assert.Empty(host.ListCatalogueFiles());
    }

    [Fact]
    public void TheUndo_RunsWhenTheShapeNeverGotItsMetadata_BecauseTeklaRejectedIt()
    {
        var host = NewHost(out _);
        host.RejectMetadata = true;
        var receipt = Run(host, Request("create", new[] { Member("A", 1000) }));
        Assert.Equal("read-back-mismatch", (string?)Node(receipt)["status"]);
        Assert.True(string.IsNullOrEmpty(host.Shapes.Values.Single().Digest));
        UndoFromReceiptRemovesEverything(host, receipt);
    }

    [Fact]
    public void TheUndo_RunsWhenTheInsertThrewBeforeTheMetadataWasSet()
    {
        var host = NewHost(out _);
        host.FailAtWrite = 2;                      // shape inserted, then the metadata call throws
        var receipt = Run(host, Request("create", new[] { Member("A", 1000), Member("B", 2400) }));
        Assert.Equal("partial", (string?)Node(receipt)["status"]);
        Assert.True(string.IsNullOrEmpty(host.Shapes.Values.Single().Digest));
        UndoFromReceiptRemovesEverything(host, receipt);
    }

    [Fact]
    public void TheUndo_RunsWhenTheShapeInsertItselfThrewAfterWriting()
    {
        var host = NewHost(out _);
        host.FailAtWrite = 1; host.FaultAfterEffect = true;
        var receipt = Run(host, Request("create", new[] { Member("A", 1000) }));
        Assert.Equal("partial", (string?)Node(receipt)["status"]);
        UndoFromReceiptRemovesEverything(host, receipt);
    }

    [Fact]
    public void TheUndo_RunsAfterAnItemWasPlacedAndItsReadBackFailed()
    {
        var host = NewHost(out _);
        host.FailMeasure = true;
        var receipt = Run(host, Request("create", new[] { Member("A", 1000) }));
        Assert.Equal("partial", (string?)Node(receipt)["status"]);
        UndoFromReceiptRemovesEverything(host, receipt);
    }

    [Fact]
    public void Delete_ChecksTheGuardAgainAtTheWrite_WhenAnItemAppearsAfterTheFirstRead()
    {
        var host = WithTwoItemsOnOneShape(out var shape);
        var guids = host.Items.Keys.ToList();
        host.SneakItemAfterDelete = true;
        var r = Node(Run(host, Delete(host, shape, null, "[\"" + guids[0] + "\",\"" + guids[1] + "\"]")));
        // both listed Items go; a new one appeared on the shape meanwhile, so the shape stays
        Assert.Equal("partial", (string?)r["status"]);
        Assert.Contains("now use", (string?)r["error"]);
        Assert.Single(host.Shapes);
        Assert.Equal(2, ((JsonArray)r["items"]!).Count);                         // what already went is listed
        Assert.All(((JsonArray)r["items"]!).Select(i => (string?)i!["status"]), s => Assert.Equal("deleted", s));
    }

    [Fact]
    public void Delete_WhenTeklaWillNotCountTheInstances_StillRespectsTheModelScan_AndSaysSo()
    {
        var host = WithTwoItemsOnOneShape(out var shape);
        host.HideInstanceCount = true;
        var r = Node(Run(host, Delete(host, shape)));
        Assert.Equal("shape-in-use", (string?)r["status"]);
        Assert.Contains("would not count", string.Join(" ", ((JsonArray)r["warnings"]!).Select(w => (string?)w)));
    }

    [Fact]
    public void ADigestTagTeklaDoesNotAccept_IsReportedNotPassedOff()
    {
        var host = NewHost(out _);
        host.RejectMetadata = true;
        var r = Node(Run(host, Request("create", new[] { Member("A", 1000) })));
        Assert.Equal("read-back-mismatch", (string?)r["status"]);
        Assert.False((bool)r["ok"]!);
        Assert.Contains("aware.digest", string.Join(" ", ((JsonArray)r["warnings"]!).Select(w => (string?)w)));
        Assert.Single(host.Items);       // what was written stays, and is in the undo
        Assert.NotNull(r["undo"]);
    }

    [Fact]
    public void PlacingStopsAtTheFirstItemThatDoesNotReadBack_AndTheRestAreNotPlaced()
    {
        var host = NewHost(out _);
        host.SkewVolume = true;
        var r = Node(Run(host, Request("create", new[] { Member("A", 1000), Member("B", 2400), Member("C", 1000) })));
        Assert.Equal("read-back-mismatch", (string?)r["status"]);
        Assert.Single(host.Items);
        var statuses = ((JsonArray)r["items"]!).Select(i => (string?)i!["status"]).ToArray();
        Assert.Equal(new[] { "read-back-mismatch", "not-placed", "not-placed" }, statuses);
        Assert.Contains("not-placed", (string?)r["message"]);
    }

    [Fact]
    public void AShapeTeklaWroteBeforeItsInsertThrew_IsInTheLedgerAndTheUndo()
    {
        var host = NewHost(out _);
        host.FailAtWrite = 1; host.FaultAfterEffect = true;
        var r = Node(Run(host, Request("create", new[] { Member("A", 1000) })));
        Assert.Equal("partial", (string?)r["status"]);
        Assert.Equal("shape", (string?)((JsonArray)r["ledger"]!).Single()!["kind"]);
    }

    // ── a request that passes Parse must be one that runs ─────────────────────

    [Fact]
    public void Parse_RequiresAModelForAWrite_AndRefusesPinsOfTheWrongType()
    {
        var noTarget = "{\"mode\":\"create\",\"consent\":{\"source\":\"ui-click\"},\"members\":[" + Member("A", 1000) + "]}";
        Assert.Contains("which model", Refusal(noTarget));
        Assert.Contains("target.model_guid must be text", Refusal(Request("create", new[] { Member("A", 1000) }, ",\"target\":{\"model_guid\":123}")));
        Assert.Contains("target.model_path must be text", Refusal(Request("create", new[] { Member("A", 1000) }, ",\"target\":{\"model_path\":true}")));
        Assert.Contains("shared_model must be true or false", Refusal(Request("create", new[] { Member("A", 1000) }, ",\"target\":{\"model_guid\":\"g\",\"shared_model\":\"no\"}")));
        // a plan needs no model: it only reports
        Assert.Null(Refusal(Request("plan", new[] { Member("A", 1000) })));
        Assert.Contains("mode must be text", Refusal("{\"mode\":5}"));
        Assert.Contains("version must be text", Refusal(Request("plan", new[] { Member("A", 1000) }, ",\"version\":5")));
        Assert.Contains("solid_digest must be text", Refusal(Request("plan", new[] { Member("A", 1000, ",\"solid_digest\":7") })));
        Assert.Contains("label must be text", Refusal(Request("plan", new[] { Member("A", 1000).Replace("\"label\":\"Member A\"", "\"label\":{}") })));
    }

    [Fact]
    public void Parse_CapsTheHoles_AndDeduplicatesDeletedShapeNamesByCase()
    {
        var holes = string.Join(",", Enumerable.Range(0, PrismSolid.MaxHoles + 1).Select(i => $"[[{i * 100 + 10},0],[{i * 100 + 20},0],[{i * 100 + 15},10]]"));
        var big = Member("A", 1000, outer: "[[-100,-100],[100000,-100],[100000,100],[-100,100]]", holes: "[" + holes + "]");
        Assert.Contains("holes", Refusal(Request("plan", new[] { big })));
        var del = Parsed("{\"mode\":\"delete\",\"consent\":{\"source\":\"ui-click\"},\"target\":{\"model_guid\":\"g\"},\"shapes\":[\"AWI-ab\",\"AWI-AB\"]}");
        Assert.Single(del.ShapeNames);
    }

    // ── the file half ─────────────────────────────────────────────────────────

    [Fact]
    public void ModelGuid_IsReadFromTheModelFile()
    {
        var dir = Path.Combine(_root, "m");
        Directory.CreateDirectory(dir);
        File.WriteAllText(Path.Combine(dir, "TeklaStructuresModel.xml"), "<TeklaStructuresModels><Model><Name>x</Name><Guid>C7CE9DE7-9F43-4950-B735-04698C3B9312</Guid></Model></TeklaStructuresModels>");
        Assert.Equal("c7ce9de7-9f43-4950-b735-04698c3b9312", TeklaItemCreate.ReadModelGuid(dir));
        Assert.Null(TeklaItemCreate.ReadModelGuid(Path.Combine(_root, "nothing")));
    }

    // ── the Tekla half, on its source ─────────────────────────────────────────

    static string Source(string file)
    {
        var dir = new DirectoryInfo(AppContext.BaseDirectory);
        while (dir != null && !File.Exists(Path.Combine(dir.FullName, "cli-tekla.csproj"))) dir = dir.Parent;
        Assert.NotNull(dir);
        return File.ReadAllText(Path.Combine(dir!.FullName, file));
    }

    [Fact]
    public void Script_ParsesAndHoldsTheLiveFindings()
    {
        var code = TeklaItemCreateScript.Code;
        var tree = Microsoft.CodeAnalysis.CSharp.CSharpSyntaxTree.ParseText(code, new Microsoft.CodeAnalysis.CSharp.CSharpParseOptions(kind: SourceCodeKind.Script));
        Assert.DoesNotContain(tree.GetDiagnostics(), d => d.Severity == DiagnosticSeverity.Error);
        // the position settings the live run needed, all three explicit
        Assert.Contains("Position.PlaneEnum.MIDDLE", code);
        Assert.Contains("Position.DepthEnum.MIDDLE", code);
        Assert.Contains("Position.RotationEnum.FRONT", code);
        // Tekla's validator runs on the brep; the insert that reports matches is the one used, never one that overwrites
        Assert.Contains("Polymesh.Validate(", code);
        Assert.Contains("InsertOrGetGuidsOfShapesWithMatchingGeometry", code);
        foreach (var forbidden in new[] { ".Modify(", "InsertUsingNormalsAllowDuplicates", "ImportShapeItems", ".Rename(", "Directory.Delete", "File.Delete" })
            Assert.DoesNotContain(forbidden, code);
        // ShapeItem.Delete is reachable only through the host's DeleteShape, which the engine calls only after its count check
        Assert.Equal(1, CountOf(code, ".Delete()") - CountOf(code, "item.Delete()"));
        Assert.Contains("return s.Select(name) && s.Delete();", code);
        // the connection and the commit happen on the script's own terms
        Assert.Contains("!model0.GetConnectionStatus()", code);
        Assert.Contains("m.CommitChanges()", code);
        Assert.Contains("SetCurrentTransformationPlane(previous)", code);
        // the insert and the read-back each take the global work plane (Tekla reports coordinates in the current one)
        Assert.Equal(2, CountOf(code, "SetCurrentTransformationPlane(new TransformationPlane())"));
    }

    static int CountOf(string text, string needle)
    {
        int n = 0, at = 0;
        while ((at = text.IndexOf(needle, at, StringComparison.Ordinal)) >= 0) { n++; at += needle.Length; }
        return n;
    }

    [Fact]
    public void Engine_ChecksInTheDesignsOrder_AndOnlyWritesThroughTheHost()
    {
        var engine = Source("TeklaItemCreate.cs");
        int At(string needle)
        {
            var at = engine.IndexOf(needle, StringComparison.Ordinal);
            Assert.True(at >= 0, "expected to find: " + needle);
            return at;
        }
        Assert.True(At("TargetChanged(req, target)") < At("host.FindShape(plan.Name)"));
        Assert.True(At("host.FindShape(plan.Name)") < At("host.ValidateBrep("));
        Assert.True(At("host.ValidateBrep(") < At("host.BackupCatalogue(runId)"));
        Assert.True(At("host.BackupCatalogue(runId)") < At("host.InsertShape("));
        Assert.True(At("host.InsertShape(") < At("host.InsertItem("));
        // the ledger entry is recorded the moment Tekla says it inserted, before the follow-up calls
        Assert.True(At("ledger.Add(entry);") < At("host.SetShapeMetadata("));
        // the delete guard decides before the first delete
        var del = engine.Substring(At("static Dictionary<string, object?> RunDelete("));
        Assert.True(del.IndexOf("shape-in-use", StringComparison.Ordinal) < del.IndexOf("host.DeleteItem(", StringComparison.Ordinal));
        Assert.True(del.IndexOf("delete-count-mismatch", StringComparison.Ordinal) < del.IndexOf("host.DeleteShape(", StringComparison.Ordinal));
    }

    [Fact]
    public void Verb_IsDispatched_RefusesBeforeTekla_AndRunsScriptOwned()
    {
        var program = Source("Program.cs");
        Assert.Contains("case \"item-create\":", program);
        var handler = program.Substring(program.IndexOf("static int ItemCreate(", StringComparison.Ordinal));
        Assert.True(handler.IndexOf("TeklaItemCreate.Parse(input", StringComparison.Ordinal) < handler.IndexOf("TryReadPid(", StringComparison.Ordinal));
        var call = handler.Substring(handler.IndexOf("ExecuteResolvedScript(\"item-create\"", StringComparison.Ordinal));
        Assert.Contains("TeklaItemCreateScript.Code", call.Substring(0, call.IndexOf(';')));
        Assert.Contains("ScriptCommitPolicy.ScriptOwned", call.Substring(0, call.IndexOf(';')));
    }

    [Fact]
    public void ParseArgs_TakesDeleteWithItems_AsAWholeNumber()
    {
        Assert.Equal(3, Program.ParseArgs(new[] { "--json-stdin", "--delete-with-items", "3" }).DeleteWithItems);
        Assert.Null(Program.ParseArgs(new[] { "--json-stdin" }).DeleteWithItems);
        Assert.Throws<InvalidOperationException>(() => Program.ParseArgs(new[] { "--delete-with-items", "two" }));
        Assert.Throws<InvalidOperationException>(() => Program.ParseArgs(new[] { "--delete-with-items", "-1" }));
        Assert.Throws<InvalidOperationException>(() => Program.ParseArgs(new[] { "--delete-with-items" }));
        Assert.True(Program.ParseArgs(new[] { "--json-stdin", "--not-ours-confirmed" }).NotOursConfirmed);
        Assert.False(Program.ParseArgs(new[] { "--json-stdin" }).NotOursConfirmed);
    }

    [Fact]
    public void ItemCreateResult_WithOkFalse_IsAFailureReceipt_ButOtherVerbsAreNotAffected()
    {
        var notOk = JsonNode.Parse("{\"ok\":false,\"status\":\"partial\"}");
        var ok = JsonNode.Parse("{\"ok\":true,\"status\":\"created\"}");
        Assert.True(Program.ScriptResultReportsFailure("item-create", notOk));
        Assert.False(Program.ScriptResultReportsFailure("item-create", ok));
        Assert.False(Program.ScriptResultReportsFailure("section-measure", notOk));
    }

    /// <summary>
    /// A local gate, not a CI one (CI has no Tekla): when a Tekla install is present, the script is compiled against its
    /// assemblies with the bridge's own script options, which is how a wrong Open API name is caught without Tekla running.
    /// </summary>
    [Fact]
    public void Script_CompilesAgainstTheTeklaAssemblies_WhenAnInstallIsPresent()
    {
        var install = new[] { @"C:\Program Files\Tekla Structures\2026.0", @"C:\Program Files\Tekla Structures\2025.0" }.FirstOrDefault(Directory.Exists);
        if (install == null || Environment.GetEnvironmentVariable("AWARE_SKIP_TEKLA_COMPILE") == "1") return;
        var (references, _) = Program.ResolveTeklaReferences(install);
        Assert.NotEmpty(references);
        var script = Microsoft.CodeAnalysis.CSharp.Scripting.CSharpScript.Create<object>(
            TeklaItemCreateScript.Code, Program.CreateScriptOptions(references), typeof(ExecGlobals));
        var errors = script.Compile().Where(d => d.Severity == DiagnosticSeverity.Error).Select(d => d.ToString()).ToList();
        Assert.True(errors.Count == 0, "item-create script does not compile against " + install + ":\n" + string.Join("\n", errors));
    }
}
