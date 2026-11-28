"""(e) Output routes: tessellation for a three.js viewer at stated tolerances (size, time, measured deviation from the exact
surface), USD BRep round trip, and what the wheel does NOT offer (STEP/IFC/IGES).
Usage: python -I run_e.py -> out/kernel_e.json (+ out/mesh/viewer_*.json for the Node check)"""
import json
import os
import sys
import time

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import usd_brep as sm  # noqa: E402

from common import SEC, mesh_metrics, poly_to_arrays  # noqa: E402
from cases import OUT, cases_a  # noqa: E402
from run_kernel import build_a  # noqa: E402


def coped_beam():
    c = [x for x in cases_a() if x['id'] == 'a2-cope+skew25'][0]
    b, cut, slab = build_a(c)
    r = sm.boolean_difference(b, cut)
    return sm.boolean_difference(r, slab)


def main():
    b = coped_beam()
    exact_v = b.volume(1e-12)
    exact_a = b.area(1e-9)
    res = dict(exact_volume=exact_v, exact_area=exact_a, faces=b.face_count(), tess=[])
    os.makedirs(os.path.join(OUT, 'mesh'), exist_ok=True)
    rnd = np.random.default_rng(1)
    for ch in (0.5, 0.1, 0.01, 0.001):
        t0 = time.perf_counter()
        m, rep = b.tessellate(chord_height_tolerance=ch, return_report=True)
        ms = (time.perf_counter() - t0) * 1000
        pts, tri = poly_to_arrays(m)
        mm = mesh_metrics(pts, tri)
        # measured deviation: distance from random points ON the triangles (barycentric samples, where the chord error lives) to the exact B-rep
        idx = rnd.choice(len(tri), size=min(400, len(tri)), replace=False)
        dev = []
        t1 = time.perf_counter()
        for i in idx:
            u = rnd.dirichlet([1, 1, 1])
            p = (pts[tri[i]] * u[:, None]).sum(0)
            _, d = sm.brep_closest_point(b, tuple(float(x) for x in p))
            dev.append(abs(d))
        dev_ms = (time.perf_counter() - t1) * 1000
        arr = m.to_numpy_arrays(face_varying_normals=True)
        row = dict(chord_height=ch, ms=ms, tris=mm['tris'], verts=mm['verts'], vol_rel_err=(mm['volume'] - exact_v) / exact_v,
                   open_edges=mm['open_edges'], nonmanifold=mm['nonmanifold_edges'], zero_area=mm['zero_area_tris'], inconsistent=mm['inconsistent_edges'],
                   max_sampled_deviation=float(np.max(dev)), p95_sampled_deviation=float(np.percentile(dev, 95)), closest_point_ms_each=dev_ms / len(idx),
                   report=str(rep)[:200], array_keys=sorted(arr.keys()), normals_shape=list(np.asarray(arr['normals']).shape))
        res['tess'].append(row)
        if ch in (0.1, 0.01):
            json.dump(dict(positions=pts.reshape(-1).tolist(), indices=tri.reshape(-1).tolist()), open(os.path.join(OUT, 'mesh', f'viewer_ch{ch}.json'), 'w'))
        print(row['chord_height'], row['tris'], row['ms'], row['max_sampled_deviation'], flush=True)
    # USD round trip
    path = os.path.join(OUT, 'mesh', 'coped.usda')
    t0 = time.perf_counter()
    sm.usd.export_brep(b, path)
    exp_ms = (time.perf_counter() - t0) * 1000
    t0 = time.perf_counter()
    (back,) = sm.usd.import_breps(path)
    imp_ms = (time.perf_counter() - t0) * 1000
    res['usd_roundtrip'] = dict(export_ms=exp_ms, import_ms=imp_ms, file_bytes=os.path.getsize(path), faces=back.face_count(), manifold=bool(back.is_manifold_solid()),
                                volume_rel_diff=(back.volume(1e-12) - exact_v) / exact_v)
    # native file round trip
    p2 = os.path.join(OUT, 'mesh', 'coped.sm.txt')
    b.write_to_file(p2)
    b2 = sm.Brep.read_from_file(p2)
    res['native_roundtrip'] = dict(file_bytes=os.path.getsize(p2), volume_rel_diff=(b2.volume(1e-12) - exact_v) / exact_v, faces=b2.face_count())
    res['no_step_ifc_symbols'] = [n for n in dir(sm) + dir(sm.usd) if any(k in n.lower() for k in ('step', 'ifc', 'iges', 'parasolid', 'acis'))]
    json.dump(res, open(os.path.join(OUT, 'kernel_e.json'), 'w'), indent=1)
    print(json.dumps({k: v for k, v in res.items() if k != 'tess'}, indent=1))


if __name__ == '__main__':
    main()
