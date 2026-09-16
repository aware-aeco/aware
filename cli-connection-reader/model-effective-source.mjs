import {
  assertClosedObject, assertSha256, canonicalJsonBytes, ModelReaderError, sha256,
} from './model-contract.mjs';

const REPORT_SCHEMA = 'aware.model-provider-dependency-report/v1';
const POLICY_SCHEMA = 'aware.model-dependency-policy/v1';
const EFFECTIVE_SCHEMA = 'model-effective-source/v2';
const OPAQUE_ID = /^[A-Za-z0-9._-]{1,128}$/;
const WINDOWS_RESERVED = /^(?:con|prn|aux|nul|com[1-9]|lpt[1-9])(?:\.|$)/i;

function sourceError(code, message, details = undefined) {
  throw new ModelReaderError(code, 'discovery', false, message, details);
}

function closed(value, required, optional, label, code = 'reference-dependency-report-invalid') {
  try { assertClosedObject(value, required, optional, label); }
  catch (error) { sourceError(code, `The ${label} is invalid.`, error); }
  return value;
}

function opaque(value, label, code = 'reference-dependency-report-invalid') {
  if (typeof value !== 'string' || !OPAQUE_ID.test(value)) {
    sourceError(code, `${label} is not an opaque identifier.`);
  }
  return value;
}

function isUnicodeScalarString(value) {
  if (typeof value !== 'string') return false;
  for (let index = 0; index < value.length; index += 1) {
    const unit = value.charCodeAt(index);
    if (unit >= 0xd800 && unit <= 0xdbff) {
      const next = value.charCodeAt(index + 1);
      if (!(next >= 0xdc00 && next <= 0xdfff)) return false;
      index += 1;
    } else if (unit >= 0xdc00 && unit <= 0xdfff) return false;
  }
  return true;
}

function bounded(value, label, maximum = 1024) {
  if (!isUnicodeScalarString(value) || !value || Array.from(value).length > maximum
      || /[\u0000-\u001f\u007f]/.test(value) || value !== value.normalize('NFC')) {
    sourceError('reference-dependency-report-invalid', `${label} is invalid.`);
  }
  return value;
}

function digest(value, label) {
  try { return assertSha256(value, label); }
  catch (error) { sourceError('reference-dependency-report-invalid', `${label} is invalid.`, error); }
}

function fileKey(namespaceId, relativePath) {
  return `${namespaceId}\0${relativePath}`;
}

function captureNamespaceId(value) {
  if (typeof value !== 'string' || !OPAQUE_ID.test(value) || value === '.' || value === '..'
      || /[. ]$/.test(value) || WINDOWS_RESERVED.test(value)) {
    sourceError('reference-dependency-report-invalid', 'A source capture namespace ID is invalid.');
  }
  return value;
}

function portableCapturePath(value) {
  if (!isUnicodeScalarString(value) || !value || value.length > 4096
      || value !== value.normalize('NFC') || Buffer.byteLength(value, 'utf8') > 4096) {
    sourceError('reference-dependency-report-invalid', 'A source capture path is invalid.');
  }
  const parts = value.split('/');
  if (parts.length > 128 || parts.some((part) => !part || part === '.' || part === '..'
      || part.includes('\\') || part.includes(':') || /[. ]$/.test(part)
      || /[\u0000-\u001f\u007f]/.test(part) || WINDOWS_RESERVED.test(part)
      || Buffer.byteLength(part, 'utf8') > 255)) {
    sourceError('reference-dependency-report-invalid', 'A source capture path is invalid.');
  }
  return value;
}

function validateCapture(manifest, expectedSha256) {
  closed(manifest, ['schemaVersion', 'namespaces'], [], 'source capture manifest');
  if (manifest.schemaVersion !== 'aware.model-source-capture/v1' || !Array.isArray(manifest.namespaces)
      || manifest.namespaces.length < 1 || manifest.namespaces.length > 16) {
    sourceError('reference-dependency-report-invalid', 'The source capture manifest is unsupported.');
  }
  let manifestSha256;
  try { manifestSha256 = sha256(canonicalJsonBytes(manifest)); }
  catch (error) { sourceError('reference-dependency-report-invalid', 'The source capture manifest is invalid.', error); }
  if (manifestSha256 !== digest(expectedSha256, 'captureManifestSha256')) {
    sourceError('reference-source-changed', 'The source capture manifest does not match its digest.');
  }
  const files = new Map(); const namespaces = new Set(); let fileCount = 0; let aggregateBytes = 0;
  for (const namespace of manifest.namespaces) {
    closed(namespace, ['namespaceId', 'files'], [], 'source capture namespace');
    captureNamespaceId(namespace.namespaceId);
    const foldedNamespaceId = namespace.namespaceId.toLowerCase();
    if (namespaces.has(foldedNamespaceId)) sourceError('reference-dependency-report-invalid', 'The source capture repeats a namespace.');
    namespaces.add(foldedNamespaceId);
    if (!Array.isArray(namespace.files) || namespace.files.length > 100_000) {
      sourceError('reference-dependency-report-invalid', 'Capture files must be a bounded array.');
    }
    const foldedPaths = new Set();
    for (const file of namespace.files) {
      closed(file, ['path', 'bytes', 'sha256'], [], 'source capture file');
      portableCapturePath(file.path);
      if (!Number.isSafeInteger(file.bytes) || file.bytes < 0 || file.bytes > 4 * 1024 * 1024 * 1024) {
        sourceError('reference-dependency-report-invalid', 'A source capture file receipt is invalid.');
      }
      digest(file.sha256, 'source file sha256');
      fileCount += 1; aggregateBytes += file.bytes;
      if (fileCount > 100_000 || aggregateBytes > 16 * 1024 * 1024 * 1024) {
        sourceError('reference-dependency-report-invalid', 'The source capture exceeds its aggregate limits.');
      }
      const foldedPath = file.path.toLowerCase();
      if (foldedPaths.has(foldedPath)) sourceError('reference-dependency-report-invalid', 'The source capture contains colliding paths.');
      foldedPaths.add(foldedPath);
      const key = fileKey(namespace.namespaceId, file.path);
      if (files.has(key)) sourceError('reference-dependency-report-invalid', 'The source capture repeats a file.');
      files.set(key, { namespaceId: namespace.namespaceId, path: file.path, bytes: file.bytes, sha256: file.sha256 });
    }
  }
  return files;
}

export function validateDependencyPolicy(policy, capabilityId, providerFingerprintSha256) {
  closed(policy, ['schemaVersion', 'policyId', 'capabilityId', 'providerFingerprintSha256', 'roles'], [],
    'dependency policy', 'reference-dependency-policy-invalid');
  if (policy.schemaVersion !== POLICY_SCHEMA || policy.capabilityId !== capabilityId
      || policy.providerFingerprintSha256 !== providerFingerprintSha256 || !Array.isArray(policy.roles)
      || policy.roles.length < 1 || policy.roles.length > 10_000) {
    sourceError('reference-dependency-policy-invalid', 'The dependency policy does not match the selected provider.');
  }
  opaque(policy.policyId, 'policyId', 'reference-dependency-policy-invalid');
  try { assertSha256(policy.providerFingerprintSha256, 'providerFingerprintSha256'); }
  catch (error) { sourceError('reference-dependency-policy-invalid', 'The providerFingerprintSha256 is invalid.', error); }
  const roles = new Map();
  for (const entry of policy.roles) {
    closed(entry, ['role', 'classification', 'affectedDomains'], [], 'dependency policy role',
      'reference-dependency-policy-invalid');
    opaque(entry.role, 'role', 'reference-dependency-policy-invalid');
    if (!['mandatory', 'optional', 'degraded'].includes(entry.classification)
        || !Array.isArray(entry.affectedDomains)
        || entry.affectedDomains.length > 64
        || entry.affectedDomains.some((domain) => typeof domain !== 'string' || !OPAQUE_ID.test(domain))
        || (entry.classification === 'degraded') !== (entry.affectedDomains.length > 0)
        || roles.has(entry.role)) {
      sourceError('reference-dependency-policy-invalid', 'The dependency policy contains an invalid role.');
    }
    roles.set(entry.role, { classification: entry.classification, affectedDomains: [...entry.affectedDomains].sort() });
  }
  return { roles, sha256: sha256(canonicalJsonBytes(policy)) };
}

function validateEvidence(values, label) {
  if (!Array.isArray(values) || values.length > 10_000) sourceError('reference-dependency-report-invalid', `${label} must be a bounded array.`);
  return values.map((entry) => {
    closed(entry, ['kind', 'subject', 'evidence'], [], label);
    return { kind: opaque(entry.kind, 'evidence kind'), subject: bounded(entry.subject, 'evidence subject'), evidence: bounded(entry.evidence, 'evidence') };
  }).sort((left, right) => canonicalJsonBytes(left).compare(canonicalJsonBytes(right)));
}

export function buildEffectiveSource(options) {
  const {
    captureManifest, captureManifestSha256, dependencyReport, policy, formatId, capabilityId,
    providerFingerprintSha256, providerPackageManifestSha256, degradedMode = 'refuse',
  } = options ?? {};
  opaque(formatId, 'formatId'); opaque(capabilityId, 'capabilityId');
  digest(providerFingerprintSha256, 'providerFingerprintSha256');
  digest(providerPackageManifestSha256, 'providerPackageManifestSha256');
  const captured = validateCapture(captureManifest, captureManifestSha256);
  const admittedPolicy = validateDependencyPolicy(policy, capabilityId, providerFingerprintSha256);

  closed(dependencyReport, [
    'schemaVersion', 'protocolVersion', 'capabilityId', 'captureManifestSha256', 'primary',
    'files', 'absent', 'unsupportedExternal', 'authentication', 'crossFileEvidence',
  ], [], 'provider dependency report');
  if (dependencyReport.schemaVersion !== REPORT_SCHEMA || dependencyReport.protocolVersion !== '3'
      || dependencyReport.capabilityId !== capabilityId || dependencyReport.captureManifestSha256 !== captureManifestSha256
      || !Array.isArray(dependencyReport.files) || !Array.isArray(dependencyReport.absent)
      || !Array.isArray(dependencyReport.unsupportedExternal) || dependencyReport.files.length > 100_000
      || dependencyReport.absent.length > 10_000) {
    sourceError('reference-dependency-report-invalid', 'The provider dependency report does not match the request.');
  }
  if (dependencyReport.unsupportedExternal.length) {
    sourceError('reference-external-dependency-unsupported', 'The model contains an unsupported external dependency.');
  }

  const reported = new Set(); const consumed = []; const consumedRoles = new Set();
  for (const file of dependencyReport.files) {
    closed(file, ['namespaceId', 'path', 'role', 'disposition'], [], 'provider dependency file');
    opaque(file.namespaceId, 'namespaceId');
    portableCapturePath(file.path);
    if (file.disposition !== 'consumed' && file.disposition !== 'ignored') {
      sourceError('reference-dependency-report-invalid', 'A dependency file has an invalid disposition.');
    }
    const key = fileKey(file.namespaceId, file.path);
    const receipt = captured.get(key);
    if (!receipt || reported.has(key)) sourceError('reference-dependency-report-invalid', 'The dependency report does not reconcile with the capture.');
    reported.add(key);
    if (file.disposition === 'ignored') {
      if (file.role !== null) sourceError('reference-dependency-report-invalid', 'Ignored files cannot claim a dependency role.');
    } else {
      opaque(file.role, 'role');
      if (!admittedPolicy.roles.has(file.role)) sourceError('reference-dependency-unknown', 'The provider reported a role outside the dependency policy.');
      consumed.push({ ...receipt, role: file.role });
      consumedRoles.add(file.role);
    }
  }
  if (reported.size !== captured.size) sourceError('reference-dependency-report-invalid', 'The dependency report did not classify every captured file.');

  closed(dependencyReport.primary, ['namespaceId', 'path', 'role'], [], 'primary dependency');
  opaque(dependencyReport.primary.namespaceId, 'primary namespaceId');
  portableCapturePath(dependencyReport.primary.path);
  opaque(dependencyReport.primary.role, 'primary role');
  const primaryKey = fileKey(dependencyReport.primary.namespaceId, dependencyReport.primary.path);
  const primaryMatches = consumed.filter((file) => fileKey(file.namespaceId, file.path) === primaryKey
    && file.role === dependencyReport.primary.role);
  if (primaryMatches.length !== 1) sourceError('reference-primary-source-invalid', 'The provider did not identify exactly one consumed primary source.');

  const absent = []; const absentRoles = new Set(); let completeness = 'complete';
  for (const entry of dependencyReport.absent) {
    closed(entry, ['role'], [], 'absent dependency');
    opaque(entry.role, 'role');
    const classification = admittedPolicy.roles.get(entry.role);
    if (!classification) sourceError('reference-dependency-unknown', 'The provider reported an absence outside the dependency policy.');
    if (absentRoles.has(entry.role)) sourceError('reference-dependency-report-invalid', 'The dependency report repeats an absent role.');
    absentRoles.add(entry.role);
    if (consumedRoles.has(entry.role)) sourceError('reference-dependency-report-invalid', 'A dependency role cannot be present and absent.');
    if (classification.classification === 'mandatory') sourceError('reference-dependency-missing', 'A mandatory model dependency is missing.');
    if (classification.classification === 'degraded') {
      if (degradedMode !== 'allow') sourceError('reference-degraded-conversion-refused', 'A degraded model conversion requires explicit allowance.');
      completeness = 'degraded';
    }
    absent.push({ role: entry.role, ...classification });
  }
  const observedRoles = new Set([...consumedRoles, ...absentRoles]);
  if (observedRoles.size !== admittedPolicy.roles.size
      || [...admittedPolicy.roles.keys()].some((role) => !observedRoles.has(role))) {
    sourceError('reference-dependency-report-invalid', 'The dependency report omitted a policy role.');
  }

  const effectiveSource = {
    schemaVersion: EFFECTIVE_SCHEMA, formatId, protocolVersion: '3', capabilityId,
    providerFingerprintSha256, providerPackageManifestSha256,
    discoveryPolicy: { policyId: policy.policyId, sha256: admittedPolicy.sha256 },
    completeness,
    primary: { namespaceId: dependencyReport.primary.namespaceId, path: dependencyReport.primary.path, role: dependencyReport.primary.role },
    consumed: consumed.sort((left, right) => canonicalJsonBytes(left).compare(canonicalJsonBytes(right))),
    absent: absent.sort((left, right) => left.role.localeCompare(right.role, 'en')),
    unsupportedExternal: [],
    authentication: validateEvidence(dependencyReport.authentication, 'content authentication evidence'),
    crossFileEvidence: validateEvidence(dependencyReport.crossFileEvidence, 'cross-file evidence'),
  };
  const bytes = canonicalJsonBytes(effectiveSource);
  return { effectiveSource, bytes, sha256: sha256(bytes), dependencyPolicySha256: admittedPolicy.sha256 };
}
