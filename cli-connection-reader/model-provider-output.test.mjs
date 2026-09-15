import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

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
    files.push({ path: relative, kind, ordinal: 0, mediaType, bytes: bytes.length, sha256: sha256(bytes), count });
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
