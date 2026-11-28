"""Kernel-side measurements for the SMLib trial. Usage:  python -I run_kernel.py a|b|c|d|e|all
Writes out/kernel_<part>.json. Meshes for cross-engine comparison go to out/mesh/ (not committed)."""
import json
import math
import os
import sys
import time
import traceback

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import usd_brep as sm  # noqa: E402

from common import SEC, L, cope_zc, mesh_metrics, poly_to_arrays, reference_volume, timed  # noqa: E402
from cases import OUT, cases_a  # noqa: E402
import kernel_ops as ko  # noqa: E402

ACC = 1e-12  # relative_accuracy for exact volume; the default 1e-3 is a coarse numerical integral (see report)
CH = 0.01  # chord height (mm) for the "viewer" tessellation


def rot_frame(axis, angle_deg):
    a = np.array(axis, float)
    a /= np.linalg.norm(a)
    t = math.radians(angle_deg)
    K = np.array([[0, -a[2], a[1]], [a[2], 0, -a[0]], [-a[1], a[0], 0]])
    R = np.eye(3) * math.cos(t) + math.sin(t) * K + (1 - math.cos(t)) * np.outer(a, a)
    return R[:, 0].tolist(), R[:, 1].tolist()


def ref_vol_a(c):
    ang = math.radians(c['skew'])
    d = SEC['d']
    yb = d / 2 - c['depth']
    zk = lambda x, y: np.maximum(-x * math.tan(ang), cope_zc(y, d, SEC['tf'], c['depth'], c['length'], c['rc']))
    return reference_volume(zk, **SEC, ybreaks=(yb, yb + c['rc']))


def build_a(c):
    parts = [ko.beam(), ko.cope_cutter(c['depth'], c['length'], c['rc'], exact=c['exact'])]
    slab = ko.skew_slab(c['skew']) if c['skew'] != 0 else None
    pl = c.get('place')
    if pl:
        xa, ya = rot_frame(pl['axis'], pl['angle'])
        parts = [ko.place(p, pl['t'], xa, ya) for p in parts]
        if slab is not None:
            slab = ko.place(slab, pl['t'], xa, ya)
    return parts[0], parts[1], slab


def run_a(case_filter=None):
    cs = cases_a()
    res = []
    os.makedirs(os.path.join(OUT, 'mesh'), exist_ok=True)
    for c in cs:
        if case_filter and not case_filter(c):
            continue
        row = dict(id=c['id'], params={k: v for k, v in c.items() if k != 'place'}, placed=bool(c.get('place')))
        try:
            ref = ref_vol_a(c)
            row['ref_volume'] = ref
            t0 = time.perf_counter()
            b, cut, slab = build_a(c)
            row['build_ms'] = (time.perf_counter() - t0) * 1000
            t0 = time.perf_counter()
            r = sm.boolean_difference(b, cut)
            if slab is not None:
                r = sm.boolean_difference(r, slab)
            row['boolean_ms'] = (time.perf_counter() - t0) * 1000
            row['manifold_solid'] = bool(r.is_manifold_solid())
            row['faces'] = r.face_count()
            v = r.volume(ACC)
            row['volume'] = v
            row['rel_err'] = (v - ref) / ref
            t0 = time.perf_counter()
            m = r.tessellate(chord_height_tolerance=CH)
            row['tess_ms'] = (time.perf_counter() - t0) * 1000
            pts, tri = poly_to_arrays(m)
            row['mesh'] = mesh_metrics(pts, tri)
            row['mesh_volume_rel_err'] = (row['mesh']['volume'] - ref) / ref
            if not c.get('place'):
                np.savez_compressed(os.path.join(OUT, 'mesh', f"kernel_{c['id']}.npz"), pts=pts, tri=tri)
            row['ok'] = bool(row['manifold_solid'] and abs(row['rel_err']) < 1e-6 and row['mesh']['open_edges'] == 0
                             and row['mesh']['nonmanifold_edges'] == 0 and row['mesh']['zero_area_tris'] == 0
                             and row['mesh']['coincident_opposite_planes'] == 0)
        except Exception as e:  # a kernel exception is itself a datum
            row['error'] = f'{type(e).__name__}: {e}'
            row['ok'] = False
        res.append(row)
        print(row['id'], 'ok' if row['ok'] else 'FAIL', row.get('error', ''), flush=True)
    return res


if __name__ == '__main__':
    part = sys.argv[1] if len(sys.argv) > 1 else 'a'
    os.makedirs(OUT, exist_ok=True)
    out = {}
    if part == 'a-far':
        # robustness at survey-scale coordinates: the same random cases with the placement pushed out 20x (up to 1,000 m)
        import cases as _cases
        _orig = _cases.cases_a
        def _far(*a, **k):
            cs = _orig(*a, **k)
            for c in cs:
                if c.get('place'):
                    c['place']['t'] = [x * 20 for x in c['place']['t']]
            return cs
        cases_a = _far
        globals()['cases_a'] = _far
        out['a_far'] = run_a(lambda c: c['id'].startswith('r'))
        json.dump(out['a_far'], open(os.path.join(OUT, 'kernel_a_far.json'), 'w'), indent=1)
        print(sum(r['ok'] for r in out['a_far']), '/', len(out['a_far']), 'ok; max rel err', max(abs(r.get('rel_err', 0)) for r in out['a_far']))
    if part in ('a', 'all'):
        out['a'] = run_a()
        json.dump(out['a'], open(os.path.join(OUT, 'kernel_a.json'), 'w'), indent=1)
