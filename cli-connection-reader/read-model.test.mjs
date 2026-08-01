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
import { openApi, closeApi, readModel, probeModel, toWebIfcYUp, mergeInherited } from './index.mjs';

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

// Both commands against ONE open of the same file — the comparison the frame test is about.
async function probeAndRead(path) {
  const h = await openApi(path);
  try {
    return { probe: probeModel(h.api, h.modelID), model: readModel(h.api, h.modelID) };
  } finally {
    closeApi(h);
  }
}

// The extent of every returned object's vertices, in mm. `map` re-reads each vertex in another frame
// (the #343 test passes `toWebIfcYUp` to measure what read-model used to return).
function extent(objects, map = (p) => p) {
  const min = [Infinity, Infinity, Infinity], max = [-Infinity, -Infinity, -Infinity];
  for (const o of objects) {
    for (let i = 0; i + 2 < o.positions.length; i += 3) {
      const p = map([o.positions[i], o.positions[i + 1], o.positions[i + 2]]);
      for (let k = 0; k < 3; k++) {
        const v = p[k];
        if (v < min[k]) min[k] = v;
        if (v > max[k]) max[k] = v;
      }
    }
  }
  return { min, max, span: max.map((v, k) => v - min[k]), ctr: max.map((v, k) => (v + min[k]) / 2) };
}

test('our own export comes back at true size, every member present', async (t) => {
  if (!sample('example-steel-framing.ifc')) return t.skip(skipReason('example-steel-framing.ifc'));
  const out = await read('example-steel-framing.ifc');
  assert.equal(out.objects.length, 13); // 6 columns + 7 beams
  const { span } = extent(out.objects);
  // The 12 m x 6 m grid, plus section overhang; columns 4500 tall plus beam depth. Measured 2026-07-25.
  // This is the headline test: it is OUR export, so a reference of it must coincide with the native model.
  //
  // Axis order is the FILE's own Z-up world frame (#343, re-measured 2026-08-01): grid along X and Y,
  // height in Z. Until read-model undid web-ifc's baked Z-up -> Y-up rotation these read 12150 / 4625 /
  // 6150 — the same box, lying on its side, in a frame nothing else in this agent used.
  assert.ok(Math.abs(span[0] - 12150) < 100, `expected ~12150 mm across the grid, got ${span[0]}`);
  assert.ok(Math.abs(span[1] - 6150) < 100, `expected ~6150 mm, got ${span[1]}`);
  assert.ok(Math.abs(span[2] - 4625) < 100, `expected ~4625 mm tall, got ${span[2]}`);
});

// ── #343: probe and read-model must answer in the SAME frame ────────────────────────────────────
//
// `probe` reads IfcCartesianPoints straight off the file, so its bbox is in the file's own Z-up world
// frame. `read-model` goes through web-ifc, which bakes a fixed Z-up -> Y-up rotation into every flat
// mesh transform. Leaving that in made the two commands describe the same file in different frames:
// every reference model rendered on its side, and probe's bbox could not be used to sanity-check the
// mesh it was next to. The consumer that found it had to add a rotation of its own — a workaround the
// next consumer would have had to rediscover.
//
// These fixtures are IN-REPO, so this runs everywhere; the Downloads-gated test below is the same
// claim measured against a full building.
const FRAME_FIXTURES = ['baseplate-bp1.ifc', 'shearplate-sp1.ifc', 'baseplate-rot.ifc'];

// Height, not position: all three fixtures sit at a site offset and baseplate-rot is yawed 30°, so a
// plan-position claim would be about placement composition (which probe's bbox deliberately ignores —
// see probe.md) rather than about the frame. The VERTICAL range is yaw-invariant, so it isolates the
// one question this test asks: do the two commands agree about which axis is up?
const overlaps = (a, b) => a[0] <= b[1] && b[0] <= a[1];

test('probe and read-model agree about which axis is up (#343)', async () => {
  for (const name of FRAME_FIXTURES) {
    const { probe, model } = await probeAndRead(join('test-fixtures', name));
    assert.ok(probe.bbox, `${name}: probe should establish a bbox`);
    assert.ok(model.objects.length > 0, `${name}: read-model should return geometry`);
    const probeZ = [probe.bbox.min[2], probe.bbox.max[2]];

    const m = extent(model.objects);
    assert.ok(overlaps([m.min[2], m.max[2]], probeZ),
      `${name}: the mesh spans Z ${Math.round(m.min[2])}..${Math.round(m.max[2])} mm, but the file's own ` +
      `points span Z ${probeZ.map(Math.round)} — read-model is not answering in the file's frame`);

    // The arm that makes the check discriminating: the SAME meshes re-read in web-ifc's Y-up frame —
    // exactly what this command returned before the fix — must FAIL it. Without this, a check that
    // happened to hold in either frame would look like proof and prove nothing.
    const y = extent(model.objects, toWebIfcYUp);
    assert.ok(!overlaps([y.min[2], y.max[2]], probeZ),
      `${name}: the pre-fix Y-up reading spans Z ${Math.round(y.min[2])}..${Math.round(y.max[2])} and still ` +
      `overlaps the file's ${probeZ.map(Math.round)} — this fixture cannot tell the two frames apart, so it ` +
      `does not belong in FRAME_FIXTURES`);
  }
});

test('both commands STATE their frame, because no version number can (#343)', async () => {
  // The agent manifest cannot answer "which frame did I just get": the bridge binary that produces
  // the geometry is installed separately (`aware sidecar install connection-reader`) and a stale one
  // only warns before running, and an app's `requires:` pin is enforced at neither compile nor run
  // time (measured 2026-08-01). The producing binary saying so in its own payload is the only
  // trustworthy answer — so the field is part of the contract, not a nicety.
  const { probe, model } = await probeAndRead(join('test-fixtures', 'baseplate-bp1.ifc'));
  assert.equal(probe.frame, 'z-up');
  assert.equal(model.frame, 'z-up');
  // And it must describe the geometry actually returned, not be a constant nobody checks: the same
  // fixture's ~1125 mm column-plus-plate height is in Z, which is what "z-up" claims.
  const { span } = extent(model.objects);
  assert.ok(span[2] > span[0] && span[2] > span[1],
    `frame says z-up but the tall axis is not Z: span ${span.map(Math.round)}`);
});

test('a base plate is horizontal in the frame read-model returns (#343)', async () => {
  // Ground truth from the fixture generators, not from probe: make-baseplate.py authors a HORIZONTAL
  // 400x400 plate with a column stub standing on it, so the assembly is ~400 mm in plan and ~1125 mm
  // tall. In the file's Z-up frame the tall axis is therefore Z. Before the fix that height sat in Y —
  // which is why every reference model rendered on its side.
  for (const name of ['baseplate-bp1.ifc', 'baseplate-rot.ifc']) {
    const { model } = await probeAndRead(join('test-fixtures', name));
    const { span } = extent(model.objects);
    assert.ok(span[2] > 1000, `${name}: expected the ~1125 mm column-plus-plate height in Z, got ${Math.round(span[2])}`);
    assert.ok(span[0] < 600 && span[1] < 600,
      `${name}: expected a ~400 mm plan footprint, got ${span.slice(0, 2).map(Math.round)}`);
  }
});

test('probe\'s bbox and read-model\'s mesh describe the same box, axis for axis (#343)', async (t) => {
  if (!sample('example-steel-framing.ifc')) return t.skip(skipReason('example-steel-framing.ifc'));
  const { probe, model } = await probeAndRead(sample('example-steel-framing.ifc'));
  const { span } = extent(model.objects);
  const probeSpan = probe.bbox.max.map((v, k) => v - probe.bbox.min[k]);
  // 12000 x 6000 x 4500 from the file's points; 12150 x 6150 x 4625 tessellated. The differences are
  // half-profile margins and beam depth — the point is that they now line up axis for axis, which is
  // what the manifest's "the file's own world frame" claims and what a consumer comparing the two needs.
  for (let k = 0; k < 3; k++) {
    assert.ok(Math.abs(span[k] - probeSpan[k]) < 250,
      `axis ${k}: probe says ${Math.round(probeSpan[k])} mm, the mesh says ${Math.round(span[k])} mm`);
  }
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

test('property sets ride along, because in a real file that is where the meaning is', async (t) => {
  if (!sample('11134_V_Motebello_Heistopp_Rev.ifc')) return t.skip(skipReason('11134_V_Motebello_Heistopp_Rev.ifc'));
  const out = await read('11134_V_Motebello_Heistopp_Rev.ifc');

  // THE FILE THAT MAKES THE CASE. Its objects carry almost nothing in the well-known fields — every
  // one is an IfcBuildingElementProxy — so a reader returning only name/type/storey/profile/material
  // hands a consumer an object tree in which nothing can be told apart. The properties are the answer.
  const sets = out.objects.flatMap((o) => o.propertySets);
  const values = sets.flatMap((s) => s.properties);
  assert.equal(sets.length, 31, 'the design doc measured 31 property sets in this file');
  assert.equal(values.length, 271, 'and 271 values across them');
  assert.ok(out.objects.every((o) => o.propertySets.length > 0), 'every object here carries properties');

  // VERBATIM — no normalisation, no translation, no renaming. These are vendor sets in Norwegian, and
  // showing them as authored is the entire value; a reader that tidied them would be discarding the
  // only thing that distinguishes one proxy from another.
  const names = new Set(sets.map((s) => s.name));
  assert.ok(names.has('AllplanAttributes'), `expected the vendor set verbatim, got ${[...names]}`);
  assert.ok(values.some((p) => /[æøå]/i.test(p.name) || /^V\d+$/.test(p.name)),
    'the authored property names must survive as written');
});

test('a standard property set reads its values as authored', async (t) => {
  if (!sample('Building-Architecture.ifc')) return t.skip(skipReason('Building-Architecture.ifc'));
  const out = await read('Building-Architecture.ifc');
  const slab = out.objects.find((o) => o.propertySets.some((s) => s.name === 'Pset_SlabCommon'));
  assert.ok(slab, 'the architecture file carries a standard Pset_SlabCommon');
  const props = slab.propertySets.find((s) => s.name === 'Pset_SlabCommon').properties;
  const value = (n) => (props.find((p) => p.name === n) || {}).value;

  // Strings, booleans and codes all come back as the file wrote them. `FireRating` in particular is
  // free text in IFC ("REI30"), so anything that tried to interpret it would be inventing meaning.
  assert.equal(value('FireRating'), 'REI30');
  assert.equal(value('AcousticRating'), '29dB Rw');
  assert.equal(value('IsExternal'), 'true');
  assert.equal(value('LoadBearing'), 'false');
});

// Type-level property inheritance, tested DIRECTLY rather than through a file.
//
// None of the four sample files exercises it: three have no IfcRelDefinesByType at all, and the fourth
// has a single type carrying sets with no occurrence of the same name to collide with. A file-based
// test would therefore go green without the merge rule ever running, which is worse than no test —
// so the rule is driven at its own boundary.

test('a type set the occurrence does not have is inherited whole', () => {
  const merged = mergeInherited([], [{ name: 'Pset_WallCommon', properties: [{ name: 'FireRating', value: 'REI60' }] }]);
  assert.deepEqual(merged, [{ name: 'Pset_WallCommon', properties: [{ name: 'FireRating', value: 'REI60' }] }]);
});

test('a same-named type set merges PROPERTY BY PROPERTY, and type-only properties survive', () => {
  // THE DATA-LOSS CASE. Set-level precedence — "the occurrence declared this set, so use only that" —
  // would silently drop LoadBearing, which the file plainly states. It is also what the sibling
  // `ifc-inspector.entities.get-by-guid` does, and two agents reading one file must not disagree.
  const own = [{ name: 'Pset_WallCommon', properties: [{ name: 'FireRating', value: 'REI30' }] }];
  const type = [{ name: 'Pset_WallCommon', properties: [
    { name: 'FireRating', value: 'REI60' },      // the occurrence overrides this
    { name: 'LoadBearing', value: 'true' },      // and this must NOT be lost
  ] }];
  const merged = mergeInherited(own, type);
  assert.equal(merged.length, 1);
  const byName = Object.fromEntries(merged[0].properties.map((p) => [p.name, p.value]));
  assert.equal(byName.FireRating, 'REI30', 'the occurrence wins the property it declares');
  assert.equal(byName.LoadBearing, 'true', 'and the type keeps the one it does not');
});

test('merging never mutates its inputs, which are shared across every element', () => {
  // Parsed sets are cached and handed to every element that references them. Appending to one in place
  // would leak one element's type properties into another's — a corruption that grows with file size
  // and would look like the file saying something it does not.
  const own = [{ name: 'P', properties: [{ name: 'a', value: '1' }] }];
  const type = [{ name: 'P', properties: [{ name: 'b', value: '2' }] }];
  const merged = mergeInherited(own, type);
  assert.equal(merged[0].properties.length, 2);
  assert.equal(own[0].properties.length, 1, 'the occurrence set must be untouched');
  assert.equal(type[0].properties.length, 1, 'and so must the cached type set');
});

test('propertySets is ALWAYS an array, so absent and empty are not the same question', async (t) => {
  if (!sample('example-steel-framing.ifc')) return t.skip(skipReason('example-steel-framing.ifc'));
  const out = await read('example-steel-framing.ifc');
  // This file carries no property sets at all. The field must still be there and still be iterable:
  // a consumer rendering an inspect panel should print "no properties", not crash on undefined, and
  // should never have to distinguish "the reader did not look" from "the file did not say".
  assert.ok(out.objects.length > 0);
  for (const o of out.objects) {
    assert.ok(Array.isArray(o.propertySets), `${o.name}: propertySets must be an array`);
    assert.equal(o.propertySets.length, 0, `${o.name}: this file carries no property sets`);
  }
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
