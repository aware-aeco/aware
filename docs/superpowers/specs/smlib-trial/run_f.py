"""(f) Extra robustness probe on ordinary steel-detailing operations, with coincident faces by construction:
  f1  bolt-hole plates: a plate minus an n-hole grid of cylinders (cylinder ends overshoot the plate, or are EXACTLY coplanar with it)
  f2  a plate unioned onto a beam's top flange (plate's underside coplanar with the flange face; in half the cases one plate
      edge is also coplanar with the flange edge), then the flange-face contact is a shared, partly overlapping face
Random rigid placement up to 50 m. Reference volumes are closed-form. Usage: python -I run_f.py [N] -> out/kernel_f.json"""
import json
import math
import os
import random
import statistics as st
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import usd_brep as sm  # noqa: E402

from common import SEC, poly_to_arrays, mesh_metrics  # noqa: E402
from cases import OUT  # noqa: E402
from run_kernel import rot_frame  # noqa: E402
import kernel_ops as ko  # noqa: E402


def f1(rnd):
    T = rnd.choice([10.0, 12.7, 19.05, 25.4])
    d = rnd.choice([17.5, 20.5, 23.5, 26.5])  # standard hole diameters
    nx, ny = rnd.randint(1, 4), rnd.randint(1, 4)
    pitch = rnd.uniform(2.67 * d, 3.2 * d)
    edge = rnd.uniform(1.75 * d, 3.0 * d)  # the plate is sized FROM the grid so every hole lies fully inside it
    Ln, W = (nx - 1) * pitch + 2 * edge, (ny - 1) * pitch + 2 * edge
    exact = rnd.random() < 0.5
    plate = sm.create_box((0.0, 0.0, 0.0), Ln, W, T)
    x0 = Ln / 2 - (nx - 1) * pitch / 2
    y0 = W / 2 - (ny - 1) * pitch / 2
    holes = []
    for i in range(nx):
        for j in range(ny):
            if exact:
                h = sm.create_cylinder((x0 + i * pitch, y0 + j * pitch, 0.0), d / 2, T)  # end caps coplanar with the plate faces
            else:
                h = sm.create_cylinder((x0 + i * pitch, y0 + j * pitch, -5.0), d / 2, T + 10.0)
            holes.append(h)
    vol = Ln * W * T - nx * ny * math.pi * (d / 2) ** 2 * T
    return plate, holes, vol, dict(T=T, W=W, L=Ln, d=d, nx=nx, ny=ny, exact=exact)


def run_f1(n, seed=17):
    rnd = random.Random(seed)
    rows = []
    for i in range(n):
        plate, holes, vol, par = f1(rnd)
        row = dict(i=i, **par)
        try:
            xa, ya = rot_frame([rnd.gauss(0, 1) for _ in range(3)], rnd.uniform(0, 360))
            t = [rnd.uniform(-5e4, 5e4) for _ in range(3)]
            plate = ko.place(plate, t, xa, ya)
            holes = [ko.place(h, t, xa, ya) for h in holes]
            t0 = time.perf_counter()
            r = plate
            for h in holes:
                r = sm.boolean_difference(r, h)
            row['ms'] = (time.perf_counter() - t0) * 1000
            row['faces'] = r.face_count()
            row['expected_faces'] = 6 + par['nx'] * par['ny']
            row['solid'] = bool(r.is_manifold_solid())
            row['rel_err'] = (r.volume(1e-12) - vol) / vol
            row['ok'] = bool(row['solid'] and abs(row['rel_err']) < 1e-9 and row['faces'] == row['expected_faces'])
        except Exception as e:
            row['error'] = f'{type(e).__name__}: {e}'
            row['ok'] = False
        rows.append(row)
    return rows


def run_f2(n, seed=18):
    rnd = random.Random(seed)
    rows = []
    d, bf, tf = SEC['d'], SEC['bf'], SEC['tf']
    BL = 800.0
    for i in range(n):
        T = rnd.choice([10.0, 12.7, 16.0, 20.0])
        pl, pw = rnd.uniform(80, 300), rnd.uniform(40, bf)
        flush_edge = rnd.random() < 0.5
        px = -bf / 2 if flush_edge else rnd.uniform(-bf / 2 + 1, bf / 2 - pw - 1)
        pz = rnd.uniform(50, BL - pl - 50)
        row = dict(i=i, T=T, pl=pl, pw=pw, flush_edge=flush_edge)
        try:
            beam = ko.beam(length=BL)
            # plate on the TOP flange face y = d/2: build with its underside exactly at y = d/2
            plate = sm.create_box((px, d / 2, pz), pw, T, pl)  # (x, y, z) extents: width pw, thickness T (y), length pl (z)
            xa, ya = rot_frame([rnd.gauss(0, 1) for _ in range(3)], rnd.uniform(0, 360))
            t = [rnd.uniform(-5e4, 5e4) for _ in range(3)]
            beam, plate = ko.place(beam, t, xa, ya), ko.place(plate, t, xa, ya)
            t0 = time.perf_counter()
            r = sm.boolean_union(beam, plate)
            row['ms'] = (time.perf_counter() - t0) * 1000
            exp = (ko.beam(length=BL).volume(1e-12)) + pw * T * pl
            row['solid'] = bool(r.is_manifold_solid())
            row['rel_err'] = (r.volume(1e-12) - exp) / exp
            row['faces'] = r.face_count()
            row['ok'] = bool(row['solid'] and abs(row['rel_err']) < 1e-9)
        except Exception as e:
            row['error'] = f'{type(e).__name__}: {e}'
            row['ok'] = False
        rows.append(row)
    return rows


def summ(rows):
    okr = [r for r in rows if 'ms' in r]
    return dict(n=len(rows), ok=sum(r['ok'] for r in rows), errors=sum('error' in r for r in rows), ms_median=st.median(r['ms'] for r in okr) if okr else None,
                ms_max=max(r['ms'] for r in okr) if okr else None, max_abs_rel_err=max((abs(r['rel_err']) for r in okr if 'rel_err' in r), default=None),
                bad=[{k: v for k, v in r.items()} for r in rows if not r['ok']][:6])


if __name__ == '__main__':
    n = int(sys.argv[1]) if len(sys.argv) > 1 else 100
    a, b = run_f1(n), run_f2(n)
    out = dict(f1=summ(a), f2=summ(b), rows=dict(f1=a, f2=b))
    json.dump(out, open(os.path.join(OUT, 'kernel_f.json'), 'w'), indent=1)
    print(json.dumps(dict(f1=out['f1'], f2=out['f2']), indent=1))
