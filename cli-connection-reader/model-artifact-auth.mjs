import { sign } from 'node:crypto';
import { canonicalJsonBytes, sha256 } from './model-contract.mjs';
import { signerFingerprintSha256 } from './model-cache.mjs';

function uint64be(value) {
  const bytes = Buffer.alloc(8);
  bytes.writeBigUInt64BE(BigInt(value));
  return bytes;
}

export function signArtifactPreimage(domain, preimage, signingKey) {
  if (typeof domain !== 'string' || !domain.startsWith('AWARE\0') || !domain.endsWith('\0')) {
    throw new TypeError('artifact signature domain is invalid');
  }
  const preimageBytes = canonicalJsonBytes(preimage);
  const signatureInput = Buffer.from(sha256(Buffer.concat([
    Buffer.from(domain, 'ascii'), uint64be(preimageBytes.length), preimageBytes,
  ])), 'hex');
  return {
    schemaVersion: '1',
    algorithm: 'Ed25519-SHA256',
    keyFingerprintSha256: signerFingerprintSha256(signingKey.publicKeyBytes),
    publicKeyBase64: signingKey.publicKeyBytes.toString('base64'),
    preimageSha256: sha256(preimageBytes),
    signatureBase64: sign(null, signatureInput, signingKey.privateKey).toString('base64'),
  };
}
