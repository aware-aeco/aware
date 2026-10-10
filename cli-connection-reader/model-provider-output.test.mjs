import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import { REQUEST, runAdmission, sealOutput } from './bench-admission.mjs';
import { canonicalJsonBytes, sha256 } from './model-contract.mjs';
import { verifyProviderOutput } from './model-provider-output.mjs';

async function fixture(t) {
  const root = await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-provider-output-'));
  t.after(() => fs.rm(root, { recursive: true, force: true }));
  await fs.mkdir(path.join(root, 'geometry')); await fs.mkdir(path.join(root, 'metadata'));
  const payloads = [
    ['geometry/000000.glb', 'geometry', 'model/gltf-binary', Buffer.from('glb'), 1],
    ['metadata/entities-000000.jsonl', 'entities', 'application/x-ndjson', Buffer.from('{"id":"one"}\n'), 1],
    ['metadata/properties-000000.jsonl', 'properties', 'application/x-ndjson', Buffer.from(''), 0],
  ];
  const files = [];
  for (const [relative, kind, mediaType, bytes, count] of payloads) {
    await fs.writeFile(path.join(root, ...relative.split('/')), bytes);
    files.push({ path: relative, kind, ordinal: 0, mediaType, bytes: bytes.length, sha256: sha256(bytes), count,
      ...(kind === 'geometry' ? { bounds: [0, 0, 0, 1, 1, 1] } : {}) });
  }
  const manifest = {
    schemaVersion: 'aware.model-provider-output-manifest/v1', protocolVersion: '3',
    formatId: 'format.synthetic', capabilityId: 'capability.synthetic',
    providerPackageManifestSha256: sha256(Buffer.from('package')),
    effectiveSourceSha256: sha256(Buffer.from('source')),
    conversionRequestSha256: sha256(Buffer.from('request')), files,
  };
  const manifestBytes = canonicalJsonBytes(manifest);
  await fs.writeFile(path.join(root, 'intermediate-manifest.json'), manifestBytes);
  const completion = {
    schemaVersion: 'aware.model-provider-output-completion/v1', manifestPath: 'intermediate-manifest.json',
    manifestBytes: manifestBytes.length, manifestSha256: sha256(manifestBytes),
  };
  await fs.writeFile(path.join(root, 'complete.json'), canonicalJsonBytes(completion));
  const admittedRoot = path.join(root, '..', `${path.basename(root)}-admitted`);
  t.after(() => fs.rm(admittedRoot, { recursive: true, force: true }));
  return {
    root, manifest,
    options: {
      formatId: manifest.formatId, capabilityId: manifest.capabilityId,
      providerPackageManifestSha256: manifest.providerPackageManifestSha256,
      effectiveSourceSha256: manifest.effectiveSourceSha256,
      conversionRequestSha256: manifest.conversionRequestSha256,
      admittedRoot,
    },
  };
}

test('verifies a closed canonical provider output package', async (t) => {
  const value = await fixture(t);
  const result = await verifyProviderOutput(value.root, value.options);
  assert.equal(result.manifestSha256, sha256(canonicalJsonBytes(value.manifest)));
  assert.equal(result.root, value.options.admittedRoot);
  assert.equal(await fs.readFile(path.join(result.root, 'geometry', '000000.glb'), 'utf8'), 'glb');
  assert.deepEqual(result.files.map((entry) => entry.path), value.manifest.files.map((entry) => entry.path));
});

test('refuses output before the completion marker exists', async (t) => {
  const value = await fixture(t);
  await fs.rm(path.join(value.root, 'complete.json'));
  await assert.rejects(() => verifyProviderOutput(value.root, value.options), (error) => error.code === 'reference-provider-output-incomplete');
});

test('refuses unreceipted and changed payload bytes', async (t) => {
  const extra = await fixture(t);
  await fs.writeFile(path.join(extra.root, 'metadata', 'extra.bin'), 'extra');
  await assert.rejects(() => verifyProviderOutput(extra.root, extra.options), (error) => error.code === 'reference-provider-output-invalid');
  const changed = await fixture(t);
  await fs.writeFile(path.join(changed.root, 'geometry', '000000.glb'), 'changed');
  await assert.rejects(() => verifyProviderOutput(changed.root, changed.options), (error) => error.code === 'reference-provider-output-invalid');
});

test('refuses non-contiguous shard ordinals', async (t) => {
  const value = await fixture(t);
  const entry = value.manifest.files[1];
  entry.path = 'metadata/entities-000001.jsonl'; entry.ordinal = 1;
  await fs.rename(path.join(value.root, 'metadata', 'entities-000000.jsonl'), path.join(value.root, 'metadata', 'entities-000001.jsonl'));
  const bytes = canonicalJsonBytes(value.manifest);
  await fs.writeFile(path.join(value.root, 'intermediate-manifest.json'), bytes);
  await fs.writeFile(path.join(value.root, 'complete.json'), canonicalJsonBytes({
    schemaVersion: 'aware.model-provider-output-completion/v1', manifestPath: 'intermediate-manifest.json',
    manifestBytes: bytes.length, manifestSha256: sha256(bytes),
  }));
  await assert.rejects(() => verifyProviderOutput(value.root, value.options), (error) => error.code === 'reference-provider-output-invalid');
});

test('enforces caller-selected output ceilings', async (t) => {
  const value = await fixture(t);
  await assert.rejects(
    () => verifyProviderOutput(value.root, { ...value.options, limits: { aggregateBytes: 2 } }),
    (error) => error.code === 'reference-provider-output-limit',
  );
});

test('maps uncanonicalizable provider JSON to the provider-output error contract', async (t) => {
  const value = await fixture(t);
  const manifestPath = path.join(value.root, 'intermediate-manifest.json');
  const raw = Buffer.from((await fs.readFile(manifestPath, 'utf8')).replace('"count":1', '"count":1e20'));
  await fs.writeFile(manifestPath, raw);
  await fs.writeFile(path.join(value.root, 'complete.json'), canonicalJsonBytes({
    schemaVersion: 'aware.model-provider-output-completion/v1', manifestPath: 'intermediate-manifest.json',
    manifestBytes: raw.length, manifestSha256: sha256(raw),
  }));
  await assert.rejects(
    () => verifyProviderOutput(value.root, value.options),
    (error) => error.code === 'reference-provider-output-invalid',
  );
});

test('counts JSONL records from the payload instead of trusting the receipt', async (t) => {
  const value = await fixture(t);
  const relative = 'metadata/entities-000000.jsonl';
  const payload = Buffer.from('{"id":"one"}\n{"id":"two"}\n');
  await fs.writeFile(path.join(value.root, ...relative.split('/')), payload);
  const receipt = value.manifest.files.find((entry) => entry.path === relative);
  receipt.bytes = payload.length; receipt.sha256 = sha256(payload);
  const manifestBytes = canonicalJsonBytes(value.manifest);
  await fs.writeFile(path.join(value.root, 'intermediate-manifest.json'), manifestBytes);
  await fs.writeFile(path.join(value.root, 'complete.json'), canonicalJsonBytes({
    schemaVersion: 'aware.model-provider-output-completion/v1', manifestPath: 'intermediate-manifest.json',
    manifestBytes: manifestBytes.length, manifestSha256: sha256(manifestBytes),
  }));
  await assert.rejects(
    () => verifyProviderOutput(value.root, value.options),
    (error) => error.code === 'reference-provider-output-invalid',
  );
});

test('binds provider output to the exact conversion request', async (t) => {
  const value = await fixture(t);
  await assert.rejects(
    () => verifyProviderOutput(value.root, { ...value.options, effectiveSourceSha256: sha256(Buffer.from('other')) }),
    (error) => error.code === 'reference-provider-output-invalid',
  );
});

test('rejects inherited kind names through the provider-output error contract', async (t) => {
  const value = await fixture(t);
  value.manifest.files[0].kind = 'toString';
  const manifestBytes = canonicalJsonBytes(value.manifest);
  await fs.writeFile(path.join(value.root, 'intermediate-manifest.json'), manifestBytes);
  await fs.writeFile(path.join(value.root, 'complete.json'), canonicalJsonBytes({
    schemaVersion: 'aware.model-provider-output-completion/v1', manifestPath: 'intermediate-manifest.json',
    manifestBytes: manifestBytes.length, manifestSha256: sha256(manifestBytes),
  }));
  await assert.rejects(
    () => verifyProviderOutput(value.root, value.options),
    (error) => error.code === 'reference-provider-output-invalid',
  );
});

test('requires string format and capability IDs', async (t) => {
  const value = await fixture(t);
  await assert.rejects(
    () => verifyProviderOutput(value.root, { ...value.options, formatId: 123 }),
    (error) => error.code === 'reference-provider-output-request-invalid',
  );
});

test('never removes a pre-existing admitted output root', async (t) => {
  const value = await fixture(t);
  await fs.mkdir(value.options.admittedRoot);
  const sentinel = path.join(value.options.admittedRoot, 'sentinel.txt');
  await fs.writeFile(sentinel, 'keep');
  await assert.rejects(
    () => verifyProviderOutput(value.root, value.options),
    (error) => error.code === 'reference-provider-output-root-invalid',
  );
  assert.equal(await fs.readFile(sentinel, 'utf8'), 'keep');
});

test('rejects malformed JSONL before copying it into the admitted snapshot', async (t) => {
  const value = await fixture(t);
  const relative = 'metadata/entities-000000.jsonl';
  const payload = Buffer.from('not-json\n');
  await fs.writeFile(path.join(value.root, ...relative.split('/')), payload);
  const receipt = value.manifest.files.find((entry) => entry.path === relative);
  receipt.bytes = payload.length; receipt.sha256 = sha256(payload);
  const manifestBytes = canonicalJsonBytes(value.manifest);
  await fs.writeFile(path.join(value.root, 'intermediate-manifest.json'), manifestBytes);
  await fs.writeFile(path.join(value.root, 'complete.json'), canonicalJsonBytes({
    schemaVersion: 'aware.model-provider-output-completion/v1', manifestPath: 'intermediate-manifest.json',
    manifestBytes: manifestBytes.length, manifestSha256: sha256(manifestBytes),
  }));
  await assert.rejects(
    () => verifyProviderOutput(value.root, value.options),
    (error) => error.code === 'reference-provider-output-invalid',
  );
  await assert.rejects(fs.stat(value.options.admittedRoot), (error) => error.code === 'ENOENT');
});

test('honors cancellation and removes its partial admitted snapshot', async (t) => {
  const value = await fixture(t);
  const controller = new AbortController();
  controller.abort();
  await assert.rejects(
    () => verifyProviderOutput(value.root, { ...value.options, signal: controller.signal }),
    (error) => error.code === 'reference-cancelled' && error.retryable === false,
  );
  await assert.rejects(fs.stat(value.options.admittedRoot), (error) => error.code === 'ENOENT');
});

test('admits an output with no geometry tiles; completeness is gated at canonicalization (#604)', async (t) => {
  const value = await fixture(t);
  await fs.rm(path.join(value.root, 'geometry', '000000.glb'));
  value.manifest.files = value.manifest.files.filter((entry) => entry.kind !== 'geometry');
  const manifestBytes = canonicalJsonBytes(value.manifest);
  await fs.writeFile(path.join(value.root, 'intermediate-manifest.json'), manifestBytes);
  await fs.writeFile(path.join(value.root, 'complete.json'), canonicalJsonBytes({
    schemaVersion: 'aware.model-provider-output-completion/v1', manifestPath: 'intermediate-manifest.json',
    manifestBytes: manifestBytes.length, manifestSha256: sha256(manifestBytes),
  }));
  const result = await verifyProviderOutput(value.root, value.options);
  assert.deepEqual(result.files.map((entry) => entry.kind), ['entities', 'properties']);
  assert.deepEqual(await fs.readdir(path.join(result.root, 'geometry')), []);
});

test('still requires entity shards and bounds on every geometry receipt (#604)', async (t) => {
  const rewrite = async (value) => {
    const manifestBytes = canonicalJsonBytes(value.manifest);
    await fs.writeFile(path.join(value.root, 'intermediate-manifest.json'), manifestBytes);
    await fs.writeFile(path.join(value.root, 'complete.json'), canonicalJsonBytes({
      schemaVersion: 'aware.model-provider-output-completion/v1', manifestPath: 'intermediate-manifest.json',
      manifestBytes: manifestBytes.length, manifestSha256: sha256(manifestBytes),
    }));
  };
  const noEntities = await fixture(t);
  await fs.rm(path.join(noEntities.root, 'metadata', 'entities-000000.jsonl'));
  noEntities.manifest.files = noEntities.manifest.files.filter((entry) => entry.kind !== 'entities');
  await rewrite(noEntities);
  await assert.rejects(() => verifyProviderOutput(noEntities.root, noEntities.options),
    (error) => error.code === 'reference-provider-output-invalid');

  const noBounds = await fixture(t);
  delete noBounds.manifest.files.find((entry) => entry.kind === 'geometry').bounds;
  await rewrite(noBounds);
  await assert.rejects(() => verifyProviderOutput(noBounds.root, noBounds.options),
    (error) => error.code === 'reference-provider-output-invalid'
      && error.message === 'A geometry tile receipt has invalid bounds.');
});

// ---- admission read-once behavior (aware-aeco/aware#681) ----------------------------------------


function lcg(seed) {
  let state = seed >>> 0;
  return () => { state = (Math.imul(state, 1664525) + 1013904223) >>> 0; return state / 0x100000000; };
}

// A provider output with caller-chosen JSONL records (so records can straddle the 1 MiB read chunks).
async function largeFixture(t, seed, perShard) {
  const root = await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-admission-'));
  t.after(() => fs.rm(root, { recursive: true, force: true }));
  await fs.mkdir(path.join(root, 'geometry')); await fs.mkdir(path.join(root, 'metadata'));
  const random = lcg(seed); const files = []; const sources = new Map();
  const add = async (relative, kind, mediaType, bytes, count) => {
    await fs.writeFile(path.join(root, ...relative.split('/')), bytes);
    sources.set(relative, bytes);
    files.push({ path: relative, kind, ordinal: 0, mediaType, bytes: bytes.length, sha256: sha256(bytes), count,
      ...(kind === 'geometry' ? { bounds: [0, 0, 0, 1, 1, 1] } : {}) });
  };
  await add('geometry/000000.glb', 'geometry', 'model/gltf-binary', Buffer.from('glb'), 1);
  for (const kind of ['entities', 'properties', 'relationships']) {
    const lines = [];
    for (let index = 0; index < perShard; index += 1) {
      // Record sizes from tiny to ~9 KB, with multi-byte text, so chunk edges land everywhere.
      const text = `${'é中x'.repeat(Math.floor(random() * 1500))}${index}`;
      lines.push(canonicalJsonBytes({ id: `${kind}:${index}`, text }).toString('utf8'));
    }
    await add(`metadata/${kind}-000000.jsonl`, kind, 'application/x-ndjson',
      Buffer.from(`${lines.join('\n')}\n`), perShard);
  }
  await sealOutput(root, files);
  const admittedRoot = path.join(root, '..', `${path.basename(root)}-admitted`);
  t.after(() => fs.rm(admittedRoot, { recursive: true, force: true }));
  return { root, files, sources, options: { ...REQUEST, admittedRoot } };
}

test('records straddling read chunks are admitted and the copy is byte-identical to the source', async (t) => {
  for (const seed of [1, 2, 3]) {
    const value = await largeFixture(t, seed, 700);
    assert.ok(value.sources.get('metadata/entities-000000.jsonl').length > 2 * 1024 * 1024,
      'the shard must span several 1 MiB reads');
    const result = await verifyProviderOutput(value.root, value.options);
    for (const [relative, bytes] of value.sources) {
      assert.ok(bytes.equals(await fs.readFile(path.join(result.root, ...relative.split('/')))), relative);
    }
    await fs.rm(value.options.admittedRoot, { recursive: true, force: true });
  }
});

test('a record straddling a read chunk that is not canonical is still refused', async (t) => {
  const value = await largeFixture(t, 9, 700);
  const entities = path.join(value.root, 'metadata', 'entities-000000.jsonl');
  const bytes = await fs.readFile(entities);
  // Break canonical form (space after a colon) in the record that STARTS before the first 1 MiB read
  // boundary and ENDS after it, so it is assembled from two chunks.
  const start = bytes.lastIndexOf(0x0a, 1024 * 1024 - 1) + 1;
  assert.ok(start < 1024 * 1024 && bytes.indexOf(0x0a, start) >= 1024 * 1024, 'record must span the boundary');
  const at = bytes.indexOf(Buffer.from('"id":'), start);
  assert.ok(at >= start && at < bytes.indexOf(0x0a, start));
  const broken = Buffer.concat([bytes.subarray(0, at + 5), Buffer.from(' '), bytes.subarray(at + 5)]);
  await fs.writeFile(entities, broken);
  const receipt = value.files.find((entry) => entry.path === 'metadata/entities-000000.jsonl');
  receipt.bytes = broken.length; receipt.sha256 = sha256(broken);
  await sealOutput(value.root, value.files);
  await assert.rejects(() => verifyProviderOutput(value.root, value.options),
    (error) => error.code === 'reference-provider-output-invalid');
});

test('no third read: each shard is opened once from the source and twice at the admitted path', async (t) => {
  const value = await largeFixture(t, 4, 50);
  const opened = new Map(); const original = fs.open;
  fs.open = async (pathname, ...rest) => {
    opened.set(String(pathname), (opened.get(String(pathname)) ?? 0) + 1);
    return original(pathname, ...rest);
  };
  try { await verifyProviderOutput(value.root, value.options); } finally { fs.open = original; }
  for (const entry of value.files) {
    assert.equal(opened.get(path.join(value.root, ...entry.path.split('/'))), 1, `${entry.path} source opens`);
    // One 'wx' open creates the copy, one more re-reads it.
    assert.equal(opened.get(path.join(value.options.admittedRoot, ...entry.path.split('/'))), 2, `${entry.path} admitted opens`);
  }
});

test('records are canonical-checked once (the admitted copy is not re-parsed)', async (t) => {
  const records = 3 * 50;
  const value = await largeFixture(t, 6, 50);
  // Every record check ends in one Buffer#equals against its canonical re-encoding; the old second
  // pass over the admitted copy doubled the count (about 2 x records).
  let calls = 0; const original = Buffer.prototype.equals;
  Buffer.prototype.equals = function counted(other) { calls += 1; return original.call(this, other); };
  try { await verifyProviderOutput(value.root, value.options); } finally { Buffer.prototype.equals = original; }
  assert.ok(calls >= records, `expected at least ${records} record checks, saw ${calls}`);
  assert.ok(calls <= records + 20, `records were checked more than once: ${calls} checks for ${records} records`);
});

test('a corrupted admitted copy is refused and removed (the copy is digest-checked, not trusted)', async (t) => {
  const value = await largeFixture(t, 5, 50);
  const original = fs.open;
  fs.open = async (pathname, flags, ...rest) => {
    const handle = await original(pathname, flags, ...rest);
    if (flags === 'wx' && String(pathname).endsWith('properties-000000.jsonl')) {
      const write = handle.writeFile.bind(handle);
      handle.writeFile = (chunk) => { const bad = Buffer.from(chunk); bad[bad.length - 2] ^= 1; return write(bad); };
    }
    return handle;
  };
  try {
    await assert.rejects(() => verifyProviderOutput(value.root, value.options),
      (error) => error.code === 'reference-provider-output-invalid');
  } finally { fs.open = original; }
  await assert.rejects(() => fs.stat(value.options.admittedRoot));
});

test('benchmark harness admits a small synthetic output deterministically', async (t) => {
  const directory = await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-admission-bench-'));
  t.after(() => fs.rm(directory, { recursive: true, force: true }));
  const shape = { entities: 40, properties: 200, relationships: 20, glbMb: 0.1 };
  const first = await runAdmission(shape, directory);
  const second = await runAdmission(shape, directory);
  assert.equal(first.digest.combined, second.digest.combined);
  assert.equal(Object.keys(first.digest.files).length, 6);
});
