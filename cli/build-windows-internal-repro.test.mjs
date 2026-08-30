import assert from 'node:assert/strict';
import test from 'node:test';
import {
  assertVerboseCargoProof, cargoArguments, controlledEnvironment, rejectedAmbientKeys,
} from './build-windows-internal-repro.mjs';

test('Cargo invocation is locked, offline, release, verbose, and Windows-specific', () => {
  assert.deepEqual(cargoArguments('C:/src/cli/Cargo.toml'), [
    'build', '--manifest-path', 'C:/src/cli/Cargo.toml', '--release', '--locked', '--offline',
    '--target', 'x86_64-pc-windows-msvc', '--verbose', '--verbose',
  ]);
});

test('controlled environment owns reproducible Rust and native MSVC flags', () => {
  const locator = { tools: { rustc: 'RUSTC', rustdoc: 'RUSTDOC', cl: 'CL', lib: 'LIB' }, environment: {
    PATH: 'PATH', INCLUDE: 'INCLUDE', LIB: 'LIBS', LIBPATH: 'LIBPATH', SystemRoot: 'SYSTEM',
    WINDIR: 'WINDOWS', ComSpec: 'CMD', PATHEXT: '.EXE',
  } };
  const env = controlledEnvironment({ locator, workRoot: 'WORK', sourceRoot: 'SOURCE', cargoHome: 'CARGO', tempRoot: 'TEMP' });
  assert.match(env.RUSTFLAGS, /^-C link-arg=\/Brepro/);
  assert.match(env.RUSTFLAGS, /--remap-path-prefix=SOURCE=<source>/);
  assert.equal(env.CFLAGS, '/Brepro'); assert.equal(env.CL, '/Brepro');
  assert.equal(env.CARGO_NET_OFFLINE, 'true'); assert.equal(env.RUSTC, 'RUSTC');
  assert.equal(env.NODE_OPTIONS, undefined); assert.equal(env.GOOGLE_CLIENT_SECRET, undefined);
});

test('ambient authority detector covers compiler, npm, dotnet, and credentials', () => {
  assert.deepEqual(rejectedAmbientKeys({ PATH: 'ok', LINK: 'poison', npm_config_cache: 'x', COREHOST_TRACE: '1' }),
    ['COREHOST_TRACE', 'LINK', 'npm_config_cache']);
});

test('verbose proof goes red if the actual command loses /Brepro or a locked flag', () => {
  const complete = 'cargo build --release --locked --offline --target x86_64-pc-windows-msvc -C link-arg=/Brepro';
  assert.doesNotThrow(() => assertVerboseCargoProof(complete));
  assert.throws(() => assertVerboseCargoProof(complete.replace('/Brepro', '/DEBUG')), /rustc \/Brepro/);
  assert.throws(() => assertVerboseCargoProof(complete.replace('--locked', '')), /locked mode/);
});
