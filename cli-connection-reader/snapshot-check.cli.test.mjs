// The process-boundary path for `check-snapshots`: files in, canonical JSON out, through the real entry points
// (`index.mjs` and the shipped dispatcher). `snapshot-check.test.mjs` covers the rules with plain objects; this
// proves the dispatch accepts a command with no `ifc-path`, that stdout carries ONLY the result, that the input
// digests are the sha256 of the exact file bytes, and that a bad request fails without writing a half result.
import test from 'node:test';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { PROFILE } from './snapshot-compare-fixtures.mjs';
import { canonicalJson } from './snapshot-compare.mjs';
import { CHECK_PROFILE_SCHEMA } from './snapshot-check.mjs';
import { box, clashChecks, world } from './snapshot-clash-fixtures.mjs';

const here = new URL('.', import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1');
const sha = (bytes) => createHash('sha256').update(bytes).digest('hex');
const CHECKS = {
  schema: CHECK_PROFILE_SCHEMA, id: 'floless.change-checks', version: '1.0.0',
  budgets: { findings: 2000, focusResults: 20000 },
  rules: [{
    id: 'integrity.part-name-blank', version: '1.0.0', severity: 'warning', predicate: 'blank', target: 'entity.name',
    appliesTo: { types: ['Beam'] }, worsening: null,
  }],
};

function writeSide(dir, objects) {
  mkdirSync(dir, { recursive: true });
  const entities = objects.map((o, i) => ({ id: `element:${i + 1}`, type: 'Beam', name: o.name }));
  const properties = objects.map((o, i) => ({ id: `p:${i}`, entityId: `element:${i + 1}`, groupName: 'g', groupOrdinal: 0, name: 'GUID', value: o.guid, valueType: 'string', unit: null, propertyOrdinal: 0, semantic: 'x', dimension: 'none', isUda: false, sourceKey: 'native:guid' }));
  const files = { 'entities.json': { entities }, 'properties.json': { properties }, 'relationships.json': { relationships: [] } };
  for (const [name, doc] of Object.entries(files)) writeFileSync(join(dir, name), canonicalJson(doc));
}

function fixture() {
  const root = mkdtempSync(join(tmpdir(), 'snapshot-check-'));
  writeSide(join(root, 'base'), [{ guid: 'G1', name: 'one' }, { guid: 'G2', name: '' }, { guid: 'G3', name: 'three' }]);
  writeSide(join(root, 'revised'), [{ guid: 'G1', name: '' }, { guid: 'G2', name: '' }, { guid: 'G3', name: 'three' }, { guid: 'G4', name: ' ' }]);
  writeFileSync(join(root, 'profile.json'), JSON.stringify(PROFILE, null, 2));
  writeFileSync(join(root, 'checks.json'), JSON.stringify(CHECKS, null, 2));
  writeFileSync(join(root, 'focus.json'), JSON.stringify({ identities: ['lineage-A:G2', 'lineage-A:G3', 'lineage-A:G9'] }));
  return root;
}
const request = (root, extra = {}) => ({
  'base-dir': join(root, 'base'), 'revised-dir': join(root, 'revised'),
  'profile-path': join(root, 'profile.json'), 'check-profile-path': join(root, 'checks.json'), ...extra,
});
const run = (entry, input) => spawnSync(process.execPath, [join(here, entry), 'check-snapshots'], { input: JSON.stringify(input), encoding: 'utf8', cwd: here });

for (const entry of ['index.mjs', 'model-dispatcher.mjs']) {
  test(`${entry} check-snapshots: files in, one canonical JSON document on stdout, digests of the exact bytes`, () => {
    const root = fixture();
    try {
      const r = run(entry, request(root, { 'focus-path': join(root, 'focus.json') }));
      assert.equal(r.status, 0, r.stderr);
      const out = JSON.parse(r.stdout);                       // stdout is PURE JSON: nothing else was written to it
      assert.equal(r.stdout, canonicalJson(out), 'the bytes written are the canonical form');
      assert.equal(out.schema, 'aware.model-change-findings/v1');
      assert.deepEqual(out.findings.map((f) => `${f.participants[0].identity}:${f.presence}`).sort(), [
        'lineage-A:G1:introduced', 'lineage-A:G2:pre-existing', 'lineage-A:G4:introduced',
      ]);
      assert.deepEqual(out.focusResults.map((x) => `${x.identity}:${x.state}`), [
        'lineage-A:G2:finding', 'lineage-A:G3:passed', 'lineage-A:G9:absent',
      ]);
      for (const side of ['base', 'revised']) {
        for (const file of ['entities', 'properties', 'relationships']) {
          assert.equal(out.inputs[side][file], sha(readFileSync(join(root, side, `${file}.json`))), `${side} ${file} digest is the sha256 of the file's bytes`);
        }
      }
      assert.equal(out.inputs.profileDigest, sha(canonicalJson(PROFILE)));
      assert.equal(out.inputs.checkProfileDigest, sha(canonicalJson(CHECKS)));
      assert.equal(out.inputs.focusDigest, sha(canonicalJson({ identities: ['lineage-A:G2', 'lineage-A:G3', 'lineage-A:G9'] })));
      // Second run: byte-identical. Without a focus file the focus digest is null and no re-check rows are written.
      assert.equal(run(entry, request(root, { 'focus-path': join(root, 'focus.json') })).stdout, r.stdout);
      const bare = JSON.parse(run(entry, request(root)).stdout);
      assert.equal(bare.inputs.focusDigest, null);
      assert.deepEqual(bare.focusResults, []);
    } finally { rmSync(root, { recursive: true, force: true }); }
  });
}

test('check-snapshots fails cleanly: no result on stdout, a message on stderr, non-zero exit', () => {
  const root = fixture();
  try {
    const missing = run('index.mjs', { 'base-dir': join(root, 'base'), 'revised-dir': join(root, 'revised'), 'profile-path': join(root, 'profile.json') });
    assert.notEqual(missing.status, 0);
    assert.equal(missing.stdout, '');
    assert.match(missing.stderr, /`check-profile-path` is required/);

    writeFileSync(join(root, 'checks.json'), JSON.stringify({ ...CHECKS, surprise: true }));
    const strict = run('index.mjs', request(root));
    assert.notEqual(strict.status, 0);
    assert.equal(strict.stdout, '');
    assert.match(strict.stderr, /unknown key 'surprise'/);

    writeFileSync(join(root, 'checks.json'), JSON.stringify(CHECKS));
    writeFileSync(join(root, 'focus.json'), JSON.stringify({ identities: ['a', 'a'] }));
    const focus = run('index.mjs', request(root, { 'focus-path': join(root, 'focus.json') }));
    assert.notEqual(focus.status, 0);
    assert.equal(focus.stdout, '');
    assert.match(focus.stderr, /names 'a' twice/);

    writeFileSync(join(root, 'base', 'entities.json'), '{"entities":[],"extra":[]}');
    const shape = run('index.mjs', request(root));
    assert.notEqual(shape.status, 0);
    assert.equal(shape.stdout, '');
    assert.match(shape.stderr, /must be exactly \{ entities/);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

// ─── clash: geometry directories in, solids measured, through the real entry points ─────────────────

function writeGeometry(dir, tiles) {
  mkdirSync(dir, { recursive: true });
  for (const tile of tiles) writeFileSync(join(dir, tile.file), tile.bytes);
  writeFileSync(join(dir, 'geometry.json'), JSON.stringify({ tiles: tiles.map(({ ordinal, file, sha256 }) => ({ ordinal, file, sha256 })) }));
}

function clashFixture() {
  const root = mkdtempSync(join(tmpdir(), 'snapshot-clash-'));
  const plate = (guid, z0, z1) => ({ guid, type: 'Contour plate', solid: box([0, 0, z0], [100, 100, z1]) });
  const base = world([plate('A', 0, 10), plate('B', 20, 30)]);
  const revised = world([plate('A', 0, 10), plate('B', 5.24, 15.24)]);
  for (const [name, w] of [['base', base], ['revised', revised]]) {
    mkdirSync(join(root, name), { recursive: true });
    writeFileSync(join(root, name, 'entities.json'), canonicalJson(w.docs.entities));
    writeFileSync(join(root, name, 'properties.json'), canonicalJson(w.docs.properties));
    writeFileSync(join(root, name, 'relationships.json'), canonicalJson(w.docs.relationships));
    writeGeometry(join(root, `${name}-geometry`), w.tiles.tiles);
  }
  writeFileSync(join(root, 'profile.json'), JSON.stringify(PROFILE));
  writeFileSync(join(root, 'checks.json'), JSON.stringify(clashChecks()));
  return { root, revised };
}

for (const entry of ['index.mjs', 'model-dispatcher.mjs']) {
  test(`${entry} check-snapshots with clash rules: geometry directories in, the 4.76 mm overlap out, digests of the exact bytes`, () => {
    const { root, revised } = clashFixture();
    try {
      const extra = { 'base-geometry-dir': join(root, 'base-geometry'), 'revised-geometry-dir': join(root, 'revised-geometry') };
      const r = run(entry, request(root, extra));
      assert.equal(r.status, 0, r.stderr);
      const out = JSON.parse(r.stdout);
      assert.equal(r.stdout, canonicalJson(out));
      assert.deepEqual(out.findings.map((f) => `${f.ruleId}:${f.evidence.overlapThicknessMm}`), ['clash.interpenetration:4.76']);
      assert.equal(out.inputs.geometry.revised.tiles[0].sha256, sha(revised.tiles.tiles[0].bytes));
      assert.equal(out.inputs.geometry.revised.tiles[0].sha256, sha(readFileSync(join(root, 'revised-geometry', 'tile-0.glb'))));
      assert.deepEqual(out.focusSets, []);
      assert.equal(run(entry, request(root, extra)).stdout, r.stdout, 'a second run is byte-identical');
    } finally { rmSync(root, { recursive: true, force: true }); }
  });
}

test('check-snapshots with clash rules and no geometry input is refused, and a tile that does not match its digest is too', () => {
  const { root } = clashFixture();
  try {
    const none = run('index.mjs', request(root));
    assert.notEqual(none.status, 0);
    assert.equal(none.stdout, '');
    assert.match(none.stderr, /`base-geometry-dir` and `revised-geometry-dir` are required/);

    writeFileSync(join(root, 'revised-geometry', 'tile-0.glb'), Buffer.concat([readFileSync(join(root, 'revised-geometry', 'tile-0.glb')), Buffer.alloc(4)]));
    const tampered = run('index.mjs', request(root, { 'base-geometry-dir': join(root, 'base-geometry'), 'revised-geometry-dir': join(root, 'revised-geometry') }));
    assert.notEqual(tampered.status, 0);
    assert.equal(tampered.stdout, '');
    assert.match(tampered.stderr, /does not match its sha256/);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test('a profile without clash rules ignores geometry directories entirely', () => {
  const root = fixture();
  try {
    const r = run('index.mjs', request(root, { 'base-geometry-dir': join(root, 'nowhere'), 'revised-geometry-dir': join(root, 'nowhere') }));
    assert.equal(r.status, 0, r.stderr);
    assert.equal(JSON.parse(r.stdout).inputs.geometry, undefined);
    assert.equal(r.stdout, run('index.mjs', request(root)).stdout);
  } finally { rmSync(root, { recursive: true, force: true }); }
});
