// Required Windows integration: real Cargo, a dependency outside the build root, and native bytes.
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { createCargoBuild, assertVerboseCargoProof, verifyExtractedInputs, runningInputFiles,
  normalizeBuildText, WINDOWS_LOGICAL_RUST_FLAGS } from './build-windows-internal-repro.mjs';
import { prepareNativeCompiler, nativeVersionProof, nativeToolsProof, nativeBootstrapProof, nativeLifecycleProof } from './windows-compiler-native-fixture.mjs';
import { beneath, loaderObservedWindows, compilerStartupPolicy } from './windows-compiler-closure.mjs';

assert.equal(process.platform, 'win32', 'native repro gate requires Windows');
assert.equal(process.arch, 'x64', 'native repro gate requires x64');
function run(tool, args, options = {}) {
  const result = spawnSync(tool, args, { encoding: 'utf8', windowsHide: true, timeout: 120000, maxBuffer: 16 * 1024 * 1024, ...options });
  assert.equal(result.error, undefined, String(result.error));
  assert.equal(result.status, 0, `${tool} ${args.join(' ')}\n${result.stdout}\n${result.stderr}`);
  return `${result.stdout ?? ''}${result.stderr ?? ''}`;
}
const root = mkdtempSync(join(tmpdir(), 'aware vendor repro '));
// Only this fresh empty fixture changes its compression attribute. Two uncompressed
// compiler copies fit the native gate's budget and avoid repeated decompression on
// every mandatory byte check; source installations and retained evidence are untouched.
const hash = path => createHash('sha256').update(readFileSync(path)).digest('hex');
const records = [];
try {
  run(join(loaderObservedWindows().system32, 'compact.exe'), ['/U', root]);
  nativeBootstrapProof(root);
  nativeLifecycleProof(root);
  for (const side of ['a', 'b']) {
    const base = join(root, `builder ${side} Łódź 😀 with a supported long source location`), work = join(base, 'work'), source = join(work, 'source'), crateRoot = join(source, 'cli');
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
    const native = prepareNativeCompiler({ base, work, source, closure, side, run });
    const { compiler, privateClosure, audit } = native;
    const denied = compilerStartupPolicy(compiler).deniedImage;
    if (denied) {
      // Cargo owns this real build-script descendant. Its direct invocation ensures
      // startup denial is exercised even on hosts whose linker does not start it.
      writeFileSync(join(crateRoot, 'build.rs'), `fn main() { let status = std::process::Command::new(r#"${denied.path}"#).status().expect("private telemetry child"); assert_eq!(status.code().unwrap() as u32, ${denied.exitCode}u32); }\n`);
    }
    const { cargoVendor, env, args } = createCargoBuild({ compiler, workRoot: work, sourceRoot: source, cargoHome, cargoClosure: privateClosure, tempRoot });
    nativeVersionProof({ native, env });
    const config = args.flatMap((arg, index) => arg === '--config' ? [arg, args[index + 1]] : []);
    audit('cargo', ['generate-lockfile', '--manifest-path', join(crateRoot, 'Cargo.toml'), '--offline', ...config], env, 'cargo-lock');
    const verbose = audit('cargo', args, env, 'cargo-build');
    if (denied) {
      const proof = JSON.parse(readFileSync(join(native.evidence, 'cargo-build-audit.local.json'), 'utf8'));
      assert.ok(proof.processes.some(process => process.action === 'blocked-telemetry' && process.exitCode === denied.exitCode), 'real private telemetry descendant was denied and its exit observed');
    }
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
    for (const [index, spelling] of [join(cargoVendor, 'path-probe', 'src', 'lib.rs'), join(cargoVendor, 'path-probe', 'src', 'lib.rs').replaceAll('\\', '/')].entries()) {
      const probeSource = join(source, 'spelling.rs');
      writeFileSync(probeSource, `#[path=${JSON.stringify(spelling)}] mod dependency;\nfn main() {println!("{}", dependency::origin());}\n`);
      const output = join(work, 'spelling.exe');
      audit('rustc', [probeSource, '--crate-name', 'spelling_probe', '-O', '-C', 'debuginfo=0', '-C', `linker=${compiler.tools.link}`, ...flags, '-o', output], env, `rust-spelling-${index}`);
      assert.equal(run(output, [], { env }).trim().replaceAll('\\', '/'), '<cargo-vendor>/path-probe/src/lib.rs');
    }

    const badFlags = flags.filter(flag => !flag.endsWith('=<cargo-vendor>'));
    assert.equal(flags.length - badFlags.length, 2, 'mutation removes BOTH Windows vendor spellings');
    const badEnv = { ...env, CARGO_ENCODED_RUSTFLAGS: badFlags.join('\x1f'), CARGO_TARGET_DIR: join(work, 'cargo-target', 'mutated-target') };
    audit('cargo', args, badEnv, 'cargo-red-control');
    const badExecutable = join(work, 'cargo-target', 'mutated-target', 'x86_64-pc-windows-msvc', 'release', 'vendor-repro-probe.exe');
    const badOrigin = run(badExecutable, [], { env: badEnv }).trim().replaceAll('\\', '/');
    assert.equal(badOrigin, '<work>/cargo-closure/vendor/path-probe/src/lib.rs', 'mutation exposes only the broad work remap');
    assert.notEqual(hash(badExecutable), goodHash, 'mutation changes the executable');
    const cHash = nativeToolsProof({ native, env, run });
    records.push({ side, origin, goodHash, badOrigin, badHash: hash(badExecutable), cHash });
    native.finish(records.at(-1));
    assert.ok(beneath(base, root) && resolve(base) !== resolve(root));
    rmSync(base, { recursive: true, force: true });
  }
  assert.equal(records[0].goodHash, records[1].goodHash, 'independent Cargo builds are byte-identical');
  assert.equal(records[0].cHash, records[1].cHash, 'independent native C executables are byte-identical');
  // The private vendor now lives under work: removing its specific remaps changes both
  // executables to the same wrong logical path, rather than exposing the original cache root.
  for (const record of records) assert.notEqual(record.badHash, record.goodHash);

  // Real Git bundle materialization must not let an old runner build a different source script.
  const repo = join(root, 'source authority'), extracted = join(root, 'extracted source');
  mkdirSync(join(repo, 'cli'), { recursive: true }); mkdirSync(join(repo, 'cli-connection-reader'));
  const oldRoot = join(root, 'old source'); mkdirSync(join(oldRoot, 'cli'), { recursive: true }); mkdirSync(join(oldRoot, 'cli-connection-reader'));
  const script = join(repo, 'cli', 'build-windows-internal-repro.mjs'), running = join(oldRoot, 'cli', 'build-windows-internal-repro.mjs');
  for (const builder of [script, running]) for (const [id, path] of Object.entries(runningInputFiles(builder))) writeFileSync(path, id);
  writeFileSync(join(repo, 'cli', 'Cargo.lock'), 'lock'); writeFileSync(join(repo, 'cli-connection-reader', 'package-lock.json'), '{}');
  const git = 'git';
  run(git, ['init', repo]); run(git, ['-C', repo, 'add', '.']);
  const commitArgs = ['-c', 'user.name=Repro fixture', '-c', 'user.email=repro@example.invalid', '-c', 'commit.gpgsign=false', '-c', 'core.hooksPath=NUL', 'commit', '-qm'];
  run(git, ['-C', repo, ...commitArgs, 'old fixture']);
  const inputs = { 'aware-cargo-lock': hash(join(repo, 'cli', 'Cargo.lock')), 'reader-package-lock': hash(join(repo, 'cli-connection-reader', 'package-lock.json')),
    ...Object.fromEntries(Object.entries(runningInputFiles(script)).map(([id, path]) => [id, hash(path)])) };
  writeFileSync(script, 'new builder'); run(git, ['-C', repo, 'add', '.']); run(git, ['-C', repo, ...commitArgs, 'new fixture']);
  const bundle = join(root, 'source.bundle'); run(git, ['-C', repo, 'bundle', 'create', bundle, 'HEAD']); run(git, ['clone', bundle, extracted]);
  assert.throws(() => verifyExtractedInputs(extracted, inputs, running), /extracted source/);
  inputs['builder-script'] = hash(join(extracted, 'cli', 'build-windows-internal-repro.mjs'));
  assert.throws(() => verifyExtractedInputs(extracted, inputs, running), /extracted source/);
  console.log(`Windows Cargo vendor-path repro passed: 2 byte-identical executables, both path spellings, spaced paths, two red mutations, old-runner/new-bundle refusal. ${records[0].goodHash}`);
} finally {
  // This test owns the unique temporary directory it created above.
  if (!beneath(root, tmpdir()) || resolve(root) === resolve(tmpdir())) throw new Error('test cleanup escaped its temporary parent');
  rmSync(root, { recursive: true, force: true });
}
