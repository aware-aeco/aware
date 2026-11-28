"""Open Cascade (OCCT, via the PyPI wheel `cadquery-ocp`, import name OCP) behind the SAME call surface that the SMLib
trial scripts use (`usd_brep as sm`). `run_occt.py` installs this module as `usd_brep` so cases.py, common.py,
kernel_ops.py and run_*.py run on OCCT without edits.

Only what the trial calls is implemented. Choices:
- Booleans are BRepAlgoAPI_Cut/Fuse/Common with SetRunParallel(False) (single thread, like the SMLib timings).
  `FUZZY` (module global, model units = mm) is passed to SetFuzzyValue; 0 means "off".
- A boolean that reports IsDone() false, or returns a null/empty shape, RAISES (a throw is a datum, not hidden).
- Volume is BRepGProp.VolumeProperties (Gauss integration over the exact surfaces, no mesh).
- Validity is BRepCheck_Analyzer; `is_manifold_solid` also needs exactly one closed solid with positive volume.
- Unlike SMLib, booleans do NOT mutate their operands, so `.copy()` is cheap insurance only.
- Tessellation is BRepMesh_IncrementalMesh (linear deflection = the requested chord height, angular 0.5 rad), read back
  in float64 (SMLib hands back float32).
"""
import math
import os
import time

import numpy as np
from OCP.BRep import BRep_Tool
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import (BRepBuilderAPI_Copy, BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeVertex, BRepBuilderAPI_MakeWire, BRepBuilderAPI_Transform)
from OCP.BRepCheck import BRepCheck_Analyzer
from OCP.BRepExtrema import BRepExtrema_DistShapeShape
from OCP.BRepGProp import BRepGProp
from OCP.BRepMesh import BRepMesh_IncrementalMesh
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakePrism
from OCP.BRepTools import BRepTools
from OCP.GProp import GProp_GProps
from OCP.IFSelect import IFSelect_RetDone
from OCP.STEPControl import STEPControl_AsIs, STEPControl_Reader, STEPControl_Writer
from OCP.TopAbs import TopAbs_FACE, TopAbs_REVERSED, TopAbs_SOLID, TopAbs_SHELL
from OCP.TopExp import TopExp, TopExp_Explorer
from OCP.TopLoc import TopLoc_Location
from OCP.OCP.collections import IndexedMap_TopoDS_Shape_TopTools_ShapeMapHasher as TopTools_IndexedMapOfShape, List_TopoDS_Shape as TopTools_ListOfShape
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax1, gp_Ax2, gp_Circ, gp_Dir, gp_Pnt, gp_Trsf, gp_Vec

COPY_TRANSFORM = True  # False = placement only adds a TopLoc_Location (geometry not rewritten)
ADAPTIVE = False  # volume(accuracy) uses the adaptive integrator when True
FUZZY = 0.0  # SetFuzzyValue for every boolean (mm); the launcher sets it from --fuzzy
BOOL_LOG = []  # (name, ms) of every boolean, for per-operation timing summaries


class OcctError(RuntimeError):
    pass


def _pnt(p):
    return gp_Pnt(float(p[0]), float(p[1]), float(p[2]))


def _count(shape, kind):
    m = TopTools_IndexedMapOfShape()
    TopExp.MapShapes_s(shape, kind, m)
    return m.Extent()


def _solids(shape):
    out = []
    ex = TopExp_Explorer(shape, TopAbs_SOLID)
    while ex.More():
        out.append(ex.Current())
        ex.Next()
    return out


class Brep:
    """Wrapper with the subset of the SMLib Brep surface the trial uses."""

    def __init__(self, shape):
        self.shape = shape

    def copy(self):
        return Brep(BRepBuilderAPI_Copy(self.shape, True, False).Shape())

    def face_count(self):
        return _count(self.shape, TopAbs_FACE)

    def volume(self, accuracy=None):
        p = GProp_GProps()
        if ADAPTIVE and accuracy:  # adaptive Gauss-Kronrod to a relative target (slower); the default is fixed-order Gauss
            BRepGProp.VolumeProperties_s(self.shape, p, max(float(accuracy), 1e-10), False, False)
        else:
            BRepGProp.VolumeProperties_s(self.shape, p, False, False, False)
        return p.Mass()

    def area(self, accuracy=None):
        p = GProp_GProps()
        BRepGProp.SurfaceProperties_s(self.shape, p, False, False)
        return p.Mass()

    def is_valid(self):
        return bool(BRepCheck_Analyzer(self.shape).IsValid())

    def is_manifold_solid(self):
        """Valid per BRepCheck, exactly one solid, its shell closed, volume > 0."""
        if self.shape.IsNull() or not self.is_valid():
            return False
        sol = _solids(self.shape)
        if len(sol) != 1:
            return False
        ex = TopExp_Explorer(sol[0], TopAbs_SHELL)
        n = 0
        while ex.More():
            n += 1
            if not BRep_Tool.IsClosed_s(ex.Current()):
                return False
            ex.Next()
        return n == 1 and self.volume() > 0

    def relationship_to(self, other):
        """separate / touching / interpenetrating, from the exact distance and the exact common volume."""
        d = brep_distance(self, other)
        if d > 1e-7:
            return dict(relation='separate', distance=d)
        try:
            v = boolean_intersection(self, other).volume()
        except OcctError:
            v = float('nan')
        return dict(relation='interpenetrating' if v > 1e-9 else 'touching', distance=d, overlap_volume=v)

    def tessellate(self, chord_height_tolerance=0.1, return_report=False, angular=0.5):
        BRepTools.Clean_s(self.shape)
        t0 = time.perf_counter()
        BRepMesh_IncrementalMesh(self.shape, float(chord_height_tolerance), False, float(angular), False)
        pts, tri = [], []
        off = 0
        ex = TopExp_Explorer(self.shape, TopAbs_FACE)
        while ex.More():
            face = TopoDS.Face(ex.Current())
            loc = TopLoc_Location()
            t = BRep_Tool.Triangulation_s(face, loc)
            if t is not None:
                tr = loc.Transformation()
                n = t.NbNodes()
                for i in range(1, n + 1):
                    q = t.Node(i).Transformed(tr)
                    pts.append((q.X(), q.Y(), q.Z()))
                rev = face.Orientation() == TopAbs_REVERSED
                for i in range(1, t.NbTriangles() + 1):
                    a, b, c = t.Triangle(i).Get()
                    if rev:
                        b, c = c, b
                    tri.append((off + a - 1, off + b - 1, off + c - 1))
                off += n
            ex.Next()
        ms = (time.perf_counter() - t0) * 1000
        P = np.array(pts, float).reshape(-1, 3)
        T = np.array(tri, int).reshape(-1, 3)
        # OCCT keeps separate nodes per face; weld them (1e-6 mm grid, as the metrics do) so the mesh is an indexed closed mesh
        _, first, inv = np.unique(np.round(P / 1e-6).astype(np.int64), axis=0, return_index=True, return_inverse=True)
        inv = inv.reshape(-1)
        m = _Mesh(P[first], inv[T])
        return (m, dict(ms=ms)) if return_report else m

    def write_step(self, path):
        w = STEPControl_Writer()
        w.Transfer(self.shape, STEPControl_AsIs)
        if int(w.Write(path)) != int(IFSelect_RetDone):
            raise OcctError('STEP write failed')

    @staticmethod
    def read_step(path):
        r = STEPControl_Reader()
        if int(r.ReadFile(path)) != int(IFSelect_RetDone):
            raise OcctError('STEP read failed')
        r.TransferRoots()
        return Brep(r.OneShape())


class _Mesh:
    def __init__(self, pts, tri):
        self.pts, self.tri = pts, tri

    def to_numpy_arrays(self, face_varying_normals=False):
        faces = np.hstack([np.full((len(self.tri), 1), 3), self.tri]).reshape(-1)
        return dict(points=self.pts, faces=faces)


# ---------- builders ----------
def create_box(origin, dx, dy, dz):
    return Brep(BRepPrimAPI_MakeBox(_pnt(origin), float(dx), float(dy), float(dz)).Shape())


def create_cylinder(origin, radius, height):
    return Brep(BRepPrimAPI_MakeCylinder(gp_Ax2(_pnt(origin), gp_Dir(0, 0, 1)), float(radius), float(height)).Shape())


def create_line_segment(p0, p1):
    return BRepBuilderAPI_MakeEdge(_pnt(p0), _pnt(p1)).Edge()


def create_arc(centre, radius, a0_deg, a1_deg):
    c = gp_Circ(gp_Ax2(_pnt(centre), gp_Dir(0, 0, 1)), float(radius))
    return BRepBuilderAPI_MakeEdge(c, math.radians(a0_deg), math.radians(a1_deg)).Edge()


def linear_sweep(curves, direction, distance):
    w = BRepBuilderAPI_MakeWire()
    for e in curves:
        w.Add(e)
    if not w.IsDone():
        raise OcctError('wire not built')
    face = BRepBuilderAPI_MakeFace(w.Wire(), True).Face()
    v = gp_Vec(*[float(x) * float(distance) for x in direction])
    s = BRepPrimAPI_MakePrism(face, v).Shape()
    b = Brep(s)
    if b.volume() < 0:  # clockwise profile gives an inside-out solid
        b = Brep(s.Reversed())
    return b


# ---------- placement ----------
def _apply(b, trsf):
    return Brep(BRepBuilderAPI_Transform(b.shape, trsf, COPY_TRANSFORM).Shape())


class Axis2Placement:
    def __init__(self, origin, xdir, ydir):
        x = np.array(xdir, float)
        y = np.array(ydir, float)
        x /= np.linalg.norm(x)
        y = y - x * float(x @ y)
        y /= np.linalg.norm(y)
        z = np.cross(x, y)
        t = gp_Trsf()
        # local -> world: columns are the local axes in world coordinates
        t.SetValues(x[0], y[0], z[0], float(origin[0]), x[1], y[1], z[1], float(origin[1]), x[2], y[2], z[2], float(origin[2]))
        self.trsf = t


def transform(b, placement):
    return _apply(b, placement.trsf)


def translate(b, v):
    t = gp_Trsf()
    t.SetTranslation(gp_Vec(float(v[0]), float(v[1]), float(v[2])))
    return _apply(b, t)


def rotate(b, point, axis, deg):
    t = gp_Trsf()
    t.SetRotation(gp_Ax1(_pnt(point), gp_Dir(*[float(a) for a in axis])), math.radians(deg))
    return _apply(b, t)


# ---------- booleans ----------
def _boolean(cls, name, a, b):
    op = cls()
    args, tools = TopTools_ListOfShape(), TopTools_ListOfShape()
    args.Append(a.shape)
    tools.Append(b.shape)
    op.SetArguments(args)
    op.SetTools(tools)
    op.SetRunParallel(False)
    if FUZZY > 0:
        op.SetFuzzyValue(float(FUZZY))
    t0 = time.perf_counter()
    op.Build()
    BOOL_LOG.append((name, (time.perf_counter() - t0) * 1000))
    if not op.IsDone():
        raise OcctError(f'{name}: boolean not done (error flag)')
    s = op.Shape()
    if s.IsNull():
        raise OcctError(f'{name}: null result')
    return Brep(s)


def boolean_difference(a, b):
    return _boolean(BRepAlgoAPI_Cut, 'cut', a, b)


def boolean_union(a, b):
    return _boolean(BRepAlgoAPI_Fuse, 'fuse', a, b)


def boolean_intersection(a, b):
    return _boolean(BRepAlgoAPI_Common, 'common', a, b)


# ---------- measures ----------
def brep_distance(a, b):
    d = BRepExtrema_DistShapeShape(a.shape, b.shape)
    if not d.IsDone():
        raise OcctError('distance failed')
    return d.Value()


def brep_closest_point(b, p):
    v = BRepBuilderAPI_MakeVertex(_pnt(p)).Vertex()
    d = BRepExtrema_DistShapeShape(b.shape, v)
    if not d.IsDone():
        raise OcctError('closest point failed')
    q = d.PointOnShape1(1)
    return (q.X(), q.Y(), q.Z()), d.Value()
