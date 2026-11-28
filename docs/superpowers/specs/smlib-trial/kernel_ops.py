"""Kernel-side construction of the trial parts (usd_brep 1.0.2, Python bindings). Local frame, mm, member along +Z."""
import math

import usd_brep as sm

from common import SEC, L, i_outline


def _curves(pcs, plane_z=0.0):
    out = []
    for pc in pcs:
        if pc[0] == 'L':
            out.append(sm.create_line_segment((*pc[1], plane_z), (*pc[2], plane_z)))
        else:
            out.append(sm.create_arc((*pc[1], plane_z), pc[2], pc[3], pc[4]))
    return out


def beam(sec=SEC, length=L):
    """Wide-flange member: extruded exact outline (lines + circular root fillets)."""
    return sm.linear_sweep(_curves(i_outline(**sec)), (0, 0, 1), length)


def cutter_outline(y0, z_lo, z_hi, y_hi, rc, exact=False):
    """Profile in (u=y, v=z): rectangle [y0..y_hi] x [z_lo..z_hi] with the re-entrant corner (y0, z_hi) rounded by rc."""
    pcs = []
    if rc > 0:
        pcs = [('L', (y0, z_lo), (y0, z_hi - rc)),
               ('A', (y0 + rc, z_hi - rc), rc, 90, 180),  # corner arc, CCW range 90..180: the rounded re-entrant corner
               ('L', (y0 + rc, z_hi), (y_hi, z_hi)),
               ('L', (y_hi, z_hi), (y_hi, z_lo)),
               ('L', (y_hi, z_lo), (y0, z_lo))]
    else:
        pcs = [('L', (y0, z_lo), (y0, z_hi)), ('L', (y0, z_hi), (y_hi, z_hi)), ('L', (y_hi, z_hi), (y_hi, z_lo)), ('L', (y_hi, z_lo), (y0, z_lo))]
    return pcs


def cope_cutter(depth, length, rc, sec=SEC, pad=10.0, z_lo=-80.0, exact=False):
    """Cutter removing the top of the member end: y in [hy-depth, hy(+pad)], z in [z_lo, length], full width.
    exact=True makes it coplanar with the member's top face (y = hy), the member's sides (x = +-bf/2) and the end
    face (z = 0): the stress case a padded cutter hides."""
    hy, hx = sec['d'] / 2, sec['bf'] / 2
    y_hi = hy if exact else hy + pad
    x0 = -hx if exact else -hx - pad
    wid = 2 * hx if exact else 2 * hx + 2 * pad
    if exact:
        z_lo = 0.0
    prof = _curves(cutter_outline(hy - depth, z_lo, length, y_hi, rc))
    s = sm.linear_sweep(prof, (0, 0, 1), wid)  # swept along local w
    # local (u,v,w) -> world (y,z,x): cyclic permutation, a proper rotation
    pl = sm.Axis2Placement((x0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0))
    return sm.transform(s, pl)


def skew_slab(angle_deg):
    """Half-space (as a big slab) removing z < -x*tan(angle): its top face is the plane through the origin."""
    slab = sm.create_box((-600.0, -600.0, -1200.0), 1200.0, 1200.0, 1200.0)
    return sm.rotate(slab, (0.0, 0.0, 0.0), (0.0, 1.0, 0.0), angle_deg)


def place(obj, origin, xa, ya):
    return sm.transform(obj, sm.Axis2Placement(tuple(origin), tuple(xa), tuple(ya)))
