import assert from 'node:assert/strict';
import { createPrivateKey, createPublicKey } from 'node:crypto';
import test from 'node:test';
import { signArtifactPreimage } from './model-artifact-auth.mjs';
import { SOURCE_ARTIFACT_DOMAIN } from './model-snapshot.mjs';

const SECRET_PREFIX = Buffer.from('302e020100300506032b657004220420', 'hex');

test('artifact signature bytes match the frozen Ed25519/JCS golden vector', () => {
  const secret = Buffer.from(Array.from({ length: 32 }, (_, index) => index));
  const privateKey = createPrivateKey({ key: Buffer.concat([SECRET_PREFIX, secret]), format: 'der', type: 'pkcs8' });
  const publicKeyBytes = createPublicKey(privateKey).export({ format: 'der', type: 'spki' }).subarray(-32);
  const preimage = {
    schemaVersion: '1', source: { z: 'last', a: 'first' },
    outputs: [{ logicalName: 'geometry', mediaType: 'model/gltf-binary', bytes: 4, items: 1, sha256: '0'.repeat(64) }],
  };
  assert.deepEqual(signArtifactPreimage(SOURCE_ARTIFACT_DOMAIN, preimage, { privateKey, publicKeyBytes }), {
    schemaVersion: '1',
    algorithm: 'Ed25519-SHA256',
    keyFingerprintSha256: '56475aa75463474c0285df5dbf2bcab73da651358839e9b77481b2eab107708c',
    publicKeyBase64: 'A6EHv/POEL4dcN0Y50vAmWfk1jCbpQ1fHdyGZBJVMbg=',
    preimageSha256: '786b1cc779f0f5fff1669b493eb061b81a1fde26a8f81b3fe8c2ee2365a4891f',
    signatureBase64: 't8yHCucQKyU/O96YnOkSkjebI6u/6ZDGoqbMoTfEStR/PqX7LW68LntYc/41aVhpu7JbvIMkh1UzMiIG9j4HDQ==',
  });
});

test('artifact signatures are domain separated', () => {
  const secret = Buffer.alloc(32, 7);
  const privateKey = createPrivateKey({ key: Buffer.concat([SECRET_PREFIX, secret]), format: 'der', type: 'pkcs8' });
  const publicKeyBytes = createPublicKey(privateKey).export({ format: 'der', type: 'spki' }).subarray(-32);
  const key = { privateKey, publicKeyBytes };
  const preimage = { schemaVersion: '1', source: {}, outputs: [] };
  const source = signArtifactPreimage(SOURCE_ARTIFACT_DOMAIN, preimage, key);
  const other = signArtifactPreimage('AWARE\0model-reference-reader\0package-set\0v1\0', preimage, key);
  assert.equal(source.preimageSha256, other.preimageSha256);
  assert.notEqual(source.signatureBase64, other.signatureBase64);
});
