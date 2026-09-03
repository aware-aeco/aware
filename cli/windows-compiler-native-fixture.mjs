// Native test support; all compiler execution uses the production closure and auditor helpers.
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, renameSync, writeFileSync } from 'node:fs';
import { basename, dirname, join, resolve, win32 } from 'node:path';
import { runningInputFiles, materializeClosure } from './build-windows-internal-repro.mjs';
import { COMPILER_IDS, INPUT_IDS, NONCOMPILER_TOOL_IDS, inventory, copyDirectory, fileDigest, canonicalJson, discoverSystemHost,
  loaderObservedWindows, systemEnvironment, auditorTempParent, exactKeys, materializeCompiler, verifyPrivateCompiler, validateCompilerLocator, runAuditedCompiler, beneath } from './windows-compiler-closure.mjs';
import { createWindowsBuilderRecords } from './create-windows-internal-repro-inputs.mjs';

const windowsPath = path => win32.normalize(path).toLowerCase();
export function verifyNativeIncludes(document, source, roots) {
  exactKeys(document, ['Version', 'Data'], 'native include report');
  assert.equal(document.Version, '1.2', 'native include schema differs');
  exactKeys(document.Data, ['Source', 'ProvidedModule', 'Includes'], 'native include data');
  assert.equal(windowsPath(document.Data.Source), windowsPath(source), 'native include source differs');
  assert.equal(document.Data.ProvidedModule, '');
  const includes = document.Data.Includes;
  assert.ok(Array.isArray(includes) && includes.length > 0, 'native includes are missing');
  assert.ok(includes.some(path => /windows\.h$/i.test(path)) && includes.some(path => /stdio\.h$/i.test(path)), 'real SDK and CRT include provenance');
  for (const path of includes) {
    assert.ok(win32.isAbsolute(path) && roots.some(root => {
      const part = win32.relative(windowsPath(root), windowsPath(path));
      return part && part !== '..' && !part.startsWith('..\\') && !win32.isAbsolute(part);
    }), `unbound C header: ${path}`);
  }
  return includes;
}
export function verifyNativeLinkInputs(directory, compiler, object) {
  const files = inventory(directory), objectName = basename(object);
  for (const name of ['env.setting', 'link.rsp', objectName]) assert.ok(files.some(file => file.path === name && file.size > 0), `native link input missing: ${name}`);
  const copiedObject = files.find(file => file.path === objectName);
  assert.equal(copiedObject.sha256, fileDigest(object), 'native link object differs');
  const libraries = files.filter(file => /\.lib$/i.test(file.path));
  assert.ok(libraries.some(file => /^kernel32\.lib$/i.test(file.path)) && libraries.some(file => /^libcmt\.lib$/i.test(file.path)), 'real SDK and CRT library provenance');
  const declared = ['compiler-msvc-lib', 'compiler-sdk-um-lib', 'compiler-sdk-ucrt-lib'].flatMap(id => compiler.manifest.closures[id].files);
  for (const file of files) {
    if (['env.setting', 'link.rsp', objectName].includes(file.path)) continue;
    assert.ok(/^[^/]+\.lib$/i.test(file.path) && declared.some(input =>
      win32.basename(input.path).toLowerCase() === file.path.toLowerCase() && input.size === file.size && input.sha256 === file.sha256), `unbound native link input: ${file.path}`);
  }
  return files;
}

export function nativeBootstrapProof(root) {
  const host = loaderObservedWindows(), auditScript = runningInputFiles()['compiler-audit-script'];
  const original = readFileSync(auditScript, 'utf8');
  const creation = '$auditDirectory = [IO.Directory]::CreateDirectory($auditTempPath, $auditAcl)';
  const declaration = 'public static class AwareCompilerAudit {';
  assert.ok(original.includes(creation) && original.includes(declaration), 'native bootstrap proof must instrument the actual implementation');
  for (const failure of [false, true]) {
    const work = join(root, `bootstrap Łódź 😀 ${failure ? 'failure' : 'success'}`); mkdirSync(work);
    const captured = join(work, 'owned-temp.txt'), instrumented = join(work, 'audit.ps1');
    const encoded = Buffer.from(captured, 'utf8').toString('base64');
    let script = original.replace(creation, `${creation}\n[IO.File]::WriteAllText([Text.Encoding]::UTF8.GetString([Convert]::FromBase64String('${encoded}')), $auditTempPath)`);
    if (failure) script = script.replace(declaration, 'public static class AwareCompilerAudit SYNTAX_ERROR {');
    writeFileSync(instrumented, script);
    const options = { locator: { tools: { powershell: host.powershell } },
      manifest: { tools: { powershell: { sha256: fileDigest(host.powershell) } }, inputs: { 'compiler-audit-script': fileDigest(instrumented) } },
      auditScript: instrumented, workRoot: work };
    if (failure) assert.throws(() => discoverSystemHost(options), error => {
      const stdout = readFileSync(join(work, 'windows-host-stdout.local.bin'));
      const stderr = readFileSync(join(work, 'windows-host-stderr.local.bin'));
      const launch = JSON.parse(readFileSync(join(work, 'windows-host-launch.local.json'), 'utf8'));
      assert.equal(launch.status, 1); assert.equal(launch.error, null);
      assert.ok(stderr.length > 0); assert.equal(readFileSync(join(work, 'windows-host-command.local.log'), 'utf8'), `${stdout}${stderr}`);
      assert.equal(error.message, `compiler auditor failed: 1\n${stdout}${stderr}`);
      assert.equal(existsSync(join(work, 'windows-host.local.json')), false); return true;
    });
    else assert.equal(windowsPath(discoverSystemHost(options).system32), windowsPath(host.system32));
    const owned = readFileSync(captured, 'utf8').replace(/^\uFEFF/, '');
    assert.equal(windowsPath(win32.dirname(owned)), windowsPath(win32.join(host.windows, 'Temp')));
    assert.match(win32.basename(owned), /^aware-compiler-audit-[0-9a-f]{32}$/);
    assert.equal(existsSync(owned), false, 'bootstrap temporary directory must be removed on success and compilation failure');
  }
  console.log('Native bootstrap: Unicode work paths and owned-temp cleanup passed on success and failure');
}

export function nativeLifecycleProof(root, sourceOverride) {
  const source = sourceOverride ?? readFileSync(runningInputFiles()['compiler-audit-script'], 'utf8');
  const marker = '$utf8 = New-Object System.Text.UTF8Encoding($false)';
  assert.equal(source.split(marker).length, 2, 'unique authenticated bootstrap boundary');
  assert.equal(source.split("Add-Type -TypeDefinition @'").length, 2, 'unique production C# literal');
  assert.equal(source.split("\n'@").length, 2, 'unique production C# terminator');
  // The replay compiles the actual type; these checks also prevent an unused
  // helper from satisfying the replay while Run keeps the defective old state.
  for (const call of ['var lifetimes=new ProcessLifetimes(report)', 'lifetimes.RequireNew(pid)',
    'else lifetimes.RequireActive(pid)', 'lifetimes.Begin(image,process)',
    'lifetimes.Dll(Capture(pid,"dll",file,lifetimes.Handle(pid)',
    'lifetimes.End(pid,(uint)Marshal.ReadInt32(eventBuffer,16))', 'lifetimes.InitialBreakpoint(pid)']) {
    assert.ok(source.includes(call), `native loop must use lifetime transition: ${call}`);
  }
  const work = join(root, 'lifetime replay'); mkdirSync(work);
  const script = join(work, 'replay.ps1'), request = join(work, 'request.json'), output = join(work, 'report.json');
  writeFileSync(request, JSON.stringify({ output }));
  writeFileSync(script, source.slice(0, source.indexOf(marker)) + `
function Check($condition, $message) { if (!$condition) { throw $message } }
function Refused([scriptblock]$operation) { $failed = $false; try { & $operation } catch { $failed = $true }; Check $failed 'Invalid lifecycle transition was accepted' }
function NewImage([uint32]$number, [string]$kind) {
  $image = New-Object AwareCompilerAudit+Image
  $image.pid = $number; $image.kind = $kind; $image.path = 'C:\\private\\compiler.exe'; $image.size = 1; $image.sha256 = 'a' * 64
  return $image
}
$report = New-Object AwareCompilerAudit+Report
$state = [AwareCompilerAudit+ProcessLifetimes]::new($report)
Refused { $state.End(480, 0) }
Refused { $state.Dll((NewImage 480 'dll')) }
Refused { $state.InitialBreakpoint(480) }
$report.eventCount = 1; $first = $state.Begin((NewImage 42 'process'), [IntPtr]42)
$report.eventCount = 2; $old = $state.Begin((NewImage 480 'process'), [IntPtr]100)
Refused { $state.RequireNew(480) }
Refused { $state.Begin((NewImage 480 'process'), [IntPtr]999) }
Check ($report.processes.Count -eq 2 -and $report.images.Count -eq 2) 'Duplicate appended history'
$report.eventCount = 3
Check ($state.InitialBreakpoint(480)) 'First breakpoint missing'
Check (!$state.InitialBreakpoint(480)) 'Repeated breakpoint was accepted'
$report.eventCount = 4; $state.Dll((NewImage 480 'dll'))
$report.eventCount = 5; [void]$state.End(480, 0)
Check ($state.Count -eq 1) 'Exited lifetime remains active'
Refused { $state.Handle(480) }
Refused { $state.RequireActive(480) }
Refused { $state.End(480, 99) }
$report.eventCount = 6; $new = $state.Begin((NewImage 480 'process'), [IntPtr]200)
Check ($new.instance -eq 3 -and $state.Handle(480) -eq [IntPtr]200) 'Reused lifetime has stale identity or handle'
Check ($state.InitialBreakpoint(480)) 'Reused lifetime inherited a breakpoint'
Check (!$state.InitialBreakpoint(480)) 'Reused lifetime accepts duplicate breakpoint'
$report.eventCount = 7; $state.Dll((NewImage 480 'dll'))
$report.eventCount = 8; [void]$state.End(480, 7)
$report.eventCount = 9; [void]$state.End(42, 0)
Check ($state.Count -eq 0 -and $state.RootExited) 'Replay did not retire all processes'
Check ($old.exitCode -eq 0 -and $old.exitEvent -eq 5 -and $new.exitCode -eq 7 -and $new.exitEvent -eq 8) 'History was overwritten'
# Reuse the root PID too; its later exit must not replace the original root status.
$report.eventCount = 10; [void]$state.Begin((NewImage 42 'process'), [IntPtr]300)
$report.eventCount = 11; [void]$state.End(42, 9)
Check ($report.exitCode -eq 0 -and $state.RootExited) 'Root identity followed a reused PID'
[IO.File]::WriteAllText($request.output, ($report | ConvertTo-Json -Depth 15), [Text.UTF8Encoding]::new($false))
`);
  const host = loaderObservedWindows();
  const result = spawnSync(host.powershell, ['-NoProfile', '-NonInteractive', '-File', script, '-RequestPath', request],
    { env: systemEnvironment(host, auditorTempParent(host)), encoding: null, windowsHide: true, timeout: 30000 });
  writeFileSync(join(work, 'stdout.bin'), result.stdout ?? Buffer.alloc(0)); writeFileSync(join(work, 'stderr.bin'), result.stderr ?? Buffer.alloc(0));
  assert.ifError(result.error); assert.equal(result.status, 0, String(result.stderr));
  const report = JSON.parse(readFileSync(output, 'utf8'));
  assert.deepEqual(report.processes.map(record => [record.instance, record.pid, record.startEvent, record.exitEvent, record.exitCode]),
    [[1,42,1,9,0],[2,480,2,5,0],[3,480,6,8,7],[4,42,10,11,9]]);
  assert.deepEqual(report.images.map(image => [image.instance, image.pid, image.event, image.kind]),
    [[1,42,1,'process'],[2,480,2,'process'],[2,480,4,'dll'],[3,480,6,'process'],[3,480,7,'dll'],[4,42,10,'process']]);
  console.log('Native production lifecycle replay: PID reuse, root identity, breakpoint and handle retirement passed');
}

function installedRoots(run) {
  if (process.env.AWARE_REPRO_COMPILER_ROOTS) return JSON.parse(readFileSync(process.env.AWARE_REPRO_COMPILER_ROOTS, 'utf8'));
  const rust = dirname(dirname(run('rustup', ['which', '--toolchain', '1.95.0', 'rustc']).trim()));
  const msvc = process.env.VCToolsInstallDir, sdk = process.env.WindowsSdkDir, version = process.env.WindowsSDKVersion?.replace(/[\\/]+$/, '');
  assert.ok(msvc && sdk && version, 'native gate needs the VS developer environment or AWARE_REPRO_COMPILER_ROOTS');
  return { 'rust-bin': join(rust, 'bin'), 'rust-lib': join(rust, 'lib'),
    'msvc-bin': join(msvc, 'bin', 'Hostx64', 'x64'), 'msvc-include': join(msvc, 'include'), 'msvc-lib': join(msvc, 'lib', 'x64'),
    'sdk-include': join(sdk, 'Include', version), 'sdk-um-lib': join(sdk, 'Lib', version, 'um', 'x64'),
    'sdk-ucrt-lib': join(sdk, 'Lib', version, 'ucrt', 'x64'), 'sdk-bin': join(sdk, 'bin', version, 'x64') };
}
export function prepareNativeCompiler({ base, work, source, closure, side, run }) {
  console.log(`Native ${side}: copy and inventory actual compiler inputs`);
  const owned = join(base, 'owned compiler inputs'), installed = installedRoots(run);
  const evidence = process.env.AWARE_REPRO_TEST_EVIDENCE ? join(resolve(process.env.AWARE_REPRO_TEST_EVIDENCE), side) : join(base, 'evidence');
  mkdirSync(owned); mkdirSync(evidence, { recursive: true }); mkdirSync(join(work, 'cargo-target'));
  const closures = { 'cargo-home': closure, 'npm-cache': join(owned, 'npm') };
  mkdirSync(closures['npm-cache']); writeFileSync(join(closures['npm-cache'], 'fixture'), 'unused npm fixture');
  for (const id of COMPILER_IDS) {
    closures[id] = join(owned, id);
    copyDirectory(installed[id.slice('compiler-'.length)], closures[id]);
  }
  const fixture = join(owned, 'unused-source'); writeFileSync(fixture, 'native probe authority');
  const files = runningInputFiles(), auditScript = files['compiler-audit-script'];
  const { manifest, locator } = createWindowsBuilderRecords({ schema: 'aware-windows-repro-builder-inputs/v1',
    source: { commit: 'a'.repeat(40), tree: 'b'.repeat(40), bundle: fixture },
    inputs: Object.fromEntries(INPUT_IDS.map(id => [id, files[id] ?? fixture])),
    tools: Object.fromEntries(NONCOMPILER_TOOL_IDS.map(id => [id, id === 'powershell' ? loaderObservedWindows().powershell : process.execPath])), closures });
  const host = discoverSystemHost({ manifest, locator, auditScript, workRoot: work });
  const compiler = materializeCompiler({ manifest, locator, workRoot: work, host });
  const privateClosure = materializeClosure('cargo-home', closure, join(work, 'cargo-closure'), manifest, inventory);
  writeFileSync(join(closure, 'vendor', 'path-probe', 'src', 'lib.rs'), 'changed original after copying');
  assert.equal(canonicalJson(inventory(privateClosure)), canonicalJson(manifest.closures['cargo-home'].files));
  renameSync(owned, join(base, 'hidden original compiler')); renameSync(closure, join(base, 'hidden original cargo'));
  for (const path of Object.values(closures)) assert.equal(existsSync(path), false, 'original source path is unavailable');
  const audits = [];
  function audit(id, args, env, label, timeout = 180000) {
    const result = runAuditedCompiler({ compiler, toolPath: compiler.tools[id], args, env, label, cwd: source,
      auditScript, evidenceRoot: evidence, targetRoot: join(work, 'cargo-target'), timeout });
    audits.push({ label, sha256: result.evidenceSha256, processes: result.report.processes.length });
    writeFileSync(join(evidence, `${label}.log`), result.text); return result.text;
  }
  function finish(record) {
    verifyPrivateCompiler(compiler);
    assert.equal(canonicalJson(inventory(privateClosure)), canonicalJson(manifest.closures['cargo-home'].files));
    writeFileSync(join(evidence, 'summary.json'), canonicalJson({ ...record, audits }));
  }
  return { compiler, privateClosure, audit, finish, evidence, source, work, locator };
}
export function nativeVersionProof({ native, env }) {
  const { compiler, audit } = native;
  assert.equal(env.VCINSTALLDIR, join(compiler.root, 'msvc'));
  assert.equal(env.VSCMD_ARG_TGT_ARCH, 'x64');
  assert.match(audit('cargo', ['--version'], env, 'cargo-version'), /^cargo 1\.95\.0\b/);
  assert.match(audit('rustc', ['--version'], env, 'rust-version'), /^rustc 1\.95\.0\b/);
  for (const query of ['sysroot', 'target-libdir']) {
    const path = audit('rustc', ['--print', query], env, `rust-${query}`).trim();
    assert.ok(existsSync(path) && beneath(path, join(compiler.root, 'rust')));
  }
  assert.throws(() => audit('rustc', ['--version'], env, 'audit-deadline', 1), /compiler auditor failed/);
  const incomplete = JSON.parse(readFileSync(join(native.evidence, 'audit-deadline-audit.local.json'), 'utf8'));
  assert.equal(incomplete.complete, false); assert.match(incomplete.error, /deadline exceeded/);
  const launch = JSON.parse(readFileSync(join(native.evidence, 'audit-deadline-launch.local.json'), 'utf8'));
  assert.equal(launch.status, 1);
  assert.ok(readFileSync(join(native.evidence, 'audit-deadline-stderr.local.bin')).length > 0);
}
export function nativeToolsProof({ native, env, run }) {
  const { compiler, audit, evidence, source, work, locator } = native, target = join(work, 'cargo-target');
  console.log('Native compiler: mutation refusals, SDK/CRT provenance and auxiliary tools');
  for (const [index, path] of [join(compiler.roots['compiler-msvc-include'], 'vcruntime.h'), join(compiler.roots['compiler-sdk-um-lib'], 'kernel32.lib')].entries()) {
    const original = readFileSync(path), label = `mutation-${index}`, output = join(target, `must-not-exist-${index}.obj`);
    try {
      writeFileSync(path, Buffer.concat([original, Buffer.from('mutation')]));
      assert.throws(() => audit('cl', ['/c', join(source, 'native.c'), `/Fo${output}`], env, label), /private compiler changed/);
      assert.equal(existsSync(join(evidence, `${label}-request.local.json`)), false); assert.equal(existsSync(output), false);
    } finally { writeFileSync(path, original); }
  }
  assert.throws(() => { validateCompilerLocator({ ...locator, environment: { PATH: 'C:\\untrusted' } }); audit('rustc', ['--version'], env, 'hostile-locator'); }, /locator/);
  assert.equal(existsSync(join(evidence, 'hostile-locator-request.local.json')), false);
  const cSource = join(source, 'native.c'), object = join(target, 'native.obj'), library = join(target, 'native.lib'), executable = join(target, 'native.exe');
  writeFileSync(cSource, '#include <windows.h>\n#include <stdio.h>\nint main(void) { puts(GetCurrentProcessId() ? "private-compiler-ok" : "error"); return 0; }\n');
  const dependencies = join(target, 'includes.json'), linkRepro = join(target, 'link-repro');
  audit('cl', ['/nologo', '/c', '/MT', '/showIncludes', '/sourceDependencies', dependencies, `/Fo${object}`, cSource], env, 'native-compile');
  const includeReport = JSON.parse(readFileSync(dependencies, 'utf8').replace(/^\uFEFF/, ''));
  verifyNativeIncludes(includeReport, cSource, [compiler.roots['compiler-sdk-include'], compiler.roots['compiler-msvc-include']]);
  writeFileSync(join(evidence, 'native-includes.json'), canonicalJson(includeReport));
  audit('lib', ['/nologo', '/Brepro', `/OUT:${library}`, object], env, 'native-library');
  mkdirSync(linkRepro);
  audit('link', ['/nologo', '/Brepro', '/VERBOSE:LIB', `/LINKREPRO:${linkRepro}`, `/OUT:${executable}`, object, 'kernel32.lib'], env, 'native-link');
  const linkInputs = verifyNativeLinkInputs(linkRepro, compiler, object);
  writeFileSync(join(evidence, 'native-link-inputs.json'), canonicalJson(linkInputs));
  assert.equal(run(executable, [], { env }).trim(), 'private-compiler-ok');
  const resource = join(source, 'native.rc'), res = join(target, 'native.res');
  writeFileSync(resource, '1 RCDATA\nBEGIN\n123\nEND\n'); audit('rc', ['/nologo', `/fo${res}`, resource], env, 'native-resource'); assert.ok(existsSync(res));
  const docSource = join(source, 'docs.rs'); writeFileSync(docSource, '/// Private documentation probe.\npub fn probe() {}\n');
  audit('rustdoc', ['--crate-name', 'private_docs', docSource, '-o', join(target, 'docs')], env, 'native-rustdoc');
  assert.ok(existsSync(join(target, 'docs', 'private_docs', 'index.html')));
  return fileDigest(executable);
}
