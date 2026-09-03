// Required Windows integration: real Cargo, a dependency outside the build root, and native bytes.
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { createCargoBuild, assertVerboseCargoProof, verifyExtractedInputs,
  normalizeBuildText, WINDOWS_LOGICAL_RUST_FLAGS } from './build-windows-internal-repro.mjs';

assert.equal(process.platform, 'win32', 'native repro gate requires Windows');
assert.equal(process.arch, 'x64', 'native repro gate requires x64');
function run(tool, args, options = {}) {
  const result = spawnSync(tool, args, { encoding: 'utf8', windowsHide: true, timeout: 120000, maxBuffer: 16 * 1024 * 1024, ...options });
  assert.equal(result.error, undefined, String(result.error));
  assert.equal(result.status, 0, `${tool} ${args.join(' ')}\n${result.stdout}\n${result.stderr}`);
  return `${result.stdout ?? ''}${result.stderr ?? ''}`;
}
function hostLocator() {
  const rust = name => run('rustup', ['which', '--toolchain', '1.95.0', name]).trim();
  const msvc = name => run('where.exe', [name]).trim().split(/\r?\n/).find(path => existsSync(path));
  return { tools: { cargo: rust('cargo'), rustc: rust('rustc'), rustdoc: rust('rustdoc'), cl: msvc('cl.exe'), link: msvc('link.exe'), lib: msvc('lib.exe') },
    environment: Object.fromEntries(['PATH', 'INCLUDE', 'LIB', 'LIBPATH', 'SystemRoot', 'WINDIR', 'ComSpec', 'PATHEXT'].map(key => [key, process.env[key]])) };
}
const locator = process.env.AWARE_REPRO_TEST_LOCATOR
  ? JSON.parse(readFileSync(process.env.AWARE_REPRO_TEST_LOCATOR, 'utf8')) : hostLocator();
assert.match(run(locator.tools.rustc, ['--version']), /^rustc 1\.95\.0\b/);
assert.match(run(locator.tools.cargo, ['--version']), /^cargo 1\.95\.0\b/);
const root = mkdtempSync(join(tmpdir(), 'aware vendor repro '));
const hash = path => createHash('sha256').update(readFileSync(path)).digest('hex');
const records = [];
try {
  for (const side of ['a', 'b']) {
    const base = join(root, `builder ${side}`), work = join(base, 'work'), source = join(work, 'source'), crateRoot = join(source, 'cli');
    const closure = join(base, 'sealed cache'), vendor = join(closure, 'vendor'), dependency = join(vendor, 'path-probe');
    const cargoHome = join(work, 'cargo-home'), tempRoot = join(work, 'temp');
    for (const dir of [join(crateRoot, 'src'), join(dependency, 'src'), cargoHome, tempRoot]) mkdirSync(dir, { recursive: true });
    writeFileSync(join(dependency, 'Cargo.toml'), '[package]\nname="path-probe"\nversion="0.1.0"\nedition="2021"\n');
    writeFileSync(join(dependency, 'src', 'lib.rs'), 'pub fn origin() -> &\'static str { file!() }\n');
    writeFileSync(join(dependency, '.cargo-checksum.json'), JSON.stringify({ package: '0'.repeat(64), files: {
      'Cargo.toml': hash(join(dependency, 'Cargo.toml')), 'src/lib.rs': hash(join(dependency, 'src', 'lib.rs')),
    } }));
    writeFileSync(join(crateRoot, 'Cargo.toml'), '[package]\nname="vendor-repro-probe"\nversion="0.1.0"\nedition="2021"\n[dependencies]\npath-probe="=0.1.0"\n[profile.release]\ndebug=0\n');
    writeFileSync(join(crateRoot, 'src', 'main.rs'), 'fn main() { println!("{}", path_probe::origin()); }\n');
    const { cargoVendor, env, args } = createCargoBuild({ locator, workRoot: work, sourceRoot: source, cargoHome, cargoClosure: closure, tempRoot });
    const config = args.flatMap((arg, index) => arg === '--config' ? [arg, args[index + 1]] : []);
    run(locator.tools.cargo, ['generate-lockfile', '--manifest-path', join(crateRoot, 'Cargo.toml'), '--offline', ...config], { cwd: source, env });
    const verbose = run(locator.tools.cargo, args, { cwd: source, env });
    const flags = env.CARGO_ENCODED_RUSTFLAGS.split('\x1f');
    assert.equal(normalizeBuildText(flags.join(' '), [[work, '<work>'], [source, '<source>'],
      [cargoHome, '<cargo-home>'], [cargoVendor, '<cargo-vendor>']]), WINDOWS_LOGICAL_RUST_FLAGS,
    'the receipt records the complete logical compiler arguments');
    assertVerboseCargoProof(`${args.join(' ')}\n${verbose}`, flags);
    const executable = join(work, 'cargo-target', 'x86_64-pc-windows-msvc', 'release', 'vendor-repro-probe.exe');
    const origin = run(executable, [], { env }).trim().replaceAll('\\', '/');
    assert.equal(origin, '<cargo-vendor>/path-probe/src/lib.rs');
    const goodHash = hash(executable);

    // Exercise both argument spellings directly, in addition to Cargo's native dependency invocation.
    for (const spelling of [join(dependency, 'src', 'lib.rs'), join(dependency, 'src', 'lib.rs').replaceAll('\\', '/')]) {
      const probeSource = join(vendor, 'spelling.rs');
      writeFileSync(probeSource, `#[path=${JSON.stringify(spelling)}] mod dependency;\nfn main() {println!("{}", dependency::origin());}\n`);
      const output = join(work, 'spelling.exe');
      run(locator.tools.rustc, [probeSource, '--crate-name', 'spelling_probe', '-O', '-C', 'debuginfo=0', '-C', `linker=${locator.tools.link}`, ...flags, '-o', output], { env });
      assert.equal(run(output, [], { env }).trim().replaceAll('\\', '/'), '<cargo-vendor>/path-probe/src/lib.rs');
    }

    const badFlags = flags.filter(flag => !flag.endsWith('=<cargo-vendor>'));
    assert.equal(flags.length - badFlags.length, 2, 'mutation removes BOTH Windows vendor spellings');
    const badEnv = { ...env, CARGO_ENCODED_RUSTFLAGS: badFlags.join('\x1f'), CARGO_TARGET_DIR: join(work, 'mutated-target') };
    run(locator.tools.cargo, args, { cwd: source, env: badEnv });
    const badExecutable = join(work, 'mutated-target', 'x86_64-pc-windows-msvc', 'release', 'vendor-repro-probe.exe');
    const badOrigin = run(badExecutable, [], { env: badEnv }).trim().replaceAll('\\', '/');
    assert.ok(badOrigin.includes(cargoVendor.replaceAll('\\', '/')), 'mutation exposes the physical vendor path');
    assert.notEqual(hash(badExecutable), goodHash, 'mutation changes the executable');
    records.push({ side, origin, goodHash, badOrigin, badHash: hash(badExecutable) });
  }
  assert.equal(records[0].goodHash, records[1].goodHash, 'independent Cargo builds are byte-identical');
  assert.notEqual(records[0].badHash, records[1].badHash, 'removing vendor maps breaks byte equality');

  // Real Git bundle materialization must not let an old runner build a different source script.
  const repo = join(root, 'source authority'), extracted = join(root, 'extracted source');
  mkdirSync(join(repo, 'cli'), { recursive: true }); mkdirSync(join(repo, 'cli-connection-reader'));
  const script = join(repo, 'cli', 'build-windows-internal-repro.mjs'), running = join(root, 'old-runner.mjs');
  writeFileSync(script, 'old builder'); writeFileSync(running, 'old builder');
  writeFileSync(join(repo, 'cli', 'Cargo.lock'), 'lock'); writeFileSync(join(repo, 'cli-connection-reader', 'package-lock.json'), '{}');
  const git = locator.tools.git ?? 'git';
  run(git, ['init', repo]); run(git, ['-C', repo, 'add', '.']);
  const commitArgs = ['-c', 'user.name=Repro fixture', '-c', 'user.email=repro@example.invalid', '-c', 'commit.gpgsign=false', '-c', 'core.hooksPath=NUL', 'commit', '-qm'];
  run(git, ['-C', repo, ...commitArgs, 'old fixture']);
  const inputs = { 'aware-cargo-lock': hash(join(repo, 'cli', 'Cargo.lock')), 'reader-package-lock': hash(join(repo, 'cli-connection-reader', 'package-lock.json')), 'builder-script': hash(script) };
  writeFileSync(script, 'new builder'); run(git, ['-C', repo, 'add', '.']); run(git, ['-C', repo, ...commitArgs, 'new fixture']);
  const bundle = join(root, 'source.bundle'); run(git, ['-C', repo, 'bundle', 'create', bundle, 'HEAD']); run(git, ['clone', bundle, extracted]);
  assert.throws(() => verifyExtractedInputs(extracted, inputs, running), /extracted source/);
  inputs['builder-script'] = hash(join(extracted, 'cli', 'build-windows-internal-repro.mjs'));
  assert.throws(() => verifyExtractedInputs(extracted, inputs, running), /extracted source/);
  console.log(`Windows Cargo vendor-path repro passed: 2 byte-identical executables, both path spellings, spaced paths, two red mutations, old-runner/new-bundle refusal. ${records[0].goodHash}`);
} finally {
  // This test owns the unique temporary directory it created above.
  if (!resolve(root).startsWith(resolve(tmpdir()))) throw new Error('test cleanup escaped its temporary parent');
  rmSync(root, { recursive: true, force: true });
}
