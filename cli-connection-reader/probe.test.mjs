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
import { openApi, probeModel, closeApi } from './index.mjs';

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
