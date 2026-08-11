// Tests for `read-model` — the WHOLE file as reference geometry (borrowed geometry a consumer
// overlays but never owns), as opposed to `extract`'s one-connection-to-import job.
//
// Real files again, deliberately: the three failure modes this command exists to survive — instanced
// geometry, a schema with zero profile definitions, and a model at real site coordinates — are all
// properties of real exports that a synthetic fixture would define away. See the reference-objects
// design doc §8 for what each file proves.
import test from 'node:test';
import assert from 'node:assert/strict';
import { existsSync, mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import * as WebIFC from 'web-ifc';
import { openApi, closeApi, readModel, probeModel, toWebIfcYUp, mergeInherited, propertySetsByElement, fileAuthorsColour, pushColourRun } from './index.mjs';

const DOWNLOADS = join(process.env.USERPROFILE ?? process.env.HOME ?? '', 'Downloads');
const sample = (name) => (existsSync(join(DOWNLOADS, name)) ? join(DOWNLOADS, name) : null);
const skipReason = (name) => `sample file not present: ${join(DOWNLOADS, name)} — see the reference-objects design doc §8`;

async function read(name, maxVertices, opts) {
  const h = await openApi(sample(name));
  try {
    return readModel(h.api, h.modelID, maxVertices, opts);
  } finally {
    closeApi(h);
  }
}

// The big real coordination models live in a subfolder — they are the files that motivated #352/#353
// and are far too large to commit. Same skip-when-absent contract as `sample`.
const BIG = join(DOWNLOADS, 'ifc');
const bigSample = (name) => (existsSync(join(BIG, name)) ? join(BIG, name) : null);
const bigSkip = (name) => `large sample not present: ${join(BIG, name)} — see aware-aeco/aware#352`;

async function readBig(name, maxVertices, opts) {
  const h = await openApi(bigSample(name));
  try {
    return readModel(h.api, h.modelID, maxVertices, opts);
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

// ── Property-set DISCOVERY, driven through a fake web-ifc API ────────────────────────────────
//
// These run in CI; the file-based tests above do not. Every sample lives in ~/Downloads and self-skips
// when absent, and CI expects exactly that (ci.yml) — so before these existed, deleting the entire
// type-inheritance traversal would have left the suite green. A fake api is the only way to drive the
// discovery paths on inputs the four samples do not contain.

/** The two calls `propertySetsByElement` makes: enumerate ids of a type, and fetch one line. */
function fakeApi(lines, byType) {
  return {
    GetLineIDsWithType: (_m, t) => {
      const ids = byType[t] || [];
      return { size: () => ids.length, get: (i) => ids[i] };
    },
    GetLine: (_m, id) => {
      if (!(id in lines)) throw new Error(`no line ${id}`);   // real web-ifc throws; the code catches
      return lines[id];
    },
  };
}
const pset = (name, props) => ({ Name: { value: name }, HasProperties: props.map((p) => ({ value: p })) });
const prop = (name, value) => ({ Name: { value: name }, NominalValue: { value } });
const REL_PROPS = WebIFC.IFCRELDEFINESBYPROPERTIES;
const REL_TYPE = WebIFC.IFCRELDEFINESBYTYPE;

test('ONE relationship carrying SEVERAL sets — the IFC4 aggregate select — keeps all of them', () => {
  // THE REGRESSION THIS EXISTS FOR. IFC4 widened RelatingPropertyDefinition to an
  // IfcPropertySetDefinitionSet, which web-ifc returns as `{ value: [20, 21] }`. Reading `.value` as
  // one id passed the ARRAY to GetLine, matched nothing, and dropped every set in the relationship —
  // a silent total loss on a valid, ordinary encoding.
  const api = fakeApi({
    10: { RelatingPropertyDefinition: { value: [20, 21] }, RelatedObjects: [{ value: 1 }] },
    20: pset('A', [30]),
    21: pset('B', [31]),
    30: prop('p1', 'v1'),
    31: prop('p2', 'v2'),
  }, { [REL_PROPS]: [10], [REL_TYPE]: [] });

  const sets = propertySetsByElement(api, 0).get(1);
  assert.deepEqual(sets.map((s) => s.name), ['A', 'B'], 'both sets on the one relationship must survive');
  assert.deepEqual(sets[1].properties, [{ name: 'p2', value: 'v2' }]);
});

test('a single-set relationship still works, and handles carried as objects are flattened', () => {
  const api = fakeApi({
    10: { RelatingPropertyDefinition: { value: 20 }, RelatedObjects: [{ value: 1 }] },
    11: { RelatingPropertyDefinition: { value: [{ value: 21 }] }, RelatedObjects: [{ value: 1 }] },
    20: pset('A', [30]),
    21: pset('B', [31]),
    30: prop('p1', 'v1'),
    31: prop('p2', 'v2'),
  }, { [REL_PROPS]: [10, 11], [REL_TYPE]: [] });

  const sets = propertySetsByElement(api, 0).get(1);
  assert.deepEqual(sets.map((s) => s.name), ['A', 'B']);
});

test('a type\'s own HasPropertySets reaches its occurrences, merged property by property', () => {
  // The path no sample file exercises: three of the four have no IfcRelDefinesByType at all.
  const api = fakeApi({
    10: { RelatingPropertyDefinition: { value: 20 }, RelatedObjects: [{ value: 1 }] },
    20: pset('Common', [30]),
    30: prop('shared', 'fromOccurrence'),
    12: { RelatingType: { value: 40 }, RelatedObjects: [{ value: 1 }] },
    40: { HasPropertySets: [{ value: 22 }] },
    22: pset('Common', [32, 33]),
    32: prop('shared', 'fromType'),
    33: prop('typeOnly', 'kept'),
  }, { [REL_PROPS]: [10], [REL_TYPE]: [12] });

  const sets = propertySetsByElement(api, 0).get(1);
  assert.equal(sets.length, 1, 'the same-named set is merged, not duplicated');
  const byName = Object.fromEntries(sets[0].properties.map((p) => [p.name, p.value]));
  assert.equal(byName.shared, 'fromOccurrence', 'the occurrence wins the property it declares');
  assert.equal(byName.typeOnly, 'kept', 'and the type property it never mentioned is NOT lost');
});

test('an IfcElementQuantity is not mistaken for a property set', () => {
  // It rides the same relationship. It carries `Quantities`, not `HasProperties`, and this command
  // returns IfcPropertySet only — matching ifc-inspector, and stated in the contract.
  const api = fakeApi({
    10: { RelatingPropertyDefinition: { value: 20 }, RelatedObjects: [{ value: 1 }] },
    20: { Name: { value: 'BaseQuantities' }, Quantities: [{ value: 30 }] },
    30: { Name: { value: 'GrossVolume' }, VolumeValue: { value: 3 } },
  }, { [REL_PROPS]: [10], [REL_TYPE]: [] });

  assert.equal(propertySetsByElement(api, 0).get(1), undefined, 'a quantity set contributes nothing');
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

// ── colours ────────────────────────────────────────────────────────────────────────────────────
//
// The whole point of the `colors` field is that an unstyled file must NOT come back white. web-ifc
// reports opaque white for geometry nobody styled, which is indistinguishable from a wall the
// architect painted white — so absence, not whiteness, is how "this file has no colours" is said.

test('a file that authors no colour omits `colors` — it does not report white', async (t) => {
  if (!sample('example-steel-framing.ifc')) return t.skip(skipReason('example-steel-framing.ifc'));
  const out = await read('example-steel-framing.ifc');
  assert.ok(out.objects.length > 0);
  for (const o of out.objects) {
    // ABSENT, and the assertion says absent rather than falsy on purpose. `[]` would be a claim —
    // "this object has no colour" — when the truth is that the file has none to give, and a consumer
    // told the first has no way back to its own default. This file carries zero style entities.
    assert.equal('colors' in o, false, `${o.name}: an unstyled file must not carry a colors field`);
  }
});

test("a styled file's colours are the file's own, covering every triangle exactly", async (t) => {
  if (!sample('11134_V_Motebello_Heistopp_Rev.ifc')) return t.skip(skipReason('11134_V_Motebello_Heistopp_Rev.ifc'));
  const out = await read('11134_V_Motebello_Heistopp_Rev.ifc');
  assert.ok(out.objects.length > 0);
  for (const o of out.objects) {
    assert.ok(Array.isArray(o.colors) && o.colors.length > 0, `${o.id}: expected colour runs`);
    // THE RUNS MUST TILE THE INDEX BUFFER: contiguous from 0, no gap, no overlap, ending exactly at
    // the end. A consumer maps these straight onto draw groups, so a run that points past the buffer
    // (or leaves a hole) is not a wrong colour — it is geometry that renders as nothing.
    let at = 0;
    for (const c of o.colors) {
      assert.equal(c.start, at, `${o.id}: run must start where the last one ended`);
      assert.ok(c.count > 0, `${o.id}: an empty run should never be emitted`);
      assert.equal(c.rgba.length, 4);
      for (const v of c.rgba) assert.ok(v >= 0 && v <= 1, `${o.id}: ${v} is not a 0..1 channel`);
      at += c.count;
    }
    assert.equal(at, o.indices.length, `${o.id}: runs must cover the whole index buffer`);
  }
  // And they are REAL colours read off the file, not a uniform default. This file paints its 19
  // objects in three: measured 2026-08-03 as black, cyan and green.
  const seen = new Set(out.objects.flatMap((o) => o.colors.map((c) => c.rgba.join(','))));
  assert.ok(seen.size > 1, `expected several distinct colours, got ${[...seen]}`);
  assert.ok(seen.has('0,1,1,1'), `expected the file's cyan among ${[...seen]}`);
});

test('NAMED objects carry THEIR OWN colour, not merely some colour', async (t) => {
  if (!sample('Building-Architecture.ifc')) return t.skip(skipReason('Building-Architecture.ifc'));
  const out = await read('Building-Architecture.ifc');
  // AN ORACLE, and the tests above needed one. Everything else here checks shape — that the runs tile
  // the buffer, that several colours appear, that white survives — and review found the hole: rotate
  // every geometry's colour onto the next geometry and all of it still passes. The runs still tile,
  // the palette is unchanged, the multi-coloured objects are still multi-coloured. Only pinning a
  // KNOWN object to a KNOWN colour falsifies a wrong association.
  //
  // Measured 2026-08-03 by GlobalId, which is stable across reads in a way array order is not.
  const expect = {
    '3_4VN63S96DfWiJjgG8j1C': [0.8588, 0.7725, 0.5961, 1],   // "sand bedding" — sandy brown
    '3zR0BOEcLADRKln4HYporH': [0.5765, 0.5765, 0.5765, 1],   // "floor" — mid grey
    '0ZTBBPo6f6bxqV2K7Oelrq': [0.9647, 0.6863, 0.498, 1],    // "house - roof - slab left" — terracotta
    '1AQAupaRP1txwK1AGiN61V': [1, 1, 1, 1],                  // an outer wall — authored WHITE
  };
  for (const [id, rgba] of Object.entries(expect)) {
    const o = out.objects.find((x) => x.id === id);
    assert.ok(o, `expected an object with GlobalId ${id}`);
    assert.equal(o.colors.length, 1, `${o.name}: this object is a single colour`);
    assert.deepEqual(o.colors[0].rgba, rgba, `${o.name} should be ${rgba}`);
    assert.equal(o.colors[0].count, o.indices.length, `${o.name}: its run must cover all its triangles`);
  }
});

test('authored TRANSPARENCY survives — a<1 is the file speaking, not a default', async (t) => {
  if (!sample('Building-Architecture.ifc')) return t.skip(skipReason('Building-Architecture.ifc'));
  const out = await read('Building-Architecture.ifc');
  // Alpha is the channel most easily dropped by accident (three-channel colour is the common shape),
  // and a consumer that renders glass as solid concrete has silently lost information the file gave.
  // "house - gross volume" is styled at 0.149 opacity in this file.
  const zone = out.objects.find((o) => o.id === '1yP7NInQz5uQzbiOpVFFJr');
  assert.ok(zone, 'expected the gross-volume zone');
  assert.equal(zone.colors[0].rgba[3], 0.149, 'its authored alpha must come through unrounded to 1');
  assert.ok(out.objects.some((o) => o.colors.every((c) => c.rgba[3] === 1)),
    'and opaque objects must stay opaque, or alpha is being invented');
});

test('the response says whether colour could be answered AT ALL', async (t) => {
  if (!sample('example-steel-framing.ifc') || !sample('Building-Architecture.ifc')) {
    return t.skip(skipReason('example-steel-framing.ifc / Building-Architecture.ifc'));
  }
  // Two silences that are NOT the same answer: a file with no palette, and a pre-1.3.0 bridge that
  // cannot report one. Both leave every object without `colors`, and only one of them is fixed by
  // installing a newer bridge — so the receipt is what lets a consumer tell a permanent condition
  // from a stale install. `false` is an answer; ABSENT is the old bridge.
  assert.equal((await read('example-steel-framing.ifc')).colorsAvailable, false);
  assert.equal((await read('Building-Architecture.ifc')).colorsAvailable, true);
});

test('an authored white is kept, because a styled file means it', async (t) => {
  if (!sample('Building-Architecture.ifc')) return t.skip(skipReason('Building-Architecture.ifc'));
  const out = await read('Building-Architecture.ifc');
  // This is the case that rules out "treat white as unstyled": half of this file's objects report
  // opaque white and they are genuinely styled that way (measured 2026-08-03 — 6 of 12). A heuristic
  // on the colour value would silently repaint them, so the gate is the FILE, not the pixel.
  const whites = out.objects.filter((o) => o.colors.some((c) => c.rgba.join(',') === '1,1,1,1'));
  assert.ok(whites.length > 0, 'expected authored whites in a file that has a palette');
  const coloured = out.objects.filter((o) => o.colors.some((c) => c.rgba.join(',') !== '1,1,1,1'));
  assert.ok(coloured.length > 0, 'and non-white ones beside them, or this file proves nothing');
});

test('one object with several colours keeps all of them', async (t) => {
  if (!bigSample('Hospital Arch.ifc')) return t.skip(bigSkip('Hospital Arch.ifc'));
  const out = await readBig('Hospital Arch.ifc');
  // Multi-coloured objects are ORDINARY, not exotic: measured 2026-08-03, 1,127 of this file's
  // 14,409 objects carry more than one colour among their placed geometries, and 6,358 of the 77,118
  // in `Steel IFC.ifc` do. Collapsing to one colour per object would be a visible lie on 8% of an
  // ordinary steel export, which is why colour is a run over the index buffer and not a field.
  const multi = out.objects.filter((o) => o.colors && o.colors.length > 1);
  assert.ok(multi.length > 500, `expected many multi-coloured objects, got ${multi.length}`);
  const runs = multi[0].colors;
  assert.ok(new Set(runs.map((c) => c.rgba.join(','))).size > 1, 'its runs must differ in colour');
});

test('adjacent runs of one colour merge; a change or a gap does not', () => {
  // Unmerged, 206,621 placed geometries would emit 206,621 runs to say 16 things. Merging is what
  // keeps the field small — but it must never merge across a colour change or a discontinuity,
  // because either would hand a consumer triangles painted with the wrong run's colour.
  const red = [1, 0, 0, 1], blue = [0, 0, 1, 1];
  let runs = [];
  runs = pushColourRun(runs, red, 0, 3);
  runs = pushColourRun(runs, red, 3, 6);      // same colour, contiguous -> merges
  assert.deepEqual(runs, [{ rgba: red, start: 0, count: 9 }]);
  runs = pushColourRun(runs, blue, 9, 3);     // different colour -> new run
  assert.equal(runs.length, 2);
  runs = pushColourRun(runs, blue, 99, 3);    // same colour but NOT contiguous -> new run
  assert.equal(runs.length, 3);
  assert.deepEqual(runs[2], { rgba: blue, start: 99, count: 3 });
  // A zero-length run is never recorded: it would name a start nothing occupies.
  assert.equal(pushColourRun([], red, 0, 0).length, 0);
});

test('the colour gate asks the FILE, and answers both ways', async (t) => {
  if (!sample('example-steel-framing.ifc') || !sample('Building-Architecture.ifc')) {
    return t.skip(skipReason('example-steel-framing.ifc / Building-Architecture.ifc'));
  }
  // Both arms, or this proves nothing: a gate stuck at `false` would pass the unstyled assertion
  // above on its own, and a gate stuck at `true` would pass the styled ones.
  for (const [name, expected] of [['example-steel-framing.ifc', false], ['Building-Architecture.ifc', true]]) {
    const h = await openApi(sample(name));
    try {
      assert.equal(fileAuthorsColour(h.api, h.modelID), expected, `${name} should ${expected ? '' : 'not '}author colour`);
    } finally {
      closeApi(h);
    }
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

// ── #353: read PART of a model — the filters, and what they must not do quietly ──────────────────
//
// The gap these close: `read-model` was all-or-nothing, so a 12-storey coordination model was 289 MB
// of it or nothing. Combined with the serialisation ceiling (#352) that made three of five real files
// unreadable at every budget. The filters are resolved to expressIDs and handed to `StreamMeshes`, so
// what nobody asked for is never tessellated — the cost is avoided, not just the bytes.

test('an ifc-types filter returns those types and nothing else', async (t) => {
  if (!sample('example-steel-framing.ifc')) return t.skip(skipReason('example-steel-framing.ifc'));
  const out = await read('example-steel-framing.ifc', undefined, { ifcTypes: ['IFCCOLUMN'] });
  assert.equal(out.objects.length, 6);                       // 6 columns of the 13 members
  assert.ok(out.objects.every((o) => o.ifcType === 'IFCCOLUMN'), 'a non-column survived the filter');
});

test('a type name is matched case-insensitively — it is copied or typed, not parsed', async (t) => {
  if (!sample('example-steel-framing.ifc')) return t.skip(skipReason('example-steel-framing.ifc'));
  const out = await read('example-steel-framing.ifc', undefined, { ifcTypes: ['IfcColumn'] });
  assert.equal(out.objects.length, 6);
});

test('filters INTERSECT — narrowing twice does not widen', async (t) => {
  if (!sample('example-steel-framing.ifc')) return t.skip(skipReason('example-steel-framing.ifc'));
  const all = await read('example-steel-framing.ifc');
  const storey = all.objects.find((o) => o.storey)?.storey;
  assert.ok(storey, 'fixture has no storey to filter on');
  const both = await read('example-steel-framing.ifc', undefined, { storeys: [storey], ifcTypes: ['IFCBEAM'] });
  assert.equal(both.objects.length, 7);                      // the beams ON that storey, not beams + storey
  assert.ok(both.objects.every((o) => o.ifcType === 'IFCBEAM' && o.storey === storey));
});

test('a filter value that matches NOTHING is reported, not silently zero', async (t) => {
  if (!sample('example-steel-framing.ifc')) return t.skip(skipReason('example-steel-framing.ifc'));
  // Zero objects is indistinguishable from "this model has none of those" — which is exactly the
  // wrong thing to tell a user who mistyped a storey name. The unmatched value has to come back.
  const out = await read('example-steel-framing.ifc', undefined, { storeys: ['Nivo 47'], ifcTypes: ['IFCNOTATHING'] });
  assert.equal(out.objects.length, 0);
  assert.deepEqual(out.selected.unmatched, [{ storey: 'Nivo 47' }, { ifcType: 'IFCNOTATHING' }]);
});

test('NO filter reports no selection, so "unfiltered" and "matched everything" stay different', async (t) => {
  if (!sample('example-steel-framing.ifc')) return t.skip(skipReason('example-steel-framing.ifc'));
  const out = await read('example-steel-framing.ifc');
  assert.equal(out.selected, undefined);
  assert.equal(out.objects.length, 13);
  // …and a filter that happens to select everything DOES report itself.
  const wide = await read('example-steel-framing.ifc', undefined, { ifcTypes: ['IFCCOLUMN', 'IFCBEAM'] });
  assert.equal(wide.objects.length, 13);
  assert.deepEqual(wide.selected.unmatched, []);
});

// ── #352: the byte budget refuses in words, where the ceiling used to crash in V8's ──────────────

test('the byte budget names the size, the limit, and the way out', async (t) => {
  if (!sample('example-steel-framing.ifc')) return t.skip(skipReason('example-steel-framing.ifc'));
  await assert.rejects(
    async () => read('example-steel-framing.ifc', undefined, { maxBytes: 1024 }),
    (e) => {
      assert.match(e.message, /budget allows/);
      assert.match(e.message, /storeys/);        // the actionable half — "read one storey at a time"
      assert.match(e.message, /ifc-types/);
      return true;
    },
  );
});

test('no byte budget means no byte limit — the ceiling is gone, not relocated', async (t) => {
  if (!sample('example-steel-framing.ifc')) return t.skip(skipReason('example-steel-framing.ifc'));
  const out = await read('example-steel-framing.ifc', undefined, {});
  assert.equal(out.objects.length, 13);
});

// ── The real coordination model: 12 storeys, 17,460 objects, 289 MB whole ────────────────────────

test('one storey of a 12-storey model reads WITHOUT reading the other eleven', async (t) => {
  if (!bigSample('2023-05-09 30 Daldy Street Model.ifc')) {
    return t.skip(bigSkip('2023-05-09 30 Daldy Street Model.ifc'));
  }
  const out = await readBig('2023-05-09 30 Daldy Street Model.ifc', undefined, { storeys: ['BASEMENT'] });
  assert.ok(out.objects.length > 0, 'BASEMENT selected nothing');
  assert.ok(out.objects.every((o) => o.storey === 'BASEMENT'), 'an object from another storey came back');
  // The whole file is 17,460 objects. A storey must be a genuine fraction of that, or the filter is
  // decorative — this is the assertion that would fail if filtering happened after tessellation and
  // merely discarded, since the cost (and the count) would be unchanged.
  assert.ok(out.objects.length < 5000, `expected a fraction of 17,460, got ${out.objects.length}`);
  assert.deepEqual(out.selected.unmatched, []);
});

test('an ids filter reaches an object OUTSIDE the spatial structure', async (t) => {
  const name = '2023-05-09 30 Daldy Street Model.ifc';
  if (!bigSample(name)) return t.skip(bigSkip(name));
  // This file's IfcSite carries a surface of its own while BEING the top of the spatial structure, so
  // nothing contains it and it reads back with `storey: null`. Resolving GlobalIds against the storey
  // walk alone therefore could not name it — one object in 17,460, invisible on any file whose site
  // carries no geometry. Reached here by type first, then by the id that type read.
  const sites = await readBig(name, undefined, { ifcTypes: ['IFCSITE'] });
  assert.equal(sites.objects.length, 3, 'expected the three IfcSites that carry geometry');
  const orphan = sites.objects.find((o) => o.storey == null);
  assert.ok(orphan, 'expected one site to sit outside the spatial structure');

  const byId = await readBig(name, undefined, { ids: [orphan.id] });
  assert.equal(byId.objects.length, 1, 'the ids filter could not name an object the same read returns');
  assert.equal(byId.objects[0].id, orphan.id);
  assert.deepEqual(byId.selected.unmatched, [], 'the site read as an unmatched GlobalId');
});

test('a filter can only ever select a SUBSET — it never conjures void geometry', async (t) => {
  const name = '2023-05-09 30 Daldy Street Model.ifc';
  if (!bigSample(name)) return t.skip(bigSkip(name));
  // `StreamMeshes` honours whatever ids it is handed, and this file's 740 IfcOpeningElements tessellate
  // perfectly well — they are the prisms cut out of walls for doors and windows. An unfiltered read
  // returns 17,460 objects and none of them; asking for openings by name must therefore return none
  // either, or the same file would describe two different models depending on whether you narrowed it.
  const openings = await readBig(name, undefined, { ifcTypes: ['IFCOPENINGELEMENT'] });
  assert.equal(openings.objects.length, 0, `a filter returned ${openings.objects.length} voids as objects`);
  // …and the type still RESOLVED, so this is "there are none to give you", not "no such type".
  assert.deepEqual(openings.selected.unmatched, []);
});

// ── Found by adversarial review (Codex, 2026-08-01) ──────────────────────────────────────────────

test('an EMPTY filter selects nothing — it must never mean "the whole building"', async (t) => {
  if (!sample('example-steel-framing.ifc')) return t.skip(skipReason('example-steel-framing.ifc'));
  // The dangerous direction. `storeys: []` normalises to an empty list, and treating that as "no
  // filter given" hands the entire model to a caller who asked for a subset precisely because it
  // cannot afford the entire model. Present-but-empty narrows to nothing; only ABSENT means unfiltered.
  for (const opts of [{ storeys: [] }, { ifcTypes: [] }, { ids: [] }, { storeys: [''] }, { storeys: ['   '] }]) {
    const out = await read('example-steel-framing.ifc', undefined, opts);
    assert.equal(out.objects.length, 0, `${JSON.stringify(opts)} returned ${out.objects.length} objects`);
    assert.ok(out.selected, `${JSON.stringify(opts)} reported no selection, so it read as unfiltered`);
    assert.equal(out.selected.candidates, 0);
  }
});

test('a valid IFC type the model does not contain is reported unmatched', async (t) => {
  if (!sample('example-steel-framing.ifc')) return t.skip(skipReason('example-steel-framing.ifc'));
  // IFCCHIMNEY is a real entity; this file has none. Reporting only UNKNOWN names made that
  // indistinguishable from a typo — both returned zero objects and an empty `unmatched`.
  const out = await read('example-steel-framing.ifc', undefined, { ifcTypes: ['IFCCHIMNEY'] });
  assert.equal(out.objects.length, 0);
  assert.deepEqual(out.selected.unmatched, [{ ifcType: 'IFCCHIMNEY' }]);
});

test('a filter never returns IfcSpace — web-ifc skips THREE classes, not one', async (t) => {
  if (!sample('Building-Architecture.ifc')) return t.skip(skipReason('Building-Architecture.ifc'));
  // The subset invariant's second half. Excluding IfcFeatureElementSubtraction covers both opening
  // types (the standard case is a subtype) and misses IFCSPACE entirely — so a `storeys:` or a broad
  // `ifc-types:` filter would hand back the air in every room as geometry that a whole read omits.
  const whole = await read('Building-Architecture.ifc');
  assert.ok(whole.objects.every((o) => o.ifcType !== 'IFCSPACE'), 'an unfiltered read returned a space');

  const spaces = await read('Building-Architecture.ifc', undefined, { ifcTypes: ['IFCSPACE'] });
  assert.equal(spaces.objects.length, 0, `a filter returned ${spaces.objects.length} room volumes`);

  // …and via a storey filter, which is how a user would actually hit this.
  const storey = whole.objects.find((o) => o.storey)?.storey;
  if (storey) {
    const byStorey = await read('Building-Architecture.ifc', undefined, { storeys: [storey] });
    assert.ok(byStorey.objects.every((o) => o.ifcType !== 'IFCSPACE'), 'a storey filter returned a space');
  }
});

test('the byte budget counts UTF-8 BYTES, not UTF-16 code units', async (t) => {
  if (!sample('11134_V_Motebello_Heistopp_Rev.ifc')) return t.skip(skipReason('11134_V_Motebello_Heistopp_Rev.ifc'));
  // This file's 271 property values are Norwegian, so its JSON is measurably longer in bytes than in
  // string length. A budget measured with `.length` promises a size it then exceeds on exactly the
  // files this reader exists for.
  const whole = await read('11134_V_Motebello_Heistopp_Rev.ifc');
  const chars = whole.objects.reduce((n, o) => n + JSON.stringify(o).length, 0);
  const bytes = whole.objects.reduce((n, o) => n + Buffer.byteLength(JSON.stringify(o), 'utf8'), 0);
  assert.ok(bytes > chars, `fixture is pure ASCII (${bytes} bytes vs ${chars} units) — it cannot test this`);

  // A budget set between the two must REFUSE. Measured by length it would look affordable.
  await assert.rejects(
    async () => read('11134_V_Motebello_Heistopp_Rev.ifc', undefined, { maxBytes: chars + 1 }),
    (e) => { assert.match(e.message, /budget allows/); return true; },
  );
});

test('a byte budget comes back as a receipt, so "honoured" is checkable', async (t) => {
  if (!sample('example-steel-framing.ifc')) return t.skip(skipReason('example-steel-framing.ifc'));
  // A caller passing only max-bytes got no acknowledgement from either bridge — a new one emits no
  // `selected` (no subset filter), an old one ignores the input silently. Identical successful shapes.
  const out = await read('example-steel-framing.ifc', undefined, { maxBytes: 50 * 1024 * 1024 });
  assert.equal(out.budget.maxBytes, 50 * 1024 * 1024);
  assert.ok(out.budget.bytes > 0 && out.budget.bytes < 50 * 1024 * 1024);
  // …and absent when no budget was asked for, so the receipt means something.
  const plain = await read('example-steel-framing.ifc');
  assert.equal(plain.budget, undefined);
});

test('the refusal names a small limit in real units, not "0 MB"', async (t) => {
  if (!sample('example-steel-framing.ifc')) return t.skip(skipReason('example-steel-framing.ifc'));
  await assert.rejects(
    async () => read('example-steel-framing.ifc', undefined, { maxBytes: 1024 }),
    (e) => {
      assert.doesNotMatch(e.message, /\b0 MB\b/, `rounded the limit away: ${e.message}`);
      assert.match(e.message, /1\.0 KB budget/);
      return true;
    },
  );
});

// ── The STREAMED response, exercised through the CLI the way AWARE invokes it ────────────────────
//
// Everything above calls readModel() directly, which never touches readModelStreamed — so none of it
// would notice if the streaming path emitted malformed JSON. Review named this as the gap: the fix
// for #352 lives in main(), and nothing spawned main(). These do.

import { execFileSync } from 'node:child_process';

/** Run the bridge as the runtime does: argv command, JSON config on stdin, JSON on stdout. */
function cli(command, config, env = {}) {
  return execFileSync(process.execPath, ['index.mjs', command], {
    input: JSON.stringify(config), encoding: 'utf8', maxBuffer: 1 << 30, cwd: import.meta.dirname,
    env: { ...process.env, ...env },
  });
}

test('AWARE artifact mode writes geometry outside stdout (#402)', () => {
  const artifactDir = mkdtempSync(join(tmpdir(), 'aware-ifc-artifact-'));
  try {
    const raw = cli('read-model', {
      'ifc-path': join('test-fixtures', 'baseplate-bp1.ifc'),
    }, { AWARE_ARTIFACT_DIR: artifactDir });
    const descriptor = JSON.parse(raw);
    assert.deepEqual(Object.keys(descriptor), ['$aware-artifact']);
    assert.equal(descriptor['$aware-artifact'].id, 'read-model.json');
    assert.equal(descriptor['$aware-artifact'].mediaType, 'application/json');
    assert.ok(descriptor['$aware-artifact'].bytes > 1024);

    const artifact = JSON.parse(readFileSync(join(artifactDir, 'read-model.json'), 'utf8'));
    assert.equal(artifact.frame, 'z-up');
    assert.ok(artifact.count > 0);
    assert.equal(artifact.count, artifact.objects.length);
  } finally {
    rmSync(artifactDir, { recursive: true, force: true });
  }
});

test('the streamed response is valid JSON, and says what the direct call says', async (t) => {
  if (!sample('example-steel-framing.ifc')) return t.skip(skipReason('example-steel-framing.ifc'));
  const raw = cli('read-model', { 'ifc-path': sample('example-steel-framing.ifc') });
  const out = JSON.parse(raw);                       // the whole point: it must parse
  assert.equal(out.frame, 'z-up');
  assert.equal(out.objects.length, 13);
  assert.equal(out.count, 13);
  assert.equal(out.skipped, 0);
  assert.equal(out.selected, undefined);
  // Written in pieces, not one stringify — so the document is assembled across many writes and the
  // tail keys land after the array. Both must survive the trip.
  assert.ok(raw.startsWith('{"frame":"z-up","objects":['), `prefix was: ${raw.slice(0, 40)}`);
  assert.ok(raw.trimEnd().endsWith('}'));
});

test('ZERO objects still streams a well-formed document', async (t) => {
  if (!sample('example-steel-framing.ifc')) return t.skip(skipReason('example-steel-framing.ifc'));
  // The empty array is the edge a hand-assembled JSON document gets wrong — a stray comma, or an
  // array that never closes.
  const out = JSON.parse(cli('read-model', {
    'ifc-path': sample('example-steel-framing.ifc'), 'ifc-types': ['IFCCHIMNEY'],
  }));
  assert.deepEqual(out.objects, []);
  assert.equal(out.count, 0);
  assert.deepEqual(out.selected.unmatched, [{ ifcType: 'IFCCHIMNEY' }]);
});

test('the CLI accepts the kebab-case names the manifest declares', async (t) => {
  if (!sample('example-steel-framing.ifc')) return t.skip(skipReason('example-steel-framing.ifc'));
  // `ifc-types` and `max-bytes` are what a .flo writes; the internal API takes ifcTypes/maxBytes. A
  // mapping typo in main() would leave the filter silently unapplied — the whole-building failure.
  const out = JSON.parse(cli('read-model', {
    'ifc-path': sample('example-steel-framing.ifc'), 'ifc-types': ['IFCCOLUMN'], 'max-bytes': 50 * 1024 * 1024,
  }));
  assert.equal(out.count, 6, 'the kebab-case ifc-types filter was not applied');
  assert.equal(out.budget.maxBytes, 50 * 1024 * 1024, 'the kebab-case max-bytes budget was not applied');
});

test('a mid-stream refusal exits non-zero and says WHY on stderr', async (t) => {
  if (!sample('example-steel-framing.ifc')) return t.skip(skipReason('example-steel-framing.ifc'));
  // Streaming changes what a failure leaves behind: bytes are already on stdout when the budget
  // trips, so the document is truncated. That is safe only because the exit code is non-zero and the
  // actionable sentence is on STDERR — which is what the runtime now reports (see invoker.rs).
  let threw = false;
  try {
    cli('read-model', { 'ifc-path': sample('example-steel-framing.ifc'), 'max-bytes': 1024 });
  } catch (e) {
    threw = true;
    assert.equal(e.status, 1, `expected exit 1, got ${e.status}`);
    assert.match(String(e.stderr), /budget allows/, `stderr was: ${String(e.stderr).slice(0, 200)}`);
    assert.match(String(e.stderr), /storeys/);
    // …and stdout holds a TRUNCATED document, which is exactly why the runtime must not report it.
    assert.ok(String(e.stdout).startsWith('{"frame"'), 'expected a partial document on stdout');
    assert.throws(() => JSON.parse(String(e.stdout)), 'the partial document should NOT parse');
  }
  assert.ok(threw, 'the byte budget did not refuse');
});
