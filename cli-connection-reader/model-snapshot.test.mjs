import assert from 'node:assert/strict';
import { generateKeyPairSync } from 'node:crypto';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { canonicalJsonBytes, sha256 } from './model-contract.mjs';
import { buildAndPublishSnapshot } from './model-snapshot.mjs';

test('snapshot parsing honors the configured component limit above the strict parser default', async (t) => {
  const root = await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-model-snapshot-'));
  t.after(() => fs.rm(root, { recursive: true, force: true }));
  const { privateKey, publicKey } = generateKeyPairSync('ed25519');
  const publicKeyBytes = publicKey.export({ format: 'der', type: 'spki' }).subarray(-32);
  const largeProperties = canonicalJsonBytes({
    schemaVersion: '1', properties: [{ value: 'x'.repeat((16 * 1024 * 1024) + 1) }],
  });
  assert.equal(largeProperties.length > 16 * 1024 * 1024, true, 'fixture must cross the old implicit parser ceiling');
  const artifacts = {
    'geometry.glb': Buffer.from([0x67, 0x6c, 0x54, 0x46]),
    'entities.json': canonicalJsonBytes({ schemaVersion: '1', entities: [] }),
    'properties.json': largeProperties,
    'relationships.json': canonicalJsonBytes({ schemaVersion: '1', relationships: [] }),
    'manifest.json': canonicalJsonBytes({ schemaVersion: 'model-reference-manifest/v1' }),
  };
  const result = {
    key: '1'.repeat(64),
    cache: {
      receiptSha256: '2'.repeat(64), artifacts,
      manifest: {
        identity: {
          sourceSha256: '3'.repeat(64), signerFingerprintSha256: sha256(publicKeyBytes),
        },
        canonicalRequestSha256: '4'.repeat(64), providerFingerprintSha256: '5'.repeat(64),
        frame: { units: 'mm', up: 'z', handedness: 'right', axes: ['x', 'y', 'z'] },
        coverage: { unclaimedGeometryNodes: [] },
      },
    },
  };
  const output = await buildAndPublishSnapshot(result, { privateKey, publicKeyBytes }, path.join(root, 'artifacts'), {
    limits: { maxComponentJsonBytes: 20 * 1024 * 1024 },
  });
  assert.equal(output.sourceArtifactPreimage.outputs.find((item) => item.logicalName === 'properties').bytes, largeProperties.length);
  assert.equal(output.packageArtifacts['properties-000000'].bytes, largeProperties.length);
});
