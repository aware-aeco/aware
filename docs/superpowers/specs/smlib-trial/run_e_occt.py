"""(e) on Open Cascade: tessellation at the same chord heights (time, triangles, sampled deviation, mesh volume error),
and the STEP write -> read round trip of the coped, skewed beam (volume, faces, validity, file size, time), plus the same
for a random subset of the 120 placed cases. Usage: python -I run_e_occt.py -> out/occt_kernel_e.json"""
import json
import os
import statistics as st
import sys
import tempfile
import time

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import occt_ops as sm  # noqa: E402

from common import mesh_metrics, poly_to_arrays  # noqa: E402
from cases import OUT, cases_a  # noqa: E402
from run_kernel import build_a, ref_vol_a  # noqa: E402


def coped(c):
    b, cut, slab = build_a(c)
    r = sm.boolean_difference(b, cut)
    return sm.boolean_difference(r, slab) if slab is not None else r


def main():
    cs = {x['id']: x for x in cases_a()}
    b = coped(cs['a2-cope+skew25'])
    exact_v = b.volume()
    res = dict(exact_volume=exact_v, exact_area=b.area(), faces=b.face_count(), tess=[])
    rnd = np.random.default_rng(1)
    for ch in (0.5, 0.1, 0.01, 0.001):
        t0 = time.perf_counter()
        m = b.tessellate(chord_height_tolerance=ch)
        ms = (time.perf_counter() - t0) * 1000
        pts, tri = poly_to_arrays(m)
        mm = mesh_metrics(pts, tri)
        idx = rnd.choice(len(tri), size=min(400, len(tri)), replace=False)
        dev = []
        for i in idx:
            u = rnd.dirichlet([1, 1, 1])
            p = (pts[tri[i]] * u[:, None]).sum(0)
            _, d = sm.brep_closest_point(b, tuple(float(x) for x in p))
            dev.append(abs(d))
        res['tess'].append(dict(chord_height=ch, ms=ms, tris=mm['tris'], vol_rel_err=(mm['volume'] - exact_v) / exact_v, open_edges=mm['open_edges'],
                                nonmanifold=mm['nonmanifold_edges'], zero_area=mm['zero_area_tris'], inconsistent=mm['inconsistent_edges'],
                                max_sampled_deviation=float(np.max(dev)), p95_sampled_deviation=float(np.percentile(dev, 95))))
        print(res['tess'][-1], flush=True)
    d = tempfile.mkdtemp(prefix='occt_step_')
    path = os.path.join(d, 'coped.step')
    t0 = time.perf_counter()
    b.write_step(path)
    w_ms = (time.perf_counter() - t0) * 1000
    t0 = time.perf_counter()
    back = sm.Brep.read_step(path)
    r_ms = (time.perf_counter() - t0) * 1000
    res['step_roundtrip'] = dict(write_ms=w_ms, read_ms=r_ms, file_bytes=os.path.getsize(path), faces=back.face_count(), valid=back.is_valid(),
                                 manifold_solid=back.is_manifold_solid(), volume_rel_diff=(back.volume() - exact_v) / exact_v)
    # round trip over placed random cases (the near ones the booleans get right)
    rows = []
    for cid in [k for k in cs if k.startswith('r')][:40]:
        c = cs[cid]
        try:
            r = coped(c)
            v = r.volume()
            p = os.path.join(d, cid + '.step')
            r.write_step(p)
            bk = sm.Brep.read_step(p)
            rows.append(dict(id=cid, ok=bool(bk.is_manifold_solid()), faces_same=bk.face_count() == r.face_count(), vol_rel_diff=(bk.volume() - v) / v,
                             vs_ref=(v - ref_vol_a(c)) / ref_vol_a(c)))
        except Exception as e:
            rows.append(dict(id=cid, ok=False, error=f'{type(e).__name__}: {e}'))
    res['step_roundtrip_placed'] = dict(n=len(rows), ok=sum(r['ok'] for r in rows), faces_same=sum(r.get('faces_same', False) for r in rows),
                                        max_vol_rel_diff=max((abs(r['vol_rel_diff']) for r in rows if 'vol_rel_diff' in r), default=None),
                                        failures=[r for r in rows if not r['ok']][:5])
    json.dump(res, open(os.path.join(OUT, 'occt_kernel_e.json'), 'w'), indent=1)
    print(json.dumps({k: v for k, v in res.items() if k != 'tess'}, indent=1))


if __name__ == '__main__':
    main()
