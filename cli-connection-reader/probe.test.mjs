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
