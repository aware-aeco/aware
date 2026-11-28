"""(c) Clash between two trimmed member solids: exact B-rep interference vs a mesh check on the kernel's own tessellation.
Geometry: a girder along Y; a coped (top and bottom) beam framing into its web with a skewed end (the 'trimmed' beam),
at a chosen gap (positive) or penetration (negative) from the web face. Then two round pipes at a micro gap/overlap.
Usage: python -I run_c.py  -> out/kernel_c.json"""
import json
import math
import os
import sys
import time

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import manifold3d as m3  # noqa: E402
import usd_brep as sm  # noqa: E402

from common import SEC, poly_to_arrays  # noqa: E402
from cases import OUT  # noqa: E402
import kernel_ops as ko  # noqa: E402

TW = SEC['tw']
COPE_D, COPE_L, COPE_R = 35.0, 100.0, 12.0
SKEW = 10.0


def girder():
    g = ko.beam(length=2000.0)  # local z -> world -Y, local y -> world Z; centred about Y=0 by the origin below
    return ko.place(g, (0.0, 1000.0, 0.0), (1.0, 0.0, 0.0), (0.0, 0.0, 1.0))


def framing_beam(x0, skew):
    """Beam A, coped top and bottom, with its end plane skewed in plan, starting at X = x0 and running +X."""
    b = ko.beam(length=1500.0)
    top = ko.cope_cutter(COPE_D, COPE_L, COPE_R)
    # bottom cope = the top cope mirrored through the member's mid-depth: rotate the cutter 180 deg about the z axis
    bot = sm.rotate(ko.cope_cutter(COPE_D, COPE_L, COPE_R), (0.0, 0.0, 0.0), (0.0, 0.0, 1.0), 180.0)
    b = sm.boolean_difference(b, top)
    b = sm.boolean_difference(b, bot)
    if skew:
        b = sm.boolean_difference(b, ko.skew_slab(skew))
    return ko.place(b, (x0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0))


def mesh_of(brep, ch):
    pts, tri = poly_to_arrays(brep.tessellate(chord_height_tolerance=ch))
    return pts, tri


def m3_of(pts, tri):
    return m3.Manifold(m3.Mesh(vert_properties=np.array(pts, dtype=np.float32, order='C'), tri_verts=np.array(tri, dtype=np.uint32, order='C')))


def mesh_overlap(a, b, ch):
    pa, ta = mesh_of(a, ch)
    pb, tb = mesh_of(b, ch)
    t0 = time.perf_counter()
    ma, mb = m3_of(pa, ta), m3_of(pb, tb)
    inter = ma ^ mb  # intersection
    vol = inter.volume()
    ms = (time.perf_counter() - t0) * 1000
    return vol, ms, (len(ta), len(tb)), pa, ta, pb, tb


def min_mesh_distance(pa, ta, pb, tb):
    """Min distance between two triangle meshes: vertex-to-triangle both ways (brute force, vectorised). Edge-edge
    approach is not covered, so this is a lower-effort UPPER bound on the true mesh distance."""
    def pts_to_tris(P, V, T):
        best = 1e30
        A, B, C = V[T[:, 0]], V[T[:, 1]], V[T[:, 2]]
        for i in range(0, len(P), 64):
            p = P[i:i + 64][:, None, :]
            best = min(best, float(np.min(_pt_tri(p, A[None], B[None], C[None]))))
        return best
    return min(pts_to_tris(pa, pb, tb), pts_to_tris(pb, pa, ta))


def _pt_tri(p, a, b, c):
    ab, ac, ap = b - a, c - a, p - a
    d1, d2 = (ab * ap).sum(-1), (ac * ap).sum(-1)
    bp = p - b
    d3, d4 = (ab * bp).sum(-1), (ac * bp).sum(-1)
    cp = p - c
    d5, d6 = (ab * cp).sum(-1), (ac * cp).sum(-1)
    vc, vb, va = d1 * d4 - d3 * d2, d5 * d2 - d1 * d6, d3 * d6 - d5 * d4
    # barycentric, clamped by projecting onto the triangle with the generic (Ericson) region test, simplified via clamping
    denom = va + vb + vc
    denom = np.where(np.abs(denom) < 1e-30, 1e-30, denom)
    v, w = vb / denom, vc / denom
    q = a + ab * v[..., None] + ac * w[..., None]
    inside = (v >= 0) & (w >= 0) & (v + w <= 1)
    d_in = np.linalg.norm(p - q, axis=-1)

    def seg(p, s0, s1):
        e = s1 - s0
        t = np.clip(((p - s0) * e).sum(-1) / np.maximum((e * e).sum(-1), 1e-30), 0, 1)
        return np.linalg.norm(p - (s0 + e * t[..., None]), axis=-1)
    d_edge = np.minimum(np.minimum(seg(p, a, b), seg(p, b, c)), seg(p, c, a))
    return np.where(inside, d_in, d_edge)


def run():
    rows = []
    girder_vol = None
    for name, gap, skew, expected in [
        ('clear 12.7 mm', 12.7, SKEW, ('dist', 12.7 - 0.0)),
        ('clear 0.05 mm', 0.05, SKEW, ('dist', 0.05)),
        ('touching (0 gap)', 0.0, 0.0, ('vol', 0.0)),
        # the skew slab only trims the x<0 half of the web end (z >= 0 for x>0), so the analytic overlap is
        # 288 * (4*2 + integral_0^4 (2 - x tan10) dx) = 4201.74, NOT 288*8*2
        ('overlap 2 mm, end skew 10 deg', -2.0, SKEW, ('vol', 288.0 * (8.0 + 8.0 - 8.0 * math.tan(math.radians(SKEW))))),
        ('overlap 0.01 mm, end skew 0', -0.01, 0.0, ('vol', 288.0 * TW * 0.01)),
        ('overlap 0.001 mm, end skew 0', -0.001, 0.0, ('vol', 288.0 * TW * 0.001)),
    ]:
        x0 = TW / 2 + gap
        row = dict(case=name, gap=gap, skew=skew, expected=expected)
        try:
            G = girder()
            A = framing_beam(x0, skew)
            row['A_solid'] = bool(A.is_manifold_solid())
            t0 = time.perf_counter()
            d = sm.brep_distance(G.copy(), A.copy())
            row['exact_distance'] = d
            row['exact_distance_ms'] = (time.perf_counter() - t0) * 1000
            t0 = time.perf_counter()
            try:
                rel = G.copy().relationship_to(A.copy())
                row['relationship'] = {k: (v if isinstance(v, (int, float, str, bool)) else str(v)) for k, v in rel.items()}
            except Exception as e:
                row['relationship'] = f'ERR {type(e).__name__}: {e}'
            row['relationship_ms'] = (time.perf_counter() - t0) * 1000
            t0 = time.perf_counter()
            try:
                inter = sm.boolean_intersection(G.copy(), A.copy())
                row['exact_overlap_volume'] = inter.volume(1e-12)
            except Exception as e:
                row['exact_overlap_volume'] = f'ERR {type(e).__name__}: {e}'
            row['exact_overlap_ms'] = (time.perf_counter() - t0) * 1000
            for ch in (0.1, 0.01):
                vol, ms, tris, pa, ta, pb, tb = mesh_overlap(G.copy(), A.copy(), ch)
                row[f'mesh_ch{ch}'] = dict(overlap_volume=vol, ms=ms, tris=tris,
                                           min_dist_upper_bound=min_mesh_distance(pa, ta, pb, tb) if gap >= 0 else None)
        except Exception as e:
            row['error'] = f'{type(e).__name__}: {e}'
        rows.append(row)
        print(name, json.dumps({k: v for k, v in row.items() if k not in ('expected',)})[:400], flush=True)
    return rows


def run_pipes():
    """Two round members whose axes cross at 90 degrees, surfaces a micro-gap apart / overlapping. The exact answer:
    distance = (axis distance) - r1 - r2, and the overlap is a lens whose volume is computed by the kernel only."""
    rows = []
    r1, r2 = 100.0, 60.0
    for name, off in [('pipes gap 0.05 mm', 0.05), ('pipes gap 0.5 mm', 0.5), ('pipes overlap 0.005 mm', -0.005), ('pipes overlap 0.02 mm', -0.02), ('pipes overlap 0.05 mm', -0.05), ('pipes overlap 0.5 mm', -0.5)]:
        axis_dist = r1 + r2 + off
        row = dict(case=name, off=off, expected_distance=max(off, 0.0))
        try:
            c1 = sm.create_cylinder((0.0, 0.0, -500.0), r1, 1000.0)  # axis along z
            c2 = sm.create_cylinder((-500.0, axis_dist, 0.0), r2, 1000.0)  # axis along x after rotating, offset in y
            c2 = sm.rotate(c2, (0.0, axis_dist, 0.0), (0.0, 1.0, 0.0), 90.0)
            t0 = time.perf_counter()
            row['exact_distance'] = sm.brep_distance(c1.copy(), c2.copy())
            row['exact_distance_ms'] = (time.perf_counter() - t0) * 1000
            try:
                row['exact_overlap_volume'] = sm.boolean_intersection(c1.copy(), c2.copy()).volume(1e-12)
            except Exception as e:
                row['exact_overlap_volume'] = f'ERR {type(e).__name__}: {e}'
            for ch in (0.1, 0.01):
                vol, ms, tris, pa, ta, pb, tb = mesh_overlap(c1.copy(), c2.copy(), ch)
                row[f'mesh_ch{ch}'] = dict(overlap_volume=vol, ms=ms, tris=tris, min_dist_upper_bound=min_mesh_distance(pa, ta, pb, tb))
        except Exception as e:
            row['error'] = f'{type(e).__name__}: {e}'
        rows.append(row)
        print(name, json.dumps(row)[:500], flush=True)
    return rows


if __name__ == '__main__':
    out = dict(members=run(), pipes=run_pipes())
    json.dump(out, open(os.path.join(OUT, 'kernel_c.json'), 'w'), indent=1)
