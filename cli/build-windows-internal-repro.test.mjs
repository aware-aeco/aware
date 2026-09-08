import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';
import {
  assertVerboseCargoProof, cargoArguments, closedGitEnvironment, COMMAND_OUTPUT_BUFFER_BYTES,
  controlledEnvironment, materializeClosure, rejectedAmbientKeys, SOURCE_PATHS,
  writeBuilderManifestEvidence, rustCompilerArguments, verifiedVendorDirectory, normalizeBuildText,
  verifyExtractedInputs,
  createCargoBuild, verifyCargoVendorBinding, logicalCargoCommand, LOGICAL_CARGO_COMMAND,
  runningInputFiles, loadVerifiedBuildModules,
  nativeCompilerEnvironmentFlags, WINDOWS_LOGICAL_NATIVE_FLAGS, verifyRustHostVersion, rejectSourceCargoConfiguration,
} from './build-windows-internal-repro.mjs';

const fakeCompiler = () => ({ tools: { rustc: 'RUSTC', rustdoc: 'RUSTDOC', cl: 'CL', lib: 'LIB', link: 'LINK' },
  host: { windows: 'SYSTEM', system32: 'SYSTEM32' },
  environment: { PATH: 'PRIVATE_PATH', INCLUDE: 'PRIVATE_INCLUDE', LIB: 'PRIVATE_LIB', LIBPATH: 'PRIVATE_LIBPATH', PATHEXT: '.EXE', _NO_DEBUG_HEAP: '1' } });

test('verbose command evidence has an explicit bounded buffer large enough for a full Cargo proof', () => {
  assert.equal(COMMAND_OUTPUT_BUFFER_BYTES, 128 * 1024 * 1024);
});

test('builder extracts only the two closed runtime source roots', () => {
  assert.deepEqual(SOURCE_PATHS, ['cli', 'cli-connection-reader']);
  assert.equal(Object.isFrozen(SOURCE_PATHS), true);
});

test('Cargo invocation is locked, offline, release, verbose, and Windows-specific', () => {
  assert.deepEqual(cargoArguments('C:/src/cli/Cargo.toml', 'C:\\closure\\vendor'), [
    'build', '--manifest-path', 'C:/src/cli/Cargo.toml', '--release', '--locked', '--offline',
    '--config', 'source.crates-io.replace-with="vendored-sources"',
    '--config', 'source.vendored-sources.directory="C:/closure/vendor"',
    '--verbose', '--verbose',
  ]);
});

test('controlled environment owns reproducible Rust and native MSVC flags', () => {
  const root = mkdtempSync(join(tmpdir(), 'aware-vendor-'));
  mkdirSync(join(root, 'vendor'));
  try {
  const locator = { tools: { rustc: 'RUSTC', rustdoc: 'RUSTDOC', cl: 'CL', lib: 'LIB' }, environment: {
    PATH: 'PATH', INCLUDE: 'INCLUDE', LIB: 'LIBS', LIBPATH: 'LIBPATH', SystemRoot: 'SYSTEM',
    WINDIR: 'WINDOWS', ComSpec: 'CMD', PATHEXT: '.EXE',
  } };
  const vendor = verifiedVendorDirectory(root);
  const options = { compiler: fakeCompiler(), workRoot: 'C:\\WORK', sourceRoot: 'C:\\WORK\\SOURCE', cargoHome: 'C:\\WORK\\CARGO', cargoVendor: vendor, tempRoot: 'TEMP' };
  const env = controlledEnvironment(options);
  assert.equal(env.RUSTFLAGS, undefined);
  assert.deepEqual(env.CARGO_ENCODED_RUSTFLAGS.split('\x1f'), [
    '-C', 'link-arg=/Brepro',
    '--remap-path-prefix=C:\\WORK=<work>', '--remap-path-prefix=C:/WORK=<work>',
    '--remap-path-prefix=C:\\WORK\\SOURCE=<source>', '--remap-path-prefix=C:/WORK/SOURCE=<source>',
    '--remap-path-prefix=C:\\WORK\\CARGO=<cargo-home>', '--remap-path-prefix=C:/WORK/CARGO=<cargo-home>',
    ...[...new Set([vendor, vendor.replaceAll('\\', '/')])].map(path => `--remap-path-prefix=${path}=<cargo-vendor>`),
  ]);
  for (const cargoVendor of [undefined, '', join(root, 'missing')]) {
    assert.throws(() => controlledEnvironment({ ...options, cargoVendor }), /vendor path/);
  }
  writeFileSync(join(root, 'not-a-directory'), 'x');
  assert.throws(() => rustCompilerArguments({ ...options, cargoVendor: join(root, 'not-a-directory') }), /vendor path/);
  assert.equal(env.CFLAGS, '/Brepro');
  assert.equal(normalizeBuildText(env.CL, [['C:\\WORK', '<work>']]), WINDOWS_LOGICAL_NATIVE_FLAGS);
  assert.equal(env.AR, join('C:\\WORK', 'cargo-target', 'native-tools', 'aware-lib.exe'));
  assert.equal(env.AR_x86_64_pc_windows_msvc, env.AR);
  assert.equal(env.CARGO_NET_OFFLINE, 'true'); assert.equal(env.RUSTC, 'RUSTC');
  assert.equal(env.CARGO_BUILD_JOBS, '1');
  assert.equal(env._NO_DEBUG_HEAP, '1');
  assert.equal(env.NODE_OPTIONS, undefined); assert.equal(env.GOOGLE_CLIENT_SECRET, undefined);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test('verified closure and vendor operands cannot silently become an empty Cargo home', () => {
  const root = mkdtempSync(join(tmpdir(), 'aware-vendor-root-'));
  try {
    assert.throws(() => verifiedVendorDirectory(undefined), /closure is required/);
    assert.throws(() => verifiedVendorDirectory(''), /closure is required/);
    assert.throws(() => verifiedVendorDirectory(root), /existing directory/);
    writeFileSync(join(root, 'vendor'), 'not a directory');
    assert.throws(() => verifiedVendorDirectory(root), /existing directory/);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test('the production Cargo contract rejects divergent existing vendor roots and missing config operands', () => {
  const root = mkdtempSync(join(tmpdir(), 'aware-cargo-contract-'));
  try {
    const closures = [join(root, 'sealed a'), join(root, 'sealed b')];
    for (const closure of closures) mkdirSync(join(closure, 'vendor'), { recursive: true });
    const locator = { tools: { rustc: 'RUSTC', rustdoc: 'RUSTDOC', cl: 'CL', lib: 'LIB' }, environment: {
      PATH: 'PATH', INCLUDE: 'INCLUDE', LIB: 'LIBS', LIBPATH: 'LIBPATH', SystemRoot: 'SYSTEM',
      WINDIR: 'WINDOWS', ComSpec: 'CMD', PATHEXT: '.EXE',
    } };
    // This is a Windows command contract even when its pure tests run on Linux.
    const options = { compiler: fakeCompiler(), workRoot: 'C:\\WORK', sourceRoot: join(root, 'work', 'source'), cargoHome: join(root, 'work', 'cargo-home'), tempRoot: join(root, 'temp') };
    const [a, b] = closures.map(cargoClosure => createCargoBuild({ ...options, cargoClosure }));
    assert.equal(a.command, LOGICAL_CARGO_COMMAND);
    assert.doesNotThrow(() => verifyCargoVendorBinding(a));
    const isVendor = arg => arg.startsWith('source.vendored-sources.directory=');
    const divergentArgs = a.args.map(arg => isVendor(arg) ? b.args.find(isVendor) : arg);
    assert.throws(() => verifyCargoVendorBinding({ ...a, args: divergentArgs }), /vendor operand differs/);
    assert.throws(() => verifyCargoVendorBinding({ ...a, env: b.env }), /vendor remaps differ/);
    for (let index = 0; index < a.args.length; index++) {
      if (a.args[index] !== '--config') continue;
      const missingOperand = [...a.args]; missingOperand.splice(index, 2);
      assert.throws(() => logicalCargoCommand(missingOperand, [[options.sourceRoot, '<source>'], [a.cargoVendor, '<cargo-vendor>']]), /closed build contract/);
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test('logical evidence replaces specific roots first in both Windows spellings and case variants', () => {
  const roots = [['C:\\WORK', '<work>'], ['C:\\WORK\\source', '<source>'], ['C:\\SEALED A\\vendor', '<cargo-vendor>']];
  assert.equal(normalizeBuildText('C:\\WORK\\source\\main.rs c:/work/source/main.rs C:\\SEALED A\\vendor\\dep.rs', roots),
    '<source>\\main.rs <source>/main.rs <cargo-vendor>\\dep.rs');
  assert.equal(normalizeBuildText('C:\\\\SEALED A\\\\vendor\\\\dep.rs', roots), '<cargo-vendor>\\\\dep.rs');
});

test('a new source builder cannot be built using an old running script and lock-only manifest', () => {
  const root = mkdtempSync(join(tmpdir(), 'aware-builder-source-'));
  try {
    mkdirSync(join(root, 'cli')); mkdirSync(join(root, 'cli-connection-reader'));
    const files = { 'aware-cargo-lock': join(root, 'cli', 'Cargo.lock'), 'reader-package-lock': join(root, 'cli-connection-reader', 'package-lock.json'), ...runningInputFiles(join(root, 'cli', 'build-windows-internal-repro.mjs')) };
    for (const [id, path] of Object.entries(files)) writeFileSync(path, id);
    const running = join(root, 'running', 'cli', 'build-windows-internal-repro.mjs');
    mkdirSync(join(root, 'running', 'cli'), { recursive: true }); mkdirSync(join(root, 'running', 'cli-connection-reader'));
    for (const [id, path] of Object.entries(runningInputFiles(running))) writeFileSync(path, id);
    const inputs = Object.fromEntries(Object.entries(files).map(([id, path]) => [id, createHash('sha256').update(readFileSync(path)).digest('hex')]));
    assert.doesNotThrow(() => verifyExtractedInputs(root, inputs, running));
    writeFileSync(files['builder-script'], 'new source builder');
    assert.throws(() => verifyExtractedInputs(root, inputs, running), /extracted source/);
    inputs['builder-script'] = createHash('sha256').update(readFileSync(files['builder-script'])).digest('hex');
    assert.throws(() => verifyExtractedInputs(root, inputs, running), /extracted source/);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test('offline cache use is isolated in a verified private copy', () => {
  const root = mkdtempSync(join(tmpdir(), 'aware-builder-closure-'));
  try {
    const source = join(root, 'source'); const destination = join(root, 'destination');
    mkdirSync(join(source, 'cache'), { recursive: true });
    writeFileSync(join(source, 'cache', 'index'), 'closed');
    const digest = createHash('sha256').update('closed').digest('hex');
    const manifest = { closures: { cache: { files: [{ path: 'cache/index', size: 6, sha256: digest }] } } };
    assert.equal(materializeClosure('cache', source, destination, manifest), destination);
    writeFileSync(join(destination, 'cache', 'index'), 'mutate');
    assert.equal(readFileSync(join(source, 'cache', 'index'), 'utf8'), 'closed');
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test('ambient authority detector covers compiler, npm, dotnet, and credentials', () => {
  assert.deepEqual(rejectedAmbientKeys({ PATH: 'ok', LINK: 'poison', npm_config_cache: 'x', COREHOST_TRACE: '1' }),
    ['COREHOST_TRACE', 'LINK', 'PATH', 'npm_config_cache']);
});

test('Git cannot consult host configuration, templates, prompts, or network transports', () => {
  const env = closedGitEnvironment({ SystemRoot: 'WINDOWS' });
  assert.equal(env.GIT_CONFIG_NOSYSTEM, '1');
  assert.equal(env.GIT_CONFIG_GLOBAL, 'NUL');
  assert.equal(env.GIT_CONFIG_COUNT, '0');
  assert.equal(env.GIT_ALLOW_PROTOCOL, 'file');
  assert.equal(env.GIT_TERMINAL_PROMPT, '0');
});

test('verbose proof goes red if the actual command loses /Brepro or a locked flag', () => {
  const header = 'cargo build --release --locked --offline';
  const command = args => '     Running `set CARGO=private&& "C:\\private compiler\\rustc.exe" --crate-name probe -C link-arg=/Brepro '+args+'`';
  const complete = header+'\n'+command('');
  assert.doesNotThrow(() => assertVerboseCargoProof(complete));
  assert.throws(() => assertVerboseCargoProof(complete.replace('/Brepro', '/DEBUG')), /omitted \/Brepro/);
  assert.throws(() => assertVerboseCargoProof(complete.replace('--locked', '')), /locked mode/);
  const vendorArgs = ['--remap-path-prefix=C:\\sealed cache\\vendor=<cargo-vendor>', '--remap-path-prefix=C:/sealed cache/vendor=<cargo-vendor>'];
  const mapped = header+'\n'+command(vendorArgs.map(a=>'"'+a+'"').join(' '));
  assert.doesNotThrow(() => assertVerboseCargoProof(mapped, vendorArgs, 'C:\\private compiler\\rustc.exe'));
  const rawExecutable = mapped.replace('"C:\\private compiler\\rustc.exe"', 'C:\\private compiler\\rustc.exe');
  assert.doesNotThrow(() => assertVerboseCargoProof(rawExecutable, vendorArgs, 'C:\\private compiler\\rustc.exe'), 'real Windows Cargo display leaves the spaced executable unquoted');
  const buildScript = '     Running `set RUSTC=C:\\private compiler\\rustc.exe&& C:\\target path\\build-script-build.exe`';
  assert.doesNotThrow(() => assertVerboseCargoProof(rawExecutable+'\n'+buildScript, vendorArgs, 'C:\\private compiler\\rustc.exe'), 'an environment assignment does not turn a build script into rustc');
  for (const missing of vendorArgs) {
    assert.throws(() => assertVerboseCargoProof(header+'\n'+command(vendorArgs.filter(arg => arg !== missing).join(' ')), vendorArgs), /compiler remap/);
  }
  assert.throws(() => assertVerboseCargoProof(mapped+'\n'+command(''), vendorArgs), /compiler remap/, 'a fully mapped target cannot cover a host compilation');
  assert.throws(() => assertVerboseCargoProof(mapped+'\n'+command('').slice(0, -1), vendorArgs), /unparseable rustc/, 'a malformed host command cannot disappear');
  assert.throws(() => assertVerboseCargoProof(mapped+'\n'+command('"unterminated'), vendorArgs), /unparseable rustc/);
  assert.throws(() => assertVerboseCargoProof(mapped.replaceAll('=<cargo-vendor>', '=<cargo-vendor>-wrong'), vendorArgs), /compiler remap/, 'a partial remap match is not proof');
  assert.throws(() => assertVerboseCargoProof(mapped.replace('link-arg=/Brepro', 'link-arg=/Brepro-wrong'), vendorArgs), /omitted \/Brepro/);
  assert.throws(() => assertVerboseCargoProof(header+' '+vendorArgs.join(' ')), /actual rustc compilations/);
  assert.throws(() => assertVerboseCargoProof(mapped, vendorArgs, 'C:\\different\\rustc.exe'), /different rustc/);
});

test('native mappings fit CL limits and quote supported long Unicode paths', () => {
  const work = 'C:\\'+('long Łódź directory '.repeat(9)).trim();
  const flags = nativeCompilerEnvironmentFlags(work);
  assert.ok(flags.length <= 1024);
  assert.equal(normalizeBuildText(flags, [[work, '<work>']]), WINDOWS_LOGICAL_NATIVE_FLAGS);
  for (const bad of ['C:\\root"injection', 'C:\\root;extra', '\\\\server\\share', 'C:relative', 'C:\\'+('a'.repeat(200))]) {
    assert.throws(() => nativeCompilerEnvironmentFlags(bad), /unsafe bootstrap/);
  }
});

test('implicit native Cargo target requires the exact audited Rust host', () => {
  const proof = 'rustc 1.95.0 (59807616e 2026-04-14)\nbinary: rustc\nhost: x86_64-pc-windows-msvc\nrelease: 1.95.0\n';
  assert.doesNotThrow(() => verifyRustHostVersion(proof));
  for (const bad of [proof.replace('1.95.0', '1.94.0'), proof.replace('x86_64-pc-windows-msvc', 'aarch64-pc-windows-msvc'), proof+'host: x86_64-pc-windows-msvc\n', 'rustc 1.95.0']) {
    assert.throws(() => verifyRustHostVersion(bad), /version\/host mismatch/);
  }
});

test('Cargo source, private home and every ancestor config refuse before a launch', () => {
  const root = mkdtempSync(join(tmpdir(), 'aware-config-refusal-'));
  const source = join(root, 'output', 'work', 'source'), home = join(root, 'output', 'work', 'cargo-home');
  mkdirSync(source, {recursive:true}); mkdirSync(home);
  try {
    assert.doesNotThrow(() => rejectSourceCargoConfiguration(source, home));
    for (const directory of [join(root,'.cargo'), join(source,'.cargo'), join(source,'cli','.cargo'), home]) {
      mkdirSync(directory,{recursive:true});
      for (const name of ['config','config.toml']) {
        const file=join(directory,name); writeFileSync(file,'[build]\ntarget="x86_64-pc-windows-msvc"\n');
        let launched=false;
        assert.throws(()=>{rejectSourceCargoConfiguration(source,home);launched=true;},/Cargo configuration/);
        assert.equal(launched,false); rmSync(file);
      }
    }
  } finally { rmSync(root,{recursive:true,force:true}); }
});

test('builder manifest is retained byte-for-byte as independently digestible evidence', () => {
  const root = mkdtempSync(join(tmpdir(), 'aware-builder-manifest-'));
  try {
    const manifestText = '{\n  "schema": "aware-windows-repro-builder/v1"\n}\n';
    const record = writeBuilderManifestEvidence({ artifactsRoot: root, manifestText });
    assert.equal(readFileSync(join(root, 'builder-manifest.json'), 'utf8'), manifestText);
    assert.equal(record.size, Buffer.byteLength(manifestText));
    assert.match(record.sha256, /^[0-9a-f]{64}$/);
    assert.throws(() => writeBuilderManifestEvidence({ artifactsRoot: root, manifestText }), /EEXIST/);
    assert.throws(() => writeBuilderManifestEvidence({ artifactsRoot: root, manifestText: '{"schema":"x"}\n' }),
      /canonical JSON/);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
