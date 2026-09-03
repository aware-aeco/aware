#!/usr/bin/env node
// Dedicated private Windows-x64 build boundary. It extracts an immutable Git bundle into a new root,
// admits only digest-bound tools/offline closures, and builds Rust plus the connection-reader without
// consulting a developer checkout, PATH tool, or mutable shared target directory.
import { execFileSync, spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import {
  copyFileSync, existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, realpathSync, rmSync,
  writeFileSync,
} from 'node:fs';
import { basename, dirname, join, relative, resolve, sep, win32 } from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptPath = fileURLToPath(import.meta.url);
const SHA256 = /^[0-9a-f]{64}$/;
const SHA1 = /^[0-9a-f]{40}$/;
const TARGET = 'x86_64-pc-windows-msvc';
export const COMMAND_OUTPUT_BUFFER_BYTES = 128 * 1024 * 1024;
export const SOURCE_PATHS = Object.freeze(['cli', 'cli-connection-reader']);
const LOGICAL_ROOTS = ['<work>', '<source>', '<cargo-home>', '<cargo-vendor>'];
export const WINDOWS_LOGICAL_RUST_FLAGS = ['-C', 'link-arg=/Brepro',
  ...LOGICAL_ROOTS.flatMap((root) => Array(2).fill(`--remap-path-prefix=${root}=${root}`)),
].join(' ');
export const LOGICAL_CARGO_COMMAND = '<cargo> build --manifest-path <source>/cli/Cargo.toml --release --locked --offline --config source.crates-io.replace-with="vendored-sources" --config source.vendored-sources.directory="<cargo-vendor>" --target x86_64-pc-windows-msvc --verbose --verbose';
const EXACT_POISON = new Set([
  'RUSTFLAGS', 'RUSTDOCFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'CC', 'AR', 'CFLAGS', 'CL', 'LINK', 'LIB', 'INCLUDE',
  'PATH', 'LIBPATH', '_CL_', 'RUSTC', 'RUSTDOC', 'VSLANG', 'VCINSTALLDIR', 'VCTOOLSINSTALLDIR', 'VSINSTALLDIR', 'SDKROOT',
  'NODE_OPTIONS', 'ESBUILD_BINARY_PATH', 'GOOGLE_CLIENT_SECRET', 'AWARE_GOOGLE_CLIENT_SECRET',
]);
const POISON_PREFIXES = ['npm_config_', 'DOTNET_', 'COREHOST_', 'COMPLUS_', 'CARGO_', 'RUSTUP_',
  'VSCMD_', 'WINDOWSSDK', 'SDK_', 'CC_', 'AR_', 'CFLAGS_', 'RUSTC_', 'RUSTDOC_'];
const CODE_INPUT_IDS = ['builder-script', 'compiler-closure-script', 'reader-settings-script', 'compiler-audit-script'];
export function runningInputFiles(builderScript = scriptPath) {
  const cli = dirname(builderScript);
  return { 'builder-script': builderScript, 'compiler-closure-script': join(cli, 'windows-compiler-closure.mjs'),
    'reader-settings-script': join(cli, '..', 'cli-connection-reader', 'repro-settings.mjs'),
    'compiler-audit-script': join(cli, 'windows-compiler-audit.ps1') };
}
export function verifyBootstrapInputs(inputs, files = runningInputFiles()) {
  const bytes = {};
  for (const id of CODE_INPUT_IDS) {
    if (!SHA256.test(inputs?.[id] ?? '') || !existsSync(files[id])) {
      throw new Error(`running ${id} differs from its manifest authority`);
    }
    bytes[id] = readFileSync(files[id]);
    if (sha256Bytes(bytes[id]) !== inputs[id]) throw new Error(`running ${id} differs from its manifest authority`);
  }
  return bytes;
}
export async function loadVerifiedBuildModules(inputs, files = runningInputFiles(), sourceRoot) {
  const bytes = verifyBootstrapInputs(inputs, files);
  if (!sourceRoot) throw new Error('extracted source must be authenticated before helper evaluation');
  verifyExtractedInputs(sourceRoot, inputs, files['builder-script']);
  const compiler = await import(`data:text/javascript;base64,${bytes['compiler-closure-script'].toString('base64')}`);
  const settings = await import(`data:text/javascript;base64,${bytes['reader-settings-script'].toString('base64')}`);
  return { compiler, settings: settings.READER_BUILD_SETTINGS };
}

// These small validators are deliberately inline: no unauthenticated helper or
// rejected network/device path may be evaluated or dereferenced during bootstrap.
export function validateBootstrapPath(path, label) {
  if (typeof path !== 'string' || !/^[a-z]:[\\/]/i.test(path) || path.length > 200
    || /[;=<>"|?*\x00-\x1f\x7f]/.test(path) || path.slice(2).includes(':')
    || path.slice(3).split(/[\\/]/).some(part => !part || part === '.' || part === '..' || /[. ]$/.test(part)
      || /^(con|prn|aux|nul|com[1-9]|lpt[1-9])(?:\.|$)/i.test(part))) throw new Error(`unsafe bootstrap ${label} path`);
}
export function validateBootstrapLocator(locator) {
  if (!locator || canonicalJson(Object.keys(locator).sort()) !== canonicalJson(['closures', 'schema', 'sourceBundle', 'tools'])) throw new Error('invalid bootstrap locator keys');
  if (locator.schema !== 'aware-windows-repro-locator/v1') throw new Error('unsupported bootstrap locator schema');
  const toolIds = ['git', 'node', 'npm-cli', 'postject', 'web-ifc-wasm', 'powershell'];
  const closureIds = ['npm-cache', 'cargo-home', ...['rust-bin', 'rust-lib', 'msvc-bin', 'msvc-include', 'msvc-lib', 'sdk-include', 'sdk-um-lib', 'sdk-ucrt-lib', 'sdk-bin'].map(id => `compiler-${id}`)];
  for (const [kind, ids] of [['tools', toolIds], ['closures', closureIds]]) {
    if (!locator[kind] || canonicalJson(Object.keys(locator[kind]).sort()) !== canonicalJson([...ids].sort())) throw new Error(`invalid bootstrap ${kind}`);
    for (const id of ids) validateBootstrapPath(locator[kind][id], id);
  }
  validateBootstrapPath(locator.sourceBundle, 'source bundle');
}
export function bootstrapSystemEnvironment(tempRoot, sharedObjects = process.report.getReport().sharedObjects) {
  const dirs = ['kernel32.dll', 'ntdll.dll'].map(name => {
    const matches = [...new Set(sharedObjects.filter(path => win32.basename(path).toLowerCase() === name).map(path => win32.dirname(path).toLowerCase()))];
    if (matches.length !== 1 || win32.basename(matches[0]) !== 'system32') throw new Error(`invalid loader-observed ${name}`);
    return matches[0];
  });
  if (dirs[0] !== dirs[1]) throw new Error('loader-observed Windows modules disagree');
  const system32 = dirs[0], windows = win32.dirname(system32);
  return { SystemRoot: windows, WINDIR: windows, ComSpec: win32.join(system32, 'cmd.exe'), PATH: system32,
    PATHEXT: '.COM;.EXE;.BAT;.CMD', TEMP: tempRoot, TMP: tempRoot };
}

const sha256Bytes = (bytes) => createHash('sha256').update(bytes).digest('hex');
const sha256File = (path) => sha256Bytes(readFileSync(path));
const canonical = (value) => Array.isArray(value) ? value.map(canonical)
  : value && typeof value === 'object'
    ? Object.fromEntries(Object.keys(value).sort().map((key) => [key, canonical(value[key])])) : value;
const canonicalJson = (value) => `${JSON.stringify(canonical(value), null, 2)}\n`;
const portable = (path) => path.split(sep).join('/');

export function writeBuilderManifestEvidence({ artifactsRoot, manifestText }) {
  if (canonicalJson(JSON.parse(manifestText)) !== manifestText) {
    throw new Error('builder manifest evidence must already be canonical JSON');
  }
  const path = join(artifactsRoot, 'builder-manifest.json');
  writeFileSync(path, manifestText, { encoding: 'utf8', flag: 'wx' });
  return { size: lstatSync(path).size, sha256: sha256File(path) };
}

export function rejectedAmbientKeys(env) {
  const counts = Object.keys(env).reduce((map, key) => map.set(key.toLowerCase(), (map.get(key.toLowerCase()) ?? 0) + 1), new Map());
  return Object.keys(env).filter((key) => counts.get(key.toLowerCase()) > 1 || EXACT_POISON.has(key.toUpperCase())
    || POISON_PREFIXES.some((prefix) => key.toLowerCase().startsWith(prefix.toLowerCase())))
    .sort();
}

export function closedGitEnvironment(systemEnv) {
  return {
    ...systemEnv,
    GIT_ALLOW_PROTOCOL: 'file',
    GIT_CONFIG_COUNT: '0',
    GIT_CONFIG_GLOBAL: 'NUL',
    GIT_CONFIG_NOSYSTEM: '1',
    GIT_TERMINAL_PROMPT: '0',
  };
}

function verifyFileRecord(id, manifest, locator) {
  const record = manifest.tools?.[id]; const path = locator.tools?.[id];
  if (!record || record.id !== id || !SHA256.test(record.sha256 ?? '')) throw new Error(`invalid tool record: ${id}`);
  if (typeof path !== 'string' || !existsSync(path) || !lstatSync(path).isFile()) throw new Error(`missing tool path: ${id}`);
  const actual = sha256File(path);
  if (actual !== record.sha256) throw new Error(`tool digest mismatch for ${id}: ${actual}`);
  return resolve(path);
}

function inventory(root) {
  const walk = (path) => readdirSync(path, { withFileTypes: true }).flatMap((entry) => {
    const child = join(path, entry.name);
    if (entry.isSymbolicLink()) throw new Error(`offline closure contains symbolic link: ${child}`);
    if (entry.isDirectory()) return walk(child);
    if (!entry.isFile()) throw new Error(`offline closure contains unsupported entry: ${child}`);
    return [child];
  });
  return walk(root).map((path) => ({
    path: portable(relative(root, path)), size: lstatSync(path).size, sha256: sha256File(path),
  })).sort((a, b) => Buffer.compare(Buffer.from(a.path), Buffer.from(b.path)));
}

function verifyClosure(id, manifest, locator, inventoryFunction = inventory) {
  const root = locator.closures?.[id]; const expected = manifest.closures?.[id];
  if (typeof root !== 'string' || !existsSync(root) || !lstatSync(root).isDirectory()) throw new Error(`missing closure: ${id}`);
  const actual = inventoryFunction(resolve(root));
  if (canonicalJson(actual) !== canonicalJson(expected?.files)) throw new Error(`offline closure inventory mismatch: ${id}`);
  return resolve(root);
}
export function verifyConsumedClosure(id, root, manifest, inventoryFunction = inventory) {
  return verifyClosure(id, manifest, { closures: { [id]: root } }, inventoryFunction);
}

export function materializeClosure(id, source, destination, manifest, inventoryFunction = inventory) {
  const files = inventoryFunction(source);
  if (canonicalJson(files) !== canonicalJson(manifest.closures?.[id]?.files)) throw new Error(`source offline closure inventory mismatch: ${id}`);
  if (existsSync(destination)) throw new Error('private closure destination must be fresh');
  mkdirSync(destination, { recursive: true });
  for (const record of files) {
    const output = join(destination, ...record.path.split('/')); mkdirSync(dirname(output), { recursive: true });
    copyFileSync(join(source, ...record.path.split('/')), output);
  }
  if (canonicalJson(inventoryFunction(destination)) !== canonicalJson(manifest.closures?.[id]?.files)) {
    throw new Error(`materialized offline closure inventory mismatch: ${id}`);
  }
  return destination;
}

export function cargoArguments(manifestPath, vendorDirectory) {
  if (typeof vendorDirectory !== 'string' || !vendorDirectory) throw new Error('Cargo vendor directory is required');
  const vendor = vendorDirectory.replaceAll('\\', '/').replaceAll('"', '\\"');
  return ['build', '--manifest-path', manifestPath, '--release', '--locked', '--offline',
    '--config', 'source.crates-io.replace-with="vendored-sources"',
    '--config', `source.vendored-sources.directory="${vendor}"`,
    '--target', TARGET, '--verbose', '--verbose'];
}

export function verifiedVendorDirectory(cargoClosure) {
  if (typeof cargoClosure !== 'string' || !cargoClosure.trim()) throw new Error('verified Cargo closure is required');
  const vendor = resolve(cargoClosure, 'vendor');
  if (!existsSync(vendor) || !lstatSync(vendor).isDirectory() || lstatSync(vendor).isSymbolicLink()) {
    throw new Error('verified Cargo vendor path must be an existing directory');
  }
  return vendor;
}

function pathSpellings(path) {
  return [...new Set([path, path.replaceAll('\\', '/')])];
}

export function rustCompilerArguments({ workRoot, sourceRoot, cargoHome, cargoVendor }) {
  if (typeof cargoVendor !== 'string' || !cargoVendor.trim()
    || !existsSync(cargoVendor) || !lstatSync(cargoVendor).isDirectory() || lstatSync(cargoVendor).isSymbolicLink()) {
    throw new Error('Cargo vendor path must be an existing directory');
  }
  // rustc applies the LAST matching textual prefix: broad roots precede their children.
  const roots = [[workRoot, '<work>'], [sourceRoot, '<source>'], [cargoHome, '<cargo-home>'], [cargoVendor, '<cargo-vendor>']];
  const remaps = roots.flatMap(([path, token]) => {
    if (typeof path !== 'string' || !path || /[\x00\x1f]/.test(path)) throw new Error('invalid compiler remap root');
    return pathSpellings(path).map((spelling) => `--remap-path-prefix=${spelling}=${token}`);
  });
  return ['-C', 'link-arg=/Brepro', ...remaps];
}

export function normalizeBuildText(text, roots) {
  const replacements = roots.flatMap(([path, token]) => pathSpellings(path)
    .flatMap((spelling) => [[spelling, token], [spelling.replaceAll('\\', '\\\\'), token]]))
    .sort((left, right) => right[0].length - left[0].length);
  for (const [path, token] of replacements) {
    const escaped = path.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    text = text.replace(new RegExp(escaped, 'gi'), () => token);
  }
  return text;
}

export function controlledEnvironment({ compiler, workRoot, sourceRoot, cargoHome, cargoVendor, tempRoot }) {
  if (!compiler?.host?.windows || !compiler.host.system32 || !compiler.environment || !compiler.tools) throw new Error('verified private compiler is required');
  return {
    SystemRoot: compiler.host.windows, WINDIR: compiler.host.windows, ComSpec: join(compiler.host.system32, 'cmd.exe'),
    ...compiler.environment,
    TEMP: tempRoot, TMP: tempRoot,
    CARGO_HOME: cargoHome, CARGO_TARGET_DIR: join(workRoot, 'cargo-target'), CARGO_NET_OFFLINE: 'true',
    CARGO_BUILD_JOBS: '1',
    RUSTC: compiler.tools.rustc, RUSTDOC: compiler.tools.rustdoc,
    CARGO_ENCODED_RUSTFLAGS: rustCompilerArguments({ workRoot, sourceRoot, cargoHome, cargoVendor }).join('\x1f'),
    CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER: compiler.tools.link,
    CC: compiler.tools.cl, AR: compiler.tools.lib, CFLAGS: '/Brepro', CL: '/Brepro',
    CC_x86_64_pc_windows_msvc: compiler.tools.cl, AR_x86_64_pc_windows_msvc: compiler.tools.lib,
    SOURCE_DATE_EPOCH: '0', TZ: 'UTC',
  };
}

export function verifyCargoVendorBinding({ args, env, cargoVendor }) {
  const operand = `source.vendored-sources.directory="${cargoVendor.replaceAll('\\', '/')}"`;
  const vendors = args.filter(arg => arg.startsWith('source.vendored-sources.directory='));
  if (vendors.length !== 1 || vendors[0] !== operand) throw new Error('Cargo vendor operand differs from the verified root');
  const flags = env.CARGO_ENCODED_RUSTFLAGS.split('\x1f');
  const vendorMaps = flags.filter(arg => arg.endsWith('=<cargo-vendor>'));
  const expected = pathSpellings(cargoVendor).map(path => `--remap-path-prefix=${path}=<cargo-vendor>`);
  if (canonicalJson(vendorMaps) !== canonicalJson(expected)) throw new Error('compiler vendor remaps differ from the verified root');
}

export function logicalCargoCommand(args, roots) {
  const command = normalizeBuildText(['<cargo>', ...args].join(' '), roots).replaceAll('\\', '/');
  if (command !== LOGICAL_CARGO_COMMAND) throw new Error('Cargo command differs from the closed build contract');
  return command;
}

export function createCargoBuild({ compiler, workRoot, sourceRoot, cargoHome, cargoClosure, tempRoot }) {
  const cargoVendor = verifiedVendorDirectory(cargoClosure);
  const env = Object.freeze(controlledEnvironment({ compiler, workRoot, sourceRoot, cargoHome, cargoVendor, tempRoot }));
  const args = Object.freeze(cargoArguments(join(sourceRoot, 'cli', 'Cargo.toml'), cargoVendor));
  verifyCargoVendorBinding({ args, env, cargoVendor });
  const command = logicalCargoCommand(args, [[sourceRoot, '<source>'], [cargoVendor, '<cargo-vendor>']]);
  return Object.freeze({ cargoVendor, env, args, command });
}

export function assertVerboseCargoProof(text, rustArgs = []) {
  const checks = [
    [/--release/, 'release profile'], [/--locked/, 'locked mode'], [/--offline/, 'offline mode'],
    [/x86_64-pc-windows-msvc/, 'Windows MSVC target'], [/(?:link-arg=|link-arg\s+)\/?Brepro/i, 'rustc /Brepro'],
  ];
  for (const [pattern, label] of checks) if (!pattern.test(text)) throw new Error(`verbose Cargo proof omitted ${label}`);
  for (const argument of rustArgs.filter((arg) => arg.startsWith('--remap-path-prefix='))) {
    if (!text.includes(argument) && !text.includes(argument.replaceAll('\\', '\\\\'))) {
      throw new Error(`verbose Cargo proof omitted compiler remap: ${argument}`);
    }
  }
}

export function verifyExtractedInputs(source, inputs, runningScript = scriptPath) {
  const running = runningInputFiles(runningScript), extracted = runningInputFiles(join(source, 'cli', 'build-windows-internal-repro.mjs'));
  const actual = {
    'aware-cargo-lock': sha256File(join(source, 'cli', 'Cargo.lock')),
    'reader-package-lock': sha256File(join(source, 'cli-connection-reader', 'package-lock.json')),
    ...Object.fromEntries(CODE_INPUT_IDS.map(id => [id, sha256File(extracted[id])])),
  };
  if (Object.entries(actual).some(([id, digest]) => inputs?.[id] !== digest)
    || CODE_INPUT_IDS.some(id => actual[id] !== sha256File(running[id]))) {
    throw new Error('extracted source locks or builder script differ from running builder and manifest');
  }
}

function run(path, args, options = {}) {
  const result = spawnSync(path, args, {
    encoding: 'utf8', windowsHide: true, maxBuffer: COMMAND_OUTPUT_BUFFER_BYTES, ...options,
  });
  const combined = `${result.stdout ?? ''}${result.stderr ?? ''}`;
  if (result.error || result.status !== 0) throw new Error(`${basename(path)} failed (${result.status}): ${result.error?.message ?? combined}`);
  return combined;
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

export function verifyBuildAuthority({ manifest, locator, env = process.env }) {
  validateBootstrapLocator(locator);
  if (manifest?.schema !== 'aware-windows-repro-builder/v1') throw new Error('unsupported builder manifest schema');
  if (locator?.schema !== 'aware-windows-repro-locator/v1') throw new Error('unsupported builder locator schema');
  const poisoned = rejectedAmbientKeys(env);
  if (poisoned.length) throw new Error(`ambient build authority is forbidden: ${poisoned.join(', ')}`);
  if (manifest.platform !== 'win32' || manifest.arch !== 'x64' || manifest.nodeVersion !== '24.14.0'
    || manifest.rustVersion !== '1.95.0' || manifest.target !== TARGET) throw new Error('unsupported pinned build platform/toolchain');
  if (process.platform !== 'win32' || process.arch !== 'x64' || process.versions.node !== manifest.nodeVersion) {
    throw new Error('wrapper must run under the pinned Windows x64 Node');
  }
  const toolIds = ['git', 'node', 'npm-cli', 'postject', 'web-ifc-wasm', 'powershell'];
  const tools = Object.fromEntries(toolIds.map((id) => [id, verifyFileRecord(id, manifest, locator)]));
  verifyBootstrapInputs(manifest.inputs);
  if (realpathSync(tools.node) !== realpathSync(process.execPath)) throw new Error('verified node.exe is not the running Node');
  const bundle = locator.sourceBundle;
  if (typeof bundle !== 'string' || !existsSync(bundle) || sha256File(bundle) !== manifest.source?.bundleSha256) {
    throw new Error('source bundle digest mismatch');
  }
  if (!SHA1.test(manifest.source?.commit ?? '') || !SHA1.test(manifest.source?.tree ?? '')) throw new Error('invalid source commit/tree');
  return { tools, bundle: resolve(bundle) };
}

export async function buildWindowsInternal({ manifestPath, locatorPath, outputRoot, env = process.env }) {
  for (const [label, path] of Object.entries({ manifestPath, locatorPath, outputRoot })) validateBootstrapPath(path, label);
  const manifestText = canonicalJson(JSON.parse(readFileSync(manifestPath, 'utf8')));
  const manifest = JSON.parse(manifestText); const locator = JSON.parse(readFileSync(locatorPath, 'utf8'));
  const authority = verifyBuildAuthority({ manifest, locator, env });
  const output = resolve(outputRoot);
  if (existsSync(output) && readdirSync(output).length) throw new Error('output root must be absent or empty');
  mkdirSync(output, { recursive: true });
  const work = join(output, 'work'); const artifacts = join(output, 'artifacts'); const evidence = join(output, 'evidence');
  const source = join(work, 'source'); const tempRoot = join(work, 'temp');
  mkdirSync(tempRoot, { recursive: true }); mkdirSync(source); mkdirSync(artifacts); mkdirSync(evidence);
  const gitEnv = closedGitEnvironment(bootstrapSystemEnvironment(tempRoot));
  run(authority.tools.git, ['clone', '--no-checkout', '--config', 'core.autocrlf=false', authority.bundle, source], { env: gitEnv });
  run(authority.tools.git, ['sparse-checkout', 'init', '--no-cone'], { cwd: source, env: gitEnv });
  run(authority.tools.git, ['sparse-checkout', 'set', '--no-cone', ...SOURCE_PATHS.map((path) => `/${path}/`)],
    { cwd: source, env: gitEnv });
  run(authority.tools.git, ['checkout', '--detach', manifest.source.commit], { cwd: source, env: gitEnv });
  const commit = run(authority.tools.git, ['rev-parse', 'HEAD'], { cwd: source, env: gitEnv }).trim();
  const tree = run(authority.tools.git, ['rev-parse', 'HEAD^{tree}'], { cwd: source, env: gitEnv }).trim();
  if (commit !== manifest.source.commit || tree !== manifest.source.tree) throw new Error('extracted source identity mismatch');
  const unexpectedSource = readdirSync(source).filter((name) => name !== '.git' && !SOURCE_PATHS.includes(name));
  if (unexpectedSource.length) throw new Error(`sparse source boundary contains unexpected roots: ${unexpectedSource.join(', ')}`);

  verifyExtractedInputs(source, manifest.inputs);
  const { compiler: compilerModule, settings } = await loadVerifiedBuildModules(manifest.inputs, runningInputFiles(), source);
  if (canonicalJson(manifest.settings) !== canonicalJson(settings)) throw new Error('reader build settings differ from the closed implementation');
  compilerModule.validateCompilerManifest(manifest); compilerModule.validateCompilerLocator(locator);
  compilerModule.validateWindowsPath(outputRoot, 'output root');
  const npmClosure = verifyClosure('npm-cache', manifest, locator, compilerModule.inventory);
  const cargoSourceClosure = verifyClosure('cargo-home', manifest, locator, compilerModule.inventory);
  const auditScript = runningInputFiles()['compiler-audit-script'];
  const host = compilerModule.discoverSystemHost({ locator, manifest, auditScript, workRoot: work });
  const compiler = compilerModule.materializeCompiler({ manifest, locator, workRoot: work, host });
  const npmCache = materializeClosure('npm-cache', npmClosure, join(work, 'npm-cache'), manifest, compilerModule.inventory);
  const cargoClosure = materializeClosure('cargo-home', cargoSourceClosure, join(work, 'cargo-closure'), manifest, compilerModule.inventory);
  const cargoHome = join(work, 'cargo-home'); mkdirSync(cargoHome);
  const cargoBuild = createCargoBuild({ compiler, workRoot: work, sourceRoot: source, cargoHome, cargoClosure, tempRoot });
  const { cargoVendor, env: controlled, args: cargoArgs } = cargoBuild;
  const rustArgs = controlled.CARGO_ENCODED_RUSTFLAGS.split('\x1f');
  const roots = [[source, '<source>'], [work, '<work>'], [output, '<output>'],
    [cargoHome, '<cargo-home>'], [cargoVendor, '<cargo-vendor>'], [cargoClosure, '<cargo-closure>'],
    [compiler.root, '<compiler>']];
  const logicalRustFlags = normalizeBuildText(rustArgs.join(' '), roots);
  if (logicalRustFlags !== WINDOWS_LOGICAL_RUST_FLAGS) throw new Error('compiler flags differ from the closed Windows build contract');
  const audits = [];
  const executeCompiler = (id, args, label) => {
    const result = compilerModule.runAuditedCompiler({ compiler, toolPath: compiler.tools[id], args, label,
      cwd: source, env: controlled, auditScript, evidenceRoot: evidence, targetRoot: join(work, 'cargo-target') });
    audits.push({ label, path: portable(relative(output, result.evidencePath)), sha256: result.evidenceSha256 });
    return result.text;
  };
  const cargoVersion = executeCompiler('cargo', ['--version'], 'cargo-version').trim();
  const rustVersion = executeCompiler('rustc', ['--version'], 'rust-version').trim();
  if (!/^cargo 1\.95\.0\b/.test(cargoVersion) || !/^rustc 1\.95\.0\b/.test(rustVersion)) {
    throw new Error(`pinned Rust toolchain version mismatch: ${cargoVersion}; ${rustVersion}`);
  }
  for (const query of ['sysroot', 'target-libdir']) {
    const path = executeCompiler('rustc', ['--print', query], `rust-${query}`).trim();
    if (!existsSync(path) || !compilerModule.beneath(realpathSync.native(path), join(compiler.root, 'rust'))) throw new Error(`rustc ${query} escaped its private compiler`);
  }
  const npmEnv = {
    SystemRoot: controlled.SystemRoot, WINDIR: controlled.WINDIR, ComSpec: controlled.ComSpec,
    PATHEXT: controlled.PATHEXT, PATH: dirname(authority.tools.node), TEMP: tempRoot, TMP: tempRoot,
    npm_config_cache: npmCache, npm_config_offline: 'true', npm_config_ignore_scripts: 'true',
    npm_config_audit: 'false', npm_config_fund: 'false', npm_config_update_notifier: 'false',
    npm_config_logs_dir: join(work, 'npm-logs'), npm_config_logs_max: '0',
  };
  const readerRoot = join(source, 'cli-connection-reader');
  run(authority.tools.node, [authority.tools['npm-cli'], 'ci', '--offline', '--ignore-scripts'], { cwd: readerRoot, env: npmEnv });

  const cargoLog = executeCompiler('cargo', cargoArgs, 'cargo-build');
  compilerModule.verifyPrivateCompiler(compiler);
  if (canonicalJson(compilerModule.inventory(cargoClosure)) !== canonicalJson(manifest.closures['cargo-home'].files)) throw new Error('private Cargo source changed during compilation');
  assertVerboseCargoProof(`${cargoArgs.join(' ')}\n${cargoLog}`, rustArgs);
  const normalizedCargo = normalizeBuildText(`<cargo> ${cargoArgs.join(' ')}\n${cargoLog}`, roots);
  writeFileSync(join(evidence, 'cargo-verbose.local.txt'), cargoLog, 'utf8');
  writeFileSync(join(evidence, 'cargo-verbose.normalized.txt'), normalizedCargo, 'utf8');

  const readerLocator = join(work, 'reader-locator.local.json');
  writeFileSync(readerLocator, canonicalJson({
    schema: 'aware-windows-repro-locator/v1', tools: {
      node: authority.tools.node,
      postject: join(readerRoot, 'node_modules', 'postject', 'dist', 'cli.js'),
      'web-ifc-wasm': join(readerRoot, 'node_modules', 'web-ifc', 'web-ifc-node.wasm'),
    },
  }), 'utf8');
  for (const [id, path] of Object.entries(JSON.parse(readFileSync(readerLocator, 'utf8')).tools)) {
    const expected = manifest.tools[id]?.sha256; if (sha256File(path) !== expected) throw new Error(`installed reader tool differs: ${id}`);
  }
  run(authority.tools.node, [join(readerRoot, 'build-internal-repro.mjs'),
    '--manifest', manifestPath, '--locator', readerLocator, '--output', join(artifacts, 'reader')], { cwd: readerRoot, env: {
    SystemRoot: controlled.SystemRoot, WINDIR: controlled.WINDIR, ComSpec: controlled.ComSpec,
    PATHEXT: controlled.PATHEXT, PATH: dirname(authority.tools.node), TEMP: tempRoot, TMP: tempRoot,
  } });

  const awareSource = join(work, 'cargo-target', TARGET, 'release', 'aware.exe');
  if (!existsSync(awareSource)) throw new Error('Cargo did not produce aware.exe');
  copyFileSync(awareSource, join(artifacts, 'aware.exe'));
  verifyConsumedClosure('npm-cache', npmCache, manifest, compilerModule.inventory);
  const builderManifestRecord = writeBuilderManifestEvidence({ artifactsRoot: artifacts, manifestText });
  const receipt = {
    schema: 'aware-windows-runtime-build-receipt/v1',
    buildId: sha256Bytes(Buffer.from(manifestText)), builderManifestSha256: sha256Bytes(Buffer.from(manifestText)),
    source: manifest.source, inputs: manifest.inputs, target: TARGET,
    compiler: compilerModule.compilerSummary(manifest),
    flags: { rust: logicalRustFlags, native: '/Brepro', cargo: ['--release', '--locked', '--offline'] },
    outputs: {
      'aware.exe': { size: lstatSync(join(artifacts, 'aware.exe')).size, sha256: sha256File(join(artifacts, 'aware.exe')) },
      'builder-manifest.json': builderManifestRecord,
      'reader/build-receipt.json': {
        size: lstatSync(join(artifacts, 'reader', 'build-receipt.json')).size,
        sha256: sha256File(join(artifacts, 'reader', 'build-receipt.json')),
      },
    },
    commands: {
      cargo: cargoBuild.command,
      reader: '<node> <source>/cli-connection-reader/build-internal-repro.mjs --manifest <manifest> --locator <local-locator> --output <artifacts>/reader',
    },
    unsignedTestMedia: true,
  };
  writeFileSync(join(artifacts, 'build-receipt.json'), canonicalJson(receipt), 'utf8');
  writeFileSync(join(evidence, 'compiler-provenance.json'), canonicalJson({ schema: 'aware-compiler-provenance/v1',
    source: manifest.source, buildId: receipt.buildId, compiler: receipt.compiler, audits,
    artifacts: inventory(artifacts) }), 'utf8');
  return receipt;
}

if (realpathSync(scriptPath) === realpathSync(process.argv[1])) {
  const args = parseArgs(process.argv.slice(2));
  await buildWindowsInternal({ manifestPath: resolve(args.manifest), locatorPath: resolve(args.locator), outputRoot: resolve(args.output) });
}
