#!/usr/bin/env node
// canonicalizeProviderOutput throughput benchmark (aware-aeco/aware#679).
//
// A cold Tekla/Revit reference read pays `canonicalizeProviderOutput` once per model. On a
// 11,746-entity / ~86k-record provider output with one 74 MB GLB tile it took ~106 s. This script
// synthesises a provider output of that shape (deterministically — the same bytes every time, so a
// run on two commits is directly comparable), canonicalizes it, and prints
//
//   * wall-clock for the geometry stage (GLB validate + hash) and for the whole call,
//   * a SHA-256 for every canonical artifact object, index, and the signed-root preimage, and a
//     single combined digest of all of them.
//
// Two runs on two commits that print the same combined digest produced byte-identical artifacts.
//
// Usage:
//   node bench-canonicalize.mjs                       # ~11.7k entities, 74k properties, 74 MB GLB
//   node bench-canonicalize.mjs --entities 2000 --properties 12000 --glb-mb 8
//   node bench-canonicalize.mjs --dir C:/tmp/bench679 --reuse  # keep + reuse the generated fixture
//   node --cpu-prof bench-canonicalize.mjs             # profile the same run
//   node bench-canonicalize.mjs --json                 # machine-readable result on stdout

import { createHash } from 'node:crypto';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { performance } from 'node:perf_hooks';
import { pathToFileURL } from 'node:url';

import { canonicalJsonBytes, sha256 } from './model-contract.mjs';
import { canonicalizeProviderOutput } from './model-canonical-v2.mjs';
import { validateGlbTile } from './model-glb-tile.mjs';

function option(name, fallback) {
  const at = process.argv.indexOf(`--${name}`);
  return at < 0 ? fallback : process.argv[at + 1];
}

// Deterministic generator: the benchmark must hand every commit the identical input bytes.
function lcg(seed) {
  let state = seed >>> 0;
  return () => {
    state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
    return state / 0x100000000;
  };
}

export function syntheticGlb(megabytes, bound = 100) {
  const points = Math.max(2, Math.floor((megabytes * 1024 * 1024) / 12));
  const binary = Buffer.alloc(points * 12);
  const random = lcg(0x679);
  for (let at = 0; at < points * 3; at += 1) binary.writeFloatLE(random() * bound, at * 4);
  // Pin the extremes so the derived bounds are exactly [0,0,0,bound,bound,bound].
  [0, 0, 0, bound, bound, bound].forEach((value, at) => binary.writeFloatLE(value, at * 4));
  const json = Buffer.from(JSON.stringify({
    asset: { version: '2.0' }, buffers: [{ byteLength: binary.length }],
    scene: 0, scenes: [{ nodes: [0] }],
    bufferViews: [{ buffer: 0, byteLength: binary.length }],
    accessors: [{ bufferView: 0, componentType: 5126, count: points, type: 'VEC3' }],
    meshes: [{ primitives: [{ attributes: { POSITION: 0 } }] }], nodes: [{ mesh: 0 }],
  }));
  const paddedJson = Buffer.concat([json, Buffer.alloc((4 - json.length % 4) % 4, 0x20)]);
  const total = 12 + 8 + paddedJson.length + 8 + binary.length;
  const header = Buffer.alloc(12);
  header.writeUInt32LE(0x46546c67, 0); header.writeUInt32LE(2, 4); header.writeUInt32LE(total, 8);
  const jsonHeader = Buffer.alloc(8);
  jsonHeader.writeUInt32LE(paddedJson.length, 0); jsonHeader.writeUInt32LE(0x4e4f534a, 4);
  const binaryHeader = Buffer.alloc(8);
  binaryHeader.writeUInt32LE(binary.length, 0); binaryHeader.writeUInt32LE(0x004e4942, 4);
  return Buffer.concat([header, jsonHeader, paddedJson, binaryHeader, binary]);
}

const PROPERTY_NAMES = [
  'Mark', 'Profile', 'Material', 'Grade', 'Weight', 'Length', 'Finish', 'Phase',
  'Assembly Position', 'Part Position', 'Class', 'Numbering Series', 'Comment', 'Is Galvanised',
];

function* propertyRecords(entities, count, random) {
  for (let index = 0; index < count; index += 1) {
    const entity = Math.floor(random() * entities);
    const name = PROPERTY_NAMES[index % PROPERTY_NAMES.length];
    const kind = index % 4;
    const value = kind === 0 ? `S${Math.floor(random() * 1e6)}-\u00e9\u4e2d`
      : kind === 1 ? Math.round(random() * 1e6) / 100
        : kind === 2 ? random() < 0.5 : null;
    yield {
      id: `property:${index}`, entityId: `entity:${entity}`, name,
      normalizedName: name.normalize('NFKC').toLocaleLowerCase('en-US'), value,
      valueType: value === null ? 'null' : typeof value, unit: kind === 1 ? 'mm' : null,
      source: 'provider', status: 'readable', provenance: 'model',
    };
  }
}

export async function writeFixture(directory, shape, reuse = false) {
  const outputRoot = path.join(directory, 'output');
  const markerPath = path.join(directory, 'fixture.json');
  if (reuse) {
    try {
      const marker = JSON.parse(await fs.readFile(markerPath, 'utf8'));
      if (JSON.stringify(marker.shape) === JSON.stringify(shape)) return { root: outputRoot, files: marker.files };
    } catch { /* no usable fixture: generate one */ }
  }
  await fs.rm(outputRoot, { recursive: true, force: true });
  await fs.mkdir(path.join(outputRoot, 'geometry'), { recursive: true });
  await fs.mkdir(path.join(outputRoot, 'metadata'), { recursive: true });
  const files = [];
  const tile = syntheticGlb(shape.glbMb);
  const tilePath = 'geometry/000000.glb';
  await fs.writeFile(path.join(outputRoot, tilePath), tile);
  files.push({ path: tilePath, kind: 'geometry', ordinal: 0, mediaType: 'model/gltf-binary',
    bytes: tile.length, sha256: sha256(tile), count: 1, bounds: [0, 0, 0, 100, 100, 100] });
  const random = lcg(0x1679);
  const families = {
    entities: function* entities() {
      for (let index = shape.entities - 1; index >= 0; index -= 1) {
        yield { id: `entity:${index}`, type: 'member', name: index % 7 ? `Beam ${index}` : null,
          geometry: [{ tileOrdinal: 0, bounds: [0, 0, 0, 1 + (index % 50), 1, 1] }] };
      }
    },
    properties: () => propertyRecords(shape.entities, shape.properties, random),
    relationships: function* relationships() {
      for (let index = 1; index < shape.relationships; index += 1) {
        yield { id: `relation:${index}`, kind: 'contains',
          from: `entity:${index % shape.entities}`, to: `entity:${(index * 7 + 1) % shape.entities}` };
      }
    },
  };
  for (const [kind, generate] of Object.entries(families)) {
    const lines = [];
    let count = 0;
    for (const record of generate()) { lines.push(canonicalJsonBytes(record).toString('utf8')); count += 1; }
    const bytes = Buffer.from(`${lines.join('\n')}\n`);
    const relative = `metadata/${kind}-000000.jsonl`;
    await fs.writeFile(path.join(outputRoot, relative), bytes);
    files.push({ path: relative, kind, ordinal: 0, mediaType: 'application/x-ndjson',
      bytes: bytes.length, sha256: sha256(bytes), count });
  }
  await fs.writeFile(markerPath, JSON.stringify({ shape, files }));
  return { root: outputRoot, files };
}

function effectiveSource() {
  return {
    schemaVersion: 'model-effective-source/v2', formatId: 'format.synthetic', protocolVersion: '3',
    capabilityId: 'capability.synthetic', providerFingerprintSha256: sha256(Buffer.from('provider')),
    providerPackageManifestSha256: sha256(Buffer.from('package')),
    discoveryPolicy: { policyId: 'policy.synthetic', sha256: sha256(Buffer.from('policy')) },
    completeness: 'complete', primary: { namespaceId: 'model', path: 'model.db', role: 'primary' },
    consumed: [], absent: [], unsupportedExternal: [], authentication: [], crossFileEvidence: [],
  };
}

export async function digestCanonical(result) {
  const digests = {};
  for (const object of result.objects) {
    digests[object.receipt.logicalPath] = sha256(await fs.readFile(object.pathname));
  }
  for (const index of Object.values(result.indexes)) digests[index.receipt.logicalPath] = sha256(index.bytes);
  digests['model-reference-manifest.json'] = result.root.sha256;
  const combined = createHash('sha256');
  for (const name of Object.keys(digests).sort()) combined.update(`${name}\0${digests[name]}\n`);
  return { files: digests, combined: combined.digest('hex') };
}

export async function runBenchmark(shape, directory, reuse = false) {
  const fixtureStart = performance.now();
  const output = await writeFixture(directory, shape, reuse);
  const fixtureMs = performance.now() - fixtureStart;
  const workRoot = path.join(directory, 'work');
  await fs.rm(workRoot, { recursive: true, force: true });
  await fs.mkdir(workRoot, { recursive: true });
  const geometry = output.files.find((entry) => entry.kind === 'geometry');
  const tileBytes = await fs.readFile(path.join(output.root, geometry.path));
  const geometryStart = performance.now();
  validateGlbTile(tileBytes, {});
  sha256(tileBytes);
  const geometryMs = performance.now() - geometryStart;
  const cpuStart = process.cpuUsage();
  const start = performance.now();
  const result = await canonicalizeProviderOutput({
    workRoot, output, effectiveSource: effectiveSource(), formatId: 'format.synthetic',
    capabilityId: 'capability.synthetic',
    providerPackageManifestSha256: sha256(Buffer.from('package')),
    conversionRequestSha256: sha256(Buffer.from('request')),
  });
  const canonicalizeMs = performance.now() - start;
  const cpu = process.cpuUsage(cpuStart);
  const digest = await digestCanonical(result);
  return {
    shape, fixtureBytes: output.files.reduce((sum, entry) => sum + entry.bytes, 0),
    geometryStageMs: Math.round(geometryMs), canonicalizeMs: Math.round(canonicalizeMs),
    cpuMs: Math.round((cpu.user + cpu.system) / 1000),
    metadataStageMs: Math.round(canonicalizeMs - geometryMs), digest,
  };
}

if (import.meta.url === pathToFileURL(process.argv[1] ?? '').href) {
  const shape = {
    entities: Number(option('entities', 11746)), properties: Number(option('properties', 74000)),
    relationships: Number(option('relationships', 2000)), glbMb: Number(option('glb-mb', 74)),
  };
  const given = option('dir');
  const directory = given ?? await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-bench-679-'));
  try {
    const report = await runBenchmark(shape, directory, process.argv.includes('--reuse'));
    if (process.argv.includes('--json')) {
      process.stdout.write(`${JSON.stringify(report)}\n`);
    } else {
      process.stdout.write(`fixture: ${shape.entities} entities, ${shape.properties} properties, `
        + `${shape.relationships} relationships, ${shape.glbMb} MB GLB `
        + `(${(report.fixtureBytes / 1048576).toFixed(1)} MiB)\n`
        + `geometry stage (GLB validate + hash): ${report.geometryStageMs} ms\n`
        + `canonicalizeProviderOutput total:     ${report.canonicalizeMs} ms wall, ${report.cpuMs} ms CPU\n`
        + `combined artifact digest: ${report.digest.combined}\n`);
    }
  } finally {
    if (!given) await fs.rm(directory, { recursive: true, force: true });
  }
}
