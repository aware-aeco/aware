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
    "sourceId", "marker", "layer",
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
if re.fullmatch(r"AWARE_BAKE_V1:[0-9a-f]{64}", marker) is None:
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
    # Rhino v1 deliberately realizes the only dimensions the host-members
    # contract guarantees: authored width and depth. It does not infer flange,
    # web or wall thickness from a designation.
    hw = width / 2.0
    hd = depth / 2.0
    return [[-hw, -hd], [hw, -hd], [hw, hd], [-hw, hd]]

elements = scene.get("elements") if isinstance(scene.get("elements"), list) else []
by_id = {str(e.get("id")): e for e in elements if isinstance(e, dict)}
plans = []
failed = []

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

        start = [v * scale for v in start_mm]
        direction = Rhino.Geometry.Vector3d(
            axis_mm[0] * scale, axis_mm[1] * scale, axis_mm[2] * scale)
        points = []
        for ox_mm, oy_mm in rectangular_outline(width, depth):
            ox = ox_mm * scale
            oy = oy_mm * scale
            points.append(Rhino.Geometry.Point3d(
                start[0] + xaxis[0] * ox + yaxis[0] * oy,
                start[1] + xaxis[1] * ox + yaxis[1] * oy,
                start[2] + xaxis[2] * ox + yaxis[2] * oy))
        points.append(points[0])
        curve = Rhino.Geometry.PolylineCurve(points)
        if not curve.IsValid or not curve.IsClosed:
            raise ValueError("{}: section outline is not a valid closed curve".format(record_id))
        surface = Rhino.Geometry.Surface.CreateExtrusion(curve, direction)
        if surface is None:
            raise ValueError("{}: Rhino could not extrude the section".format(record_id))
        brep = surface.ToBrep()
        capped = brep.CapPlanarHoles(tol)
        if capped is None:
            raise ValueError("{}: Rhino could not cap the extruded section".format(record_id))
        brep = capped
        if not brep.IsValid or not brep.IsSolid:
            raise ValueError("{}: Rhino did not produce a valid closed solid".format(record_id))

        element_meta = element.get("meta") if isinstance(element.get("meta"), dict) else {}
        profile = str(element_meta.get("profile") or "")
        name = str(element.get("name") or element_meta.get("name") or profile or record_id)
        plans.append({
            "id": record_id, "kind": kind, "brep": brep,
            "name": "{} [{}]".format(name, record_id) if name != record_id else record_id,
            "profile": profile,
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
        and re.fullmatch(r"AWARE_BAKE_V1:[0-9a-f]{64}", str(prior_marker or "")) is not None
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
    undo_serial = doc.BeginUndoRecord("AWARE bake-scene {}".format(str(meta.get("name") or "scene")))

    layer = doc.Layers.FindName(layer_name)
    if layer is None:
        layer = Rhino.DocObjects.Layer()
        layer.Name = layer_name
        layer.Color = System.Drawing.Color.FromArgb(59, 130, 246)
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
        emitted_row = row(active_id, plan["kind"], "emitted")
        emitted_row["nativeGuid"] = object_id.ToString()
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
    doc.Views.Redraw()
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

    failure_rows = []
    cause_seen = False
    for plan in plans:
        cause = plan["id"] == active_id and not cause_seen
        failure_rows.append(row(
            plan["id"], plan["kind"], "failed",
            "materialization-failed" if cause else "batch-aborted",
            str(ex) if cause else "Batch was aborted after another member failed."))
        if cause:
            cause_seen = True
    if not cause_seen:
        failure_rows.insert(0, row(active_id, "scene", "failed", "materialization-failed", str(ex)))
    if not cleanup_ok:
        warnings.append(row(
            "scene", "scene", "warning", "commit-state-uncertain",
            "Rhino could not prove complete cleanup. Re-run the same scene under the same sourceId to reconcile ownership."))
    doc.Views.Redraw()
    return envelope(False, [], failure_rows, warnings, 0, cleanup_ok)
