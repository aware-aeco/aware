"""Contained OCP worker for development geometry cuts."""

import base64
import io
import json
import math
import os
import sys
from fractions import Fraction

try:
    from OCP.Bnd import Bnd_Box
    from OCP.BRep import BRep_Builder, BRep_Tool
    from OCP.BRepAdaptor import BRepAdaptor_Curve, BRepAdaptor_Surface
    from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
    from OCP.BRepBndLib import BRepBndLib
    from OCP.BRepBuilderAPI import BRepBuilderAPI_Copy, BRepBuilderAPI_Transform
    from OCP.BRepCheck import BRepCheck_Analyzer
    from OCP.BRepGProp import BRepGProp
    from OCP.BRepMesh import BRepMesh_IncrementalMesh
    from OCP.BRepTools import BRepTools, BRepTools_WireExplorer
    from OCP.GeomAbs import GeomAbs_Line, GeomAbs_Plane
    from OCP.GProp import GProp_GProps
    from OCP.Precision import Precision
    from OCP.TopAbs import (TopAbs_EDGE, TopAbs_FACE, TopAbs_REVERSED, TopAbs_SHELL,
                            TopAbs_SOLID, TopAbs_VERTEX, TopAbs_WIRE)
    from OCP.TopExp import TopExp, TopExp_Explorer
    from OCP.TopLoc import TopLoc_Location
    from OCP.TopoDS import TopoDS, TopoDS_Shape
    from OCP.TopTools import TopTools_FormatVersion
    from OCP.collections import IndexedMap_TopoDS_Shape_TopTools_ShapeMapHasher, List_TopoDS_Shape
    from OCP.gp import gp_Trsf, gp_Vec
except ImportError as error:
    print(error, file=sys.stderr)
    print(json.dumps({'status': 'rejected', 'code': 'occt_import_failed', 'relation': None, 'result': None}))
    raise SystemExit(0)


POSITION = 1e-5
EXTENT = 2 ** 14
WORK = 4_000_000


class Reject(Exception):
    def __init__(self, code, relation=None):
        self.code, self.relation = code, relation


def check(condition):
    if not condition:
        raise Reject('brep_validation_failed')


def mapped(shape, kind):
    entries = IndexedMap_TopoDS_Shape_TopTools_ShapeMapHasher()
    TopExp.MapShapes_s(shape, kind, entries)
    return [entries.FindKey(index) for index in range(1, entries.Extent() + 1)]


def coordinates(point):
    return point.X(), point.Y(), point.Z()


def bounds(shape):
    box = Bnd_Box()
    BRepBndLib.AddOptimal_s(shape, box, False, True)
    check(not box.IsVoid())
    return coordinates(box.CornerMin()) + coordinates(box.CornerMax())


def vector(a, b):
    return tuple(y - x for x, y in zip(a, b))


def cross(a, b):
    return tuple(a[(i + 1) % 3] * b[(i + 2) % 3] - a[(i + 2) % 3] * b[(i + 1) % 3] for i in range(3))


def dot(a, b):
    return math.fsum(x * y for x, y in zip(a, b))


def face_loops(shape):
    loops = []
    for subshape in mapped(shape, TopAbs_FACE):
        face = TopoDS.Face(subshape)
        surface = BRepAdaptor_Surface(face)
        check(surface.GetType() == GeomAbs_Plane and BRep_Tool.Tolerance_s(face) <= POSITION)
        wire = BRepTools.OuterWire_s(face)
        check(not wire.IsNull() and len(mapped(face, TopAbs_WIRE)) == 1)
        visit = BRepTools_WireExplorer(wire, face)
        points = []
        while visit.More():
            edge, vertex = visit.Current(), visit.CurrentVertex()
            check(BRepAdaptor_Curve(edge).GetType() == GeomAbs_Line)
            check(BRep_Tool.Tolerance_s(edge) <= POSITION and BRep_Tool.Tolerance_s(vertex) <= POSITION)
            points.append(coordinates(BRep_Tool.Pnt_s(vertex)))
            visit.Next()
        check(len(points) >= 3 and len(points) == len(mapped(wire, TopAbs_EDGE)))
        normal = coordinates(surface.Plane().Axis().Direction())
        if face.Orientation() == TopAbs_REVERSED:
            normal = tuple(-v for v in normal)
        check(all(math.isfinite(v) for p in points for v in p))
        loops.append((face, points, normal))
    return loops


def check_solid(shape, empty=False):
    solids = mapped(shape, TopAbs_SOLID)
    if empty and not solids:
        check(not any(mapped(shape, kind) for kind in (TopAbs_SHELL, TopAbs_FACE, TopAbs_WIRE, TopAbs_EDGE, TopAbs_VERTEX)))
        return False
    check(len(solids) == 1 and BRepCheck_Analyzer(shape, True).IsValid())
    shells = mapped(solids[0], TopAbs_SHELL)
    check(bool(shells) and (empty or len(shells) == 1))
    signed = [signed_volume(shell) for shell in shells]
    check(all(math.isfinite(value) and value != 0 for value in signed)
          and sum(value > 0 for value in signed) == 1 and math.fsum(signed) > 0)
    for kind in (TopAbs_SHELL, TopAbs_FACE, TopAbs_WIRE, TopAbs_EDGE, TopAbs_VERTEX):
        check(len(mapped(shape, kind)) == len(mapped(solids[0], kind)))
    return True


def signed_volume(shape):
    props = GProp_GProps()
    BRepGProp.VolumeProperties_s(shape, props, 1e-12, True, False)
    return props.Mass()


def read_operand(operand):
    shape = TopoDS_Shape()
    payload = io.BytesIO(base64.standard_b64decode(operand['brep_b64']))
    BRepTools.Read_s(shape, payload, BRep_Builder())
    check(not shape.IsNull() and check_solid(shape))
    profile = operand['profile']
    xy = [tuple(map(Fraction, p)) for p in profile['xy_mm']]
    z = tuple(map(Fraction, profile['z_mm']))
    n = len(xy)
    if n * n > WORK:
        raise Reject('budget_exceeded')
    offset = operand['offset_mm']
    points = [tuple(float(v - Fraction(o)) for v, o in zip((x, y, level), offset))
              for level in z for x, y in xy]
    match_prism(shape, points, n, POSITION)
    box = bounds(shape)
    if max(map(abs, box)) > EXTENT:
        raise Reject('budget_exceeded')
    return shape, xy, z, points, box


def match_prism(shape, points, n, allowance):
    expected = {frozenset(range(n)), frozenset(range(n, 2 * n))}
    for i in range(n):
        j = (i + 1) % n
        expected.add(frozenset((i, j, i + n, j + n)))
    found = {}
    for _, loop, normal in face_loops(shape):
        indices = []
        for point in loop:
            matches = [i for i, target in enumerate(points) if max(map(abs, vector(point, target))) <= allowance]
            check(len(matches) == 1)
            index = matches[0]
            check(index not in found or found[index] == point)
            found[index] = point
            indices.append(index)
        key = frozenset(indices)
        check(len(indices) == len(key) and key in expected)
        expected.remove(key)
        if all(i < n for i in indices):
            check(normal == (0., 0., -1.) and len({p[2] for p in loop}) == 1)
        elif all(i >= n for i in indices):
            check(normal == (0., 0., 1.) and len({p[2] for p in loop}) == 1)
        else:
            check(normal[2] == 0. and all(found[i][:2] == found[i + n][:2] for i in indices if i < n))
    check(not expected and len(found) == 2 * n)
    return [found[i] for i in range(2 * n)]


def translate(shape, delta):
    copy = BRepBuilderAPI_Copy(shape, True, False).Shape()
    check(not copy.IsPartner(shape))
    trsf = gp_Trsf()
    trsf.SetTranslation(gp_Vec(*delta))
    return BRepBuilderAPI_Transform(copy, trsf, False, False).Shape()


def orient(a, b, p):
    return (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0])


def check_coincidence(xy_a, xy_b, z_a, z_b, a, b):
    n, m = len(xy_a), len(xy_b)
    for i in range(2):
        for j in range(2):
            if z_a[i] == z_b[j] and a[i * n][2] != b[j * m][2]:
                raise Reject('coincidence_mismatch', 'base_cap' if i == 0 else 'top_cap')
    encoded = [[tuple(map(Fraction, p[:2])) for p in group] for group in (a[:n], b[:m])]
    for left, right, u, v in [(xy_a, xy_b, *encoded), (xy_b, xy_a, *reversed(encoded))]:
        for i, point in enumerate(left):
            j = (i + 1) % len(left)
            for k, other in enumerate(right):
                if orient(point, left[j], other) == 0 and orient(u[i], u[j], v[k]) != 0:
                    raise Reject('coincidence_mismatch', 'side_incidence')
                if any(point[axis] == other[axis] and u[i][axis] != v[k][axis] for axis in (0, 1)):
                    raise Reject('coincidence_mismatch', 'vertex_alignment')


def prepare(request):
    a, b = request['a'], request['b']
    if (len(a['profile']['xy_mm']) + len(b['profile']['xy_mm'])) ** 2 > WORK:
        raise Reject('budget_exceeded')
    left, xy_a, z_a, p_a, box_a = read_operand(a)
    right, xy_b, z_b, p_b, box_b = read_operand(b)
    offset_a, offset_b = a['offset_mm'], b['offset_mm']
    delta = vector(offset_a, offset_b)
    margin = [(abs(u) + abs(v)) * 8 * sys.float_info.epsilon for u, v in zip(offset_a, offset_b)]
    relative_b = [bound + delta[i % 3] for i, bound in enumerate(box_b)]
    disjoint = any(box_a[i + 3] + margin[i] < relative_b[i] or relative_b[i + 3] + margin[i] < box_a[i]
                   for i in range(3))
    if disjoint:
        midpoint = [(box_a[i] + box_a[i + 3]) / 2 for i in range(3)]
    else:
        midpoint = [(min(box_a[i], relative_b[i]) + max(box_a[i + 3], relative_b[i + 3])) / 2 for i in range(3)]
    shift_a = [-v for v in midpoint]
    shift_b = vector(midpoint, delta)
    centred_left = translate(left, shift_a)
    centred_right = None if disjoint else translate(right, shift_b)
    moved_a = match_prism(centred_left, [tuple(v + shift_a[i] for i, v in enumerate(p)) for p in p_a], len(xy_a), POSITION)
    if max(map(abs, bounds(centred_left))) > EXTENT:
        raise Reject('budget_exceeded')
    if centred_right is not None:
        if max(map(abs, bounds(centred_right))) > EXTENT:
            raise Reject('budget_exceeded')
        moved_b = match_prism(centred_right, [tuple(v + shift_b[i] for i, v in enumerate(p)) for p in p_b], len(xy_b), POSITION)
        check_coincidence(xy_a, xy_b, z_a, z_b, moved_a, moved_b)
    offset = [a + b for a, b in zip(offset_a, midpoint)]
    if any(not math.isfinite(value) or abs((value - source) - centre) > POSITION
           for value, source, centre in zip(offset, offset_a, midpoint)):
        raise Reject('budget_exceeded')
    return centred_left, centred_right, offset


def configured_cut():
    operation = BRepAlgoAPI_Cut()
    operation.SetFuzzyValue(0)
    operation.SetRunParallel(False)
    operation.SetNonDestructive(True)
    check(operation.FuzzyValue() == Precision.Confusion_s() == 1e-7)
    return operation


def cut_once(a, b):
    if b is None:
        return a
    operation = configured_cut()
    arguments, tools = List_TopoDS_Shape(), List_TopoDS_Shape()
    arguments.Append(a)
    tools.Append(b)
    operation.SetArguments(arguments)
    operation.SetTools(tools)
    operation.Build()
    check(operation.IsDone())
    shape = operation.Shape()
    check(not shape.IsNull())
    return shape


def indexed(faces):
    ids, points, triangles = {}, [], []
    for locations, cells in faces:
        remap = []
        for point in locations:
            if point not in ids:
                ids[point] = len(points)
                points.append(point)
            remap.append(ids[point])
        triangles.extend(tuple(remap[i] for i in cell) for cell in cells)
    check(len(triangles) <= 100_000)
    return {'positions_mm': points, 'triangles': triangles}


def boundary_faces(loops):
    for _, points, normal in loops:
        cells = []
        for i in range(1, len(points) - 1):
            signed = dot(cross(vector(points[0], points[i]), vector(points[0], points[i + 1])), normal)
            if signed:
                cells.append((0, i, i + 1) if signed > 0 else (0, i + 1, i))
        yield points, cells


def tessellated_faces(shape, chord):
    BRepTools.Clean_s(shape)
    operation = BRepMesh_IncrementalMesh(shape, chord, False, 0.25, False)
    check(operation.IsDone())
    explore = TopExp_Explorer(shape, TopAbs_FACE)
    while explore.More():
        face = TopoDS.Face(explore.Current())
        location = TopLoc_Location()
        mesh = BRep_Tool.Triangulation_s(face, location)
        check(mesh is not None)
        transform = location.Transformation()
        points = [coordinates(mesh.Node(i).Transformed(transform)) for i in range(1, mesh.NbNodes() + 1)]
        cells = []
        for i in range(1, mesh.NbTriangles() + 1):
            a, b, c = mesh.Triangle(i).Get()
            cells.append((a - 1, c - 1, b - 1) if face.Orientation() == TopAbs_REVERSED else (a - 1, b - 1, c - 1))
        yield points, cells
        explore.Next()


def geometry(request, preflight):
    a, b, offset = prepare(request)
    if preflight:
        return {'status': 'admitted'}
    shape = cut_once(a, b)
    if not check_solid(shape, empty=True):
        empty = indexed([])
        result = {'offset_mm': request['a']['offset_mm'], 'brep_b64': None, 'mesh': empty, 'volume_mm3': 0}
        return {'status': 'ok', 'code': None, 'relation': None, 'result': result, 'native_boundary': empty}
    loops = face_loops(shape)
    native = indexed(boundary_faces(loops))
    volume = signed_volume(shape)
    check(math.isfinite(volume) and volume > 0)
    stream = io.BytesIO()
    BRepTools.Write_s(shape, stream, False, False, TopTools_FormatVersion.TopTools_FormatVersion_CURRENT)
    result = {'offset_mm': offset, 'brep_b64': base64.standard_b64encode(stream.getvalue()).decode('ascii'),
              'mesh': indexed(tessellated_faces(shape, request['chord_height_mm'])), 'volume_mm3': volume}
    return {'status': 'ok', 'code': None, 'relation': None, 'result': result, 'native_boundary': native}


def main():
    if sys.argv[1:] == ['--native-probe']:
        operation = configured_cut()
        print(json.dumps({'requested_fuzz_mm': 0, 'native_fuzzy_floor_mm': operation.FuzzyValue(),
                          'parallel': operation.RunParallel(), 'non_destructive': operation.NonDestructive()}))
        return
    # Native OCCT diagnostics may write to descriptor 1. Save a separate
    # protocol stream and route native output into the supervised stderr pipe.
    with os.fdopen(os.dup(sys.stdout.fileno()), 'w', encoding='utf-8') as protocol:
        os.dup2(sys.stderr.fileno(), sys.stdout.fileno())
        try:
            request = json.load(sys.stdin)
            reply = geometry(request, sys.argv[1:] == ['--preflight'])
        except Reject as error:
            reply = {'status': 'rejected', 'code': error.code, 'relation': error.relation, 'result': None}
        except (ValueError, RuntimeError, OverflowError):
            reply = {'status': 'rejected', 'code': 'brep_validation_failed', 'relation': None, 'result': None}
        print(json.dumps(reply, allow_nan=False, separators=(',', ':')), file=protocol)


if __name__ == '__main__':
    main()
