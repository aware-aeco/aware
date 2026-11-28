"""(d) Parametric section sweep (the #3382 catalogue shape families): I-sections with root fillets and L-angles with a
root fillet and toe radii, N random sizes each. Per size: build time, solid validity, exact volume vs analytic
area * length, tessellation at a viewer tolerance. The mesh baseline is the polygonal outline FloLess draws today
(arcs as 8 segments per quarter); its error is computed from the polygon area (no CSG needed, the extrusion is exact).
Usage: python -I run_d.py [N]  -> out/kernel_d.json"""
import json
import math
import os
import random
import statistics as st
import sys
import time

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import usd_brep as sm  # noqa: E402

from common import ARC_SEG, i_outline, mesh_metrics, poly_to_arrays, section_area  # noqa: E402
from cases import OUT  # noqa: E402
import kernel_ops as ko  # noqa: E402

LEN = 6000.0
K = 1 - math.pi / 4


def l_outline(a, b, t, r1, r2, arc_seg=None):
    pcs = [('L', (0, 0), (a, 0))]
    pcs.append(('L', (a, 0), (a, t - r2)))
    if r2 > 0:
        pcs.append(('A', (a - r2, t - r2), r2, 0, 90))
    pcs.append(('L', (a - r2, t), (t + r1, t)))
    if r1 > 0:
        pcs.append(('A', (t + r1, t + r1), r1, 180, 270))
    pcs.append(('L', (t, t + r1), (t, b - r2)))
    if r2 > 0:
        pcs.append(('A', (t - r2, b - r2), r2, 0, 90))
    pcs.append(('L', (t - r2, b), (0, b)))
    pcs.append(('L', (0, b), (0, 0)))
    if arc_seg is None:
        return pcs
    poly = []
    for pc in pcs:
        if pc[0] == 'L':
            poly += [pc[1], pc[2]]
        else:
            _, c, rr, a0, a1 = pc
            pts = [(c[0] + rr * math.cos(math.radians(a0 + (a1 - a0) * i / arc_seg)), c[1] + rr * math.sin(math.radians(a0 + (a1 - a0) * i / arc_seg))) for i in range(arc_seg + 1)]
            if poly and math.dist(pts[0], poly[-1]) > math.dist(pts[-1], poly[-1]):
                pts.reverse()
            poly += pts
    out = []
    for p in poly:
        if not out or math.dist(p, out[-1]) > 1e-9:
            out.append(p)
    if math.dist(out[0], out[-1]) < 1e-9:
        out.pop()
    return out


def l_area(a, b, t, r1, r2):
    return t * (a + b - t) + K * (r1 * r1 - 2 * r2 * r2)


def poly_area(poly):
    return 0.5 * abs(sum(poly[i][0] * poly[(i + 1) % len(poly)][1] - poly[(i + 1) % len(poly)][0] * poly[i][1] for i in range(len(poly))))


def sizes(n, seed=3382):
    rnd = random.Random(seed)
    out = []
    for i in range(n):
        d = rnd.uniform(100, 1000)
        bf = rnd.uniform(0.25, 0.6) * d
        tf = rnd.uniform(5, 0.07 * d) if rnd.random() > 0.1 else 5.0
        tw = rnd.uniform(4, min(0.04 * d, 0.5 * bf))
        rmax = min((bf - tw) / 2, (d - 2 * tf) / 2)
        r = 0.0 if rnd.random() < 0.15 else rnd.uniform(1, min(0.08 * d, rmax * 0.95))
        out.append(('I', dict(d=d, bf=bf, tf=tf, tw=tw, r=r)))
    for i in range(n):
        a = rnd.uniform(40, 250)
        b = rnd.uniform(40, 250)
        t = rnd.uniform(3, min(a, b) * 0.2)
        r1 = rnd.uniform(0, 1.5 * t)
        r2 = rnd.uniform(0, min(r1, t) * 0.9) if rnd.random() > 0.3 else 0.0
        out.append(('L', dict(a=a, b=b, t=t, r1=r1, r2=r2)))
    return out


def main(n):
    rows = []
    for kind, p in sizes(n):
        row = dict(kind=kind, params=p)
        try:
            if kind == 'I':
                pcs, area, poly = i_outline(**p), section_area(**p), i_outline(**p, arc_seg=ARC_SEG)
            else:
                pcs, area, poly = l_outline(**p), l_area(**p), l_outline(**p, arc_seg=ARC_SEG)
            t0 = time.perf_counter()
            b = sm.linear_sweep(ko._curves(pcs), (0, 0, 1), LEN)
            row['build_ms'] = (time.perf_counter() - t0) * 1000
            row['solid'] = bool(b.is_manifold_solid())
            ex = area * LEN
            v = b.volume(1e-12)
            row['rel_err'] = (v - ex) / ex
            row['faces'] = b.face_count()
            row['polygon_rel_err'] = (poly_area(poly) - area) / area  # today's drawn outline (8 segs/quarter)
            t0 = time.perf_counter()
            m = b.tessellate(chord_height_tolerance=0.02)
            row['tess_ms'] = (time.perf_counter() - t0) * 1000
            pts, tri = poly_to_arrays(m)
            mm = mesh_metrics(pts, tri)
            row['mesh'] = dict(tris=mm['tris'], open_edges=mm['open_edges'], nonmanifold=mm['nonmanifold_edges'], zero_area=mm['zero_area_tris'],
                               inconsistent=mm['inconsistent_edges'], vol_rel_err=(mm['volume'] - ex) / ex)
            row['ok'] = bool(row['solid'] and abs(row['rel_err']) < 1e-9 and mm['open_edges'] == 0 and mm['nonmanifold_edges'] == 0 and mm['inconsistent_edges'] == 0 and mm['zero_area_tris'] == 0)
        except Exception as e:
            row['error'] = f'{type(e).__name__}: {e}'
            row['ok'] = False
        rows.append(row)
    return rows


def summary(rows):
    out = {}
    for kind in ('I', 'L'):
        sub = [r for r in rows if r['kind'] == kind]
        okr = [r for r in sub if 'rel_err' in r]
        out[kind] = dict(n=len(sub), ok=sum(r['ok'] for r in sub), errors=sum('error' in r for r in sub), not_solid=sum(1 for r in okr if not r['solid']),
                         max_abs_rel_err=max(abs(r['rel_err']) for r in okr), build_ms_median=st.median(r['build_ms'] for r in okr), build_ms_max=max(r['build_ms'] for r in okr),
                         tess_ms_median=st.median(r['tess_ms'] for r in okr), tess_ms_max=max(r['tess_ms'] for r in okr),
                         tris_median=st.median(r['mesh']['tris'] for r in okr),
                         mesh_open=sum(1 for r in okr if r['mesh']['open_edges']), mesh_zero=sum(1 for r in okr if r['mesh']['zero_area']),
                         mesh_vol_err_median=st.median(abs(r['mesh']['vol_rel_err']) for r in okr), mesh_vol_err_max=max(abs(r['mesh']['vol_rel_err']) for r in okr),
                         polygon_err_median=st.median(abs(r['polygon_rel_err']) for r in okr), polygon_err_max=max(abs(r['polygon_rel_err']) for r in okr),
                         failures=[dict(params=r['params'], error=r.get('error'), rel_err=r.get('rel_err'), mesh=r.get('mesh')) for r in sub if not r['ok']][:8])
    return out


if __name__ == '__main__':
    n = int(sys.argv[1]) if len(sys.argv) > 1 else 300
    rows = main(n)
    s = summary(rows)
    json.dump(dict(summary=s, rows=rows), open(os.path.join(OUT, 'kernel_d.json'), 'w'), indent=1)
    print(json.dumps(s, indent=1))
