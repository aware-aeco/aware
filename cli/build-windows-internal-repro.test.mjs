import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';
import {
  assertVerboseCargoProof, cargoArguments, closedGitEnvironment, controlledEnvironment, rejectedAmbientKeys,
  writeBuilderManifestEvidence,
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

test('Git cannot consult host configuration, templates, prompts, or network transports', () => {
  const env = closedGitEnvironment({ SystemRoot: 'WINDOWS' });
  assert.equal(env.GIT_CONFIG_NOSYSTEM, '1');
  assert.equal(env.GIT_CONFIG_GLOBAL, 'NUL');
  assert.equal(env.GIT_CONFIG_COUNT, '0');
  assert.equal(env.GIT_ALLOW_PROTOCOL, 'file');
  assert.equal(env.GIT_TERMINAL_PROMPT, '0');
});

test('verbose proof goes red if the actual command loses /Brepro or a locked flag', () => {
  const complete = 'cargo build --release --locked --offline --target x86_64-pc-windows-msvc -C link-arg=/Brepro';
  assert.doesNotThrow(() => assertVerboseCargoProof(complete));
  assert.throws(() => assertVerboseCargoProof(complete.replace('/Brepro', '/DEBUG')), /rustc \/Brepro/);
  assert.throws(() => assertVerboseCargoProof(complete.replace('--locked', '')), /locked mode/);
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
