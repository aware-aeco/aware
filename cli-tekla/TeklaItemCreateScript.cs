namespace AwareTekla;

/// <summary>
/// `item-create` (aware#859): the Tekla half of sending a member as an Item. It implements
/// <see cref="IItemCreateHost"/> with the Open API (shape catalogue, Items, read-back) and hands the request to
/// <see cref="TeklaItemCreate.RunJson"/>, which owns the checks, the order of writes, the receipt and the undo.
/// Run ScriptOwned: the host commits each Item itself, and a shape insert is a catalogue write, not a model change.
/// What each call does here was measured live on Tekla 2026.0 (spike run 1, docs/superpowers/specs/2026-11-19-a5-tekla-item-SPIKE.md).
/// </summary>
internal static class TeklaItemCreateScript
{
    /// <summary>The script's using directives.</summary>
    internal const string Usings = """
using System;
using System.Collections;
using System.Collections.Generic;
using System.Globalization;
using System.IO;
using System.Linq;
using AwareTekla;
using Tekla.Structures.Catalogs;
using Tekla.Structures.Geometry3d;
using Tekla.Structures.Model;
using Tekla.Structures.Solid;
""";

    internal const string Body = """

class TeklaItemHost : CatalogueFileHost, IItemCreateHost
{
    readonly Model m;

    public TeklaItemHost(Model model, string modelPath) : base(modelPath) { m = model; }

    public ItemTarget ReadTarget()
    {
        var info = m.GetInfo();
        var target = new ItemTarget { ModelPath = info.ModelPath, ModelGuid = TeklaItemCreate.ReadModelGuid(info.ModelPath) };
        try { target.SharedModel = info.SharedModel; } catch (Exception) { target.SharedModel = null; }
        try { target.SingleUserModel = info.SingleUserModel; } catch (Exception) { target.SingleUserModel = null; }
        try { target.TeklaVersion = Tekla.Structures.TeklaStructuresInfo.GetCurrentProgramVersion(); } catch (Exception) { target.TeklaVersion = null; }
        return target;
    }

    static ShapeInfo Describe(ShapeItem s)
    {
        var info = new ShapeInfo { Name = s.Name, Guid = s.Guid, GeometryGuid = s.GeometryGuid };
        try { var e = s.Extrema; if (e != null) info.Extrema = new[] { e.MinPoint.X, e.MinPoint.Y, e.MinPoint.Z, e.MaxPoint.X, e.MaxPoint.Y, e.MaxPoint.Z }; } catch (Exception) { info.Extrema = null; }
        try { info.InstanceCount = s.GetInstanceCount(); } catch (Exception) { info.InstanceCount = 0; }
        try { foreach (var v in s.ShapeFacetedBrep.Vertices) info.Vertices.Add(new[] { (double)v.X, (double)v.Y, (double)v.Z }); } catch (Exception) { }
        try
        {
            var type = ShapeMetadataTypeEnum.Text; object value = null; string label = "", description = "";
            if (s.GetMetadata("aware.digest", ref type, ref value, ref label, ref description)) info.Digest = Convert.ToString(value, CultureInfo.InvariantCulture);
        }
        catch (Exception) { info.Digest = null; }
        return info;
    }

    public ShapeInfo FindShape(string name)
    {
        var s = new ShapeItem();
        return s.Select(name) ? Describe(s) : null;
    }

    public ShapeInfo FindShapeByGuid(string guid)
    {
        var all = new CatalogHandler().GetShapeItems();
        while (all.MoveNext())
        {
            var s = all.Current as ShapeItem;
            if (s != null && String.Equals(s.Guid, guid, StringComparison.OrdinalIgnoreCase)) return Describe(s);
        }
        return null;
    }

    static FacetedBrep ToBrep(PrismBrep b)
    {
        var vertices = b.Vertices.Select(v => new Vector(v[0], v[1], v[2])).ToArray();
        return new FacetedBrep(vertices, b.Faces.ToArray(), new Dictionary<int, int[][]>(b.InnerLoops));
    }

    public IReadOnlyList<string> ValidateBrep(PrismBrep brep)
    {
        var problems = new List<KeyValuePair<int, Polymesh.PolymeshHealthCheckEnum>>();
        bool valid = Polymesh.Validate(ToBrep(brep), Polymesh.PolymeshCheckerFlags.All, ref problems);
        var text = new List<string>();
        if (problems != null) foreach (var p in problems) text.Add("face " + p.Key + ": " + p.Value);
        if (!valid && text.Count == 0) text.Add("Polymesh.Validate returned false without naming a problem");
        return text;
    }

    public ShapeInsertResult InsertShape(string name, PrismBrep brep)
    {
        var result = new ShapeInsertResult();
        var shape = new ShapeItem { Name = name, ShapeFacetedBrep = ToBrep(brep), UpAxis = ShapeUpAxis.Y_Axis };
        List<string> existing;
        result.Inserted = shape.InsertOrGetGuidsOfShapesWithMatchingGeometry(out existing);
        if (!result.Inserted && existing != null) result.MatchingGuids.AddRange(existing);
        if (!result.Inserted && result.MatchingGuids.Count == 0) result.Refusal = "Tekla returned false and named no shape with the same geometry";
        return result;
    }

    public bool SetShapeMetadata(string name, string digest, string label)
    {
        var s = new ShapeItem();
        if (!s.Select(name)) return false;
        bool a = s.SetMetadata("aware.digest", ShapeMetadataTypeEnum.Text, digest, "AWARE digest", "sha256 of the neutral solid this shape was made from");
        bool b = s.SetMetadata("aware.label", ShapeMetadataTypeEnum.Text, label, "AWARE label", "the member this shape was made for");
        return a && b;
    }

    static double Dist(Point p, double[] q) => Math.Sqrt(Math.Pow(p.X - q[0], 2) + Math.Pow(p.Y - q[1], 2) + Math.Pow(p.Z - q[2], 2));

    // Reads an Item back. With a spec it also measures the Item against the request and against the shape moved into the
    // world by Tekla's own coordinate system, the check that does not depend on any convention of ours.
    ItemReadback Read(Brep item, ItemSpec spec)
    {
        var r = new ItemReadback { Exists = true, Guid = item.Identifier.GUID.ToString(), Name = item.Name, ShapeName = item.Profile.ProfileString };
        if (spec == null) return r;
        var shape = new ShapeItem();
        if (shape.Select(spec.ShapeName)) { try { r.ShapeInstanceCount = shape.GetInstanceCount(); } catch (Exception) { } }
        var cs = item.GetCoordinateSystem();
        var ax = new Vector(cs.AxisX); ax.Normalize(); var ay = new Vector(cs.AxisY); ay.Normalize();
        var az = ax.Cross(ay); az.Normalize();
        var dir = new Vector(spec.End[0] - spec.Start[0], spec.End[1] - spec.Start[1], spec.End[2] - spec.Start[2]); dir.Normalize();
        r.OriginToStartMm = Math.Sqrt(Math.Pow(cs.Origin.X - spec.Start[0], 2) + Math.Pow(cs.Origin.Y - spec.Start[1], 2) + Math.Pow(cs.Origin.Z - spec.Start[2], 2));
        r.AxisXDot = ax.Dot(dir);

        var solid = item.GetSolid();
        var points = new List<Point>();
        double volume = 0;
        var faces = solid.GetFaceEnumerator();
        while (faces.MoveNext())
        {
            var face = faces.Current as Face;
            if (face == null) continue;
            var n = new Vector(face.Normal); n.Normalize();
            double bestArea = -1; Point first = null;
            var areas = new List<double>();
            var loops = face.GetLoopEnumerator();
            while (loops.MoveNext())
            {
                var loop = loops.Current as Loop;
                if (loop == null) continue;
                var ring = new List<Point>();
                var vs = loop.GetVertexEnumerator();
                while (vs.MoveNext()) { var p = vs.Current as Point; if (p != null) { ring.Add(new Point(p.X, p.Y, p.Z)); points.Add(new Point(p.X, p.Y, p.Z)); } }
                double nx = 0, ny = 0, nz = 0;
                for (int i = 0; i < ring.Count; i++)
                {
                    var a = ring[i]; var b = ring[(i + 1) % ring.Count];
                    nx += a.Y * b.Z - a.Z * b.Y; ny += a.Z * b.X - a.X * b.Z; nz += a.X * b.Y - a.Y * b.X;
                }
                double area = 0.5 * Math.Sqrt(nx * nx + ny * ny + nz * nz);
                areas.Add(area);
                if (area > bestArea) { bestArea = area; first = ring.Count > 0 ? ring[0] : null; }
            }
            double holes = areas.Sum() - bestArea;                                     // every loop but the largest is a hole
            if (first != null) volume += (n.X * first.X + n.Y * first.Y + n.Z * first.Z) * (bestArea - holes) / 3.0;
        }
        var mn = solid.MinimumPoint; var mx = solid.MaximumPoint;
        r.SolidBox = new[] { mn.X, mn.Y, mn.Z, mx.X, mx.Y, mx.Z };
        r.VolumeFromFaces = volume;
        if (points.Count > 0)
        {
            Func<Point, double> along = p => (p.X - cs.Origin.X) * ax.X + (p.Y - cs.Origin.Y) * ax.Y + (p.Z - cs.Origin.Z) * ax.Z;
            r.ExtentAlongAxisX = points.Max(along) - points.Min(along);
        }
        var local = new List<double[]>();
        try { foreach (var v in shape.ShapeFacetedBrep.Vertices) local.Add(new[] { (double)v.X, (double)v.Y, (double)v.Z }); } catch (Exception) { }
        if (local.Count > 0 && points.Count > 0)
        {
            var moved = local.Select(c => new[]
            {
                cs.Origin.X + c[0] * ax.X + c[1] * ay.X + c[2] * az.X,
                cs.Origin.Y + c[0] * ax.Y + c[1] * ay.Y + c[2] * az.Y,
                cs.Origin.Z + c[0] * ax.Z + c[1] * ay.Z + c[2] * az.Z,
            }).ToList();
            r.MaxSolidToShapeMm = points.Max(p => moved.Min(q => Dist(p, q)));
            r.MaxShapeToSolidMm = moved.Max(q => points.Min(p => Dist(p, q)));
        }
        return r;
    }

    public string InsertItem(ItemSpec spec)
    {
        var handler = m.GetWorkPlaneHandler();
        var previous = handler.GetCurrentTransformationPlane();
        try
        {
            if (!handler.SetCurrentTransformationPlane(new TransformationPlane())) throw new Exception("could not switch to the global work plane");
            var item = new Brep(new Point(spec.Start[0], spec.Start[1], spec.Start[2]), new Point(spec.End[0], spec.End[1], spec.End[2]));
            item.Name = spec.Name;
            item.Profile.ProfileString = spec.ShapeName;
            item.Material.MaterialString = spec.Material;
            item.Class = "1";
            // Measured live: with Tekla's default Position the Item is shifted by half the section; these three put the
            // section on the start-to-end line with the shape's Y axis along world Y.
            item.Position.Plane = Position.PlaneEnum.MIDDLE;
            item.Position.Depth = Position.DepthEnum.MIDDLE;
            item.Position.Rotation = Position.RotationEnum.FRONT;
            item.Position.RotationOffset = spec.RollDeg;
            item.Position.PlaneOffset = 0.0;
            item.Position.DepthOffset = 0.0;
            if (!item.Insert()) return null;
            // Tekla has the Item from here on: tell the engine before anything below can throw.
            spec.PlacedGuid = item.Identifier.GUID.ToString();
            // Labelled as what it is: a fixed solid of one size and one length, not a profile.
            bool uda = true;
            try
            {
                uda &= item.SetUserProperty("AWARE_SENT_AS_ITEM", "1");
                uda &= item.SetUserProperty("AWARE_DIGEST", spec.Digest);
                if (!String.IsNullOrEmpty(spec.DefinitionDigest)) uda &= item.SetUserProperty("AWARE_DEFINITION_DIGEST", spec.DefinitionDigest);
                uda &= item.SetUserProperty("AWARE_MEMBER_ID", spec.MemberId);
            }
            catch (Exception) { uda = false; }
            m.CommitChanges();
            lastUdaSet = uda;
            return spec.PlacedGuid;
        }
        finally
        {
            try { handler.SetCurrentTransformationPlane(previous); } catch (Exception) { }
        }
    }

    bool lastUdaSet;

    public ItemReadback MeasureItem(ItemSpec spec, string guid)
    {
        var item = SelectItem(guid);
        if (item == null) return new ItemReadback { Exists = false, Guid = guid };
        var read = Read(item, spec);
        read.UdaSet = lastUdaSet;
        return read;
    }

    Brep SelectItem(string guid)
    {
        ModelObject mo = null;
        try { mo = m.SelectModelObject(new Tekla.Structures.Identifier(new Guid(guid))); } catch (Exception) { mo = null; }
        return mo as Brep;
    }

    public ItemReadback ReadItem(string guid)
    {
        var item = SelectItem(guid);
        return item == null ? new ItemReadback { Exists = false, Guid = guid } : Read(item, null);
    }

    public IReadOnlyList<ItemRef> ItemsUsingShape(string name)
    {
        var users = new List<ItemRef>();
        var all = m.GetModelObjectSelector().GetAllObjectsWithType(ModelObject.ModelObjectEnum.BREP);
        while (all.MoveNext())
        {
            var b = all.Current as Brep;
            if (b != null && String.Equals(b.Profile.ProfileString, name, StringComparison.Ordinal)) users.Add(new ItemRef { Guid = b.Identifier.GUID.ToString(), Name = b.Name });
        }
        return users;
    }

    public bool DeleteItem(string guid)
    {
        var item = SelectItem(guid);
        if (item == null) return false;
        bool ok = item.Delete();
        m.CommitChanges();
        return ok;
    }

    public bool DeleteShape(string name)
    {
        var s = new ShapeItem();
        return s.Select(name) && s.Delete();
    }
}

var model0 = model as Model;
if (model0 == null || !model0.GetConnectionStatus()) throw new Exception("No Tekla model is open, so there is no shape catalogue to write to.");
string modelPath0 = "";
try { modelPath0 = model0.GetInfo().ModelPath; } catch (Exception ex) { throw new Exception("Tekla did not say which model is open: " + ex.Message); }
if (String.IsNullOrWhiteSpace(modelPath0)) throw new Exception("Tekla did not say which model is open.");
object requestJson;
if (args == null || !args.TryGetValue("request_json", out requestJson) || !(requestJson is string)) throw new Exception("args.request_json is missing.");
return TeklaItemCreate.RunJson((string)requestJson, new TeklaItemHost(model0, modelPath0));
""";

    internal const string Code = Usings + "\n" + Body;
}
