// A5 SPIKE, experiments E3-E4 (aware#859). EXPERIMENTAL, WRITES ONE ITEM INTO THE OPEN MODEL. Run only through run-spike.ps1 -Write.
// What it does: inserts a Tekla Item (the Open API class Tekla.Structures.Model.Brep, whose Profile.ProfileString is the
// name of a shape in the model's shape catalogue) at args.start / args.end with a rotation, commits, and reads the result
// back: Tekla's coordinate system for the item, its solid's bounds and volume (computed from the solid's own faces, and
// Tekla's report volume where it gives one), and how far the solid sits from the catalogue shape moved into the world by
// that coordinate system. It never touches the catalogue: the shape must already exist (made by shape-create.csx).
// Refused unless: the open model is a throwaway A5SPIKE* model inside args.scratch_root, the shape name starts with AWI and
// exists, and the item name starts with AWI-ITEM.
using System;
using System.Collections;
using System.Collections.Generic;
using System.Globalization;
using System.IO;
using System.Linq;
using Tekla.Structures.Catalogs;
using Tekla.Structures.Geometry3d;
using Tekla.Structures.Model;
using Tekla.Structures.Solid;

object value;
var m = model as Model;
if (m == null || !m.GetConnectionStatus()) throw new Exception("No Tekla model is open.");
var modelPath = m.GetInfo().ModelPath;
var leaf = Path.GetFileName(modelPath.TrimEnd('\\', '/'));
if (!leaf.StartsWith("A5SPIKE", StringComparison.Ordinal)) throw new Exception("Refusing: '" + leaf + "' is not a throwaway A5SPIKE* model.");

Func<string, string> full = p => Path.GetFullPath(p).TrimEnd('\\') + "\\";
Func<string, string> argText = key => (args != null && args.TryGetValue(key, out value)) ? value as string : null;
Func<string, double[]> argPoint = key =>
{
    if (args == null || !args.TryGetValue(key, out value) || !(value is IEnumerable raw)) throw new Exception("args." + key + " must be [x,y,z].");
    var c = raw.Cast<object>().Select(x => Convert.ToDouble(x, CultureInfo.InvariantCulture)).ToArray();
    if (c.Length != 3) throw new Exception("args." + key + " must be [x,y,z].");
    return c;
};
Func<string, double, double> argNumber = (key, fallback) => (args != null && args.TryGetValue(key, out value) && value != null) ? Convert.ToDouble(value, CultureInfo.InvariantCulture) : fallback;

string scratchRoot = argText("scratch_root");
string shapeName = argText("shape");
string itemName = argText("item_name");
if (String.IsNullOrWhiteSpace(scratchRoot) || !Path.IsPathRooted(scratchRoot)) throw new Exception("args needs an absolute scratch_root.");
if (!full(modelPath).StartsWith(full(scratchRoot), StringComparison.OrdinalIgnoreCase)) throw new Exception("Refusing: the model folder is not under the scratch root.");
if (String.IsNullOrWhiteSpace(shapeName) || !shapeName.StartsWith("AWI", StringComparison.Ordinal)) throw new Exception("Refusing: shape must start with AWI.");
if (String.IsNullOrWhiteSpace(itemName) || !itemName.StartsWith("AWI-ITEM", StringComparison.Ordinal)) throw new Exception("Refusing: item_name must start with AWI-ITEM.");
var shape = new ShapeItem();
if (!shape.Select(shapeName)) throw new Exception("Refusing: no shape named '" + shapeName + "' in the model's catalogue.");

var start = argPoint("start"); var end = argPoint("end");
var handler = m.GetWorkPlaneHandler();
var previousPlane = handler.GetCurrentTransformationPlane();
var result = new Dictionary<string, object> { { "experiment", "item-place" }, { "model_path", modelPath }, { "shape", shapeName }, { "item_name", itemName } };
try
{
    if (!handler.SetCurrentTransformationPlane(new TransformationPlane())) throw new Exception("could not switch to the global work plane");

    var item = new Brep(new Point(start[0], start[1], start[2]), new Point(end[0], end[1], end[2]));
    item.Name = itemName;
    item.Profile.ProfileString = shapeName;
    item.Material.MaterialString = argText("material") ?? "Steel_Undefined";
    item.Class = argText("item_class") ?? "1";
    string rot = argText("rotation"), plane = argText("plane"), depth = argText("depth");
    if (rot != null) item.Position.Rotation = (Position.RotationEnum)Enum.Parse(typeof(Position.RotationEnum), rot);
    if (plane != null) item.Position.Plane = (Position.PlaneEnum)Enum.Parse(typeof(Position.PlaneEnum), plane);
    if (depth != null) item.Position.Depth = (Position.DepthEnum)Enum.Parse(typeof(Position.DepthEnum), depth);
    item.Position.RotationOffset = argNumber("rotation_offset", 0.0);
    item.Position.PlaneOffset = argNumber("plane_offset", 0.0);
    item.Position.DepthOffset = argNumber("depth_offset", 0.0);

    bool inserted = item.Insert();
    result["insert_returned"] = inserted;
    if (inserted)
    {
        result["commit_returned"] = m.CommitChanges();
        result["guid"] = item.Identifier.GUID.ToString();
        result["id"] = item.Identifier.ID;

        // ---- read-back ----
        var cs = item.GetCoordinateSystem();
        var ax = new Vector(cs.AxisX); ax.Normalize(); var ay = new Vector(cs.AxisY); ay.Normalize();
        var az = ax.Cross(ay); az.Normalize();
        result["coordinate_system"] = new Dictionary<string, object>
        {
            { "origin", new[] { cs.Origin.X, cs.Origin.Y, cs.Origin.Z } },
            { "axis_x", new[] { ax.X, ax.Y, ax.Z } }, { "axis_y", new[] { ay.X, ay.Y, ay.Z } }, { "axis_z", new[] { az.X, az.Y, az.Z } },
        };
        var dir = new Vector(end[0] - start[0], end[1] - start[1], end[2] - start[2]);
        double distance = dir.GetLength(); dir.Normalize();
        result["origin_to_start_mm"] = Math.Sqrt(Math.Pow(cs.Origin.X - start[0], 2) + Math.Pow(cs.Origin.Y - start[1], 2) + Math.Pow(cs.Origin.Z - start[2], 2));
        result["axis_x_dot_start_to_end"] = ax.Dot(dir);
        double[] want = null;
        if (args != null && args.TryGetValue("expect_axis_y", out value) && value is IEnumerable ea) want = ea.Cast<object>().Select(x => Convert.ToDouble(x, CultureInfo.InvariantCulture)).ToArray();
        if (want != null && want.Length == 3) result["axis_y_dot_expected"] = ay.X * want[0] + ay.Y * want[1] + ay.Z * want[2];

        var solid = item.GetSolid();
        var points = new List<Point>();
        double volume = 0;
        var faces = solid.GetFaceEnumerator();
        while (faces.MoveNext())
        {
            var face = faces.Current as Face;
            if (face == null) continue;
            var n = new Vector(face.Normal); n.Normalize();
            double bestArea = -1, holes = 0; Point first = null;
            var loops = face.GetLoopEnumerator();
            var areas = new List<double>();
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
            holes = areas.Sum() - bestArea;                                           // every loop but the largest is a hole
            if (first != null) volume += (n.X * first.X + n.Y * first.Y + n.Z * first.Z) * (bestArea - holes) / 3.0;
        }
        var mn = solid.MinimumPoint; var mx = solid.MaximumPoint;
        result["solid_box"] = new[] { mn.X, mn.Y, mn.Z, mx.X, mx.Y, mx.Z };
        result["solid_vertex_count"] = points.Count;
        result["volume_from_faces"] = volume;
        double reported = 0;
        try { result["volume_report_property"] = item.GetReportProperty("VOLUME", ref reported) ? (object)reported : "not-reported"; } catch (Exception ex) { result["volume_report_property"] = "error: " + ex.Message; }
        if (args != null && args.TryGetValue("expect_volume", out value) && value != null) result["volume_ratio_to_expected"] = volume / Convert.ToDouble(value, CultureInfo.InvariantCulture);

        // The shape's own extent along X against the start-to-end distance: does Tekla stretch a shape to the item's length?
        double shapeLengthX = 0;
        var local = new List<double[]>();
        try
        {
            foreach (var v in shape.ShapeFacetedBrep.Vertices) local.Add(new[] { (double)v.X, (double)v.Y, (double)v.Z });
            shapeLengthX = local.Max(c => c[0]) - local.Min(c => c[0]);
        }
        catch (Exception ex) { result["shape_read_error"] = ex.Message; }
        result["shape_extent_x"] = shapeLengthX;
        result["start_to_end_distance"] = distance;
        result["solid_extent_along_axis_x"] = points.Count == 0 ? 0.0 : points.Max(p => (p.X - cs.Origin.X) * ax.X + (p.Y - cs.Origin.Y) * ax.Y + (p.Z - cs.Origin.Z) * ax.Z) - points.Min(p => (p.X - cs.Origin.X) * ax.X + (p.Y - cs.Origin.Y) * ax.Y + (p.Z - cs.Origin.Z) * ax.Z);

        // Catalogue shape moved into the world by Tekla's own coordinate system, against the solid Tekla reports.
        if (local.Count > 0 && points.Count > 0)
        {
            var moved = local.Select(c => new[]
            {
                cs.Origin.X + c[0] * ax.X + c[1] * ay.X + c[2] * az.X,
                cs.Origin.Y + c[0] * ax.Y + c[1] * ay.Y + c[2] * az.Y,
                cs.Origin.Z + c[0] * ax.Z + c[1] * ay.Z + c[2] * az.Z,
            }).ToList();
            Func<Point, double[], double> dist = (p, q) => Math.Sqrt(Math.Pow(p.X - q[0], 2) + Math.Pow(p.Y - q[1], 2) + Math.Pow(p.Z - q[2], 2));
            result["max_solid_vertex_to_moved_shape_mm"] = points.Max(p => moved.Min(q => dist(p, q)));
            result["max_moved_shape_vertex_to_solid_mm"] = moved.Max(q => points.Min(p => dist(p, q)));
        }
        try { result["instance_count_after"] = shape.GetInstanceCount(); } catch (Exception ex) { result["instance_count_error"] = ex.Message; }
    }
}
catch (Exception ex)
{
    result["exception"] = ex.GetType().Name + ": " + ex.Message;
}
finally
{
    try { result["work_plane_restored"] = handler.SetCurrentTransformationPlane(previousPlane); } catch { result["work_plane_restored"] = false; }
}
return result;
