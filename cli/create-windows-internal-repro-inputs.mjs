#!/usr/bin/env node
// Turns local tool and closure paths into a path-free reproducible-builder manifest plus a
// local-only locator. The manifest digest is the immutable identity of the Windows build authority.
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, lstatSync, readFileSync, readdirSync, realpathSync, writeFileSync } from 'node:fs';
import { isAbsolute, join, relative, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';
import { READER_BUILD_SETTINGS } from '../cli-connection-reader/repro-settings.mjs';

const SCRIPT = fileURLToPath(import.meta.url);
const SHA1 = /^[0-9a-f]{40}$/;
const TOOL_IDS = Object.freeze([
  'git', 'node', 'npm-cli', 'cargo', 'rustc', 'rustdoc', 'cl', 'link', 'lib', 'postject', 'web-ifc-wasm',
]);
const ENVIRONMENT_KEYS = Object.freeze(['PATH', 'INCLUDE', 'LIB', 'LIBPATH', 'SystemRoot', 'WINDIR', 'ComSpec', 'PATHEXT']);
const sha256 = (path) => createHash('sha256').update(readFileSync(path)).digest('hex');
const canonical = (value) => Array.isArray(value) ? value.map(canonical)
  : value && typeof value === 'object'
    ? Object.fromEntries(Object.keys(value).sort().map((key) => [key, canonical(value[key])])) : value;
export const canonicalJson = (value) => `${JSON.stringify(canonical(value), null, 2)}\n`;
const portable = (path) => path.split(sep).join('/');

function requireFile(path, label) {
  if (typeof path !== 'string' || !isAbsolute(path) || !existsSync(path) || !lstatSync(path).isFile()) {
    throw new Error(`${label} must be an absolute existing file`);
  }
  return resolve(path);
}

function requireDirectory(path, label) {
  if (typeof path !== 'string' || !isAbsolute(path) || !existsSync(path) || !lstatSync(path).isDirectory()) {
    throw new Error(`${label} must be an absolute existing directory`);
  }
  return resolve(path);
}

export function inventory(root) {
  const walk = (directory) => readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const path = join(directory, entry.name);
    if (entry.isSymbolicLink()) throw new Error(`offline closure contains symbolic link: ${path}`);
    if (entry.isDirectory()) return walk(path);
    if (!entry.isFile()) throw new Error(`offline closure contains unsupported entry: ${path}`);
    return [path];
  });
  return walk(root).map((path) => ({
    path: portable(relative(root, path)), size: lstatSync(path).size, sha256: sha256(path),
  })).sort((a, b) => Buffer.compare(Buffer.from(a.path), Buffer.from(b.path)));
}

export function createWindowsBuilderRecords(input) {
  if (input?.schema !== 'aware-windows-repro-builder-inputs/v1') throw new Error('unsupported builder-input schema');
  if (!SHA1.test(input.source?.commit ?? '') || !SHA1.test(input.source?.tree ?? '')) throw new Error('invalid source commit/tree');
  const sourceBundle = requireFile(input.source.bundle, 'source bundle');
  const locks = {
    'aware-cargo-lock': requireFile(input.inputs?.['aware-cargo-lock'], 'AWARE Cargo.lock'),
    'reader-package-lock': requireFile(input.inputs?.['reader-package-lock'], 'reader package-lock.json'),
    'builder-script': requireFile(input.inputs?.['builder-script'], 'running builder script'),
  };
  const tools = Object.fromEntries(TOOL_IDS.map((id) => [id, requireFile(input.tools?.[id], `${id} tool`)]));
  const closures = {
    'npm-cache': requireDirectory(input.closures?.['npm-cache'], 'npm-cache closure'),
    'cargo-home': requireDirectory(input.closures?.['cargo-home'], 'cargo-home closure'),
  };
  const environment = Object.fromEntries(ENVIRONMENT_KEYS.map((key) => {
    const value = input.environment?.[key];
    if (typeof value !== 'string' || value.length === 0) throw new Error(`environment is missing ${key}`);
    return [key, value];
  }));
  const manifest = {
    schema: 'aware-windows-repro-builder/v1', platform: 'win32', arch: 'x64',
    nodeVersion: '24.14.0', rustVersion: '1.95.0', target: 'x86_64-pc-windows-msvc',
    source: { commit: input.source.commit, tree: input.source.tree, bundleSha256: sha256(sourceBundle) },
    settings: READER_BUILD_SETTINGS,
    inputs: Object.fromEntries(Object.entries(locks).map(([id, path]) => [id, sha256(path)])),
    tools: Object.fromEntries(TOOL_IDS.map((id) => [id, { id, sha256: sha256(tools[id]) }])),
    closures: Object.fromEntries(Object.entries(closures).map(([id, root]) => [id, { files: inventory(root) }])),
  };
  const manifestText = canonicalJson(manifest);
  const buildId = createHash('sha256').update(manifestText).digest('hex');
  const locator = {
    schema: 'aware-windows-repro-locator/v1', sourceBundle, tools, closures, environment,
  };
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
