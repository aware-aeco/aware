#!/usr/bin/env node
// verifyProviderOutput (provider-output admission) throughput benchmark (aware-aeco/aware#681).
//
// Reuses the deterministic #679 fixture (bench-canonicalize.mjs), wraps it in the manifest and
// completion marker a provider writes, runs admission, and prints the wall/CPU time and a SHA-256
// over every admitted file. Two runs on two commits printing the same `admitted digest` admitted
// byte-identical output.
//
// Usage:
//   node bench-admission.mjs                       # ~11.7k entities, 74k properties, 74 MB GLB
//   node bench-admission.mjs --dir C:/tmp/bench681 --reuse
//   node --cpu-prof bench-admission.mjs            # profile the same run
//   node bench-admission.mjs --json

import { createHash } from 'node:crypto';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { performance } from 'node:perf_hooks';
import { pathToFileURL } from 'node:url';

import { writeFixture } from './bench-canonicalize.mjs';
import { canonicalJsonBytes, sha256 } from './model-contract.mjs';
import { verifyProviderOutput } from './model-provider-output.mjs';

function option(name, fallback) {
  const at = process.argv.indexOf(`--${name}`);
  return at < 0 ? fallback : process.argv[at + 1];
}

export const REQUEST = {
  formatId: 'format.synthetic', capabilityId: 'capability.synthetic',
  providerPackageManifestSha256: sha256(Buffer.from('package')),
  effectiveSourceSha256: sha256(Buffer.from('source')),
  conversionRequestSha256: sha256(Buffer.from('request')),
};

// Writes intermediate-manifest.json + complete.json over `files` (receipts) under `root`.
export async function sealOutput(root, files) {
  const manifest = {
    schemaVersion: 'aware.model-provider-output-manifest/v1', protocolVersion: '3', ...REQUEST, files,
  };
  const manifestBytes = canonicalJsonBytes(manifest);
  await fs.writeFile(path.join(root, 'intermediate-manifest.json'), manifestBytes);
  await fs.writeFile(path.join(root, 'complete.json'), canonicalJsonBytes({
    schemaVersion: 'aware.model-provider-output-completion/v1', manifestPath: 'intermediate-manifest.json',
    manifestBytes: manifestBytes.length, manifestSha256: sha256(manifestBytes),
  }));
  return manifest;
}

export async function digestTree(root) {
  const names = [];
  for (const dir of ['', 'geometry', 'metadata']) {
    for (const entry of await fs.readdir(path.join(root, dir), { withFileTypes: true })) {
      if (entry.isFile()) names.push(dir ? `${dir}/${entry.name}` : entry.name);
    }
  }
  const combined = createHash('sha256'); const files = {};
  for (const name of names.sort()) {
    files[name] = sha256(await fs.readFile(path.join(root, ...name.split('/'))));
    combined.update(`${name}\0${files[name]}\n`);
  }
  return { files, combined: combined.digest('hex') };
}

export async function runAdmission(shape, directory, reuse = false) {
  const output = await writeFixture(directory, shape, reuse);
  await sealOutput(output.root, output.files);
  const admittedRoot = path.join(directory, 'admitted');
  await fs.rm(admittedRoot, { recursive: true, force: true });
  const cpuStart = process.cpuUsage(); const start = performance.now();
  await verifyProviderOutput(output.root, { ...REQUEST, admittedRoot });
  const admissionMs = performance.now() - start; const cpu = process.cpuUsage(cpuStart);
  return {
    shape, admissionMs: Math.round(admissionMs), cpuMs: Math.round((cpu.user + cpu.system) / 1000),
    digest: await digestTree(admittedRoot),
  };
}

if (import.meta.url === pathToFileURL(process.argv[1] ?? '').href) {
  const shape = {
    entities: Number(option('entities', 11746)), properties: Number(option('properties', 74000)),
    relationships: Number(option('relationships', 2000)), glbMb: Number(option('glb-mb', 74)),
  };
  const given = option('dir');
  const directory = given ?? await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-bench-681-'));
  try {
    const report = await runAdmission(shape, directory, process.argv.includes('--reuse'));
    if (process.argv.includes('--json')) process.stdout.write(`${JSON.stringify(report)}\n`);
    else {
      process.stdout.write(`admission: ${report.admissionMs} ms wall, ${report.cpuMs} ms CPU\n`
        + `admitted digest: ${report.digest.combined}\n`);
    }
  } finally {
    if (!given) await fs.rm(directory, { recursive: true, force: true });
  }
}
