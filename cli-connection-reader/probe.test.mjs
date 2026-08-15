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
import { existsSync, readFileSync, readdirSync } from 'node:fs';
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
// Every assertion IN THIS FILE that touches `bbox` needs a third-party file from ~/Downloads, so on CI
// — and on any fresh checkout — all of them skip. Before these tests existed, of the nine tests here
// exactly two ran, both pure unit-conversion checks against a fake `GetLine` (that count is a
// statement about this file as it stood at #391, not a live figure — it grows as tests are added).
//
// What had never executed anywhere is any assertion about WHERE the box is. `read-model.test.mjs` and
// `extract.test.mjs` do assert on `probe.bbox` on CI — but only that its Z range overlaps the mesh's,
// for the frame question (#343), and `read-model.test.mjs` asserts fixture SIZE too. None of them
// touches `min`, the midpoint, or containment, which is the whole subject of #348 and of the doc
// rewrite that shipped in #391. That is the ground these tests cover.
//
// Meanwhile five IFC fixtures ship in-repo and exhibit the documented behaviour exactly, so the
// measurements that have been living in #348's comment thread since 2026-08-05 become executable.
//
// These are CHARACTERISATION tests: they pin what the number does today, including where that is
// wrong. Each one that records a defect says so and says what to do when it starts failing, so it
// documents the defect rather than cementing it. All of them were verified to FAIL under a simulated
// fix that sets `bbox` to the true geometry AABB.
// The four fixtures authored at a site offset. `baseplate-origin.ifc` is deliberately NOT here — it
// is the same connection authored AT the origin, and it exists to exercise the opposite arm (see the
// last test). Adding an offset fixture to `test-fixtures/` without adding it here would silently
// leave it uncovered, so the next test asserts this list against the directory.
const CONNECTION_FIXTURES = ['baseplate-bp1.ifc', 'baseplate-rot.ifc', 'shearplate-sp1.ifc', 'shearplate-2col.ifc'];
const ORIGIN_FIXTURE = 'baseplate-origin.ifc';

const box = (min, max) => ({
  min, max,
  span: max.map((v, k) => v - min[k]),
  ctr: max.map((v, k) => (v + min[k]) / 2),
});
const boxOf = (b) => box(b.min, b.max);
const dist = (a, b) => Math.hypot(...a.map((v, k) => v - b[k]));
// The quantity the deleted rule was built on: how far the box reaches from the origin, over how wide
// it is. Named once because two tests need the SAME number the doc used to prescribe.
//
// The reach is taken over BOTH corners, not just `max`. The rule said `|max|`, which is the same
// number whenever the box contains the origin (then `|min|` is the smaller side) — true of every
// origin-pinned box, which is the case the rule was about. Taking both is what keeps it meaningful on
// a box that does NOT contain the origin, which is the comparison the second arm below makes.
function reachOverSpan(b) {
  const widest = Math.max(...b.span);
  assert.ok(widest > 0, `a degenerate box has no span to compare against: ${JSON.stringify(b.min)}..${JSON.stringify(b.max)}`);
  return Math.max(...b.max.map(Math.abs), ...b.min.map(Math.abs)) / widest;
}

// probe and read-model against ONE open of the same file — the comparison every test below makes.
//
// THE GUARDS BELOW ARE LOAD-BEARING, not ceremony. Without them an empty `model.objects` leaves the
// mesh box at ±Infinity, and `Infinity >= 0 && -Infinity <= 5025` makes the containment test's
// `contains` VACUOUSLY TRUE — so a total geometry-read failure would report itself as "probe.bbox now
// CONTAINS the mesh", i.e. as #348 being FIXED, and the remedy that test prints would turn it into a
// permanent vacuous pass. `readModel` already counts what it drops (`count`/`skipped`) precisely so
// geometry loss cannot be silent; this reads them rather than discarding them. Same guards, same
// reason, as the sibling helper in `read-model.test.mjs`.
async function probeAndMesh(name) {
  const h = await openApi(join('test-fixtures', name));
  try {
    const probe = probeModel(h.api, h.modelID);
    const model = readModel(h.api, h.modelID);
    // `bbox: null` is a deliberate, documented answer ("could not resolve the unit", "no usable 3D
    // points"), so say that — three of the tests below dereference `.min`/`.max` and would otherwise
    // fail with a bare null-deref that reads as a broken harness rather than as probe answering.
    assert.ok(probe.bbox, `${name}: probe returned bbox:null — it could not resolve the length unit or found no usable 3D points`);
    assert.ok(model.objects.length > 0,
      `${name}: read-model returned no geometry (count ${model.count}, skipped ${model.skipped}) — the `
      + 'mesh these bbox tests measure against is empty, so nothing below is a statement about bbox');
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
  } catch (e) {
    // A THROWN error (web-ifc internals, read-model's too-complex circuit breaker) carries no fixture
    // identity otherwise, and every test here loops over several files under one title. The guards
    // above already name it, so only label what is not labelled — a doubled prefix reads as a bug in
    // the harness at exactly the moment someone is trying to read a real failure.
    if (!String(e.message).startsWith(`${name}: `)) e.message = `${name}: ${e.message}`;
    throw e;
  } finally {
    closeApi(h);
  }
}

test('#348: the fixture list covers every offset fixture that ships', () => {
  // `CONNECTION_FIXTURES` is hand-written, so a fixture added to the directory would be silently
  // uncovered while this file still claims to pin "the fixtures that ship in this repo". Asserting
  // the two agree is what makes that claim true rather than true-as-of-writing.
  const onDisk = readdirSync('test-fixtures').filter((f) => f.endsWith('.ifc')).sort();
  assert.deepEqual(onDisk, [...CONNECTION_FIXTURES, ORIGIN_FIXTURE].sort(),
    'test-fixtures/ and the fixture lists in this file disagree — add the new fixture to '
    + 'CONNECTION_FIXTURES (offset) or ORIGIN_FIXTURE (authored at the origin), or these tests silently skip it');
});

test('#348: probe.bbox is pinned to the world origin on every in-repo fixture', async () => {
  // The mechanism the whole issue turns on. A file's points include every placement origin — the
  // representation context's WorldCoordinateSystem, the site and building placements, each product's —
  // and those sit at (0,0,0). So `min` is not "where the model starts"; it is the origin, exactly,
  // while `max` is ~23 m away. Asserting min is EXACTLY zero rules out coincidental smallness — it is
  // still a statement about these fixtures, and `baseplate-origin.ifc` below is the counter-case that
  // shows it is NOT a universal law: a file authored across the origin has a negative `min`.
  for (const name of CONNECTION_FIXTURES) {
    const { probe, mesh } = await probeAndMesh(name);
    assert.deepEqual(probe.bbox.min, [0, 0, 0],
      `${name}: expected the origin-pinned min these offset fixtures produce, got ${JSON.stringify(probe.bbox.min)}`);
    // And nothing is actually there: the geometry sits ~23 m away. Without this arm a model genuinely
    // authored at the origin would satisfy the assertion above and prove nothing.
    //
    // DISTANCE of the model's centre, not a per-axis minimum: `baseplate-rot` is yawed 30° about the
    // vertical, which swings it to x = -1563, so an axiswise "every coordinate exceeds 1 m" test reads
    // that as "at the origin" when it is 23 m out. The claim is about the model's position, so it has
    // to be measured as one.
    assert.ok(dist(mesh.ctr, [0, 0, 0]) > 5000,
      `${name}: the mesh should sit far from the origin, but its centre is ` +
      `${Math.round(dist(mesh.ctr, [0, 0, 0]))} mm out. If read-model returned real geometry and this ` +
      'fixture really is authored at the origin, it belongs in ORIGIN_FIXTURE, not CONNECTION_FIXTURES');
  }
});

test('#348: probe.bbox does NOT contain the geometry read-model returns', async () => {
  // The defect, characterised. `bbox` is not an upper bound on anything: a point-based extent cannot
  // see a swept solid, whose size lives in numbers (IFCRECTANGLEPROFILEDEF, IFCEXTRUDEDAREASOLID
  // depth) and not in any IfcCartesianPoint.
  //
  // WHEN THIS TEST STARTS FAILING, THE BUG IS FIXED. That is the intent: read #348, confirm the box
  // now contains the mesh on every fixture, and replace this test with the containment assertion it
  // has been standing in for. Do not "repair" it by loosening the bound — and note that an EMPTY mesh
  // would make `contains` vacuously true, which is why `probeAndMesh` refuses to return one.
  //
  // The shortfall is asserted as a MAGNITUDE, not just a direction: it is exactly the invisible
  // extrusion depth, so a number here is a statement about the mechanism. Measured 2026-08-07 and
  // traceable to the generators: 1000 mm of column on the baseplates (`make-baseplate.py`, the 1 m
  // stub), 430 mm on the shear plates (`make-shearplate.py`).
  const Z_SHORTFALL = {
    'baseplate-bp1.ifc': 1000, 'baseplate-rot.ifc': 1000,
    'shearplate-sp1.ifc': 430, 'shearplate-2col.ifc': 430,
  };
  for (const name of CONNECTION_FIXTURES) {
    const { probe, mesh } = await probeAndMesh(name);
    const contains = mesh.min.every((v, k) => v >= probe.bbox.min[k])
      && mesh.max.every((v, k) => v <= probe.bbox.max[k]);
    assert.equal(contains, false,
      `${name}: probe.bbox now CONTAINS the mesh — if that is deliberate, #348 is fixed: turn this ` +
      'into a containment assertion rather than relaxing it');
    // Name the axis AND the amount, so the characterisation records WHY rather than just that.
    const short = mesh.max[2] - probe.bbox.max[2];
    assert.ok(Math.abs(short - Z_SHORTFALL[name]) < 1,
      `${name}: expected the box to fall exactly ${Z_SHORTFALL[name]} mm short in Z — the extrusion ` +
      `depth no point can see — but it falls short by ${short.toFixed(1)} mm. If a fixture generator ` +
      'changed the column length, update Z_SHORTFALL; if not, the point-selection changed');
  }
});

test('#348: the midpoint of probe.bbox is ~10 model longest-edges from the model', async () => {
  // What the only real consumer actually reads (floless.app's `verdictFor` feeds this midpoint to an
  // off-screen check), and therefore the number worth pinning. Because the box runs from the origin to
  // the model, its centre lands about half way there — so the error is distance-from-origin / 2, a
  // property of the FILE rather than of the algorithm.
  //
  // The unit is the model's own longest edge, which is what makes the figure comparable across
  // fixtures. Measured 2026-08-07: 10.21, 12.60, 9.97, 9.64. The band is wide because the exact value
  // is set by the site offset in the generators (`OX, OY, OZ` in `make-baseplate.py`), not by probe.
  for (const name of CONNECTION_FIXTURES) {
    const { probe, mesh } = await probeAndMesh(name);
    const err = dist(boxOf(probe.bbox).ctr, mesh.ctr) / Math.max(...mesh.span);
    assert.ok(err > 9 && err < 13,
      `${name}: expected the midpoint 9–13 model longest-edges out (the documented 9.6–12.6), got ` +
      `${err.toFixed(2)}. Near zero means #348 is fixed — replace this with the accuracy assertion it ` +
      'stands in for. Otherwise a generator\'s OX/OY/OZ site offset changed, and the doc table with it');
  }
});

test('#348: reach-over-span can never fire on a box that contains the origin', async () => {
  // A guard on the doc, not on the code. `probe.md` and probeModel's own comment used to tell a
  // consumer: "when `|max|` is much larger than the box's own span, the box is origin-pinned and the
  // midpoint is meaningless. When the two are comparable, the file is authored near the origin and the
  // midpoint is fine." This test is here so that rule cannot quietly come back.
  //
  // The reason it cannot fire HERE: whenever a box contains the origin, `|max|` is at most the box's
  // own span on each axis, so the ratio is <= 1 and "much larger" is unreachable. It is exactly 1 when
  // `min` is [0,0,0], and below 1 otherwise — 0.50 on `baseplate-origin.ifc`. Every in-repo fixture
  // contains the origin because a file's points include every placement origin and these files anchor
  // those at (0,0,0), so the FIRST assertion below is a precondition check, not decoration.
  //
  // Containing the origin is a property of the FILES, not of probeModel, which bounds the file's own
  // 3D points and never inserts the origin — see the ratio-above-1 test below, which is the other half
  // of this one. So the doc must not claim the rule can never fire in general; only that it cannot on
  // anything measured, and that a ratio it COULD fire on still would not license the midpoint.
  //
  // So the rule only ever reports its SECOND arm, "the file is authored near the origin and the
  // midpoint is fine" — which is false on all four offset fixtures, whose midpoints the test above
  // measures at 9.6-12.6 longest-edges out. That clause is the strongest evidence against the rule,
  // which is why it is quoted here in full rather than paraphrased.
  for (const name of [...CONNECTION_FIXTURES, ORIGIN_FIXTURE]) {
    const { probe } = await probeAndMesh(name);
    const b = boxOf(probe.bbox);
    assert.ok(b.min.every((v) => v <= 0) && b.max.every((v) => v >= 0),
      `${name}: probe.bbox no longer contains the origin (${JSON.stringify(b.min)}..${JSON.stringify(b.max)}) — ` +
      'the lemma this test rests on is about origin-containing boxes, so re-derive the probe.md paragraph it guards');
    assert.ok(reachOverSpan(b) <= 1,
      `${name}: reach-over-span is ${reachOverSpan(b).toFixed(4)}, above 1 on a box containing the ` +
      'origin — that should be impossible, so either the box or the lemma is wrong');
  }

  // And the equality case, separately: exactly 1 whenever `min` is the origin. Exact float equality is
  // safe because `max[k] - 0` is `max[k]` itself in IEEE-754, so numerator and denominator are the
  // SAME float, not merely equal ones.
  for (const name of CONNECTION_FIXTURES) {
    const { probe } = await probeAndMesh(name);
    assert.equal(reachOverSpan(boxOf(probe.bbox)), 1, `${name}: expected exactly 1 on an origin-pinned box`);
  }
});

test('#348: reach-over-span CAN exceed 1 — the <= 1 result is about the files, not the algorithm', () => {
  // The other half of the test above, and the reason this file's claim is scoped the way it is.
  //
  // It is tempting to write "the ratio is <= 1 on every box probe emits", which reads as a property of
  // probeModel. It is not one. `probeModel` bounds the file's own 3D IfcCartesianPoints and never
  // inserts the origin (see the loop in index.mjs), so a file whose WorldCoordinateSystem and every
  // placement location are nonzero produces a box that excludes the origin — and then the ratio is
  // unbounded above. Every fixture in this repo anchors those points at (0,0,0), which is ordinary but
  // not guaranteed, and generalising from five files to a law is exactly the mistake #348 is about.
  //
  // Asserted on the pure helper rather than through a sixth fixture: the claim is arithmetic about a
  // box, so a box is the honest input. A fixture would additionally prove that IFC can express such a
  // file — true, but not what the doc sentence rests on.
  const farAway = box([10000, 20000, 5000], [10400, 20400, 6000]);
  assert.ok(farAway.min.some((v) => v > 0),
    'this box is meant to EXCLUDE the origin — otherwise it cannot demonstrate anything');
  assert.ok(reachOverSpan(farAway) > 1,
    `a box excluding the origin should score above 1, got ${reachOverSpan(farAway).toFixed(2)} — if this ` +
    'now fails, reachOverSpan changed and the scoped claim in probe.md needs re-deriving');
  // And the boundary: a box whose min is exactly the origin sits at 1, which is where the two arms of
  // the lemma meet. Pinning it stops the inequality being quietly widened to `< 1`.
  assert.equal(reachOverSpan(box([0, 0, 0], [400, 400, 1000])), 1,
    'a box with min at the origin marks the boundary of the lemma and must score exactly 1');
});

test('#348: a file authored AT the origin has no origin-pinned min, and a near-exact midpoint', async () => {
  // The other arm of the doc's claim, which until now rested entirely on `example-steel-framing.ifc` —
  // a ~/Downloads file whose tests skip on CI. `baseplate-origin.ifc` is the SAME connection as
  // `baseplate-bp1.ifc` with the generator's site offset set to 0 0 0, so the only difference between
  // them is where the file is authored. That is what makes the contrast evidence rather than anecdote.
  //
  // It also falsifies the tempting generalisation that `min` is always [0,0,0]: this file's anchors
  // straddle the origin, so `min` is NEGATIVE. The origin-pinned min is a property of files authored
  // wholly in the positive octant, not a law about probe.
  const { probe, mesh } = await probeAndMesh(ORIGIN_FIXTURE);
  assert.ok(probe.bbox.min.some((v) => v < 0),
    `expected a negative min on a file authored across the origin, got ${JSON.stringify(probe.bbox.min)}`);
  assert.ok(dist(mesh.ctr, [0, 0, 0]) < 1000,
    `${ORIGIN_FIXTURE} should be authored at the origin, but its mesh centre is ` +
    `${Math.round(dist(mesh.ctr, [0, 0, 0]))} mm out`);

  // The payoff: the midpoint error collapses. ~0.44 longest-edges here against 9.6-12.6 on the offset
  // fixtures — so the error really is a property of how far the file is authored from the origin, and
  // not of the algorithm. Not ZERO, though, and that matters: the box still cannot see the swept
  // column, so even at the origin the midpoint is only approximately the model's.
  const err = dist(boxOf(probe.bbox).ctr, mesh.ctr) / Math.max(...mesh.span);
  assert.ok(err < 1,
    `expected the midpoint within one longest-edge for an origin-authored file, got ${err.toFixed(2)}`);
  assert.ok(err > 0.05,
    `expected a small but NON-zero midpoint error (the swept column is still invisible), got ${err.toFixed(3)} — ` +
    'if this is now ~0, probe can see swept solids and #348 is fixed');
});

test('#348: the deleted reach-over-span rule has not come back to probe.md', () => {
  // The tests above pin the NUMBER the rule was built on; nothing mechanically stopped the RULE itself
  // being pasted back into the doc. Since the whole finding is that a false sentence survived because
  // nothing executed it, the guard has to read the sentence.
  const doc = readFileSync(
    join('..', '20-agents', 'aeco', 'engineering', 'ifc-reference-reader', 'commands', 'probe.md'), 'utf8');
  // The rebuttal legitimately QUOTES the rule, so a bare substring ban would forbid the correction
  // along with the error. The discriminator is instead WHERE it appears: the rule may stand only
  // inside a blockquote. A line mentioning it that does not start with `>` is the rule being asserted
  // again rather than cited.
  const asserted = doc.split('\n')
    .filter((l) => /much larger than the box/.test(l) && /own span/.test(l))
    .filter((l) => !l.trimStart().startsWith('>'));
  assert.deepEqual(asserted, [],
    'probe.md states the reach-over-span rule outside a blockquote. #348 measured it as unable to fire on '
    + 'any file here (the ratio is <= 1 on any box CONTAINING the origin, which every in-repo fixture does) '
    + 'and as no use where it could fire (a high ratio only means the box excludes the origin, which does '
    + 'not make the midpoint trustworthy), so it may be quoted as the error it was — not given as advice.');
  assert.ok(!/You can tell which case you are in from `bbox` alone/.test(doc),
    'probe.md has re-acquired the claim that bbox alone discriminates the two cases — it cannot.');
  // And the replacement must still be there, so this guard cannot be satisfied by deleting the section.
  assert.ok(/does NOT tell you which case you are in/.test(doc),
    'probe.md no longer carries the correction from #348 — the guard above would now pass vacuously');
});
