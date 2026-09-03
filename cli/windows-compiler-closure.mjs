// Shared private Windows compiler contract. The production bootstrap authenticates this file before import.
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { copyFileSync, existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, realpathSync, writeFileSync } from 'node:fs';
import { basename, dirname, isAbsolute, join, relative, resolve, sep, win32 } from 'node:path';

const SHA256 = /^[0-9a-f]{64}$/;
const canonical = value => Array.isArray(value) ? value.map(canonical) : value && typeof value === 'object'
  ? Object.fromEntries(Object.keys(value).sort().map(key => [key, canonical(value[key])])) : value;
export const canonicalJson = value => `${JSON.stringify(canonical(value), null, 2)}\n`;
export const digest = bytes => createHash('sha256').update(bytes).digest('hex');
export const fileDigest = path => digest(readFileSync(path));
const portable = path => path.split(sep).join('/');
const same = (a, b, label) => assert.equal(canonicalJson(a), canonicalJson(b), label);
export function exactKeys(value, keys, label) {
  assert.ok(value && typeof value === 'object' && !Array.isArray(value), `${label} must be an object`);
  same(Object.keys(value).sort(), [...keys].sort(), `${label} has unexpected or missing keys`);
}
export const INPUT_IDS = Object.freeze(['aware-cargo-lock', 'reader-package-lock', 'builder-script',
  'compiler-closure-script', 'reader-settings-script', 'compiler-audit-script']);
export const NONCOMPILER_TOOL_IDS = Object.freeze(['git', 'node', 'npm-cli', 'postject', 'web-ifc-wasm', 'powershell']);
export const COMPILER_LAYOUT = Object.freeze({
  'compiler-rust-bin': 'rust/bin', 'compiler-rust-lib': 'rust/lib',
  'compiler-msvc-bin': 'msvc/bin', 'compiler-msvc-include': 'msvc/include', 'compiler-msvc-lib': 'msvc/lib',
  'compiler-sdk-include': 'sdk/include', 'compiler-sdk-um-lib': 'sdk/um-lib',
  'compiler-sdk-ucrt-lib': 'sdk/ucrt-lib', 'compiler-sdk-bin': 'sdk/bin',
});
export const COMPILER_IDS = Object.freeze(Object.keys(COMPILER_LAYOUT));
export const CLOSURE_IDS = Object.freeze(['npm-cache', 'cargo-home', ...COMPILER_IDS]);
const tool = (closure, path) => Object.freeze({ closure, path });
export const COMPILER_DESCRIPTOR = Object.freeze({
  schema: 'aware-windows-compiler/v1', layout: COMPILER_LAYOUT,
  tools: Object.freeze({ cargo: tool('compiler-rust-bin', 'cargo.exe'), rustc: tool('compiler-rust-bin', 'rustc.exe'),
    rustdoc: tool('compiler-rust-bin', 'rustdoc.exe'), cl: tool('compiler-msvc-bin', 'cl.exe'),
    link: tool('compiler-msvc-bin', 'link.exe'), lib: tool('compiler-msvc-bin', 'lib.exe'), rc: tool('compiler-sdk-bin', 'rc.exe') }),
  environment: Object.freeze({ PATH: ['msvc/bin', 'sdk/bin', 'rust/bin', '<system32>'],
    INCLUDE: ['msvc/include', 'sdk/include/ucrt', 'sdk/include/shared', 'sdk/include/um', 'sdk/include/winrt', 'sdk/include/cppwinrt'],
    LIB: ['msvc/lib', 'sdk/ucrt-lib', 'sdk/um-lib'], LIBPATH: ['msvc/lib', 'sdk/ucrt-lib', 'sdk/um-lib'],
    PATHEXT: '.COM;.EXE;.BAT;.CMD', VSLANG: '1033',
    VCINSTALLDIR: ['msvc'], VSCMD_ARG_TGT_ARCH: 'x64' }),
  auditPolicy: 'aware-private-compiler-debug-events/v2',
  startupPolicy: Object.freeze({ identity: 'aware-private-msvc-telemetry-denial/v1',
    closure: 'compiler-msvc-bin', path: 'vctip.exe', exitCode: 0xe0000488 }),
});
const REQUIRED = Object.freeze({
  'compiler-rust-bin': [/^cargo\.exe$/, /^rustc\.exe$/, /^rustdoc\.exe$/, /^rustc_driver-[^/]+\.dll$/, /^std-[^/]+\.dll$/],
  'compiler-rust-lib': [/^rustlib\/x86_64-pc-windows-msvc\/lib\/libstd-[^/]+\.rlib$/, /^rustlib\/x86_64-pc-windows-msvc\/lib\/libcore-[^/]+\.rlib$/],
  'compiler-msvc-bin': [/^cl\.exe$/, /^link\.exe$/, /^lib\.exe$/, /^c1\.dll$/, /^c2\.dll$/, /^msvcp140\.dll$/, /^vcruntime140\.dll$/],
  'compiler-msvc-include': [/^vcruntime\.h$/],
  'compiler-msvc-lib': [/^libcmt\.lib$/, /^libvcruntime\.lib$/],
  'compiler-sdk-include': [/^ucrt\/stdio\.h$/, /^shared\/winerror\.h$/, /^um\/windows\.h$/, /^winrt\//, /^cppwinrt\//],
  'compiler-sdk-um-lib': [/^kernel32\.lib$/, /^user32\.lib$/],
  'compiler-sdk-ucrt-lib': [/^ucrt\.lib$/, /^libucrt\.lib$/],
  'compiler-sdk-bin': [/^rc\.exe$/, /^rcdll\.dll$/],
});
export function validateRecordPath(path) {
  assert.ok(typeof path === 'string' && path && !/[\\:<>"|?*\x00-\x1f\x7f]/.test(path), 'unsafe portable record path');
  for (const part of path.split('/')) {
    assert.ok(part && part !== '.' && part !== '..' && !/[. ]$/.test(part)
      && !/^(con|prn|aux|nul|com[1-9]|lpt[1-9])(?:\.|$)/i.test(part), 'unsafe portable record component');
  }
}
export function validateInventory(files, label) {
  assert.ok(Array.isArray(files) && files.length, `${label} inventory must be nonempty`);
  const seen = new Set(); let previous;
  for (const record of files) {
    exactKeys(record, ['path', 'size', 'sha256'], `${label} record`); validateRecordPath(record.path);
    assert.ok(Number.isSafeInteger(record.size) && record.size >= 0 && SHA256.test(record.sha256), `${label} invalid file record`);
    assert.ok(!seen.has(record.path.toLowerCase()), `${label} case-colliding file records`); seen.add(record.path.toLowerCase());
    if (previous != null) assert.ok(Buffer.compare(Buffer.from(previous), Buffer.from(record.path)) < 0, `${label} records are not sorted`);
    previous = record.path;
  }
}
export function validateCompilerManifest(manifest) {
  exactKeys(manifest, ['schema', 'platform', 'arch', 'nodeVersion', 'rustVersion', 'target', 'source', 'settings', 'compiler', 'inputs', 'tools', 'closures'], 'Windows builder manifest');
  assert.equal(manifest.schema, 'aware-windows-repro-builder/v1');
  assert.equal(manifest.platform, 'win32'); assert.equal(manifest.arch, 'x64');
  assert.equal(manifest.nodeVersion, '24.14.0'); assert.equal(manifest.rustVersion, '1.95.0');
  assert.equal(manifest.target, 'x86_64-pc-windows-msvc');
  exactKeys(manifest.source, ['commit', 'tree', 'bundleSha256'], 'builder source');
  assert.ok(/^[0-9a-f]{40}$/.test(manifest.source.commit) && /^[0-9a-f]{40}$/.test(manifest.source.tree) && SHA256.test(manifest.source.bundleSha256), 'invalid builder source identity');
  exactKeys(manifest.inputs, INPUT_IDS, 'builder code/lock inputs');
  for (const value of Object.values(manifest.inputs)) assert.match(value, SHA256);
  exactKeys(manifest.tools, NONCOMPILER_TOOL_IDS, 'noncompiler tool records');
  for (const id of NONCOMPILER_TOOL_IDS) { exactKeys(manifest.tools[id], ['id', 'sha256'], `${id} tool record`); assert.equal(manifest.tools[id].id, id); assert.match(manifest.tools[id].sha256, SHA256); }
  same(manifest.compiler, COMPILER_DESCRIPTOR, 'compiler descriptor differs from the closed contract');
  exactKeys(manifest.closures, CLOSURE_IDS, 'compiler/dependency closures');
  for (const id of CLOSURE_IDS) {
    exactKeys(manifest.closures[id], ['files'], `${id} closure`);
    validateInventory(manifest.closures[id].files, id);
    const names = manifest.closures[id].files.map(record => record.path.toLowerCase());
    for (const required of REQUIRED[id] ?? []) assert.ok(names.some(path => required.test(path)), `${id} lacks mandatory compiler support ${required}`);
  }
  const rust = manifest.closures['compiler-rust-bin'].files;
  assert.equal(new Set(['cargo.exe', 'rustc.exe', 'rustdoc.exe'].map(name => rust.find(file => file.path.toLowerCase() === name)?.sha256)).size,
    3, 'Rust roles must identify distinct direct binaries, not rustup shims');
  same(manifest.settings, { bundle: { platform: 'node', format: 'cjs', target: 'node24' },
    sea: { disableExperimentalWarning: true, section: 'NODE_SEA_BLOB', sentinelFuse: 'NODE_SEA_FUSE_fce680ab2cc467b6e072b8b5df1996b2' } }, 'reader settings differ from compiler authority');
}
export function validateWindowsPath(path, label = 'physical root', maximumLength = 200) {
  assert.ok(typeof path === 'string' && /^[a-z]:[\\/]/i.test(path) && path.length <= maximumLength
    && !/[;=<>"|?*\x00-\x1f\x7f]/.test(path) && !path.slice(2).includes(':'), `${label} must be a bounded local Windows drive path`);
  for (const part of path.slice(3).split(/[\\/]/)) {
    assert.ok(part && part !== '.' && part !== '..' && !/[. ]$/.test(part)
      && !/^(con|prn|aux|nul|com[1-9]|lpt[1-9])(?:\.|$)/i.test(part), `${label} has an unsafe path component`);
  }
  return win32.normalize(path);
}
export function beneath(path, root) {
  const child = resolve(path).toLowerCase(), parent = resolve(root).toLowerCase();
  return child === parent || child.startsWith(`${parent}${sep}`);
}
function checkedEntry(path, canonicalRoot) {
  const stat = lstatSync(path); assert.ok(!stat.isSymbolicLink(), `path-redirection link refused: ${path}`);
  assert.ok(beneath(realpathSync.native(path), canonicalRoot), `canonical path escaped closure: ${path}`);
  return stat;
}
export function inventory(root) {
  assert.ok(isAbsolute(root), 'closure root must be absolute');
  const stat = lstatSync(root); assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), 'closure root must be a real directory');
  const files = [], canonicalRoot = realpathSync.native(root);
  function walk(dir) {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const path = join(dir, entry.name), current = checkedEntry(path, canonicalRoot);
      if (current.isDirectory()) walk(path);
      else { assert.ok(current.isFile(), 'unsupported closure entry'); files.push({ path: portable(relative(root, path)), size: current.size, sha256: fileDigest(path) }); }
    }
  }
  walk(root); files.sort((a, b) => Buffer.compare(Buffer.from(a.path), Buffer.from(b.path)));
  validateInventory(files, root); return files;
}
export function copyDirectory(source, destination) {
  const files = inventory(source);
  assert.ok(!existsSync(destination), 'private copy destination must be fresh');
  mkdirSync(destination, { recursive: true });
  // Node 24.14.0 cpSync mangles Unicode directory names on Windows. Explicit
  // file copies preserve the inventoried names and do not share mutable bytes.
  for (const record of files) {
    const output = join(destination, ...record.path.split('/')); mkdirSync(dirname(output), { recursive: true });
    copyFileSync(join(source, ...record.path.split('/')), output);
  }
  same(inventory(destination), files, 'private directory copy differs from its source inventory');
}
export function validateCompilerLocator(locator) {
  exactKeys(locator, ['schema', 'sourceBundle', 'tools', 'closures'], 'Windows builder locator');
  assert.equal(locator.schema, 'aware-windows-repro-locator/v1');
  exactKeys(locator.tools, NONCOMPILER_TOOL_IDS, 'noncompiler tool locator');
  exactKeys(locator.closures, CLOSURE_IDS, 'closure locator');
  for (const [id, path] of [...Object.entries(locator.tools), ...Object.entries(locator.closures), ['source bundle', locator.sourceBundle]]) validateWindowsPath(path, id);
}
export function compilerSummary(manifest) {
  validateCompilerManifest(manifest);
  return { schema: 'aware-compiler-authority/v1', descriptorSha256: digest(canonicalJson(COMPILER_DESCRIPTOR)),
    auditPolicy: COMPILER_DESCRIPTOR.auditPolicy,
    closures: Object.fromEntries(COMPILER_IDS.map(id => [id, digest(canonicalJson(manifest.closures[id].files))])) };
}
export function materializeCompiler({ manifest, locator, workRoot, host }) {
  validateCompilerManifest(manifest); validateCompilerLocator(locator); validateWindowsPath(workRoot, 'work root');
  const root = join(workRoot, 'compiler'); assert.ok(!existsSync(root), 'private compiler root must be fresh');
  const roots = {};
  for (const id of COMPILER_IDS) {
    same(inventory(locator.closures[id]), manifest.closures[id].files, `${id} source inventory changed`);
    const destination = join(root, ...COMPILER_LAYOUT[id].split('/')); mkdirSync(dirname(destination), { recursive: true });
    copyDirectory(locator.closures[id], destination); roots[id] = destination;
    same(inventory(destination), manifest.closures[id].files, `${id} private inventory changed`);
  }
  const tools = Object.fromEntries(Object.entries(COMPILER_DESCRIPTOR.tools).map(([id, role]) => [id, join(roots[role.closure], role.path)]));
  const environment = Object.fromEntries(Object.entries(COMPILER_DESCRIPTOR.environment).map(([key, value]) => [key,
    Array.isArray(value) ? value.map(path => path === '<system32>' ? host.system32 : join(root, ...path.split('/'))).join(';') : value]));
  return Object.freeze({ root, roots: Object.freeze(roots), tools: Object.freeze(tools), environment: Object.freeze(environment), host, manifest });
}
export function verifyPrivateCompiler(compiler) {
  validateCompilerManifest(compiler.manifest);
  for (const id of COMPILER_IDS) same(inventory(compiler.roots[id]), compiler.manifest.closures[id].files, `${id} private compiler changed`);
}
export function loaderObservedWindows(sharedObjects = process.report.getReport().sharedObjects) {
  const locate = name => {
    const matches = [...new Set(sharedObjects.filter(path => win32.basename(path).toLowerCase() === name).map(path => win32.normalize(path).toLowerCase()))];
    assert.equal(matches.length, 1, `expected one loader-observed ${name}`); return matches[0];
  };
  const kernel = win32.dirname(locate('kernel32.dll')), ntdll = win32.dirname(locate('ntdll.dll'));
  assert.equal(kernel, ntdll, 'loader-observed Windows modules disagree'); assert.equal(win32.basename(kernel), 'system32');
  return { windows: win32.dirname(kernel), system32: kernel, powershell: win32.join(kernel, 'WindowsPowerShell', 'v1.0', 'powershell.exe') };
}
export function systemEnvironment(host, tempRoot) {
  return { SystemRoot: host.windows, WINDIR: host.windows, ComSpec: join(host.system32, 'cmd.exe'),
    PATHEXT: COMPILER_DESCRIPTOR.environment.PATHEXT, PATH: host.system32,
    PSModulePath: join(host.system32, 'WindowsPowerShell', 'v1.0', 'Modules'), TEMP: tempRoot, TMP: tempRoot };
}
export function auditorTempParent(host) {
  const parent = win32.join(host.windows, 'Temp');
  validateWindowsPath(parent, 'auditor temporary parent');
  assert.match(parent, /^[\x20-\x7e]+$/, 'inbox compiler auditor needs an ASCII Windows Temp path');
  const entry = lstatSync(parent);
  assert.ok(entry.isDirectory() && !entry.isSymbolicLink(), 'auditor temporary parent must be a real directory');
  assert.equal(realpathSync.native(parent).toLowerCase(), parent.toLowerCase(), 'auditor temporary parent redirects elsewhere');
  return parent;
}
function launchAuditor({ host, auditScript, auditDigest, request, requestPath, cwd, timeout = 120000 }) {
  const bytes = readFileSync(auditScript);
  assert.equal(digest(bytes), auditDigest, 'compiler auditor script changed before evaluation');
  writeFileSync(requestPath, canonicalJson(request), { flag: 'wx' });
  // Feed exact authenticated bytes through stdin. -File would re-read a mutable
  // path; embedding the whole script in -EncodedCommand exceeds Windows argv limits.
  const pathBytes = Buffer.from(requestPath, 'utf8').toString('base64');
  const command = `& ([ScriptBlock]::Create([Text.Encoding]::UTF8.GetString([Convert]::FromBase64String([Console]::In.ReadToEnd())))) -RequestPath ([Text.Encoding]::UTF8.GetString([Convert]::FromBase64String('${pathBytes}')))`;
  const result = spawnSync(host.powershell, ['-NoProfile', '-NonInteractive', '-Command', command],
    { cwd, env: systemEnvironment(host, auditorTempParent(host)), input: bytes.toString('base64'), encoding: null, windowsHide: true, timeout, maxBuffer: 128 * 1024 * 1024 });
  if (result.error || result.status !== 0) throw new Error(`compiler auditor failed: ${result.error?.message ?? result.status}\n${result.stdout ?? ''}${result.stderr ?? ''}`);
  return { text: `${result.stdout ?? ''}${result.stderr ?? ''}`, stdout: result.stdout, stderr: result.stderr };
}
export function discoverSystemHost({ locator, manifest, auditScript, workRoot }) {
  const observed = loaderObservedWindows();
  assert.equal(realpathSync.native(locator.tools.powershell).toLowerCase(), realpathSync.native(observed.powershell).toLowerCase(), 'only canonical inbox PowerShell is allowed');
  assert.equal(fileDigest(observed.powershell), manifest.tools.powershell.sha256, 'inbox PowerShell digest changed');
  assert.equal(fileDigest(auditScript), manifest.inputs['compiler-audit-script'], 'audit script digest changed');
  const cwd = join(workRoot, 'bootstrap'); mkdirSync(cwd, { recursive: true }); assert.equal(readdirSync(cwd).length, 0, 'bootstrap cwd must be empty');
  const output = join(workRoot, 'windows-host.local.json');
  launchAuditor({ host: observed, auditScript, auditDigest: manifest.inputs['compiler-audit-script'], request: { mode: 'host', output }, requestPath: join(workRoot, 'windows-host-request.local.json'), cwd });
  const actual = JSON.parse(readFileSync(output, 'utf8'));
  exactKeys(actual, ['windows', 'system32'], 'Windows API host roots');
  for (const key of ['windows', 'system32']) assert.equal(realpathSync.native(actual[key]).toLowerCase(), realpathSync.native(observed[key]).toLowerCase(), `Windows API ${key} disagrees with loaded OS modules`);
  return Object.freeze({ ...actual, powershell: observed.powershell });
}
export function protectedWindowsPath(path, host) {
  const value = win32.resolve(path);
  if (win32.dirname(value).toLowerCase() === win32.resolve(host.system32).toLowerCase()) return true;
  return ['WinSxS', 'Microsoft.NET/Framework', 'Microsoft.NET/Framework64', 'Microsoft.NET/assembly', 'System32/WindowsPowerShell']
    .some(part => windowsBeneath(value, win32.join(host.windows, ...part.split('/'))));
}
// Audit records always describe Windows paths, including when portable validation runs on Linux.
function windowsBeneath(path, root) {
  const value = win32.relative(win32.resolve(root).toLowerCase(), win32.resolve(path).toLowerCase());
  return value !== '' && value !== '..' && !value.startsWith('..\\') && !win32.isAbsolute(value);
}
export function compilerStartupPolicy(compiler) {
  const policy = COMPILER_DESCRIPTOR.startupPolicy;
  const file = compiler.manifest.closures[policy.closure].files.find(file => file.path.toLowerCase() === policy.path);
  return { identity: policy.identity, deniedImage: file ? { closure: policy.closure, relativePath: file.path,
    path: win32.resolve(compiler.roots[policy.closure], file.path), size: file.size, sha256: file.sha256, exitCode: policy.exitCode } : null };
}
export function verifyCompilerAudit(report, { compiler, targetRoot, toolPath }) {
  exactKeys(report, ['schema', 'error', 'complete', 'exitCode', 'totalProcesses', 'activeProcesses', 'processes', 'images', 'identity', 'startupPolicy'], 'compiler audit');
  assert.equal(report.schema, 'aware-compiler-debug-audit/v2'); assert.equal(report.complete, true, 'compiler audit is incomplete');
  const startupPolicy = compilerStartupPolicy(compiler), denied = startupPolicy.deniedImage;
  same(report.startupPolicy, startupPolicy, 'compiler startup policy differs');
  assert.equal(report.error, null, 'compiler audit contains an error');
  assert.ok(Array.isArray(report.processes) && Array.isArray(report.images), 'compiler audit lists are missing');
  assert.ok(report.processes.length > 0 && report.images.length >= report.processes.length, 'compiler audit is empty');
  assert.equal(report.totalProcesses, report.processes.length, 'compiler audit missed a child process'); assert.equal(report.activeProcesses, 0);
  assert.ok(Number.isSafeInteger(report.exitCode) && report.exitCode >= 0, 'compiler audit exit status is invalid');
  const pids = new Set();
  for (const process of report.processes) {
    exactKeys(process, ['pid', 'path', 'exitCode', 'action'], 'audited process');
    assert.ok(Number.isSafeInteger(process.pid) && process.pid > 0 && !pids.has(process.pid), 'invalid or repeated audited process'); pids.add(process.pid);
    assert.ok(Number.isSafeInteger(process.exitCode) && process.exitCode >= 0, 'audited process never exited');
    assert.equal(report.images.filter(image => image.pid === process.pid && image.kind === 'process' && image.path === process.path).length, 1, 'audited process image is missing or repeated');
    const matches = denied && win32.resolve(process.path).toLowerCase() === denied.path.toLowerCase();
    assert.equal(process.action, matches ? 'blocked-telemetry' : 'observed', 'compiler process disposition differs');
    if (matches) {
      assert.notEqual(process.pid, report.processes[0].pid, 'root compiler cannot be blocked telemetry');
      assert.equal(process.exitCode, denied.exitCode, 'blocked telemetry exit status differs');
      const image = report.images.find(image => image.pid === process.pid && image.kind === 'process');
      assert.equal(image.sha256, denied.sha256, 'blocked telemetry digest differs');
      assert.equal(image.size, denied.size, 'blocked telemetry size differs');
    }
  }
  assert.equal(report.processes[0].exitCode, report.exitCode, 'root compiler exit status differs');
  assert.ok(toolPath && win32.resolve(report.processes[0].path).toLowerCase() === win32.resolve(toolPath).toLowerCase(), 'compiler audit root differs from the requested tool');
  same(report.identity, { source: compiler.manifest.source, buildId: digest(canonicalJson(compiler.manifest)),
    auditScriptSha256: compiler.manifest.inputs['compiler-audit-script'] }, 'compiler audit identity differs');
  const classified = report.images.map(image => {
    exactKeys(image, ['pid', 'path', 'kind', 'sha256', 'size'], 'audited image');
    assert.ok(pids.has(image.pid) && ['process', 'dll'].includes(image.kind), 'unclassified process/image event');
    validateWindowsPath(image.path, 'audited image', 32760);
    assert.ok(Number.isSafeInteger(image.size) && image.size > 0 && SHA256.test(image.sha256), 'unhashed compiler image');
    if (protectedWindowsPath(image.path, compiler.host)) return { ...image, role: 'windows' };
    for (const id of COMPILER_IDS) if (windowsBeneath(image.path, compiler.roots[id])) {
      const path = win32.relative(compiler.roots[id], image.path).replaceAll('\\', '/').toLowerCase();
      const expected = compiler.manifest.closures[id].files.find(file => file.path.toLowerCase() === path);
      assert.ok(expected && expected.size === image.size && expected.sha256 === image.sha256, `unbound compiler image: ${path}`);
      return { ...image, role: id };
    }
    assert.ok(windowsBeneath(image.path, targetRoot) && /\.(exe|dll)$/i.test(image.path), `compiler loaded an image outside its authority: ${image.path}`);
    return { ...image, role: 'derived-cargo-output' };
  });
  return { ...report, images: classified };
}
export function runAuditedCompiler({ compiler, toolPath, args, cwd, env, auditScript, evidenceRoot, label, targetRoot, timeout = 5400000 }) {
  verifyPrivateCompiler(compiler);
  assert.equal(fileDigest(auditScript), compiler.manifest.inputs['compiler-audit-script'], 'compiler auditor script changed');
  assert.ok(Object.values(compiler.tools).some(path => resolve(path).toLowerCase() === resolve(toolPath).toLowerCase()), 'audited command is not a private compiler role');
  assert.match(label, /^[a-z0-9-]+$/); mkdirSync(evidenceRoot, { recursive: true });
  const output = join(evidenceRoot, `${label}-audit.local.json`);
  const request = { mode: 'run', output, executable: toolPath, args, cwd, environment: env, timeoutMs: timeout,
    startupPolicy: compilerStartupPolicy(compiler),
    windows: compiler.host.windows, system32: compiler.host.system32,
    identity: { source: compiler.manifest.source, buildId: digest(canonicalJson(compiler.manifest)), auditScriptSha256: compiler.manifest.inputs['compiler-audit-script'] } };
  const captured = launchAuditor({ host: compiler.host, auditScript, auditDigest: compiler.manifest.inputs['compiler-audit-script'], request, requestPath: join(evidenceRoot, `${label}-request.local.json`), cwd: join(dirname(compiler.root), 'bootstrap'), timeout: timeout + 30000 });
  const { text } = captured;
  writeFileSync(join(evidenceRoot, `${label}-command.local.log`), text);
  writeFileSync(join(evidenceRoot, `${label}-stdout.local.bin`), captured.stdout);
  writeFileSync(join(evidenceRoot, `${label}-stderr.local.bin`), captured.stderr);
  const report = verifyCompilerAudit(JSON.parse(readFileSync(output, 'utf8')), { compiler, targetRoot, toolPath });
  assert.equal(report.exitCode, 0, `private compiler failed: ${text}`);
  return { ...captured, report, evidencePath: output, evidenceSha256: fileDigest(output) };
}

// Outer A/B acceptance must validate each complete process audit, not just compare artifact bytes.
export function verifyCompilerProvenance({ buildRoot, manifest, host = loaderObservedWindows() }) {
  validateCompilerManifest(manifest);
  const read = path => JSON.parse(readFileSync(path, 'utf8'));
  const record = read(join(buildRoot, 'evidence', 'compiler-provenance.json'));
  exactKeys(record, ['schema', 'source', 'buildId', 'compiler', 'audits', 'artifacts'], 'compiler provenance');
  assert.equal(record.schema, 'aware-compiler-provenance/v1');
  same(record.source, manifest.source, 'compiler provenance source differs');
  assert.equal(record.buildId, digest(canonicalJson(manifest)), 'compiler provenance build differs');
  same(record.compiler, compilerSummary(manifest), 'compiler provenance authority differs');
  same(record.artifacts, inventory(join(buildRoot, 'artifacts')), 'compiler provenance artifact inventory differs');
  const root = join(buildRoot, 'work', 'compiler');
  const roots = Object.fromEntries(COMPILER_IDS.map(id => [id, join(root, ...COMPILER_LAYOUT[id].split('/'))]));
  const compiler = { root, roots, host, manifest };
  verifyPrivateCompiler(compiler);
  const labels = ['cargo-version', 'rust-version', 'rust-sysroot', 'rust-target-libdir', 'cargo-build'];
  assert.ok(Array.isArray(record.audits) && record.audits.length === labels.length, 'compiler provenance audit set is incomplete');
  same(record.audits.map(audit => audit.label).sort(), [...labels].sort(), 'compiler provenance audit labels differ');
  for (const audit of record.audits) {
    exactKeys(audit, ['label', 'path', 'sha256'], 'compiler provenance audit reference');
    assert.equal(audit.path, `evidence/${audit.label}-audit.local.json`);
    assert.equal(fileDigest(join(buildRoot, ...audit.path.split('/'))), audit.sha256, 'retained compiler audit digest differs');
    const role = audit.label.startsWith('cargo-') ? COMPILER_DESCRIPTOR.tools.cargo : COMPILER_DESCRIPTOR.tools.rustc;
    const toolPath = join(roots[role.closure], role.path);
    const report = verifyCompilerAudit(read(join(buildRoot, ...audit.path.split('/'))), { compiler, toolPath, targetRoot: join(buildRoot, 'work', 'cargo-target') });
    assert.equal(report.exitCode, 0, 'retained compiler failed');
  }
  return { sha256: fileDigest(join(buildRoot, 'evidence', 'compiler-provenance.json')), audits: record.audits };
}
