"""OCCT at survey-scale coordinates: how far from the origin does the coped-beam boolean go silently wrong, and does
re-centring the two operands before the boolean fix it? The 120 random placed cases of (a), translation stretched.
Usage: python -I run_occt.py probe_far.py   -> out/occt_kernel_probe_far.json (run through run_occt.py so occt_ops is `usd_brep`)."""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import usd_brep as sm  # noqa: E402
from cases import OUT, cases_a  # noqa: E402
from run_kernel import build_a, ref_vol_a  # noqa: E402

base = [c for c in cases_a() if c['id'].startswith('r')]
res = {}
for scale in (1, 2, 4, 8, 20, 100):  # base translation is up to 50 m
    wrong_direct = wrong_rec = 0
    worst_direct = worst_rec = 0.0
    for c0 in base:
        c = json.loads(json.dumps(c0))
        c['place']['t'] = [x * scale for x in c0['place']['t']]
        ref = ref_vol_a(c)
        b, cut, slab = build_a(c)
        t = c['place']['t']
        mv = lambda x: sm.translate(x, (-t[0], -t[1], -t[2]))
        for mode in ('direct', 'recentred'):
            try:
                if mode == 'direct':
                    r = sm.boolean_difference(b, cut)
                    if slab is not None:
                        r = sm.boolean_difference(r, slab)
                else:
                    r = sm.boolean_difference(mv(b), mv(cut))
                    if slab is not None:
                        r = sm.boolean_difference(r, mv(slab))
                e = abs(r.volume() - ref) / ref
            except Exception:
                e = 1.0
            if mode == 'direct':
                wrong_direct += e > 1e-6
                worst_direct = max(worst_direct, e)
            else:
                wrong_rec += e > 1e-6
                worst_rec = max(worst_rec, e)
    res[f'up to {50 * scale} m'] = dict(n=len(base), wrong_direct=int(wrong_direct), worst_direct=worst_direct, wrong_after_recentring=int(wrong_rec), worst_recentred=worst_rec)
    print(f'up to {50 * scale} m', res[f'up to {50 * scale} m'], flush=True)
json.dump(res, open(os.path.join(OUT, 'occt_kernel_probe_far.json'), 'w'), indent=1)
