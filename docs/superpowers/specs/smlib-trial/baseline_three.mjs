// Baseline: today's FloLess cope path, run headless. Same engine (three-bvh-csg Evaluator, SUBTRACTION), same
// cutter construction as web/steel-3d-view.js applyCopes (ExtrudeGeometry, curveSegments 8, quadratic re-entrant
// corner), applied to the same wide-flange outline as the kernel run. Usage:
//   node --import ./three_register.mjs baseline_three.mjs
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import * as THREE from 'three';
import { Brush, Evaluator, SUBTRACTION } from 'three-bvh-csg';

const here = path.dirname(fileURLToPath(import.meta.url));
const out = path.join(here, 'out');
fs.mkdirSync(path.join(out, 'mesh'), { recursive: true });
const cases = JSON.parse(fs.readFileSync(path.join(out, 'cases_a.json'), 'utf8'));  // random cases carry a placement for the KERNEL run; the app cuts in the member-local frame, so it is ignored here
const SEC = { d: 358, bf: 172, tf: 13, tw: 8, r: 12 };
const LEN = 3000;
const ARC = 8;

function outline() {
  const { d, bf, tf, tw, r } = SEC; const hx = bf / 2, hy = d / 2, w = tw / 2;
  const s = new THREE.Shape();
  const arcPts = (cx, cy, a0, a1) => { const p = []; for (let i = 0; i <= ARC; i++) { const a = (a0 + (a1 - a0) * i / ARC) * Math.PI / 180; p.push([cx + r * Math.cos(a), cy + r * Math.sin(a)]); } return p; };
  const pts = [[-hx, -hy], [hx, -hy], [hx, -hy + tf], [w + r, -hy + tf], ...arcPts(w + r, -hy + tf + r, 270, 180),
    [w, hy - tf - r], ...arcPts(w + r, hy - tf - r, 180, 90), [hx, hy - tf], [hx, hy], [-hx, hy], [-hx, hy - tf],
    [-w - r, hy - tf], ...arcPts(-w - r, hy - tf - r, 90, 0), [-w, -hy + tf + r], ...arcPts(-w - r, -hy + tf + r, 360, 270), [-hx, -hy + tf]];
  const dedup = pts.filter((p, i) => i === 0 || Math.hypot(p[0] - pts[i - 1][0], p[1] - pts[i - 1][1]) > 1e-9);
  s.moveTo(...dedup[0]); for (const p of dedup.slice(1)) s.lineTo(...p); s.closePath();
  return s;
}

// the cutter profile in (u=y, v=z): rectangle with ONLY the re-entrant corner rounded, via a quadratic curve (as in the app)
function cutterShape(y0, zlo, zhi, yhi, rc) {
  const s = new THREE.Shape();
  if (rc > 0) {
    s.moveTo(y0, zlo); s.lineTo(y0, zhi - rc); s.quadraticCurveTo(y0, zhi, y0 + rc, zhi); s.lineTo(yhi, zhi); s.lineTo(yhi, zlo); s.closePath();
  } else { s.moveTo(y0, zlo); s.lineTo(y0, zhi); s.lineTo(yhi, zhi); s.lineTo(yhi, zlo); s.closePath(); }
  return s;
}

function brushOf(geom, mat) { const b = new Brush(geom); b.applyMatrix4(mat || new THREE.Matrix4()); b.updateMatrixWorld(true); return b; }

function cutterBrush(c, variant) {
  const hy = SEC.d / 2, hx = SEC.bf / 2;
  let depth = c.depth, length = c.length, yhi, x0, wid, zlo = -80;
  if (variant === 'exact') { yhi = hy; x0 = -hx; wid = 2 * hx; zlo = 0; }
  else if (variant === 'app') { // web/steel-3d-view.js: every dimension +0.6 (centred), so the cut is 0.3 bigger each way
    depth += 0.3; length += 0.3; yhi = hy + 0.3; x0 = -hx - 0.3; wid = 2 * hx + 0.6; zlo = -0.3; }
  else { yhi = hy + 10; x0 = -hx - 10; wid = 2 * hx + 20; }
  const g = new THREE.ExtrudeGeometry(cutterShape(hy - depth, zlo, length, yhi, c.rc), { depth: wid, bevelEnabled: false, curveSegments: ARC });
  // local (u,v,w) -> world (y,z,x)
  const m = new THREE.Matrix4().makeBasis(new THREE.Vector3(0, 1, 0), new THREE.Vector3(0, 0, 1), new THREE.Vector3(1, 0, 0));
  m.setPosition(x0, 0, 0);
  return brushOf(g, m);
}

function slabBrush(angDeg) {
  const g = new THREE.BoxGeometry(1200, 1200, 1200); g.translate(0, 0, -600);
  const m = new THREE.Matrix4().makeRotationY(angDeg * Math.PI / 180);
  return brushOf(g, m);
}

const ev = new Evaluator(); ev.useGroups = false;
const results = [];
for (const c of cases) {
  const variants = c.exact ? ['exact'] : ['pad10', 'app'];
  if (/^a[1-6]/.test(c.id)) { variants.length = 0; variants.push('exact', 'pad10', 'app'); }
  for (const variant of variants) {
    const row = { id: c.id, variant };
    try {
      const memberGeo = new THREE.ExtrudeGeometry(outline(), { depth: LEN, bevelEnabled: false, curveSegments: ARC });
      let res = brushOf(memberGeo);
      const t0 = performance.now();
      res = ev.evaluate(res, cutterBrush(c, variant), SUBTRACTION);
      if (c.skew !== 0) res = ev.evaluate(res, slabBrush(c.skew), SUBTRACTION);
      row.ms = performance.now() - t0;
      const g = res.geometry.index ? res.geometry.toNonIndexed() : res.geometry;
      const pos = g.attributes.position;
      const f = Array.from(pos.array);
      row.tris = pos.count / 3;
      fs.writeFileSync(path.join(out, 'mesh', `three_${c.id}_${variant}.json`), JSON.stringify(f));
    } catch (e) { row.error = String(e && e.message || e); }
    results.push(row);
  }
}
fs.writeFileSync(path.join(out, 'three_a.json'), JSON.stringify(results, null, 1));
console.log(results.length, 'runs;', results.filter((r) => r.error).length, 'threw');
