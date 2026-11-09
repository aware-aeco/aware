// The process-boundary path for `compare-snapshots`: files in, canonical JSON out, through the real
// entry points (`index.mjs` and the shipped dispatcher). `snapshot-compare.test.mjs` covers the rules with
// plain objects; this proves the dispatch accepts a command with no `ifc-path`, that it never opens web-ifc,
// that stdout carries ONLY the result, and that a bad request fails without writing a half result.
import test from 'node:test';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { PROFILE } from './snapshot-compare-fixtures.mjs';
import { canonicalJson } from './snapshot-compare.mjs';

const here = new URL('.', import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1');
const sha = (bytes) => createHash('sha256').update(bytes).digest('hex');

function writeSide(dir, objects) {
  mkdirSync(dir, { recursive: true });
  const entities = objects.map((o, i) => ({ id: `element:${i + 1}`, type: 'BEAM', name: o.name ?? 'b', geometry: [{ tileOrdinal: 0, bounds: [0, 0, 0, 1, 1, 1] }] }));
  const properties = objects.flatMap((o, i) => [
    { id: `p:${i}:0`, entityId: `element:${i + 1}`, groupName: 'g', groupOrdinal: 0, name: 'GUID', value: o.guid, valueType: 'string', unit: null, propertyOrdinal: 0, semantic: 'x', dimension: 'none', isUda: false, sourceKey: 'native:guid' },
    { id: `p:${i}:1`, entityId: `element:${i + 1}`, groupName: 'g', groupOrdinal: 0, name: 'profile', value: o.profile ?? 'HEA200', valueType: 'string', unit: null, propertyOrdinal: 1, semantic: 'x', dimension: 'none', isUda: false, sourceKey: 'native:profile' },
  ]);
  const files = { 'entities.json': { entities }, 'properties.json': { properties }, 'relationships.json': { relationships: [] } };
  for (const [name, doc] of Object.entries(files)) writeFileSync(join(dir, name), canonicalJson(doc));
}

function fixture() {
  const root = mkdtempSync(join(tmpdir(), 'snapshot-compare-'));
  writeSide(join(root, 'base'), [{ guid: 'G1' }, { guid: 'G2' }, { guid: 'G3' }]);
  writeSide(join(root, 'revised'), [{ guid: 'G1' }, { guid: 'G2', profile: 'HEB300' }, { guid: 'G4' }]);
  writeFileSync(join(root, 'profile.json'), JSON.stringify(PROFILE, null, 2));
  writeFileSync(join(root, 'witness.json'), JSON.stringify({ entries: [{ identity: 'lineage-A:G2', action: 'modified' }] }));
  return root;
}
const request = (root, extra = {}) => ({ 'base-dir': join(root, 'base'), 'revised-dir': join(root, 'revised'), 'profile-path': join(root, 'profile.json'), ...extra });
const run = (entry, input) => spawnSync(process.execPath, [join(here, entry), 'compare-snapshots'], { input: JSON.stringify(input), encoding: 'utf8', cwd: here });

for (const entry of ['index.mjs', 'model-dispatcher.mjs']) {
  test(`${entry} compare-snapshots: files in, one canonical JSON document on stdout, digests of the exact bytes`, () => {
    const root = fixture();
    try {
      const r = run(entry, request(root, { 'witness-path': join(root, 'witness.json') }));
      assert.equal(r.status, 0, r.stderr);
      const out = JSON.parse(r.stdout);                       // stdout is PURE JSON: nothing else was written to it
      assert.equal(r.stdout, canonicalJson(out), 'the bytes written are the canonical form');
      assert.equal(out.schema, 'aware.model-change-set/v1');
      assert.deepEqual([out.summary.added, out.summary.removed, out.summary.changed, out.summary.unchanged], [1, 1, 1, 1]);
      assert.deepEqual(out.changes.map((c) => `${c.status}:${c.identity}`), ['removed:lineage-A:G3', 'added:lineage-A:G4', 'changed:lineage-A:G2']);
      const changed = out.changes[2];
      assert.deepEqual(changed.fields.map((f) => [f.key, f.kind, f.before, f.after]), [['native:profile', 'shape-fields', 'HEA200', 'HEB300']]);
      for (const side of ['base', 'revised']) {
        for (const file of ['entities', 'properties', 'relationships']) {
          assert.equal(out.inputs[side][file], sha(readFileSync(join(root, side, `${file}.json`))), `${side} ${file} digest is the sha256 of the file's bytes`);
        }
      }
      assert.equal(out.inputs.profileDigest, sha(canonicalJson(PROFILE)));
      assert.equal(out.inputs.witnessDigest, sha(readFileSync(join(root, 'witness.json'))));
      assert.equal(out.witness.agrees.count, 1);
      // Second run: byte-identical.
      assert.equal(run(entry, request(root, { 'witness-path': join(root, 'witness.json') })).stdout, r.stdout);
    } finally { rmSync(root, { recursive: true, force: true }); }
  });
}

test('compare-snapshots fails cleanly: no result on stdout, a message on stderr, non-zero exit', () => {
  const root = fixture();
  try {
    const missing = run('index.mjs', { 'base-dir': join(root, 'base') });
    assert.notEqual(missing.status, 0);
    assert.equal(missing.stdout, '');
    assert.match(missing.stderr, /`revised-dir` is required/);

    writeFileSync(join(root, 'profile.json'), JSON.stringify({ ...PROFILE, surprise: true }));
    const strict = run('index.mjs', request(root));
    assert.notEqual(strict.status, 0);
    assert.equal(strict.stdout, '');
    assert.match(strict.stderr, /unknown key 'surprise'/);

    writeFileSync(join(root, 'profile.json'), JSON.stringify(PROFILE));
    writeFileSync(join(root, 'base', 'entities.json'), '{"entities":[],"extra":[]}');
    const shape = run('index.mjs', request(root));
    assert.notEqual(shape.status, 0);
    assert.equal(shape.stdout, '');
    assert.match(shape.stderr, /must be exactly \{ entities/);
  } finally { rmSync(root, { recursive: true, force: true }); }
});
