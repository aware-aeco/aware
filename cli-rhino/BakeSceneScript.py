# AWARE Rhino bake-scene materializer.
#
# This body is embedded in aware-rhino and executed through the same rhinocode
# result-file path as `exec`. It consumes the unchanged canonical millimetre
# scene, converts it into the active document's units, and returns the shared
# host-members receipt.

import math
import re

def row(record_id, kind, status, code=None, message=None):
    value = {"id": record_id, "kind": kind, "status": status}
    if code:
        value["code"] = code
    if message:
        value["message"] = message
    return value

class BakeFailure(Exception):
    """A document-level precondition failure that names its own receipt code.

    Everything raised in the mutation block otherwise lands as
    `materialization-failed`, which is right for geometry that Rhino refused to
    build but wrong for a precondition the document itself denied — a caller
    cannot tell "this shape is bad" from "this document would not give us an
    undo record" and so cannot tell a retry from a fix. Carrying the code keeps
    that distinction.

    It is scene-level by construction: it reports what the document refused,
    never what a record contained, so it must never be attributed to a plan row
    — not even when a record is legitimately named `scene` (ids are not a
    reserved vocabulary, so that collision is reachable).
    """

    def __init__(self, code, message):
        Exception.__init__(self, message)
        self.code = code

scene = args.get("scene")
supported = args.get("supported")
unsupported = args.get("unsupported")
ownership = args.get("ownership")
materialization_hash = str(args.get("materializationHash") or "")
attempt_id = System.Guid.NewGuid().ToString("N")

if not isinstance(scene, dict):
    return {
        "ok": False, "sourceId": "", "sceneHash": "",
        "materializationHash": materialization_hash, "attemptId": attempt_id,
        "created": 0, "retired": 0, "emitted": [],
        "failed": [row("scene", "scene", "failed", "invalid-scene", "scene must be an object")],
        "unsupported": [], "warnings": [], "rolledBack": True,
    }

meta = scene.get("meta") if isinstance(scene.get("meta"), dict) else {}
source_id = str(meta.get("sourceId") or "").strip()
scene_hash = str(meta.get("sceneHash") or "").strip()
units = str(meta.get("units") or "").strip()
supported = supported if isinstance(supported, list) else []
unsupported = unsupported if isinstance(unsupported, list) else []
ownership = ownership if isinstance(ownership, dict) else {}

def envelope(ok, emitted, failed, warnings, retired_count=0, rolled_back=False):
    return {
        "ok": ok,
        "sourceId": source_id,
        "sceneHash": scene_hash,
        "materializationHash": materialization_hash,
        "attemptId": attempt_id,
        "created": len(emitted),
        "retired": retired_count,
        "emitted": emitted,
        "failed": failed,
        "unsupported": unsupported,
        "warnings": warnings,
        "rolledBack": rolled_back,
    }

if units and units.lower() != "mm":
    return envelope(False, [], [
        row("scene", "scene", "failed", "unsupported-units",
            "Rhino bake-scene accepts only a canonical millimetre scene")
    ], [], 0, True)
if not source_id or not scene_hash:
    return envelope(False, [], [
        row("scene", "scene", "failed", "invalid-identity",
            "scene.meta.sourceId and sceneHash are required")
    ], [], 0, True)

required_ownership = [
    "sourceIdKey", "recordIdKey", "sceneHashKey", "markerKey",
    "sourceId", "marker", "layer", "geometryRevision",
]
if any(not str(ownership.get(k) or "") for k in required_ownership):
    return envelope(False, [], [
        row("scene", "scene", "failed", "invalid-ownership",
            "the sidecar did not supply the complete Rhino ownership vocabulary")
    ], [], 0, True)
if str(ownership.get("sourceId")) != source_id:
    return envelope(False, [], [
        row("scene", "scene", "failed", "invalid-ownership",
            "ownership sourceId does not match scene.meta.sourceId")
    ], [], 0, True)

marker = str(ownership["marker"])
geometry_revision = str(ownership["geometryRevision"])
if geometry_revision != "rhino-profile-v4" or re.fullmatch(
        r"AWARE_BAKE_V2:[0-9a-f]{64}", marker) is None:
    return envelope(False, [], [
        row("scene", "scene", "failed", "invalid-ownership",
            "ownership marker is malformed")
    ], [], 0, True)

doc = Rhino.RhinoDoc.ActiveDoc
if doc is None:
    return envelope(False, [], [
        row("scene", "scene", "failed", "host-unavailable", "No active Rhino document is open.")
    ], [], 0, True)

try:
    scale = float(Rhino.RhinoMath.UnitScale(
        Rhino.UnitSystem.Millimeters, doc.ModelUnitSystem))
except Exception:
    scale = float("nan")
if not math.isfinite(scale) or scale <= 0:
    return envelope(False, [], [
        row("scene", "scene", "failed", "host-unit-conversion",
            "The active Rhino document does not expose a usable millimetre conversion.")
    ], [], 0, True)
tol = float(doc.ModelAbsoluteTolerance)
if not math.isfinite(tol) or tol <= 0:
    return envelope(False, [], [
        row("scene", "scene", "failed", "host-tolerance",
            "The active Rhino document does not expose a usable absolute tolerance.")
    ], [], 0, True)

def number(value):
    if isinstance(value, bool):
        return None
    try:
        result = float(value)
        return result if math.isfinite(result) else None
    except Exception:
        return None

def vec3(value):
    if not isinstance(value, list) or len(value) < 3:
        return None
    result = [number(value[i]) for i in range(3)]
    return result if all(v is not None for v in result) else None

def vlen(v):
    return math.sqrt(sum(x * x for x in v))

def vdot(a, b):
    return sum(a[i] * b[i] for i in range(3))

def vcross(a, b):
    return [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]

def vnorm(v):
    length = vlen(v)
    return [x / length for x in v]

def vrot(v, axis, angle):
    c = math.cos(angle)
    s = math.sin(angle)
    cross = vcross(axis, v)
    dot = vdot(axis, v) * (1.0 - c)
    return [v[i] * c + cross[i] * s + axis[i] * dot for i in range(3)]

def rectangular_outline(width, depth):
    hw = width / 2.0
    hd = depth / 2.0
    return [[-hw, -hd], [hw, -hd], [hw, hd], [-hw, hd]]

def profile_components(profile):
    # The sidecar already decoded and normalized the canonical xsection. The
    # live script consumes that plan; it never parses a designation, accepts an
    # alias, or invents a thickness.
    shape = str(profile["shape"])
    width = float(profile["w"])
    depth = float(profile["d"])
    dimensions = profile["dimensions"]
    hw = width / 2.0
    hd = depth / 2.0

    if shape == "i":
        tf = float(dimensions["tf"])
        tw = float(dimensions["tw"])
        return [([
                [-hw, -hd], [hw, -hd], [hw, -hd + tf], [tw / 2.0, -hd + tf],
                [tw / 2.0, hd - tf], [hw, hd - tf], [hw, hd], [-hw, hd],
                [-hw, hd - tf], [-tw / 2.0, hd - tf],
                [-tw / 2.0, -hd + tf], [-hw, -hd + tf],
            ], None)]
    if shape == "channel":
        tf = float(dimensions["tf"])
        tw = float(dimensions["tw"])
        return [([
                [-hw, -hd], [hw, -hd], [hw, -hd + tf],
                [-hw + tw, -hd + tf], [-hw + tw, hd - tf],
                [hw, hd - tf], [hw, hd], [-hw, hd],
            ], None)]
    if shape == "angle":
        thickness = float(dimensions["t"])
        return [([
                [-hw, -hd], [hw, -hd], [hw, -hd + thickness],
                [-hw + thickness, -hd + thickness],
                [-hw + thickness, hd], [-hw, hd],
            ], None)]
    if shape == "rhs":
        thickness = float(dimensions["t"])
        return [(rectangular_outline(width, depth), [
                [-hw + thickness, -hd + thickness],
                [hw - thickness, -hd + thickness],
                [hw - thickness, hd - thickness],
                [-hw + thickness, hd - thickness],
            ])]
    if shape == "double-angle":
        return [(outline, None) for outline in profile["components"]]
    return [(rectangular_outline(width, depth), None)]

elements = scene.get("elements") if isinstance(scene.get("elements"), list) else []
by_id = {str(e.get("id")): e for e in elements if isinstance(e, dict)}
plans = []
failed = []

def local_polygon(outline):
    points = [
        Rhino.Geometry.Point3d(point[0] * scale, point[1] * scale, 0.0)
        for point in outline
    ]
    points.append(points[0])
    curve = Rhino.Geometry.PolylineCurve(points)
    return curve

def normalize_outward(brep):
    if brep.SolidOrientation == Rhino.Geometry.BrepSolidOrientation.Inward:
        brep.Flip()

def validate_brep(
        record_id, brep, profile, frame, axis, length, context,
        measure_volume):
    if brep is None or not brep.IsValid or not brep.IsSolid:
        raise ValueError(
            "{}: {} is not a valid closed solid".format(record_id, context))
    if brep.SolidOrientation != Rhino.Geometry.BrepSolidOrientation.Outward:
        raise ValueError(
            "{}: {} does not have outward solid orientation".format(record_id, context))

    expected_profiles = int(profile["profileCount"])
    expected_components = int(profile["componentCount"])
    cap_loops = []
    for face in brep.Faces:
        if not face.IsPlanar(tol):
            continue
        u = (face.Domain(0).Min + face.Domain(0).Max) / 2.0
        v = (face.Domain(1).Min + face.Domain(1).Max) / 2.0
        normal = face.NormalAt(u, v)
        if normal.Unitize() and abs(abs(
                normal.X * axis.X + normal.Y * axis.Y + normal.Z * axis.Z) - 1.0) <= 1.0e-7:
            cap_loops.append(face.Loops.Count)
    if (len(cap_loops) != 2 * expected_components
            or any(count != expected_profiles for count in cap_loops)):
        raise ValueError(
            "{}: {} requires {} component cap(s) with {} profile loop(s)".format(
                record_id, context, 2 * expected_components, expected_profiles))

    bbox = brep.GetBoundingBox(frame)
    width = float(profile["w"]) * scale
    depth = float(profile["d"]) * scale
    expected_area = float(profile["area"]) * scale * scale
    expected_perimeter = float(profile["perimeter"]) * scale
    expected_volume = expected_area * length
    linear_e = max(tol, 1.0e-9 * max(scale, width, depth, length))
    expected_bounds = [
        (-width / 2.0, bbox.Min.X), (width / 2.0, bbox.Max.X),
        (-depth / 2.0, bbox.Min.Y), (depth / 2.0, bbox.Max.Y),
        (0.0, bbox.Min.Z), (length, bbox.Max.Z),
    ]
    if any(abs(expected - actual) > linear_e for expected, actual in expected_bounds):
        raise ValueError(
            "{}: {} endpoints or section envelope differ from the normalized plan".format(
                record_id, context))

    diagnostics = {
        "revision": geometry_revision,
        "shape": str(profile["shape"]),
        "dimensions": profile["dimensions"],
        "profileCount": expected_profiles,
        "componentCount": expected_components,
        "capCount": len(cap_loops),
    }
    # Mass properties are materially more expensive than topology and envelope
    # checks. The rigid transform cannot change volume, so local/transformed
    # preflight uses the exact analytic profile while every authoritative Brep
    # read back from RhinoDoc is measured independently.
    measured_volume = None
    volume_basis = None
    if measure_volume:
        mass = Rhino.Geometry.VolumeMassProperties.Compute(brep)
        if mass is None:
            raise ValueError(
                "{}: {} volume could not be measured".format(record_id, context))
        measured_volume = abs(mass.Volume)
        volume_basis = "document-readback"
    if measured_volume is not None:
        area_tolerance = max(
            expected_perimeter * linear_e + math.pi * linear_e * linear_e,
            expected_area * 1.0e-9)
        volume_tolerance = max(
            expected_area * linear_e + length * area_tolerance
            + linear_e * area_tolerance,
            expected_volume * 1.0e-9)
        if abs(measured_volume - expected_volume) > volume_tolerance:
            raise ValueError(
                "{}: {} volume differs from the normalized profile".format(
                    record_id, context))
        diagnostics["volume"] = measured_volume
        diagnostics["volumeBasis"] = volume_basis
    return diagnostics

# Full live-host preflight: every Brep exists in memory before BeginUndoRecord
# or a layer/object table mutation.
for supported_row in supported:
    record_id = str(supported_row.get("id") if isinstance(supported_row, dict) else supported_row)
    kind = str(supported_row.get("kind") if isinstance(supported_row, dict) else "member") or "member"
    try:
        element = by_id.get(record_id)
        if element is None:
            raise ValueError("{}: scene element is missing".format(record_id))
        start_mm = vec3(element.get("from"))
        end_mm = vec3(element.get("to"))
        if start_mm is None or end_mm is None:
            raise ValueError("{}: member requires finite from/to points".format(record_id))
        axis_mm = [end_mm[i] - start_mm[i] for i in range(3)]
        length_mm = vlen(axis_mm)
        if length_mm * scale <= tol:
            raise ValueError("{}: member axis is at or below Rhino document tolerance".format(record_id))
        section = element.get("section") if isinstance(element.get("section"), dict) else {}
        width = number(section.get("w"))
        depth = number(section.get("d"))
        if width is None or depth is None or width <= 0 or depth <= 0:
            raise ValueError("{}: member requires positive section {{w,d}}; Rhino never invents a section".format(record_id))
        if width * scale <= tol or depth * scale <= tol:
            raise ValueError("{}: member section is at or below Rhino document tolerance".format(record_id))
        profile_plan = supported_row.get("profile") if isinstance(supported_row, dict) else None
        if not isinstance(profile_plan, dict) or str(
                profile_plan.get("revision") or "") != geometry_revision:
            raise ValueError(
                "{}: normalized xsection plan is missing or has the wrong revision".format(record_id))
        residuals = profile_plan.get("residuals")
        if not isinstance(residuals, list) or any(
                number(value) is None or number(value) * scale <= tol for value in residuals):
            raise ValueError(
                "{}: xsection edge, thickness, or void is at or below Rhino document tolerance".format(record_id))

        zaxis = vnorm(axis_mm)
        dz = vdot([0.0, 0.0, 1.0], zaxis)
        projected_up = [-zaxis[0] * dz, -zaxis[1] * dz, 1.0 - zaxis[2] * dz]
        yaxis = vnorm(projected_up) if vlen(projected_up) > 1.0e-9 else [0.0, 1.0, 0.0]
        xaxis = vcross(yaxis, zaxis)
        roll = number(element.get("rot")) or 0.0
        if roll:
            radians = roll * math.pi / 180.0
            xaxis = vrot(xaxis, zaxis, radians)
            yaxis = vrot(yaxis, zaxis, radians)

        length = length_mm * scale
        component_curves = []
        if str(profile_plan["shape"]) == "chs":
            dimensions = profile_plan["dimensions"]
            radius = float(dimensions["od"]) * scale / 2.0
            thickness = float(dimensions["t"]) * scale
            outer_curve = Rhino.Geometry.Circle(
                Rhino.Geometry.Plane.WorldXY, radius).ToNurbsCurve()
            inner_curve = Rhino.Geometry.Circle(
                Rhino.Geometry.Plane.WorldXY, radius - thickness).ToNurbsCurve()
            component_curves.append((outer_curve, inner_curve))
        else:
            for outer_outline, inner_outline in profile_components(profile_plan):
                component_curves.append((
                    local_polygon(outer_outline),
                    local_polygon(inner_outline) if inner_outline is not None else None))
        if len(component_curves) != int(profile_plan["componentCount"]):
            raise ValueError("{}: normalized component count is wrong".format(record_id))

        brep = Rhino.Geometry.Brep()
        for outer_curve, inner_curve in component_curves:
            if (not outer_curve.IsValid or not outer_curve.IsClosed
                    or (inner_curve is not None and (
                        not inner_curve.IsValid or not inner_curve.IsClosed))):
                raise ValueError(
                    "{}: section outline is not a valid closed curve".format(record_id))
            extrusion = Rhino.Geometry.Extrusion()
            if not extrusion.SetPathAndUp(
                    Rhino.Geometry.Point3d.Origin,
                    Rhino.Geometry.Point3d(0.0, 0.0, length),
                    Rhino.Geometry.Vector3d.YAxis):
                raise ValueError("{}: Rhino refused the local extrusion path".format(record_id))
            if not extrusion.SetOuterProfile(outer_curve, True):
                raise ValueError("{}: Rhino refused the outer section profile".format(record_id))
            if inner_curve is not None and not extrusion.AddInnerProfile(inner_curve):
                raise ValueError("{}: Rhino refused the inner section profile".format(record_id))
            if extrusion.ProfileCount != int(profile_plan["profileCount"]):
                raise ValueError("{}: Rhino extrusion profile count is wrong".format(record_id))
            component = extrusion.ToBrep()
            if component is None:
                raise ValueError("{}: Rhino could not extrude the section".format(record_id))
            normalize_outward(component)
            brep.Append(component)
        validate_brep(
            record_id, brep, profile_plan, Rhino.Geometry.Plane.WorldXY,
            Rhino.Geometry.Vector3d.ZAxis, length, "local extrusion", False)

        start = [v * scale for v in start_mm]
        origin = Rhino.Geometry.Point3d(start[0], start[1], start[2])
        member_plane = Rhino.Geometry.Plane(
            origin,
            Rhino.Geometry.Vector3d(xaxis[0], xaxis[1], xaxis[2]),
            Rhino.Geometry.Vector3d(yaxis[0], yaxis[1], yaxis[2]))
        transform = Rhino.Geometry.Transform.PlaneToPlane(
            Rhino.Geometry.Plane.WorldXY, member_plane)
        if not brep.Transform(transform):
            raise ValueError("{}: Rhino could not transform the section into place".format(record_id))
        normalize_outward(brep)
        world_axis = Rhino.Geometry.Vector3d(zaxis[0], zaxis[1], zaxis[2])
        diagnostics = validate_brep(
            record_id, brep, profile_plan, member_plane, world_axis,
            length, "transformed extrusion", False)

        element_meta = element.get("meta") if isinstance(element.get("meta"), dict) else {}
        profile = str(element_meta.get("profile") or "")
        name = str(element.get("name") or element_meta.get("name") or profile or record_id)
        plans.append({
            "id": record_id, "kind": kind, "brep": brep,
            "name": "{} [{}]".format(name, record_id) if name != record_id else record_id,
            "profile": profile, "profilePlan": profile_plan,
            "frame": member_plane, "axis": world_axis, "length": length,
            "diagnostics": diagnostics,
        })
    except Exception as ex:
        failed.append(row(record_id, kind, "failed", "invalid-geometry", str(ex)))

if failed:
    failed_ids = set(r["id"] for r in failed)
    for supported_row in supported:
        record_id = str(supported_row.get("id") if isinstance(supported_row, dict) else supported_row)
        kind = str(supported_row.get("kind") if isinstance(supported_row, dict) else "member") or "member"
        if record_id not in failed_ids:
            failed.append(row(record_id, kind, "failed", "batch-aborted",
                              "Batch was aborted because another member failed preflight."))
    return envelope(False, [], failed, [], 0, True)

k_source = str(ownership["sourceIdKey"])
k_record = str(ownership["recordIdKey"])
k_scene = str(ownership["sceneHashKey"])
k_marker = str(ownership["markerKey"])
layer_name = str(ownership["layer"])

def owned(obj):
    attrs = obj.Attributes
    source = attrs.GetUserString(k_source)
    record_id = attrs.GetUserString(k_record)
    prior_hash = attrs.GetUserString(k_scene)
    prior_marker = attrs.GetUserString(k_marker)
    return (
        source == source_id and bool(record_id) and bool(prior_hash)
        and re.fullmatch(r"AWARE_BAKE_V[12]:[0-9a-f]{64}", str(prior_marker or "")) is not None
    )

prior = [obj for obj in doc.Objects if owned(obj)]
staged_ids = []
retired_serials = []
emitted = []
warnings = []
undo_serial = 0
layer_index = -1
layer_created = False
active_id = "scene"

try:
    # Rhino answers 0 when it will not open a record — undo recording is off, or
    # a record is already open and this one would nest. Every mutation below is
    # only reversible because it belongs to this record, so a zero serial has to
    # abort BEFORE the first layer read, not merely skip EndUndoRecord.
    undo_serial = doc.BeginUndoRecord("AWARE bake-scene {}".format(str(meta.get("name") or "scene")))
    if not undo_serial:
        raise BakeFailure(
            "undo-unavailable",
            "Rhino refused to begin the bake undo record, so the batch would not be undoable as one step")

    layer = doc.Layers.FindName(layer_name)
    if layer is None:
        layer = Rhino.DocObjects.Layer()
        layer.Name = layer_name
        layer.Color = System.Drawing.Color.FromArgb(164, 169, 172)
        layer_index = doc.Layers.Add(layer)
        if layer_index < 0:
            raise RuntimeError("Rhino refused to create the AWARE layer")
        layer_created = True
    else:
        layer_index = layer.Index

    for plan in plans:
        active_id = plan["id"]
        attrs = doc.CreateDefaultAttributes()
        attrs.LayerIndex = layer_index
        attrs.ColorSource = Rhino.DocObjects.ObjectColorSource.ColorFromLayer
        attrs.Name = plan["name"]
        if not attrs.SetUserString(k_source, source_id):
            raise RuntimeError("{}: source ownership stamp failed".format(active_id))
        if not attrs.SetUserString(k_record, active_id):
            raise RuntimeError("{}: record ownership stamp failed".format(active_id))
        if not attrs.SetUserString(k_scene, scene_hash):
            raise RuntimeError("{}: scene ownership stamp failed".format(active_id))
        if not attrs.SetUserString(k_marker, marker):
            raise RuntimeError("{}: materialization ownership stamp failed".format(active_id))
        if plan["profile"]:
            attrs.SetUserString("AWARE.PROFILE", plan["profile"])
        object_id = doc.Objects.AddBrep(plan["brep"], attrs)
        if object_id == System.Guid.Empty:
            raise RuntimeError("{}: Rhino refused to add the Brep".format(active_id))
        staged_ids.append(object_id)

        created = doc.Objects.FindId(object_id)
        if created is None or created.IsDeleted:
            raise RuntimeError("{}: created object read-back failed".format(active_id))
        created_attrs = created.Attributes
        if (created_attrs.GetUserString(k_source) != source_id
                or created_attrs.GetUserString(k_record) != active_id
                or created_attrs.GetUserString(k_scene) != scene_hash
                or created_attrs.GetUserString(k_marker) != marker):
            raise RuntimeError("{}: ownership read-back differs from the request".format(active_id))
        readback = created.Geometry
        if not isinstance(readback, Rhino.Geometry.Brep):
            raise RuntimeError("{}: created object is not a Rhino Brep".format(active_id))
        readback_diagnostics = validate_brep(
            active_id, readback, plan["profilePlan"], plan["frame"],
            plan["axis"], plan["length"], "document read-back",
            True)
        emitted_row = row(active_id, plan["kind"], "emitted")
        emitted_row["nativeGuid"] = object_id.ToString()
        emitted_row["geometry"] = readback_diagnostics
        emitted.append(emitted_row)

    # Retire only after the entire replacement set exists and its ownership
    # stamps have been read back.
    for old in prior:
        active_id = str(old.Attributes.GetUserString(k_record) or "scene")
        serial = old.RuntimeSerialNumber
        if not doc.Objects.Delete(old, True):
            raise RuntimeError("{}: prior source-owned object could not be retired".format(active_id))
        retired_serials.append(serial)

    for old in prior:
        if not old.IsDeleted:
            raise RuntimeError("a prior source-owned object remains after retirement")

    if undo_serial:
        if not doc.EndUndoRecord(undo_serial):
            raise RuntimeError("Rhino could not close the bake undo record")
        undo_serial = 0

    # A newly baked model can be far from Rhino's untouched startup camera.
    # Frame only the active model view: every other model/layout/detail camera
    # belongs to the user and must remain untouched. Camera failure is non-fatal
    # because the committed geometry remains valid.
    frame_failure = None
    try:
        active_view = doc.Views.ActiveView
        if active_view is None:
            frame_failure = "there is no active model view"
        elif isinstance(active_view, Rhino.Display.RhinoPageView):
            frame_failure = "the active view is a layout"
        elif not active_view.ActiveViewport.ZoomExtents():
            frame_failure = "{} refused Zoom Extents".format(
                str(active_view.ActiveViewport.Name or "the active model view"))
        doc.Views.Redraw()
    except Exception as ex:
        frame_failure = str(ex)
    if frame_failure:
        warnings.append(row(
            "scene", "scene", "warning", "view-frame-failed",
            "The model was baked, but Rhino could not frame it automatically: {}".format(frame_failure)))
    return envelope(True, emitted, [], warnings, len(retired_serials), False)
except Exception as ex:
    cleanup_ok = True
    for object_id in reversed(staged_ids):
        try:
            current = doc.Objects.FindId(object_id)
            if current is not None and not current.IsDeleted and not doc.Objects.Delete(object_id, True):
                cleanup_ok = False
        except Exception:
            cleanup_ok = False
    for serial in reversed(retired_serials):
        try:
            if not doc.Objects.Undelete(serial):
                cleanup_ok = False
        except Exception:
            cleanup_ok = False
    if layer_created:
        try:
            if not doc.Layers.Delete(layer_index, True):
                cleanup_ok = False
        except Exception:
            cleanup_ok = False
    if undo_serial:
        try:
            if not doc.EndUndoRecord(undo_serial):
                cleanup_ok = False
        except Exception:
            cleanup_ok = False
        undo_serial = 0

    # Verify both sides of the restoration before claiming rollback.
    try:
        for object_id in staged_ids:
            current = doc.Objects.FindId(object_id)
            if current is not None and not current.IsDeleted:
                cleanup_ok = False
        for old in prior:
            if old.IsDeleted:
                cleanup_ok = False
    except Exception:
        cleanup_ok = False

    # A BakeFailure is the document's refusal, so it never matches a plan even
    # if a member is named `scene`; it always lands on its own synthetic row.
    scene_level = isinstance(ex, BakeFailure)
    cause_code = ex.code if scene_level else "materialization-failed"
    failure_rows = []
    cause_seen = False
    for plan in plans:
        cause = not scene_level and plan["id"] == active_id and not cause_seen
        failure_rows.append(row(
            plan["id"], plan["kind"], "failed",
            cause_code if cause else "batch-aborted",
            str(ex) if cause else "Batch was aborted after another member failed."))
        if cause:
            cause_seen = True
    if not cause_seen:
        failure_rows.insert(0, row(
            "scene" if scene_level else active_id, "scene", "failed", cause_code, str(ex)))
    if not cleanup_ok:
        warnings.append(row(
            "scene", "scene", "warning", "commit-state-uncertain",
            "Rhino could not prove complete cleanup. Re-run the same scene under the same sourceId to reconcile ownership."))
    doc.Views.Redraw()
    return envelope(False, [], failure_rows, warnings, 0, cleanup_ok)
