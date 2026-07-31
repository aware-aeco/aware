// Tests for `read-model` — the WHOLE file as reference geometry (borrowed geometry a consumer
// overlays but never owns), as opposed to `extract`'s one-connection-to-import job.
//
// Real files again, deliberately: the three failure modes this command exists to survive — instanced
// geometry, a schema with zero profile definitions, and a model at real site coordinates — are all
// properties of real exports that a synthetic fixture would define away. See the reference-objects
// design doc §8 for what each file proves.
import test from 'node:test';
import assert from 'node:assert/strict';
import { existsSync } from 'node:fs';
import { join } from 'node:path';
import { openApi, closeApi, readModel } from './index.mjs';

const DOWNLOADS = join(process.env.USERPROFILE ?? process.env.HOME ?? '', 'Downloads');
const sample = (name) => (existsSync(join(DOWNLOADS, name)) ? join(DOWNLOADS, name) : null);
const skipReason = (name) => `sample file not present: ${join(DOWNLOADS, name)} — see the reference-objects design doc §8`;

async function read(name, maxVertices) {
  const h = await openApi(sample(name));
  try {
    return readModel(h.api, h.modelID, maxVertices);
  } finally {
    closeApi(h);
  }
}

// The extent of every returned object's vertices, in mm.
function extent(objects) {
  const min = [Infinity, Infinity, Infinity], max = [-Infinity, -Infinity, -Infinity];
  for (const o of objects) {
    for (let i = 0; i + 2 < o.positions.length; i += 3) {
      for (let k = 0; k < 3; k++) {
        const v = o.positions[i + k];
        if (v < min[k]) min[k] = v;
        if (v > max[k]) max[k] = v;
      }
    }
  }
  return { min, max, span: max.map((v, k) => v - min[k]) };
}

test('our own export comes back at true size, every member present', async (t) => {
  if (!sample('example-steel-framing.ifc')) return t.skip(skipReason('example-steel-framing.ifc'));
  const out = await read('example-steel-framing.ifc');
  assert.equal(out.objects.length, 13); // 6 columns + 7 beams
  const { span } = extent(out.objects);
  // The 12 m x 6 m grid, plus section overhang; columns 4500 tall plus beam depth. Measured 2026-07-25.
  // This is the headline test: it is OUR export, so a reference of it must coincide with the native model.
  assert.ok(Math.abs(span[0] - 12150) < 100, `expected ~12150 mm across the grid, got ${span[0]}`);
  assert.ok(Math.abs(span[2] - 6150) < 100, `expected ~6150 mm, got ${span[2]}`);
  assert.ok(Math.abs(span[1] - 4625) < 100, `expected ~4625 mm tall, got ${span[1]}`);
});

test('profile names ride along, because conversion must never measure a section off a mesh', async (t) => {
  if (!sample('example-steel-framing.ifc')) return t.skip(skipReason('example-steel-framing.ifc'));
  const out = await read('example-steel-framing.ifc');
  const profiles = new Set(out.objects.map((o) => o.profile).filter(Boolean));
  // In this very file W10x33 is written as a 150x250 box while the true section is 247x202 — so a
  // converter that measured the mesh would come out ~25% narrow and nothing on screen would look wrong.
  // Carrying the NAME is what makes the catalogue lookup possible.
  //
  // Asserted VERBATIM, including the lowercase "x". The name is passed through exactly as authored and
  // is not normalised here — normalising at the reader would hide how much designation spelling varies
  // between exporters. The consequence belongs downstream: catalogue lookup at conversion must match
  // case-insensitively, or "W10x33" will miss a catalogue that stores "W10X33".
  assert.ok(profiles.has('W10x33'), `expected a W10x33 profile name, got ${[...profiles]}`);
  assert.ok(profiles.has('HSS6x6x3/8'), `expected an HSS6x6x3/8 profile name, got ${[...profiles]}`);
});

test('instanced geometry is not silently dropped', async (t) => {
  if (!sample('11134_V_Motebello_Heistopp_Rev.ifc')) return t.skip(skipReason('11134_V_Motebello_Heistopp_Rev.ifc'));
  const out = await read('11134_V_Motebello_Heistopp_Rev.ifc');
  // 19 objects served from 14 shapes via mapped items. A reader that de-duplicated by geometry
  // expressID would return 14 and silently lose five walls — the failure this assertion exists to catch.
  assert.equal(out.objects.length, 19);
});

test('a mesh-only file with zero profile definitions still returns geometry', async (t) => {
  if (!sample('Building-Structural.ifc')) return t.skip(skipReason('Building-Structural.ifc'));
  const out = await read('Building-Structural.ifc');
  assert.ok(out.objects.length >= 16, `expected at least 16 objects, got ${out.objects.length}`);
  // Every returned object must be renderable — a degenerate part is worse than an absent one,
  // because it looks like successful loading.
  for (const o of out.objects) {
    assert.ok(o.positions.length >= 9 && o.positions.length % 3 === 0, `${o.name}: bad positions`);
    assert.ok(o.indices.length >= 3, `${o.name}: bad indices`);
  }
});

test('material rides along, because it is the signal that says "do not convert this"', async (t) => {
  if (!sample('Building-Structural.ifc')) return t.skip(skipReason('Building-Structural.ifc'));
  const out = await read('Building-Structural.ifc');
  const materials = out.objects.map((o) => o.material);
  // The beams here are named "girder" but are wood_spruce_beam. Type-based mapping would happily
  // convert them into steel; the material string is what stops it.
  assert.ok(materials.some((m) => m && /wood/i.test(m)), `expected a timber material, got ${materials}`);
  // EVERY object must resolve a material, not just some. IFC allows the material to hang off the
  // element TYPE with the occurrence overriding it, and following only direct
  // IfcRelAssociatesMaterial links returned null for ordinary objects in this very file — a null
  // here silently re-arms the "convert timber as steel" mistake the field exists to prevent.
  assert.equal(materials.filter(Boolean).length, materials.length,
    `every object should resolve a material; got ${materials.filter(Boolean).length}/${materials.length}`);
});

test('nothing is dropped silently — a skipped count is always reported', async (t) => {
  if (!sample('Building-Structural.ifc')) return t.skip(skipReason('Building-Structural.ifc'));
  const out = await read('Building-Structural.ifc');
  // The command claims to return every element, so the objects it could not draw must be COUNTED
  // rather than quietly missing. A consumer comparing this against probe's element count needs to
  // know the difference is accounted for.
  assert.equal(typeof out.skipped, 'number');
  assert.equal(out.skipped, 0);
});

test('assembly children inherit their storey instead of reporting null', async (t) => {
  if (!sample('example-steel-framing.ifc')) return t.skip(skipReason('example-steel-framing.ifc'));
  const out = await read('example-steel-framing.ifc');
  // IFC forbids an assembly's parts from also being spatially contained, so reading direct
  // containment alone reports storey:null for every part of every assembly.
  assert.ok(out.objects.every((o) => o.storey !== null), 'every object should resolve a storey');
});

test('the vertex budget aborts DURING tessellation, not after it', async (t) => {
  if (!sample('Building-Structural.ifc')) return t.skip(skipReason('Building-Structural.ifc'));
  // A cap enforced after read-model returns would fire only once the payload that causes the freeze
  // has already been built and serialised. Prove the breaker trips mid-walk by setting it absurdly low.
  await assert.rejects(
    async () => read('Building-Structural.ifc', 10),
    /too complex/i,
  );
});

test('a generous budget does not trip', async (t) => {
  if (!sample('Building-Structural.ifc')) return t.skip(skipReason('Building-Structural.ifc'));
  const out = await read('Building-Structural.ifc', 8_000_000);
  assert.ok(out.objects.length >= 16);
});
