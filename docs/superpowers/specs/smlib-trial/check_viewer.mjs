// Feeds the kernel's tessellation (positions + indices, as dumped by run_e.py) into FloLess' own vendored three.js
// BufferGeometry, the way the viewer would, and reports what three sees. node --import ./three_register.mjs check_viewer.mjs
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import * as THREE from 'three';

const here = path.dirname(fileURLToPath(import.meta.url));
for (const ch of ['0.1', '0.01']) {
  const j = JSON.parse(fs.readFileSync(path.join(here, 'out', 'mesh', `viewer_ch${ch}.json`), 'utf8'));
  const g = new THREE.BufferGeometry();
  g.setAttribute('position', new THREE.Float32BufferAttribute(j.positions, 3));
  g.setIndex(j.indices);
  const t0 = performance.now();
  g.computeVertexNormals();
  g.computeBoundingBox();
  const ms = performance.now() - t0;
  // signed volume of the indexed mesh as three reads it
  const p = g.attributes.position.array, ix = g.index.array; let v = 0;
  for (let i = 0; i < ix.length; i += 3) {
    const a = ix[i] * 3, b = ix[i + 1] * 3, c = ix[i + 2] * 3;
    v += (p[a] * (p[b + 1] * p[c + 2] - p[b + 2] * p[c + 1]) - p[a + 1] * (p[b] * p[c + 2] - p[b + 2] * p[c]) + p[a + 2] * (p[b] * p[c + 1] - p[b + 1] * p[c])) / 6;
  }
  console.log(JSON.stringify({ chord: ch, verts: g.attributes.position.count, tris: ix.length / 3, signedVolume_mm3: v, bbox: g.boundingBox.min.toArray().map((x) => +x.toFixed(3)).concat(g.boundingBox.max.toArray().map((x) => +x.toFixed(3))), computeNormals_ms: +ms.toFixed(2) }));
}
