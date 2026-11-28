"""Independent reference for the pipe-on-pipe overlap volume of (c): two cylinders r1=100 (axis z) and r2=60 (axis x),
axes `r1 + r2 + off` apart. V = integral over y of [2 sqrt(r2^2-(y-y0)^2)] * [2 sqrt(r1^2-y^2)], y in [y0-r2, r1].
The trial's second pipe, as built by run_c.py, spans x from 0 to 1000 and is centred at z = 500 (the first pipe ends at
z = 500), so only a quarter of the lens exists: V/4 (OCCT bounding boxes: pipe 2 x 0..1000, z 440..560).
The substitution y = a + (r1-a) sin^2(phi) removes both endpoint square roots; then Gauss-Legendre. No kernel, no mesh.
Usage: python -I pipes_ref.py"""
import json
import math
import os

import numpy as np

r1, r2 = 100.0, 60.0
x, w = np.polynomial.legendre.leggauss(400)
out = {}
for off in (-0.005, -0.02, -0.05, -0.5):
    y0 = r1 + r2 + off
    a = y0 - r2
    phi = 0.25 * math.pi * (x + 1)  # 0..pi/2
    wp = 0.25 * math.pi * w
    y = a + (r1 - a) * np.sin(phi) ** 2
    dy = (r1 - a) * 2 * np.sin(phi) * np.cos(phi)
    f = 2 * np.sqrt(np.maximum(r2 * r2 - (y - y0) ** 2, 0)) * 2 * np.sqrt(np.maximum(r1 * r1 - y * y, 0))
    out[f'pipes overlap {-off} mm'] = float(np.sum(wp * f * dy)) / 4.0
print(json.dumps(out, indent=1))
HERE = os.path.dirname(os.path.abspath(__file__))
for fn in ('kernel_c.json', 'occt_kernel_c.json', 'occt_f1e-04_kernel_c.json', 'occt_f1e-06_kernel_c.json'):
    p = os.path.join(HERE, 'out', fn)
    if os.path.exists(p):
        rows = {r['case']: r for r in json.load(open(p))['pipes']}
        print(fn, {k: '%.2e' % ((rows[k]['exact_overlap_volume'] - v) / v) for k, v in out.items() if k in rows})
json.dump(out, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'out', 'pipes_reference.json'), 'w'), indent=1)
