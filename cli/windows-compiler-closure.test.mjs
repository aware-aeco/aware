import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, rmSync, symlinkSync, writeFileSync, existsSync } from 'node:fs';
import { dirname, join } from 'node:path';
import test from 'node:test';
import { compilerFixture, COMPILER_FIXTURE_FILES } from './windows-compiler-fixture.mjs';
import { createWindowsBuilderRecords } from './create-windows-internal-repro-inputs.mjs';
import { verifyNativeIncludes, verifyNativeLinkInputs } from './windows-compiler-native-fixture.mjs';
import { COMPILER_IDS, COMPILER_LAYOUT, COMPILER_DESCRIPTOR, compilerStartupPolicy, canonicalJson, digest, fileDigest, inventory, copyDirectory, loaderObservedWindows, verifyCompilerAudit,
  protectedWindowsPath, retainAuditorResult, runAuditedCompiler, validateCompilerLocator, validateCompilerManifest, validateInventory, validateRecordPath, validateWindowsPath } from './windows-compiler-closure.mjs';
import { loadVerifiedBuildModules, runningInputFiles, rejectedAmbientKeys, validateBootstrapLocator, verifyBuildAuthority,
  bootstrapSystemEnvironment, verifyConsumedClosure } from './build-windows-internal-repro.mjs';

test('structured native include proof preserves Unicode and refuses foreign or missing inputs', () => {
  const source = 'C:\\build Łódź 😀\\native.c', roots = ['C:\\private Łódź 😀\\sdk', 'C:\\private Łódź 😀\\msvc'];
  const report = { Version: '1.2', Data: { Source: source.toLowerCase(), ProvidedModule: '', Includes: [roots[0] + '\\um\\windows.h', roots[1] + '\\stdio.h'] } };
  assert.equal(verifyNativeIncludes(report, source, roots).length, 2);
  for (const modify of [
    copy => { copy.Version = 'unknown'; },
    copy => { copy.Data.Source = 'C:\\foreign\\native.c'; },
    copy => { copy.Data.Includes = []; },
    copy => { copy.Data.Includes.pop(); },
    copy => { copy.Data.Includes.push('C:\\private Łódź 😀\\sdk-other\\foreign.h'); },
    copy => { copy.Data.Includes.push('C:\\private Łódź 😀\\sdk\\..\\foreign.h'); },
    copy => { copy.Data.Includes.push('relative.h'); },
  ]) { const changed = structuredClone(report); modify(changed); assert.throws(() => verifyNativeIncludes(changed, source, roots)); }
});

test('auditor diagnostics retain real nonzero, startup and timeout results before throwing', () => {
  const fixture = compilerFixture();
  try {
    const samples = [
      ['exit', spawnSync(process.execPath, ['-e', 'process.stdout.write(Buffer.from([0,255,1]));process.stderr.write(Buffer.from([254,0,2]));process.exitCode=7'], { encoding: null, windowsHide: true }), undefined],
      ['startup', spawnSync(join(fixture.root, 'missing-executable'), [], { encoding: null, windowsHide: true }), 'ENOENT'],
      ['timeout', spawnSync(process.execPath, ['-e', 'setTimeout(()=>{},30000)'], { encoding: null, windowsHide: true, timeout: 100 }), 'ETIMEDOUT'],
    ];
    for (const [label, result, code] of samples) {
      if (code) assert.equal(result.error?.code, code); else assert.equal(result.status, 7);
      const prefix = join(fixture.root, label), request = `${prefix}-request.local.json`;
      writeFileSync(request, '{}', { flag: 'wx' });
      assert.throws(() => retainAuditorResult(request, result, 100), error => {
        assert.match(error.message, /compiler auditor failed/);
        assert.equal(error.cause, result.error); return true;
      });
      assert.deepEqual(readFileSync(`${prefix}-stdout.local.bin`), result.stdout ?? Buffer.alloc(0));
      assert.deepEqual(readFileSync(`${prefix}-stderr.local.bin`), result.stderr ?? Buffer.alloc(0));
      const launch = JSON.parse(readFileSync(`${prefix}-launch.local.json`, 'utf8'));
      assert.equal(launch.error?.code, code); assert.equal(launch.status, result.status ?? null);
      assert.equal(launch.signal, result.signal ?? null); assert.equal(launch.timeoutMs, 100);
    }
    const result = samples[0][1], prefix = join(fixture.root, 'collision');
    writeFileSync(`${prefix}-stdout.local.bin`, 'preserve');
    assert.throws(() => retainAuditorResult(`${prefix}-request.local.json`, result, 100), error => {
      assert.ok(error instanceof AggregateError); assert.equal(error.errors.length, 2);
      assert.match(error.errors[0].message, /compiler auditor failed/); assert.equal(error.errors[1].code, 'EEXIST'); return true;
    });
    assert.equal(readFileSync(`${prefix}-stdout.local.bin`, 'utf8'), 'preserve');
    assert.deepEqual(readFileSync(`${prefix}-stderr.local.bin`), result.stderr);
    assert.ok(existsSync(`${prefix}-command.local.log`) && existsSync(`${prefix}-launch.local.json`));
  } finally { rmSync(fixture.root, { recursive: true, force: true }); }
});

test('native link capture binds every library and object to actual declared bytes', () => {
  const fixture = compilerFixture();
  try {
    const captured = join(fixture.root, 'link-repro'), object = join(fixture.root, 'native.obj');
    mkdirSync(captured); writeFileSync(object, 'actual object');
    for (const [name, content] of Object.entries({ 'native.obj': 'actual object', 'env.setting': 'LIB=private', 'link.rsp': 'native.obj', 'kernel32.lib': 'actual SDK', 'LIBCMT.lib': 'actual CRT' })) writeFileSync(join(captured, name), content);
    const files = inventory(captured);
    const compiler = { manifest: { closures: { 'compiler-msvc-lib': { files: files.filter(file => file.path === 'LIBCMT.lib') }, 'compiler-sdk-um-lib': { files: files.filter(file => file.path === 'kernel32.lib') }, 'compiler-sdk-ucrt-lib': { files: [] } } } };
    assert.equal(verifyNativeLinkInputs(captured, compiler, object).length, 5);
    for (const name of ['native.obj', 'kernel32.lib', 'LIBCMT.lib']) {
      const path = join(captured, name), original = readFileSync(path);
      writeFileSync(path, 'mutated'); assert.throws(() => verifyNativeLinkInputs(captured, compiler, object)); writeFileSync(path, original);
      rmSync(path); assert.throws(() => verifyNativeLinkInputs(captured, compiler, object)); writeFileSync(path, original);
    }
    for (const name of ['foreign.lib', 'foreign.obj']) {
      const path = join(captured, name); writeFileSync(path, 'unbound');
      assert.throws(() => verifyNativeLinkInputs(captured, compiler, object), /unbound/); rmSync(path);
    }
  } finally { rmSync(fixture.root, { recursive: true, force: true }); }
});

test('compiler authority is independent of physical source locations', () => {
  const a = compilerFixture(), b = compilerFixture();
  try {
    const first = createWindowsBuilderRecords(a.input), second = createWindowsBuilderRecords(b.input);
    assert.equal(first.manifestText, second.manifestText); assert.equal(first.buildId, second.buildId);
    assert.equal(Object.keys(first.manifest.closures).length, 11);
    assert.equal(Object.keys(first.manifest.inputs).length, 6);
    assert.equal('environment' in first.locator, false);
    assert.equal('cargo' in first.locator.tools, false);
    assert.doesNotThrow(() => validateCompilerManifest(first.manifest));
  } finally { rmSync(a.root, { recursive: true, force: true }); rmSync(b.root, { recursive: true, force: true }); }
});

test('private copies preserve real Unicode names and are independent of original bytes', () => {
  const fixture = compilerFixture();
  try {
    const source = fixture.input.closures['compiler-msvc-include'], destination = join(fixture.root, 'Łódź with spaces', 'private');
    writeFileSync(join(source, 'żółć.h'), 'Unicode header'); copyDirectory(source, destination);
    assert.equal(readFileSync(join(destination, 'żółć.h'), 'utf8'), 'Unicode header');
    assert.equal(canonicalJson(inventory(source)), canonicalJson(inventory(destination)));
    writeFileSync(join(source, 'żółć.h'), 'changed original');
    assert.equal(readFileSync(join(destination, 'żółć.h'), 'utf8'), 'Unicode header');
    assert.throws(() => copyDirectory(source, destination), /fresh/);
  } finally { rmSync(fixture.root, { recursive: true, force: true }); }
});

test('self-consistent manifests cannot drop mandatory compiler inputs', () => {
  const fixture = compilerFixture();
  try {
    const original = createWindowsBuilderRecords(fixture.input).manifest;
    for (const id of COMPILER_IDS) for (const path of COMPILER_FIXTURE_FILES[id]) {
      const changed = structuredClone(original);
      changed.closures[id].files = changed.closures[id].files.filter(file => file.path !== path);
      assert.throws(() => validateCompilerManifest(changed), /mandatory compiler support|nonempty/, `${id}/${path}`);
    }
    const shims = structuredClone(original);
    for (const file of shims.closures['compiler-rust-bin'].files.filter(file => /^(cargo|rustc|rustdoc)\.exe$/.test(file.path))) file.sha256 = 'e'.repeat(64);
    assert.throws(() => validateCompilerManifest(shims), /rustup shims/);
    const reordered = structuredClone(original); reordered.compiler.environment.LIB.reverse();
    assert.throws(() => validateCompilerManifest(reordered), /descriptor differs/);
    const oldPolicy = structuredClone(original); oldPolicy.compiler.auditPolicy = 'aware-private-compiler-debug-events/v2';
    assert.throws(() => validateCompilerManifest(oldPolicy), /descriptor differs/);
  } finally { rmSync(fixture.root, { recursive: true, force: true }); }
});

test('portable records reject unsafe aliases, case collisions and unsorted input', () => {
  for (const path of ['../x', 'a/../x', '/x', 'a//x', 'a\\x', 'c:x', 'x.', 'x ', 'NUL.exe', 'a/COM1', 'a?b', 'a*b', 'a<b', 'a|b', 'a"b']) assert.throws(() => validateRecordPath(path), /unsafe/);
  const records = [{ path: 'A.h', size: 1, sha256: 'a'.repeat(64) }, { path: 'a.h', size: 1, sha256: 'b'.repeat(64) }];
  assert.throws(() => validateInventory(records, 'fixture'), /case-colliding/);
  assert.throws(() => validateInventory([{ ...records[0], path: 'z.h' }, { ...records[1], path: 'b.h' }], 'fixture'), /not sorted/);
});

test('physical compiler roots have explicit Windows syntax and length limits', () => {
  for (const root of ['C:\\build with spaces\\żółć', `D:\\${'x'.repeat(180)}`]) assert.doesNotThrow(() => validateWindowsPath(root));
  for (const root of ['C:relative', '\\\\server\\share', '\\\\?\\C:\\work', 'C:\\bad;path', 'C:\\bad=path', 'C:\\bad\npath', 'C:\\bad.', 'C:\\..\\escape', `C:\\${'x'.repeat(201)}`]) assert.throws(() => validateWindowsPath(root));
});

test('compiler locator refuses environment authority and redundant compiler paths', () => {
  const fixture = compilerFixture();
  try {
    const locator = createWindowsBuilderRecords(fixture.input).locator;
    assert.throws(() => validateCompilerLocator({ ...locator, environment: { PATH: 'C:\\hostile' } }), /unexpected or missing keys/);
    assert.throws(() => validateCompilerLocator({ ...locator, tools: { ...locator.tools, cargo: 'C:\\hostile\\cargo.exe' } }), /unexpected or missing keys/);
    assert.throws(() => createWindowsBuilderRecords({ ...fixture.input, environment: {} }), /unexpected or missing keys/);
  } finally { rmSync(fixture.root, { recursive: true, force: true }); }
});

test('every locator path is rejected syntactically before any filesystem lookup', () => {
  const fixture = compilerFixture();
  try {
    const { manifest } = createWindowsBuilderRecords(fixture.input);
    const locator = { schema: 'aware-windows-repro-locator/v1', sourceBundle: 'C:/unread/source.bundle',
      tools: Object.fromEntries(Object.keys(fixture.input.tools).map(id => [id, `C:/unread/${id}`])),
      closures: Object.fromEntries(Object.keys(fixture.input.closures).map(id => [id, `C:/unread/${id}`])) };
    for (const family of ['tools', 'closures']) for (const id of Object.keys(locator[family])) {
      const bad = structuredClone(locator); bad[family][id] = '\\\\must-not-contact.invalid\\share';
      assert.throws(() => verifyBuildAuthority({ manifest, locator: bad, env: {} }), /unsafe bootstrap/);
    }
    assert.throws(() => verifyBuildAuthority({ manifest, locator: { ...locator, sourceBundle: '\\\\?\\C:\\device' }, env: {} }), /unsafe bootstrap/);
    assert.doesNotThrow(() => validateBootstrapLocator(locator));
    const env = bootstrapSystemEnvironment('C:/temp', ['C:\\Windows\\System32\\kernel32.dll', 'C:\\Windows\\System32\\ntdll.dll']);
    assert.equal(env.SystemRoot, 'c:\\windows');
  } finally { rmSync(fixture.root, { recursive: true, force: true }); }
});

test('private npm cache mutation is refused at the post-consumption boundary', () => {
  const fixture = compilerFixture();
  try {
    const { manifest } = createWindowsBuilderRecords(fixture.input), root = fixture.input.closures['npm-cache'];
    assert.equal(verifyConsumedClosure('npm-cache', root, manifest, inventory), root);
    writeFileSync(join(root, 'new-npm-state'), 'mutation');
    assert.throws(() => verifyConsumedClosure('npm-cache', root, manifest, inventory), /inventory mismatch/);
  } finally { rmSync(fixture.root, { recursive: true, force: true }); }
});

test('ambient compiler overrides are rejected case-insensitively', () => {
  const poisoned = { Path: 'x', RustFlags: 'x', _cl_: 'x', RuStUp_HoMe: 'x', cArGo_HoMe: 'x', CC_x86_64_pc_windows_msvc: 'x', WindowsSdkDir: 'x', VCToolsInstallDir: 'x', vcInstallDir: 'x', vScMd_ArG_TgT_ArCh: 'x64', _nO_dEbUg_HeAp: '1' };
  assert.deepEqual(rejectedAmbientKeys(poisoned), Object.keys(poisoned).sort());
  assert.deepEqual(rejectedAmbientKeys({ SystemRoot: 'a', systemroot: 'b' }), ['SystemRoot', 'systemroot']);
});

test('compiler heap setting is fixed manifest authority and rejects alternate spellings', () => {
  const fixture = compilerFixture();
  try {
    const { manifest } = createWindowsBuilderRecords(fixture.input);
    assert.equal(manifest.compiler.environment._NO_DEBUG_HEAP, '1');
    for (const value of [undefined, '0', 1]) {
      const changed = structuredClone(manifest);
      if (value === undefined) delete changed.compiler.environment._NO_DEBUG_HEAP;
      else changed.compiler.environment._NO_DEBUG_HEAP = value;
      assert.throws(() => validateCompilerManifest(changed), /descriptor differs/);
    }
    for (const duplicate of [false, true]) {
      const changed = structuredClone(manifest);
      changed.compiler.environment._no_debug_heap = '1';
      if (!duplicate) delete changed.compiler.environment._NO_DEBUG_HEAP;
      assert.throws(() => validateCompilerManifest(changed), /descriptor differs/);
    }
  } finally { rmSync(fixture.root, { recursive: true, force: true }); }
});

test('auditor rejects missing, altered or ambiguous heap mode before filesystem access', () => {
  for (const env of [undefined, {}, { _NO_DEBUG_HEAP: '0' }, { _NO_DEBUG_HEAP: 1 },
    { _no_debug_heap: '1' }, { _NO_DEBUG_HEAP: '1', _no_debug_heap: '0' }]) {
    // No compiler or paths exist: a bypass would reach private-compiler verification and fail differently.
    assert.throws(() => runAuditedCompiler({ env }), /fixed _NO_DEBUG_HEAP=1/);
  }
});

test('MSVC discovery markers are fixed descriptor authority, never host inputs', () => {
  const fixture = compilerFixture();
  try {
    const { manifest } = createWindowsBuilderRecords(fixture.input);
    assert.deepEqual(manifest.compiler.environment.VCINSTALLDIR, ['msvc']);
    assert.equal(manifest.compiler.environment.VSCMD_ARG_TGT_ARCH, 'x64');
    for (const key of ['VCINSTALLDIR', 'VSCMD_ARG_TGT_ARCH']) {
      const missing = structuredClone(manifest); delete missing.compiler.environment[key];
      assert.throws(() => validateCompilerManifest(missing), /descriptor differs/);
      const changed = structuredClone(manifest); changed.compiler.environment[key] = key === 'VCINSTALLDIR' ? ['C:/hostile'] : 'x86';
      assert.throws(() => validateCompilerManifest(changed), /descriptor differs/);
    }
  } finally { rmSync(fixture.root, { recursive: true, force: true }); }
});

test('inventory rejects redirected roots and nested directories on the real filesystem', () => {
  const fixture = compilerFixture();
  try {
    const redirected = join(fixture.root, 'redirected');
    symlinkSync(fixture.input.closures['compiler-msvc-include'], redirected, process.platform === 'win32' ? 'junction' : 'dir');
    assert.throws(() => inventory(redirected), /real directory/);
    const nested = join(fixture.input.closures['compiler-rust-bin'], 'redirected');
    symlinkSync(fixture.input.closures['compiler-msvc-include'], nested, process.platform === 'win32' ? 'junction' : 'dir');
    assert.throws(() => inventory(fixture.input.closures['compiler-rust-bin']), /path-redirection/);
  } finally { rmSync(fixture.root, { recursive: true, force: true }); }
});

test('a wrong-hash helper is refused before top-level code can execute', async () => {
  const fixture = compilerFixture();
  try {
    const builder = join(fixture.root, 'running', 'cli', 'build-windows-internal-repro.mjs');
    const files = runningInputFiles(builder), marker = join(fixture.root, 'executed-marker');
    for (const [id, path] of Object.entries(files)) { mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, id); }
    const inputs = Object.fromEntries(Object.entries(files).map(([id, path]) => [id, fileDigest(path)]));
    writeFileSync(files['compiler-closure-script'], `import {writeFileSync} from 'node:fs'; writeFileSync(${JSON.stringify(marker)}, 'executed');`);
    await assert.rejects(loadVerifiedBuildModules(inputs, files), /running compiler-closure-script differs/);
    assert.equal(existsSync(marker), false);
  } finally { rmSync(fixture.root, { recursive: true, force: true }); }
});

test('manifest-valid helpers cannot execute before matching the extracted source', async () => {
  const fixture = compilerFixture();
  try {
    const source = join(fixture.root, 'extracted'), running = join(fixture.root, 'runner', 'cli', 'build-windows-internal-repro.mjs');
    const files = runningInputFiles(running), extracted = runningInputFiles(join(source, 'cli', 'build-windows-internal-repro.mjs'));
    const marker = join(fixture.root, 'source-mismatch-executed');
    for (const map of [files, extracted]) for (const [id, path] of Object.entries(map)) { mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, id); }
    writeFileSync(files['compiler-closure-script'], `import {writeFileSync} from 'node:fs'; writeFileSync(${JSON.stringify(marker)}, 'executed');`);
    writeFileSync(join(source, 'cli', 'Cargo.lock'), 'lock'); writeFileSync(join(source, 'cli-connection-reader', 'package-lock.json'), '{}');
    const inputs = { 'aware-cargo-lock': fileDigest(join(source, 'cli', 'Cargo.lock')), 'reader-package-lock': fileDigest(join(source, 'cli-connection-reader', 'package-lock.json')),
      ...Object.fromEntries(Object.entries(files).map(([id, path]) => [id, fileDigest(path)])) };
    await assert.rejects(loadVerifiedBuildModules(inputs, files, source), /extracted source/); assert.equal(existsSync(marker), false);
    // Source-equal minimal modules are evaluated from their authenticated bytes.
    for (const map of [files, extracted]) {
      writeFileSync(map['compiler-closure-script'], 'export const authenticated = true;');
      writeFileSync(map['reader-settings-script'], 'export const READER_BUILD_SETTINGS = {authenticated:true};');
    }
    for (const [id, path] of Object.entries(files)) inputs[id] = fileDigest(path);
    const loaded = await loadVerifiedBuildModules(inputs, files, source);
    assert.equal(loaded.compiler.authenticated, true); assert.equal(loaded.settings.authenticated, true);
  } finally { rmSync(fixture.root, { recursive: true, force: true }); }
});

test('OS discovery uses loaded Windows modules and excludes writable Windows paths', () => {
  const host = loaderObservedWindows(['C:\\Windows\\System32\\KERNEL32.DLL', 'C:\\Windows\\SYSTEM32\\ntdll.dll']);
  assert.equal(host.powershell.toLowerCase(), 'c:\\windows\\system32\\windowspowershell\\v1.0\\powershell.exe');
  assert.throws(() => loaderObservedWindows(['C:\\fake\\kernel32.dll', 'C:\\Windows\\System32\\ntdll.dll']), /disagree/);
  // Native filesystem path semantics here; protectedWindowsPath never accepts a Windows-root prefix alone.
  const nativeHost = { windows: join(process.cwd(), 'Windows'), system32: join(process.cwd(), 'Windows', 'System32') };
  assert.equal(protectedWindowsPath(join(nativeHost.system32, 'kernel32.dll'), nativeHost), true);
  assert.equal(protectedWindowsPath(join(nativeHost.windows, 'Temp', 'evil.dll'), nativeHost), false);
  assert.equal(protectedWindowsPath(join(nativeHost.system32, 'Temp', 'evil.dll'), nativeHost), false);
  assert.equal(protectedWindowsPath(join(nativeHost.windows, 'Microsoft.NET', 'Framework64', 'v4.0.30319', 'clr.dll'), nativeHost), true);
});

test('compiler audit requires complete bound process/image evidence and the requested root tool', () => {
  const fixture = compilerFixture();
  try {
    writeFileSync(join(fixture.input.closures['compiler-msvc-bin'], 'vctip.exe'), 'private telemetry image');
    const manifest = createWindowsBuilderRecords(fixture.input).manifest;
    const roots = Object.fromEntries(COMPILER_IDS.map(id => [id, join('C:/private/compiler', COMPILER_LAYOUT[id])]));
    const toolPath = join(roots['compiler-rust-bin'], 'rustc.exe');
    const rustc = manifest.closures['compiler-rust-bin'].files.find(file => file.path === 'rustc.exe');
    const compiler = { manifest, roots, host: { windows: 'C:/Windows', system32: 'C:/Windows/System32' } };
    const options = { compiler, toolPath, targetRoot: 'C:/private/cargo-target' };
    const report = { schema: 'aware-compiler-debug-audit/v3', eventCount: 10, startupPolicy: compilerStartupPolicy(compiler), error: null, complete: true, exitCode: 0, totalProcesses: 1, activeProcesses: 0,
      processes: [{ pid: 42, instance: 1, startEvent: 1, exitEvent: 10, path: toolPath, exitCode: 0, action: 'observed' }],
      images: [{ pid: 42, instance: 1, event: 1, path: toolPath, kind: 'process', sha256: rustc.sha256, size: rustc.size }],
      identity: { source: manifest.source, buildId: digest(canonicalJson(manifest)), auditScriptSha256: manifest.inputs['compiler-audit-script'] } };
    assert.equal(verifyCompilerAudit(report, options).images[0].role, 'compiler-rust-bin');
    const omittedPolicy = structuredClone(report); delete omittedPolicy.startupPolicy;
    assert.throws(() => verifyCompilerAudit(omittedPolicy, options), /missing keys/);
    for (const policy of [{ ...report.startupPolicy, deniedImage: null }, { ...report.startupPolicy, identity: 'unarmed' }]) {
      assert.throws(() => verifyCompilerAudit({ ...report, startupPolicy: policy }, options), /startup policy differs/);
    }
    const denied = report.startupPolicy.deniedImage;
    const blocked = { ...structuredClone(report), totalProcesses: 2,
      processes: [...report.processes, { pid: 43, instance: 2, startEvent: 3, exitEvent: 7, path: denied.path, exitCode: denied.exitCode, action: 'blocked-telemetry' }],
      images: [...report.images, { pid: 43, instance: 2, event: 3, path: denied.path, kind: 'process', size: denied.size, sha256: denied.sha256 }] };
    assert.doesNotThrow(() => verifyCompilerAudit(blocked, options));
    for (const mutate of [
      r => { r.processes[1].action = 'observed'; }, r => { r.processes[1].exitCode = 0; },
      r => { r.images[1].sha256 = 'f'.repeat(64); }, r => { r.images[1].size++; },
      r => { r.processes[1].path = r.images[1].path = join(roots['compiler-msvc-bin'], 'link.exe'); },
      r => { r.processes.pop(); }, r => { r.images.pop(); }, r => { r.activeProcesses = 1; },
      r => { r.processes[0].action = 'blocked-telemetry'; },
    ]) { const changed = structuredClone(blocked); mutate(changed); assert.throws(() => verifyCompilerAudit(changed, options)); }
    const noTelemetryCompiler = structuredClone(compiler);
    noTelemetryCompiler.manifest.closures['compiler-msvc-bin'].files = noTelemetryCompiler.manifest.closures['compiler-msvc-bin'].files.filter(file => file.path !== 'vctip.exe');
    assert.equal(compilerStartupPolicy(noTelemetryCompiler).deniedImage, null);
    for (const changed of [{ ...report, complete: false }, { ...report, error: 'lost event' }, { ...report, activeProcesses: 1 },
      { ...report, totalProcesses: 2 }, { ...report, processes: [{ ...report.processes[0], exitCode: null }] },
      { ...report, images: [{ ...report.images[0], sha256: 'f'.repeat(64) }] }]) assert.throws(() => verifyCompilerAudit(changed, options));
    assert.throws(() => verifyCompilerAudit(report, { ...options, toolPath: join(roots['compiler-rust-bin'], 'cargo.exe') }), /requested tool/);
    const outside = { ...report, images: [...report.images, { ...report.images[0], event: 2, kind: 'dll', path: 'C:/Windows/Temp/evil.dll' }] };
    assert.throws(() => verifyCompilerAudit(outside, options), /outside its authority/);
    const missingHash = { ...report, images: [...report.images, { ...report.images[0], event: 2, kind: 'dll', path: 'C:/Windows/System32/kernel32.dll', sha256: '' }] };
    assert.throws(() => verifyCompilerAudit(missingHash, options), /unhashed/);

    // The root exits before its descendants. Its numeric PID can then reappear
    // as a different compiler lifetime without changing the root's exit status.
    const reused = structuredClone(report);
    reused.eventCount = 30; reused.totalProcesses = 3; reused.processes[0].exitEvent = 5;
    reused.processes.push(
      { ...report.processes[0], instance: 2, pid: 480, startEvent: 3, exitEvent: 30, exitCode: 7 },
      { ...report.processes[0], instance: 3, startEvent: 8, exitEvent: 20, exitCode: 9 });
    reused.images.push(
      { ...report.images[0], instance: 2, pid: 480, event: 3 },
      { ...report.images[0], instance: 2, pid: 480, event: 4, kind: 'dll', path: 'C:/Windows/System32/kernel32.dll' },
      { ...report.images[0], instance: 3, event: 8 },
      { ...report.images[0], instance: 3, event: 9, kind: 'dll', path: 'C:/Windows/System32/kernel32.dll' });
    assert.equal(verifyCompilerAudit(reused, options).processes[2].exitCode, 9);
    for (const mutate of [
      r => { r.schema = 'aware-compiler-debug-audit/v2'; },
      r => { r.processes[2].instance = 2; },
      r => { r.processes[2].startEvent = r.images[3].event = 4; },
      r => { r.processes[2].exitEvent = null; },
      r => { r.processes[2].exitEvent = r.processes[2].startEvent; },
      r => { r.processes[2].startEvent = 31; },
      r => { r.eventCount = 31; },
      r => { r.eventCount = 29; },
      r => { r.images[4].instance = 1; },
      r => { r.images[4].pid = 480; },
      r => { r.images[4].instance = 4; },
      r => { r.images[4].event = 20; },
      r => { r.images[4].event = 21; },
      r => { r.images[4].event = 7; },
      r => { r.images[3].event = 9; r.images[4].event = 10; },
      r => { r.images[3].path = 'C:/Windows/System32/kernel32.dll'; },
      r => { r.images[4].kind = 'process'; },
      r => { r.images.push({ ...r.images[4], event: 11, kind: 'process' }); },
      r => { r.images[2].event = 5; },
      r => { r.images.reverse(); },
      r => { r.processes.reverse(); },
      r => { r.images.splice(3, 1); },
      r => { r.exitCode = 9; },
    ]) { const changed = structuredClone(reused); mutate(changed); assert.throws(() => verifyCompilerAudit(changed, options)); }
  } finally { rmSync(fixture.root, { recursive: true, force: true }); }
});
