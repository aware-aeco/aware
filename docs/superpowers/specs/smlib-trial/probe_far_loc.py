"""Is the far-coordinate failure caused by baking the placement into the geometry? Same 120 cases at up to 1,000 m, placement
done as a TopLoc_Location only (copy=False) vs baked (copy=True). Usage: python -I run_occt.py probe_far_loc.py"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import usd_brep as sm  # noqa: E402
from cases import OUT, cases_a  # noqa: E402
from run_kernel import build_a, ref_vol_a  # noqa: E402

res = {}
for copy in (True, False):
    sm.COPY_TRANSFORM = copy
    wrong = 0
    for c0 in [c for c in cases_a() if c['id'].startswith('r')]:
        c = json.loads(json.dumps(c0))
        c['place']['t'] = [x * 20 for x in c0['place']['t']]
        ref = ref_vol_a(c)
        b, cut, slab = build_a(c)
        try:
            r = sm.boolean_difference(b, cut)
            if slab is not None:
                r = sm.boolean_difference(r, slab)
            wrong += abs(r.volume() - ref) / ref > 1e-6
        except Exception:
            wrong += 1
    res['baked geometry' if copy else 'location only'] = dict(n=120, wrong_at_1000m=int(wrong))
    print(res, flush=True)
json.dump(res, open(os.path.join(OUT, 'occt_kernel_probe_far_loc.json'), 'w'), indent=1)
