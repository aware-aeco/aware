#!/usr/bin/env node
// Turns local tool and closure paths into a path-free reproducible-builder manifest plus a
// local-only locator. The manifest digest is the immutable identity of the Windows build authority.
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, lstatSync, readFileSync, realpathSync, writeFileSync } from 'node:fs';
import { isAbsolute, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { READER_BUILD_SETTINGS } from '../cli-connection-reader/repro-settings.mjs';
import { CLOSURE_IDS, COMPILER_DESCRIPTOR, INPUT_IDS, NONCOMPILER_TOOL_IDS, exactKeys,
  inventory as compilerInventory, validateCompilerManifest, validateCompilerLocator, validateWindowsPath } from './windows-compiler-closure.mjs';

const SCRIPT = fileURLToPath(import.meta.url);
const SHA1 = /^[0-9a-f]{40}$/;
const TOOL_IDS = NONCOMPILER_TOOL_IDS;
const sha256 = (path) => createHash('sha256').update(readFileSync(path)).digest('hex');
const canonical = (value) => Array.isArray(value) ? value.map(canonical)
  : value && typeof value === 'object'
    ? Object.fromEntries(Object.keys(value).sort().map((key) => [key, canonical(value[key])])) : value;
export const canonicalJson = (value) => `${JSON.stringify(canonical(value), null, 2)}\n`;

function requireFile(path, label) {
  if (typeof path !== 'string' || !isAbsolute(path) || !existsSync(path) || !lstatSync(path).isFile()) {
    throw new Error(`${label} must be an absolute existing file`);
  }
  return resolve(path);
}

function requireDirectory(path, label) {
  if (typeof path !== 'string' || !isAbsolute(path) || !existsSync(path) || !lstatSync(path).isDirectory()) {
    throw new Error(`${label} must be an absolute existing directory: ${path}`);
  }
  return resolve(path);
}

export const inventory = compilerInventory;

export function createWindowsBuilderRecords(input) {
  if (input?.schema !== 'aware-windows-repro-builder-inputs/v1') throw new Error('unsupported builder-input schema');
  exactKeys(input, ['schema', 'source', 'inputs', 'tools', 'closures'], 'builder inputs');
  exactKeys(input.source, ['commit', 'tree', 'bundle'], 'builder input source');
  exactKeys(input.inputs, INPUT_IDS, 'builder script/lock inputs');
  exactKeys(input.tools, TOOL_IDS, 'builder noncompiler tools');
  exactKeys(input.closures, CLOSURE_IDS, 'builder compiler/dependency roots');
  // Physical Windows admission occurs before stat/hash calls. POSIX fixtures exercise
  // only portable manifest semantics; production builders require Windows.
  if (process.platform === 'win32') {
    for (const [id, path] of [...Object.entries(input.inputs), ...Object.entries(input.tools), ...Object.entries(input.closures), ['bundle', input.source.bundle]]) validateWindowsPath(path, id);
  }
  if (!SHA1.test(input.source?.commit ?? '') || !SHA1.test(input.source?.tree ?? '')) throw new Error('invalid source commit/tree');
  const sourceBundle = requireFile(input.source.bundle, 'source bundle');
  const locks = Object.fromEntries(INPUT_IDS.map(id => [id, requireFile(input.inputs[id], id)]));
  const tools = Object.fromEntries(TOOL_IDS.map((id) => [id, requireFile(input.tools?.[id], `${id} tool`)]));
  const closures = Object.fromEntries(CLOSURE_IDS.map(id => [id, requireDirectory(input.closures[id], id)]));
  const manifest = {
    schema: 'aware-windows-repro-builder/v1', platform: 'win32', arch: 'x64',
    nodeVersion: '24.14.0', rustVersion: '1.95.0', target: 'x86_64-pc-windows-msvc',
    source: { commit: input.source.commit, tree: input.source.tree, bundleSha256: sha256(sourceBundle) },
    settings: READER_BUILD_SETTINGS,
    compiler: COMPILER_DESCRIPTOR,
    inputs: Object.fromEntries(Object.entries(locks).map(([id, path]) => [id, sha256(path)])),
    tools: Object.fromEntries(TOOL_IDS.map((id) => [id, { id, sha256: sha256(tools[id]) }])),
    closures: Object.fromEntries(Object.entries(closures).map(([id, root]) => [id, { files: compilerInventory(root) }])),
  };
  validateCompilerManifest(manifest);
  const manifestText = canonicalJson(manifest);
  const buildId = createHash('sha256').update(manifestText).digest('hex');
  const locator = {
    schema: 'aware-windows-repro-locator/v1', sourceBundle, tools, closures,
  };
  if (process.platform === 'win32') validateCompilerLocator(locator);
  return { manifest, manifestText, locator, locatorText: canonicalJson(locator), buildId };
}

function parseArgs(argv) {
  const out = {};
  for (let index = 0; index < argv.length; index += 2) {
    if (!argv[index]?.startsWith('--') || argv[index + 1] == null) {
      throw new Error('arguments are --input FILE --manifest FILE --locator FILE');
    }
    out[argv[index].slice(2)] = argv[index + 1];
  }
  for (const key of ['input', 'manifest', 'locator']) if (!isAbsolute(out[key] ?? '')) throw new Error(`--${key} must be absolute`);
  return out;
}

function git(path, args) {
  const result = spawnSync(path, args, { encoding: 'utf8', windowsHide: true });
  if (result.error || result.status !== 0) throw new Error(`git ${args.join(' ')} failed: ${result.error?.message ?? result.stderr}`);
  return String(result.stdout);
}

if (realpathSync.native(SCRIPT).toLowerCase() === realpathSync.native(process.argv[1]).toLowerCase()) {
  const args = parseArgs(process.argv.slice(2));
  const input = JSON.parse(readFileSync(args.input, 'utf8'));
  const records = createWindowsBuilderRecords(input);
  const heads = git(records.locator.tools.git, ['bundle', 'list-heads', records.locator.sourceBundle]);
  if (!heads.split(/\r?\n/).some((line) => line.startsWith(`${records.manifest.source.commit} `))) {
    throw new Error('source bundle does not advertise the selected commit');
  }
  writeFileSync(args.manifest, records.manifestText, { encoding: 'utf8', flag: 'wx' });
  writeFileSync(args.locator, records.locatorText, { encoding: 'utf8', flag: 'wx' });
  console.log(`AWARE Windows builder manifest ${records.buildId}`);
}
