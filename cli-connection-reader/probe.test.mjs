// Tests for `probe` — the cheap pre-flight read (units, schema, element count, extent) that a consumer
// runs BEFORE deciding whether to tessellate a file at all.
//
// These run against REAL IFC files rather than synthetic fixtures, deliberately. The whole risk in this
// command is real-file behaviour: what a 2019 Allplan export declares, what a SketchUp IFC4X3 file omits,
// whether web-ifc has already applied the declared unit. A hand-built fixture would encode the very
// assumption under test and go vacuously green. See
// floless.app/docs/superpowers/specs/2026-07-25-reference-objects-units-evidence.md.
import test from 'node:test';
import assert from 'node:assert/strict';
import { existsSync } from 'node:fs';
import { join } from 'node:path';
import { openApi, probeModel, readModel, closeApi } from './index.mjs';

const DOWNLOADS = join(process.env.USERPROFILE ?? process.env.HOME ?? '', 'Downloads');
const sample = (name) => join(DOWNLOADS, name);

// The sample set lives outside the repo (they are third-party files we may not redistribute), so a
// machine without them must SKIP LOUDLY rather than silently pass — a quiet skip here would hide the
// only tests that prove the units story.
function needs(name) {
  const path = sample(name);
  if (existsSync(path)) return path;
  return null;
}
const skipReason = (name) => `sample file not present: ${sample(name)} — see the reference-objects design doc §8`;

test('probe reports MILLI.METRE and the element count for our own export', async (t) => {
  const path = needs('example-steel-framing.ifc');
  if (!path) return t.skip(skipReason('example-steel-framing.ifc'));
  const h = await openApi(path);
  try {
    const out = probeModel(h.api, h.modelID);
    assert.equal(out.units.declared, 'MILLI.METRE');
    assert.equal(out.schema, 'IFC4');
    assert.equal(out.elements, 13); // 6 columns + 7 beams
  } finally {
    closeApi(h);
  }
});

test('probe reads METRE from a file that declares no prefix', async (t) => {
  const path = needs('11134_V_Motebello_Heistopp_Rev.ifc');
  if (!path) return t.skip(skipReason('11134_V_Motebello_Heistopp_Rev.ifc'));
  const h = await openApi(path);
  try {
    const out = probeModel(h.api, h.modelID);
    // The whole reason this file is in the test matrix: IFC2X3, declares METRE with no MILLI prefix.
    assert.equal(out.units.declared, 'METRE');
    assert.equal(out.schema, 'IFC2X3');
    assert.equal(out.elements, 19); // all IfcBuildingElementProxy
  } finally {
    closeApi(h);
  }
});

test('probe reports an extent far from the origin, so a consumer can offer to move it', async (t) => {
  const path = needs('11134_V_Motebello_Heistopp_Rev.ifc');
  if (!path) return t.skip(skipReason('11134_V_Motebello_Heistopp_Rev.ifc'));
  const h = await openApi(path);
  try {
    const { bbox } = probeModel(h.api, h.modelID);
    // Measured 2026-07-25: this file sits at real site coordinates — ~74 m up. It loads at the CORRECT
    // size and entirely off-screen beside a model at the origin, which is the defect the size-sanity
    // check was originally (wrongly) aimed at. Assert the distance, since that is what earns the
    // "zoom to reference" and "move to origin" affordances.
    const far = Math.max(...bbox.max.map(Math.abs), ...bbox.min.map(Math.abs));
    assert.ok(far > 40_000, `expected the extent to sit far from the origin, got ${far} mm`);
  } finally {
    closeApi(h);
  }
});

test('probe is honest when a file declares no length unit', async (t) => {
  const path = needs('Building-Structural.ifc');
  if (!path) return t.skip(skipReason('Building-Structural.ifc'));
  const h = await openApi(path);
  try {
    const out = probeModel(h.api, h.modelID);
    // Either it declares one or it does not — but `declared` must never be a GUESS. null means
    // "the file did not say", which is what puts the override dropdown in front of the user.
    assert.ok(out.units.declared === null || typeof out.units.declared === 'string');
    assert.equal(out.schema, 'IFC4X3_ADD2');
  } finally {
    closeApi(h);
  }
});

test('an imperial file is scaled by its real conversion factor, not silently by 1', async (t) => {
  // The bug this pins: an earlier five-entry metric lookup fell back to a factor of 1 for anything
  // it did not recognise, so an inch-based file produced a bbox 25.4x too small while still being
  // labelled millimetres. A wrong extent is worse than no extent — it is exactly what a consumer's
  // "this looks 1000x off" check reads. Built inline because none of the sample files are imperial.
  const { unitToMMForTest } = await import('./index.mjs');
  if (!unitToMMForTest) return t.skip('unitToMM not exported for testing');
  // IfcConversionBasedUnit(inch) -> IfcMeasureWithUnit(25.4, IfcSIUnit(MILLI, METRE))
  const fake = {
    GetLine: (_m, id) => ({
      1: { Name: { value: 'inch' }, ConversionFactor: { value: 2 } },
      2: { ValueComponent: { value: 25.4 }, UnitComponent: { value: 3 } },
      3: { Name: { value: 'METRE' }, Prefix: { value: 'MILLI' } },
    }[id]),
  };
  assert.equal(unitToMMForTest(fake, 0, 1), 25.4);
  // And the metric ladder still resolves through the SI prefix rather than a lookup table.
  assert.equal(unitToMMForTest(fake, 0, 3), 1);
});

test('an unresolvable unit yields bbox null, never a plausible-looking box', async (t) => {
  const { unitToMMForTest } = await import('./index.mjs');
  if (!unitToMMForTest) return t.skip('unitToMM not exported for testing');
  // A unit that is neither METRE nor convertible must come back null so probeModel can say
  // "I could not tell" instead of emitting a box scaled by a guess.
  const fake = { GetLine: () => ({ Name: { value: 'FURLONG' } }) };
  assert.equal(unitToMMForTest(fake, 0, 1), null);
});

// ── #353: the breakdown that lets a consumer offer "which part do you want?" ─────────────────────
//
// `probe` is the cheap call — no tessellation — so it is the only place a consumer can afford to ask
// what is in a file before committing to a read it may not be able to pay for. Without a breakdown a
// consumer knows a file has 5,878 elements and nothing about how to ask for less.

const BIG = join(DOWNLOADS, 'ifc');
const bigNeeds = (name) => (existsSync(join(BIG, name)) ? join(BIG, name) : null);

async function probeOf(path) {
  const h = await openApi(path);
  try { return probeModel(h.api, h.modelID); } finally { closeApi(h); }
}

test('probe lists the storeys and the IFC types, commonest first', async (t) => {
  const path = bigNeeds('2023-05-09 30 Daldy Street Model.ifc');
  if (!path) return t.skip(`large sample not present: ${join(BIG, '2023-05-09 30 Daldy Street Model.ifc')}`);
  const out = await probeOf(path);

  // ELEVEN, and the discrepancy is the point: a full read of this file yields TWELVE distinct storey
  // values, because one object — the IfcSite's own surface — sits outside the spatial structure and
  // reads back as `storey: null`. The breakdown covers the structure, so it lists eleven. Asserting
  // twelve here is what caught that, and asserting eleven is what keeps the population honest.
  assert.equal(out.storeys.length, 11, `expected the 11 storeys in the spatial structure, got ${out.storeys.length}`);
  assert.ok(Array.isArray(out.types) && out.types.length >= 10, `expected 10+ types, got ${out.types?.length}`);
  assert.ok(out.storeys.some((s) => s.name === 'BASEMENT'), 'BASEMENT missing from the breakdown');
  assert.ok(out.types.some((s) => s.name === 'IFCMEMBER'), 'IFCMEMBER missing from the breakdown');

  // Descending, so the rows worth offering come first and a consumer need not sort to show anything.
  for (const rows of [out.storeys, out.types]) {
    for (let i = 1; i < rows.length; i++) {
      assert.ok(rows[i - 1].elements >= rows[i].elements, 'breakdown is not sorted by count, descending');
    }
  }
});

test('the breakdown counts what a FILTER selects, not what `elements` counts', async (t) => {
  const path = bigNeeds('2023-05-09 30 Daldy Street Model.ifc');
  if (!path) return t.skip(`large sample not present: ${join(BIG, '2023-05-09 30 Daldy Street Model.ifc')}`);
  const out = await probeOf(path);

  // The two populations genuinely differ — `elements` is what the author placed DIRECTLY in the
  // spatial structure (5,878 here) while a read returns those plus everything aggregated beneath them
  // (17,460). A breakdown that summed to `elements` would under-predict every read by 3x, which is the
  // one thing a consumer would use it for.
  const total = out.storeys.reduce((n, s) => n + s.elements, 0);
  assert.ok(total > out.elements,
    `the breakdown sums to ${total} but elements is ${out.elements} — it is counting the wrong population`);
  assert.equal(total, out.types.reduce((n, s) => n + s.elements, 0), 'the two breakdowns disagree about the total');
});

test('a single-storey file still gets a breakdown, so a consumer never special-cases absence', async (t) => {
  const path = needs('11134_V_Motebello_Heistopp_Rev.ifc');
  if (!path) return t.skip(skipReason('11134_V_Motebello_Heistopp_Rev.ifc'));
  const out = await probeOf(path);
  assert.ok(Array.isArray(out.storeys) && out.storeys.length >= 1);
  // Every object in this file is an IfcBuildingElementProxy — which is exactly why the breakdown
  // cannot be the whole answer for it, and why propertySets exist. One type, honestly reported.
  assert.deepEqual(out.types.map((t2) => t2.name), ['IFCBUILDINGELEMENTPROXY']);
});

// ── #348: the `bbox` contract, pinned against the fixtures that ship in this repo ─────────────────
//
// Every assertion above that touches `bbox` needs a third-party file from ~/Downloads, so on CI — and
// on any fresh checkout — ALL of them skip. Measured on this branch before these tests existed: of the
// nine tests in this file exactly two ran, both pure unit-conversion checks against a fake `GetLine`.
// Not one assertion about `bbox` had ever executed in CI, while `bbox` is the entire subject of #348
// and of the doc rewrite that shipped in #391.
//
// Meanwhile four IFC fixtures ship in-repo and exhibit the documented behaviour exactly. So the
// measurements that have been living in #348's comment thread since 2026-08-05 become executable here.
// `read-model.test.mjs` already compares the two commands on these fixtures, but deliberately only
// about WHICH AXIS IS UP (#343) — it avoids position and size claims, which is precisely the ground
// left uncovered.
//
// These are CHARACTERISATION tests: they pin what the number does today, including where that is
// wrong. Each one that records a defect says so and says what to do when it starts failing, so it
// documents the defect rather than cementing it.
const CONNECTION_FIXTURES = ['baseplate-bp1.ifc', 'baseplate-rot.ifc', 'shearplate-sp1.ifc', 'shearplate-2col.ifc'];

const box = (min, max) => ({
  min, max,
  span: max.map((v, k) => v - min[k]),
  ctr: max.map((v, k) => (v + min[k]) / 2),
});
const boxOf = (b) => box(b.min, b.max);
const dist = (a, b) => Math.hypot(...a.map((v, k) => v - b[k]));
// The ratio the deleted rule was built on: how far the box reaches from the origin, against how wide
// it is. Named here because two tests below need to compute the SAME number the doc used to prescribe.
const reachOverSpan = (b) => Math.max(...b.max.map(Math.abs), ...b.min.map(Math.abs)) / Math.max(...b.span);

// probe and read-model against ONE open of the same file — the comparison every test below makes.
async function probeAndMesh(name) {
  const h = await openApi(join('test-fixtures', name));
  try {
    const probe = probeModel(h.api, h.modelID);
    const model = readModel(h.api, h.modelID);
    const min = [Infinity, Infinity, Infinity], max = [-Infinity, -Infinity, -Infinity];
    for (const o of model.objects) {
      for (let i = 0; i + 2 < o.positions.length; i += 3) {
        for (let k = 0; k < 3; k++) {
          const v = o.positions[i + k];
          if (v < min[k]) min[k] = v;
          if (v > max[k]) max[k] = v;
        }
      }
    }
    return { probe, mesh: box(min, max) };
  } finally {
    closeApi(h);
  }
}

test('#348: probe.bbox is pinned to the world origin on every in-repo fixture', async () => {
  // The mechanism the whole issue turns on. A file's points include every placement origin — the
  // representation context's WorldCoordinateSystem, the site and building placements, each product's —
  // and those sit at (0,0,0). So `min` is not "where the model starts"; it is the origin, exactly,
  // while `max` is ~20 m away. Asserting min is EXACTLY zero (not merely small) is what makes this a
  // statement about the mechanism rather than about these particular fixtures.
  for (const name of CONNECTION_FIXTURES) {
    const { probe, mesh } = await probeAndMesh(name);
    assert.ok(probe.bbox, `${name}: probe should establish a bbox`);
    assert.deepEqual(probe.bbox.min, [0, 0, 0],
      `${name}: expected the origin-pinned min the mechanism produces, got ${probe.bbox.min}`);
    // And nothing is actually there: the geometry sits ~22 m away. Without this arm a model genuinely
    // authored at the origin would satisfy the assertion above and prove nothing.
    //
    // DISTANCE of the model's centre, not a per-axis minimum: `baseplate-rot` is yawed 30° about the
    // vertical, which swings it to x = -1563, so an axiswise "every coordinate exceeds 1 m" test reads
    // that as "at the origin" when it is 22 m out. The claim is about the model's position, so it has
    // to be measured as one.
    assert.ok(dist(mesh.ctr, [0, 0, 0]) > 5000,
      `${name}: the mesh should sit far from the origin, but its centre is ` +
      `${Math.round(dist(mesh.ctr, [0, 0, 0]))} mm out — if a fixture is ever authored at the origin ` +
      'it does not belong in CONNECTION_FIXTURES');
  }
});

test('#348: probe.bbox does NOT contain the geometry read-model returns', async () => {
  // The defect, characterised. `bbox` is not an upper bound on anything: a point-based extent cannot
  // see a swept solid, whose size lives in numbers (IFCRECTANGLEPROFILEDEF, IFCEXTRUDEDAREASOLID
  // depth) and not in any IfcCartesianPoint. Measured 2026-08-07 the box falls short at the TOP of
  // every fixture — 1000 mm of column on the baseplates, 430 mm on the shear plates.
  //
  // WHEN THIS TEST STARTS FAILING, THE BUG IS FIXED. That is the intent: read #348, confirm the box
  // now contains the mesh on every fixture, and replace this test with the containment assertion it
  // has been standing in for. Do not "repair" it by loosening the bound.
  for (const name of CONNECTION_FIXTURES) {
    const { probe, mesh } = await probeAndMesh(name);
    const contains = mesh.min.every((v, k) => v >= probe.bbox.min[k])
      && mesh.max.every((v, k) => v <= probe.bbox.max[k]);
    assert.equal(contains, false,
      `${name}: probe.bbox now CONTAINS the mesh — if that is deliberate, #348 is fixed: turn this ` +
      'into a containment assertion rather than relaxing it');
    // Name the axis, so the characterisation records WHY rather than just that. The top of the model
    // is above the top of the box on all four, which is the swept-solid blindness specifically.
    assert.ok(mesh.max[2] > probe.bbox.max[2],
      `${name}: expected the mesh to overtop the box in Z (the invisible extrusion depth), but ` +
      `mesh Z ends at ${Math.round(mesh.max[2])} and the box at ${Math.round(probe.bbox.max[2])}`);
  }
});

test('#348: the midpoint of probe.bbox is ~10 model longest-edges from the model', async () => {
  // What the only real consumer actually reads (floless.app's `verdictFor` feeds this midpoint to an
  // off-screen check), and therefore the number worth pinning. Because the box runs from the origin to
  // the model, its centre lands about half way there — so the error is distance-from-origin / 2, a
  // property of the FILE rather than of the algorithm.
  //
  // The unit is the model's own longest edge, which is what makes the figure comparable across
  // fixtures. Measured 2026-08-07: 10.21, 12.60, 9.97, 9.64.
  for (const name of CONNECTION_FIXTURES) {
    const { probe, mesh } = await probeAndMesh(name);
    const err = dist(boxOf(probe.bbox).ctr, mesh.ctr) / Math.max(...mesh.span);
    assert.ok(err > 9 && err < 13,
      `${name}: expected the midpoint 9–13 model longest-edges out (the documented 9.6–12.6), got ${err.toFixed(2)}`);
  }
});

test('#348: |max|-over-span cannot discriminate an origin-pinned box — it is 1 by construction', async () => {
  // A guard on the doc, not on the code. `probe.md` and probeModel's own comment used to tell a
  // consumer: "when |max| is much larger than the box's own span the box is origin-pinned and the
  // midpoint is meaningless; when the two are comparable the midpoint is fine." Both arms are wrong,
  // and this test is here so the rule cannot quietly come back.
  //
  // Arm one: on an origin-pinned box min is [0,0,0], so span === max and the ratio is IDENTICALLY 1.
  // It can never report "much larger" about the very boxes it exists to flag, and read literally it
  // says the midpoint is fine — about a midpoint the test above measures at ~10 longest-edges out.
  for (const name of CONNECTION_FIXTURES) {
    const { probe } = await probeAndMesh(name);
    assert.equal(reachOverSpan(boxOf(probe.bbox)), 1,
      `${name}: the ratio should be exactly 1 on an origin-pinned box; if it is not, min is no longer ` +
      'the origin and this test and the probe.md paragraph it guards both need re-deriving');
  }

  // Arm two, and the reason "backwards" is the right word: a ratio much larger than 1 requires a box
  // that EXCLUDES the origin. read-model's real mesh AABB for the same fixture is exactly such a box —
  // tight, ~20 m out — and its midpoint is the model's true position, the case the deleted rule called
  // meaningless. Using a measured box rather than a synthetic one keeps both arms about real files.
  const { mesh } = await probeAndMesh('baseplate-bp1.ifc');
  assert.ok(reachOverSpan(mesh) > 10,
    `the trustworthy far-out box should be the one scoring "much larger", got ${reachOverSpan(mesh).toFixed(1)}`);
});
