"""Case list shared by the kernel run (run_kernel.py) and the mesh baselines (baseline_three.mjs, baseline_manifold.py).
Writes out/cases_a.json. Deterministic (seeded)."""
import json
import math
import os
import random

OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'out')


def cases_a(n_random=120, seed=20261123):
    cs = [
        dict(id='a1-cope', depth=35.0, length=100.0, rc=12.0, skew=0.0, exact=False),
        dict(id='a2-cope+skew25', depth=35.0, length=100.0, rc=12.0, skew=25.0, exact=False),
        dict(id='a3-cope-coplanar', depth=35.0, length=100.0, rc=12.0, skew=0.0, exact=True),
        dict(id='a4-cope+skew-coplanar', depth=35.0, length=100.0, rc=12.0, skew=25.0, exact=True),
        # the cutter's face at exactly the flange inner face (y = hy - tf) and exactly the web-fillet tangent line
        dict(id='a5-cope-at-flange-face', depth=13.0, length=100.0, rc=0.0, skew=0.0, exact=False),
        dict(id='a6-cope-tangent-to-fillet', depth=25.0, length=100.0, rc=12.0, skew=0.0, exact=False),
    ]
    rnd = random.Random(seed)
    for i in range(n_random):
        rc = 0.0 if rnd.random() < 0.2 else rnd.uniform(3.0, 15.0)
        depth = rnd.uniform(8.0, 60.0)
        length = rnd.uniform(max(45.0, rc + 5), 200.0)
        skew = rnd.choice([0.0, 0.0, rnd.uniform(-40.0, 40.0)])
        # a random rigid placement (rotation about a random axis + a translation up to 50 m): the generic-orientation case
        axis = [rnd.gauss(0, 1) for _ in range(3)]
        n = math.sqrt(sum(a * a for a in axis))
        axis = [a / n for a in axis]
        cs.append(dict(id=f'r{i:03d}', depth=depth, length=length, rc=rc, skew=skew, exact=rnd.random() < 0.25,
                       place=dict(axis=axis, angle=rnd.uniform(0, 360), t=[rnd.uniform(-50000, 50000) for _ in range(3)])))
    return cs


if __name__ == '__main__':
    os.makedirs(OUT, exist_ok=True)
    cs = cases_a()
    json.dump(cs, open(os.path.join(OUT, 'cases_a.json'), 'w'), indent=1)
    print(len(cs), 'cases')
