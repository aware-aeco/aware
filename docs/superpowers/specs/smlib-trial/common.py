"""Shared helpers for the SMLib kernel trial (2026-11-23).

Units are millimetres. The member runs along +Z, the section lies in the XY plane:
x is the flange width direction, y the depth direction (y = +d/2 is the top flange).

Run every script with the interpreter's isolated mode:  python -I script.py
"""
import math
import time

import numpy as np

# A W360x57-like wide flange (rolled, with root fillets). Same numbers feed the kernel and the mesh baseline.
SEC = dict(d=358.0, bf=172.0, tf=13.0, tw=8.0, r=12.0)
L = 3000.0
ARC_SEG = 8  # segments per quarter arc in every mesh baseline (matches three ExtrudeGeometry curveSegments: 8)


def i_outline(d, bf, tf, tw, r, arc_seg=None):
    """Closed CCW outline of the I-section as a list of ('L', p0, p1) / ('A', centre, r, a0, a1) pieces
    (xy only). With arc_seg the arcs are expanded to a polygon instead (returns a list of (x, y))."""
    hx, hy, w = bf / 2, d / 2, tw / 2
    P = lambda x, y: (x, y)
    pcs = []
    if r > 0:
        pcs += [('L', P(-hx, -hy), P(hx, -hy)), ('L', P(hx, -hy), P(hx, -hy + tf)), ('L', P(hx, -hy + tf), P(w + r, -hy + tf)),
                ('A', P(w + r, -hy + tf + r), r, 180, 270),
                ('L', P(w, -hy + tf + r), P(w, hy - tf - r)),
                ('A', P(w + r, hy - tf - r), r, 90, 180),
                ('L', P(w + r, hy - tf), P(hx, hy - tf)), ('L', P(hx, hy - tf), P(hx, hy)), ('L', P(hx, hy), P(-hx, hy)),
                ('L', P(-hx, hy), P(-hx, hy - tf)), ('L', P(-hx, hy - tf), P(-w - r, hy - tf)),
                ('A', P(-w - r, hy - tf - r), r, 0, 90),
                ('L', P(-w, hy - tf - r), P(-w, -hy + tf + r)),
                ('A', P(-w - r, -hy + tf + r), r, 270, 360),
                ('L', P(-w - r, -hy + tf), P(-hx, -hy + tf)), ('L', P(-hx, -hy + tf), P(-hx, -hy))]
    else:
        v = [(-hx, -hy), (hx, -hy), (hx, -hy + tf), (w, -hy + tf), (w, hy - tf), (hx, hy - tf), (hx, hy), (-hx, hy),
             (-hx, hy - tf), (-w, hy - tf), (-w, -hy + tf), (-hx, -hy + tf)]
        pcs = [('L', v[i], v[(i + 1) % len(v)]) for i in range(len(v))]
    if arc_seg is None:
        return pcs
    poly = []
    for pc in pcs:
        if pc[0] == 'L':
            poly.append(pc[1])
            poly.append(pc[2])
        else:
            _, c, rr, a0, a1 = pc
            # arcs in `pcs` are stored CCW (a0 < a1) but the outline walks some of them clockwise
            n = arc_seg
            pts = [(c[0] + rr * math.cos(math.radians(a0 + (a1 - a0) * i / n)), c[1] + rr * math.sin(math.radians(a0 + (a1 - a0) * i / n))) for i in range(n + 1)]
            if math.dist(pts[0], poly[-1]) > math.dist(pts[-1], poly[-1]):
                pts.reverse()
            poly.extend(pts)
    # remove consecutive duplicates
    out = []
    for p in poly:
        if not out or math.dist(p, out[-1]) > 1e-9:
            out.append(p)
    if math.dist(out[0], out[-1]) < 1e-9:
        out.pop()
    return out


def section_area(d, bf, tf, tw, r):
    return 2 * bf * tf + (d - 2 * tf) * tw + (4 - math.pi) * r * r


# ---------- independent reference volume (numeric, no kernel, no mesh) ----------
def section_y_intervals(x, d, bf, tf, tw, r):
    """The y-intervals of the section at abscissa x (exact, including the root fillets)."""
    hx, hy, w = bf / 2, d / 2, tw / 2
    ax = abs(x)
    if ax > hx:
        return []
    iv = [(-hy, -hy + tf), (hy - tf, hy)]
    if ax <= w:
        iv.append((-hy + tf, hy - tf))
    elif r > 0 and ax < w + r:
        g = r - math.sqrt(max(0.0, r * r - (w + r - ax) ** 2))
        iv[0] = (-hy, -hy + tf + g)
        iv[1] = (hy - tf - g, hy)
    return iv


_GX, _GW = np.polynomial.legendre.leggauss(12)


def _gl_pieces(breaks, cells):
    """Gauss-Legendre nodes/weights over consecutive `breaks`, each piece split into `cells` sub-cells."""
    xs, ws = [], []
    for a, b in zip(breaks[:-1], breaks[1:]):
        if b - a < 1e-12:
            continue
        edges = np.linspace(a, b, cells + 1)
        for lo, hi in zip(edges[:-1], edges[1:]):
            xs.append(0.5 * (hi - lo) * _GX + 0.5 * (hi + lo))
            ws.append(0.5 * (hi - lo) * _GW)
    return np.concatenate(xs), np.concatenate(ws)


def reference_volume(zkeep, d, bf, tf, tw, r, ybreaks=(), cells=24):
    """Volume of {(x,y,z) in section, zkeep(x,y) <= z <= L}, by piecewise Gauss-Legendre quadrature over the section
    (breaks at the web and fillet edges in x, and at `ybreaks` plus the section's own y-interval ends). zkeep(x, y)
    is vectorised over y. This is an independent numerical reference (no kernel, no mesh); it is NOT exact, expect
    agreement to about 1e-7 relative on smooth cases."""
    hx, w = bf / 2, tw / 2
    xb = sorted({-hx, -w - r, -w, w, w + r, hx})
    xs, xw = _gl_pieces(xb, cells)
    tot = 0.0
    for x, wx in zip(xs, xw):
        for (y0, y1) in section_y_intervals(float(x), d, bf, tf, tw, r):
            yb = sorted({y0, y1} | {b for b in ybreaks if y0 < b < y1})
            ys, yw = _gl_pieces(yb, max(2, cells // 4))
            z0 = np.clip(zkeep(float(x), ys), 0.0, L)
            tot += wx * float(np.sum(yw * (L - z0)))
    return tot


def cope_zc(y, d, tf, depth, length, rc):
    """z of the cope cutter's far face at height y (cutter occupies z <= zc(y) for y >= hy - depth),
    with the re-entrant corner (z=length, y=hy-depth) rounded by rc. Vectorised."""
    hy = d / 2
    yb = hy - depth
    zc = np.where(y >= yb, length, -1e9)
    if rc > 0:
        inr = (y >= yb) & (y < yb + rc)
        zc = np.where(inr, length - rc + np.sqrt(np.maximum(0.0, rc * rc - (yb + rc - y) ** 2)), zc)
    return zc


# ---------- mesh metrics (used for the baseline AND for the kernel's tessellation) ----------
def mesh_metrics(V, F, area_eps=1e-9, overlap_eps=1e-3, weld=1e-6):
    """V (n,3) float, F (m,3) int. Returns watertight/manifold facts, volume, zero-area tris, and the area of
    coincident OPPOSITE-facing triangle pairs (the #4005 'sheet' signature)."""
    V = np.asarray(V, float)
    F = np.asarray(F, int)
    a, b, c = V[F[:, 0]], V[F[:, 1]], V[F[:, 2]]
    cr = np.cross(b - a, c - a)
    ar = 0.5 * np.linalg.norm(cr, axis=1)
    vol = float(np.sum(np.einsum('ij,ij->i', a, np.cross(b, c))) / 6.0)
    # edge manifoldness by position-welded vertices (1e-6 mm)
    key = np.round(V / weld).astype(np.int64)
    _, inv = np.unique(key, axis=0, return_inverse=True)
    inv = inv.reshape(-1)
    Fw = inv[F]
    from collections import Counter
    cnt = Counter()
    dirc = Counter()
    for t in Fw:
        for i in range(3):
            u, v = int(t[i]), int(t[(i + 1) % 3])
            if u == v:
                continue
            cnt[(min(u, v), max(u, v))] += 1
            dirc[(u, v)] += 1
    open_edges = sum(1 for k, n in cnt.items() if n == 1)
    nonmanifold_edges = sum(1 for k, n in cnt.items() if n > 2)
    inconsistent = sum(1 for (u, v) in dirc if cnt[(min(u, v), max(u, v))] == 2 and (v, u) not in dirc)  # two faces, same winding
    # coincident opposite faces: bucket by plane (normal rounded, offset rounded), then 2D overlap by triangle-vs-triangle in-plane
    nz = ar > area_eps
    n = np.zeros_like(cr)
    n[nz] = cr[nz] / (2 * ar[nz])[:, None]
    off = np.einsum('ij,ij->i', n, a)
    buckets = {}
    for i in np.nonzero(nz)[0]:
        k = (tuple(np.round(n[i], 4)), round(float(off[i]), 3))
        buckets.setdefault(k, []).append(i)
    overlap_area = 0.0
    pairs = 0
    for k, idx in buckets.items():
        nk = np.array(k[0])
        # the same geometric plane seen from the other side: normal flipped, offset flipped
        k2 = (tuple(np.round(-nk, 4)), round(-k[1], 3))
        if k2 in buckets and k < k2:
            ia, ib = idx, buckets[k2]
            ov = _plane_overlap(V, F, nk, ia, ib)
            if ov > overlap_eps:
                overlap_area += ov
                pairs += 1
    return dict(tris=int(len(F)), verts=int(len(V)), volume=vol, zero_area_tris=int((ar <= area_eps).sum()),
                open_edges=int(open_edges), nonmanifold_edges=int(nonmanifold_edges), inconsistent_edges=int(inconsistent),
                coincident_opposite_area_mm2=float(overlap_area), coincident_opposite_planes=int(pairs))


def _plane_overlap(V, F, n, ia, ib):
    """Area of intersection between the union of triangles ia and the union of triangles ib, which lie in one plane
    with normal n. Uses shapely-free Sutherland-Hodgman clipping pairwise (small sets only)."""
    # basis in plane
    ref = np.array([1, 0, 0]) if abs(n[0]) < 0.9 else np.array([0, 1, 0])
    e1 = np.cross(n, ref)
    e1 /= np.linalg.norm(e1)
    e2 = np.cross(n, e1)
    P2 = lambda i: [(float(np.dot(V[j], e1)), float(np.dot(V[j], e2))) for j in F[i]]
    A = [P2(i) for i in ia]
    B = [P2(i) for i in ib]
    tot = 0.0
    for ta in A:
        for tb in B:
            tot += _clip_area(ta, tb)
    return tot


def _orient(p):
    a = 0.5 * sum(p[i][0] * p[(i + 1) % len(p)][1] - p[(i + 1) % len(p)][0] * p[i][1] for i in range(len(p)))
    return p if a >= 0 else p[::-1]


def _clip_area(subject, clip):
    subject, clip = _orient(subject), _orient(clip)
    out = subject
    for i in range(len(clip)):
        a, b = clip[i], clip[(i + 1) % len(clip)]
        inp, out = out, []
        if not inp:
            return 0.0
        def inside(p):
            return (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0]) >= -1e-12
        def inter(p, q):
            x1, y1, x2, y2 = p[0], p[1], q[0], q[1]
            x3, y3, x4, y4 = a[0], a[1], b[0], b[1]
            den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4)
            if abs(den) < 1e-18:
                return q
            t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den
            return (x1 + t * (x2 - x1), y1 + t * (y2 - y1))
        s = inp[-1]
        for e in inp:
            if inside(e):
                if not inside(s):
                    out.append(inter(s, e))
                out.append(e)
            elif inside(s):
                out.append(inter(s, e))
            s = e
    if len(out) < 3:
        return 0.0
    return abs(0.5 * sum(out[i][0] * out[(i + 1) % len(out)][1] - out[(i + 1) % len(out)][0] * out[i][1] for i in range(len(out))))


def timed(fn, *a, **k):
    t = time.perf_counter()
    r = fn(*a, **k)
    return r, (time.perf_counter() - t) * 1000.0


def poly_to_arrays(poly_brep):
    arr = poly_brep.to_numpy_arrays()
    pts = np.asarray(arr['points'], float)
    f = np.asarray(arr['faces'], int)
    # faces are [3, i, j, k, 3, i, j, k, ...]
    if len(f) % 4 != 0 or not np.all(f[0::4] == 3):
        raise ValueError('faces not all triangles')
    tri = f.reshape(-1, 4)[:, 1:]
    return pts, tri
