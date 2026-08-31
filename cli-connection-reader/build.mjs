// build.mjs — package aware-connection-reader as a path-independent Windows SEA.
//
// The ordinary developer entrypoint deliberately stays usable by the public release workflow. The
// stricter internal reproducibility boundary lives in build-internal-repro.mjs and supplies the
// verified tool/input identities plus the canonical receipt written by this module.
import { build } from 'esbuild';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import {
  copyFileSync, mkdirSync, readFileSync, realpathSync, rmSync, writeFileSync,
} from 'node:fs';
import { basename, dirname, isAbsolute, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { READER_BUILD_SETTINGS } from './repro-settings.mjs';

export { READER_BUILD_SETTINGS } from './repro-settings.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const EXE_NAME = 'aware-connection-reader.exe';

export const sha256File = (path) => createHash('sha256').update(readFileSync(path)).digest('hex');

function canonical(value) {
  if (Array.isArray(value)) return value.map(canonical);
  if (!value || typeof value !== 'object') return value;
  return Object.fromEntries(Object.keys(value).sort().map((key) => [key, canonical(value[key])]));
}

export function canonicalJson(value) {
  return `${JSON.stringify(canonical(value), null, 2)}\n`;
}

function under(root, path) {
  const rel = relative(resolve(root), resolve(path));
  return rel === '' || (!rel.startsWith('..') && !isAbsolute(rel));
}

export async function buildConnectionReader(options = {}) {
  const outputDir = resolve(options.outputDir ?? join(here, 'dist'));
  const nodePath = resolve(options.nodePath ?? process.execPath);
  const postjectPath = resolve(options.postjectPath ?? join(here, 'node_modules', 'postject', 'dist', 'cli.js'));
  const wasmPath = resolve(options.wasmPath ?? join(here, 'node_modules', 'web-ifc', 'web-ifc-node.wasm'));
  const receipt = options.receipt ?? null;

  const outputOwnsSource = relative(outputDir, here);
  if (outputOwnsSource === '' || (!outputOwnsSource.startsWith('..') && !isAbsolute(outputOwnsSource))) {
    throw new Error('SEA output root must not be the reader source root or one of its ancestors');
  }
  if (process.platform !== 'win32' || process.arch !== 'x64') {
    throw new Error('aware-connection-reader SEA build requires Windows x64');
  }
  if (basename(nodePath).toLowerCase() !== 'node.exe') throw new Error('SEA base runtime must be node.exe');
  for (const path of [postjectPath, wasmPath]) {
    if (!options.verifiedExternalTools && !under(here, path)) {
      throw new Error(`reader dependency escaped the package root without verified external-tool authority: ${path}`);
    }
  }

  rmSync(outputDir, { recursive: true, force: true });
  mkdirSync(outputDir, { recursive: true });

  console.log('[build] bundling with esbuild…');
  await build({
    absWorkingDir: here,
    entryPoints: ['model-dispatcher.mjs'],
    bundle: true,
    platform: READER_BUILD_SETTINGS.bundle.platform,
    format: READER_BUILD_SETTINGS.bundle.format,
    target: READER_BUILD_SETTINGS.bundle.target,
    outfile: join(outputDir, 'bundle.cjs'),
  });

  // Relative names plus outputDir as cwd are intentional. Absolute roots in this file are embedded
  // into the SEA blob by Node even though the bundled JavaScript itself is byte-identical.
  console.log('[build] generating SEA blob…');
  writeFileSync(join(outputDir, 'sea-config.json'), canonicalJson({
    disableExperimentalSEAWarning: READER_BUILD_SETTINGS.sea.disableExperimentalWarning,
    main: 'bundle.cjs',
    output: 'sea-prep.blob',
  }), 'utf8');
  execFileSync(nodePath, ['--experimental-sea-config', 'sea-config.json'], {
    cwd: outputDir, stdio: 'inherit', windowsHide: true,
  });

  console.log('[build] injecting blob into exe…');
  const exe = join(outputDir, EXE_NAME);
  copyFileSync(nodePath, exe);
  execFileSync(nodePath, [
    postjectPath, EXE_NAME, READER_BUILD_SETTINGS.sea.section, 'sea-prep.blob',
    '--sentinel-fuse', READER_BUILD_SETTINGS.sea.sentinelFuse,
  ], { cwd: outputDir, stdio: 'inherit', windowsHide: true });
  copyFileSync(wasmPath, join(outputDir, 'web-ifc-node.wasm'));

  const outputs = Object.fromEntries([
    'bundle.cjs', 'sea-prep.blob', EXE_NAME, 'web-ifc-node.wasm',
  ].map((name) => [name, { sha256: sha256File(join(outputDir, name)), size: readFileSync(join(outputDir, name)).length }]));
  if (receipt) writeFileSync(join(outputDir, 'build-receipt.json'), canonicalJson({ ...receipt, outputs }), 'utf8');
  console.log(`[build] done → ${exe} (+ web-ifc-node.wasm)`);
  return { outputDir, outputs, receiptPath: receipt ? join(outputDir, 'build-receipt.json') : null };
}

function isEntryModule() {
  try { return realpathSync(fileURLToPath(import.meta.url)) === realpathSync(process.argv[1]); }
  catch { return fileURLToPath(import.meta.url) === resolve(process.argv[1] || ''); }
}

if (isEntryModule()) {
  if (process.argv.length !== 2) throw new Error('build.mjs accepts no arguments; use build-internal-repro.mjs for the controlled build');
  await buildConnectionReader();
}
