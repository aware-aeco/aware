"""Scores the three-bvh-csg baseline (today's cope path) against the same independent reference volume as the kernel,
with the SAME mesh metrics. Usage: python -I analyze_a.py   (after run_kernel.py a and baseline_three.mjs)"""
import json
import os
import statistics as st
import sys

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from common import mesh_metrics  # noqa: E402
from cases import OUT, cases_a  # noqa: E402
from run_kernel import ref_vol_a  # noqa: E402

cases = {c['id']: c for c in cases_a()}
rows = json.load(open(os.path.join(OUT, 'three_a.json')))
res = []
for r in rows:
    c = dict(cases[r['id']])
    out = dict(id=r['id'], variant=r['variant'], ms=r.get('ms'))
    if 'error' in r:
        out.update(error=r['error'], ok=False)
        res.append(out)
        continue
    if r['variant'] == 'app':
        c = dict(c, depth=c['depth'] + 0.3, length=c['length'] + 0.3)  # the app's 0.6 mm pad moves the inner faces 0.3
    ref = ref_vol_a(c)
    f = np.array(json.load(open(os.path.join(OUT, 'mesh', f"three_{r['id']}_{r['variant']}.json"))), float).reshape(-1, 3)
    n = len(f) // 3
    F = np.arange(len(f)).reshape(n, 3)
    # three-bvh-csg output is unwelded and float32: weld at 1e-3 mm for the topology checks
    m = mesh_metrics(f, F, area_eps=1e-6, overlap_eps=1e-2, weld=1e-3)
    out.update(mesh=m, ref=ref, vol_rel_err=(m['volume'] - ref) / ref)
    # open_edges is reported but NOT a failure here: three-bvh-csg leaves T-junctions (unmatched edges that are not leaks)
    out['ok'] = bool(m['nonmanifold_edges'] == 0 and m['inconsistent_edges'] == 0 and m['zero_area_tris'] == 0
                     and m['coincident_opposite_planes'] == 0 and abs(out['vol_rel_err']) < 1e-3)
    res.append(out)
json.dump(res, open(os.path.join(OUT, 'three_a_scored.json'), 'w'), indent=1)
for v in ('exact', 'pad10', 'app'):
    sub = [x for x in res if x['variant'] == v]
    okc = sum(x['ok'] for x in sub)
    errs = [abs(x['vol_rel_err']) for x in sub if 'vol_rel_err' in x]
    print(v, 'runs', len(sub), 'clean', okc, 'threw', sum('error' in x for x in sub),
          'open_edge_runs', sum(1 for x in sub if x.get('mesh', {}).get('open_edges', 0) > 0),
          'zero_area_runs', sum(1 for x in sub if x.get('mesh', {}).get('zero_area_tris', 0) > 0),
          'coincident_runs', sum(1 for x in sub if x.get('mesh', {}).get('coincident_opposite_planes', 0) > 0),
          'vol_err median %.2e max %.2e' % (st.median(errs), max(errs)), 'ms median %.1f max %.1f' % (st.median(x['ms'] for x in sub), max(x['ms'] for x in sub)))
for x in res[:18]:
    m = x.get('mesh', {})
    print(x['id'], x['variant'], 'ok' if x['ok'] else 'NOT-CLEAN', x.get('error', ''), 'verr %.2e' % x.get('vol_rel_err', 0), 'open', m.get('open_edges'), 'nonman', m.get('nonmanifold_edges'), 'zero', m.get('zero_area_tris'), 'coinc', m.get('coincident_opposite_planes'), 'tris', m.get('tris'))
