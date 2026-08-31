#!/usr/bin/env node
// Fail-closed entrypoint for the private Windows reproducibility proof. Real machine paths live only
// in the per-builder locator; the canonical manifest and receipt contain logical IDs and digests.
import { createHash } from 'node:crypto';
import { existsSync, readFileSync, realpathSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { buildConnectionReader, canonicalJson, READER_BUILD_SETTINGS, sha256File } from './build.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const SHA256 = /^[0-9a-f]{64}$/;
const POISONED_EXACT = new Set([
  'RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'CC', 'CFLAGS', 'CL', 'LINK', 'LIB', 'INCLUDE',
  'NODE_OPTIONS', 'ESBUILD_BINARY_PATH', 'GOOGLE_CLIENT_SECRET', 'AWARE_GOOGLE_CLIENT_SECRET',
]);
const POISONED_PREFIXES = ['npm_config_', 'DOTNET_', 'COREHOST_'];

const digestText = (text) => createHash('sha256').update(text).digest('hex');
const readJson = (path) => JSON.parse(readFileSync(path, 'utf8'));

export function rejectedAmbientKeys(env) {
  return Object.keys(env).filter((key) => POISONED_EXACT.has(key)
    || POISONED_PREFIXES.some((prefix) => key.toLowerCase().startsWith(prefix.toLowerCase())))
    .sort();
}

function verifyRecord(id, record, locator) {
  if (!record || record.id !== id || !SHA256.test(record.sha256 ?? '')) throw new Error(`invalid tool record ${id}`);
  const path = locator?.tools?.[id];
  if (typeof path !== 'string' || !existsSync(path)) throw new Error(`missing local path for tool ${id}`);
  const actual = sha256File(path);
  if (actual !== record.sha256) throw new Error(`tool digest mismatch for ${id}: ${actual}`);
  return resolve(path);
}

export function verifyInternalInputs({ manifest, locator, env = process.env }) {
  if (manifest?.schema !== 'aware-windows-repro-builder/v1') throw new Error('unsupported builder manifest schema');
  if (locator?.schema !== 'aware-windows-repro-locator/v1') throw new Error('unsupported builder locator schema');
  const poison = rejectedAmbientKeys(env);
  if (poison.length) throw new Error(`ambient build authority is forbidden: ${poison.join(', ')}`);
  if (manifest.platform !== 'win32' || manifest.arch !== 'x64' || manifest.nodeVersion !== '24.14.0') {
    throw new Error('builder manifest must pin Windows x64 and Node 24.14.0');
  }
  if (canonicalJson(manifest.settings) !== canonicalJson(READER_BUILD_SETTINGS)) {
    throw new Error('reader build settings differ from the closed implementation');
  }
  if (process.platform !== 'win32' || process.arch !== 'x64' || process.versions.node !== manifest.nodeVersion) {
    throw new Error(`running Node must be exactly ${manifest.nodeVersion} on Windows x64`);
  }
  const paths = Object.fromEntries(['node', 'postject', 'web-ifc-wasm'].map((id) => [id,
    verifyRecord(id, manifest.tools?.[id], locator),
  ]));
  if (realpathSync(paths.node) !== realpathSync(process.execPath)) throw new Error('the verified Node is not the running Node');
  for (const [name, path] of Object.entries({
    'reader-package-lock': join(here, 'package-lock.json'),
    'aware-cargo-lock': join(here, '..', 'cli', 'Cargo.lock'),
  })) {
    const expected = manifest.inputs?.[name];
    if (!SHA256.test(expected ?? '') || sha256File(path) !== expected) throw new Error(`input digest mismatch for ${name}`);
  }
  if (!SHA256.test(manifest.source?.bundleSha256 ?? '') || !/^[0-9a-f]{40}$/.test(manifest.source?.commit ?? '')
    || !/^[0-9a-f]{40}$/.test(manifest.source?.tree ?? '')) throw new Error('invalid source identity in builder manifest');
  return paths;
}

function parseArgs(argv) {
  const out = {};
  for (let i = 0; i < argv.length; i += 2) {
    if (!argv[i]?.startsWith('--') || argv[i + 1] == null) throw new Error('arguments are --manifest FILE --locator FILE --output DIR');
    out[argv[i].slice(2)] = argv[i + 1];
  }
  if (!out.manifest || !out.locator || !out.output) throw new Error('arguments are --manifest FILE --locator FILE --output DIR');
  return out;
}

export async function runInternalReaderBuild({ manifestPath, locatorPath, outputDir, env = process.env }) {
  const manifestText = canonicalJson(readJson(manifestPath));
  const manifest = JSON.parse(manifestText);
  const locator = readJson(locatorPath);
  const paths = verifyInternalInputs({ manifest, locator, env });
  const receipt = {
    schema: 'aware-connection-reader-build-receipt/v1',
    buildId: digestText(manifestText),
    builderManifestSha256: digestText(manifestText),
    source: manifest.source,
    inputs: manifest.inputs,
    settings: manifest.settings,
    tools: Object.fromEntries(Object.entries(manifest.tools).map(([id, record]) => [id, {
      id: record.id, sha256: record.sha256,
    }])),
    commands: {
      bundle: '<esbuild> model-dispatcher.mjs -> <output>/bundle.cjs',
      sea: '<node> --experimental-sea-config sea-config.json [cwd=<output>]',
      inject: '<node> <postject> aware-connection-reader.exe NODE_SEA_BLOB sea-prep.blob [cwd=<output>]',
    },
    rootTokens: ['<source>', '<output>', '<toolchain>', '<cache>', '<profile>'],
  };
  return buildConnectionReader({
    outputDir, nodePath: paths.node, postjectPath: paths.postject,
    wasmPath: paths['web-ifc-wasm'], receipt, verifiedExternalTools: true,
  });
}

if (realpathSync(fileURLToPath(import.meta.url)) === realpathSync(process.argv[1])) {
  const args = parseArgs(process.argv.slice(2));
  await runInternalReaderBuild({ manifestPath: args.manifest, locatorPath: args.locator, outputDir: args.output });
}
