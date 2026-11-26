import path from 'node:path';

import { assertClosedObject, canonicalJsonBytes, ModelReaderError, parseJsonStrict } from './model-contract.mjs';

// A protocol-v3 provider may refuse a source it cannot read by exiting nonzero with one exact
// canonical JSON object on stderr. Only the codes below are recognised, each in the phases it is
// meaningful in, and none of the provider's text is forwarded: the public message and details are
// either AWARE's own text or values AWARE has recomputed from the request it sent (#883).
const PROVIDER_REFUSAL_BYTES = 4096;
const UUID = /^[0-9a-f]{8}-[0-9a-f]{4}-[1-8][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;
const COVERAGE_CODE = 'reference-model-coverage-incomplete';
const PATH_CODE = 'reference-source-path-too-long';
const ALLOWED_PROVIDER_REFUSALS = Object.freeze({
  [COVERAGE_CODE]: Object.freeze({
    phases: Object.freeze(['conversion']),
    message: 'The model contains geometry this reader cannot safely place. No partial model was imported.',
  }),
  [PATH_CODE]: Object.freeze({
    phases: Object.freeze(['discovery', 'conversion']),
    message: 'A source file path is longer than this system can open. No partial model was imported.',
  }),
});
const COVERAGE_COUNTS = /^coverage-counts-v1:(?:0|[1-9][0-9]{0,7})(?:,(?:0|[1-9][0-9]{0,7})){5}$/;
const PATH_LENGTH = /^path-length-v1:([1-9][0-9]{0,4}),([1-9][0-9]{0,4})$/;
// Classic Windows MAX_PATH is 260 UTF-16 units including the terminating NUL, so the longest
// usable path is 259. It is the only limit AWARE can corroborate today, so it is the only one
// accepted; 32767 is the extended-length (\\?\) ceiling.
export const SOURCE_PATH_LIMIT = 259;
const EXTENDED_PATH_LIMIT = 32_767;
// The Rust bridge (`structured_bridge_error`) drops an envelope with a detail value over 256 bytes.
const DETAIL_VALUE_BYTES = 256;

function safeCoverageCounts(message) {
  if (message.match(COVERAGE_COUNTS)?.[0] !== message) return null;
  const counts = message.slice('coverage-counts-v1:'.length).split(',').map(Number);
  return counts.every((count) => count <= 10_000_000) && counts.some((count) => count !== 0)
    ? message : null;
}

function capturedFile(capture, source) {
  if (!source || typeof source !== 'object' || Array.isArray(source)) return null;
  try { assertClosedObject(source, ['namespaceId', 'path'], [], 'provider refusal source'); }
  catch { return null; }
  if (typeof source.namespaceId !== 'string' || typeof source.path !== 'string') return null;
  const namespaces = capture?.manifest?.namespaces;
  if (!Array.isArray(namespaces)) return null;
  for (const namespace of namespaces) {
    if (namespace?.namespaceId !== source.namespaceId || !Array.isArray(namespace.files)) continue;
    const file = namespace.files.find((entry) => entry?.path === source.path);
    // AWARE's own manifest strings are forwarded, never the provider's copies of them.
    if (file) return { namespaceId: namespace.namespaceId, path: file.path };
  }
  return null;
}

// The provider measures the absolute path it opened, formed from the `capture.root` AWARE sent
// it. AWARE recomputes that length itself, so every forwarded number is one AWARE corroborated.
function safePathRefusal(refusal, capture, platform) {
  if (platform !== 'win32' || typeof capture?.stagingRoot !== 'string') return null;
  const match = refusal.message.match(PATH_LENGTH);
  if (!match || match[0] !== refusal.message) return null;
  const length = Number(match[1]); const limit = Number(match[2]);
  if (limit !== SOURCE_PATH_LIMIT || length <= limit || length > EXTENDED_PATH_LIMIT) return null;
  const file = capturedFile(capture, refusal.source);
  if (!file) return null;
  const absolute = path.win32.join(capture.stagingRoot, 'namespaces', file.namespaceId, ...file.path.split('/'));
  if (absolute.length !== length) return null;
  const details = { pathLength: String(length), pathLimit: String(limit) };
  if (Buffer.byteLength(file.namespaceId, 'utf8') <= DETAIL_VALUE_BYTES
      && Buffer.byteLength(file.path, 'utf8') <= DETAIL_VALUE_BYTES) {
    details.sourceNamespace = file.namespaceId;
    details.sourcePath = file.path;
  }
  return { code: PATH_CODE, message: `path-length-v1:${length},${limit}`, details };
}

/**
 * Admit a provider refusal from a failed provider call, or return null so the caller keeps its
 * generic `reference-provider-failed`. `capture` is the capture handed to THAT call (discovery:
 * the full capture; conversion: the consumed closure).
 */
export function parseProviderRefusal(result, { phase, stderrLimit, capture, platform = process.platform }) {
  if (!result || !Number.isInteger(result.exitCode) || result.exitCode === 0
      || !Buffer.isBuffer(result.stdout)
      || result.stdout.length !== 0 || !Buffer.isBuffer(result.stderr)
      || result.stderr.length === 0
      || result.stderr.length > Math.min(PROVIDER_REFUSAL_BYTES, stderrLimit)) return null;
  try {
    const refusal = parseJsonStrict(result.stderr, { maxBytes: PROVIDER_REFUSAL_BYTES, maxDepth: 3 });
    const allowed = typeof refusal?.code === 'string' && Object.hasOwn(ALLOWED_PROVIDER_REFUSALS, refusal.code)
      ? ALLOWED_PROVIDER_REFUSALS[refusal.code] : null;
    if (!allowed) return null;
    assertClosedObject(refusal, ['code', 'phase', 'retryable', 'message', 'diagnosticId'],
      refusal.code === PATH_CODE ? ['source'] : [], 'provider refusal');
    if (refusal.phase !== phase || !allowed.phases.includes(phase) || refusal.retryable !== false
        || typeof refusal.message !== 'string' || !refusal.message.trim()
        || refusal.message.length > 240 || /[\u0000-\u001f\u007f]/.test(refusal.message)
        || typeof refusal.diagnosticId !== 'string' || !UUID.test(refusal.diagnosticId)
        || !result.stderr.equals(canonicalJsonBytes(refusal))) return null;
    if (refusal.code === PATH_CODE) return safePathRefusal(refusal, capture, platform);
    return { code: refusal.code, message: safeCoverageCounts(refusal.message) ?? allowed.message };
  } catch {
    return null;
  }
}

/** The non-retryable error for an admitted refusal; raw provider output stays private evidence. */
export function providerRefusalError(phase, refusal, result) {
  const error = new ModelReaderError(refusal.code, phase, false, refusal.message, result);
  if (refusal.details) error.setPublicDetails(refusal.details);
  return error;
}
