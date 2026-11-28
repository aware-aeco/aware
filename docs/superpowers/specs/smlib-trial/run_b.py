"""(b) The #4005 shape: a part cut whose operand SHARES faces with the part (a corner clip flush with the plate's edges
and faces). Today's parser evaluates `solid - cutter` with manifold3d; here the same shape is cut with manifold3d (the
baseline) and with the SMLib exact boolean. Real Tekla data is not used (it is not in this repo and not public): the shape
is synthesised from the issue's description, under random rigid placements with large coordinates.

Usage: python -I run_b.py   -> out/kernel_b.json (and prints a summary)"""
import json
import math
import os
import statistics as st
import sys
import time

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import manifold3d as m3  # noqa: E402
import usd_brep as sm  # noqa: E402

from common import mesh_metrics, poly_to_arrays  # noqa: E402
from cases import OUT  # noqa: E402
from run_kernel import rot_frame  # noqa: E402
import kernel_ops as ko  # noqa: E402

PL, PW, PT = 400.0, 250.0, 20.0  # plate length, width, thickness
LEG_A, LEG_B = 60.0, 45.0  # the clipped corner's legs


def clip_corner_poly(over):
    """Corner clip triangle at the plate's (0,0) corner. over=0: its two legs lie exactly ON the plate edges;
    over>0: extends past the plate by `over` in plane."""
    return [(-over, -over), (LEG_A + over * (LEG_A / LEG_B), -over), (-over, LEG_B + over * (LEG_B / LEG_A))]


VARIANTS = {
    # name: (in-plane overshoot, z0, z1) of the operand relative to the plate's z in [0, PT]
    'flush-edges+faces': (0.0, 0.0, PT),   # shares ALL FOUR planes with the plate: the issue's trigger
    'flush-faces-only': (5.0, 0.0, PT),    # overshoots in plane, but top and bottom faces are shared
    'flush-edges-only': (0.0, -5.0, PT + 5.0),  # legs on plate edges, thicker than the plate
    'overshoot(control)': (5.0, -5.0, PT + 5.0),
}
SCALES = (1e3, 1e4, 1e5)  # |coordinate| bound of the random placement, mm
ANALYTIC_REMOVED = 0.5 * LEG_A * LEG_B * PT


def sheet_ref_volume():
    return PL * PW * PT - ANALYTIC_REMOVED


def build_m3(poly, z0, z1, R, t, rng_noise):
    """Mesh the plate and operand independently and place each with its own float64 rigid transform, the way the parser
    builds world-space solids (so shared planes are only equal to rounding, never bit-exact)."""
    plate = m3.Manifold.cube((PL, PW, PT))
    cs = m3.CrossSection([poly])
    op = m3.Manifold.extrude(cs, z1 - z0).translate((0, 0, z0))

    def world(man):
        me = man.to_mesh()
        v = np.asarray(me.vert_properties, float)[:, :3]
        v = v @ R.T + t
        return m3.Manifold(m3.Mesh(vert_properties=np.array(v, dtype=np.float32, order="C"), tri_verts=np.array(me.tri_verts, dtype=np.uint32, order="C")))
    return world(plate), world(op)


def run_variant(name, n, seed, scale):
    over, z0, z1 = VARIANTS[name]
    poly = clip_corner_poly(over)
    rnd = np.random.default_rng(seed)
    rows = []
    for i in range(n):
        ax = rnd.normal(size=3)
        ax /= np.linalg.norm(ax)
        ang = rnd.uniform(0, 360)
        xa, ya = rot_frame(ax.tolist(), ang)
        za = np.cross(xa, ya)
        R = np.stack([xa, ya, za], axis=1)
        t = rnd.uniform(-scale, scale, size=3)
        row = dict(variant=name, scale=scale, i=i)
        # --- baseline: manifold3d, as the parser does it
        plate, op = build_m3(poly, z0, z1, R, t, None)
        t0 = time.perf_counter()
        r = plate - op
        me = r.to_mesh()  # manifold3d is lazy: the boolean runs here, so it is inside the timed span
        row['m3_ms'] = (time.perf_counter() - t0) * 1000
        v = np.asarray(me.vert_properties, float)[:, :3]
        f = np.asarray(me.tri_verts, int)
        mm = mesh_metrics(v, f, area_eps=1e-6, overlap_eps=1e-3, weld=1e-6)
        row['m3'] = dict(vol_err=(r.volume() - sheet_ref_volume()) / sheet_ref_volume(), tris=mm['tris'], zero_area=mm['zero_area_tris'],
                         coincident_planes=mm['coincident_opposite_planes'], coincident_mm2=mm['coincident_opposite_area_mm2'],
                         open_edges=mm['open_edges'], genus=int(r.genus()))
        # --- SMLib: the exact boolean on the same placement
        try:
            pb = sm.create_box((0.0, 0.0, 0.0), PL, PW, PT)
            curves = [sm.create_line_segment((*poly[k], 0.0), (*poly[(k + 1) % 3], 0.0)) for k in range(3)]
            opb = sm.linear_sweep(curves, (0, 0, 1), z1 - z0)
            opb = sm.translate(opb, (0.0, 0.0, z0))
            pb = ko.place(pb, t.tolist(), xa, ya)
            opb = ko.place(opb, t.tolist(), xa, ya)
            t0 = time.perf_counter()
            rb = sm.boolean_difference(pb, opb)
            row['sm_ms'] = (time.perf_counter() - t0) * 1000
            vol = rb.volume(1e-12)
            mesh = rb.tessellate(chord_height_tolerance=0.01)
            pts, tri = poly_to_arrays(mesh)
            km = mesh_metrics(pts, tri, area_eps=1e-6, overlap_eps=1e-3, weld=1e-6)
            row['sm'] = dict(vol_err=(vol - sheet_ref_volume()) / sheet_ref_volume(), faces=rb.face_count(), manifold=bool(rb.is_manifold_solid()),
                             tris=km['tris'], zero_area=km['zero_area_tris'], coincident_planes=km['coincident_opposite_planes'],
                             coincident_mm2=km['coincident_opposite_area_mm2'], open_edges=km['open_edges'])
        except Exception as e:
            row['sm'] = dict(error=f'{type(e).__name__}: {e}')
        rows.append(row)
    return rows


def summarize(rows):
    out = {}
    for name, scale in [(a, b) for a in VARIANTS for b in SCALES]:
        sub = [r for r in rows if r['variant'] == name and r['scale'] == scale]
        m3r = [r['m3'] for r in sub]
        smr = [r['sm'] for r in sub]
        out[f'{name} @{scale:g}mm'] = dict(
            n=len(sub),
            m3_with_coincident=sum(1 for x in m3r if x['coincident_planes'] > 0),
            m3_with_zero_area=sum(1 for x in m3r if x['zero_area'] > 0),
            m3_max_vol_err=max(abs(x['vol_err']) for x in m3r),
            m3_ms_median=st.median(r['m3_ms'] for r in sub),
            sm_errors=sum(1 for x in smr if 'error' in x),
            sm_not_manifold=sum(1 for x in smr if 'manifold' in x and not x['manifold']),
            sm_with_coincident=sum(1 for x in smr if x.get('coincident_planes', 0) > 0),
            sm_with_zero_area=sum(1 for x in smr if x.get('zero_area', 0) > 0),
            sm_faces=sorted({x['faces'] for x in smr if 'faces' in x}),
            sm_max_vol_err=max((abs(x['vol_err']) for x in smr if 'vol_err' in x), default=None),
            sm_ms_median=st.median(r['sm_ms'] for r in sub if 'sm_ms' in r) if any('sm_ms' in r for r in sub) else None)
    return out


if __name__ == '__main__':
    n = int(sys.argv[1]) if len(sys.argv) > 1 else 100
    rows = []
    for k, name in enumerate(VARIANTS):
        for scale in SCALES:
            rows += run_variant(name, n, 4005 + k, scale)
    os.makedirs(OUT, exist_ok=True)
    json.dump(dict(summary=summarize(rows), rows=rows), open(os.path.join(OUT, 'kernel_b.json'), 'w'), indent=1)
    print(json.dumps(summarize(rows), indent=1))
