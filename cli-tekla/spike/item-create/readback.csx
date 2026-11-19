// A5 SPIKE, experiment E7 (aware#859). READ-ONLY. EXPERIMENTAL: run through `exec` by run-spike.ps1.
// What it does: lists every Item (Tekla.Structures.Model.Brep) in the open model whose profile string starts with AWI,
// with its shape, coordinate system, solid bounds and the volume computed from the solid's own faces, and lists every
// shape whose name starts with AWI with its extrema and metadata. Run it again after Tekla has been closed through its
// own exit and the same scratch model reopened, to prove the shape and the item survive a save and reload.
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using Tekla.Structures.Catalogs;
using Tekla.Structures.Geometry3d;
using Tekla.Structures.Model;
using Tekla.Structures.Solid;

var m = model as Model;
if (m == null || !m.GetConnectionStatus()) throw new Exception("No Tekla model is open.");
var modelPath = m.GetInfo().ModelPath;
var leaf = Path.GetFileName(modelPath.TrimEnd('\\', '/'));
if (!leaf.StartsWith("A5SPIKE", StringComparison.Ordinal)) throw new Exception("Refusing: '" + leaf + "' is not a throwaway A5SPIKE* model.");

var handler = m.GetWorkPlaneHandler();
var previousPlane = handler.GetCurrentTransformationPlane();
var items = new List<object>();
var shapes = new List<object>();
try
{
    if (!handler.SetCurrentTransformationPlane(new TransformationPlane())) throw new Exception("could not switch to the global work plane");
    var moe = m.GetModelObjectSelector().GetAllObjectsWithType(ModelObject.ModelObjectEnum.BREP);
    while (moe.MoveNext())
    {
        var item = moe.Current as Brep;
        if (item == null || item.Profile == null || !item.Profile.ProfileString.StartsWith("AWI", StringComparison.Ordinal)) continue;
        var row = new Dictionary<string, object>
        {
            { "guid", item.Identifier.GUID.ToString() }, { "name", item.Name }, { "shape", item.Profile.ProfileString },
            { "start", new[] { item.StartPoint.X, item.StartPoint.Y, item.StartPoint.Z } }, { "end", new[] { item.EndPoint.X, item.EndPoint.Y, item.EndPoint.Z } },
            { "rotation", item.Position.Rotation.ToString() }, { "rotation_offset", item.Position.RotationOffset },
        };
        var cs = item.GetCoordinateSystem();
        row["origin"] = new[] { cs.Origin.X, cs.Origin.Y, cs.Origin.Z };
        row["axis_x"] = new[] { cs.AxisX.X, cs.AxisX.Y, cs.AxisX.Z }; row["axis_y"] = new[] { cs.AxisY.X, cs.AxisY.Y, cs.AxisY.Z };
        var solid = item.GetSolid();
        double volume = 0;
        var faces = solid.GetFaceEnumerator();
        while (faces.MoveNext())
        {
            var face = faces.Current as Face;
            if (face == null) continue;
            var n = new Vector(face.Normal); n.Normalize();
            double best = -1; Point first = null; var areas = new List<double>();
            var loops = face.GetLoopEnumerator();
            while (loops.MoveNext())
            {
                var loop = loops.Current as Loop;
                if (loop == null) continue;
                var ring = new List<Point>();
                var vs = loop.GetVertexEnumerator();
                while (vs.MoveNext()) { var p = vs.Current as Point; if (p != null) ring.Add(new Point(p.X, p.Y, p.Z)); }
                double nx = 0, ny = 0, nz = 0;
                for (int i = 0; i < ring.Count; i++)
                {
                    var a = ring[i]; var b = ring[(i + 1) % ring.Count];
                    nx += a.Y * b.Z - a.Z * b.Y; ny += a.Z * b.X - a.X * b.Z; nz += a.X * b.Y - a.Y * b.X;
                }
                double area = 0.5 * Math.Sqrt(nx * nx + ny * ny + nz * nz);
                areas.Add(area);
                if (area > best) { best = area; first = ring.Count > 0 ? ring[0] : null; }
            }
            if (first != null) volume += (n.X * first.X + n.Y * first.Y + n.Z * first.Z) * (best - (areas.Sum() - best)) / 3.0;
        }
        var mn = solid.MinimumPoint; var mx = solid.MaximumPoint;
        row["solid_box"] = new[] { mn.X, mn.Y, mn.Z, mx.X, mx.Y, mx.Z };
        row["volume_from_faces"] = volume;
        items.Add(row);
    }
}
finally
{
    try { handler.SetCurrentTransformationPlane(previousPlane); } catch { }
}

var en = new CatalogHandler().GetShapeItems();
while (en.MoveNext())
{
    var s = en.Current as ShapeItem;
    if (s == null || !s.Name.StartsWith("AWI", StringComparison.Ordinal)) continue;
    var row = new Dictionary<string, object> { { "name", s.Name }, { "guid", s.Guid }, { "geometry_guid", s.GeometryGuid }, { "fingerprint", s.Fingerprint }, { "is_solid", s.IsSolid }, { "instance_count", s.GetInstanceCount() } };
    try { var e = s.Extrema; row["extrema"] = new[] { e.MinPoint.X, e.MinPoint.Y, e.MinPoint.Z, e.MaxPoint.X, e.MaxPoint.Y, e.MaxPoint.Z }; } catch { }
    var meta = new Dictionary<string, object>();
    try { foreach (var key in s.GetAllMetadataKeys()) { var t = ShapeMetadataTypeEnum.Text; object v = null; string label = "", desc = ""; if (s.GetMetadata(key, ref t, ref v, ref label, ref desc)) meta[key] = t + ":" + v; } } catch (Exception ex) { meta["error"] = ex.Message; }
    row["metadata"] = meta;
    shapes.Add(row);
}

return new Dictionary<string, object> { { "experiment", "E7-readback" }, { "model_path", modelPath }, { "items", items }, { "shapes", shapes } };
